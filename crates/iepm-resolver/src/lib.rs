use iepm_core::{
    Component, LockedPackage, Lockfile, Manifest, PackageRecord, Registry, Release, Toolchain,
};
use semver::{Version, VersionReq};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ResolveError {
    #[error("manifest schema {0} is unsupported")]
    Schema(u32),
    #[error("package `{0}` is not in the registry")]
    MissingPackage(String),
    #[error("package `{package}` has no release compatible with `{game}`")]
    Incompatible { package: String, game: String },
    #[error("package `{package}` has no release for `{game}` that matches {requirements}")]
    NoMatchingVersion {
        package: String,
        game: String,
        requirements: String,
    },
    #[error("package `{package}` has invalid version requirement `{requirement}`")]
    InvalidVersionRequirement {
        package: String,
        requirement: String,
    },
    #[error("package `{package}` has invalid release version `{version}`")]
    InvalidReleaseVersion { package: String, version: String },
    #[error("package `{package}` requests unknown component `{component}`")]
    MissingComponent { package: String, component: String },
    #[error("exclusive capability `{capability}` is provided by both `{first}` and `{second}`")]
    Capability {
        capability: String,
        first: String,
        second: String,
    },
    #[error("ordering graph contains a cycle")]
    Cycle,
}

struct Selection<'a> {
    package: &'a str,
    release: &'a Release,
    components: Vec<String>,
}

#[derive(Default)]
struct Request {
    requirements: BTreeSet<String>,
    components: BTreeSet<String>,
}

impl Request {
    fn merge(&mut self, version: Option<&str>, components: &[String]) -> bool {
        let before = (self.requirements.len(), self.components.len());
        if let Some(version) = version {
            self.requirements.insert(version.to_owned());
        }
        self.components.extend(components.iter().cloned());
        before != (self.requirements.len(), self.components.len())
    }
}

pub fn resolve(
    manifest: &Manifest,
    registry: &Registry,
    registry_revision: &str,
    toolchain: Toolchain,
) -> Result<Lockfile, ResolveError> {
    if manifest.schema != 1 {
        return Err(ResolveError::Schema(manifest.schema));
    }
    let mut requested = BTreeMap::<String, Request>::new();
    for requested_mod in &manifest.mods {
        requested
            .entry(requested_mod.package().to_owned())
            .or_default()
            .merge(requested_mod.version(), requested_mod.components());
    }

    let mut pending = requested.keys().cloned().collect::<Vec<_>>();
    while let Some(package) = pending.pop() {
        let record = registry
            .get(&package)
            .ok_or_else(|| ResolveError::MissingPackage(package.clone()))?;
        let release = select_release(
            record,
            &manifest.game.target,
            &requested[&package].requirements,
        )?;
        for dependency in &release.dependencies {
            let is_new = !requested.contains_key(&dependency.package);
            let changed = requested
                .entry(dependency.package.clone())
                .or_default()
                .merge(dependency.version.as_deref(), &dependency.components);
            if is_new || changed {
                pending.push(dependency.package.clone());
            }
        }
    }

    let mut selections = Vec::new();
    for (package, request) in &requested {
        let record = registry
            .get(package)
            .expect("all requested packages were checked");
        let release = select_release(record, &manifest.game.target, &request.requirements)?;
        let components = request.components.iter().cloned().collect::<Vec<_>>();
        validate_components(package, release, &components)?;
        selections.push(Selection {
            package,
            release,
            components,
        });
    }
    validate_capabilities(&selections)?;
    let order = order(&selections)?;

    let selected = selections
        .into_iter()
        .map(|selection| LockedPackage {
            package: selection.package.to_owned(),
            version: selection.release.version.clone(),
            phase: selection.release.install.phase,
            artifact: selection.release.artifact.clone(),
            components: selection.components,
            dependencies: selection.release.dependencies.clone(),
            provenance: selection.release.provenance,
        })
        .collect();

    Ok(Lockfile {
        schema: 1,
        game: manifest.game.clone(),
        registry_revision: registry_revision.to_owned(),
        toolchain,
        packages: selected,
        install_order: order,
    })
}

fn select_release<'a>(
    record: &'a PackageRecord,
    game: &str,
    requirements: &BTreeSet<String>,
) -> Result<&'a Release, ResolveError> {
    let requirements = requirements
        .iter()
        .map(|requirement| {
            VersionReq::parse(requirement).map_err(|_| ResolveError::InvalidVersionRequirement {
                package: record.package.clone(),
                requirement: requirement.clone(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut has_compatible_release = false;
    let mut selected = None;
    for release in &record.releases {
        let version = parse_release_version(&record.package, &release.version)?;
        if !release
            .compatibility
            .games
            .iter()
            .any(|candidate| candidate == game)
        {
            continue;
        }
        has_compatible_release = true;
        if requirements
            .iter()
            .all(|requirement| requirement.matches(&version))
            && selected
                .as_ref()
                .is_none_or(|(_, current)| version > *current)
        {
            selected = Some((release, version));
        }
    }
    selected.map(|(release, _)| release).ok_or_else(|| {
        if has_compatible_release {
            ResolveError::NoMatchingVersion {
                package: record.package.clone(),
                game: game.to_owned(),
                requirements: if requirements.is_empty() {
                    "any version".to_owned()
                } else {
                    requirements
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            }
        } else {
            ResolveError::Incompatible {
                package: record.package.clone(),
                game: game.to_owned(),
            }
        }
    })
}

fn parse_release_version(package: &str, raw: &str) -> Result<Version, ResolveError> {
    let raw = raw.trim().strip_prefix('v').unwrap_or(raw.trim());
    let suffix_index = raw.find(['-', '+']).unwrap_or(raw.len());
    let (core, suffix) = raw.split_at(suffix_index);
    let parts = core.split('.').count();
    let normalized = match parts {
        1 => format!("{core}.0.0{suffix}"),
        2 => format!("{core}.0{suffix}"),
        _ => raw.to_owned(),
    };
    Version::parse(&normalized).map_err(|_| ResolveError::InvalidReleaseVersion {
        package: package.to_owned(),
        version: raw.to_owned(),
    })
}

fn validate_components(
    package: &str,
    release: &Release,
    selected: &[String],
) -> Result<(), ResolveError> {
    for component in selected {
        if !release
            .components
            .iter()
            .any(|candidate| candidate.id == *component)
        {
            return Err(ResolveError::MissingComponent {
                package: package.to_owned(),
                component: component.clone(),
            });
        }
    }
    Ok(())
}

fn validate_capabilities(selections: &[Selection<'_>]) -> Result<(), ResolveError> {
    let mut exclusive = BTreeMap::<String, String>::new();
    for selection in selections {
        let components: Vec<&Component> = if selection.components.is_empty() {
            Vec::new()
        } else {
            selection
                .release
                .components
                .iter()
                .filter(|component| selection.components.contains(&component.id))
                .collect()
        };
        for component in components {
            for capability in &component.provides {
                if capability.exclusive {
                    let provider = format!("{}:{}", selection.package, component.id);
                    if let Some(first) = exclusive.insert(capability.name.clone(), provider.clone())
                    {
                        return Err(ResolveError::Capability {
                            capability: capability.name.clone(),
                            first,
                            second: provider,
                        });
                    }
                }
            }
        }
    }
    Ok(())
}

fn order(selections: &[Selection<'_>]) -> Result<Vec<String>, ResolveError> {
    let selected: BTreeSet<&str> = selections
        .iter()
        .map(|selection| selection.package)
        .collect();
    let mut edges = BTreeMap::<&str, BTreeSet<&str>>::new();
    let mut indegree = BTreeMap::<&str, usize>::new();
    for selection in selections {
        indegree.insert(selection.package, 0);
    }
    for selection in selections {
        for target in &selection.release.install.before {
            if selected.contains(target.as_str())
                && edges.entry(selection.package).or_default().insert(target)
            {
                *indegree.get_mut(target.as_str()).expect("target selected") += 1;
            }
        }
        for source in &selection.release.install.after {
            if selected.contains(source.as_str())
                && edges.entry(source).or_default().insert(selection.package)
            {
                *indegree
                    .get_mut(selection.package)
                    .expect("package selected") += 1;
            }
        }
    }
    let phase = selections
        .iter()
        .map(|selection| (selection.package, selection.release.install.phase))
        .collect::<BTreeMap<_, _>>();
    let mut ready = indegree
        .iter()
        .filter_map(|(package, degree)| (*degree == 0).then_some(*package))
        .collect::<Vec<_>>();
    let mut output = Vec::new();
    while !ready.is_empty() {
        ready.sort_by_key(|package| (phase[package], *package));
        let current = ready.remove(0);
        output.push(current.to_owned());
        for target in edges.get(current).into_iter().flatten() {
            let degree = indegree.get_mut(target).expect("edge target selected");
            *degree -= 1;
            if *degree == 0 {
                ready.push(target);
            }
        }
    }
    if output.len() != selections.len() {
        return Err(ResolveError::Cycle);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iepm_core::{
        Artifact, Compatibility, Dependency, GameTarget, Install, Manifest, PackageRecord, Phase,
        Provenance, RequestedMod,
    };
    use std::path::PathBuf;

    fn record(id: &str, phase: Phase) -> PackageRecord {
        PackageRecord {
            schema: 1,
            package: id.to_owned(),
            releases: vec![release("1", phase)],
        }
    }

    fn release(version: &str, phase: Phase) -> Release {
        Release {
            version: version.to_owned(),
            artifact: None,
            compatibility: Compatibility {
                games: vec!["eet".to_owned()],
            },
            install: Install {
                phase,
                before: vec![],
                after: vec![],
            },
            provenance: Provenance::Verified,
            dependencies: vec![],
            components: vec![],
        }
    }

    fn test_toolchain() -> Toolchain {
        Toolchain {
            iepm: "test-manager".to_owned(),
            weidu: Some("24600".to_owned()),
        }
    }

    #[test]
    fn phases_order_independent_packages() {
        let mut registry = Registry::new();
        registry.insert("post".to_owned(), record("post", Phase::PostEetEnd));
        registry.insert("base".to_owned(), record("base", Phase::Eet));
        let manifest = Manifest {
            schema: 1,
            game: GameTarget {
                target: "eet".to_owned(),
                version: None,
                fingerprint: None,
            },
            mods: vec![
                RequestedMod::Package("post".to_owned()),
                RequestedMod::Package("base".to_owned()),
            ],
        };
        assert_eq!(
            resolve(&manifest, &registry, "test", test_toolchain())
                .unwrap()
                .install_order,
            vec!["base", "post"]
        );
    }

    #[test]
    fn eet_fixture_has_a_deterministic_complete_graph() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let manifest_text =
            std::fs::read_to_string(root.join("examples/eet-balanced/modpack.yaml")).unwrap();
        let manifest: Manifest = serde_yaml::from_str(&manifest_text).unwrap();
        let registry = iepm_registry::load(&root.join("registry")).unwrap();
        let lockfile = resolve(&manifest, &registry, "fixture", test_toolchain()).unwrap();
        assert_eq!(lockfile.packages.len(), 10);
        assert_eq!(
            lockfile.install_order,
            vec![
                "dlc-merger",
                "eet",
                "ascension",
                "eeex",
                "bubbs-spell-menu",
                "hidden-gameplay-options",
                "infinity-ui-plus-plus",
                "tweaks-anthology",
                "eet-end",
                "tactics-remix",
            ]
        );
    }

    #[test]
    fn chooses_the_highest_release_matching_a_manifest_requirement() {
        let mut registry = Registry::new();
        let mut package = record("package", Phase::Eet);
        package.releases = vec![release("1.4", Phase::Eet), release("2.0", Phase::Eet)];
        registry.insert("package".to_owned(), package);
        let manifest = Manifest {
            schema: 1,
            game: GameTarget {
                target: "eet".to_owned(),
                version: None,
                fingerprint: None,
            },
            mods: vec![RequestedMod::Selection {
                package: "package".to_owned(),
                version: Some(">=1.0, <2.0".to_owned()),
                components: vec![],
            }],
        };

        let lockfile = resolve(&manifest, &registry, "test", test_toolchain()).unwrap();
        assert_eq!(lockfile.packages[0].version, "1.4");
    }

    #[test]
    fn dependency_requirements_choose_a_compatible_transitive_release() {
        let mut registry = Registry::new();
        let mut package = record("package", Phase::Eet);
        package.releases[0].dependencies = vec![Dependency {
            package: "support".to_owned(),
            version: Some(">=2.0".to_owned()),
            components: vec![],
        }];
        let mut support = record("support", Phase::Eet);
        support.releases = vec![release("1.0", Phase::Eet), release("2.1", Phase::Eet)];
        registry.insert("package".to_owned(), package);
        registry.insert("support".to_owned(), support);
        let manifest = Manifest {
            schema: 1,
            game: GameTarget {
                target: "eet".to_owned(),
                version: None,
                fingerprint: None,
            },
            mods: vec![RequestedMod::Package("package".to_owned())],
        };

        let lockfile = resolve(&manifest, &registry, "test", test_toolchain()).unwrap();
        assert!(
            lockfile
                .packages
                .iter()
                .any(|package| package.package == "support" && package.version == "2.1")
        );
    }

    #[test]
    fn rejects_an_invalid_version_requirement() {
        let mut registry = Registry::new();
        registry.insert("package".to_owned(), record("package", Phase::Eet));
        let manifest = Manifest {
            schema: 1,
            game: GameTarget {
                target: "eet".to_owned(),
                version: None,
                fingerprint: None,
            },
            mods: vec![RequestedMod::Selection {
                package: "package".to_owned(),
                version: Some("not-a-requirement".to_owned()),
                components: vec![],
            }],
        };

        assert_eq!(
            resolve(&manifest, &registry, "test", test_toolchain()).unwrap_err(),
            ResolveError::InvalidVersionRequirement {
                package: "package".to_owned(),
                requirement: "not-a-requirement".to_owned(),
            }
        );
    }

    #[test]
    fn lockfile_preserves_artifact_edges_and_toolchain_identity() {
        let mut registry = Registry::new();
        let mut package = record("package", Phase::Eet);
        package.releases[0].artifact = Some(Artifact {
            url: "https://example.invalid/package-1.0.zip".to_owned(),
            sha256: "a".repeat(64),
            format: Default::default(),
            platforms: vec![],
        });
        package.releases[0].dependencies = vec![
            Dependency {
                package: "support".to_owned(),
                version: Some("=1.0".to_owned()),
                components: vec!["required-component".to_owned()],
            },
            Dependency {
                package: "unconstrained".to_owned(),
                version: None,
                components: vec![],
            },
        ];
        let mut support = record("support", Phase::Eet);
        support.releases[0].components = vec![Component {
            id: "required-component".to_owned(),
            provides: vec![],
        }];
        registry.insert("package".to_owned(), package);
        registry.insert("support".to_owned(), support);
        registry.insert(
            "unconstrained".to_owned(),
            record("unconstrained", Phase::Eet),
        );
        let manifest = Manifest {
            schema: 1,
            game: GameTarget {
                target: "eet".to_owned(),
                version: Some("2.6.6".to_owned()),
                fingerprint: Some("sha256:game-fixture".to_owned()),
            },
            mods: vec![RequestedMod::Package("package".to_owned())],
        };

        let lockfile = resolve(&manifest, &registry, "registry-fixture", test_toolchain()).unwrap();
        let package = lockfile
            .packages
            .iter()
            .find(|package| package.package == "package")
            .unwrap();
        assert_eq!(lockfile.registry_revision, "registry-fixture");
        assert_eq!(lockfile.toolchain.iepm, "test-manager");
        assert_eq!(lockfile.toolchain.weidu.as_deref(), Some("24600"));
        assert_eq!(
            lockfile.game.fingerprint.as_deref(),
            Some("sha256:game-fixture")
        );
        assert_eq!(
            package
                .artifact
                .as_ref()
                .map(|artifact| artifact.url.as_str()),
            Some("https://example.invalid/package-1.0.zip")
        );
        assert_eq!(package.dependencies[0].package, "support");
        assert_eq!(package.dependencies[0].components, ["required-component"]);

        let serialized = serde_json::to_value(lockfile).unwrap();
        assert_eq!(serialized["toolchain"]["iepm"], "test-manager");
        let serialized_package = serialized["packages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|package| package["package"] == "package")
            .unwrap();
        assert!(
            serialized_package["dependencies"][1]
                .get("version")
                .is_none()
        );
        assert!(
            serialized_package["dependencies"][1]
                .get("components")
                .is_none()
        );
    }
}

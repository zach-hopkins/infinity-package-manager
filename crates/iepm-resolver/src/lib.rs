use iepm_core::{
    Component, LockedPackage, Lockfile, Manifest, PackageRecord, Phase, Registry, Release,
};
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

pub fn resolve(
    manifest: &Manifest,
    registry: &Registry,
    registry_revision: &str,
) -> Result<Lockfile, ResolveError> {
    if manifest.schema != 1 {
        return Err(ResolveError::Schema(manifest.schema));
    }
    let mut requested = BTreeMap::<String, Vec<String>>::new();
    for requested_mod in &manifest.mods {
        requested.insert(
            requested_mod.package().to_owned(),
            requested_mod.components().to_vec(),
        );
    }

    let mut cursor = 0;
    while cursor < requested.len() {
        let package = requested
            .keys()
            .nth(cursor)
            .expect("cursor is in range")
            .clone();
        let record = registry
            .get(&package)
            .ok_or_else(|| ResolveError::MissingPackage(package.clone()))?;
        let release = compatible_release(record, &manifest.game.target).ok_or_else(|| {
            ResolveError::Incompatible {
                package: package.clone(),
                game: manifest.game.target.clone(),
            }
        })?;
        for dependency in &release.dependencies {
            requested.entry(dependency.package.clone()).or_default();
        }
        cursor += 1;
    }

    let mut selections = Vec::new();
    for (package, components) in &requested {
        let record = registry
            .get(package)
            .expect("all requested packages were checked");
        let release =
            compatible_release(record, &manifest.game.target).expect("compatibility checked");
        validate_components(package, release, components)?;
        selections.push(Selection {
            package,
            release,
            components: components.clone(),
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
            artifact_sha256: selection
                .release
                .artifact
                .as_ref()
                .map(|artifact| artifact.sha256.clone()),
            components: selection.components,
            provenance: selection.release.provenance,
        })
        .collect();

    Ok(Lockfile {
        schema: 1,
        game: manifest.game.clone(),
        registry_revision: registry_revision.to_owned(),
        packages: selected,
        install_order: order,
    })
}

fn compatible_release<'a>(record: &'a PackageRecord, game: &str) -> Option<&'a Release> {
    record.releases.iter().find(|release| {
        release
            .compatibility
            .games
            .iter()
            .any(|candidate| candidate == game)
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
        Compatibility, GameTarget, Install, Manifest, PackageRecord, Provenance, RequestedMod,
    };
    use std::path::PathBuf;

    fn record(id: &str, phase: Phase) -> PackageRecord {
        PackageRecord {
            schema: 1,
            package: id.to_owned(),
            releases: vec![Release {
                version: "1".to_owned(),
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
            }],
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
            resolve(&manifest, &registry, "test").unwrap().install_order,
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
        let lockfile = resolve(&manifest, &registry, "fixture").unwrap();
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
}

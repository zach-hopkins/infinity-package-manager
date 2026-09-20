use iepm_core::{
    Artifact, Capability, Dependency, ExecutionNode, ExecutionReadiness, GameEnvironment,
    GameFingerprint, LockedComponent, LockedPackage, Lockfile, Manifest, PackageRecord, Provenance,
    Registry, RelationshipKind, Release, Toolchain,
};
use semver::{Version, VersionReq};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ResolveError {
    #[error("manifest schema {0} is unsupported")]
    Schema(u32),
    #[error("manifest has no game environments")]
    NoEnvironments,
    #[error("package `{package}` references unknown environment `{environment}`")]
    MissingEnvironment {
        package: String,
        environment: String,
    },
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
    #[error("package `{package}` has invalid SemVer requirement `{requirement}`")]
    InvalidVersionRequirement {
        package: String,
        requirement: String,
    },
    #[error("package `{package}` has invalid SemVer projection `{version}`")]
    InvalidReleaseVersion { package: String, version: String },
    #[error("package `{package}` requests unknown component `{component}`")]
    MissingComponent { package: String, component: String },
    #[error("package `{package}` has conflicting values for installer input `{input}`")]
    ConflictingInstallerInput { package: String, input: String },
    #[error("package `{package}` requires installer input `{input}`")]
    MissingInstallerInput { package: String, input: String },
    #[error("package `{package}` has a non-portable installer input `{input}`")]
    NonPortableInstallerInput { package: String, input: String },
    #[error("exclusive capability `{capability}` is provided by both `{first}` and `{second}`")]
    Capability {
        capability: String,
        first: String,
        second: String,
    },
    #[error("`{first}` conflicts with `{second}`")]
    Conflict { first: String, second: String },
    #[error("ordering `{before}` before `{after}` contradicts the declared phases")]
    PhaseViolation { before: String, after: String },
    #[error("ordering graph contains a cycle")]
    Cycle,
    #[error("resolution exceeded the candidate-search limit")]
    SearchLimit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    environment: String,
    package: String,
}

impl Key {
    fn node_id(&self) -> String {
        format!("{}::{}", self.environment, self.package)
    }
}

#[derive(Debug, Clone, Default)]
struct Request {
    requirements: BTreeSet<String>,
    components: BTreeSet<String>,
    language: Option<String>,
    installer_inputs: BTreeMap<String, String>,
}

impl Request {
    fn merge_dependency(&mut self, dependency: &Dependency) -> bool {
        let before = (self.requirements.len(), self.components.len());
        if let Some(version) = &dependency.version {
            self.requirements.insert(version.clone());
        }
        self.components
            .extend(dependency.components.iter().cloned());
        before != (self.requirements.len(), self.components.len())
    }

    fn merge_root(
        &mut self,
        version: Option<&str>,
        components: &[String],
        language: Option<&str>,
        inputs: &BTreeMap<String, String>,
        package: &str,
    ) -> Result<(), ResolveError> {
        if let Some(version) = version {
            self.requirements.insert(version.to_owned());
        }
        self.components.extend(components.iter().cloned());
        if self.language.is_none() {
            self.language = language.map(str::to_owned);
        }
        for (name, value) in inputs {
            if looks_like_absolute_path(value) {
                return Err(ResolveError::NonPortableInstallerInput {
                    package: package.to_owned(),
                    input: name.clone(),
                });
            }
            if let Some(existing) = self.installer_inputs.insert(name.clone(), value.clone())
                && existing != *value
            {
                return Err(ResolveError::ConflictingInstallerInput {
                    package: package.to_owned(),
                    input: name.clone(),
                });
            }
        }
        Ok(())
    }
}

fn looks_like_absolute_path(value: &str) -> bool {
    let value = value.trim();
    value.starts_with('/')
        || value.starts_with("\\\\")
        || (value.len() >= 3
            && value.as_bytes()[0].is_ascii_alphabetic()
            && value.as_bytes()[1] == b':'
            && matches!(value.as_bytes()[2], b'\\' | b'/'))
}

#[derive(Clone)]
struct Solution {
    requests: BTreeMap<Key, Request>,
    selected: BTreeMap<Key, usize>,
}

/// Resolve human intent into an environment-aware execution plan. Schema-1
/// manifests remain readable, but all produced lockfiles use schema 2.
pub fn resolve(
    manifest: &Manifest,
    registry: &Registry,
    registry_revision: &str,
    toolchain: Toolchain,
) -> Result<Lockfile, ResolveError> {
    if !matches!(manifest.schema, 1 | 2) {
        return Err(ResolveError::Schema(manifest.schema));
    }
    let (environments, migrated_v1) = normalized_environments(manifest)?;
    let default_environment = environments
        .keys()
        .next()
        .expect("normalized environments are non-empty")
        .clone();
    let mut roots = BTreeMap::<Key, Request>::new();
    for requested in &manifest.mods {
        let environment = requested
            .environment()
            .unwrap_or(&default_environment)
            .to_owned();
        if !environments.contains_key(&environment) {
            return Err(ResolveError::MissingEnvironment {
                package: requested.package().to_owned(),
                environment,
            });
        }
        let key = Key {
            environment,
            package: canonical_package(registry, requested.package())?,
        };
        roots.entry(key).or_default().merge_root(
            requested.version(),
            requested.components(),
            requested.language(),
            requested.installer_inputs(),
            requested.package(),
        )?;
    }

    let solution = solve(&roots, BTreeMap::new(), registry, &environments, 0)?;
    validate_capabilities(&solution, registry)?;
    validate_conflicts(&solution, registry, &environments)?;
    let execution = order(&solution, registry, &environments)?;
    let mut warnings = BTreeSet::new();
    let mut blocking_reasons = BTreeSet::new();
    if migrated_v1 {
        warnings.insert("migrated schema-1 manifest; rewrite it as schema 2".to_owned());
    }

    let packages = solution
        .selected
        .iter()
        .map(|(key, index)| {
            let request = &solution.requests[key];
            let release = release(registry, key, *index);
            validate_installer_inputs(&key.package, release, request)?;
            let selected_components = selected_components(&key.package, release, request)?;
            let components = selected_components
                .iter()
                .map(|component| {
                    if component.weidu.is_none() {
                        warnings.insert(format!(
                            "{} has no WeiDU selector for component {}",
                            key.node_id(),
                            component.id
                        ));
                        blocking_reasons.insert(format!(
                            "{} selects component {} without a WeiDU selector",
                            key.node_id(),
                            component.id
                        ));
                    }
                    Ok(LockedComponent {
                        id: component.id.clone(),
                        weidu: component.weidu.clone(),
                    })
                })
                .collect::<Result<Vec<_>, ResolveError>>()?;
            let artifact = select_artifact(release, &environments[&key.environment]);
            if artifact.is_none() {
                warnings.insert(format!("{} has no verified artifact", key.node_id()));
                blocking_reasons.insert(format!("{} has no verified artifact", key.node_id()));
            }
            if selected_components.is_empty() && release.installers.is_empty() {
                blocking_reasons.insert(format!(
                    "{} has no installer specification for A5 execution",
                    key.node_id()
                ));
            }
            if release.provenance == Provenance::Unverified {
                warnings.insert(format!("{} is unverified", key.node_id()));
            }
            Ok(LockedPackage {
                package: key.package.clone(),
                release_id: release.id().to_owned(),
                version: release.version.clone(),
                environment: key.environment.clone(),
                phase: release.install.phase,
                artifact,
                materialization: release.materialization.clone(),
                components,
                language: request
                    .language
                    .clone()
                    .or_else(|| environments[&key.environment].language.clone()),
                installer_inputs: request.installer_inputs.clone(),
                dependencies: required_dependencies(release, &environments[&key.environment]),
                provenance: release.provenance,
            })
        })
        .collect::<Result<Vec<_>, ResolveError>>()?;

    for (name, environment) in &environments {
        if environment.fingerprint.is_none() {
            warnings.insert(format!("environment `{name}` has no fingerprint"));
        }
    }
    Ok(Lockfile {
        schema: 2,
        environments,
        registry_revision: registry_revision.to_owned(),
        toolchain,
        packages,
        execution,
        execution_readiness: if blocking_reasons.is_empty() {
            ExecutionReadiness::Executable
        } else {
            ExecutionReadiness::AnalysisOnly
        },
        blocking_reasons: blocking_reasons.into_iter().collect(),
        warnings: warnings.into_iter().collect(),
    })
}

fn normalized_environments(
    manifest: &Manifest,
) -> Result<(BTreeMap<String, GameEnvironment>, bool), ResolveError> {
    if !manifest.environments.is_empty() {
        return Ok((manifest.environments.clone(), false));
    }
    let game = manifest.game.as_ref().ok_or(ResolveError::NoEnvironments)?;
    let mut environments = BTreeMap::new();
    environments.insert(
        "target".to_owned(),
        GameEnvironment {
            target: game.target.clone(),
            version: game.version.clone(),
            fingerprint: game.fingerprint.as_ref().map(|value| GameFingerprint {
                profile: "legacy-string-v1".to_owned(),
                value: value.clone(),
            }),
            platform: None,
            store: None,
            language: None,
        },
    );
    Ok((environments, true))
}

fn solve(
    roots: &BTreeMap<Key, Request>,
    selected: BTreeMap<Key, usize>,
    registry: &Registry,
    environments: &BTreeMap<String, GameEnvironment>,
    depth: usize,
) -> Result<Solution, ResolveError> {
    if depth > 4096 {
        return Err(ResolveError::SearchLimit);
    }
    let requests = expand_requests(roots, &selected, registry, environments)?;
    let mut selected = selected;
    selected.retain(|key, _| requests.contains_key(key));

    if let Some(key) = selected.iter().find_map(|(key, index)| {
        (!release_matches(
            release(registry, key, *index),
            &requests[key],
            &environments[&key.environment].target,
            &key.package,
        )
        .unwrap_or(false))
        .then(|| key.clone())
    }) {
        selected.remove(&key);
        return solve(roots, selected, registry, environments, depth + 1);
    }

    let Some(key) = requests
        .keys()
        .find(|key| !selected.contains_key(*key))
        .cloned()
    else {
        return Ok(Solution { requests, selected });
    };
    let candidates = candidates(
        registry
            .get(&key.package)
            .ok_or_else(|| ResolveError::MissingPackage(key.package.clone()))?,
        &requests[&key],
        &environments[&key.environment].target,
    )?;
    let mut last_error = None;
    for index in candidates {
        let mut next = selected.clone();
        next.insert(key.clone(), index);
        match solve(roots, next, registry, environments, depth + 1) {
            Ok(solution) => return Ok(solution),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.unwrap_or_else(|| {
        no_match(
            registry.get(&key.package).expect("record checked"),
            &requests[&key],
            &environments[&key.environment].target,
        )
    }))
}

fn expand_requests(
    roots: &BTreeMap<Key, Request>,
    selected: &BTreeMap<Key, usize>,
    registry: &Registry,
    environments: &BTreeMap<String, GameEnvironment>,
) -> Result<BTreeMap<Key, Request>, ResolveError> {
    let mut requests = roots.clone();
    let mut pending = roots.keys().cloned().collect::<VecDeque<_>>();
    let mut visited = BTreeSet::new();
    while let Some(key) = pending.pop_front() {
        if !visited.insert(key.clone()) {
            continue;
        }
        let Some(index) = selected.get(&key) else {
            continue;
        };
        let record = registry
            .get(&key.package)
            .ok_or_else(|| ResolveError::MissingPackage(key.package.clone()))?;
        let release = &record.releases[*index];
        for dependency in required_dependencies(release, &environments[&key.environment]) {
            let dependency_key = Key {
                environment: dependency
                    .environment
                    .clone()
                    .unwrap_or_else(|| key.environment.clone()),
                package: canonical_package(registry, &dependency.package)?,
            };
            if !environments.contains_key(&dependency_key.environment) {
                return Err(ResolveError::MissingEnvironment {
                    package: dependency_key.package.clone(),
                    environment: dependency_key.environment,
                });
            }
            let changed = requests
                .entry(dependency_key.clone())
                .or_default()
                .merge_dependency(&dependency);
            if changed || selected.contains_key(&dependency_key) {
                pending.push_back(dependency_key);
            }
        }
    }
    Ok(requests)
}

fn required_dependencies(release: &Release, environment: &GameEnvironment) -> Vec<Dependency> {
    release
        .dependencies
        .iter()
        .cloned()
        .chain(
            release
                .relationships
                .iter()
                .filter(|relationship| {
                    relationship.kind == RelationshipKind::Requires
                        && relationship.when.matches_game(&environment.target)
                })
                .map(|relationship| Dependency {
                    package: relationship.package.clone(),
                    environment: relationship.environment.clone(),
                    version: relationship.version.clone(),
                    components: relationship.components.clone(),
                }),
        )
        .collect()
}

fn release_matches(
    release: &Release,
    request: &Request,
    game: &str,
    package: &str,
) -> Result<bool, ResolveError> {
    if !release
        .compatibility
        .games
        .iter()
        .any(|candidate| candidate == game)
    {
        return Ok(false);
    }
    request
        .requirements
        .iter()
        .map(|requirement| requirement_matches(release, requirement, package))
        .collect::<Result<Vec<_>, _>>()
        .map(|matches| matches.into_iter().all(|value| value))
}

fn candidates(
    record: &PackageRecord,
    request: &Request,
    game: &str,
) -> Result<Vec<usize>, ResolveError> {
    let mut result = record
        .releases
        .iter()
        .enumerate()
        .filter_map(|(index, release)| {
            match release_matches(release, request, game, &record.package) {
                Ok(true) => Some(Ok(index)),
                Ok(false) => None,
                Err(error) => Some(Err(error)),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    result.sort_by(|left, right| {
        release_sort_key(&record.releases[*right]).cmp(&release_sort_key(&record.releases[*left]))
    });
    if result.is_empty() {
        return Err(no_match(record, request, game));
    }
    Ok(result)
}

fn release_sort_key(release: &Release) -> (u8, Version, String) {
    match release_semver(release, "") {
        Ok(Some(version)) => (1, version, release.id().to_owned()),
        _ => (0, Version::new(0, 0, 0), release.id().to_owned()),
    }
}

fn requirement_matches(
    release: &Release,
    requirement: &str,
    package: &str,
) -> Result<bool, ResolveError> {
    let trimmed = requirement.trim();
    if let Some(exact) = trimmed.strip_prefix("id:") {
        return Ok(release.id() == exact);
    }
    if let Some(exact) = trimmed.strip_prefix('=') {
        return Ok(release.id() == exact || release.version == exact);
    }
    if !looks_like_range(trimmed) {
        return Ok(release.id() == trimmed || release.version == trimmed);
    }
    let requirement =
        VersionReq::parse(trimmed).map_err(|_| ResolveError::InvalidVersionRequirement {
            package: package.to_owned(),
            requirement: trimmed.to_owned(),
        })?;
    Ok(release_semver(release, package)?.is_some_and(|version| requirement.matches(&version)))
}

fn looks_like_range(value: &str) -> bool {
    value.contains(['>', '<', '^', '~', '*', ','])
}

fn release_semver(release: &Release, package: &str) -> Result<Option<Version>, ResolveError> {
    let raw = release.semver.as_deref().unwrap_or(&release.version).trim();
    let normalized = raw.strip_prefix('v').unwrap_or(raw);
    let suffix_index = normalized.find(['-', '+']).unwrap_or(normalized.len());
    let (core, suffix) = normalized.split_at(suffix_index);
    let parts = core.split('.').count();
    let normalized = match parts {
        1 => format!("{core}.0.0{suffix}"),
        2 => format!("{core}.0{suffix}"),
        _ => normalized.to_owned(),
    };
    match Version::parse(&normalized) {
        Ok(version) => Ok(Some(version)),
        Err(_) if release.semver.is_none() => Ok(None),
        Err(_) => Err(ResolveError::InvalidReleaseVersion {
            package: package.to_owned(),
            version: raw.to_owned(),
        }),
    }
}

fn no_match(record: &PackageRecord, request: &Request, game: &str) -> ResolveError {
    let compatible = record.releases.iter().any(|release| {
        release
            .compatibility
            .games
            .iter()
            .any(|candidate| candidate == game)
    });
    if compatible {
        ResolveError::NoMatchingVersion {
            package: record.package.clone(),
            game: game.to_owned(),
            requirements: if request.requirements.is_empty() {
                "an exact opaque release or SemVer range".to_owned()
            } else {
                request
                    .requirements
                    .iter()
                    .cloned()
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
}

fn release<'a>(registry: &'a Registry, key: &Key, index: usize) -> &'a Release {
    &registry[&key.package].releases[index]
}

fn canonical_package(registry: &Registry, requested: &str) -> Result<String, ResolveError> {
    if registry.contains_key(requested) {
        return Ok(requested.to_owned());
    }
    registry
        .iter()
        .find_map(|(canonical, record)| {
            record
                .aliases
                .iter()
                .any(|alias| alias == requested)
                .then(|| canonical.clone())
        })
        .ok_or_else(|| ResolveError::MissingPackage(requested.to_owned()))
}

fn selected_components<'a>(
    package: &str,
    release: &'a Release,
    request: &Request,
) -> Result<Vec<&'a iepm_core::Component>, ResolveError> {
    for component in &request.components {
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
    Ok(release
        .components
        .iter()
        .filter(|component| {
            request.components.contains(&component.id)
                || (request.components.is_empty() && component.default_selected)
        })
        .collect())
}

fn validate_installer_inputs(
    package: &str,
    release: &Release,
    request: &Request,
) -> Result<(), ResolveError> {
    for installer in &release.installers {
        for input in &installer.inputs {
            if input.required && !request.installer_inputs.contains_key(&input.name) {
                return Err(ResolveError::MissingInstallerInput {
                    package: package.to_owned(),
                    input: input.name.clone(),
                });
            }
        }
    }
    Ok(())
}

fn validate_capabilities(solution: &Solution, registry: &Registry) -> Result<(), ResolveError> {
    let mut exclusive = BTreeMap::<String, String>::new();
    for (key, index) in &solution.selected {
        let release = release(registry, key, *index);
        for component in selected_components(&key.package, release, &solution.requests[key])? {
            for Capability {
                name,
                exclusive: is_exclusive,
            } in &component.provides
            {
                if *is_exclusive {
                    let provider = format!("{}:{}", key.node_id(), component.id);
                    if let Some(first) = exclusive.insert(name.clone(), provider.clone()) {
                        return Err(ResolveError::Capability {
                            capability: name.clone(),
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

fn validate_conflicts(
    solution: &Solution,
    registry: &Registry,
    environments: &BTreeMap<String, GameEnvironment>,
) -> Result<(), ResolveError> {
    for (key, index) in &solution.selected {
        let selected_release = release(registry, key, *index);
        for relationship in &selected_release.relationships {
            if relationship.kind != RelationshipKind::Conflicts
                || !relationship
                    .when
                    .matches_game(&environments[&key.environment].target)
            {
                continue;
            }
            let other_key = Key {
                environment: key.environment.clone(),
                package: canonical_package(registry, &relationship.package)?,
            };
            let Some(other_index) = solution.selected.get(&other_key) else {
                continue;
            };
            let other = release(registry, &other_key, *other_index);
            let other_components =
                selected_components(&other_key.package, other, &solution.requests[&other_key])?;
            if relationship.components.is_empty()
                || other_components
                    .iter()
                    .any(|component| relationship.components.contains(&component.id))
            {
                return Err(ResolveError::Conflict {
                    first: key.node_id(),
                    second: other_key.node_id(),
                });
            }
        }
    }
    Ok(())
}

fn order(
    solution: &Solution,
    registry: &Registry,
    environments: &BTreeMap<String, GameEnvironment>,
) -> Result<Vec<ExecutionNode>, ResolveError> {
    let mut edges = BTreeMap::<Key, BTreeSet<Key>>::new();
    let mut indegree = solution
        .selected
        .keys()
        .cloned()
        .map(|key| (key, 0_usize))
        .collect::<BTreeMap<_, _>>();
    let phases = solution
        .selected
        .iter()
        .map(|(key, index)| (key.clone(), release(registry, key, *index).install.phase))
        .collect::<BTreeMap<_, _>>();
    let selected = solution.selected.keys().cloned().collect::<BTreeSet<_>>();

    let mut add_edge = |before: &Key, after: &Key| -> Result<(), ResolveError> {
        if before.environment == after.environment && phases[before] > phases[after] {
            return Err(ResolveError::PhaseViolation {
                before: before.node_id(),
                after: after.node_id(),
            });
        }
        if edges
            .entry(before.clone())
            .or_default()
            .insert(after.clone())
        {
            *indegree.get_mut(after).expect("selected edge target") += 1;
        }
        Ok(())
    };

    for (key, index) in &solution.selected {
        let release = release(registry, key, *index);
        for dependency in required_dependencies(release, &environments[&key.environment]) {
            let dependency_key = Key {
                environment: dependency
                    .environment
                    .clone()
                    .unwrap_or_else(|| key.environment.clone()),
                package: canonical_package(registry, &dependency.package)?,
            };
            if selected.contains(&dependency_key) {
                add_edge(&dependency_key, key)?;
            }
        }
        for target in &release.install.before {
            let target = Key {
                environment: key.environment.clone(),
                package: canonical_package(registry, target)?,
            };
            if selected.contains(&target) {
                add_edge(key, &target)?;
            }
        }
        for source in &release.install.after {
            let source = Key {
                environment: key.environment.clone(),
                package: canonical_package(registry, source)?,
            };
            if selected.contains(&source) {
                add_edge(&source, key)?;
            }
        }
        for relationship in &release.relationships {
            if !relationship
                .when
                .matches_game(&environments[&key.environment].target)
            {
                continue;
            }
            let other = Key {
                environment: key.environment.clone(),
                package: canonical_package(registry, &relationship.package)?,
            };
            if !selected.contains(&other) {
                continue;
            }
            match relationship.kind {
                RelationshipKind::Before => add_edge(key, &other)?,
                RelationshipKind::After => add_edge(&other, key)?,
                _ => {}
            }
        }
    }
    // Phases are hard barriers inside an environment. This turns a confusing
    // priority convention into an actionable invalid-plan diagnostic.
    let keys = selected.iter().cloned().collect::<Vec<_>>();
    for before in &keys {
        for after in &keys {
            if before.environment == after.environment && phases[before] < phases[after] {
                add_edge(before, after)?;
            }
        }
    }

    let mut ready = indegree
        .iter()
        .filter_map(|(key, degree)| (*degree == 0).then_some(key.clone()))
        .collect::<Vec<_>>();
    let mut output = Vec::new();
    while !ready.is_empty() {
        ready.sort();
        let current = ready.remove(0);
        let predecessors = edges
            .iter()
            .filter_map(|(source, targets)| targets.contains(&current).then_some(source.node_id()))
            .collect();
        output.push(ExecutionNode {
            id: current.node_id(),
            package: current.package.clone(),
            environment: current.environment.clone(),
            phase: phases[&current],
            predecessors,
        });
        for target in edges.get(&current).into_iter().flatten() {
            let degree = indegree.get_mut(target).expect("selected target");
            *degree -= 1;
            if *degree == 0 {
                ready.push(target.clone());
            }
        }
    }
    if output.len() != solution.selected.len() {
        return Err(ResolveError::Cycle);
    }
    Ok(output)
}

fn select_artifact(release: &Release, environment: &GameEnvironment) -> Option<Artifact> {
    let mut artifacts = release
        .artifacts()
        .filter(|artifact| {
            environment.platform.is_none()
                || artifact.platforms.is_empty()
                || artifact
                    .platforms
                    .contains(&environment.platform.expect("checked"))
        })
        .cloned()
        .collect::<Vec<_>>();
    artifacts.sort_by(|left, right| left.sha256.cmp(&right.sha256));
    artifacts.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use iepm_core::{
        Artifact, Compatibility, Install, PackageRecord, Phase, Relationship,
        RelationshipCondition, RequestedMod,
    };

    fn env() -> BTreeMap<String, GameEnvironment> {
        BTreeMap::from([(
            "target".to_owned(),
            GameEnvironment {
                target: "eet".to_owned(),
                version: Some("2.6.6".to_owned()),
                fingerprint: Some(GameFingerprint {
                    profile: "core-v1".to_owned(),
                    value: "fixture".to_owned(),
                }),
                platform: None,
                store: None,
                language: Some("English".to_owned()),
            },
        )])
    }

    fn manifest(mods: Vec<RequestedMod>) -> Manifest {
        Manifest {
            schema: 2,
            game: None,
            environments: env(),
            mods,
        }
    }

    fn release(version: &str, phase: Phase) -> Release {
        Release {
            version: version.to_owned(),
            release_id: None,
            semver: None,
            artifact: None,
            artifacts: vec![],
            materialization: Default::default(),
            compatibility: Compatibility {
                games: vec!["eet".to_owned()],
            },
            install: Install {
                phase,
                before: vec![],
                after: vec![],
            },
            provenance: Provenance::Verified,
            claims: vec![],
            dependencies: vec![],
            relationships: vec![],
            components: vec![],
            installers: vec![],
        }
    }

    fn record(id: &str, releases: Vec<Release>) -> PackageRecord {
        PackageRecord {
            schema: 2,
            package: id.to_owned(),
            aliases: vec![],
            lineage: vec![],
            releases,
        }
    }

    fn toolchain() -> Toolchain {
        Toolchain {
            iepm: "test".to_owned(),
            weidu: Some("24600".to_owned()),
        }
    }

    #[test]
    fn emits_environment_aware_replayable_lockfile() {
        let mut registry = Registry::new();
        let mut package = release("Beta 5", Phase::Eet);
        package.release_id = Some("upstream-2026-09-20".to_owned());
        package.components.push(iepm_core::Component {
            id: "smart-mages".to_owned(),
            default_selected: false,
            weidu: Some(iepm_core::WeiDUComponent {
                tp2: "mod/setup-mod.tp2".to_owned(),
                label: Some("smart_mages".to_owned()),
                number: None,
                subcomponent: None,
            }),
            provides: vec![],
        });
        registry.insert("package".to_owned(), record("package", vec![package]));
        let lock = resolve(
            &manifest(vec![RequestedMod::Selection {
                package: "package".to_owned(),
                version: Some("id:upstream-2026-09-20".to_owned()),
                components: vec!["smart-mages".to_owned()],
                environment: Some("target".to_owned()),
                language: Some("French".to_owned()),
                installer_inputs: BTreeMap::from([("source-root".to_owned(), "bgee".to_owned())]),
            }]),
            &registry,
            "registry-fixture",
            toolchain(),
        )
        .unwrap();
        assert_eq!(lock.schema, 2);
        assert_eq!(lock.execution[0].id, "target::package");
        assert_eq!(lock.packages[0].release_id, "upstream-2026-09-20");
        assert_eq!(lock.packages[0].language.as_deref(), Some("French"));
        assert_eq!(
            lock.packages[0].components[0]
                .weidu
                .as_ref()
                .unwrap()
                .label
                .as_deref(),
            Some("smart_mages")
        );
        assert_eq!(lock.execution_readiness, ExecutionReadiness::AnalysisOnly);
        assert!(
            lock.blocking_reasons
                .iter()
                .any(|reason| reason.contains("no verified artifact"))
        );
    }

    #[test]
    fn opaque_versions_are_exact_and_semver_ranges_are_opt_in() {
        let mut registry = Registry::new();
        let mut newer = release("Beta 5", Phase::Eet);
        newer.release_id = Some("beta-five".to_owned());
        let mut stable = release("v10", Phase::Eet);
        stable.release_id = Some("release-ten".to_owned());
        stable.semver = Some("10.0.0".to_owned());
        registry.insert("package".to_owned(), record("package", vec![newer, stable]));
        let exact = resolve(
            &manifest(vec![RequestedMod::Selection {
                package: "package".to_owned(),
                version: Some("Beta 5".to_owned()),
                components: vec![],
                environment: None,
                language: None,
                installer_inputs: BTreeMap::new(),
            }]),
            &registry,
            "test",
            toolchain(),
        )
        .unwrap();
        assert_eq!(exact.packages[0].release_id, "beta-five");
        let ranged = resolve(
            &manifest(vec![RequestedMod::Selection {
                package: "package".to_owned(),
                version: Some(">=9, <11".to_owned()),
                components: vec![],
                environment: None,
                language: None,
                installer_inputs: BTreeMap::new(),
            }]),
            &registry,
            "test",
            toolchain(),
        )
        .unwrap();
        assert_eq!(ranged.packages[0].release_id, "release-ten");
    }

    #[test]
    fn requires_declared_installer_inputs() {
        let mut registry = Registry::new();
        let mut package = release("1", Phase::Eet);
        package.installers.push(iepm_core::Installer {
            tp2: "setup-package.tp2".to_owned(),
            inputs: vec![iepm_core::InstallerInput {
                name: "source-environment".to_owned(),
                required: true,
            }],
        });
        registry.insert("package".to_owned(), record("package", vec![package]));
        assert!(matches!(
            resolve(
                &manifest(vec![RequestedMod::Package("package".to_owned())]),
                &registry,
                "test",
                toolchain()
            ),
            Err(ResolveError::MissingInstallerInput { .. })
        ));
    }

    #[test]
    fn rejects_machine_specific_installer_paths_from_portable_lockfiles() {
        let mut registry = Registry::new();
        registry.insert(
            "package".to_owned(),
            record("package", vec![release("1", Phase::Eet)]),
        );
        assert!(matches!(
            resolve(
                &manifest(vec![RequestedMod::Selection {
                    package: "package".to_owned(),
                    version: None,
                    components: vec![],
                    environment: None,
                    language: None,
                    installer_inputs: BTreeMap::from([(
                        "source-path".to_owned(),
                        "C:\\Games\\BGEE".to_owned()
                    )]),
                }]),
                &registry,
                "test",
                toolchain()
            ),
            Err(ResolveError::NonPortableInstallerInput { .. })
        ));
    }

    #[test]
    fn backtracks_to_a_compatible_release_without_stale_dependencies() {
        let mut registry = Registry::new();
        let mut latest = release("2.0", Phase::Eet);
        latest.dependencies.push(Dependency {
            package: "support".to_owned(),
            environment: None,
            version: Some(">=2".to_owned()),
            components: vec![],
        });
        let mut old = release("1.0", Phase::Eet);
        old.dependencies.push(Dependency {
            package: "support".to_owned(),
            environment: None,
            version: Some("<2".to_owned()),
            components: vec![],
        });
        let mut peer = release("1.0", Phase::Eet);
        peer.dependencies.push(Dependency {
            package: "support".to_owned(),
            environment: None,
            version: Some("<2".to_owned()),
            components: vec![],
        });
        registry.insert("package".to_owned(), record("package", vec![latest, old]));
        registry.insert("peer".to_owned(), record("peer", vec![peer]));
        registry.insert(
            "support".to_owned(),
            record(
                "support",
                vec![release("2.0", Phase::Eet), release("1.0", Phase::Eet)],
            ),
        );
        let lock = resolve(
            &manifest(vec![
                RequestedMod::Package("package".to_owned()),
                RequestedMod::Package("peer".to_owned()),
            ]),
            &registry,
            "test",
            toolchain(),
        )
        .unwrap();
        assert_eq!(
            lock.packages
                .iter()
                .find(|package| package.package == "package")
                .unwrap()
                .version,
            "1.0"
        );
        assert_eq!(
            lock.packages
                .iter()
                .find(|package| package.package == "support")
                .unwrap()
                .version,
            "1.0"
        );
    }

    #[test]
    fn conflicts_and_phase_inversions_are_actionable() {
        let mut registry = Registry::new();
        let mut early = release("1", Phase::Eet);
        early.relationships.push(Relationship {
            kind: RelationshipKind::Before,
            package: "late".to_owned(),
            environment: None,
            components: vec![],
            version: None,
            when: RelationshipCondition::default(),
        });
        let late = release("1", Phase::Preprocess);
        registry.insert("early".to_owned(), record("early", vec![early]));
        registry.insert("late".to_owned(), record("late", vec![late]));
        assert!(matches!(
            resolve(
                &manifest(vec![
                    RequestedMod::Package("early".to_owned()),
                    RequestedMod::Package("late".to_owned())
                ]),
                &registry,
                "test",
                toolchain()
            ),
            Err(ResolveError::PhaseViolation { .. })
        ));
    }

    #[test]
    fn capability_checks_default_components() {
        let mut registry = Registry::new();
        let mut left = release("1", Phase::Eet);
        left.components.push(component("ai", "mage-ai"));
        let mut right = release("1", Phase::Eet);
        right.components.push(component("ai", "mage-ai"));
        registry.insert("left".to_owned(), record("left", vec![left]));
        registry.insert("right".to_owned(), record("right", vec![right]));
        assert!(matches!(
            resolve(
                &manifest(vec![
                    RequestedMod::Package("left".to_owned()),
                    RequestedMod::Package("right".to_owned())
                ]),
                &registry,
                "test",
                toolchain()
            ),
            Err(ResolveError::Capability { .. })
        ));
    }

    fn component(id: &str, capability: &str) -> iepm_core::Component {
        iepm_core::Component {
            id: id.to_owned(),
            default_selected: true,
            weidu: None,
            provides: vec![Capability {
                name: capability.to_owned(),
                exclusive: true,
            }],
        }
    }

    #[test]
    fn selects_content_identity_deterministically() {
        let mut registry = Registry::new();
        let mut package = release("1", Phase::Eet);
        package.artifacts = vec![
            Artifact {
                url: "https://example.invalid/b".to_owned(),
                sha256: "b".repeat(64),
                mirrors: vec![],
                format: Default::default(),
                platforms: vec![],
                architectures: vec![],
            },
            Artifact {
                url: "https://example.invalid/a".to_owned(),
                sha256: "a".repeat(64),
                mirrors: vec!["https://mirror.invalid/a".to_owned()],
                format: Default::default(),
                platforms: vec![],
                architectures: vec![],
            },
        ];
        registry.insert("package".to_owned(), record("package", vec![package]));
        let lock = resolve(
            &manifest(vec![RequestedMod::Package("package".to_owned())]),
            &registry,
            "test",
            toolchain(),
        )
        .unwrap();
        assert_eq!(
            lock.packages[0].artifact.as_ref().unwrap().sha256,
            "a".repeat(64)
        );
    }

    #[test]
    fn eet_fixture_models_a_source_environment_and_import_edge() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let text =
            std::fs::read_to_string(root.join("examples/eet-balanced/modpack.yaml")).unwrap();
        let fixture: Manifest = serde_yaml::from_str(&text).unwrap();
        let registry = iepm_registry::load(&root.join("registry")).unwrap();
        let lock = resolve(&fixture, &registry, "fixture", toolchain()).unwrap();
        assert!(lock.environments.contains_key("bgee-source"));
        assert!(lock.environments.contains_key("eet-target"));
        assert!(lock.execution.iter().any(|node| {
            node.id == "eet-target::eet" && node.predecessors == ["bgee-source::dlc-merger"]
        }));
    }
}

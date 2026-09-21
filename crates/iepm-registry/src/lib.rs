use anyhow::{Context, Result, bail};
use iepm_core::{Installer, PackageRecord, Registry, Release};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use walkdir::WalkDir;

pub fn load(path: &Path) -> Result<Registry> {
    let mut registry = Registry::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if !entry.file_type().is_file()
            || !matches!(
                entry.path().extension().and_then(|value| value.to_str()),
                Some("yaml" | "yml")
            )
        {
            continue;
        }
        let source = std::fs::read_to_string(entry.path())
            .with_context(|| format!("could not read {}", entry.path().display()))?;
        let record: PackageRecord = serde_yaml::from_str(&source)
            .with_context(|| format!("could not parse {}", entry.path().display()))?;
        if !matches!(record.schema, 1 | 2) {
            bail!(
                "{} uses unsupported schema {}",
                record.package,
                record.schema
            );
        }
        validate_record(&record)?;
        if registry.insert(record.package.clone(), record).is_some() {
            bail!("duplicate package record in {}", entry.path().display());
        }
    }
    if registry.is_empty() {
        bail!("no YAML package records found under {}", path.display());
    }
    validate_aliases(&registry)?;
    Ok(registry)
}

/// Versioned, inert structural facts observed in a TP2 source file. These are
/// deliberately not compatibility or execution claims: parsing never expands
/// an INCLUDE, evaluates an ACTION, or answers an installer prompt.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Tp2Observation {
    pub schema: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub languages: Vec<Tp2Language>,
    pub components: Vec<Tp2Component>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Tp2Language {
    /// WeiDU's language index is declaration order, not a portable language
    /// identity. It is retained only for a release-specific installer mapping.
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Tp2Component {
    /// The raw selector following BEGIN, such as @10. It is evidence only and
    /// is not translated or otherwise evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Read the small, stable structural surface of a TP2 file. The parser is
/// intentionally line-oriented; complex WeiDU source remains WeiDU's domain.
pub fn inspect_tp2(source: &str) -> Tp2Observation {
    let mut observation = Tp2Observation {
        schema: 1,
        version: None,
        languages: Vec::new(),
        components: Vec::new(),
        warnings: Vec::new(),
    };
    let mut current: Option<Tp2Component> = None;

    for raw_line in source.trim_start_matches('\u{feff}').lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if starts_keyword(line, "VERSION") {
            if observation.version.is_none() {
                observation.version = value_after_keyword(line, "VERSION");
            }
            continue;
        }
        if starts_keyword(line, "LANGUAGE") {
            if let Some(name) = value_after_keyword(line, "LANGUAGE") {
                observation.languages.push(Tp2Language {
                    id: observation.languages.len() as u32,
                    name,
                });
            }
            continue;
        }
        if starts_keyword(line, "BEGIN") {
            if let Some(component) = current.take() {
                observation.components.push(component);
            }
            current = Some(Tp2Component {
                begin: value_after_keyword(line, "BEGIN"),
                number: number_after_keyword(line, "DESIGNATED")
                    .or_else(|| number_after_keyword(line, "BEGIN")),
                label: value_after_keyword(line, "LABEL"),
            });
            continue;
        }
        if let Some(component) = current.as_mut() {
            if component.number.is_none() {
                component.number = number_after_keyword(line, "DESIGNATED");
            }
            if component.label.is_none() {
                component.label = value_after_keyword(line, "LABEL");
            }
        }
    }
    if let Some(component) = current {
        observation.components.push(component);
    }
    if observation.version.is_none() {
        observation
            .warnings
            .push("no VERSION declaration observed".to_owned());
    }
    if observation.components.is_empty() {
        observation
            .warnings
            .push("no BEGIN declarations observed".to_owned());
    }
    observation
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Tp2Review {
    pub schema: u32,
    pub package: String,
    pub release_id: String,
    pub installer_tp2: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp2_version: Option<String>,
    pub registry_version: String,
    pub status: Tp2ReviewStatus,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<Tp2Finding>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Tp2ReviewStatus {
    Match,
    Drift,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Tp2Finding {
    pub kind: String,
    pub message: String,
}

/// Compare an inspected TP2 against one already-curated release. The result is
/// review evidence, not an update operation: it intentionally has no route to
/// inherit compatibility, relationships, artifacts, prompts, or verification.
pub fn review_tp2(
    registry: &Registry,
    package_id: &str,
    release_id: &str,
    installer_tp2: Option<&str>,
    observed: &Tp2Observation,
) -> Result<Tp2Review> {
    let package = registry
        .get(package_id)
        .ok_or_else(|| anyhow::anyhow!("package not found: {package_id}"))?;
    let release = package
        .releases
        .iter()
        .find(|candidate| candidate.id() == release_id)
        .ok_or_else(|| anyhow::anyhow!("{package_id} has no release {release_id}"))?;
    let installer = select_installer(release, installer_tp2)?;
    let mut findings = Vec::new();

    match observed.version.as_deref() {
        Some(version) if version != release.version => findings.push(Tp2Finding {
            kind: "tp2-version-drift".to_owned(),
            message: format!(
                "registry display version {} differs from TP2 VERSION {}",
                release.version, version
            ),
        }),
        None => findings.push(Tp2Finding {
            kind: "tp2-version-unobserved".to_owned(),
            message: "the TP2 has no observed VERSION declaration to compare".to_owned(),
        }),
        _ => {}
    }

    review_languages(installer, observed, &mut findings);
    review_components(release, installer, observed, &mut findings);
    review_duplicates(observed, &mut findings);

    Ok(Tp2Review {
        schema: 1,
        package: package.package.clone(),
        release_id: release.id().to_owned(),
        installer_tp2: installer.tp2.clone(),
        tp2_version: observed.version.clone(),
        registry_version: release.version.clone(),
        status: if findings.is_empty() {
            Tp2ReviewStatus::Match
        } else {
            Tp2ReviewStatus::Drift
        },
        findings,
    })
}

fn select_installer<'a>(release: &'a Release, requested: Option<&str>) -> Result<&'a Installer> {
    if let Some(tp2) = requested {
        return release
            .installers
            .iter()
            .find(|installer| same_tp2(&installer.tp2, tp2))
            .ok_or_else(|| anyhow::anyhow!("{} has no installer TP2 {tp2}", release.id()));
    }
    match release.installers.as_slice() {
        [installer] => Ok(installer),
        [] => bail!("{} has no installer metadata to review", release.id()),
        _ => bail!(
            "{} has multiple installer TP2 files; pass an explicit installer TP2 selector",
            release.id()
        ),
    }
}

fn review_languages(
    installer: &Installer,
    observed: &Tp2Observation,
    findings: &mut Vec<Tp2Finding>,
) {
    for expected in &installer.languages {
        let observed_language = observed
            .languages
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(&expected.name));
        match observed_language {
            None => findings.push(Tp2Finding {
                kind: "language-mapping-missing".to_owned(),
                message: format!(
                    "registry language {} (index {}) is not declared by the TP2",
                    expected.name, expected.id
                ),
            }),
            Some(actual) if actual.id != expected.id => findings.push(Tp2Finding {
                kind: "language-index-drift".to_owned(),
                message: format!(
                    "registry language {} has index {}, but TP2 declaration order gives {}",
                    expected.name, expected.id, actual.id
                ),
            }),
            _ => {}
        }
    }
}

fn review_components(
    release: &Release,
    installer: &Installer,
    observed: &Tp2Observation,
    findings: &mut Vec<Tp2Finding>,
) {
    for component in &release.components {
        let Some(selector) = component.weidu.as_ref() else {
            continue;
        };
        if !same_tp2(&selector.tp2, &installer.tp2) {
            continue;
        }
        if let Some(number) = selector.number {
            if !observed
                .components
                .iter()
                .any(|candidate| candidate.number == Some(number))
            {
                findings.push(Tp2Finding {
                    kind: "component-number-missing".to_owned(),
                    message: format!(
                        "component {} expects designated number {}, which the TP2 does not expose",
                        component.id, number
                    ),
                });
            }
        }
        if let Some(label) = selector.label.as_deref() {
            if !observed
                .components
                .iter()
                .any(|candidate| candidate.label.as_deref() == Some(label))
            {
                findings.push(Tp2Finding {
                    kind: "component-label-missing".to_owned(),
                    message: format!(
                        "component {} expects LABEL {}, which the TP2 does not expose",
                        component.id, label
                    ),
                });
            }
        }
        if let (Some(number), Some(label)) = (selector.number, selector.label.as_deref()) {
            if !observed.components.iter().any(|candidate| {
                candidate.number == Some(number) && candidate.label.as_deref() == Some(label)
            }) {
                findings.push(Tp2Finding {
                    kind: "component-selector-pair-drift".to_owned(),
                    message: format!(
                        "component {} does not expose designated number {} and LABEL {} together",
                        component.id, number, label
                    ),
                });
            }
        }
    }
}

fn review_duplicates(observed: &Tp2Observation, findings: &mut Vec<Tp2Finding>) {
    let mut numbers = BTreeMap::<u32, usize>::new();
    let mut labels = BTreeMap::<&str, usize>::new();
    for component in &observed.components {
        if let Some(number) = component.number {
            *numbers.entry(number).or_default() += 1;
        }
        if let Some(label) = component.label.as_deref() {
            *labels.entry(label).or_default() += 1;
        }
    }
    for (number, count) in numbers.into_iter().filter(|(_, count)| *count > 1) {
        findings.push(Tp2Finding {
            kind: "duplicate-designated-number".to_owned(),
            message: format!("TP2 exposes designated number {number} {count} times"),
        });
    }
    for (label, count) in labels.into_iter().filter(|(_, count)| *count > 1) {
        findings.push(Tp2Finding {
            kind: "duplicate-label".to_owned(),
            message: format!("TP2 exposes LABEL {label} {count} times"),
        });
    }
}

fn same_tp2(left: &str, right: &str) -> bool {
    left.replace('\\', "/")
        .eq_ignore_ascii_case(&right.replace('\\', "/"))
}

fn starts_keyword(line: &str, keyword: &str) -> bool {
    line.get(..keyword.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(keyword))
        && line[keyword.len()..]
            .chars()
            .next()
            .is_none_or(|character| !is_identifier_character(character))
}

fn value_after_keyword(line: &str, keyword: &str) -> Option<String> {
    let offset = find_keyword(line, keyword)? + keyword.len();
    let value = line[offset..].trim_start();
    let delimiter = match value.chars().next()? {
        '~' => '~',
        '"' => '"',
        _ => return value.split_whitespace().next().map(str::to_owned),
    };
    let content = &value[delimiter.len_utf8()..];
    content.find(delimiter).map(|end| content[..end].to_owned())
}

fn number_after_keyword(line: &str, keyword: &str) -> Option<u32> {
    let offset = find_keyword(line, keyword)? + keyword.len();
    line[offset..].split_whitespace().next()?.parse().ok()
}

fn find_keyword(line: &str, keyword: &str) -> Option<usize> {
    let upper_line = line.to_ascii_uppercase();
    let keyword = keyword.to_ascii_uppercase();
    let mut start = 0;
    while let Some(relative) = upper_line[start..].find(&keyword) {
        let index = start + relative;
        let before = line[..index].chars().next_back();
        let after = line[index + keyword.len()..].chars().next();
        if before.is_none_or(|character| !is_identifier_character(character))
            && after.is_none_or(|character| !is_identifier_character(character))
        {
            return Some(index);
        }
        start = index + keyword.len();
    }
    None
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod tests {
    use super::*;

    fn registry() -> Registry {
        let record: PackageRecord = serde_yaml::from_str(
            r#"
schema: 2
package: sample-mod
releases:
  - version: "1.2.3"
    release_id: sample-release
    compatibility:
      games: [bg2ee]
    install:
      phase: eet
    provenance: derived
    installers:
      - tp2: Sample/Sample.tp2
        languages:
          - id: 0
            name: English
    components:
      - id: enable-feature
        weidu:
          tp2: Sample/Sample.tp2
          label: SAMPLE-ENABLE
          number: 10
"#,
        )
        .expect("fixture registry parses");
        Registry::from([(record.package.clone(), record)])
    }

    #[test]
    fn inspect_tp2_reads_only_structural_declarations() {
        let observation = inspect_tp2(
            r#"
// VERSION ~not-a-version~
VERSION ~1.2.3~
LANGUAGE ~English~
BEGIN @100
  DESIGNATED 10
  LABEL ~SAMPLE-ENABLE~
ACTION_DEFINE_ASSOCIATIVE_ARRAY ignored BEGIN LABEL
"#,
        );

        assert_eq!(observation.version.as_deref(), Some("1.2.3"));
        assert_eq!(
            observation.languages,
            vec![Tp2Language {
                id: 0,
                name: "English".to_owned()
            }]
        );
        assert_eq!(
            observation.components,
            vec![Tp2Component {
                begin: Some("@100".to_owned()),
                number: Some(10),
                label: Some("SAMPLE-ENABLE".to_owned()),
            }]
        );
    }

    #[test]
    fn review_tp2_reports_release_specific_drift_without_mutation() {
        let observed = inspect_tp2(
            r#"
VERSION ~1.2.4~
LANGUAGE ~French~
LANGUAGE ~English~
BEGIN @100 DESIGNATED 11 LABEL ~SAMPLE-RENAMED~
"#,
        );
        let review = review_tp2(&registry(), "sample-mod", "sample-release", None, &observed)
            .expect("review succeeds");

        assert_eq!(review.status, Tp2ReviewStatus::Drift);
        assert!(
            review
                .findings
                .iter()
                .any(|finding| finding.kind == "tp2-version-drift")
        );
        assert!(
            review
                .findings
                .iter()
                .any(|finding| finding.kind == "language-index-drift")
        );
        assert!(
            review
                .findings
                .iter()
                .any(|finding| finding.kind == "component-number-missing")
        );
        assert!(
            review
                .findings
                .iter()
                .any(|finding| finding.kind == "component-label-missing")
        );
    }

    #[test]
    fn review_tp2_accepts_matching_structural_selectors() {
        let observed = inspect_tp2(
            r#"
VERSION ~1.2.3~
LANGUAGE ~English~
BEGIN @100 DESIGNATED 10 LABEL ~SAMPLE-ENABLE~
"#,
        );
        let review = review_tp2(&registry(), "sample-mod", "sample-release", None, &observed)
            .expect("review succeeds");

        assert_eq!(review.status, Tp2ReviewStatus::Match);
        assert!(review.findings.is_empty());
    }

    #[test]
    fn review_tp2_requires_a_number_and_label_to_describe_the_same_component() {
        let observed = inspect_tp2(
            r#"
VERSION ~1.2.3~
LANGUAGE ~English~
BEGIN @100 DESIGNATED 10 LABEL ~SAMPLE-OTHER~
BEGIN @101 DESIGNATED 11 LABEL ~SAMPLE-ENABLE~
"#,
        );
        let review = review_tp2(&registry(), "sample-mod", "sample-release", None, &observed)
            .expect("review succeeds");

        assert!(
            review
                .findings
                .iter()
                .any(|finding| finding.kind == "component-selector-pair-drift")
        );
    }
}

fn validate_record(record: &PackageRecord) -> Result<()> {
    if !valid_package_id(&record.package) {
        bail!("{} has an invalid package ID", record.package);
    }
    if record.releases.is_empty() {
        bail!("{} has no releases", record.package);
    }
    let mut release_ids = BTreeSet::new();
    for release in &record.releases {
        if !release_ids.insert(release.id()) {
            bail!(
                "{} has duplicate release ID {}",
                record.package,
                release.id()
            );
        }
        let mut components = BTreeSet::new();
        for component in &release.components {
            if !components.insert(&component.id) {
                bail!(
                    "{} {} has duplicate component {}",
                    record.package,
                    release.id(),
                    component.id
                );
            }
        }
        for artifact in release.artifacts() {
            if !artifact.url.starts_with("https://")
                || artifact.sha256.len() != 64
                || !artifact
                    .sha256
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
            {
                bail!(
                    "{} {} has an invalid artifact",
                    record.package,
                    release.id()
                );
            }
        }
    }
    Ok(())
}

fn validate_aliases(registry: &Registry) -> Result<()> {
    let mut names = BTreeSet::new();
    for record in registry.values() {
        names.insert(record.package.as_str());
    }
    for record in registry.values() {
        for alias in &record.aliases {
            if !valid_package_id(alias) || alias == &record.package {
                bail!("{} has invalid alias {}", record.package, alias);
            }
            if !names.insert(alias) {
                bail!("{} has duplicate alias {}", record.package, alias);
            }
        }
    }
    Ok(())
}

fn valid_package_id(value: &str) -> bool {
    !value.is_empty()
        && value.split('-').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        })
}

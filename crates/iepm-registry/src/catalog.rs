use anyhow::{Context, Result, bail};
use iepm_core::Registry;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const DISCOVERY_CATALOG_SCHEMA: u32 = 1;
pub const FORGE_SOURCE_KIND: &str = "infinity-mod-forge";

/// A versioned, display-oriented ecosystem catalog. It is intentionally
/// separate from executable `PackageRecord`s: an entry here never participates
/// in resolution or asserts artifact identity, compatibility, or support.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryCatalog {
    pub schema: u32,
    pub source: DiscoveryCatalogSource,
    pub entries: Vec<DiscoveryCatalogEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryCatalogSource {
    pub kind: String,
    pub repository: String,
    pub revision: String,
    pub license: String,
    /// Repository-relative upstream input path, not a local checkout path.
    pub input: String,
    pub input_sha256: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryCatalogEntry {
    /// Stable upstream identity within `source`, retained even if the display
    /// name or TP2 token later changes.
    pub source_id: u64,
    pub tp2_name: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub category: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub narrative_phases: Vec<String>,
    /// Literal/derived source filtering observations. Omission means the
    /// source did not publish a game predicate; it is not a compatibility
    /// claim for every target.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observed_game_targets: Vec<String>,
    pub source_game_predicate_observed: bool,
    pub component_count: u32,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub language_indices: BTreeMap<String, u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<DiscoveryGithubRepository>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    pub source_install_order: i64,
    /// A stable, generated suggestion. It is not a resolver identity until a
    /// curated package record explicitly adopts it or records an exact alias.
    pub candidate_package: String,
    pub identity_status: DiscoveryIdentityStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curated_package: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryGithubRepository {
    pub owner: String,
    pub repository: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DiscoveryIdentityStatus {
    Candidate,
    MatchedCurated,
    CandidateCollision,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryCatalogImportReport {
    pub schema: u32,
    pub source: DiscoveryCatalogSource,
    pub records_read: u64,
    pub records_imported: u64,
    pub rejections: Vec<DiscoveryCatalogRejection>,
    pub candidate_collisions: Vec<DiscoveryCandidateCollision>,
    pub curated_matches: Vec<DiscoveryCuratedMatch>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryCatalogRejection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<u64>,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryCandidateCollision {
    pub candidate_package: String,
    pub source_ids: Vec<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiscoveryCuratedMatch {
    pub source_id: u64,
    pub candidate_package: String,
    pub curated_package: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryCatalogImport {
    pub catalog: DiscoveryCatalog,
    pub report: DiscoveryCatalogImportReport,
}

/// Import Forge's generated `data/mods-index.json` from an already pinned
/// source checkout. This is deliberately offline: source retrieval and link
/// health belong to PA-2, while this step is reproducible from a chosen commit.
pub fn import_infinity_mod_forge_index(
    input: &Path,
    repository: &str,
    revision: &str,
    license: &str,
    curated_registry: Option<&Registry>,
) -> Result<DiscoveryCatalogImport> {
    if repository.trim().is_empty() || revision.trim().is_empty() || license.trim().is_empty() {
        bail!("catalog source repository, revision, and license are required");
    }
    let bytes = fs::read(input).with_context(|| format!("could not read {}", input.display()))?;
    let values: Vec<serde_json::Value> = serde_json::from_slice(&bytes)
        .with_context(|| format!("{} is not a Forge index array", input.display()))?;
    let source = DiscoveryCatalogSource {
        kind: FORGE_SOURCE_KIND.to_owned(),
        repository: repository.to_owned(),
        revision: revision.to_owned(),
        license: license.to_owned(),
        input: "data/mods-index.json".to_owned(),
        input_sha256: sha256(&bytes),
    };
    let curated_lookup = curated_registry
        .map(build_curated_lookup)
        .unwrap_or_default();
    let mut entries = Vec::new();
    let mut rejections = Vec::new();
    let mut seen_source_ids = BTreeSet::new();

    for value in values.iter().cloned() {
        let source_id = value.get("i").and_then(serde_json::Value::as_u64);
        let record: ForgeIndexRecord = match serde_json::from_value(value) {
            Ok(record) => record,
            Err(error) => {
                rejections.push(DiscoveryCatalogRejection {
                    source_id,
                    reason: format!("invalid Forge index record: {error}"),
                });
                continue;
            }
        };
        if !seen_source_ids.insert(record.id) {
            rejections.push(DiscoveryCatalogRejection {
                source_id: Some(record.id),
                reason: "duplicate Forge source ID".to_owned(),
            });
            continue;
        }
        let tp2_name = record.tp2_name.trim().to_owned();
        let name = record.name.trim().to_owned();
        if tp2_name.is_empty() || name.is_empty() || record.category.trim().is_empty() {
            rejections.push(DiscoveryCatalogRejection {
                source_id: Some(record.id),
                reason: "TP2 name, display name, and category are required".to_owned(),
            });
            continue;
        }
        let candidate_package = candidate_package_id(&tp2_name, record.id);
        let curated_packages = curated_lookup
            .get(&normalize_tp2_identity(&tp2_name))
            .cloned()
            .unwrap_or_default();
        let curated_package = (curated_packages.len() == 1).then(|| {
            curated_packages
                .iter()
                .next()
                .expect("checked one curated package")
                .clone()
        });
        entries.push(DiscoveryCatalogEntry {
            source_id: record.id,
            tp2_name,
            name,
            author: nonempty(record.author),
            summary: nonempty(record.summary),
            category: record.category,
            tags: sorted_unique(record.tags),
            narrative_phases: sorted_unique(record.narrative_phases),
            source_game_predicate_observed: record.games.is_some(),
            observed_game_targets: sorted_unique(record.games.unwrap_or_default()),
            component_count: record.component_count,
            language_indices: record.languages,
            homepage: nonempty(record.homepage),
            github: record.github.map(|github| DiscoveryGithubRepository {
                owner: github.owner,
                repository: github.repository,
            }),
            source_version: nonempty(record.version),
            source_install_order: record.install_order.unwrap_or_default(),
            candidate_package,
            identity_status: if curated_package.is_some() {
                DiscoveryIdentityStatus::MatchedCurated
            } else {
                DiscoveryIdentityStatus::Candidate
            },
            curated_package,
        });
    }

    entries.sort_by_key(|entry| entry.source_id);
    let collisions = candidate_collisions(&entries);
    let colliding_candidates = collisions
        .iter()
        .map(|collision| collision.candidate_package.as_str())
        .collect::<BTreeSet<_>>();
    for entry in &mut entries {
        if colliding_candidates.contains(entry.candidate_package.as_str()) {
            entry.identity_status = DiscoveryIdentityStatus::CandidateCollision;
            entry.curated_package = None;
        }
    }
    let curated_matches = entries
        .iter()
        .filter_map(|entry| {
            entry
                .curated_package
                .as_ref()
                .map(|curated_package| DiscoveryCuratedMatch {
                    source_id: entry.source_id,
                    candidate_package: entry.candidate_package.clone(),
                    curated_package: curated_package.clone(),
                })
        })
        .collect::<Vec<_>>();
    let catalog = DiscoveryCatalog {
        schema: DISCOVERY_CATALOG_SCHEMA,
        source: source.clone(),
        entries,
    };
    validate_discovery_catalog(&catalog)?;
    let report = DiscoveryCatalogImportReport {
        schema: DISCOVERY_CATALOG_SCHEMA,
        source,
        records_read: values.len() as u64,
        records_imported: catalog.entries.len() as u64,
        rejections,
        candidate_collisions: collisions,
        curated_matches,
    };
    Ok(DiscoveryCatalogImport { catalog, report })
}

pub fn write_discovery_catalog_import(
    import: &DiscoveryCatalogImport,
    catalog_output: &Path,
    report_output: &Path,
) -> Result<()> {
    write_json(catalog_output, &import.catalog)?;
    write_json(report_output, &import.report)
}

pub fn load_discovery_catalog(path: &Path) -> Result<DiscoveryCatalog> {
    let source =
        fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?;
    let catalog: DiscoveryCatalog = serde_json::from_str(&source)
        .with_context(|| format!("could not parse {}", path.display()))?;
    validate_discovery_catalog(&catalog)?;
    Ok(catalog)
}

pub fn validate_discovery_catalog(catalog: &DiscoveryCatalog) -> Result<()> {
    if catalog.schema != DISCOVERY_CATALOG_SCHEMA {
        bail!("unsupported discovery catalog schema {}", catalog.schema);
    }
    if catalog.source.kind != FORGE_SOURCE_KIND
        || catalog.source.repository.trim().is_empty()
        || catalog.source.revision.trim().is_empty()
        || catalog.source.license.trim().is_empty()
        || catalog.source.input != "data/mods-index.json"
        || !is_sha256(&catalog.source.input_sha256)
    {
        bail!("invalid discovery catalog source");
    }
    let mut source_ids = BTreeSet::new();
    let mut previous = None;
    for entry in &catalog.entries {
        if !source_ids.insert(entry.source_id) {
            bail!("duplicate discovery source ID {}", entry.source_id);
        }
        if previous.is_some_and(|previous| previous >= entry.source_id) {
            bail!("discovery catalog entries are not sorted by source ID");
        }
        previous = Some(entry.source_id);
        if entry.tp2_name.trim().is_empty()
            || entry.name.trim().is_empty()
            || entry.category.trim().is_empty()
            || !valid_package_id(&entry.candidate_package)
        {
            bail!("invalid discovery entry {}", entry.source_id);
        }
        if entry.identity_status == DiscoveryIdentityStatus::MatchedCurated
            && entry.curated_package.is_none()
        {
            bail!(
                "matched discovery entry {} has no curated package",
                entry.source_id
            );
        }
        if entry.identity_status != DiscoveryIdentityStatus::MatchedCurated
            && entry.curated_package.is_some()
        {
            bail!(
                "unmatched discovery entry {} has a curated package",
                entry.source_id
            );
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct ForgeIndexRecord {
    #[serde(rename = "i")]
    id: u64,
    #[serde(rename = "t")]
    tp2_name: String,
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "c")]
    category: String,
    #[serde(rename = "a", default)]
    author: Option<String>,
    #[serde(rename = "sum", default)]
    summary: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(rename = "ph", default)]
    narrative_phases: Vec<String>,
    #[serde(default)]
    games: Option<Vec<String>>,
    #[serde(rename = "cc", default)]
    component_count: u32,
    #[serde(rename = "langs", default)]
    languages: BTreeMap<String, u32>,
    #[serde(rename = "u", default)]
    homepage: Option<String>,
    #[serde(rename = "gh", default)]
    github: Option<ForgeGithubRepository>,
    #[serde(rename = "v", default)]
    version: Option<String>,
    #[serde(rename = "ord", default)]
    install_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ForgeGithubRepository {
    #[serde(rename = "o")]
    owner: String,
    #[serde(rename = "r")]
    repository: String,
}

fn build_curated_lookup(registry: &Registry) -> BTreeMap<String, BTreeSet<String>> {
    let mut lookup = BTreeMap::<String, BTreeSet<String>>::new();
    for record in registry.values() {
        for alias in std::iter::once(&record.package).chain(record.aliases.iter()) {
            lookup
                .entry(normalize_tp2_identity(alias))
                .or_default()
                .insert(record.package.clone());
        }
        for release in &record.releases {
            for installer in &release.installers {
                let path = Path::new(&installer.tp2);
                if let Some(stem) = path.file_stem().and_then(|value| value.to_str()) {
                    lookup
                        .entry(normalize_tp2_identity(stem))
                        .or_default()
                        .insert(record.package.clone());
                }
            }
        }
    }
    lookup
}

fn candidate_collisions(entries: &[DiscoveryCatalogEntry]) -> Vec<DiscoveryCandidateCollision> {
    let mut candidates = BTreeMap::<String, Vec<u64>>::new();
    for entry in entries {
        candidates
            .entry(entry.candidate_package.clone())
            .or_default()
            .push(entry.source_id);
    }
    candidates
        .into_iter()
        .filter_map(|(candidate_package, source_ids)| {
            (source_ids.len() > 1).then_some(DiscoveryCandidateCollision {
                candidate_package,
                source_ids,
            })
        })
        .collect()
}

fn candidate_package_id(tp2_name: &str, source_id: u64) -> String {
    let value = normalize_tp2_identity(tp2_name);
    if valid_package_id(&value) {
        value
    } else {
        format!("forge-{source_id}")
    }
}

fn normalize_tp2_identity(value: &str) -> String {
    let value = value.trim().trim_end_matches(".tp2");
    let value = value.strip_prefix("setup-").unwrap_or(value);
    let mut output = String::new();
    let mut previous_separator = true;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            previous_separator = false;
        } else if !previous_separator {
            output.push('-');
            previous_separator = true;
        }
    }
    output.trim_matches('-').to_owned()
}

fn sorted_unique(mut values: Vec<String>) -> Vec<String> {
    values.retain(|value| !value.trim().is_empty());
    values.sort();
    values.dedup();
    values
}

fn nonempty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.trim().is_empty())
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
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

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("{} has no parent directory", path.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("could not create {}", parent.display()))?;
    let source = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{source}\n"))
        .with_context(|| format!("could not write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use iepm_core::{Compatibility, Install, PackageRecord, Phase, Provenance, Release};

    fn registry() -> Registry {
        let record = PackageRecord {
            schema: 2,
            package: "tweaks-anthology".to_owned(),
            aliases: vec!["cdtweaks".to_owned()],
            lineage: vec![],
            releases: vec![Release {
                version: "v18".to_owned(),
                release_id: Some("v18".to_owned()),
                semver: None,
                artifact: None,
                artifacts: vec![],
                materialization: Default::default(),
                compatibility: Compatibility {
                    games: vec!["bg2ee".to_owned()],
                },
                install: Install {
                    phase: Phase::Eet,
                    before: vec![],
                    after: vec![],
                },
                provenance: Provenance::Unverified,
                claims: vec![],
                dependencies: vec![],
                relationships: vec![],
                components: vec![],
                installers: vec![],
            }],
        };
        Registry::from([(record.package.clone(), record)])
    }

    #[test]
    fn imports_discovery_facts_without_promoting_compatibility() {
        let root = std::env::temp_dir().join(format!("iepm-catalog-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let input = root.join("mods-index.json");
        fs::write(
            &input,
            r#"[
  {"i": 18, "t": "cdtweaks", "n": "Tweaks Anthology", "c": "TWEAKS", "games": ["bg2ee"], "cc": 200, "ph": ["SoA"], "langs": {"en": 0}, "gh": {"o": "Gibberlings3", "r": "Tweaks-Anthology"}},
  {"i": 19, "t": "Unknown Mod", "n": "Unknown Mod", "c": "QUESTS", "cc": 1}
]"#,
        )
        .unwrap();
        let imported = import_infinity_mod_forge_index(
            &input,
            "https://example.invalid/forge",
            "deadbeef",
            "MIT",
            Some(&registry()),
        )
        .unwrap();

        assert_eq!(imported.catalog.entries.len(), 2);
        assert_eq!(
            imported.catalog.entries[0].curated_package.as_deref(),
            Some("tweaks-anthology")
        );
        assert_eq!(
            imported.catalog.entries[0].identity_status,
            DiscoveryIdentityStatus::MatchedCurated
        );
        assert_eq!(
            imported.catalog.entries[0].observed_game_targets,
            vec!["bg2ee"]
        );
        assert!(imported.catalog.entries[1].observed_game_targets.is_empty());
        assert!(!imported.catalog.entries[1].source_game_predicate_observed);
        assert_eq!(imported.catalog.entries[1].candidate_package, "unknown-mod");
        assert_eq!(imported.report.records_read, 2);
        assert!(imported.report.rejections.is_empty());
    }

    #[test]
    fn reports_candidate_collisions_and_rejections() {
        let root =
            std::env::temp_dir().join(format!("iepm-catalog-collision-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let input = root.join("mods-index.json");
        fs::write(
            &input,
            r#"[
  {"i": 1, "t": "My Mod", "n": "First", "c": "TWEAKS"},
  {"i": 2, "t": "my_mod", "n": "Second", "c": "TWEAKS"},
  {"i": 3, "t": "", "n": "Bad", "c": "TWEAKS"}
]"#,
        )
        .unwrap();
        let imported = import_infinity_mod_forge_index(
            &input,
            "https://example.invalid/forge",
            "deadbeef",
            "MIT",
            None,
        )
        .unwrap();

        assert_eq!(imported.catalog.entries.len(), 2);
        assert_eq!(imported.report.rejections.len(), 1);
        assert_eq!(
            imported.report.candidate_collisions[0].candidate_package,
            "my-mod"
        );
        assert!(
            imported
                .catalog
                .entries
                .iter()
                .all(|entry| entry.identity_status == DiscoveryIdentityStatus::CandidateCollision)
        );
    }

    #[test]
    fn output_round_trips_through_validation() {
        let root = std::env::temp_dir().join(format!("iepm-catalog-output-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let input = root.join("mods-index.json");
        fs::write(
            &input,
            r#"[{"i": 1, "t": "sample", "n": "Sample", "c": "TWEAKS"}]"#,
        )
        .unwrap();
        let imported = import_infinity_mod_forge_index(
            &input,
            "https://example.invalid/forge",
            "deadbeef",
            "MIT",
            None,
        )
        .unwrap();
        let catalog = root.join("catalog.json");
        let report = root.join("report.json");
        write_discovery_catalog_import(&imported, &catalog, &report).unwrap();
        assert_eq!(load_discovery_catalog(&catalog).unwrap(), imported.catalog);
    }

    #[test]
    fn checked_in_forge_catalog_is_valid_and_accounts_for_the_snapshot() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let catalog =
            load_discovery_catalog(&root.join("registry/catalog/infinity-mod-forge.json"))
                .expect("checked-in Forge catalog should validate");
        let report_source =
            fs::read_to_string(root.join("registry/catalog/infinity-mod-forge-report.json"))
                .expect("checked-in Forge report should exist");
        let report: DiscoveryCatalogImportReport =
            serde_json::from_str(&report_source).expect("checked-in Forge report should parse");

        assert_eq!(catalog.entries.len(), 813);
        assert_eq!(report.records_read, 813);
        assert_eq!(report.records_imported, 813);
        assert!(report.rejections.is_empty());
        assert!(report.candidate_collisions.is_empty());
        assert_eq!(catalog.source, report.source);
    }
}

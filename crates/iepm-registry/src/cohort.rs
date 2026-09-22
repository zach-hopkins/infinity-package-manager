//! Reproducible Product A cohort selection.  This module deliberately works
//! from discovery identities and frozen source observations.  It does not
//! create releases, artifacts, or compatibility claims.

use crate::catalog::DiscoveryCatalog;
use anyhow::{Context, Result, bail};
use iepm_core::Manifest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const SELECTION_CORPUS_SCHEMA: u32 = 1;
pub const PRODUCT_A_COHORT_SCHEMA: u32 = 1;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SelectionCorpus {
    pub schema: u32,
    pub captured_on: String,
    pub catalog_source_revision: String,
    pub sources: Vec<SelectionCorpusSource>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SelectionCorpusSource {
    pub id: String,
    /// A source family is informative only.  The algorithm never compares
    /// source-specific raw popularity numbers.
    pub family: String,
    pub label: String,
    pub input: String,
    pub input_sha256: String,
    pub deduplication_key: String,
    /// Reference fixtures are included to enforce required coverage, but do
    /// not inflate popularity coverage.
    pub counts_toward_coverage: bool,
    pub selections: Vec<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unmapped_tokens: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProductACohort {
    pub schema: u32,
    pub corpus_captured_on: String,
    pub catalog_source_revision: String,
    pub minimum_packages: usize,
    pub minimum_coverage_basis_points: u32,
    pub packages: Vec<CohortPackage>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CohortPackage {
    pub source_id: u64,
    pub candidate_package: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curated_package: Option<String>,
    pub name: String,
    pub category: String,
    pub observed_game_targets: Vec<String>,
    /// This says why PA-4 needs to examine the package; it is never a public
    /// support label.
    pub inclusion_reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProductACoverageReport {
    pub schema: u32,
    pub corpus_captured_on: String,
    pub catalog_source_revision: String,
    pub counted_sources: usize,
    pub reference_sources: usize,
    pub unique_selected_candidates: usize,
    pub weighted_selection_total: f64,
    pub weighted_selection_covered: f64,
    pub coverage: f64,
    pub selected_packages: usize,
    pub source_diversity: BTreeMap<String, usize>,
    pub deferred_candidates: Vec<DeferredCandidate>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DeferredCandidate {
    pub source_id: u64,
    pub candidate_package: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CohortBuild {
    pub cohort: ProductACohort,
    pub report: ProductACoverageReport,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CohortBuildOptions {
    pub minimum_packages: usize,
    pub minimum_coverage: f64,
    pub mandatory_packages: Vec<String>,
}

/// Capture a small, dated source ledger without committing the user's source
/// list or its local filesystem location.  Forge preset keys identify an
/// upstream package by `<source-id>-<component>`; components are collapsed to
/// one package selection per stack.
pub fn capture_product_a_selection_corpus(
    catalog: &DiscoveryCatalog,
    forge_presets: &Path,
    manifests: &[std::path::PathBuf],
    reference_list: Option<&Path>,
    captured_on: &str,
) -> Result<SelectionCorpus> {
    require_date(captured_on)?;
    let mut sources = Vec::new();

    let bytes = fs::read(forge_presets)
        .with_context(|| format!("could not read {}", forge_presets.display()))?;
    let presets: Vec<ForgePreset> = serde_json::from_slice(&bytes)
        .with_context(|| format!("{} is not a Forge presets array", forge_presets.display()))?;
    for preset in presets {
        let selections = preset
            .keys
            .iter()
            .filter_map(|key| {
                key.split_once('-')
                    .and_then(|(id, _)| id.parse::<u64>().ok())
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        sources.push(SelectionCorpusSource {
            id: format!("forge-preset-{}", slug(&preset.id)),
            family: "forge-preset".to_owned(),
            label: preset.name,
            input: "Infinity Mod Forge data/presets.json".to_owned(),
            input_sha256: sha256(&bytes),
            deduplication_key: format!("forge-preset:{}", preset.id),
            counts_toward_coverage: true,
            selections,
            unmapped_tokens: Vec::new(),
        });
    }

    for manifest_path in manifests {
        let bytes = fs::read(manifest_path)
            .with_context(|| format!("could not read {}", manifest_path.display()))?;
        let manifest: Manifest = serde_yaml::from_slice(&bytes)
            .with_context(|| format!("could not parse {}", manifest_path.display()))?;
        let mut selections = BTreeSet::new();
        let mut unmapped_tokens = Vec::new();
        for selection in manifest.mods {
            if let Some(source_id) = map_catalog_token(catalog, selection.package()) {
                selections.insert(source_id);
            } else {
                unmapped_tokens.push(selection.package().to_owned());
            }
        }
        let label = manifest_path
            .parent()
            .and_then(Path::file_name)
            .and_then(|value| value.to_str())
            .unwrap_or("IEPM reference manifest")
            .to_owned();
        sources.push(SelectionCorpusSource {
            id: format!("iepm-reference-{}", slug(&label)),
            family: "iepm-reference".to_owned(),
            label,
            input: "IEPM checked-in example manifest".to_owned(),
            input_sha256: sha256(&bytes),
            deduplication_key: format!("iepm-reference:{}", sha256(&bytes)),
            counts_toward_coverage: false,
            selections: selections.into_iter().collect(),
            unmapped_tokens,
        });
    }

    if let Some(reference_list) = reference_list {
        let bytes = fs::read(reference_list)
            .with_context(|| format!("could not read {}", reference_list.display()))?;
        let text = String::from_utf8(bytes.clone())
            .with_context(|| format!("{} is not UTF-8", reference_list.display()))?;
        let mut selections = BTreeSet::new();
        let mut unmapped_tokens = BTreeSet::new();
        for line in text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
        {
            let token = line
                .split(';')
                .next()
                .unwrap_or(line)
                .split(':')
                .next()
                .unwrap_or(line);
            let source_id = map_catalog_token(catalog, token);
            if let Some(source_id) = source_id {
                selections.insert(source_id);
            } else {
                unmapped_tokens.insert(token.to_owned());
            }
        }
        sources.push(SelectionCorpusSource {
            id: "user-forge-reference-list".to_owned(),
            family: "user-reference".to_owned(),
            label: "User-supplied Forge reference list".to_owned(),
            input: "user-supplied Forge-style reference list (not committed)".to_owned(),
            input_sha256: sha256(&bytes),
            deduplication_key: format!("user-reference:{}", sha256(&bytes)),
            counts_toward_coverage: true,
            selections: selections.into_iter().collect(),
            unmapped_tokens: unmapped_tokens.into_iter().collect(),
        });
    }
    sources.sort_by(|left, right| left.id.cmp(&right.id));
    let corpus = SelectionCorpus {
        schema: SELECTION_CORPUS_SCHEMA,
        captured_on: captured_on.to_owned(),
        catalog_source_revision: catalog.source.revision.clone(),
        sources,
    };
    validate_selection_corpus(&corpus, catalog)?;
    Ok(corpus)
}

pub fn build_product_a_cohort(
    catalog: &DiscoveryCatalog,
    corpus: &SelectionCorpus,
    options: &CohortBuildOptions,
) -> Result<CohortBuild> {
    validate_selection_corpus(corpus, catalog)?;
    if options.minimum_packages == 0 || !(0.0..=1.0).contains(&options.minimum_coverage) {
        bail!(
            "minimum packages must be positive and minimum coverage must be between zero and one"
        );
    }
    let entries = catalog
        .entries
        .iter()
        .map(|entry| (entry.source_id, entry))
        .collect::<BTreeMap<_, _>>();
    let mut weights = BTreeMap::<u64, f64>::new();
    let mut source_diversity = BTreeMap::<String, BTreeSet<String>>::new();
    let mut counted_sources = 0;
    let mut reference_sources = 0;
    for source in &corpus.sources {
        if source.counts_toward_coverage {
            counted_sources += 1;
            let weight = 1.0 / source.selections.len() as f64;
            for source_id in &source.selections {
                *weights.entry(*source_id).or_default() += weight;
                source_diversity
                    .entry(source.family.clone())
                    .or_default()
                    .insert(source.id.clone());
            }
        } else {
            reference_sources += 1;
        }
    }
    let total_weight = weights.values().sum::<f64>();
    let mut mandatory = BTreeSet::new();
    for source in &corpus.sources {
        if !source.counts_toward_coverage {
            mandatory.extend(&source.selections);
        }
    }
    for requested in &options.mandatory_packages {
        let entry = catalog.entries.iter().find(|entry| {
            entry.candidate_package == *requested
                || entry.curated_package.as_deref() == Some(requested)
        });
        let Some(entry) = entry else {
            bail!("mandatory package `{requested}` is not a discovery catalog identity");
        };
        mandatory.insert(entry.source_id);
    }
    let mut ranked = weights.keys().copied().collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        weights[right]
            .partial_cmp(&weights[left])
            .expect("finite weights")
            .then_with(|| left.cmp(right))
    });
    let mut selected = mandatory.clone();
    let mut covered = selected
        .iter()
        .filter_map(|id| weights.get(id))
        .sum::<f64>();
    for source_id in ranked {
        if selected.len() >= options.minimum_packages
            && covered + f64::EPSILON >= total_weight * options.minimum_coverage
        {
            break;
        }
        if selected.insert(source_id) {
            covered += weights.get(&source_id).copied().unwrap_or_default();
        }
    }
    let packages = selected
        .iter()
        .filter_map(|source_id| entries.get(source_id).map(|entry| (*source_id, *entry)))
        .map(|(source_id, entry)| CohortPackage {
            source_id,
            candidate_package: entry.candidate_package.clone(),
            curated_package: entry.curated_package.clone(),
            name: entry.name.clone(),
            category: entry.category.clone(),
            observed_game_targets: entry.observed_game_targets.clone(),
            inclusion_reason: inclusion_reason(source_id, &mandatory, &weights),
        })
        .collect::<Vec<_>>();
    let deferred_candidates = catalog
        .entries
        .iter()
        .filter(|entry| {
            weights.contains_key(&entry.source_id) && !selected.contains(&entry.source_id)
        })
        .map(|entry| DeferredCandidate {
            source_id: entry.source_id,
            candidate_package: entry.candidate_package.clone(),
            name: entry.name.clone(),
            category: entry.category.clone(),
        })
        .collect();
    let report = ProductACoverageReport {
        schema: PRODUCT_A_COHORT_SCHEMA,
        corpus_captured_on: corpus.captured_on.clone(),
        catalog_source_revision: catalog.source.revision.clone(),
        counted_sources,
        reference_sources,
        unique_selected_candidates: weights.len(),
        weighted_selection_total: total_weight,
        weighted_selection_covered: covered,
        coverage: if total_weight == 0.0 { 0.0 } else { covered / total_weight },
        selected_packages: packages.len(),
        source_diversity: source_diversity.into_iter().map(|(family, ids)| (family, ids.len())).collect(),
        deferred_candidates,
        limitations: vec![
            "Coverage is a deterministic measurement of this frozen corpus, not a claim about every player or every mod combination.".to_owned(),
            "Each counted stack receives one total vote distributed across its distinct packages, preventing large component-heavy stacks from dominating smaller stacks.".to_owned(),
            "IEPM reference manifests enforce required current-profile inclusion but do not add popularity weight.".to_owned(),
            "Cohort inclusion is a PA-4 work queue, not an artifact, compatibility, or public support claim.".to_owned(),
        ],
    };
    let cohort = ProductACohort {
        schema: PRODUCT_A_COHORT_SCHEMA,
        corpus_captured_on: corpus.captured_on.clone(),
        catalog_source_revision: catalog.source.revision.clone(),
        minimum_packages: options.minimum_packages,
        minimum_coverage_basis_points: (options.minimum_coverage * 10_000.0).round() as u32,
        packages,
    };
    Ok(CohortBuild { cohort, report })
}

pub fn load_selection_corpus(path: &Path) -> Result<SelectionCorpus> {
    let source =
        fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?;
    serde_json::from_str(&source).with_context(|| format!("could not parse {}", path.display()))
}

pub fn write_selection_corpus(corpus: &SelectionCorpus, output: &Path) -> Result<()> {
    write_json(corpus, output)
}

pub fn write_cohort_build(
    build: &CohortBuild,
    cohort_output: &Path,
    report_output: &Path,
) -> Result<()> {
    write_json(&build.cohort, cohort_output)?;
    write_json(&build.report, report_output)
}

fn validate_selection_corpus(corpus: &SelectionCorpus, catalog: &DiscoveryCatalog) -> Result<()> {
    if corpus.schema != SELECTION_CORPUS_SCHEMA
        || corpus.catalog_source_revision != catalog.source.revision
    {
        bail!("selection corpus schema or catalog revision does not match the loaded catalog");
    }
    require_date(&corpus.captured_on)?;
    let known = catalog
        .entries
        .iter()
        .map(|entry| entry.source_id)
        .collect::<BTreeSet<_>>();
    let mut source_ids = BTreeSet::new();
    let mut deduplication_keys = BTreeSet::new();
    for source in &corpus.sources {
        if source.id.is_empty()
            || !source_ids.insert(&source.id)
            || !deduplication_keys.insert(&source.deduplication_key)
        {
            bail!("selection corpus has duplicate or empty source identity");
        }
        if source.selections.windows(2).any(|pair| pair[0] >= pair[1]) {
            bail!(
                "selection corpus source {} has unsorted selections",
                source.id
            );
        }
        if source.selections.iter().any(|id| !known.contains(id)) {
            bail!(
                "selection corpus source {} references an unknown catalog ID",
                source.id
            );
        }
    }
    Ok(())
}

fn inclusion_reason(
    source_id: u64,
    mandatory: &BTreeSet<u64>,
    weights: &BTreeMap<u64, f64>,
) -> String {
    if mandatory.contains(&source_id) && weights.contains_key(&source_id) {
        "required current reference profile and observed in counted corpus".to_owned()
    } else if mandatory.contains(&source_id) {
        "required current reference profile".to_owned()
    } else {
        "meets frozen weighted-selection coverage threshold".to_owned()
    }
}

#[derive(Debug, Deserialize)]
struct ForgePreset {
    id: String,
    name: String,
    keys: Vec<String>,
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
}

/// Map an external selection token to one catalog identity without claiming
/// that it is a release alias. Exact curated/candidate/TP2/display matches
/// win. A component-style token may use a unique, longest TP2 prefix; this is
/// only an observation about the source list's own component naming pattern.
fn map_catalog_token(catalog: &DiscoveryCatalog, token: &str) -> Option<u64> {
    let normalized = normalize(token);
    let exact = catalog
        .entries
        .iter()
        .filter(|entry| {
            normalize(&entry.tp2_name) == normalized
                || normalize(&entry.candidate_package) == normalized
                || normalize(&entry.name) == normalized
                || entry
                    .curated_package
                    .as_ref()
                    .is_some_and(|package| normalize(package) == normalized)
        })
        .map(|entry| entry.source_id)
        .collect::<BTreeSet<_>>();
    if exact.len() == 1 {
        return exact.into_iter().next();
    }
    let longest = catalog
        .entries
        .iter()
        .filter_map(|entry| {
            let prefix = normalize(&entry.tp2_name);
            (normalized.starts_with(&prefix) && prefix.len() >= 3)
                .then_some((prefix.len(), entry.source_id))
        })
        .max_by_key(|(length, _)| *length)?;
    let ties = catalog
        .entries
        .iter()
        .filter(|entry| {
            let prefix = normalize(&entry.tp2_name);
            normalized.starts_with(&prefix) && prefix.len() == longest.0
        })
        .count();
    (ties == 1).then_some(longest.1)
}

fn slug(value: &str) -> String {
    let value = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    value.trim_matches('-').to_owned()
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn require_date(value: &str) -> Result<()> {
    if value.len() != 10
        || !value.as_bytes().iter().enumerate().all(|(index, byte)| {
            (index == 4 || index == 7 && *byte == b'-')
                || (index != 4 && index != 7 && byte.is_ascii_digit())
        })
    {
        bail!("captured_on must be an ISO calendar date (YYYY-MM-DD)");
    }
    Ok(())
}

fn write_json<T: Serialize>(value: &T, output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        output,
        format!("{}\n", serde_json::to_string_pretty(value)?),
    )
    .with_context(|| format!("could not write {}", output.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{DiscoveryCatalogEntry, DiscoveryCatalogSource, DiscoveryIdentityStatus};

    fn catalog() -> DiscoveryCatalog {
        DiscoveryCatalog {
            schema: 1,
            source: DiscoveryCatalogSource {
                kind: "infinity-mod-forge".to_owned(),
                repository: "https://example.test".to_owned(),
                revision: "pinned".to_owned(),
                license: "MIT".to_owned(),
                input: "data/mods-index.json".to_owned(),
                input_sha256: "a".repeat(64),
            },
            entries: vec![
                entry(1, "one", Some("one")),
                entry(2, "two", None),
                entry(3, "three", None),
            ],
        }
    }
    fn entry(
        source_id: u64,
        candidate_package: &str,
        curated_package: Option<&str>,
    ) -> DiscoveryCatalogEntry {
        DiscoveryCatalogEntry {
            source_id,
            tp2_name: candidate_package.to_owned(),
            name: candidate_package.to_owned(),
            author: None,
            summary: None,
            category: "QUEST".to_owned(),
            tags: vec![],
            narrative_phases: vec![],
            observed_game_targets: vec!["bg2ee".to_owned()],
            source_game_predicate_observed: false,
            component_count: 1,
            language_indices: BTreeMap::new(),
            homepage: None,
            github: None,
            source_version: None,
            source_install_order: 0,
            candidate_package: candidate_package.to_owned(),
            identity_status: if curated_package.is_some() {
                DiscoveryIdentityStatus::MatchedCurated
            } else {
                DiscoveryIdentityStatus::Candidate
            },
            curated_package: curated_package.map(str::to_owned),
        }
    }
    #[test]
    fn equal_stack_weighting_prevents_a_large_stack_from_dominating() {
        let corpus = SelectionCorpus {
            schema: 1,
            captured_on: "2026-09-21".to_owned(),
            catalog_source_revision: "pinned".to_owned(),
            sources: vec![
                SelectionCorpusSource {
                    id: "small".to_owned(),
                    family: "guide".to_owned(),
                    label: "small".to_owned(),
                    input: "x".to_owned(),
                    input_sha256: "a".repeat(64),
                    deduplication_key: "small".to_owned(),
                    counts_toward_coverage: true,
                    selections: vec![1],
                    unmapped_tokens: vec![],
                },
                SelectionCorpusSource {
                    id: "large".to_owned(),
                    family: "guide".to_owned(),
                    label: "large".to_owned(),
                    input: "x".to_owned(),
                    input_sha256: "b".repeat(64),
                    deduplication_key: "large".to_owned(),
                    counts_toward_coverage: true,
                    selections: vec![2, 3],
                    unmapped_tokens: vec![],
                },
            ],
        };
        let build = build_product_a_cohort(
            &catalog(),
            &corpus,
            &CohortBuildOptions {
                minimum_packages: 1,
                minimum_coverage: 0.5,
                mandatory_packages: vec![],
            },
        )
        .expect("valid cohort");
        assert_eq!(build.cohort.packages.len(), 1);
        assert_eq!(build.cohort.packages[0].source_id, 1);
    }
}

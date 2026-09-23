pub mod catalog;
pub mod cohort;
pub mod health;

use anyhow::{Context, Result, bail};
use iepm_core::{Installer, PackageRecord, Registry, Release};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;
use zip::ZipArchive;

const MAX_INSPECTED_PACKAGE_BYTES: u64 = 8 * 1024 * 1024 * 1024;
const MAX_INSPECTED_TP2_BYTES: u64 = 64 * 1024 * 1024;

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
    /// Literal `GAME_IS` clauses observed in the TP2. They are source facts,
    /// not a normalized compatibility verdict.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub game_predicates: Vec<String>,
    /// Literal component relationship clauses. They are intentionally left as
    /// source text because resolving package identity from a TP2 token is an
    /// ecosystem-curation question, not a parser fact.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub component_predicates: Vec<Tp2Predicate>,
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tp2Predicate {
    pub kind: String,
    pub expression: String,
}

/// Structural observations of one local directory or ZIP-family package. This
/// is suitable for an opaque local release: it identifies potential WeiDU TP2
/// source without treating any installer code as safe to run.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PackageObservation {
    pub schema: u32,
    pub source_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    pub weidu_package: bool,
    pub tp2_files: Vec<ObservedTp2File>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ObservedTp2File {
    pub path: String,
    #[serde(flatten)]
    pub observation: Tp2Observation,
}

/// Inspect a local extracted package directory or a ZIP-family archive without
/// execution or extraction. ZIP entry paths, declared sizes, and symlinks are
/// checked before any TP2 content is read.
pub fn inspect_package(path: &Path) -> Result<PackageObservation> {
    if path.is_dir() {
        return inspect_directory(path);
    }
    if path.is_file() {
        return inspect_zip_package(path);
    }
    bail!("package path does not exist: {}", path.display())
}

/// Render a deliberately incomplete, author-reviewable schema-2 package
/// record from mechanical observations. Callers must explicitly provide games
/// and phase because those are not safely inferred from TP2 text.
pub fn derive_bgmod_candidate(
    package: &str,
    release_id: &str,
    version: &str,
    games: &[String],
    phase: &str,
    observation: &PackageObservation,
) -> Result<String> {
    if !valid_package_id(package) {
        bail!("{package} is not a valid package ID");
    }
    if release_id.trim().is_empty() || version.trim().is_empty() || games.is_empty() {
        bail!("release ID, version, and at least one explicitly supplied game are required");
    }
    let allowed_phases = [
        "preprocess",
        "bgee",
        "eet-import",
        "eet",
        "eet-end",
        "post-eet-end",
    ];
    if !allowed_phases.contains(&phase) {
        bail!("{phase} is not a valid IEPM install phase");
    }

    let mut component_ids = BTreeSet::new();
    let mut installers = Vec::new();
    let mut components = Vec::new();
    for file in &observation.tp2_files {
        installers.push(CandidateInstaller {
            tp2: file.path.clone(),
            languages: file.observation.languages.clone(),
        });
        let stem = slug_component(
            Path::new(&file.path)
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("tp2"),
        );
        for (ordinal, component) in file.observation.components.iter().enumerate() {
            let Some(id) = candidate_component_id(&stem, ordinal, component) else {
                continue;
            };
            let id = unique_component_id(id, &mut component_ids);
            components.push(CandidateComponent {
                id,
                weidu: CandidateWeiDUComponent {
                    tp2: file.path.clone(),
                    label: component.label.clone(),
                    number: component.number,
                },
            });
        }
    }

    let mut claims = vec![CandidateClaim {
        claim: "TP2 paths, VERSION text, language declaration order, component selectors, and literal predicates in this candidate were mechanically derived from the selected local package; compatibility, ordering, installer behavior, and verification require separate evidence.".to_owned(),
        provenance: "derived".to_owned(),
    }];
    if let Some(sha256) = &observation.sha256 {
        claims.push(CandidateClaim {
            claim: format!("The inspected local archive has SHA-256 {sha256}. A source URL must be curated before it becomes an IEPM artifact location."),
            provenance: "derived".to_owned(),
        });
    }

    let candidate = CandidatePackage {
        schema: 2,
        package: package.to_owned(),
        releases: vec![CandidateRelease {
            release_id: release_id.to_owned(),
            version: version.to_owned(),
            compatibility: CandidateCompatibility {
                games: games.to_vec(),
            },
            install: CandidateInstall {
                phase: phase.to_owned(),
            },
            provenance: "derived".to_owned(),
            claims,
            installers,
            components,
        }],
    };
    Ok(serde_yaml::to_string(&candidate)?)
}

#[derive(Serialize)]
struct CandidatePackage {
    schema: u32,
    package: String,
    releases: Vec<CandidateRelease>,
}

#[derive(Serialize)]
struct CandidateRelease {
    release_id: String,
    version: String,
    compatibility: CandidateCompatibility,
    install: CandidateInstall,
    provenance: String,
    claims: Vec<CandidateClaim>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    installers: Vec<CandidateInstaller>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    components: Vec<CandidateComponent>,
}

#[derive(Serialize)]
struct CandidateCompatibility {
    games: Vec<String>,
}

#[derive(Serialize)]
struct CandidateInstall {
    phase: String,
}

#[derive(Serialize)]
struct CandidateClaim {
    claim: String,
    provenance: String,
}

#[derive(Serialize)]
struct CandidateInstaller {
    tp2: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    languages: Vec<Tp2Language>,
}

#[derive(Serialize)]
struct CandidateComponent {
    id: String,
    weidu: CandidateWeiDUComponent,
}

#[derive(Serialize)]
struct CandidateWeiDUComponent {
    tp2: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<u32>,
}

/// Read the small, stable structural surface of a TP2 file. The parser is
/// intentionally line-oriented; complex WeiDU source remains WeiDU's domain.
pub fn inspect_tp2(source: &str) -> Tp2Observation {
    let mut observation = Tp2Observation {
        schema: 1,
        version: None,
        languages: Vec::new(),
        components: Vec::new(),
        game_predicates: Vec::new(),
        component_predicates: Vec::new(),
        warnings: Vec::new(),
    };
    let mut current: Option<Tp2Component> = None;
    let mut awaiting_language_name = false;
    let mut control_flow_depth = 0_usize;
    let mut awaits_control_flow_begin = false;
    let mut in_block_comment = false;
    let mut in_inlined_file = false;
    let mut multiline_component_title: Option<(char, String)> = None;

    for raw_line in source.trim_start_matches('\u{feff}').lines() {
        // WeiDU can embed a complete BAF/D script between file delimiters.
        // Its `BEGIN dialogue_name` is not a TP2 component declaration.
        if raw_line.trim_start().starts_with("<<<<<<<<") {
            in_inlined_file = true;
            continue;
        }
        if in_inlined_file {
            if raw_line.trim_start().starts_with(">>>>>>>>") {
                in_inlined_file = false;
            }
            continue;
        }
        // A quoted WeiDU component title may span lines. Until its closing
        // delimiter, declaration-looking words are title text, not TP2 code.
        if let Some((delimiter, title)) = multiline_component_title.as_mut() {
            let fragment = raw_line.trim();
            if let Some(end) = fragment.find(*delimiter) {
                title.push('\n');
                title.push_str(&fragment[..end]);
                if let Some(component) = current.as_mut() {
                    component.begin = Some(title.clone());
                }
                multiline_component_title = None;
            } else {
                title.push('\n');
                title.push_str(fragment);
            }
            continue;
        }
        let raw_line = strip_tp2_comments(raw_line, &mut in_block_comment);
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if awaiting_language_name {
            if let Some(name) = value_at_start(line) {
                observation.languages.push(Tp2Language {
                    id: observation.languages.len() as u32,
                    name,
                });
                awaiting_language_name = false;
                continue;
            }
            awaiting_language_name = false;
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
            } else {
                awaiting_language_name = true;
            }
            continue;
        }
        // `ACTION_IF` and `ACTION_FOR_EACH` may put their control-flow BEGIN
        // at column zero. Those are not install components, even though the
        // same token starts a real top-level component declaration.
        if starts_keyword(line, "ACTION_IF") || starts_keyword(line, "ACTION_FOR_EACH") {
            if line.contains("BEGIN") {
                control_flow_depth += 1;
            } else {
                awaits_control_flow_begin = true;
            }
            continue;
        }
        // WeiDU often writes `ACTION_IF condition` followed by `THEN BEGIN`
        // on the next line. If the pending branch is not consumed here, the
        // next real component BEGIN is incorrectly swallowed as action code.
        if awaits_control_flow_begin
            && (starts_keyword(line, "THEN") || starts_keyword(line, "ELSE"))
            && find_keyword(line, "BEGIN").is_some()
        {
            control_flow_depth += 1;
            awaits_control_flow_begin = false;
            continue;
        }
        if awaits_control_flow_begin
            && starts_keyword(line, "BEGIN")
            && !line["BEGIN".len()..].trim_start().starts_with('@')
        {
            // Some real TP2s place the entire branch on one line:
            // `BEGIN OUTER_SPRINT value ~x~ END ELSE`. Leaving that BEGIN
            // open hides the next top-level component from this inventory.
            if find_keyword(line, "END").is_none() {
                control_flow_depth += 1;
            }
            awaits_control_flow_begin = line.trim_end().ends_with("ELSE");
            continue;
        }
        // A labeled, explicitly numbered WeiDU component begins with
        // `BEGIN @...`. This is never the bare BEGIN of an action block.
        // Recover top-level scope here when a preceding complex ACTION_IF
        // made this intentionally shallow scanner overcount control depth.
        // BG1 Romantic Encounters v16 otherwise loses its first 24 selectors.
        if starts_keyword(line, "BEGIN")
            && line["BEGIN".len()..].trim_start().starts_with('@')
            && control_flow_depth > 0
        {
            control_flow_depth = 0;
            awaits_control_flow_begin = false;
        }
        if control_flow_depth > 0 && starts_keyword(line, "END") {
            control_flow_depth -= 1;
            if find_keyword(line, "BEGIN").is_some() {
                control_flow_depth += 1;
            }
            continue;
        }
        // Indentation does not distinguish a real WeiDU component from an
        // action block: author TP2s contain both indented declarations and
        // indented control-flow BEGINs. A component has a title argument;
        // action `BEGIN` is bare. The tracked action context adds another
        // guard for controls whose expression precedes BEGIN on its line.
        if starts_keyword(line, "BEGIN") && control_flow_depth == 0 {
            let title_source = line["BEGIN".len()..].trim_start();
            // An inline action branch such as `BEGIN SET value = 1 END`
            // is not an unquoted component title.
            if starts_keyword(title_source, "SET") && find_keyword(title_source, "END").is_some() {
                continue;
            }
            let pending_title = match title_source.chars().next() {
                Some(delimiter @ ('~' | '"'))
                    if !title_source[delimiter.len_utf8()..].contains(delimiter) =>
                {
                    Some((delimiter, title_source[delimiter.len_utf8()..].to_owned()))
                }
                _ => None,
            };
            if pending_title.is_some() || value_after_keyword(line, "BEGIN").is_some() {
                if let Some(component) = current.take() {
                    observation.components.push(component);
                }
                current = Some(Tp2Component {
                    begin: value_after_keyword(line, "BEGIN"),
                    // A bare BEGIN receives WeiDU's declaration-order selector.
                    // Numeric text inside BEGIN is a title, not a selector.
                    number: number_after_keyword(line, "DESIGNATED")
                        .or(Some(observation.components.len() as u32)),
                    label: value_after_keyword(line, "LABEL"),
                });
                multiline_component_title = pending_title;
                continue;
            }
        }
        for (keyword, kind) in [
            ("GAME_IS", "game-is"),
            ("REQUIRE_COMPONENT", "require-component"),
            ("FORBID_COMPONENT", "forbid-component"),
        ] {
            if let Some(offset) = find_keyword(line, keyword) {
                let expression = line[offset + keyword.len()..].trim().to_owned();
                if keyword == "GAME_IS" {
                    observation.game_predicates.push(expression);
                } else {
                    observation.component_predicates.push(Tp2Predicate {
                        kind: kind.to_owned(),
                        expression,
                    });
                }
            }
        }
        if let Some(component) = current.as_mut() {
            if let Some(number) = number_after_keyword(line, "DESIGNATED") {
                component.number = Some(number);
            }
            if component.label.is_none() {
                component.label = value_after_keyword(line, "LABEL");
            }
        }
    }
    if let Some(component) = current {
        observation.components.push(component);
    }
    observation.game_predicates.sort();
    observation.game_predicates.dedup();
    observation.component_predicates.sort();
    observation.component_predicates.dedup();
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

// Keep spaces in place of comments so column-zero component declarations are
// still distinguishable from indented action BEGIN blocks.
fn strip_tp2_comments(line: &str, in_block_comment: &mut bool) -> String {
    let mut output = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    let mut quote = None;
    while let Some(character) = chars.next() {
        let next = chars.peek().copied();
        if *in_block_comment {
            if character == '*' && next == Some('/') {
                output.push(' ');
                output.push(' ');
                chars.next();
                *in_block_comment = false;
            } else {
                output.push(' ');
            }
            continue;
        }
        if let Some(delimiter) = quote {
            output.push(character);
            if character == delimiter {
                quote = None;
            }
            continue;
        }
        if character == '~' || character == '"' {
            quote = Some(character);
            output.push(character);
        } else if character == '/' && next == Some('*') {
            output.push(' ');
            output.push(' ');
            chars.next();
            *in_block_comment = true;
        } else if character == '/' && next == Some('/') {
            break;
        } else {
            output.push(character);
        }
    }
    output
}

fn inspect_directory(path: &Path) -> Result<PackageObservation> {
    let mut files = WalkDir::new(path)
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|entry| entry.file_type().is_file() && is_tp2_path(entry.path()))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    files.sort();
    let mut tp2_files = Vec::new();
    for file in files {
        let relative = file
            .strip_prefix(path)
            .expect("walk result is rooted in package")
            .to_string_lossy()
            .replace('\\', "/");
        let source = std::fs::read(&file)
            .with_context(|| format!("could not read TP2 source {}", file.display()))?;
        if source.len() as u64 > MAX_INSPECTED_TP2_BYTES {
            bail!("TP2 source exceeds inspection limit: {}", file.display());
        }
        tp2_files.push(ObservedTp2File {
            path: relative,
            observation: inspect_tp2(&String::from_utf8_lossy(&source)),
        });
    }
    Ok(PackageObservation {
        schema: 1,
        source_kind: "directory".to_owned(),
        sha256: None,
        weidu_package: !tp2_files.is_empty(),
        tp2_files,
        warnings: Vec::new(),
    })
}

fn inspect_zip_package(path: &Path) -> Result<PackageObservation> {
    let sha256 = sha256_file(path)?;
    let file = File::open(path).with_context(|| format!("could not open {}", path.display()))?;
    let mut archive = ZipArchive::new(file)
        .with_context(|| format!("{} is not a valid ZIP-family archive", path.display()))?;
    let mut total_size = 0_u64;
    let mut tp2_files = Vec::new();
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| anyhow::anyhow!("archive entry has unsafe path: {}", entry.name()))?
            .to_owned();
        total_size = total_size
            .checked_add(entry.size())
            .ok_or_else(|| anyhow::anyhow!("archive exceeds inspection size limit"))?;
        if total_size > MAX_INSPECTED_PACKAGE_BYTES {
            bail!(
                "archive exceeds {} byte inspection limit",
                MAX_INSPECTED_PACKAGE_BYTES
            );
        }
        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170000 == 0o120000)
        {
            bail!("archive contains a symbolic-link entry: {}", entry.name());
        }
        if entry.is_dir() || !is_tp2_path(&enclosed) {
            continue;
        }
        if entry.size() > MAX_INSPECTED_TP2_BYTES {
            bail!("TP2 entry exceeds inspection limit: {}", entry.name());
        }
        let mut bytes = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut bytes)?;
        tp2_files.push(ObservedTp2File {
            path: enclosed.to_string_lossy().replace('\\', "/"),
            observation: inspect_tp2(&String::from_utf8_lossy(&bytes)),
        });
    }
    tp2_files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(PackageObservation {
        schema: 1,
        source_kind: "zip".to_owned(),
        sha256: Some(sha256),
        weidu_package: !tp2_files.is_empty(),
        tp2_files,
        warnings: Vec::new(),
    })
}

fn is_tp2_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("tp2"))
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file =
        File::open(path).with_context(|| format!("could not open {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let bytes = file.read(&mut buffer)?;
        if bytes == 0 {
            break;
        }
        hasher.update(&buffer[..bytes]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn candidate_component_id(stem: &str, ordinal: usize, component: &Tp2Component) -> Option<String> {
    component
        .number
        .map(|number| format!("derived-{stem}-{number}"))
        .or_else(|| {
            component
                .label
                .as_deref()
                .map(|label| format!("derived-{stem}-{}", slug_component(label)))
        })
        .or_else(|| {
            component
                .begin
                .as_deref()
                .map(|_| format!("derived-{stem}-ordinal-{}", ordinal + 1))
        })
}

fn unique_component_id(candidate: String, seen: &mut BTreeSet<String>) -> String {
    if seen.insert(candidate.clone()) {
        return candidate;
    }
    let mut suffix = 2_u32;
    loop {
        let value = format!("{candidate}-{suffix}");
        if seen.insert(value.clone()) {
            return value;
        }
        suffix += 1;
    }
}

fn slug_component(value: &str) -> String {
    let mut output = String::new();
    let mut separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            separator = false;
        } else if !separator && !output.is_empty() {
            output.push('-');
            separator = true;
        }
    }
    output.trim_matches('-').to_owned()
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
    /// Complete inert structural inventory for this one installer. It exposes
    /// coverage gaps without inventing stable component IDs for TP2 entries
    /// that have not yet received a curator-reviewed user-intent name.
    pub component_catalog: Tp2ComponentCatalogAudit,
    pub status: Tp2ReviewStatus,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<Tp2Finding>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Tp2ComponentCatalogAudit {
    pub observed_components: usize,
    pub mapped_registry_components: Vec<String>,
    pub unmapped_observed_components: Vec<Tp2Component>,
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
    let component_catalog = audit_component_catalog(release, installer, observed);
    if !component_catalog.unmapped_observed_components.is_empty() {
        findings.push(Tp2Finding {
            kind: "component-unmapped".to_owned(),
            message: format!(
                "{} observed TP2 component(s) have no registry mapping",
                component_catalog.unmapped_observed_components.len()
            ),
        });
    }

    Ok(Tp2Review {
        schema: 1,
        package: package.package.clone(),
        release_id: release.id().to_owned(),
        installer_tp2: installer.tp2.clone(),
        tp2_version: observed.version.clone(),
        registry_version: release.version.clone(),
        component_catalog,
        status: if findings.is_empty() {
            Tp2ReviewStatus::Match
        } else {
            Tp2ReviewStatus::Drift
        },
        findings,
    })
}

fn audit_component_catalog(
    release: &Release,
    installer: &Installer,
    observed: &Tp2Observation,
) -> Tp2ComponentCatalogAudit {
    let mapped = release
        .components
        .iter()
        .filter(|component| {
            component
                .weidu
                .as_ref()
                .is_some_and(|selector| same_tp2(&selector.tp2, &installer.tp2))
        })
        .filter(|component| {
            let selector = component.weidu.as_ref().expect("filtered above");
            observed.components.iter().any(|candidate| {
                selector_matches_observed(selector.number, selector.label.as_deref(), candidate)
            })
        })
        .map(|component| component.id.clone())
        .collect::<Vec<_>>();
    let unmapped_observed_components = observed
        .components
        .iter()
        .filter(|observed_component| {
            !release.components.iter().any(|component| {
                component.weidu.as_ref().is_some_and(|selector| {
                    same_tp2(&selector.tp2, &installer.tp2)
                        && selector_matches_observed(
                            selector.number,
                            selector.label.as_deref(),
                            observed_component,
                        )
                })
            })
        })
        .cloned()
        .collect();
    Tp2ComponentCatalogAudit {
        observed_components: observed.components.len(),
        mapped_registry_components: mapped,
        unmapped_observed_components,
    }
}

fn selector_matches_observed(
    number: Option<u32>,
    label: Option<&str>,
    observed: &Tp2Component,
) -> bool {
    match (number, label) {
        (Some(number), Some(label)) => {
            observed.number == Some(number) && observed.label.as_deref() == Some(label)
        }
        (Some(number), None) => observed.number == Some(number),
        (None, Some(label)) => observed.label.as_deref() == Some(label),
        (None, None) => false,
    }
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
    value_at_start(&line[offset..])
}

fn value_at_start(value: &str) -> Option<String> {
    let value = value.trim_start();
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
        assert!(observation.game_predicates.is_empty());
        assert!(observation.component_predicates.is_empty());
    }

    #[test]
    fn inspection_ignores_block_commented_components_and_predicates() {
        let observation = inspect_tp2(
            r#"
VERSION ~1.0~
/*
BEGIN @10 DESIGNATED 10 LABEL ~DISABLED~
REQUIRE_PREDICATE GAME_IS ~iwd~
*/
BEGIN @20 DESIGNATED 20 LABEL ~ACTIVE~ // /* no comment begins here
PRINT ~literal /* text */~
/* BEGIN @30 DESIGNATED 30 LABEL ~ALSO-DISABLED~ */
BEGIN @40 DESIGNATED 40 LABEL ~SECOND~
"#,
        );

        assert_eq!(observation.components.len(), 2);
        assert_eq!(observation.components[0].label.as_deref(), Some("ACTIVE"));
        assert_eq!(observation.components[1].label.as_deref(), Some("SECOND"));
        assert!(observation.game_predicates.is_empty());
    }

    #[test]
    fn inspection_handles_multiline_languages_and_implicit_component_numbers() {
        let observation = inspect_tp2(
            r#"
LANGUAGE
  ~English~ ~mod/tra~ ~mod/tra/english.tra~
BEGIN @100
BEGIN @200
BEGIN ~1~
DESIGNATED 20
"#,
        );

        assert_eq!(
            observation.languages,
            vec![Tp2Language {
                id: 0,
                name: "English".to_owned(),
            }]
        );
        assert_eq!(
            observation
                .components
                .iter()
                .map(|component| component.number)
                .collect::<Vec<_>>(),
            vec![Some(0), Some(1), Some(20)]
        );
    }

    #[test]
    fn inspection_separates_multiline_component_title_from_prior_selector() {
        let observation = inspect_tp2(
            r#"
BEGIN @0
DESIGNATED 0
LABEL ~main~
BEGIN ~Option 1 :

Make optional items less powerful.
~
DESIGNATED 10
SUBCOMPONENT ~Nerfs~
LABEL ~optional-nerfs~
REQUIRE_PREDICATE (MOD_IS_INSTALLED ~example.tp2~ ~0~) ~Install main first~
"#,
        );

        assert_eq!(observation.components.len(), 2);
        assert_eq!(observation.components[0].number, Some(0));
        assert_eq!(observation.components[0].label.as_deref(), Some("main"));
        assert_eq!(observation.components[1].number, Some(10));
        assert_eq!(
            observation.components[1].label.as_deref(),
            Some("optional-nerfs")
        );
        assert!(
            observation.components[1]
                .begin
                .as_deref()
                .is_some_and(|title| title.contains("Make optional items less powerful."))
        );
    }

    #[test]
    fn inspection_ignores_indented_control_flow_begin_blocks() {
        let observation = inspect_tp2(
            r#"
BEGIN @100
  ACTION_IF TRUE BEGIN
    PRINT ~nested action~
  END
BEGIN @200
"#,
        );

        assert_eq!(
            observation
                .components
                .iter()
                .map(|component| component.begin.as_deref())
                .collect::<Vec<_>>(),
            vec![Some("@100"), Some("@200")]
        );
    }

    #[test]
    fn inspection_ignores_unindented_action_control_flow_begin_blocks() {
        let observation = inspect_tp2(
            r#"
BEGIN @100 DESIGNATED 0
ACTION_FOR_EACH resource IN file1 file2 BEGIN
  COPY ~%resource%~ ~override~
END
ACTION_FOR_EACH creature IN
  foo
  bar
BEGIN
  COPY ~%creature%~ ~override~
END
BEGIN @200 DESIGNATED 10
"#,
        );

        assert_eq!(
            observation
                .components
                .iter()
                .map(|component| component.number)
                .collect::<Vec<_>>(),
            vec![Some(0), Some(10)]
        );
    }

    #[test]
    fn inspection_keeps_components_after_then_begin_action_blocks() {
        let observation = inspect_tp2(
            r#"
BEGIN ~True Paladin~
ACTION_IF FILE_EXISTS_IN_GAME ~bdcaelar.cre~
THEN BEGIN
  COPY_EXISTING ~c02siren.cre~ ~override~
END
BEGIN ~Cavalier~
ACTION_IF FILE_EXISTS_IN_GAME ~bdcaelar.cre~
THEN BEGIN
  COPY_EXISTING ~c02siren.cre~ ~override~
END
BEGIN ~Inquisitor~
"#,
        );

        assert_eq!(
            observation
                .components
                .iter()
                .map(|component| component.begin.as_deref())
                .collect::<Vec<_>>(),
            vec![Some("True Paladin"), Some("Cavalier"), Some("Inquisitor")]
        );
    }

    #[test]
    fn inspection_keeps_components_after_single_line_if_branches() {
        let observation = inspect_tp2(
            r#"
BEGIN @900
LABEL ~RANDOM_ENCOUNTERS~
ACTION_IF MOD_IS_INSTALLED ~other.tp2~ ~4~
BEGIN OUTER_SPRINT path ~other.baf~ END ELSE
ACTION_IF FILE_EXISTS ~dir.ids~
BEGIN OUTER_SPRINT path ~ee.baf~ END ELSE
BEGIN OUTER_SPRINT path ~classic.baf~ END
BEGIN @1000
LABEL ~MINOR_RESTORATIONS~
BEGIN ~~
DEPRECATED @1100
BEGIN @1200
LABEL ~BETTER_ITEM_IMPORT~
"#,
        );

        assert_eq!(
            observation
                .components
                .iter()
                .map(|component| (component.number, component.label.as_deref()))
                .collect::<Vec<_>>(),
            vec![
                (Some(0), Some("RANDOM_ENCOUNTERS")),
                (Some(1), Some("MINOR_RESTORATIONS")),
                (Some(2), None),
                (Some(3), Some("BETTER_ITEM_IMPORT")),
            ]
        );
    }

    #[test]
    fn inspection_recovers_explicit_components_after_complex_action_scope() {
        let observation = inspect_tp2(
            r#"
ACTION_IF GAME_IS ~eet~ THEN BEGIN
  OUTER_SPRINT path ~eet~
BEGIN @99181
DESIGNATED 100
LABEL bg1re-required-teen_skip
BEGIN @11 DESIGNATED 1
LABEL bg1re_bardolans_briefing
"#,
        );
        assert_eq!(observation.components.len(), 2);
        assert_eq!(observation.components[0].number, Some(100));
        assert_eq!(
            observation.components[1].label.as_deref(),
            Some("bg1re_bardolans_briefing")
        );

        let pending_branch = inspect_tp2(
            r#"
ACTION_IF GAME_IS ~bgee~
BEGIN @99181
DESIGNATED 100
LABEL bg1re-required-teen_skip
"#,
        );
        assert_eq!(pending_branch.components.len(), 1);
        assert_eq!(pending_branch.components[0].number, Some(100));
    }

    #[test]
    fn inspection_does_not_count_inline_set_branch_as_component() {
        let observation = inspect_tp2(
            r#"
BEGIN @1
PATCH_IF TRUE
BEGIN SET "RESULT" = 1 END
BEGIN @2
"#,
        );
        assert_eq!(observation.components.len(), 2);
        assert_eq!(observation.components[1].begin.as_deref(), Some("@2"));
    }

    #[test]
    fn inspection_ignores_embedded_dialogue_begin_blocks() {
        let observation = inspect_tp2(
            r#"
BEGIN @1 DESIGNATED 0 LABEL ~FIRST~
<<<<<<<< .../npc-dialogue.d
BEGIN C#NPC
IF ~~ THEN greeting
SAY ~Hello~
END
>>>>>>>>
BEGIN @2 DESIGNATED 1 LABEL ~SECOND~
"#,
        );
        assert_eq!(observation.components.len(), 2);
        assert_eq!(observation.components[0].label.as_deref(), Some("FIRST"));
        assert_eq!(observation.components[1].label.as_deref(), Some("SECOND"));
    }

    #[test]
    fn inspection_keeps_components_after_indented_begin_action_blocks() {
        let observation = inspect_tp2(
            r#"
ALWAYS
  ACTION_IF GAME_IS ~bgee~
  BEGIN
    OUTER_SET is_eet = 0
  END
END
BEGIN @1
DESIGNATED 0
LABEL ~FIRST~
  ACTION_IF GAME_IS ~eet~
  BEGIN
    COPY_EXISTING ~example.cre~ ~override~
  END
BEGIN @2
DESIGNATED 1
LABEL ~SECOND~
"#,
        );

        assert_eq!(
            observation
                .components
                .iter()
                .map(|component| component.label.as_deref())
                .collect::<Vec<_>>(),
            vec![Some("FIRST"), Some("SECOND")]
        );
    }

    #[test]
    fn inspection_keeps_indented_component_declarations() {
        let observation = inspect_tp2(
            r#"
  BEGIN ~Core NPC~
  INCLUDE ~mod/core.tpa~
BEGIN ~Portrait A~
FORCED_SUBCOMPONENT ~Choose portrait~
"#,
        );

        assert_eq!(observation.components.len(), 2);
        assert_eq!(observation.components[0].begin.as_deref(), Some("Core NPC"));
        assert_eq!(observation.components[0].number, Some(0));
        assert_eq!(observation.components[1].number, Some(1));
    }

    #[test]
    fn inspection_does_not_promote_bare_nested_begin_to_component() {
        let observation = inspect_tp2(
            r#"
BEGIN @0 DESIGNATED 10
  PATCH_IF some_condition BEGIN
    BEGIN
      WRITE_BYTE 0 1
    END
  END
BEGIN @1 DESIGNATED 20
"#,
        );

        assert_eq!(
            observation
                .components
                .iter()
                .map(|component| component.number)
                .collect::<Vec<_>>(),
            vec![Some(10), Some(20)]
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
        assert_eq!(review.component_catalog.observed_components, 1);
        assert_eq!(
            review.component_catalog.mapped_registry_components,
            vec!["enable-feature"]
        );
        assert!(
            review
                .component_catalog
                .unmapped_observed_components
                .is_empty()
        );
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
        assert_eq!(review.component_catalog.observed_components, 2);
        assert!(
            review
                .component_catalog
                .mapped_registry_components
                .is_empty()
        );
        assert_eq!(
            review.component_catalog.unmapped_observed_components.len(),
            2
        );
    }

    #[test]
    fn review_tp2_marks_extra_observed_component_as_drift() {
        let observed = inspect_tp2(
            r#"
VERSION ~1.2.3~
LANGUAGE ~English~
BEGIN @100 DESIGNATED 10 LABEL ~SAMPLE-ENABLE~
BEGIN @101 DESIGNATED 11 LABEL ~SAMPLE-EXTRA~
"#,
        );
        let review = review_tp2(&registry(), "sample-mod", "sample-release", None, &observed)
            .expect("review succeeds");

        assert_eq!(review.status, Tp2ReviewStatus::Drift);
        assert_eq!(
            review.component_catalog.unmapped_observed_components.len(),
            1
        );
        assert!(
            review
                .findings
                .iter()
                .any(|finding| finding.kind == "component-unmapped")
        );
    }

    #[test]
    fn inspection_keeps_literal_predicates_as_source_facts() {
        let observation = inspect_tp2(
            r#"
GAME_IS ~bgee~ OR ~eet~
REQUIRE_COMPONENT ~other-mod/setup-other.tp2~ ~0~
FORBID_COMPONENT ~other-mod/setup-other.tp2~ ~10~
BEGIN @1 DESIGNATED 0 LABEL ~SAMPLE-CORE~
"#,
        );

        assert_eq!(observation.game_predicates, vec!["~bgee~ OR ~eet~"]);
        assert_eq!(
            observation.component_predicates,
            vec![
                Tp2Predicate {
                    kind: "forbid-component".to_owned(),
                    expression: "~other-mod/setup-other.tp2~ ~10~".to_owned(),
                },
                Tp2Predicate {
                    kind: "require-component".to_owned(),
                    expression: "~other-mod/setup-other.tp2~ ~0~".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn candidate_requires_explicit_nonmechanical_inputs() {
        let observation = PackageObservation {
            schema: 1,
            source_kind: "directory".to_owned(),
            sha256: None,
            weidu_package: true,
            tp2_files: vec![ObservedTp2File {
                path: "Sample/setup-sample.tp2".to_owned(),
                observation: inspect_tp2(
                    "VERSION ~1.2.3~\nLANGUAGE ~English~\nBEGIN @1 DESIGNATED 0 LABEL ~SAMPLE-CORE~",
                ),
            }],
            warnings: vec![],
        };
        let yaml = derive_bgmod_candidate(
            "sample-mod",
            "sample-release",
            "1.2.3",
            &["bg2ee".to_owned()],
            "eet",
            &observation,
        )
        .expect("candidate succeeds");

        assert!(yaml.contains("provenance: derived"));
        assert!(yaml.contains("derived-setup-sample-0"));
        assert!(yaml.contains("SAMPLE-CORE"));
        assert!(
            derive_bgmod_candidate(
                "sample-mod",
                "sample-release",
                "1.2.3",
                &[],
                "eet",
                &observation,
            )
            .is_err()
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
        for relationship in &release.relationships {
            for selected_component in &relationship.when.selected_components {
                if !components.contains(selected_component) {
                    bail!(
                        "{} {} relationship to {} names unknown selected component {}",
                        record.package,
                        release.id(),
                        relationship.package,
                        selected_component
                    );
                }
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

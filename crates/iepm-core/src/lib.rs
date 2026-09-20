use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Human intent. Schema 2 uses `environments`; `game` remains readable only so
/// existing schema-1 manifests can be migrated by the resolver.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game: Option<GameTarget>,
    #[serde(default)]
    pub environments: BTreeMap<String, GameEnvironment>,
    pub mods: Vec<RequestedMod>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameTarget {
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameEnvironment {
    pub target: String,
    /// The target identity after the environment's `eet-import` phase. This
    /// represents EET's documented BG2EE-to-EET workspace transformation.
    /// Before and during that phase, `target` remains the active game.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_eet_import: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<GameFingerprint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<ArtifactPlatform>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Components already present in the clean, named environment. These are
    /// preflight assertions, not execution nodes: A5 must verify them rather
    /// than try to install them a second time.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub baseline: Vec<WeiDULogEntry>,
}

/// The profile is versioned so fingerprint algorithms can change without
/// making old lockfiles ambiguous. It should cover selected core files and
/// DLC/layout facts, not an entire game tree.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GameFingerprint {
    pub profile: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RequestedMod {
    Package(String),
    Selection {
        package: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        version: Option<String>,
        #[serde(default)]
        components: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        environment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        language: Option<String>,
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        installer_inputs: BTreeMap<String, String>,
    },
}

impl RequestedMod {
    pub fn package(&self) -> &str {
        match self {
            Self::Package(package) => package,
            Self::Selection { package, .. } => package,
        }
    }

    pub fn components(&self) -> &[String] {
        match self {
            Self::Package(_) => &[],
            Self::Selection { components, .. } => components,
        }
    }

    pub fn version(&self) -> Option<&str> {
        match self {
            Self::Package(_) => None,
            Self::Selection { version, .. } => version.as_deref(),
        }
    }

    pub fn environment(&self) -> Option<&str> {
        match self {
            Self::Package(_) => None,
            Self::Selection { environment, .. } => environment.as_deref(),
        }
    }

    pub fn language(&self) -> Option<&str> {
        match self {
            Self::Package(_) => None,
            Self::Selection { language, .. } => language.as_deref(),
        }
    }

    pub fn installer_inputs(&self) -> &BTreeMap<String, String> {
        match self {
            Self::Package(_) => empty_inputs(),
            Self::Selection {
                installer_inputs, ..
            } => installer_inputs,
        }
    }
}

fn empty_inputs() -> &'static BTreeMap<String, String> {
    static EMPTY: std::sync::OnceLock<BTreeMap<String, String>> = std::sync::OnceLock::new();
    EMPTY.get_or_init(BTreeMap::new)
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackageRecord {
    pub schema: u32,
    pub package: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub lineage: Vec<Lineage>,
    pub releases: Vec<Release>,
}

/// Informational project history. Unlike aliases, lineage is never used to
/// silently substitute one package for another during resolution.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lineage {
    pub package: String,
    pub relation: LineageRelation,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LineageRelation {
    Predecessor,
    Fork,
    Continuation,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    /// Human-facing upstream version. It is not assumed to be SemVer.
    pub version: String,
    /// Stable opaque release identity. Legacy records use `version` as the ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    /// Optional SemVer projection used only when range solving is requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semver: Option<String>,
    #[serde(default)]
    pub artifact: Option<Artifact>,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    #[serde(default)]
    pub materialization: Materialization,
    pub compatibility: Compatibility,
    pub install: Install,
    pub provenance: Provenance,
    #[serde(default)]
    pub claims: Vec<ClaimProvenance>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
    #[serde(default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub installers: Vec<Installer>,
}

impl Release {
    pub fn id(&self) -> &str {
        self.release_id.as_deref().unwrap_or(&self.version)
    }

    pub fn artifacts(&self) -> impl Iterator<Item = &Artifact> {
        self.artifacts.iter().chain(self.artifact.iter())
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Materialization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_root: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Artifact {
    /// Preferred location. Content identity is always `sha256`, not this URL.
    pub url: String,
    pub sha256: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mirrors: Vec<String>,
    #[serde(default)]
    pub format: ArchiveFormat,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<ArtifactPlatform>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub architectures: Vec<ArtifactArchitecture>,
}

impl Artifact {
    pub fn locations(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.url.as_str()).chain(self.mirrors.iter().map(String::as_str))
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ArchiveFormat {
    #[default]
    Zip,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactPlatform {
    Windows,
    Linux,
    Macos,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactArchitecture {
    X86,
    X86_64,
    Aarch64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Compatibility {
    pub games: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Install {
    /// A conventional label. Explicit graph edges remain authoritative.
    pub phase: Phase,
    #[serde(default)]
    pub before: Vec<String>,
    #[serde(default)]
    pub after: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Phase {
    Preprocess,
    Bgee,
    EetImport,
    Eet,
    EetEnd,
    PostEetEnd,
}

impl GameEnvironment {
    /// The game identity against which a package executes. EET itself runs
    /// against BG2EE during `eet-import`; later phases run against the
    /// transformed EET tree when one is declared.
    pub fn active_target(&self, phase: Phase) -> &str {
        if phase > Phase::EetImport {
            self.after_eet_import.as_deref().unwrap_or(&self.target)
        } else {
            &self.target
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependency {
    pub package: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Relationship {
    pub kind: RelationshipKind,
    pub package: String,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub components: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub when: RelationshipCondition,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RelationshipKind {
    Requires,
    Optional,
    Recommends,
    Conflicts,
    Before,
    After,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RelationshipCondition {
    #[serde(default)]
    pub games: Vec<String>,
}

impl RelationshipCondition {
    pub fn matches_game(&self, game: &str) -> bool {
        self.games.is_empty() || self.games.iter().any(|candidate| candidate == game)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    pub id: String,
    #[serde(default)]
    pub default_selected: bool,
    #[serde(default)]
    pub weidu: Option<WeiDUComponent>,
    #[serde(default)]
    pub provides: Vec<Capability>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeiDUComponent {
    pub tp2: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcomponent: Option<String>,
}

/// A single, ordered entry expected in an environment's WeiDU baseline log.
/// It intentionally records WeiDU's numeric language and component identity,
/// rather than a display name that may change between releases.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct WeiDULogEntry {
    pub tp2: String,
    pub language: u32,
    pub component: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Installer {
    pub tp2: String,
    /// Relative launcher path inside the materialized package, such as
    /// `setup-example.exe`. It is required for an executable A5 plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    /// A package may ship a setup launcher, or be invoked by the pinned shared
    /// WeiDU toolchain against its TP2. The latter is common for extracted
    /// packages and avoids fabricating a launcher that is not present.
    #[serde(default)]
    pub launcher: InstallerLauncher,
    #[serde(default)]
    pub languages: Vec<InstallerLanguage>,
    #[serde(default)]
    pub inputs: Vec<InstallerInput>,
    /// Typed extra arguments required by this installer. This is deliberately
    /// not a shell template: only literal tokens and named environment
    /// bindings are representable.
    #[serde(default)]
    pub arguments: Vec<InstallerArgument>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InstallerLauncher {
    #[default]
    Bundled,
    Toolchain,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum InstallerArgument {
    Literal { value: String },
    EnvironmentInput { input: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstallerLanguage {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstallerInput {
    pub name: String,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Capability {
    pub name: String,
    #[serde(default)]
    pub exclusive: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Provenance {
    Declared,
    Derived,
    Verified,
    Community,
    Unverified,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClaimProvenance {
    pub claim: String,
    pub provenance: Provenance,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lockfile {
    pub schema: u32,
    pub environments: BTreeMap<String, GameEnvironment>,
    pub registry_revision: String,
    pub toolchain: Toolchain,
    pub packages: Vec<LockedPackage>,
    pub execution: Vec<ExecutionNode>,
    #[serde(default)]
    pub execution_readiness: ExecutionReadiness,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocking_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

/// `analysis-only` lockfiles are useful for review and diagnostics but are not
/// eligible for A5 execution.
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionReadiness {
    Executable,
    #[default]
    AnalysisOnly,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Toolchain {
    pub iepm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weidu: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LockedPackage {
    pub package: String,
    pub release_id: String,
    pub version: String,
    pub environment: String,
    pub phase: Phase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<Artifact>,
    #[serde(default, skip_serializing_if = "is_default_materialization")]
    pub materialization: Materialization,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub installers: Vec<Installer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<LockedComponent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub installer_inputs: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
    pub provenance: Provenance,
}

fn is_default_materialization(value: &Materialization) -> bool {
    value.source_root.is_none() && value.include.is_empty()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LockedComponent {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weidu: Option<WeiDUComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionNode {
    pub id: String,
    pub package: String,
    pub environment: String,
    pub phase: Phase,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub predecessors: Vec<String>,
}

pub type Registry = BTreeMap<String, PackageRecord>;

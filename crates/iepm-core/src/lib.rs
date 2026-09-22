use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Human intent. Schema 2 uses `environments`; `game` remains readable only so
/// existing schema-1 manifests can be migrated by the resolver.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    /// Game-install locale used by the shared WeiDU executor, such as
    /// `en_US`. This is distinct from a mod's selected WeiDU language index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    /// An exact release asset whose content can be hashed and selected, but
    /// which A4 must not prepare or execute generically. A package-specific
    /// materialization fixture is required before it can become runnable.
    Executable,
    /// A Windows WinRAR self-extracting archive. This is deliberately a
    /// separate format from a generic executable: A4 invokes its documented
    /// extraction mode into the artifact cache, never as a game installer.
    WindowsRarSfx,
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
    /// Stable component IDs in the same package that must also be selected.
    /// This is deliberately package-local; cross-package requirements belong
    /// on the release relationship that owns both package identities.
    #[serde(default)]
    pub requires: Vec<String>,
    /// Selector and component-level evidence can differ from a release's
    /// overall provenance. This is particularly important for mechanically
    /// observed TP2 selectors combined with separately curated semantics.
    #[serde(default)]
    pub claims: Vec<ClaimProvenance>,
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
    /// Some release-specific selectors dispatch nested WeiDU work rather than
    /// writing their own line to WeiDU.log. They remain executable selectors,
    /// but another selected recording component must supply the receipt.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub non_recording: bool,
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

/// The small, user-facing support vocabulary. Detailed provenance remains in
/// registry claims and verification records; the UI should not expose that
/// complexity as a collection of competing badges.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PublicSupportStatus {
    Verified,
    Supported,
    Untested,
    Incompatible,
}

impl PublicSupportStatus {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Verified => "Verified",
            Self::Supported => "Supported",
            Self::Untested => "Untested",
            Self::Incompatible => "Incompatible",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationScope {
    Component,
    Release,
    Relationship,
    Configuration,
}

/// Portable identity for one evidence-bearing run. It deliberately contains
/// content hashes and named environments, never machine-local paths.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationRecord {
    pub schema: u32,
    pub scope: VerificationScope,
    pub subject: String,
    pub identity: VerificationIdentity,
    #[serde(default)]
    pub checks: VerificationChecks,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub incompatibility: Vec<IncompatibilityEvidence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationIdentity {
    pub registry_revision: String,
    pub platform: String,
    pub weidu_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_sha256: Option<String>,
    pub environments: BTreeMap<String, GameEnvironmentEvidence>,
    pub packages: Vec<VerifiedSelection>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub execution_order: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameEnvironmentEvidence {
    pub target: String,
    pub fingerprint: GameFingerprint,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedSelection {
    pub package: String,
    pub release_id: String,
    pub environment: String,
    pub artifact_sha256: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
}

/// Direct observations used by the fixed support gates. A false value means
/// "not demonstrated by this record", not "known to be broken".
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationChecks {
    #[serde(default)]
    pub artifact_verified: bool,
    #[serde(default)]
    pub selector_known: bool,
    #[serde(default)]
    pub requirements_modeled: bool,
    #[serde(default)]
    pub component_catalog_complete: bool,
    #[serde(default)]
    pub all_components_classified: bool,
    #[serde(default)]
    pub core_components_supported: bool,
    #[serde(default)]
    pub relationship_evidence: RelationshipEvidence,
    #[serde(default)]
    pub selected_components_supported: bool,
    #[serde(default)]
    pub relationships_satisfied: bool,
    #[serde(default)]
    pub no_opaque_selections: bool,
    #[serde(default)]
    pub clean_disposable_install: bool,
    #[serde(default)]
    pub all_actions_completed: bool,
    #[serde(default)]
    pub requested_components_recorded: bool,
    #[serde(default)]
    pub no_components_skipped: bool,
    #[serde(default)]
    pub warnings_classified: bool,
    #[serde(default)]
    pub sealed: bool,
    #[serde(default)]
    pub eet_end_required: bool,
    #[serde(default)]
    pub eet_end_completed: bool,
    #[serde(default)]
    pub main_menu_smoke: bool,
    #[serde(default)]
    pub gameplay_smoke: bool,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RelationshipEvidence {
    #[default]
    None,
    Mechanical,
    AuthorDeclared,
    AutomatedFixture,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IncompatibilityEvidence {
    pub kind: IncompatibilityKind,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deterministic_failures: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum IncompatibilityKind {
    MechanicalImpossibility,
    ExclusiveCapability,
    AuthorDeclared,
    DeterministicFailure,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct VerificationAssessment {
    pub status: PublicSupportStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub incompatibility: Vec<String>,
}

impl VerificationRecord {
    /// Apply the measurable public support gates. This is intentionally a
    /// concrete decision table, not a configurable compatibility rule engine.
    pub fn assess(&self) -> VerificationAssessment {
        let mut identity_missing = self.identity.missing_for(self.scope);
        if self.schema != 1 {
            identity_missing.insert(0, "verification record schema must be 1".to_owned());
        }
        if self.subject.trim().is_empty() {
            identity_missing.push("subject is required".to_owned());
        }
        for reference in &self.evidence {
            require(
                &mut identity_missing,
                is_portable_reference(reference),
                "evidence references are repository-relative paths or durable URLs",
            );
        }
        for claim in &self.incompatibility {
            require(
                &mut identity_missing,
                !claim.summary.trim().is_empty(),
                "incompatibility evidence includes a summary",
            );
            for reference in &claim.references {
                require(
                    &mut identity_missing,
                    is_portable_reference(reference),
                    "incompatibility references are repository-relative paths or durable URLs",
                );
            }
        }

        let incompatible = self
            .incompatibility
            .iter()
            .filter(|item| item.is_conclusive())
            .map(|item| item.summary.clone())
            .collect::<Vec<_>>();
        if !incompatible.is_empty() && identity_missing.is_empty() {
            return VerificationAssessment {
                status: PublicSupportStatus::Incompatible,
                missing: Vec::new(),
                incompatibility: incompatible,
            };
        }
        if !incompatible.is_empty() {
            identity_missing.push(
                "conclusive incompatibility requires a complete portable identity".to_owned(),
            );
        }

        let (status, mut missing) = match self.scope {
            VerificationScope::Component => self.assess_component(),
            VerificationScope::Release => self.assess_release(),
            VerificationScope::Relationship => self.assess_relationship(),
            VerificationScope::Configuration => self.assess_configuration(),
        };
        let identity_is_complete = identity_missing.is_empty();
        missing.splice(0..0, identity_missing);

        VerificationAssessment {
            status: if identity_is_complete {
                status
            } else {
                PublicSupportStatus::Untested
            },
            missing,
            incompatibility: Vec::new(),
        }
    }

    fn assess_component(&self) -> (PublicSupportStatus, Vec<String>) {
        let mut missing = Vec::new();
        require(
            &mut missing,
            self.checks.artifact_verified,
            "artifact SHA-256 verified",
        );
        require(
            &mut missing,
            self.checks.selector_known,
            "component selector known",
        );
        require(
            &mut missing,
            self.checks.requirements_modeled,
            "requirements modeled",
        );
        self.require_install_receipt(&mut missing);
        require(
            &mut missing,
            self.checks.main_menu_smoke,
            "main-menu launch smoke passed",
        );
        status_from_support_gates(missing)
    }

    fn assess_release(&self) -> (PublicSupportStatus, Vec<String>) {
        let mut missing = Vec::new();
        require(
            &mut missing,
            self.checks.artifact_verified,
            "artifact SHA-256 verified",
        );
        require(
            &mut missing,
            self.checks.component_catalog_complete,
            "component catalog complete",
        );
        require(
            &mut missing,
            self.checks.all_components_classified,
            "every exposed component classified",
        );
        require(
            &mut missing,
            self.checks.core_components_supported,
            "default/core components supported",
        );
        require(
            &mut missing,
            self.checks.relationships_satisfied,
            "known release relationships modeled",
        );
        status_from_support_gates(missing)
    }

    fn assess_relationship(&self) -> (PublicSupportStatus, Vec<String>) {
        let mut missing = Vec::new();
        require(
            &mut missing,
            self.checks.artifact_verified,
            "all participating artifacts verified",
        );
        require(
            &mut missing,
            self.checks.relationship_evidence != RelationshipEvidence::None,
            "mechanical, author-declared, or automated relationship evidence recorded",
        );
        status_from_support_gates(missing)
    }

    fn assess_configuration(&self) -> (PublicSupportStatus, Vec<String>) {
        let mut supported_missing = Vec::new();
        require(
            &mut supported_missing,
            self.checks.artifact_verified,
            "all artifact SHA-256 values verified",
        );
        require(
            &mut supported_missing,
            self.checks.selected_components_supported,
            "every selected component supported",
        );
        require(
            &mut supported_missing,
            self.checks.relationships_satisfied,
            "dependencies, conflicts, and ordering satisfied",
        );
        require(
            &mut supported_missing,
            self.checks.no_opaque_selections,
            "no opaque or untested selections",
        );
        if !supported_missing.is_empty() {
            return (PublicSupportStatus::Untested, supported_missing);
        }

        let mut verified_missing = Vec::new();
        self.require_install_receipt(&mut verified_missing);
        require(
            &mut verified_missing,
            self.checks.sealed,
            "successful build sealed",
        );
        if self.checks.eet_end_required {
            require(
                &mut verified_missing,
                self.checks.eet_end_completed,
                "EET_End completed",
            );
        }
        require(
            &mut verified_missing,
            self.checks.main_menu_smoke,
            "main-menu launch smoke passed",
        );
        require(
            &mut verified_missing,
            self.checks.gameplay_smoke,
            "new-game or known-save smoke passed",
        );

        if verified_missing.is_empty() {
            (PublicSupportStatus::Verified, Vec::new())
        } else {
            (PublicSupportStatus::Supported, verified_missing)
        }
    }

    fn require_install_receipt(&self, missing: &mut Vec<String>) {
        require(
            missing,
            self.checks.clean_disposable_install,
            "clean disposable install completed",
        );
        require(
            missing,
            self.checks.all_actions_completed,
            "all planned actions completed",
        );
        require(
            missing,
            self.checks.requested_components_recorded,
            "all requested recording components appear in WeiDU.log",
        );
        require(
            missing,
            self.checks.no_components_skipped,
            "no requested component skipped",
        );
        require(
            missing,
            self.checks.warnings_classified,
            "all warnings classified",
        );
    }
}

impl VerificationIdentity {
    fn missing_for(&self, scope: VerificationScope) -> Vec<String> {
        let mut missing = Vec::new();
        require(
            &mut missing,
            !self.registry_revision.trim().is_empty() && self.registry_revision != "working-tree",
            "exact registry revision recorded",
        );
        require(
            &mut missing,
            !self.platform.trim().is_empty(),
            "platform recorded",
        );
        require(
            &mut missing,
            !self.weidu_version.trim().is_empty(),
            "WeiDU version recorded",
        );
        require(
            &mut missing,
            !self.environments.is_empty(),
            "at least one game environment fingerprint recorded",
        );
        for (name, environment) in &self.environments {
            require(
                &mut missing,
                !name.trim().is_empty(),
                "environment name is not empty",
            );
            require(
                &mut missing,
                !environment.target.trim().is_empty(),
                "environment target recorded",
            );
            require(
                &mut missing,
                !environment.fingerprint.profile.trim().is_empty(),
                "fingerprint profile recorded",
            );
            require(
                &mut missing,
                !environment.fingerprint.value.trim().is_empty(),
                "fingerprint value recorded",
            );
        }
        require(
            &mut missing,
            !self.packages.is_empty(),
            "at least one exact package selection recorded",
        );
        for package in &self.packages {
            require(
                &mut missing,
                !package.package.trim().is_empty(),
                "package identity recorded",
            );
            require(
                &mut missing,
                !package.release_id.trim().is_empty(),
                "release identity recorded",
            );
            require(
                &mut missing,
                !package.environment.trim().is_empty(),
                "package environment recorded",
            );
            require(
                &mut missing,
                is_sha256(&package.artifact_sha256),
                "artifact SHA-256 is a lowercase 64-digit hash",
            );
        }

        match scope {
            VerificationScope::Component => require(
                &mut missing,
                self.packages
                    .iter()
                    .any(|package| !package.components.is_empty()),
                "at least one stable component ID recorded",
            ),
            VerificationScope::Relationship => require(
                &mut missing,
                self.packages.len() >= 2,
                "at least two package selections recorded for the relationship",
            ),
            VerificationScope::Configuration => {
                require(
                    &mut missing,
                    self.lock_sha256.as_deref().is_some_and(is_sha256),
                    "portable lockfile SHA-256 recorded",
                );
                require(
                    &mut missing,
                    !self.execution_order.is_empty(),
                    "resolved execution order recorded",
                );
            }
            VerificationScope::Release => {}
        }
        missing
    }
}

impl IncompatibilityEvidence {
    fn is_conclusive(&self) -> bool {
        match self.kind {
            IncompatibilityKind::MechanicalImpossibility
            | IncompatibilityKind::ExclusiveCapability
            | IncompatibilityKind::AuthorDeclared => true,
            IncompatibilityKind::DeterministicFailure => {
                self.deterministic_failures.unwrap_or_default() >= 2
            }
        }
    }
}

fn require(missing: &mut Vec<String>, condition: bool, description: &str) {
    if !condition {
        missing.push(description.to_owned());
    }
}

fn status_from_support_gates(missing: Vec<String>) -> (PublicSupportStatus, Vec<String>) {
    if missing.is_empty() {
        (PublicSupportStatus::Supported, missing)
    } else {
        (PublicSupportStatus::Untested, missing)
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_portable_reference(value: &str) -> bool {
    let bytes = value.as_bytes();
    let windows_absolute = bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/');
    let unc = value.starts_with("\\\\");
    let unix_absolute = value.starts_with('/');
    !(value.trim().is_empty() || windows_absolute || unc || unix_absolute)
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

#[cfg(test)]
mod verification_tests {
    use super::*;

    fn configuration_record() -> VerificationRecord {
        VerificationRecord {
            schema: 1,
            scope: VerificationScope::Configuration,
            subject: "fixture-stack".to_owned(),
            identity: VerificationIdentity {
                registry_revision: "abc123".to_owned(),
                platform: "windows-x86-64".to_owned(),
                weidu_version: "25100".to_owned(),
                lock_sha256: Some("a".repeat(64)),
                environments: BTreeMap::from([(
                    "target".to_owned(),
                    GameEnvironmentEvidence {
                        target: "bg2ee".to_owned(),
                        fingerprint: GameFingerprint {
                            profile: "iepm-core-layout-v1".to_owned(),
                            value: "fixture-fingerprint".to_owned(),
                        },
                    },
                )]),
                packages: vec![VerifiedSelection {
                    package: "example".to_owned(),
                    release_id: "example-v1".to_owned(),
                    environment: "target".to_owned(),
                    artifact_sha256: "b".repeat(64),
                    components: vec!["core".to_owned()],
                }],
                execution_order: vec!["target::example".to_owned()],
            },
            checks: VerificationChecks {
                artifact_verified: true,
                selected_components_supported: true,
                relationships_satisfied: true,
                no_opaque_selections: true,
                clean_disposable_install: true,
                all_actions_completed: true,
                requested_components_recorded: true,
                no_components_skipped: true,
                warnings_classified: true,
                sealed: true,
                main_menu_smoke: true,
                gameplay_smoke: true,
                ..VerificationChecks::default()
            },
            incompatibility: Vec::new(),
            evidence: vec!["evidence/fixture/receipt.json".to_owned()],
        }
    }

    #[test]
    fn exact_configuration_with_every_gate_is_verified() {
        let assessment = configuration_record().assess();
        assert_eq!(assessment.status, PublicSupportStatus::Verified);
        assert!(assessment.missing.is_empty());
    }

    #[test]
    fn otherwise_supported_configuration_stays_supported_until_smoke_passes() {
        let mut record = configuration_record();
        record.checks.main_menu_smoke = false;
        record.checks.gameplay_smoke = false;

        let assessment = record.assess();
        assert_eq!(assessment.status, PublicSupportStatus::Supported);
        assert_eq!(assessment.missing.len(), 2);
    }

    #[test]
    fn missing_component_support_makes_configuration_untested() {
        let mut record = configuration_record();
        record.checks.selected_components_supported = false;

        let assessment = record.assess();
        assert_eq!(assessment.status, PublicSupportStatus::Untested);
        assert!(
            assessment
                .missing
                .contains(&"every selected component supported".to_owned())
        );
    }

    #[test]
    fn conclusive_incompatibility_overrides_success_checks() {
        let mut record = configuration_record();
        record.incompatibility.push(IncompatibilityEvidence {
            kind: IncompatibilityKind::MechanicalImpossibility,
            summary: "two selected components provide one exclusive capability".to_owned(),
            deterministic_failures: None,
            references: Vec::new(),
        });

        let assessment = record.assess();
        assert_eq!(assessment.status, PublicSupportStatus::Incompatible);
    }

    #[test]
    fn one_failure_is_not_enough_to_claim_incompatible() {
        let mut record = configuration_record();
        record.incompatibility.push(IncompatibilityEvidence {
            kind: IncompatibilityKind::DeterministicFailure,
            summary: "one clean run failed".to_owned(),
            deterministic_failures: Some(1),
            references: Vec::new(),
        });

        let assessment = record.assess();
        assert_eq!(assessment.status, PublicSupportStatus::Verified);
    }

    #[test]
    fn configuration_identity_requires_a_lock_hash() {
        let mut record = configuration_record();
        record.identity.lock_sha256 = None;

        let assessment = record.assess();
        assert_eq!(assessment.status, PublicSupportStatus::Untested);
        assert!(
            assessment
                .missing
                .contains(&"portable lockfile SHA-256 recorded".to_owned())
        );
    }

    #[test]
    fn component_support_requires_install_receipt_and_launch_smoke() {
        let mut record = configuration_record();
        record.scope = VerificationScope::Component;
        record.identity.lock_sha256 = None;
        record.identity.execution_order.clear();
        record.checks.selector_known = true;
        record.checks.requirements_modeled = true;

        assert_eq!(record.assess().status, PublicSupportStatus::Supported);

        record.checks.main_menu_smoke = false;
        assert_eq!(record.assess().status, PublicSupportStatus::Untested);
    }

    #[test]
    fn complete_release_catalog_is_supported() {
        let mut record = configuration_record();
        record.scope = VerificationScope::Release;
        record.identity.lock_sha256 = None;
        record.identity.execution_order.clear();
        record.checks.component_catalog_complete = true;
        record.checks.all_components_classified = true;
        record.checks.core_components_supported = true;

        assert_eq!(record.assess().status, PublicSupportStatus::Supported);
    }

    #[test]
    fn relationship_needs_two_exact_participants() {
        let mut record = configuration_record();
        record.scope = VerificationScope::Relationship;
        record.identity.lock_sha256 = None;
        record.identity.execution_order.clear();
        record.checks.relationship_evidence = RelationshipEvidence::AutomatedFixture;

        assert_eq!(record.assess().status, PublicSupportStatus::Untested);

        record.identity.packages.push(VerifiedSelection {
            package: "other".to_owned(),
            release_id: "other-v1".to_owned(),
            environment: "target".to_owned(),
            artifact_sha256: "c".repeat(64),
            components: vec!["core".to_owned()],
        });
        assert_eq!(record.assess().status, PublicSupportStatus::Supported);
    }

    #[test]
    fn two_deterministic_failures_are_incompatible() {
        let mut record = configuration_record();
        record.incompatibility.push(IncompatibilityEvidence {
            kind: IncompatibilityKind::DeterministicFailure,
            summary: "same clean failure repeated".to_owned(),
            deterministic_failures: Some(2),
            references: Vec::new(),
        });

        assert_eq!(record.assess().status, PublicSupportStatus::Incompatible);
    }

    #[test]
    fn incompatibility_is_not_published_without_exact_identity() {
        let mut record = configuration_record();
        record.identity.registry_revision = "working-tree".to_owned();
        record.incompatibility.push(IncompatibilityEvidence {
            kind: IncompatibilityKind::AuthorDeclared,
            summary: "author excludes this release pair".to_owned(),
            deterministic_failures: None,
            references: vec!["https://example.invalid/release-notes".to_owned()],
        });

        let assessment = record.assess();
        assert_eq!(assessment.status, PublicSupportStatus::Untested);
        assert!(assessment.incompatibility.is_empty());
    }

    #[test]
    fn working_tree_and_absolute_evidence_paths_are_not_portable_identity() {
        let mut record = configuration_record();
        record.identity.registry_revision = "working-tree".to_owned();
        record.evidence = vec![r"C:\local\receipt.json".to_owned()];

        let assessment = record.assess();
        assert_eq!(assessment.status, PublicSupportStatus::Untested);
        assert!(
            assessment
                .missing
                .contains(&"exact registry revision recorded".to_owned())
        );
        assert!(assessment.missing.contains(
            &"evidence references are repository-relative paths or durable URLs".to_owned()
        ));
    }
}

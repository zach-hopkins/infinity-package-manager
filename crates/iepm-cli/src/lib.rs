use anyhow::{Context, Result, bail};
use iepm_artifacts::ArtifactStore;
use iepm_core::{
    ArchiveFormat, Artifact, ArtifactArchitecture, ArtifactPlatform, ExecutionReadiness,
    GameFingerprint, Installer, InstallerArgument, InstallerLauncher, LockedPackage, Lockfile,
    Manifest, Registry, RequestedMod, WeiDUComponent,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

/// A deliberately small, versioned profile over files that identify an EE
/// installation's executable/key/DLC/log state. It is not a whole-tree hash.
pub const CORE_FINGERPRINT_PROFILE: &str = "iepm-core-layout-v1";
pub const DEFAULT_WEIDU_VERSION: &str = "25100";

/// The exact normal 64-bit Windows WeiDU v251 package used by the established
/// BGEE/BG2EE execution fixtures. The archive SHA-256 is GitHub's release-asset
/// digest for WeiDUorg/weidu v251.00, not a mutable "latest" reference.
const DEFAULT_WEIDU_WINDOWS_251_URL: &str =
    "https://github.com/WeiDUorg/weidu/releases/download/v251.00/WeiDU-Windows-251.zip";
const DEFAULT_WEIDU_WINDOWS_251_SHA256: &str =
    "a54c6198d6ebed8139793fcacb225500d563657c23fc775b840383f9750493d8";

#[derive(Debug, Clone, Serialize)]
pub struct ManagedWeiduToolchain {
    pub version: String,
    pub executable: PathBuf,
}

/// Prepare IEPM's currently supported shared Windows WeiDU toolchain under
/// the local store. Artifact acquisition stays hash-verified and separate
/// from game mutation; callers may then pass the returned path to A5.
pub fn prepare_default_weidu_toolchain(store: &Path) -> Result<ManagedWeiduToolchain> {
    let artifact = default_weidu_windows_251_artifact();
    let prepared = ArtifactStore::new(store.join("toolchains").join("weidu-25100"))?
        .prepare("weidu-25100", &artifact)?;
    let executable = find_weidu_executable(&prepared.extracted)?;
    Ok(ManagedWeiduToolchain {
        version: DEFAULT_WEIDU_VERSION.to_owned(),
        executable,
    })
}

/// Validate an explicitly user-selected local escape hatch when Windows security
/// prevents acquisition of the normal SHA-pinned release. This is intentionally
/// separate from the default route: the executable's version is checked, but
/// IEPM cannot establish its release-archive provenance from a loose file.
pub fn validate_local_weidu_override(executable: &Path) -> Result<ManagedWeiduToolchain> {
    if !executable.is_file() {
        bail!(
            "the selected WeiDU executable does not exist: {}",
            executable.display()
        );
    }
    let output = Command::new(executable)
        .arg("--version")
        .output()
        .with_context(|| format!("could not run {} --version", executable.display()))?;
    if !output.status.success() {
        bail!(
            "{} --version exited with {}",
            executable.display(),
            output.status
        );
    }
    let version_output = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if !version_output
        .lines()
        .any(|line| line.trim() == format!("WeiDU version {DEFAULT_WEIDU_VERSION}"))
    {
        bail!(
            "the selected executable is not the supported WeiDU v{}: {}",
            DEFAULT_WEIDU_VERSION,
            version_output.trim()
        );
    }
    Ok(ManagedWeiduToolchain {
        version: DEFAULT_WEIDU_VERSION.to_owned(),
        executable: executable.to_path_buf(),
    })
}

fn find_weidu_executable(extracted: &Path) -> Result<PathBuf> {
    let executables = WalkDir::new(extracted)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .eq_ignore_ascii_case("weidu.exe")
        })
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    match executables.as_slice() {
        [executable] => Ok(executable.clone()),
        [] => bail!(
            "verified WeiDU v251 archive did not contain weidu.exe: {}",
            extracted.display()
        ),
        _ => bail!(
            "verified WeiDU v251 archive contained more than one weidu.exe; refusing to guess: {}",
            extracted.display()
        ),
    }
}

fn default_weidu_windows_251_artifact() -> Artifact {
    Artifact {
        url: DEFAULT_WEIDU_WINDOWS_251_URL.to_owned(),
        sha256: DEFAULT_WEIDU_WINDOWS_251_SHA256.to_owned(),
        mirrors: Vec::new(),
        format: ArchiveFormat::Zip,
        platforms: vec![ArtifactPlatform::Windows],
        architectures: vec![ArtifactArchitecture::X86_64],
    }
}

/// Bind the familiar human game choices to every named manifest environment.
/// EET remains explicit in the manifest, but a normal UI need only collect a
/// clean BGEE/SoD source when it is actually required and a clean BG2EE source.
pub fn bind_standard_game_sources(
    manifest_path: &Path,
    bgee: Option<PathBuf>,
    bg2ee: Option<PathBuf>,
) -> Result<BTreeMap<String, PathBuf>> {
    let source = fs::read_to_string(manifest_path)
        .with_context(|| format!("could not read {}", manifest_path.display()))?;
    let manifest: Manifest = serde_yaml::from_str(&source)
        .with_context(|| format!("could not parse {}", manifest_path.display()))?;
    if manifest.environments.is_empty() {
        bail!("the selected manifest has no named schema-2 environments");
    }
    let mut bindings = BTreeMap::new();
    for (name, environment) in manifest.environments {
        let source = match environment.target.as_str() {
            "bgee" => bgee.as_ref(),
            "bg2ee" => bg2ee.as_ref(),
            other => bail!(
                "environment {name} targets {other}; the desktop MVP currently supports BGEE and BG2EE source selection"
            ),
        }
        .ok_or_else(|| {
            anyhow::anyhow!(
                "environment {name} requires a clean {} source folder",
                if environment.target == "bgee" {
                    "BG:EE / Siege of Dragonspear"
                } else {
                    "BG2:EE"
                }
            )
        })?;
        bindings.insert(name, source.clone());
    }
    Ok(bindings)
}

pub fn default_experience_name() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("mod-experience-{seconds}")
}

/// Explicit human intent accepted by `iepm add`. It deliberately does not
/// select a release, artifact, or component on the user's behalf.
pub struct AddSelection {
    pub package: String,
    pub environment: Option<String>,
    pub version: Option<String>,
    pub components: Vec<String>,
    pub language: Option<String>,
}

/// Add one canonical selection to a schema-2 manifest without writing it.
/// Callers must explicitly choose to persist the returned, normalized YAML.
pub fn append_manifest_selection(
    manifest: &Manifest,
    registry: &Registry,
    selection: AddSelection,
) -> Result<Manifest> {
    if manifest.schema != 2 {
        bail!(
            "iepm add only edits schema-2 manifests; migrate schema {} before changing it",
            manifest.schema
        );
    }
    let package = canonical_package_id(registry, &selection.package)?;
    let environment = match selection.environment {
        Some(environment) => {
            if !manifest.environments.contains_key(&environment) {
                bail!("manifest has no environment {environment}");
            }
            environment
        }
        None if manifest.environments.len() == 1 => manifest
            .environments
            .keys()
            .next()
            .expect("one environment has a key")
            .clone(),
        None => bail!(
            "manifest has multiple environments; pass --environment to make the target explicit"
        ),
    };
    let unique_components = selection.components.iter().collect::<BTreeSet<_>>();
    if unique_components.len() != selection.components.len() {
        bail!("a component was requested more than once");
    }
    let already_requested = manifest.mods.iter().any(|requested| {
        canonical_package_id(registry, requested.package())
            .is_ok_and(|existing| existing == package)
            && requested.environment().unwrap_or(&environment) == environment
    });
    if already_requested {
        bail!("{package} is already requested for environment {environment}");
    }

    let mut updated = manifest.clone();
    updated.mods.push(RequestedMod::Selection {
        package,
        version: selection.version,
        components: selection.components,
        environment: Some(environment),
        language: selection.language,
        installer_inputs: BTreeMap::new(),
    });
    Ok(updated)
}

/// Canonicalize only exact aliases recorded by the registry. Lineage is never
/// considered because it is history, not permission to substitute a package.
pub fn canonical_package_id(registry: &Registry, requested: &str) -> Result<String> {
    if registry.contains_key(requested) {
        return Ok(requested.to_owned());
    }
    registry
        .values()
        .find(|record| record.aliases.iter().any(|alias| alias == requested))
        .map(|record| record.package.clone())
        .ok_or_else(|| anyhow::anyhow!("package not found in registry: {requested}"))
}

pub struct LockfileVerification {
    pub structurally_valid: bool,
    pub execution_readiness: ExecutionReadiness,
    pub structural_problems: Vec<String>,
    pub blocking_reasons: Vec<String>,
    pub warnings: Vec<String>,
}

/// Verify the portable lockfile's internal references without fetching an
/// artifact, touching a cache, or mutating a game workspace.
pub fn verify_lockfile(lockfile: &Lockfile) -> LockfileVerification {
    let mut problems = Vec::new();
    if lockfile.schema != 3 {
        problems.push(format!(
            "schema {} is not the replayable schema-3 lockfile format",
            lockfile.schema
        ));
    }
    if lockfile.environments.is_empty() {
        problems.push("lockfile has no named environments".to_owned());
    }
    let mut package_keys = BTreeSet::new();
    for package in &lockfile.packages {
        let key = (package.environment.as_str(), package.package.as_str());
        if !lockfile.environments.contains_key(&package.environment) {
            problems.push(format!(
                "locked package {} references unknown environment {}",
                package.package, package.environment
            ));
        }
        if !package_keys.insert(key) {
            problems.push(format!(
                "lockfile repeats package {} in environment {}",
                package.package, package.environment
            ));
        }
    }
    let mut node_ids = BTreeSet::new();
    let mut execution_keys = BTreeSet::new();
    for node in &lockfile.execution {
        if !node_ids.insert(node.id.as_str()) {
            problems.push(format!("lockfile repeats execution node {}", node.id));
        }
        if !lockfile.environments.contains_key(&node.environment) {
            problems.push(format!(
                "execution node {} references unknown environment {}",
                node.id, node.environment
            ));
        }
        let key = (node.environment.as_str(), node.package.as_str());
        if !package_keys.contains(&key) {
            problems.push(format!(
                "execution node {} has no matching locked package {} in {}",
                node.id, node.package, node.environment
            ));
        }
        if !execution_keys.insert(key) {
            problems.push(format!(
                "lockfile repeats execution for package {} in environment {}",
                node.package, node.environment
            ));
        }
    }
    for key in package_keys.difference(&execution_keys) {
        problems.push(format!(
            "locked package {} in {} has no execution node",
            key.1, key.0
        ));
    }
    for node in &lockfile.execution {
        for predecessor in &node.predecessors {
            if predecessor == &node.id {
                problems.push(format!("execution node {} depends on itself", node.id));
            } else if !node_ids.contains(predecessor.as_str()) {
                problems.push(format!(
                    "execution node {} references missing predecessor {}",
                    node.id, predecessor
                ));
            }
        }
    }
    if lockfile.execution_readiness == ExecutionReadiness::AnalysisOnly
        && lockfile.blocking_reasons.is_empty()
    {
        problems.push("analysis-only lockfile has no causal blocking reasons".to_owned());
    }
    if lockfile.execution_readiness == ExecutionReadiness::Executable {
        if let Err(error) = render_plan(lockfile) {
            problems.push(format!("executable plan preflight failed: {error}"));
        }
    }
    LockfileVerification {
        structurally_valid: problems.is_empty(),
        execution_readiness: lockfile.execution_readiness,
        structural_problems: problems,
        blocking_reasons: lockfile.blocking_reasons.clone(),
        warnings: lockfile.warnings.clone(),
    }
}

pub fn render_verification(report: &LockfileVerification) -> String {
    let mut lines = vec![
        "IEPM lockfile verification (non-mutating)".to_owned(),
        format!(
            "Structure: {}",
            if report.structurally_valid {
                "valid"
            } else {
                "invalid"
            }
        ),
        format!(
            "Execution readiness: {}",
            match report.execution_readiness {
                ExecutionReadiness::Executable => "executable",
                ExecutionReadiness::AnalysisOnly => "analysis-only",
            }
        ),
        "Artifact bytes were not fetched or checked; use `iepm fetch` for verified preparation."
            .to_owned(),
    ];
    if !report.structural_problems.is_empty() {
        lines.push("Structural problems:".to_owned());
        lines.extend(
            report
                .structural_problems
                .iter()
                .map(|problem| format!("- {problem}")),
        );
    }
    if !report.blocking_reasons.is_empty() {
        lines.push("Execution blockers:".to_owned());
        lines.extend(
            report
                .blocking_reasons
                .iter()
                .map(|reason| format!("- {reason}")),
        );
    }
    if !report.warnings.is_empty() {
        lines.push("Warnings:".to_owned());
        lines.extend(report.warnings.iter().map(|warning| format!("- {warning}")));
    }
    format!("{}\n", lines.join("\n"))
}

const WORKSPACE_STATE_FILE: &str = ".iepm-workspace.json";
const BUILD_STATE_FILE: &str = ".iepm-build.json";

/// Inputs for the one-command personal build workflow. Machine-local paths
/// stay here and never enter the portable manifest or lockfile.
#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub registry: PathBuf,
    pub manifest: PathBuf,
    pub store: PathBuf,
    pub build: String,
    pub sources: BTreeMap<String, PathBuf>,
    pub weidu: PathBuf,
    pub weidu_version: String,
    pub registry_revision: String,
    pub confirm_disposable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildProgress {
    pub stage: String,
    pub message: String,
    /// Coarse overall build completion, intentionally bounded to observable
    /// lifecycle milestones and individual WeiDU actions rather than pretending
    /// that an external installer's internal work is measurable.
    pub percent: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actions: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BuildReport {
    pub sealed_build: PathBuf,
    pub lockfile: PathBuf,
    pub log_dir: PathBuf,
    pub actions: usize,
    pub warnings: Vec<String>,
    pub weidu_warnings: Vec<WeiduWarningReceipt>,
}

/// An audited WeiDU warning that did not prevent the requested components
/// from being recorded as installed. The full command output remains in the
/// ordinary action logs; this is the concise user-facing receipt.
#[derive(Debug, Clone, Serialize)]
pub struct WeiduWarningReceipt {
    pub action: usize,
    pub package: String,
    pub log: PathBuf,
    pub details: String,
}

pub struct PreparedBuild {
    pub manifest: Manifest,
    pub lockfile: Lockfile,
    pub plan: String,
    pub verification: String,
}

/// Perform the complete non-mutating part of `iepm build`: bind clean local
/// sources, derive or validate their fingerprints, resolve, verify, and render
/// the exact execution plan. The manifest on disk is never rewritten.
pub fn prepare_build(options: &BuildOptions) -> Result<PreparedBuild> {
    validate_store_name(&options.build, "build name")?;
    if !options.weidu.is_file() {
        bail!(
            "shared WeiDU executable does not exist: {}",
            options.weidu.display()
        );
    }
    let source = fs::read_to_string(&options.manifest)
        .with_context(|| format!("could not read {}", options.manifest.display()))?;
    let mut manifest: Manifest = serde_yaml::from_str(&source)
        .with_context(|| format!("could not parse {}", options.manifest.display()))?;
    if manifest.environments.is_empty() {
        bail!("iepm build requires a schema-2 manifest with named environments");
    }
    if options.sources.len() != manifest.environments.len()
        || !options.sources.keys().eq(manifest.environments.keys())
    {
        bail!("source bindings must name every and only every manifest environment");
    }
    for (name, environment) in &mut manifest.environments {
        let source = &options.sources[name];
        let actual = measure_workspace_fingerprint(source, environment.locale.as_deref())?;
        if let Some(expected) = &environment.fingerprint {
            if expected != &actual {
                bail!(
                    "clean source fingerprint mismatch for {name}: expected {}:{}, got {}:{}",
                    expected.profile,
                    expected.value,
                    actual.profile,
                    actual.value
                );
            }
        } else {
            environment.fingerprint = Some(actual);
        }
    }

    let registry = iepm_registry::load(&options.registry)?;
    let lockfile = iepm_resolver::resolve(
        &manifest,
        &registry,
        &options.registry_revision,
        iepm_core::Toolchain {
            iepm: env!("CARGO_PKG_VERSION").to_owned(),
            weidu: Some(options.weidu_version.clone()),
        },
    )?;
    let verification_report = verify_lockfile(&lockfile);
    let verification = render_verification(&verification_report);
    if !verification_report.structurally_valid {
        bail!("resolved lockfile failed structural verification:\n{verification}");
    }
    if lockfile.execution_readiness != ExecutionReadiness::Executable {
        let reasons = lockfile
            .blocking_reasons
            .iter()
            .map(|reason| format!("- {reason}"))
            .collect::<Vec<_>>()
            .join("\n");
        bail!("build is not executable:\n{reasons}");
    }
    let plan = render_plan(&lockfile)?;
    Ok(PreparedBuild {
        manifest,
        lockfile,
        plan,
        verification,
    })
}

/// Run the complete snapshot -> workspace -> resolve/install -> seal workflow.
/// Progress is deliberately a small concrete event shape shared by the CLI
/// and desktop shell; package-management decisions remain in Rust.
pub fn run_build_with_progress<F>(options: &BuildOptions, mut progress: F) -> Result<BuildReport>
where
    F: FnMut(BuildProgress),
{
    if !options.confirm_disposable {
        bail!("refusing to build without explicit disposable-workspace confirmation");
    }
    progress(BuildProgress {
        stage: "preflight".to_owned(),
        message: "Resolving and checking the requested build".to_owned(),
        percent: 2,
        action: None,
        total_actions: None,
    });
    let prepared = prepare_build(options)?;

    let workspace_root = options.store.join("workspaces").join(&options.build);
    let sealed_build = options.store.join("builds").join(&options.build);
    let log_dir = options.store.join("logs").join(&options.build);
    for (description, path) in [
        ("workspace", &workspace_root),
        ("sealed build", &sealed_build),
        ("log directory", &log_dir),
    ] {
        if path.exists() {
            bail!(
                "{description} already exists: {}; choose a fresh build name",
                path.display()
            );
        }
    }

    progress(BuildProgress {
        stage: "snapshot".to_owned(),
        message: "Preparing immutable clean-game snapshots".to_owned(),
        percent: 8,
        action: None,
        total_actions: None,
    });
    let mut snapshots = BTreeMap::new();
    for (name, environment) in &prepared.manifest.environments {
        let fingerprint = environment
            .fingerprint
            .as_ref()
            .expect("prepare_build filled every environment fingerprint");
        if let Some(snapshot) = reusable_source_snapshot(&options.store, fingerprint)? {
            snapshots.insert(name.clone(), snapshot);
        } else {
            let snapshot = import_source_snapshot(
                &options.sources[name],
                &options.store,
                name,
                environment.locale.as_deref(),
            )?;
            snapshots.insert(name.clone(), snapshot);
        }
    }

    progress(BuildProgress {
        stage: "workspace".to_owned(),
        message: "Creating fresh disposable workspaces".to_owned(),
        percent: 15,
        action: None,
        total_actions: None,
    });
    let workspace_root = create_disposable_workspaces(&snapshots, &options.store, &options.build)?;
    let workspaces = prepared
        .manifest
        .environments
        .keys()
        .map(|name| (name.clone(), workspace_root.join(name)))
        .collect::<BTreeMap<_, _>>();

    fs::create_dir_all(&log_dir)
        .with_context(|| format!("could not create {}", log_dir.display()))?;
    let lockfile_path = log_dir.join("modpack.lock.json");
    fs::write(
        &lockfile_path,
        format!("{}\n", serde_json::to_string_pretty(&prepared.lockfile)?),
    )?;
    fs::write(
        log_dir.join("effective-manifest.yaml"),
        serde_yaml::to_string(&prepared.manifest)?,
    )?;
    fs::write(log_dir.join("execution-plan.txt"), &prepared.plan)?;
    fs::write(
        log_dir.join("preflight-verification.txt"),
        &prepared.verification,
    )?;

    let execution = execute_with_progress(
        &prepared.lockfile,
        &ExecuteOptions {
            cache: options.store.join("cache"),
            weidu: options.weidu.clone(),
            workspaces,
            log_dir: log_dir.clone(),
            confirm_disposable: true,
        },
        &mut progress,
    )?;

    progress(BuildProgress {
        stage: "seal".to_owned(),
        message: "Sealing the completed build and its receipts".to_owned(),
        percent: 94,
        action: None,
        total_actions: Some(execution.actions),
    });
    let sealed_build =
        seal_successful_build(&workspace_root, &options.store, &options.build, &log_dir)?;
    let report = BuildReport {
        sealed_build,
        lockfile: lockfile_path,
        log_dir,
        actions: execution.actions,
        warnings: prepared.lockfile.warnings,
        weidu_warnings: execution.weidu_warnings,
    };
    progress(BuildProgress {
        stage: "complete".to_owned(),
        message: format!("Build is ready at {}", report.sealed_build.display()),
        percent: 100,
        action: Some(report.actions),
        total_actions: Some(report.actions),
    });
    Ok(report)
}

pub fn run_build(options: &BuildOptions) -> Result<BuildReport> {
    run_build_with_progress(options, |_| {})
}

fn reusable_source_snapshot(
    store: &Path,
    fingerprint: &GameFingerprint,
) -> Result<Option<PathBuf>> {
    let sources = store.join("sources");
    if !sources.is_dir() {
        return Ok(None);
    }
    let mut candidates = fs::read_dir(&sources)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.path().join(&fingerprint.value))
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    candidates.sort();
    if let Some(candidate) = candidates.into_iter().next() {
        let state = read_workspace_state(&candidate)?;
        expect_workspace_state(&state, "source-snapshot", "sealed", &candidate)?;
        let stored: GameFingerprint = serde_json::from_value(
            state
                .get("fingerprint")
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("source snapshot has no fingerprint"))?,
        )?;
        if &stored != fingerprint {
            bail!(
                "stored snapshot fingerprint does not match its path: {}",
                candidate.display()
            );
        }
        return Ok(Some(candidate));
    }
    Ok(None)
}

/// Import a full local game copy as an IEPM-managed source snapshot. IEPM
/// treats this copy as immutable: it may be cloned, but it can never be bound
/// to A5 execution as a writable workspace.
pub fn import_source_snapshot(
    source: &Path,
    store: &Path,
    name: &str,
    locale: Option<&str>,
) -> Result<PathBuf> {
    validate_store_name(name, "snapshot name")?;
    if !source.is_dir() {
        bail!(
            "source snapshot input is not a directory: {}",
            source.display()
        );
    }
    let source = source.canonicalize().map_err(|error| {
        anyhow::anyhow!(
            "could not canonicalize source {}: {error}",
            source.display()
        )
    })?;
    let fingerprint = measure_workspace_fingerprint(&source, locale)?;
    let destination = projected_canonical_path(store)?
        .join("sources")
        .join(name)
        .join(&fingerprint.value);
    if destination.starts_with(&source) {
        bail!(
            "source snapshot destination must not be inside its source tree: {}",
            destination.display()
        );
    }
    if destination.exists() {
        bail!(
            "source snapshot already exists: {}; use its immutable copy rather than overwriting it",
            destination.display()
        );
    }
    copy_tree(&source, &destination)?;
    write_workspace_state(
        &destination,
        "source-snapshot",
        "sealed",
        Some(&fingerprint),
    )?;
    Ok(destination)
}

/// Create a new full-copy workspace set from explicitly named source snapshots.
/// The returned build root is local state; no path enters a manifest or lockfile.
pub fn create_disposable_workspaces(
    snapshots: &BTreeMap<String, PathBuf>,
    store: &Path,
    build: &str,
) -> Result<PathBuf> {
    validate_store_name(build, "build name")?;
    if snapshots.is_empty() {
        bail!("at least one named source snapshot is required");
    }
    let root = store.join("workspaces").join(build);
    if root.exists() {
        bail!(
            "workspace build already exists: {}; create a fresh named build instead",
            root.display()
        );
    }
    let mut source_fingerprints = BTreeMap::new();
    for (environment, snapshot) in snapshots {
        validate_store_name(environment, "environment name")?;
        let state = read_workspace_state(snapshot)?;
        expect_workspace_state(&state, "source-snapshot", "sealed", snapshot)?;
        let fingerprint = state.get("fingerprint").cloned().ok_or_else(|| {
            anyhow::anyhow!("source snapshot has no fingerprint: {}", snapshot.display())
        })?;
        let destination = root.join(environment);
        copy_tree(snapshot, &destination)?;
        write_workspace_state_value(&destination, "workspace", "ready", fingerprint.clone())?;
        source_fingerprints.insert(environment, fingerprint);
    }
    write_json_atomically(
        &root.join(BUILD_STATE_FILE),
        &serde_json::json!({
            "schema": 1,
            "kind": "workspace-build",
            "status": "ready",
            "sources": source_fingerprints,
        }),
        false,
    )?;
    Ok(root)
}

/// Copy a successfully completed disposable workspace set into a sealed local
/// build. This keeps the completed build immutable to IEPM and leaves the
/// original workspace available for manual inspection or deletion.
pub fn seal_successful_build(
    workspace_root: &Path,
    store: &Path,
    name: &str,
    log_dir: &Path,
) -> Result<PathBuf> {
    validate_store_name(name, "sealed build name")?;
    let receipt: serde_json::Value = read_json(&log_dir.join("iepm-run-receipt.json"))?;
    if receipt.get("status").and_then(serde_json::Value::as_str) != Some("completed") {
        bail!(
            "execution receipt does not record a completed build: {}",
            log_dir.display()
        );
    }
    let build_state: serde_json::Value = read_json(&workspace_root.join(BUILD_STATE_FILE))?;
    expect_workspace_state(&build_state, "workspace-build", "ready", workspace_root)?;

    let mut environments = Vec::new();
    for entry in fs::read_dir(workspace_root)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let state = read_workspace_state(&entry.path())?;
        expect_workspace_state(&state, "workspace", "completed", &entry.path())?;
        environments.push(entry.file_name());
    }
    if environments.is_empty() {
        bail!("workspace build has no managed environment directories");
    }
    let destination = store.join("builds").join(name);
    if destination.exists() {
        bail!(
            "sealed build already exists: {}; choose a new name rather than overwriting it",
            destination.display()
        );
    }
    copy_tree(workspace_root, &destination)?;
    copy_tree(log_dir, &destination.join("iepm-logs"))?;
    for environment in &environments {
        let path = destination.join(environment);
        let state = read_workspace_state(&path)?;
        let fingerprint = state.get("fingerprint").cloned().ok_or_else(|| {
            anyhow::anyhow!("workspace has no source fingerprint: {}", path.display())
        })?;
        write_workspace_state_value(&path, "sealed-build", "sealed", fingerprint)?;
    }
    write_json_atomically(
        &destination.join(BUILD_STATE_FILE),
        &serde_json::json!({
            "schema": 1,
            "kind": "sealed-build",
            "status": "sealed",
            "execution_receipt": "iepm-logs/iepm-run-receipt.json",
        }),
        true,
    )?;
    Ok(destination)
}

fn validate_store_name(value: &str, description: &str) -> Result<()> {
    if value.is_empty()
        || value.contains(['/', '\\'])
        || value == "."
        || value == ".."
        || value
            .bytes()
            .any(|byte| !(byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'))
    {
        bail!("{description} must use only letters, digits, hyphens, or underscores: {value}");
    }
    Ok(())
}

fn projected_canonical_path(path: &Path) -> Result<PathBuf> {
    let mut candidate = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut missing = Vec::new();
    while !candidate.exists() {
        let name = candidate.file_name().ok_or_else(|| {
            anyhow::anyhow!("could not find an existing parent for {}", path.display())
        })?;
        missing.push(name.to_os_string());
        candidate = candidate
            .parent()
            .ok_or_else(|| {
                anyhow::anyhow!("could not find an existing parent for {}", path.display())
            })?
            .to_owned();
    }
    let mut resolved = candidate.canonicalize().map_err(|error| {
        anyhow::anyhow!("could not canonicalize {}: {error}", candidate.display())
    })?;
    for name in missing.iter().rev() {
        resolved.push(name);
    }
    Ok(resolved)
}

fn workspace_state_path(path: &Path) -> PathBuf {
    path.join(WORKSPACE_STATE_FILE)
}

fn read_workspace_state(path: &Path) -> Result<serde_json::Value> {
    read_json(&workspace_state_path(path))
}

fn read_json(path: &Path) -> Result<serde_json::Value> {
    let source = fs::read_to_string(path)
        .map_err(|error| anyhow::anyhow!("could not read {}: {error}", path.display()))?;
    serde_json::from_str(&source)
        .map_err(|error| anyhow::anyhow!("could not parse {}: {error}", path.display()))
}

fn expect_workspace_state(
    value: &serde_json::Value,
    kind: &str,
    status: &str,
    path: &Path,
) -> Result<()> {
    if value.get("schema").and_then(serde_json::Value::as_u64) != Some(1)
        || value.get("kind").and_then(serde_json::Value::as_str) != Some(kind)
        || value.get("status").and_then(serde_json::Value::as_str) != Some(status)
    {
        bail!(
            "{} is not an IEPM {} in {} state",
            path.display(),
            kind,
            status
        );
    }
    Ok(())
}

fn write_workspace_state(
    path: &Path,
    kind: &str,
    status: &str,
    fingerprint: Option<&GameFingerprint>,
) -> Result<()> {
    write_workspace_state_value(
        path,
        kind,
        status,
        fingerprint
            .map(serde_json::to_value)
            .transpose()?
            .unwrap_or(serde_json::Value::Null),
    )
}

fn write_workspace_state_value(
    path: &Path,
    kind: &str,
    status: &str,
    fingerprint: serde_json::Value,
) -> Result<()> {
    write_json_atomically(
        &workspace_state_path(path),
        &serde_json::json!({
            "schema": 1,
            "kind": kind,
            "status": status,
            "fingerprint": fingerprint,
        }),
        true,
    )
}

/// Measure the core state of a bound game workspace without modifying it.
pub fn measure_workspace_fingerprint(
    workspace: &Path,
    locale: Option<&str>,
) -> Result<GameFingerprint> {
    if !workspace.join("chitin.key").is_file() {
        bail!("workspace has no chitin.key: {}", workspace.display());
    }
    let mut paths = vec![
        PathBuf::from("Baldur.exe"),
        PathBuf::from("chitin.key"),
        PathBuf::from("engine.lua"),
        PathBuf::from("WeiDU.log"),
        PathBuf::from("EET.flag"),
        PathBuf::from("dlc/sod-dlc.zip"),
        PathBuf::from("dlc/sod-dlc.disabled"),
    ];
    if let Some(locale) = locale {
        paths.push(PathBuf::from("lang").join(locale).join("dialog.tlk"));
    }
    paths.sort();
    let mut hasher = Sha256::new();
    hasher.update(format!("{CORE_FINGERPRINT_PROFILE}\n").as_bytes());
    for relative in paths {
        let path = workspace.join(&relative);
        hasher.update(relative.to_string_lossy().replace('\\', "/").as_bytes());
        hasher.update([0]);
        if path.is_file() {
            hasher.update(b"file\0");
            hasher.update(hash_file(&path)?.as_bytes());
        } else if path.exists() {
            bail!("fingerprint path is not a file: {}", path.display());
        } else {
            hasher.update(b"missing");
        }
        hasher.update([0]);
    }
    Ok(GameFingerprint {
        profile: CORE_FINGERPRINT_PROFILE.to_owned(),
        value: format!("{:x}", hasher.finalize()),
    })
}

fn hash_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)
        .map_err(|error| anyhow::anyhow!("could not read {}: {error}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Machine-local bindings and explicit authority required for A5 mutation.
/// They are intentionally not stored in the portable lockfile.
pub struct ExecuteOptions {
    pub cache: PathBuf,
    pub weidu: PathBuf,
    pub workspaces: BTreeMap<String, PathBuf>,
    pub log_dir: PathBuf,
    pub confirm_disposable: bool,
}

pub struct ExecutionReport {
    pub actions: usize,
    pub log_dir: PathBuf,
    pub final_fingerprints: BTreeMap<String, GameFingerprint>,
    pub weidu_warnings: Vec<WeiduWarningReceipt>,
}

/// Parse repeated `environment=path` CLI values without allowing an implicit
/// environment-name convention or a machine path in the lockfile.
pub fn parse_workspace_bindings(values: &[String]) -> Result<BTreeMap<String, PathBuf>> {
    let mut bindings = BTreeMap::new();
    for value in values {
        let (name, path) = value
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("workspace binding must be NAME=PATH: {value}"))?;
        if name.is_empty() || path.is_empty() {
            bail!("workspace binding must have both NAME and PATH: {value}");
        }
        if bindings
            .insert(name.to_owned(), PathBuf::from(path))
            .is_some()
        {
            bail!("workspace environment was bound more than once: {name}");
        }
    }
    Ok(bindings)
}

/// Execute a schema-3, executable lockfile only when the caller explicitly
/// confirms every binding is a disposable workspace. This performs the narrow
/// first A5 path: verified artifact preparation, safe materialization, and
/// supervised WeiDU processes with retained stdout/stderr logs.
pub fn execute(lockfile: &Lockfile, options: &ExecuteOptions) -> Result<ExecutionReport> {
    execute_with_progress(lockfile, options, |_| {})
}

/// Execute a lockfile while reporting coarse build state and the exact WeiDU
/// action about to run. The percentage is an honest progress estimate across
/// IEPM-controlled work; WeiDU remains an external process with no reliable
/// internal completion API.
pub fn execute_with_progress<F>(
    lockfile: &Lockfile,
    options: &ExecuteOptions,
    mut progress: F,
) -> Result<ExecutionReport>
where
    F: FnMut(BuildProgress),
{
    ensure_executable(lockfile)?;
    if !options.confirm_disposable {
        bail!(
            "refusing to mutate game workspaces without --confirm-disposable; use only fresh copies"
        );
    }
    if !options.weidu.is_file() {
        bail!(
            "shared WeiDU executable does not exist: {}",
            options.weidu.display()
        );
    }
    validate_workspace_bindings(lockfile, &options.workspaces)?;
    validate_log_dir(&options.log_dir, &options.workspaces)?;
    fs::create_dir_all(&options.log_dir).map_err(|error| {
        anyhow::anyhow!("could not create {}: {error}", options.log_dir.display())
    })?;
    let run_state = options.log_dir.join("iepm-run-state.json");
    if run_state.exists() {
        bail!(
            "{} already records a previous run; rebuild fresh workspaces and use a new log directory instead of resuming in place",
            run_state.display()
        );
    }
    for (name, environment) in &lockfile.environments {
        if !environment.baseline.is_empty() {
            bail!(
                "{} declares a baseline; A5 execution cannot verify pre-existing components yet",
                name
            );
        }
        let expected = environment
            .fingerprint
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("{name} has no locked environment fingerprint"))?;
        let actual = measure_workspace_fingerprint(
            &options.workspaces[name],
            environment.locale.as_deref(),
        )?;
        if &actual != expected {
            bail!(
                "workspace fingerprint mismatch for {name}: expected {}:{}, got {}:{}",
                expected.profile,
                expected.value,
                actual.profile,
                actual.value
            );
        }
    }

    for workspace in options.workspaces.values() {
        let state = read_workspace_state(workspace)?;
        expect_workspace_state(&state, "workspace", "ready", workspace)?;
    }
    for workspace in options.workspaces.values() {
        let state = read_workspace_state(workspace)?;
        let fingerprint = state.get("fingerprint").cloned().ok_or_else(|| {
            anyhow::anyhow!(
                "workspace has no source fingerprint: {}",
                workspace.display()
            )
        })?;
        write_workspace_state_value(workspace, "workspace", "running", fingerprint)?;
    }

    write_json_atomically(
        &run_state,
        &serde_json::json!({
            "schema": 1,
            "status": "running",
            "registry_revision": lockfile.registry_revision,
            "toolchain": lockfile.toolchain,
            "environments": lockfile.environments.keys().collect::<Vec<_>>(),
        }),
        false,
    )?;
    let store = ArtifactStore::new(&options.cache)?;
    progress(BuildProgress {
        stage: "artifacts".to_owned(),
        message: format!(
            "Downloading and verifying {} mod package{}",
            lockfile.packages.len(),
            if lockfile.packages.len() == 1 {
                ""
            } else {
                "s"
            }
        ),
        percent: 20,
        action: None,
        total_actions: None,
    });
    let prepared = store.prepare_lockfile(lockfile)?;
    let artifacts = prepared
        .into_iter()
        .map(|artifact| {
            (
                artifact
                    .archive
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                artifact.extracted,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let packages = lockfile
        .packages
        .iter()
        .map(|package| {
            (
                (package.environment.as_str(), package.package.as_str()),
                package,
            )
        })
        .collect::<BTreeMap<_, _>>();

    // A fresh Enhanced Edition game does not necessarily include this standard
    // WeiDU state file. Some legitimate TP2 components read it during their
    // first action, before WeiDU has had a chance to write its first receipt.
    // Initializing only an absent, regular file keeps first-install behavior
    // deterministic without replacing any existing install history.
    for workspace in options.workspaces.values() {
        ensure_weidu_log(workspace)?;
    }

    let mut actions = 0_usize;
    let total_actions = lockfile
        .execution
        .iter()
        .map(|node| {
            let package = packages
                .get(&(node.environment.as_str(), node.package.as_str()))
                .expect("execution node package was preflighted");
            components_by_tp2(package).map(|groups| groups.len())
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum::<usize>();
    let mut weidu_warnings = Vec::new();
    for node in &lockfile.execution {
        let package = packages
            .get(&(node.environment.as_str(), node.package.as_str()))
            .ok_or_else(|| anyhow::anyhow!("execution node {} has no locked package", node.id))?;
        let workspace = &options.workspaces[&node.environment];
        let artifact = package
            .artifact
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("{} has no locked artifact", node.id))?;
        let extracted = artifacts
            .get(&artifact.sha256)
            .ok_or_else(|| anyhow::anyhow!("{} was not prepared in the artifact cache", node.id))?;
        materialize(package, extracted, workspace)?;

        let environment = &lockfile.environments[&node.environment];
        let language = package
            .language
            .as_deref()
            .or(environment.language.as_deref())
            .ok_or_else(|| anyhow::anyhow!("{} has no locked WeiDU language", node.id))?;
        for (tp2, components) in components_by_tp2(package)? {
            let installer = package
                .installers
                .iter()
                .find(|installer| installer.tp2 == tp2)
                .expect("executable lockfile was preflighted");
            let language_id = installer
                .languages
                .iter()
                .find(|candidate| candidate.name == language)
                .map(|candidate| candidate.id)
                .ok_or_else(|| anyhow::anyhow!("{tp2} has no language mapping for {language}"))?;
            let component_numbers = components
                .iter()
                .map(|component| {
                    component
                        .number
                        .expect("executable lockfile was preflighted")
                        .to_string()
                })
                .collect::<Vec<_>>();
            let receipt_components = components
                .iter()
                .filter(|component| !component.non_recording)
                .map(|component| {
                    component
                        .number
                        .expect("executable lockfile was preflighted")
                        .to_string()
                })
                .collect::<Vec<_>>();
            actions += 1;
            progress(BuildProgress {
                stage: "install".to_owned(),
                message: format!(
                    "Installing {}: {}",
                    package.package,
                    progress_component_summary(package, tp2)
                ),
                percent: install_progress_percent(actions, total_actions),
                action: Some(actions),
                total_actions: Some(total_actions),
            });
            run_weidu(
                installer,
                package,
                lockfile,
                &options.workspaces,
                workspace,
                &options.weidu,
                language_id,
                &component_numbers,
                &receipt_components,
                actions,
                &options.log_dir,
                &mut weidu_warnings,
            )?;
        }
    }
    let final_fingerprints = lockfile
        .environments
        .iter()
        .map(|(name, environment)| {
            Ok((
                name.clone(),
                measure_workspace_fingerprint(
                    &options.workspaces[name],
                    environment.locale.as_deref(),
                )?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let receipt = serde_json::json!({
        "schema": 1,
        "status": "completed",
        "actions": actions,
        "registry_revision": lockfile.registry_revision,
        "toolchain": lockfile.toolchain,
        "final_fingerprints": final_fingerprints,
    });
    write_json_atomically(
        &options.log_dir.join("iepm-run-receipt.json"),
        &receipt,
        false,
    )?;
    write_json_atomically(
        &run_state,
        &serde_json::json!({
            "schema": 1,
            "status": "completed",
            "receipt": "iepm-run-receipt.json",
        }),
        true,
    )?;
    for workspace in options.workspaces.values() {
        let state = read_workspace_state(workspace)?;
        let fingerprint = state.get("fingerprint").cloned().ok_or_else(|| {
            anyhow::anyhow!(
                "workspace has no source fingerprint: {}",
                workspace.display()
            )
        })?;
        write_workspace_state_value(workspace, "workspace", "completed", fingerprint)?;
    }
    Ok(ExecutionReport {
        actions,
        log_dir: options.log_dir.clone(),
        final_fingerprints,
        weidu_warnings,
    })
}

fn install_progress_percent(action: usize, total_actions: usize) -> u8 {
    const INSTALL_START: usize = 20;
    const INSTALL_END: usize = 90;
    if total_actions == 0 {
        return INSTALL_START as u8;
    }
    let completed_before_current = action.saturating_sub(1).min(total_actions);
    (INSTALL_START + (INSTALL_END - INSTALL_START) * completed_before_current / total_actions) as u8
}

fn progress_component_summary(package: &LockedPackage, tp2: &str) -> String {
    let components = package
        .components
        .iter()
        .filter(|component| {
            component
                .weidu
                .as_ref()
                .is_some_and(|weidu| weidu.tp2 == tp2)
        })
        .map(|component| humanize_component_id(&component.id))
        .collect::<Vec<_>>();
    match components.as_slice() {
        [] => "selected components".to_owned(),
        [only] => only.clone(),
        [first, second] => format!("{first} and {second}"),
        [first, second, third] => format!("{first}, {second}, and {third}"),
        [first, second, third, rest @ ..] => format!(
            "{first}, {second}, {third}, and {} more component{}",
            rest.len(),
            if rest.len() == 1 { "" } else { "s" }
        ),
    }
}

fn humanize_component_id(id: &str) -> String {
    id.replace('-', " ")
}

fn write_json_atomically(
    path: &Path,
    value: &serde_json::Value,
    replace_existing: bool,
) -> Result<()> {
    let temporary = path.with_extension("part");
    if temporary.exists() {
        bail!(
            "refusing to overwrite incomplete receipt: {}",
            temporary.display()
        );
    }
    if path.exists() {
        if !replace_existing {
            bail!("refusing to overwrite receipt: {}", path.display());
        }
        fs::remove_file(path)
            .map_err(|error| anyhow::anyhow!("could not replace {}: {error}", path.display()))?;
    }
    let serialized = serde_json::to_vec_pretty(value)?;
    fs::write(&temporary, serialized)
        .map_err(|error| anyhow::anyhow!("could not write {}: {error}", temporary.display()))?;
    fs::rename(&temporary, path)
        .map_err(|error| anyhow::anyhow!("could not finalize {}: {error}", path.display()))?;
    Ok(())
}

fn ensure_executable(lockfile: &Lockfile) -> Result<()> {
    if lockfile.schema != 3 {
        bail!(
            "lockfile schema {} cannot enter A5 execution",
            lockfile.schema
        );
    }
    if lockfile.execution_readiness != ExecutionReadiness::Executable {
        bail!("cannot execute an analysis-only lockfile");
    }
    if lockfile.toolchain.weidu.is_none() {
        bail!("executable lockfile has no WeiDU toolchain version");
    }
    Ok(())
}

fn validate_workspace_bindings(
    lockfile: &Lockfile,
    workspaces: &BTreeMap<String, PathBuf>,
) -> Result<()> {
    if workspaces.len() != lockfile.environments.len()
        || !workspaces.keys().eq(lockfile.environments.keys())
    {
        bail!("workspace bindings must name every and only every lockfile environment");
    }
    let mut canonical = Vec::new();
    for (name, workspace) in workspaces {
        if !workspace.is_dir() {
            bail!(
                "workspace for {name} is not an existing directory: {}",
                workspace.display()
            );
        }
        if !workspace.join("chitin.key").is_file() {
            bail!(
                "workspace for {name} has no chitin.key: {}",
                workspace.display()
            );
        }
        let state = read_workspace_state(workspace)?;
        expect_workspace_state(&state, "workspace", "ready", workspace)?;
        let path = workspace.canonicalize().map_err(|error| {
            anyhow::anyhow!("could not canonicalize {}: {error}", workspace.display())
        })?;
        canonical.push((name, path));
    }
    for (index, (left_name, left)) in canonical.iter().enumerate() {
        for (right_name, right) in canonical.iter().skip(index + 1) {
            if left == right || left.starts_with(right) || right.starts_with(left) {
                bail!(
                    "workspaces {left_name} and {right_name} must be distinct, non-nested directories"
                );
            }
        }
    }
    Ok(())
}

fn validate_log_dir(log_dir: &Path, workspaces: &BTreeMap<String, PathBuf>) -> Result<()> {
    let projected = if log_dir.exists() {
        log_dir.canonicalize().map_err(|error| {
            anyhow::anyhow!(
                "could not canonicalize log directory {}: {error}",
                log_dir.display()
            )
        })?
    } else {
        let parent = log_dir
            .parent()
            .ok_or_else(|| anyhow::anyhow!("log directory has no parent: {}", log_dir.display()))?;
        let name = log_dir
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("log directory has no name: {}", log_dir.display()))?;
        parent
            .canonicalize()
            .map_err(|error| {
                anyhow::anyhow!(
                    "log directory parent must exist before execution {}: {error}",
                    parent.display()
                )
            })?
            .join(name)
    };
    for workspace in workspaces.values() {
        let workspace = workspace.canonicalize()?;
        if projected == workspace
            || projected.starts_with(&workspace)
            || workspace.starts_with(&projected)
        {
            bail!(
                "log directory must be separate from every workspace: {}",
                projected.display()
            );
        }
    }
    Ok(())
}

fn materialize(package: &LockedPackage, extracted: &Path, workspace: &Path) -> Result<()> {
    let source = match package.materialization.source_root.as_deref() {
        Some(root) => extracted.join(safe_relative(root)?),
        None => extracted.to_owned(),
    };
    if !source.is_dir() {
        bail!(
            "{} materialization root is not a directory: {}",
            package.package,
            source.display()
        );
    }
    if package.materialization.include.is_empty() {
        if package.materialization.source_root.is_some() {
            let name = source.file_name().ok_or_else(|| {
                anyhow::anyhow!("{} materialization root has no name", package.package)
            })?;
            copy_tree(&source, &workspace.join(name))?;
        } else {
            copy_contents(&source, workspace)?;
        }
    } else {
        for include in &package.materialization.include {
            let relative = safe_relative(include)?;
            let selected = source.join(relative);
            if selected.is_dir() {
                copy_tree(&selected, &workspace.join(relative))?;
            } else if selected.is_file() {
                copy_file(&selected, &workspace.join(relative))?;
            } else {
                bail!(
                    "{} materialization include does not exist: {}",
                    package.package,
                    selected.display()
                );
            }
        }
    }
    Ok(())
}

fn safe_relative(value: &str) -> Result<&Path> {
    let path = Path::new(value);
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        bail!("materialization path must be a non-empty relative path: {value}");
    }
    Ok(path)
}

fn copy_contents(source: &Path, destination: &Path) -> Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_name() == ".iepm-complete" {
            continue;
        }
        let output = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &output)?;
        } else {
            copy_file(&entry.path(), &output)?;
        }
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    if destination.exists() {
        bail!(
            "refusing to overwrite materialized path: {}",
            destination.display()
        );
    }
    for entry in WalkDir::new(source) {
        let entry = entry?;
        if entry.file_type().is_symlink() {
            bail!(
                "refusing symbolic link during materialization: {}",
                entry.path().display()
            );
        }
        let relative = entry
            .path()
            .strip_prefix(source)
            .expect("walk starts at source");
        let output = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(output)?;
        } else {
            copy_file(entry.path(), &output)?;
        }
    }
    Ok(())
}

fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    if destination.exists() {
        bail!(
            "refusing to overwrite materialized file: {}",
            destination.display()
        );
    }
    let parent = destination
        .parent()
        .ok_or_else(|| anyhow::anyhow!("materialization destination has no parent"))?;
    fs::create_dir_all(parent)?;
    fs::copy(source, destination)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_weidu(
    installer: &Installer,
    package: &LockedPackage,
    lockfile: &Lockfile,
    workspaces: &BTreeMap<String, PathBuf>,
    workspace: &Path,
    weidu: &Path,
    language_id: u32,
    components: &[String],
    receipt_components: &[String],
    action: usize,
    log_dir: &Path,
    weidu_warnings: &mut Vec<WeiduWarningReceipt>,
) -> Result<()> {
    let locale = lockfile.environments[&package.environment]
        .locale
        .as_deref()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "{} has no game locale for WeiDU execution",
                package.environment
            )
        })?;
    let program = match installer.launcher {
        InstallerLauncher::Bundled => {
            workspace.join(safe_relative(installer.program.as_deref().ok_or_else(
                || anyhow::anyhow!("{} has no executable program", installer.tp2),
            )?)?)
        }
        InstallerLauncher::Toolchain => weidu.to_owned(),
    };
    let mut args = Vec::new();
    match installer.launcher {
        InstallerLauncher::Bundled => {
            args.extend(["--language".to_owned(), language_id.to_string()]);
            args.extend(["--use-lang".to_owned(), locale.to_owned()]);
            args.extend([
                "--skip-at-view".to_owned(),
                "--no-exit-pause".to_owned(),
                "--noautoupdate".to_owned(),
            ]);
            for component in components {
                args.extend(["--force-install".to_owned(), component.clone()]);
            }
        }
        InstallerLauncher::Toolchain => {
            args.extend([
                installer.tp2.clone(),
                "--game".to_owned(),
                workspace.display().to_string(),
                "--language".to_owned(),
                language_id.to_string(),
                "--use-lang".to_owned(),
                locale.to_owned(),
                "--skip-at-view".to_owned(),
                "--no-exit-pause".to_owned(),
                "--noautoupdate".to_owned(),
            ]);
            for component in components {
                args.extend(["--force-install".to_owned(), component.clone()]);
            }
        }
    }
    for argument in &installer.arguments {
        match argument {
            InstallerArgument::Literal { value } => args.push(value.clone()),
            InstallerArgument::EnvironmentInput { input } => {
                let environment = package.installer_inputs.get(input).ok_or_else(|| {
                    anyhow::anyhow!("{} requires installer input {input}", package.package)
                })?;
                if !lockfile.environments.contains_key(environment) {
                    bail!(
                        "{} binds {input} to unknown environment {environment}",
                        package.package
                    );
                }
                let path = workspaces.get(environment).ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} has no local workspace binding for {environment}",
                        package.package
                    )
                })?;
                args.push(path.display().to_string());
            }
        }
    }
    let stem = format!("{action:02}-{}", package.package.replace(['/', '\\'], "_"));
    let stdout_path = log_dir.join(format!("{stem}.stdout.log"));
    let stderr_path = log_dir.join(format!("{stem}.stderr.log"));
    let command_path = log_dir.join(format!("{stem}.command.txt"));
    fs::write(
        &command_path,
        format!("{}\n{}\n", program.display(), args.join("\n")),
    )?;
    let output = Command::new(&program)
        .args(&args)
        .current_dir(workspace)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| anyhow::anyhow!("could not start {}: {error}", program.display()))?;
    fs::write(&stdout_path, &output.stdout)?;
    fs::write(&stderr_path, &output.stderr)?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let warning_receipt =
        write_weidu_warning_receipt(&stdout, action, &package.package, log_dir, &stem)?;
    if !output.status.success() {
        if output.status.code() == Some(3)
            && stdout.contains("INSTALLED WITH WARNINGS")
            && installed_components_are_logged(
                workspace,
                &installer.tp2,
                language_id,
                receipt_components,
            )?
        {
            if let Some(receipt) = warning_receipt {
                weidu_warnings.push(receipt);
            }
            return Ok(());
        }
        bail!(
            "WeiDU action {action} for {} failed with {}; see {} and {}{}",
            package.package,
            output.status,
            stdout_path.display(),
            stderr_path.display(),
            ""
        );
    }
    if let Some(receipt) = warning_receipt {
        weidu_warnings.push(receipt);
    }
    if !installed_components_are_logged(workspace, &installer.tp2, language_id, receipt_components)?
    {
        bail!(
            "WeiDU action {action} for {} exited successfully but did not record every requested component in WeiDU.log; inspect {}",
            package.package,
            stdout_path.display(),
        );
    }
    Ok(())
}

fn write_weidu_warning_receipt(
    stdout: &str,
    action: usize,
    package: &str,
    log_dir: &Path,
    stem: &str,
) -> Result<Option<WeiduWarningReceipt>> {
    let details = stdout
        .lines()
        .filter(|line| line.contains("WARNING") || line.contains("INSTALLED WITH WARNINGS"))
        .collect::<Vec<_>>()
        .join("\n");
    if details.is_empty() {
        return Ok(None);
    }
    let log = log_dir.join(format!("{stem}.warnings.log"));
    fs::write(&log, format!("{details}\n"))?;
    Ok(Some(WeiduWarningReceipt {
        action,
        package: package.to_owned(),
        log,
        details,
    }))
}

fn installed_components_are_logged(
    workspace: &Path,
    tp2: &str,
    language_id: u32,
    components: &[String],
) -> Result<bool> {
    let log = fs::read_to_string(workspace.join("WeiDU.log"))?;
    let tp2 = tp2.replace('/', "\\").to_ascii_uppercase();
    Ok(components.iter().all(|component| {
        let expected = format!("~{tp2}~ #{language_id} #{component}");
        log.lines()
            .any(|line| line.to_ascii_uppercase().contains(&expected))
    }))
}

fn ensure_weidu_log(workspace: &Path) -> Result<()> {
    let log = workspace.join("WeiDU.log");
    if !log.exists() {
        fs::write(&log, "").with_context(|| format!("could not initialize {}", log.display()))?;
    }
    if !log.is_file() {
        bail!("WeiDU log is not a regular file: {}", log.display());
    }
    Ok(())
}

/// Render the first A5 deliverable: a non-mutating, auditable execution plan.
/// It never fetches, materializes, starts a process, or touches a game tree.
pub fn render_plan(lockfile: &Lockfile) -> Result<String> {
    if lockfile.schema != 3 {
        bail!(
            "lockfile schema {} cannot enter A5 planning; re-resolve it to schema 3",
            lockfile.schema
        );
    }
    if lockfile.execution_readiness != ExecutionReadiness::Executable {
        let reasons = if lockfile.blocking_reasons.is_empty() {
            "the lockfile did not provide causal blocking reasons".to_owned()
        } else {
            lockfile
                .blocking_reasons
                .iter()
                .map(|reason| format!("- {reason}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        bail!("cannot create an executable plan from an analysis-only lockfile:\n{reasons}");
    }
    let weidu = lockfile
        .toolchain
        .weidu
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("executable lockfile has no WeiDU toolchain version"))?;
    let packages = lockfile
        .packages
        .iter()
        .map(|package| {
            (
                (package.environment.as_str(), package.package.as_str()),
                package,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut output = vec![
        "IEPM execution plan (non-mutating)".to_owned(),
        format!("Registry revision: {}", lockfile.registry_revision),
        format!("WeiDU version: {weidu}"),
        "No artifact fetches, materialization, subprocesses, or filesystem mutations occur."
            .to_owned(),
    ];
    let mut action = 1_usize;
    let mut checked_baselines = BTreeSet::new();
    for node in &lockfile.execution {
        let package = packages
            .get(&(node.environment.as_str(), node.package.as_str()))
            .ok_or_else(|| anyhow::anyhow!("execution node {} has no locked package", node.id))?;
        let environment = lockfile
            .environments
            .get(&node.environment)
            .ok_or_else(|| anyhow::anyhow!("execution node {} has no environment", node.id))?;
        output.push(String::new());
        output.push(format!(
            "Environment: {} ({})",
            node.environment,
            environment_description(environment, node.phase)
        ));
        if checked_baselines.insert(node.environment.as_str()) && !environment.baseline.is_empty() {
            output.push("  Preflight baseline (verify; do not reinstall):".to_owned());
            for entry in &environment.baseline {
                output.push(format!(
                    "    ~{}~ #{} #{}",
                    entry.tp2, entry.language, entry.component
                ));
            }
        }
        if !node.predecessors.is_empty() {
            output.push(format!("  Requires: {}", node.predecessors.join(", ")));
        }
        output.push(format!(
            "{action}. Materialize {}@{}",
            package.package, package.release_id
        ));
        output.push(format!("   Artifact: {}", artifact_description(package)?));
        output.push(format!("   {}", materialization_description(package)));
        action += 1;

        let language = package
            .language
            .as_deref()
            .or(environment.language.as_deref())
            .ok_or_else(|| anyhow::anyhow!("{} has no locked WeiDU language", node.id))?;
        for (tp2, components) in components_by_tp2(package)? {
            let installer = package
                .installers
                .iter()
                .find(|installer| installer.tp2 == tp2)
                .expect("preflighted installer route");
            let language_id = installer
                .languages
                .iter()
                .find(|candidate| candidate.name == language)
                .map(|candidate| candidate.id)
                .ok_or_else(|| anyhow::anyhow!("{tp2} has no language mapping for {language}"))?;
            let numbers = components
                .iter()
                .map(|component| {
                    component
                        .number
                        .expect("preflighted numeric selector")
                        .to_string()
                })
                .collect::<Vec<_>>();
            output.push(format!("{action}. Run WeiDU for {}", package.package));
            output.push(format!("   Installer TP2: {tp2}"));
            output.push(format!(
                "   Command: {}",
                render_command(installer, package, lockfile, language_id, &numbers)?
            ));
            if !package.installer_inputs.is_empty() {
                output.push("   Portable inputs:".to_owned());
                for (name, value) in &package.installer_inputs {
                    output.push(format!("     {name} = {value}"));
                }
            }
            action += 1;
        }
    }
    Ok(format!("{}\n", output.join("\n")))
}

fn environment_description(
    environment: &iepm_core::GameEnvironment,
    phase: iepm_core::Phase,
) -> String {
    match (
        environment.after_eet_import.as_deref(),
        phase > iepm_core::Phase::EetImport,
    ) {
        (Some(result), true) => format!("{result}; transformed from {}", environment.target),
        (Some(result), false) if phase == iepm_core::Phase::EetImport => {
            format!("{}; transforms to {result}", environment.target)
        }
        _ => environment.target.clone(),
    }
}

fn render_command(
    installer: &Installer,
    package: &LockedPackage,
    lockfile: &Lockfile,
    language_id: u32,
    components: &[String],
) -> Result<String> {
    let environment = lockfile
        .environments
        .get(&package.environment)
        .expect("locked package environment was preflighted");
    let locale = environment.locale.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "{} has no game locale for WeiDU execution",
            package.environment
        )
    })?;
    let mut command = match installer.launcher {
        InstallerLauncher::Bundled => vec![format!(
            "\"{}\"",
            installer
                .program
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("{} has no executable program", installer.tp2))?
        )],
        InstallerLauncher::Toolchain => {
            vec![
                "$IEPM_WEIDU".to_owned(),
                quote(&installer.tp2),
                "--game".to_owned(),
                format!("<bound-workspace:{}>", package.environment),
                "--language".to_owned(),
                language_id.to_string(),
                "--use-lang".to_owned(),
                quote(locale),
                "--skip-at-view".to_owned(),
                "--no-exit-pause".to_owned(),
                "--noautoupdate".to_owned(),
            ]
        }
    };
    if installer.launcher == InstallerLauncher::Bundled {
        command.push("--language".to_owned());
        command.push(language_id.to_string());
        command.push("--use-lang".to_owned());
        command.push(quote(locale));
        command.extend([
            "--skip-at-view".to_owned(),
            "--no-exit-pause".to_owned(),
            "--noautoupdate".to_owned(),
        ]);
    }
    for component in components {
        command.push("--force-install".to_owned());
        command.push(component.clone());
    }
    for argument in &installer.arguments {
        match argument {
            InstallerArgument::Literal { value } => command.push(quote(value)),
            InstallerArgument::EnvironmentInput { input } => {
                let environment = package.installer_inputs.get(input).ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} requires installer input {} for its command",
                        package.package,
                        input
                    )
                })?;
                if !lockfile.environments.contains_key(environment) {
                    bail!(
                        "{} binds {} to unknown environment {}",
                        package.package,
                        input,
                        environment
                    );
                }
                command.push(format!("<bound-environment:{environment}>"));
            }
        }
    }
    Ok(command.join(" "))
}

fn quote(value: &str) -> String {
    format!("\"{value}\"")
}

fn artifact_description(package: &LockedPackage) -> Result<String> {
    let artifact = package
        .artifact
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("{} has no locked artifact", package.package))?;
    Ok(format!("sha256:{} ({})", artifact.sha256, artifact.url))
}

fn materialization_description(package: &LockedPackage) -> String {
    match (
        &package.materialization.source_root,
        package.materialization.include.is_empty(),
    ) {
        (Some(root), true) => format!("Materialization root: {root}"),
        (Some(root), false) => format!(
            "Materialization root: {root}; include: {}",
            package.materialization.include.join(", ")
        ),
        (None, false) => format!(
            "Materialize: {}",
            package.materialization.include.join(", ")
        ),
        (None, true) => "Materialize: package root".to_owned(),
    }
}

fn components_by_tp2(package: &LockedPackage) -> Result<BTreeMap<&str, Vec<&WeiDUComponent>>> {
    let mut groups = BTreeMap::new();
    for component in &package.components {
        let selector = component.weidu.as_ref().ok_or_else(|| {
            anyhow::anyhow!("{}:{} has no WeiDU selector", package.package, component.id)
        })?;
        if selector.number.is_none() {
            bail!(
                "{}:{} has no numeric WeiDU invocation",
                package.package,
                component.id
            );
        }
        if !package
            .installers
            .iter()
            .any(|installer| installer.tp2 == selector.tp2)
        {
            bail!(
                "{}:{} references undeclared installer {}",
                package.package,
                component.id,
                selector.tp2
            );
        }
        groups
            .entry(selector.tp2.as_str())
            .or_insert_with(Vec::new)
            .push(selector);
    }
    if groups.is_empty() {
        bail!("{} has no executable WeiDU components", package.package);
    }
    Ok(groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_windows_weidu_artifact_is_exact_and_hash_pinned() {
        let artifact = default_weidu_windows_251_artifact();
        assert_eq!(DEFAULT_WEIDU_VERSION, "25100");
        assert_eq!(
            artifact.url,
            "https://github.com/WeiDUorg/weidu/releases/download/v251.00/WeiDU-Windows-251.zip"
        );
        assert_eq!(
            artifact.sha256,
            "a54c6198d6ebed8139793fcacb225500d563657c23fc775b840383f9750493d8"
        );
        assert_eq!(artifact.format, ArchiveFormat::Zip);
        assert_eq!(artifact.platforms, vec![ArtifactPlatform::Windows]);
        assert_eq!(artifact.architectures, vec![ArtifactArchitecture::X86_64]);
    }

    #[test]
    fn maps_simple_game_choices_to_each_eet_environment() {
        let fixture = std::env::temp_dir().join(format!(
            "iepm-standard-source-bindings-{}-{}.yaml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(
            &fixture,
            "schema: 2\nenvironments:\n  bgee-source:\n    target: bgee\n  eet-target:\n    target: bg2ee\nmods: []\n",
        )
        .unwrap();
        let bgee = PathBuf::from("C:\\games\\bgee");
        let bg2ee = PathBuf::from("C:\\games\\bg2ee");

        let bindings =
            bind_standard_game_sources(&fixture, Some(bgee.clone()), Some(bg2ee.clone())).unwrap();
        assert_eq!(bindings["bgee-source"], bgee);
        assert_eq!(bindings["eet-target"], bg2ee);

        std::fs::remove_file(fixture).unwrap();
    }

    #[test]
    fn finds_weidu_inside_its_release_directory() {
        let root = std::env::temp_dir().join(format!(
            "iepm-weidu-release-layout-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let nested = root.join("WeiDU-Windows");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join("weidu.exe"), "fixture").unwrap();

        assert_eq!(
            find_weidu_executable(&root).unwrap(),
            nested.join("weidu.exe")
        );
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn accepts_a_user_selected_matching_weidu_override() {
        let executable = std::env::temp_dir().join(format!(
            "iepm-weidu-override-{}-{}.cmd",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(&executable, "@echo WeiDU version 25100\r\n").unwrap();

        let toolchain = validate_local_weidu_override(&executable).unwrap();
        assert_eq!(toolchain.version, DEFAULT_WEIDU_VERSION);
        assert_eq!(toolchain.executable, executable);
        std::fs::remove_file(&toolchain.executable).unwrap();
    }

    #[test]
    #[ignore = "downloads the official WeiDU release and requires network access"]
    fn prepares_the_pinned_official_weidu_release() {
        let root = std::env::temp_dir().join(format!(
            "iepm-managed-weidu-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let toolchain = prepare_default_weidu_toolchain(&root).unwrap();
        let output = Command::new(&toolchain.executable)
            .arg("--version")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains(&toolchain.version));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn parses_explicit_workspace_bindings_without_duplicates() {
        let bindings = parse_workspace_bindings(&[
            "bgee-source=C:\\games\\bgee".to_owned(),
            "eet-target=C:\\games\\bg2ee".to_owned(),
        ])
        .unwrap();
        assert_eq!(bindings["bgee-source"], PathBuf::from("C:\\games\\bgee"));
        assert!(
            parse_workspace_bindings(&[
                "bgee-source=C:\\games\\one".to_owned(),
                "bgee-source=C:\\games\\two".to_owned(),
            ])
            .is_err()
        );
    }

    #[test]
    fn accepts_a_warning_only_when_the_requested_component_is_logged() {
        let workspace =
            std::env::temp_dir().join(format!("iepm-weidu-log-test-{}", std::process::id()));
        std::fs::create_dir_all(&workspace).unwrap();
        std::fs::write(
            workspace.join("WeiDU.log"),
            "~EET\\EET.TP2~ #0 #0 // EET core\n",
        )
        .unwrap();
        assert!(
            installed_components_are_logged(&workspace, "EET/EET.tp2", 0, &["0".to_owned()])
                .unwrap()
        );
        assert!(
            !installed_components_are_logged(&workspace, "EET/EET.tp2", 0, &["1".to_owned()])
                .unwrap()
        );
        let receipt = write_weidu_warning_receipt(
            "WARNING: missing optional resource\nINSTALLED WITH WARNINGS EET core",
            4,
            "eet",
            &workspace,
            "04-eet",
        )
        .unwrap()
        .unwrap();
        assert_eq!(receipt.action, 4);
        assert_eq!(receipt.package, "eet");
        assert!(receipt.log.is_file());
        assert!(receipt.details.contains("INSTALLED WITH WARNINGS"));
        std::fs::remove_dir_all(workspace).unwrap();
    }

    #[test]
    fn initializes_an_absent_weidu_log_without_replacing_history() {
        let workspace = std::env::temp_dir().join(format!(
            "iepm-initialize-weidu-log-test-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&workspace).unwrap();
        ensure_weidu_log(&workspace).unwrap();
        assert_eq!(
            std::fs::read_to_string(workspace.join("WeiDU.log")).unwrap(),
            ""
        );
        std::fs::write(workspace.join("WeiDU.log"), "existing history\n").unwrap();
        ensure_weidu_log(&workspace).unwrap();
        assert_eq!(
            std::fs::read_to_string(workspace.join("WeiDU.log")).unwrap(),
            "existing history\n"
        );
        std::fs::remove_dir_all(workspace).unwrap();
    }

    #[test]
    fn core_fingerprint_changes_when_a_selected_layout_fact_changes() {
        let workspace = std::env::temp_dir().join(format!(
            "iepm-fingerprint-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&workspace).unwrap();
        std::fs::write(workspace.join("chitin.key"), "fixture key").unwrap();

        let before = measure_workspace_fingerprint(&workspace, None).unwrap();
        std::fs::write(workspace.join("EET.flag"), "created by EET").unwrap();
        let after = measure_workspace_fingerprint(&workspace, None).unwrap();

        assert_eq!(before.profile, CORE_FINGERPRINT_PROFILE);
        assert_ne!(before.value, after.value);
        std::fs::remove_dir_all(workspace).unwrap();
    }

    fn lockfile(readiness: &str) -> Lockfile {
        serde_json::from_str(&format!(
            r#"{{
                "schema": 3,
                "environments": {{"target": {{"target": "bg2ee", "language": "English", "locale": "en_US"}}}},
                "registry_revision": "fixture",
                "toolchain": {{"iepm": "test", "weidu": "24600"}},
                "packages": [{{
                    "package": "fixture", "release_id": "release-1", "version": "One",
                    "environment": "target", "phase": "eet",
                    "artifact": {{"url": "https://example.invalid/fixture.zip", "sha256": "{}"}},
                    "materialization": {{"source_root": "fixture", "include": ["mod"]}},
                    "installers": [{{"tp2": "mod/setup-fixture.tp2", "program": "setup-fixture.exe", "languages": [{{"id": 0, "name": "English"}}]}}],
                    "components": [{{"id": "main", "weidu": {{"tp2": "mod/setup-fixture.tp2", "label": "main", "number": 0}}}}],
                    "language": "English", "provenance": "verified"
                }}],
                "execution": [{{"id": "target::fixture", "package": "fixture", "environment": "target", "phase": "eet"}}],
                "execution_readiness": "{}",
                "blocking_reasons": ["fixture reason"]
            }}"#,
            "a".repeat(64), readiness
        ))
        .unwrap()
    }

    #[test]
    fn renders_an_auditable_non_mutating_plan() {
        let plan = render_plan(&lockfile("executable")).unwrap();
        assert!(plan.contains("IEPM execution plan (non-mutating)"));
        assert!(plan.contains("Materialize fixture@release-1"));
        assert!(plan.contains("\"setup-fixture.exe\" --language 0 --use-lang \"en_US\" --skip-at-view --no-exit-pause --noautoupdate --force-install 0"));
        assert!(plan.contains("No artifact fetches"));
    }

    #[test]
    fn refuses_analysis_only_lockfiles_with_reasons() {
        let error = render_plan(&lockfile("analysis-only")).unwrap_err();
        assert!(error.to_string().contains("fixture reason"));
    }

    #[test]
    fn legacy_lockfiles_default_to_analysis_only_and_require_reresolution() {
        let mut value = serde_json::to_value(lockfile("executable")).unwrap();
        let object = value.as_object_mut().unwrap();
        object.insert("schema".to_owned(), serde_json::json!(2));
        object.remove("execution_readiness");
        object.remove("blocking_reasons");
        let legacy: Lockfile = serde_json::from_value(value).unwrap();
        assert_eq!(legacy.execution_readiness, ExecutionReadiness::AnalysisOnly);
        assert!(
            render_plan(&legacy)
                .unwrap_err()
                .to_string()
                .contains("re-resolve it to schema 3")
        );
    }

    #[test]
    fn renders_a_toolchain_route_and_baseline_without_machine_paths() {
        let lock: Lockfile = serde_json::from_value(serde_json::json!({
            "schema": 3,
            "environments": {
                "bgee-source": {"target": "bgee"},
                "eet-target": {
                    "target": "eet", "language": "English", "locale": "en_US",
                    "baseline": [{
                        "tp2": "eefixpack/setup-eefixpack.tp2",
                        "language": 0,
                        "component": 0
                    }]
                }
            },
            "registry_revision": "fixture",
            "toolchain": {"iepm": "test", "weidu": "24600"},
            "packages": [{
                "package": "eet", "release_id": "release-1", "version": "One",
                "environment": "eet-target", "phase": "eet-import",
                "artifact": {"url": "https://example.invalid/eet.zip", "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"},
                "installers": [{
                    "tp2": "EET/EET.tp2", "launcher": "toolchain",
                    "languages": [{"id": 0, "name": "English"}],
                    "inputs": [{"name": "source-environment", "required": true}],
                    "arguments": [
                        {"kind": "literal", "value": "--args-list"},
                        {"kind": "literal", "value": "sp"},
                        {"kind": "environment-input", "input": "source-environment"}
                    ]
                }],
                "components": [{"id": "core", "weidu": {"tp2": "EET/EET.tp2", "number": 0}}],
                "language": "English",
                "installer_inputs": {"source-environment": "bgee-source"},
                "provenance": "derived"
            }],
            "execution": [{"id": "eet-target::eet", "package": "eet", "environment": "eet-target", "phase": "eet-import"}],
            "execution_readiness": "executable"
        }))
        .unwrap();
        let plan = render_plan(&lock).unwrap();
        assert!(plan.contains("$IEPM_WEIDU \"EET/EET.tp2\" --game <bound-workspace:eet-target> --language 0 --use-lang \"en_US\" --skip-at-view --no-exit-pause --noautoupdate --force-install 0 \"--args-list\" \"sp\" <bound-environment:bgee-source>"));
        assert!(plan.contains("Preflight baseline (verify; do not reinstall):"));
        assert!(!plan.contains("C:\\"));
    }

    #[test]
    fn labels_the_active_target_on_each_side_of_eet_import() {
        let environment: iepm_core::GameEnvironment = serde_json::from_value(serde_json::json!({
            "target": "bg2ee", "after_eet_import": "eet"
        }))
        .unwrap();
        assert_eq!(
            environment_description(&environment, iepm_core::Phase::EetImport),
            "bg2ee; transforms to eet"
        );
        assert_eq!(
            environment_description(&environment, iepm_core::Phase::Eet),
            "eet; transformed from bg2ee"
        );
    }

    #[test]
    fn verifies_lockfile_references_without_fetching_artifacts() {
        let mut valid = lockfile("executable");
        let report = verify_lockfile(&valid);
        assert!(report.structurally_valid);
        assert!(render_verification(&report).contains("Artifact bytes were not fetched"));

        valid.execution[0].predecessors.push("missing".to_owned());
        let invalid = verify_lockfile(&valid);
        assert!(!invalid.structurally_valid);
        assert!(
            invalid
                .structural_problems
                .iter()
                .any(|problem| problem.contains("missing predecessor"))
        );
    }

    #[test]
    fn add_canonicalizes_exact_aliases_and_makes_environment_explicit() {
        let record: iepm_core::PackageRecord = serde_yaml::from_str(
            r#"
schema: 2
package: sample-mod
aliases: [old-sample]
releases:
  - version: "1.0"
    compatibility:
      games: [bg2ee]
    install:
      phase: eet
    provenance: unverified
"#,
        )
        .unwrap();
        let registry = Registry::from([(record.package.clone(), record)]);
        let manifest: Manifest = serde_yaml::from_str(
            r#"
schema: 2
environments:
  target:
    target: bg2ee
mods: []
"#,
        )
        .unwrap();
        let updated = append_manifest_selection(
            &manifest,
            &registry,
            AddSelection {
                package: "old-sample".to_owned(),
                environment: None,
                version: Some("1.0".to_owned()),
                components: Vec::new(),
                language: None,
            },
        )
        .unwrap();

        assert_eq!(updated.mods[0].package(), "sample-mod");
        assert_eq!(updated.mods[0].environment(), Some("target"));
    }

    #[test]
    fn manages_full_copy_snapshots_workspaces_and_sealed_builds() {
        let root = std::env::temp_dir().join(format!(
            "iepm-workspace-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let source = root.join("source");
        let store = root.join("store");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("chitin.key"), "fixture key").unwrap();
        std::fs::write(source.join("Baldur.exe"), "fixture executable").unwrap();

        assert!(import_source_snapshot(&source, &source, "nested", None).is_err());

        let snapshot = import_source_snapshot(&source, &store, "bg2ee-clean", None).unwrap();
        let snapshot_state = read_workspace_state(&snapshot).unwrap();
        expect_workspace_state(&snapshot_state, "source-snapshot", "sealed", &snapshot).unwrap();

        let workspaces = BTreeMap::from([("target".to_owned(), snapshot)]);
        let workspace_root =
            create_disposable_workspaces(&workspaces, &store, "trial-one").unwrap();
        let workspace = workspace_root.join("target");
        let workspace_state = read_workspace_state(&workspace).unwrap();
        expect_workspace_state(&workspace_state, "workspace", "ready", &workspace).unwrap();
        let workspace_bindings = BTreeMap::from([("target".to_owned(), workspace.clone())]);
        assert!(validate_log_dir(&workspace.join("logs"), &workspace_bindings).is_err());

        let fingerprint = workspace_state.get("fingerprint").cloned().unwrap();
        write_workspace_state_value(&workspace, "workspace", "completed", fingerprint).unwrap();
        let logs = root.join("logs");
        std::fs::create_dir_all(&logs).unwrap();
        std::fs::write(
            logs.join("iepm-run-receipt.json"),
            r#"{"schema":1,"status":"completed"}"#,
        )
        .unwrap();

        let sealed =
            seal_successful_build(&workspace_root, &store, "trial-one-final", &logs).unwrap();
        let sealed_workspace = sealed.join("target");
        let sealed_state = read_workspace_state(&sealed_workspace).unwrap();
        expect_workspace_state(&sealed_state, "sealed-build", "sealed", &sealed_workspace).unwrap();
        assert!(workspace.join("Baldur.exe").is_file());
        assert!(sealed_workspace.join("Baldur.exe").is_file());
        assert!(
            sealed
                .join("iepm-logs")
                .join("iepm-run-receipt.json")
                .is_file()
        );

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn prepares_one_command_build_without_rewriting_the_manifest() {
        let root = std::env::temp_dir().join(format!(
            "iepm-build-preflight-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let registry = root.join("registry");
        let source = root.join("source");
        std::fs::create_dir_all(&registry).unwrap();
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("chitin.key"), "fixture key").unwrap();
        std::fs::write(
            registry.join("fixture.yaml"),
            format!(
                r#"schema: 2
package: fixture
releases:
  - version: One
    release_id: fixture-one
    artifact:
      url: https://example.invalid/fixture.zip
      sha256: {}
    compatibility:
      games: [bg2ee]
    install:
      phase: eet
    provenance: unverified
    components:
      - id: main
        weidu:
          tp2: fixture/setup-fixture.tp2
          number: 0
    installers:
      - tp2: fixture/setup-fixture.tp2
        launcher: toolchain
        languages:
          - id: 0
            name: English
"#,
                "a".repeat(64)
            ),
        )
        .unwrap();
        let manifest = root.join("modpack.yaml");
        let manifest_source = r#"schema: 2
environments:
  target:
    target: bg2ee
    language: English
    locale: en_US
mods:
  - package: fixture
    environment: target
    components: [main]
"#;
        std::fs::write(&manifest, manifest_source).unwrap();
        let weidu = root.join("weidu.exe");
        std::fs::write(&weidu, "fixture").unwrap();

        let prepared = prepare_build(&BuildOptions {
            registry,
            manifest: manifest.clone(),
            store: root.join("store"),
            build: "trial-one".to_owned(),
            sources: BTreeMap::from([("target".to_owned(), source)]),
            weidu,
            weidu_version: "25100".to_owned(),
            registry_revision: "fixture".to_owned(),
            confirm_disposable: true,
        })
        .unwrap();

        assert!(
            prepared.manifest.environments["target"]
                .fingerprint
                .is_some()
        );
        assert_eq!(
            prepared.lockfile.execution_readiness,
            ExecutionReadiness::Executable
        );
        assert!(prepared.plan.contains("Run WeiDU for fixture"));
        assert_eq!(std::fs::read_to_string(manifest).unwrap(), manifest_source);
        assert!(!root.join("store").exists());

        std::fs::remove_dir_all(root).unwrap();
    }
}

use anyhow::{Result, bail};
use iepm_artifacts::ArtifactStore;
use iepm_core::{
    ExecutionReadiness, GameFingerprint, Installer, InstallerArgument, InstallerLauncher,
    LockedPackage, Lockfile, WeiDUComponent,
};
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
    pub allow_weidu_warnings: bool,
}

pub struct ExecutionReport {
    pub actions: usize,
    pub log_dir: PathBuf,
    pub final_fingerprints: BTreeMap<String, GameFingerprint>,
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

    let mut actions = 0_usize;
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
            actions += 1;
            run_weidu(
                installer,
                package,
                lockfile,
                &options.workspaces,
                workspace,
                &options.weidu,
                language_id,
                &component_numbers,
                actions,
                &options.log_dir,
                options.allow_weidu_warnings,
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
    Ok(ExecutionReport {
        actions,
        log_dir: options.log_dir.clone(),
        final_fingerprints,
    })
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
    action: usize,
    log_dir: &Path,
    allow_weidu_warnings: bool,
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
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if allow_weidu_warnings
            && output.status.code() == Some(3)
            && stdout.contains("INSTALLED WITH WARNINGS")
            && installed_components_are_logged(workspace, &installer.tp2, language_id, components)?
        {
            let warnings = stdout
                .lines()
                .filter(|line| line.contains("WARNING") || line.contains("INSTALLED WITH WARNINGS"))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(
                log_dir.join(format!("{stem}.warnings.log")),
                format!("{warnings}\n"),
            )?;
            return Ok(());
        }
        bail!(
            "WeiDU action {action} for {} failed with {}; see {} and {}{}",
            package.package,
            output.status,
            stdout_path.display(),
            stderr_path.display(),
            if output.status.code() == Some(3) && stdout.contains("INSTALLED WITH WARNINGS") {
                "; re-run with --allow-weidu-warnings only if the warning receipt is acceptable"
            } else {
                ""
            }
        );
    }
    Ok(())
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
}

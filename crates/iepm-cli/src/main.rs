use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use iepm_core::{ExecutionReadiness, Manifest, Registry, Toolchain};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "iepm", about = "Infinity Package Manager")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Resolve {
        #[arg(long, default_value = "registry")]
        registry: PathBuf,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long, default_value = "modpack.lock.json")]
        output: PathBuf,
        #[arg(long, default_value = "working-tree")]
        registry_revision: String,
        #[arg(long, help = "Exact WeiDU version that will execute this lockfile")]
        weidu_version: Option<String>,
    },
    Fetch {
        #[arg(long, default_value = "modpack.lock.json")]
        lockfile: PathBuf,
        #[arg(long, default_value = ".iepm-cache")]
        cache: PathBuf,
    },
    /// Search curated package and release metadata without selecting or
    /// installing anything.
    Search {
        #[arg(long, default_value = "registry")]
        registry: PathBuf,
        #[arg(help = "Substring of a package, alias, release, or component ID")]
        query: Option<String>,
        #[arg(long, help = "Show only releases that declare this game target")]
        game: Option<String>,
    },
    /// Preview an explicit manifest selection. The source manifest remains
    /// unchanged unless --write is passed.
    Add {
        #[arg(long, default_value = "registry")]
        registry: PathBuf,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        package: String,
        #[arg(long)]
        environment: Option<String>,
        #[arg(long, help = "Exact release ID/version or an explicit SemVer range")]
        version: Option<String>,
        #[arg(long = "component")]
        components: Vec<String>,
        #[arg(long)]
        language: Option<String>,
        #[arg(
            long,
            help = "Write normalized YAML back to --manifest; comments are not preserved"
        )]
        write: bool,
    },
    /// Verify lockfile references and readiness without fetching artifacts or
    /// touching a game workspace.
    Verify {
        #[arg(long, default_value = "modpack.lock.json")]
        lockfile: PathBuf,
        #[arg(long, help = "Fail unless the lockfile is executable")]
        require_executable: bool,
    },
    /// Import a full local game copy into an IEPM-managed immutable source
    /// snapshot. The resulting directory can be cloned but never executed.
    Snapshot {
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        name: String,
        #[arg(
            long,
            help = "Game locale included in the source fingerprint, for example en_US"
        )]
        locale: Option<String>,
    },
    /// Create a new disposable full-copy workspace build from named immutable
    /// source snapshots. Repeat --snapshot as ENVIRONMENT=PATH.
    Workspace {
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        build: String,
        #[arg(long = "snapshot", value_name = "ENVIRONMENT=PATH", required = true)]
        snapshots: Vec<String>,
    },
    /// Seal a completed managed workspace build as a separate full-copy output.
    Seal {
        #[arg(long)]
        workspace_root: PathBuf,
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        name: String,
        #[arg(long)]
        log_dir: PathBuf,
    },
    /// Render a non-mutating A5 execution preflight from an executable lockfile.
    Plan {
        #[arg(long, default_value = "modpack.lock.json")]
        lockfile: PathBuf,
    },
    /// Measure a non-mutating versioned core fingerprint for a game workspace.
    Fingerprint {
        #[arg(long)]
        workspace: PathBuf,
        #[arg(long)]
        locale: Option<String>,
    },
    /// Read inert structural facts from a local TP2 source file. This does not
    /// execute WeiDU code or change registry records.
    InspectTp2 {
        #[arg(long)]
        tp2: PathBuf,
    },
    /// Compare an inspected TP2 with one curated registry release. A drift is
    /// review evidence only; this command never updates registry metadata.
    ReviewTp2 {
        #[arg(long, default_value = "registry")]
        registry: PathBuf,
        #[arg(long)]
        package: String,
        #[arg(long)]
        release_id: String,
        #[arg(long)]
        tp2: PathBuf,
        #[arg(
            long,
            help = "Registry-relative TP2 path; required only when the release has multiple installers"
        )]
        installer_tp2: Option<String>,
    },
    /// Materialize and execute an executable lockfile in explicitly bound,
    /// IEPM-managed disposable workspaces. This never accepts machine paths in a lockfile.
    #[command(visible_alias = "install")]
    Execute {
        #[arg(long, default_value = "modpack.lock.json")]
        lockfile: PathBuf,
        #[arg(long, default_value = ".iepm-cache")]
        cache: PathBuf,
        #[arg(long)]
        weidu: PathBuf,
        #[arg(long = "workspace", value_name = "NAME=PATH", required = true)]
        workspace: Vec<String>,
        #[arg(long)]
        log_dir: PathBuf,
        #[arg(long, help = "Confirm every --workspace is a fresh disposable copy")]
        confirm_disposable: bool,
        #[arg(
            long,
            help = "Continue only past WeiDU exit code 3 when every requested component is logged as installed with warnings"
        )]
        allow_weidu_warnings: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Resolve {
            registry,
            manifest,
            output,
            registry_revision,
            weidu_version,
        } => {
            let manifest_text = std::fs::read_to_string(&manifest)
                .with_context(|| format!("could not read {}", manifest.display()))?;
            let manifest: Manifest = serde_yaml::from_str(&manifest_text)
                .with_context(|| format!("could not parse {}", manifest.display()))?;
            let registry = iepm_registry::load(&registry)?;
            let lockfile = iepm_resolver::resolve(
                &manifest,
                &registry,
                &registry_revision,
                Toolchain {
                    iepm: env!("CARGO_PKG_VERSION").to_owned(),
                    weidu: weidu_version,
                },
            )?;
            let json = serde_json::to_string_pretty(&lockfile)?;
            std::fs::write(&output, format!("{json}\n"))
                .with_context(|| format!("could not write {}", output.display()))?;
            println!(
                "resolved {} package(s) into {}",
                lockfile.packages.len(),
                output.display()
            );
        }
        Command::Fetch { lockfile, cache } => {
            let lockfile_text = std::fs::read_to_string(&lockfile)
                .with_context(|| format!("could not read {}", lockfile.display()))?;
            let lockfile: iepm_core::Lockfile = serde_json::from_str(&lockfile_text)
                .with_context(|| format!("could not parse {}", lockfile.display()))?;
            let store = iepm_artifacts::ArtifactStore::new(&cache)
                .with_context(|| format!("could not initialize cache {}", cache.display()))?;
            let prepared = store.prepare_lockfile(&lockfile)?;
            for artifact in &prepared {
                println!(
                    "prepared {}: {}",
                    artifact.package,
                    artifact.extracted.display()
                );
            }
            println!("prepared {} artifact(s)", prepared.len());
        }
        Command::Search {
            registry,
            query,
            game,
        } => {
            let registry = iepm_registry::load(&registry)?;
            print!(
                "{}",
                render_search(&registry, query.as_deref(), game.as_deref())
            );
        }
        Command::Add {
            registry,
            manifest,
            package,
            environment,
            version,
            components,
            language,
            write,
        } => {
            let source = std::fs::read_to_string(&manifest)
                .with_context(|| format!("could not read {}", manifest.display()))?;
            let manifest_value: Manifest = serde_yaml::from_str(&source)
                .with_context(|| format!("could not parse {}", manifest.display()))?;
            let registry = iepm_registry::load(&registry)?;
            let updated = iepm::append_manifest_selection(
                &manifest_value,
                &registry,
                iepm::AddSelection {
                    package,
                    environment,
                    version,
                    components,
                    language,
                },
            )?;
            let resolved = iepm_resolver::resolve(
                &updated,
                &registry,
                "add-preview",
                Toolchain {
                    iepm: env!("CARGO_PKG_VERSION").to_owned(),
                    weidu: None,
                },
            )?;
            let yaml = serde_yaml::to_string(&updated)?;
            if write {
                std::fs::write(&manifest, &yaml)
                    .with_context(|| format!("could not write {}", manifest.display()))?;
                println!(
                    "added selection and wrote {} ({} package(s)); run resolve to produce its lockfile",
                    manifest.display(),
                    resolved.packages.len()
                );
            } else {
                println!(
                    "preview only: resolver accepted {} package(s). Original manifest unchanged.",
                    resolved.packages.len()
                );
                println!(
                    "Re-run with --write to replace the manifest with this normalized YAML (comments are not preserved):\n"
                );
                print!("{yaml}");
            }
        }
        Command::Verify {
            lockfile,
            require_executable,
        } => {
            let source = std::fs::read_to_string(&lockfile)
                .with_context(|| format!("could not read {}", lockfile.display()))?;
            let lockfile: iepm_core::Lockfile = serde_json::from_str(&source)
                .with_context(|| format!("could not parse {}", lockfile.display()))?;
            let report = iepm::verify_lockfile(&lockfile);
            print!("{}", iepm::render_verification(&report));
            if !report.structurally_valid {
                anyhow::bail!("lockfile verification found structural problems");
            }
            if require_executable && report.execution_readiness != ExecutionReadiness::Executable {
                anyhow::bail!(
                    "lockfile is analysis-only; resolve its execution blockers before continuing"
                );
            }
        }
        Command::Snapshot {
            source,
            store,
            name,
            locale,
        } => {
            let snapshot = iepm::import_source_snapshot(&source, &store, &name, locale.as_deref())?;
            println!(
                "imported immutable IEPM source snapshot: {}",
                snapshot.display()
            );
        }
        Command::Workspace {
            store,
            build,
            snapshots,
        } => {
            let snapshots = iepm::parse_workspace_bindings(&snapshots)?;
            let root = iepm::create_disposable_workspaces(&snapshots, &store, &build)?;
            println!("created disposable workspace build: {}", root.display());
        }
        Command::Seal {
            workspace_root,
            store,
            name,
            log_dir,
        } => {
            let sealed = iepm::seal_successful_build(&workspace_root, &store, &name, &log_dir)?;
            println!("sealed successful build: {}", sealed.display());
        }
        Command::Plan { lockfile } => {
            let lockfile_text = std::fs::read_to_string(&lockfile)
                .with_context(|| format!("could not read {}", lockfile.display()))?;
            let lockfile: iepm_core::Lockfile = serde_json::from_str(&lockfile_text)
                .with_context(|| format!("could not parse {}", lockfile.display()))?;
            print!("{}", iepm::render_plan(&lockfile)?);
        }
        Command::Fingerprint { workspace, locale } => {
            let fingerprint = iepm::measure_workspace_fingerprint(&workspace, locale.as_deref())?;
            println!("{}", serde_json::to_string_pretty(&fingerprint)?);
        }
        Command::InspectTp2 { tp2 } => {
            let source = std::fs::read_to_string(&tp2)
                .with_context(|| format!("could not read {}", tp2.display()))?;
            println!(
                "{}",
                serde_json::to_string_pretty(&iepm_registry::inspect_tp2(&source))?
            );
        }
        Command::ReviewTp2 {
            registry,
            package,
            release_id,
            tp2,
            installer_tp2,
        } => {
            let source = std::fs::read_to_string(&tp2)
                .with_context(|| format!("could not read {}", tp2.display()))?;
            let registry = iepm_registry::load(&registry)?;
            let observed = iepm_registry::inspect_tp2(&source);
            let review = iepm_registry::review_tp2(
                &registry,
                &package,
                &release_id,
                installer_tp2.as_deref(),
                &observed,
            )?;
            println!("{}", serde_json::to_string_pretty(&review)?);
        }
        Command::Execute {
            lockfile,
            cache,
            weidu,
            workspace,
            log_dir,
            confirm_disposable,
            allow_weidu_warnings,
        } => {
            let lockfile_text = std::fs::read_to_string(&lockfile)
                .with_context(|| format!("could not read {}", lockfile.display()))?;
            let lockfile: iepm_core::Lockfile = serde_json::from_str(&lockfile_text)
                .with_context(|| format!("could not parse {}", lockfile.display()))?;
            let report = iepm::execute(
                &lockfile,
                &iepm::ExecuteOptions {
                    cache,
                    weidu,
                    workspaces: iepm::parse_workspace_bindings(&workspace)?,
                    log_dir,
                    confirm_disposable,
                    allow_weidu_warnings,
                },
            )?;
            println!(
                "completed {} WeiDU action(s); logs: {}",
                report.actions,
                report.log_dir.display()
            );
        }
    }
    Ok(())
}

fn render_search(registry: &Registry, query: Option<&str>, game: Option<&str>) -> String {
    let query = query.unwrap_or("").to_ascii_lowercase();
    let mut lines =
        vec!["IEPM registry search (curated metadata, not compatibility proof)".to_owned()];
    let mut matches = 0_usize;
    for record in registry.values() {
        let searchable_record = format!("{} {}", record.package, record.aliases.join(" "));
        let releases = record
            .releases
            .iter()
            .filter(|release| {
                game.is_none_or(|game| {
                    release
                        .compatibility
                        .games
                        .iter()
                        .any(|candidate| candidate == game)
                }) && (query.is_empty()
                    || searchable_record.to_ascii_lowercase().contains(&query)
                    || release.id().to_ascii_lowercase().contains(&query)
                    || release.version.to_ascii_lowercase().contains(&query)
                    || release
                        .components
                        .iter()
                        .any(|component| component.id.to_ascii_lowercase().contains(&query)))
            })
            .collect::<Vec<_>>();
        if releases.is_empty() {
            continue;
        }
        matches += 1;
        lines.push(String::new());
        lines.push(record.package.clone());
        if !record.aliases.is_empty() {
            lines.push(format!("  aliases: {}", record.aliases.join(", ")));
        }
        for release in releases {
            let components = release
                .components
                .iter()
                .map(|component| component.id.as_str())
                .collect::<Vec<_>>();
            lines.push(format!(
                "  {} (display {}; declares games: {}; provenance: {:?}; artifact: {}; components: {})",
                release.id(),
                release.version,
                release.compatibility.games.join(", "),
                release.provenance,
                if release.artifacts().next().is_some() { "present" } else { "none" },
                if components.is_empty() { "none".to_owned() } else { components.join(", ") },
            ));
        }
    }
    if matches == 0 {
        lines.push("No curated package releases matched.".to_owned());
    }
    format!("{}\n", lines.join("\n"))
}

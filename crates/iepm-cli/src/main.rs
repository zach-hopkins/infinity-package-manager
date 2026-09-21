use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use iepm_core::{Manifest, Toolchain};
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
    /// disposable game workspaces. This never accepts machine paths in a lockfile.
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

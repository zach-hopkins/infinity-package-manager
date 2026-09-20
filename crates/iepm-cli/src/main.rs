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
    }
    Ok(())
}

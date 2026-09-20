use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use iepm_core::Manifest;
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
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Resolve {
            registry,
            manifest,
            output,
            registry_revision,
        } => {
            let manifest_text = std::fs::read_to_string(&manifest)
                .with_context(|| format!("could not read {}", manifest.display()))?;
            let manifest: Manifest = serde_yaml::from_str(&manifest_text)
                .with_context(|| format!("could not parse {}", manifest.display()))?;
            let registry = iepm_registry::load(&registry)?;
            let lockfile = iepm_resolver::resolve(&manifest, &registry, &registry_revision)?;
            let json = serde_json::to_string_pretty(&lockfile)?;
            std::fs::write(&output, format!("{json}\n"))
                .with_context(|| format!("could not write {}", output.display()))?;
            println!(
                "resolved {} package(s) into {}",
                lockfile.packages.len(),
                output.display()
            );
        }
    }
    Ok(())
}

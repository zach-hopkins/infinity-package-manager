use anyhow::{Context, Result, bail};
use iepm_core::{PackageRecord, Registry};
use std::path::Path;
use walkdir::WalkDir;

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
        if record.schema != 1 {
            bail!(
                "{} uses unsupported schema {}",
                record.package,
                record.schema
            );
        }
        if record.releases.is_empty() {
            bail!("{} has no releases", record.package);
        }
        if registry.insert(record.package.clone(), record).is_some() {
            bail!("duplicate package record in {}", entry.path().display());
        }
    }
    if registry.is_empty() {
        bail!("no YAML package records found under {}", path.display());
    }
    Ok(registry)
}

use anyhow::{Context, Result, bail};
use iepm_core::{PackageRecord, Registry};
use std::collections::BTreeSet;
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

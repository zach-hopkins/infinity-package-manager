use anyhow::{Context, Result, bail};
use iepm_core::{ArchiveFormat, Artifact, ArtifactPlatform, Lockfile};
use reqwest::Url;
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

const MAX_EXTRACTED_BYTES: u64 = 8 * 1024 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct PreparedArtifact {
    pub package: String,
    pub archive: PathBuf,
    pub extracted: PathBuf,
}

pub struct ArtifactStore {
    root: PathBuf,
    client: Client,
}

impl ArtifactStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(root.join("archives"))?;
        fs::create_dir_all(root.join("extracted"))?;
        let client = Client::builder()
            .https_only(true)
            .build()
            .context("could not create HTTPS client")?;
        Ok(Self { root, client })
    }

    pub fn prepare_lockfile(&self, lockfile: &Lockfile) -> Result<Vec<PreparedArtifact>> {
        lockfile
            .packages
            .iter()
            .filter_map(|package| {
                package
                    .artifact
                    .as_ref()
                    .map(|artifact| (&package.package, artifact))
            })
            .map(|(package, artifact)| self.prepare(package, artifact))
            .collect()
    }

    pub fn prepare(&self, package: &str, artifact: &Artifact) -> Result<PreparedArtifact> {
        self.ensure_platform_compatible(artifact)?;
        let archive = self
            .fetch(artifact)
            .with_context(|| format!("could not obtain artifact for {package}"))?;
        let extracted = self
            .extract(artifact, &archive)
            .with_context(|| format!("could not extract artifact for {package}"))?;
        Ok(PreparedArtifact {
            package: package.to_owned(),
            archive,
            extracted,
        })
    }

    fn fetch(&self, artifact: &Artifact) -> Result<PathBuf> {
        validate_sha256(&artifact.sha256)?;
        let url = Url::parse(&artifact.url)
            .with_context(|| format!("invalid artifact URL `{}`", artifact.url))?;
        if url.scheme() != "https" {
            bail!("artifact URL must use HTTPS: {}", artifact.url);
        }
        let destination = self.archive_path(artifact)?;
        if destination.exists() {
            if verify_sha256(&destination, &artifact.sha256).is_ok() {
                return Ok(destination);
            }
            fs::remove_file(&destination).with_context(|| {
                format!(
                    "could not remove corrupt cache entry {}",
                    destination.display()
                )
            })?;
        }

        let temporary = destination.with_extension("part");
        let _ = fs::remove_file(&temporary);
        let mut response = self
            .client
            .get(url)
            .send()
            .context("artifact download failed")?
            .error_for_status()
            .context("artifact server returned an error")?;
        let mut output = File::create(&temporary)
            .with_context(|| format!("could not create {}", temporary.display()))?;
        io::copy(&mut response, &mut output).context("could not write downloaded artifact")?;
        output.flush()?;
        drop(output);
        if let Err(error) = verify_sha256(&temporary, &artifact.sha256) {
            let _ = fs::remove_file(&temporary);
            return Err(error).context("downloaded artifact failed SHA-256 verification");
        }
        fs::rename(&temporary, &destination).with_context(|| {
            format!(
                "could not finalize cached artifact {}",
                destination.display()
            )
        })?;
        Ok(destination)
    }

    fn extract(&self, artifact: &Artifact, archive_path: &Path) -> Result<PathBuf> {
        let destination = self.extracted_path(artifact)?;
        let marker = destination.join(".iepm-complete");
        if marker.exists() {
            return Ok(destination);
        }
        if destination.exists() {
            fs::remove_dir_all(&destination).with_context(|| {
                format!(
                    "could not clear incomplete extraction {}",
                    destination.display()
                )
            })?;
        }
        fs::create_dir_all(&destination)?;
        match artifact.format {
            ArchiveFormat::Zip => extract_zip(archive_path, &destination)?,
        }
        File::create(&marker).with_context(|| {
            format!("could not mark extraction complete at {}", marker.display())
        })?;
        Ok(destination)
    }

    fn archive_path(&self, artifact: &Artifact) -> Result<PathBuf> {
        validate_sha256(&artifact.sha256)?;
        let extension = match artifact.format {
            ArchiveFormat::Zip => "zip",
        };
        Ok(self
            .root
            .join("archives")
            .join(format!("{}.{}", artifact.sha256, extension)))
    }

    fn extracted_path(&self, artifact: &Artifact) -> Result<PathBuf> {
        validate_sha256(&artifact.sha256)?;
        Ok(self.root.join("extracted").join(&artifact.sha256))
    }

    fn ensure_platform_compatible(&self, artifact: &Artifact) -> Result<()> {
        if artifact.platforms.is_empty() || artifact.platforms.contains(&host_platform()) {
            return Ok(());
        }
        bail!(
            "artifact is unavailable for this host; supported platforms: {}",
            artifact
                .platforms
                .iter()
                .map(|platform| format!("{platform:?}").to_lowercase())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn host_platform() -> ArtifactPlatform {
    #[cfg(target_os = "windows")]
    return ArtifactPlatform::Windows;
    #[cfg(target_os = "linux")]
    return ArtifactPlatform::Linux;
    #[cfg(target_os = "macos")]
    return ArtifactPlatform::Macos;
    #[allow(unreachable_code)]
    ArtifactPlatform::Windows
}

fn validate_sha256(value: &str) -> Result<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        bail!("artifact SHA-256 must be 64 lowercase hexadecimal characters")
    }
}

fn verify_sha256(path: &Path, expected: &str) -> Result<()> {
    let actual = sha256_file(path)?;
    if actual == expected {
        Ok(())
    } else {
        bail!("SHA-256 mismatch: expected {expected}, got {actual}")
    }
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file =
        File::open(path).with_context(|| format!("could not open {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let bytes = file.read(&mut buffer)?;
        if bytes == 0 {
            break;
        }
        hasher.update(&buffer[..bytes]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn extract_zip(archive_path: &Path, destination: &Path) -> Result<()> {
    let archive_file = File::open(archive_path)
        .with_context(|| format!("could not open archive {}", archive_path.display()))?;
    let mut archive =
        ZipArchive::new(archive_file).context("artifact is not a valid ZIP archive")?;
    let mut extracted_bytes = 0_u64;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let name = entry
            .enclosed_name()
            .ok_or_else(|| anyhow::anyhow!("archive entry has unsafe path: {}", entry.name()))?
            .to_owned();
        extracted_bytes = extracted_bytes
            .checked_add(entry.size())
            .ok_or_else(|| anyhow::anyhow!("archive exceeds extraction size limit"))?;
        if extracted_bytes > MAX_EXTRACTED_BYTES {
            bail!(
                "archive exceeds {} byte extraction limit",
                MAX_EXTRACTED_BYTES
            );
        }
        let output_path = destination.join(name);
        if entry.is_dir() {
            fs::create_dir_all(&output_path)?;
            continue;
        }
        let parent = output_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("archive entry has no parent path"))?;
        fs::create_dir_all(parent)?;
        let mut output = File::create(&output_path).with_context(|| {
            format!("could not create extracted file {}", output_path.display())
        })?;
        io::copy(&mut entry, &mut output)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    fn temporary_directory(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!("iepm-{label}-{nonce}"));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

    fn artifact(sha256: String) -> Artifact {
        Artifact {
            url: "https://example.invalid/fixture.zip".to_owned(),
            sha256,
            format: ArchiveFormat::Zip,
            platforms: vec![],
        }
    }

    #[test]
    fn verifies_and_extracts_a_cached_zip_artifact() {
        let root = temporary_directory("artifact");
        let source = root.join("fixture.zip");
        let mut writer = ZipWriter::new(File::create(&source).unwrap());
        writer
            .start_file("nested/hello.txt", SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"hello from IEPM").unwrap();
        writer.finish().unwrap();
        let sha256 = sha256_file(&source).unwrap();
        let artifact = artifact(sha256);
        let store = ArtifactStore::new(root.join("cache")).unwrap();
        fs::copy(&source, store.archive_path(&artifact).unwrap()).unwrap();

        let prepared = store.prepare("fixture", &artifact).unwrap();
        assert_eq!(prepared.package, "fixture");
        assert_eq!(
            fs::read_to_string(prepared.extracted.join("nested/hello.txt")).unwrap(),
            "hello from IEPM"
        );
        assert!(prepared.extracted.join(".iepm-complete").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_archives_with_unsafe_paths() {
        let root = temporary_directory("unsafe-archive");
        let source = root.join("fixture.zip");
        let mut writer = ZipWriter::new(File::create(&source).unwrap());
        writer
            .start_file("../outside.txt", SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"not allowed").unwrap();
        writer.finish().unwrap();
        let sha256 = sha256_file(&source).unwrap();
        let artifact = artifact(sha256);
        let store = ArtifactStore::new(root.join("cache")).unwrap();
        fs::copy(&source, store.archive_path(&artifact).unwrap()).unwrap();

        assert!(store.prepare("fixture", &artifact).is_err());
        assert!(!root.join("outside.txt").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_non_https_downloads_before_network_access() {
        let root = temporary_directory("http-artifact");
        let store = ArtifactStore::new(&root).unwrap();
        let mut artifact = artifact("a".repeat(64));
        artifact.url = "http://example.invalid/fixture.zip".to_owned();

        assert!(store.prepare("fixture", &artifact).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_platform_incompatible_artifacts_before_network_access() {
        let root = temporary_directory("platform-artifact");
        let store = ArtifactStore::new(&root).unwrap();
        let incompatible = match host_platform() {
            ArtifactPlatform::Windows => ArtifactPlatform::Linux,
            ArtifactPlatform::Linux | ArtifactPlatform::Macos => ArtifactPlatform::Windows,
        };
        let mut artifact = artifact("a".repeat(64));
        artifact.platforms = vec![incompatible];

        assert!(store.prepare("fixture", &artifact).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}

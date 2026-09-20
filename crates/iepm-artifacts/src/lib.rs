use anyhow::{Context, Result, bail};
use iepm_core::{ArchiveFormat, Artifact, ArtifactPlatform, Lockfile};
use reqwest::Url;
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use zip::ZipArchive;

const MAX_EXTRACTED_BYTES: u64 = 8 * 1024 * 1024 * 1024;
const MAX_DOWNLOAD_BYTES: u64 = 2 * 1024 * 1024 * 1024;

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
            .timeout(Duration::from_secs(120))
            .redirect(reqwest::redirect::Policy::limited(5))
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
        let mut failures = Vec::new();
        for location in artifact.locations() {
            match self.download(location, &temporary, artifact) {
                Ok(()) => {
                    fs::rename(&temporary, &destination).with_context(|| {
                        format!(
                            "could not finalize cached artifact {}",
                            destination.display()
                        )
                    })?;
                    return Ok(destination);
                }
                Err(error) => {
                    let _ = fs::remove_file(&temporary);
                    failures.push(format!("{location}: {error:#}"));
                }
            }
        }
        bail!("all artifact locations failed: {}", failures.join("; "))
    }

    fn download(&self, location: &str, temporary: &Path, artifact: &Artifact) -> Result<()> {
        let url =
            Url::parse(location).with_context(|| format!("invalid artifact URL `{location}`"))?;
        if url.scheme() != "https" {
            bail!("artifact URL must use HTTPS: {location}");
        }
        let mut response = self
            .client
            .get(url)
            .send()
            .context("artifact download failed")?
            .error_for_status()
            .context("artifact server returned an error")?;
        if response
            .content_length()
            .is_some_and(|length| length > MAX_DOWNLOAD_BYTES)
        {
            bail!(
                "artifact download exceeds {} byte limit",
                MAX_DOWNLOAD_BYTES
            );
        }
        let mut output = File::create(temporary)
            .with_context(|| format!("could not create {}", temporary.display()))?;
        let mut buffer = [0_u8; 64 * 1024];
        let mut downloaded = 0_u64;
        loop {
            let bytes = response.read(&mut buffer)?;
            if bytes == 0 {
                break;
            }
            downloaded = downloaded
                .checked_add(bytes as u64)
                .ok_or_else(|| anyhow::anyhow!("artifact download exceeds size limit"))?;
            if downloaded > MAX_DOWNLOAD_BYTES {
                bail!(
                    "artifact download exceeds {} byte limit",
                    MAX_DOWNLOAD_BYTES
                );
            }
            output.write_all(&buffer[..bytes])?;
        }
        output.flush()?;
        drop(output);
        verify_sha256(temporary, &artifact.sha256)
            .context("downloaded artifact failed SHA-256 verification")
    }

    fn extract(&self, artifact: &Artifact, archive_path: &Path) -> Result<PathBuf> {
        let destination = self.extracted_path(artifact)?;
        let marker = destination.join(".iepm-complete");
        if marker.exists() && extraction_is_intact(&destination, &marker, &artifact.sha256)? {
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
        let digest = tree_digest(&destination)?;
        fs::write(&marker, format!("{}\n{}\n", artifact.sha256, digest)).with_context(|| {
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
        if !artifact.platforms.is_empty() && !artifact.platforms.contains(&host_platform()) {
            bail!(
                "artifact is unavailable for this host; supported platforms: {}",
                artifact
                    .platforms
                    .iter()
                    .map(|platform| format!("{platform:?}").to_lowercase())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if artifact.architectures.is_empty()
            || artifact.architectures.contains(&host_architecture())
        {
            return Ok(());
        }
        bail!("artifact is unavailable for this CPU architecture")
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

fn host_architecture() -> iepm_core::ArtifactArchitecture {
    #[cfg(target_arch = "x86")]
    return iepm_core::ArtifactArchitecture::X86;
    #[cfg(target_arch = "x86_64")]
    return iepm_core::ArtifactArchitecture::X86_64;
    #[cfg(target_arch = "aarch64")]
    return iepm_core::ArtifactArchitecture::Aarch64;
    #[allow(unreachable_code)]
    iepm_core::ArtifactArchitecture::X86_64
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
    let mut files = BTreeSet::new();
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
        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170000 == 0o120000)
        {
            bail!("archive contains a symbolic-link entry: {}", entry.name());
        }
        let collision_key = output_path
            .strip_prefix(destination)
            .expect("output is rooted in destination")
            .to_string_lossy()
            .replace('\\', "/")
            .to_lowercase();
        if !files.insert(collision_key) {
            bail!(
                "archive contains duplicate or case-colliding path: {}",
                entry.name()
            );
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

fn extraction_is_intact(destination: &Path, marker: &Path, expected_sha256: &str) -> Result<bool> {
    let marker = fs::read_to_string(marker)?;
    let mut lines = marker.lines();
    let archive_hash = lines.next();
    let expected_tree = lines.next();
    Ok(archive_hash == Some(expected_sha256)
        && expected_tree.is_some_and(|expected| {
            tree_digest(destination).is_ok_and(|actual| actual == expected)
        }))
}

fn tree_digest(destination: &Path) -> Result<String> {
    let mut files = walkdir::WalkDir::new(destination)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.file_name() != ".iepm-complete")
        .map(|entry| entry.path().to_owned())
        .collect::<Vec<_>>();
    files.sort();
    let mut hasher = Sha256::new();
    for path in files {
        let relative = path.strip_prefix(destination).expect("walk root");
        hasher.update(relative.to_string_lossy().replace('\\', "/").as_bytes());
        hasher.update([0]);
        let mut file = File::open(&path)?;
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            let bytes = file.read(&mut buffer)?;
            if bytes == 0 {
                break;
            }
            hasher.update(&buffer[..bytes]);
        }
    }
    Ok(format!("{:x}", hasher.finalize()))
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
            mirrors: vec![],
            format: ArchiveFormat::Zip,
            platforms: vec![],
            architectures: vec![],
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
    fn repairs_an_extraction_mutated_after_completion() {
        let root = temporary_directory("mutated-extraction");
        let source = root.join("fixture.zip");
        let mut writer = ZipWriter::new(File::create(&source).unwrap());
        writer
            .start_file("mod/readme.txt", SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"original").unwrap();
        writer.finish().unwrap();
        let artifact = artifact(sha256_file(&source).unwrap());
        let store = ArtifactStore::new(root.join("cache")).unwrap();
        fs::copy(&source, store.archive_path(&artifact).unwrap()).unwrap();
        let prepared = store.prepare("fixture", &artifact).unwrap();
        fs::write(prepared.extracted.join("mod/readme.txt"), "mutated").unwrap();

        let repaired = store.prepare("fixture", &artifact).unwrap();
        assert_eq!(
            fs::read_to_string(repaired.extracted.join("mod/readme.txt")).unwrap(),
            "original"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_case_colliding_archive_paths() {
        let root = temporary_directory("case-collision");
        let source = root.join("fixture.zip");
        let mut writer = ZipWriter::new(File::create(&source).unwrap());
        writer
            .start_file("Mod/Readme.txt", SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"one").unwrap();
        writer
            .start_file("mod/readme.txt", SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"two").unwrap();
        writer.finish().unwrap();
        let artifact = artifact(sha256_file(&source).unwrap());
        let store = ArtifactStore::new(root.join("cache")).unwrap();
        fs::copy(&source, store.archive_path(&artifact).unwrap()).unwrap();

        assert!(store.prepare("fixture", &artifact).is_err());
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

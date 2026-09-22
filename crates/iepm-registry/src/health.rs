use crate::catalog::{DiscoveryCatalog, DiscoveryCatalogEntry, load_discovery_catalog};
use anyhow::{Context, Result, bail};
use reqwest::blocking::{Client, Response};
use reqwest::header::{CONTENT_TYPE, RANGE, USER_AGENT};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const ACQUISITION_HEALTH_SCHEMA: u32 = 1;
const PROBE_BYTES: usize = 64 * 1024;

/// The current, non-authoritative health of one discovery route. A successful
/// probe means only that the page/bytes were reachable at the stated time.
/// It never implies a SHA-verified IEPM artifact.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AcquisitionHealthReport {
    pub schema: u32,
    pub catalog_sha256: String,
    pub catalog_source_revision: String,
    pub version_cache_sha256: String,
    pub checked_unix_seconds: u64,
    pub catalog_entries_total: u64,
    pub entries_checked: u64,
    pub entries: Vec<AcquisitionHealthEntry>,
    pub summary: AcquisitionHealthSummary,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AcquisitionHealthEntry {
    pub source_id: u64,
    pub catalog_url: UrlHealth,
    pub acquisition: AcquisitionRoute,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_probe: Option<ArtifactProbe>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct UrlHealth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub status: UrlHealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<UrlHealthMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum UrlHealthStatus {
    Missing,
    Reachable,
    Redirected,
    HttpError,
    RequestError,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum UrlHealthMethod {
    Head,
    GetRange,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AcquisitionRoute {
    pub kind: AcquisitionRouteKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub note: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AcquisitionRouteKind {
    GithubTagSourceArchive,
    GithubBranchSourceArchive,
    ManualBrowser,
    Unavailable,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArtifactProbe {
    pub url: String,
    pub status: ArtifactProbeStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_examined: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactProbeStatus {
    ArchiveSignatureDetected,
    NonArchiveResponse,
    HttpError,
    RequestError,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct AcquisitionHealthSummary {
    pub catalog_reachable: u64,
    pub catalog_redirected: u64,
    pub catalog_failed: u64,
    pub github_tag_candidates: u64,
    pub github_branch_candidates: u64,
    pub manual_routes: u64,
    pub unavailable_routes: u64,
    pub archive_signatures_detected: u64,
    pub non_archive_responses: u64,
    pub artifact_probe_failures: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct AcquisitionHealthOptions {
    pub workers: usize,
    pub minimum_host_delay: Duration,
    pub timeout: Duration,
    pub max_records: Option<usize>,
}

impl Default for AcquisitionHealthOptions {
    fn default() -> Self {
        Self {
            workers: 4,
            minimum_host_delay: Duration::from_millis(250),
            timeout: Duration::from_secs(20),
            max_records: None,
        }
    }
}

/// Build acquisition candidates using the exact public fallback order in the
/// inspected Infinity Mod Runner snapshot: a Forge/Runner tag archive first,
/// then a mutable branch archive, then browser-assisted/manual acquisition.
/// These are deliberately candidates, not exact IEPM artifact records.
pub fn check_infinity_mod_forge_acquisition(
    catalog: &DiscoveryCatalog,
    version_cache_path: &Path,
    options: AcquisitionHealthOptions,
) -> Result<AcquisitionHealthReport> {
    if options.workers == 0 {
        bail!("at least one acquisition-health worker is required");
    }
    let cache_bytes = fs::read(version_cache_path)
        .with_context(|| format!("could not read {}", version_cache_path.display()))?;
    let version_cache: BTreeMap<String, ForgeVersionCacheEntry> =
        serde_json::from_slice(&cache_bytes).with_context(|| {
            format!(
                "{} is not an Infinity Mod Forge version cache object",
                version_cache_path.display()
            )
        })?;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(options.timeout)
        .build()
        .context("could not initialize acquisition-health HTTP client")?;
    let limiter = HostRateLimiter::new(options.minimum_host_delay);
    let tasks = Arc::new(Mutex::new(
        catalog
            .entries
            .iter()
            .take(options.max_records.unwrap_or(usize::MAX))
            .map(|entry| HealthTask {
                entry: entry.clone(),
                acquisition: acquisition_route(entry, &version_cache),
            })
            .collect::<VecDeque<_>>(),
    ));
    let results = Arc::new(Mutex::new(Vec::<AcquisitionHealthEntry>::new()));
    thread::scope(|scope| {
        for _ in 0..options.workers {
            let tasks = Arc::clone(&tasks);
            let results = Arc::clone(&results);
            let client = client.clone();
            let limiter = limiter.clone();
            scope.spawn(move || {
                loop {
                    let task = tasks.lock().expect("health task lock poisoned").pop_front();
                    let Some(task) = task else {
                        return;
                    };
                    let entry = check_entry(&client, &limiter, task);
                    results
                        .lock()
                        .expect("health result lock poisoned")
                        .push(entry);
                }
            });
        }
    });
    let mut entries = Arc::try_unwrap(results)
        .expect("all health workers should release result handles")
        .into_inner()
        .expect("health result lock poisoned");
    entries.sort_by_key(|entry| entry.source_id);
    let summary = summarize(&entries);
    Ok(AcquisitionHealthReport {
        schema: ACQUISITION_HEALTH_SCHEMA,
        catalog_sha256: sha256(&serde_json::to_vec(catalog)?),
        catalog_source_revision: catalog.source.revision.clone(),
        version_cache_sha256: sha256(&cache_bytes),
        checked_unix_seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        catalog_entries_total: catalog.entries.len() as u64,
        entries_checked: entries.len() as u64,
        entries,
        summary,
    })
}

pub fn check_infinity_mod_forge_acquisition_from_paths(
    catalog_path: &Path,
    version_cache_path: &Path,
    options: AcquisitionHealthOptions,
) -> Result<AcquisitionHealthReport> {
    let catalog = load_discovery_catalog(catalog_path)?;
    check_infinity_mod_forge_acquisition(&catalog, version_cache_path, options)
}

pub fn write_acquisition_health_report(
    report: &AcquisitionHealthReport,
    output: &Path,
) -> Result<()> {
    let parent = output
        .parent()
        .ok_or_else(|| anyhow::anyhow!("{} has no parent directory", output.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("could not create {}", parent.display()))?;
    let source = serde_json::to_string_pretty(report)?;
    fs::write(output, format!("{source}\n"))
        .with_context(|| format!("could not write {}", output.display()))
}

#[derive(Debug, Clone)]
struct HealthTask {
    entry: DiscoveryCatalogEntry,
    acquisition: AcquisitionRoute,
}

#[derive(Debug, Deserialize)]
struct ForgeVersionCacheEntry {
    #[serde(default)]
    tag: String,
}

fn acquisition_route(
    entry: &DiscoveryCatalogEntry,
    version_cache: &BTreeMap<String, ForgeVersionCacheEntry>,
) -> AcquisitionRoute {
    let Some(homepage) = entry.homepage.as_deref() else {
        return AcquisitionRoute {
            kind: AcquisitionRouteKind::Unavailable,
            candidate_url: None,
            source_page: None,
            repository: None,
            tag: None,
            note: "Forge published no project/download page for this record.".to_owned(),
        };
    };
    let Some((owner, repository)) = github_from_homepage(homepage) else {
        return AcquisitionRoute {
            kind: AcquisitionRouteKind::ManualBrowser,
            candidate_url: None,
            source_page: Some(homepage.to_owned()),
            repository: None,
            tag: None,
            note: "This host requires browser-assisted/manual acquisition until IEPM resolves an exact downloadable artifact.".to_owned(),
        };
    };
    let repository_id = format!("{owner}/{repository}");
    if let Some(tag) = version_cache
        .get(&repository_id)
        .map(|entry| entry.tag.trim())
        .filter(|tag| !tag.is_empty())
    {
        return AcquisitionRoute {
            kind: AcquisitionRouteKind::GithubTagSourceArchive,
            candidate_url: Some(format!(
                "https://github.com/{owner}/{repository}/archive/refs/tags/{tag}.zip"
            )),
            source_page: Some(homepage.to_owned()),
            repository: Some(repository_id),
            tag: Some(tag.to_owned()),
            note: "Runner-derived GitHub tag source archive candidate. IEPM must still download exact bytes, safely inspect the package, and record SHA-256 before this becomes an artifact route.".to_owned(),
        };
    }
    AcquisitionRoute {
        kind: AcquisitionRouteKind::GithubBranchSourceArchive,
        candidate_url: Some(format!(
            "https://github.com/{owner}/{repository}/archive/refs/heads/main.zip"
        )),
        source_page: Some(homepage.to_owned()),
        repository: Some(repository_id),
        tag: None,
        note: "Runner-derived default-branch source archive candidate. The branch is mutable and must be resolved to an exact commit plus SHA-256 before IEPM can use it reproducibly.".to_owned(),
    }
}

fn check_entry(
    client: &Client,
    limiter: &HostRateLimiter,
    task: HealthTask,
) -> AcquisitionHealthEntry {
    let catalog_url = check_catalog_url(client, limiter, task.entry.homepage.as_deref());
    let artifact_probe = task
        .acquisition
        .candidate_url
        .as_deref()
        .map(|url| probe_artifact(client, limiter, url));
    AcquisitionHealthEntry {
        source_id: task.entry.source_id,
        catalog_url,
        acquisition: task.acquisition,
        artifact_probe,
    }
}

fn check_catalog_url(client: &Client, limiter: &HostRateLimiter, url: Option<&str>) -> UrlHealth {
    let Some(url) = url else {
        return UrlHealth {
            url: None,
            status: UrlHealthStatus::Missing,
            method: None,
            http_status: None,
            final_url: None,
            content_type: None,
            error: None,
        };
    };
    limiter.wait(url);
    match client.head(url).header(USER_AGENT, user_agent()).send() {
        Ok(response) if response.status().is_success() => {
            health_from_response(url, response, UrlHealthMethod::Head)
        }
        Ok(response) if response.status().as_u16() != 405 && response.status().as_u16() != 501 => {
            health_from_response(url, response, UrlHealthMethod::Head)
        }
        Ok(_) | Err(_) => check_catalog_url_range(client, limiter, url),
    }
}

fn check_catalog_url_range(client: &Client, limiter: &HostRateLimiter, url: &str) -> UrlHealth {
    limiter.wait(url);
    match client
        .get(url)
        .header(USER_AGENT, user_agent())
        .header(RANGE, "bytes=0-0")
        .send()
    {
        Ok(response) => health_from_response(url, response, UrlHealthMethod::GetRange),
        Err(error) => UrlHealth {
            url: Some(url.to_owned()),
            status: UrlHealthStatus::RequestError,
            method: Some(UrlHealthMethod::GetRange),
            http_status: None,
            final_url: None,
            content_type: None,
            error: Some(error.to_string()),
        },
    }
}

fn health_from_response(url: &str, response: Response, method: UrlHealthMethod) -> UrlHealth {
    let status = response.status();
    let final_url = response.url().to_string();
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    UrlHealth {
        url: Some(url.to_owned()),
        status: if status.is_success() {
            if final_url == url {
                UrlHealthStatus::Reachable
            } else {
                UrlHealthStatus::Redirected
            }
        } else {
            UrlHealthStatus::HttpError
        },
        method: Some(method),
        http_status: Some(status.as_u16()),
        final_url: Some(final_url),
        content_type,
        error: None,
    }
}

fn probe_artifact(client: &Client, limiter: &HostRateLimiter, url: &str) -> ArtifactProbe {
    limiter.wait(url);
    let response = match client
        .get(url)
        .header(USER_AGENT, user_agent())
        .header(RANGE, format!("bytes=0-{}", PROBE_BYTES - 1))
        .send()
    {
        Ok(response) => response,
        Err(error) => {
            return ArtifactProbe {
                url: url.to_owned(),
                status: ArtifactProbeStatus::RequestError,
                http_status: None,
                final_url: None,
                content_type: None,
                bytes_examined: None,
                error: Some(error.to_string()),
            };
        }
    };
    let http_status = response.status().as_u16();
    let final_url = response.url().to_string();
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    if !response.status().is_success() {
        return ArtifactProbe {
            url: url.to_owned(),
            status: ArtifactProbeStatus::HttpError,
            http_status: Some(http_status),
            final_url: Some(final_url),
            content_type,
            bytes_examined: None,
            error: None,
        };
    }
    let mut response = response;
    let mut bytes = Vec::with_capacity(PROBE_BYTES);
    match response
        .by_ref()
        .take(PROBE_BYTES as u64)
        .read_to_end(&mut bytes)
    {
        Ok(_) => ArtifactProbe {
            url: url.to_owned(),
            status: if bytes.starts_with(b"PK\x03\x04")
                || bytes.starts_with(b"PK\x05\x06")
                || bytes.starts_with(b"PK\x07\x08")
            {
                ArtifactProbeStatus::ArchiveSignatureDetected
            } else {
                ArtifactProbeStatus::NonArchiveResponse
            },
            http_status: Some(http_status),
            final_url: Some(final_url),
            content_type,
            bytes_examined: Some(bytes.len() as u64),
            error: None,
        },
        Err(error) => ArtifactProbe {
            url: url.to_owned(),
            status: ArtifactProbeStatus::RequestError,
            http_status: Some(http_status),
            final_url: Some(final_url),
            content_type,
            bytes_examined: Some(bytes.len() as u64),
            error: Some(error.to_string()),
        },
    }
}

fn summarize(entries: &[AcquisitionHealthEntry]) -> AcquisitionHealthSummary {
    let mut summary = AcquisitionHealthSummary::default();
    for entry in entries {
        match entry.catalog_url.status {
            UrlHealthStatus::Reachable => summary.catalog_reachable += 1,
            UrlHealthStatus::Redirected => summary.catalog_redirected += 1,
            UrlHealthStatus::HttpError
            | UrlHealthStatus::RequestError
            | UrlHealthStatus::Missing => summary.catalog_failed += 1,
        }
        match entry.acquisition.kind {
            AcquisitionRouteKind::GithubTagSourceArchive => summary.github_tag_candidates += 1,
            AcquisitionRouteKind::GithubBranchSourceArchive => {
                summary.github_branch_candidates += 1
            }
            AcquisitionRouteKind::ManualBrowser => summary.manual_routes += 1,
            AcquisitionRouteKind::Unavailable => summary.unavailable_routes += 1,
        }
        if let Some(probe) = &entry.artifact_probe {
            match probe.status {
                ArtifactProbeStatus::ArchiveSignatureDetected => {
                    summary.archive_signatures_detected += 1
                }
                ArtifactProbeStatus::NonArchiveResponse => summary.non_archive_responses += 1,
                ArtifactProbeStatus::HttpError | ArtifactProbeStatus::RequestError => {
                    summary.artifact_probe_failures += 1
                }
            }
        }
    }
    summary
}

#[derive(Clone)]
struct HostRateLimiter {
    delay: Duration,
    last_request: Arc<Mutex<BTreeMap<String, Instant>>>,
}

impl HostRateLimiter {
    fn new(delay: Duration) -> Self {
        Self {
            delay,
            last_request: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    fn wait(&self, url: &str) {
        let host = reqwest::Url::parse(url)
            .ok()
            .and_then(|url| url.host_str().map(str::to_owned))
            .unwrap_or_else(|| "invalid-host".to_owned());
        loop {
            let now = Instant::now();
            let mut last_request = self
                .last_request
                .lock()
                .expect("host limiter lock poisoned");
            match last_request.get(&host).copied() {
                Some(previous) if now.duration_since(previous) < self.delay => {
                    let remaining = self.delay - now.duration_since(previous);
                    drop(last_request);
                    thread::sleep(remaining);
                }
                _ => {
                    last_request.insert(host.clone(), now);
                    return;
                }
            }
        }
    }
}

fn github_from_homepage(homepage: &str) -> Option<(String, String)> {
    let url = reqwest::Url::parse(homepage).ok()?;
    if !url.host_str()?.eq_ignore_ascii_case("github.com") {
        return None;
    }
    let mut segments = url.path_segments()?;
    let owner = segments.next()?.trim();
    let repository = segments.next()?.trim().trim_end_matches(".git");
    if owner.is_empty() || repository.is_empty() {
        return None;
    }
    Some((owner.to_owned(), repository.to_owned()))
}

fn user_agent() -> &'static str {
    "IEPM-catalog-health/0.1 (https://github.com/zach-hopkins/infinity-package-manager)"
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{DiscoveryCatalogSource, DiscoveryIdentityStatus};

    fn entry(id: u64, homepage: &str) -> DiscoveryCatalogEntry {
        DiscoveryCatalogEntry {
            source_id: id,
            tp2_name: format!("sample-{id}"),
            name: format!("Sample {id}"),
            author: None,
            summary: None,
            category: "TWEAKS".to_owned(),
            tags: vec![],
            narrative_phases: vec![],
            observed_game_targets: vec![],
            source_game_predicate_observed: false,
            component_count: 1,
            language_indices: BTreeMap::new(),
            homepage: Some(homepage.to_owned()),
            github: None,
            source_version: None,
            source_install_order: 0,
            candidate_package: format!("sample-{id}"),
            identity_status: DiscoveryIdentityStatus::Candidate,
            curated_package: None,
        }
    }

    #[test]
    fn runner_order_yields_tag_branch_and_manual_candidates() {
        let cache = BTreeMap::from([(
            "Owner/Tagged".to_owned(),
            ForgeVersionCacheEntry {
                tag: "v1.2.3".to_owned(),
            },
        )]);
        let tagged = acquisition_route(
            &entry(1, "https://github.com/Owner/Tagged/releases"),
            &cache,
        );
        assert_eq!(tagged.kind, AcquisitionRouteKind::GithubTagSourceArchive);
        assert_eq!(
            tagged.candidate_url.as_deref(),
            Some("https://github.com/Owner/Tagged/archive/refs/tags/v1.2.3.zip")
        );
        let branch = acquisition_route(&entry(2, "https://github.com/Owner/Branch"), &cache);
        assert_eq!(branch.kind, AcquisitionRouteKind::GithubBranchSourceArchive);
        let manual = acquisition_route(&entry(3, "https://www.gibberlings3.net/mods"), &cache);
        assert_eq!(manual.kind, AcquisitionRouteKind::ManualBrowser);
    }

    #[test]
    fn summary_keeps_candidate_archives_distinct_from_verified_artifacts() {
        let entries = vec![AcquisitionHealthEntry {
            source_id: 1,
            catalog_url: UrlHealth {
                url: Some("https://example.invalid".to_owned()),
                status: UrlHealthStatus::Reachable,
                method: Some(UrlHealthMethod::Head),
                http_status: Some(200),
                final_url: Some("https://example.invalid".to_owned()),
                content_type: None,
                error: None,
            },
            acquisition: AcquisitionRoute {
                kind: AcquisitionRouteKind::GithubTagSourceArchive,
                candidate_url: Some("https://example.invalid/archive.zip".to_owned()),
                source_page: None,
                repository: None,
                tag: Some("v1".to_owned()),
                note: "candidate".to_owned(),
            },
            artifact_probe: Some(ArtifactProbe {
                url: "https://example.invalid/archive.zip".to_owned(),
                status: ArtifactProbeStatus::ArchiveSignatureDetected,
                http_status: Some(206),
                final_url: None,
                content_type: Some("application/zip".to_owned()),
                bytes_examined: Some(4),
                error: None,
            }),
        }];
        let summary = summarize(&entries);
        assert_eq!(summary.github_tag_candidates, 1);
        assert_eq!(summary.archive_signatures_detected, 1);
        assert_eq!(summary.catalog_reachable, 1);
    }

    #[test]
    fn health_report_hashes_catalog_identity() {
        let mut sample = entry(1, "https://github.com/Owner/Branch");
        sample.homepage = None;
        let catalog = DiscoveryCatalog {
            schema: 1,
            source: DiscoveryCatalogSource {
                kind: "infinity-mod-forge".to_owned(),
                repository: "https://example.invalid/forge".to_owned(),
                revision: "abc".to_owned(),
                license: "MIT".to_owned(),
                input: "data/mods-index.json".to_owned(),
                input_sha256: "a".repeat(64),
            },
            entries: vec![sample],
        };
        let root = std::env::temp_dir().join(format!("iepm-health-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let cache = root.join("version-cache.json");
        fs::write(&cache, "{}").unwrap();
        let report = check_infinity_mod_forge_acquisition(
            &catalog,
            &cache,
            AcquisitionHealthOptions {
                workers: 1,
                minimum_host_delay: Duration::ZERO,
                timeout: Duration::from_millis(1),
                max_records: None,
            },
        )
        .unwrap();
        assert_eq!(report.entries.len(), 1);
        assert_eq!(report.catalog_source_revision, "abc");
        assert_eq!(report.summary.unavailable_routes, 1);
    }
}

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use iepm_core::{ExecutionReadiness, Manifest, Registry, Toolchain, VerificationRecord};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "iepm", about = "Infinity Package Manager")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build and seal a complete modded game from a manifest in one command.
    Build {
        #[arg(long, default_value = "registry")]
        registry: PathBuf,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        build: String,
        #[arg(
            long = "source",
            value_name = "ENVIRONMENT=PATH",
            required = true,
            help = "Clean game source. NAME=PATH may be repeated; PATH alone is allowed for a one-environment manifest"
        )]
        sources: Vec<String>,
        #[arg(long)]
        weidu: PathBuf,
        #[arg(long, help = "Exact WeiDU version used for this build")]
        weidu_version: String,
        #[arg(long, default_value = "working-tree")]
        registry_revision: String,
        #[arg(
            long,
            help = "Confirm IEPM may create and mutate fresh disposable copies"
        )]
        confirm_disposable: bool,
    },
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
    /// Generate the separate, non-executable discovery catalog from a pinned
    /// Infinity Mod Forge `data/mods-index.json` checkout. This never creates
    /// package releases, artifacts, compatibility claims, or lockfile input.
    CatalogImportForge {
        #[arg(long, help = "Pinned Forge data/mods-index.json input")]
        source: PathBuf,
        #[arg(long, default_value = "registry/catalog/infinity-mod-forge.json")]
        output: PathBuf,
        #[arg(
            long,
            default_value = "registry/catalog/infinity-mod-forge-report.json"
        )]
        report: PathBuf,
        #[arg(long, default_value = "registry")]
        curated_registry: PathBuf,
        #[arg(
            long,
            default_value = "https://github.com/krionashnald/InfinityModForge-Personal"
        )]
        source_repository: String,
        #[arg(long, default_value = "046414eec9315eb1500f2e68147aa281ab35873d")]
        source_revision: String,
        #[arg(long, default_value = "MIT")]
        source_license: String,
    },
    /// Check project pages and Runner-derived acquisition candidates from a
    /// discovery catalog. It never downloads a full artifact, hashes it, or
    /// changes executable registry records.
    CatalogCheckForge {
        #[arg(long, default_value = "registry/catalog/infinity-mod-forge.json")]
        catalog: PathBuf,
        #[arg(long, help = "Pinned Forge data/version_cache.json input")]
        version_cache: PathBuf,
        #[arg(
            long,
            default_value = "registry/catalog/infinity-mod-forge-health.json"
        )]
        output: PathBuf,
        #[arg(long, default_value_t = 4)]
        workers: usize,
        #[arg(long, default_value_t = 250)]
        host_delay_ms: u64,
        #[arg(long, default_value_t = 20)]
        timeout_secs: u64,
        #[arg(
            long,
            help = "Check only the first N catalog records; report remains explicitly partial"
        )]
        max_records: Option<usize>,
    },
    /// Freeze a selection corpus from Forge presets, IEPM reference manifests,
    /// and an optional user-supplied Forge-style list. The output contains
    /// source IDs only; it never turns those observations into registry facts.
    CatalogCaptureCohort {
        #[arg(long, default_value = "registry/catalog/infinity-mod-forge.json")]
        catalog: PathBuf,
        #[arg(long, help = "Pinned Forge data/presets.json input")]
        forge_presets: PathBuf,
        #[arg(long = "manifest", help = "IEPM reference manifest; may be repeated")]
        manifests: Vec<PathBuf>,
        #[arg(long, help = "Optional user-supplied Forge-style reference list")]
        reference_list: Option<PathBuf>,
        #[arg(long, help = "ISO calendar date on which the inputs were captured")]
        captured_on: String,
        #[arg(
            long,
            default_value = "registry/cohorts/product-a-selection-corpus.json"
        )]
        output: PathBuf,
    },
    /// Deterministically select the smallest Product A coverage cohort from a
    /// frozen selection corpus. This is an evidence-planning aid, not a
    /// support or compatibility decision.
    CatalogBuildCohort {
        #[arg(long, default_value = "registry/catalog/infinity-mod-forge.json")]
        catalog: PathBuf,
        #[arg(
            long,
            default_value = "registry/cohorts/product-a-selection-corpus.json"
        )]
        corpus: PathBuf,
        #[arg(long, default_value = "registry/cohorts/product-a-cohort.json")]
        output: PathBuf,
        #[arg(
            long,
            default_value = "registry/cohorts/product-a-coverage-report.json"
        )]
        report: PathBuf,
        #[arg(long, default_value_t = 50)]
        minimum_packages: usize,
        #[arg(long, default_value_t = 0.95)]
        minimum_coverage: f64,
        #[arg(
            long = "mandatory-package",
            help = "Curated or candidate package ID; may be repeated"
        )]
        mandatory_packages: Vec<String>,
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
    /// Inspect an extracted directory or ZIP-family archive for inert WeiDU
    /// package structure. This never runs setup binaries or TP2 code.
    InspectPackage {
        #[arg(long)]
        path: PathBuf,
    },
    /// Emit an author-reviewable package-record candidate from a local
    /// directory or ZIP-family archive. Games and phase are explicit inputs:
    /// they are not inferred from mechanical TP2 observations.
    DeriveBgmod {
        #[arg(long)]
        path: PathBuf,
        #[arg(long)]
        package: String,
        #[arg(long)]
        release_id: String,
        #[arg(long)]
        version: String,
        #[arg(long = "game", required = true)]
        games: Vec<String>,
        #[arg(long)]
        phase: String,
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
    /// Assess one portable verification record against IEPM's fixed public
    /// status gates. This reads evidence only and never touches a game tree.
    EvidenceStatus {
        #[arg(long)]
        record: PathBuf,
        #[arg(long, help = "Emit the complete machine-readable assessment")]
        json: bool,
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
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Build {
            registry,
            manifest,
            store,
            build,
            sources,
            weidu,
            weidu_version,
            registry_revision,
            confirm_disposable,
        } => {
            let sources = parse_build_sources(&manifest, &sources)?;
            let report = iepm::run_build_with_progress(
                &iepm::BuildOptions {
                    registry,
                    manifest,
                    store,
                    build,
                    sources,
                    weidu,
                    weidu_version,
                    registry_revision,
                    confirm_disposable,
                },
                |event| println!("[{}] {}", event.stage, event.message),
            )?;
            if !report.warnings.is_empty() {
                println!("Warnings (installation was allowed):");
                for warning in &report.warnings {
                    println!("- {warning}");
                }
            }
            println!(
                "completed {} WeiDU action(s); sealed build: {}",
                report.actions,
                report.sealed_build.display()
            );
            println!("receipts: {}", report.log_dir.display());
        }
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
        Command::CatalogImportForge {
            source,
            output,
            report,
            curated_registry,
            source_repository,
            source_revision,
            source_license,
        } => {
            let curated_registry = iepm_registry::load(&curated_registry)?;
            let imported = iepm_registry::catalog::import_infinity_mod_forge_index(
                &source,
                &source_repository,
                &source_revision,
                &source_license,
                Some(&curated_registry),
            )?;
            iepm_registry::catalog::write_discovery_catalog_import(&imported, &output, &report)?;
            println!(
                "imported {} of {} Forge records into {}; report: {}",
                imported.report.records_imported,
                imported.report.records_read,
                output.display(),
                report.display()
            );
            if !imported.report.rejections.is_empty()
                || !imported.report.candidate_collisions.is_empty()
            {
                println!(
                    "review required: {} rejected record(s), {} candidate identity collision(s)",
                    imported.report.rejections.len(),
                    imported.report.candidate_collisions.len()
                );
            }
        }
        Command::CatalogCheckForge {
            catalog,
            version_cache,
            output,
            workers,
            host_delay_ms,
            timeout_secs,
            max_records,
        } => {
            let report = iepm_registry::health::check_infinity_mod_forge_acquisition_from_paths(
                &catalog,
                &version_cache,
                iepm_registry::health::AcquisitionHealthOptions {
                    workers,
                    minimum_host_delay: std::time::Duration::from_millis(host_delay_ms),
                    timeout: std::time::Duration::from_secs(timeout_secs),
                    max_records,
                },
            )?;
            iepm_registry::health::write_acquisition_health_report(&report, &output)?;
            let summary = &report.summary;
            println!(
                "checked {} of {} catalog record(s): {} reachable, {} redirected, {} page failure(s); {} tag candidate(s), {} branch candidate(s), {} manual route(s); {} archive signature(s), {} non-archive response(s), {} artifact probe failure(s). report: {}",
                report.entries_checked,
                report.catalog_entries_total,
                summary.catalog_reachable,
                summary.catalog_redirected,
                summary.catalog_failed,
                summary.github_tag_candidates,
                summary.github_branch_candidates,
                summary.manual_routes,
                summary.archive_signatures_detected,
                summary.non_archive_responses,
                summary.artifact_probe_failures,
                output.display(),
            );
        }
        Command::CatalogCaptureCohort {
            catalog,
            forge_presets,
            manifests,
            reference_list,
            captured_on,
            output,
        } => {
            let catalog = iepm_registry::catalog::load_discovery_catalog(&catalog)?;
            let corpus = iepm_registry::cohort::capture_product_a_selection_corpus(
                &catalog,
                &forge_presets,
                &manifests,
                reference_list.as_deref(),
                &captured_on,
            )?;
            iepm_registry::cohort::write_selection_corpus(&corpus, &output)?;
            println!(
                "captured {} corpus source(s), {} mapped package selection(s), and {} unmapped token(s): {}",
                corpus.sources.len(),
                corpus
                    .sources
                    .iter()
                    .map(|source| source.selections.len())
                    .sum::<usize>(),
                corpus
                    .sources
                    .iter()
                    .map(|source| source.unmapped_tokens.len())
                    .sum::<usize>(),
                output.display(),
            );
        }
        Command::CatalogBuildCohort {
            catalog,
            corpus,
            output,
            report,
            minimum_packages,
            minimum_coverage,
            mandatory_packages,
        } => {
            let catalog = iepm_registry::catalog::load_discovery_catalog(&catalog)?;
            let corpus = iepm_registry::cohort::load_selection_corpus(&corpus)?;
            let result = iepm_registry::cohort::build_product_a_cohort(
                &catalog,
                &corpus,
                &iepm_registry::cohort::CohortBuildOptions {
                    minimum_packages,
                    minimum_coverage,
                    mandatory_packages,
                },
            )?;
            iepm_registry::cohort::write_cohort_build(&result, &output, &report)?;
            println!(
                "selected {} cohort package(s): {:.2}% weighted coverage across {} counted source(s); {} deferred candidate(s). cohort: {}; report: {}",
                result.cohort.packages.len(),
                result.report.coverage * 100.0,
                result.report.counted_sources,
                result.report.deferred_candidates.len(),
                output.display(),
                report.display(),
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
            let source =
                std::fs::read(&tp2).with_context(|| format!("could not read {}", tp2.display()))?;
            println!(
                "{}",
                serde_json::to_string_pretty(&iepm_registry::inspect_tp2(
                    &String::from_utf8_lossy(&source)
                ))?
            );
        }
        Command::InspectPackage { path } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&iepm_registry::inspect_package(&path)?)?
            );
        }
        Command::DeriveBgmod {
            path,
            package,
            release_id,
            version,
            games,
            phase,
        } => {
            let observation = iepm_registry::inspect_package(&path)?;
            print!(
                "{}",
                iepm_registry::derive_bgmod_candidate(
                    &package,
                    &release_id,
                    &version,
                    &games,
                    &phase,
                    &observation,
                )?
            );
        }
        Command::ReviewTp2 {
            registry,
            package,
            release_id,
            tp2,
            installer_tp2,
        } => {
            let source =
                std::fs::read(&tp2).with_context(|| format!("could not read {}", tp2.display()))?;
            let registry = iepm_registry::load(&registry)?;
            let observed = iepm_registry::inspect_tp2(&String::from_utf8_lossy(&source));
            let review = iepm_registry::review_tp2(
                &registry,
                &package,
                &release_id,
                installer_tp2.as_deref(),
                &observed,
            )?;
            println!("{}", serde_json::to_string_pretty(&review)?);
        }
        Command::EvidenceStatus { record, json } => {
            let source = std::fs::read_to_string(&record)
                .with_context(|| format!("could not read {}", record.display()))?;
            let record: VerificationRecord = serde_json::from_str(&source)
                .with_context(|| format!("could not parse {}", record.display()))?;
            let assessment = record.assess();
            if json {
                println!("{}", serde_json::to_string_pretty(&assessment)?);
            } else {
                println!("{}: {}", record.subject, assessment.status.display_name());
                for reason in &assessment.incompatibility {
                    println!("- incompatible: {reason}");
                }
                if !assessment.missing.is_empty() {
                    let label = if assessment.status == iepm_core::PublicSupportStatus::Supported {
                        "Needed for Verified"
                    } else {
                        "Missing support evidence"
                    };
                    println!("{label}:");
                    for item in &assessment.missing {
                        println!("- {item}");
                    }
                }
            }
        }
        Command::Execute {
            lockfile,
            cache,
            weidu,
            workspace,
            log_dir,
            confirm_disposable,
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

fn parse_build_sources(manifest: &PathBuf, values: &[String]) -> Result<BTreeMap<String, PathBuf>> {
    if values.len() == 1 && !values[0].contains('=') {
        let source = std::fs::read_to_string(manifest)
            .with_context(|| format!("could not read {}", manifest.display()))?;
        let manifest: Manifest = serde_yaml::from_str(&source)
            .with_context(|| format!("could not parse {}", manifest.display()))?;
        if manifest.environments.len() != 1 {
            anyhow::bail!(
                "--source PATH shorthand requires exactly one named manifest environment; use NAME=PATH bindings"
            );
        }
        let name = manifest
            .environments
            .keys()
            .next()
            .expect("checked one environment")
            .clone();
        return Ok(BTreeMap::from([(name, PathBuf::from(&values[0]))]));
    }
    iepm::parse_workspace_bindings(values)
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

#[cfg(test)]
mod verification_cli_tests {
    use super::*;

    #[test]
    fn checked_in_json_fixture_reaches_supported_boundary() {
        let source =
            include_str!("../../../tests/fixtures/verification/supported-configuration.json");
        let record: VerificationRecord =
            serde_json::from_str(source).expect("fixture should follow the evidence contract");

        let assessment = record.assess();
        assert_eq!(assessment.status, iepm_core::PublicSupportStatus::Supported);
        assert!(
            assessment
                .missing
                .contains(&"clean disposable install completed".to_owned())
        );

        let misspelled = source.replace("artifact_verified", "artifact_verifieed");
        assert!(serde_json::from_str::<VerificationRecord>(&misspelled).is_err());
    }
}

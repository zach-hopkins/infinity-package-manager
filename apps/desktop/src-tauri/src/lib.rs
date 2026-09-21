use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildRequest {
    registry: String,
    manifest: String,
    store: String,
    build: String,
    sources: BTreeMap<String, String>,
    weidu: String,
    weidu_version: String,
    registry_revision: Option<String>,
    confirm_disposable: bool,
    allow_weidu_warnings: bool,
}

impl BuildRequest {
    fn into_options(self) -> iepm::BuildOptions {
        iepm::BuildOptions {
            registry: PathBuf::from(self.registry),
            manifest: PathBuf::from(self.manifest),
            store: PathBuf::from(self.store),
            build: self.build,
            sources: self
                .sources
                .into_iter()
                .map(|(name, path)| (name, PathBuf::from(path)))
                .collect(),
            weidu: PathBuf::from(self.weidu),
            weidu_version: self.weidu_version,
            registry_revision: self
                .registry_revision
                .unwrap_or_else(|| "working-tree".to_owned()),
            confirm_disposable: self.confirm_disposable,
            allow_weidu_warnings: self.allow_weidu_warnings,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CoreStatus {
    version: &'static str,
    architecture: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BuildPreview {
    plan: String,
    verification: String,
    warnings: Vec<String>,
    package_count: usize,
    environments: Vec<String>,
}

#[tauri::command]
fn core_status() -> CoreStatus {
    CoreStatus {
        version: env!("CARGO_PKG_VERSION"),
        architecture: "SvelteKit -> Tauri commands/events -> IEPM Rust core",
    }
}

#[tauri::command]
async fn preview_build(request: BuildRequest) -> Result<BuildPreview, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let prepared = iepm::prepare_build(&request.into_options()).map_err(|e| e.to_string())?;
        Ok(BuildPreview {
            plan: prepared.plan,
            verification: prepared.verification,
            warnings: prepared.lockfile.warnings.clone(),
            package_count: prepared.lockfile.packages.len(),
            environments: prepared.lockfile.environments.keys().cloned().collect(),
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
async fn start_build(app: AppHandle, request: BuildRequest) -> Result<iepm::BuildReport, String> {
    tauri::async_runtime::spawn_blocking(move || {
        iepm::run_build_with_progress(&request.into_options(), |event| {
            let _ = app.emit("iepm://build-progress", event);
        })
        .map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| error.to_string())?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core_status,
            preview_build,
            start_build
        ])
        .run(tauri::generate_context!())
        .expect("error while running IEPM desktop");
}

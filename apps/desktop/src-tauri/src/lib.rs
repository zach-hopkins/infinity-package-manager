use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DesktopSettings {
    library_location: Option<String>,
    weidu_override: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuidedBuildRequest {
    manifest: String,
    experience_name: Option<String>,
    bgee_directory: Option<String>,
    bg2ee_directory: Option<String>,
    allow_weidu_warnings: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CoreStatus {
    version: &'static str,
    architecture: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GameDetection {
    game: String,
    label: String,
    path: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StartupState {
    library_location: Option<String>,
    weidu_override: Option<String>,
    detected_games: Vec<GameDetection>,
    default_weidu_version: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BuildPreview {
    plan: String,
    verification: String,
    warnings: Vec<String>,
    package_count: usize,
    environments: Vec<String>,
    weidu_version: String,
}

#[tauri::command]
fn core_status() -> CoreStatus {
    CoreStatus {
        version: env!("CARGO_PKG_VERSION"),
        architecture: "SvelteKit -> Tauri commands/events -> IEPM Rust core",
    }
}

#[tauri::command]
fn startup_state(app: AppHandle) -> Result<StartupState, String> {
    let settings = read_settings(&app)?;
    Ok(StartupState {
        library_location: settings.library_location,
        weidu_override: settings.weidu_override,
        detected_games: detect_common_games(),
        default_weidu_version: iepm::DEFAULT_WEIDU_VERSION,
    })
}

#[tauri::command]
fn save_weidu_override(app: AppHandle, path: Option<String>) -> Result<StartupState, String> {
    let mut settings = read_settings(&app)?;
    settings.weidu_override = path
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_owned());
    if let Some(override_path) = settings.weidu_override.as_deref() {
        iepm::validate_local_weidu_override(Path::new(override_path))
            .map_err(|error| format!("Could not use the local WeiDU override: {error}"))?;
    }
    write_settings(&app, &settings)?;
    startup_state(app)
}

#[tauri::command]
fn save_library_location(app: AppHandle, path: String) -> Result<StartupState, String> {
    let library = PathBuf::from(path.trim());
    if library.as_os_str().is_empty() {
        return Err("Choose a folder for the IEPM library".to_owned());
    }
    if library.join("chitin.key").is_file() {
        return Err("The IEPM library must be separate from a game installation".to_owned());
    }
    fs::create_dir_all(&library)
        .map_err(|error| format!("Could not create {}: {error}", library.display()))?;
    write_settings(
        &app,
        &DesktopSettings {
            library_location: Some(library.to_string_lossy().into_owned()),
            weidu_override: read_settings(&app)?.weidu_override,
        },
    )?;
    startup_state(app)
}

#[tauri::command]
async fn preview_build(
    app: AppHandle,
    request: GuidedBuildRequest,
) -> Result<BuildPreview, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let (options, toolchain, uses_local_override) = desktop_build_options(&app, request)?;
        let prepared = iepm::prepare_build(&options).map_err(|e| e.to_string())?;
        let mut warnings = prepared.lockfile.warnings.clone();
        if uses_local_override {
            warnings.push(
                "WeiDU is a user-selected local override. Its v251 version was checked, but IEPM could not verify it against the official release archive SHA-256."
                    .to_owned(),
            );
        }
        Ok(BuildPreview {
            plan: prepared.plan,
            verification: prepared.verification,
            warnings,
            package_count: prepared.lockfile.packages.len(),
            environments: prepared.lockfile.environments.keys().cloned().collect(),
            weidu_version: toolchain.version,
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
async fn start_build(
    app: AppHandle,
    request: GuidedBuildRequest,
) -> Result<iepm::BuildReport, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _ = app.emit(
            "iepm://build-progress",
            iepm::BuildProgress {
                stage: "toolchain".to_owned(),
                message: "Preparing the verified WeiDU installation engine".to_owned(),
            },
        );
        let (options, _, _) = desktop_build_options(&app, request)?;
        iepm::run_build_with_progress(&options, |event| {
            let _ = app.emit("iepm://build-progress", event);
        })
        .map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| error.to_string())?
}

fn desktop_build_options(
    app: &AppHandle,
    request: GuidedBuildRequest,
) -> Result<(iepm::BuildOptions, iepm::ManagedWeiduToolchain, bool), String> {
    let settings = read_settings(app)?;
    let store = settings
        .library_location
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Choose an IEPM library folder before creating a mod experience".to_owned()
        })?;
    let manifest = PathBuf::from(request.manifest.trim());
    if !manifest.is_file() {
        return Err(format!(
            "Choose a mod-list YAML file; not found: {}",
            manifest.display()
        ));
    }
    let sources = iepm::bind_standard_game_sources(
        &manifest,
        request
            .bgee_directory
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from),
        request
            .bg2ee_directory
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from),
    )
    .map_err(|error| error.to_string())?;
    let uses_local_override = settings.weidu_override.is_some();
    let toolchain = match settings.weidu_override.as_deref() {
        Some(override_path) => iepm::validate_local_weidu_override(Path::new(override_path))
            .map_err(|error| format!("Could not use the local WeiDU override: {error}"))?,
        None => iepm::prepare_default_weidu_toolchain(&store).map_err(|error| {
            format!(
                "Could not prepare IEPM's verified WeiDU {} toolchain: {error}\n\nWindows may have blocked the official download. IEPM will not bypass Windows security. In Settings, choose a trusted local WeiDU v{} executable as an advanced override.",
                iepm::DEFAULT_WEIDU_VERSION,
                iepm::DEFAULT_WEIDU_VERSION,
            )
        })?,
    };
    let build = request
        .experience_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(iepm::default_experience_name);
    Ok((
        iepm::BuildOptions {
            registry: bundled_registry_path(app)?,
            manifest,
            store,
            build,
            sources,
            weidu: toolchain.executable.clone(),
            weidu_version: toolchain.version.clone(),
            registry_revision: "desktop-bundled-registry".to_owned(),
            confirm_disposable: true,
            allow_weidu_warnings: request.allow_weidu_warnings,
        },
        toolchain,
        uses_local_override,
    ))
}

fn bundled_registry_path(app: &AppHandle) -> Result<PathBuf, String> {
    let bundled = app
        .path()
        .resource_dir()
        .map_err(|error| format!("Could not locate bundled IEPM resources: {error}"))?
        .join("registry");
    if bundled.is_dir() {
        return Ok(bundled);
    }
    let development = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../registry");
    if development.is_dir() {
        return Ok(development);
    }
    Err("The bundled IEPM registry is unavailable; reinstall the application".to_owned())
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Could not locate IEPM settings: {error}"))?;
    fs::create_dir_all(&directory)
        .map_err(|error| format!("Could not create IEPM settings directory: {error}"))?;
    Ok(directory.join("settings.json"))
}

fn read_settings(app: &AppHandle) -> Result<DesktopSettings, String> {
    let path = settings_path(app)?;
    if !path.exists() {
        return Ok(DesktopSettings::default());
    }
    let source = fs::read_to_string(&path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    serde_json::from_str(&source)
        .map_err(|error| format!("Could not parse {}: {error}", path.display()))
}

fn write_settings(app: &AppHandle, settings: &DesktopSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    fs::write(
        &path,
        serde_json::to_string_pretty(settings)
            .map_err(|error| format!("Could not serialize IEPM settings: {error}"))?,
    )
    .map_err(|error| format!("Could not write {}: {error}", path.display()))
}

fn detect_common_games() -> Vec<GameDetection> {
    let mut roots = BTreeSet::new();
    for variable in ["ProgramFiles(x86)", "ProgramW6432"] {
        if let Some(program_files) = std::env::var_os(variable) {
            roots.insert(
                PathBuf::from(program_files)
                    .join("Steam")
                    .join("steamapps")
                    .join("common"),
            );
        }
    }
    roots.insert(PathBuf::from(
        "C:\\Program Files (x86)\\Steam\\steamapps\\common",
    ));
    roots.insert(PathBuf::from("C:\\Program Files\\Steam\\steamapps\\common"));

    let mut games = Vec::new();
    for root in roots {
        for (game, directory, label) in [
            (
                "bgee",
                "Baldur's Gate Enhanced Edition",
                "BG:EE / Siege of Dragonspear",
            ),
            ("bg2ee", "Baldur's Gate II Enhanced Edition", "BG2:EE"),
        ] {
            let path = root.join(directory);
            if is_game_directory(&path) {
                games.push(GameDetection {
                    game: game.to_owned(),
                    label: label.to_owned(),
                    path: path.to_string_lossy().into_owned(),
                });
            }
        }
    }
    games.sort_by(|left, right| left.path.cmp(&right.path));
    games.dedup_by(|left, right| left.path == right.path);
    games
}

fn is_game_directory(path: &Path) -> bool {
    path.is_dir() && path.join("chitin.key").is_file()
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            core_status,
            startup_state,
            save_library_location,
            save_weidu_override,
            preview_build,
            start_build
        ])
        .run(tauri::generate_context!())
        .expect("error while running IEPM desktop");
}

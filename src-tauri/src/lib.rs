mod actions;
mod history;
mod icons;
mod metadata;
mod storage;
mod terminal;
mod updates;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use wait_timeout::ChildExt;

use crate::history::HistoryEntry;
use crate::metadata::{LocalMetadata, MetadataMap};
use crate::terminal::{hide_console_window, is_risky_command, TERMINAL_TIMEOUT_SECONDS};
use crate::updates::WingetReport;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct InstalledApp {
    id: String,
    name: String,
    version: Option<String>,
    publisher: Option<String>,
    install_location: Option<String>,
    install_date: Option<String>,
    estimated_size_mb: Option<u64>,
    uninstall_string: Option<String>,
    icon_source: Option<String>,
    executable_path: Option<String>,
    source: String,
    category: String,
    update_hint: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareInventory {
    scanned_at: String,
    total: usize,
    apps: Vec<InstalledApp>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TerminalRequest {
    shell: String,
    command: String,
    #[serde(default)]
    confirmed_risky: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TerminalResult {
    shell: String,
    command: String,
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    duration_ms: u128,
    risky: bool,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionRequest {
    target: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UninstallRequest {
    uninstall_string: String,
    confirmed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetadataUpdateRequest {
    app_id: String,
    metadata: LocalMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRequest {
    app_id: String,
    source_path: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct IconResponse {
    app_id: String,
    base64_png: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RiskyCheckResponse {
    command: String,
    risky: bool,
}

#[tauri::command]
fn scan_installed_apps() -> Result<SoftwareInventory, String> {
    let apps = collect_installed_apps()?;
    Ok(SoftwareInventory {
        scanned_at: current_timestamp(),
        total: apps.len(),
        apps,
    })
}

#[tauri::command]
fn check_command_risk(command: String) -> RiskyCheckResponse {
    RiskyCheckResponse {
        risky: is_risky_command(&command),
        command,
    }
}

#[tauri::command]
fn run_terminal_command(request: TerminalRequest) -> Result<TerminalResult, String> {
    let shell = request.shell.trim().to_lowercase();
    let command_text = request.command.trim();

    if command_text.is_empty() {
        return Err("Commande vide.".to_string());
    }

    let risky = is_risky_command(command_text);
    if risky && !request.confirmed_risky {
        return Err("Commande sensible : confirmation requise.".to_string());
    }

    let started = Instant::now();
    let mut process = match shell.as_str() {
        "powershell" => {
            let mut process = Command::new("powershell.exe");
            let encoded = terminal::powershell_encoded_command(command_text);
            process.args([
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-ExecutionPolicy",
                "Bypass",
                "-EncodedCommand",
                &encoded,
            ]);
            process
        }
        "cmd" => {
            let mut process = Command::new("cmd.exe");
            process.args(["/C", command_text]);
            process
        }
        _ => return Err("Shell non supporte. Utilise powershell ou cmd.".to_string()),
    };

    hide_console_window(&mut process);
    process.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = process
        .spawn()
        .map_err(|error| format!("Impossible d'executer la commande: {error}"))?;

    match child
        .wait_timeout(Duration::from_secs(TERMINAL_TIMEOUT_SECONDS))
        .map_err(|error| format!("Impossible d'attendre la commande: {error}"))?
    {
        Some(_) => {}
        None => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!(
                "Commande arretee apres {TERMINAL_TIMEOUT_SECONDS} secondes."
            ));
        }
    };

    let output = child
        .wait_with_output()
        .map_err(|error| format!("Impossible de lire la sortie de commande: {error}"))?;

    let result = TerminalResult {
        shell,
        command: command_text.to_string(),
        success: output.status.success(),
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string(),
        stderr: String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string(),
        duration_ms: started.elapsed().as_millis(),
        risky,
        timestamp: now_seconds(),
    };

    let _ = history::append(HistoryEntry {
        shell: result.shell.clone(),
        command: result.command.clone(),
        success: result.success,
        exit_code: result.exit_code,
        stdout: result.stdout.clone(),
        stderr: result.stderr.clone(),
        duration_ms: result.duration_ms,
        timestamp: result.timestamp,
    });

    Ok(result)
}

#[tauri::command]
fn load_terminal_history() -> Result<Vec<HistoryEntry>, String> {
    history::load()
}

#[tauri::command]
fn clear_terminal_history() -> Result<(), String> {
    history::clear()
}

#[tauri::command]
fn launch_app(request: ActionRequest) -> Result<(), String> {
    actions::launch_executable(&request.target)
}

#[tauri::command]
fn open_install_folder(request: ActionRequest) -> Result<(), String> {
    actions::open_in_explorer(&request.target)
}

#[tauri::command]
fn uninstall_app(request: UninstallRequest) -> Result<(), String> {
    if !request.confirmed {
        return Err("Confirmation desinstallation requise.".to_string());
    }
    actions::run_uninstall(&request.uninstall_string)
}

#[tauri::command]
fn load_local_metadata() -> Result<MetadataMap, String> {
    metadata::load()
}

#[tauri::command]
fn save_local_metadata(request: MetadataUpdateRequest) -> Result<MetadataMap, String> {
    let mut map = metadata::load().unwrap_or_default();
    if request.metadata.is_empty() {
        map.remove(&request.app_id);
    } else {
        map.insert(request.app_id, request.metadata);
    }
    metadata::save(&map)?;
    Ok(map)
}

#[tauri::command]
fn get_app_icon(request: IconRequest) -> Result<IconResponse, String> {
    let cleaned = icons::clean_icon_path(&request.source_path);
    if let Some(cached) = icons::read_cached(&cleaned) {
        return Ok(IconResponse {
            app_id: request.app_id,
            base64_png: cached,
        });
    }

    let png = icons::extract_icon_png(&cleaned)?;
    icons::store_cache(&cleaned, &png)?;
    Ok(IconResponse {
        app_id: request.app_id,
        base64_png: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png),
    })
}

#[tauri::command]
fn check_software_updates() -> Result<WingetReport, String> {
    updates::check_winget_upgrades()
}

#[cfg(windows)]
fn collect_installed_apps() -> Result<Vec<InstalledApp>, String> {
    use winreg::enums::{
        HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY,
    };
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let mut apps = Vec::new();

    read_uninstall_key(
        &hklm,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        KEY_READ | KEY_WOW64_64KEY,
        "HKLM 64-bit",
        &mut apps,
    );
    read_uninstall_key(
        &hklm,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        KEY_READ | KEY_WOW64_32KEY,
        "HKLM 32-bit",
        &mut apps,
    );
    read_uninstall_key(
        &hkcu,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        KEY_READ,
        "HKCU",
        &mut apps,
    );

    let mut deduped = BTreeMap::new();
    for app in apps {
        let key = format!(
            "{}|{}|{}",
            app.name.to_lowercase(),
            app.publisher.clone().unwrap_or_default().to_lowercase(),
            app.version.clone().unwrap_or_default().to_lowercase()
        );
        deduped.entry(key).or_insert(app);
    }

    let mut apps: Vec<InstalledApp> = deduped.into_values().collect();
    apps.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
    Ok(apps)
}

#[cfg(not(windows))]
fn collect_installed_apps() -> Result<Vec<InstalledApp>, String> {
    Err("Le scan des logiciels est disponible uniquement sur Windows.".to_string())
}

#[cfg(windows)]
fn read_uninstall_key(
    root: &winreg::RegKey,
    path: &str,
    flags: u32,
    source: &str,
    apps: &mut Vec<InstalledApp>,
) {
    let Ok(key) = root.open_subkey_with_flags(path, flags) else {
        return;
    };

    for subkey_name in key.enum_keys().filter_map(Result::ok) {
        let Ok(subkey) = key.open_subkey_with_flags(&subkey_name, flags) else {
            continue;
        };

        let Some(name) = get_registry_string(&subkey, "DisplayName") else {
            continue;
        };

        if should_skip_entry(&subkey) {
            continue;
        }

        let publisher = get_registry_string(&subkey, "Publisher");
        let version = get_registry_string(&subkey, "DisplayVersion");
        let install_location = get_registry_string(&subkey, "InstallLocation");
        let install_date = get_registry_string(&subkey, "InstallDate").map(format_install_date);
        let estimated_size_mb = get_registry_u32(&subkey, "EstimatedSize")
            .map(|size_kb| (size_kb as u64 + 1023) / 1024);
        let uninstall_string = get_registry_string(&subkey, "UninstallString");
        let display_icon = get_registry_string(&subkey, "DisplayIcon");
        let icon_source = display_icon
            .as_ref()
            .map(|raw| icons::clean_icon_path(raw))
            .filter(|path| !path.trim().is_empty());
        let executable_path = guess_executable(&icon_source, install_location.as_deref());
        let category = categorize_app(&name, publisher.as_deref());

        apps.push(InstalledApp {
            id: format!("{source}:{subkey_name}"),
            name,
            version,
            publisher,
            install_location,
            install_date,
            estimated_size_mb,
            uninstall_string,
            icon_source,
            executable_path,
            source: source.to_string(),
            category,
            update_hint: "Verifie via winget".to_string(),
        });
    }
}

#[cfg(windows)]
fn guess_executable(
    icon_source: &Option<String>,
    install_location: Option<&str>,
) -> Option<String> {
    if let Some(icon) = icon_source {
        if icon.to_lowercase().ends_with(".exe") {
            let path = std::path::Path::new(icon);
            if path.exists() {
                return Some(icon.clone());
            }
        }
    }

    let location = install_location?.trim();
    if location.is_empty() {
        return None;
    }

    let dir = std::path::Path::new(location);
    if !dir.is_dir() {
        return None;
    }

    let mut candidates: Vec<std::path::PathBuf> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .map(|extension| extension.eq_ignore_ascii_case("exe"))
                        .unwrap_or(false)
            })
            .collect(),
        Err(_) => return None,
    };

    candidates.sort_by_key(|path| {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_lowercase();
        let demote = name.contains("uninstall")
            || name.contains("setup")
            || name.contains("install")
            || name.contains("update")
            || name.starts_with("unins");
        (demote as u8, name)
    });

    candidates
        .first()
        .and_then(|path| path.to_str().map(|s| s.to_string()))
}

#[cfg(windows)]
fn get_registry_string(key: &winreg::RegKey, value_name: &str) -> Option<String> {
    key.get_value::<String, _>(value_name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(windows)]
fn get_registry_u32(key: &winreg::RegKey, value_name: &str) -> Option<u32> {
    key.get_value::<u32, _>(value_name).ok()
}

#[cfg(windows)]
fn should_skip_entry(key: &winreg::RegKey) -> bool {
    get_registry_u32(key, "SystemComponent") == Some(1)
        || get_registry_string(key, "ParentKeyName").is_some()
        || matches!(
            get_registry_string(key, "ReleaseType")
                .unwrap_or_default()
                .to_lowercase()
                .as_str(),
            "hotfix" | "security update" | "update rollup"
        )
}

fn categorize_app(name: &str, publisher: Option<&str>) -> String {
    let haystack = format!("{} {}", name, publisher.unwrap_or_default()).to_lowercase();

    let rules = [
        (
            "Navigateurs",
            ["chrome", "firefox", "edge", "brave", "opera", "vivaldi"].as_slice(),
        ),
        (
            "Developpement",
            [
                "visual studio",
                "code",
                "git",
                "node",
                "python",
                "rust",
                "docker",
                "postman",
                "jetbrains",
                "sdk",
                "powershell",
            ]
            .as_slice(),
        ),
        (
            "Creation",
            [
                "adobe", "figma", "blender", "gimp", "obs", "davinci", "affinity",
            ]
            .as_slice(),
        ),
        (
            "Communication",
            ["discord", "slack", "teams", "zoom", "skype", "signal"].as_slice(),
        ),
        (
            "Jeux",
            [
                "steam",
                "epic games",
                "ubisoft",
                "riot",
                "battle.net",
                "gog",
            ]
            .as_slice(),
        ),
        (
            "Securite",
            [
                "defender",
                "vpn",
                "malware",
                "antivirus",
                "bitdefender",
                "avast",
            ]
            .as_slice(),
        ),
        (
            "Productivite",
            [
                "office",
                "word",
                "excel",
                "notion",
                "libreoffice",
                "pdf",
                "onedrive",
            ]
            .as_slice(),
        ),
        (
            "Systeme",
            [
                "driver",
                "runtime",
                "redistributable",
                "framework",
                "windows",
                "microsoft visual c++",
            ]
            .as_slice(),
        ),
    ];

    rules
        .iter()
        .find_map(|(category, needles)| {
            needles
                .iter()
                .any(|needle| haystack.contains(needle))
                .then(|| (*category).to_string())
        })
        .unwrap_or_else(|| "Autres".to_string())
}

fn format_install_date(value: String) -> String {
    if value.len() == 8 && value.chars().all(|character| character.is_ascii_digit()) {
        format!("{}-{}-{}", &value[0..4], &value[4..6], &value[6..8])
    } else {
        value
    }
}

fn current_timestamp() -> String {
    now_seconds().to_string()
}

fn now_seconds() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_installed_apps,
            run_terminal_command,
            load_terminal_history,
            clear_terminal_history,
            check_command_risk,
            launch_app,
            open_install_folder,
            uninstall_app,
            load_local_metadata,
            save_local_metadata,
            get_app_icon,
            check_software_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::{categorize_app, format_install_date};

    #[test]
    fn categorizes_browser_from_name() {
        assert_eq!(categorize_app("Mozilla Firefox", None), "Navigateurs");
    }

    #[test]
    fn categorizes_developer_tool_from_publisher() {
        assert_eq!(
            categorize_app("Code", Some("Microsoft Corporation")),
            "Developpement"
        );
    }

    #[test]
    fn keeps_unknown_apps_in_other_category() {
        assert_eq!(categorize_app("Unlisted Tool", None), "Autres");
    }

    #[test]
    fn formats_registry_install_date() {
        assert_eq!(format_install_date("20260506".to_string()), "2026-05-06");
    }

    #[test]
    fn keeps_non_registry_date_as_is() {
        assert_eq!(format_install_date("06/05/2026".to_string()), "06/05/2026");
    }
}

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use wait_timeout::ChildExt;

const TERMINAL_TIMEOUT_SECONDS: u64 = 30;

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
fn run_terminal_command(request: TerminalRequest) -> Result<TerminalResult, String> {
    let shell = request.shell.trim().to_lowercase();
    let command = request.command.trim();

    if command.is_empty() {
        return Err("Commande vide.".to_string());
    }

    let started = Instant::now();
    let mut process = match shell.as_str() {
        "powershell" => {
            let mut process = Command::new("powershell.exe");
            process.args([
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                command,
            ]);
            process
        }
        "cmd" => {
            let mut process = Command::new("cmd.exe");
            process.args(["/C", command]);
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

    Ok(TerminalResult {
        shell,
        command: command.to_string(),
        success: output.status.success(),
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string(),
        stderr: String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string(),
        duration_ms: started.elapsed().as_millis(),
    })
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
            source: source.to_string(),
            category,
            update_hint: "Verification manuelle non lancee".to_string(),
        });
    }
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
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

#[cfg(windows)]
fn hide_console_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_console_window(_command: &mut Command) {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_installed_apps,
            run_terminal_command
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

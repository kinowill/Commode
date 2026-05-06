use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::process::Command;

pub const TERMINAL_TIMEOUT_SECONDS: u64 = 30;

pub fn is_risky_command(command: &str) -> bool {
    let lowered = command.to_lowercase();

    const RISKY_TOKENS: &[&str] = &[
        "rm ",
        "rm -",
        "rmdir ",
        "del ",
        "del /",
        "erase ",
        "format ",
        "format:",
        "diskpart",
        "shutdown",
        "logoff",
        "taskkill",
        "stop-process",
        "remove-item",
        "ri ",
        "rd ",
        "reg delete",
        "reg add",
        "regedit",
        "schtasks",
        "net user",
        "net localgroup",
        "bcdedit",
        "cipher /w",
        "fsutil",
        "robocopy /mir",
        "wevtutil cl",
        "clear-eventlog",
        "uninstall",
        "msiexec /x",
        "wmic",
        " > ",
        " >> ",
        "icacls",
        "takeown",
        "sfc /scannow",
        "dism ",
    ];

    RISKY_TOKENS.iter().any(|token| lowered.contains(token))
}

pub fn powershell_encoded_command(command: &str) -> String {
    let script = powershell_script(command);
    let bytes: Vec<u8> = script
        .encode_utf16()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    BASE64.encode(bytes)
}

fn powershell_script(command: &str) -> String {
    format!(
        "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; \
         $OutputEncoding = [System.Text.Encoding]::UTF8; \
         Import-Module Microsoft.PowerShell.Management,Microsoft.PowerShell.Utility \
         -ErrorAction SilentlyContinue; \
         {command}"
    )
}

#[cfg(windows)]
pub fn hide_console_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
pub fn hide_console_window(_command: &mut Command) {}

#[cfg(test)]
mod tests {
    use super::{is_risky_command, powershell_encoded_command, powershell_script};

    #[test]
    fn flags_remove_item() {
        assert!(is_risky_command("Remove-Item -Recurse -Force C:\\Temp"));
    }

    #[test]
    fn flags_format() {
        assert!(is_risky_command("format C:"));
    }

    #[test]
    fn flags_reg_delete() {
        assert!(is_risky_command("reg delete HKCU\\Software\\Foo /f"));
    }

    #[test]
    fn allows_status_commands() {
        assert!(!is_risky_command("Get-Process | Select-Object -First 5"));
        assert!(!is_risky_command("winget --version"));
        assert!(!is_risky_command("dir"));
    }

    #[test]
    fn flags_msiexec_uninstall() {
        assert!(is_risky_command(
            "msiexec /x{00000000-0000-0000-0000-000000000000}"
        ));
    }

    #[test]
    fn powershell_script_imports_standard_modules() {
        let script = powershell_script("Get-Process");
        assert!(script.contains("Import-Module Microsoft.PowerShell.Management"));
        assert!(script.contains("Get-Process"));
    }

    #[test]
    fn powershell_encoded_command_is_not_empty() {
        assert!(!powershell_encoded_command("Get-Process").is_empty());
    }
}

use crate::terminal::{hide_console_window, TERMINAL_TIMEOUT_SECONDS};
use serde::Serialize;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

const WINGET_TIMEOUT_SECONDS: u64 = 60;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WingetUpgrade {
    pub name: String,
    pub id: String,
    pub current_version: String,
    pub available_version: String,
    pub source: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WingetReport {
    pub success: bool,
    pub checked_at: u64,
    pub upgrades: Vec<WingetUpgrade>,
    pub raw_output: String,
    pub message: Option<String>,
}

pub fn check_winget_upgrades() -> Result<WingetReport, String> {
    let mut command = Command::new("winget");
    command.args(["upgrade", "--include-unknown", "--accept-source-agreements"]);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    hide_console_window(&mut command);

    let mut child = command
        .spawn()
        .map_err(|error| format!("winget introuvable: {error}"))?;

    let waited = child
        .wait_timeout(Duration::from_secs(
            WINGET_TIMEOUT_SECONDS.max(TERMINAL_TIMEOUT_SECONDS),
        ))
        .map_err(|error| format!("Attente winget impossible: {error}"))?;

    if waited.is_none() {
        let _ = child.kill();
        let _ = child.wait();
        return Err(format!(
            "winget arrete apres {WINGET_TIMEOUT_SECONDS} secondes."
        ));
    }

    let output = child
        .wait_with_output()
        .map_err(|error| format!("Lecture sortie winget: {error}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() && stdout.trim().is_empty() {
        return Ok(WingetReport {
            success: false,
            checked_at: now(),
            upgrades: Vec::new(),
            raw_output: stderr.clone(),
            message: Some(format!(
                "winget a echoue (code {}).",
                output.status.code().unwrap_or(-1)
            )),
        });
    }

    let upgrades = parse_winget_upgrade_table(&stdout);

    Ok(WingetReport {
        success: true,
        checked_at: now(),
        upgrades,
        raw_output: stdout,
        message: None,
    })
}

fn now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn parse_winget_upgrade_table(output: &str) -> Vec<WingetUpgrade> {
    let mut lines = output.lines().peekable();
    let mut header_line: Option<&str> = None;

    while let Some(line) = lines.next() {
        let lower = line.to_lowercase();
        if lower.contains("name") && lower.contains("id") && lower.contains("version") {
            header_line = Some(line);
            break;
        }
    }

    let Some(header) = header_line else {
        return Vec::new();
    };

    let columns = match find_columns(header) {
        Some(value) => value,
        None => return Vec::new(),
    };

    if let Some(next) = lines.peek() {
        if next.trim_start().starts_with('-') {
            lines.next();
        }
    }

    let mut upgrades = Vec::new();

    for raw in lines {
        let line = raw.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }

        let lower = line.to_lowercase();
        if lower.contains("upgrades available")
            || lower.contains("paquets ")
            || lower.contains("the following packages")
            || lower.contains("no applicable")
            || lower.contains("no installed package")
            || lower.starts_with("avertissement")
            || lower.starts_with("warning")
        {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let name = slice_chars(&chars, columns.name, columns.id);
        let id = slice_chars(&chars, columns.id, columns.version);
        let current_version = slice_chars(&chars, columns.version, columns.available);
        let available_version = slice_chars(
            &chars,
            columns.available,
            columns.source.unwrap_or(chars.len()),
        );
        let source = match columns.source {
            Some(start) => slice_chars(&chars, start, chars.len()),
            None => String::new(),
        };

        if name.trim().is_empty() || id.trim().is_empty() {
            continue;
        }
        if current_version.trim().is_empty() && available_version.trim().is_empty() {
            continue;
        }

        upgrades.push(WingetUpgrade {
            name: name.trim().to_string(),
            id: id.trim().to_string(),
            current_version: current_version.trim().to_string(),
            available_version: available_version.trim().to_string(),
            source: source.trim().to_string(),
        });
    }

    upgrades
}

#[derive(Debug)]
struct ColumnLayout {
    name: usize,
    id: usize,
    version: usize,
    available: usize,
    source: Option<usize>,
}

fn find_columns(header: &str) -> Option<ColumnLayout> {
    let lower = header.to_lowercase();
    let chars: Vec<char> = header.chars().collect();

    let name = char_index(&lower, "name", &chars)?;
    let id = char_index(&lower, "id", &chars)?;
    let version = char_index(&lower, "version", &chars)?;
    let available = char_index(&lower, "available", &chars).or_else(|| {
        char_index(&lower, "newer", &chars).or_else(|| char_index(&lower, "disponible", &chars))
    })?;
    let source = char_index(&lower, "source", &chars);

    if id <= name || version <= id || available <= version {
        return None;
    }

    Some(ColumnLayout {
        name,
        id,
        version,
        available,
        source,
    })
}

fn char_index(haystack_lower: &str, needle: &str, chars: &[char]) -> Option<usize> {
    let byte_index = haystack_lower.find(needle)?;
    let mut count = 0usize;
    let mut total_bytes = 0usize;
    for ch in chars {
        if total_bytes >= byte_index {
            return Some(count);
        }
        total_bytes += ch.len_utf8();
        count += 1;
    }
    Some(count)
}

fn slice_chars(chars: &[char], start: usize, end: usize) -> String {
    let end = end.min(chars.len());
    if start >= end {
        return String::new();
    }
    chars[start..end].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::parse_winget_upgrade_table;

    #[test]
    fn parses_single_row() {
        let output = "\
Name                       Id                            Version       Available    Source
-----------------------------------------------------------------------------------------
Visual Studio Code         Microsoft.VisualStudioCode    1.85.0        1.86.0       winget
";
        let upgrades = parse_winget_upgrade_table(output);
        assert_eq!(upgrades.len(), 1);
        assert_eq!(upgrades[0].name, "Visual Studio Code");
        assert_eq!(upgrades[0].id, "Microsoft.VisualStudioCode");
        assert_eq!(upgrades[0].current_version, "1.85.0");
        assert_eq!(upgrades[0].available_version, "1.86.0");
        assert_eq!(upgrades[0].source, "winget");
    }

    #[test]
    fn skips_summary_lines() {
        let output = "\
Name                       Id                            Version       Available    Source
-----------------------------------------------------------------------------------------
Foo                        Vendor.Foo                    1.0           1.1          winget

3 upgrades available.
";
        let upgrades = parse_winget_upgrade_table(output);
        assert_eq!(upgrades.len(), 1);
    }

    #[test]
    fn returns_empty_on_no_header() {
        assert!(parse_winget_upgrade_table("nothing here").is_empty());
    }
}

use crate::storage;
use serde::{Deserialize, Serialize};
use std::fs;

const HISTORY_LIMIT: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub shell: String,
    pub command: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
    pub timestamp: u64,
}

pub fn load() -> Result<Vec<HistoryEntry>, String> {
    let path = storage::terminal_history_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content =
        fs::read_to_string(&path).map_err(|error| format!("Lecture historique: {error}"))?;
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&content).map_err(|error| format!("Parsing historique: {error}"))
}

pub fn save(entries: &[HistoryEntry]) -> Result<(), String> {
    let limited: Vec<&HistoryEntry> = entries.iter().take(HISTORY_LIMIT).collect();
    let path = storage::terminal_history_path()?;
    let serialized = serde_json::to_string_pretty(&limited)
        .map_err(|error| format!("Serialisation historique: {error}"))?;
    fs::write(&path, serialized).map_err(|error| format!("Ecriture historique: {error}"))
}

pub fn append(new_entry: HistoryEntry) -> Result<Vec<HistoryEntry>, String> {
    let mut entries = load().unwrap_or_default();
    entries.insert(0, new_entry);
    if entries.len() > HISTORY_LIMIT {
        entries.truncate(HISTORY_LIMIT);
    }
    save(&entries)?;
    Ok(entries)
}

pub fn clear() -> Result<(), String> {
    let path = storage::terminal_history_path()?;
    if path.exists() {
        fs::remove_file(&path).map_err(|error| format!("Suppression historique: {error}"))?;
    }
    Ok(())
}

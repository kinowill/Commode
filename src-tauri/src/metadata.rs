use crate::storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalMetadata {
    #[serde(default)]
    pub custom_category: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub hidden: bool,
}

impl LocalMetadata {
    pub fn is_empty(&self) -> bool {
        self.custom_category.is_none() && self.note.is_none() && !self.favorite && !self.hidden
    }
}

pub type MetadataMap = HashMap<String, LocalMetadata>;

pub fn load() -> Result<MetadataMap, String> {
    let path = storage::metadata_path()?;
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let content =
        fs::read_to_string(&path).map_err(|error| format!("Lecture metadata: {error}"))?;
    if content.trim().is_empty() {
        return Ok(HashMap::new());
    }

    serde_json::from_str(&content).map_err(|error| format!("Parsing metadata: {error}"))
}

pub fn save(map: &MetadataMap) -> Result<(), String> {
    let cleaned: MetadataMap = map
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();

    let path = storage::metadata_path()?;
    let serialized = serde_json::to_string_pretty(&cleaned)
        .map_err(|error| format!("Serialisation metadata: {error}"))?;
    fs::write(&path, serialized).map_err(|error| format!("Ecriture metadata: {error}"))
}

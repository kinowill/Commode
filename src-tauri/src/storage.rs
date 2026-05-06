use std::fs;
use std::path::PathBuf;

const APP_DIR_NAME: &str = "Commode";

pub fn app_data_dir() -> Result<PathBuf, String> {
    let base =
        dirs::data_local_dir().ok_or_else(|| "Repertoire LocalAppData introuvable.".to_string())?;
    let dir = base.join(APP_DIR_NAME);
    fs::create_dir_all(&dir).map_err(|error| format!("Creation repertoire: {error}"))?;
    Ok(dir)
}

pub fn icons_dir() -> Result<PathBuf, String> {
    let dir = app_data_dir()?.join("icons");
    fs::create_dir_all(&dir).map_err(|error| format!("Creation repertoire icones: {error}"))?;
    Ok(dir)
}

pub fn metadata_path() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("metadata.json"))
}

pub fn terminal_history_path() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("terminal_history.json"))
}

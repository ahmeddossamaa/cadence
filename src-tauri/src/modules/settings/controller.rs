use crate::db::Db;
use crate::types::settings::Settings;
use std::sync::Arc;

#[tauri::command]
pub fn settings_get(db: tauri::State<'_, Arc<Db>>) -> Result<Settings, String> {
    db.get_all_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn settings_update(
    db: tauri::State<'_, Arc<Db>>,
    settings: Settings,
) -> Result<Settings, String> {
    db.update_settings(&settings).map_err(|e| e.to_string())?;
    db.get_all_settings().map_err(|e| e.to_string())
}

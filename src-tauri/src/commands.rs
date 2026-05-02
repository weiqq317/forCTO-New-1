use tauri::State;
use walkdir::WalkDir;
use crate::models::Photo;
use crate::AppState;
use crate::search;

#[tauri::command]
pub fn get_axum_port(state: State<'_, AppState>) -> u16 {
    state.axum_port
}

#[tauri::command]
pub fn get_photos(state: State<'_, AppState>, limit: i64, offset: i64) -> Result<Vec<Photo>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_photos_paginated(limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_photos(state: State<'_, AppState>, query: String, limit: i64, offset: i64) -> Result<Vec<Photo>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    search::search_photos(&db, &query, limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_folder(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(folder_path) = rfd::AsyncFileDialog::new().pick_folder().await {
        let path = folder_path.path().to_path_buf();

        let mut files_to_insert = Vec::new();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_lowercase();
                    if ["png", "jpg", "jpeg", "gif", "webp"].contains(&ext.as_str()) {
                        files_to_insert.push((
                            entry.path().to_string_lossy().to_string(),
                            ext
                        ));
                    }
                }
            }
        }

        let db = state.db.lock().map_err(|e| e.to_string())?;
        for (file, ext) in files_to_insert {
            let _ = db.insert_photo(&file, None, None, None, Some(&ext), None);
        }
    }
    Ok(())
}
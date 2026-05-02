// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod server;
mod models;
mod cache_manager;

use std::sync::Mutex;
use tauri::State;
use walkdir::WalkDir;
use models::Photo;
use cache_manager::CacheManager;

struct AppState {
    db: Mutex<db::Db>,
    cache: Mutex<CacheManager>,
    axum_port: u16,
}

#[tauri::command]
fn get_axum_port(state: State<'_, AppState>) -> u16 {
    state.axum_port
}

#[tauri::command]
fn get_photos(state: State<'_, AppState>, limit: i64, offset: i64) -> Result<Vec<Photo>, String> {
    let db = state.db.lock().unwrap();
    db.get_photos_paginated(limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
fn search_photos(state: State<'_, AppState>, query: String) -> Result<Vec<Photo>, String> {
    let db = state.db.lock().unwrap();
    db.search_photos(&query).map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_folder(state: State<'_, AppState>) -> Result<(), String> {
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

        let db = state.db.lock().unwrap();
        for (file, ext) in files_to_insert {
            let _ = db.insert_photo(&file, None, None, None, Some(&ext));
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let axum_port = server::start_server().await;

    tauri::Builder::default()
        .setup(move |app| {
            let app_dir = app
                .path_resolver()
                .app_data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            std::fs::create_dir_all(&app_dir).unwrap();
            let db_path = app_dir.join("index.db");
            let db = db::Db::new(db_path).expect("Failed to initialize database");
            let cache_dir = app_dir.join("cache");
            let cache = CacheManager::new(cache_dir, 1000, 100);

            app.manage(AppState {
                db: Mutex::new(db),
                cache: Mutex::new(cache),
                axum_port,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_axum_port,
            get_photos,
            search_photos,
            import_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

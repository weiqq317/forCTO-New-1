// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod web_server;
mod models;
mod backup;

use std::sync::Mutex;
use tauri::State;
use walkdir::WalkDir;
use models::Photo;
use backup::{BackupSystem, BackupTarget};

struct AppState {
    db: Mutex<db::Db>,
    axum_port: u16,
}

#[tauri::command]
fn get_axum_port(state: State<'_, AppState>) -> u16 {
    state.axum_port
}

#[tauri::command]
fn fetch_media(state: State<'_, AppState>) -> Result<Vec<Photo>, String> {
    let db = state.db.lock().unwrap();
    db.get_photos_paginated(100, 0).map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_directory(state: State<'_, AppState>) -> Result<(), String> {
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

#[tauri::command]
async fn trigger_backup(state: State<'_, AppState>, target: BackupTarget) -> Result<(), String> {
    let photos = {
        let db = state.db.lock().unwrap();
        db.get_photos_paginated(10000, 0).map_err(|e| e.to_string())?
    };
    
    let backup_sys = BackupSystem::new(target);
    
    for photo in photos {
        let path = std::path::PathBuf::from(&photo.path);
        if let Some(file_name) = path.file_name() {
            let remote_path = format!("{}", file_name.to_string_lossy());
            backup_sys.backup_file(&path, &remote_path).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let app_dir = app
                .path_resolver()
                .app_data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            std::fs::create_dir_all(&app_dir).unwrap();
            let db_path = app_dir.join("index.db");
            let db = db::Db::new(db_path.clone()).expect("Failed to initialize database");

            let (tx, rx) = std::sync::mpsc::channel();
            let db_path_clone = db_path.clone();
            tokio::spawn(async move {
                let port = web_server::start_server(db_path_clone).await;
                let _ = tx.send(port);
            });
            let axum_port = rx.recv().expect("Failed to get port from web server task");

            app.manage(AppState {
                db: Mutex::new(db),
                axum_port,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_axum_port,
            fetch_media,
            import_directory,
            trigger_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod web_server;
mod models;
mod backup;

use std::sync::{Arc, Mutex};
use tauri::State;
use walkdir::WalkDir;
use models::Photo;

struct AppState {
    db: Arc<Mutex<db::Db>>,
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
async fn trigger_backup(state: State<'_, AppState>) -> Result<(), String> {
    let photos = {
        let db = state.db.lock().unwrap();
        // Just get all photos for this simplified example
        db.get_photos_paginated(10000, 0).map_err(|e| e.to_string())?
    };

    // Note: For a real app, these values would come from settings.
    // We demonstrate S3 and WebDAV usage here per requirements.
    // For now we'll create a dummy target just to verify compilation and integration.
    let target = backup::BackupTarget::S3 {
        endpoint: "http://localhost:9000".into(),
        bucket: "backups".into(),
        access_key: "minioadmin".into(),
        secret_key: "minioadmin".into(),
        region: "us-east-1".into(),
    };

    backup::backup_photos(photos, target).await.map_err(|e| format!("{:?}", e))?;
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
            
            let db = db::Db::new(db_path).expect("Failed to initialize database");
            let db_arc = Arc::new(Mutex::new(db));

            // Start axum server and pass db reference
            let axum_port = tauri::async_runtime::block_on(async {
                web_server::start_server(db_arc.clone()).await
            });

            app.manage(AppState {
                db: db_arc,
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

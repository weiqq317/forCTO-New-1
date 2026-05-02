// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod web_server;
mod backup;

use std::sync::{Arc, Mutex};
use tauri::State;
use walkdir::WalkDir;
use models::Photo;
use backup::{BackupSystem, BackupTarget};
use std::path::Path;

struct AppState {
    db: Arc<Mutex<db::Db>>,
    axum_port: u16,
    backup_system: Arc<Mutex<Option<BackupSystem>>>,
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
fn configure_backup_s3(
    state: State<'_, AppState>,
    bucket: String,
    region: String,
    endpoint_url: Option<String>,
    access_key: String,
    secret_key: String,
) -> Result<(), String> {
    let target = BackupTarget::S3 { bucket, region, endpoint_url, access_key, secret_key };
    let mut bs = state.backup_system.lock().unwrap();
    *bs = Some(BackupSystem::new(target));
    Ok(())
}

#[tauri::command]
async fn trigger_backup(state: State<'_, AppState>) -> Result<(), String> {
    let backup_system = {
        let bs_guard = state.backup_system.lock().unwrap();
        if bs_guard.is_none() {
            return Err("Backup system not configured".to_string());
        }
        BackupSystem::new(bs_guard.as_ref().unwrap().clone_target())
    };

    let photos = {
        let db = state.db.lock().unwrap();
        db.get_photos_paginated(10000, 0).map_err(|e| e.to_string())?
    };

    for photo in photos {
        let path = Path::new(&photo.path);
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            let remote_path = format!("backup/{}", file_name);
            let _ = backup_system.upload_file(path, &remote_path).await;
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
            let db = db::Db::new(db_path).expect("Failed to initialize database");
            let db_arc = Arc::new(Mutex::new(db));

            // Run the server initialization
            let axum_port = web_server::start_server(db_arc.clone());

            app.manage(AppState {
                db: db_arc,
                axum_port,
                backup_system: Arc::new(Mutex::new(None)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_axum_port,
            fetch_media,
            import_directory,
            configure_backup_s3,
            trigger_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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
async fn run_backup(
    state: State<'_, AppState>,
    target_type: String, // "s3" or "webdav"
    url_or_bucket: String,
    region: Option<String>,
    username_or_access_key: Option<String>,
    password_or_secret: Option<String>,
) -> Result<String, String> {
    let target = match target_type.as_str() {
        "s3" => backup::BackupTarget::S3 {
            bucket: url_or_bucket,
            region: region.unwrap_or_else(|| "us-east-1".to_string()),
            access_key: username_or_access_key.unwrap_or_default(),
            secret_key: password_or_secret.unwrap_or_default(),
            endpoint_url: None,
        },
        "webdav" => backup::BackupTarget::WebDav {
            url: url_or_bucket,
            username: username_or_access_key,
            password: password_or_secret,
        },
        _ => return Err("Invalid target type".to_string()),
    };

    let backup_system = backup::BackupSystem::new(target);

    // Fetch all photos
    let photos = {
        let db = state.db.lock().unwrap();
        db.get_photos_paginated(10000, 0).map_err(|e| e.to_string())?
    };

    let mut success_count = 0;
    for photo in photos {
        let path = std::path::Path::new(&photo.path);
        if let Some(file_name) = path.file_name() {
            let remote_path = file_name.to_string_lossy().to_string();
            match backup_system.backup_file(&photo.path, &remote_path).await {
                Ok(_) => success_count += 1,
                Err(e) => eprintln!("Failed to backup {}: {}", photo.path, e),
            }
        }
    }

    Ok(format!("Successfully backed up {} files", success_count))
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path_resolver()
                .app_data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            std::fs::create_dir_all(&app_dir).unwrap();
            let db_path = app_dir.join("index.db");
            let db = db::Db::new(db_path).expect("Failed to initialize database");
            
            let db_arc = std::sync::Arc::new(std::sync::Mutex::new(db));
            
            let db_for_server = db_arc.clone();
            
            let axum_port = tauri::async_runtime::block_on(async {
                web_server::start_server(db_for_server).await
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
            run_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod server;
mod models;
mod media_processor;
mod ai_pipeline;

use std::sync::{Arc, Mutex};
use tauri::State;
use models::Photo;

struct AppState {
    db: Arc<Mutex<db::Db>>,
    axum_port: u16,
    ai_pipeline: Arc<ai_pipeline::AiPipeline>,
    cache_dir: std::path::PathBuf,
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
        let path_str = path.to_string_lossy().to_string();
        let cache_dir = state.cache_dir.clone();
        
        let results = media_processor::import_folder(&path_str, &cache_dir).await?;
        
        for info in results {
            let id = {
                let db = state.db.lock().unwrap();
                db.insert_photo(
                    &info.path,
                    Some(&info.hash),
                    info.width.map(|w| w as i64),
                    info.height.map(|h| h as i64),
                    Some(&info.ext)
                ).unwrap_or(0)
            };
            
            if id > 0 {
                let id_str = id.to_string();
                // Send tasks to pipeline
                state.ai_pipeline.submit_task(ai_pipeline::AiTask {
                    photo_id: id_str.clone(),
                    task_type: "CLIP".to_string(),
                }).await;
                state.ai_pipeline.submit_task(ai_pipeline::AiTask {
                    photo_id: id_str.clone(),
                    task_type: "FACE".to_string(),
                }).await;
                state.ai_pipeline.submit_task(ai_pipeline::AiTask {
                    photo_id: id_str.clone(),
                    task_type: "OCR".to_string(),
                }).await;
            }
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
            
            let cache_dir = app_dir.join("thumbnails");
            std::fs::create_dir_all(&cache_dir).unwrap();
            
            let db_path = app_dir.join("index.db");
            let db = Arc::new(Mutex::new(db::Db::new(db_path).expect("Failed to initialize database")));
            
            let pipeline = Arc::new(ai_pipeline::AiPipeline::new(db.clone(), 4));

            app.manage(AppState {
                db,
                axum_port,
                ai_pipeline: pipeline,
                cache_dir,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_axum_port,
            fetch_media,
            import_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

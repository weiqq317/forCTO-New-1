// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod server;
mod models;
mod search;
mod commands;

use std::sync::Mutex;

pub struct AppState {
    db: Mutex<db::Db>,
    axum_port: u16,
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

            app.manage(AppState {
                db: Mutex::new(db),
                axum_port,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_axum_port,
            commands::get_photos,
            commands::search_photos,
            commands::import_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

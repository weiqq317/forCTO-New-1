use axum::{
    extract::{Query, State},
    http::{header::AUTHORIZATION, StatusCode, HeaderMap},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::db::Db;
use crate::models::Photo;

pub struct AppState {
    pub db: Arc<Mutex<Db>>,
    // Add credentials for basic auth. For simplicity, we hardcode or read from env.
    // Here we just use a dummy check or allow configuration.
}

#[derive(Deserialize)]
struct MediaParams {
    path: String,
}

#[derive(Deserialize)]
struct ListParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

fn check_auth(headers: &HeaderMap) -> bool {
    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Basic ") {
                let b64 = &auth_str[6..];
                use base64::{Engine as _, engine::general_purpose};
                if let Ok(decoded) = general_purpose::STANDARD.decode(b64) {
                    if let Ok(creds) = String::from_utf8(decoded) {
                        // "admin:password"
                        let parts: Vec<&str> = creds.splitn(2, ':').collect();
                        if parts.len() == 2 && parts[0] == "admin" && parts[1] == "password" {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

async fn serve_media(
    headers: HeaderMap,
    Query(params): Query<MediaParams>
) -> impl IntoResponse {
    if !check_auth(&headers) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let path = std::path::PathBuf::from(params.path);
    if path.exists() && path.is_file() {
        match std::fs::read(&path) {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                ([(axum::http::header::CONTENT_TYPE, mime.as_ref())], bytes).into_response()
            }
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Not Found").into_response()
    }
}

async fn list_photos(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    if !check_auth(&headers) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);

    let db_lock = match state.db.lock() {
        Ok(lock) => lock,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB lock failed").into_response(),
    };

    match db_lock.get_photos_paginated(limit, offset) {
        Ok(photos) => (StatusCode::OK, Json(photos)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB query failed").into_response(),
    }
}

pub async fn start_server(db: Arc<Mutex<Db>>) -> u16 {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let shared_state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/media", get(serve_media))
        .route("/photos", get(list_photos))
        .layer(cors)
        .with_state(shared_state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    port
}

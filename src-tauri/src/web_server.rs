use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Router,
    http::{header, StatusCode},
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use base64::{Engine as _, engine::general_purpose::STANDARD};

use crate::db::Db;

#[derive(Deserialize)]
pub struct MediaParams {
    pub path: String,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub struct ServerState {
    pub db: Arc<Mutex<Db>>,
    pub expected_auth: String,
}

pub async fn start_server(db: Arc<Mutex<Db>>, auth_user: &str, auth_pass: &str) -> u16 {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let auth_str = format!("{}:{}", auth_user, auth_pass);
    let expected_auth = format!("Basic {}", STANDARD.encode(auth_str.as_bytes()));

    let state = Arc::new(ServerState { db, expected_auth });

    let app = Router::new()
        .route("/media", get(serve_media))
        .route("/photos", get(list_photos))
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    port
}

fn check_auth(headers: &axum::http::HeaderMap, state: &Arc<ServerState>) -> bool {
    if let Some(auth_val) = headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_val.to_str() {
            return auth_str == state.expected_auth;
        }
    }
    false
}

async fn serve_media(
    State(state): State<Arc<ServerState>>,
    headers: axum::http::HeaderMap,
    Query(params): Query<MediaParams>,
) -> impl IntoResponse {
    if !check_auth(&headers, &state) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let path = std::path::PathBuf::from(params.path);
    if path.exists() && path.is_file() {
        match std::fs::read(&path) {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], bytes).into_response()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn list_photos(
    State(state): State<Arc<ServerState>>,
    headers: axum::http::HeaderMap,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    if !check_auth(&headers, &state) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);

    let db = state.db.lock().unwrap();
    match db.get_photos_paginated(limit, offset) {
        Ok(photos) => (StatusCode::OK, axum::Json(photos)).into_response(),
        Err(e) => {
            eprintln!("DB Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::Response;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::db::Db;
use crate::models::Photo;

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Mutex<Db>>,
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

#[derive(Serialize)]
struct ListResponse {
    photos: Vec<Photo>,
}

async fn basic_auth(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(auth_header) if auth_header.starts_with("Basic ") => {
            let b64_credentials = &auth_header[6..];
            if let Ok(decoded) = BASE64.decode(b64_credentials) {
                if let Ok(credentials) = String::from_utf8(decoded) {
                    // For demo purposes, we're hardcoding admin:admin
                    // In a real app, you would verify against database or settings
                    if credentials == "admin:admin" {
                        return Ok(next.run(req).await);
                    }
                }
            }
            Err(StatusCode::UNAUTHORIZED)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn list_photos(
    State(state): State<ServerState>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);
    
    let db = state.db.lock().unwrap();
    match db.get_photos_paginated(limit, offset) {
        Ok(photos) => axum::Json(ListResponse { photos }).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn serve_media(Query(params): Query<MediaParams>) -> impl IntoResponse {
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

pub async fn start_server(db: Arc<Mutex<Db>>) -> u16 {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let state = ServerState { db };

    // We can add auth to some routes and not others
    // For now we'll put both list and media behind auth for security
    let app = Router::new()
        .route("/api/photos", get(list_photos))
        .route("/media", get(serve_media))
        .layer(middleware::from_fn(basic_auth))
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    port
}

use axum::{
    extract::{Query, State, Request},
    http::{header, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router, Json,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use base64::{engine::general_purpose, Engine as _};

use crate::db::Db;

#[derive(Clone)]
struct ServerState {
    db: Arc<Mutex<Db>>,
}

#[derive(Deserialize)]
struct MediaParams {
    path: String,
}

#[derive(Deserialize)]
struct PaginationParams {
    limit: Option<i64>,
    offset: Option<i64>,
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

async fn list_photos(
    State(state): State<ServerState>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);
    
    let db = state.db.lock().unwrap();
    match db.get_photos_paginated(limit, offset) {
        Ok(photos) => (StatusCode::OK, Json(photos)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn basic_auth(req: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    let auth_header = req.headers().get(header::AUTHORIZATION);
    if let Some(auth_header) = auth_header {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Basic ") {
                let b64 = auth_str.trim_start_matches("Basic ");
                if let Ok(decoded) = general_purpose::STANDARD.decode(b64) {
                    if let Ok(credentials) = String::from_utf8(decoded) {
                        if credentials == "admin:admin" {
                            return Ok(next.run(req).await);
                        }
                    }
                }
            }
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}

pub fn start_server(db: Arc<Mutex<Db>>) -> u16 {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let state = ServerState { db };

    let app = Router::new()
        .route("/media", get(serve_media))
        .route("/photos", get(list_photos))
        .layer(middleware::from_fn(basic_auth))
        .layer(cors)
        .with_state(state);

    let std_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    std_listener.set_nonblocking(true).unwrap();
    let port = std_listener.local_addr().unwrap().port();
    
    tokio::spawn(async move {
        let listener = TcpListener::from_std(std_listener).unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    port
}

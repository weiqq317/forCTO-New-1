use axum::{
    extract::{Query, State},
    http::{header::AUTHORIZATION, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::Response;
use base64::{engine::general_purpose, Engine as _};

use crate::db::Db;
use crate::models::Photo;

#[derive(Clone)]
struct AppState {
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

async fn basic_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(AUTHORIZATION).and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Basic ") {
            let credentials = auth_header.trim_start_matches("Basic ");
            if let Ok(decoded) = general_purpose::STANDARD.decode(credentials) {
                if let Ok(decoded_str) = String::from_utf8(decoded) {
                    let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
                    if parts.len() == 2 && parts[0] == "admin" && parts[1] == "password" {
                        return Ok(next.run(req).await);
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

async fn serve_media(Query(params): Query<MediaParams>) -> impl IntoResponse {
    let path = PathBuf::from(params.path);
    if path.exists() && path.is_file() {
        match std::fs::read(&path) {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                ([(axum::http::header::CONTENT_TYPE, mime.as_ref())], bytes).into_response()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn list_photos(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Photo>>, StatusCode> {
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);

    let db = state.db.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match db.get_photos_paginated(limit, offset) {
        Ok(photos) => Ok(Json(photos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn start_server(db_path: PathBuf) -> u16 {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let db = Db::new(db_path).expect("Failed to init db for web server");
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    let app = Router::new()
        .route("/media", get(serve_media))
        .route("/photos", get(list_photos))
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

use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct MediaParams {
    path: String,
}

async fn serve_media(Query(params): Query<MediaParams>) -> impl IntoResponse {
    let path = std::path::PathBuf::from(params.path);
    if path.exists() && path.is_file() {
        match std::fs::read(&path) {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                ([(axum::http::header::CONTENT_TYPE, mime.as_ref())], bytes).into_response()
            }
            Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn start_server() -> u16 {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().route("/media", get(serve_media)).layer(cors);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    port
}

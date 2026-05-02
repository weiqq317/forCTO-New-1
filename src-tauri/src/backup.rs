use crate::models::Photo;
use crate::db::Db;
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use std::path::Path;
use std::io::Read;
use reqwest::Client as ReqwestClient;
use tokio_util::io::ReaderStream;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::Region;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use tauri::State;

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum BackupConfig {
    S3 {
        bucket: String,
        endpoint: Option<String>,
        region: Option<String>,
    },
    WebDav {
        base_url: String,
        username: Option<String>,
        password: Option<String>,
    }
}

fn calculate_hash(path: &Path) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub struct BackupSystem {
    db: Arc<Mutex<Db>>,
}

impl BackupSystem {
    pub fn new(db: Arc<Mutex<Db>>) -> Self {
        Self { db }
    }

    pub async fn run_backup(&self, config: BackupConfig) -> Result<(), String> {
        let photos = {
            let db = self.db.lock().unwrap();
            db.get_photos_paginated(10000, 0).map_err(|e| e.to_string())?
        };

        match config {
            BackupConfig::S3 { bucket, endpoint, region } => {
                let region_provider = RegionProviderChain::first_try(region.map(Region::new)).or_default_provider();
                let mut config_loader = aws_config::from_env().region(region_provider);
                if let Some(ep) = endpoint {
                    config_loader = config_loader.endpoint_url(ep);
                }
                let aws_config = config_loader.load().await;
                let client = S3Client::new(&aws_config);

                for photo in photos {
                    self.backup_to_s3(&client, &bucket, &photo).await?;
                }
            }
            BackupConfig::WebDav { base_url, username, password } => {
                let client = ReqwestClient::new();
                for photo in photos {
                    self.backup_to_webdav(&client, &base_url, &username, &password, &photo).await?;
                }
            }
        }
        Ok(())
    }

    async fn backup_to_s3(&self, client: &S3Client, bucket: &str, photo: &Photo) -> Result<(), String> {
        let path = Path::new(&photo.path);
        if !path.exists() { return Ok(()); }
        
        let hash = match photo.hash.clone() {
            Some(h) => h,
            None => calculate_hash(path).map_err(|e| e.to_string())?,
        };

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let remote_key = if ext.is_empty() { hash } else { format!("{}.{}", hash, ext) };

        let exists = client.head_object()
            .bucket(bucket)
            .key(&remote_key)
            .send()
            .await
            .is_ok();

        if !exists {
            let body = ByteStream::from_path(path).await.map_err(|e| e.to_string())?;
            client.put_object()
                .bucket(bucket)
                .key(&remote_key)
                .body(body)
                .send()
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn backup_to_webdav(
        &self, 
        client: &ReqwestClient, 
        base_url: &str, 
        username: &Option<String>, 
        password: &Option<String>, 
        photo: &Photo
    ) -> Result<(), String> {
        let path = Path::new(&photo.path);
        if !path.exists() { return Ok(()); }

        let hash = match photo.hash.clone() {
            Some(h) => h,
            None => calculate_hash(path).map_err(|e| e.to_string())?,
        };

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let remote_key = if ext.is_empty() { hash } else { format!("{}.{}", hash, ext) };
        let url = format!("{}/{}", base_url.trim_end_matches('/'), remote_key);

        let mut req = client.request(reqwest::Method::from_bytes(b"PROPFIND").unwrap(), &url);
        if let (Some(u), Some(p)) = (username, password) {
            req = req.basic_auth(u, Some(p));
        }
        let exists = match req.send().await {
            Ok(res) => res.status().is_success(),
            Err(_) => false,
        };

        if !exists {
            let file = tokio::fs::File::open(path).await.map_err(|e| e.to_string())?;
            let stream = ReaderStream::new(file);
            let body = reqwest::Body::wrap_stream(stream);

            let mut put_req = client.put(&url).body(body);
            if let (Some(u), Some(p)) = (username, password) {
                put_req = put_req.basic_auth(u, Some(p));
            }
            let res = put_req.send().await.map_err(|e| e.to_string())?;
            if !res.status().is_success() {
                return Err(format!("Upload failed: {}", res.status()));
            }
        }

        Ok(())
    }
}

#[tauri::command]
pub async fn start_backup(state: State<'_, crate::AppState>, config: BackupConfig) -> Result<(), String> {
    let backup_system = BackupSystem::new(state.db.clone());
    backup_system.run_backup(config).await
}
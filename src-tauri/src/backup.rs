use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use std::fs;
use reqwest::Client;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

// Backup strategy: S3 and WebDAV
#[async_trait]
pub trait BackupProvider: Send + Sync {
    async fn upload_file(&self, local_path: &Path, remote_path: &str) -> Result<(), String>;
}

// S3
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
pub struct S3Backup {
    client: S3Client,
    bucket: String,
}

impl S3Backup {
    pub async fn new(bucket: String) -> Result<Self, String> {
        let config = aws_config::load_from_env().await;
        let client = S3Client::new(&config);
        Ok(Self { client, bucket })
    }
}

#[async_trait]
impl BackupProvider for S3Backup {
    async fn upload_file(&self, local_path: &Path, remote_path: &str) -> Result<(), String> {
        let body = ByteStream::from_path(local_path).await.map_err(|e| e.to_string())?;
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(remote_path)
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// WebDAV
pub struct WebDavBackup {
    client: Client,
    base_url: String,
    auth: Option<(String, String)>,
}

impl WebDavBackup {
    pub fn new(base_url: String, user: Option<String>, pass: Option<String>) -> Self {
        let auth = match (user, pass) {
            (Some(u), Some(p)) => Some((u, p)),
            _ => None,
        };
        Self { client: Client::new(), base_url, auth }
    }
}

#[async_trait]
impl BackupProvider for WebDavBackup {
    async fn upload_file(&self, local_path: &Path, remote_path: &str) -> Result<(), String> {
        let file_content = fs::read(local_path).map_err(|e| e.to_string())?;
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), remote_path.trim_start_matches('/'));
        
        let mut req = self.client.put(&url).body(file_content);
        if let Some((user, pass)) = &self.auth {
            req = req.basic_auth(user, Some(pass));
        }

        let resp = req.send().await.map_err(|e| e.to_string())?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("WebDAV upload failed: {}", resp.status()))
        }
    }
}

pub struct BackupSystem {
    provider: Box<dyn BackupProvider>,
    db: Arc<Mutex<crate::db::Db>>,
}

impl BackupSystem {
    pub fn new(provider: Box<dyn BackupProvider>, db: Arc<Mutex<crate::db::Db>>) -> Self {
        // Create table for incremental sync
        {
            let db_lock = db.lock().unwrap();
            let _ = db_lock.conn.execute(
                "CREATE TABLE IF NOT EXISTS backup_state (
                    photo_id INTEGER PRIMARY KEY,
                    hash TEXT NOT NULL
                )",
                [],
            );
        }
        Self { provider, db }
    }

    pub fn compute_sha256(path: &Path) -> Result<String, String> {
        use std::io::Read;
        let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];
        loop {
            let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }
        Ok(hex::encode(hasher.finalize()))
    }

    pub async fn backup_all(&self) -> Result<usize, String> {
        let photos = {
            let db = self.db.lock().unwrap();
            db.get_photos_paginated(10000, 0).map_err(|e| e.to_string())?
        };

        let mut backed_up = 0;
        for photo in photos {
            let local_path = PathBuf::from(&photo.path);
            if !local_path.exists() {
                continue;
            }

            let hash = Self::compute_sha256(&local_path)?;
            
            // Check incremental sync
            let needs_backup = {
                let db = self.db.lock().unwrap();
                let result: Result<String, _> = db.conn.query_row(
                    "SELECT hash FROM backup_state WHERE photo_id = ?1",
                    [photo.id],
                    |row| row.get(0),
                );
                
                match result {
                    Ok(stored_hash) => stored_hash != hash,
                    Err(_) => true, // Not found
                }
            };

            if !needs_backup {
                continue;
            }

            let remote_path = format!("{}_{}", hash, local_path.file_name().unwrap_or_default().to_string_lossy());
            
            match self.provider.upload_file(&local_path, &remote_path).await {
                Ok(_) => {
                    backed_up += 1;
                    // Update state
                    let db = self.db.lock().unwrap();
                    let _ = db.conn.execute(
                        "INSERT INTO backup_state (photo_id, hash) VALUES (?1, ?2)
                         ON CONFLICT(photo_id) DO UPDATE SET hash=excluded.hash",
                        rusqlite::params![photo.id, hash],
                    );
                }
                Err(e) => {
                    eprintln!("Failed to backup photo {}: {}", photo.path, e);
                }
            }
        }
        Ok(backed_up)
    }
}

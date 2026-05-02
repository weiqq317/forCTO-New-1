use aws_sdk_s3::{Client as S3Client, config::Region};
use aws_credential_types::Credentials;
use reqwest::Client as HttpClient;
use sha2::{Sha256, Digest};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub enum BackupTarget {
    S3 {
        bucket: String,
        region: String,
        access_key: String,
        secret_key: String,
        endpoint_url: Option<String>,
    },
    WebDav {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
}

pub struct BackupSystem {
    target: BackupTarget,
}

impl BackupSystem {
    pub fn new(target: BackupTarget) -> Self {
        Self { target }
    }

    async fn calculate_sha256(path: &Path) -> Result<String, String> {
        let mut file = File::open(path).await.map_err(|e| e.to_string())?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];
        loop {
            let count = file.read(&mut buffer).await.map_err(|e| e.to_string())?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        Ok(hex::encode(hasher.finalize()))
    }

    pub async fn backup_file(&self, local_path: &str, remote_path: &str) -> Result<(), String> {
        let path = Path::new(local_path);
        if !path.exists() {
            return Err("File does not exist".to_string());
        }

        let sha256_hash = Self::calculate_sha256(path).await?;
        let file_data = tokio::fs::read(path).await.map_err(|e| e.to_string())?;

        match &self.target {
            BackupTarget::S3 { bucket, region, access_key, secret_key, endpoint_url } => {
                let credentials = Credentials::new(
                    access_key.clone(),
                    secret_key.clone(),
                    None::<String>,
                    None::<std::time::SystemTime>,
                    "manual",
                );

                let mut config_builder = aws_sdk_s3::config::Builder::new()
                    .region(Region::new(region.clone()))
                    .credentials_provider(credentials);
                
                if let Some(endpoint) = endpoint_url {
                    config_builder = config_builder.endpoint_url(endpoint.clone());
                }

                let config = config_builder.build();
                let client = S3Client::from_conf(config);

                // Incremental sync: check if file exists and matches size (or we could use head_object)
                let head = client.head_object()
                    .bucket(bucket)
                    .key(remote_path)
                    .send()
                    .await;

                if head.is_ok() {
                    // Object exists, assuming it's the same for simplified incremental sync.
                    return Ok(());
                }

                // Upload
                client.put_object()
                    .bucket(bucket)
                    .key(remote_path)
                    .body(file_data.into())
                    .checksum_sha256(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hex::decode(&sha256_hash).unwrap()))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            BackupTarget::WebDav { url, username, password } => {
                let client = HttpClient::new();
                
                // Form the full URL
                let full_url = format!("{}/{}", url.trim_end_matches('/'), remote_path.trim_start_matches('/'));

                // Incremental check: PROPFIND or HEAD (using HEAD for simplicity)
                let mut head_req = client.head(&full_url);
                if let (Some(u), Some(p)) = (username, password) {
                    head_req = head_req.basic_auth(u, Some(p));
                }
                
                if let Ok(res) = head_req.send().await {
                    if res.status().is_success() {
                        // Exists
                        return Ok(());
                    }
                }

                let mut put_req = client.put(&full_url)
                    .body(file_data)
                    .header("X-SHA256", sha256_hash);

                if let (Some(u), Some(p)) = (username, password) {
                    put_req = put_req.basic_auth(u, Some(p));
                }

                let response = put_req.send().await.map_err(|e| e.to_string())?;

                if !response.status().is_success() {
                    return Err(format!("WebDAV upload failed with status: {}", response.status()));
                }
            }
        }

        Ok(())
    }
}

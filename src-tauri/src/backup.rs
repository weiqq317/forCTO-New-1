use aws_sdk_s3::{Client as S3Client, config::{Credentials, Region, Builder as S3ConfigBuilder}};
use reqwest::Client as HttpClient;
use sha2::{Sha256, Digest};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub enum BackupTarget {
    S3 {
        bucket: String,
        region: String,
        endpoint_url: Option<String>,
        access_key: String,
        secret_key: String,
    },
    WebDAV {
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

    pub fn clone_target(&self) -> BackupTarget {
        self.target.clone()
    }

    pub async fn calculate_sha256(path: &Path) -> std::io::Result<String> {
        let mut file = fs::File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0; 8192];
        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub async fn upload_file(&self, local_path: &Path, remote_path: &str) -> Result<(), String> {
        let file_bytes = fs::read(local_path).await.map_err(|e| e.to_string())?;
        
        let _hash = Self::calculate_sha256(local_path).await.map_err(|e| e.to_string())?;

        match &self.target {
            BackupTarget::S3 { bucket, region, endpoint_url, access_key, secret_key } => {
                let credentials = Credentials::new(
                    access_key,
                    secret_key,
                    None,
                    None,
                    "manual"
                );
                
                let mut config_builder = S3ConfigBuilder::new()
                    .region(Region::new(region.clone()))
                    .credentials_provider(credentials)
                    .force_path_style(true);
                    
                if let Some(endpoint) = endpoint_url {
                    config_builder = config_builder.endpoint_url(endpoint);
                }
                
                let client = S3Client::from_conf(config_builder.build());

                let exists = client.head_object()
                    .bucket(bucket)
                    .key(remote_path)
                    .send()
                    .await;

                if exists.is_ok() {
                    return Ok(());
                }

                client.put_object()
                    .bucket(bucket)
                    .key(remote_path)
                    .body(file_bytes.into())
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                
                Ok(())
            }
            BackupTarget::WebDAV { url, username, password } => {
                let client = HttpClient::new();
                let full_url = format!("{}/{}", url.trim_end_matches('/'), remote_path);
                
                let head_req = client.head(&full_url);
                let head_req = if let (Some(u), Some(p)) = (username, password) {
                    head_req.basic_auth(u, Some(p))
                } else {
                    head_req
                };

                let head_res = head_req.send().await.map_err(|e| e.to_string())?;
                if head_res.status().is_success() {
                    return Ok(());
                }

                let mut req = client.put(&full_url).body(file_bytes);
                if let (Some(u), Some(p)) = (username, password) {
                    req = req.basic_auth(u, Some(p));
                }

                let res = req.send().await.map_err(|e| e.to_string())?;
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("WebDAV upload failed: {}", res.status()))
                }
            }
        }
    }
}

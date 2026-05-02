use aws_sdk_s3 as s3;
use s3::Client;
use std::path::Path;
use sha2::{Sha256, Digest};
use std::fs;
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
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
    }
}

pub struct BackupSystem {
    target: BackupTarget,
}

impl BackupSystem {
    pub fn new(target: BackupTarget) -> Self {
        Self { target }
    }

    pub async fn backup_file(&self, local_path: &Path, remote_path: &str) -> Result<(), String> {
        let content = fs::read(local_path).map_err(|e| e.to_string())?;
        let hash = self.compute_hash(&content);
        
        match &self.target {
            BackupTarget::S3 { bucket, region: _region, endpoint_url, access_key, secret_key } => {
                let creds = aws_sdk_s3::config::Credentials::new(
                    access_key,
                    secret_key,
                    None,
                    None,
                    "manual",
                );
                
                let mut config_builder = aws_sdk_s3::config::Config::builder()
                    .credentials_provider(creds)
                    .region(aws_sdk_s3::config::Region::new(_region.clone()));
                
                if let Some(endpoint) = endpoint_url {
                    config_builder = config_builder.endpoint_url(endpoint.clone());
                }
                
                let config = config_builder.build();
                let client = Client::from_conf(config);

                // Check if exists with same hash (incremental sync)
                let head = client.head_object()
                    .bucket(bucket)
                    .key(remote_path)
                    .send()
                    .await;
                
                if let Ok(meta) = head {
                    if let Some(metadata_hash) = meta.metadata().and_then(|m| m.get("sha256")) {
                        if metadata_hash == &hash {
                            return Ok(()); // already synced
                        }
                    }
                }
                
                client.put_object()
                    .bucket(bucket)
                    .key(remote_path)
                    .body(content.into())
                    .metadata("sha256", hash)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
            },
            BackupTarget::WebDAV { url, username, password } => {
                let client = reqwest::Client::new();
                let dest_url = format!("{}/{}", url.trim_end_matches('/'), remote_path);
                
                // For webdav, checking hash might require PROPFIND. We will do a HEAD to check if file exists, 
                // but standard WebDAV doesn't expose SHA256 easily without custom properties.
                // We'll just do a PUT which overwrites.
                
                let mut req = client.put(&dest_url).body(content);
                if let (Some(u), Some(p)) = (username, password) {
                    req = req.basic_auth(u, Some(p));
                }
                
                let res = req.send().await.map_err(|e| e.to_string())?;
                if !res.status().is_success() {
                    return Err(format!("WebDAV upload failed: {}", res.status()));
                }
            }
        }
        Ok(())
    }

    fn compute_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        hex::encode(result)
    }
}

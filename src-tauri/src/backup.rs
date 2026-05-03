use aws_config::BehaviorVersion;
use aws_sdk_s3::{config::Credentials, config::Region, Client as S3Client};
use reqwest::Client as HttpClient;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use tokio::io::AsyncReadExt;

use crate::models::Photo;

pub enum BackupTarget {
    S3 {
        endpoint: String,
        bucket: String,
        access_key: String,
        secret_key: String,
        region: String,
    },
    WebDav {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
}

#[derive(Debug)]
pub enum BackupError {
    Io(std::io::Error),
    S3(String),
    WebDav(String),
}

impl From<std::io::Error> for BackupError {
    fn from(err: std::io::Error) -> Self {
        BackupError::Io(err)
    }
}

pub async fn compute_sha256(path: &Path) -> Result<String, std::io::Error> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hex::encode(hasher.finalize()))
}

pub async fn backup_photos(photos: Vec<Photo>, target: BackupTarget) -> Result<(), BackupError> {
    match target {
        BackupTarget::S3 {
            endpoint,
            bucket,
            access_key,
            secret_key,
            region,
        } => {
            let credentials = Credentials::new(access_key, secret_key, None, None, "manual");
            let config = aws_config::SdkConfig::builder()
                .credentials_provider(credentials)
                .region(Region::new(region))
                .endpoint_url(endpoint)
                .behavior_version(BehaviorVersion::latest())
                .build();
            let client = S3Client::new(&config);

            for photo in photos {
                let path = Path::new(&photo.path);
                if !path.exists() {
                    continue;
                }

                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                let sha256_hash = compute_sha256(path).await?;

                // Check if file exists and has same hash (Incremental sync)
                let head_res = client
                    .head_object()
                    .bucket(&bucket)
                    .key(&file_name)
                    .send()
                    .await;

                let mut should_upload = true;
                if let Ok(head) = head_res {
                    if let Some(metadata) = head.metadata() {
                        if metadata.get("sha256") == Some(&sha256_hash) {
                            should_upload = false; // Already backed up
                        }
                    }
                }

                if should_upload {
                    let body = aws_sdk_s3::primitives::ByteStream::from_path(path)
                        .await
                        .map_err(|e| BackupError::Io(e.into()))?;

                    client
                        .put_object()
                        .bucket(&bucket)
                        .key(&file_name)
                        .body(body)
                        .metadata("sha256", &sha256_hash)
                        .send()
                        .await
                        .map_err(|e| BackupError::S3(e.to_string()))?;
                }
            }
        }
        BackupTarget::WebDav {
            url,
            username,
            password,
        } => {
            let client = HttpClient::new();

            for photo in photos {
                let path = Path::new(&photo.path);
                if !path.exists() {
                    continue;
                }

                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                let file_url = format!("{}/{}", url.trim_end_matches('/'), file_name);
                let sha256_hash = compute_sha256(path).await?;

                // PROPFIND to get custom property or check exist
                let mut req = client.request(reqwest::Method::from_bytes(b"PROPFIND").unwrap(), &file_url);
                if let (Some(u), Some(p)) = (&username, &password) {
                    req = req.basic_auth(u, Some(p));
                }

                let res = req.send().await.map_err(|e| BackupError::WebDav(e.to_string()))?;
                
                let mut should_upload = true;
                if res.status().is_success() {
                    // For WebDAV, a proper PROPFIND parsing would be needed to extract sha256
                    // For simplicity, we just rely on existence or HEAD if custom headers were supported.
                    // Let's assume if it exists, we skip, or if we wanted to be more robust we upload.
                    // We'll upload if it doesn't exist.
                    should_upload = false;
                }

                if should_upload {
                    let file_bytes = fs::read(path)?;
                    let mut put_req = client.put(&file_url).body(file_bytes);
                    if let (Some(u), Some(p)) = (&username, &password) {
                        put_req = put_req.basic_auth(u, Some(p));
                    }
                    
                    // Store hash in a custom header (some WebDAV servers support this)
                    put_req = put_req.header("X-File-SHA256", sha256_hash);

                    put_req
                        .send()
                        .await
                        .map_err(|e| BackupError::WebDav(e.to_string()))?
                        .error_for_status()
                        .map_err(|e| BackupError::WebDav(e.to_string()))?;
                }
            }
        }
    }
    Ok(())
}

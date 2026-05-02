use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use image::imageops::FilterType;
use std::process::Command;
use walkdir::WalkDir;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct MediaInfo {
    pub path: String,
    pub hash: String,
    pub exif_data: Option<String>,
    pub thumbnail_path: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub ext: String,
}

pub async fn import_folder(folder_path: &str, cache_dir: &Path) -> Result<Vec<MediaInfo>, String> {
    let folder_path = folder_path.to_string();
    let cache_dir = cache_dir.to_path_buf();
    
    // Non-blocking traversal
    let paths: Vec<PathBuf> = tokio::task::spawn_blocking(move || {
        WalkDir::new(folder_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect()
    }).await.map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    
    for path in paths {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if ["jpg", "jpeg", "png", "mp4"].contains(&ext.as_str()) {
            let path_str = path.to_string_lossy().to_string();
            let cache_dir_clone = cache_dir.clone();
            
            // Process file in a blocking task since hash, exif, image ops are CPU bound or blocking I/O
            let res = tokio::task::spawn_blocking(move || {
                process_file(&path_str, &cache_dir_clone, &ext)
            }).await.map_err(|e| e.to_string())?;
            
            if let Ok(info) = res {
                results.push(info);
            }
        }
    }

    Ok(results)
}

fn process_file(path: &str, cache_dir: &Path, ext: &str) -> Result<MediaInfo, String> {
    let file_bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    
    // 1. SHA256 Hash
    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let hash = hex::encode(hasher.finalize());
    
    // 2. Extract EXIF
    let exif_data = extract_metadata_sync(path).unwrap_or(None);
    
    // 3. Generate Thumbnail
    let thumbnail_filename = format!("{}_thumb.jpg", hash);
    let thumbnail_path = cache_dir.join(&thumbnail_filename);
    
    let mut width = None;
    let mut height = None;
    let mut final_thumb_path = None;
    
    let thumb_res = generate_thumbnail_sync(path, ext, &thumbnail_path);
    if let Ok((w, h)) = thumb_res {
        width = Some(w);
        height = Some(h);
        final_thumb_path = Some(thumbnail_path.to_string_lossy().to_string());
    }
    
    Ok(MediaInfo {
        path: path.to_string(),
        hash,
        exif_data,
        thumbnail_path: final_thumb_path,
        width,
        height,
        ext: ext.to_string(),
    })
}

pub async fn generate_thumbnail(path: &str, ext: &str, out_path: &Path) -> Result<(u32, u32), String> {
    let path_str = path.to_string();
    let ext_str = ext.to_string();
    let out_path_buf = out_path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        generate_thumbnail_sync(&path_str, &ext_str, &out_path_buf)
    }).await.map_err(|e| e.to_string())?
}

fn generate_thumbnail_sync(path: &str, ext: &str, out_path: &Path) -> Result<(u32, u32), String> {
    if ext == "mp4" {
        // Use ffmpeg for mp4
        let output = Command::new("ffmpeg")
            .args(&[
                "-y", "-i", path,
                "-vf", "thumbnail,scale=300:-1",
                "-frames:v", "1",
                out_path.to_str().unwrap()
            ])
            .output()
            .map_err(|e| e.to_string())?;
            
        if !output.status.success() {
            return Err("Failed to generate mp4 thumbnail".into());
        }
        
        if let Ok(img) = image::open(out_path) {
            return Ok((img.width(), img.height()));
        }
        Ok((300, 300))
    } else {
        // Use image crate for jpg/png
        let img = image::open(path).map_err(|e| e.to_string())?;
        let width = img.width();
        let height = img.height();
        
        let thumb = img.resize(300, 300, FilterType::Lanczos3);
        thumb.save(out_path).map_err(|e| e.to_string())?;
        
        Ok((width, height))
    }
}

pub async fn extract_metadata(path: &str) -> Result<Option<String>, String> {
    let path_str = path.to_string();
    tokio::task::spawn_blocking(move || {
        extract_metadata_sync(&path_str)
    }).await.map_err(|e| e.to_string())?
}

fn extract_metadata_sync(path: &str) -> Result<Option<String>, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => {
            let mut meta = serde_json::Map::new();
            for f in exif.fields() {
                let tag = format!("{}", f.tag);
                let value = format!("{}", f.display_value().with_unit(&exif));
                meta.insert(tag, json!(value));
            }
            Ok(Some(serde_json::to_string(&meta).unwrap()))
        },
        Err(_) => Ok(None),
    }
}

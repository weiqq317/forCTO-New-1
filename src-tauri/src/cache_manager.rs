use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::fs;

pub struct CacheManager {
    thumbnail_dir: PathBuf,
    preview_dir: PathBuf,
    thumbnail_cache: LruCache<String, PathBuf>,
    preview_cache: LruCache<String, PathBuf>,
}

impl CacheManager {
    pub fn new<P: AsRef<Path>>(base_dir: P, max_thumbnails: usize, max_previews: usize) -> Self {
        let base_dir = base_dir.as_ref();
        let thumbnail_dir = base_dir.join("thumbnails");
        let preview_dir = base_dir.join("previews");

        fs::create_dir_all(&thumbnail_dir).unwrap_or_default();
        fs::create_dir_all(&preview_dir).unwrap_or_default();

        let thumbnail_cache = LruCache::new(NonZeroUsize::new(max_thumbnails.max(1)).unwrap());
        let preview_cache = LruCache::new(NonZeroUsize::new(max_previews.max(1)).unwrap());

        Self {
            thumbnail_dir,
            preview_dir,
            thumbnail_cache,
            preview_cache,
        }
    }

    pub fn put_thumbnail(&mut self, key: String, data: &[u8]) -> Option<PathBuf> {
        let file_path = self.thumbnail_dir.join(&key);
        if fs::write(&file_path, data).is_ok() {
            if let Some((_, evicted_path)) = self.thumbnail_cache.push(key, file_path.clone()) {
                let _ = fs::remove_file(evicted_path);
            }
            Some(file_path)
        } else {
            None
        }
    }

    pub fn get_thumbnail(&mut self, key: &str) -> Option<PathBuf> {
        if let Some(path) = self.thumbnail_cache.get(key) {
            return Some(path.clone());
        }
        
        let file_path = self.thumbnail_dir.join(key);
        if file_path.exists() {
            self.thumbnail_cache.push(key.to_string(), file_path.clone());
            Some(file_path)
        } else {
            None
        }
    }

    pub fn put_preview(&mut self, key: String, data: &[u8]) -> Option<PathBuf> {
        let file_path = self.preview_dir.join(&key);
        if fs::write(&file_path, data).is_ok() {
            if let Some((_, evicted_path)) = self.preview_cache.push(key, file_path.clone()) {
                 let _ = fs::remove_file(evicted_path);
            }
            Some(file_path)
        } else {
            None
        }
    }

    pub fn get_preview(&mut self, key: &str) -> Option<PathBuf> {
        if let Some(path) = self.preview_cache.get(key) {
            return Some(path.clone());
        }
        
        let file_path = self.preview_dir.join(key);
        if file_path.exists() {
            self.preview_cache.push(key.to_string(), file_path.clone());
            Some(file_path)
        } else {
            None
        }
    }
}

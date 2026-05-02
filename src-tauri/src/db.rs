use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS media_assets (
                id INTEGER PRIMARY KEY,
                file_path TEXT NOT NULL UNIQUE,
                imported_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn insert_asset(&self, file_path: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO media_assets (file_path) VALUES (?1)",
            params![file_path],
        )?;
        Ok(())
    }

    pub fn list_assets(&self) -> Result<Vec<crate::MediaAsset>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, file_path, imported_at FROM media_assets")?;
        let media_iter = stmt.query_map([], |row| {
            Ok(crate::MediaAsset {
                id: row.get(0)?,
                file_path: row.get(1)?,
                imported_at: row.get(2)?,
            })
        })?;

        let mut assets = Vec::new();
        for m in media_iter {
            if let Ok(asset) = m {
                assets.push(asset);
            }
        }
        Ok(assets)
    }
}

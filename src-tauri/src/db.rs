// Database layer implementation
use rusqlite::{params, Connection, Result};
use std::path::Path;
use crate::models::{Photo, Tag, PhotoTag, Face, Person};

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        let schema = include_str!("schema.sql");
        conn.execute_batch(schema)?;

        Ok(Self { conn })
    }

    pub fn insert_photo(
        &self,
        path: &str,
        hash: Option<&str>,
        width: Option<i64>,
        height: Option<i64>,
        photo_type: Option<&str>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO photos (path, hash, width, height, type) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(path) DO UPDATE SET 
             hash=excluded.hash, width=excluded.width, height=excluded.height, type=excluded.type",
            params![path, hash, width, height, photo_type],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_photos_paginated(&self, limit: i64, offset: i64) -> Result<Vec<Photo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, path, hash, created_at, width, height, type 
             FROM photos 
             ORDER BY created_at DESC 
             LIMIT ?1 OFFSET ?2",
        )?;

        let photo_iter = stmt.query_map(params![limit, offset], |row| {
            Ok(Photo {
                id: row.get(0)?,
                path: row.get(1)?,
                hash: row.get(2)?,
                created_at: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                photo_type: row.get(6)?,
            })
        })?;

        let mut photos = Vec::new();
        for photo in photo_iter {
            photos.push(photo?);
        }
        Ok(photos)
    }

    pub fn add_tag(&self, photo_id: i64, tag_name: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
            params![tag_name],
        )?;

        let tag_id: i64 = self.conn.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![tag_name],
            |row| row.get(0),
        )?;

        self.conn.execute(
            "INSERT OR IGNORE INTO photo_tags (photo_id, tag_id) VALUES (?1, ?2)",
            params![photo_id, tag_id],
        )?;

        Ok(())
    }

    pub fn search_photos(&self, query_tag: &str) -> Result<Vec<Photo>> {
        let mut stmt = self.conn.prepare(
            "SELECT p.id, p.path, p.hash, p.created_at, p.width, p.height, p.type 
             FROM photos p
             JOIN photo_tags pt ON p.id = pt.photo_id
             JOIN tags t ON pt.tag_id = t.id
             WHERE t.name = ?1
             ORDER BY p.created_at DESC",
        )?;

        let photo_iter = stmt.query_map(params![query_tag], |row| {
            Ok(Photo {
                id: row.get(0)?,
                path: row.get(1)?,
                hash: row.get(2)?,
                created_at: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                photo_type: row.get(6)?,
            })
        })?;

        let mut photos = Vec::new();
        for photo in photo_iter {
            photos.push(photo?);
        }
        Ok(photos)
    }
}

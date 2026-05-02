use rusqlite::{params, Result};
use crate::db::Db;
use crate::models::Photo;

pub fn search_photos(
    db: &Db,
    query: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Photo>> {
    let mut sql = String::from(
        "SELECT DISTINCT p.id, p.path, p.hash, p.created_at, p.width, p.height, p.type, p.ocr_text
         FROM photos p
         LEFT JOIN photo_tags pt ON p.id = pt.photo_id
         LEFT JOIN tags t ON pt.tag_id = t.id
         WHERE 1=1"
    );

    let mut params_vec: Vec<String> = Vec::new();
    let query = query.trim();

    if !query.is_empty() {
        // Search by filename (path), OCR text, or tags.
        // Also supports basic date searching if the date string is provided in query
        sql.push_str(" AND (p.path LIKE ?1 OR p.ocr_text LIKE ?1 OR t.name LIKE ?1 OR p.created_at LIKE ?1)");
        params_vec.push(format!("%{}%", query));
        sql.push_str(" ORDER BY p.created_at DESC LIMIT ?2 OFFSET ?3");
    } else {
        sql.push_str(" ORDER BY p.created_at DESC LIMIT ?1 OFFSET ?2");
    }

    let mut stmt = db.conn.prepare(&sql)?;

    let photo_iter = if !query.is_empty() {
        stmt.query_map(rusqlite::params![&params_vec[0], limit, offset], |row| {
            Ok(Photo {
                id: row.get(0)?,
                path: row.get(1)?,
                hash: row.get(2)?,
                created_at: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                photo_type: row.get(6)?,
                ocr_text: row.get(7)?,
            })
        })?
    } else {
        stmt.query_map(rusqlite::params![limit, offset], |row| {
            Ok(Photo {
                id: row.get(0)?,
                path: row.get(1)?,
                hash: row.get(2)?,
                created_at: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                photo_type: row.get(6)?,
                ocr_text: row.get(7)?,
            })
        })?
    };

    let mut photos = Vec::new();
    for photo in photo_iter {
        photos.push(photo?);
    }

    Ok(photos)
}

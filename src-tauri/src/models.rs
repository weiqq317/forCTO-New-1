use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Photo {
    pub id: i64,
    pub path: String,
    pub hash: Option<String>,
    pub created_at: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    #[serde(rename = "type")]
    pub photo_type: Option<String>,
    pub ocr_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoTag {
    pub photo_id: i64,
    pub tag_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Face {
    pub id: i64,
    pub photo_id: i64,
    pub embedding: Option<Vec<u8>>,
    pub person_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: i64,
    pub name: String,
}

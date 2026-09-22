use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub content: String,
    pub position: Position,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

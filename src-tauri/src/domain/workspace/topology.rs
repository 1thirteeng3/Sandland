use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewportState {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasNode {
    pub id: String,
    pub item_id: Option<String>,
    pub local_cell_path: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color_preset: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasEdge {
    pub id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub from_side: NodeSide,
    pub to_side: NodeSide,
    pub label: Option<String>,
    pub directed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardTopology {
    pub workspace_id: String,
    pub viewport: ViewportState,
    pub nodes: Vec<CanvasNode>,
    pub edges: Vec<CanvasEdge>,
    pub updated_at: i64,
}

impl BoardTopology {
    pub fn new(workspace_id: String) -> Self {
        Self {
            workspace_id,
            viewport: ViewportState::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }
}

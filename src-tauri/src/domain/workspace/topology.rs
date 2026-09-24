use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeSide {
    #[serde(alias = "left", alias = "Left")]
    Left,
    #[serde(alias = "right", alias = "Right")]
    Right,
    #[serde(alias = "top", alias = "Top")]
    Top,
    #[serde(alias = "bottom", alias = "Bottom")]
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
    pub node_type: Option<String>,
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
    #[serde(default)]
    pub revision: u64,
    pub updated_at: i64,
}

impl BoardTopology {
    pub fn new(workspace_id: String) -> Self {
        Self {
            workspace_id,
            viewport: ViewportState::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            revision: 1,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    /// Valida invariantes da topologia
    pub fn validate(&self) -> Result<(), String> {
        for edge in &self.edges {
            if edge.source_node_id == edge.target_node_id {
                return Err(format!("Auto-loop detectado e rejeitado na aresta '{}'", edge.id));
            }
        }
        Ok(())
    }

    /// Remove um nó e executa a exclusão em cascata de todas as arestas vinculadas
    pub fn delete_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n.id != node_id);
        self.edges.retain(|e| e.source_node_id != node_id && e.target_node_id != node_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Adiciona uma aresta garantindo que não seja auto-loop
    pub fn add_edge(&mut self, edge: CanvasEdge) -> Result<(), String> {
        if edge.source_node_id == edge.target_node_id {
            return Err("Arestas não podem conectar um nó a ele mesmo".to_string());
        }
        self.edges.retain(|e| e.id != edge.id);
        self.edges.push(edge);
        self.updated_at = chrono::Utc::now().timestamp();
        Ok(())
    }
}

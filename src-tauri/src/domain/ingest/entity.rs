use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IngestState {
    Pending,
    Extracting,
    Classifying,
    Classified,
    NeedsManualReview,
    Failed,
}

impl IngestState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Extracting => "Extracting",
            Self::Classifying => "Classifying",
            Self::Classified => "Classified",
            Self::NeedsManualReview => "NeedsManualReview",
            Self::Failed => "Failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Extracting" => Self::Extracting,
            "Classifying" => Self::Classifying,
            "Classified" => Self::Classified,
            "NeedsManualReview" => Self::NeedsManualReview,
            "Failed" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestedItem {
    pub id: String,
    pub vault_path: String,
    pub item_type: String, // "note", "web_snapshot", "media_transcript"
    pub title: String,
    pub content_hash: String,
    pub summary: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub state: IngestState,
    pub needs_manual_review: bool,
    pub read_only: bool,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

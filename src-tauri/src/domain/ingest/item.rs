use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "text")]
    Text,
}

impl SourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Url => "url",
            Self::Text => "text",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "url" => Self::Url,
            "text" => Self::Text,
            _ => Self::File,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IngestStatus {
    Ingested,
    Classified,
    Promoted,
}

impl IngestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ingested => "Ingested",
            Self::Classified => "Classified",
            Self::Promoted => "Promoted",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Classified" => Self::Classified,
            "Promoted" => Self::Promoted,
            _ => Self::Ingested,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestedItem {
    pub id: String,
    pub source_type: SourceType,
    pub source_path: String,
    pub canonical_uri: String,
    pub title: String,
    pub summary: Option<String>,
    pub content_hash: String,
    pub word_count: usize,
    pub tags: Vec<String>,
    pub status: IngestStatus,
    pub ingested_at: i64,
    pub updated_at: i64,
}

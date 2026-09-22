use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDetectedEvent {
    pub vault_path: String,
    pub file_size: u64,
    pub detected_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestStateChangedEvent {
    pub item_id: String,
    pub previous_state: String,
    pub new_state: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDownloadProgressEvent {
    pub model_name: String,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWatcherEvent {
    pub vault_path: String,
    pub kind: String, // "Created", "Modified", "Removed"
}

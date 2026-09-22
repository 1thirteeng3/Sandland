use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum DomainEvent {
    DocumentCommitted {
        rel_path: String,
        revision: u64,
        hash: String,
        timestamp: i64,
    },
    AssetStored {
        hash: String,
        extension: String,
        size_bytes: u64,
    },
    VaultOpened {
        vault_id: String,
        root_path: String,
    },
    VaultClosed,
    SecurityAlert {
        attempted_path: String,
        reason: String,
    },
    ItemDetected(ItemDetectedEvent),
    IngestStateChanged(IngestStateChangedEvent),
    ModelDownloadProgress(ModelDownloadProgressEvent),
    FileWatcher(FileWatcherEvent),
}

#[derive(Clone)]
pub struct EventBus {
    sender: Arc<broadcast::Sender<DomainEvent>>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender: Arc::new(sender),
        }
    }

    pub fn publish(&self, event: DomainEvent) -> Result<usize, String> {
        // If there are no active subscribers, broadcast::send returns an error, which is acceptable in decoupled architectures
        match self.sender.send(event) {
            Ok(subscribers) => Ok(subscribers),
            Err(_) => Ok(0),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(256)
    }
}

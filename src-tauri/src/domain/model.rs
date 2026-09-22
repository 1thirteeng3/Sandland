use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompatibilityFlags {
    pub min_supported_app_version: String,
    pub features_enabled: Vec<String>,
}

impl Default for CompatibilityFlags {
    fn default() -> Self {
        Self {
            min_supported_app_version: "0.1.0".to_string(),
            features_enabled: vec![
                "cas_sha256".to_string(),
                "journal_v2".to_string(),
                "board_json".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultManifest {
    pub vault_id: Uuid,
    pub vault_name: String,
    pub schema_version: String,
    pub created_at: DateTime<Utc>,
    pub last_opened_at: DateTime<Utc>,
    pub compatibility: CompatibilityFlags,
}

impl VaultManifest {
    pub fn new(vault_name: String) -> Self {
        let now = Utc::now();
        Self {
            vault_id: Uuid::new_v4(),
            vault_name,
            schema_version: "2.0.0".to_string(),
            created_at: now,
            last_opened_at: now,
            compatibility: CompatibilityFlags::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetRef {
    pub sha256_hash: String,
    pub byte_size: u64,
    pub extension: String,
    pub created_at: DateTime<Utc>,
    pub canonical_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JournalOperation {
    Create,
    Update,
    Delete,
    Move,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalRecord {
    pub v: u32,
    pub id: String,
    pub ts: i64,
    pub op: JournalOperation,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_rev: Option<u64>,
    pub new_rev: u64,
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectSnapshot {
    pub object_hash: String,
    pub content_bytes: Vec<u8>,
    pub stored_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecoveryReport {
    pub replayed_operations: usize,
    pub purged_temporary_files: usize,
    pub corrupted_records: usize,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultManifestDto {
    pub vault_id: String,
    pub vault_name: String,
    pub schema_version: String,
    pub created_at: String,
    pub last_opened_at: String,
    pub root_path: String,
}

impl From<(VaultManifest, String)> for VaultManifestDto {
    fn from((manifest, root_path): (VaultManifest, String)) -> Self {
        Self {
            vault_id: manifest.vault_id.to_string(),
            vault_name: manifest.vault_name,
            schema_version: manifest.schema_version,
            created_at: manifest.created_at.to_rfc3339(),
            last_opened_at: manifest.last_opened_at.to_rfc3339(),
            root_path,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetRefDto {
    pub sha256_hash: String,
    pub byte_size: u64,
    pub extension: String,
    pub canonical_uri: String,
}

impl From<AssetRef> for AssetRefDto {
    fn from(asset: AssetRef) -> Self {
        Self {
            sha256_hash: asset.sha256_hash,
            byte_size: asset.byte_size,
            extension: asset.extension,
            canonical_uri: asset.canonical_uri,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultStatusDto {
    pub is_open: bool,
    pub vault_id: Option<String>,
    pub vault_name: Option<String>,
    pub root_path: Option<String>,
    pub total_assets: usize,
    pub total_events: usize,
}

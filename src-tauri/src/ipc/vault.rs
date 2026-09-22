use crate::domain::core::errors::SandlandError;
use crate::domain::core::events::EventBus;
use crate::domain::model::{
    AssetRefDto, RecoveryReport, VaultManifest, VaultManifestDto, VaultStatusDto,
};
use crate::infra::ai::embeddings::EmbeddingEngine;
use crate::infra::ai::llm::LlmEngine;
use crate::infra::db::{init_database, DbPool};
use crate::infra::fs::cas::{read_cas_asset, store_cas_asset, ASSETS_DIR};
use crate::infra::fs::journal::read_all_journals;
use crate::infra::fs::lifecycle::{init_vault as fs_init_vault, open_vault as fs_open_vault};
use crate::infra::fs::recovery::run_boot_recovery;
use crate::infra::fs::vault::{hydrate_vault_from_disk, initialize_vault_structure};
use crate::infra::fs::writer::commit_document_file;
use crate::infra::security::VaultGuard;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultInfoDTO {
    pub vault_path: String,
    pub name: String,
    pub total_items: i64,
    pub total_workspaces: i64,
    pub last_opened_at: i64,
}

pub struct AppState {
    pub vault_guard: Arc<Mutex<Option<VaultGuard>>>,
    pub active_manifest: Arc<Mutex<Option<VaultManifest>>>,
    pub event_bus: Arc<EventBus>,
    pub db: Arc<Mutex<Option<DbPool>>>,
    pub embeddings: Arc<EmbeddingEngine>,
    pub llm: Arc<LlmEngine>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            vault_guard: Arc::new(Mutex::new(None)),
            active_manifest: Arc::new(Mutex::new(None)),
            event_bus: Arc::new(EventBus::default()),
            db: Arc::new(Mutex::new(None)),
            embeddings: Arc::new(EmbeddingEngine::new()),
            llm: Arc::new(LlmEngine::new()),
        }
    }
}

/// Comando IPC para inicializar um novo cofre canônico Sandland
#[tauri::command]
pub async fn init_vault(
    state: State<'_, AppState>,
    path: String,
    name: String,
) -> Result<VaultManifestDto, String> {
    let root = PathBuf::from(&path);
    let manifest = fs_init_vault(&root, &name).map_err(|e| e.to_string())?;

    let guard = VaultGuard::new(root.clone()).map_err(|e| e.to_string())?;

    // Inicializa índices derivados de apoio
    let db_path = root.join(".system/index.db");
    if let Ok(mut conn) = init_database(&db_path) {
        let _ = hydrate_vault_from_disk(&guard, &mut conn);
        *state.db.lock().unwrap() = Some(Arc::new(Mutex::new(conn)));
    }

    *state.vault_guard.lock().unwrap() = Some(guard);
    *state.active_manifest.lock().unwrap() = Some(manifest.clone());

    Ok(VaultManifestDto::from((manifest, path)))
}

/// Comando IPC para abrir e validar um cofre existente
#[tauri::command]
pub async fn open_vault(
    state: State<'_, AppState>,
    path: String,
) -> Result<VaultManifestDto, String> {
    let root = PathBuf::from(&path);
    let manifest = fs_open_vault(&root).map_err(|e| e.to_string())?;

    // Executa rotina de recuperação idempotente no boot
    let _recovery: RecoveryReport = run_boot_recovery(&root).unwrap_or_default();

    let guard = VaultGuard::new(root.clone()).map_err(|e| e.to_string())?;

    // Sincroniza banco SQLite derivado
    let db_path = root.join(".system/index.db");
    if let Ok(mut conn) = init_database(&db_path) {
        let _ = hydrate_vault_from_disk(&guard, &mut conn);
        *state.db.lock().unwrap() = Some(Arc::new(Mutex::new(conn)));
    }

    *state.vault_guard.lock().unwrap() = Some(guard);
    *state.active_manifest.lock().unwrap() = Some(manifest.clone());

    Ok(VaultManifestDto::from((manifest, path)))
}

/// Comando IPC para persistência atômica em duas fases via file-commit
#[tauri::command]
pub async fn commit_document(
    state: State<'_, AppState>,
    rel_path: String,
    content: String,
    expected_revision: Option<u64>,
) -> Result<u64, String> {
    let guard_lock = state.vault_guard.lock().unwrap();
    let guard = guard_lock
        .as_ref()
        .ok_or_else(|| "Nenhum cofre ativo aberto".to_string())?;

    let root_path = guard.vault_root();
    let relative = Path::new(&rel_path);

    // Validação léxica pelo VaultGuard
    guard
        .resolve_relative_path(relative)
        .map_err(|e| e.to_string())?;

    let new_rev = commit_document_file(
        root_path,
        relative,
        content.as_bytes(),
        expected_revision,
        Some(&state.event_bus),
    )
    .map_err(|e| e.to_string())?;

    Ok(new_rev)
}

/// Comando IPC para armazenar um ativo binário no CAS por hash SHA-256
#[tauri::command]
pub async fn store_asset(
    state: State<'_, AppState>,
    bytes_base64: String,
    extension: String,
) -> Result<AssetRefDto, String> {
    let guard_lock = state.vault_guard.lock().unwrap();
    let guard = guard_lock
        .as_ref()
        .ok_or_else(|| "Nenhum cofre ativo aberto".to_string())?;

    let raw_bytes = BASE64
        .decode(&bytes_base64)
        .map_err(|e| format!("Base64 inválido: {e}"))?;

    let asset_ref = store_cas_asset(guard.vault_root(), &raw_bytes, &extension)
        .map_err(|e| e.to_string())?;

    Ok(AssetRefDto::from(asset_ref))
}

/// Comando IPC para ler um ativo binário do CAS pelo hash SHA-256
#[tauri::command]
pub async fn read_asset(
    state: State<'_, AppState>,
    sha256_hash: String,
) -> Result<String, String> {
    let guard_lock = state.vault_guard.lock().unwrap();
    let guard = guard_lock
        .as_ref()
        .ok_or_else(|| "Nenhum cofre ativo aberto".to_string())?;

    let bytes = read_cas_asset(guard.vault_root(), &sha256_hash)
        .map_err(|e| e.to_string())?;

    Ok(BASE64.encode(&bytes))
}

/// Comando IPC para obter o status de atividade e integridade do cofre
#[tauri::command]
pub async fn get_vault_status(
    state: State<'_, AppState>,
) -> Result<VaultStatusDto, String> {
    let guard_lock = state.vault_guard.lock().unwrap();
    let manifest_lock = state.active_manifest.lock().unwrap();

    if let (Some(guard), Some(manifest)) = (guard_lock.as_ref(), manifest_lock.as_ref()) {
        let root = guard.vault_root();

        let total_assets = std::fs::read_dir(root.join(ASSETS_DIR))
            .map(|entries| entries.filter_map(|e| e.ok()).filter(|e| e.path().is_file()).count())
            .unwrap_or(0);

        let total_events = read_all_journals(root).map(|evs| evs.len()).unwrap_or(0);

        Ok(VaultStatusDto {
            is_open: true,
            vault_id: Some(manifest.vault_id.to_string()),
            vault_name: Some(manifest.vault_name.clone()),
            root_path: Some(root.to_string_lossy().to_string()),
            total_assets,
            total_events,
        })
    } else {
        Ok(VaultStatusDto {
            is_open: false,
            vault_id: None,
            vault_name: None,
            root_path: None,
            total_assets: 0,
            total_events: 0,
        })
    }
}

/// Comando legado mantido para compatibilidade com o frontend v0.0.1
#[tauri::command]
pub async fn create_vault(
    state: State<'_, AppState>,
    vault_path: String,
) -> Result<VaultInfoDTO, SandlandError> {
    let path = PathBuf::from(&vault_path);
    if path.exists() && path.join(".system/index.db").exists() {
        return Err(SandlandError::VaultAlreadyExists(path));
    }

    initialize_vault_structure(&path)?;

    let db_path = path.join(".system/index.db");
    let mut conn = init_database(&db_path)?;

    let vault_guard = VaultGuard::new(path.clone())?;
    let _ = hydrate_vault_from_disk(&vault_guard, &mut conn);

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Meu Cofre".to_string());

    let db_pool = Arc::new(Mutex::new(conn));

    *state.vault_guard.lock().unwrap() = Some(vault_guard);
    *state.db.lock().unwrap() = Some(db_pool);

    Ok(VaultInfoDTO {
        vault_path,
        name,
        total_items: 0,
        total_workspaces: 0,
        last_opened_at: chrono::Utc::now().timestamp(),
    })
}

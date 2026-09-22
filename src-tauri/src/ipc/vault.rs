use crate::domain::core::errors::SandlandError;
use crate::infra::db::{init_database, DbPool};
use crate::infra::fs::vault::{hydrate_vault_from_disk, initialize_vault_structure, VaultGuard};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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

use crate::infra::ai::embeddings::EmbeddingEngine;
use crate::infra::ai::llm::LlmEngine;

pub struct AppState {
    pub vault_guard: Arc<Mutex<Option<VaultGuard>>>,
    pub db: Arc<Mutex<Option<DbPool>>>,
    pub embeddings: Arc<EmbeddingEngine>,
    pub llm: Arc<LlmEngine>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            vault_guard: Arc::new(Mutex::new(None)),
            db: Arc::new(Mutex::new(None)),
            embeddings: Arc::new(EmbeddingEngine::new()),
            llm: Arc::new(LlmEngine::new()),
        }
    }
}

#[tauri::command]
pub async fn create_vault(
    state: State<'_, AppState>,
    vault_path: String,
) -> Result<VaultInfoDTO, SandlandError> {
    let path = PathBuf::from(&vault_path);
    if path.exists() && path.join(".system/index.db").exists() {
        return Err(SandlandError::VaultAlreadyExists(path));
    }

    // Cria diretórios canônicos
    initialize_vault_structure(&path)?;

    // Inicializa banco de dados e executa migrações refinery
    let db_path = path.join(".system/index.db");
    let mut conn = init_database(&db_path)?;

    let vault_guard = VaultGuard::new(path.clone())?;
    
    // Executa hidratação inicial se houver notas pré-existentes
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

#[tauri::command]
pub async fn open_vault(
    state: State<'_, AppState>,
    vault_path: String,
) -> Result<VaultInfoDTO, SandlandError> {
    let path = PathBuf::from(&vault_path);
    if !path.exists() {
        return Err(SandlandError::VaultNotFound(path));
    }

    // Inicializa estrutura se faltar subdiretórios
    initialize_vault_structure(&path)?;

    let db_path = path.join(".system/index.db");
    let mut conn = init_database(&db_path)?;

    let vault_guard = VaultGuard::new(path.clone())?;

    // Hidratação a quente a partir do sistema de arquivos (File-as-Truth)
    let _ = hydrate_vault_from_disk(&vault_guard, &mut conn);

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM items")?;
    let total_items: i64 = stmt.query_row([], |row| row.get(0))?;
    drop(stmt);

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Cofre".to_string());

    let db_pool = Arc::new(Mutex::new(conn));

    *state.vault_guard.lock().unwrap() = Some(vault_guard);
    *state.db.lock().unwrap() = Some(db_pool);

    Ok(VaultInfoDTO {
        vault_path,
        name,
        total_items,
        total_workspaces: 0,
        last_opened_at: chrono::Utc::now().timestamp(),
    })
}

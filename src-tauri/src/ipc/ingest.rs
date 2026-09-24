use crate::domain::core::errors::SandlandError;
use crate::domain::core::security::validate_url_for_ssrf;
use crate::domain::ingest::item::IngestStatus;
use crate::domain::workspace::topology::CanvasNode;
use crate::infra::db::indexer::{IngestFilterDTO, IngestedItemDTO};
use crate::infra::db::ingest_repo::IngestRepository;
use crate::infra::fs::canvas_io::CanvasStorage;
use crate::infra::fs::ingest_storage::IngestStorage;
use crate::ipc::vault::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{Emitter, State};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromoteToCellResponse {
    pub cell_id: String,
    pub workspace_id: String,
}

#[tauri::command]
pub async fn ingest_file(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    file_path: String,
) -> Result<IngestedItemDTO, SandlandError> {
    let source_path = PathBuf::from(&file_path);
    if !source_path.exists() {
        return Err(SandlandError::IngestFailed {
            item_id: file_path,
            reason: "Arquivo de origem não existe".to_string(),
        });
    }

    let raw_content = fs::read_to_string(&source_path)
        .map_err(|e| SandlandError::IoError(format!("Falha ao ler arquivo: {}", e)))?;

    let raw_title = source_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Sem Título".to_string());

    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto no momento".to_string())
    })?;

    // 1. Salva arquivo canônico físico em ingest/notes/
    let (item, canonical_markdown) = IngestStorage::save_note_content(guard, &raw_title, &raw_content)?;

    // 2. Indexa no SQLite index.db
    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    IngestRepository::upsert(&mut conn, &item, &canonical_markdown)?;

    let dto = IngestedItemDTO {
        id: item.id.clone(),
        vault_path: item.canonical_uri.clone(),
        title: item.title,
        item_type: "note".to_string(),
        summary: item.summary.clone(),
        category: None,
        tags: item.tags,
        state: item.status.as_str().to_string(),
        needs_manual_review: false,
        content_snippet: item.summary.unwrap_or_else(|| raw_title.clone()),
        created_at: item.ingested_at,
        updated_at: item.updated_at,
    };

    let _ = app_handle.emit(
        "ingest://state-changed",
        serde_json::json!({
            "itemId": dto.id,
            "previousState": null,
            "newState": dto.state
        }),
    );

    Ok(dto)
}

#[tauri::command]
pub async fn ingest_file_content(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    file_name: String,
    content: String,
) -> Result<IngestedItemDTO, SandlandError> {
    let stem = Path::new(&file_name)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| file_name.clone());

    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto no momento".to_string())
    })?;

    let (item, canonical_markdown) = IngestStorage::save_note_content(guard, &stem, &content)?;

    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    IngestRepository::upsert(&mut conn, &item, &canonical_markdown)?;

    let dto = IngestedItemDTO {
        id: item.id.clone(),
        vault_path: item.canonical_uri.clone(),
        title: item.title,
        item_type: "note".to_string(),
        summary: item.summary.clone(),
        category: None,
        tags: item.tags,
        state: item.status.as_str().to_string(),
        needs_manual_review: false,
        content_snippet: item.summary.unwrap_or_else(|| stem.clone()),
        created_at: item.ingested_at,
        updated_at: item.updated_at,
    };

    let _ = app_handle.emit(
        "ingest://state-changed",
        serde_json::json!({
            "itemId": dto.id,
            "previousState": null,
            "newState": dto.state
        }),
    );

    Ok(dto)
}

#[tauri::command]
pub async fn ingest_url(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    url: String,
) -> Result<IngestedItemDTO, SandlandError> {
    // Validação de Segurança Anti-SSRF (Princípio III da Constituição)
    validate_url_for_ssrf(&url)?;

    let title = format!("Web Snapshot: {}", url);
    let body = format!(
        "# {}\n\nURL Capturada: {}\nData da Captura: {}\n\nSnapshot textual do documento capturado.",
        title, url, chrono::Utc::now().to_rfc3339()
    );

    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto no momento".to_string())
    })?;

    let (item, canonical_markdown) = IngestStorage::save_web_snapshot(guard, &url, &title, &body)?;

    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    IngestRepository::upsert(&mut conn, &item, &canonical_markdown)?;

    let dto = IngestedItemDTO {
        id: item.id.clone(),
        vault_path: item.canonical_uri.clone(),
        title: item.title,
        item_type: "web_snapshot".to_string(),
        summary: item.summary.clone(),
        category: None,
        tags: item.tags,
        state: item.status.as_str().to_string(),
        needs_manual_review: false,
        content_snippet: item.summary.unwrap_or_else(|| title.clone()),
        created_at: item.ingested_at,
        updated_at: item.updated_at,
    };

    let _ = app_handle.emit(
        "ingest://state-changed",
        serde_json::json!({
            "itemId": dto.id,
            "previousState": null,
            "newState": dto.state
        }),
    );

    Ok(dto)
}

#[tauri::command]
pub async fn list_ingested_items(
    state: State<'_, AppState>,
    filter: Option<IngestFilterDTO>,
) -> Result<Vec<IngestedItemDTO>, SandlandError> {
    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let conn = db_pool.lock().unwrap();

    let filter_val = filter.unwrap_or_default();
    IngestRepository::list(&conn, &filter_val)
}

#[tauri::command]
pub async fn promote_to_cell(
    state: State<'_, AppState>,
    workspace_id: String,
    item_id: String,
    position_x: Option<f32>,
    position_y: Option<f32>,
) -> Result<PromoteToCellResponse, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto no momento".to_string())
    })?;

    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    // 1. Busca o item de origem
    let item = IngestRepository::get_by_id(&conn, &item_id)?
        .ok_or_else(|| SandlandError::NotFound(format!("Item ingerido '{}' não encontrado", item_id)))?;

    // 2. Carrega a topologia do workspace
    let mut topology = CanvasStorage::load_topology(guard, &workspace_id)?;

    let node_id = format!("node-{}", &Uuid::now_v7().to_string()[..8]);
    let x = position_x.unwrap_or(240.0 + (topology.nodes.len() as f32 * 30.0));
    let y = position_y.unwrap_or(200.0 + (topology.nodes.len() as f32 * 30.0));

    // 3. Cria nó desacoplado com vínculo de proveniência (Fork-on-Insert)
    let new_node = CanvasNode {
        id: node_id.clone(),
        item_id: Some(item.id.clone()),
        local_cell_path: Some(item.canonical_uri.clone()),
        title: Some(item.title.clone()),
        content: Some(item.summary.clone().unwrap_or(item.title.clone())),
        x,
        y,
        width: 280.0,
        height: 160.0,
        color_preset: Some("blue".to_string()),
        node_type: Some("note".to_string()),
    };

    topology.nodes.push(new_node);

    // 4. Salva a topologia atualizada
    CanvasStorage::sync_idle_json(guard, &workspace_id, &topology)?;

    // 5. Marca status como Promoted
    let mut updated_item = item;
    updated_item.status = IngestStatus::Promoted;
    updated_item.updated_at = chrono::Utc::now().timestamp();
    IngestRepository::upsert(&mut conn, &updated_item, &updated_item.summary.clone().unwrap_or_default())?;

    Ok(PromoteToCellResponse {
        cell_id: node_id,
        workspace_id,
    })
}

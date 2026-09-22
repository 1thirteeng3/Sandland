use crate::domain::core::errors::SandlandError;
use crate::infra::db::indexer::{list_items, sanitize_snippet, upsert_item, IngestFilterDTO, IngestedItemDTO, ItemRecord};
use crate::infra::fs::vault::format_markdown_with_frontmatter;
use crate::ipc::vault::AppState;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{Emitter, State};
use uuid::Uuid;

fn process_and_save_note(
    app_handle: &tauri::AppHandle,
    state: &State<'_, AppState>,
    raw_title: &str,
    raw_content: &str,
) -> Result<IngestedItemDTO, SandlandError> {
    let title = raw_content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").trim().to_string())
        .unwrap_or_else(|| raw_title.to_string());

    let id = Uuid::now_v7().to_string();
    let sanitized_stem: String = title
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    let file_name = format!("{}-{}.md", sanitized_stem, &id[..8]);
    let rel_vault_path = format!("ingest/notes/{}", file_name);

    let mut hasher = Sha256::new();
    hasher.update(raw_content.as_bytes());
    let content_hash = format!("{:x}", hasher.finalize());

    let now = chrono::Utc::now().timestamp();

    // Bloqueia e valida o cofre
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto no momento".to_string())
    })?;

    // Escreve com Frontmatter padronizado
    let markdown = format_markdown_with_frontmatter(
        &id,
        &title,
        "note",
        None,
        &[],
        None,
        raw_content,
    );
    guard.secure_write(Path::new(&rel_vault_path), markdown.as_bytes())?;

    // Registra no SQLite
    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    let record = ItemRecord {
        id: id.clone(),
        vault_path: rel_vault_path.clone(),
        item_type: "note".to_string(),
        title: title.clone(),
        content_hash,
        summary: None,
        category: None,
        state: "Pending".to_string(),
        needs_manual_review: false,
        read_only: true,
        created_at: now,
        updated_at: now,
    };

    upsert_item(&mut conn, &record, raw_content)?;

    let dto = IngestedItemDTO {
        id: id.clone(),
        vault_path: rel_vault_path,
        title,
        item_type: "note".to_string(),
        summary: None,
        category: None,
        tags: vec![],
        state: "Pending".to_string(),
        needs_manual_review: false,
        content_snippet: sanitize_snippet(raw_content, 180),
        created_at: now,
        updated_at: now,
    };

    let _ = app_handle.emit(
        "ingest://state-changed",
        serde_json::json!({
            "itemId": id,
            "previousState": null,
            "newState": "Pending"
        }),
    );

    Ok(dto)
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

    process_and_save_note(&app_handle, &state, &raw_title, &raw_content)
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

    process_and_save_note(&app_handle, &state, &stem, &content)
}

#[tauri::command]
pub async fn ingest_url(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    url: String,
) -> Result<IngestedItemDTO, SandlandError> {
    let id = Uuid::now_v7().to_string();
    let title = format!("Web Snapshot: {}", url);
    let now = chrono::Utc::now().timestamp();
    let file_name = format!("web-{}.md", &id[..8]);
    let rel_vault_path = format!("ingest/web/{}", file_name);

    let raw_content = format!(
        "# {}\n\nURL Capturada: {}\nData da Captura: {}\n\n[Conteúdo simplificado para v0.1]",
        title, url, chrono::Utc::now().to_rfc3339()
    );

    let mut hasher = Sha256::new();
    hasher.update(raw_content.as_bytes());
    let content_hash = format!("{:x}", hasher.finalize());

    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto no momento".to_string())
    })?;

    let markdown = format_markdown_with_frontmatter(
        &id,
        &title,
        "web_snapshot",
        None,
        &[],
        None,
        &raw_content,
    );
    guard.secure_write(Path::new(&rel_vault_path), markdown.as_bytes())?;

    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    let record = ItemRecord {
        id: id.clone(),
        vault_path: rel_vault_path.clone(),
        item_type: "web_snapshot".to_string(),
        title: title.clone(),
        content_hash,
        summary: None,
        category: None,
        state: "Pending".to_string(),
        needs_manual_review: false,
        read_only: true,
        created_at: now,
        updated_at: now,
    };

    upsert_item(&mut conn, &record, &raw_content)?;

    let dto = IngestedItemDTO {
        id: id.clone(),
        vault_path: rel_vault_path,
        title,
        item_type: "web_snapshot".to_string(),
        summary: None,
        category: None,
        tags: vec![],
        state: "Pending".to_string(),
        needs_manual_review: false,
        content_snippet: sanitize_snippet(&raw_content, 180),
        created_at: now,
        updated_at: now,
    };

    let _ = app_handle.emit(
        "ingest://state-changed",
        serde_json::json!({
            "itemId": id,
            "previousState": null,
            "newState": "Pending"
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
    list_items(&conn, &filter_val)
}

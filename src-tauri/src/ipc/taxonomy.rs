use crate::domain::core::errors::SandlandError;
use crate::infra::ai::fallback::{ClassificationCascade, ClassificationResultDTO};
use crate::infra::db::indexer::save_classification_atomic;
use crate::ipc::vault::AppState;
use std::path::Path;
use tauri::{Emitter, State};

#[tauri::command]
pub async fn trigger_classification(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    item_id: String,
) -> Result<ClassificationResultDTO, SandlandError> {
    // 1. Obtém informações e caminho do item no SQLite
    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    let (vault_path, title): (String, String) = conn
        .query_row(
            "SELECT vault_path, title FROM items WHERE id = ?1",
            rusqlite::params![item_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| {
            SandlandError::IngestFailed {
                item_id: item_id.clone(),
                reason: format!("Item não encontrado no cofre: {}", e),
            }
        })?;

    // 2. Lê conteúdo do arquivo
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    let content = guard
        .secure_read_to_string(Path::new(&vault_path))
        .unwrap_or_else(|_| title.clone());

    // 3. Busca vocabulário taxonômico existente no cofre
    let mut stmt = conn
        .prepare("SELECT DISTINCT term FROM taxonomy_terms WHERE term_type = 'tag'")
        .map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

    let known_tags: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| SandlandError::DatabaseError(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();
    drop(stmt);

    // 4. Executa cascata de classificação
    let classification = ClassificationCascade::classify_with_fallback(
        &state.llm,
        &item_id,
        &content,
        &known_tags,
    );

    // 5. Gera embeddings multilíngues via fastembed
    let embedding = state.embeddings.generate_embedding(&content).ok();

    // 6. Transação atômica no SQLite
    save_classification_atomic(
        &mut conn,
        &item_id,
        &classification.category,
        &classification.tags,
        &classification.summary,
        embedding.as_deref(),
    )?;

    let _ = app_handle.emit(
        "ingest://state-changed",
        serde_json::json!({
            "itemId": item_id,
            "previousState": "Extracting",
            "newState": "Classified"
        }),
    );

    Ok(classification)
}

#[tauri::command]
pub async fn update_item_tags(
    state: State<'_, AppState>,
    item_id: String,
    category: String,
    tags: Vec<String>,
) -> Result<(), SandlandError> {
    let db_opt = state.db.lock().unwrap();
    let db_pool = db_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Banco de dados não inicializado".to_string())
    })?;
    let mut conn = db_pool.lock().unwrap();

    save_classification_atomic(
        &mut conn,
        &item_id,
        &category,
        &tags,
        "",
        None,
    )?;

    // Limpa flag de revisão manual
    let _ = conn.execute(
        "UPDATE items SET needs_manual_review = 0 WHERE id = ?1",
        rusqlite::params![item_id],
    );

    Ok(())
}

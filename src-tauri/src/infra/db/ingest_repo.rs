use crate::domain::core::errors::{SandlandError, SandlandResult};
use crate::domain::ingest::item::{IngestStatus, IngestedItem, SourceType};
use crate::infra::db::indexer::{get_item_tags, IngestFilterDTO, IngestedItemDTO};
use rusqlite::{params, Connection};

pub struct IngestRepository;

impl IngestRepository {
    /// Insere ou atualiza um item ingerido no SQLite e no índice FTS5
    pub fn upsert(
        conn: &mut Connection,
        item: &IngestedItem,
        content_raw: &str,
    ) -> SandlandResult<()> {
        let tx = conn
            .transaction()
            .map_err(|e| SandlandError::DatabaseError(format!("Falha ao iniciar transação: {}", e)))?;

        // 1. Inserir ou atualizar na tabela items
        tx.execute(
            r#"
            INSERT INTO items (
                id, vault_path, item_type, title, content_hash, summary,
                category, state, needs_manual_review, read_only, created_at, updated_at,
                word_count, canonical_uri, source_path
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
            ON CONFLICT(id) DO UPDATE SET
                vault_path = excluded.vault_path,
                item_type = excluded.item_type,
                title = excluded.title,
                content_hash = excluded.content_hash,
                summary = COALESCE(excluded.summary, items.summary),
                state = excluded.state,
                updated_at = excluded.updated_at,
                word_count = excluded.word_count,
                canonical_uri = excluded.canonical_uri,
                source_path = excluded.source_path
            "#,
            params![
                item.id,
                item.canonical_uri,
                item.source_type.as_str(),
                item.title,
                item.content_hash,
                item.summary,
                None::<String>,
                item.status.as_str(),
                false,
                true,
                item.ingested_at,
                item.updated_at,
                item.word_count as i64,
                item.canonical_uri,
                item.source_path,
            ],
        )
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao gravar item no banco: {}", e)))?;

        // 2. Atualizar tags se houver
        let _ = tx.execute("DELETE FROM item_taxonomy WHERE item_id = ?1", params![item.id]);
        for tag in &item.tags {
            let clean_tag = tag.trim().to_lowercase();
            if !clean_tag.is_empty() {
                let now = chrono::Utc::now().timestamp();
                let _ = tx.execute(
                    "INSERT OR IGNORE INTO taxonomy_terms (term, term_type, created_at) VALUES (?1, 'tag', ?2)",
                    params![clean_tag, now],
                );
                let _ = tx.execute(
                    "INSERT OR REPLACE INTO item_taxonomy (item_id, term) VALUES (?1, ?2)",
                    params![item.id, clean_tag],
                );
            }
        }

        // 3. Atualizar FTS5
        let _ = tx.execute("DELETE FROM items_fts WHERE item_id = ?1", params![item.id]);
        tx.execute(
            "INSERT INTO items_fts (item_id, title, content) VALUES (?1, ?2, ?3)",
            params![item.id, item.title, content_raw],
        )
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao indexar FTS5: {}", e)))?;

        tx.commit()
            .map_err(|e| SandlandError::DatabaseError(format!("Falha ao confirmar transação: {}", e)))?;

        Ok(())
    }

    /// Busca um item pelo seu ID
    pub fn get_by_id(conn: &Connection, id: &str) -> SandlandResult<Option<IngestedItem>> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT id, COALESCE(item_type, 'note'), COALESCE(source_path, vault_path),
                       COALESCE(canonical_uri, vault_path), title, summary, content_hash,
                       COALESCE(word_count, 0), state, created_at, updated_at
                FROM items WHERE id = ?1
                "#,
            )
            .map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

        let mut rows = stmt
            .query(params![id])
            .map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| SandlandError::DatabaseError(e.to_string()))? {
            let item_id: String = row.get(0)?;
            let src_type_str: String = row.get(1)?;
            let src_path: String = row.get(2)?;
            let can_uri: String = row.get(3)?;
            let title: String = row.get(4)?;
            let summary: Option<String> = row.get(5)?;
            let hash: String = row.get(6)?;
            let words: i64 = row.get(7)?;
            let status_str: String = row.get(8)?;
            let created: i64 = row.get(9)?;
            let updated: i64 = row.get(10)?;

            let tags = get_item_tags(conn, &item_id).unwrap_or_default();

            Ok(Some(IngestedItem {
                id: item_id,
                source_type: SourceType::from_str(&src_type_str),
                source_path: src_path,
                canonical_uri: can_uri,
                title,
                summary,
                content_hash: hash,
                word_count: words as usize,
                tags,
                status: IngestStatus::from_str(&status_str),
                ingested_at: created,
                updated_at: updated,
            }))
        } else {
            Ok(None)
        }
    }

    /// Lista itens com filtro textual, de tag ou categoria
    pub fn list(conn: &Connection, filter: &IngestFilterDTO) -> SandlandResult<Vec<IngestedItemDTO>> {
        let mut query = String::from(
            r#"
            SELECT DISTINCT i.id, i.vault_path, i.title, i.item_type, i.summary,
                   i.category, i.state, i.needs_manual_review, i.created_at, i.updated_at
            FROM items i
            "#,
        );

        let mut conditions = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref q) = filter.query {
            let clean_q = q.trim();
            if !clean_q.is_empty() {
                // FTS5 join
                query.push_str(" JOIN items_fts fts ON i.id = fts.item_id");
                conditions.push("items_fts MATCH ?");
                params_vec.push(Box::new(format!("\"{}\"*", clean_q.replace('"', "\"\""))));
            }
        }

        if let Some(ref tag) = filter.tag {
            let clean_t = tag.trim().to_lowercase();
            if !clean_t.is_empty() {
                query.push_str(" JOIN item_taxonomy it ON i.id = it.item_id");
                conditions.push("it.term = ?");
                params_vec.push(Box::new(clean_t));
            }
        }

        if let Some(ref cat) = filter.category {
            conditions.push("i.category = ?");
            params_vec.push(Box::new(cat.clone()));
        }

        if let Some(true) = filter.needs_manual_review_only {
            conditions.push("i.needs_manual_review = 1");
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY i.created_at DESC");

        let limit = filter.limit.unwrap_or(50);
        let offset = filter.offset.unwrap_or(0);
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        let mut stmt = conn
            .prepare(&query)
            .map_err(|e| SandlandError::DatabaseError(format!("Erro ao preparar listagem: {}", e)))?;

        let params_slice: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

        let rows = stmt
            .query_map(params_slice.as_slice(), |row| {
                let id: String = row.get(0)?;
                let vault_path: String = row.get(1)?;
                let title: String = row.get(2)?;
                let item_type: String = row.get(3)?;
                let summary: Option<String> = row.get(4)?;
                let category: Option<String> = row.get(5)?;
                let state: String = row.get(6)?;
                let needs_manual_review: bool = row.get(7)?;
                let created_at: i64 = row.get(8)?;
                let updated_at: i64 = row.get(9)?;

                Ok((
                    id,
                    vault_path,
                    title,
                    item_type,
                    summary,
                    category,
                    state,
                    needs_manual_review,
                    created_at,
                    updated_at,
                ))
            })
            .map_err(|e| SandlandError::DatabaseError(format!("Erro ao executar consulta: {}", e)))?;

        let mut result = Vec::new();
        for r in rows {
            let (
                id,
                vault_path,
                title,
                item_type,
                summary,
                category,
                state,
                needs_manual_review,
                created_at,
                updated_at,
            ) = r.map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

            let tags = get_item_tags(conn, &id).unwrap_or_default();
            let snippet = summary.clone().unwrap_or_else(|| title.clone());

            result.push(IngestedItemDTO {
                id,
                vault_path,
                title,
                item_type,
                summary,
                category,
                tags,
                state,
                needs_manual_review,
                content_snippet: snippet,
                created_at,
                updated_at,
            });
        }

        Ok(result)
    }
}

use crate::domain::core::errors::{SandlandError, SandlandResult};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestedItemDTO {
    pub id: String,
    pub vault_path: String,
    pub title: String,
    pub item_type: String,
    pub summary: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub state: String,
    pub needs_manual_review: bool,
    pub content_snippet: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestFilterDTO {
    pub query: Option<String>,
    pub category: Option<String>,
    pub tag: Option<String>,
    pub needs_manual_review_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub struct ItemRecord {
    pub id: String,
    pub vault_path: String,
    pub item_type: String,
    pub title: String,
    pub content_hash: String,
    pub summary: Option<String>,
    pub category: Option<String>,
    pub state: String,
    pub needs_manual_review: bool,
    pub read_only: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Indexa ou atualiza um item no banco relacional e FTS5
pub fn upsert_item(
    conn: &mut Connection,
    record: &ItemRecord,
    content_raw: &str,
) -> SandlandResult<i64> {
    let tx = conn
        .transaction()
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao iniciar transação: {}", e)))?;

    // 1. Inserir ou atualizar tabela canônica items
    tx.execute(
        r#"
        INSERT INTO items (
            id, vault_path, item_type, title, content_hash, summary,
            category, state, needs_manual_review, read_only, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
        ON CONFLICT(id) DO UPDATE SET
            vault_path = excluded.vault_path,
            title = excluded.title,
            content_hash = excluded.content_hash,
            summary = COALESCE(excluded.summary, items.summary),
            category = COALESCE(excluded.category, items.category),
            state = excluded.state,
            needs_manual_review = excluded.needs_manual_review,
            updated_at = excluded.updated_at
        "#,
        params![
            record.id,
            record.vault_path,
            record.item_type,
            record.title,
            record.content_hash,
            record.summary,
            record.category,
            record.state,
            record.needs_manual_review,
            record.read_only,
            record.created_at,
            record.updated_at,
        ],
    )
    .map_err(|e| SandlandError::DatabaseError(format!("Falha ao salvar item: {}", e)))?;

    // Recupera o rowid_item
    let rowid_item: i64 = tx
        .query_row(
            "SELECT rowid_item FROM items WHERE id = ?1",
            params![record.id],
            |r| r.get(0),
        )
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao obter rowid_item: {}", e)))?;

    // 2. Atualizar índice FTS5 (deleta ocorrência anterior se houver)
    let _ = tx.execute(
        "DELETE FROM items_fts WHERE item_id = ?1",
        params![record.id],
    );

    tx.execute(
        "INSERT INTO items_fts (item_id, title, content) VALUES (?1, ?2, ?3)",
        params![record.id, record.title, content_raw],
    )
    .map_err(|e| SandlandError::DatabaseError(format!("Falha ao indexar FTS5: {}", e)))?;

    tx.commit()
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao commitar transação: {}", e)))?;

    Ok(rowid_item)
}

/// Busca tags associadas a um item
pub fn get_item_tags(conn: &Connection, item_id: &str) -> SandlandResult<Vec<String>> {
    let mut stmt = conn
        .prepare("SELECT term FROM item_taxonomy WHERE item_id = ?1 ORDER BY term ASC")
        .map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

    let tag_iter = stmt
        .query_map(params![item_id], |row| row.get::<_, String>(0))
        .map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

    let mut tags = Vec::new();
    for tag in tag_iter {
        if let Ok(t) = tag {
            tags.push(t);
        }
    }
    Ok(tags)
}

/// Sanitiza o texto de pré-visualização removendo marcadores Markdown, entidades HTML e quebras de linha excessivas
pub fn sanitize_snippet(text: &str, max_chars: usize) -> String {
    let body = if text.trim_start().starts_with("---") {
        if let Some(end_idx) = text[3..].find("---") {
            &text[3 + end_idx + 3..]
        } else {
            text
        }
    } else {
        text
    };

    let mut clean = String::with_capacity(body.len());
    let mut in_tag = false;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let content_line = trimmed.trim_start_matches('#').trim();
        if content_line.is_empty() {
            continue;
        }

        for ch in content_line.chars() {
            if ch == '<' {
                in_tag = true;
                continue;
            } else if ch == '>' {
                in_tag = false;
                continue;
            }
            if !in_tag {
                clean.push(ch);
            }
        }
        clean.push(' ');
    }

    let unescaped = clean
        .replace("&nbsp;", " ")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&#39;", "'")
        .replace("&mdash;", "—")
        .replace("&ndash;", "–");

    let words: Vec<&str> = unescaped.split_whitespace().collect();
    let normalized = words.join(" ");

    normalized.chars().take(max_chars).collect()
}

/// Lista itens aplicando filtros e paginação
pub fn list_items(
    conn: &Connection,
    filter: &IngestFilterDTO,
) -> SandlandResult<Vec<IngestedItemDTO>> {
    let limit = filter.limit.unwrap_or(50).min(200);
    let offset = filter.offset.unwrap_or(0);

    let mut sql = String::from(
        r#"
        SELECT DISTINCT i.id, i.vault_path, i.item_type, i.title, i.summary,
               i.category, i.state, i.needs_manual_review, i.created_at, i.updated_at,
               fts.content
        FROM items i
        LEFT JOIN items_fts fts ON fts.item_id = i.id
        "#,
    );

    let mut conditions = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(ref q) = filter.query {
        if !q.trim().is_empty() {
            sql.push_str(" JOIN items_fts f ON i.id = f.item_id");
            // Sanitize and format for fts5 match
            let clean_q = q.replace('"', "\"\"").replace('*', "");
            let match_q = format!("\"{}\"*", clean_q);
            conditions.push("f MATCH ?");
            params_vec.push(Box::new(match_q));
        }
    }

    if let Some(ref tag) = filter.tag {
        if !tag.trim().is_empty() {
            sql.push_str(" JOIN item_taxonomy it ON i.id = it.item_id");
            conditions.push("it.term = ?");
            params_vec.push(Box::new(tag.clone()));
        }
    }

    if let Some(ref cat) = filter.category {
        if !cat.trim().is_empty() {
            conditions.push("i.category = ?");
            params_vec.push(Box::new(cat.clone()));
        }
    }

    if let Some(review_only) = filter.needs_manual_review_only {
        if review_only {
            conditions.push("i.needs_manual_review = 1");
        }
    }

    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }

    sql.push_str(" ORDER BY i.updated_at DESC LIMIT ? OFFSET ?");
    params_vec.push(Box::new(limit));
    params_vec.push(Box::new(offset));

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| SandlandError::DatabaseError(format!("Erro de query SQL: {}", e)))?;

    let slice_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

    let rows = stmt
        .query_map(slice_params.as_slice(), |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, bool>(7)?,
                row.get::<_, i64>(8)?,
                row.get::<_, i64>(9)?,
                row.get::<_, Option<String>>(10)?,
            ))
        })
        .map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

    let mut items = Vec::new();
    for row in rows {
        let (
            id,
            vault_path,
            item_type,
            title,
            summary,
            category,
            state,
            needs_manual_review,
            created_at,
            updated_at,
            fts_content,
        ) = row.map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

        let tags = get_item_tags(conn, &id).unwrap_or_default();
        let raw_preview = summary
            .as_deref()
            .or(fts_content.as_deref())
            .unwrap_or(&title);
        let content_snippet = sanitize_snippet(raw_preview, 180);

        items.push(IngestedItemDTO {
            id,
            vault_path,
            title,
            item_type,
            summary,
            category,
            tags,
            state,
            needs_manual_review,
            content_snippet,
            created_at,
            updated_at,
        });
    }

    Ok(items)
}

/// Salva classificação taxonômica atômica (T030)
pub fn save_classification_atomic(
    conn: &mut Connection,
    item_id: &str,
    category: &str,
    tags: &[String],
    summary: &str,
    embedding: Option<&[f32]>,
) -> SandlandResult<()> {
    let tx = conn
        .transaction()
        .map_err(|e| SandlandError::DatabaseError(format!("Falha na transação: {}", e)))?;

    let now = chrono::Utc::now().timestamp();

    // 1. Atualiza categoria, summary e state no item
    tx.execute(
        r#"
        UPDATE items
        SET category = ?1, summary = ?2, state = 'Classified', updated_at = ?3
        WHERE id = ?4
        "#,
        params![category, summary, now, item_id],
    )
    .map_err(|e| SandlandError::DatabaseError(format!("Falha ao atualizar item: {}", e)))?;

    // 2. Insere/atualiza termos de taxonomia (categoria e tags)
    tx.execute(
        r#"
        INSERT INTO taxonomy_terms (term, term_type, frequency, created_at)
        VALUES (?1, 'category', 1, ?2)
        ON CONFLICT(term) DO UPDATE SET frequency = frequency + 1
        "#,
        params![category, now],
    )
    .map_err(|e| SandlandError::DatabaseError(format!("Falha ao registrar termo: {}", e)))?;

    // Remove tags antigas do item
    let _ = tx.execute("DELETE FROM item_taxonomy WHERE item_id = ?1", params![item_id]);

    for tag in tags {
        tx.execute(
            r#"
            INSERT INTO taxonomy_terms (term, term_type, frequency, created_at)
            VALUES (?1, 'tag', 1, ?2)
            ON CONFLICT(term) DO UPDATE SET frequency = frequency + 1
            "#,
            params![tag, now],
        )
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao salvar tag: {}", e)))?;

        tx.execute(
            r#"
            INSERT OR IGNORE INTO item_taxonomy (item_id, term)
            VALUES (?1, ?2)
            "#,
            params![item_id, tag],
        )
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao associar tag: {}", e)))?;
    }

    // 3. Se houver embedding, insere em vec_items usando o rowid_item
    if let Some(vec) = embedding {
        let rowid_item: i64 = tx
            .query_row(
                "SELECT rowid_item FROM items WHERE id = ?1",
                params![item_id],
                |r| r.get(0),
            )
            .map_err(|e| SandlandError::DatabaseError(format!("Falha ao obter rowid_item: {}", e)))?;

        // Converte &[f32] em bytes para a vtab vec0
        let bytes: &[u8] = bytemuck_cast_slice(vec);

        let _ = tx.execute("DELETE FROM vec_items WHERE rowid = ?1", params![rowid_item]);
        let _ = tx.execute(
            "INSERT INTO vec_items (rowid, embedding) VALUES (?1, ?2)",
            params![rowid_item, bytes],
        );
    }

    tx.commit()
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao commitar taxonomia: {}", e)))?;

    Ok(())
}

fn bytemuck_cast_slice(floats: &[f32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            floats.as_ptr() as *const u8,
            floats.len() * std::mem::size_of::<f32>(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RrfSearchResultDTO {
    pub id: String,
    pub vault_path: String,
    pub title: String,
    pub category: Option<String>,
    pub summary: Option<String>,
    pub rrf_score: f64,
}

/// T046: Busca Híbrida Unificada com Reciprocal Rank Fusion (RRF k=60)
pub fn search_rrf_hybrid(
    conn: &Connection,
    query_text: &str,
    embedding: Option<&[f32]>,
    limit: i64,
) -> SandlandResult<Vec<RrfSearchResultDTO>> {
    let clean_q = query_text.replace('"', "\"\"").replace('*', "");
    let match_q = format!("\"{}\"*", clean_q);

    let sql = r#"
        WITH fts_results AS (
            SELECT f.rowid AS rowid_item,
                   ROW_NUMBER() OVER (ORDER BY rank) as rank_fts
            FROM items_fts f
            WHERE items_fts MATCH ?1
            LIMIT 50
        ),
        vec_results AS (
            SELECT rowid AS rowid_item,
                   ROW_NUMBER() OVER (ORDER BY distance ASC) as rank_vec
            FROM vec_items
            WHERE embedding MATCH ?2 AND k = 50
        )
        SELECT i.id, i.vault_path, i.title, i.category, i.summary,
               COALESCE(1.0 / (60.0 + f.rank_fts), 0.0) +
               COALESCE(1.0 / (60.0 + v.rank_vec), 0.0) AS rrf_score
        FROM items i
        LEFT JOIN fts_results f ON i.rowid_item = f.rowid_item
        LEFT JOIN vec_results v ON i.rowid_item = v.rowid_item
        WHERE f.rowid_item IS NOT NULL OR v.rowid_item IS NOT NULL
        ORDER BY rrf_score DESC
        LIMIT ?3;
    "#;

    let dummy_vec: [f32; 384] = [0.0; 384];
    let vec_bytes: &[u8] = bytemuck_cast_slice(embedding.unwrap_or(&dummy_vec));

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao preparar query RRF: {}", e)))?;

    let rows = stmt
        .query_map(params![match_q, vec_bytes, limit], |row| {
            Ok(RrfSearchResultDTO {
                id: row.get(0)?,
                vault_path: row.get(1)?,
                title: row.get(2)?,
                category: row.get(3)?,
                summary: row.get(4)?,
                rrf_score: row.get(5)?,
            })
        })
        .map_err(|e| SandlandError::DatabaseError(e.to_string()))?;

    let mut results = Vec::new();
    for r in rows {
        if let Ok(res) = r {
            results.push(res);
        }
    }

    Ok(results)
}

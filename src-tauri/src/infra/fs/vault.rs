use crate::domain::core::errors::{SandlandError, SandlandResult};
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::path::Path;
use walkdir::WalkDir;
pub use crate::infra::security::VaultGuard;

/// Serializa nota em Markdown com YAML Frontmatter limpo e canônico
pub fn format_markdown_with_frontmatter(
    id: &str,
    title: &str,
    item_type: &str,
    category: Option<&str>,
    tags: &[String],
    summary: Option<&str>,
    body: &str,
) -> String {
    let now = chrono::Utc::now().timestamp();
    let mut frontmatter = format!(
        "---\nid: \"{}\"\ntitle: \"{}\"\nitem_type: \"{}\"\n",
        id, title, item_type
    );

    if let Some(cat) = category {
        frontmatter.push_str(&format!("category: \"{}\"\n", cat));
    }

    if !tags.is_empty() {
        frontmatter.push_str("tags:\n");
        for tag in tags {
            frontmatter.push_str(&format!("  - \"{}\"\n", tag));
        }
    }

    if let Some(sum) = summary {
        frontmatter.push_str(&format!("summary: \"{}\"\n", sum));
    }

    frontmatter.push_str(&format!(
        "created_at: {}\nupdated_at: {}\n---\n\n{}",
        now, now, body
    ));

    frontmatter
}

/// Extrai metadados do Frontmatter e corpo limpo de um texto Markdown
pub fn parse_frontmatter(raw: &str) -> (Option<serde_json::Value>, String) {
    if raw.starts_with("---\n") {
        if let Some(end_idx) = raw[4..].find("\n---\n") {
            let frontmatter_str = &raw[4..4 + end_idx];
            let body = &raw[4 + end_idx + 5..];
            
            // Simples parser chave-valor para v0.1
            let mut map = serde_json::Map::new();
            for line in frontmatter_str.lines() {
                if let Some((k, v)) = line.split_once(':') {
                    let key = k.trim().to_string();
                    let val = v.trim().trim_matches('"').to_string();
                    map.insert(key, serde_json::Value::String(val));
                }
            }
            return (Some(serde_json::Value::Object(map)), body.trim().to_string());
        }
    }
    (None, raw.trim().to_string())
}

/// Inicializa a estrutura de diretórios canônica do cofre
pub fn initialize_vault_structure(vault_root: &Path) -> SandlandResult<()> {
    let subdirs = [
        ".system",
        ".history/wal",
        ".history/events",
        "ingest/notes",
        "ingest/web",
        "ingest/media",
        "workspaces",
        "workspaces/default-workspace",
        "workspaces/default-workspace/cells",
        "assets",
        "intentions",
    ];

    for subdir in &subdirs {
        let dir = vault_root.join(subdir);
        std::fs::create_dir_all(&dir)?;
    }

    Ok(())
}

/// Garante a estrutura de diretórios canônica para um workspace específico
pub fn ensure_workspace_structure(vault_root: &Path, workspace_id: &str) -> SandlandResult<std::path::PathBuf> {
    let ws_dir = vault_root.join("workspaces").join(workspace_id);
    let cells_dir = ws_dir.join("cells");
    std::fs::create_dir_all(&cells_dir)?;
    Ok(ws_dir)
}

/// Rotina de Hidratação do Cofre no Startup (File-as-Truth):
/// Percorre `/vault/ingest/` e sincroniza notas novas ou alteradas no SQLite `index.db`
pub fn hydrate_vault_from_disk(vault_guard: &VaultGuard, conn: &mut Connection) -> SandlandResult<usize> {
    let ingest_path = vault_guard.base_path().join("ingest");
    if !ingest_path.exists() {
        return Ok(0);
    }

    let mut hydrated_count = 0;
    let tx = conn
        .transaction()
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao iniciar transação de hidratação: {}", e)))?;

    for entry in WalkDir::new(&ingest_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md" || ext == "txt") {
            let relative_path = path.strip_prefix(vault_guard.base_path()).unwrap_or(path);
            let vault_path_str = relative_path.to_string_lossy().replace('\\', "/");

            if let Ok(content) = std::fs::read_to_string(path) {
                let mut hasher = Sha256::new();
                hasher.update(content.as_bytes());
                let content_hash = format!("{:x}", hasher.finalize());

                // Verifica se já está indexado com o mesmo hash e obtém o id existente se houver
                let mut existing_info: Option<(String, String)> = None;
                {
                    let mut stmt = tx.prepare("SELECT id, content_hash FROM items WHERE vault_path = ?1")?;
                    let mut rows = stmt.query([&vault_path_str])?;
                    if let Some(row) = rows.next()? {
                        let id: String = row.get(0)?;
                        let hash: String = row.get(1)?;
                        existing_info = Some((id, hash));
                    }
                }

                let needs_update = match &existing_info {
                    Some((_, existing_hash)) => existing_hash != &content_hash,
                    None => true,
                };

                if needs_update {
                    let (fm_opt, body) = parse_frontmatter(&content);

                    let id = fm_opt
                        .as_ref()
                        .and_then(|fm| fm.get("id"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| existing_info.map(|(old_id, _)| old_id))
                        .unwrap_or_else(|| uuid::Uuid::now_v7().to_string());

                    let title = fm_opt
                        .as_ref()
                        .and_then(|fm| fm.get("title"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| {
                            body.lines()
                                .find(|l| l.starts_with("# "))
                                .map(|l| l.trim_start_matches("# ").trim().to_string())
                        })
                        .unwrap_or_else(|| {
                            path.file_stem()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_else(|| "Nota sem título".to_string())
                        });

                    let item_type = fm_opt
                        .as_ref()
                        .and_then(|fm| fm.get("item_type"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_else(|| {
                            if vault_path_str.contains("/web/") {
                                "web_snapshot"
                            } else {
                                "note"
                            }
                        });

                    let now = chrono::Utc::now().timestamp();

                    tx.execute(
                        r#"
                        INSERT INTO items (id, vault_path, item_type, title, content_hash, state, created_at, updated_at)
                        VALUES (?1, ?2, ?3, ?4, ?5, 'Pending', ?6, ?6)
                        ON CONFLICT(vault_path) DO UPDATE SET
                            id = excluded.id,
                            title = excluded.title,
                            content_hash = excluded.content_hash,
                            updated_at = excluded.updated_at
                        "#,
                        rusqlite::params![id, vault_path_str, item_type, title, content_hash, now],
                    )?;

                    // Indexa no FTS5
                    tx.execute(
                        "INSERT OR REPLACE INTO items_fts (item_id, title, content) VALUES (?1, ?2, ?3)",
                        rusqlite::params![id, title, body],
                    )?;

                    hydrated_count += 1;
                }
            }
        }
    }

    tx.commit()
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao confirmar transação de hidratação: {}", e)))?;

    Ok(hydrated_count)
}

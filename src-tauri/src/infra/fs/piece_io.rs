use crate::domain::core::errors::{SandlandError, SandlandResult};
use crate::domain::workspace::cell::WorkspaceCell;
use crate::domain::workspace::piece::{CitationDrift, EditorialPiece};
use crate::infra::fs::vault::VaultGuard;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PieceSummary {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub slug: String,
    pub citation_count: usize,
    pub word_count: u32,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationDriftItem {
    pub citation_id: String,
    pub source_cell_id: String,
    pub status: CitationDrift,
    pub cited_revision: u64,
    pub current_revision: Option<u64>,
    pub current_title: Option<String>,
    pub current_snippet: Option<String>,
}

pub struct PieceStorage;

impl PieceStorage {
    /// Salva uma Peça Editorial de forma atômica no disco em workspaces/<id>/pieces/<piece_id>.md
    pub fn save_piece(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        mut piece: EditorialPiece,
    ) -> SandlandResult<(String, u32, i64)> {
        piece.recalculate_metrics();

        let pieces_dir = format!("workspaces/{}/pieces", workspace_id);
        let piece_rel = format!("{}/{}.md", pieces_dir, piece.id);
        let piece_path = Path::new(&piece_rel);

        let md_content = piece.to_markdown();
        vault_guard.secure_write(piece_path, md_content.as_bytes())?;

        Ok((piece_rel, piece.word_count, piece.updated_at))
    }

    /// Lê o conteúdo de uma Peça do disco
    pub fn read_piece(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        piece_id: &str,
    ) -> SandlandResult<EditorialPiece> {
        // Tenta primeiro pelo nome padrão <piece_id>.md
        let direct_rel = format!("workspaces/{}/pieces/{}.md", workspace_id, piece_id);
        if let Ok(raw) = vault_guard.secure_read_to_string(Path::new(&direct_rel)) {
            return EditorialPiece::from_markdown(&raw)
                .map_err(|e| SandlandError::SerializationError(e));
        }

        // Se não encontrar direto, busca pelo ID no frontmatter percorrendo a pasta pieces
        let pieces_dir_rel = format!("workspaces/{}/pieces", workspace_id);
        let full_dir = vault_guard.resolve_relative_path(Path::new(&pieces_dir_rel))?;

        if full_dir.exists() {
            for entry in WalkDir::new(&full_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        if let Ok(parsed) = EditorialPiece::from_markdown(&content) {
                            if parsed.id == piece_id || parsed.slug == piece_id {
                                return Ok(parsed);
                            }
                        }
                    }
                }
            }
        }

        Err(SandlandError::NotFound(format!(
            "Peça editorial '{}' não encontrada no workspace '{}'",
            piece_id, workspace_id
        )))
    }

    /// Lista todas as Peças editoriais registradas no workspace
    pub fn list_pieces(
        vault_guard: &VaultGuard,
        workspace_id: &str,
    ) -> SandlandResult<Vec<PieceSummary>> {
        let pieces_dir_rel = format!("workspaces/{}/pieces", workspace_id);
        let full_dir = match vault_guard.resolve_relative_path(Path::new(&pieces_dir_rel)) {
            Ok(p) => p,
            Err(_) => return Ok(vec![]),
        };

        if !full_dir.exists() {
            return Ok(vec![]);
        }

        let mut summaries = Vec::new();
        for entry in WalkDir::new(&full_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(parsed) = EditorialPiece::from_markdown(&content) {
                        summaries.push(PieceSummary {
                            id: parsed.id,
                            workspace_id: parsed.workspace_id,
                            title: parsed.title,
                            slug: parsed.slug,
                            citation_count: parsed.citations.len(),
                            word_count: parsed.word_count,
                            updated_at: parsed.updated_at,
                        });
                    }
                }
            }
        }

        summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(summaries)
    }

    /// Executa auditoria determinística de divergência de citações contra as células em disco
    pub fn check_drift(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        piece_id: &str,
    ) -> SandlandResult<Vec<CitationDriftItem>> {
        let piece = Self::read_piece(vault_guard, workspace_id, piece_id)?;
        let mut results = Vec::new();

        for cit in &piece.citations {
            let cell_rel = format!("workspaces/{}/cells/{}.md", workspace_id, cit.source_cell_id);
            let cell_path = Path::new(&cell_rel);

            match vault_guard.secure_read_to_string(cell_path) {
                Ok(raw) => {
                    let cell = WorkspaceCell::from_markdown(&raw)
                        .unwrap_or_else(|_| WorkspaceCell::new(cit.source_cell_id.clone(), workspace_id.to_string()));

                    let status = if cell.revision == cit.source_revision {
                        CitationDrift::Synchronized
                    } else {
                        CitationDrift::Diverged
                    };

                    let snippet = if cell.content.len() > 140 {
                        format!("{}...", &cell.content[..140])
                    } else {
                        cell.content.clone()
                    };

                    results.push(CitationDriftItem {
                        citation_id: cit.id.clone(),
                        source_cell_id: cit.source_cell_id.clone(),
                        status,
                        cited_revision: cit.source_revision,
                        current_revision: Some(cell.revision),
                        current_title: Some(cell.title),
                        current_snippet: Some(snippet),
                    });
                }
                Err(_) => {
                    results.push(CitationDriftItem {
                        citation_id: cit.id.clone(),
                        source_cell_id: cit.source_cell_id.clone(),
                        status: CitationDrift::Orphaned,
                        cited_revision: cit.source_revision,
                        current_revision: None,
                        current_title: None,
                        current_snippet: None,
                    });
                }
            }
        }

        Ok(results)
    }

    /// Compila a Peça para exportação com apêndice bibliográfico de citações
    pub fn compile_export(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        piece_id: &str,
        include_references: bool,
    ) -> SandlandResult<String> {
        let piece = Self::read_piece(vault_guard, workspace_id, piece_id)?;
        let mut out = String::new();

        if !piece.title.is_empty() && !piece.body.starts_with("# ") {
            out.push_str(&format!("# {}\n\n", piece.title));
        }
        out.push_str(&piece.body);

        if include_references && !piece.citations.is_empty() {
            out.push_str("\n\n---\n\n## Referências & Proveniência\n\n");
            for (idx, cit) in piece.citations.iter().enumerate() {
                out.push_str(&format!(
                    "{}. **{}** (Célula `{}`) - Revisão {} citada em {}.\n   > \"{}\"\n\n",
                    idx + 1,
                    cit.source_title,
                    cit.source_cell_id,
                    cit.source_revision,
                    chrono::DateTime::from_timestamp(cit.inserted_at, 0)
                        .map(|dt| dt.format("%d/%m/%Y %H:%M").to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                    cit.quote.replace('\n', " ")
                ));
            }
        }

        Ok(out)
    }
}

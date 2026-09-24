use crate::domain::core::errors::{SandlandError, SandlandResult};
use crate::domain::workspace::cell::WorkspaceCell;
use crate::domain::workspace::topology::BoardTopology;
use crate::infra::fs::vault::VaultGuard;
use std::path::Path;

pub struct CanvasStorage;

impl CanvasStorage {
    /// Carrega a topologia do canvas, priorizando board.canvas.json sobre topology.json e .mpk
    pub fn load_topology(
        vault_guard: &VaultGuard,
        workspace_id: &str,
    ) -> SandlandResult<BoardTopology> {
        let ws_dir = format!("workspaces/{}", workspace_id);
        let json_rel = format!("{}/board.canvas.json", ws_dir);
        let topo_rel = format!("{}/topology.json", ws_dir);
        let mpk_rel = format!("{}/board.canvas.mpk", ws_dir);

        // 1. Tenta carregar board.canvas.json (canônico)
        if let Ok(content) = vault_guard.secure_read_to_string(Path::new(&json_rel)) {
            if let Ok(topology) = serde_json::from_str::<BoardTopology>(&content) {
                return Ok(topology);
            }
        }

        // 2. Fallback para topology.json
        if let Ok(content) = vault_guard.secure_read_to_string(Path::new(&topo_rel)) {
            if let Ok(topology) = serde_json::from_str::<BoardTopology>(&content) {
                return Ok(topology);
            }
        }

        // 3. Fallback para MessagePack
        if let Ok((mut file, _)) = vault_guard.secure_open_read(Path::new(&mpk_rel)) {
            if let Ok(topology) = rmp_serde::from_read::<_, BoardTopology>(&mut file) {
                return Ok(topology);
            }
        }

        // 4. Se não existir, retorna nova topologia inicial
        Ok(BoardTopology::new(workspace_id.to_string()))
    }

    /// Salva a topologia atomicamente com controle de concorrência otimista (OCC)
    pub fn save_board_topology(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        mut topology: BoardTopology,
        expected_revision: Option<u64>,
    ) -> SandlandResult<u64> {
        // Valida invariantes do domínio (ex: auto-loops)
        topology
            .validate()
            .map_err(|e| SandlandError::InvalidInput(e))?;

        let ws_dir = format!("workspaces/{}", workspace_id);
        let json_rel = format!("{}/board.canvas.json", ws_dir);

        // Se o arquivo já existe em disco, checa a revisão atual para controle OCC
        let current_disk_revision = match vault_guard.secure_read_to_string(Path::new(&json_rel)) {
            Ok(content) => match serde_json::from_str::<BoardTopology>(&content) {
                Ok(existing) => Some(existing.revision),
                Err(_) => None,
            },
            Err(_) => None,
        };

        if let Some(actual_rev) = current_disk_revision {
            if let Some(exp_rev) = expected_revision {
                if actual_rev != exp_rev {
                    return Err(SandlandError::RevisionConflict {
                        expected: Some(exp_rev),
                        actual: actual_rev,
                    });
                }
            }
            topology.revision = actual_rev + 1;
        } else {
            topology.revision = expected_revision.unwrap_or(0) + 1;
        }

        topology.updated_at = chrono::Utc::now().timestamp();

        let json_str = serde_json::to_string_pretty(&topology)
            .map_err(|e| SandlandError::SerializationError(e.to_string()))?;

        // 1. Grava board.canvas.json canônico com fsync atômico
        vault_guard.secure_write(Path::new(&json_rel), json_str.as_bytes())?;

        // 2. Sincroniza topology.json para compatibilidade
        let topo_rel = format!("{}/topology.json", ws_dir);
        let _ = vault_guard.secure_write(Path::new(&topo_rel), json_str.as_bytes());

        // 3. Salva MessagePack binário
        if let Ok(mpk_bytes) = rmp_serde::to_vec_named(&topology) {
            let mpk_rel = format!("{}/board.canvas.mpk", ws_dir);
            let _ = vault_guard.secure_write(Path::new(&mpk_rel), &mpk_bytes);
        }

        Ok(topology.revision)
    }

    /// Salva a topologia de forma canônica (compatibilidade com versão anterior)
    pub fn save_topology(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        topology: &BoardTopology,
    ) -> SandlandResult<()> {
        Self::save_board_topology(vault_guard, workspace_id, topology.clone(), None)?;
        Ok(())
    }

    /// Salva canonicamente uma célula em workspaces/<id>/cells/<cell_id>.md
    pub fn save_workspace_cell(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        cell_id: &str,
        content: &str,
        frontmatter: Option<serde_json::Value>,
    ) -> SandlandResult<(String, u64)> {
        let cell_rel = format!("workspaces/{}/cells/{}.md", workspace_id, cell_id);
        let cell_path = Path::new(&cell_rel);

        let mut cell = if let Ok(raw) = vault_guard.secure_read_to_string(cell_path) {
            WorkspaceCell::from_markdown(&raw)
                .unwrap_or_else(|_| WorkspaceCell::new(cell_id.to_string(), workspace_id.to_string()))
        } else {
            WorkspaceCell::new(cell_id.to_string(), workspace_id.to_string())
        };

        // Aplica atualizações do frontmatter se informados
        if let Some(fm) = frontmatter {
            if let Some(obj) = fm.as_object() {
                if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
                    cell.title = title.to_string();
                }
                if let Some(node_id) = obj.get("nodeId").and_then(|v| v.as_str()) {
                    cell.node_id = Some(node_id.to_string());
                }
                if let Some(item_id) = obj.get("itemId").and_then(|v| v.as_str()) {
                    cell.item_id = Some(item_id.to_string());
                }
                if let Some(source_path) = obj.get("sourcePath").and_then(|v| v.as_str()) {
                    cell.source_path = Some(source_path.to_string());
                }
                if let Some(source_hash) = obj.get("sourceHash").and_then(|v| v.as_str()) {
                    cell.source_hash = Some(source_hash.to_string());
                }
                if let Some(tags_val) = obj.get("tags").and_then(|v| v.as_array()) {
                    cell.tags = tags_val
                        .iter()
                        .filter_map(|t| t.as_str().map(|s| s.to_string()))
                        .collect();
                }
            }
        }

        cell.content = content.to_string();
        cell.revision += 1;
        cell.updated_at = chrono::Utc::now().timestamp();

        let md_content = cell.to_markdown();
        vault_guard.secure_write(cell_path, md_content.as_bytes())?;

        Ok((cell_rel, cell.revision))
    }

    /// Lê o conteúdo em Markdown de uma célula salva no workspace
    pub fn read_workspace_cell(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        cell_id: &str,
    ) -> SandlandResult<String> {
        let cell_rel = format!("workspaces/{}/cells/{}.md", workspace_id, cell_id);
        vault_guard
            .secure_read_to_string(Path::new(&cell_rel))
            .map_err(|_| SandlandError::NotFound(format!("Célula '{}' não encontrada", cell_id)))
    }

    /// Salva de alta frequência em MessagePack (.mpk) a cada 350ms
    pub fn save_fast_mpk(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        mpk_bytes: &[u8],
    ) -> SandlandResult<()> {
        let mpk_rel = format!("workspaces/{}/board.canvas.mpk", workspace_id);
        vault_guard.secure_write(Path::new(&mpk_rel), mpk_bytes)?;
        Ok(())
    }

    /// Sincroniza em repouso (>2s) em formato JSON legível
    pub fn sync_idle_json(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        topology: &BoardTopology,
    ) -> SandlandResult<()> {
        Self::save_topology(vault_guard, workspace_id, topology)
    }
}

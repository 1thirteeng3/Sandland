use crate::domain::core::errors::{SandlandError, SandlandResult};
use crate::domain::workspace::topology::BoardTopology;
use crate::infra::fs::vault::VaultGuard;
use std::path::Path;

pub struct CanvasStorage;

impl CanvasStorage {
    /// Carrega a topologia do canvas, priorizando topology.json sobre board.canvas.json e .mpk
    pub fn load_topology(
        vault_guard: &VaultGuard,
        workspace_id: &str,
    ) -> SandlandResult<BoardTopology> {
        let ws_dir = format!("workspaces/{}", workspace_id);
        let topo_rel = format!("{}/topology.json", ws_dir);
        let json_rel = format!("{}/board.canvas.json", ws_dir);
        let mpk_rel = format!("{}/board.canvas.mpk", ws_dir);

        // 1. Tenta carregar topology.json
        if let Ok(content) = vault_guard.secure_read_to_string(Path::new(&topo_rel)) {
            if let Ok(topology) = serde_json::from_str::<BoardTopology>(&content) {
                return Ok(topology);
            }
        }

        // 2. Fallback para board.canvas.json
        if let Ok(content) = vault_guard.secure_read_to_string(Path::new(&json_rel)) {
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

    /// Salva a topologia de forma canônica em topology.json e sincroniza board.canvas.json
    pub fn save_topology(
        vault_guard: &VaultGuard,
        workspace_id: &str,
        topology: &BoardTopology,
    ) -> SandlandResult<()> {
        let topo_rel = format!("workspaces/{}/topology.json", workspace_id);
        let json_str = serde_json::to_string_pretty(topology)
            .map_err(|e| SandlandError::SerializationError(e.to_string()))?;
        vault_guard.secure_write(Path::new(&topo_rel), json_str.as_bytes())?;

        // Também sincroniza board.canvas.json para compatibilidade
        let json_rel = format!("workspaces/{}/board.canvas.json", workspace_id);
        let _ = vault_guard.secure_write(Path::new(&json_rel), json_str.as_bytes());

        // Salva MessagePack binário
        if let Ok(mpk_bytes) = rmp_serde::to_vec_named(topology) {
            let mpk_rel = format!("workspaces/{}/board.canvas.mpk", workspace_id);
            let _ = vault_guard.secure_write(Path::new(&mpk_rel), &mpk_bytes);
        }

        Ok(())
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

use crate::domain::core::errors::SandlandError;
use crate::domain::workspace::topology::{BoardTopology, CanvasEdge, CanvasNode, ViewportState};
use crate::infra::fs::canvas_io::CanvasStorage;
use crate::ipc::vault::AppState;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDTO {
    pub id: String,
    pub title: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDTO {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellDTO {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub content: String,
    pub position: PositionDTO,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasTopologyDTO {
    pub workspace_id: String,
    pub viewport: ViewportState,
    pub nodes: Vec<CanvasNode>,
    pub edges: Vec<CanvasEdge>,
    pub revision: u64,
    pub updated_at: i64,
}

pub type BoardTopologyDTO = CanvasTopologyDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCellResponseDTO {
    pub cell_id: String,
    pub path: String,
    pub revision: u64,
}

#[tauri::command]
pub async fn create_workspace(
    state: State<'_, AppState>,
    title: String,
) -> Result<WorkspaceDTO, SandlandError> {
    let id = Uuid::now_v7().to_string();
    let now = chrono::Utc::now().timestamp();

    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    // Inicializa diretório do workspace
    let ws_dir = format!("workspaces/{}", id);
    guard.secure_write(Path::new(&format!("{}/.keep", ws_dir)), b"")?;

    // Cria topologia inicial
    let topology = BoardTopology::new(id.clone());
    CanvasStorage::sync_idle_json(guard, &id, &topology)?;

    Ok(WorkspaceDTO {
        id,
        title,
        created_at: now,
    })
}

#[tauri::command]
pub async fn load_board_topology(
    state: State<'_, AppState>,
    workspace_id: String,
) -> Result<CanvasTopologyDTO, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    let topology = CanvasStorage::load_topology(guard, &workspace_id)?;

    Ok(CanvasTopologyDTO {
        workspace_id: topology.workspace_id,
        viewport: topology.viewport,
        nodes: topology.nodes,
        edges: topology.edges,
        revision: topology.revision,
        updated_at: topology.updated_at,
    })
}

#[tauri::command]
pub async fn save_board_topology_fast(
    state: State<'_, AppState>,
    workspace_id: String,
    topology_mpk: Vec<u8>,
) -> Result<(), SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    CanvasStorage::save_fast_mpk(guard, &workspace_id, &topology_mpk)?;

    // Tenta desserializar em background para atualizar o json em repouso
    if let Ok(topology) = rmp_serde::from_slice::<BoardTopology>(&topology_mpk) {
        let _ = CanvasStorage::sync_idle_json(guard, &workspace_id, &topology);
    }

    Ok(())
}

#[tauri::command]
pub async fn save_board_topology(
    state: State<'_, AppState>,
    workspace_id: String,
    topology: BoardTopology,
    expected_revision: Option<u64>,
) -> Result<u64, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    let new_rev = CanvasStorage::save_board_topology(guard, &workspace_id, topology, expected_revision)?;
    Ok(new_rev)
}

#[tauri::command]
pub async fn save_workspace_cell(
    state: State<'_, AppState>,
    workspace_id: String,
    cell_id: String,
    content: String,
    frontmatter: Option<serde_json::Value>,
) -> Result<SaveCellResponseDTO, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    let (path, revision) = CanvasStorage::save_workspace_cell(
        guard,
        &workspace_id,
        &cell_id,
        &content,
        frontmatter,
    )?;

    Ok(SaveCellResponseDTO {
        cell_id,
        path,
        revision,
    })
}

#[tauri::command]
pub async fn read_workspace_cell(
    state: State<'_, AppState>,
    workspace_id: String,
    cell_id: String,
) -> Result<String, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    CanvasStorage::read_workspace_cell(guard, &workspace_id, &cell_id)
}

#[tauri::command]
pub async fn create_cell(
    state: State<'_, AppState>,
    workspace_id: String,
    content: String,
    position: PositionDTO,
) -> Result<CellDTO, SandlandError> {
    let id = Uuid::now_v7().to_string();
    let now = chrono::Utc::now().timestamp();

    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    let title = content
        .lines()
        .next()
        .unwrap_or("Nova Célula")
        .chars()
        .take(40)
        .collect::<String>();

    let cell_path = format!("workspaces/{}/cells/{}.md", workspace_id, id);
    guard.secure_write(Path::new(&cell_path), content.as_bytes())?;

    Ok(CellDTO {
        id,
        workspace_id,
        title,
        content,
        position,
        tags: vec![],
        created_at: now,
        updated_at: now,
    })
}

#[tauri::command]
pub async fn list_cells(
    _state: State<'_, AppState>,
    _workspace_id: String,
) -> Result<Vec<CellDTO>, SandlandError> {
    Ok(vec![])
}

// ---------------------------------------------------------
// COMANDOS IPC DE PEÇAS EDITORIAIS & PROVENIÊNCIA (SPRINT 04)
// ---------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PieceSaveResponseDTO {
    pub piece_id: String,
    pub path: String,
    pub word_count: u32,
    pub updated_at: i64,
}

#[tauri::command]
pub async fn list_workspace_pieces(
    state: State<'_, AppState>,
    workspace_id: String,
) -> Result<Vec<crate::infra::fs::piece_io::PieceSummary>, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    crate::infra::fs::piece_io::PieceStorage::list_pieces(guard, &workspace_id)
}

#[tauri::command]
pub async fn read_workspace_piece(
    state: State<'_, AppState>,
    workspace_id: String,
    piece_id: String,
) -> Result<crate::domain::workspace::piece::EditorialPiece, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    crate::infra::fs::piece_io::PieceStorage::read_piece(guard, &workspace_id, &piece_id)
}

#[tauri::command]
pub async fn save_workspace_piece(
    state: State<'_, AppState>,
    workspace_id: String,
    piece: crate::domain::workspace::piece::EditorialPiece,
) -> Result<PieceSaveResponseDTO, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    let piece_id = piece.id.clone();
    let (path, word_count, updated_at) =
        crate::infra::fs::piece_io::PieceStorage::save_piece(guard, &workspace_id, piece)?;

    Ok(PieceSaveResponseDTO {
        piece_id,
        path,
        word_count,
        updated_at,
    })
}

#[tauri::command]
pub async fn check_piece_citations_drift(
    state: State<'_, AppState>,
    workspace_id: String,
    piece_id: String,
) -> Result<Vec<crate::infra::fs::piece_io::CitationDriftItem>, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    crate::infra::fs::piece_io::PieceStorage::check_drift(guard, &workspace_id, &piece_id)
}

#[tauri::command]
pub async fn compile_piece_export(
    state: State<'_, AppState>,
    workspace_id: String,
    piece_id: String,
    include_references: Option<bool>,
) -> Result<String, SandlandError> {
    let guard_opt = state.vault_guard.lock().unwrap();
    let guard = guard_opt.as_ref().ok_or_else(|| {
        SandlandError::DatabaseError("Nenhum cofre aberto".to_string())
    })?;

    crate::infra::fs::piece_io::PieceStorage::compile_export(
        guard,
        &workspace_id,
        &piece_id,
        include_references.unwrap_or(true),
    )
}


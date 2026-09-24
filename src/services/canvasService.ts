import { invokeCommand } from './ipc';
import type { BoardTopologyDTO, CellDTO, CellRecordDTO, PositionDTO } from '../types/canvas';

export async function createWorkspace(title: string): Promise<{ id: string; title: string; createdAt: number }> {
  return invokeCommand('create_workspace', { title });
}

export async function loadBoardTopology(workspaceId: string): Promise<BoardTopologyDTO> {
  return invokeCommand<BoardTopologyDTO>('load_board_topology', { workspaceId });
}

export async function saveBoardTopology(
  workspaceId: string,
  topology: BoardTopologyDTO,
  expectedRevision?: number | null
): Promise<number> {
  return invokeCommand<number>('save_board_topology', {
    workspaceId,
    topology,
    expectedRevision: expectedRevision ?? null,
  });
}

export async function saveBoardTopologyFast(workspaceId: string, topologyMpk: Uint8Array): Promise<void> {
  return invokeCommand<void>('save_board_topology_fast', {
    workspaceId,
    topologyMpk: Array.from(topologyMpk),
  });
}

export async function saveWorkspaceCell(
  workspaceId: string,
  cellId: string,
  content: string,
  frontmatter?: Record<string, unknown>
): Promise<CellRecordDTO> {
  return invokeCommand<CellRecordDTO>('save_workspace_cell', {
    workspaceId,
    cellId,
    content,
    frontmatter: frontmatter ?? {},
  });
}

export async function readWorkspaceCell(
  workspaceId: string,
  cellId: string
): Promise<string> {
  return invokeCommand<string>('read_workspace_cell', {
    workspaceId,
    cellId,
  });
}

export async function createCell(
  workspaceId: string,
  content: string,
  position: PositionDTO
): Promise<CellDTO> {
  return invokeCommand<CellDTO>('create_cell', { workspaceId, content, position });
}

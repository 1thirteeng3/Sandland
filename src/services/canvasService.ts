import { invokeCommand } from './ipc';
import type { CanvasTopologyDTO, CellDTO, PositionDTO } from '../types/canvas';

export async function createWorkspace(title: string): Promise<{ id: string; title: string; createdAt: number }> {
  return invokeCommand('create_workspace', { title });
}

export async function loadBoardTopology(workspaceId: string): Promise<CanvasTopologyDTO> {
  return invokeCommand<CanvasTopologyDTO>('load_board_topology', { workspaceId });
}

export async function saveBoardTopology(workspaceId: string, topology: CanvasTopologyDTO): Promise<void> {
  return invokeCommand<void>('save_board_topology', {
    workspaceId,
    topology,
  });
}

export async function saveBoardTopologyFast(workspaceId: string, topologyMpk: Uint8Array): Promise<void> {
  return invokeCommand<void>('save_board_topology_fast', {
    workspaceId,
    topologyMpk: Array.from(topologyMpk),
  });
}

export async function createCell(
  workspaceId: string,
  content: string,
  position: PositionDTO
): Promise<CellDTO> {
  return invokeCommand<CellDTO>('create_cell', { workspaceId, content, position });
}

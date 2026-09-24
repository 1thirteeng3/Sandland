import { invokeCommand } from './ipc';
import type {
  EditorialPieceDTO,
  PieceSummaryDTO,
  PieceSaveResponseDTO,
  CitationDriftItemDTO,
} from '../types/piece';

export async function listWorkspacePieces(workspaceId: string): Promise<PieceSummaryDTO[]> {
  return invokeCommand<PieceSummaryDTO[]>('list_workspace_pieces', { workspaceId });
}

export async function readWorkspacePiece(
  workspaceId: string,
  pieceId: string
): Promise<EditorialPieceDTO> {
  return invokeCommand<EditorialPieceDTO>('read_workspace_piece', {
    workspaceId,
    pieceId,
  });
}

export async function saveWorkspacePiece(
  workspaceId: string,
  piece: EditorialPieceDTO
): Promise<PieceSaveResponseDTO> {
  return invokeCommand<PieceSaveResponseDTO>('save_workspace_piece', {
    workspaceId,
    piece,
  });
}

export async function checkPieceCitationsDrift(
  workspaceId: string,
  pieceId: string
): Promise<CitationDriftItemDTO[]> {
  return invokeCommand<CitationDriftItemDTO[]>('check_piece_citations_drift', {
    workspaceId,
    pieceId,
  });
}

export async function compilePieceExport(
  workspaceId: string,
  pieceId: string,
  includeReferences: boolean = true
): Promise<string> {
  return invokeCommand<string>('compile_piece_export', {
    workspaceId,
    pieceId,
    includeReferences,
  });
}

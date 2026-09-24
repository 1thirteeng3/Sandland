export type CitationDriftStatus = 'Synchronized' | 'Diverged' | 'Orphaned';

export interface CitationRecordDTO {
  id: string;
  sourceCellId: string;
  sourceRevision: number;
  sourceTitle: string;
  sourceAssetHash?: string | null;
  quote: string;
  quoteHash: string;
  insertedAt: number;
}

export interface EditorialPieceDTO {
  id: string;
  workspaceId: string;
  title: string;
  slug: string;
  body: string;
  citations: CitationRecordDTO[];
  wordCount?: number;
  createdAt?: number;
  updatedAt?: number;
}

export interface PieceSummaryDTO {
  id: string;
  workspaceId: string;
  title: string;
  slug: string;
  citationCount: number;
  wordCount: number;
  updatedAt: number;
}

export interface PieceSaveResponseDTO {
  pieceId: string;
  path: string;
  wordCount: number;
  updatedAt: number;
}

export interface CitationDriftItemDTO {
  citationId: string;
  sourceCellId: string;
  status: CitationDriftStatus;
  citedRevision: number;
  currentRevision?: number | null;
  currentTitle?: string | null;
  currentSnippet?: string | null;
}

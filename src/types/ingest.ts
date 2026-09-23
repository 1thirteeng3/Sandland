export type IngestState =
  | 'Pending'
  | 'Extracting'
  | 'Classifying'
  | 'Classified'
  | 'NeedsManualReview'
  | 'Failed';

export type IngestStatus = 'Ingested' | 'Classified' | 'Promoted' | IngestState;

export interface IngestedItemDTO {
  id: string;
  sourceType?: 'file' | 'url' | 'text';
  sourcePath?: string;
  canonicalUri?: string;
  vaultPath?: string;
  title: string;
  itemType?: 'note' | 'web_snapshot';
  summary?: string | null;
  category?: string;
  tags: string[];
  wordCount?: number;
  status?: IngestStatus;
  state?: IngestState;
  needsManualReview?: boolean;
  contentSnippet?: string;
  ingestedAt?: number | string;
  createdAt?: number;
  updatedAt?: number;
}

export interface IngestFilterDTO {
  query?: string;
  sourceType?: string;
  category?: string;
  tag?: string;
  needsManualReviewOnly?: boolean;
  limit?: number;
  offset?: number;
}

export interface IngestStateChangedPayload {
  itemId: string;
  previousState?: IngestState;
  newState: IngestState;
  error?: string;
}

export interface FileWatcherPayload {
  vaultPath: string;
  kind: 'Created' | 'Modified' | 'Removed';
}

export interface PromoteToCellRequest {
  workspaceId: string;
  itemId: string;
  position: {
    x: number;
    y: number;
  };
}

export interface PromoteToCellResponse {
  cellId: string;
  workspaceId: string;
}

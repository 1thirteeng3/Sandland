export type IngestState =
  | 'Pending'
  | 'Extracting'
  | 'Classifying'
  | 'Classified'
  | 'NeedsManualReview'
  | 'Failed';

export interface IngestedItemDTO {
  id: string;
  vaultPath: string;
  title: string;
  itemType: 'note' | 'web_snapshot';
  summary?: string;
  category?: string;
  tags: string[];
  state: IngestState;
  needsManualReview: boolean;
  contentSnippet: string;
  createdAt: number;
  updatedAt: number;
}

export interface IngestFilterDTO {
  query?: string;
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

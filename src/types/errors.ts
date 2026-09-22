export type SandlandErrorCode =
  | 'VaultNotFound'
  | 'VaultAlreadyExists'
  | 'SandboxEscape'
  | 'IngestFailed'
  | 'ClassificationFailed'
  | 'ModelNotLoaded'
  | 'ModelCorrupted'
  | 'DatabaseError'
  | 'SerializationError'
  | 'IoError';

export interface SandlandError {
  code: SandlandErrorCode;
  details?: unknown;
  message?: string;
}

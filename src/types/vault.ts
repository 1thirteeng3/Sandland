export interface CompatibilityFlagsDto {
  allowDowngrade: boolean;
  legacyMigrationApplied: boolean;
}

export interface VaultManifestDto {
  vaultId: string;
  vaultName: string;
  schemaVersion: string;
  createdAt: string;
  updatedAt: string;
  compatibility: CompatibilityFlagsDto;
  rootPath?: string;
}

export interface VaultStatusDto {
  isOpen: boolean;
  vaultId?: string | null;
  vaultName?: string | null;
  rootPath?: string | null;
  totalAssets: number;
  totalEvents: number;
}

export interface AssetRefDto {
  sha256Hash: string;
  byteSize: number;
  extension: string;
  createdAt: string;
  canonicalUri: string;
}

export type SyncState = 'synced' | 'saving' | 'conflict' | 'error' | 'offline';

import { invokeSafe } from "./ipc";
import type {
  VaultManifestDto,
  VaultStatusDto,
  AssetRefDto,
} from "../types/vault";

export interface VaultInfoDTO {
  vaultPath: string;
  name: string;
  totalItems: number;
  totalWorkspaces: number;
  lastOpenedAt: number;
}

export const vaultService = {
  /**
   * Inicializa um novo cofre com manifesto canônico vault.json (Sprint 01)
   */
  async initVault(path: string, name: string): Promise<VaultManifestDto> {
    return invokeSafe<VaultManifestDto>("init_vault", { path, name });
  },

  /**
   * Abre e valida um cofre existente contra o schema 2.0.0 (Sprint 01)
   */
  async openVault(path: string): Promise<VaultManifestDto> {
    return invokeSafe<VaultManifestDto>("open_vault", { path });
  },

  /**
   * Grava documento usando protocolo atômico em duas fases com journal e snapshot (Sprint 01)
   */
  async commitDocument(
    relPath: string,
    content: string,
    expectedRevision?: number
  ): Promise<number> {
    return invokeSafe<number>("commit_document", {
      relPath,
      content,
      expectedRevision: expectedRevision ?? null,
    });
  },

  /**
   * Armazena um anexo/mídia no CAS sob assets/<sha256>.<ext> com deduplicação (Sprint 01)
   */
  async storeAsset(bytesBase64: string, extension: string): Promise<AssetRefDto> {
    return invokeSafe<AssetRefDto>("store_asset", {
      bytesBase64,
      extension,
    });
  },

  /**
   * Lê os bytes base64 de um anexo no CAS pelo seu hash SHA-256 (Sprint 01)
   */
  async readAsset(sha256Hash: string): Promise<string> {
    return invokeSafe<string>("read_asset", { sha256Hash });
  },

  /**
   * Obtém status operacional e integridade do cofre ativo (Sprint 01)
   */
  async getVaultStatus(): Promise<VaultStatusDto> {
    return invokeSafe<VaultStatusDto>("get_vault_status");
  },

  /**
   * Comando de compatibilidade legado (v0.0.1)
   */
  async createVault(vaultPath: string): Promise<VaultInfoDTO> {
    return invokeSafe<VaultInfoDTO>("create_vault", { vaultPath });
  },
};

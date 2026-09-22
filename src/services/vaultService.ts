import { invokeSafe } from "./ipc";

export interface VaultInfoDTO {
  vaultPath: string;
  name: string;
  totalItems: number;
  totalWorkspaces: number;
  lastOpenedAt: number;
}

export const vaultService = {
  async createVault(vaultPath: string): Promise<VaultInfoDTO> {
    return invokeSafe<VaultInfoDTO>("create_vault", { vaultPath });
  },

  async openVault(vaultPath: string): Promise<VaultInfoDTO> {
    return invokeSafe<VaultInfoDTO>("open_vault", { vaultPath });
  }
};

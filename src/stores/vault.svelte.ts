import { vaultService } from "../services/vaultService";
import type {
  VaultManifestDto,
  VaultStatusDto,
  SyncState,
} from "../types/vault";

const STORAGE_KEY_LAST_VAULT = "sandland_last_vault_path";
const STORAGE_KEY_RECENT_VAULTS = "sandland_recent_vaults";

export class VaultStore {
  isOpen = $state<boolean>(false);
  manifest = $state<VaultManifestDto | null>(null);
  status = $state<VaultStatusDto | null>(null);
  syncState = $state<SyncState>("synced");
  lastError = $state<string | null>(null);
  recentVaults = $state<string[]>([]);
  isModalOpen = $state<boolean>(false);

  vaultName = $derived(
    this.manifest?.vaultName || this.status?.vaultName || "Sem Cofre Aberto"
  );
  rootPath = $derived(this.manifest?.rootPath || this.status?.rootPath || "");
  totalAssets = $derived(this.status?.totalAssets ?? 0);
  totalEvents = $derived(this.status?.totalEvents ?? 0);

  constructor() {
    this.loadRecentVaults();
  }

  private loadRecentVaults() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY_RECENT_VAULTS);
      if (raw) {
        this.recentVaults = JSON.parse(raw);
      }
    } catch {
      this.recentVaults = [];
    }
  }

  private addRecentVault(path: string) {
    const next = [path, ...this.recentVaults.filter((p) => p !== path)].slice(
      0,
      5
    );
    this.recentVaults = next;
    try {
      localStorage.setItem(STORAGE_KEY_RECENT_VAULTS, JSON.stringify(next));
      localStorage.setItem(STORAGE_KEY_LAST_VAULT, path);
    } catch {
      // Ignora falhas de localStorage
    }
  }

  async initFromEnvironment() {
    this.lastError = null;
    try {
      const status = await vaultService.getVaultStatus();
      if (status.isOpen) {
        this.status = status;
        this.isOpen = true;
        if (status.rootPath) {
          this.addRecentVault(status.rootPath);
        }
        return;
      }
    } catch {
      // Backend não possui cofre ativo na inicialização
    }

    // Tenta reabrir o último cofre salvo no localStorage
    const lastPath = localStorage.getItem(STORAGE_KEY_LAST_VAULT);
    if (lastPath) {
      try {
        await this.openVault(lastPath);
        return;
      } catch (err) {
        console.warn("[VaultStore] Falha ao reabrir último cofre:", err);
      }
    }

    // Se nenhum cofre foi aberto, abre o modal de seleção
    this.isModalOpen = true;
  }

  async initVault(path: string, name: string): Promise<VaultManifestDto> {
    this.lastError = null;
    this.syncState = "saving";
    try {
      const manifest = await vaultService.initVault(path, name);
      this.manifest = manifest;
      this.isOpen = true;
      this.addRecentVault(path);
      await this.refreshStatus();
      this.syncState = "synced";
      this.isModalOpen = false;
      return manifest;
    } catch (err: any) {
      const msg = err?.message || String(err);
      this.lastError = msg;
      this.syncState = "error";
      throw err;
    }
  }

  async openVault(path: string): Promise<VaultManifestDto> {
    this.lastError = null;
    this.syncState = "saving";
    try {
      const manifest = await vaultService.openVault(path);
      this.manifest = manifest;
      this.isOpen = true;
      this.addRecentVault(path);
      await this.refreshStatus();
      this.syncState = "synced";
      this.isModalOpen = false;
      return manifest;
    } catch (err: any) {
      const msg = err?.message || String(err);
      this.lastError = msg;
      this.syncState = "error";
      throw err;
    }
  }

  async refreshStatus() {
    try {
      const status = await vaultService.getVaultStatus();
      this.status = status;
      this.isOpen = status.isOpen;
    } catch {
      // Silencia erros de polling
    }
  }

  setSyncState(state: SyncState) {
    this.syncState = state;
  }

  toggleModal() {
    this.isModalOpen = !this.isModalOpen;
  }

  closeModal() {
    this.isModalOpen = false;
  }
}

export const vaultStore = new VaultStore();

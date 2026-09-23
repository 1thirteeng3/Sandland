import type { IngestedItemDTO, IngestFilterDTO } from '../types/ingest';
import {
  listIngestedItems,
  ingestFile,
  ingestFileContent,
  ingestUrl,
  promoteToCell,
} from '../services/ingestService';

export class IngestStore {
  items = $state<IngestedItemDTO[]>([]);
  isLoading = $state(false);
  filter = $state<IngestFilterDTO>({ limit: 50, offset: 0 });
  selectedItem = $state<IngestedItemDTO | null>(null);

  totalCount = $derived(this.items.length);
  needsReviewCount = $derived(this.items.filter((i) => i.needsManualReview).length);

  async loadItems() {
    this.isLoading = true;
    try {
      this.items = await listIngestedItems(this.filter);
    } catch (err) {
      console.error('Falha ao carregar itens ingeridos:', err);
    } finally {
      this.isLoading = false;
    }
  }

  async addFile(path: string) {
    this.isLoading = true;
    try {
      const item = await ingestFile(path);
      this.items = [item, ...this.items.filter((i) => i.id !== item.id)];
      return item;
    } catch (err: any) {
      const errMsg = err?.message || (typeof err === "object" ? JSON.stringify(err) : String(err));
      console.error(`[IngestStore] Erro ao ingerir arquivo "${path}":`, errMsg);
      throw new Error(errMsg);
    } finally {
      this.isLoading = false;
    }
  }

  async addFileContent(fileName: string, content: string) {
    this.isLoading = true;
    try {
      const item = await ingestFileContent(fileName, content);
      this.items = [item, ...this.items.filter((i) => i.id !== item.id)];
      return item;
    } catch (err: any) {
      const errMsg = err?.message || (typeof err === "object" ? JSON.stringify(err) : String(err));
      console.error(`[IngestStore] Erro ao ingerir conteúdo de "${fileName}":`, errMsg);
      throw new Error(errMsg);
    } finally {
      this.isLoading = false;
    }
  }

  async addUrl(url: string) {
    this.isLoading = true;
    try {
      const item = await ingestUrl(url);
      this.items = [item, ...this.items.filter((i) => i.id !== item.id)];
      return item;
    } catch (err: any) {
      const errMsg = err?.message || (typeof err === "object" ? JSON.stringify(err) : String(err));
      console.error(`[IngestStore] Erro ao capturar URL "${url}":`, errMsg);
      throw new Error(errMsg);
    } finally {
      this.isLoading = false;
    }
  }

  async promoteToWorkspace(workspaceId: string, itemId: string, position?: { x: number; y: number }) {
    this.isLoading = true;
    try {
      const res = await promoteToCell(workspaceId, itemId, position);
      this.updateItemState(itemId, 'Classified');
      const item = this.items.find((i) => i.id === itemId);
      if (item) {
        item.status = 'Promoted';
      }
      return res;
    } catch (err: any) {
      const errMsg = err?.message || (typeof err === 'object' ? JSON.stringify(err) : String(err));
      console.error(`[IngestStore] Erro ao promover item "${itemId}":`, errMsg);
      throw new Error(errMsg);
    } finally {
      this.isLoading = false;
    }
  }

  updateItemState(itemId: string, newState: IngestedItemDTO['state']) {
    const item = this.items.find((i) => i.id === itemId);
    if (item) {
      item.state = newState;
    }
  }

  setFilter(newFilter: Partial<IngestFilterDTO>) {
    this.filter = { ...this.filter, ...newFilter };
    this.loadItems();
  }
}

export const ingestStore = new IngestStore();

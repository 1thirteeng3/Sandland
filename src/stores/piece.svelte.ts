import type {
  EditorialPieceDTO,
  PieceSummaryDTO,
  CitationRecordDTO,
  CitationDriftItemDTO,
} from '../types/piece';
import {
  listWorkspacePieces,
  readWorkspacePiece,
  saveWorkspacePiece,
  checkPieceCitationsDrift,
  compilePieceExport,
} from '../services/pieceService';

async function computeSha256(text: string): Promise<string> {
  if (typeof crypto !== 'undefined' && crypto.subtle) {
    const encoder = new TextEncoder();
    const data = encoder.encode(text);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');
  }
  // Fallback simples caso crypto.subtle não esteja disponível
  let hash = 0;
  for (let i = 0; i < text.length; i++) {
    hash = (hash << 5) - hash + text.charCodeAt(i);
    hash |= 0;
  }
  return Math.abs(hash).toString(16).padStart(64, '0');
}

export type ViewMode = 'board' | 'piece' | 'split';

export class PieceStore {
  workspaceId = $state<string>('default-workspace');
  viewMode = $state<ViewMode>('board');
  pieces = $state<PieceSummaryDTO[]>([]);
  activePiece = $state<EditorialPieceDTO | null>(null);
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  lastSavedAt = $state<number | null>(null);
  saveError = $state<string | null>(null);

  // Drift Detection
  driftReport = $state<CitationDriftItemDTO[]>([]);
  isCheckingDrift = $state<boolean>(false);
  inspectingCitationId = $state<string | null>(null);

  // Export Modal
  isExportModalOpen = $state<boolean>(false);
  exportCompiledText = $state<string>('');
  isExporting = $state<boolean>(false);

  private autoSaveTimer: any = null;

  activeCitationCount = $derived(this.activePiece?.citations?.length ?? 0);
  activeWordCount = $derived(
    this.activePiece?.body
      ? this.activePiece.body.trim().split(/\s+/).filter(Boolean).length
      : 0
  );
  hasDivergedCitations = $derived(
    this.driftReport.some((d) => d.status === 'Diverged')
  );
  inspectingDriftItem = $derived(
    this.driftReport.find((d) => d.citationId === this.inspectingCitationId) ?? null
  );

  async loadWorkspace(wsId: string) {
    this.workspaceId = wsId;
    this.isLoading = true;
    try {
      const list = await listWorkspacePieces(wsId);
      this.pieces = list;
      if (list.length > 0) {
        if (!this.activePiece || !list.some((p) => p.id === this.activePiece?.id)) {
          await this.selectPiece(list[0].id);
        }
      } else {
        // Criar a primeira peça editorial padrão
        await this.createPiece('Síntese Editorial');
      }
    } catch (e: any) {
      console.error('[PieceStore] Erro ao carregar peças do workspace:', e);
    } finally {
      this.isLoading = false;
    }
  }

  async selectPiece(pieceId: string) {
    if (this.autoSaveTimer) {
      clearTimeout(this.autoSaveTimer);
      this.autoSaveTimer = null;
      await this.saveNow();
    }
    this.isLoading = true;
    try {
      const piece = await readWorkspacePiece(this.workspaceId, pieceId);
      this.activePiece = piece;
      await this.checkDrift();
    } catch (e: any) {
      console.error(`[PieceStore] Falha ao ler peça ${pieceId}:`, e);
    } finally {
      this.isLoading = false;
    }
  }

  async createPiece(title: string = 'Nova Peça'): Promise<EditorialPieceDTO> {
    const id = `piece-${Date.now()}`;
    const slug = title
      .toLowerCase()
      .trim()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-+|-+$/g, '');

    const newPiece: EditorialPieceDTO = {
      id,
      workspaceId: this.workspaceId,
      title,
      slug: slug || 'nova-peca',
      body: `# ${title}\n\nComece a redigir sua síntese ou arraste cartões da Mesa Espacial para citar com proveniência imutável.`,
      citations: [],
      createdAt: Math.floor(Date.now() / 1000),
      updatedAt: Math.floor(Date.now() / 1000),
    };

    await saveWorkspacePiece(this.workspaceId, newPiece);
    await this.refreshList();
    this.activePiece = newPiece;
    this.driftReport = [];
    return newPiece;
  }

  updateTitle(newTitle: string) {
    if (!this.activePiece) return;
    this.activePiece.title = newTitle;
    this.activePiece.slug = newTitle
      .toLowerCase()
      .trim()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-+|-+$/g, '');
    this.scheduleAutoSave();
  }

  updateBody(newBody: string) {
    if (!this.activePiece) return;
    this.activePiece.body = newBody;
    this.scheduleAutoSave();
  }

  scheduleAutoSave() {
    if (this.autoSaveTimer) {
      clearTimeout(this.autoSaveTimer);
    }
    this.autoSaveTimer = setTimeout(() => {
      this.saveNow();
    }, 400); // 400ms debounce (especificado em T009 e requisitos de autosave <= 500ms)
  }

  async saveNow() {
    if (!this.activePiece) return;
    this.isSaving = true;
    this.saveError = null;
    try {
      const resp = await saveWorkspacePiece(this.workspaceId, $state.snapshot(this.activePiece));
      this.lastSavedAt = resp.updatedAt;
      // Atualiza resumo na lista local
      const idx = this.pieces.findIndex((p) => p.id === this.activePiece?.id);
      if (idx >= 0) {
        this.pieces[idx].title = this.activePiece.title;
        this.pieces[idx].slug = this.activePiece.slug;
        this.pieces[idx].citationCount = this.activePiece.citations.length;
        this.pieces[idx].wordCount = resp.wordCount;
        this.pieces[idx].updatedAt = resp.updatedAt;
      }
    } catch (e: any) {
      console.error('[PieceStore] Erro ao salvar peça:', e);
      this.saveError = e.message || String(e);
    } finally {
      this.isSaving = false;
    }
  }

  async refreshList() {
    try {
      this.pieces = await listWorkspacePieces(this.workspaceId);
    } catch (e) {
      console.error('[PieceStore] Falha ao recarregar lista:', e);
    }
  }

  async insertCitationFromNode(node: {
    id: string;
    title?: string;
    content?: string;
    revision?: number;
    assetHash?: string | null;
  }) {
    if (!this.activePiece) return;

    const citationId = `cit-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`;
    const quote = (node.content || '').trim();
    const quoteHash = await computeSha256(quote);
    const sourceTitle = (node.title || 'Cartão da Mesa').trim();
    const sourceRevision = node.revision ?? 1;

    const record: CitationRecordDTO = {
      id: citationId,
      sourceCellId: node.id,
      sourceRevision,
      sourceTitle,
      sourceAssetHash: node.assetHash || null,
      quote,
      quoteHash,
      insertedAt: Math.floor(Date.now() / 1000),
    };

    // Imunidade retroativa: Clonamos a citação no frontmatter YAML da Peça
    this.activePiece.citations = [...this.activePiece.citations, record];

    // Inserir trecho no corpo da Peça com formatação de citação e âncora
    const citationMarkdown = `\n\n> "${quote}"\n> — *${sourceTitle}* [^${citationId}]\n\n`;
    this.activePiece.body = (this.activePiece.body || '') + citationMarkdown;

    // Salvar imediatamente
    await this.saveNow();
    await this.checkDrift();
  }

  async checkDrift() {
    if (!this.activePiece) return;
    this.isCheckingDrift = true;
    try {
      this.driftReport = await checkPieceCitationsDrift(this.workspaceId, this.activePiece.id);
    } catch (e) {
      console.warn('[PieceStore] Falha ao verificar drift das citações:', e);
    } finally {
      this.isCheckingDrift = false;
    }
  }

  openDriftInspector(citationId: string) {
    this.inspectingCitationId = citationId;
  }

  closeDriftInspector() {
    this.inspectingCitationId = null;
  }

  async syncCitation(citationId: string) {
    const item = this.driftReport.find((d) => d.citationId === citationId);
    if (!item || !this.activePiece) return;

    const citIndex = this.activePiece.citations.findIndex((c) => c.id === citationId);
    if (citIndex === -1) return;

    const oldQuote = this.activePiece.citations[citIndex].quote;
    const newQuote = item.currentSnippet ?? oldQuote;
    const newTitle = item.currentTitle ?? this.activePiece.citations[citIndex].sourceTitle;
    const newRevision = item.currentRevision ?? this.activePiece.citations[citIndex].sourceRevision;
    const newHash = await computeSha256(newQuote);

    // Atualiza metadados da citação
    this.activePiece.citations[citIndex].sourceRevision = newRevision;
    this.activePiece.citations[citIndex].sourceTitle = newTitle;
    this.activePiece.citations[citIndex].quote = newQuote;
    this.activePiece.citations[citIndex].quoteHash = newHash;

    // Atualiza citação no corpo se for encontrada a citação antiga exata
    if (oldQuote && this.activePiece.body.includes(oldQuote)) {
      this.activePiece.body = this.activePiece.body.replace(oldQuote, newQuote);
    }

    await this.saveNow();
    await this.checkDrift();
    this.inspectingCitationId = null;
  }

  async openExportModal() {
    if (!this.activePiece) return;
    this.isExporting = true;
    try {
      this.exportCompiledText = await compilePieceExport(
        this.workspaceId,
        this.activePiece.id,
        true
      );
      this.isExportModalOpen = true;
    } catch (e: any) {
      console.error('[PieceStore] Falha ao compilar exportação:', e);
    } finally {
      this.isExporting = false;
    }
  }

  closeExportModal() {
    this.isExportModalOpen = false;
  }

  setViewMode(mode: ViewMode) {
    this.viewMode = mode;
  }
}

export const pieceStore = new PieceStore();

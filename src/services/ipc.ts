import { invoke } from "@tauri-apps/api/core";
import type { SandlandError } from "../types/errors";

export function isTauri(): boolean {
  return (
    typeof window !== "undefined" &&
    typeof (window as any)?.__TAURI_INTERNALS__?.invoke === "function"
  );
}

// Mock em memória e localStorage para permitir desenvolvimento e preview fluido no browser (localhost:1420)
function handleBrowserMock<T>(cmd: string, args?: Record<string, unknown>): T {
  console.info(`[Sandland Web-Mock] Invocando comando '${cmd}':`, args);

  switch (cmd) {
    case "get_vault_status": {
      const activeRaw = localStorage.getItem("sandland_mock_vault");
      if (activeRaw) {
        const active = JSON.parse(activeRaw);
        return {
          isOpen: true,
          vaultId: active.vaultId || "mock-vault-uuid-v4",
          vaultName: active.vaultName || "Meu Cofre Web",
          rootPath: active.rootPath || "C:\\Users\\giovanni.barcelos\\SandlandVault",
          totalAssets: Number(localStorage.getItem("sandland_mock_assets_count") || 1),
          totalEvents: Number(localStorage.getItem("sandland_mock_events_count") || 4),
        } as T;
      }
      return {
        isOpen: false,
        vaultId: null,
        vaultName: null,
        rootPath: null,
        totalAssets: 0,
        totalEvents: 0,
      } as T;
    }

    case "init_vault":
    case "create_vault": {
      const path = (args?.path || args?.vaultPath || "C:\\SandlandVault") as string;
      const name = (args?.name || "Meu Cofre") as string;
      const manifest = {
        vaultId: `vlt-${Date.now()}`,
        vaultName: name,
        schemaVersion: "2.0.0",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        compatibility: { allowDowngrade: false, legacyMigrationApplied: false },
        rootPath: path,
      };
      localStorage.setItem("sandland_mock_vault", JSON.stringify(manifest));
      localStorage.setItem("sandland_mock_events_count", "1");
      return manifest as T;
    }

    case "open_vault": {
      const path = (args?.path || args?.vaultPath || "") as string;
      const folderName = path.split(/[\/\\]/).filter(Boolean).pop() || "Cofre Aberto";
      const manifest = {
        vaultId: `vlt-${Date.now()}`,
        vaultName: folderName,
        schemaVersion: "2.0.0",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        compatibility: { allowDowngrade: false, legacyMigrationApplied: false },
        rootPath: path,
      };
      localStorage.setItem("sandland_mock_vault", JSON.stringify(manifest));
      return manifest as T;
    }

    case "commit_document": {
      const relPath = args?.relPath as string;
      const content = args?.content as string;
      if (relPath && content) {
        localStorage.setItem(`sandland_doc:${relPath}`, content);
      }
      const count = Number(localStorage.getItem("sandland_mock_events_count") || 0) + 1;
      localStorage.setItem("sandland_mock_events_count", String(count));
      return count as T;
    }

    case "store_asset": {
      const bytesB64 = args?.bytesBase64 as string;
      const ext = (args?.extension || "png") as string;
      const mockHash =
        Array.from(crypto.getRandomValues(new Uint8Array(16)))
          .map((b) => b.toString(16).padStart(2, "0"))
          .join("") + "abcd1234";
      if (bytesB64) {
        try {
          localStorage.setItem(`sandland_asset:${mockHash}`, bytesB64);
        } catch {
          // Ignora se estourar cota do localStorage no browser
        }
      }
      const assetCount = Number(localStorage.getItem("sandland_mock_assets_count") || 0) + 1;
      localStorage.setItem("sandland_mock_assets_count", String(assetCount));
      return {
        sha256Hash: mockHash,
        byteSize: bytesB64 ? Math.floor(bytesB64.length * 0.75) : 1024,
        extension: ext,
        createdAt: new Date().toISOString(),
        canonicalUri: `assets/${mockHash}.${ext}`,
      } as T;
    }

    case "read_asset": {
      const hash = args?.sha256Hash as string;
      const cached = localStorage.getItem(`sandland_asset:${hash}`);
      if (cached) return cached as T;
      return "" as T;
    }

    case "load_board_topology": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const cached = localStorage.getItem(`sandland_topology:${wsId}`);
      if (cached) {
        const parsed = JSON.parse(cached);
        if (parsed.revision === undefined) parsed.revision = 1;
        return parsed as T;
      }
      return {
        workspaceId: wsId,
        viewport: { x: 0, y: 0, zoom: 1.0 },
        nodes: [],
        edges: [],
        revision: 1,
        updatedAt: Math.floor(Date.now() / 1000),
      } as T;
    }

    case "save_board_topology": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const topo = (args?.topology || {}) as any;
      const expectedRev = args?.expectedRevision as number | null | undefined;
      const currentRevStr = localStorage.getItem(`sandland_topology_rev:${wsId}`);
      const currentRev = currentRevStr ? Number(currentRevStr) : 1;

      if (expectedRev !== undefined && expectedRev !== null && expectedRev !== currentRev) {
        throw new Error(`RevisionConflict: Conflito de revisão otimista (esperada ${expectedRev}, atual ${currentRev})`);
      }

      const nextRev = currentRev + 1;
      topo.revision = nextRev;
      topo.updatedAt = Math.floor(Date.now() / 1000);

      localStorage.setItem(`sandland_topology:${wsId}`, JSON.stringify(topo));
      localStorage.setItem(`sandland_topology_rev:${wsId}`, String(nextRev));
      return nextRev as T;
    }

    case "save_board_topology_fast": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const topo = args?.topology;
      if (topo) {
        localStorage.setItem(`sandland_topology:${wsId}`, JSON.stringify(topo));
      }
      return undefined as T;
    }

    case "save_workspace_cell": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const cellId = (args?.cellId || `cell-${Date.now()}`) as string;
      const content = (args?.content || "") as string;
      const path = `workspaces/${wsId}/cells/${cellId}.md`;

      const prevRev = Number(localStorage.getItem(`sandland_cell_rev:${cellId}`) || 1);
      const nextRev = prevRev + 1;

      localStorage.setItem(`sandland_cell:${cellId}`, content);
      localStorage.setItem(`sandland_cell_rev:${cellId}`, String(nextRev));

      return {
        cellId,
        path,
        revision: nextRev,
      } as T;
    }

    case "read_workspace_cell": {
      const cellId = (args?.cellId || "") as string;
      const cached = localStorage.getItem(`sandland_cell:${cellId}`);
      if (cached) return cached as T;
      return `# Célula ${cellId}\n\nConteúdo local simulado em memória.` as T;
    }

    case "create_workspace": {
      return (args?.title || "Novo Workspace") as T;
    }

    case "create_cell": {
      return {
        id: `cell-${Date.now()}`,
        workspaceId: args?.workspaceId || "default-workspace",
        content: args?.content || "",
        position: args?.position || { x: 200, y: 200 },
        revision: 1,
      } as T;
    }

    case "list_ingested_items": {
      const itemsRaw = localStorage.getItem("sandland_mock_ingest_items");
      return (itemsRaw ? JSON.parse(itemsRaw) : []) as T;
    }

    case "ingest_file":
    case "ingest_file_content": {
      const name = (args?.fileName || args?.filePath || "documento.md") as string;
      const baseName = name.split(/[\/\\]/).pop() || name;
      const item = {
        id: `ingest-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
        sourceType: "File",
        sourcePath: name,
        canonicalUri: `ingest/notes/${baseName}`,
        title: baseName,
        contentSnippet: "Nota física ingerida com sucesso no cofre (modo navegador).",
        state: "Classified",
        status: "Pending",
        wordCount: 142,
        createdAt: Math.floor(Date.now() / 1000),
        updatedAt: Math.floor(Date.now() / 1000),
        tags: ["nota", "markdown"],
        needsManualReview: false,
      };
      const itemsRaw = localStorage.getItem("sandland_mock_ingest_items");
      const list = itemsRaw ? JSON.parse(itemsRaw) : [];
      list.unshift(item);
      localStorage.setItem("sandland_mock_ingest_items", JSON.stringify(list));
      return item as T;
    }

    case "ingest_url": {
      const url = (args?.url || "https://example.com") as string;
      const host = new URL(url).hostname;
      const item = {
        id: `ingest-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
        sourceType: "Url",
        sourcePath: url,
        canonicalUri: `ingest/web/${host}.md`,
        title: `Snapshot: ${host}`,
        contentSnippet: `Conteúdo limpo capturado de ${url}`,
        state: "Classified",
        status: "Pending",
        wordCount: 380,
        createdAt: Math.floor(Date.now() / 1000),
        updatedAt: Math.floor(Date.now() / 1000),
        tags: ["web-snapshot", "leitura"],
        needsManualReview: false,
      };
      const itemsRaw = localStorage.getItem("sandland_mock_ingest_items");
      const list = itemsRaw ? JSON.parse(itemsRaw) : [];
      list.unshift(item);
      localStorage.setItem("sandland_mock_ingest_items", JSON.stringify(list));
      return item as T;
    }

    case "promote_to_cell": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const itemId = (args?.itemId || `item-${Date.now()}`) as string;
      const cellId = `cell-promoted-${Date.now()}`;
      const nodeId = `node-promoted-${Date.now()}`;
      return {
        cellId,
        workspaceId: wsId,
        nodeId,
        itemId,
        topologyUpdated: true,
      } as T;
    }

    case "list_workspace_pieces": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const raw = localStorage.getItem(`sandland_pieces:${wsId}`);
      if (!raw) return [] as T;
      const pieces = JSON.parse(raw);
      return pieces.map((p: any) => ({
        id: p.id,
        workspaceId: p.workspaceId || wsId,
        title: p.title,
        slug: p.slug,
        citationCount: p.citations ? p.citations.length : 0,
        wordCount: p.body ? p.body.trim().split(/\s+/).filter(Boolean).length : 0,
        updatedAt: p.updatedAt || Math.floor(Date.now() / 1000),
      })) as T;
    }

    case "read_workspace_piece": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const pieceId = args?.pieceId as string;
      const raw = localStorage.getItem(`sandland_pieces:${wsId}`);
      const pieces = raw ? JSON.parse(raw) : [];
      const found = pieces.find((p: any) => p.id === pieceId);
      if (!found) {
        throw new Error(`NotFound: Peça '${pieceId}' não encontrada.`);
      }
      return found as T;
    }

    case "save_workspace_piece": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const piece = (args?.piece || {}) as any;
      const raw = localStorage.getItem(`sandland_pieces:${wsId}`);
      let pieces = raw ? JSON.parse(raw) : [];
      const now = Math.floor(Date.now() / 1000);
      piece.updatedAt = now;
      if (!piece.createdAt) piece.createdAt = now;

      const idx = pieces.findIndex((p: any) => p.id === piece.id);
      if (idx >= 0) {
        pieces[idx] = piece;
      } else {
        pieces.push(piece);
      }
      localStorage.setItem(`sandland_pieces:${wsId}`, JSON.stringify(pieces));
      const wordCount = piece.body ? piece.body.trim().split(/\s+/).filter(Boolean).length : 0;
      return {
        pieceId: piece.id,
        path: `workspaces/${wsId}/pieces/${piece.id}.md`,
        wordCount,
        updatedAt: now,
      } as T;
    }

    case "check_piece_citations_drift": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const pieceId = args?.pieceId as string;
      const raw = localStorage.getItem(`sandland_pieces:${wsId}`);
      const pieces = raw ? JSON.parse(raw) : [];
      const found = pieces.find((p: any) => p.id === pieceId);
      if (!found || !found.citations) return [] as T;

      // Mock drift status: if cited revision matches current cell revision, Synchronized
      const topoRaw = localStorage.getItem(`sandland_topology:${wsId}`);
      const topo = topoRaw ? JSON.parse(topoRaw) : { nodes: [] };
      return found.citations.map((c: any) => {
        const node = topo.nodes.find((n: any) => n.id === c.sourceCellId || n.cellId === c.sourceCellId);
        if (!node) {
          return {
            citationId: c.id,
            sourceCellId: c.sourceCellId,
            status: "Orphaned",
            citedRevision: c.sourceRevision,
          };
        }
        const currentRev = node.revision ?? 1;
        const status = currentRev > c.sourceRevision ? "Diverged" : "Synchronized";
        return {
          citationId: c.id,
          sourceCellId: c.sourceCellId,
          status,
          citedRevision: c.sourceRevision,
          currentRevision: currentRev,
          currentTitle: node.title,
          currentSnippet: node.content,
        };
      }) as T;
    }

    case "compile_piece_export": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const pieceId = args?.pieceId as string;
      const includeRef = args?.includeReferences !== false;
      const raw = localStorage.getItem(`sandland_pieces:${wsId}`);
      const pieces = raw ? JSON.parse(raw) : [];
      const found = pieces.find((p: any) => p.id === pieceId);
      if (!found) throw new Error("NotFound: Peça não encontrada");

      let out = `# ${found.title}\n\n${found.body}`;
      if (includeRef && found.citations && found.citations.length > 0) {
        out += "\n\n---\n\n## Referências & Proveniência\n\n";
        found.citations.forEach((c: any, i: number) => {
          out += `${i + 1}. **${c.sourceTitle || c.sourceCellId}** (rev. ${c.sourceRevision}): "${c.quote}"\n`;
        });
      }
      return out as T;
    }

    default:
      return undefined as T;
  }
}

export function extractErrorMessage(error: unknown): string {
  if (typeof error === "string") return error;
  if (typeof error === "object" && error !== null) {
    const err = error as Record<string, unknown>;
    if (typeof err.message === "string" && err.message) return err.message;
    if (err.details) {
      if (typeof err.details === "string")
        return `${err.code ? String(err.code) + ": " : ""}${err.details}`;
      if (typeof err.details === "object" && err.details !== null) {
        const details = err.details as Record<string, unknown>;
        if (typeof details.reason === "string")
          return `${String(err.code || "Erro")}: ${details.reason}`;
        return JSON.stringify(err.details);
      }
    }
    if (err.code) return `Erro [${String(err.code)}]`;
  }
  return String(error);
}

export async function invokeSafe<T>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  // Se estiver rodando em browser puro (ex: http://localhost:1420), ativa fallback simulado
  if (!isTauri()) {
    return handleBrowserMock<T>(cmd, args);
  }

  try {
    return await invoke<T>(cmd, args);
  } catch (error: any) {
    // Caso a chamada direta ao invoke falhe por ausência do bridge Tauri
    if (
      (error instanceof TypeError || typeof error === "object") &&
      String(error?.message || error).includes("invoke")
    ) {
      console.warn(`[Sandland IPC] Bridge nativa não encontrada para '${cmd}'. Executando mock de browser.`);
      return handleBrowserMock<T>(cmd, args);
    }

    const msg = extractErrorMessage(error);
    if (typeof error === "object" && error !== null && "code" in error) {
      const sandErr = error as SandlandError;
      sandErr.message = sandErr.message || msg;
      throw sandErr;
    }
    throw {
      code: "IoError",
      message: msg,
      details: error,
    } as SandlandError;
  }
}

export const invokeCommand = invokeSafe;

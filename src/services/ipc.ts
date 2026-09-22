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
        return JSON.parse(cached) as T;
      }
      return {
        workspaceId: wsId,
        viewport: { x: 0, y: 0, zoom: 1.0 },
        nodes: [],
        edges: [],
        updatedAt: Math.floor(Date.now() / 1000),
      } as T;
    }

    case "save_board_topology":
    case "save_board_topology_fast": {
      const wsId = (args?.workspaceId || "default-workspace") as string;
      const topo = args?.topology;
      if (topo) {
        localStorage.setItem(`sandland_topology:${wsId}`, JSON.stringify(topo));
      }
      return undefined as T;
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
      const item = {
        id: `ingest-${Date.now()}`,
        sourceType: "file",
        sourcePath: name,
        title: name.split(/[\/\\]/).pop() || name,
        summary: "Nota importada em modo Web Browser.",
        ingestedAt: new Date().toISOString(),
        tags: ["web-import"],
      };
      const itemsRaw = localStorage.getItem("sandland_mock_ingest_items");
      const list = itemsRaw ? JSON.parse(itemsRaw) : [];
      list.unshift(item);
      localStorage.setItem("sandland_mock_ingest_items", JSON.stringify(list));
      return item as T;
    }

    case "ingest_url": {
      const url = (args?.url || "https://example.com") as string;
      const item = {
        id: `ingest-${Date.now()}`,
        sourceType: "url",
        sourcePath: url,
        title: `Snapshot: ${url}`,
        summary: "Captura de URL em modo Web Browser.",
        ingestedAt: new Date().toISOString(),
        tags: ["web-snapshot"],
      };
      const itemsRaw = localStorage.getItem("sandland_mock_ingest_items");
      const list = itemsRaw ? JSON.parse(itemsRaw) : [];
      list.unshift(item);
      localStorage.setItem("sandland_mock_ingest_items", JSON.stringify(list));
      return item as T;
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

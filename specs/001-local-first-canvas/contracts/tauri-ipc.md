# IPC Contracts & Tauri Commands: Núcleo Local-First e Canvas Básico (v0.1)

**Feature**: `001-local-first-canvas`  
**Date**: 2026-09-21 (Revisão Completa de Contratos SDD)  
**Status**: Ready & Exhaustive  

Este documento define todos os comandos invocados pelo Frontend (`@tauri-apps/api/core`) e os eventos assíncronos emitidos pelo Backend em Rust (`tauri::Emitter`).

---

## 1. Taxonomia Canônica de Erros (`SandlandError`)

Todos os comandos IPC retornam `Result<T, SandlandError>`. No TypeScript, erros são capturados via `catch (err: SandlandError)`.

### 1.1. Definição em Rust (`src-tauri/src/domain/core/errors.rs`)

```rust
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(tag = "code", content = "details")]
pub enum SandlandError {
    #[error("Cofre não encontrado no caminho: {0}")]
    VaultNotFound(PathBuf),

    #[error("Já existe um cofre inicializado no caminho: {0}")]
    VaultAlreadyExists(PathBuf),

    #[error("Tentativa de fuga do Sandbox detectada: {attempted} fora de {boundary}")]
    SandboxEscape { attempted: PathBuf, boundary: PathBuf },

    #[error("Falha ao ingerir o documento '{item_id}': {reason}")]
    IngestFailed { item_id: String, reason: String },

    #[error("Falha na classificação taxonômica após {attempts} tentativas")]
    ClassificationFailed { item_id: String, attempts: u8 },

    #[error("Modelo local '{name}' ausente ou não inicializado")]
    ModelNotLoaded { name: String },

    #[error("Integridade corrompida no modelo '{name}'. Hash esperado: {expected_sha}")]
    ModelCorrupted { name: String, expected_sha: String },

    #[error("Erro de banco de dados SQLite: {0}")]
    DatabaseError(String),

    #[error("Erro de serialização ou parsing: {0}")]
    SerializationError(String),

    #[error("Erro de I/O no disco local: {0}")]
    IoError(String),
}
```

### 1.2. Mapeamento TypeScript (`src/types/errors.ts`)

```typescript
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
  details: unknown;
  message: string;
}
```

---

## 2. Comandos IPC Exaustivos (Tauri Commands)

### 2.1. Gestão de Cofres (*Vault Management*)

#### `create_vault`
Inicializa uma nova estrutura de cofre em disco (`/vault/.system/`, `/vault/ingest/notes/`, `/vault/workspaces/`).
```rust
#[tauri::command]
pub async fn create_vault(
    app_handle: tauri::AppHandle,
    vault_path: String
) -> Result<VaultInfoDTO, SandlandError>;
```

#### `open_vault`
Abre e valida a integridade de um cofre existente, executando migrações pendentes no `index.db`.
```rust
#[tauri::command]
pub async fn open_vault(
    app_handle: tauri::AppHandle,
    vault_path: String
) -> Result<VaultInfoDTO, SandlandError>;
```

*DTOs TypeScript:*
```typescript
export interface VaultInfoDTO {
  vaultPath: string;
  name: string;
  totalItems: number;
  totalWorkspaces: number;
  lastOpenedAt: number;
}
```

---

### 2.2. Ingestão e Processamento de Documentos (*Ingest Domain*)

#### `ingest_file`
Recebe o caminho de um arquivo local e o absorve de forma atômica no diretório `/vault/ingest/notes/`.
```rust
#[tauri::command]
pub async fn ingest_file(
    app_handle: tauri::AppHandle,
    file_path: String
) -> Result<IngestedItemDTO, SandlandError>;
```

#### `ingest_url` (v0.1 simplificado)
Captura o conteúdo de uma URL via cliente HTTP Rust nativo e salva snapshot Markdown em `/vault/ingest/web/`.
```rust
#[tauri::command]
pub async fn ingest_url(
    app_handle: tauri::AppHandle,
    url: String
) -> Result<IngestedItemDTO, SandlandError>;
```

#### `list_ingested_items`
Retorna a lista paginada de itens recebidos no cofre.
```rust
#[tauri::command]
pub async fn list_ingested_items(
    filter: IngestFilterDTO
) -> Result<Vec<IngestedItemDTO>, SandlandError>;
```

*DTOs TypeScript:*
```typescript
export type IngestState = 'Pending' | 'Extracting' | 'Classifying' | 'Classified' | 'NeedsManualReview' | 'Failed';

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
  limit: number;
  offset: number;
}
```

---

### 2.3. Taxonomia e Inferência SLM (*Taxonomy Domain*)

#### `trigger_classification`
Dispara manualmente ou reexecuta o auto-tagging local do SLM com gramática GBNF.
```rust
#[tauri::command]
pub async fn trigger_classification(
    app_handle: tauri::AppHandle,
    item_id: String
) -> Result<ClassificationResultDTO, SandlandError>;
```

#### `update_item_tags`
Atualiza manualmente a categoria e as tags de um documento após revisão do usuário.
```rust
#[tauri::command]
pub async fn update_item_tags(
    item_id: String,
    category: String,
    tags: Vec<String>
) -> Result<(), SandlandError>;
```

*DTOs TypeScript:*
```typescript
export interface ClassificationResultDTO {
  itemId: string;
  category: string;
  tags: string[];
  summary: string;
  confidence: number;
  fallbackUsed: boolean;
}
```

---

### 2.4. Workspaces e Topologia de Canvas (*Workspace Domain*)

#### `create_workspace`
Cria uma nova área de trabalho espacial (pasta `/vault/workspaces/{workspace_id}/`).
```rust
#[tauri::command]
pub async fn create_workspace(
    title: String
) -> Result<WorkspaceDTO, SandlandError>;
```

#### `load_board_topology`
Carrega os nós e conexões ativas do canvas.
```rust
#[tauri::command]
pub async fn load_board_topology(
    workspace_id: String
) -> Result<CanvasTopologyDTO, SandlandError>;
```

#### `save_board_topology_fast`
Persistência binária de alta frequência (MessagePack serializado) disparada a cada 350ms em movimentação de nós.
```rust
#[tauri::command]
pub async fn save_board_topology_fast(
    workspace_id: String,
    topology_mpk: Vec<u8>
) -> Result<(), SandlandError>;
```

#### `create_cell`
Cria uma nova célula de texto livre diretamente na mesa de trabalho.
```rust
#[tauri::command]
pub async fn create_cell(
    workspace_id: String,
    content: String,
    position: PositionDTO
) -> Result<CellDTO, SandlandError>;
```

#### `list_cells`
Lista todas as células atômicas ativas vinculadas ao workspace.
```rust
#[tauri::command]
pub async fn list_cells(
    workspace_id: String
) -> Result<Vec<CellDTO>, SandlandError>;
```

*DTOs TypeScript:*
```typescript
export interface WorkspaceDTO {
  id: string;
  title: string;
  createdAt: number;
}

export interface PositionDTO {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface CellDTO {
  id: string;
  workspaceId: string;
  title: string;
  content: string;
  position: PositionDTO;
  tags: string[];
  createdAt: number;
  updatedAt: number;
}

export interface CanvasTopologyDTO {
  workspaceId: string;
  viewport: {
    x: number;
    y: number;
    zoom: number;
  };
  nodes: CanvasNodeDTO[];
  edges: CanvasEdgeDTO[];
  updatedAt: number;
}

export type NodeSide = 'left' | 'right' | 'top' | 'bottom';

export interface CanvasNodeDTO {
  id: string;               // Identificador espacial da instância
  itemId?: string;          // Ponteiro relacional ao documento no index.db
  localCellPath?: string;   // Caminho da célula autoral do workspace
  x: number;
  y: number;
  width: number;
  height: number;
  colorPreset?: string;
}

export interface CanvasEdgeDTO {
  id: string;
  sourceNodeId: string;
  targetNodeId: string;
  fromSide: NodeSide;       // Âncora de saída (docking anchor)
  toSide: NodeSide;         // Âncora de entrada (docking anchor)
  label?: string;
  directed: boolean;
}
```

---

## 3. Eventos Assíncronos (Tauri Events via `app_handle.emit`)

### `ingest://state-changed`
Emitido a cada transição de estado no ciclo de vida do documento ingerido:
```typescript
export interface IngestStateChangedPayload {
  itemId: string;
  previousState: IngestState;
  newState: IngestState;
  error?: string;
}
```

### `model://download-progress`
Emitido pelo gerenciador de modelos durante o download inicial dos pesos GGUF/ONNX:
```typescript
export interface ModelDownloadProgressPayload {
  modelName: string;
  bytesDownloaded: number;
  totalBytes: number;
  percentage: number;
}
```

### `vault://file-watcher-event`
Emitido quando um arquivo externo é detectado pelo `notify`:
```typescript
export interface FileWatcherPayload {
  vaultPath: string;
  kind: 'Created' | 'Modified' | 'Removed';
}
```

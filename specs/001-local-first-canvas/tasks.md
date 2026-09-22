# Implementation Tasks: Núcleo Local-First e Canvas Básico (v0.1)

**Feature**: `001-local-first-canvas`  
**Date**: 2026-09-21 (Revisão Arquitetural: Compilação Estática, Hidratação File-as-Truth, User Opt-in e Granularidade)  
**Status**: Ready for Execution  

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Inicialização do workspace híbrido Tauri v2 + Svelte 5 e dependências de compilação em Rust nativo.

> **Nota Arquitetural**: A lógica da IA (bindings do `llama.cpp` e `fastembed`) DEVE ser embutida/compilada estaticamente no binário do Rust. No entanto, o download dos arquivos pesados de modelo (`.gguf`, `.onnx`) deve ser "Lazy" / User Opt-in (baixados apenas quando a funcionalidade for ativada pelo usuário na interface).

- [X] T001 Initialize Tauri v2 desktop project with Svelte 5 (Runes), TypeScript, and Vite in `package.json`, `vite.config.ts`, and `index.html`
- [X] T002 Initialize Rust backend workspace with `src-tauri/Cargo.toml` including dependencies (`tauri` v2, `tokio`, `rusqlite`, `refinery`, `llama-cpp-2`, `fastembed`, `notify-debouncer-full`, `rmp-serde`, `thiserror`, `serde`)
- [X] T003.1 [P] Baixar os arquivos de código-fonte de amalgamation (`sqlite-vec.c` e `sqlite-vec.h`) e armazená-los na pasta `src-tauri/vendor/sqlite-vec/`
- [X] T003.2 [P] Configurar `src-tauri/build.rs` usando o crate `cc` para compilar estaticamente `vendor/sqlite-vec/sqlite-vec.c` e vinculá-lo estaticamente ao `rusqlite`
- [X] T004 [P] Configure Tauri v2 application window, security permissions, and IPC endpoints in `src-tauri/tauri.conf.json`
- [X] T005 [P] Define canonical error enum `SandlandError` and Result types in `src-tauri/src/domain/core/errors.rs` and mirror in `src/types/errors.ts`
- [X] T006 [P] Define core domain events (`ItemDetected`, `IngestStateChanged`, `ModelDownloadProgress`) in `src-tauri/src/domain/core/events.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Infraestrutura de persistência, banco de dados SQLite com migrações versionadas, segurança de I/O contra TOCTOU e rotina de hidratação do cofre.

> **CRITICAL**: Nenhuma tarefa de história de usuário pode iniciar antes da conclusão desta fase.

- [X] T007 Create initial SQLite database migration `V1__initial_schema.sql` with `items` (`rowid_item INTEGER PRIMARY KEY AUTOINCREMENT`, `id TEXT UNIQUE`, `vault_path TEXT UNIQUE`, `title TEXT`, `content_hash TEXT`, `category TEXT`, `summary TEXT`, `state TEXT`, `needs_manual_review BOOLEAN`, `read_only BOOLEAN`), `items_fts` (FTS5 with unicode61), `vec_items` (`rowid INTEGER PRIMARY KEY`, `embedding FLOAT[384]`), `taxonomy_terms`, and `item_taxonomy` in `src-tauri/migrations/V1__initial_schema.sql`
- [X] T008 [P] Create audit events migration `V2__audit_events.sql` with `audit_events` table and timestamp index in `src-tauri/migrations/V2__audit_events.sql`
- [X] T009 Implement database connection manager, WAL mode PRAGMAs, and `refinery` embedded migration runner in `src-tauri/src/infra/db/mod.rs`
- [X] T010 [P] Implement secure filesystem Path Guard with `canonicalize()` and strict boundary validation in `src-tauri/src/infra/fs/vault.rs` to prevent symlink traversal and TOCTOU
- [X] T010.5 Implement initial vault hydration routine: scan `/vault/ingest/` on startup and synchronize missing or modified files into the SQLite database, respecting the File-as-Truth architecture in `src-tauri/src/infra/fs/vault.rs`
- [X] T011 [P] Implement high-frequency audit logger with debounced `NODE_DRAG_END` in `src-tauri/src/infra/db/audit.rs`
- [X] T012 Implement vault lifecycle IPC commands (`create_vault` and `open_vault`) in `src-tauri/src/ipc/vault.rs`
- [X] T013 [P] Create frontend IPC client wrapper with strict typing and `SandlandError` handling in `src/services/ipc.ts` and `src/services/vaultService.ts`
- [X] T014 Setup Svelte 5 application shell and layout in `src/App.svelte` and design system CSS tokens in `src/styles/tokens.css`

**Checkpoint**: Fundação pronta — banco de dados, I/O seguro, hidratação e IPC inicializados. As histórias de usuário podem ser executadas.

---

## Phase 3: User Story 1 - Ingestão Transparente de Documentos (Priority: P1) 🎯 MVP

**Goal**: Permitir ingestão imediata e passiva de notas e artigos Markdown sem fricção prévia, salvando no disco local com YAML Frontmatter padronizado (sem metadados extrínsecos) e registrando no `index.db`.

**Independent Test**: Salvar ou soltar um arquivo `.md` na pasta do cofre; o sistema deve absorver o arquivo, calcular o hash SHA-256 no banco, gerar Frontmatter limpo e listá-lo na interface em menos de 1 segundo.

### Implementation for User Story 1

- [X] T015 [P] [US1] Create Ingest domain entity `IngestedItem` and `IngestState` enum (`Pending`, `Extracting`, `Classifying`, `Classified`, `NeedsManualReview`, `Failed`) in `src-tauri/src/domain/ingest/entity.rs`
- [X] T016 [P] [US1] Implement Ingest State Machine transition logic with timeout enforcement (15s) in `src-tauri/src/domain/ingest/state_machine.rs`
- [X] T017 [US1] Implement atomic file writer and reader for Markdown with YAML Frontmatter parser in `src-tauri/src/infra/fs/vault.rs`
- [X] T018 [P] [US1] Implement reactive file watcher with 250ms debouncer using `notify-debouncer-full` on `/vault/ingest/` in `src-tauri/src/infra/fs/watcher.rs`
- [X] T019 [US1] Implement `items` repository and FTS5 search indexer in `src-tauri/src/infra/db/indexer.rs`
- [X] T020 [US1] Implement IPC commands `ingest_file`, `ingest_url`, and `list_ingested_items` in `src-tauri/src/ipc/ingest.rs`
- [X] T021 [P] [US1] Create frontend TypeScript types and Ingest reactive store using Svelte 5 Runes in `src/types/ingest.ts` and `src/stores/ingest.svelte.ts`
- [X] T022 [P] [US1] Implement `IngestDrawer.svelte` and drag-and-drop `Dropzone.svelte` in `src/components/ingest/Dropzone.svelte` and `src/components/ingest/IngestDrawer.svelte`
- [X] T023 [US1] Implement `IngestItemList.svelte` displaying captured documents with state badges and review indicators in `src/components/ingest/IngestItemList.svelte`
- [X] T024 [US1] Integration test verifying passive ingestion, watcher detection, and SQLite state persistence in `tests/integration/test_ingest.rs`

**Checkpoint**: Neste ponto, o MVP do Sandland (User Story 1) está 100% funcional e testável de forma autônoma.

---

## Phase 4: User Story 2 - Auto-Tagging Inteligente e Classificação Local (Priority: P1)

**Goal**: Classificar documentos ingeridos usando SLM local (`llama.cpp` com GBNF) e embeddings multilíngues (`fastembed`), consolidando categorias e tags na tabela N:N `item_taxonomy` com ativação sob demanda (User Opt-in).

**Independent Test**: Ingerir documentos sobre temas correlatos; verificar que embeddings de 384 dimensões são gerados em < 15ms, tags existentes são reutilizadas, e falhas acionam a cascata de contingência com marcação de revisão manual.

### Implementation for User Story 2

- [X] T025 [P] [US2] Create Taxonomy domain entity `TaxonTerm` and normalizer (kebab-case, singularization, Levenshtein distance <= 2) in `src-tauri/src/domain/taxonomy/normalizer.rs`
- [X] T026 [P] [US2] Implement multilingual embeddings generator using `fastembed` with `paraphrase-multilingual-MiniLM-L12-v2` in `src-tauri/src/infra/ai/embeddings.rs`
- [X] T027 [P] [US2] Implement GBNF grammar parser and strict GGML JSON schema in `src-tauri/src/infra/ai/gbnf.rs`
- [X] T028.1 [US2] Implement resilient model downloader with SHA-256 verification and progress events in `src-tauri/src/infra/ai/model_manager.rs` (User Opt-in executado na interface antes da inicialização do LLM)
- [X] T028.2 [US2] Configure `llama-cpp-2` FFI bindings and implement basic Qwen2.5-1.5B GGUF loading and unloading logic in memory in `src-tauri/src/infra/ai/llm.rs`
- [X] T028.3 [US2] Implement prompt formatting, inference execution, and integration with the GBNF grammar parser in `src-tauri/src/infra/ai/llm.rs`
- [X] T029 [P] [US2] Implement 3-tier classification fallback cascade (thermal retry temp 0.0 -> lexical TF-IDF + cosine similarity -> `needs_manual_review: true`) in `src-tauri/src/infra/ai/fallback.rs`
- [X] T030 [US2] Implement atomic database transaction for classification: persist into `items`, insert into `item_taxonomy`, and insert embedding into `vec_items` (matching `rowid_item`) in `src-tauri/src/infra/db/indexer.rs`
- [X] T031 [US2] Implement IPC commands `trigger_classification` and `update_item_tags` in `src-tauri/src/ipc/taxonomy.rs`
- [X] T033 [US2] Integration test for classification pipeline, GBNF schema compliance, and fallback cascade in `tests/integration/test_taxonomy.rs`

**Checkpoint**: User Stories 1 e 2 funcionam juntas e de forma independente. O acervo é ingerido e classificado semanticamente no cofre.

---

## Phase 5: User Story 3 - Mesa de Trabalho Espacial / Canvas Infinito (Priority: P2)

**Goal**: Fornecer um whiteboard infinito acelerado por WebGL (PixiJS v8) com persistência dupla (MessagePack a 350ms e JSON em repouso), conexões com âncoras e edição de texto rica via overlay DOM em Svelte 5.

**Independent Test**: Abrir o canvas, instanciar nós, conectá-los por arestas direcionadas, editar texto com duplo clique e verificar recuperação exata após encerramento forçado do processo.

### Implementation for User Story 3

- [X] T034 [P] [US3] Create Workspace domain entities `BoardTopology`, `CanvasNode`, and `CanvasEdge` with `NodeSide` docking anchors in `src-tauri/src/domain/workspace/topology.rs`
- [X] T035 [P] [US3] Implement high-frequency MessagePack serialization and async idle JSON writer in `src-tauri/src/infra/fs/canvas_io.rs`
- [X] T036 [US3] Implement IPC commands `create_workspace`, `load_board_topology`, `save_board_topology_fast`, `create_cell`, and `list_cells` in `src-tauri/src/ipc/workspace.rs`
- [X] T037 [P] [US3] Create Canvas reactive store with Svelte 5 Runes for viewport camera, node selection, and hydration from SQLite cache in `src/stores/canvas.svelte.ts`
- [X] T038.1 [US3] Initialize PixiJS v8 Application and Viewport container with basic pan and zoom capabilities in `src/components/canvas/PixiRenderer.ts`
- [X] T038.2 [US3] Implement CanvasNode rendering (graphics and text primitives) and coordinate synchronization with Svelte stores in `src/components/canvas/PixiRenderer.ts`
- [X] T038.3 [US3] Implement interactive drag events for nodes and mathematical routing for docking bezier edges (CanvasEdge) in `src/components/canvas/PixiRenderer.ts`
- [X] T039 [US3] Implement projected DOM overlay micro-editor (`CellOverlay.svelte`) positioned via viewport camera transformation matrix for cell text editing in `src/components/canvas/CellOverlay.svelte`
- [X] T040 [P] [US3] Implement canvas viewport container and camera toolbar controls in `src/components/canvas/CanvasViewport.svelte` and `src/components/canvas/CanvasControls.svelte`
- [X] T041 [US3] Integration test for canvas topology persistence, MessagePack / JSON parity, and node hydration in `tests/integration/test_canvas.rs`

**Checkpoint**: User Stories 1, 2 e 3 estão integradas. O usuário explora o conhecimento espacialmente e conecta ideias.

---

## Phase 6: User Story 4 - Navegação Fluida com Múltiplos Níveis de Detalhe (Priority: P3)

**Goal**: Sustentar 60 FPS estáveis no canvas mesmo com mais de 100 cartões através de Frustum Culling e adaptação visual em 3 níveis de LOD (detalhado, intermediário e proxy simplificado).

**Independent Test**: Posicionar 100 nós no canvas e alternar o zoom entre valores macro (<0.3) e detalhados (>0.6); a renderização deve transicionar de cartões sólidos para texto sem quedas perceptíveis de taxa de quadros.

### Implementation for User Story 4

- [X] T042 [US4] Implement Viewport Frustum Culling in `PixiRenderer.ts` to cull off-screen nodes from WebGL draw calls in `src/components/canvas/PixiRenderer.ts`
- [X] T043 [US4] Implement 3-stage Level-of-Detail (LOD) rendering logic (Zoom > 0.6: text/tags; 0.3 <= Z <= 0.6: title & outline; Z < 0.3: solid proxy card) in `src/components/canvas/PixiRenderer.ts`
- [X] T044 [P] [US4] Add camera animation smoothing, minimap, and zoom preset shortcuts in `src/components/canvas/CanvasControls.svelte`
- [X] T045 [US4] Performance benchmark test verifying 60 FPS rendering with 100+ nodes in `tests/integration/test_lod_performance.rs`

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Unificação de consultas, otimização de memória do SLM e validação de segurança.

- [X] T046 [P] Implement unified Reciprocal Rank Fusion (RRF k=60) query joining FTS5 and `vec_items` on `rowid_item` in `src-tauri/src/infra/db/indexer.rs`
- [X] T047 [P] Implement automated memory garbage collector thread enforcing SLM unload after 5 minutes of inactivity in `src-tauri/src/infra/ai/llm.rs`
- [X] T048 Security hardening: verify all file I/O passes through Path Guard (`canonicalize()` check) with dedicated tests in `tests/unit/test_security_path_guard.rs`
- [X] T049 Execute end-to-end validation scenarios described in `specs/001-local-first-canvas/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

```
[Phase 1: Setup] ──► [Phase 2: Foundational] ──► [Phase 3: US1 (MVP)] ──► [Phase 4: US2]
                                                        │                         │
                                                        ▼                         ▼
                                                [Phase 5: US3] ─────────► [Phase 6: US4]
                                                        │                         │
                                                        └────────────┬────────────┘
                                                                     ▼
                                                         [Phase 7: Polish]
```

- **Phase 1 (Setup)**: Pode iniciar imediatamente.
- **Phase 2 (Foundational)**: Depende de Setup — **BLOQUEIA** todas as histórias de usuário.
- **Phase 3 (User Story 1 - MVP)**: Pode iniciar assim que Phase 2 terminar.
- **Phase 4 (User Story 2)**: Depende de Phase 2 e integra com as entidades de ingestão da Phase 3.
- **Phase 5 (User Story 3)**: Depende de Phase 2 e consome itens da Phase 3 no canvas.
- **Phase 6 (User Story 4)**: Depende do motor PixiJS implementado na Phase 5.
- **Phase 7 (Polish)**: Executada após a conclusão das histórias desejadas.

---

## Parallel Execution Examples

### Parallel Opportunities in Phase 1 & 2
```bash
# Execução paralela em Phase 1:
T003.1: "Download sqlite-vec amalgamation source files"
T003.2: "Configurar build.rs para sqlite-vec em C99"
T004:   "tauri.conf.json window and IPC config"
T005:   "SandlandError enum em Rust e TypeScript"
T006:   "Domain events em events.rs"

# Execução paralela em Phase 2:
T008:   "V2__audit_events.sql migration"
T010:   "Path Guard em vault.rs"
T010.5: "Vault hydration routine em vault.rs"
T011:   "Audit logger com debouncing em audit.rs"
T013:   "IPC client wrapper em ipc.ts"
```

### Parallel Opportunities in User Story 1 (MVP)
```bash
# Execução paralela de entidades e infraestrutura US1:
T015: "IngestedItem entity e IngestState enum"
T016: "Ingest State Machine transition logic"
T018: "File watcher debouncer em watcher.rs"
T021: "Frontend TypeScript types e store Svelte 5"
T022: "IngestDrawer.svelte e Dropzone.svelte"
```

---

## Implementation Strategy

1. **MVP Primeiro (User Story 1 Apenas)**:
   - Concluir Fase 1 (Setup) e Fase 2 (Foundational com Hidratação T010.5).
   - Concluir Fase 3 (User Story 1).
   - Validar ingestão passiva, detecção via watcher e gravação no `index.db`. Neste ponto, o sistema já entrega valor real de captura de conhecimento.
2. **Entrega Incremental**:
   - Adicionar Fase 4 (Auto-tagging local + `fastembed`): o acervo ganha inteligência semântica com download Lazy/User Opt-in de modelos.
   - Adicionar Fase 5 (Canvas + Svelte DOM overlay granular T038.1 a T038.3): o usuário ganha exploração espacial e edição.
   - Adicionar Fase 6 (LOD + Culling): a performance em larga escala é garantida.
   - Executar Fase 7 (Polish): refinamento final e checagem de qualidade.

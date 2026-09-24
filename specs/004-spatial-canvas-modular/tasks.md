# Tasks: Mesa Espacial e Edição Modular (Canvas & BlockSuite)

**Feature**: `004-spatial-canvas-modular`  
**Input**: Design artifacts from `specs/004-spatial-canvas-modular/` (`plan.md`, `spec.md`, `data-model.md`, `contracts/canvas-ipc.json`, `research.md`, `quickstart.md`)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Estruturação inicial de tipos, diretórios canônicos e contratos para a Mesa Espacial

- [ ] T001 [P] Ensure canonical workspace directories tree (`workspaces/<id>/`, `workspaces/<id>/cells/`) in `src-tauri/src/infra/fs/vault.rs`
- [ ] T002 [P] Define TypeScript DTOs matching `contracts/canvas-ipc.json` (`CanvasNodeDTO`, `CanvasEdgeDTO`, `BoardTopologyDTO`, `NodeSide`) in `src/types/canvas.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Modelos de domínio em Rust, validações geométricas e infraestrutura de persistência que bloqueiam as estórias de usuário

**⚠️ CRITICAL**: Nenhuma estória de usuário pode ser iniciada antes da conclusão desta fase.

- [ ] T003 [P] Implement domain entities `BoardTopology`, `CanvasNode`, `CanvasEdge`, and `NodeSide` enum in `src-tauri/src/domain/workspace/topology.rs`
- [ ] T004 [P] Implement `WorkspaceCell` entity with YAML frontmatter serialization in `src-tauri/src/domain/workspace/cell.rs`
- [ ] T005 Implement atomic two-phase write and read for `board.canvas.json` and `cells/*.md` with OCC revision check in `src-tauri/src/infra/fs/canvas_io.rs`
- [ ] T006 Expose Tauri IPC commands `load_board_topology`, `save_board_topology`, `save_workspace_cell`, and `read_workspace_cell` in `src-tauri/src/ipc/workspace.rs` and register in `src-tauri/src/lib.rs`
- [ ] T007 [P] Implement typed client IPC methods (`loadBoardTopology`, `saveBoardTopology`, `saveWorkspaceCell`, `readWorkspaceCell`) in `src/services/canvasService.ts`

**Checkpoint**: Fundação pronta - a implementação das estórias de usuário pode iniciar em paralelo.

---

## Phase 3: User Story 1 - Edição Modular sob Demanda no Canvas via Overlay DOM (Priority: P1) 🎯 MVP

**Goal**: Permitir que o usuário dê duplo-clique (ou tecle Enter) em qualquer cartão na Mesa para abrir um editor de texto rico montado dinamicamente no overlay DOM exatamente sobre o nó, destruindo a instância ao sair (Princípio II da Constituição: máximo de 1 a 2 instâncias ativas).

**Independent Test**: Dar duplo-clique em um cartão na Mesa, verificar o surgimento do container overlay DOM posicionado sobre o nó com foco imediato, digitar conteúdo e clicar fora para confirmar a desmontagem e atualização da textura no PixiJS.

### Tests for User Story 1

- [ ] T008 [P] [US1] Create integration test for cell markdown persistence and frontmatter round-trip in `tests/cell_markdown_tests.rs`

### Implementation for User Story 1

- [ ] T009 [US1] Update `src/stores/canvas.svelte.ts` to manage active editing node state (`editingNodeId`, `editingContent`, `editingBounds`) with 400ms autosave debounce
- [ ] T010 [US1] Implement dynamic overlay editor component with keyboard shortcuts (`Esc` to blur, `Cmd/Ctrl+Enter` to commit) in `src/components/canvas/CellOverlay.svelte`
- [ ] T011 [US1] Integrate `CellOverlay.svelte` over `src/components/canvas/CanvasViewport.svelte` with synchronized screen-coordinate transformation
- [ ] T012 [US1] Wire double-click and Enter key triggers on nodes in `src/components/canvas/PixiRenderer.ts` to open the overlay editor

**Checkpoint**: User Story 1 (MVP) concluída e testável de forma independente.

---

## Phase 4: User Story 2 - Criação e Gestão de Conexões Relacionais entre Nós (Priority: P2)

**Goal**: Permitir que o usuário conecte cartões através de alças de ancoragem magnéticas (`Top`, `Bottom`, `Left`, `Right`), desenhando curvas Bézier em tempo real no PixiJS com atualização fluida durante movimentações.

**Independent Test**: Puxar uma linha a partir da porta direita do "Nó A" até o "Nó B", verificar a fixação magnética da curva Bézier com seta direcional e mover o "Nó A" confirmando o re-layout da aresta a 60 FPS.

### Tests for User Story 2

- [ ] T013 [P] [US2] Create integration test for relational edge creation, self-loop rejection, and cascading deletion in `tests/canvas_topology_tests.rs`

### Implementation for User Story 2

- [ ] T014 [US2] Implement anchor ports geometry (`Top`, `Bottom`, `Left`, `Right`) and hover indicators on nodes in `src/components/canvas/PixiRenderer.ts`
- [ ] T015 [US2] Implement interactive drag-and-connect mechanic with elastic preview line in `src/components/canvas/PixiRenderer.ts`
- [ ] T016 [US2] Implement cubic Bézier curve calculation and directional arrow rendering in `src/components/canvas/PixiRenderer.ts`
- [ ] T017 [US2] Add edge management methods (`createEdge`, `deleteEdge`, `updateEdgeLabel`) with self-loop prevention in `src/stores/canvas.svelte.ts`
- [ ] T018 [US2] Add keyboard deletion (`Delete` / `Backspace`) for selected edges in `src/components/canvas/CanvasViewport.svelte`

**Checkpoint**: User Stories 1 e 2 operam de forma integrada e independente.

---

## Phase 5: User Story 3 - Persistência Canônica da Topologia e Células do Workspace (Priority: P3)

**Goal**: Persistir toda a geometria e conexões em `workspaces/<id>/board.canvas.json` com controle OCC (`revision`) e o texto de cada célula individual em `workspaces/<id>/cells/<id>.md`.

**Independent Test**: Modificar nós e arestas na Mesa, inspecionar os arquivos físicos gerados na pasta `workspaces/` do cofre e simular concorrência validando o tratamento de `RevisionConflict`.

### Tests for User Story 3

- [ ] T019 [P] [US3] Create integration test for two-phase atomic commit of `board.canvas.json` and OCC collision recovery in `tests/atomic_persistence_tests.rs`

### Implementation for User Story 3

- [ ] T020 [US3] Implement canonical cell file writer (`cells/<id>.md`) with YAML frontmatter in `src-tauri/src/infra/fs/canvas_io.rs`
- [ ] T021 [US3] Update `CanvasStore.saveTopology` in `src/stores/canvas.svelte.ts` to write both `board.canvas.json` and modified `cells/*.md`
- [ ] T022 [US3] Add optimistic concurrency conflict notification and automatic reload banner in `src/components/canvas/CanvasViewport.svelte`

**Checkpoint**: Persistência canônica dupla (topologia e células) validada em disco.

---

## Phase 6: User Story 4 - Navegação Espacial com Frustum Culling e Níveis de Detalhe (LOD) (Priority: P4)

**Goal**: Garantir navegação ultra-fluida (pan e zoom de 0.1x a 3.0x) sustentando 60 FPS com até 1.000 nós no canvas através de Frustum Culling geométrico e LOD adaptativo.

**Independent Test**: Carregar um canvas com dezenas de nós, dar zoom out abaixo de `0.35x` e verificar a transição para blocos simplificados (LOD macro), e dar zoom in acima de `0.4x` para verificar renderização com textos nítidos.

### Tests for User Story 4

- [ ] T023 [P] [US4] Create unit test for Frustum Culling AABB calculation and LOD threshold bounds in `tests/integration/test_lod_performance.rs`

### Implementation for User Story 4

- [ ] T024 [US4] Implement AABB viewport bounding box calculation and node visibility culling in `src/components/canvas/PixiRenderer.ts`
- [ ] T025 [US4] Implement Level of Detail (LOD) rendering layers (macro block vs. detailed text) based on camera zoom in `src/components/canvas/PixiRenderer.ts`
- [ ] T026 [US4] Implement viewport toolbar controls (zoom in, zoom out, fit to view / centralizar nós, zoom reset 100%) in `src/components/canvas/CanvasControls.svelte`

**Checkpoint**: Todas as 4 estórias de usuário operando a 60 FPS com persistência canônica e edição modular.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Sincronização, suporte a navegador web puro e validação de qualidade

- [ ] T027 [P] Update browser mock handler in `src/services/ipc.ts` to support `load_board_topology`, `save_board_topology`, and `save_workspace_cell` in `localhost:1420`
- [ ] T028 Run full test suite (`cargo test --no-default-features --tests`, `npm.cmd run check`, `npm.cmd run build`) and validate against scenarios in `specs/004-spatial-canvas-modular/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Sem dependências - inicia imediatamente.
- **Foundational (Phase 2)**: Depende da Fase 1 - BLOQUEIA todas as estórias de usuário.
- **User Story 1 (Phase 3)**: Depende da Fase 2.
- **User Story 2 (Phase 4)**: Depende da Fase 2.
- **User Story 3 (Phase 5)**: Depende da Fase 3 e Fase 4.
- **User Story 4 (Phase 6)**: Depende da Fase 2.
- **Polish (Phase 7)**: Depende da conclusão das estórias implementadas.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Concluir Fase 1 (Setup) e Fase 2 (Foundational).
2. Concluir Fase 3 (User Story 1 - Edição modular sob demanda via overlay DOM).
3. **Validar MVP**: Duplo-clique no cartão, edição rápida de texto, fechamento suave e persistência sem instanciar múltiplos editores.
4. Avançar para a Fase 4 (Arestas Bézier), Fase 5 (Persistência Canônica) e Fase 6 (Culling/LOD).

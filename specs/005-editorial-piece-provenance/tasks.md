# Tasks: Peça Editorial e Proveniência Canônica (Fork-on-Insert Estendido)

**Feature**: `005-editorial-piece-provenance`  
**Input**: Design artifacts from `specs/005-editorial-piece-provenance/` (`plan.md`, `spec.md`, `data-model.md`, `contracts/pieces-ipc.json`, `research.md`, `quickstart.md`)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Estruturação inicial de diretórios físicos, contratos e tipos de dados compartilhados

- [X] T001 [P] Ensure canonical `workspaces/<id>/pieces/` directory creation in `src-tauri/src/infra/fs/vault.rs`
- [X] T002 [P] Define TypeScript DTOs matching `contracts/pieces-ipc.json` (`EditorialPieceDTO`, `CitationRecordDTO`, `PieceSummaryDTO`, `CitationDriftItemDTO`, `CitationDriftStatus`) in `src/types/piece.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Entidades de domínio em Rust, I/O atômico com fsync e contratos IPC que bloqueiam as estórias de usuário

**⚠️ CRITICAL**: Nenhuma estória de usuário pode ser iniciada antes da conclusão desta fase.

- [X] T003 [P] Implement domain entities `EditorialPiece`, `CitationRecord`, and `CitationDrift` enum in `src-tauri/src/domain/workspace/piece.rs`
- [X] T004 [P] Implement YAML frontmatter serializer and parser for `EditorialPiece` with citations array in `src-tauri/src/domain/workspace/piece.rs`
- [X] T005 Implement atomic two-phase write and read for `pieces/*.md` with fsync and deterministic drift check against `cells/*.md` in `src-tauri/src/infra/fs/piece_io.rs`
- [X] T006 Expose Tauri IPC commands `list_workspace_pieces`, `read_workspace_piece`, `save_workspace_piece`, `check_piece_citations_drift`, and `compile_piece_export` in `src-tauri/src/ipc/workspace.rs` and register in `src-tauri/src/lib.rs`
- [X] T007 [P] Implement typed client IPC methods (`listWorkspacePieces`, `readWorkspacePiece`, `saveWorkspacePiece`, `checkPieceCitationsDrift`, `compilePieceExport`) in `src/services/pieceService.ts`

**Checkpoint**: Fundação pronta - a implementação das estórias de usuário pode iniciar em paralelo.

---

## Phase 3: User Story 1 - Criação e Redação de Peças Editoriais no Workspace (Priority: P1) 🎯 MVP

**Goal**: Permitir que o usuário crie, redija e organize documentos longos estruturados denominados "Peças" (`workspaces/<id>/pieces/<piece_id>.md`) com salvamento atômico durável ($\le 500\text{ ms}$) e persistência canônica em Markdown (Princípio I).

**Independent Test**: Criar uma Peça "Síntese da Arquitetura", redigir parágrafos e títulos, fechar e reabrir o aplicativo confirmando a integridade física do arquivo `.md` no cofre.

### Tests for User Story 1

- [X] T008 [P] [US1] Create integration test for `EditorialPiece` serialization, frontmatter round-trip, and file persistence in `tests/piece_markdown_tests.rs`

### Implementation for User Story 1

- [X] T009 [US1] Implement Svelte 5 store `src/stores/piece.svelte.ts` managing active piece state, pieces list, and 400ms autosave debounce
- [X] T010 [US1] Implement structured editorial long-form text editor component with Markdown shortcuts in `src/components/editorial/PieceEditor.svelte`
- [X] T011 [US1] Implement pieces list drawer/sidebar with create and switch piece actions in `src/components/editorial/PieceSidebar.svelte`
- [X] T012 [US1] Add view mode switcher (Mesa / Peça / Split-View) in `src/App.svelte` and `src/components/layout/Header.svelte`

**Checkpoint**: User Story 1 (MVP) concluída e testável de forma independente.

---

## Phase 4: User Story 2 - Fork-on-Insert da Mesa para a Peça com Citação Canônica (Priority: P2)

**Goal**: Permitir que o usuário transfira cartões ou trechos da Mesa para a Peça Editorial (arrastar e soltar ou comando de menu), clonando o conteúdo de forma totalmente autônoma e gravando metadados de proveniência no frontmatter YAML da Peça (Princípio II).

**Independent Test**: Arrastar um cartão da Mesa para dentro do editor da Peça, verificar a inserção do texto como citação com badge visual e confirmar a gravação de `source_cell_id`, `source_revision`, `quote` e `quote_hash` no frontmatter do arquivo físico.

### Tests for User Story 2

- [X] T013 [P] [US2] Create integration test for Fork-on-Insert citation generation, SHA-256 quote hash, and retroactive mutation immunity in `tests/piece_citation_tests.rs`

### Implementation for User Story 2

- [X] T014 [US2] Implement Split-View layout (Mesa à esquerda, Peça à direita) with dynamic PixiJS viewport resize in `src/components/canvas/CanvasViewport.svelte`
- [X] T015 [US2] Implement drag-and-drop transfer from PixiJS canvas nodes to the `PieceEditor` with quote snapshot extraction in `src/components/editorial/PieceEditor.svelte`
- [X] T016 [US2] Implement `CitationBadge.svelte` rendering embedded citation pill with source title, tooltip, and citation ID in `src/components/editorial/CitationBadge.svelte`
- [X] T017 [US2] Update `PieceStore` with `insertCitationFromNode` method updating both markdown body and frontmatter `citations` list in `src/stores/piece.svelte.ts`

**Checkpoint**: User Stories 1 e 2 operam de forma integrada e independente.

---

## Phase 5: User Story 3 - Auditoria de Proveniência e Detecção de Divergência (Drift Detection) (Priority: P3)

**Goal**: Alertar visualmente o autor quando o cartão original na Mesa sofrer edições posteriores à citação (`current_revision > source_revision`), com inspeção comparativa lado a lado e opção de atualizar ou manter a citação histórica.

**Independent Test**: Modificar um nó citado na Mesa, abrir a Peça, verificar a transição do badge para âmbar (*Drift detectado*), abrir a gaveta comparativa e atualizar conscientemente a citação.

### Tests for User Story 3

- [X] T018 [P] [US3] Create integration test for `check_piece_citations_drift` auditing synchronized, diverged, and orphaned citations in `tests/piece_drift_tests.rs`

### Implementation for User Story 3

- [X] T019 [US3] Implement drift status indicator (green = synchronized, amber = diverged, gray = orphaned) on `CitationBadge.svelte`
- [X] T020 [US3] Implement `DriftInspector.svelte` drawer displaying side-by-side diff between quoted snapshot and current cell revision in `src/components/editorial/DriftInspector.svelte`
- [X] T021 [US3] Implement "Atualizar Citação" action in `PieceStore` and `DriftInspector.svelte` allowing user to selectively sync cited text with current cell revision in `src/stores/piece.svelte.ts`
- [X] T022 [US3] Implement bidirectional camera navigation: clicking citation badge triggers PixiJS focus/center on source node in `src/components/canvas/PixiRenderer.ts` and `src/stores/canvas.svelte.ts`

**Checkpoint**: User Stories 1, 2 e 3 operam com rastreabilidade auditável completa.

---

## Phase 6: User Story 4 - Exportação Limpa e Compilação com Referências (Priority: P4)

**Goal**: Permitir a exportação da Peça para Markdown puro (com ou sem apêndice de notas de rodapé / bibliografia gerada a partir das citações) ou copiar para a área de transferência.

**Independent Test**: Acionar "Exportar Peça", selecionar "Markdown com Apêndice" e confirmar que o texto exportado inclui notas de rodapé numeradas remetendo aos cartões de origem.

### Tests for User Story 4

- [X] T023 [P] [US4] Create unit test for piece markdown compilation and references appendix generation in `src-tauri/src/infra/fs/piece_io.rs`

### Implementation for User Story 4

- [X] T024 [US4] Implement export modal / dropdown with options ("Copiar Markdown Limpo", "Baixar com Apêndice de Referências") in `src/components/editorial/PieceExportModal.svelte`
- [X] T025 [US4] Wire compile export IPC call and clipboard copy feedback in `src/stores/piece.svelte.ts`

**Checkpoint**: Todas as 4 estórias de usuário operando de forma coesa e independente.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Sincronização, suporte a navegador web puro e validação de qualidade

- [X] T026 [P] Update browser mock handler in `src/services/ipc.ts` to support all piece IPC commands in `localhost:1420`
- [X] T027 Run full test suite (`cargo test --no-default-features --tests`, `npm.cmd run check`, `npm.cmd run build`) and validate against scenarios in `specs/005-editorial-piece-provenance/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Sem dependências - inicia imediatamente.
- **Foundational (Phase 2)**: Depende da Fase 1 - BLOQUEIA todas as estórias de usuário.
- **User Story 1 (Phase 3)**: Depende da Fase 2.
- **User Story 2 (Phase 4)**: Depende da Fase 2 e Fase 3.
- **User Story 3 (Phase 5)**: Depende da Fase 4.
- **User Story 4 (Phase 6)**: Depende da Fase 3.
- **Polish (Phase 7)**: Depende da conclusão das estórias implementadas.

---

## Parallel Opportunities

- **Setup**: `T001` (Rust) e `T002` (TypeScript) podem ser executados em paralelo.
- **Foundational**: `T003` e `T004` (modelos de domínio) podem ser implementados em paralelo com `T007` (cliente TS).
- **User Stories**: Os testes de contrato/integração (`T008`, `T013`, `T018`, `T023`) podem ser implementados e validados antes do código de produção de cada fase.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Concluir Fase 1 (Setup) e Fase 2 (Foundational).
2. Concluir Fase 3 (User Story 1 - Criação e redação de Peças em Markdown).
3. **Validar MVP**: Criar uma Peça no app, redigir texto com autosave de 400ms e verificar o arquivo físico gerado no disco.
4. Avançar para a Fase 4 (Fork-on-Insert e Citações), Fase 5 (Drift Detection) e Fase 6 (Exportação).

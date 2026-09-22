# Tasks: Pipeline de Ingestão de Notas e Arquivos Físicos

**Feature**: `003-ingestion-pipeline`  
**Input**: Design artifacts from `specs/003-ingestion-pipeline/` (`plan.md`, `spec.md`, `data-model.md`, `contracts/ingest-ipc.json`, `research.md`, `quickstart.md`)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Estruturação inicial de diretórios e contratos de dados para ingestão física

- [ ] T001 Configure canonical ingestion directory tree (`ingest/notes/`, `ingest/web/`) in `src-tauri/src/infra/fs/vault.rs`
- [ ] T002 [P] Define TypeScript DTOs and IPC interfaces matching `contracts/ingest-ipc.json` in `src/types/ingest.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Infraestrutura de banco de dados SQLite, entidades de domínio e parsers que bloqueiam as estórias de usuário

**⚠️ CRITICAL**: Nenhuma estória de usuário pode ser iniciada antes da conclusão desta fase.

- [ ] T003 [P] Implement SQLite schema migration for `ingested_items` and FTS5 virtual table `ingested_items_fts` in `src-tauri/src/infra/db/schema.rs`
- [ ] T004 [P] Create `IngestRepository` with CRUD and FTS5 search queries in `src-tauri/src/infra/db/ingest_repo.rs`
- [ ] T005 Create domain models `IngestedItem`, `SourceType`, `IngestStatus` in `src-tauri/src/domain/ingest/item.rs`
- [ ] T006 [P] Implement Markdown frontmatter and word count parser in `src-tauri/src/domain/ingest/parser.rs`
- [ ] T007 Register `IngestRepository` and database access in Tauri `AppState` in `src-tauri/src/ipc/vault.rs` and `src-tauri/src/lib.rs`

**Checkpoint**: Fundação pronta - a implementação das estórias de usuário pode iniciar em paralelo.

---

## Phase 3: User Story 1 - Ingestão e Persistência Física de Notas Markdown (Priority: P1) 🎯 MVP

**Goal**: Permitir que arquivos `.md` e `.txt` arrastados ou selecionados sejam salvos fisicamente em `<vault_root>/ingest/notes/`, indexados no SQLite `index.db` e listados no painel do Acervo.

**Independent Test**: Arrastar um arquivo `.md` para o Dropzone, verificar a criação física do arquivo em `ingest/notes/`, confirmar o registro no `index.db` e visualizá-lo com metadados na listagem do Acervo.

### Tests for User Story 1

- [ ] T008 [P] [US1] Create integration test for physical note ingestion and SQLite indexing in `tests/ingest_lifecycle_tests.rs`

### Implementation for User Story 1

- [ ] T009 [US1] Implement physical file copy and atomic write for notes in `src-tauri/src/infra/fs/ingest_storage.rs` using `VaultGuard`
- [ ] T010 [US1] Implement IPC commands `ingest_file`, `ingest_file_content` and `list_ingested_items` in `src-tauri/src/ipc/ingest.rs`
- [ ] T011 [P] [US1] Connect `src/services/ingestService.ts` to native Tauri IPC commands
- [ ] T012 [US1] Update `src/stores/ingest.svelte.ts` to manage ingested items state, pagination and FTS search query
- [ ] T013 [US1] Update `src/components/ingest/Dropzone.svelte` and `src/components/ingest/IngestItemList.svelte` to trigger ingestion and render item cards with word count and tags

**Checkpoint**: User Story 1 (MVP) concluída e testável de forma independente.

---

## Phase 4: User Story 2 - Promoção de Itens Ingeridos para a Mesa Espacial (Priority: P2)

**Goal**: Permitir que notas do Acervo sejam promovidas como cartões visuais no Canvas com mecânica de Fork-on-Insert (célula desacoplada da nota bruta original com citação de proveniência).

**Independent Test**: Clicar em "Adicionar à Mesa" em um item do Acervo, verificar o surgimento do nó no Canvas, editar a célula na Mesa e verificar que a nota original em `ingest/notes/` permanece intacta.

### Tests for User Story 2

- [ ] T014 [P] [US2] Create integration test for Fork-on-Insert cell promotion and provenance metadata in `tests/promote_to_cell_tests.rs`

### Implementation for User Story 2

- [ ] T015 [US2] Implement IPC command `promote_to_cell` in `src-tauri/src/ipc/ingest.rs` instantiating a decoupled cell in workspace topology
- [ ] T016 [P] [US2] Implement `promoteItemToCell` method in `src/stores/canvas.svelte.ts` linking provenance attributes (`sourcePath`, `sourceHash`)
- [ ] T017 [US2] Add promotion button ("Adicionar à Mesa") and drag handle in `src/components/ingest/IngestItemList.svelte`

**Checkpoint**: User Stories 1 e 2 operam de forma integrada e independente.

---

## Phase 5: User Story 3 - Ingestão de Imagens e Ativos Binários no CAS (Priority: P3)

**Goal**: Salvar imagens e anexos arrastados para o Acervo diretamente no Content-Addressable Storage (`assets/<sha256>.<ext>`) com deduplicação criptográfica.

**Independent Test**: Enviar duas imagens idênticas, verificar que o arquivo físico é preservado no CAS com hash SHA-256 e reaproveitado sem duplicação de espaço em disco.

### Tests for User Story 3

- [ ] T018 [P] [US3] Create integration test for CAS asset ingestion and deduplication in `tests/asset_ingest_tests.rs`

### Implementation for User Story 3

- [ ] T019 [US3] Wire image drop in `src/components/ingest/Dropzone.svelte` to call `vaultService.storeAsset` and register asset record
- [ ] T020 [US3] Add asset thumbnail preview and image badge in `src/components/ingest/IngestItemList.svelte`

**Checkpoint**: Ingestão de notas e imagens no CAS operando conjuntamente.

---

## Phase 6: User Story 4 - Captura Rápida de Snapshot de URL (Priority: P4)

**Goal**: Permitir a captura textual de páginas web a partir de URLs, convertendo o conteúdo para Markdown limpo salvo em `ingest/web/` com proteção rigorosa contra SSRF.

**Independent Test**: Submeter uma URL de loopback (`127.0.0.1`) e verificar a rejeição sumária de segurança; submeter uma URL pública e verificar a gravação do snapshot em `ingest/web/`.

### Tests for User Story 4

- [ ] T021 [P] [US4] Create unit test for Anti-SSRF URL validator in `tests/ssrf_security_tests.rs`

### Implementation for User Story 4

- [ ] T022 [US4] Implement Anti-SSRF validator (blocking loopback, private RFC 1918 IPs and link-local) in `src-tauri/src/domain/core/security.rs`
- [ ] T023 [US4] Implement URL snapshot fetching and Markdown conversion in `src-tauri/src/infra/fs/ingest_storage.rs`
- [ ] T024 [US4] Implement IPC command `ingest_url` in `src-tauri/src/ipc/ingest.rs` and connect to URL input in `src/components/ingest/Dropzone.svelte`

**Checkpoint**: Todas as 4 estórias de usuário implementadas e validadas.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Sincronização e validação ponta a ponta

- [ ] T025 [P] Update browser mock handler in `src/services/ipc.ts` to simulate all new ingestion behaviors in `localhost:1420`
- [ ] T026 Implement cold start rehydration of physical notes from `ingest/notes/` into `index.db` in `src-tauri/src/infra/fs/vault.rs`
- [ ] T027 Run full test suite (`cargo test`, `npm.cmd run check`, `npm.cmd run build`) and validate against scenarios in `specs/003-ingestion-pipeline/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Sem dependências - inicia imediatamente.
- **Foundational (Phase 2)**: Depende da Fase 1 - BLOQUEIA todas as estórias de usuário.
- **User Story 1 (Phase 3)**: Depende da Fase 2.
- **User Story 2 (Phase 4)**: Depende da Fase 3 (US1).
- **User Story 3 (Phase 5)**: Depende da Fase 2.
- **User Story 4 (Phase 6)**: Depende da Fase 2.
- **Polish (Phase 7)**: Depende da conclusão das estórias desejadas.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Concluir Fase 1 (Setup) e Fase 2 (Foundational).
2. Concluir Fase 3 (User Story 1 - Persistência física de notas em `ingest/notes/` e listagem no Acervo).
3. **Validar MVP**: Arrastar nota no app, verificar o arquivo salvo fisicamente no disco e indexado no SQLite.
4. Avançar para a Fase 4 (Promoção para o Canvas com Fork-on-Insert) e demais estórias.

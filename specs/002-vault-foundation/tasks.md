# Implementation Tasks: Fundação do Vault e Protocolo de Persistência (v0.1)

**Feature**: `002-vault-foundation` | **Branch**: `002-vault-foundation` | **Spec**: [specs/002-vault-foundation/spec.md](spec.md) | **Plan**: [specs/002-vault-foundation/plan.md](plan.md)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Inicialização das dependências, infraestrutura de tratamento de erros e barramento de eventos do Rust Core.

- [X] T001 Configure workspace dependencies (tokio, serde, sha2, uuid v4/v7, chrono, thiserror, windows-sys) in `src-tauri/Cargo.toml`
- [X] T002 [P] Implement core error types (VaultError, SecurityError::SandboxEscapeAttempt, JournalError) in `src-tauri/src/domain/core/errors.rs`
- [X] T003 [P] Implement internal domain event bus and notifications in `src-tauri/src/domain/core/events.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Modelos de domínio, trait VaultStore e gerador de identidades que bloqueiam todas as histórias de usuário.

**⚠️ CRITICAL**: Nenhuma tarefa de história de usuário pode iniciar antes da conclusão desta fase.

- [X] T004 Define domain models (VaultManifest, CompatibilityFlags, AssetRef, JournalRecord, ObjectSnapshot) in `src-tauri/src/domain/model.rs`
- [X] T005 [P] Implement identity generator and UUID v7 monotonic timestamped IDs in `src-tauri/src/domain/identity.rs`
- [X] T006 [P] Define async `VaultStore` trait interface in `src-tauri/src/infra/fs/traits.rs`
- [X] T007 Implement atomic file replace and cross-platform sync utilities in `src-tauri/src/infra/fs/mod.rs`

**Checkpoint**: Fundação pronta - a implementação das histórias de usuário pode prosseguir de forma independente.

---

## Phase 3: User Story 1 - Inicialização, Criação e Abertura Segura de Vaults (Priority: P1) 🎯 MVP

**Goal**: Permitir ao usuário criar um novo cofre local com manifesto canônico `vault.json` e estrutura padronizada, ou abrir um cofre existente com validação de esquema e recusa segura de pastas inválidas.

**Independent Test**: Executar `cargo test --test vault_lifecycle_tests`, criando um cofre em pasta vazia e verificando `vault.json`, seguido de tentativa de abertura em pasta arbitrária comprovando recusa sem mutação.

### Tests for User Story 1

- [X] T008 [P] [US1] Create lifecycle integration test suite in `tests/vault_lifecycle_tests.rs`

### Implementation for User Story 1

- [X] T009 [US1] Implement canonical directory scaffolding (assets/, workspaces/, .history/, .system/cache/) in `src-tauri/src/infra/fs/lifecycle.rs`
- [X] T010 [US1] Implement `vault.json` generation and strict validation (UUID v4, schema_version: "2.0.0") in `src-tauri/src/infra/fs/lifecycle.rs`
- [X] T011 [US1] Implement Tauri IPC commands `init_vault` and `open_vault` in `src-tauri/src/ipc/vault.rs`

**Checkpoint**: User Story 1 concluída e testável como MVP autônomo.

---

## Phase 4: User Story 2 - Persistência Atômica e Imunidade a Quedas (Priority: P1)

**Goal**: Garantir persistência atômica em duas fases (`file-commit`) com journal contínuo `.jsonl`, limite máximo de perda de 500 ms de digitação e recuperação idempotente no boot após travamentos forçados.

**Independent Test**: Executar `cargo test --test atomic_persistence_tests`, disparando gravações concorrentes e interrupções forçadas de processo, comprovando recuperação exata sem arquivos corrompidos.

### Tests for User Story 2

- [X] T012 [P] [US2] Create atomic persistence and crash simulation test suite in `tests/atomic_persistence_tests.rs`

### Implementation for User Story 2

- [X] T013 [P] [US2] Implement append-only journal writer with FlushFileBuffers/fdatasync in `src-tauri/src/infra/fs/journal.rs`
- [X] T014 [US2] Implement two-phase file commit (.tmp_<uuid> -> journal flush -> atomic rename) in `src-tauri/src/infra/fs/writer.rs`
- [X] T015 [US2] Implement idempotent crash recovery and orphan `.tmp_*` purging at boot in `src-tauri/src/infra/fs/recovery.rs`
- [X] T016 [US2] Implement Tauri IPC command `commit_document` with expected_revision check in `src-tauri/src/ipc/vault.rs`

**Checkpoint**: User Stories 1 e 2 operando integradas com resiliência durável garantida.

---

## Phase 5: User Story 3 - Armazenamento de Anexos por Conteúdo (CAS) e Deduplicação (Priority: P2)

**Goal**: Armazenar mídias e binários de forma imutável em `assets/<sha256>.<ext>` com deduplicação proativa e preservação de integridade referencial.

**Independent Test**: Executar `cargo test --test cas_deduplication_tests`, salvando múltiplos arquivos idênticos e atestando a presença de exatamente 1 arquivo físico em `assets/`.

### Tests for User Story 3

- [X] T017 [P] [US3] Create CAS deduplication and integrity test suite in `tests/cas_deduplication_tests.rs`

### Implementation for User Story 3

- [X] T018 [US3] Implement SHA-256 streaming hash computation and CAS layout in `src-tauri/src/infra/fs/cas.rs`
- [X] T019 [US3] Implement deduplication check, read-only file attributes, and asset resolution in `src-tauri/src/infra/fs/cas.rs`
- [X] T020 [US3] Implement Tauri IPC commands `store_asset` and `read_asset` in `src-tauri/src/ipc/vault.rs`

**Checkpoint**: User Story 3 concluída; anexos binários armazenados com imutabilidade e deduplicação nativa.

---

## Phase 6: User Story 4 - Confinamento de Acesso e Proteção Anti-TOCTOU (Priority: P2)

**Goal**: Mediar todo I/O através do `VaultGuard`, bloqueando travessias `..`, symlinks maliciosos e condições de corrida entre verificação e abertura física.

**Independent Test**: Executar `cargo test --test anti_toctou_tests`, submetendo caminhos com `..`, symlinks externos e renomeações concorrentes, confirmando disparo de `SecurityError::SandboxEscapeAttempt`.

### Tests for User Story 4

- [X] T021 [P] [US4] Create anti-TOCTOU and path traversal test suite in `tests/anti_toctou_tests.rs`

### Implementation for User Story 4

- [X] T022 [US4] Implement lexical path validation (blocking `..`, drive letters, and null bytes) in `src-tauri/src/infra/security/broker.rs`
- [X] T023 [P] [US4] Implement Linux descriptor-based resolution (`openat` with O_NOFOLLOW / RESOLVE_BENEATH) in `src-tauri/src/infra/security/platform_unix.rs`
- [X] T024 [P] [US4] Implement Windows reparse-point safe directory handle resolution in `src-tauri/src/infra/security/platform_windows.rs`
- [X] T025 [US4] Integrate `VaultGuard` descriptor enforcement with `lifecycle.rs` and `writer.rs` in `src-tauri/src/infra/security/broker.rs`

**Checkpoint**: Confinamento estrito e proteção anti-TOCTOU ativos sobre todas as operações do cofre.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Integração final de IPC, compilação limpa e validação dos cenários do quickstart.

- [X] T026 [P] Register and export all vault IPC commands in `src-tauri/src/lib.rs`
- [X] T027 [P] Execute full test suite via `cargo test --all` and audit dependencies via `cargo check`
- [X] T028 Execute end-to-end quickstart validation scenarios from `specs/002-vault-foundation/quickstart.md`

---

## Dependencies & Execution Order

```text
Phase 1: Setup (T001-T003)
      │
      ▼
Phase 2: Foundational (T004-T007)
      │
      ├───────────────────────┬───────────────────────┐
      ▼                       ▼                       ▼
Phase 3: US1 (Lifecycle)  Phase 4: US2 (Persist.) Phase 5: US3 (CAS)
(T008-T011) [MVP]         (T012-T016)             (T017-T020)
      │                       │                       │
      └───────────────────────┼───────────────────────┘
                              ▼
               Phase 6: US4 (Anti-TOCTOU & Guard)
               (T021-T025)
                              │
                              ▼
               Phase 7: Polish & Validation
               (T026-T028)
```

### Story Completion Order
1. **US1 (P1 - MVP)**: Estabelece a criação da pasta do cofre e do `vault.json`. Sem ela, não há pasta para persistir dados.
2. **US2 (P1)**: Estabelece o journal `.jsonl` e escrita atômica. Sem ela, escritas de notas e topologias não são confiáveis.
3. **US3 (P2)**: Adiciona suporte a mídias e binários no CAS (`assets/`).
4. **US4 (P2)**: Enrijece a mediação de descritores no `VaultGuard` contra TOCTOU e symlinks para fora do cofre.

### Parallel Opportunities

- **Setup & Foundational**:
  - T002 e T003 podem ser implementados em paralelo.
  - T005 e T006 podem ser implementados em paralelo após T004.
- **Por User Story**:
  - T008 (testes US1) pode ser escrito em paralelo com o design inicial.
  - T012 (testes US2) e T013 (journal writer) podem ser desenvolvidos em paralelo.
  - T017 (testes US3) e T018 (hashing SHA-256) podem ser desenvolvidos em paralelo.
  - T023 (Unix) e T024 (Windows) podem ser implementados em paralelo em arquivos separados.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Concluir Phase 1 (Setup) e Phase 2 (Foundational).
2. Concluir Phase 3 (User Story 1 - T008 a T011).
3. Executar `cargo test --test vault_lifecycle_tests` para validar criação e abertura de cofres.

### Entrega Incremental
1. Concluir Phase 4 (User Story 2) $\rightarrow$ Validar resiliência com simulação de crash.
2. Concluir Phase 5 (User Story 3) $\rightarrow$ Validar deduplicação de mídias por hash SHA-256.
3. Concluir Phase 6 (User Story 4) $\rightarrow$ Validar blindagem contra symlinks e TOCTOU.
4. Concluir Phase 7 (Polish) $\rightarrow$ Rodar `specs/002-vault-foundation/quickstart.md`.

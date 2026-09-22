# Implementation Plan: Fundação do Vault e Protocolo de Persistência (v0.1)

**Branch**: `002-vault-foundation` | **Date**: 2026-09-22 | **Spec**: [specs/002-vault-foundation/spec.md](spec.md)

**Input**: Feature specification from `/specs/002-vault-foundation/spec.md`

## Summary

Implementar a camada fundamental de soberania de dados e persistência do Sandland (Sprint 01 / v0.1):
1. **Ciclo de vida do Vault**: Criação, validação, abertura e detecção de integridade baseada no manifesto canônico `vault.json` e estrutura canônica padronizada.
2. **Protocolo de Persistência Atômica em Duas Fases (`file-commit`)**: Journaling contínuo em `.history/events/*.jsonl` com `fsync`/`FlushFileBuffers` seguido de substituição atômica (`rename`), garantindo tolerância a quedas com perda máxima $\le 500\text{ ms}$ de digitação ativa e rotina idempotente de recuperação no boot.
3. **Content-Addressable Storage (CAS)**: Armazenamento imutável de binários/mídias em `assets/<hash>.<ext>` endereçados por hash SHA-256 com deduplicação nativa.
4. **Mediação de I/O Anti-TOCTOU e Confinamento (`VaultGuard`)**: Resolução segura de caminhos baseada em descritores vinculados à raiz do cofre, bloqueando corridas de sistema de arquivos, travessias `..` e symlinks maliciosos (`SecurityError::SandboxEscapeAttempt`).

## Technical Context

**Language/Version**: Rust 1.80+ (edição 2021)
**Primary Dependencies**:
- `tokio` (runtime assíncrono para fila de I/O)
- `serde`, `serde_json` (serialização de manifesto, topologia e journal)
- `sha2` (computação de hash SHA-256 para o CAS)
- `uuid` (UUID v4 para identidade de Vaults, sessões e operações)
- `chrono` (timestamps UTC ISO-8601 de alta precisão)
- `thiserror` (erros fortemente tipados de domínio e segurança)
- `windows-sys` / `libc` (primitivas de descritores de baixo nível, `FlushFileBuffers` / `sync_all` e resolução anti-TOCTOU)
**Storage**: Sistema de arquivos local com arquivos abertos legíveis (Markdown, JSON, `.jsonl`, CAS SHA-256). Proibições expressas de persistir histórico autoral em SQLite efêmero.
**Testing**: `cargo test` (testes unitários e de integração), suíte de testes de simulação de falha brusca (crash simulation), suíte de vetores de ataque TOCTOU/symlink traversal.
**Target Platform**: Windows x86_64, Linux (x86_64, aarch64), macOS (Apple Silicon, Intel).
**Project Type**: Rust Core Library (`crates/vault`, `crates/domain`, `crates/security`) integrada ao shell Tauri v2.
**Performance Goals**:
- Inicialização e validação de cofre existente $\le 150\text{ ms}$ em NVMe.
- Confirmação de journal e I/O durável dentro da janela de $\le 500\text{ ms}$.
- Deduplicação de assets a 100% de precisão por SHA-256.
**Constraints**:
- 100% autônomo e offline (zero dependência de rede).
- Eliminação absoluta de caminhos de arquivo hardcoded.
- Proibição de I/O de arquivo bloqueante na thread de renderização da UI.
**Scale/Scope**: Cofres com até 100.000 arquivos e 10 GB de mídias sem degradação do journal ou do boot.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Princípio Constitucional | Regra / Diretriz | Status | Conformidade no Plano |
|---|---|---|---|
| **I. Soberania do Dado** | File-as-Truth & Local-First. 4 classes de dados. Histórico em `.history/`. SQLite descartável. | **PASS** | O plano adota `vault.json`, `board.canvas.json`, `.history/events/*.jsonl` e CAS SHA-256 como dados canônicos; SQLite é apenas índice derivado. |
| **II. Separação Cognitiva** | Ingest append-only, fork-on-insert estendido, fronteira PixiJS/BlockSuite. | **PASS** | `assets/` e `ingest/` são imutáveis e append-only; metadados canônicos de citação são mapeados nas entidades de domínio. |
| **III. Contenção Rigorosa** | Anti-TOCTOU real (descritores restritos), sem caminhos hardcoded, egress deny por padrão. | **PASS** | Substituição da sequência ingênua por descritores vinculados à raiz (`VaultGuard`); resolução dinâmica via `app.path()`. |
| **IV. Núcleo Nativo** | Rust como autoridade de dados e persistência. Metas de boot e idle. | **PASS** | Implementação estritamente em Rust nativo; I/O assíncrono serializado; boot < 150 ms. |
| **V. Context Engine & Busca** | Vectorless Context Engine para a Mesa e Busca Híbrida desacoplada. | **PASS** | O plano garante que a topologia canônica `board.canvas.json` é preservada e desacoplada de índices de busca. |
| **VI. Decisões de Taxonomia** | Decisões tipadas, sem merge cego, agnóstico a modelos. | **PASS** | Esquema canônico reserva `taxonomy/terms.json` com IDs estáveis e termos auditáveis. |
| **VII. Resiliência Operacional** | Perda máxima $\le 500\text{ ms}$, journal atômico em duas fases, recuperação idempotente no boot. | **PASS** | Protocolo formal `file-commit` implementado com journal `.jsonl` sequencial e recuperação no boot. |

## Project Structure

### Documentation (this feature)

```text
specs/002-vault-foundation/
├── spec.md              # Especificação de requisitos e histórias de usuário
├── plan.md              # Este plano de implementação
├── research.md          # Fase 0: Decisões técnicas e mitigação de incógnitas
├── data-model.md        # Fase 1: Entidades, schemas e estados de persistência
├── quickstart.md        # Fase 1: Guia prático de validação e cenários de teste
├── contracts/           # Fase 1: Contratos e interfaces tipadas
│   ├── vault-store.md
│   ├── file-commit-protocol.md
│   └── io-broker.md
└── checklists/
    └── requirements.md  # Checklist de validação de qualidade
```

### Source Code (repository layout)

```text
src-tauri/
├── Cargo.toml
└── src/
    ├── domain/
    │   ├── identity.rs             # Geração de IDs v7, validação e desacoplamento de caminhos
    │   ├── model.rs                # VaultManifest, AssetRef, JournalRecord
    │   └── mod.rs
    ├── infra/
    │   ├── fs/
    │   │   ├── lifecycle.rs        # Criação, validação, manifesto e verificação de raiz
    │   │   ├── journal.rs          # Log append-only durável (.history/events/*.jsonl)
    │   │   ├── writer.rs           # Atomic file commit (write tmp + fsync + rename)
    │   │   ├── cas.rs              # Content-Addressable Storage (SHA-256 deduplication)
    │   │   ├── recovery.rs         # Rotina de recuperação idempotente no boot
    │   │   └── mod.rs
    │   └── security/
    │       ├── broker.rs           # VaultGuard: mediação de I/O anti-TOCTOU
    │       ├── platform_unix.rs    # openat2 / O_NOFOLLOW / RESOLVE_BENEATH
    │       ├── platform_windows.rs # Reparse-point safe directory handles
    │       └── mod.rs
    ├── ipc/
    │   ├── vault.rs                # Comandos Tauri IPC tipados para o Vault
    │   └── mod.rs
    └── lib.rs
tests/
├── vault_lifecycle_tests.rs        # Testes de criação, abertura e recusa segura
├── atomic_persistence_tests.rs     # Testes de duas fases e recuperação de falha
├── cas_deduplication_tests.rs      # Testes de integridade e deduplicação de assets
└── anti_toctou_tests.rs            # Suíte de vetores de fuga de sandbox e symlinks
```

**Structure Decision**: A implementação do Sprint 01 é organizada em módulos coesos dentro de `src-tauri/src/` (`domain`, `infra/fs`, `infra/security`, `ipc`), preparando a base arquitetural para futura extração em crates independentes (`crates/vault`, `crates/security`) sem overhead prematuro de workspace no build inicial do Tauri v2.

## Complexity Tracking

> **Nenhuma violação constitucional identificada.** Todas as regras de soberania do dado, resiliência, anti-TOCTOU e local-first são cumpridas integralmente.

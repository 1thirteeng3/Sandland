# Implementation Plan: Pipeline de Ingestão de Notas e Arquivos Físicos

**Branch**: `003-ingestion-pipeline` | **Date**: 2026-09-22 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/003-ingestion-pipeline/spec.md`

## Summary

Implementar o pipeline completo de ingestão e processamento físico de documentos no Sandland v2.0. As notas e arquivos recebidos no painel lateral serão salvos como arquivos reais na pasta canônica do cofre (`ingest/notes/` e `assets/`), indexados com busca FTS5 no SQLite (`index.db`) e habilitados para promoção desacoplada (Fork-on-Insert) como cartões interativos na Mesa Espacial (Canvas).

## Technical Context

**Language/Version**: Rust 1.80+ (Backend Tauri v2), TypeScript 5.5+ & Svelte 5 (Frontend).  
**Primary Dependencies**: Tauri v2, `rusqlite` (FTS5 + bundled), `tokio`, `serde`, `sha2`, `pixi.js`, `svelte 5`.  
**Storage**: Sistema de arquivos canônico (`ingest/notes/`, `assets/<hash>.<ext>`), SQLite `index.db` (projeção descartável).  
**Testing**: `cargo test` para testes unitários e de integração de I/O e Anti-SSRF; `svelte-check` e `npm run test` no frontend.  
**Target Platform**: Windows 10/11 Desktop, macOS, Linux (com fallback de simulação Web para `localhost:1420`).  
**Project Type**: Desktop Local-First Knowledge Environment (Tauri v2).  
**Performance Goals**: Ingestão e gravação física < 80ms para notas de 1MB; Promoção para o Canvas a 60 FPS (< 16ms).  
**Constraints**: Zero conectividade externa obrigatória (100% offline-ready); Anti-TOCTOU e proteção rigorosa contra SSRF; File-as-Truth inegociável.  
**Scale/Scope**: Suporte a até 50.000 notas indexadas localmente em FTS5 sem perda de fluidez na busca.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Princípio I (File-as-Truth)**: O arquivo físico `.md` em `ingest/notes/` é a fonte primária; SQLite é estritamente derivado e reidratável a frio.
- [x] **Princípio II (Fork-on-Insert Estendido)**: Ao promover uma nota para o Canvas, cria-se uma cópia independente com link de proveniência sem two-way binding mutável.
- [x] **Princípio III (Anti-TOCTOU & Anti-SSRF)**: Resolução de caminhos via `VaultGuard`; bloqueio de IPs privados e loopback na captura de URLs.
- [x] **Princípio IV (Zero-Config & Eficiência)**: Sem dependência de serviços externos pesados para o pipeline básico.
- [x] **Princípio VII (Journaling Atômico & Resiliência)**: Gravações em disco utilizam o protocolo atômico em duas fases com journal WAL.

## Project Structure

### Documentation (this feature)

```text
specs/003-ingestion-pipeline/
├── spec.md              # Especificação funcional de requisitos
├── plan.md              # Este plano de implementação
├── research.md          # Decisões de pesquisa técnica (Phase 0)
├── data-model.md        # Esquema relacional SQLite e DTOs (Phase 1)
├── contracts/           # Contratos IPC JSON Schema (Phase 1)
│   └── ingest-ipc.json
├── quickstart.md        # Cenários de validação executáveis (Phase 1)
├── checklists/
│   └── requirements.md  # Checklist de qualidade da especificação
└── tasks.md             # Tarefas de implementação (Phase 2 - via /speckit-tasks)
```

### Source Code

```text
src-tauri/src/
├── domain/
│   ├── ingest/                 # Entidades e regras de ingestão
│   │   ├── mod.rs
│   │   ├── item.rs             # IngestedItem, SourceType, IngestStatus
│   │   └── parser.rs           # Extração de título, tags YAML e contagem de palavras
│   └── core/
│       └── security.rs         # Validador Anti-SSRF para URLs
├── infra/
│   ├── db/
│   │   ├── schema.rs           # Migrations da tabela ingested_items e FTS5
│   │   └── ingest_repo.rs      # Repositório SQLite para ingested_items
│   └── fs/
│       └── ingest_storage.rs   # Gravador atômico em ingest/notes/ e ingest/web/
├── ipc/
│   └── ingest.rs               # Comandos Tauri: ingest_file, ingest_url, list_ingested_items, promote_to_cell
└── lib.rs

src/
├── components/
│   ├── ingest/
│   │   ├── Dropzone.svelte         # Captura drag-and-drop e URL
│   │   ├── IngestItemList.svelte   # Lista de notas com busca FTS
│   │   └── IngestDrawer.svelte     # Drawer lateral
│   └── canvas/
│       └── CanvasViewport.svelte   # Receptor de drop e criação de cards
├── services/
│   ├── ingestService.ts        # Chamadas IPC tipadas
│   └── ipc.ts                  # Mock de browser expandido
├── stores/
│   ├── ingest.svelte.ts        # Store reativa Svelte 5 de itens ingeridos
│   └── canvas.svelte.ts        # Inserção de nó a partir de item do acervo
└── types/
    └── ingest.ts               # Interfaces TypeScript
```

**Structure Decision**: Utiliza a estrutura canônica em camadas do Sandland com autoridade em Rust e interface em Svelte 5 + PixiJS.

## Complexity Tracking

> Nenhuma violação aos princípios constitucionais foi identificada; dispensada justificativa de exceção.

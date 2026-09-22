# Implementation Plan: Núcleo Local-First e Canvas Básico (v0.1)

**Branch**: `001-local-first-canvas` | **Date**: 2026-09-21 (Revisão Técnica Pós-Análise Crítica SDD+DDD) | **Spec**: [specs/001-local-first-canvas/spec.md](spec.md)

**Input**: Feature specification from `/specs/001-local-first-canvas/spec.md`

---

## Summary

Implementação técnica da versão base v0.1 do Sandland, estabelecendo o ecossistema desktop local-first através do runtime Tauri v2 com arquitetura orientada ao domínio (DDD). O plano cobre a criação do cofre físico (*File-as-Truth* com Markdown e YAML Frontmatter), o subsistema de captura com máquina de estados determinística e monitoramento reativo de disco (`notify`), o índice descartável em SQLite gerenciado via `refinery` e enriquecido com busca vetorial (`sqlite-vec` via `fastembed`) e busca textual (`fts5`), a inferência local de SLM via `llama.cpp` com amostragem forçada por gramática GBNF para auto-tagging seguro, e a mesa de trabalho espacial híbrida (Whiteboard) com renderização acelerada por hardware em WebGL utilizando PixiJS v8 combinada com projeção de overlay DOM (Svelte 5) para edição interativa de texto rica.

---

## Technical Context

**Language/Version**: Rust 1.80+ (Backend nativo com runtime assíncrono `tokio`), TypeScript 5.5+ / Svelte 5 (Frontend)  
**Primary Dependencies**: 
- *Backend*: Tauri v2, `tokio` (full), `rusqlite` (compilação estática com extensão `sqlite-vec` via C99), `refinery` (migrações versionadas), `llama-cpp-2` (FFI nativo para SLM generativo com gramática GBNF), `fastembed` (embeddings vetoriais leves ONNX multilíngues `paraphrase-multilingual-MiniLM-L12-v2`), `notify-debouncer-full`, `rmp-serde` (MessagePack), `thiserror`
- *Frontend*: Svelte 5 (Runes), PixiJS v8, Vite, `@tauri-apps/api`  
**Storage**: Sistema de arquivos local do Vault (`.md` com YAML Frontmatter em `/vault/ingest/`), índices SQLite efêmeros (`.system/index.db` e `.system/audit_log.db`), arquivos de topologia (`board.canvas.mpk` e `board.canvas.json`)  
**Testing**: `cargo test` (unitários de domínio, integração de banco/migração e testes de segurança de caminhos), `vitest` / Playwright (testes de componentes Svelte e renderização da Webview)  
**Target Platform (v0.1)**: Windows 11/10 (x86_64) e Linux (x86_64) (*macOS tratado como target de validação na v0.2*)  
**Project Type**: Desktop Application (Tauri v2)  
**Performance Goals**: 
- Renderização de Canvas a 60 FPS estáveis no Viewport via Frustum Culling e LOD
- Tempo de inicialização a frio < 2.2s em NVMe
- Ingestão e gravação de arquivos em disco < 1.0s
- Inferência de classificação taxonômica local: P50 < 3.0s, P95 < 8.0s em CPUs com AVX2
- Geração de embeddings vetoriais via `fastembed`: < 15ms por documento
- Latência de comando síncrono IPC < 10ms  
**Constraints**: 
- Consumo de memória em repouso (*idle*) $\le 350\text{ MB}$
- Descarregamento obrigatório (*unload*) do SLM da memória após 5 minutos de inatividade
- Operação 100% offline-first sem tráfego de rede para serviços externos
- Confinamento estrito de I/O em disco via validação de caminhos canônicos em Rust (*Path Guard* contra TOCTOU e symlink traversal)  
**Scale/Scope (v0.1)**: 1.000 documentos indexados no cofre com busca instantânea e dezenas de nós no canvas simultaneamente (escala de 10.000+ programada para v0.2)  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Princípio Constitucional | Avaliação no Design | Status |
| :--- | :--- | :---: |
| **I. Soberania do Dado (*File-as-Truth*)** | Todo documento é persistido fisicamente como `.md` no disco com Frontmatter. SQLite (`index.db`) atua unicamente como índice derivado descartável e reconstruível a frio via `refinery`. | **PASS** |
| **II. Separação Cognitiva & Ingest Passivo** | O Ingest é uma camada de absorção passiva (*append-only*) com máquina de estados explícita (`Pending -> Extracting -> Classifying -> Classified`); o Canvas opera como sandbox visual independente. | **PASS** |
| **III. Contenção Rigorosa & I/O Seguro** | Todas as rotinas de I/O resolvem caminhos canônicos e validam limites de prefixo contra o `base_workspace`, prevenindo TOCTOU e symlink escape. | **PASS** |
| **IV. Desempenho Nativo & Limite de Recursos** | Core 100% em Rust nativo com `tokio` e Tauri v2. Svelte 5 (sem Virtual DOM) e PixiJS WebGL com LOD e culling de nós invisíveis. Memória mantida abaixo de 350 MB no repouso com garbage collection do SLM aos 5 minutos. | **PASS** |
| **V. Arquitetura Determinística de IA & GBNF** | A inferência local utiliza gramática GBNF estrita que garante schema JSON válido, com cascata de contingência para heurística léxica/cosseno e marcação manual. Embeddings gerados de forma desacoplada via `fastembed`. | **PASS** |
| **VI. Resiliência Operacional & Versionamento Duplo** | Topologia do canvas gravada a cada 350ms em MessagePack (`board.canvas.mpk`) e sincronizada em JSON legível para o Git no repouso (>2s). Recuperação de falhas via `audit_log.db` com perda < 500ms. | **PASS** |

*Resultado do Gate*: **APROVADO (6/6 Passos Conformes)**. Nenhuma exceção ou violação identificada.

---

## Project Structure (Organização Orientada ao Domínio - DDD)

### Documentation (this feature)

```text
specs/001-local-first-canvas/
├── spec.md                  # Especificação funcional v0.1
├── plan.md                  # Este plano de implementação técnica
├── research.md              # Decisões arquiteturais B1-B4, DOM overlay, GBNF e fastembed
├── data-model.md            # Esquemas de banco de dados, Frontmatter YAML e máquina de estados
├── quickstart.md            # Guia de compilação, boot e validação ponta a ponta
├── contracts/
│   └── tauri-ipc.md         # Contratos IPC canônicos, SandlandError enum e DTOs
└── checklists/
    └── requirements.md      # Validação de qualidade da especificação
```

### Source Code (repository layout)

```text
src/                          # Frontend Svelte 5 (Runes) + TypeScript + PixiJS v8 + Vite
├── index.html                # Ponto de montagem da WebView
├── package.json              # Dependências frontend (svelte@5, pixi.js@8, @tauri-apps/api@2)
├── vite.config.ts            # Configuração de bundling e alias
├── src/
│   ├── main.ts               # Ponto de entrada da aplicação
│   ├── App.svelte            # Casca principal (Layout com Canvas e Drawer)
│   ├── styles/               # Design tokens, tipografia editorial e tema escuro
│   ├── components/
│   │   ├── canvas/           # Mesa de trabalho espacial
│   │   │   ├── CanvasViewport.svelte # Container WebGL do PixiJS + overlay DOM
│   │   │   ├── PixiRenderer.ts       # Engine PixiJS (nós, arestas, LOD e Frustum Culling)
│   │   │   ├── CellOverlay.svelte    # Micro-editor de texto projetado dinamicamente no duplo clique
│   │   │   └── CanvasControls.svelte # Controles de zoom, pan e reset de câmera
│   │   ├── ingest/           # Camada de captura passiva
│   │   │   ├── IngestDrawer.svelte   # Gaveta lateral retrátil de itens
│   │   │   ├── Dropzone.svelte       # Área de arrastar-e-soltar arquivos
│   │   │   ├── IngestItemList.svelte # Lista reativa de documentos capturados
│   │   │   └── ModelProgress.svelte  # Indicador de download inicial do modelo SLM
│   │   └── common/           # Badges de tags, botões, modais e avisos de revisão
│   ├── stores/               # Estado reativo com Runes do Svelte 5
│   │   ├── canvas.svelte.ts  # Estado da viewport, nós ativos e seleção
│   │   ├── ingest.svelte.ts  # Lista reativa de itens e eventos de processamento
│   │   └── vault.svelte.ts   # Metadados do cofre aberto
│   ├── services/             # Clientes IPC tipados (invocações do @tauri-apps/api/core)
│   │   ├── ipc.ts            # Wrapper com tipagem estrita e tratamento de SandlandError
│   │   ├── vaultService.ts   # Chamadas de create/open vault
│   │   ├── ingestService.ts  # Chamadas de ingest_file, trigger_classification
│   │   └── canvasService.ts  # Chamadas de load_board_topology, save_fast
│   └── types/                # Definições de tipos TypeScript sincronizadas com os contratos IPC
│       ├── errors.ts         # SandlandError e códigos canônicos
│       ├── ingest.ts         # IngestedItemDTO, IngestState
│       └── canvas.ts         # CanvasTopologyDTO, CanvasNodeDTO, CanvasEdgeDTO

src-tauri/                    # Backend Nativo em Rust
├── Cargo.toml                # Dependências Rust (tauri v2, tokio, rusqlite, refinery, llama-cpp-2, fastembed)
├── build.rs                  # Compilação estática em C99 da extensão sqlite-vec
├── tauri.conf.json           # Configurações de janela, permissões e plugins do Tauri v2
├── migrations/               # Migrações SQL gerenciadas via refinery
│   ├── V1__initial_schema.sql # DDL das tabelas items, items_fts, vec_items, taxonomy
│   └── V2__audit_events.sql   # DDL da tabela de auditoria de eventos
└── src/
    ├── main.rs               # Entrypoint do executável nativo
    ├── lib.rs                # Inicialização do Tauri v2, plugins e registro de comandos IPC
    ├── domain/               # CAMADA DE DOMÍNIO (Lógica de Negócio Pura & Bounded Contexts)
    │   ├── core/             # Tipos primitivos, SandlandError enum e eventos de domínio
    │   │   ├── mod.rs
    │   │   ├── errors.rs     # Enum unificado SandlandError
    │   │   └── events.rs     # Definições de eventos (ItemDetected, IngestStateChanged)
    │   ├── ingest/           # Bounded Context de Ingestão de Documentos
    │   │   ├── mod.rs
    │   │   ├── entity.rs     # IngestedItem, ItemType
    │   │   └── state_machine.rs # Máquina de estados (Pending -> Extracting -> Classifying -> Classified)
    │   ├── taxonomy/         # Bounded Context de Classificação e Taxonomia
    │   │   ├── mod.rs
    │   │   ├── entity.rs     # TaxonTerm, Category
    │   │   └── normalizer.rs # Regras de kebab-case, singularização e Levenshtein
    │   └── workspace/        # Bounded Context da Mesa de Trabalho Espacial
    │       ├── mod.rs
    │       ├── topology.rs   # BoardTopology, CanvasNode, CanvasEdge, CanvasGroup
    │       └── cell.rs       # Células atômicas estruturadas e rotina de promoção a nota
    ├── infra/                # CAMADA DE INFRAESTRUTURA (Adaptadores Técnicos)
    │   ├── db/               # Banco de dados SQLite e índices derivados
    │   │   ├── mod.rs        # Pool de conexões e execução de migrações refinery
    │   │   ├── indexer.rs    # Algoritmo de busca híbrida com Reciprocal Rank Fusion (RRF k=60)
    │   │   └── audit.rs      # Gravação de alta frequência no audit_log.db
    │   ├── fs/               # Sistema de arquivos físico do Vault (File-as-Truth)
    │   │   ├── mod.rs
    │   │   ├── vault.rs      # Leitura/escrita segura contra TOCTOU e validação de canonicidade
    │   │   └── watcher.rs    # notify-debouncer para /vault/ingest/notes/
    │   ├── ai/               # Subsistema de Inteligência Artificial Model-Agnostic & RLM
    │   │   ├── mod.rs
    │   │   ├── provider.rs   # Trait unificada ModelProvider e tipos ChatMessage/GenerationConfig
    │   │   ├── local_llama.rs# Adaptador LocalLlamaProvider (GGUF offline via llama.cpp + GBNF)
    │   │   ├── cloud_api.rs  # Adaptador CloudApiProvider (OpenAI, Anthropic, OpenRouter - BYOK)
    │   │   ├── rlm.rs        # Orquestrador RLM & Gerador em tempo de execução do Skeleton Map
    │   │   ├── embeddings.rs # Geração desacoplada de vetores ONNX via fastembed para Acervo (<15ms)
    │   │   └── fallback.rs   # Cascata de contingência (TF-IDF léxico e similaridade de cosseno)
    │   └── sandbox/          # Path Guard de isolamento e validação dinâmica de limites do OS
    └── ipc/                  # CAMADA DE APRESENTAÇÃO IPC (Tauri Commands)
        ├── mod.rs
        ├── vault.rs          # create_vault, open_vault
        ├── ingest.rs         # ingest_file, ingest_file_content, ingest_url, list_ingested_items
        ├── taxonomy.rs       # trigger_classification, update_item_tags
        └── workspace.rs      # create_workspace, load_board_topology, save_board_topology_fast, create_cell, promote_cell_to_note

tests/
├── contract/                 # Validação de contratos IPC e serialização de DTOs e SandlandError
├── integration/              # Testes ponta a ponta: Ingest -> StateMachine -> DB -> fastembed -> Canvas
└── unit/                     # Testes unitários puros (Path Guard, GBNF parser, normalizador taxonômico, Skeleton Map)
```

**Structure Decision**: Adoção de arquitetura em camadas orientada ao domínio (DDD). A lógica pura de regras de negócio reside em `domain/`, os adaptadores de banco, I/O e IA residem em `infra/`, os comandos expostos à interface residem em `ipc/`, e o frontend em Svelte 5 reside em `src/`, garantindo separação clara de responsabilidades e facilitando a geração autônoma e precisa de código pelos agentes de IA. A camada de IA adota a trait `ModelProvider` para suportar tanto execução local (GGUF) quanto nuvem (BYOK), e o canvas utiliza RLM com Skeleton Map (Vectorless RAG).

---

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violação Identificada | Por que é estritamente necessária | Alternativa mais simples rejeitada e por quê |
| :--- | :--- | :--- |
| *Nenhuma violação* | O design revisado adere integralmente a todos os princípios constitucionais e resolve todos os gargalos técnicos identificados. | N/A |

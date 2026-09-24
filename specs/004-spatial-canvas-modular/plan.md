# Implementation Plan: Mesa Espacial e Edição Modular (Canvas & BlockSuite)

**Branch**: `004-spatial-canvas-modular` | **Date**: 2026-09-24 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/004-spatial-canvas-modular/spec.md`

## Summary

Implementar a arquitetura completa da Mesa Espacial com edição modular sob demanda, criação interativa de conexões relacionais (arestas Bézier), persistência canônica dupla (`workspaces/<id>/board.canvas.json` e notas em `workspaces/<id>/cells/<id>.md`) e renderização acelerada por GPU a 60 FPS com Frustum Culling e Níveis de Detalhe (LOD).

## Technical Context

**Language/Version**: Rust 1.80+ (Backend Tauri v2), TypeScript 5.5+ & Svelte 5 (Frontend).  
**Primary Dependencies**: Tauri v2, PixiJS v8 (`pixi.js`), `svelte 5`, `rusqlite`, `serde`, `serde_json`, `tokio`, `uuid`, `chrono`.  
**Storage**: Sistema de arquivos canônico local (`workspaces/<id>/board.canvas.json` e `workspaces/<id>/cells/*.md`), SQLite `index.db` (projeções descartáveis de busca).  
**Testing**: `cargo test` para persistência atômica e IPC de topologia/células; `svelte-check` e `npm run test` para componentes de canvas.  
**Target Platform**: Windows 10/11 Desktop, macOS, Linux (com simulação web dual-mode para `localhost:1420`).  
**Project Type**: Desktop Local-First Analytical Knowledge Environment (Tauri v2).  
**Performance Goals**: 60 FPS contínuos com 1.000 nós no canvas com culling e LOD; Instanciação/desmontagem de editor overlay < 50ms; Latência de re-layout de arestas $\le 4$ ms.  
**Constraints**: Máximo de 1 a 2 editores ativos no overlay DOM simultâneos (Princípio II); Persistência em duas fases livre de corrupção em crashes (Princípio VII); Operação 100% offline (Princípio I).  
**Scale/Scope**: Suporte a dezenas de workspaces isolados, cada um contendo até 1.000 nós e 2.000 arestas.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Princípio I (Soberania do Dado & File-as-Truth)**: A topologia é salva no arquivo canônico `board.canvas.json` e as células em `cells/<id>.md` abertas e legíveis.
- [x] **Princípio II (Separação Cognitiva, Edição Modular & Fork-on-Insert)**: O editor é montado estritamente sob demanda no overlay DOM apenas na célula ativa; nós passivos utilizam previews estáticos; notas do Acervo promovidas mantêm proveniência sem mutação da fonte bruta.
- [x] **Princípio III (Contenção Rigorosa & I/O Anti-TOCTOU)**: Gravações de arquivos e diretórios de workspaces passam por validação de limites no `VaultGuard`.
- [x] **Princípio IV (Núcleo Nativo & Eficiência de Recursos)**: Consumo de memória em repouso $< 350$ MB; 60 FPS com Frustum Culling e LOD ativados.
- [x] **Princípio VII (Resiliência Operacional & Journaling Atômico)**: Gravação atômica em duas fases com OCC (`revision`) e tolerância a quedas $< 500$ ms.

## Project Structure

### Documentation (this feature)

```text
specs/004-spatial-canvas-modular/
├── spec.md              # Especificação funcional de requisitos
├── plan.md              # Este plano técnico de implementação
├── research.md          # Decisões de pesquisa técnica (Phase 0)
├── data-model.md        # Entidades, DTOs e validações (Phase 1)
├── contracts/           # Contratos IPC JSON Schema (Phase 1)
│   └── canvas-ipc.json
├── quickstart.md        # Cenários de validação executáveis (Phase 1)
├── checklists/
│   └── requirements.md  # Checklist de qualidade da especificação
└── tasks.md             # Tarefas de implementação (Phase 2 - via /speckit-tasks)
```

### Source Code

```text
src-tauri/src/
├── domain/
│   └── workspace/
│       ├── mod.rs
│       ├── cell.rs             # Entidade WorkspaceCell (conteúdo, metadados, proveniência)
│       └── topology.rs         # Entidades BoardTopology, CanvasNode, CanvasEdge, ViewportState
├── infra/
│   └── fs/
│       └── canvas_io.rs        # Gravação e leitura atômica de board.canvas.json e cells/*.md
├── ipc/
│   ├── workspace.rs            # Comandos Tauri: load/save topology, save_cell, delete_cell, create_edge
│   └── mod.rs
└── lib.rs

src/
├── components/
│   └── canvas/
│       ├── CanvasViewport.svelte   # Viewport PixiJS, pan, zoom e detecção de culling
│       ├── CanvasControls.svelte   # Botões de zoom, centralizar e criar nós
│       ├── CellOverlay.svelte      # Editor modular sob demanda (BlockSuite/rich text overlay)
│       └── PixiRenderer.ts         # Motor de renderização PixiJS v8, curvas Bézier e nós
├── services/
│   ├── canvasService.ts        # Chamadas IPC tipadas para o backend Tauri
│   └── ipc.ts                  # Mock handlers expandidos para topologia e células no browser
├── stores/
│   └── canvas.svelte.ts        # Store Svelte 5 com gerenciamento de estado espacial e OCC
└── types/
    └── canvas.ts               # Tipos TypeScript: CanvasNodeDTO, CanvasEdgeDTO, BoardTopologyDTO
```

## Complexity Tracking

> Nenhuma violação aos princípios constitucionais foi identificada; dispensada justificativa de exceção.

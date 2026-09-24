# Implementation Plan: Peça Editorial e Proveniência Canônica (Fork-on-Insert Estendido)

**Branch**: `005-editorial-piece-provenance` | **Date**: 2026-09-24 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/005-editorial-piece-provenance/spec.md`

---

## Summary

Implementar a camada editorial de documentos longos ("Peças" em `workspaces/<workspace_id>/pieces/<piece_id>.md`) com a mecânica de Fork-on-Insert estendido a partir dos cartões da Mesa Espacial, conferindo rastreabilidade canônica via frontmatter YAML (`citations`), imunidade irrestrita a mutações retroativas automáticas, detecção determinística de divergência (*drift detection*), visualização dividida (Split-View) e exportação limpa com apêndice bibliográfico.

---

## Technical Context

**Language/Version**: Rust 2021 (Backend nativo), TypeScript 5.6+ / Svelte 5 com Runes (Frontend Webview)  
**Primary Dependencies**: Tauri v2, PixiJS v8 (viewport resize), Svelte 5, serde / serde_json, chrono, uuid, rusqlite  
**Storage**: Sistema de arquivos local canônico em `workspaces/<id>/pieces/<id>.md` (File-as-Truth), com gravação atômica via `FlushFileBuffers` / `fsync` e `VaultGuard`  
**Testing**: `cargo test --no-default-features --tests` (Rust), `svelte-check`, `npm run build`  
**Target Platform**: Desktop (Windows, macOS, Linux via Tauri v2) + Suporte a navegador web puro (`localhost:1420`) via mock reativo  
**Project Type**: Aplicação Desktop Local-First com Webview acelerada por hardware  
**Performance Goals**: Autosave com debounce de 400ms, inserção via Fork-on-Insert < 150ms, alternância de Split-View instantânea sem reinicializar o canvas PixiJS  
**Constraints**: Consumo de memória em repouso $\le 350\text{ MB}$ (Princípio IV), tolerância a quedas com janela máxima de 500ms (Princípio VII), zero chamadas de rede para verificação de citações  
**Scale/Scope**: Múltiplas peças por workspace, documentos de até 50.000 palavras, dezenas de citações estruturadas por peça  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Princípio Constitucional | Verificação de Conformidade | Status |
| :--- | :--- | :---: |
| **I. Soberania do Dado e File-as-Truth** | Peças são arquivos físicos individuais Markdown (`.md`) com frontmatter YAML canônico. Nenhuma dependência de nuvem. | **✓ PASS** |
| **II. Separação Cognitiva & Fork-on-Insert Estendido** | Conteúdo da Mesa é clonado com independência autoral. Edições no canvas NUNCA modificam a Peça retroativamente sem confirmação. | **✓ PASS** |
| **III. Contenção Rigorosa & Anti-TOCTOU** | Toda leitura/escrita de peças passa por mediação central no `VaultGuard` garantindo confinamento ao workspace. | **✓ PASS** |
| **IV. Núcleo Nativo & Eficiência de Recursos** | I/O e auditoria em Rust nativo. Interface leve em Svelte 5 sem múltiplos editores pesados simultâneos; memória $\le 350\text{ MB}$. | **✓ PASS** |
| **VII. Resiliência Operacional & Journaling Atômico** | Escritas atômicas com `fsync` antes do rename no SO. Janela máxima de perda de digitação $\le 500\text{ ms}$. | **✓ PASS** |

---

## Project Structure

### Documentation (this feature)

```text
specs/005-editorial-piece-provenance/
├── spec.md              # Especificação de requisitos e estórias de usuário
├── checklists/          # Checklist de qualidade (requirements.md)
├── plan.md              # Este plano técnico de arquitetura
├── research.md          # Decisões técnicas e fundamentações
├── data-model.md        # Esquemas de dados, entidades e invariantes
├── quickstart.md        # Roteiro de validação prática e testes
├── contracts/           # Contratos formais de IPC (pieces-ipc.json)
└── tasks.md             # Decomposição de tarefas (/speckit-tasks)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── domain/workspace/
│   │   ├── mod.rs
│   │   ├── cell.rs                  # WorkspaceCell canônica
│   │   ├── piece.rs                 # [NOVO] EditorialPiece e CitationRecord
│   │   └── topology.rs              # BoardTopology e CanvasNode
│   ├── infra/fs/
│   │   ├── canvas_io.rs             # I/O do canvas e células
│   │   ├── piece_io.rs              # [NOVO] I/O atômico para pieces/*.md e drift detection
│   │   └── vault.rs                 # ensure_workspace_structure com pasta pieces/
│   ├── ipc/
│   │   ├── workspace.rs             # Endpoints Tauri para peças e auditoria
│   │   └── mod.rs
│   └── lib.rs                       # Registro de handlers Tauri
tests/
├── piece_markdown_tests.rs          # [NOVO] Teste de round-trip e frontmatter YAML
└── piece_drift_tests.rs             # [NOVO] Teste determinístico de divergência (drift)

src/
├── types/
│   └── piece.ts                     # [NOVO] DTOs de EditorialPiece, Citation e Drift
├── services/
│   ├── pieceService.ts              # [NOVO] Cliente IPC tipado para peças
│   └── ipc.ts                       # Mocks de navegador para localhost:1420
├── stores/
│   ├── piece.svelte.ts              # [NOVO] Store Svelte 5 para peça ativa e histórico
│   └── canvas.svelte.ts             # Suporte a Split-View e seleção de nó para citação
└── components/
    ├── editorial/
    │   ├── PieceEditor.svelte       # [NOVO] Editor estruturado de texto longo
    │   ├── CitationBadge.svelte     # [NOVO] Badge visual de proveniência com drift
    │   ├── DriftInspector.svelte    # [NOVO] Gaveta comparativa de versões
    │   └── PieceSidebar.svelte      # [NOVO] Lista e gerenciamento de peças do workspace
    └── canvas/
        └── CanvasViewport.svelte    # Suporte a Split-View lado a lado
```

---

## Complexity Tracking

*Nenhuma violação constitucional identificada. A arquitetura estende de forma limpa os padrões de I/O atômico e `VaultGuard` já consolidados nos sprints anteriores.*

# Data Model: Mesa Espacial e Edição Modular

**Feature**: `004-spatial-canvas-modular`  
**Date**: 2026-09-24  
**Status**: Approved  

## Entities & Schemas

### 1. BoardTopology (`workspaces/<workspace_id>/board.canvas.json`)

Representação canônica da cena espacial e relacionamentos do workspace.

| Campo | Tipo | Descrição | Regras de Validação |
| :--- | :--- | :--- | :--- |
| `workspaceId` | `String` | Identificador único do workspace (ex: `default-workspace`) | Não vazio, caracteres alfanuméricos e hífens |
| `viewport` | `ViewportState` | Estado atual da câmera virtual | Zoom entre `0.1` e `3.0` |
| `nodes` | `Vec<CanvasNode>` | Coleção de nós e cartões instanciados | IDs únicos por workspace |
| `edges` | `Vec<CanvasEdge>` | Coleção de arestas relacionais entre nós | Sem auto-loops (`source != target`) |
| `revision` | `u64` | Contador sequencial para controle de concorrência otimista (OCC) | Incrementado a cada commit |
| `updatedAt` | `i64` | Timestamp Unix em segundos do último salvamento | Valor positivo |

```json
{
  "workspaceId": "default-workspace",
  "viewport": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
  "nodes": [
    {
      "id": "node-101",
      "itemId": "ingest-uuid-1",
      "localCellPath": "cells/cell-101.md",
      "title": "Arquitetura Local-First",
      "content": "Resumo dos princípios de soberania do dado...",
      "x": 240.0,
      "y": 180.0,
      "width": 280.0,
      "height": 160.0,
      "colorPreset": "blue",
      "nodeType": "note"
    }
  ],
  "edges": [
    {
      "id": "edge-1",
      "sourceNodeId": "node-101",
      "targetNodeId": "node-102",
      "fromSide": "Right",
      "toSide": "Left",
      "label": "fundamenta",
      "directed": true
    }
  ],
  "revision": 4,
  "updatedAt": 1727180000
}
```

---

### 2. CanvasNode

Representa a projeção visual de um cartão na Mesa Espacial gerenciada pelo PixiJS.

| Campo | Tipo | Descrição | Validação |
| :--- | :--- | :--- | :--- |
| `id` | `String` | ID do nó no canvas | Prefixo `node-`, obrigatório e único |
| `itemId` | `Option<String>` | ID do item de origem no Acervo (se promovido via Fork-on-Insert) | UUID opcional |
| `localCellPath` | `Option<String>` | Caminho relativo para a nota canônica da célula (`cells/<id>.md`) | Caminho relativo sanitizado |
| `title` | `String` | Título do cartão exibido no cabeçalho | Máximo 255 caracteres |
| `content` | `String` | Trecho inicial ou corpo Markdown da célula | Texto UTF-8 válido |
| `x` | `f32` | Posição horizontal no plano infinito | Valor numérico finito |
| `y` | `f32` | Posição vertical no plano infinito | Valor numérico finito |
| `width` | `f32` | Largura do retângulo na tela | Mínimo 180px, padrão 280px |
| `height` | `f32` | Altura do retângulo na tela | Mínimo 120px, padrão 160px |
| `colorPreset` | `Option<String>` | Esquema de cores (`blue`, `emerald`, `amber`, `purple`, `rose`) | Validação por enum |
| `nodeType` | `String` | Tipo do nó (`note`, `asset`, `text`) | Padrão `note` |

---

### 3. CanvasEdge

Representa a aresta conectando dois cartões através de alças de ancoragem.

| Campo | Tipo | Descrição | Validação |
| :--- | :--- | :--- | :--- |
| `id` | `String` | ID único da aresta | Prefixo `edge-` |
| `sourceNodeId` | `String` | ID do nó de origem da conexão | Deve existir em `nodes` |
| `targetNodeId` | `String` | ID do nó de destino da conexão | Deve existir em `nodes`; `source != target` |
| `fromSide` | `NodeSide` | Borda de ancoragem de saída (`Top`, `Bottom`, `Left`, `Right`) | Enumeração estrita |
| `toSide` | `NodeSide` | Borda de ancoragem de entrada (`Top`, `Bottom`, `Left`, `Right`) | Enumeração estrita |
| `label` | `Option<String>` | Rótulo semântico sobre a linha | Opcional, máximo 64 caracteres |
| `directed` | `bool` | Indicador se a aresta possui seta de direção | Padrão `true` |

---

### 4. WorkspaceCell (`workspaces/<id>/cells/<cell_id>.md`)

Persistência física da nota individual associada a um nó no canvas (File-as-Truth).

```markdown
---
id: "cell-101"
workspace_id: "default-workspace"
node_id: "node-101"
item_id: "ingest-uuid-1"
source_path: "ingest/notes/pesquisa.md"
source_hash: "a1b2c3d4..."
revision: 2
created_at: 1727180000
updated_at: 1727180400
---

# Título da Célula

Conteúdo da nota editado modularmente dentro da Mesa Espacial.
```

---

## State Transitions (Editor Modular sob Demanda)

```
[Repouso na GPU (PixiJS)]
         │
         │ Duplo clique ou Tecla Enter no nó
         ▼
[Montagem do Overlay DOM] ─── (Cria container HTML sobre as coordenadas de tela do nó)
         │
         │ Digitação pelo usuário
         ▼
[Edição Ativa no DOM] ─── (Autosave em buffer com debounce de 400ms)
         │
         │ Blur (clique fora) ou Tecla Esc
         ▼
[Desmontagem do Editor] ─── (Salva cells/<id>.md + board.canvas.json)
         │
         ▼
[Atualização de Textura no PixiJS]
```

## Validation Rules

1. **Rejeição de Auto-loops**: `sourceNodeId` NUNCA pode ser igual a `targetNodeId`.
2. **Exclusão em Cascata de Arestas**: Ao remover um `CanvasNode`, todas as arestas com `sourceNodeId == id || targetNodeId == id` são removidas imediatamente.
3. **Controle de Concorrência Otimista (OCC)**: O salvamento de `board.canvas.json` verifica se a revisão atual do arquivo em disco coincide com a revisão em memória. Se houver divergência, retorna erro `RevisionConflict`.

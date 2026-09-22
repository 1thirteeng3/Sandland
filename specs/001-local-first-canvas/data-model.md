# Data Model & Storage Specifications: Núcleo Local-First e Canvas Básico (v0.1)

**Feature**: `001-local-first-canvas`  
**Date**: 2026-09-21 (Revisão Técnica Canônica: Integridade Relacional, Vec0 e Anti-Patterns Eliminados)  
**Status**: Ready & Approved  

---

## 1. O Modelo "File-as-Truth" (Persistência Primária em Disco)

O sistema de arquivos físico é a fonte canônica da verdade. Todos os documentos ingeridos são salvos como arquivos Markdown enriquecidos com YAML Frontmatter estritamente intrínseco.

### 1.1. Esquema do Frontmatter YAML Canônico (Sem Redundância Extrínseca)

> **Regra Arquitetural**: Metadados extrínsecos ao arquivo (como `vault_path` físico e `content_hash`) foram **eliminados** do Frontmatter. O `id` (UUIDv7) é a âncora imutável; o SQLite mapeia dinamicamente `id -> vault_path` através dos eventos do file watcher, garantindo mobilidade no sistema de arquivos sem quebra de integridade.

```yaml
---
id: "019213a8-7b2c-7000-8000-000000000001"
title: "Princípios de Gerenciamento de Memória em Rust"
item_type: "note"
category: "engenharia-de-sistemas"
tags:
  - "rust"
  - "memory-safety"
  - "borrow-checker"
summary: "Análise técnica sobre alocação estática na stack vs heap e ciclo de vida de ponteiros."
state: "Classified"
needs_manual_review: false
read_only: true
created_at: 1726916400
updated_at: 1726916400
---

# Princípios de Gerenciamento de Memória em Rust

Corpo completo do documento em Markdown legível...
```

---

## 2. Máquina de Estados do Ingest (*State Machine*)

Para garantir que o agente e o backend operem com transições determinísticas sem funções lineares caóticas:

```
[ Ingestão Disparada ]
         │
         ▼
     [ PENDING ] ────────( Falha de Leitura )───────► [ FAILED ]
         │
         ▼
   [ EXTRACTING ] (Leitura, hash SHA-256 e Frontmatter inicial)
         │
         ▼
   [ CLASSIFYING ] (Inferência SLM via llama.cpp + GBNF)
         │
         ├───( Sucesso GBNF )───────────────────────► [ CLASSIFIED ]
         │
         ├───( Fallback: TF-IDF + Cosseno )─────────► [ CLASSIFIED (fallback) ]
         │
         └───( Timeout / Erro Persistente )─────────► [ NEEDS_MANUAL_REVIEW ]
```

### Regras de Transição e Prazos:
- **Timeout por Estado**: Máximo de 15 segundos para `Classifying`.
- **Tentativas de Repetição**: No máximo 2 tentativas com temperatura 0.0 antes de degradar para `NEEDS_MANUAL_REVIEW`.
- **Persistência**: Toda mudança de estado grava no Frontmatter do arquivo `.md` e atualiza a coluna `state` no `index.db`, emitindo o evento `ingest://state-changed`.

---

## 3. Esquema SQLite com Integridade Relacional e Suporte Estrito ao `sqlite-vec`

Localizado em `.system/index.db`. Gerenciado via crate `refinery` sob `src-tauri/migrations/`.

### 3.1. DDL Refatorado (`V1__initial_schema.sql`)

```sql
PRAGMA foreign_keys = ON;

-- Metadados de Itens (Tabela Canônica de Registro)
CREATE TABLE IF NOT EXISTS items (
    rowid_item INTEGER PRIMARY KEY AUTOINCREMENT, -- Mapeador inteiro de 64-bit para vec_items
    id TEXT NOT NULL UNIQUE,                       -- UUIDv7 do documento
    vault_path TEXT NOT NULL UNIQUE,              -- Localização atual em disco (mapeada dinamicamente)
    item_type TEXT NOT NULL,                      -- 'note', 'web_snapshot', 'media_transcript'
    title TEXT NOT NULL,
    content_hash TEXT NOT NULL,                   -- SHA-256 gerenciado pelo indexador Rust
    summary TEXT,
    category TEXT,
    state TEXT NOT NULL DEFAULT 'Pending',        -- 'Pending', 'Extracting', 'Classifying', 'Classified', 'NeedsManualReview', 'Failed'
    needs_manual_review BOOLEAN DEFAULT 0,
    read_only BOOLEAN DEFAULT 1,
    created_at INTEGER NOT NULL,                  -- Unix Epoch (segundos)
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_items_path ON items(vault_path);
CREATE INDEX IF NOT EXISTS idx_items_category ON items(category);

-- Taxonomia e Relacionamentos N:N (Elimina Tags Órfãs)
CREATE TABLE IF NOT EXISTS taxonomy_terms (
    term TEXT PRIMARY KEY,
    term_type TEXT CHECK(term_type IN ('category', 'tag')),
    frequency INTEGER DEFAULT 1,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS item_taxonomy (
    item_id TEXT REFERENCES items(id) ON DELETE CASCADE,
    term TEXT REFERENCES taxonomy_terms(term) ON DELETE CASCADE,
    PRIMARY KEY (item_id, term)
);

CREATE INDEX IF NOT EXISTS idx_item_taxonomy_term ON item_taxonomy(term);

-- Busca Textual FTS5 (Gerenciada atomicamente via transações pelo Rust Worker)
CREATE VIRTUAL TABLE IF NOT EXISTS items_fts USING fts5(
    item_id UNINDEXED,
    title,
    content,
    tokenize="unicode61 remove_diacritics 1"
);

-- Tabela Vetorial Integrada (Mapeada estritamente via rowid inteiro de 64-bit)
-- Dimensão: 384 (modelo multilíngue paraphrase-multilingual-MiniLM-L12-v2 via fastembed)
CREATE VIRTUAL TABLE IF NOT EXISTS vec_items USING vec0(
    rowid INTEGER PRIMARY KEY,
    embedding FLOAT[384]
);
```

### 3.2. Consulta Canônica de Busca Híbrida (RRF com $k = 60$)

```sql
WITH fts_results AS (
    SELECT item_id as id, ROW_NUMBER() OVER(ORDER BY rank) as rank_fts
    FROM items_fts
    WHERE items_fts MATCH :query
    LIMIT 100
),
vec_results AS (
    SELECT i.id, ROW_NUMBER() OVER(ORDER BY v.distance) as rank_vec
    FROM vec_items v
    JOIN items i ON i.rowid_item = v.rowid
    WHERE v.embedding MATCH :query_vector AND v.distance < 0.85
    ORDER BY v.distance ASC
    LIMIT 100
)
SELECT 
    i.id, 
    i.title, 
    i.vault_path,
    i.category,
    i.summary,
    COALESCE(f.rank_fts, 999) as r_fts,
    COALESCE(v.rank_vec, 999) as r_vec,
    (
      CASE WHEN f.rank_fts IS NOT NULL THEN 1.0 / (60.0 + f.rank_fts) ELSE 0.0 END +
      CASE WHEN v.rank_vec IS NOT NULL THEN 1.0 / (60.0 + v.rank_vec) ELSE 0.0 END
    ) as rrf_score
FROM items i
LEFT JOIN fts_results f ON i.id = f.id
LEFT JOIN vec_results v ON i.id = v.id
WHERE f.id IS NOT NULL OR v.id IS NOT NULL
ORDER BY rrf_score DESC
LIMIT :limit;
```

---

## 4. Topologia Espacial do Canvas (`BoardTopology`)

Armazenada sob `/vault/workspaces/{workspace_id}/` em dois arquivos sincronizados:
- `board.canvas.mpk`: Serialização binária compacta para gravação instantânea (350ms).
- `board.canvas.json`: Serialização JSON formatada para controle de versão em repouso (> 2s idle).

> **Otimização Crítica de Cache**: `CanvasNode` **não duplica** título, tags ou resumo do documento. Ele contém estritamente metadados espaciais de instância e o ponteiro relacional (`item_id` ou `local_cell_path`). Os dados textuais são hidratados pela UI a partir do cache local/SQLite no momento da renderização.

### 4.1. Estrutura TypeScript da Topologia

```typescript
export type NodeSide = 'left' | 'right' | 'top' | 'bottom';

export interface CanvasNode {
  id: string;               // Identificador espacial único (ex: "node_01j8m9...")
  item_id?: string;         // Referência relacional ao documento no index.db
  local_cell_path?: string; // Caminho se for uma Célula autoral direta do workspace
  x: number;                // Coordenada espacial X
  y: number;                // Coordenada espacial Y
  width: number;            // Largura visual (padrão: 320px)
  height: number;           // Altura calculada ou fixa
  color_preset?: string;    // Tema de cor para identificação de categoria
}

export interface CanvasEdge {
  id: string;               // Identificador único da conexão (ex: "edge_01j8m9...")
  source_node_id: string;
  target_node_id: string;
  from_side: NodeSide;      // Âncora de saída (docking anchor)
  to_side: NodeSide;        // Âncora de entrada (docking anchor)
  label?: string;
  directed: boolean;        // default: true
}

export interface BoardTopology {
  workspace_id: string;
  version: number;
  viewport: {
    x: number;              // Centro da câmera X
    y: number;              // Centro da câmera Y
    zoom: number;           // Fator de escala
  };
  nodes: CanvasNode[];
  edges: CanvasEdge[];
  updated_at: number;
}
```

---

## 5. Log de Eventos e Auditoria Local (`.system/audit_log.db`)

Garante a recuperação do estado operacional com janela de perda menor que 500 milissegundos, sem gerar degradação de I/O em arrastes de tela.

> **Regra de Desempenho**: O evento `NODE_MOVED` **NUNCA** é emitido durante o arraste a 60 FPS. Ele é registrado exclusivamente no término do movimento (`drag_end`) com debouncing. Micro-mudanças contínuas pertencem unicamente ao arquivo binário temporário `board.canvas.mpk`. O `audit_log.db` concentra-se em mutações semânticas e transações de negócio.

```sql
CREATE TABLE IF NOT EXISTS audit_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,      -- Unix Epoch em milissegundos
    event_type TEXT NOT NULL,         -- 'ITEM_INGESTED', 'TAXONOMY_TAGGED', 'NODE_CREATED', 'NODE_DRAG_END', 'EDGE_CREATED', 'ITEM_MUTATED'
    entity_id TEXT NOT NULL,
    payload_json TEXT NOT NULL,       -- Estado antes/depois da mutação estrutural
    reversible BOOLEAN DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_events(timestamp DESC);
```

---

## 6. Separação Estrutural de Motores de Conhecimento

O Sandland estabelece uma divisão estrita entre seus dois subsistemas de recuperação de informação:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                        COEXISTÊNCIA DE MOTORES DE CONHECIMENTO                         │
├───────────────────────────────────────────┬────────────────────────────────────────────┤
│ 1. ACERVO DE DOCUMENTOS BRUTOS (INGEST)   │ 2. MESA DE PESQUISA (CANVAS / BOARDS)      │
├───────────────────────────────────────────┼────────────────────────────────────────────┤
│ • Pipeline Híbrido: Vetores + BM25 Lexical│ • Paradigma Vectorless RAG (PageIndex)     │
│ • SQLite `sqlite-vec` (FastEmbed 384d)    │ • Zero vetorização ou chunking cego na mesa│
│ • SQLite `fts5` com tokenizador unicode61 │ • Contexto preservado por topologia pura   │
│ • Reciprocal Rank Fusion (RRF k=60)       │ • Nós, grupos e arestas em `topology.json` │
│ • Foco: Notas longas, PDFs, Web Clippings │ • Foco: Ideias, relações causais, sínteses │
└───────────────────────────────────────────┴────────────────────────────────────────────┘
```

---

## 7. Ciclo de Vida de Células no Canvas: Política Anti-Poluição de Disco

1. **Estado Volátil / Estruturado (Canvas Node):**
   - As células criadas na mesa de pesquisa vivem unicamente como entradas no array `nodes` do arquivo `topology.json`.
   - **Nenhum arquivo `.md` físico é criado automaticamente no cofre** durante a criação, edição ou manipulação de células na mesa, evitando a proliferação de micro-arquivos temporários.
2. **Promoção Explícita a Nota Canônica:**
   - A criação de uma nota física no cofre ocorre unicamente mediante a ação deliberada do usuário ("Promover Célula a Nota").
   - Quando disparada, o backend Rust grava `ingest/notes/<slug>-<uuid>.md` com Frontmatter canônico, indexa o documento no `index.db` e vincula o `source_item_id` da nota à célula no `topology.json`.

---

## 8. Especificação do Skeleton Map (Injeção de Contexto RLM)

Em tempo de execução, o backend sintetiza o grafo visual do board ativo em um mapa hierárquico em árvore (**Skeleton Map**), injetável na janela de contexto de modelos locais ou em nuvem:

```text
=== WORKSPACE: <workspace_id> ===
[GRUPO: <group_title>]
  • [NÓ: <node_id>] "<node_title>"
    Síntese: <content_resumo>
    Conexões:
      ──(<relation_type>)──► [<target_node_id>] "<target_title>"
[CÉLULAS INDEPENDENTES]
  • [NÓ: <node_id>] "<node_title>"
    Síntese: <content_resumo>
```

- **Economia de Tokens:** Reduz um canvas complexo a uma faixa de 800 a 2.000 tokens estruturados.
- **PageIndex Tool Calling:** Se o modelo necessitar do conteúdo integral de um nó citado, aciona a ferramenta `read_cell(node_id)`.

---

## 9. Invariante de Portabilidade Multiplataforma

- **Proibição Estrita de Caminhos Hardcoded:** É expressamente proibido codificar caminhos estáticos de pastas de usuário ou de sistema operacional no código-fonte (`src/` e `src-tauri/`) ou em scripts de build.
- **Resolução Dinâmica:** Toda localização de cofre, cache ou executáveis auxiliares DEVE utilizar resolução dinâmica via `app.path().home_dir()`, `app.path().document_dir()` ou `app.path().app_data_dir()`.
- **Compatibilidade Windows UNC:** Verificações de sandbox no `VaultGuard` devem tratar uniformemente caminhos canônicos locais e com prefixo estendido `\\?\`.

# Data Model: Pipeline de Ingestão e Processamento de Documentos

**Feature**: `003-ingestion-pipeline`  
**Date**: 2026-09-22

## Relational Schema (`.system/index.db`)

### Tabela `ingested_items`

Armazena as projeções relacionais dos itens ingeridos no cofre.

```sql
CREATE TABLE IF NOT EXISTS ingested_items (
    id TEXT PRIMARY KEY,                       -- UUID v4
    source_type TEXT NOT NULL,                 -- 'file' | 'url' | 'text'
    source_path TEXT NOT NULL,                 -- Caminho relativo dentro do cofre ou URL original
    canonical_uri TEXT NOT NULL,               -- URI relativa do arquivo canônico (ex: 'ingest/notes/pesquisa.md')
    title TEXT NOT NULL,                       -- Título do documento
    summary TEXT,                              -- Resumo textual ou snippet inicial
    content_hash TEXT NOT NULL,                -- Hash SHA-256 do conteúdo textual
    word_count INTEGER NOT NULL DEFAULT 0,     -- Contagem de palavras
    tags TEXT,                                 -- JSON Array de tags (ex: '["pesquisa", "local-first"]')
    status TEXT NOT NULL DEFAULT 'Ingested',   -- 'Ingested' | 'Classified' | 'Promoted'
    ingested_at INTEGER NOT NULL,              -- UNIX Timestamp em segundos
    updated_at INTEGER NOT NULL                -- UNIX Timestamp em segundos
);

CREATE INDEX IF NOT EXISTS idx_ingested_items_source_type ON ingested_items(source_type);
CREATE INDEX IF NOT EXISTS idx_ingested_items_ingested_at ON ingested_items(ingested_at DESC);
```

### Tabela Virtual FTS5 `ingested_items_fts`

```sql
CREATE VIRTUAL TABLE IF NOT EXISTS ingested_items_fts USING fts5(
    title,
    summary,
    content='ingested_items',
    content_rowid='rowid',
    tokenize='unicode61'
);
```

---

## File Layout on Disk

```text
<vault_root>/
├── ingest/
│   ├── notes/
│   │   ├── pesquisa-mestrado.md
│   │   └── notas-reuniao.md
│   └── web/
│       └── 20260922-artigo-local-first.md
├── assets/
│   ├── 5f81a3d9...png
│   └── e3b0c442...svg
└── .system/
    └── index.db
```

---

## DTOs & Rust Types

### `IngestedItemDTO`
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestedItemDTO {
    pub id: String,
    pub source_type: String,
    pub source_path: String,
    pub canonical_uri: String,
    pub title: String,
    pub summary: Option<String>,
    pub word_count: usize,
    pub tags: Vec<String>,
    pub status: String,
    pub ingested_at: i64,
}
```

### `PromoteToCellRequest`
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromoteToCellRequest {
    pub workspace_id: String,
    pub item_id: String,
    pub position_x: f64,
    pub position_y: f64,
}
```

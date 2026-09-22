-- V1: Initial Schema for Sandland Vault Index
PRAGMA foreign_keys = ON;

-- Metadados de Itens (Tabela Canônica de Registro)
CREATE TABLE IF NOT EXISTS items (
    rowid_item INTEGER PRIMARY KEY AUTOINCREMENT, -- Mapeador inteiro de 64-bit para vec_items
    id TEXT NOT NULL UNIQUE,                       -- UUIDv7 do documento
    vault_path TEXT NOT NULL UNIQUE,              -- Localização atual em disco (mapeada dinamicamente)
    item_type TEXT NOT NULL,                      -- 'note', 'web_snapshot', 'media_transcript'
    title TEXT NOT NULL,
    content_hash TEXT NOT NULL,                   -- SHA-256 gerenciado pelo indexador
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
CREATE INDEX IF NOT EXISTS idx_items_state ON items(state);

-- Taxonomia e Relacionamentos N:N
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

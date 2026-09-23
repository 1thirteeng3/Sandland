-- V3: Ingestion Pipeline Schema for Feature 003
ALTER TABLE items ADD COLUMN word_count INTEGER DEFAULT 0;
ALTER TABLE items ADD COLUMN canonical_uri TEXT;
ALTER TABLE items ADD COLUMN source_path TEXT;

CREATE INDEX IF NOT EXISTS idx_items_canonical_uri ON items(canonical_uri);

-- View para compatibilidade canônica com o contrato de Ingestão
CREATE VIEW IF NOT EXISTS ingested_items AS
SELECT
    id,
    COALESCE(item_type, 'note') as source_type,
    COALESCE(source_path, vault_path) as source_path,
    COALESCE(canonical_uri, vault_path) as canonical_uri,
    title,
    summary,
    content_hash,
    COALESCE(word_count, 0) as word_count,
    state as status,
    created_at as ingested_at,
    updated_at
FROM items;

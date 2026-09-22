-- V2: Audit Events Table for Crash Recovery & Rollback
CREATE TABLE IF NOT EXISTS audit_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,      -- Unix Epoch em milissegundos
    event_type TEXT NOT NULL,         -- 'ITEM_INGESTED', 'TAXONOMY_TAGGED', 'NODE_CREATED', 'NODE_DRAG_END', 'EDGE_CREATED', 'ITEM_MUTATED'
    entity_id TEXT NOT NULL,
    payload_json TEXT NOT NULL,       -- Estado antes/depois da mutação estrutural
    reversible BOOLEAN DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_events(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_audit_entity ON audit_events(entity_id);

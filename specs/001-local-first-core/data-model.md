# Phase 1 Data Model: SANDLAND v0.1 Local-First Core

**Date**: 2026-09-21 | **Plan**: [plan.md](./plan.md) | **Research**: [research.md](./research.md)

Two stores, two lifecycles: the **vault** (user content, authoritative, immutable-by-default) and
**`.system/`** (derived index, disposable; durable audit log, forward-only migrations). Every entity below
states which store owns it — that ownership is the enforceable reading of constitution §I.2.

## 1. Vault on Disk (File-as-Truth)

```text
<vault>/
├── .system/                        # derived + durable runtime state; deletable except audit_log.db
│   ├── index.db                    # SQLite: items, items_fts (external content), vec_items, terms  [DISPOSABLE]
│   ├── audit_log.db                # SQLite: reversible event history                                [DURABLE]
│   ├── taxonomy_cache.json         # term vector cache for cosine merge/alias                        [DISPOSABLE]
│   ├── sandbox/                    # generated per-task profiles (Seatbelt .sb), mode 0600           [EPHEMERAL]
│   └── budgets.lock                # measured budget baselines written by the perf harness           [DISPOSABLE]
├── intentions/
│   └── intentions.json             # teleology layer; advisory only in v0.1 (RFC §9)
├── assets/                         # CAS: content-addressed binaries, no subdirectories
│   └── <sha256-hex-lowercase>      # e.g. 4f8a...3e ; extension-less; never rewritten in place
├── ingest/                         # APPEND-ONLY lake; body immutable after write
│   ├── web/20260921T101500Z-transformers.md
│   ├── media/20260921T090000Z-podcast-ep12.md
│   └── notes/20260921T080012Z-pensamento-solto.md
└── workspaces/
    └── <workspace_id>/             # one isolated sandbox per workspace
        ├── workspace.toml          # title, created_at, confinement record (path digest)
        ├── board.canvas.mpk        # authoritative topology (MessagePack)
        ├── board.canvas.json       # canonical mirror for git/human review (idle-only writes)
        ├── cells/<cell_id>.md      # atomic note, own document
        └── scratchpad/             # volatile area, index-excluded, wipeable
```

### 1.1 Path and naming rules (validation)

| Rule | Constraint | Failure |
| --- | --- | --- |
| Identifier charset | `[a-z0-9_-]{1,64}` per segment, no leading `.`, no `..`, no NUL | Reject `InvalidSegment` |
| Encoded characters | `percent/HTML entity`-free; NFC Unicode normalization for display names | Reject `InvalidSegment` |
| Case sensitivity | Ids lowercase; on case-insensitive volumes `vault_open` warns and enforces unique-lowercase | Warn `VaultUnsafeFilesystem` |
| Asset name | exactly 64 lowercase hex chars, content-derived | Orphan detected by fsck → GC candidate, never auto-deleted |
| Item filename | `<YYYYMMDDTHHMMSSZ>-<slug>.md` under its layer dir; slug ≤ 48 chars | Backend rewrites the name; caller never chooses paths (D5) |
| Scratchpad | excluded from index, not watched, deletable without data loss | Warn once, then silent |
| Vault root | must be a real directory, not a symlink; `st_dev` recorded | Reject `InvalidBase` |

`workspace_id` and all ids are **ULID-style** (Crockford base32, monotonic within a process, e.g.
`01j8k9m3q7t1vx`), sorted-by-construction so file order equals creation order and mirrors diff cleanly.

### 1.2 Frontmatter schemas (YAML, flat + one nested list)

Item — `ingest/**.md` (only `taxonomy`, `review`, and `updated_at` may be appended post-capture, D6):

```yaml
---
schema: sandland.item/1
id: 01j8k9m3q7t1vx
layer: notes                    # notes | web | media
title: "Pensamento solto"
created_at: 2026-09-21T08:00:12Z
updated_at: 2026-09-21T08:00:12Z      # only rewritten by metadata appends
content_hash: sha256:<64hex>          # body only, excludes frontmatter
source_uri: null                      # set for layer=web
assets: [ "sha256:4f8a...", "sha256:9b1c..." ]
read_only: false                      # true ⇒ UI edit affordances disabled
taxonomy_v: 1                         # bumped by classification writes
category: { term: "systems", confidence: 0.91, is_new: false }
tags:
  - { term: "rust", confidence: 0.97, is_new: false }
  - { term: "memory-management", confidence: 0.88, is_new: true,
      justification_if_new: "no existing term covered reclamation timing" }
classification:
  stage: llm_gbnf                     # llm_gbnf | llm_retry_temp0 | heuristic | manual
  model: qwen2.5-1.5b-instruct-q4_k_m
  grammar_sha: sha256:<12hex>
  attempts: 1
  duration_ms: 1834
  error: null
needs_manual_review: false            # ladder terminus; UI indicator source
---
```

Cell — `workspaces/<ws>/cells/<id>.md` (mutable by the user; body may change, provenance is append-only):

```yaml
---
schema: sandland.cell/1
id: 01j8k9m3q8w2vy
workspace: 01j8k9m3q7t1vx
title: "Atenção seletiva em redes neurais"
kind: note                            # note | media | link | code
created_at: 2026-09-21T10:15:00Z
updated_at: 2026-09-21T10:30:00Z
content_hash: sha256:<64hex>
board_ref: { x: 412.5, y: 96.0, w: 280, h: 168, z: 3, group: "g_arch" }  # geometry lives in board, mirrored
forked_from: null                     # { cell_id, piece_id } when this cell came from a piece (v0.2)
tags: [ "attention", "neuroscience" ]
---
```

`schema:` is a required version tag. Unknown major schema ⇒ vault opens read-only for that item with an audit
event, never a silent rewrite (§I.5).

### 1.3 Asset CAS

`assets/<sha256>` holds bytes exactly as captured; the hash is computed streaming at 1 MiB blocks (same shape
as the RFC's integrity reader). Dedup is by digest, refcounting is **derived** (counted from item frontmatter
at reindex) and never stored in the vault. GC is a manual, opt-in `vault_fsck` that lists orphans; deleting is
a user action logged to the audit log (§V.5, and §I's rule that no automated path may destroy content).

## 2. Entity Model (logical)

| Entity | Owner store | Key | Relationships |
| --- | --- | --- | --- |
| Item | vault (file) | `id`, unique `vault_path` | 0..n → Asset (by digest); 1 → TaxonomyTerm category; 0..n ↔ TaxonomyTerm tags; 1 → AuditEvent trail |
| Asset | vault (file, CAS) | `sha256` | n ↔ Item; refcount derived |
| Workspace | vault (dir + `workspace.toml`) | `id` | 1 → Board; 1..n → Cell; defines the confinement boundary |
| Board | vault (`board.canvas.mpk`) | `workspace_id` | 1..n → Node (cell or link or group proxy); positions are topology, not cell content |
| Node | board file | `node_id` | → Cell (if content-bearing); 0..n → Edge |
| Edge | board file | `src`,`dst`,`kind` | link / group / reference |
| Cell | vault (file) | `id` | 1 → Workspace; mirrored as Node |
| TaxonomyTerm | vault (aggregated via frontmatter) + index (projection) | `term` | 1..n Alias; 0..n Item usage |
| Alias | index (rebuilt from frontmatter) | `alias` → `term` | provenance: cosine ≥ 0.92 or Levenshtein ≤ 2 |
| ReviewItem | derived from item frontmatter | `item_id` | state: `pending` \| `resolved_*` |
| Intent | vault (`intentions.json`) | `id` | n → TaxonomyTerm (monitored tags); advisory |
| AuditEvent | `.system/audit_log.db` | `seq` | actor, target, before/after, model/grammar versions (§V.1) |
| ModelManifestEntry | `crates/sandland-models/models.lock.toml` | `role` | 1 → file on disk + sha256 + footprint |
| Task | in-memory (transient) | `task_id` | reindex / classify_batch / model_load; progress via events |

Relationships are expressed by ids inside vault files, never by foreign keys in the index; SQLite FKs exist
only *within* `index.db` for internal consistency (`items.id → vec_items.item_id`, `items.rid → items_fts`),
because an index may be rebuilt but content must never depend on it (§I.2).

## 3. `index.db` Topology (disposable, no migrations)

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous  = NORMAL;
PRAGMA busy_timeout = 5000;
PRAGMA foreign_keys = ON;

-- Single integer anchor so FTS5 external-content can mirror rows without duplicating text.
CREATE TABLE items (
  rid           INTEGER PRIMARY KEY AUTOINCREMENT,
  id            TEXT    NOT NULL UNIQUE,
  vault_path    TEXT    NOT NULL UNIQUE,
  workspace_id  TEXT,
  item_type     TEXT    NOT NULL CHECK (item_type IN ('ingest_note','ingest_web','ingest_media','cell')),
  title         TEXT    NOT NULL,
  body_hash     TEXT    NOT NULL,                 -- sha256 of body bytes, excludes frontmatter
  fm_hash       TEXT    NOT NULL,                 -- sha256 of frontmatter region
  read_only     INTEGER NOT NULL DEFAULT 0,
  needs_manual_review INTEGER NOT NULL DEFAULT 0,
  category_term TEXT,
  created_at    INTEGER NOT NULL,                 -- unix seconds
  updated_at    INTEGER NOT NULL
);
CREATE INDEX idx_items_ws       ON items(workspace_id);
CREATE INDEX idx_items_review   ON items(needs_manual_review) WHERE needs_manual_review = 1;
CREATE INDEX idx_items_category ON items(category_term);

-- Lexical leg. External content: text is read from a VIEW, so the FTS index stores no second copy.
CREATE VIEW items_searchable AS SELECT rid AS id, title,
       -- body text is not stored in the vault DB; the resolver supplies it on demand
       '' AS content
FROM items;
CREATE VIRTUAL TABLE items_fts USING fts5(
  title, content,
  content='items_searchable', content_rowid='id',
  tokenize="unicode61 remove_diacritics 1"
);

-- Semantic leg (sqlite-vec). Dim fixed by the embedding model; mismatch ⇒ rebuild, not migrate.
CREATE VIRTUAL TABLE vec_items USING vec0(
  item_id   TEXT PRIMARY KEY,
  embedding FLOAT[384]
);

-- Taxonomy projection: canonical terms + aliases + usage counts (all recomputable from frontmatter).
CREATE TABLE terms (
  term     TEXT PRIMARY KEY,
  kind     TEXT NOT NULL CHECK (kind IN ('category','tag')),
  uses     INTEGER NOT NULL DEFAULT 0,
  created_from_hash TEXT            -- body hash of first item that introduced it (when is_new)
);
CREATE TABLE term_aliases (
  alias   TEXT NOT NULL,
  term    TEXT NOT NULL REFERENCES terms(term),
  reason  TEXT NOT NULL CHECK (reason IN ('levenshtein','cosine','manual')),
  score   REAL NOT NULL,
  PRIMARY KEY (alias, term)
);
CREATE VIRTUAL TABLE terms_fts USING fts5(term, tokenize="unicode61");

-- Asset projection for dedup/refcount queries (bytes themselves live in assets/).
CREATE TABLE assets (
  sha256     TEXT PRIMARY KEY,
  bytes      INTEGER NOT NULL,
  mime_guess TEXT,
  refcount   INTEGER NOT NULL DEFAULT 0
);

-- Board topology projection: enables culling/queries without parsing mpk on the hot path.
CREATE TABLE board_nodes (
  node_id TEXT PRIMARY KEY, workspace_id TEXT NOT NULL, cell_id TEXT,
  x REAL NOT NULL, y REAL NOT NULL, w REAL NOT NULL, h REAL NOT NULL, z INTEGER NOT NULL,
  group_id TEXT, kind TEXT NOT NULL
);
CREATE INDEX idx_nodes_ws_bbox ON board_nodes(workspace_id, x, y);

CREATE TABLE board_edges (
  workspace_id TEXT NOT NULL, src TEXT NOT NULL, dst TEXT NOT NULL,
  kind TEXT NOT NULL CHECK (kind IN ('link','flow','group')),
  PRIMARY KEY (workspace_id, src, dst, kind)
);

-- Reindex bookkeeping: resume-safe and idempotent; nothing here is user content.
CREATE TABLE index_state (
  key TEXT PRIMARY KEY, value TEXT NOT NULL
);   -- keys: status(warming|ready|stale|rebuilding), last_full_rebuild_at, corpus_item_count,
     --      writer_epoch, embedding_model, grammar_sha
```

**Version gate (D7)**: `PRAGMA user_version` packs `INDEX_SCHEMA_VERSION` and a digest of the embedding model
id + grammar sha. On mismatch, or on `SQLITE_CORRUPT`/`NOTADB`, the app deletes `index.db` (never the vault)
and rebuilds from a full scan — the same code path as the constitution's index-destruction gate.

**Hybrid query** is the RFC §4 RRF statement (k = 60, `distance < 0.85`, 100-result legs, single optimized
statement, `LEFT JOIN` on both legs, `ORDER BY rrf_score DESC LIMIT :limit`), wrapped so `items_fts` content
resolution reads through `items_searchable`. Tuning any constant requires the golden-query eval (D13).

## 4. `audit_log.db` Topology (durable, forward-only migrations)

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = FULL;              -- irrecoverable data
PRAGMA busy_timeout = 5000;

CREATE TABLE schema_version (version INTEGER NOT NULL, applied_at INTEGER NOT NULL);

CREATE TABLE audit_events (
  seq         INTEGER PRIMARY KEY AUTOINCREMENT,
  ts          INTEGER NOT NULL,                  -- wall clock for display
  mono_ms     INTEGER NOT NULL,                  -- monotonic for ordering ties
  actor       TEXT NOT NULL CHECK (actor IN ('user','classifier','copilot','watcher','system')),
  action      TEXT NOT NULL,                     -- e.g. 'item.capture','item.metadata_append','term.alias_bind'
  target_kind TEXT NOT NULL,                     -- item | cell | board | asset | term | model | workspace
  target_id   TEXT NOT NULL,
  workspace_id TEXT,
  before_hash TEXT,                              -- sha256 of prior content, NULL for creates
  after_hash  TEXT,
  payload     BLOB NOT NULL,                     -- MessagePack: deltas / before-after snapshots
  undoable    INTEGER NOT NULL DEFAULT 1,
  task_id     TEXT,                              -- batch grouping for undo-batch
  model_role  TEXT, model_id TEXT, model_sha TEXT, grammar_sha TEXT, rule_ver TEXT,
  outcome     TEXT NOT NULL CHECK (outcome IN ('ok','failed','denied')),
  error_code  TEXT, error_detail TEXT
);
CREATE INDEX idx_audit_target  ON audit_events(target_kind, target_id, seq);
CREATE INDEX idx_audit_task    ON audit_events(task_id, seq);
CREATE INDEX idx_audit_pending ON audit_events(seq) WHERE undoable = 1;

CREATE TABLE undo_batches (
  task_id  TEXT PRIMARY KEY,
  created_at INTEGER NOT NULL,
  item_count INTEGER NOT NULL,
  state TEXT NOT NULL CHECK (state IN ('open','applied','rolled_back','partial')),
  note TEXT
);
```

Migration policy: `schema_version` is checked at open; missing rows are applied forward in order; a **future**
version means the log opens read-only with a warning, and the index still rebuilds (so a downgrade remains
possible without content loss). Truncation/compaction is only ever "drop events older than N days" as an
explicit user action, never automatic (§V.2: history, not content — but still history).

Security events share the same table with `outcome='denied'` and `error_code` from the `Sandbox*` family
(D4), which is what makes FR-022 and SC-008 measurable from a single query.

## 5. Validation Rules (testable invariants)

1. `body_hash` recorded in the index equals the body hash computed from the file on every read ⇒ and every
   metadata append leaves it unchanged (D6); violation = `IntegrityMismatch` and the item is quarantined in
   the review queue.
2. No index row may exist without a resolvable `vault_path`; reindex deletes orphans. Never the reverse.
3. `assets/<digest>` content hash equals its filename, verified on first read per session (§V.5).
4. `board.canvas.mpk` node count equals `cells/*.md` count plus link-only proxies; a mismatch resolves toward
   the `.mpk` and rewrites the mirror (D8).
5. Every term in an item's frontmatter resolves through `terms`/`term_aliases` to exactly one canonical term.
6. `needs_manual_review = true` iff the ladder terminated at stage 4 — a model-authored "I'm unsure" may not
   set it directly.
7. Undo of an `item.metadata_append` restores the exact prior frontmatter bytes (byte-equality, not semantic).
8. `vec_items.embedding` dimension is 384 for every row; a row with another dimension is dropped and re-queued.
9. No `index.db` mutation happens outside the single writer task (asserted by a task-local token type in the
   `store.rs` API, not by convention).
10. Every mutation of user content has exactly one `audit_events` row with `outcome='ok'` in the same task; a
    content write without a logged event fails the transaction (audit-before-publish ordering, D7).

## 6. State Machines

**Item / classification lifecycle** (spec FR-004…FR-011, ladder per §VII.4):

```text
captured ──► queued ──► embedding ──► stage1 llm_gbnf ──ok──► classified
   (body      (index     (vec_items    (grammar-locked          │
    written)   writer)     populated)    JSON accepted)         ├─► metadata_appended (frontmatter, logged)
                           │                 │                   └─► needs_manual_review=false
                     timeout/OOM/malformed   │
                             ▼               ▼
                       stage2 retry temp=0.0, ctx=2000 ──ok──► classified
                             │ fail
                             ▼
                       stage3 tfidf+regex ∪ cosine (score > 0.72) ──ok──► classified(stage=heuristic)
                             │ fail / hardware distress
                             ▼
                       stage4 record needs_manual_review=true + audit_event(error) + UI indicator
                             │
                       user resolves ──► accepted | edited | rejected  (each a logged metadata append)
```

Terminal states: `classified`, `needs_manual_review`. No silent terminal — SC-004 ("0 items in unknown state
within 60 s") is enforced by an invariant test that every queued item reaches a terminal state or a task error.

**Model lifecycle** (`sandland-infer` scheduler, sole loader; spec FR-023/024, SC-009/SC-010):

```text
absent ──► verifying(sha256 streaming) ──match──► ready ──load──► loaded ──5 min idle──► unloading ──► ready
                │ mismatch/missing/truncated                                       (scheduler owns unload)
                ▼
        quarantined ──► refetch(verified, resumable) ──► verifying
                │
                └─ refuse inference; classification proceeds at stage3/4 (honest degradation, §VII.6)
```

**Board persistence** (FR-015/016, D8): `editing ──(≥350 ms since last mutation)──► mpk_flushed ──(2 s idle)──►
mirror_written`; any mutation while flushing re-enters `editing`. `mpk_flushed` is the durability boundary the
500 ms loss budget is measured against, and `mirror_written` is interop only (never a restore source).

**Task** (index rebuild, batch classify, model load): `pending ──► running ──► (succeeded | failed(reason) |
cancelled)`; `warming` index state is derived from a running rebuild task and gates result completeness
(spec's partial-results edge case).

## 7. Volume Estimates (5,000-item reference corpus)

| Artifact | Estimate | Budget note |
| --- | --- | --- |
| `items` rows | 5,000 (+ cells) | ~1.5 MB |
| `items_fts` | 150 MB of text indexed, external content | ~55 MB index → within reindex-time, not RAM |
| `vec_items` | 5,000 × 384 × 4 B | ~7.3 MB + row overhead |
| `board_nodes` (1,000-node board) | 1,000 rows + edges | ~180 KB |
| `board.canvas.mpk` | per node ≈ 96 B packed | ~96 KB, ≤ 350 ms cadence |
| `board.canvas.json` | ~4× mpk, canonical | written only at idle |
| `audit_log.db` | ~10k events/day | 500 KB/day; compaction is manual |
| Idle RSS contribution (SQLite page cache, 8 MB cap; watcher; WebView) | — | must fit inside the 350 MB budget |
| Embedding model (384-dim, f32) + classifier GGUF | ~0.1 GB + ~1.12 GB | 650 MB indexing / 2.8 GB inference ceilings (D2) |

## 8. Entity → Requirement Traceability

| Entity / rule | FR | SC | Constitution |
| --- | --- | --- | --- |
| Item file + `body_hash` immutability | FR-001, FR-004, FR-030 | SC-001 | §I.2, §VI.3 |
| Index wipe-and-rebuild gate | FR-002 | SC-002 | §I.3, §I.8 |
| CAS asset + dedup | FR-005 | SC-001 | §V.5 |
| `items_fts` + `vec_items` + RRF | FR-006, FR-029 | SC-003 | §III.3, §Add. Constraints |
| Ladder states + `classification.stage` | FR-007…FR-010 | SC-004, SC-005 | §VII.3–VII.5 |
| Review queue (`needs_manual_review`) | FR-011 | SC-004 | §VII.4 |
| `audit_events` + `undo_batches` | FR-012, FR-013 | SC-006 | §V.1, §V.2 |
| Workspace/board/cell split | FR-014…FR-018 | SC-006, SC-007 | §I.5, §VI.4, §VI.5 |
| `path_guard` + sandbox probe | FR-019…FR-022 | SC-008 | §IV.2–§IV.6 |
| `models.lock.toml` + scheduler | FR-023, FR-024 | SC-009, SC-010 | §V.6, §III.4 |
| Offline-only paths | FR-025 | SC-012 | §II.2–§II.5 |
| Budget consts + harness baseline | FR-026, FR-027 | SC-010, SC-011 | §III.2 |

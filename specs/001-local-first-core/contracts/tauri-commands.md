# Contract: Tauri Command Surface (v0.1)

**Plan**: [../plan.md](../plan.md) | **Data model**: [../data-model.md](../data-model.md) | **Events**: [./tauri-events.md](./tauri-events.md)

Types below live once, in `crates/sandland-proto`, and are exported to TypeScript by `tauri-specta` into
`apps/desktop/web/src/ipc/bindings.ts` (generated; hand-edits fail CI). Every command is `async`, takes
identifiers only (never an absolute path — D5), and returns `Result<_, IpcError>`.

## Conventions

- **Ids**: `ItemId`, `CellId`, `WorkspaceId`, `TaskId`, `NodeId` — opaque ULID strings.
- **Hashes**: `"sha256:<64 lowercase hex>"`.
- **Timestamps**: RFC 3339 UTC with `Z` for IPC; integers (unix seconds) only inside SQL.
- **Idempotency**: writes carry an optional `client_op_id`; replay of the same id returns the first result and
  records `audit_events.outcome='ok'` once.
- **Concurrency**: a stale write is rejected with `Conflict { expected_body_hash }`; the caller must re-read.
  There is no last-writer-wins anywhere on the surface (spec's concurrent-editor edge case).
- **Batch limits**: `ids` args accept ≤ 500 entries; larger work is a task.
- **Authorisation**: all commands are workspace-scoped; `current_workspace` is server-managed state and is the
  confinement boundary (§IV.3, §VI.5).

## `IpcError`

```rust
pub enum IpcError {
    InvalidSegment { segment: String },
    InvalidBase,
    FileNotFound { id: String },
    Conflict { expected_body_hash: String, found_body_hash: String },
    SandboxEscapeAttempt { attempted: String, boundary: String },
    SandboxUnavailable { platform: String, requirement: String },   // fail-closed (FR-020)
    IntegrityMismatch { path: String, expected: String, actual: String },
    ModelMissing { role: String },
    ModelCorrupted { role: String },
    VaultUnsafeFilesystem { reason: String },         // case-insensitive / network mount (FR-028)
    CapacityExceeded { kind: String, limit: u32 },
    BudgetExceeded { metric: String, limit: u64, actual: u64 },
    TaskCancelled { task_id: String },
    ReviewPending { item_id: String },
    UnsupportedForPlatform { feature: String, platforms: Vec<String> },  // Windows agent features (V1)
    Internal { trace_id: String },
}
```

Serialised as `{ "code": "sandbox_escape_attempt", "message": "...", "details": { ... }, "trace_id": "..." }`;
`code` is stable and snake_case — the TS side switches on it, never on `message`.

## Vault & workspace

| Command | Args | Returns | Notes |
| --- | --- | --- | --- |
| `vault_open` | `{ root: string }` | `{ vault: VaultInfo }` | only place a path is accepted; validated in `path_guard`, warns `VaultUnsafeFilesystem` |
| `vault_info` | — | `{ root_label, item_count, index_status, audit_head: number, schema: string[] }` | `index_status: "ready" \| "warming" \| "stale" \| "missing"` |
| `vault_reindex` | `{ mode: "auto" \| "full" }` | `{ task_id }` | "auto" = hash diff against index; `full` = wipe `.system/index.db` + rebuild (the §I.8 gate path) |
| `vault_fsck` | `{ include_orphans: bool }` | `{ task_id }` | read-only report; deletes are a separate explicit command in v0.2 |
| `workspace_create` | `{ title: string }` | `{ workspace_id }` | creates dir + `workspace.toml` + empty board; logs `workspace.create` |
| `workspace_list` | — | `{ workspaces: Workspace[] }` | |
| `workspace_activate` | `{ workspace_id }` | `{ workspace: Workspace, board: BoardSnapshot }` | switches the confinement boundary; aborts any running agent task first |

## Ingest & classification

| Command | Args | Returns | Notes |
| --- | --- | --- | --- |
| `ingest_capture_text` | `{ layer: "notes"\|"web"\|"media", title, text, source_uri? }` | `{ item: ItemSummary, task_id }` | body written first, index second (FR-001); classification queued |
| `ingest_capture_files` | `{ layer, files: { name, bytes }[] }` | `{ items: CaptureResult[] }` | ≤ 20 per call; per-file errors, no all-or-nothing |
| `ingest_list` | `{ layer?, tag?, since?, limit, cursor? }` | `{ items, next_cursor }` | |
| `classify_now` | `{ item_ids: ItemId[] }` | `{ task_id }` | forces the ladder, bypassing batch pacing; UI-triggered |
| `classify_ladder_status` | `{ item_id }` | `{ stage, attempts, error? }` | read of `classification.*` frontmatter |
| `review_queue_list` | `{ limit, cursor? }` | `{ items: ReviewItem[] }` | `needs_manual_review` items only |
| `review_resolve` | `{ item_id, action: "accept"\|"edit"\|"reject", category?, tags? }` | `{ item }` | metadata append + `audit_events`; never touches body (D6) |

## Search

| Command | Args | Returns | Notes |
| --- | --- | --- | --- |
| `search_hybrid` | `{ query, limit?, workspace_only?, modes?: ("fts"\|"vec")[] }` | `{ results: SearchHit[], elapsed_ms, index_status }` | RRF k=60 (D7); `partial: true` when index warming |
| `search_explain` | `{ query, item_id }` | `{ rrf: { fts_rank, vec_rank, score }, snippet }` | debugging/review aid; also the contract test oracle |
| `embed_rebuild` | `{ item_ids? }` | `{ task_id }` | recompute vectors after a model change |

`SearchHit`: `{ item_id, title, vault_path, score: number, source: ("fts"\|"vec"\|"both"), snippet, category, tags, needs_manual_review }`.

## Taxonomy

| Command | Args | Returns | Notes |
| --- | --- | --- | --- |
| `taxonomy_terms` | `{ kind?, q?, limit }` | `{ terms: Term[] }` | `Term { term, kind, uses, aliases: Alias[] }` |
| `taxonomy_merge` | `{ from: string, into: string }` | `{ merged: number }` | manual alias bind; `reason='manual'`, score 1.0 |
| `taxonomy_rename` | `{ from, to }` | `{ task_id }` | rewrites frontmatter across affected items; every rewrite logged and undoable |
| `taxonomy_export` | — | `{ path_label, bytes }` | human-readable dump used by the git mirror tests |

## Board / canvas

| Command | Args | Returns | Notes |
| --- | --- | --- | --- |
| `board_snapshot` | `{ workspace_id }` | `BoardSnapshot` | full topology for load |
| `board_query_viewport` | `{ workspace_id, viewport: Rect, zoom: number }` | `{ nodes: NodeView[] }` | server-side culling assist at 10k nodes; LOD tier included |
| `board_mutate` | `{ workspace_id, ops: BoardOp[], base_rev: number }` | `{ rev, flushed_at }` | batched ops, optimistic UI; `Conflict` on stale `base_rev` |
| `board_cell_create` | `{ workspace_id, kind, title?, body? }` | `{ cell_id, node_id }` | writes `cells/<id>.md` + node op in one task |
| `board_cell_update` | `{ cell_id, body, expected_body_hash }` | `{ cell }` | atomic write + `fsync` + rename (≤ 500 ms loss window) |
| `board_mirror_flush` | `{ workspace_id }` | `{ written: bool }` | forces the canonical JSON mirror (used by tests/git workflow) |

`BoardOp`: `{ kind: "move" \| "resize" \| "link" \| "unlink" \| "group" \| "ungroup" \| "zorder" \| "label",
node_id?, src?, dst?, group_id?, rect?, z? }`. `BoardSnapshot`: `{ rev, nodes, edges, viewport, lod_policy }`.

## Models & runtime

| Command | Args | Returns | Notes |
| --- | --- | --- | --- |
| `model_status` | — | `{ entries: ModelEntry[] }` | `ModelEntry { role, path_label, expected_sha, actual_sha?, state, bytes, rss_estimate }` |
| `model_verify` | `{ role? }` | `{ task_id }` | streaming SHA-256; mismatch ⇒ `ModelCorrupted` + quarantine (FR-023) |
| `model_provide` | `{ role, local_dir_label }` | `{ entry }` | v0.1 intake path for an operator-supplied model file; verified before use |
| `model_preload` / `model_unload` | `{ role }` | `{ state }` | scheduler-mediated; the only way to change residency (D9); unload-after-5-min still automatic |
| `sandbox_status` | — | `{ platform, mechanism, available: bool, kernel_or_os_min: string, agent_features_enabled: bool, denied_right_count: number }` | FR-020/021 evidence; UI banner source |
| `budget_report` | — | `{ idle_mb, indexing_mb, inference_mb, cold_start_ms, command_p95_ms, canvas_fps }` | reads the harness baseline + live guard; SC-010/011 surface |
| `audit_list` | `{ target_id?, actor?, outcome?, since_seq?, limit }` | `{ events, next_seq }` | |
| `audit_undo_last` / `audit_undo_batch` | `{ }` / `{ task_id }` | `{ rolled_back: number }` | restores exact prior bytes (invariant 7) |

## Capability and permission notes

`apps/desktop/src-tauri/tauri.conf.json` grants the WebView **no** filesystem plugin and lists `capabilities`
per window; commands that touch the vault are registered on the main window only. The agent/worker process
inherits Landlock/Seatbelt rules for `workspaces/<active>` + `assets/` (read) — nothing else — and no network
right at all in v0.1 (D4, FR-019/025, SC-012).

## Contract tests (blocking, per §VIII.4)

1. **Schema lockstep**: generated TS compiles against `../tests/contract/fixtures/*.json` golden payloads for
   every command (success + each `IpcError` variant).
2. **Round-trip**: `board_mutate` ops → `.mpk` → snapshot → same topology; and mirror canonicality (byte-equal
   rewrite yields an identical file).
3. **Path-free surface**: a test asserts no command's generated TS type contains a field named `path` other
   than `vault_open`'s `root` and `model_provide`'s directory label — this is the mechanical guard for D5 and
   it fails if someone re-adds a generic path API.
4. **Stale-write rejection**: `Conflict` returned and no partial mutation.
5. **Undo identity**: capture → classify → resolve → `audit_undo_last` yields byte-identical frontmatter.

# Tasks: SANDLAND v0.1 — Local-First Core, Index, Ingest and Spatial Canvas

**Input**: Design documents from `/specs/001-local-first-core/`

**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓,
`.specify/memory/constitution.md` v1.0.0 ✓

**Tests**: INCLUDED AND MANDATORY. The template treats tests as optional; they are not here. Constitution
§VIII.1 requires a failing, reviewed test before implementation per unit, so every story phase below opens
with its test tasks and they are expected to fail red until the implementation tasks land. §VIII.2 additionally
requires ≥ 90 % coverage with property tests on `sandland-vault`, `-sandbox`, `-audit`, `-taxonomy`, which
produces the paired property/fuzz tasks.

**⚠️ Blocking owner input (5 items)**: T001–T005 resolve the plan's *Owner Decisions*. Perf gates (T044) and the
model-integrity tests (T081) **cannot** pass before T001 and T004 land; the CI perf stage is deliberately wired
to fail-closed on an unnamed reference machine (research D12). Everything else can proceed.

**Organization**: one phase per user story from spec.md; each phase is an independently testable increment.

---

## Phase 1: Setup (Governance gates + workspace scaffold)

**Purpose**: Clear the five owner decisions that gate CI configuration, then create the cargo workspace so later
tasks have real paths to land in.

- [ ] T001 Resolve OD-1: name the performance reference machine in `docs/adr/0006-reference-machine.md` (exact CPU, GPU, RAM, NVMe/HDD model, OS build) and emit the matching id in `tests/perf/ref_machine.toml`; perf gates must read this file and refuse to run when absent
- [ ] T002 [P] Resolve OD-2: record the v0.1 Windows posture in `docs/adr/0004-windows-agent-capabilities-deferred.md` (build target yes, agent features no, `UnsupportedForPlatform` surfaced not hidden)
- [ ] T003 [P] Resolve OD-3: approve or reject `docs/adr/0001-embeddings-via-llama-cpp.md`; rejection restores ONNX and requires a written §III exception in the same ADR
- [ ] T004 Resolve OD-4: fill `crates/sandland-models/models.lock.toml` with full 64-lowercase-hex SHA-256 per entry plus `bytes` and `rss_estimate` for embeddings + classifier (closes constitution `TODO(SHA256_CHECKSUMS)`; the 2.8 GB ceiling stays open until the copilot/whisper footprints arrive)
- [ ] T005 [P] Resolve OD-5: confirm or amend the acceptance corpus in `tests/fixtures/MANIFEST.md` (5,000 items ≈ 150 MB Markdown + 2,000 assets; 1,000-node reference board; 50,000 / 10,000 ceilings)
- [ ] T006 Create `Cargo.toml` workspace root with members `crates/sandland-{proto,vault,index,taxonomy,infer,ingest,canvas,audit,sandbox,models,fixtures}` and `apps/desktop/src-tauri`, plus `[workspace.lints.rust] unsafe_code = "warn"` and `[workspace.lints.clippy] all = { level = "deny" }`
- [ ] T007 Pin the toolchain in `rust-toolchain.toml`: stable ≥ 1.85 with components `["miri", "llvm-tools"]` and targets for all five release triples
- [ ] T008 Write `deny.toml`: allow `MIT | Apache-2.0 | BSD-2-Clause | BSD-3-Clause | ISC`, `yank = "deny"`, `unlicensed = "deny"`, copyleft `deny`, sources limited to crates.io + the pinned `llama.cpp` git rev; no network fetch of models at build time
- [ ] T009 Configure `.cargo/config.toml` with CI `rustflags = ["-D", "warnings"]` and per-target linker notes for `aarch64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`
- [ ] T010 Scaffold `apps/desktop/src-tauri/`: `tauri.conf.json` (CSP `default-src 'self'`, **no** fs/shell plugin), `capabilities/default.json` (vault commands on main window only), `src/main.rs`, `src/lib.rs`, `build.rs`
- [ ] T011 [P] Scaffold `apps/desktop/web/`: `package.json`, `tsconfig.json` with `strict: true`, `vite.config.ts`, `index.html`, `src/main.ts`
- [ ] T012 [P] Add `.gitignore` entries `target/`, `node_modules/`, `dist/`, `apps/desktop/web/src/ipc/bindings.ts.bak`, `*.tmp.*`, `tests/fixtures/corpus-5k-vault/` and write `docs/dev/architecture.md` linking the spec artifacts
- [ ] T013 [P] File `docs/adr/0003-index-is-disposable-no-migrations.md` (research D7: version mismatch ⇒ delete + rebuild for `index.db`, forward-only migrations for `audit_log.db`)
- [ ] T014 [P] File `docs/adr/0005-crate-admission.md` with one annex per admitted crate from research D10 (license, purpose, maintenance, `ort` explicitly not admitted, vendored `llama.cpp` rev + SHA-256)
- [ ] T015 File `docs/adr/0002-descriptor-bound-path-io.md` (research D4) recording that the RFC §4 `secure_read_to_string` snippet is superseded, with the symlink-swap PoC as evidence

**Checkpoint**: workspace builds empty (`cargo check --workspace`), all five owner decisions recorded, `deny check licenses` green.

---

## Phase 2: Foundational (Blocking prerequisites)

**Purpose**: The shared spine — one contract schema, the vault writer, the index, the audit log, the capability
probe, and the CI that enforces §VIII. Every user story depends on this phase.

**⚠️ CRITICAL**: No story work may start before this phase is complete.

- [ ] T016 Implement `crates/sandland-proto/src/ids.rs`: ULID-style ids (Crockford base32, monotonic per process, lowercase, `[a-z0-9]{26}`) for `ItemId | CellId | WorkspaceId | TaskId | NodeId`, plus `new_vault_filename(ts, slug)` enforcing "`slug` ≤ 48 chars" and the `<YYYYMMDDTHHMMSSZ>-<slug>.md` pattern
- [ ] T017 Implement `crates/sandland-proto/src/error.rs`: the 16-variant `IpcError` exactly as declared in `contracts/tauri-commands.md`, with stable snake_case `code` strings and `trace_id` on `Internal`
- [ ] T018 Implement `crates/sandland-proto/src/commands.rs`: request/response DTOs for all ~35 commands with `serde` + `specta` annotations; assert no DTO (except `vault_open.root` and `model_provide.local_dir_label`) has a path-typed field
- [ ] T019 [P] Implement `crates/sandland-proto/src/events.rs`: the 15 event payloads from `contracts/tauri-events.md`, each with `seq: u64` and `workspace_id: Option<WorkspaceId>` where scoped
- [ ] T020 [P] Implement `crates/sandland-proto/src/budgets.rs`: single const table — `IDLE_RSS_MB=350`, `INDEXING_RSS_MB=650`, `INFERENCE_RSS_MB=2_800`, `MODEL_UNLOAD_IDLE=300s`, `COLD_START_NVME_MS=2_200`, `COLD_START_HDD_MS=4_500`, `COMMAND_P95_MS=150`, `CAPTURE_P95_MS=1_000`, `CANVAS_FPS=60`, `LOSS_WINDOW_MS=500`, `MPK_COALESCE_MS=350`, `MIRROR_IDLE_MS=2_000`, `WATCH_DEBOUNCE_MS=75`, `WRITER_BATCH=200`, `RRF_K=60`, `VEC_MAX_DISTANCE=0.85`, `LEG_LIMIT=100`, `TYPED_EVENT_RING=256`, `IDS_MAX=500`, `FILES_CAPTURE_MAX=20`
- [ ] T021 Implement `crates/sandland-vault/src/path_guard.rs`: `openat2` with `RESOLVE_IN_ROOT` (rooted at the workspace dirfd) and `RESOLVE_NO_SYMLINKS | RESOLVE_NO_MAGICLINKS`, `O_NOFOLLOW` on the final component, `fstat` `st_dev`/`st_ino` identity verification, and the manual component-walking fallback for kernels < 5.6; return `SandboxEscapeAttempt { attempted, boundary }` on any escape
- [ ] T022 Implement `crates/sandland-vault/src/atomic.rs`: temp-in-same-directory → `fsync` → `rename` → parent-dir `fsync`; discard temp and leave the original untouched on `ENOSPC`; expose `WriteReceipt { post_hash }` for watcher self-suppression (research D11)
- [ ] T023 Implement `crates/sandland-vault/src/frontmatter.rs`: flat-YAML codec with required `schema: sandland.<kind>/1` tag, append-only key insertion that preserves existing key order, and `IntegrityMismatch` when the body hash changes across a metadata append
- [ ] T024 [P] Implement `crates/sandland-vault/src/cas.rs`: streaming SHA-256 in 1 MiB blocks, write-once `assets/<64-hex>`, dedup by digest, refcount derived (never persisted in the vault)
- [ ] T025 Implement `crates/sandland-vault/src/vault.rs`: open/validate root (must be a real directory, not a symlink; record `st_dev`), detect case-insensitive and network-backed volumes ⇒ `VaultUnsafeFilesystem { reason }`, and reject unknown-major `schema` items by opening them read-only with an audit event
- [ ] T026 Implement `crates/sandland-vault/src/watcher.rs`: `notify` with 75 ms per-path debounce, `(st_dev, st_ino, mtime, content_hash)` compare-then-apply, suppression via `WriteReceipt`, bounded rescan at ≤ 500 paths/s, and index-only reconciliation (never rewrites user content)
- [ ] T027 Implement `crates/sandland-index/src/schema.rs`: the full `index.db` DDL from data-model §3 verbatim, including `items.rid INTEGER PRIMARY KEY AUTOINCREMENT`, `body_hash`/`fm_hash` NOT NULL, `item_type CHECK (item_type IN ('ingest_note','ingest_web','ingest_media','cell'))`, external-content `items_fts` over the `items_searchable` view with `tokenize="unicode61 remove_diacritics 1"`, `vec_items ... embedding FLOAT[384]`, `terms/term_aliases` reasons `CHECK (reason IN ('levenshtein','cosine','manual'))`, and `board_edges.kind CHECK (kind IN ('link','flow','group'))`; snapshot it with `insta`
- [ ] T028 Implement `crates/sandland-index/src/store.rs`: `journal_mode=WAL`, `synchronous=NORMAL`, `busy_timeout=5000`, `foreign_keys=ON`, page cache capped at 8 MB; pack `PRAGMA user_version` from schema version + embedding-model id + grammar sha, and on mismatch or `SQLITE_CORRUPT`/`NOTADB` **delete the file and rebuild** (no migration code at all)
- [ ] T029 Implement `crates/sandland-index/src/writer.rs`: single-writer task owning every mutation, mpsc intake, transactions of ≤ 200 mutations, and a `WriterToken` type so no other task can compile a write (invariant 9)
- [ ] T030 Implement `crates/sandland-index/src/search.rs`: the single RRF statement (k = 60, `distance < 0.85`, both legs `LIMIT 100`, `COALESCE` ranks 999, `ORDER BY rrf_score DESC LIMIT :limit`) plus `search_explain` returning `{ fts_rank, vec_rank, score }`
- [ ] T031 Implement `crates/sandland-index/src/rebuild.rs`: full-scan reindex resumable by `body_hash`, `index_state` keys `status/last_full_rebuild_at/corpus_item_count/writer_epoch/embedding_model/grammar_sha`, and `status='warming'` while running
- [ ] T032 Implement `crates/sandland-audit/src/log.rs`: `audit_log.db` DDL from data-model §4 with `actor CHECK (actor IN ('user','classifier','copilot','watcher','system'))`, `outcome CHECK (outcome IN ('ok','failed','denied'))`, `synchronous=FULL`, `schema_version` forward-only migrations, and read-only open on a future version
- [ ] T033 Implement `crates/sandland-audit/src/undo.rs`: `undo_batches` state machine `open|applied|rolled_back|partial`, batch rollback by `task_id`, and byte-identical restore of prior frontmatter (invariant 7)
- [ ] T034 Implement `crates/sandland-audit/src/events.rs`: audit-before-publish ordering — a content write with no matching `outcome='ok'` event aborts the transaction (invariant 10)
- [ ] T035 Implement `crates/sandland-sandbox/src/probe.rs`: capability matrix — Landlock ABI v1 (kernel ≥ 5.13) on Linux, Seatbelt profile writability on macOS, `unavailable` on Windows — returning `SandboxStatus { platform, mechanism, available, kernel_or_os_min, agent_features_enabled, denied_right_count }`, and a fail-closed decision function that no caller may bypass
- [ ] T036 Implement `crates/sandland-sandbox/src/noop.rs`: the deny-all path used when `available = false` — agent tasks return `SandboxUnavailable { platform, requirement }` and perform no file access whatsoever
- [ ] T037 Implement `crates/sandland-models/src/lockfile.rs` + `integrity.rs`: parse `models.lock.toml`, stream-verify SHA-256 to `ModelStatus::{Verified, Corrupted, Missing}`, quarantine on mismatch (never load)
- [ ] T038 Create `crates/sandland-fixtures/src/main.rs`: `generate --seed 20260921` producing `tests/fixtures/corpus-5k/` (5,000 Markdown items + 2,000 assets), `boards/1k`, `boards/10k`, and `queries.toml` with the 60 golden queries — byte-deterministic across runs
- [ ] T039 Wire `apps/desktop/src-tauri/src/emit.rs`: event publisher with per-stream `seq`, 100 ms progress coalescing / 16 ms canvas, ring of 256 dropping oldest **non-terminal** on overflow, and `overflow: true` on the next event
- [ ] T040 Add `crates/sandland-proto/src/bindings.rs` codegen entry + the `pnpm --dir apps/desktop/web run bindings` script that writes `apps/desktop/web/src/ipc/bindings.ts`; add the CI check "generated bindings are committed and unchanged"
- [ ] T041 Create `tests/contract/fixtures/` golden JSON for every command (success + each `IpcError` variant) and the TS compile-against-fixtures job in `tests/contract/mod.rs`
- [ ] T042 Create `tests/fixtures/queries.toml` eval harness in `crates/sandland-index/tests/eval.rs`: rank-biased precision ≥ 0.85 over the 60 golden queries, failing CI if `RRF_K`, `VEC_MAX_DISTANCE` or leg size moves without the metric moving up
- [ ] T043 Write `tests/integration/invariants.rs` covering data-model §5 rules 1–10 (body-hash immutability, no orphan index rows, asset digest match, mpk/cell count parity, term resolution, review-flag iff stage 4, undo byte identity, `FLOAT[384]` dimension, writer-token, audit-before-publish)
- [ ] T044 Add `.github/workflows/ci.yml`: fmt → clippy `-D warnings` → `cargo deny check` → `nextest` → `llvm-cov --fail-under-lines 80` with a 90 override for `-p sandland-vault -p sandland-sandbox -p sandland-audit -p sandland-taxonomy` → `miri` on `sandland-infer` → 5-target release build matrix; perf stage gated on T001's `ref_machine.toml`
- [ ] T045 Add `.github/workflows/nightly.yml` running the `#[ignore]`-tagged `tests/security/`, `tests/crash/` suites and `cargo-fuzz` for 120 s

**Checkpoint**: `cargo nextest run --workspace` green with T043 invariants passing against a stub vault; a
rebuild-from-nothing works; the sandbox probe decides correctly on this host.

---

## Phase 3: User Story 1 — Capture without friction, find it later (Priority: P1) 🎯 MVP

**Goal**: Drop raw material in with no destination choice; have it classified later, searchable now, and fully
recoverable from files alone — the loop that proves file-as-truth end to end.

**Independent Test**: Seed `tests/fixtures/corpus-5k-vault`, let the queue settle, then (a) retrieve items by
phrase, synonym and tag, (b) `rm -rf .system/index.db` and repeat (a) identically, (c) open any item in an
external editor and confirm body + legible metadata. Maps to quickstart §1–§2, SC-001/SC-002/SC-003.

### Tests for User Story 1 (write first, expect RED) ⚠️

- [ ] T046 [P] [US1] Contract tests for `ingest_capture_text`, `ingest_capture_files`, `ingest_list`, `search_hybrid`, `vault_reindex` in `tests/contract/ingest.rs`, asserting idempotent `client_op_id` replay and `Conflict { expected_body_hash, found_body_hash }` on stale writes
- [ ] T047 [P] [US1] Integration test in `tests/integration/us1_capture_retrieve.rs`: capture → visible in ≤ 1 s p95 for a 5 MB item → search p95 < 150 ms on 5,000 items → export diff identical after full index wipe (FR-001/002/006, SC-001/SC-002/SC-003)
- [ ] T048 [P] [US1] Property tests in `crates/sandland-vault/tests/frontmatter_props.rs`: metadata appends never alter body bytes; unknown-major `schema` yields read-only + audit event; NFC normalization and the `[a-z0-9_-]{1,64}` segment rule reject `..`, leading `.`, NUL and over-long slugs
- [ ] T049 [P] [US1] Asset tests in `crates/sandland-vault/tests/cas.rs`: duplicate bytes ⇒ one object + two refs; zero-byte, invalid-UTF-8, and binary-with-text-extension files are stored and reported, never truncated or skipped; orphan detection never auto-deletes

### Implementation for User Story 1

- [ ] T050 [US1] Implement `crates/sandland-ingest/src/capture.rs`: write body to `ingest/<layer>/<generated-name>.md` **first**, then hand the row to the index writer; return `ItemSummary` + `task_id`; per-file results for batch capture (≤ 20 files), no all-or-nothing
- [ ] T051 [US1] Implement `crates/sandland-ingest/src/media.rs`: bytes → `assets/<sha256>` (write-once, verified) + `assets: []` frontmatter refs; media items get manual metadata only in v0.1
- [ ] T052 [US1] Implement `crates/sandland-vault/src/lib.rs` item API: `Item::{read, append_metadata}` with the D6 body-hash guard, `read_only: true` disabling edit affordances, and `IntegrityMismatch` quarantine into the review queue
- [ ] T053 [US1] Implement `crates/sandland-index/src/project.rs`: item upsert/delete projection, `items_fts` external-content sync through the `items_searchable` view, `board_nodes`/`board_edges` refresh, and orphan row deletion on rebuild
- [ ] T054 [US1] Implement the vector leg in `crates/sandland-index/src/vectors.rs`: insert/replace `FLOAT[384]`, drop-and-requeue any row whose dimension ≠ 384, `embed_rebuild` task
- [ ] T055 [US1] Implement `crates/sandland-taxonomy/src/store.rs`: canonical term list + `term_aliases` + usage counts materialized from frontmatter, `taxonomy_cache.json` term vectors, and `terms_fts` prefix lookup for the Top-15/Top-20 candidate sets
- [ ] T056 [US1] Implement `apps/desktop/src-tauri/src/commands/{vault.rs,ingest.rs,search.rs}`: thin dispatchers only (no business logic), returning `Result<_, IpcError>`, `search_hybrid` carrying `partial: true` + `index_status` when warming
- [ ] T057 [US1] Implement event ordering rule 1 in `apps/desktop/src-tauri/src/emit.rs` tests: `item/upserted(reason="captured")` fires only after the atomic rename completes
- [ ] T058 [US1] Implement `apps/desktop/web/src/state/vaultStore.ts` from generated `bindings.ts`: list with cursor paging, `since`/`tag`/`layer` filters, and a "warming" banner when `index_status ≠ ready`
- [ ] T059 [US1] Implement `apps/desktop/web/src/views/IngestView.tsx`: paste + file-drop capture (bytes and display name only — no paths over IPC, D5), item list, item detail rendered from the vault file, and the `VaultUnsafeFilesystem` / `sandbox_status` warning strip (FR-028)
- [ ] T060 [US1] Implement `crates/sandland-vault/tests/watcher.rs` + the UI path for `watcher/conflict`: external edits update the index, a concurrent modification shows both hashes and never auto-resolves
- [ ] T061 [US1] Implement `crates/sandland-cli` (or `-- --reindex-then-exit` in the desktop crate) for the quickstart §1 index-destruction gate: export vault + top-50 results to JSON for before/after comparison

**Checkpoint**: US1 alone is a shippable MVP — capture, search, rebuild-from-files, all outside the app.
Run quickstart §1 and §2 and stop to validate before continuing.

---

## Phase 4: User Story 2 — Think on an infinite board (Priority: P1)

**Goal**: Workspace-scoped canvas of note cells that survives drag, zoom, crash and power loss, holding 60 FPS
at 1,000 nodes with a git-reviewable mirror.

**Independent Test**: Build a 1,000-node board, pan/zoom 60 s, `SIGKILL` the process, relaunch and diff the
topology against the last interaction. Maps to quickstart §6 and §5 (durability half), SC-006/SC-007.

### Tests for User Story 2 (write first, expect RED) ⚠️

- [ ] T062 [P] [US2] Contract tests in `tests/contract/board.rs`: `board_snapshot`, `board_mutate` (`base_rev` stale ⇒ `Conflict`, same `client_op_id` ⇒ first result), `board_cell_update` (`expected_body_hash`), `board_query_viewport` (rect + zoom ⇒ culled node set + LOD tier), `board_mirror_flush`
- [ ] T063 [P] [US2] Durability suite in `tests/crash/board_durability.rs`: 200 `SIGKILL` trials, ≤ 500 ms typing loss (median **and** p95), 0 partially written `cells/*.md`, and post-flush `rev` monotonic recovery per event rule 3
- [ ] T064 [P] [US2] Mirror canonicality test in `crates/sandland-canvas/tests/codec.rs`: rewriting an unchanged board yields a byte-identical `board.canvas.json`; ids sorted; floats on a 1/1000 px grid; no timestamps; LF; 2-space indent
- [ ] T065 [US2] Performance gate in `apps/desktop/web/scripts/canvas-fps.ts` + `crates/sandland-canvas/benches/codec_mp_k10k.rs`: 60 FPS p50 at 1,000 nodes, interactive at 10,000 with far-tier proxies, `.mpk` flush ≤ 350 ms

### Implementation for User Story 2

- [ ] T066 [US2] Implement `crates/sandland-canvas/src/board.rs`: `Board { rev, nodes, edges, viewport, lod_policy }`, op validation (7 op kinds: `move|resize|link|unlink|group|ungroup|zorder|label`), and the node-count ↔ `cells/*.md` parity invariant
- [ ] T067 [US2] Implement `crates/sandland-canvas/src/codec_mp.rs`: `rmp-serde` encode/decode with schema tag; `.mpk` is authoritative; flush coalesced at 350 ms through the vault atomic writer
- [ ] T068 [US2] Implement `crates/sandland-canvas/src/codec_json.rs`: canonical mirror emitter written only after 2 s idle; `.mpk` wins any divergence and the mirror is rewritten with an audit event
- [ ] T069 [US2] Implement `crates/sandland-canvas/src/lod.rs`: tier boundaries "> 0.6 full render", "0.3 ≤ z ≤ 0.6 title + outline + icons", "< 0.3 solid proxy card, reduced opacity" as shared constants consumed by TS via bindings
- [ ] T070 [US2] Implement `apps/desktop/src-tauri/src/commands/board.rs` + `workspace.rs`: `workspace_create` (dir + `workspace.toml` with path digest + empty board, logged `workspace.create`), `workspace_activate` (abort running agent task, then move the confinement boundary), `board_mutate`, `board_query_viewport`
- [ ] T071 [US2] Implement cell CRUD in `crates/sandland-canvas/src/cells.rs`: one file per cell under `cells/<cell_id>.md` with the `sandland.cell/1` frontmatter (`kind` limited to `note|media|link|code`, `forked_from: null` in v0.1) and `board_ref` mirroring
- [ ] T072 [US2] Implement `apps/desktop/web/src/canvas/engine.ts`: PixiJS v8 WebGL2 renderer, container-per-tier, 16 ms event coalescing in, no per-frame allocations in the draw path
- [ ] T073 [US2] Implement `apps/desktop/web/src/canvas/{cull.ts,nodes.ts,viewport.ts}`: `rbush` spatial index, frustum culling against the viewport AABB, three-tier LOD draw with `markdown-it` for hydrated bodies at tier 1, and `z` ordering
- [ ] T074 [US2] Implement `apps/desktop/web/src/canvas/interactions.ts`: drag, `Ctrl+Drag` to link, marquee group/ungroup, and optimistic local ops flushed on `task/terminal` or on `board/mutated.rev` advance
- [ ] T075 [US2] Implement `apps/desktop/web/src/state/boardStore.ts`: `rev` tracking, reload-snapshot-on-conflict recovery, mirror `written: false` handling (no diff ⇒ no rewrite)
- [ ] T076 [US2] Implement `apps/desktop/web/src/views/BoardView.tsx` + `WorkspacePicker.tsx`: pill gallery of workspaces per RFC §5, board surface, and the `budget/exceeded` + `sandbox/denied` status strip
- [ ] T077 [US2] Implement workspace isolation enforcement in `crates/sandland-sandbox/src/lib.rs`: cross-workspace resolve/read/write is denied at the cage and asserted by a dedicated test (`tests/security/cross_workspace.rs`, FR-018, event contract rule 2)

**Checkpoint**: US1 **and** US2 each work independently; kill -9 and restart proves the persistence story
without touching the classifier.

---

## Phase 5: User Story 3 — Local auto-tagging that never overreaches (Priority: P2)

**Goal**: Grammar-constrained local classification with vocabulary reuse, deterministic normalization, and a
four-stage ladder whose only outcomes are a real classification or an honest review flag.

**Independent Test**: Force each stage to fail (`SANDLAND_FORCE_LADDER_STAGE`) over a corpus and assert every
item terminates at a real terminal state with the deciding stage recorded. Maps to quickstart §3 and §5
(integrity half), SC-004/SC-005/SC-009.

### Tests for User Story 3 (write first, expect RED) ⚠️

- [ ] T078 [P] [US3] Grammar lockstep suite in `tests/contract/gbnf.rs`: every `tests/contract/fixtures/classifier/*.json` parses; a `proptest` generator over `ClassifierOutput` emits only grammar-accepted payloads; serde rejects anything the grammar would accept but the schema forbids
- [ ] T079 [P] [US3] `cargo-fuzz` target `fuzz/fuzz_targets/classifier_bytes.rs`: no byte sequence decodes into an accepted-but-invalid classification; corpus seeded from the golden fixtures
- [ ] T080 [P] [US3] Ladder totality test in `crates/sandland-ingest/tests/ladder_total.rs`: for `malformed`, `oom`, `timeout`, and `hardware-distress`, every item reaches `classified` or `needs_manual_review=true` within 60 s, with `classification.stage` and an audit event naming the failing stage (FR-009, SC-004)
- [ ] T081 [P] [US3] Model integrity test in `crates/sandland-models/tests/integrity.rs`: corrupted, truncated and substituted variants of both GGUF files ⇒ 0 successful loads, `quarantined` state event, and `model_verify` reporting `actual_sha ≠ expected_sha` (SC-009)
- [ ] T082 [US3] Normalization property tests in `crates/sandland-taxonomy/tests/normalize.rs`: kebab-case + singularization idempotent; `Levenshtein ≤ 2` merges silently; cosine ≥ 0.92 binds an alias and keeps the existing term canonical; duplicate-after-normalization collapses keeping the highest confidence, ties to the existing term; `is_new: true` without `justification_if_new` fails the attempt instead of being repaired

### Implementation for User Story 3

- [ ] T083 [US3] Implement `crates/sandland-infer/src/engine.rs`: `llama-cpp-2` in-process context pool (one engine for embeddings **and** the classifier per ADR-0001), batch decode, `unsafe` isolated to this crate, 15 s hard timeout per attempt, and no network capability
- [ ] T084 [US3] Implement `crates/sandland-infer/src/grammar.rs`: `include_str!("../../../../specs/001-local-first-core/contracts/taxonomy-gbnf.gbnf")` — move the file to `crates/sandland-infer/assets/taxonomy-gbnf.gbnf` and update the spec link — plus the `grammar_sha` digest (first 12 hex) threaded into every audit event and item frontmatter
- [ ] T085 [US3] Implement `crates/sandland-infer/src/scheduler.rs`: the sole model loader; `absent → verifying → ready → loading → loaded → unloading`; unload after 300 s of inference idleness using monotonic time only; `model/state` and `model/unloaded` events with RSS before/after
- [ ] T086 [US3] Implement `crates/sandland-infer/src/embed.rs`: 384-dim vectors for items and taxonomy terms feeding `vec_items` and `taxonomy_cache.json`
- [ ] T087 [US3] Implement `crates/sandland-ingest/src/classify.rs` stage 1: retrieve Top-15 categories + Top-20 active tags from the index as the candidate set, build the "reuse first, invent only for a grave conceptual gap" prompt, decode under the grammar, then hand to the post-processor
- [ ] T088 [US3] Implement `crates/sandland-ingest/src/ladder.rs`: stage 2 retry at `temperature = 0.0` with the context truncated to the first 2000 tokens; stage 3 TF-IDF + regex ∪ cosine accepting terms scoring > 0.72; stage 4 write `needs_manual_review: true` + audit error + `review/added` event; each failure advances exactly one stage, never a retry loop
- [ ] T089 [US3] Implement `crates/sandland-ingest/src/queue.rs`: low-priority batch worker inside the 650 MB indexing budget, sharing the index writer's ≤ 200-mutation batches, cancellable by `TaskId`
- [ ] T090 [US3] Implement `crates/sandland-taxonomy/src/{normalize.rs,merge.rs}`: kebab + singularize, Levenshtein ≤ 2 silent merge, cosine ≥ 0.92 alias binding with `reason` and `score`, and the `terms.uses` recount on rename
- [ ] T091 [US3] Implement `crates/sandland-taxonomy/src/review.rs` + `review_queue_list` / `review_resolve` commands in `apps/desktop/src-tauri/src/commands/ingest.rs`: accept / edit / reject, each a logged frontmatter append; reject records `outcome='ok'` with no taxonomy change
- [ ] T092 [US3] Implement `apps/desktop/web/src/views/ReviewQueue.tsx`: pending items with the proposed category/tags, `stage` badge, confidence bars, alias-vs-new-term indicators, and the `heuristic` label for hardware-distress results
- [ ] T093 [US3] Implement `model_status`, `model_verify`, `model_provide`, `model_preload`, `model_unload` in `apps/desktop/src-tauri/src/commands/model.rs` wired to the scheduler (the only mutation path that changes residency)
- [ ] T094 [US3] Implement `taxonomy_merge` (manual alias, score 1.0, `reason='manual'`) in `crates/sandland-taxonomy/src/store.rs` and `taxonomy_rename` as an undoable batch task rewriting affected frontmatter in `apps/desktop/src-tauri/src/commands/taxonomy.rs`, emitting `taxonomy/changed` with `renamed` counts
- [ ] T095 [US3] Implement SC-005 measurement in `crates/sandland-audit/src/metrics.rs`: accepted / edited / deleted ratios over the last 7 days of `review_resolve` events, local-only counters, exposed via `budget_report`
- [ ] T096 [US3] Implement hardware-distress detection in `crates/sandland-infer/src/scheduler.rs`: on insufficient RAM/VRAM or `MODEL_UNLOAD` pressure, skip to stage 3 by design and label the result, rather than failing the item

**Checkpoint**: capture → automatic classification → review queue resolves, with the model free to be absent
and the ladder absorbing it. `cargo llvm-cov -p sandland-taxonomy` ≥ 90 %.

---

## Phase 6: User Story 4 — Nothing escapes the cage (Priority: P2)

**Goal**: Kernel-enforced confinement for agent-driven file access, deny-by-default rights, and denial
visibility — the gate that makes the other three stories safe.

**Independent Test**: Run the adversarial suite against a real caged process and require that every attempt is
denied by the **syscall**, appears in the audit log, and shows in the UI, with zero bytes written outside the
workspace. Maps to quickstart §4 and §7 (egress half), SC-008/SC-012.

### Tests for User Story 4 (write first, expect RED) ⚠️

- [ ] T097 [P] [US4] `tests/security/escape_suite.rs`: symlink-out-of-tree, rename race between validation and open, `../` traversal, absolute path in a tool arg, exec attempt, and an outbound `connect()` — each asserted as `EPERM`/`EACCES` from the syscall and never as an app-level filter
- [ ] T098 [P] [US4] `tests/security/path_guard_toctou.rs`: swap the target for a symlink at each component position during a 10,000-iteration loop; assert the read either comes from the validated descriptor or fails, and that the RFC's canonicalize-then-open pattern **fails this test** (regression proof for ADR-0002)
- [ ] T099 [P] [US4] `tests/security/no_outside_write.rs`: after the suite, assert `find <home> -newer <marker>` shows no path outside `workspaces/<active>` and `assets/`, and that `scratchpad/` is index-excluded and wipeable

### Implementation for User Story 4

- [ ] T100 [US4] Implement `crates/sandland-sandbox/src/landlock.rs`: `Ruleset` with read on `workspaces/<active>` + `assets/`, write on `workspaces/<active>` only, ABI v1 floor, `restrict_self()` on the spawned worker before any I/O, and `seccompiler` only where a v0.1 denial case genuinely needs syscall filtering (ADR annex if added)
- [ ] T101 [US4] Implement `crates/sandland-sandbox/src/seatbelt.rs`: generate the per-task profile into `.system/sandbox/<task>.sb` (mode 0600), `deny default`, allow `file-read*` on `/usr/lib` + `/System/Library` + `/private/var/db/dyld`, read/write on the temp dir + `{WORKSPACE_CANONICAL_PATH}` + `{VAULT_ASSETS_PATH}`, `deny network-outbound`, and `process-fork`/`sysctl-read`; spawn under `/usr/bin/sandbox-exec`; record the deprecation exposure in ADR-0004
- [ ] T102 [US4] Implement the Windows path in `crates/sandland-sandbox/src/probe.rs` per T002: `agent_features_enabled = false`, `SandboxUnavailable` for classifier/agent entry points, and `UnsupportedForPlatform { feature, platforms }` on the affected commands — never a silent no-op
- [ ] T103 [US4] Implement `crates/sandland-audit/src/security.rs`: every cage denial writes `audit_events (outcome='denied', error_code='sandbox_*')` and emits `sandbox/denied { attempted_kind, boundary, task_id, severity }` — the single query that makes SC-008 measurable
- [ ] T104 [US4] Implement `sandbox_status` command and the `SandboxBanner` component in `apps/desktop/web/src/views/StatusBar.tsx` (mechanism, kernel/OS minimum, `denied_right_count`, and the denial feed), so FR-020's refusal is legible to a user rather than only in logs
- [ ] T105 [US4] Implement egress denial verification in `crates/sandland-sandbox/tests/egress.rs`: the caged worker's capability set contains no socket right; SC-012 is measured at the boundary during a 30-minute offline session with 500 captures
- [ ] T106 [US4] Implement `SANDLAND_SIMULATE_NO_LANDLOCK` / `SANDLAND_SIMULATE_NO_SEATBELT` hooks in `crates/sandland-sandbox/src/probe.rs` and the fail-closed selftest path (`--selftest-agent`) used by quickstart §4
- [ ] T107 [US4] Implement per-right ADR enforcement in `crates/sandland-sandbox/src/ruleset_builder.rs`: rights are added only through a declared `SandboxRight` enum, and a test asserts the enum's variants match the ADR annex list 1:1 (so a new right cannot land undocumented)
- [ ] T108 [P] [US4] Raise `sandland-vault` and `sandland-sandbox` to ≥ 90 % coverage with property tests over `path_guard` inputs in `crates/sandland-vault/tests/path_guard_props.rs`

**Checkpoint**: all four stories independently functional with the cage enforced and denials observable;
`tests/security` green on Linux and macOS, and correct-by-refusal on Windows.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: The gates and hygiene the constitution demands before this milestone can be called v0.1.

- [ ] T109 [P] Implement the runtime budget guard in `crates/sandland-infer/src/guard.rs` (or `apps/desktop/src-tauri/src/guard.rs`): RSS sampled per state, `budget_exceeded` event after > 5 s sustained, `budget_report` command exposing `{ idle_mb, indexing_mb, inference_mb, cold_start_ms, command_p95_ms, canvas_fps }`
- [ ] T110 [P] Implement `crates/sandland-perf/src/main.rs`: `idle-rss`, `cold-start`, `offline-session` subcommands with per-OS sampling (`/proc/<pid>/status`, `task_info`, `GetProcessMemoryInfo`) writing baselines to `<vault>/.system/budgets.lock`; refuses to run without T001's machine id
- [ ] T111 Implement the 5-target release build in `.github/workflows/release.yml` (`x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`, `aarch64-apple-darwin`, `x86_64-apple-darwin`) with bundle artifacts and a per-target statement of `agent_features_enabled`
- [ ] T112 [P] Run every scenario in `specs/001-local-first-core/quickstart.md` §0–§8 end to end and record the outputs in `docs/dev/measurements/2026-XX-XX-v0.1.md` (the constitution's release ratification artifact)
- [ ] T113 [P] Close the documentation loop: finalize `docs/dev/vault-format.md` (item + cell schema tables, path rules), regenerate `docs/dev/ipc.md` from `contracts/`, and note the superseded RFC snippets (§4 `secure_read_to_string`, §10 ONNX row, §3 grammar `number` rule) with links to ADR-0001/0002
- [ ] T114 [P] Update `specs/001-local-first-core/spec.md` status from Draft to Approved after owner sign-off on T001–T005, or record the deltas the owner requested
- [ ] T115 Enforce the dispatcher-only rule: move any logic found in `apps/desktop/src-tauri/src/commands/` into the owning crate and add a CI grep test in `tests/contract/dispatcher_discipline.rs`
- [ ] T116 Add `.github/pull_request_template.md` with the constitutional review checklist (principle numbers I–VIII), the Complexity Tracking link, and the "new right / new crate ⇒ ADR" reminder
- [ ] T117 Remove the Sync Impact Report HTML comment from `.specify/memory/constitution.md` (ratified-document hygiene from the ratification step)
- [ ] T118 [P] Triage coverage gaps to the 80 % workspace floor with `cargo llvm-cov nextest --workspace --summary-only`, adding targeted tests in the crate under the bar (e.g. `crates/sandland-canvas/tests/`, `crates/sandland-index/tests/`) and no coverage padding
- [ ] T119 [P] Changelog in `CHANGELOG.md`: v0.1 entry citing the principles each change satisfies, per the constitution's docs rule
- [ ] T120 File supersession ADRs for anything the implementation changed relative to research.md (grammar move in T084, `seccompiler` decision in T100), and open follow-up specs for descoped items (Whisper, Chromium/stealth path, RLM, Piece editor) as v0.2/v1.0 candidates
- [ ] T121 Tag `v0.1.0-rc.1` after appending the release summary to `CHANGELOG.md` and linking `docs/dev/measurements/` output; verify no `TODO(` markers remain in `.specify/memory/constitution.md` except the two owner-held items (`TODO(RATIFICATION_SIGNATURES)`, `TODO(MODEL_FOOTPRINTS)`)

---

## Dependencies & Execution Order

### Phase dependencies

```text
Phase 1 Setup ──► Phase 2 Foundational ──► { US1, US2, US3, US4 } ──► Phase 7 Polish
                            │                     │
                            └─ T035/T036 probe ───┴─ US3 & US4 consume the fail-closed decision
                               (blocks US3 story start, not US1/US2)
```

- **Setup (Phase 1)**: T001/T004 gate the perf and integrity tasks (T081, T110, T044); T006–T015 gate all code.
- **Foundational (Phase 2)**: BLOCKS every story. T016–T020 (`proto`) precede T053 (index projections) and T040 (bindings). T021–T026 (`vault`) precede T050/T052/T067. T027–T031 (`index`) precede T047/T057. T032–T034 (`audit`) precede T068/T091/T103. T035–T036 (`probe`) precede T088 and all of US4.
- **Cross-story dependencies (deliberate, not accidental)**: US3's queue writes only through `WriterToken` (T029) and only inside the cage (T035); US2's mirror and US1's capture share the atomic writer (T022). US2 does **not** depend on US1's classifier — an unclassified item is still a valid cell source.

### Within each story

Tests (T046–T049, T062–T065, T078–T082, T097–T099) are written first and must be observed failing; then
models → services → commands → UI; story checkpoint before the next priority.

### Parallel opportunities

- T002, T003, T005 (owner decisions on different ADR files) — parallel.
- Within Phase 2: T019/T020, T024 and T038 are file-disjoint.
- After the Foundational checkpoint, **US1 and US2 can proceed in parallel** (disjoint crates: `ingest`/`index` vs `canvas`), as can US3 and US4 (though US3's T088 needs T100's cage landed to run end to end — order T100 first if a single developer drives both).
- Within stories, the `[P]` test tasks are always parallel; the implementation tasks marked without `[P]` are deliberately serialized on the shared writer/audit path.

## Parallel Example: User Story 1

```bash
# Tests first, all four in parallel (different files, no interdependence):
Task: "Contract tests for ingest_capture_text, ingest_list, search_hybrid, vault_reindex in tests/contract/ingest.rs"
Task: "Integration test capture→index→search→reindex-diff in tests/integration/us1_capture_retrieve.rs"
Task: "Property tests for body immutability and path rules in crates/sandland-vault/tests/frontmatter_props.rs"
Task: "Asset/CAS tests for dedup and hostile inputs in crates/sandland-vault/tests/cas.rs"

# Then the store layer in parallel:
Task: "Implement crates/sandland-index/src/project.rs (item projection + FTS sync)"
Task: "Implement crates/sandland-index/src/vectors.rs (FLOAT[384] leg + dimension rejection)"
Task: "Implement crates/sandland-taxonomy/src/store.rs (terms + aliases + candidate sets)"
```

## Implementation Strategy

### MVP first

1. Phase 1 (T001–T015) — the workspace and the five decisions.
2. Phase 2 (T016–T045) — spine + invariants + CI.
3. Phase 3 (T046–T061) — US1 capture/retrieve, i.e. the shippable core promise.
4. **Stop and validate** with quickstart §1–§2. Ship the MVP: a Local-First vault with disposable index and
   hybrid search, no AI at all — the constitution's §I and §III story is complete and demonstrable.
5. Then US2 (canvas), then US3 (classification), then US4 (cage hardening) — each with its own checkpoint.

### Incremental delivery

- **v0.1-a** = Setup + Foundational + US1: capture, search, rebuild. Offline, file-truth, no agents.
- **v0.1-b** = + US2: boards, cells, durability, 60 FPS. Still no AI, still no cage exposure.
- **v0.1-c** = + US4 first, then US3: the classifier only ever runs inside a verified cage, which is the
  sequence §IV demands. This inverts the spec's P2 order (US3 before US4) **on purpose**: US3's ladder is
  testable in isolation, but its end-to-end path must not precede the cage. Record this reordering as a note
  in the plan if it is accepted.
- Each increment keeps the previous stories' tests green; no increment may mark a §VIII gate `#[ignore]`d.

### Parallel team strategy

Two developers after the Foundational checkpoint: A on US1 → US3, B on US2 → US4 (crates are disjoint; both
share only `proto`, `vault`, `index`, `audit`). Three developers: add a dedicated `contracts` + `tests/` owner
for T041/T042/T043/T047/T063/T079 — the constitution's test-first rule makes the test lane a first-class
stream, not an afterthought.

## Notes

- Every task names exact files; where a task supersedes a snippet in the RFC, it cites the ADR.
- Field constraints are quoted verbatim from `data-model.md` / `contracts/` on purpose (dimension 384, `≤ 200`
  mutations, `≤ 500` ids, `≤ 20` files, 5-tag max, `> 0.72`, `≥ 0.92`, `≤ 2`, `0.85`, 60 FPS, 500 ms, 350 ms,
  2 s, 75 ms, 300 s) — implementation-time discretion on those numbers is what the gates exist to prevent.
- Commit after each task or logical group; a `WAIVED(...)` line for any unmet gate goes in the commit message
  **and** the plan, with an expiry date, per the constitution's exemption rule (§IV and §VIII are not waivable).
- Avoid: touching `contracts/taxonomy-gbnf.gbnf` without T078–T079 in the same change; adding a crate without
  T014's annex; editing `bindings.ts` by hand.

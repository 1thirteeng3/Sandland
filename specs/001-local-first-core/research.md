# Phase 0 Research: SANDLAND v0.1 Local-First Core

**Date**: 2026-09-21 | **Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

Every open question raised by the RFC and the spec has a decision below; the five that only the product owner
can settle are isolated in *Owner Decisions* at the end so `/speckit-tasks` can gate on them.

## D1 — IPC transport: Tauri v2 commands + events, bindings generated from one schema

**Decision**: Every frontend↔backend call is an `#[tauri::command]` returning `Result<T, IpcError>` with
`T: Deserialize` DTOs defined once in `crates/sandland-proto` and annotated with `specta`; `ts-rs`-style
TypeScript types and the command registry are generated into `apps/desktop/web/src/ipc/bindings.ts` by
`tauri-specta` at build time and committed. Long-running operations (reindex, batch classification, model
load) are initiated by a command that returns a `task_id` immediately and report through throttled events.
Blocking work runs on `spawn_blocking`/the writer task, never on the async runtime's reactor.

**Rationale**: The constitution requires the IPC surface to be a contract (§"Structure Decision" and the
contract-test gate in VIII.4), and generated bindings make drift a compile error on both sides rather than a
runtime surprise. Command + event split keeps the 150 ms interaction budget while allowing multi-second work.

**Alternatives considered**: (a) Hand-written TS interfaces — rejected, the exact drift class §VIII forbids.
(b) One `invoke("handle_json", msg)` RPC bus — rejected, loses type checking, per-command capability scoping,
and Tauri's built-in argument validation. (c) Tauri v1 pattern of a global event bus for calls — rejected, no
result typing and easy to leak events across workspaces.

## D2 — Embeddings run through llama.cpp, not ONNX Runtime

**Decision**: `bge-small-en-v1.5` is consumed as a GGUF embedding build through the same `llama-cpp-2` engine
that serves the classifier, keeping 384-dim float vectors and the `vec_items.embedding FLOAT[384]` column.
`models.lock.toml` records both files with full SHA-256 digests and the quantization actually shipped.
Requires ADR-0001 and a correction to RFC §10 (see plan V3).

**Rationale**: §III forbids a second runtime; ONNX Runtime would add ~90 MB of library plus a second thread
pool and allocator competing with the 650 MB indexing budget. One engine also means one load/unload scheduler,
which is what makes the 5-minute unload rule enforceable.

**Alternatives considered**: (a) `ort` crate for ONNX as the RFC stated — rejected for §III. (b) Separate
embedding subprocess — rejected: process churn per batch breaks the latency budget and the no-daemon rule.
(c) Skip vectors in v0.1 and ship FTS5 only — rejected: FR-006 and SC-003 require semantic recall, and §VII's
taxonomy merge depends on cosine similarity ≥ 0.92. (d) Quantize vectors to int8 in `vec_items` — deferred;
float384 at 5,000 items is ~7.3 MB, so the simpler representation wins (YAGNI).

## D3 — Grammar as a committed artifact, fuzzed and versioned with the schema

**Decision**: `specs/001-local-first-core/contracts/taxonomy-gbnf.gbnf` is the only grammar source. It is
embedded into `sandland-infer` with `include_str!`, its SHA-256 is recorded in every audit event as the rule
version, and a test triple (grammar parse, JSON schema validation, Rust struct round-trip) is asserted on
every CI run. The grammar is amended to accept integers as well as decimals for `confidence`
(`number ::= "-"? ([0-9] | [1-9][0-9]{0,3}) ( "." [0-9]+)?`), because the RFC's
`number ::= [0-9]+ "." [0-9]+` cannot express `1`, so a model returning a whole-number confidence would be
counted as malformed output and burn a fallback stage. A `proptest` generator asserts the grammar accepts
every value the Rust schema can emit (no over-constrained grammar) and that the post-processor never receives
a payload that fails the schema (no under-constrained grammar).

**Rationale**: §VII.2 requires grammar/schema lockstep in both directions. Locking the digest into the audit
log makes "which rule version produced this tag" answerable, which §V.1 requires for AI-originated changes.

**Alternatives considered**: (a) JSON-mode + serde repair — prohibited by §VII.1. (b) Repair prompt
(`"try again, output valid JSON"`) — rejected: retry count becomes the correctness metric, and the ladder
already defines the sanctioned retry. (c) Grammar generated from the Rust types — attractive, but a
one-way lockstep; the explicit file plus triple-assertion test catches both directions.

## D4 — Path safety: descriptor-bound I/O replaces canonicalize-then-open

**Decision**: All vault file access goes through `sandland-vault::path_guard`, which resolves paths
component-by-component relative to a trusted directory file descriptor, using `openat2` with
`RESOLVE_IN_ROOT` (rooted at the workspace directory) and `RESOLVE_NO_SYMLINKS | RESOLVE_NO_MAGICLINKS` for
reads the vault cannot own, plus `O_NOFOLLOW` on the final component. Identity is then verified by comparing
`fstat` `st_dev`/`st_ino` of the opened descriptor against the expected entry recorded at validation time, and
writes only ever go to a descriptor inside the cage via temp-file-in-same-directory + `fsync` + `rename` +
parent-dir `fsync`. Where `openat2` is unavailable (Linux < 5.6), the fallback is manual component walking
with `O_NOFOLLOW` and `fstat` checks per component — and, if even that cannot be established for an agent
task, the task refuses to start. On macOS the same logic runs with `O_NOFOLLOW` plus the Seatbelt subpath
cage; on Windows v0.1 the agent path is not exposed at all.

**Rationale**: The RFC's `secure_read_to_string` canonicalizes and then reopens *by path*, so a symlink or
rename landing between `canonicalize()` and `open()` still redirects the read — the exact TOCTOU class §IV.4
names. Kernel-level `RESOLVE_IN_ROOT` and dev/ino verification make the guarantee real; §IV.2 states the
boundary is the kernel and app-level string checks are only defense-in-depth, so the control is placed in
`path_guard` and backed by Landlock/Seatbelt rules rather than a `starts_with` on a `PathBuf`.

**Alternatives considered**: (a) Keep the RFC snippet — rejected, fails the symlink-swap test in
`tests/security/`. (b) `O_NOFOLLOW` alone — closes the final component but not mid-path swaps.
(c) chroot/`pivot_root` or a mount namespace per task — rejected for v0.1: needs privileges, breaks Tauri
resource paths, and duplicates what Landlock already provides unprivileged; revisit with ADR if the sandbox
test suite demands it. (d) Read the whole vault into memory at open — rejected by the 350 MB idle budget.

## D5 — No caller-supplied paths over IPC, ever

**Decision**: Tauri commands accept only opaque identifiers (`ItemId`, `CellId`, `WorkspaceId`, `TaskId`) and
vault-relative logical paths for capture (`ingest/notes/<name>.md` style names, validated and rewritten by the
backend). There is no command that takes an absolute filesystem path, a `~` path, or a `file://` URL, and the
Tauri capability set grants the WebView no `fs` plugin permissions at all.

**Rationale**: The webview is the untrusted side of the trust boundary — a prompt-injected or malicious page
state cannot be trusted to name files. Keeping paths out of the API removes an entire traversal class at the
edge instead of filtering it later, which is what §IV.3's deny-by-default posture asks for, and satisfies
§"security expectations" in the constitution's workflow section (patch the class, not the payload).

**Alternatives considered**: (a) A validated `fs_read(path)` convenience command for the canvas — rejected:
one general-purpose path command becomes the bypass everyone reuses. (b) Allow `file://` drops with backend
canonicalization — rejected for v0.1; drag-and-drop supplies bytes and a display name, not a path, so no read
capability is needed.

## D6 — Ingest immutability with metadata-only appends

**Decision**: An ingested item's body is written once and never rewritten. Classification appends metadata by
rewriting the file atomically with the guarantee "body bytes are byte-identical after the write": the writer
hashes the body before and after and the operation fails (and logs) if they differ. Frontmatter is the only
mutable region, and each mutation appends `taxonomy_v`, `classified_at`, and `needs_manual_review` keys rather
than reordering existing ones. Resolution of the review queue takes the same path.

**Rationale**: The RFC says `ingest/` is read-only/append-only (§2) yet writes classification into the
frontmatter (§3) — the constitution resolved this in favour of "body immutable, metadata appended" (§VI.3).
Hashing the body on both sides turns that rule into a test rather than a convention, which is what makes it
survive refactors.

**Alternatives considered**: (a) Sidecar `.meta.yaml` per item — rejected: two files per item reintroduce
out-of-sync risk and hurt git review. (b) Store taxonomy only in the index — rejected by §I.2 (a fact that
exists only in SQLite is a defect). (c) Treat `ingest/` files as truly read-only and put the classified copy in
`workspaces/` — rejected: the unsorted lake would become the source of derived material, inverting §VI.2.

## D7 — Index topology: single writer, external-content FTS5, RRF in SQL, wipe-on-version

**Decision**: One writer task owns all `index.db` mutations (mpsc channel, `begin`/`commit` per batch of ≤ 200
mutations); readers use separate WAL connections. `items` is the anchor table; `items_fts` is an
**external-content** FTS5 table keyed to an integer `rid` maintained in `items` (`INTEGER PRIMARY KEY AUTOINCREMENT`
alias semantics with a `TEXT` id column), so the FTS index cannot hold a second copy of the text and cannot
drift from the row it mirrors. `vec_items` is a `vec0` virtual table with `embedding FLOAT[384]` and
`item_id` mapped to the same id. Hybrid search is the RFC's single-query RRF with `k = 60`, `distance < 0.85`
and a 100-result leg. `PRAGMA application_id`/`user_version` gate the whole file: a version or embedding-model
mismatch means **delete the file and rebuild from the vault**, never migrate. `journal_mode=WAL`,
`synchronous=NORMAL`, `busy_timeout=5000`.

`audit_log.db` is the opposite: it is a record, so it carries forward-only migrations
(`schema_version` table, no `DELETE` of history), `synchronous=FULL`, and is opened read-only by any component
that only reports.

**Rationale**: External-content FTS halves the disk footprint and removes a class of desync bug; the
single-writer task is what makes "index reflects the vault" true under concurrent watcher events without
`SQLITE_BUSY` retries in business logic. Splitting disposable (index) from durable (audit) is exactly the
distinction §I.2/§V.2 force us to make — and a "migration" for a disposable cache is wasted complexity.
`synchronous=FULL` only where data is irrecoverable.

**Alternatives considered**: (a) Migrate `index.db` across versions — rejected by §I.3 and as needless code.
(b) `contentless` FTS5 (`content=''`) — rejected: deletion requires `content=`-style coordination and
rowid-based `DELETE`, more fragile than external content for a rebuildable cache. (c) Two tables with the text
duplicated — rejected on vault-size and drift. (d) `synchronous=OFF` on the index for reindex throughput —
rejected: crash mid-reindex would leave a corrupt-looking index; instead reindex resumes by hash. (e) Let the
reindex hold a write transaction for the whole vault — rejected: blocks the writer and wrecks FR-003 latency.

## D8 — Board persistence: MessagePack authoritative, JSON mirror byte-deterministic

**Decision**: `board.canvas.mpk` (via `rmp-serde`) is the authoritative topology, written by a Rust-side
writer task that coalesces mutations at 350 ms and writes atomically. `board.canvas.json` is emitted only
after 2 s of board idleness, and is canonical: keys in a fixed order, nodes/links sorted by id, floats
quantized to a 1/1000 px grid, no timestamps, LF endings, 2-space indent — so an unchanged board produces a
byte-identical file. At open, if the `.json` is newer or differs, the `.mpk` wins and the mirror is rewritten;
a mismatch is logged as an audit event.

**Rationale**: The RFC's cadences (§5) are normative per §III.5. Byte-deterministic mirror output is what
makes the optional git layer reviewable at all (constitution §V.3 allows the mirror but a noisy mirror defeats
it). Choosing `.mpk` as source of truth is stated in §V.3 as "authoritative", so reconciliation must be
explicit rather than silent.

**Alternatives considered**: (a) JSON as the only format — rejected: 10,000-node boards at 350 ms cadence make
serialization and allocation the frame-budget bottleneck. (b) Keep topology only in SQLite — rejected outright
by §I.2. (c) `bincode`/`postcard` instead of `rmp-serde` — rejected: the RFC pins MessagePack and a schema
evolution story is easier with a self-describing format; revisit if write time shows up in the perf gates.
(d) Diff-based persistence (append ops, snapshot later) — deferred: undo already lives in the audit log, and a
second replay mechanism would be redundant complexity.

## D9 — Time and memory accounting: monotonic clocks, sampled RSS, explicit budgets in code

**Decision**: The 5-minute unload, 350 ms coalesce, and 2 s idle timers use `tokio::time::Instant` /
monotonic sources only. Budget thresholds (350/650 MB, 2.8 GB, 2.2 s, 150 ms) exist as a single
`const` table in `sandland-proto` (or a shared `budgets.rs`) referenced by both the perf harness and a
lightweight runtime guard that logs a `budget_exceeded` event when RSS crosses a threshold for > 5 s.
Unload decisions are made by the model scheduler, and the scheduler is the only component that may load a
model.

**Rationale**: The constitution demands continuous measurement, so a budget must be data in the repo, not
prose in a template. The runtime guard gives a signal on user machines where CI never runs; monotonic clocks
defuse the time-jump edge case in the spec.

**Alternatives considered**: (a) Assert budgets in unit tests only — rejected: CI hardware variance makes a
single test too flaky to gate on; the harness reports medians plus a hard ceiling instead. (b) `SystemTime`
deltas — rejected for the reasons above. (c) jemalloc/mimalloc RSS tuning as a fix — deferred; if budgets
fail, the ADR documents the allocator choice rather than silently swapping allocators.

## D10 — Crate admission batch for v0.1

**Decision**: Admitted for v0.1, each with license, purpose, and a `sandland`-specific note in
`docs/adr/0005-crate-admission.md`: `tauri` + `tauri-specta` + `specta` (MIT/Apache-2.0), `serde`/`serde_json`
(MIT/Apache-2.0), `rmp-serde` (MIT), `rusqlite` bundled (MIT), `sqlite-vec` (MIT/Apache-2.0), `notify`
(MIT), `llama-cpp-2` (MIT) linking a vendored `llama.cpp` build (MIT, pinned by commit + SHA-256),
`landlock` (MIT/Apache-2.0), `seccompiler` (Apache-2.0, only if Seccomp is needed for a v0.1 denial case),
`nix`/`libc` (MIT), `sha2`/`blake3` (MIT/Apache-2.0), `thiserror` (MIT), `tokio` (MIT, features trimmed),
`proptest`/`insta`/`criterion`/`nextest` (dev-only). `ort` is **not** admitted (see D2); `chromiumoxide`,
`rquest`, `readability-rs` stay out until v1.0. Copyleft check: none of the above is GPL/AGPL; vendored
`llama.cpp` is MIT and its build provenance is recorded in `sandland-models`.
`cargo deny` enforces the allow-list and advisory DB in CI.

**Rationale**: §VIII.4 requires an ADR per new crate; batching the v0.1 set into one admission record with
annexes keeps the paper trail proportional while remaining per-crate auditable, and pinning `llama.cpp` by
commit + hash is what §V.5 asks for vendored binaries.

**Alternatives considered**: (a) One catch-all "dependencies" ADR with no annexes — rejected: loses the
per-crate justification the constitution asks for. (b) Admit a `sqlite-vec` fork — rejected: upstream is
maintained and MIT; a fork would need its own maintenance story. (c) Use `rusqlite`'s `bundled` SQLite vs.
system SQLite — chose `bundled` so FTS5/`vec0` availability is not a distro variable; system-SQLite builds are
an explicit non-goal for v0.1.

## D11 — Watcher reconciliation and self-write suppression

**Decision**: `notify` events are coalesced per path with a 75 ms debounce, then reconciled by comparing
`(st_dev, st_ino, mtime, content_hash)` against the index row; matching means "already known, drop event".
Own writes are recognised by the writer recording the post-rename hash synchronously, so no reindex round-trip
happens on our own save. A directory-missing or rename storm falls back to a bounded rescan (whole-workspace
sweep at ≤ 500 paths/s) rather than per-event churn. External edits are applied to the index but user content is
never rewritten from the watcher — a conflicting modification is surfaced in the UI, per the spec's
concurrent-editor edge case.

**Rationale**: Without hash-based suppression the app fights its own writes (a classic 350 ms-cadence failure
mode), and without a rescan fallback a rename storm turns the 650 MB indexing budget into an unbounded queue.

**Alternatives considered**: (a) Trust mtime alone — rejected: same-second external edits and copy tools
preserve mtime. (b) Poll the whole vault every N seconds — rejected on battery/IO, kept only as the
degradation path when `inotify`/`FSEvents`/`ReadDirectoryChangesW` limits are hit. (c) Watcher writes vault
files on conflict resolution — prohibited by §VI.3, so the watcher is index-only by construction.

## D12 — Performance gates: criterion for micro, a harness for the four user-visible budgets

**Decision**: Three tiers. (1) `criterion` benches for micro paths: FTS/vec query plans, RRF fusion,
normalization, mpk encode/decode. (2) A `tests/perf` harness that launches the built app against the reference
corpus and asserts: RSS in the three states, cold-start-to-interactive, command latency p95, canvas FPS over
a scripted 60 s pan/zoom — sampling `/proc/<pid>/status` (Linux), `task_info` (macOS), `GetProcessMemoryInfo`
(Windows) rather than a new dependency. (3) `#[ignore]`-tagged adversarial and crash suites run nightly and on
release candidates. Median-of-5 with a fixed 10 % regression tolerance per metric; a red harness blocks merge,
per §III.1. The harness refuses to run with an unnamed reference machine (plan's inherited TODO).

**Rationale**: Micro-benchmarks alone do not prove a 350 MB idle budget, and a single number per metric is too
noisy on shared CI hardware; medians with a tolerance make the gate enforceable rather than a flake generator.

**Alternatives considered**: (a) Assert budgets in a plain `#[test]` — rejected for flakiness. (b)
`hyperfine`/external scripts only — rejected: no cross-platform RSS story. (c) `dhat` heap profiling in CI —
kept as a diagnostic tool, not a gate, because it measures allocator views rather than user-visible RSS.
(d) `tauri-driver` + WebdriverIO for FPS — deferred to v0.2 when the copilot needs end-to-end UI tests; v0.1
measures FPS from a scripted harness driving the canvas engine directly in a headless WebView build flag.

## D13 — Search relevance is a fixture, not a feeling

**Decision**: `tests/fixtures/queries.toml` holds 60 golden queries with expected top-k ids for the reference
corpus (lexical, semantic, mixed, and deliberately Portuguese + English). A CI check compares fused RRF
results against the golden set with a rank-biased precision floor of 0.85. Changing `k`, the `0.85`
distance cutoff, or leg sizes requires the eval to move up or an ADR documenting the trade-off.

**Rationale**: The constitution's §Additional Constraints calls search tuning a behaviour change needing a
measured eval, and §"no under-tested code" for the index means relevance needs a machine-checkable target.
Without it, "we switched to vectors and search got better" is unreviewable.

**Alternatives considered**: (a) Manual spot checks — rejected. (b) BEIR/MTEB public benchmarks — rejected for
v0.1: they are English-centric and do not reflect a personal vault with Portuguese content; revisit as a
secondary signal when the corpus is larger.

## Owner Decisions (blocking for `/speckit-tasks`)

1. **Reference machine + budget sign-off** — name the exact CPU/GPU/RAM/SSD and OS build the perf gates run
   on, or accept a documented tolerance band. Blocks D9, D12, and every §III gate.
2. **Windows posture for v0.1** — confirm plan V1 (Windows ships vault/index/canvas, agent features disabled)
   or descope Windows from the v0.1 bundle entirely.
3. **Embedding runtime deviation** — approve or reject ADR-0001 (D2/V3); rejection restores ONNX and requires
   a written §III exception.
4. **Model digests and footprints** — supply the full 64-hex SHA-256 values for the shipped pair
   (`TODO(SHA256_CHECKSUMS)`) plus classifier/whisper footprints (`TODO(MODEL_FOOTPRINTS)`), so
   `models.lock.toml` and the 2.8 GB inference ceiling become verifiable.
5. **Corpus and board sizes for acceptance** — confirm the 5,000-item / 1,000-node reference (and
   50,000 / 10,000 ceilings) used by SC-003, SC-007, SC-011.

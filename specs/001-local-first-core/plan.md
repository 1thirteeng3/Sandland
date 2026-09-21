# Implementation Plan: SANDLAND v0.1 — Local-First Core, Index, Ingest and Spatial Canvas

**Branch**: `001-local-first-core` | **Date**: 2026-09-21 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/001-local-first-core/spec.md`

**Working branch note**: feature identifier is `001-local-first-core`; all work lands on the session branch
`arena/01a0c477-sandland` (Spec Kit's `feature.json` state is what the scripts resolve, not a git branch).

## Summary

Ship the v0.1 nucleus: a Markdown-plus-assets vault as the only source of truth, a disposable SQLite index
(FTS5 + `sqlite-vec`, fused with RRF) in front of it, a capture pipeline that classifies locally through a
grammar-constrained SLM with a total fallback ladder, a workspace-isolated canvas engine over atomic file
writes, and OS-level confinement for every agent-driven file access — Rust core in a Tauri v2 app, PixiJS
WebGL frontend.

Two structural commitments shape the whole plan. First, **index vs. record split**: `.system/index.db` is a
cache with a wipe-and-rebuild version gate and no migration code at all, while `.system/audit_log.db` is a
record with forward-only migrations, because the constitution allows lossless reconstruction of the first and
forbids it for the second. Second, **the boundary is the kernel**: path safety is implemented as descriptor
identity plus Landlock/Seatbelt, not as a string-prefix check, which is why the RFC's `secure_read_to_string`
snippet is replaced by an `openat2`/`fstat` design (see [research.md](./research.md) D4).

Scope is v0.1 only: no Peça editor, no web extraction, no RLM hierarchy, no Whisper, no Windows agent cage
(fail-closed instead of uncaged).

## Technical Context

**Language/Version**: Rust 1.85+ (2021 edition, stable; `unsafe` only at FFI boundaries) for the core;
TypeScript 5.x (strict) + WebGL2 for the canvas frontend; Shell/PowerShell-free — CI scripts in `sh`.

**Primary Dependencies**: Tauri v2 (core + IPC + WebView), `tauri-specta` + `specta` (typed TS bindings for
IPC), `serde`/`serde_json`, `rmp-serde` (board binary topology), `rusqlite` (bundled SQLite + FTS5),
`sqlite-vec`, `notify` (file watching), `llama-cpp-2` (in-process llama.cpp for SLM + embeddings + GBNF
grammar-constrained decode), `landlock` + `seccompiler` (Linux cage), `openat2` via `nix`/`libc`
(path resolution), `sha2` + `blake3` (integrity/content addressing), `thiserror`/`anyhow`, `tokio` (single
writer + worker tasks). Frontend: `pixi.js` v8, `rbush` (spatial index), `markdown-it` (cell rendering).
Full crate admission per principle VIII requires an ADR each — tracked in [research.md](./research.md) D10.

**Storage**: User content in the filesystem vault (`.md` + YAML frontmatter + `assets/` CAS keyed by
SHA-256). Derived state in `.system/index.db` (SQLite, WAL, disposable). Reversible history in
`.system/audit_log.db` (SQLite, durable). Ephemeral cache in `.system/taxonomy_cache.json`.

**Testing**: `cargo nextest` unit/integration, `proptest` + `cargo-fuzz` (normalization, grammar
conformance, path resolution), `insta` (contract/DDL snapshots), `cargo llvm-cov` (coverage floors),
`cargo miri` (FFI), `cargo deny` (licenses + advisories), a Rust↔TS contract test that compiles generated
bindings against golden JSON fixtures, and a `perf/` bench harness (criterion + RSS/latency sampler).

**Target Platform**: Linux x86_64 and aarch64 (kernel ≥ 5.13 for Landlock ABI v1; agent features require it),
macOS Apple Silicon and Intel (Seatbelt profile), Windows x86_64 (index/vault/canvas only; agent capabilities
fail-closed until the Job Object cage ships). Packaged as a native Tauri bundle per target.

**Project Type**: Desktop application — cargo workspace (10 library crates + 1 Tauri app crate) + 1 web
frontend package.

**Performance Goals**: 60 FPS p50 at 1,000 nodes (interactive at 10,000 with far-tier proxies), search
< 150 ms p95 on 5,000 items, capture-to-visible < 1 s p95 for 5 MB, cold start < 2.2 s NVMe / < 4.5 s HDD,
idle RSS ≤ 350 MB, indexing ≤ 650 MB, inference ≤ 2.8 GB, unload within 5 min of last inference.

**Constraints**: Local-first (no cloud round-trip in any default path, no telemetry, no resident daemons);
Rust-native backend only, Tauri WebView and (deferred) Chromium are the only non-Rust runtimes;
grammar-constrained inference mandatory with a total fallback ladder; kernel-enforced confinement with
fail-closed behaviour; atomic writes with ≤ 500 ms loss window; index reconstructibility as a release gate.
Open item inherited from the constitution: `TODO(BENCHMARK_REFERENCE_MACHINE)` — gates are meaningless until
the reference machine is named; it must be fixed by ADR before the first perf gate runs.

**Scale/Scope**: Single user; 1–10 vaults; 5,000-item reference corpus (~150 MB Markdown, 2,000 assets)
with a 50,000-item stress ceiling; 1,000-node reference board, 10,000-node ceiling; 10–15 k LOC Rust +
6–8 k LOC TS for v0.1; ~14 weeks of implementation across 5 phases (see `tasks.md`, produced by
`/speckit-tasks`).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| # | Principle (v1.0.0) | Gate applied to this plan | Verdict |
| --- | --- | --- | --- |
| I | File-as-Truth Sovereignty | Vault-only content; index has no migration path (version mismatch ⇒ rebuild); no write path touches SQLite before the file; FR-001/002/SC-001/SC-002 tests are release-blocking | PASS |
| II | Local Execution Sovereignty | Classifier + embeddings run in-process on local models; no network capability granted to `sandland-infer`/`-taxonomy`; SC-012 measured at the sandbox boundary; no updater or ping in v0.1 | PASS |
| III | Native Performance Budgets | Rust-only crates; ONNX Runtime swapped for llama.cpp embeddings to avoid a second runtime (D2); budgets encoded as CI gates with the §III table as thresholds; monotonic clocks for unload/idle timers (D9) | PASS (gate blocked on `TODO(BENCHMARK_REFERENCE_MACHINE)`) |
| IV | Kernel-Enforced Containment | `sandland-sandbox` capability probe decides before any agent task; Landlock + `openat2` descriptor-bound I/O + dev/ino identity check replace path-string checks (D4); no agent features on Windows in v0.1; deny-by-default with per-right ADRs | PASS — with a recorded deviation (see Complexity Tracking, V1) |
| V | Deterministic Auditability | `audit_log.db` durable, forward-only migrations, reversible events with actor/model/grammar versions; atomic temp+fsync+rename for every vault write; SHA-256 model pinning in `models.lock.toml` (closes `TODO(SHA256_CHECKSUMS)` as a task); `.mpk` authoritative over `.json` mirror | PASS |
| VI | Cognitive Separation | Ingest body immutable, frontmatter-only appends (D6); workspace = confinement unit and directory unit; cell-per-file; board/canvas cannot write items; no copilot writes at all in v0.1 | PASS |
| VII | Grammar-Constrained Local Inference | `contracts/taxonomy-gbnf.gbnf` is the single grammar artifact, embedded at build, fuzzed for conformance, and versioned with the Rust schema (D3); 4-stage ladder terminates in `needs_manual_review`; 15 s bounded attempts off the UI thread | PASS |
| VIII | Test-First and Admitted Dependencies | Every crate listed above requires an ADR before `Cargo.toml` changes (D10); ≥ 90 % coverage + property tests mandated for `vault`, `sandbox`, `audit`, `taxonomy` normalization; fmt/clippy `-D warnings`/deny/miri/5-target matrix in CI; TS side type-checked against generated bindings | PASS |

**Gate decision**: no unjustified violations — proceed past Phase 0. Two items are recorded in
*Complexity Tracking* (V1 Windows cage deferral against the five-target rule; V2 audit log as a durable
non-reconstructible database). One inherited blocker is not a design violation but a precondition for the
perf gates: the reference machine must be named by ADR.

**Post-Phase 1 re-check** (after `data-model.md`, `contracts/`, `quickstart.md`): all eight gates still PASS.
Specifically — the IPC contract exposes no path-free arbitrary file access (every command takes an item or
cell id, never a caller-supplied absolute path, closing a class of traversal at the API edge, D5); the
`items_fts` external-content binding cannot desync from `items` (D7); the review-queue resolution flow keeps
ingest bodies immutable (D6); the board codec cannot emit a mirror newer than its source (D8). No new
violations were introduced by the design, so the plan advances to `/speckit-tasks` without a re-plan.

## Project Structure

### Documentation (this feature)

```text
specs/001-local-first-core/
├── spec.md              # Feature spec (bootstrap specify pass; owner sign-off pending)
├── plan.md              # This file
├── research.md          # Phase 0 — decisions D1–D12 with rationale and alternatives
├── data-model.md        # Phase 1 — vault layout, frontmatter schema, DB topology, state machines
├── quickstart.md        # Phase 1 — validation guide mapped to SC-001..SC-012
├── contracts/           # Phase 1 — IPC + grammar + event contracts
│   ├── README.md        # Contract index, conformance strategy, golden fixtures
│   ├── tauri-commands.md# Tauri command surface: DTOs, errors, idempotency
│   ├── tauri-events.md  # Async event streams, sequencing, backpressure
│   └── taxonomy-gbnf.gbnf # Grammar for classifier output (single source of truth)
└── tasks.md             # Phase 2 output (/speckit-tasks — NOT created by this command)
```

ADRs required by this plan, to be created under `docs/adr/` as work starts: `0001-embeddings-via-llama-cpp`
(D2), `0002-descriptor-bound-path-io` (D4), `0003-index-is-disposable-no-migrations` (D7),
`0004-windows-agent-capabilities-deferred` (V1), `0005-crate-admission` (D10, one per crate, or one
consolidated admission record plus per-crate annexes).

### Source Code (repository root)

```text
Cargo.toml                       # [workspace] members + shared lints/deny.toml profile
rust-toolchain.toml              # pinned stable + components (miri, llvm-tools)
deny.toml                        # cargo-deny: licenses (MIT/Apache-2.0/BSD/ISC), advisories, sources
.cargo/config.toml               # target-specific rustflags (e.g. -D warnings on CI)

crates/
├── sandland-proto/              # IPC DTOs: serde + specta types, IpcError taxonomy, id/version types
│   └── src/{commands.rs,events.rs,error.rs,ids.rs}
├── sandland-vault/              # File-as-truth: frontmatter codec, atomic writer, CAS assets, path policy
│   └── src/{path_guard.rs,atomic.rs,frontmatter.rs,cas.rs,vault.rs,watcher.rs,error.rs}
├── sandland-index/              # Disposable SQLite index: schema, FTS5, sqlite-vec, RRF query, rebuild
│   └── src/{schema.rs,store.rs,writer.rs,search.rs,rebuild.rs}
├── sandland-taxonomy/           # Deterministic post-processing: kebab/singularize, levenshtein, aliases
│   └── src/{normalize.rs,merge.rs,store.rs,review.rs}
├── sandland-infer/              # llama.cpp FFI: manifest + integrity, load/unload scheduler, GBNF decode
│   └── src/{manifest.rs,integrity.rs,engine.rs,grammar.rs,scheduler.rs}
├── sandland-ingest/             # Capture pipeline + classification orchestration + fallback ladder
│   └── src/{capture.rs,classify.rs,ladder.rs,queue.rs}
├── sandland-canvas/             # Board model + codecs (mpk/json), LOD policy, topology diffing
│   └── src/{board.rs,codec_mp.rs,codec_json.rs,lod.rs}
├── sandland-audit/              # Reversible event log, undo batches, provenance records
│   └── src/{log.rs,undo.rs,events.rs}
├── sandland-sandbox/            # Capability probe + Landlock/Seatbelt/Job-Object, fail-closed policy
│   └── src/{probe.rs,landlock.rs,seatbelt.rs,noop.rs}
└── sandland-models/             # models.lock.toml + checksum fixtures, verified fetch/verify helpers
    └── src/{lockfile.rs,fetch.rs}

apps/
└── desktop/
    ├── src-tauri/               # Tauri v2 app: command impls, managed state, event emitters
    │   ├── src/{main.rs,lib.rs,commands/{vault.rs,search.rs,ingest.rs,board.rs,model.rs,audit.rs},
    │   │        state.rs,emit.rs,config.rs}
    │   ├── tauri.conf.json      # CSP, capability/permission set, bundle targets
    │   └── build.rs
    └── web/                     # Vite + TS frontend
        └── src/
            ├── ipc/{bindings.ts,invoke.ts}      # bindings.ts is GENERATED, never hand-edited
            ├── canvas/{engine.ts,nodes.ts,viewport.ts,lod.ts,cull.ts}
            ├── views/{IngestView.tsx,BoardView.tsx,ReviewQueue.tsx,StatusBar.tsx}
            ├── state/{vaultStore.ts,boardStore.ts}
            └── styles/

tests/
├── contract/                    # Rust↔TS contract tests over golden JSON (contracts/ fixtures)
├── integration/                 # vault+index+taxonomy end-to-end, reindex, watcher, undo
├── security/                    # escape suite: symlink swap, rename race, traversal, abs path, egress
├── crash/                       # SIGKILL/power-cut trials, partial-write detection
├── perf/                        # criterion benches + RSS/startup/FPS harness + budget assertions
└── fixtures/                    # reference corpus (5k items), boards (1k/10k nodes), golden outputs

docs/
├── adr/                         # 0001..0005 as listed above
└── dev/{architecture.md,ipc.md,vault-format.md}
```

**Structure Decision**: a cargo workspace of small, single-purpose library crates plus one Tauri app crate,
not one monolithic binary crate. The split is mandated by the constitution rather than chosen for taste: the
security-relevant crates (`vault`, `sandbox`, `audit`, `taxonomy`) are separately testable at ≥ 90 % coverage
and separately auditable, the inference crate is the only one allowed to hold `unsafe` FFI (so `miri` runs are
scoped to it), and `proto` exists so the IPC contract is a compile-time artifact shared by Rust and TS instead
of a hand-maintained duplicate. The frontend is a single Vite package under `apps/desktop/web`, with
`bindings.ts` generated from `sandland-proto` — no second source of truth for message shapes. The canvas
*model* lives in `sandland-canvas` (Rust, authoritative, persisted), while its *view* lives in TS: this keeps
the RFC's topology-durability guarantees on the native side and confines the WebView to rendering.

## Complexity Tracking

> **Only violations that must be justified.** Both rows below are deviations from the constitution's
> *Additional Constraints*, not from the eight core principles, and both are proposed for ADR approval rather
> than silently accepted.

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --- | --- | --- |
| V1 — Windows is a build target but not an agent-capable target in v0.1, against "all five targets are release-blocking" and the §12 v0.1 bullet that names only Landlock and Seatbelt | The v0.1 cage list in the RFC has no Windows mechanism; enabling agent-driven file access without a Job Object + restricted-token cage would ship an uncaged code path, which §IV.5 forbids absolutely and marks non-waivable | "Ship it on Windows with app-level path checks" was rejected: it demotes the kernel boundary to advisory logic and creates a platform-divergent security story. "Drop Windows from v0.1 entirely" was rejected: vault/index/canvas are genuinely cross-platform and give Windows users the P1 capture loop. Resolution: capability-gated release matrix — `agent_features = linux, macos` for v0.1, Windows Job Object cage as the v0.2 prerequisite (ADR-0004), and the UI stating "assistive features unavailable on this platform" rather than hiding them |
| V2 — `.system/audit_log.db` is a durable, migration-bearing SQLite database inside a system whose rule is "databases are disposable caches" (§I.2, §V.2) | Undo, batch rollback, and reversible-AI-mutation require second-granularity history that cannot be reconstructed from vault files; §V.1 and §V.2 demand it, while §I.2 demands the vault not depend on it | "Store history as JSONL in the vault" was rejected for write-amplification and parse cost at 5,000+ events/day, and "no undo for v0.1" was rejected because reversibility is a core promise. Resolution as designed: the log is history, never content — losing it degrades undo and audit richness but no user document, and every vault-only restore path is tested with the log deleted (quickstart §V). Forward-only migrations keep it reconstructible-as-empty |
| V3 — embeddings move from `bge-small-en-v1.5` ONNX (RFC §10 matrix) to a GGUF build of the same model under llama.cpp | §III forbids a second runtime in the process; keeping ONNX Runtime alongside llama.cpp adds a ~90 MB library, a second thread pool fighting the 650 MB indexing budget, and a second allocator under memory pressure, for a model already supported by llama.cpp's embedding path | "Run ONNX through `ort`" was rejected as an in-process second runtime (the exception list in §III.1 is closed at WebView + deferred Chromium). "Subprocess CLI for embeddings" was rejected: per-inference process churn violates the latency budget and the no-daemon rule. Resolution: same model family, same 384-dim vectors, one engine — model matrix and `models.lock.toml` updated by ADR-0001, and the RFC §10 row corrected in the same change |

**Open inherited items** (not violations, but blockers for their gates): `TODO(BENCHMARK_REFERENCE_MACHINE)`
before the first CI perf gate; `TODO(SHA256_CHECKSUMS)` closes via `crates/sandland-models/models.lock.toml`
(full 64-hex digests, supplied by the owner); `TODO(SANDBOX_CAPABILITY_MATRIX)` closes via ADR-0004 and the
`probe.rs` minimums; `TODO(RATIFICATION_SIGNATURES)` and `TODO(MODEL_FOOTPRINTS)` remain owner actions.

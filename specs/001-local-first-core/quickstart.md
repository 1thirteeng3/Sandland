# Quickstart & Validation: 001-local-first-core

**Spec**: [spec.md](./spec.md) (SC-001..SC-012) | **Data model**: [data-model.md](./data-model.md) | **Contracts**: [contracts/](./contracts/)

This is the validation route for v0.1, not a tutorial: each step names the command, what it proves, and the
pass condition. Steps are ordered so that a reviewer can stop at the first failure and know which constitutional
gate broke. Until Phase 2 (`tasks.md`) lands, the commands below are the acceptance targets for each crate.

## 0. Prerequisites

| Requirement | Detail |
| --- | --- |
| Rust | toolchain pinned by `rust-toolchain.toml` (stable ≥ 1.85) + `miri`, `llvm-tools` components |
| Node | 22 LTS for the Vite/PixiJS frontend; `pnpm` for workspace scripts |
| Linux build deps | `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libssl-dev`, `pkg-config`, `libxdo-dev` (Tauri v2) |
| macOS | Xcode CLT; macOS ≥ 13 for the Seatbelt profile generation (probe asserts the real minimum) |
| Windows | MSVC Build Tools + WebView2 runtime (v0.1: index/vault/canvas only — plan V1) |
| Models | the two approved GGUF files supplied by the operator (embeddings + classifier) with full digests recorded in `crates/sandland-models/models.lock.toml` |
| Fixtures | `tests/fixtures/corpus-5k/` and `tests/fixtures/boards/{1k,10k}/` generated deterministically: `cargo run -p sandland-fixtures -- generate --seed 20260921` |
| Reference machine | perf gates **refuse to run** until `TODO(BENCHMARK_REFERENCE_MACHINE)` is closed by ADR and `SANDLAND_REF_MACHINE=<id>` matches the recorded id |

```bash
git clone <repo> && cd Sandland
cargo nextest run --workspace                 # unit + property + contract suites
pnpm --dir apps/desktop/web install && pnpm --dir apps/desktop/web typecheck
cargo build --release -p sandland-desktop      # native bundle for the host target
cargo run -p sandland-desktop -- --vault ./tests/fixtures/corpus-5k-vault
```

Expected on a green tree: `nextest` passes with coverage floors reported, `typecheck` reports no errors against
the generated `bindings.ts`, and the app opens with `sandbox_status.agent_features_enabled = true` on
Linux/macOS (and `false`, with a stated reason, on Windows).

## 1. File-as-truth and index disposability (§I → SC-001, SC-002)

```bash
# 1a. Content readable outside the app: every item is plain Markdown + frontmatter
python3 - <<'PY'
import pathlib,sys
bad=[p for p in pathlib.Path("tests/fixtures/corpus-5k-vault/ingest").rglob("*.md")
     if not p.read_text(encoding="utf-8").startswith("---\n")]
sys.exit(f"non-conforming files: {bad[:5]}" if bad else f"OK: all ingest docs are readable Markdown")
PY

# 1b. Index destruction gate
cargo run -p sandland-cli -- vault-fsck --vault <v> --export > before.json
rm -rf <v>/.system/index.db <v>/.system/taxonomy_cache.json
cargo run -p sandland-desktop -- --vault <v> --reindex-then-exit > after.json
cmp <(jq -S . before.json) <(jq -S . after.json) && echo "OK: rebuild is lossless"
```

**Pass**: identical exported vault and identical top-50 result sets for the 60 golden queries
(`tests/fixtures/queries.toml`), and reindex completes without any network access. `rank-biased precision
≥ 0.85` against the golden set (research D13) is asserted by the same command.

## 2. Capture latency and search budget (§III → SC-003)

```bash
cargo nextest run -p sandland-ingest --release capture_latency_p95
cargo bench -p sandland-index -- search_hybrid --baseline main
```

**Pass**: capture-to-visible < 1 s p95 for a 5 MB item; `search_hybrid` < 150 ms p95 on the 5,000-item corpus;
no more than a 10 % median regression versus the recorded baseline.

## 3. Classification ladder is total, never silent (§VII → SC-004, SC-005)

Force each stage deterministically; the system must never hang, invent, or swallow a failure.

```bash
SANDLAND_FORCE_LADDER_STAGE=malformed  cargo nextest run -p sandland-ingest ladder_total
SANDLAND_FORCE_LADDER_STAGE=oom        cargo nextest run -p sandland-ingest ladder_total
SANDLAND_FORCE_LADDER_STAGE=timeout    cargo nextest run -p sandland-ingest ladder_total
cargo nextest run -p sandland-taxonomy --filters "normalize merge alias review"
```

**Pass**: every run ends in `classified` or `needs_manual_review=true` within 60 s per item (SC-004), with the
deciding `stage` recorded in frontmatter and in an audit event. Normalization tests assert kebab-case,
singularization, `Levenshtein ≤ 2` silent merge, cosine `≥ 0.92` alias binding, and the rule that `is_new: true`
without a justification fails the attempt rather than being repaired.

```bash
cargo test -p sandland-infer --test gbnf_lockstep     # grammar ↔ schema ↔ struct, three directions
cargo +nightly fuzz run classifier_bytes -- -max_total_time=120
```

**Pass**: grammar accepts every value the Rust type can emit and rejects every byte sequence that serde would
have to repair; `taxonomy-gbnf.gbnf` digest matches the one baked into the binary.

## 4. Confinement and path safety (§IV → SC-008)

```bash
cargo nextest run -p sandland-sandbox --tests security_escape_suite   # symlink swap, rename race, traversal,
                                                                       # absolute path, exec attempt, egress
cargo nextest run -p sandland-vault  --test path_guard_toctou         # the D4 replacement for canonicalize+open
SANDLAND_SIMULATE_NO_LANDLOCK=1 cargo run -p sandland-desktop -- --vault <v> --selftest-agent
```

**Pass**: every attempt is denied by the kernel (the test asserts `EPERM`/`EACCES` from the syscall, not from an
app-side check), each denial writes `audit_events.outcome='denied'`, the UI shows the banner, and the
fail-closed simulation refuses to start the agent with `SandboxUnavailable` — zero bytes written outside the
workspace, verified by a `find <home> -newer <marker>` assertion in the test.

## 5. Durability, undo, provenance (§V → SC-006)

```bash
cargo nextest run -p sandland-vault --test atomic_write_crash          # temp+fsync+rename, disk-full, power cut
cargo run -p sandland-crash-trials -- --trials 200 --kill-signal SIGKILL   # typing-loss measurement
cargo nextest run -p sandland-audit undo_batch_identity
```

**Pass**: 0 corrupt documents and ≤ 500 ms of typing lost across 200 SIGKILL trials (median and p95 both
reported); undo restores byte-identical prior frontmatter; `assets/<digest>` integrity verified on first read;
no model file whose SHA-256 differs from `models.lock.toml` loads (SC-009), including corrupted, truncated and
substituted variants.

## 6. Canvas performance (§III → SC-007)

```bash
cargo bench -p sandland-canvas -- codec_mp_k10k
pnpm --dir apps/desktop/web run canvas:fps -- --board tests/fixtures/boards/1k --seconds 60
cargo nextest run -p sandland-desktop --test board_mirror_canonical
```

**Pass**: 60 FPS p50 at 1,000 nodes (interactive at 10,000 with far-tier proxies); `.mpk` flush cadence ≤ 350 ms;
rewriting the mirror with unchanged content yields a byte-identical `board.canvas.json` (canonical floats, sorted
ids, no timestamps) — the property that keeps a git layer reviewable.

## 7. Resource and offline budgets (§II, §III → SC-010, SC-011, SC-012)

```bash
cargo run -p sandland-perf -- idle-rss --target <v> --sample 60
cargo run -p sandland-perf -- cold-start --device nvme
cargo run -p sandland-perf -- offline-session --minutes 30 --capture 500
```

**Pass**: ≤ 350 MB idle (within 10 s of last interaction), ≤ 650 MB indexing, ≤ 2.8 GB inference with automatic
unload ≤ 5 min after the last inference; cold start < 2.2 s on NVMe / < 4.5 s on HDD; and **zero** outbound
connections observed at the sandbox boundary during the offline session (measured by the cage, not by app
self-report).

## 8. Whole-feature gate before release (constitution §Additional Constraints, §VIII)

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check licenses sources advisories
cargo llvm-cov nextest --workspace --fail-under-lines 80 --fail-under-lines 90 -p sandland-vault \
     -p sandland-sandbox -p sandland-audit -p sandland-taxonomy
cargo miri test -p sandland-infer --test ffi_boundary
for t in x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-msvc \
         aarch64-apple-darwin x86_64-apple-darwin; do cargo build --release --target $t -p sandland-desktop; done
```

**Pass**: all green. A red `deny check` or a missing ADR for a crate/model that entered `Cargo.toml` since the
last review blocks the merge — §VIII.4 has no schedule exception.

## 9. Known gaps a reviewer will hit

- Perf numbers are advisory until the reference machine is named (plan *Open inherited items*); do not treat a
  soft budget miss as a pass on unnamed hardware.
- Windows users get capture, search, index and canvas; the classifier and any agent-driven file access report
  `SandboxUnavailable` by design (plan V1) — that is the constitution's fail-closed rule working, not a bug.
- The 2.8 GB inference ceiling is not yet cross-checkable against real model footprints
  (`TODO(MODEL_FOOTPRINTS)`); measure the shipped pair and update the constitution rather than the budget.
- v0.1 verifies operator-supplied model files; fetching from a CDN with mirror rotation is a later release, so
  `model_provide` (local dir) is the intake path in every scenario above.

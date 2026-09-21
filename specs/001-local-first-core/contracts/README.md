# Contracts Index — 001-local-first-core

**Plan**: [../plan.md](../plan.md) | **Data model**: [../data-model.md](../data-model.md) | **Research**: [../research.md](../research.md)

v0.1 exposes no network API and no CLI plugin surface, so "the contract" means three things: the IPC boundary
between the WebView and the Rust core, the output grammar that gates every structured inference, and the vault
files themselves as a durable format other tools can read. Those are the interfaces this directory pins.

| Artifact | What it pins | Change control |
| --- | --- | --- |
| [`tauri-commands.md`](./tauri-commands.md) | Command names, DTO shapes, `IpcError` codes, idempotency and conflict rules | Add/remove a command ⇒ update this file, `sandland-proto`, and the TS golden fixtures in the same commit |
| [`tauri-events.md`](./tauri-events.md) | Event names, payloads, `seq` ordering, coalescing and backpressure | New stream ⇒ define its snapshot-recovery path (a client that missed it must be able to catch up) |
| [`taxonomy-gbnf.gbnf`](./taxonomy-gbnf.gbnf) | The only accepted shape of classifier output, and the deterministic rules that follow it | Grammar edits require the proptest/fuzz trio in §VII.2 to be green, plus a `grammar_sha` bump (it is recorded in audit events) |
| [`../data-model.md`](../data-model.md) | Vault layout, frontmatter schemas, index/audit DDL, state machines | Vault format change ⇒ migration + lossless-rebuild test (§I.5); index change ⇒ version bump only (rebuild, no migration) |

## Generation and conformance

```text
crates/sandland-proto        # Rust DTOs (+ specta annotations)  ──┐
                                                                 ├─► tauri-specta ─► apps/desktop/web/src/ipc/bindings.ts
tests/contract/fixtures/*.json # golden payloads  ────────────────┘        (generated; committed; hand-edits fail CI)
```

- **Fixture set**: one golden JSON per command — success plus every `IpcError` variant — under
  `tests/contract/fixtures/`. `cargo nextest` validates them against the Rust types, and a TS type-check
  validates them against the generated bindings; both directions must agree.
- **Grammar conformance**: `tests/contract/gbnf.rs` runs every `fixtures/classifier/*.json` through the
  grammar and the post-processor; `tests/security/fuzz/` runs `cargo-fuzz` over raw bytes to prove nothing
  malformed reaches the vault writer.
- **Snapshot tests**: `insta` captures the DDL (`sandland-index::schema::DDL`), the generated `bindings.ts`,
  and the canonical `board.canvas.json` for a fixed fixture board. A snapshot diff in review is the signal that
  a format changed — which is exactly what the vault-format ADR rule keys off.
- **Version tags**: every persisted document declares `schema: sandland.<kind>/<major>`; the major is bumped
  only with a migration or a rebuild guarantee, per §I.5.

## Adding a command — checklist

1. Define args/return in `sandland-proto`; no absolute paths, ids only (D5) — the path-free-surface test will
   fail otherwise.
2. Implement on the Rust side in the owning crate; the app crate stays a thin dispatcher (no business logic in
   `src-tauri`, so the same code is testable without a WebView).
3. Emit an audit event if the command mutates user content (invariant 10 in the data model).
4. Add the golden fixtures (success + error) and, if it is a write, the idempotency + conflict pair.
5. Update this index and `tauri-commands.md`; if the command needs a new right (path, network, syscall), open
   the ADR first — §IV.3 requires it and §VIII.4 blocks the merge without it.

## Non-contract (explicitly out of scope for v0.1)

No public CLI, no plugin API, no third-party extension surface, no remote protocol. The `board.canvas.json`
mirror and the vault file formats are reviewed as human/git-facing output, but they are not a stable public API
until v0.2 declares them so — which is why the schema tag exists from day one.

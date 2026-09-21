# Contract: Async Event Streams (v0.1)

**Commands**: [./tauri-commands.md](./tauri-commands.md) | **Data model**: [../data-model.md](../data-model.md)

Emitted through Tauri's event channel, typed by `sandland-proto` and included in the generated bindings.
Names are `sandland://<domain>/<signal>`. Every payload carries `seq` (u64, monotonic per stream) so the
frontend can drop out-of-order or duplicate deliveries, and `workspace_id` where the event is scoped, because
delivery must not cross a workspace boundary (§VI.5).

## Delivery and backpressure rules

- **At-least-once with de-dup by `seq`.** Terminal task events are additionally returned by the starting
  command, so a dropped event cannot leave a stuck UI.
- **Coalescing**: progress events are emitted at most every 100 ms (or 16 ms for canvas mirror signals);
  intermediate states are merged, never queued, so a slow webview cannot grow an unbounded backlog.
- **Backpressure**: each stream has a bounded ring (256); on overflow the oldest non-terminal event is dropped
  and an `overflow: true` flag rides on the next event. Terminal/error events are never dropped; if the ring is
  full they evict a progress event.
- **Late subscribers**: `_*_snapshot` state is fetchable via `vault_info`, `model_status`, `budget_report`, so
  an event missed before the webview mounts is recoverable.

## Streams

| Name | Payload | Cadence | Notes |
| --- | --- | --- | --- |
| `sandland://task/progress` | `{ task_id, kind, seq, state: "running", done, total, unit, eta_ms? }` | ≤ 10 Hz | `kind`: `reindex`, `classify_batch`, `model_load`, `fsck`, `term_rename` |
| `sandland://task/terminal` | `{ task_id, kind, seq, state: "succeeded"\|"failed"\|"cancelled", error?: IpcError, summary }` | once per task | also mirrored into the audit log |
| `sandland://index/state` | `{ seq, status: "ready"\|"warming"\|"stale"\|"missing"\|"rebuilding", item_count, pending_vectors }` | on change | drives the "warming up ⇒ partial results" edge case |
| `sandland://item/upserted` | `{ item_id, workspace_id?, seq, reason: "captured"\|"classified"\|"reviewed"\|"watcher" }` | batched ≤ 50/batch | after the file write, never before (FR-001 ordering) |
| `sandland://item/removed` | `{ item_id, seq, reason }` | on change | only via explicit user action in v0.1 |
| `sandland://review/added` \| `removed` | `{ item_id, stage?, reason?, seq }` | on change | `reason='ladder_exhausted'` is the FR-009 terminus |
| `sandland://taxonomy/changed` | `{ terms_changed, aliases_merged, seq }` | on batch end | UI invalidates term lists; includes `renamed` counts after `taxonomy_rename` |
| `sandland://model/state` | `{ role, seq, state: "absent"\|"verifying"\|"ready"\|"loading"\|"loaded"\|"unloading"\|"quarantined", bytes?, error? }` | on change | `quarantined` always implies an audit event (FR-023) |
| `sandland://model/unloaded` | `{ role, seq, idle_ms, rss_before_mb, rss_after_mb }` | per unload | evidence for SC-010 and the 5-minute rule |
| `sandland://budget/exceeded` | `{ metric, limit, actual, window_ms, seq }` | > 5 s sustained | runtime guard (D9); logged, never blocks the user |
| `sandland://sandbox/denied` | `{ seq, attempted_kind: "symlink"\|"traversal"\|"absolute"\|"network"\|"exec", boundary, task_id?, severity }` | immediate | FR-022 / SC-008; UI banner + audit `outcome='denied'` |
| `sandland://sandbox/unavailable` | `{ platform, mechanism, requirement, agent_features_enabled: false, seq }` | at startup + on probe change | FR-020 fail-closed explanation (plan V1 on Windows) |
| `sandland://board/mutated` | `{ workspace_id, rev, seq, op_count, flushed_at_ms? }` | coalesced 16 ms | `flushed_at_ms` present only on `.mpk` flush (durability boundary, D8) |
| `sandland://board/mirror` | `{ workspace_id, seq, written: bool, bytes, delta_lines? }` | after 2 s idle | `written: false` when canonical content was unchanged (byte-determinism proof) |
| `sandland://watcher/conflict` | `{ item_id, seq, ours_hash, theirs_hash }` | immediate | spec's concurrent-editor case; never auto-resolves (§VI.3) |
| `sandland://vault/warning` | `{ code: "case_insensitive"\|"network_mount"\|"missing_assets"\|"future_schema", message, seq }` | at open / on detect | FR-028 and schema-forward reads |

## Sequencing guarantees

1. `item/upserted(reason="captured")` is emitted **after** the atomic rename completes, so the UI never offers
   an item the disk does not have.
2. `task/terminal` for a classify batch is emitted **after** every `review/added` of that batch, so a completed
   batch cannot show a stale queue.
3. `board/mutated.rev` strictly increases per workspace; a client seeing a lower `rev` must reload the
   snapshot (and is how a `Conflict` recovers).
4. `sandbox/denied` may arrive with no preceding event; nothing else is withheld to keep it ordered, since the
   denial is already recorded in the audit log with its own `seq`.

## Contract tests

- **Ordering**: a scripted capture with an injected 200 ms flush delay asserts rule 1 and rule 2.
- **Isolation**: with `workspace_activate(A)` racing a `board/mutated` from B, no B event carries A's
  `workspace_id`, and no A-side handler receives B (spec's isolation story, FR-018).
- **Overflow**: 10,000 progress events into a stalled subscriber ⇒ exactly the non-terminal ones drop, and the
  terminal event still arrives (used as the regression test for the ring buffer).
- **Recovery**: kill the app after `board/mutated(flushed_at_ms)` and relaunch ⇒ snapshot `rev` ≥ last flushed
  `rev`, tying FR-015 to SC-006.

# Feature Specification: SANDLAND v0.1 — Local-First Core, Index, Ingest and Spatial Canvas

**Feature Branch**: `001-local-first-core`

**Created**: 2026-09-21

**Status**: Draft — awaiting owner sign-off (condensed specify pass; see *Origin of this spec*)

**Input**: User description: "Elabore o plano de implementação técnica para a especificação do núcleo
v0.1. Siga estritamente as restrições arquiteturais: backend em Rust nativo, frontend via Tauri v2
(WebView), motor do Canvas em PixiJS (WebGL) e banco de dados SQLite com sqlite-vec. Foque na topologia
do banco de dados, estrutura de arquivos, contratos IPC (Tauri Commands) entre front e back, e a
integração inicial do SLM via llama.cpp para auto-tagging."

## Origin of this spec

`/speckit-specify` was not executed before this feature was planned. To satisfy the constitution's
workflow gate ("no implementation task is created for a feature without an approved spec and plan"), the
specification below was derived from the approved RFC §12 (v0.1 milestone) and §13 (engineering acceptance
criteria), sliced to the v0.1 scope only. It is a condensed pass: review the *Assumptions* section and
re-run `/speckit-specify` for a fuller elaboration if any requirement is contested.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Capture without friction, find it later (Priority: P1)

A researcher drops raw material into Sandland — a pasted note, a saved article file, an image — without
choosing a folder, a category, or a tag. The system stores the material, classifies it in the background,
and later the researcher locates it again by searching a phrase, a near-synonym, or a tag. Closing and
reopening the app shows everything exactly where it was left, and the files remain readable with any plain
text editor.

**Why this priority**: Capture plus retrieval is the minimum viable loop. Without it there is no product,
and it is the slice that proves the file-as-truth claim end to end.

**Independent Test**: Seed a directory of mixed raw items, let classification settle, then (a) retrieve
each item by a phrase, a synonym, and a tag, (b) delete the derived index and repeat (a) unchanged,
(c) open any stored item in an external editor and confirm the text is intact and its metadata is legible.

**Acceptance Scenarios**:

1. **Given** an empty vault, **When** the user drops a plain-text note, **Then** the item appears in the
   unsorted view within one second of the drop and no dialog asks where it should go.
2. **Given** a captured item, **When** classification finishes, **Then** the item shows a summary, exactly
   one category, and between zero and five tags, all drawn from the existing vocabulary unless a genuinely
   new concept was found.
3. **Given** classification that failed or timed out, **When** the item settles, **Then** it is marked as
   needing manual review, is visible in a review queue, and is still fully searchable by text.
4. **Given** a vault with 5,000 indexed items, **When** the user deletes the derived index and reopens the
   app, **Then** search returns the same results for the same queries after reindex, with no user content
   lost.
5. **Given** a search query, **When** results render, **Then** the item matching the exact phrase and the
   item matching only conceptually both appear, ranked above unrelated material.

---

### User Story 2 - Think on an infinite board (Priority: P1)

The user opens a workspace, sees a canvas of notes ("cells") they can drag, link, group and label
spatially, and zoom out to see hundreds of nodes as a legible landscape. Position and grouping are the
meaning: the arrangement survives restart, crash and a hard power cut.

**Why this priority**: The spatial layer is what distinguishes Sandland from a note app, and the RFC scopes
the basic canvas to v0.1. It shares the persistence guarantees of P1 capture, so the two are one MVP.

**Independent Test**: Build a board of 1,000+ nodes, pan/zoom for 60 s, kill the process with SIGKILL,
relaunch, and verify the topology (positions, links, node bodies) matches the last interaction within the
stated tolerance.

**Acceptance Scenarios**:

1. **Given** an open board, **When** the user drags a node, **Then** the frame rate holds steady while
   off-screen nodes are skipped and distant nodes render as simplified proxies.
2. **Given** a board left mid-edit, **When** the machine loses power and the app restarts, **Then** at most
   500 ms of typing is lost and no note file is left partially written.
3. **Given** two workspaces, **When** the user works in one, **Then** nothing in that session can read or
   write the other workspace's files.
4. **Given** a board with 1,000 nodes, **When** the user zooms out past the far threshold, **Then** text is
   dropped in favour of solid proxy shapes and interaction remains responsive.
5. **Given** an idle board, **When** the user stops interacting for a couple of seconds, **Then** a
   human-readable mirror of the topology appears alongside the fast binary state, byte-stable for diffing.

---

### User Story 3 - Local auto-tagging that never overreaches (Priority: P2)

Classification runs entirely on the user's machine using a small local model. The model proposes, existing
vocabulary is reused, and when the model is unavailable — out of memory, busy, or producing malformed
output — the system degrades to a deterministic keyword pass and finally to a manual-review flag. It never
invents a confident-looking answer, and it never fails silently.

**Why this priority**: This is the mechanism that keeps the unsorted lake navigable. It can ship behind a
kill-switch in v0.1 with the fallback ladder as the guaranteed floor, which is why it ranks below the
capture/store loop.

**Independent Test**: Run the classifier against a corpus with the model forced to fail at each stage of
the ladder (malformed output, OOM, timeout) and verify the outcome is always either a valid classification
or an honest review flag, with an audit record explaining which stage decided.

**Acceptance Scenarios**:

1. **Given** a model file whose checksum does not match the approved manifest, **When** classification is
   requested, **Then** the model is refused, quarantined, re-fetched, and the item is queued for the
   fallback path — the model is never loaded.
2. **Given** a classification request, **When** the response arrives, **Then** it always parses against the
   approved output grammar, or the request moves to the next fallback stage.
3. **Given** a proposed tag "processors" when "processor" exists, **When** normalization runs, **Then** the
   near-duplicate merges silently onto the existing term.
4. **Given** a proposed tag 95% similar in meaning to an existing term, **When** normalization runs,
   **Then** the existing term is recorded as the canonical one and the proposal becomes an alias.
5. **Given** the whole ladder exhausted, **When** the item settles, **Then** the user sees a review
   indicator and the audit log records the failing stage and reason.
6. **Given** classification activity, **When** the app is idle, **Then** the model has been unloaded within
   five minutes of its last inference and memory is returned.

---

### User Story 4 - Nothing escapes the cage (Priority: P2)

Whatever the assistant or an extraction routine tries to do, it can only touch the active workspace and its
assets, enforced by the operating system rather than by application goodwill. Attempts to escape are
blocked, logged, and visible.

**Why this priority**: Non-negotiable as a constraint, but in v0.1 the agent surface is narrow (auto-tagging
plus file access), so it is a gate on the other stories rather than a standalone user-visible feature.

**Independent test**: Run the adversarial escape suite (symlink out of the workspace, rename race between
check and open, `../` traversal, absolute path in a tool argument) against a real caged process and confirm
every attempt is denied by the kernel, surfaced in the audit log, and reported to the user.

**Acceptance Scenarios**:

1. **Given** a caged worker, **When** it opens a path that resolves outside the workspace, **Then** the open
   is denied at the kernel and an escape event is recorded.
2. **Given** a workspace directory replaced by a symlink between validation and read, **When** the read
   proceeds, **Then** the data comes from the canonical descriptor validated earlier or the operation fails.
3. **Given** a platform whose sandbox mechanism is unavailable (too-old kernel, restricted profile), **When**
   an agent task is requested, **Then** the task refuses to start, states why, and no code runs uncaged.
4. **Given** a caged worker, **When** it attempts an outbound network connection during classification,
   **Then** the attempt is denied: classification is local-only by construction.

---

### Edge Cases

- A captured file larger than the ingest budget, a zero-byte file, a file with invalid UTF-8, or a binary
  with a text extension: the item is stored and reported, never silently skipped or truncated.
- Vault on a filesystem without atomic rename guarantees, on a network mount, or with case-insensitive
  names: the app detects and warns at vault open, then constrains behaviour rather than corrupting data.
- The same content captured twice: one asset object, two references — no duplicate bytes, no duplicate node.
- Two Sandland instances (or an external editor) writing the same vault: last-writer-wins is unacceptable;
  the watcher re-reads and the conflicting item is surfaced.
- Disk full or quota exceeded mid-write: the temp file is discarded, the original is untouched, and the
  failure is visible.
- Index and vault disagree (index newer, older, or partial): the index is treated as suspect and rebuilt,
  never reconciled by mutating vault files.
- Model present but its tokenizer/vocabulary incompatible, or model download interrupted: treated as missing,
  re-fetched, and the fallback ladder proceeds.
- System under memory pressure or on battery with no discrete GPU: deterministic keyword path is used and
  labelled as such in the UI.
- Time jumps (clock skew, DST, NTP correction) affecting the 5-minute unload and the 2-second mirror timer:
  monotonic clocks only.
- Reindex of a vault with 50,000 items started while the user searches: both proceed; search may return a
  partial result set flagged as warming up.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST treat the vault — Markdown documents with metadata headers plus a
  content-addressed media store — as the only source of user content, so that every stored item is readable,
  editable, and diffable outside the application.
- **FR-002**: All indexes (lexical, vector, taxonomic cache) MUST be disposable and fully reconstructible
  from the vault, and MUST NOT be required for cold start.
- **FR-003**: Users MUST be able to capture text, files, and media into the vault without choosing a
  destination, category, or tag.
- **FR-004**: Captured material MUST be stored as an append-only record: the captured body never modified
  afterwards; only classification metadata may be appended to its header.
- **FR-005**: Duplicate content MUST resolve to a single stored object identified by content hash.
- **FR-006**: The system MUST search across lexical form and meaning, fusing both result sets into one
  ranking, and MUST return results in under 150 ms for command-triggered queries on the reference corpus.
- **FR-007**: Classification MUST run locally, MUST propose a summary, one category, and up to five tags,
  and MUST prefer existing vocabulary over new terms.
- **FR-008**: Every structured model response MUST be validated against an approved output grammar before
  use; non-conforming output MUST advance the system to the next fallback stage rather than being repaired.
- **FR-009**: The fallback ladder MUST terminate in exactly two possible outcomes — a classification, or a
  manual-review flag with a recorded reason.
- **FR-010**: New terms MUST be normalized (case, separators, singular form) and MUST merge onto an existing
  term when near-identical by spelling or when highly similar in meaning, in which case the proposal is kept
  as an alias.
- **FR-011**: Users MUST be able to see and resolve the review queue, accepting, editing, or rejecting a
  suggested classification, with each resolution recorded.
- **FR-012**: Every action performed by or on behalf of the assistant MUST be recorded as a reversible event
  identifying actor, target, before and after state, and the model and rule versions involved.
- **FR-013**: Users MUST be able to undo a classification batch and restore the prior metadata state.
- **FR-014**: The system MUST provide a canvas where note cells can be created, moved, linked, and grouped,
  with each cell stored as its own vault document.
- **FR-015**: Board topology MUST persist continuously during interaction such that an abrupt process
  termination loses no more than 500 ms of typing and never leaves a partially written document.
- **FR-016**: A second, human-readable mirror of the topology MUST be maintained for interchange and
  version control, written only while the user is idle.
- **FR-017**: The canvas MUST sustain 60 FPS with 1,000+ nodes on the reference machine by skipping
  off-screen nodes and simplifying distant ones according to the three zoom tiers.
- **FR-018**: Workspaces MUST be isolated: no read, write, or resolve from one workspace may reach another.
- **FR-019**: Assistant-driven and extraction-driven file access MUST be confined by the operating system
  to the active workspace and its assets, denying everything else by default.
- **FR-020**: When the OS-level confinement mechanism is unavailable, the system MUST refuse to run the
  affected capability and MUST explain why; running it without confinement is prohibited.
- **FR-021**: Path validation and the resulting read MUST bind to the same canonical filesystem object, so
  that a swap of the path in between cannot redirect the operation.
- **FR-022**: Escape attempts MUST be logged and surfaced to the user.
- **FR-023**: Local models MUST be loaded only from an approved manifest with a verified content hash; a
  mismatch MUST block the load and trigger a verified re-fetch.
- **FR-024**: Models MUST be unloaded after five minutes without inference, and idle memory MUST return to
  the idle budget.
- **FR-025**: The application MUST be fully usable offline; no default behaviour may depend on a remote
  service, and no usage data may leave the machine without explicit per-destination consent.
- **FR-026**: Cold start MUST reach interactive canvas state in under 2.2 s on NVMe and 4.5 s on
  HDD/SATA, with per-workspace lazy loading permitted.
- **FR-027**: Memory MUST stay within 350 MB idle, 650 MB while indexing, and 2.8 GB during local
  inference on all shipped targets.
- **FR-028**: Users MUST be able to open a vault from any location, and MUST be warned at open time when the
  location is network-backed, case-insensitive, or otherwise unable to guarantee atomic replacement.
- **FR-029**: All user-facing text in v0.1 (captured content, category and tag terms, and the review
  queue) MUST be stored and rendered as Unicode, and classification MUST function with Portuguese and
  English material.
- **FR-030**: A file watcher MUST keep the index consistent with external edits to the vault without
  rewriting user content.

### Key Entities

- **Item**: one unit of captured material; identity is its vault location plus content hash; carries body,
  summary, one category, tags, read-only marker, timestamps, and review state.
- **Asset**: a binary object stored by content hash and shared by every item referencing those bytes.
- **Taxonomy Term**: a category or tag with a canonical form, aliases, and the count of items using it;
  terms are merged by spelling distance or bound by meaning similarity.
- **Workspace**: an isolated working area containing a board, its cells, its volatile scratch area, and its
  confinement boundary.
- **Board**: the spatial topology of a workspace — nodes, links, viewport, zoom tiers — with a fast binary
  form and an interchange mirror.
- **Cell**: an atomic note attached to a board, stored as its own document, forkable into other documents.
- **Intent**: a stated goal with monitored terms, used for advisory affinity only (persisted in v0.1,
  surfaced in a later release).
- **Audit Event**: an append-only record of a state change with actor, target, before/after, and model/rule
  versions; the unit of undo and rollback.
- **Model Manifest Entry**: an approved local model with role, format, pinned hash, and size, loaded only on
  verified match.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of captured material is recoverable by opening the vault with external tools — no feature
  depends on the index existing.
- **SC-002**: After the derived index is deleted and rebuilt, search results and the exported vault are
  identical to before on the reference corpus.
- **SC-003**: Capture-to-visible-in-list is under 1 s for a 5 MB item, p95, and a user-triggered search
  returns in under 150 ms p95 on a 5,000-item vault.
- **SC-004**: 0 items end in an unknown state: every capture resolves to a classification, a fallback
  classification, or a review flag within 60 s.
- **SC-005**: At least 85% of auto-classifications are accepted or edited (not deleted) by users after one
  week of use, measured on the internal review queue.
- **SC-006**: A SIGKILL during active editing loses at most 500 ms of typed content and produces 0 corrupt
  documents across 200 trials.
- **SC-007**: The canvas holds 60 FPS p50 with 1,000 nodes and stays interactive at 10,000 nodes with the
  far-tier proxying active.
- **SC-008**: Every escape attempt in the adversarial suite (symlink, rename race, traversal, absolute path)
  is denied, and 100% of denials appear in the audit log and the UI.
- **SC-009**: 0 model loads succeed from an unapproved or hash-mismatched file, across corrupted, truncated,
  and substituted variants.
- **SC-010**: Idle memory ≤ 350 MB within 10 s of the last interaction on all shipped targets; inference
  memory never exceeds 2.8 GB.
- **SC-011**: Cold start to interactive canvas < 2.2 s (NVMe) and < 4.5 s (HDD) on the reference machine.
- **SC-012**: 0 outbound network requests are observed during a 30-minute offline capture-and-classify
  session, verified at the sandbox boundary rather than by app self-report.

## Out of Scope for This Feature

- The Peça block editor, fork-on-insert, and the writing copilot (RFC v0.2).
- Web discovery/extraction, the fast and stealth paths, and any Chromium dependency (RFC v1.0).
- The multi-level RLM orchestration and proactive intention alerts in the UI (RFC v1.0).
- Git-integrated vault versioning beyond producing a diff-friendly topology mirror (RFC §11).
- Voice transcription / Whisper, despite its presence in the RFC resource budgets for indexing.
- Windows confinement (Job Object + restricted token): see *Assumptions* and the plan's Complexity Tracking.
- Model download UX beyond integrity verification: v0.1 assumes approved model files are supplied.

## Assumptions

- The v0.1 release targets are Linux (x86_64, aarch64) and macOS (Apple Silicon, Intel); Windows support for
  this feature is index, vault, and canvas only, with assistant-driven file access disabled (fail-closed)
  until the Windows cage exists. Chosen because the RFC's v0.1 bullet list names only Landlock and Seatbelt,
  while the constitution forbids running agents outside a cage. **Needs owner confirmation.**
- The reference corpus for performance acceptance is 5,000 items (roughly 150 MB of Markdown, 2,000 assets)
  and a 1,000-node board. The RFC does not state a corpus size; this one is proposed and must be agreed
  before the gate is meaningful. **Needs owner confirmation.**
- Classification is offered for text-bearing items only in v0.1; media items are stored and indexed as assets
  with manual metadata.
- Approved model files (embeddings + classifier) are provided to the test environment by path or archive;
  fetching them from a CDN is a later-release concern, but hash verification is not.
- Portuguese and English are the only languages exercised in v0.1 acceptance, per the team's working
  languages; other languages are expected to degrade to the keyword path, not to fail.
- Single-user, single-instance-per-vault is assumed for v0.1; concurrent instances are an error condition
  to be detected and warned about, not a supported collaboration mode.
- The board's fast binary topology form is authoritative; the human-readable mirror is best-effort and may
  lag by the idle threshold.

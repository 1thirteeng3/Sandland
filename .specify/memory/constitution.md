<!--
SYNC IMPACT REPORT — ratification of the SANDLAND constitution
=================================================================
Version change    : (uninitialized template scaffold) → 1.0.0 (MAJOR not applicable: first adoption)
Source of truth   : "Documento de Arquitetura Técnica e Especificação de Produto (RFC/PRD)" — SANDLAND,
                    status "Aprovado para Engenharia", supplied inline by the project owner on 2026-09-21.
Authoring mode    : Bilingual — English text is NORMATIVE; PT-BR gloss under each principle is
                    explanatory and non-binding (see Governance §Precedence and Language).
Strictness mode   : Tight — principles are enforced by blocking CI gates, not by convention.

Principles added  : I.   File-as-Truth Sovereignty            (RFC §1.1, §2, §4)
                  : II.  Local Execution Sovereignty          (RFC §1 philosophy, §6, §10)
                  : III. Native Performance Budgets           (RFC §1 metrics, §5.3, §13.1-13.2)
                  : IV.  Kernel-Enforced Containment          (RFC §1.3, §4.2, §8)
                  : V.   Deterministic Auditability           (RFC §1 "Criptograficamente Auditável", §7, §10, §11, §13.3)
                  : VI.  Cognitive Separation of Subsystems   (RFC §1.2, §3, §5, §7)
                  : VII. Grammar-Constrained Local Inference (RFC §3, §8, §9)
                  : VIII.Test-First and Admitted Dependencies (owner directive: tight/non-negotiable governance)

Template sections : Section 2 → "Additional Constraints · Restrições Adicionais" (RFC §13 + stack/milestones)
                    Section 3 → "Development Workflow and Quality Gates · Fluxo de Desenvolvimento e Portões"

Gaps closed against the RFC (governance decisions, not feature changes):
                  - RFC §1.1 makes all databases disposable, yet audit_log.db (§11) holds the only rollback
                    history. Resolved: user-visible content MUST live in vault files; the event log may hold
                    history only if it is advisory for undo/rollback (§V.2).
                  - RFC §2 marks ingest/ as append-only/read-only, yet §3 writes classification results back
                    into the .md frontmatter. Resolved: frontmatter metadata may be appended; the document
                    body of an ingested item is immutable (§VI.3).
                  - RFC §8 relies on /usr/bin/sandbox-exec and Landlock ABI levels that are version-sensitive;
                    §13.4 demands kernel-level guarantees. Resolved: fail closed — an agent NEVER runs uncaged
                    when the OS cage is unavailable (§IV.5).

Deferred TODOs    : TODO(SHA256_CHECKSUMS), TODO(MODEL_FOOTPRINTS), TODO(BENCHMARK_REFERENCE_MACHINE),
                    TODO(SANDBOX_CAPABILITY_MATRIX), TODO(RATIFICATION_SIGNATURES)
Ratified date note: the RFC itself carries no adoption date; 2026-09-21 (constitution adoption) is used as
                    RATIFICATION_DATE. Replace it with the signed approval date once governance signs off.
-->

# SANDLAND Constitution

## Core Principles

### I. File-as-Truth Sovereignty · Soberania do Arquivo como Fonte da Verdade

- The vault — Markdown files with YAML frontmatter plus content-addressed binaries under
  `assets/` — is the single source of truth. `index.db`, `items_fts`, `vec_items`,
  `taxonomy_cache.json` and every other index MUST be treated as disposable, byte-for-byte
  reconstructible caches.
- Any state whose loss would change what the user sees MUST live in a vault file. If a fact
  exists only inside SQLite, it is a defect. `audit_log.db` may store history, but history is
  not content (see §V.2).
- Every mutation path MUST be expressible as: canonicalize → read file → transform → write file.
  Code that mutates the vault by writing to an index first, and "syncing the file later", MUST
  be rejected in review.
- Derived caches MUST NOT be hand-edited and MUST NOT be required for boot; cold start with
  `.system/` deleted MUST reach full functionality after reindex.
- The vault format is a public API. Breaking vault layout, frontmatter schema, or
  `board.canvas.*` shape requires an ADR, a migration path, and a rebuild-from-scratch test
  proving old vaults upgrade losslessly.
- Release gate: an index-destruction test (delete `.system/`, reindex, diff the exported vault
  and search results) MUST pass on all supported targets before any release.

> **PT-BR:** O cofre (`.md` + frontmatter + `assets/`) é a única verdade. Índices são cache
> descartável e reconstruível. Estado que só existe no SQLite é defeito. Mudar o formato do
> cofre é quebrar API pública: exige ADR, migração e teste de reconstrução sem perda.

### II. Local Execution Sovereignty · Soberania de Execução Local

- Inference, embedding, transcription, extraction, indexing and versioning MUST execute entirely
  on the user's machine. No default code path may require a vendor cloud round-trip to function.
- Telemetry, crash reporting, usage analytics and update pings are forbidden unless the user
  explicitly opts in, in-repo, per destination host, with a documented data list. No analytics
  may be enabled by default in any build, including installer-bundled ones.
- Outbound network I/O is permitted only for: (a) user-initiated web discovery/extraction (§RFC
  6), and (b) user-consented model or optional-engine downloads. Both MUST be attributable to an
  explicit user action.
- Sandland MUST NOT install daemons, services, login items, background schedulers, or
  remote-control surfaces. Everything runs for the lifetime of the app process (Windows
  `ActiveProcessLimit = 1` extends this inside agent cages).
- The app MUST be fully usable offline; degradation while offline is a designed state, never an
  error state.
- Any local HTTP listener, if one is ever introduced, MUST bind to loopback, require an auth
  token, and be documented in an ADR.

> **PT-BR:** Tudo roda na máquina do usuário. Sem nuvem obrigatória, sem telemetria por padrão,
> sem serviços residentes. Rede só quando o usuário pede (descoberta, download de modelo).
> Offline é estado suportado.

### III. Native Performance Budgets · Orçamentos de Desempenho Nativo

- Backend logic — data access, network, local inference, extraction, sandboxing — MUST be
  written in native Rust. No external interpreters, JVMs, Node daemons or persistent helper
  runtimes may sit in the background. The two sanctioned exceptions are the Tauri v2 WebView
  (presentation layer) and the optional headless Chromium (lazy, user-consented, caged, and
  disposable — never core).
- These numbers are acceptance gates, not aspirations. Each MUST be continuously measured by a
  CI performance harness on the reference machine and a regression beyond budget blocks the
  merge:

| Execution state | RAM ceiling | Other budget |
| --- | --- | --- |
| Idle (WebView + PixiJS + SQLite + watcher; models unloaded) | ≤ 350 MB | Cold start < 2.2 s (NVMe), < 4.5 s (HDD/SATA) |
| Active indexing (embeddings + Whisper, low-priority batch) | ≤ 650 MB | MUST NOT raise command latency above budget |
| Active inference (classifier or copilot loaded) | ≤ 2.8 GB | Command response < 150 ms; unload after 5 min idle |
| Canvas, 1000+ nodes | — | Stable 60 FPS with frustum culling + LOD |

- Model load/unload MUST be scheduled off the UI thread and MUST be idempotent; the 5-minute
  aggressive unload MUST NOT be traded away for convenience.
- Canvas LOD thresholds (>0.6 full render, 0.3–0.6 title + outline, <0.3 solid proxy card) and
  `board.canvas.mpk` writes every 350 ms with the formatted JSON mirror deferred to 2 s of idle
  are normative: rendering or save cadence changes require a measured before/after comparison in
  the PR.
- Perceptible latency introduced into a user-initiated command path MUST be justified by a
  measurement, not by description. `TODO(BENCHMARK_REFERENCE_MACHINE): name the exact
  CPU/GPU/SSD and OS build the CI gates run on.`

> **PT-BR:** Rust nativo, sem runtimes pesados residentes. As metas (RAM, boot < 2.2 s, comando
> < 150 ms, 60 FPS com 1000 nós) são portões de CI: se o número estourar, o PR não entra.

### IV. Kernel-Enforced Containment · Confinamento Aplicado pelo Kernel

- Every autonomous agent, RLM sub-agent, injected tool call, external script and the headless
  Chromium instance MUST run inside an OS-level cage: Landlock (+ Seccomp where syscall
  filtering is required) on Linux, a dynamically generated Seatbelt profile via `sandbox-exec`
  on macOS, and a Job Object with `CreateRestrictedToken` plus firewall rules on Windows.
- The boundary is enforced by the kernel, not by application logic. Path-prefix checks in Rust
  are defense-in-depth only and MUST NOT be presented as the security control.
- Vault access inside a cage is scoped to the active workspace plus `assets/`. Deny-by-default
  profiles: any new read, write, syscall or network right MUST be added by explicit rule and
  justified in an ADR. Broad `subpath` grants above the workspace root, `/home`, `%USERPROFILE%`
  or `/` are prohibited.
- Time-of-check/time-of-use is a first-class security concern: validation and open MUST bind to
  the canonicalized descriptor (per `secure_read_to_string`), never to the originally requested
  path. Re-checking a mutable path string between validation and read MUST fail review.
- Fail closed: if the platform cage is unavailable, unsupported, or the kernel is too old, the
  agent MUST refuse to start and MUST surface why. Running uncaged, in a "best effort" mode, or
  with app-level checks only is prohibited. `TODO(SANDBOX_CAPABILITY_MATRIX): per-platform
  minimum kernel/OS build for each cage mechanism, plus the Seatbelt-deprecation fallback plan;
  must be closed before the v0.1 gate.`
- Any `SandboxEscapeAttempt` MUST be recorded in the audit log, shown in the UI, and covered by
  a regression test (symlink, rename-race, `..` traversal, absolute-path tool call, hardlink
  into vault).

> **PT-BR:** Agentes de IA e o Chromium só executam dentro da jaula do sistema operacional
> (Landlock, Seatbelt, Job Object). Sem jaula disponível, o agente não roda — nunca roda solto.
> Checagem de caminho em Rust é reforço, não a fronteira; a fronteira é o kernel, com validação
> atômica no descritor canônico.

### V. Deterministic Auditability · Auditabilidade Determinística

- Every AI-originated mutation and every user-visible state change MUST be recorded as a
  reversible event (actor, subsystem, model + prompt/grammar version, before/after, timestamp)
  so undo, batch rollback and AI-reversal work at second granularity.
- The dual-layer versioning model is mandatory: the native event layer for granular rollback,
  and Git (optional) for durable, machine-portable audit. A vault MUST remain fully valid,
  openable and diffable when Git is absent. `board.canvas.mpk` is authoritative; a detected
  divergence from `board.canvas.json` resolves in favour of the `.mpk`, reconciled explicitly
  rather than silently.
- Provenance is a content requirement, not metadata decoration. Content entering a Peça via
  fork-on-insert MUST carry `citations[]` with `source_cell_id`, `inserted_at` and
  `original_snippet`. A piece containing AI- or source-derived assertions without resolvable
  provenance MUST NOT pass review.
- `assets/` MUST be content-addressable storage keyed by SHA-256; identical bytes resolve to one
  object, and integrity MUST be verified on first read of a session.
- Durability gate: a hard power cut during editing MUST lose at most 500 ms of typing and MUST
  NOT corrupt a Markdown file. All writes are atomic (temp + fsync + rename), and SQLite event
  replay is the recovery path.
- Model and binary integrity: every local model MUST be pinned in the homologation matrix by
  exact SHA-256; mismatch or missing file means quarantine, delete, verified re-resume from a
  pinned mirror — never load. `TODO(SHA256_CHECKSUMS): the RFC matrix carries truncated digests;
  full values must be pinned before v0.1. TODO(MODEL_FOOTPRINTS): disk/RAM footprints for the
  copilot and Whisper entries are unspecified, so the 2.8 GB inference ceiling cannot be
  validated yet.`
- Cryptographic auditability means an external party can reproduce claims from the vault: every
  exported piece MUST be checkable against cited cells and asset hashes with public tooling.

> **PT-BR:** Toda mudança fica rastreável e reversível: log de eventos por segundo, Git opcional
> por cima, proveniência obrigatória em citações, assets por hash SHA-256, escrita atômica
> (perda máxima de 500 ms), e nenhum modelo carregado sem checksum conferido.

### VI. Cognitive Separation of Subsystems · Separação Cognitiva dos Subsistemas

- The four subsystems keep distinct duties and distinct write authority: Ingest absorbs
  passively, Whiteboard organizes spatially, Intentions correlate proactively, and the Peça
  produces the editorial artifact. A feature that blurs two roles MUST be split or justified in
  an ADR.
- Ingest requires no organizing effort from the user: no mandatory folder choice, no blocking
  classification dialog, no interactive prompt on the capture path.
- The Ingest layer is append-only. The captured document body is immutable; only frontmatter
  metadata (taxonomy, `needs_manual_review`, timestamps) may be appended, written atomically,
  and each addition MUST be logged. Rewriting body text of an ingested item is prohibited.
- Canvas cells are atomic units; a fork-on-insert copy is independent and MUST NOT auto-sync
  back to its source cell. Live bidirectional editing of a forked block is prohibited unless an
  ADR redefines the mechanic.
- Each workspace is an isolated sandbox: files, agent I/O, scratchpad and permissions of
  `workspace_01j8k9/` MUST NOT be reachable from another workspace.
- Proactive suggestions (intention affinity, auto-canvas proposals) are advisory: they MUST NOT
  mutate vault content without one deliberate user action. The copilot proposes; only the user
  commits.

> **PT-BR:** Cada subsistema tem seu papel e sua autoridade de escrita. Captura é passiva e não
> exige organização; o Ingest é só acréscimo (corpo imutável, metadados anexados); o fork no
> editor é cópia independente; cada workspace é uma caixa isolada. A IA sugere, quem confirma é
> o usuário.

### VII. Grammar-Constrained Local Inference · Inferência Local com Saída Controlada por Gramática

- Every LLM/SLM call that produces structured data MUST be grammar-constrained (GBNF) to a
  schema-valid payload. Parsing structure out of free-form prose is prohibited, as is
  retry-until-it-looks-right prompting.
- The grammar and the Rust-side schema MUST stay in lockstep, and MUST be covered by a
  conformance/fuzz suite that proves (a) no non-conforming output escapes into the pipeline and
  (b) every value the schema can produce is accepted by the grammar — e.g. a `number ::= [0-9]+
  "." [0-9]+` rule rejects integer confidences, so numeric producers and the grammar are
  versioned together.
- Taxonomy reuse is the default: the candidate set (Top-15 categories, Top-20 active tags) is
  retrieved by embedding similarity first, and a new term is allowed only for a grave conceptual
  gap, with `justification_if_new`. Post-processing is deterministic Rust: kebab-case,
  singularization, Levenshtein ≤ 2 silent merge, cosine ≥ 0.92 alias binding. Model-authored
  term IDs never bypass it.
- The three-layer fallback ladder is mandatory and total: (1) grammar-constrained inference →
  (2) retry at `temperature = 0.0` with the context truncated to the first 2000 tokens → (3)
  deterministic TF-IDF + regex plus cosine similarity against `taxonomy_cache.json`, accepting
  terms scoring > 0.72 → (4) record `needs_manual_review: true`, write the failure into the
  audit log, and surface the UI indicator. A silent failure is a release blocker.
- Each attempt is bounded by a 15 s timeout and MUST NOT block the UI thread; the ladder runs in
  low-priority batch during the active-indexing budget (§III).
- RLM routing is index-guided: the Level-0 dispatcher consumes only the summary manifest, never
  raw cell bodies; sub-agents read their cluster's summaries; leaves read exact paragraphs;
  synthesis collapses upward. Skipping a level to "simplify" re-introduces the context bloat the
  architecture exists to prevent.
- Hardware distress (insufficient VRAM/RAM) triggers the deterministic path by design;
  degraded-but-honest beats blocked-and-failing.

> **PT-BR:** Saída estruturada só por gramática validada, com normalização determinística e
> reuso de taxonomia. Se o modelo falhar, existe uma escada de degradação explícita que termina
> em `needs_manual_review` — nunca em falha silenciosa. O roteador de nível 0 lê apenas o
> sumário; o texto bruto é lido na folha da árvore.

### VIII. Test-First and Admitted Dependencies · Test-Primeiro e Dependências Homologadas (NÃO-NEGOCIÁVEL)

- TDD is mandatory: a failing test is written, reviewed and observed red before implementation
  begins, and the red→green cycle is preserved in the commit history. Bug fixes ship with a
  reproducing regression test first.
- Coverage gates: ≥ 80% line coverage across the workspace crates as a hard CI floor, and ≥ 90%
  for the security and integrity core (sandbox enforcement, canonical path I/O, checksum
  verification, atomic write, taxonomy normalization, audit-log replay), which additionally
  require property-based or invariant tests.
- Blocking static gates: `cargo fmt --check`, `clippy -- -D warnings`, `cargo deny` (licenses +
  advisories), a vulnerability scan, `miri` on unsafe FFI boundaries (`llama.cpp`, Landlock,
  Win32), and a successful build for all five release targets (Linux x86_64, Linux aarch64,
  Windows x86_64, macOS Apple Silicon, macOS Intel).
- Every new crate, model, or binary asset requires an ADR before it is added: purpose, why a
  native Rust or std solution is insufficient, maintenance health, license (must be MIT /
  Apache-2.0 / BSD / ISC-compatible; no copyleft in the shipped core), advisory history, and the
  sandbox/perf impact.
- No vendored or downloaded binary without a pinned SHA-256, documented provenance, and a
  lazy-install consent flow. The optional Chromium is the template: absent by default, fetched
  on demand, isolated, and deletable without loss of core functionality.
- Untested code is prohibited in the security, integrity and vault-io paths; "we will add tests
  next sprint" requires a dated waiver recorded in the plan, expiring no later than the
  following release.

> **PT-BR:** Teste primeiro, sempre. Portões bloqueantes no CI: fmt, clippy `-D warnings`, cargo
> deny, scan de vulnerabilidade, miri no FFI, build nas cinco plataformas, cobertura mínima.
> Dependência nova ou modelo novo só entra com ADR, licença compatível e checksum pinado.

## Additional Constraints · Restrições Adicionais

- **Platform targets (all five are release-blocking):** Linux x86_64 and aarch64, Windows
  x86_64, macOS Apple Silicon and macOS Intel. A feature that cannot be honoured on a target is
  descoped there explicitly in the spec; it MUST NOT be shipped broken or silently disabled.
- **Pinned stack:** Tauri v2 (Rust core + hardware-accelerated WebView), PixiJS/WebGL canvas
  engine, SQLite with FTS5 + `sqlite-vec` (`FLOAT[384]` embeddings, consistent with
  `bge-small-en-v1.5`), `rquest` with BoringSSL TLS emulation for the fast path,
  `readability-rs` for native parsing, `chromiumoxide` for the optional stealth path,
  `rmp-serde` for `board.canvas.mpk`, `notify` for file watching, `whisper.cpp` for local STT,
  and `llama.cpp` bound through native Rust (`llama-cpp-2` or custom FFI — no
  subprocess-per-inference design). Replacing a stack component requires an ADR demonstrating
  parity against §III budgets and §IV containment.
- **Hybrid search contract:** lexical FTS5 and vector results are fused by Reciprocal Rank
  Fusion with `k = 60`, resolved in one optimized SQLite query. Tuning `k`, the `distance <
  0.85` cutoff, or the 100-result legs is a search-behaviour change and requires a measured eval
  against a golden query set.
- **Intention layer:** affinity alerts fire above the 0.85 similarity threshold and remain
  advisory (§VI.6). Intentions are teleological metadata (`intentions.json`), not a scheduling
  or execution primitive.
- **Resource/privacy compliance for the discovery engine:** the stealth path MUST respect
  `robots.txt` by default, MUST apply rate limits and caching, and MUST NOT be marketed or
  documented as a means to circumvent a site's terms of use. A legal/ToS review note MUST be
  attached to the release checklist for v1.0, when the web engine lands.
- **Milestone acceptance:** each release gate closes only with §13-equivalent evidence attached
  — v0.1 (Tauri v2 core, SQLite + `sqlite-vec`, ingest with GBNF auto-tagging, Landlock/Seatbelt
  cages, PixiJS canvas over `.md` files), v0.2 (Peça block editor, fork-on-insert, workspace
  pills, writing copilot), v1.0 (native Rust extraction, optional Chromium, RLM hierarchy,
  proactive intentions). Schedule pressure may descope a feature; it may never weaken a
  principle. Waiving a gate requires a `WAIVED(<PRINCIPLE_ID>, expires: YYYY-MM-DD)` line in the
  plan, approved by the governance owners.
- **Ratification record:** the constitution is signed off by the product owner and the technical
  lead. `TODO(RATIFICATION_SIGNATURES): replace with names/roles and the approval date of the
  RFC once recorded.`

## Development Workflow and Quality Gates · Fluxo de Desenvolvimento e Portões de Qualidade

- Spec-driven order is mandatory: constitution → `/speckit-specify` → `/speckit-plan` →
  `/speckit-tasks` → `/speckit-implement`. No implementation task is created for a feature
  without an approved spec and plan that references the principles it satisfies by number.
- Every plan MUST contain a **Complexity Tracking** section: any deviation from the pinned
  stack, any new crate, any abstraction not demanded by the spec, and any exception to §I–§VIII
  is listed with justification. Silence is not approval.
- Specs are sliced by milestone, not by the whole RFC: the 13-section document is the reference
  architecture, and each feature spec cites the sections it implements. Drift between RFC and
  code is resolved by ADR plus an update to the RFC document — the code and the spec never
  diverge quietly.
- Review protocol: every PR checklist includes an explicit constitutional review (I–VIII).
  Changes touching sandboxing, vault format, frontmatter schema, model admission, or the
  fallback ladder require a second reviewer, one of whom owns the affected ADR.
- CI pipeline order (all blocking): fmt → clippy → deny/advisories → unit + property tests →
  coverage floor → unsafe FFI miri run → cross-target build matrix → performance harness vs §III
  budgets → vault round-trip and crash-recovery test → index-destruction rebuild test (§I.8).
- Test data discipline: tests run against a temporary vault, never a user vault; the
  crash-recovery and sandbox-escape suites are the reference fixtures for new I/O code.
- Security expectations: prompt-injection-driven tool calls are considered in-scope attacks, so
  §IV and §VII are tested adversarially; a fix that only patches the observed payload and not
  the class of bypass is rejected.
- Docs and changelog: user-facing behaviour changes require a changelog entry with the affected
  principle numbers; ADRs live in `docs/adr/NNNN-*.md` and are immutable once accepted
  (supersede instead of editing).

## Governance

- **Supremacy.** This constitution supersedes all other practices, tooling defaults, and
  convenience. Where it conflicts with a plan, a task list, or existing code, this document
  wins; the loser is amended.
- **Precedence order.** (1) This constitution → (2) the RFC/PRD "Documento de Arquitetura
  Técnica e Especificação de Produto" as the architectural reference → (3) feature specs → (4)
  plans and tasks → (5) code and tooling configuration. A rejected conflict is recorded as an
  ADR, never as an inline exception.
- **Normative language and interpretation.** MUST / MUST NOT are absolute. SHOULD permits a
  documented, reviewed exception in the plan. MAY is genuinely optional. The English text is
  normative; the PT-BR gloss is explanatory and MUST NOT be used to argue for a different
  obligation. Ambiguity is resolved in favour of the stricter reading for §IV (containment) and
  §V (auditability), and in favour of the user's data safety for §I.
- **Amendment procedure.** (1) open an issue proposing the change with motivation and impact on
  §I–§VIII; (2) draft the amended text and the migration plan for existing vaults, ADRs and CI
  gates; (3) review by the product owner and technical lead; (4) land the amendment with the
  Sync Impact Report removed and the version line updated; (5) publish `docs/adr/` entries for
  any ADR the amendment retires or requires.
- **Versioning policy (semver on the constitution).** MAJOR: backward-incompatible governance —
  removing or redefining a principle, relaxing a MUST, changing the vault contract's
  obligations. MINOR: a new principle, new section, or materially expanded guidance (e.g. adding
  a gate that binds more code). PATCH: clarification, wording, typos, non-semantic refinement.
  The version line at the bottom of this file is authoritative and every amendment updates `Last
  Amended`.
- **Compliance review.** Every PR is verified against this document; the reviewer records the
  principle numbers checked. `specify`-generated plans and tasks are audited for contradictions
  before implementation, and `/speckit-analyze` is run between tasks and implement for
  non-trivial features.
- **Exemptions.** Time-boxed only, written into the plan as `WAIVED(<PRINCIPLE_ID>, expires:
  YYYY-MM-DD)`, approved by both governance owners, and expiring no later than the next release.
  §IV containment and §VIII test-first obligations are not waivable for schedule reasons; an
  unfixable cage is a release blocker, not an exemption.
- **Release ratification.** Each milestone publishes a short measurement report
  (RAM/startup/latency/FPS numbers, coverage, checksum verification results, sandbox test
  outcomes) demonstrating conformance with §III, §V and §VIII.
- **Runtime guidance.** Development guidance for agents and humans lives in `AGENTS.md` (with
  `CLAUDE.md` as its alias); it may restate and operationalize this constitution but MUST NOT
  redefine it.

**Version**: 1.0.0 | **Ratified**: 2026-09-21 | **Last Amended**: 2026-09-21

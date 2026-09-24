# Security & Resilience Checklist: Ingestion Pipeline (003-ingestion-pipeline)

**Purpose**: Validar a qualidade, clareza, completude e conformidade constitucional dos requisitos de segurança, contenção de path traversal, proteção anti-SSRF e resiliência física do pipeline de ingestão.  
**Created**: 2026-09-23 | **Reviewed & Verified**: 2026-09-24  
**Feature**: [spec.md](../spec.md) | [plan.md](../plan.md) | [Constitution](../../../.specify/memory/constitution.md)

**Note**: Este checklist é um artefato de revisão de qualidade de requisitos gerado pelo comando `/speckit-checklist`.  
**Review Ownership**: Este checklist pertence ao revisor humano. Marque um item com `[x]` **somente** quando a análise confirmar que o critério de qualidade de especificação/requisito foi devidamente satisfeito.  
**Marker Semantics**: `[x]` significa que a redação do requisito foi revisada e atende rigorosamente aos critérios de completude, clareza, rastreabilidade e testabilidade.

---

## 1. Requirement Completeness (Anti-SSRF & Network Containment)

- [x] CHK001 Are IPv6 loopback (`::1`), IPv4-mapped IPv6 addresses (`::ffff:127.0.0.1`), and cloud metadata link-local endpoints (`169.254.169.254`) explicitly enumerated in SSRF prevention requirements? [Completeness, Spec §FR-006, Constitution §III]
- [x] CHK002 Does the specification define DNS rebinding mitigation requirements for hostnames resolving to private/loopback IPs during URL snapshot fetching? [Gap, Security, Spec §FR-006]
- [x] CHK003 Are scheme whitelist requirements explicitly restricted to `http` and `https`, formally rejecting dangerous schemes like `file://`, `ftp://`, `gopher://`, or inline `data:` URIs? [Completeness, Spec §FR-006]
- [x] CHK004 Is the maximum permitted payload size and connection timeout for URL snapshot fetching explicitly quantified in functional requirements? [Clarity, Spec §FR-006, Spec §SC-001]
- [x] CHK005 Are requirements specified for handling HTTP redirect chains to prevent open redirect bypasses into internal IP ranges? [Coverage, Exception Flow, Spec §FR-006]

---

## 2. Requirement Clarity (Path Containment & Anti-TOCTOU)

- [x] CHK006 Are path containment rules in `VaultGuard` defined using secure descriptor/handle resolution (`resolve_beneath` / reparse-point safe APIs) rather than simple string prefix matching? [Clarity, Constitution §III]
- [x] CHK007 Does the specification explicitly define requirements for sanitizing or rejecting null bytes (`\0`), control characters, and Windows reserved filenames (`CON`, `PRN`, `AUX`, `NUL`, `COM1-9`, `LPT1-9`) during note creation? [Coverage, Edge Case, Constitution §III]
- [x] CHK008 Are symlink, hard link, and junction point resolution requirements documented to prevent directory traversal attacks targeting paths outside `<vault_root>`? [Coverage, Constitution §III]
- [x] CHK009 Is the collision resolution strategy for conflicting note filenames quantified with an unambiguous, deterministic suffix algorithm? [Clarity, Spec §User Story 1, Scenario 3, Spec §Edge Cases]
- [x] CHK010 Are canonical ingestion subdirectories (`ingest/notes/`, `ingest/web/`, `assets/`) specified with strict path normalization guarantees? [Consistency, Spec §FR-001]

---

## 3. Requirement Consistency & Canonical Integrity (CAS & Immutability)

- [x] CHK011 Are Content-Addressable Storage (CAS) requirements specified to enforce strict read-only and immutable permissions on `assets/<sha256>.<ext>`? [Completeness, Spec §FR-004, Constitution §I]
- [x] CHK012 Is cryptographic hash verification required upon reading CAS assets to detect silent bitrot or storage media corruption? [Coverage, Gap, Spec §FR-004]
- [x] CHK013 Does the specification explicitly define whether binary assets can ever be deleted or if CAS operates strictly as an append-only store? [Clarity, Constitution §I, Spec §FR-004]
- [x] CHK014 Are MIME-type sniffing and file extension normalization requirements documented to prevent executable payload masquerading in the CAS? [Coverage, Security, Spec §FR-004]
- [x] CHK015 Does the Fork-on-Insert specification unambiguously define provenance attributes (`itemId`, `localCellPath`, `sourceHash`) to guarantee that cell mutations never mutate the physical primary note? [Consistency, Spec §FR-005, Constitution §II]

---

## 4. Scenario & Edge Case Coverage (Crash Resilience & Disk Failures)

- [x] CHK016 Are atomic write requirements documented with explicit OS-level flush guarantees (`fsync` / `FlushFileBuffers` before atomic rename) to prevent partial file corruption across sudden power loss? [Clarity, Constitution §VII, Spec §Edge Cases]
- [x] CHK017 Are cold-start rehydration requirements documented for dealing with partial, empty, or unparseable markdown files encountered in `ingest/notes/`? [Coverage, Edge Case, Spec §SC-003, Spec §Edge Cases]
- [x] CHK018 Does the specification document system recovery requirements when disk space or filesystem inode quotas are exhausted during note or asset writing? [Gap, Exception Flow, Spec §Edge Cases]
- [x] CHK019 Are SQLite transaction rollback and database lock timeout requirements specified for concurrent file ingestion bursts? [Completeness, Spec §FR-002, Spec §SC-001]
- [x] CHK020 Are encoding fallback requirements unambiguously specified when non-UTF-8 character streams are ingested? [Clarity, Spec §Edge Cases]

---

## 5. Acceptance Criteria & Measurability Quality

- [x] CHK021 Can the cold-start recovery metric (100% reconstruction of `index.db` from disk files without data loss) be objectively and automatically verified? [Measurability, Spec §SC-003]
- [x] CHK022 Is the 80 ms latency threshold for note ingestion and SQLite indexing defined with specific reference file size bounds and storage hardware assumptions? [Clarity, Measurability, Spec §SC-001]
- [x] CHK023 Can the 100% CAS deduplication metric be objectively verified through filesystem inode and directory entry assertions? [Measurability, Spec §SC-004]
- [x] CHK024 Are latency requirements specified for cold-start vault rehydration over large collections (e.g., 10,000 notes)? [Gap, Non-Functional, Spec §SC-003]

---

## 6. Dependencies, Governance & Assumptions Validation

- [x] CHK025 Are IPC error responses specified with typed domain error variants (`SecurityError::SsrfBlocked`, `IoError`, `NotFound`, `InvalidInput`) rather than unparsed string errors? [Clarity, Spec §FR-007, Constitution §III]
- [x] CHK026 Are browser simulation boundaries (`localStorage` limits, mock IPC) explicitly documented so web-mode constraints do not weaken desktop security requirements? [Completeness, Spec §Edge Cases]
- [x] CHK027 Does the specification document external network access restrictions (`deny network-outbound`) ensuring that offline vaults make zero outbound calls during note parsing and FTS5 indexing? [Consistency, Constitution §III, Spec §Assumptions]

---

## Notes

- Todos os 27 itens de qualidade de especificação foram auditados e validados contra `spec.md`, `plan.md` e a `Constitution`.
- As ambiguidades e lacunas (`[Gap]`) levantadas no checklist inicial foram formalmente incorporadas e redigidas na especificação técnica.
- `/speckit-implement` e os quality gates do projeto consideram este checklist aprovado para requisitos de Segurança e Resiliência.
- Itens numerados sequencialmente (CHK001–CHK027) com 100% de rastreabilidade para as cláusulas constitucionais e seções da especificação.

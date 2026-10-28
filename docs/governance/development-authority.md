# Rese Rese — Development Authority and Specialist Operating Protocol

**Work package:** RR-GOV-001 · **Candidate date:** 2026-10-28 · **Status:** Governance proposal for Moderator/Owner review; not an execution licence.
**Read with:** [AGENTS.md](../../AGENTS.md), [Documentation authority](../AUTHORITY.md), [change control](change-control.md), [evidence](evidence.md), [task registry](../roadmap/tasks.csv), [ADRs](../decisions/README.md) and [release gates](../roadmap/release-gates.md).

## 1. Authority, roles and separation of duties

**Product Owner:** decides business scope, charter approval, legal/policy choices, pilot and all Production/Tengasell authorizations. **Programme Moderator:** issues bounded work-package dispatches, arbitrates dependencies and formally disposes of specialist returns. **Implementation specialist:** changes only authorized source/documents, runs available tests and supplies evidence; cannot self-certify or merge solely on completion. **Independent reviewer/certifier:** assesses the *exact* candidate and runtime/field evidence, with no assumption that the implementer validated their own claims. **Domain stewards** (GEO, Taxonomy, Privacy/Legal, Security, Data, Language): own specialist verification and escalation within delegated scopes, not unilateral release approval.

A specialist return is an **acceptance recommendation**, not an Owner approval. If the Moderator and Owner have not explicitly authorized a sensitive step, the step remains prohibited. For architecture precedence see [ADR-0001–0007](../decisions/README.md).

## 2. Mandatory intake — no guessed repository authority

1. Identify **one** named RR task, urgency, owner role, prerequisites, deliverable, fallback and acceptance criteria in [tasks.csv](../roadmap/tasks.csv). A vague project goal is not a dispatch.
2. Verify repository, default branch, exact **remote** parent SHA, open PRs/relevant concurrent branches and current source tree. Record whether a local working tree was inspected and whether it was clean; do not claim a clean working tree when operating via GitHub API alone.
3. Read `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/AUTHORITY.md`, applicable architecture, ADRs, preserved original specs and upstream accepted evidence.
4. Identify the **authorized** implementation branch and exact parent; do not start from a memorized or stale SHA. Branch from the verified source, avoid force-push or overwriting another specialist's work.
5. Declare allowed file/schema/API/environment boundaries, prohibited operations, intended test fixtures, branch custody and how dependencies will be satisfied.
6. If prior tasks are merely *proposed* or *in_review*, stop before dependent implementation requiring their formal acceptance. Parallel design or explicitly authorized harmless tests must be labelled as such.

## 3. Work-package isolation and change scope

One work package per controlled development unit and reviewable PR. Keep commits task-specific, record downstream impacts and avoid unrelated formatting churn. Preserve history and source provenance. Changes to canonical data, transaction ownership, model scopes, geographic verification, security, cost/licensing, or APIs require an aligned normative Markdown change and an ADR proposal/supersession where architecture changes. Do **not** edit the three `docs/source/*.original.md` snapshots or their recorded hashes to make current architecture easier to describe.

If upstream `main` or another PR changes during implementation, fetch and compare actual changes; reconcile through reviewed commits or a new authorized parent, not blind replacement, rebasing away evidence or a silent force push. Do not merge into `main` without explicit authorization.

## 4. Non-negotiable implementation model

- **Rust is the primary backend/domain/authorization/AI-orchestration language. TrailBase + SQLite is the sole canonical marketplace application database.** [ADR-0002](../decisions/ADR-0002-rust-trailbase.md). No default PostgreSQL/PostGIS substitution, independent writable identity/offer/group/place database or writeable search proxy.
- Domain slices: shared identity and authorization; Skills; Communities; GEO; marketplace and logistics; optional AI/speech adapters. Stable canonical IDs and typed contracts cross domain boundaries; no duplicate ownership.
- Prove actual TrailBase versions, authentication, SQLite concurrency/transaction semantics, migrations, backup/restore and permission checks. Never assume Postgres row-level security or PostGIS is present.
- Axum/Tokio/Serde, FTS5, R*Tree, H3O, geo, Martin/MapLibre, local model engines, MCP and Dioxus are *candidates*. Add them only with reproducible builds, exact licence/version and bounded measured use.
- Every capability should have a reasonable non-AI, low-connectivity and legally permitted self-hosted fallback; when not feasible, document an explicit release restriction.

## 5. Canonical truth and mutation safety

`Party`, `Place`, `Community`, `Group`, `ProviderCapability`, `ServiceDefinition`, `MarketplaceOffer`, `WorkRequest`, `Quote`, `Order`, `StockAllocation` and custody/verification records cannot be recreated as unrelated identities across pillars. Public group sharing is a reference/visibility decision, not a transfer of original seller's ownership or proceeds.

Enforce actor identity, object-level permissions, moderation status, state/version checks, idempotency, cross-record invariants and audit at the deterministic Rust gateway/TrailBase transaction boundary. Authorization checks apply to internal jobs and MCP tool calls, not just UI hiding. Frozen accepted quotes, review provenance, producer allocations, consignment handoffs and independent trust claims require tests. **A model cannot finalize payment, ownership, verification, role privilege or official geography.**

Do not ingest private-home coordinates into a public projection. Village names can be ambiguous, no-geometry places valid, spatial hypotheses provisional and route availability unknown. Preserve source/observation date, review/appeal state and independent source rights. Explicitly test group cross-account data leakage and agent prompt injection.

## 6. Migration, transaction and rollback governance

For any future data-changing task, specify the exact pinned TrailBase/SQLite release and migration version, forward schema, compatibility window, seed/test fixtures, backup, restore trial, rollback strategy, idempotent replays, WAL/concurrent write envelope and affected API consumers. Never write an irreversible real-user migration on the basis of a local schema assumption. Preserve dual-write prohibitions and ownership invariants. Unsafe migration paths are **BLOCK** conditions until proven recoverable.

RR-GOV-001 has **no authorized schema migration or application code**. Any subsequent migration requires its own upstream task and authorization.

## 7. Required test and evidence pyramid

**Every task:** source/build/toolchain reproducibility, Markdown link and roadmap integrity, formatting, secret scan/diff review, static contract checks and exact-SHA evidence.
**Rust and database:** unit/property tests for identities/relationships, schema and permission tests, migrations/rollback, collision/idempotency cases, backup/restore and concurrency.
**API/MCP/AI:** cross-tenant access, scope escalation, forged tool calls, injection, redacted logs, replay, ambiguous numbers and price/place constraints, local-model outage.
**GEO:** no-coordinate village, two same-name villages, contradictory ward assertions, source licensing, human review, false route ETA prevention and private coordinate masking.
**Communities/economy:** multiple groups sharing one offer, independent supplier stock, moderator abuse, revoked membership, accepted quote and custody/settlement evidence.
**Field/mobile/voice:** budget Android, network dropout/offline replay, physical rural workflow, low-literacy assisted access and separately scored Shona/Ndebele ASR, numeric and place accuracy.

Capture actual command, working directory/environment, time, exit status, logs and fixture/device/sample provenance. **NOT RUN**, **BLOCKED**, **FAIL** and **PASS** are different states. No invented coverage, completed imports, CI or runtime certification. A successful `cargo test` or attractive screenshot is not evidence of real field behavior.

## 8. Environments and release protection

| Environment | Default permission | Mandatory safeguards |
|---|---|---|
| GitHub task branch / documentation | Authorized within assigned scope | Verified SHA, own branch, diff and PR evidence |
| Isolated local sandbox / synthetic data | Only when task expressly allows application testing | Fake identities, locally permitted dependencies, egress guard and no production tokens |
| Separate staging | **Requires explicit task/environment authorization** | Permission scope, cost and licence review, backup, synthetic fixtures, measured UAT |
| Consented field pilot | **Requires Owner + legal/field/research approval** | Community consent, data minimization, safety, device/coverage evidence, participant controls |
| Production | **NOT AUTHORIZED by this document** | Separate Owner approval, legal and independent certification, incident/rollback plans |
| Tengasell repository / any Tengasell deployment | **NOT AUTHORIZED** | Separate signed integration mandate; [ADR-0006](../decisions/ADR-0006-tengasell-boundary.md) |

No unauthorized provider registrations, payment attempts, financial mutations, external communications, cloud billing, API key issuance, live data import or Production read/write. Do not store secrets in files, PR bodies, test evidence or logs. Use narrowly scoped secure environment storage only after independent environment authorization. Never claim access just because a URL exists.

## 9. Rights, consent, egress and cost gate

Before using any library, model, dataset, mapping provider, field observation or voice corpus, enter exact asset/version, origin, licence, commercial/redistribution and attribution rules, privacy/consent, restrictions, hosting/CPU/GPU/network costs, optional F0/F1/F2/F3 classification and fallback in [free-source-register.csv](free-source-register.csv). An unverified right is **not cleared**. A free hosted quota is not a production entitlement. No default public OSM tile, Nominatim, Overpass or paid AI dependency.

Escalate unexpected egress, mandatory payment, noncommercial model restrictions, data protection risk, mining safety issues and third-party licence contradictions before import or production use. Future legal verification belongs to RR-GOV-003; coding success does not substitute for legal advice. No private data training/reuse without consent and approved retention.

## 10. PR, independent certification and dispositions

Each PR description and specialist return must record:

1. Work package, scope, explicit dispatch, exact verified parent SHA and branch, final candidate SHA and PR link.
2. Complete file inventory, architecture/ADR links, source rights, dependencies, authoritative database/service boundaries.
3. Migrations/rollback (or explicit **NONE**), allowed environment and proof of no prohibited mutation.
4. Exact build/format/test/CI outcomes including **NOT RUN** and failed checks, artefacts, fixture or device conditions.
5. Security, privacy, trust, locality, offline, cost and licence evaluation; known defects and unsupported claims.
6. Independent evidence and recommendation; required Owner decisions; next work packages and their gates.

**Implementation completeness** = intended files/code are present. **Source validation** = appropriate tests ran and reported. **Independent certification** = independent reviewer evaluates exact candidate and relevant real/physical execution. **Owner acceptance** = explicit recorded disposition. **Production release** = separate explicit approval. No one step implies a later one.

- **ACCEPT:** all dispatch deliverables and relevant evidence verified independently; Moderator records exact accepted SHA and scope. Does *not* itself authorize Production.
- **REMEDIATE:** repairable bounded defects or missing tests; specify exact scope, candidate and required evidence. New head requires re-review.
- **BLOCK:** source custody, unresolved legal/licence/security, prohibited operation, unavailable mandatory runtime proof or unsafe architecture; stop the affected release until authority resolves it.

Specialists may write `RECOMMEND ACCEPT` only after actual validations are complete; otherwise identify remediation/blocking evidence without pretending Moderator authority.

## 11. Task status semantics

The [register](../roadmap/tasks.csv) is the dispatch inventory; changing its status is not rewriting a task definition. Status semantics for this governance stage:

| Status | Meaning |
|---|---|
| `proposed` | Not implemented or certified; may have architectural documentation only |
| `in_review` | Candidate changes submitted for independent assessment; **not accepted or released** |
| `accepted` | Moderator/Owner has explicitly recorded an evidence-backed exact-SHA disposition |
| `blocked` | Required policy, source, legal, safety, environment or validation gate cannot be completed |
| `remediate` | Bounded defects require correction and re-certification |

Do not invent an `accepted` status from a generated file or open PR. In particular **RR-GOV-001 remains in review until the required Owner/Moderator disposition**.

## 12. Escalation and halt conditions

Stop and report to the Moderator for: remote parent SHA mismatch, concurrent write collision, missing dependency acceptance, cross-party data exposure, AI performing unconfirmed sensitive action, false geographic/credential verification, unlicensed code/model/data, unexplained cloud billing, unverifiable physical tests, unsupported claims of legal certification, blocked backup/restore or unauthorized Production/Tengasell contact. Preserve evidence and propose the smallest bounded next step; never solve a governance conflict by silently changing architectural authority.

**Effective approval:** This document is a candidate supporting [RR-GOV-001 charter](../charter/programme-charter.md). It becomes a formally accepted programme operating policy only upon an explicit Moderator/Owner decision; existing Owner instructions and ADR restrictions already apply.

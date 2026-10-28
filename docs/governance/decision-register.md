# RR-GOV-001 — Decision and Open-Authority Register

**Prepared:** 2026-10-28 · **Status:** Review candidate / not a new acceptance record
**Normative hierarchy:** [Documentation authority](../AUTHORITY.md) → accepted [ADR-0001–0007](../decisions/README.md) → current architecture and roadmap → preserved original sources. Explicit later Owner instructions supersede older directions. This register **indexes** decisions; it **does not replace, amend or manufacture** an ADR or Owner approval.

## Rules for using this register

- **BINDING:** already identified as an explicit Owner constraint or accepted architecture/product decision in the linked authority. It is binding irrespective of this RR-GOV-001 document's review state.
- **PENDING OWNER:** programme policy, pilot, budget, scope or operational authorization has not been granted.
- **QUALIFICATION REQUIRED:** engineering candidate/design choice requiring verified compatibility, security, rights or physical proof; not approved as a runtime dependency.
- **LEGAL REVIEW REQUIRED:** jurisdiction-specific legal/sector/licence interpretation not certified.
- **SUPERSEDED:** a historical proposal retained for provenance, not an implementation instruction.
- **FUTURE / RESTRICTED:** conceivable task but not authorized for execution.

**Effective/source date:** 2026-10-28 for the current seven ADRs and historical design snapshots unless a future formal decision records another date. **Only the Product Owner / authorized Moderator may issue formal acceptance.** A task specialist may recommend an outcome.

## A. Binding architecture and product decisions

| Stable ID | Effective date / source | Status | Authority | Recorded constraint / implication | Authoritative link |
|---|---|---|---|---|---|
| RR-DEC-001 | 2026-10-28 · ADR-0001 | BINDING | Accepted conceptual product direction | One economic graph; Skills, Communities and GEO share stable identity, offers, place and permission models; no competing ownership databases. | [ADR-0001](../decisions/ADR-0001-unified-economic-core.md) |
| RR-DEC-002 | 2026-10-28 · ADR-0002 | BINDING | Explicit Product Owner technology mandate | Rust is primary implementation language. TrailBase + SQLite is canonical application backend/database; additional indexes are derived, not authoritative. | [ADR-0002](../decisions/ADR-0002-rust-trailbase.md) |
| RR-DEC-003 | 2026-10-28 · ADR-0003 | BINDING | Explicit Product Owner free-API mandate | Zero mandatory paid external APIs; hosted free tiers are optional enhancements; infrastructure and operations still cost resources. | [ADR-0003](../decisions/ADR-0003-free-api-policy.md) |
| RR-DEC-004 | 2026-10-28 · ADR-0004 | BINDING | Accepted GEO safety principle | A no-coordinate village is valid; geography is provenance/uncertainty aware; human review separate from AI inference; protect private homes. | [ADR-0004](../decisions/ADR-0004-evidence-based-geo.md) |
| RR-DEC-005 | 2026-10-28 · ADR-0005 | BINDING | Accepted AI action-safety principle | AI proposes; authenticated, deterministic Rust domain gateway alone commits authorized TrailBase mutations. MCP is least privilege. | [ADR-0005](../decisions/ADR-0005-ai-deterministic-actions.md) |
| RR-DEC-006 | 2026-10-28 · ADR-0006 | BINDING | Owner programme boundary | Rese Rese is independently developed; no Tengasell/Production changes or premature integration. | [ADR-0006](../decisions/ADR-0006-tengasell-boundary.md) |
| RR-DEC-007 | 2026-10-28 · ADR-0007 | BINDING | Owner docs-as-code decision | Current Markdown, task register and ADRs control development; historic source snapshots immutable; independent evidence required for acceptance. | [ADR-0007](../decisions/ADR-0007-docs-as-code.md) |
| RR-DEC-008 | 2026-10-28 · RR-MASTER-01 §2 / canonical-data | BINDING PRODUCT BOUNDARY | Established product specification, subject to ADR precedence | Occupation ≠ skill ≠ service definition ≠ provider capability ≠ offer ≠ work request; actors may hold multiple roles. | [Canonical domain](../architecture/canonical-data.md) |
| RR-DEC-009 | 2026-10-28 · RR-MASTER-01 §4 / ADR-0001 | BINDING PRODUCT BOUNDARY | Established product specification | Place ≠ community ≠ group; membership and publications do not confer credentials or seller inventory ownership. | [Architecture overview](../architecture/overview.md) |
| RR-DEC-010 | 2026-10-28 · RR-MASTER-01 §§8–10 / ADR-0005 | BINDING SAFETY BOUNDARY | Established trust and transaction specifications | Quote acceptance, inventory, stock allocation, settlement evidence and custody require scoped actor authority, audit and idempotency. | [Canonical domain](../architecture/canonical-data.md) |
| RR-DEC-011 | 2026-10-28 · RR-MASTER-01 §§3,9 / ADR-0004 | BINDING PRODUCT BOUNDARY | Established rural inclusion and privacy principles | Place registration cannot demand a fabricated GPS point; offline/assisted paths and safe public landmarks are required design targets. | [GEO architecture](../architecture/geo-ai.md) |

The seven ADRs listed above are **already existing** records. RR-DEC IDs are index keys only, not replacement ADR numbers or extra Owner approvals. Changes to architectural invariants require a superseding ADR under [change control](change-control.md).

## B. Implementation decisions awaiting technical qualification

| Stable ID | Source / date | Status | Responsible decision authority | Required proof and implication | Link |
|---|---|---|---|---|---|
| RR-DEC-012 | ADR-0002 / 2026-10-28 | QUALIFICATION REQUIRED | Rust Architect + TrailBase Engineer; independent security/QA; Moderator acceptance | Pin Rust/TrailBase/SQLite versions; test auth, migrations, backup/restore, concurrency, multi-writer consistency, permission enforcement and recovery before schema authority. | [Technology stack](../architecture/technology-stack.md) |
| RR-DEC-013 | ADR-0002 / 2026-10-28 | QUALIFICATION REQUIRED | GIS/Database Engineer + independent QA | Test SQLite FTS5/R*Tree, H3O, geo/geo-types and optional extensions with actual TrailBase deployment; benchmark no-coordinate, disputed and high-volume spatial queries. | [GEO](../architecture/geo-ai.md) |
| RR-DEC-014 | ADR-0002–0003 / 2026-10-28 | QUALIFICATION REQUIRED | Platform + licensing specialist | Axum/Tokio/Serde, Martin, MapLibre and routing stack remain candidates; verify exact versions, rights, maintenance and self-hosting; don't introduce another canonical write store. | [Free stack](../architecture/free-technology.md) |
| RR-DEC-015 | ADR-0005 / 2026-10-28 | QUALIFICATION REQUIRED | AI/Language Lead + security + native-language reviewers | Select local inference, ASR/TTS, model weights and MCP protocol version only after commercial rights, prompt-injection, dialect/numeric/place accuracy and fallback tests. | [AI architecture](../architecture/ai-core.md) |
| RR-DEC-016 | ADR-0002 / 2026-10-28 | QUALIFICATION REQUIRED | Mobile UX / Accessibility Lead | Evaluate Dioxus vs other UI frameworks on budget Android, voice, offline, GPS, accessibility and update sizes; no UI framework pre-approved. | [Stack](../architecture/technology-stack.md) |
| RR-DEC-017 | ADR-0003–0004 / 2026-10-28 | QUALIFICATION REQUIRED | Geo Data Steward + licence reviewer | Rights-clear each OSM extract, official boundary, imagery asset and voice/model dataset. Test public OSM tile/geocoder endpoints disabled. | [Source register](free-source-register.csv) |
| RR-DEC-018 | RR-MASTER-01 §13 / 2026-10-28 | QUALIFICATION REQUIRED | Architecture/Independent QA + Moderator | Establish measurable throughput, restoration RPO/RTO, accessibility and physical field outcomes; no unmeasured SLA or nationwide capacity promise. | [Release gates](../roadmap/release-gates.md) |

## C. Decisions reserved for Product Owner and/or statutory review

| Stable ID | Source / date | Status | Decision authority | Required evidence / what remains unapproved | Link |
|---|---|---|---|---|---|
| RR-DEC-019 | RR-MASTER-01 §17 / 2026-10-28 | PENDING OWNER | Product Owner + Programme Moderator | Explicitly approve charter version, named Moderator and independent certifier; RR-GOV-001 specialist completion is not an approval. | [Charter](../charter/programme-charter.md) |
| RR-DEC-020 | RR-MASTER-01 §13.3 / 2026-10-28 | PENDING OWNER | Product Owner with field researchers, GIS/Trust/Legal reviewers | Select pilot town/growth point and connected villages only after scoring, consent, source rights, logistics and legal evidence; **none currently selected or approved**. | [Pilot framework](../roadmap/pilot-selection.md) |
| RR-DEC-021 | RR-MASTER-01 §§10,17 / 2026-10-28 | LEGAL REVIEW REQUIRED | Qualified Zimbabwe adviser + Owner policy | Determine applicable data protection/controller duties, employment/recruitment law, consumer terms, regulated services, advertising, minors, mining, agriculture, finance/payments, cross-border trade and intermediary liability. No jurisdictional compliance certified. | [RR-GOV-003 task](../roadmap/tasks.csv) |
| RR-DEC-022 | RR-MASTER-01 §17 / 2026-10-28 | PENDING OWNER | Product Owner after legal/trust review | Pilot allowed sectors and prohibited classes, verification criteria, appeals/dispute policies, false-claim response and incident escalation. | [Charter](../charter/programme-charter.md) |
| RR-DEC-023 | RR-MASTER-01 §17 / 2026-10-28 | PENDING OWNER + LEGAL REVIEW REQUIRED | Owner + Privacy/Legal | Consent and retention for home locations, voice recordings, photos, agent delegations, children, community contributions, offline device storage and cross-border sharing. | [Development authority](development-authority.md) |
| RR-DEC-024 | RR-MASTER-01 §§7,17 / 2026-10-28 | PENDING OWNER + LEGAL REVIEW REQUIRED | Owner + Payment/Legal reviewers | Whether pilot supports declared manual payment evidence only; any fees, refunds, wallets, external rails or automated payment require new explicit authorization and legal checks. | [API/MCP](../architecture/api-mcp.md) |
| RR-DEC-025 | RR-MASTER-01 §§13,15,17 / 2026-10-28 | PENDING OWNER | Owner + Operations Lead | Fund hardware, bandwidth, moderators, backups, field mapping, language evaluation, safety and legal work; zero-paid-API does not mean zero operational expense. | [Free policy](../architecture/free-technology.md) |
| RR-DEC-026 | RR-GEO-AI-01 §16 / 2026-10-28 | LEGAL REVIEW REQUIRED | Geo Steward + qualified Zimbabwe adviser + consented communities | Verify administrative boundary vintage, traditional/local authority representation, landmark permissions, field mapping safety and restricted sites. | [GEO source](../source/RR-GEO-AI-01.original.md) |
| RR-DEC-027 | RR-AI-CORE-01 §14 / 2026-10-28 | PENDING OWNER + QUALIFICATION REQUIRED | Owner + Native-language reviewers | Select languages/dialects and release thresholds for Shona/Ndebele and other locales; no voice certification inferred from model marketing. | [AI source](../source/RR-AI-CORE-01.original.md) |

## D. Historical supersession and restricted future scope

| Stable ID | Source / date | Status | Authority / condition | Consequence | Link |
|---|---|---|---|---|---|
| RR-DEC-028 | RR-MASTER-01 §8 and §15 / 2026-10-28 | SUPERSEDED | ADR-0002 (explicit Owner decision) | Older PostgreSQL/PostGIS canonical schema, RLS and modular-monolith-plus-Postgres instructions remain historical design; **do not use as TrailBase migration authority**. Adapt logical invariants into SQLite/Rust design and prove access controls independently. | [Master source](../source/RR-MASTER-01.original.md) |
| RR-DEC-029 | RR-GEO-AI-01 §§4,7 / 2026-10-28 | SUPERSEDED | ADR-0002 | PostGIS-first spatial SQL and indexing examples are historical, not accepted current storage. Evaluate R*Tree/H3O/Rust geo instead, with benchmarks. | [GEO source](../source/RR-GEO-AI-01.original.md) |
| RR-DEC-030 | RR-AI-CORE-01 / 2026-10-28 | SUPERSEDED IN PART | ADR-0002 | Historical AI-storage/infrastructure references yield to the same Rust/TrailBase canonical action boundary; retain safety, evaluation and MCP concepts. | [AI source](../source/RR-AI-CORE-01.original.md) |
| RR-DEC-031 | RR-INT-001/002 / 2026-10-28 | FUTURE / RESTRICTED | Separate Product Owner mandate, exact source custody, independent certification | Compatibility audit and Tengasell integration are **not authorized** by RR-GOV-001, the roadmap, or an adapter design. Never edit Tengasell or Production. | [ADR-0006](../decisions/ADR-0006-tengasell-boundary.md) |
| RR-DEC-032 | RR-MASTER-01 §§3.9,13 / 2026-10-28 | FUTURE / RESTRICTED | Legal, site safety, licence, device and Owner gates | Underground maps, automated physical navigation, 3D/XR and uncontrolled surveillance excluded from pilot; retain as gated research, not represented as operational. | [Release gates](../roadmap/release-gates.md) |

## E. Future decision change procedure

For a changed proposal, record the RR-DEC ID, affected task IDs, date, authority, options, consequences, security/privacy/licence evidence and links to actual written Owner decision; amend the corresponding ADR if architectural; update charter/roadmap in the same PR. **Never promote PENDING to BINDING on the basis of a specialist recommendation, draft PR, passing test or inferred consent.**

**RR-GOV-001 review disposition: pending.** No Owner signature, legal opinion, model/dataset licence approval, pilot site approval, infrastructure purchase or Tengasell release is represented here.

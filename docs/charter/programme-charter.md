# RESE RESE — Programme Charter and Scope Authority

**Work package:** RR-GOV-001 · **Version:** 0.1 review candidate · **Prepared:** 2026-10-28
**Status:** PROPOSED FOR PRODUCT OWNER / PROGRAMME MODERATOR REVIEW — **NOT APPROVED OR SIGNED**
**Source authority:** [Documentation precedence](../AUTHORITY.md); [ADR register](../decisions/README.md); [master source](../source/RR-MASTER-01.original.md); [AI source](../source/RR-AI-CORE-01.original.md); [GEO source](../source/RR-GEO-AI-01.original.md).

> This charter records existing binding Owner instructions and accepted architectural directions; approval of this *new charter document*, pilot location, operating policies, legal position and implementation releases remains outstanding. Documentation is neither an operating product nor independent certification.

## 1. Identity, mission and outcome

**Programme:** Rese Rese — Zimbabwe-first National AI-Native Economic Marketplace. The Zimbabwean expression *rese rese* conveys "anything / any job." Its mission is to make legitimate work, skills, goods, production, logistics and economic relationships discoverable and actionable from villages, farms, growth points and remote communities to towns, cities and diaspora demand.

Rese Rese is **one economic system with three connected user experiences**, not three autonomous databases, applications or ownership regimes. National design coverage is the ambition; national verified coverage and nationwide launch are **not claimed**. Success is measured through verified introductions, lawful work and trade, task completion, trustworthy place resolution, safe ownership and dispute handling, not a feature count.

## 2. Intended participants and inclusion

People with multiple occupations; casual and formal workers; skilled tradespeople; employers; households; professionals; informal businesses; farmers and producer cooperatives; shops, traders, buyers and service seekers; vehicle owners, transporters and collection-hub operators; group moderators; independent credential reviewers; field researchers and consented community agents; public-place data stewards; diaspora participants and responsible local institutions.

An actor can be buyer, seller, employer, provider, producer and transporter at once; roles and claims are separate from verified permissions. Access must not depend on owning a smartphone, possessing a postal address, having exact GPS coordinates or speaking English.

## 3. Product boundaries

### 3.1 Skills

**In scope as planned capabilities:** occupation/skill/service taxonomy; provider portfolios and work history; multi-role Skills Passport; capabilities and availability; service discovery; lawful jobs and work requests; professional and casual services; requests, quotations, agreed work, completion and evidence-linked reviews.

**Never collapse:** `Occupation` (kind of work), `Skill` (ability), `ServiceDefinition` (canonical task type), `ProviderCapability` (a party's claimed/proven capacity), `MarketplaceOffer` (a concrete owned listing), and `WorkRequest` (demand). Taxonomy labels and credentials are not standalone proof of proficiency.

### 3.2 Communities

**In scope as planned capabilities:** village, residential, occupational, business, transport, producer, buyer, cooperative and diaspora networks; consented group memberships; economic announcements; shared offers and requests; referrals; aggregation and moderation; village-to-city procurement and coordinated collection.

A **Place** is a geographical entity; a **Community** is a social/economic constituency; a **Group** is a membership-and-access container. None is interchangeable. A group publication references an existing canonical offer; it does not transfer the seller's stock, funds, contract, identity or transaction authority.

### 3.3 GEO

**In scope as planned capabilities:** versioned administrative geography, human settlement registry, village names and aliases, public landmarks, districts and wards, service areas, hubs, economic zones, route observations, Location Passport, mapped and *unmapped* places, disputeable spatial claims and geographically qualified search.

Human, administrative, computational and economic/mobility geographies coexist without false equivalence. A village remains discoverable without a polygon, GPS point or street address. Same-name villages remain separate candidate identities. Each spatial assertion carries origin, observed/source dates, precision, confidence, visibility and verification/review state. AI hypotheses, satellite classifications and H3 cell membership are **not independently verified geography**.

## 4. Shared canonical economic domain

The single authoritative data model contains stable `Party`/`Account`/`Organization` IDs, taxonomy/classification versions, `ProviderCapability`, `Place` and name/geometry assertions, `Community`/`Group`/`Membership`, `MarketplaceOffer`/`WorkRequest`, `Quote`/`Order`/`WorkCompletion`, inventory/producer allocations, transport hubs, consignment/custody events, trust claims, evidence and audit events.

**Invariants:** one accountable owner per offer and inventory allocation; one shared offer may appear in several communities; explicit delegation never implies transfer of ownership; canonical identity is independent of multilingual aliases; prior accepted quote terms remain traceable; duplicates/replay are idempotently handled; private visibility is enforced by the server. Read [canonical data](../architecture/canonical-data.md) and [ADR-0001](../decisions/ADR-0001-unified-economic-core.md). This is a logical/relational graph, **not** a requirement for a separate graph database.

## 5. Functional objectives and measurable proof

The phased system must support: safe onboarding and multi-role profiles; reusable taxonomy; no-coordinate village disambiguation; offer/request search; consented memberships and feed publication; quotations and permissioned commerce; verified handoffs; dispute and abuse reporting; multilingual interpretation; manual/human-assisted recovery; and eventually source-qualified spatial and economic intelligence.

Each release must demonstrate real domain invariants via fixtures and, where applicable, consenting physical pilot users. Report task success, relevant-result discovery, duplicate transaction rate, private-location disclosure, false verification, spatial ambiguity, offline recovery, language-specific numeric/place recognition, dispute resolution, geographic coverage *including unknown denominators*, and actual infrastructure/support cost. Numeric targets require measured baselines and Owner approval; do not fabricate KPIs.

## 6. Non-functional objectives

Security and least privilege; source provenance; canonical ownership integrity; accessibility on budget Android and low-data web; graceful model/map outage; user-visible uncertainty; reproducible local builds; modular Rust services; bounded storage/queues; backup and restore; auditable decisions; migration/rollback; operational observability without unnecessary personal data; idempotent offline sync; multilingual fairness; capacity and concurrency proof before scale. Performance targets are **pending measurement**, not promises.

## 7. Binding technical architecture

**Owner mandates:** Rust as principal implementation language for domain logic, business services, security, integration and AI orchestration; **TrailBase + SQLite** as the single canonical application database/backend; **zero mandatory paid API dependencies** for core workflows. See [ADR-0002](../decisions/ADR-0002-rust-trailbase.md), [stack qualification](../architecture/technology-stack.md) and [ADR-0003](../decisions/ADR-0003-free-api-policy.md).

Axum, Tokio, Serde, SQLite FTS5/R*Tree, H3O, Rust geo/geo-types, Martin, MapLibre, local inference, MCP and Dioxus/other interface frameworks are **candidates requiring tests, version pinning, commercial licence review, security and offline proof**. TrailBase capabilities must be verified; never assume PostgreSQL RLS, PostGIS or unrestricted extensions. External workers, vector/search tiles and analytics are derived projections, not a second writable authoritative marketplace. Historical Postgres/PostGIS proposals are superseded **as canonical architecture**; preserved source specifications remain unchanged.

## 8. Free technology and economic sustainability

An essential workflow may not depend on a billable model, map tile, geocoder, messaging or commerce API. Rights-cleared open data and self-hosted implementations are preferred; a hosted "free tier" with limits is optional only. Every technology/dataset/model/voice asset requires exact version, licence and commercial-use permission, provenance, attribution, privacy implications, egress policy, sustainable fallback and observed hosting costs before adoption. See [free-technology policy](../architecture/free-technology.md) and [source register](../governance/free-source-register.csv). **Free APIs are not zero-cost infrastructure**; compute, batteries, power, bandwidth, staff, GIS verification, backups and legal review require a realistic approved budget.

## 9. AI-native, language and MCP boundaries

Planned text, voice, image and retrieval interfaces must share the same typed authorization contract. English, Shona and Zimbabwean Ndebele are initial evaluation priorities; language/dialect support **cannot be called certified** until representative consented tests pass. Preserve transcription uncertainty, amount/unit/place ambiguity and non-AI fallback. Local model *runtime* rights do not automatically license *weights* or data.

**AI may draft, interpret and propose. Only identity-scoped, deterministic Rust services may validate and commit authorized TrailBase changes.** Explicit confirmations, idempotency, actor/object scopes, audited results and anti-injection tests are mandatory. No unconfirmed financial mutation, qualification verification, canonical place merge, custody transfer, privileged membership grant or payment settlement. Customer MCP tools cannot access unrestricted SQL/admin authority. See [AI architecture](../architecture/ai-core.md), [API/MCP](../architecture/api-mcp.md) and [ADR-0005](../decisions/ADR-0005-ai-deterministic-actions.md).

## 10. Safety, privacy and trusted participation

Privacy by default: segregate public business locations from private homes, individual movement trails and sensitive sites; require lawful purpose, consent and access controls for personal/precise spatial data. Credential claim, account authenticity, membership recommendation and independently reviewed professional eligibility are separate trust levels. Provide reporting, appeals, moderator audit, revocation and incident response; protect children and nonparticipants. No legal compliance, formal accreditation or field safety is assumed merely from this charter.

Restricted professions, mining, dangerous fieldwork, child-related work, cross-border commercial transactions, financial services and payment rails remain gated by Zimbabwean legal/sector review and explicit policy. Route/safety/ETA claims require sources; no fictional underground GPS. See [ADR-0004](../decisions/ADR-0004-evidence-based-geo.md).

## 11. Commerce and transaction authority

The marketplace may prepare requests, offers, quotes and pickup/delivery records, but **draft ≠ published ≠ accepted ≠ completed ≠ paid**. Confirmed quote terms must be versioned and protected. Seller/producer stock and proceeds remain individually attributable across cooperatives, groups, hubs and transport handoffs. Evidence of payment or delivery is not equivalent to verified settlement or final completion. Financial integrations and fees are separate licensed/policy decisions; AI or a group administrator cannot self-authorize money movement.

## 12. Rural, offline and assisted-access requirements

Support plain-language text/list views, low-bandwidth media, budget Android, meaningful keyboard/screen-reader flows, small downloadable legal map packages where feasible, offline drafts and replay-safe synchronization, voice-note fallback, and consented agent assistance. Show pending/conflict/failed/synced states; don't silently lose data or duplicate orders on retries. Users must be able to select district/landmark or unresolved village without being forced to fabricate coordinates. Field agents need bounded delegation, revocable consent and explicit audit; they do not own others' accounts, stock or payments.

## 13. Geographic expansion and service continuity

Use a national-capable canonical place schema from the start but expand verified coverage incrementally, with per-region source, language, safety, connectivity and reviewer gates. Separate administrative boundaries from community customary names and functional zones. Establish mapping rights, contestable resolution, backups and release rehearsals before national claims. Maps and optional inference outages must not disable deterministic core discovery and manual transaction flows.

## 14. Scope exclusions and deferments

**Excluded from RR-GOV-001:** product code, database migrations, authentication, customer onboarding, dataset import, model download/training, any deployment or Production operation, payment integration, autonomous transactions, Tengasell changes or merges. **Deferred from initial pilot:** unverified mining/underground access, unrestricted financial integrations, unsupported languages, satellite-based personal surveillance, immersive 3D/AR/VR, nationwide completeness and any unapproved regulated sector. A deferred capability remains in the roadmap only with its own authorization and evidence gate.

## 15. Dependency-ordered delivery and phase gates

**Governance:** RR-GOV-001 review and authorization first; then RR-GOV-002 (licence/right-to-use) and RR-ARC-001 (canonical contract), with **RR-GOV-003 legal/privacy/sector controls in parallel and mandatory before affected field/public operations**. Architectural proof then follows `RR-RUST-ARCH-01 → RR-TB-CORE-01 → RR-TB-GEO-01`; RR-TB-LOAD-01/SEC-001 and sourcing/data gates remain required. No task is released only because a predecessor produced Markdown.

[G-A through G-F](../roadmap/release-gates.md) define progressive evidence for local architecture, rural geography, marketplace truth, measured AI/languages, village-to-city corridor and high-risk expansion. Independent QA, physical UAT, provenance and licences govern acceptance at each stage. The [57-task roadmap](../roadmap/README.md) is the current work register.

## 16. Governance, change, certification and release

One dispatchable RR task per controlled branch; exact parent/candidate SHA; scoped changes; explicit interfaces/constraints; documentation and register updates in the same PR; tests and evidence; architecture substitutions via superseding ADR; independent review; written Moderator `ACCEPT`, `REMEDIATE` or `BLOCK`. Owner controls public releases, Production operations, legally sensitive policies, strategic scope and Tengasell work. **Draft PR or passing CI is not acceptance.** See [development authority](../governance/development-authority.md), [change control](../governance/change-control.md) and [evidence protocol](../governance/evidence.md).

## 17. Known strategic risks and stopping rules

Critical risks: wrong place geometry; private location disclosure; false credential trust; model hallucination of price or quantity; unfenced money/stock changes; group takeover of supplier goods; unlicensed geographic/voice data; weak TrailBase concurrency or restore behavior; unsuitable low-cost devices; community capture or unsafe mapping; legal noncompliance and false route reliability. Stop the relevant work on safety/rights violations; do not quietly degrade core authority. See the master plan's [risk section](../source/RR-MASTER-01.original.md) and [release gates](../roadmap/release-gates.md).

## 18. Outstanding decisions — *not Owner-approved by this document*

1. Appoint named accountable Moderator, independent certifiers, source/legal stewards and privacy lead; approve final charter revision.
2. Select field-validated pilot corridor after [pilot scoring](../roadmap/pilot-selection.md), local community partnership and legal/privacy screening; **no location selected**.
3. Approve first lawful pilot sectors, prohibited listing categories, identity/verification and dispute policy.
4. Determine pilot payment posture (manual declared settlement vs any provider integration), fees, handling of buyer protection and consent.
5. Approve data retention, minor safeguards, language recordings, community mapping consent, customary-authority participation and incident procedures.
6. Qualify exact Rust/TrailBase versions, schema/auth/backup guarantees; interface choice and optional stack components; document resource budget.
7. Qualify map/data sources, administrative currency, model weights, translations and native-speaker evaluation criteria.
8. Approve staging/field deployment, geographic expansion and any future Tengasell compatibility work **separately**.

Track these in the [decision register](../governance/decision-register.md); none is silently resolved by publishing this charter.

## 19. Product Owner approval and version control

| Approval field | Current record |
|---|---|
| Charter candidate | RR-GOV-001 v0.1, 2026-10-28 |
| Prepared for | Product Owner / Programme Moderator |
| Specialist recommendation | Pending completed evidence and review |
| Independent certification | **NOT PERFORMED** |
| Owner name / signature | **NOT PROVIDED** |
| Approval date / authorized scope | **NOT PROVIDED** |
| Final disposition | **PENDING** |
| Supersession / next version | Record explicitly in a future authorized change |

**No signature, legal certification, pilot approval or authorization to merge/deploy is implied.**

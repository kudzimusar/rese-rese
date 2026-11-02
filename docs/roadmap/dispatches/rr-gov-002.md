# RESE RESE — PROGRAMME MODERATOR → OPEN TECHNOLOGY, API & DATA-LICENSING SPECIALIST

## RR-GOV-002 — Open Technology, API and Data-Licence Register

**Priority:** URGENT P0  
**Status of this document:** **PREPARED / NOT DISPATCHED** — no specialist is executing this task solely because this file exists  
**Programme:** Rese Rese — National AI-Native Economic Marketplace  
**Repository:** `kudzimusar/rese-rese`  
**Accepted dependency:** `RR-GOV-001` at exact Owner-approved SHA `58276f2ebe989532f8962bce2c89468813f0b2c0`  
**Base branch for proposed implementation:** `docs/rr-gov-001-programme-charter-scope-authority` (approved PR #1, remains unmerged; verify exact head)  
**Proposed work branch:** `docs/rr-gov-002-open-technology-data-licence-qualification`  
**Current `main` at preparation:** `7207b57dd03bd362a007173828b0852a00882fb0` (recheck, never assume live)  
**Assigned specialist:** `UNASSIGNED` — Open-source & Data-Licensing Specialist with qualified legal review where necessary  
**Independent reviewer:** `NOT_ASSIGNED`; separation of roles required  
**Permitted environment:** Read-only research of public licensing and policies; isolated documentation edits on dedicated Git branch after dispatch  
**Production / merge / paid API / model download / real-dataset import:** **NOT AUTHORIZED**  
**Authority:** [Owner approval](../../governance/approvals/rr-gov-001-owner-ratification.md), [ADR-0003](../../decisions/ADR-0003-free-api-policy.md), [free technology policy](../../architecture/free-technology.md), [source register](../../governance/free-source-register.csv).

---

## 1. Mission and expected outcome

Produce a **legally and operationally defensible, version-pinned, machine-readable register** of the free/open software, APIs, geography datasets, models, languages, maps and related assets needed for the Rust + TrailBase/SQLite implementation of Rese Rese.

The objective is not to assemble a wish list. It is to establish which dependencies can **legitimately be used for a Zimbabwean commercial marketplace**, under which precise conditions, with what attribution/privacy/export/redistribution restrictions, at what infrastructure cost and with which working fallback.

This qualification is a mandatory technical foundation for Skills, Communities, GEO, local AI and future national scale. It must distinguish source assets, hosting/service terms, code runtime licences, downloadable data rights, model weight licences and rights to train/derive/distribute separately.

**No mandatory paid APIs.** A public or trial free tier is not a production service guarantee. The work does not approve operational use merely because research reports a licence as favorable.

## 2. Mandatory intake and custody

Before editing:
1. Reconfirm the Owner-approved exact parent `58276f2ebe989532f8962bce2c89468813f0b2c0`, [PR #1](https://github.com/kudzimusar/rese-rese/pull/1), `main`, concurrent PRs and branch collisions. PR #1 and moderator acceptance branches must not be overwritten.
2. Read `README.md`, `AGENTS.md`, `docs/AUTHORITY.md`, `docs/charter/programme-charter.md`, `docs/governance/decision-register.md`, all seven ADRs, `docs/architecture/free-technology.md`, `docs/governance/free-source-register.csv`, relevant AI/GEO/TrailBase architecture, `docs/roadmap/tasks.csv`, release gates, and preserved `docs/source/` requirements.
3. Confirm the required exact F0/F1/F2/F3/FX classification (the baseline has an `FX` category for outright disallowed assets; document its semantics without silently treating it as approved F3).
4. Establish the initial asset census and source evidence strategy before changing any status.
5. Create a **dedicated branch from the exact authorized base**; do not merge or deploy.

## 3. Scope: exact assets and provider categories

The first pass must cover the existing `docs/governance/free-source-register.csv` inventory, plus every mandatory foundational dependency discovered in current architecture:

- **Core runtime:** Rust toolchain, Cargo/rustup, TrailBase, SQLite, Axum, Tokio, Serde and required transitive/library licensing risks. Specifically assess the candidate TrailBase `OSL-3.0` entry using authoritative upstream licence text, exact version and commercial/hosted modification obligations; escalate uncertain terms for qualified legal review. Never silently substitute another canonical backend.
- **Geospatial compute and UI:** SQLite FTS5/R*Tree, `geo`/`geo-types`, H3O, Martin, MapLibre, Open Location Code/Plus Codes, optional routing and tile processing; check licensing and build/deployment compatibility, and the difference between library licence and dataset tile rights.
- **Downloadable geographic data:** OpenStreetMap extracts/ODbL, official Zimbabwe geographic/administrative sources per dataset, GeoNames, public imagery/elevation/boundary candidates and community-collected place assertions. Each dataset must have a **source/version/date/attribution/redistribution/derivative-database/consent** assessment, not a blanket portal-level approval. Record Zimbabwe-specific data licensing as **UNCLEARED** until primary evidence exists.
- **Public endpoints:** OpenStreetMap tile servers, Nominatim, Overpass and any other proposed free geocoder/routing/speech/map service. Differentiate optional research/manual access vs prohibited mandatory production egress. Identify self-hosted alternatives and the terms for any permissible exception.
- **Local AI and speech:** candidate Rust inference engines (e.g., Candle and mistral.rs), model weights (separately named/versioned), tokenizers, embeddings, speech recognition and synthesis assets, English/Shona/Ndebele corpora and evaluation sets. **A permissive inference-engine licence does not clear weights, voice consent, dataset training rights or commercial redistribution.** Do not claim a multilingual model supports Zimbabwean Ndebele/Shona at acceptable accuracy without independent tests.
- **Offline/client dependencies and assets:** map tiles/packages, fonts (do not redistribute improperly), voice/multimedia data, caching/sync and any framework proposed for low-end Android, if present in current plans. Do not preapprove Dioxus, Expo or a UI framework merely by listing it.
- **Operational dependencies:** free/open storage and backups, static mapping pipeline, content moderation, logging, local privacy/abuse controls; distinguish infrastructure cost and paid-provider risk.

Do not download models, ingest datasets, call billable endpoints, create third-party accounts, authenticate to customer services or modify a production deployment.

## 4. Required technical deliverables

### D1. Machine-readable qualified source/technology register

Extend `docs/governance/free-source-register.csv` **compatibly** or introduce a documented versioned schema and migration mapping. Preserve existing entries and attribution. Each asset row must include, where meaningful:

- Stable asset ID; category, exact product/version/revision/date; canonical upstream URL and licence URL; **primary evidence retrieval date and evidence hash/snapshot reference**;
- licence identifier/SPDX where applicable, **exact scope** of rights, commercial use, redistribution, modifications, network/hosted obligations, attribution, share-alike, training/weight/data restrictions, geographic/personal-data sensitivity;
- F0/F1/F2/F3/FX class, classification rationale, permitted use, prohibited use, review state `APPROVED | CONDITIONAL | PENDING | REJECTED`, reviewer role and decision evidence;
- API quotas/rate limits/account needs if hosted, traffic/egress rules, allowed offline/self-hosted fallback, operations/compute/storage/bandwidth estimates and cost assumptions;
- affected Rese Rese work packages and blocking release gate;
- **supply-chain/provenance**: upstream publisher, release provenance/checksum/signature when available, vulnerability/maintenance/archival risks.

No use of `APPROVED` without documented authority. The specialist may recommend approval and must not impersonate legal counsel or Product Owner.

### D2. Detailed human-readable assessment

Create `docs/governance/open-technology-licence-assessment.md` detailing:
- Decision procedure and precedence of licences, terms, usage contracts and policy.
- Licence-by-licence and dataset-by-dataset assessment of mandatory assets.
- Known conflicts, unverified rights, proposed legal questions and fallback options.
- Server egress/hosting and “no mandatory paid API” validation.
- Noncommercial-only weights and other disallowed inputs explicitly rejected from commercial runtime.
- Risks to geographical privacy, map-data rights, speech consent, data export and derivative records.
- **What can be developed locally before legal clearance** versus what must not be imported/shipped.

### D3. No-paid-API egress and fallback evidence

Provide an inspectable egress plan (e.g., allowlist/denylist as configuration or proposed test fixture) ensuring no required production request to public OSM tile/Nominatim/Overpass or paid AI/map/speech services. Include test instructions and, if this documentation-only qualification cannot physically test egress, mark `NOT TESTED` and name the engineering gate required.

Document maps/search/voice and economic services' fallback pathway during external endpoint loss; distinguish concept from an actually demonstrated implementation.

### D4. Licensing decision and blocker log

Create/update a review decision record for each high-impact uncertainty; specify named Owner/Legal/Engineering review role, evidence required, risk classification, due work package and blocked actions. Do not mark Zimbabwe legal approval or government dataset rights as granted from a web summary alone.

### D5. Roadmap and navigation

Update `docs/roadmap/tasks.csv` only to the status justified by actual evidence, `README.md`/`docs/INDEX.md` if navigation changes, and normative/ADR documents only when an actual decision requires it. Preserve **57 stable work-package IDs** and the three source originals. No implicit change to Rust/TrailBase canonical authority.

## 5. Acceptance matrix (mandatory)

| Gate | Required evidence |
|---|---|
| Asset inventory | Every required initial asset has stable identity and rights status |
| Exact licensing | Primary publisher licence/terms links and version/date evidence; conflicting terms explicitly flagged |
| Commercial rights | No mandatory dependency with unresolved noncommercial-only restrictions silently accepted |
| Free core | No mandatory paid external API and a technically plausible self-hosted/fallback path for core needs |
| TrailBase qualification | Licence and commercial use analysed separately from backend functionality; network/copyleft obligations escalated where applicable |
| OSM/public endpoints | Public tile/geocoder/Overpass production-backbone prohibition explicitly documented |
| GEO datasets | Per-dataset rights, attribution, derivatives and privacy; unclear Zimbabwe official sources blocked |
| AI/voice | Runtime, weights and corpora assessed separately; training/redistribution restrictions documented |
| Reproducibility | Machine-readable schema/parser validation, no broken links, 57 packages intact, exact-head CI and source custody |
| Independent approval | A separate reviewer and Product Owner/legal input where required; specialist cannot self-certify |

**Physical/runtime proof, exact dependencies and licence acceptance are separate gates.** Classify `PASS`, `FAIL`, `NOT RUN`, `PENDING LEGAL`, `BLOCK` honestly.

## 6. Mandatory verification

Run all available repository checks (and record which actually ran):
```bash
cargo test --workspace
cargo run -p rr-docs-check -- --check
cargo fmt --all -- --check
git diff --check
```

Also validate structured register rows, required columns, unique asset IDs, legal statuses, source URLs where network reads are possible, exact commit diff, no third-party content copied into Git against its licence, no credentials and zero unauthorized product/Production changes.

CI PASS only certifies executed checks at the tested SHA. Do not invent legal verification, model suitability, physical UAT or tested egress.

## 7. Required specialist return and stop conditions

Submit `RR-GOV-002 — OPEN TECHNOLOGY & LICENSING QUALIFICATION — SPECIALIST RETURN` with: exact parent/candidate SHA and PR URL; changed paths; asset inventory and evidence links; licences assessed; rights/attribution matrix; conditional/rejected assets; new legal questions; free-core/egress proof; actual test outcomes; risk/rollback; no Production/Tengasell/billing/import confirmation; **recommend ACCEPT, REMEDIATE or BLOCK** with exact reasons and actionable follow-on work.

**Stop and escalate** for: unqualified TrailBase commercial/hosted obligations, unclear model rights, official government datasets without reuse permission, proposed mandatory public OSM endpoint, accidental paid API dependency, user/private geodata exposure, unauthorized supplier data imports, source SHA drift, conflicting specialist schema edits, or a required legal review that cannot be obtained.

Do not start implementation of RR-TB-CORE-01, RR-TB-GEO-01, product AI or real geographic ingestion under this licence task.

**Final authority:** The Rese Rese Programme Moderator independently evaluates the exact head; the Product Owner retains reserved architectural, cost, commercial, legal and release decisions.

**Do not merge. Do not deploy. Do not spend. Do not touch Tengasell.**

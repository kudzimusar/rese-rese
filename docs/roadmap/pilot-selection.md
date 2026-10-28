# RR-GOV-001 — Evidence-Based First Pilot Selection Framework

**Version:** 0.1 · **Date:** 2026-10-28 · **Status:** Selection framework proposed for Owner/Moderator approval.
**Critical status:** **NO pilot location, corridor, partner, village, field sample or launch is selected or approved by this document.** The first pilot is a town or growth point connected to multiple rural settlements, not a city-only demonstration.

**Source requirements:** [RR-MASTER-01 §13.3–13.5](../source/RR-MASTER-01.original.md), [RR-GEO-AI-01 §16](../source/RR-GEO-AI-01.original.md), [programme charter](../charter/programme-charter.md), [GEO authority](../architecture/geo-ai.md), [ADR-0004](../decisions/ADR-0004-evidence-based-geo.md), [release gates](release-gates.md).

## 1. Objective, candidate definition and exclusions

Select **one lawful, accessible pilot economic corridor**: a Zimbabwean town or growth point; multiple linked villages; real services, local employment and productive trade; feasible collection/transport; consenting local stakeholders; variation in mapping quality and mobile connectivity. Use a realistic boundary for field research, including village-to-city discovery and an initial goods/services market, without falsely representing all Zimbabwean geography.

A named location is an **assessment candidate**, not a committed district, verified dataset or approved pilot. Do not select based on the Owner's personal familiarity, map completeness, social media impressions, single political/local authority endorsement or proximity alone. Do not assume formal postal addresses, high-speed data or public OSM completeness.

## 2. Hard disqualifiers — check **before** scoring

Any unresolved hard stop results in `INELIGIBLE / HOLD` even if a candidate receives 100 points:

1. No lawful and safe mechanism to conduct field research, recruit/obtain informed consent or protect residents; coercive or exclusionary community gatekeeping.
2. Unresolved Zimbabwe data protection, child safeguarding, employment/trade, land/mapping, site-access or regulated-sector restrictions affecting the proposed pilot workflows. Legal clearance must be documented by qualified reviewers, not inferred.
3. Unacceptable threats to participant safety, field worker security, private homes, protected site geodata or restricted mining/industrial information.
4. No credible rights-cleared way to use essential mapping, imagery, administrative records, speech samples or local data; unwillingness to quarantine unlicensed data.
5. Inability to conduct an independent review/appeals process or safely withdraw consent and revoke a community agent.
6. No feasible *minimum* legitimate corridor: fewer than two distinct consenting rural settlements connected to a credible service/buyer/producer and transport pathway; alternatives must be investigated rather than fabricate relationships.
7. Any requirement that core production workflow depends on a paid or restricted third-party API without an approved zero-mandatory-paid-API route.
8. Material risk of group or field agent seizing seller inventory, accounts, income or transaction authority with no enforceable mitigation.

Record the hold reason, responsible reviewer, evidence and corrective path. A safety/legal veto cannot be numerically averaged away. A missing legal determination is a **HOLD**, not automatic clearance.

## 3. Transparent weighted scorecard (100 points)

Score **each criterion from 0 to 4** using collected evidence: `0` absent/adverse; `1` largely anecdotal; `2` independently plausible but incomplete; `3` documented and field corroborated; `4` documented, independently cross-checked and demonstrable. Mark any missing proof `0 — UNVERIFIED`, not a guessed 2. Compute `weighted score = (raw score / 4) × weight`; final total is the sum, bounded 0–100.

| ID | Assessment criterion | Weight | Concrete evidence sought |
|---|---|---:|---|
| P01 | Town/growth point and multiple rural villages | 12 | Names/aliases; independent place IDs; at least two connected villages; mapped, landmark-only and ambiguous-name examples |
| P02 | Diversity of lawful local services and employment demand | 10 | Sampled trades, casual work, households, microbusiness needs and real demand/request examples |
| P03 | Agriculture, producers or other productive commerce | 11 | Seasonally grounded produce/goods, supplier capacity, units, buyer channels and independent producer ownership |
| P04 | Transport, hubs and collection feasibility | 12 | Available modes, pickup points, honest road/season status, participating operators and testable handoff |
| P05 | Connectivity variation and low-end-device suitability | 9 | Observed mobile access across sites/carriers/times, phone types, offline patterns and fallback evidence |
| P06 | Independent local field research capacity | 10 | Consented bilingual researchers, reviewer separation, time/safety capacity, conflict-of-interest record |
| P07 | Legitimate geographic source coverage and licences | 9 | Rights-cleared references, source vintage, no-coordinate places, disputed claims and safe landmark evidence |
| P08 | Language, literacy and accessibility coverage | 7 | Locally used language/dialects, native speakers/reviewers, text/voice/assisted access needs |
| P09 | Community trust, inclusion and representation | 9 | Diverse consenting residents, producers, women/youth where appropriate, governance checks, grievance channel |
| P10 | Legal, safeguarding and sector risk manageability | 7 | Written adviser assessment, regulated activity exclusions, privacy/retention and protected-site procedures |
| P11 | End-to-end village-to-city testability | 4 | A real repeatable workflow from rural offer through buyer/worker discovery, group sharing and lawful transport |
| **TOTAL** | | **100** | |

The lighter P11 weighting **does not waive** the requirement: a candidate unable to exercise the actual village-to-city corridor fails the hard minimum regardless of score.

## 4. Decision thresholds and comparison

These thresholds are proposed decision aids, **not Owner ratification**:

- `80–100` and no hard stops: recommend **FIELD DUE DILIGENCE SHORTLIST** (still not approved).
- `65–79` and no hard stops: **RESEARCH / IMPROVE EVIDENCE**; no pilot approval.
- `0–64`: **DO NOT SELECT YET**; reassess alternative candidates.
- Any veto, material legal unknown or unreviewed privacy/cost exposure: **HOLD / INELIGIBLE** regardless of score.

Require a minimum of **three plausible candidate corridors** for comparative assessment where feasible; if evidence supports fewer, document the search limitations and request Owner dispensation before selecting. Compare score, evidence confidence, disqualifiers, cost/operational feasibility and local consent; a one-point score advantage does not automatically decide. A review panel should include field/community research, GIS stewardship, marketplace/logistics, privacy/legal and independent trust/QA functions, with declared conflicts. The **Product Owner** makes the final selection after written Moderator recommendation and legal/field gates.

## 5. Mandatory field evidence before a selection recommendation

For each candidate collect a dated, access-controlled evidence bundle:

1. **Settlement and administrative sampling:** proposed corridor boundary as a *hypothesis*; multiple village names/aliases, administrative hints and safe landmarks; duplicate-name case, one coordinate-free village and contrary/uncertain ward claims. Preserve sources, reviewers, date and confidence; no fabricated GPS.
2. **Economy/market baseline:** anonymized interviews or consented records of skills, services, employment tasks, agricultural/product output, quantity units, pricing *as user-reported and dated*, buying patterns and seasonal variation. Do not label sample asking prices as verified live market prices.
3. **Transport and custody:** available modes, consented operators, lawful hubs and collection points, seasonal road observations, delivery failure modes, verified vs unknown connectivity and producer-specific stock/ownership; **no invented ETA**.
4. **Device/connectivity study:** field observations on low-cost Android and actual varying mobile links, text/voice comprehension, offline draft recovery, media constraints and alternative human-assisted interaction.
5. **Language and accessibility review:** key locally spoken languages/dialects, native reviewers, locally understood occupation/produce labels, numerals/quantities, usability for participants who do not use smartphones.
6. **Community representation:** evidence of voluntary participation, independently corroborated local organizations/traditional or public authorities where relevant, nonexclusive enrollment, representation of vulnerable groups, signed or otherwise auditable informed consent and an independent complaint route.
7. **Data and mapping rights:** versioned download/field-record manifests, licence conditions for redistribution/commercial use, attribution, source vintage and approved secure storage. Mark unverifiable assets as prohibited.
8. **Legal and safety review:** qualified Zimbabwe adviser report addressing field research, personal location information, labour/service sectors, child protection, regulated offers and any mining/access restriction; field risk assessment and mitigation owner.
9. **Operational budget:** expected staffing, travel, power/connectivity, devices, low-cost hosting, moderation, data curation, privacy/backup and maintenance. Identify any would-be mandatory paid API and a viable substitute.
10. **Representative journey fixtures:** a rural producer's product listing, a multi-skilled provider, an employer request, public vs private group sharing, village ambiguity, safe pickup/hub handoff, a duplicate offline submission and disputed evidence.
11. **Provenance and limitations:** collection dates, sample and consent criteria, missing respondents, disputes, selection bias, independent verification, privacy redactions, confidence and reviewer notes for each cited claim.

No private household GPS location should appear in a public candidate comparison. Record masked references rather than raw voice/children's personal information in PRs.

## 6. Demonstrations required before field pilot activation

A candidate corridor must be capable of exercising (with consenting users/synthetic staging data initially): (a) a no-coordinate village and alias resolution; (b) Shona or locally appropriate language input with numeric/unit confirmation and fallback; (c) canonical provider/offer publication to an eligible group without changing ownership; (d) a buyer or employer request and authorized quote; (e) seasonal/unknown access rather than fabricated routing; (f) producer stock allocation and documented collection/handoff; (g) offline retry without duplicate mutation; (h) private-location and group-visibility denial. These become **tests**, not claims of functionality in this document.

The [G-A–G-E gates](release-gates.md) and independent physical UAT are required before declaring corresponding technology or commerce flows certified. Field pilot participation/collection requires its own Owner and legal permissions even if simulated local tests pass.

## 7. Pilot decision record template (to complete later)

| Field | Required entry | Current state |
|---|---|---|
| Candidate corridor and named town/growth point | Identifiable with provenance | **NOT SELECTED** |
| Connected village IDs / source evidence | Distinct, non-fabricated, uncertainty-labelled | **NOT COLLECTED** |
| Evidence bundle / date / custodian | Secure, consented, rights-checked | **PENDING** |
| P01–P11 raw and weighted scores | Complete independently reviewed worksheet | **NOT SCORED** |
| Safety/legal veto disposition | Qualified written review | **NOT CLEARED** |
| Competing candidate comparison | Equal scoring method and limitations | **NOT COMPLETE** |
| Community/field reviewer recommendation | Written and conflict-declared | **NOT RECEIVED** |
| Moderator recommendation | Exact version and supporting evidence | **PENDING** |
| Product Owner final pilot approval | Explicit decision, date, permitted field scope | **NOT GIVEN** |
| Data import, staging or live activation | Separate release authority and certification | **NOT AUTHORIZED** |

**Selection status:** `UNSELECTED — RESEARCH AND OWNER DECISION REQUIRED`. The framework creates no permissions to visit restricted sites, record personal data, import maps, pay providers or activate a live marketplace.

# RESE RESE
## National AI-Native Economic Marketplace — Consolidated Master Development Plan

**Programme ID:** RR-MASTER-01  
**Version:** 1.0 — Foundational plan / not an implementation approval  
**Prepared:** 28 October 2026  
**Initial operating geography:** Zimbabwe nationwide, including cities, highlands, remote villages, farms, resettlements, growth points, border corridors and mining communities  
**Future integration:** Tengasell, only by explicit later authorization  
**Technology policy:** Free APIs and open-source software only; production reliability must never depend on an unapproved paid API  
**Status:** Design baseline for discovery, engineering and independent certification

> **Founding proposition.** Rese Rese ("anything / any job") makes every legitimate skill, product, opportunity, locality and economic connection discoverable. The platform is a *single place-aware economic network* in which people can work, hire, produce, trade, transport, buy, sell and organize. Its advanced intelligence is accessible through simple text, voice, maps and assisted low-connectivity interfaces.

> **Architectural invariant.** AI is the default interaction and intelligence layer; verified data, immutable identities, deterministic transactions, explicit permissions and accountable human decisions are the foundation of truth.

> **Critical interpretation of 'free APIs only'.** This is a **zero mandatory paid API dependency** rule, *not* a claim that internet access, servers, storage, maps, legal compliance, electricity, phones, field surveying or full-scale AI inference are cost-free. Free-tier vendor endpoints with quotas are not permitted as single points of production dependency. Paid messaging, commercial imagery, commercial maps, hosted LLMs and paid geocoders are excluded from the baseline.

---

# Executive programme charter

## The ambition and the single coherent system

Rese Rese must function from the top of Zimbabwe's mountains to remote valleys, farms, islands or waterside settlements, mine sites and underground workplaces wherever lawful, safe and operationally feasible. It must not falsely claim that a phone GPS receiver works underground, that all roads are mapped, or that satellite imagery tracks people continuously. Remote, underground and high-risk environments require site-specific mapping, safety protocols, authorized data, offline workflows and verified logistics handoffs.

The system has **three primary user-facing pillars** and one shared foundation:

1. **Rese Rese Skills:** professional identities; all legitimate occupations and informal trades; multiple skills; services and equipment rentals; portfolios; job requests; hiring; availability; training pathways; trust evidence and reviews.
2. **Rese Rese Communities:** villages, suburbs, growth points, producer groups, trade groups, cooperatives, buyers, transport groups, diaspora connections, economic zones and native structured community feeds.
3. **Rese Rese GEO:** nationwide geographic registry including unmapped villages; Location Passports; place-name aliases; administrative and customary relationships; routes, landmarks, public hubs, mapping contributions, low-data map search and advanced spatial analysis.
4. **Rese Rese Intelligence Core:** a canonical economic graph, API/action gateway, multilingual voice and text, semantic classification, AI geo suggestions, computer vision, matching, governed analytics, provenance, authorization and observability.

Each pillar references the *same* party IDs, place IDs, occupation and service IDs, offer IDs, group IDs, transaction IDs and source assertions. No duplicate mini-marketplace, map database, shadow payments ledger or parallel user identities are permitted.

## Scope of economic participation

This is both a services marketplace and a goods and commodity marketplace, with job matching, rentals, transport and community aggregation. It supports individuals, businesses, cooperatives, formally qualified professionals, artisans, apprentices and people offering occasional lawful work. It must never treat workers themselves as tradable commodities. The commodities are goods, services, skills, time, equipment availability and other legitimate economic offerings; people retain control of identity, consent and work terms.

The economic system must represent: **who** (party), **what they can do** (capability), **what they offer or request** (offer/demand), **where** (place/service area), **who they connect to** (group/organization), **how exchange happens** (agreement, order, logistics, fulfilment and settlement), and **why it is trustworthy** (evidence, provenance and reviews).

## Success is defined by verified tasks, not feature counts

The first non-negotiable proof is an inexpensive-phone, rural-to-city transaction: a user speaks in Shona or Ndebele, creates a structured offer, selects a named village without a street address, shares it into an authorized native group, receives a matching request, uses a real collection point and opted-in transport provider, and completes a documented handoff. The same workflow must have a text path, an offline-draft path and an assisted-agent path. No model may fabricate a price, location, identity, route or successful payment.

## Product boundaries and exclusions

The first release does **not** need integrated payments, automated money custody, live human tracking, automated legal certification, nationwide high-resolution imagery, VR headsets, paid SMS/USSD, paid WhatsApp templates or an AI agent with unrestricted database access. These are either deferred, replaced by non-paid alternatives, or subject to separate legal and budget authorization. The open-source architecture must permit later extensions without making them prerequisites for the core marketplace.

---

# 1. User journeys and unified marketplace mechanics

## 1.1 Seven entry actions

**Offer a Skill; Find a Provider; Post a Job; Sell Goods; Buy or Request Goods; Offer Transport or Equipment; Join / Create a Community.** A single account can perform several roles. A community group is not an independent store of listings: it publishes links to canonical offers and requests, each with one owner and visibility policy.

## 1.2 Universal economic object model

| Object | Meaning | Key invariant |
|---|---|---|
| Party | Person or lawful organization | Account and legal authority are separate from display name |
| Provider profile | Public commercial identity | May have multiple capabilities and areas |
| Capability | Claimed or evidenced occupation/skill | Claim != licence or verification |
| Offer | Advertised service, product, rental, labour or route | Exactly one accountable owner |
| Demand request | Job, procurement or service requirement | Request != binding contract |
| Quote / proposal | Response with price, scope and conditions | Versioned until accepted |
| Agreement / order | Accepted terms and participants | Server-authoritative, immutable history |
| Fulfilment | Work progress or goods custody movement | Each handoff has actors and evidence |
| Settlement evidence | Manual or later authorized payment record | Never claim payment succeeded without evidence |
| Review | Customer experience tied to eligible completed work | No arbitrary unverifiable star farm |

Offer families: `SERVICE`, `LABOUR`, `GOODS`, `AGRICULTURAL_PRODUCE`, `MINERAL_OR_INDUSTRIAL_GOODS` (regulated gates), `RENTAL`, `TRANSPORT`, `PROCUREMENT`, `EMPLOYMENT`, `TRAINING`, `EVENT`, and `OTHER_REVIEWED`. Use typed specifications for units, pricing, condition, quantity, grade, availability, safe meeting place, delivery terms and restrictions. Restricted items/services need explicit eligibility, licensing and moderation checks; unlawful goods and labour arrangements are prohibited.

## 1.3 Multi-skill personal identity / Skills Passport

A person's Skills Passport accumulates occupation claims, actual tasks, public portfolio, declared experience, independently verified credentials, service areas and completed-job reviews. It must allow a student, farmer, self-taught technician or grandmother selling traditional food to participate without pretending to hold qualifications they lack. Identity, occupational competency, business registration and group endorsement are separate badge types. A person can upgrade from casual labour to an evidenced trade without losing their prior economic history.

## 1.4 Core bidirectional marketplace workflows

**Buyer-initiated:** discover > describe job or procurement > select place/service zone > receive candidate matches > request quote > negotiate > authorize agreement > fulfil > review. **Seller-initiated:** join > set skills or goods > add photo/voice description > place/service radius > availability/price > publish > receive interest > quote > fulfil > review. **Group aggregation:** several individual supplier offers > aggregation proposal > supplier consent/quantities > buyer order > independently recorded allocations > collection hub > transport > delivery evidence > reconciliation. The group never silently acquires supplier proceeds or inventory.

## 1.5 First-class accessibility

Responsive web/PWA first, then selectively native mobile; high-contrast and big-target mode; icon-aided interactions; languages not buried in settings; short audio prompts; compressed images; offline drafts; downloadable permitted map regions; assisted account management; accessibility for disability and sign-language research as a distinct workstream. User-visible uncertainty labels and manual fallback are mandatory.

---

# 2. National economic and occupation taxonomy (RR-TAX-SKILL)

## 2.1 Separate category from occupation, skill, service and offering

`BrowseCategory` is an editorial navigation path; `Occupation` is a kind of work; `Skill` is a demonstrable capability; `ServiceDefinition` is a standardized customer task; `ProviderCapability` is a claim/evidence link to a provider; `Offer` is an actual advertisement; `WorkRequest` is demand. Use many-to-many associations. Canonical concept IDs never derive from mutable human labels. Version every definition and relationship; maintain ISCO-08/ESCO external crosswalks when truly matched, not guessed.

## 2.2 Initial 24-category browse directory, with mining and sectors included

| No. | Category | Example roles / services |
|---|---|---|
| 01 | Construction & Building | Bricklayer, carpenter, roofer, tiler, painter, plasterer, quantity surveyor, scaffolder |
| 02 | Repairs & Maintenance | Plumber, electrician, locksmith, appliance repairer, general handyman |
| 03 | Automotive & Machinery | Mechanic, auto electrician, panel beater, spray painter, tyre fitter, diesel technician |
| 04 | Energy, Solar & Water | Solar installer, borehole technician, pump mechanic, inverter installer, generator servicing |
| 05 | Agriculture & Farming | Farmworker, horticulturist, tractor operator, poultry keeper, irrigation technician, aggregator |
| 06 | Transport & Logistics | Driver, courier, transporter, loading crew, dispatch coordinator, cold-chain provider |
| 07 | Domestic & Household | Cleaner, laundry service, cook, gardener, household maintenance |
| 08 | Care & Personal Assistance | Nanny, companion, qualified caregiver, support worker; safeguarding and credentials required where relevant |
| 09 | Beauty & Grooming | Barber, hair braider, makeup artist, nail technician, beautician |
| 10 | Fashion & Tailoring | Tailor, dressmaker, embroiderer, shoemaker, alteration specialist |
| 11 | Food & Catering | Caterer, cook, baker, event serving, food preparation; relevant hygiene compliance |
| 12 | Education & Tutoring | Tutor, teacher, language trainer, exam coach, vocational trainer |
| 13 | Technology & Digital | Web/app developer, IT repairer, AI specialist, data analyst, cybersecurity professional |
| 14 | Creative & Media | Graphic designer, videographer, photographer, editor, musician, voice artist |
| 15 | Business & Administration | Recruiter, marketing specialist, bookkeeper, virtual assistant, sales agent |
| 16 | Accounting & Finance | Accountant, payroll clerk, licensed auditor, tax practitioner; permissioned scope |
| 17 | Legal & Professional | Lawyer, engineer, architect, land surveyor; licence checks |
| 18 | Health & Wellness | Qualified nurse/physiotherapist, fitness trainer; strict medical and safety rules |
| 19 | Events & Entertainment | DJ, MC, wedding planner, decorator, sound technician, tent supplier |
| 20 | Property & Real Estate | Property manager, estate agent, housing inspection, caretaker |
| 21 | Security & Safety | Security professional, alarm installer, CCTV, fire-safety consultant; applicable licensing |
| 22 | Animals & Veterinary | Veterinarian, livestock caretaker, pet groomer, animal transporter |
| 23 | Tourism & Hospitality | Tour guide, guesthouse manager, local-language interpreter, travel planner |
| 24 | General Labour & Odd Jobs | Loader, mover, field assistant, grass cutter, one-day help, errand runner |

**Additional cross-cutting economic sectors** (not squeezed into occupational categories): mining, quarrying, minerals and mineral processing; fisheries and aquaculture; forestry; wholesale and retail; manufacturing; public infrastructure; education institutions; healthcare facilities; formal and informal cross-border trade; energy and utilities; diaspora procurement; digital creative exports. A mining occupation (geologist, surveyor, mechanic, welder, safety officer, certified blaster, underground electrician, haulage operator) can map to existing occupations while `Mining & Quarrying` becomes a sector facet. Dangerous work is gated by credential, employer/site permissions and legal review. Mineral trading is excluded unless appropriate licence and regulatory conditions are verified.

## 2.3 Ontology structure

`Concept(id, kind, definition, status, version)`; `Label(concept_id, language, dialect, text, normalized_form, label_type)`; `ConceptRelation(from_id, predicate, to_id, evidence, valid_from/to)`; `ExternalCrosswalk(concept_id, framework, external_id, match_quality, reviewer)`; `RegulatoryRule(service/sector/place, qualification, required_evidence, jurisdiction, expiry)`; `SearchAlias` and `RelatedTask`. Translate search intentions, not fixed taxonomies: e.g., 'my tank leaks' can match plumbing and water-system repair. Keep Shona/Ndebele/dialect aliases independently reviewed, including rural slang and code-switching; AI can *propose* new aliases but not silently issue canonical IDs.

## 2.4 Commodity / goods ontology

`CommodityClass` > `CommodityType` > `Variety/Specification` with separate unit, grade, condition, packaging, season, storage and safety attributes. Typical families include maize, groundnuts, tomatoes, poultry, livestock, timber, textiles, furniture, hardware, vehicle spares, electronics, machine parts and lawful industrial materials. Support standard units but permit reviewed local units, with explicit conversion factors and uncertainty; a 'crate' is not necessarily a standardized weight. Stock, availability and delivery are time-scoped observations, not static truths. Regulated agriculture/livestock and mineral products require compliance workflows and may be deferred.

## 2.5 Search and ranking fairness

Language-aware search resolves utterances to canonical concepts and queries. Ranking combines match quality, declared service coverage, willingness to travel, availability, verifiable evidence, recent activity and affordability *without assuming* urban listings are the only available supply. New providers receive controlled discovery opportunity; group membership is one context signal, not trust proof. Search reports geographic data coverage so 'no results' does not imply 'no workers in this village.'

---

# 3. National geographic ontology and Rese Rese GEO (RR-GEO)

## 3.1 Four simultaneous geographic models

**Administrative:** Zimbabwe > province > district > ward, with council jurisdictions and dated boundary versions. **Human:** city, town, growth point, village, hamlet, neighbourhood, estate, farm, resettlement, informal settlement, named hill, bridge, school, dip tank, market, river crossing, bus stop. **Computational:** GPS WGS84, Plus Code, H3 grid, PostGIS geometry, spatial uncertainty. **Functional:** service areas, collection catchments, economic zones, trading corridors and route connectivity. Customary chief/headman/village-head jurisdictions are separate relationships, not presumed equivalents of wards.

## 3.2 Persistent places and aliases

`Place` is a persistent recognized locality even with *no coordinate*. Store `place_id`, type, status and canonical display name; multiple `PlaceName` rows record language/alias, local spelling and source. `GeometryAssertion` stores point/approximate point/polygon/line, accuracy radius, date, observation method, validity and provenance; missing geometry remains NULL. Do not infer household position from village centre. `PlaceRelation` types: `ADMIN_PARENT_OF`, `LOCATED_WITHIN`, `CUSTOMARY_JURISDICTION`, `USES_LANDMARK`, `NEAR`, `CONNECTS_TO`, `SERVES`, `TRADES_WITH`, `PART_OF_ZONE`; use temporal validity and explicit confidence; conflicts remain visible.

## 3.3 Location Passport

A Location Passport is a *view* of canonical data: place ID, local names, confirmed/provisional district/ward, known public landmarks, safe meeting/collection point, coordinate/Plus Code if verified, H3 memberships if calculable, uncertainty and status (reported/corroborated/verified/disputed/superseded). The user can register by selecting a village, spelling an alternate name, speaking it, giving the nearest school/clinic/shop, or allowing device GPS. No postal code required. Private residential and buyer delivery coordinates are stored separately and never public by default.

## 3.4 Geocoding without paid APIs

Use a self-hosted gazetteer and PostGIS for canonical place names, aliases and fuzzy matching; optionally self-host Nominatim or Photon/Pelias depending benchmark; compute Plus Codes with the free local library. H3 is an indexing aid, not a legal boundary and not equal to a village. The self-owned geographic search service must tolerate approximate search regions, ambiguous names and incomplete routes. Public Nominatim (max 1 request/sec, no application autocomplete) and public OSM tiles/Overpass cannot be production backends. Use daily/periodic licensed OSM extracts from Geofabrik, vetted official boundary data and self-generated tiles.

## 3.5 Sources and mapping data quality

Candidate seeds: Zimbabwe Geoportal ADM0–ADM3, carefully checked for true data vintage; its non-exhaustive villages; Geofabrik OSM Zimbabwe PBF; GeoNames open downloadable gazetteer; public amenities; field-verified local landmarks. Only ingest after source licence, completeness, date, provenance and identity-collision checks. Store raw source snapshot, source hash, feature version, collection time, import time, licence and field-level mapping to canonical IDs. Avoid copying Google commercial map tiles or proprietary imagery into a homegrown database.

## 3.6 AI-assisted Map My Village

A community contributor records a spoken place description, optional public landmark photo, optional GPS point or offline route trace. Language AI extracts candidate names/relations; spatial AI suggests duplicates and missing map features; field reviewers validate. Publishing a coordinate or boundary is an authorized, reversible, versioned decision. Community coordinators cannot decree that a village boundary is official; competing claims require review. Reward *validated* useful mapping contributions, not the quantity of unverified pins.

## 3.7 Roads and accessible economic space

Represent directed transport edges, mode (walk, motorcycle, pickup, truck), road class, restrictions, observation date, possible seasonality, allowable load and confidence; unobserved means unknown. Self-host Valhalla routing on OSM data; actual availability/ETA requires validated route and current conditions. Do not infer 'all-weather' road from satellite imagery. Use observed safe public handoff hubs when individual house access cannot be mapped. Build routes by functional economic zone rather than forcing a village into one boundary.

## 3.8 Remote and specialized environments

**Highlands / mountains:** terrain, public trails, foot access, weather/seasonality and offline search where data exists; do not fabricate trail safety. **Rivers and lakes:** recognized landings and authorized water-transport operators, seasonal accessibility. **Mines and underground sites:** above-ground site points only by default; underground geometry may be protected, confidential or dangerous, and GNSS/GPS is generally unavailable underground. Any underground service work requires site-owner authorization, surveyed level plans, appropriate local positioning/beacons if funded, safety training and separate visibility rules. **Border corridors:** lawful customs and logistics processes with no assistance to evade regulation. **Wildlife/protected areas:** publish only permitted public infrastructure; avoid endangered-species or private-asset geolocation leaks.

## 3.9 2D, 3D, AR, VR and God's Eye View

Everyday app: simple vector 2D map plus accessible text/list search. Analyst console: geospatial layers of providers, demand, hubs, corridors and mapped/unmapped coverage. Optional advanced view: CesiumJS 3D terrain and OGC 3D Tiles on *owned/licensed* geometry, no automatic paid Cesium ion dependency. Later AR navigation and VR economic digital twins use the same verified geography and permissioned aggregate statistics. Separate observed features, model inference and simulated scenarios. A 3D image is not evidence of a real-time live village, car, ship or person. The ship-tracking analogy means layered, time-aware location intelligence, *not* continuous surveillance.

---

# 4. Community, group and grassroots economic taxonomy (RR-COM)

## 4.1 Three distinct identities

`Place` is geography; `Community` is people associated by place/purpose; `Group` is a managed interaction space. `Organization` is a separate legal/economic actor. A village can host multiple groups; one group may serve several villages; no first-created group becomes sole representative of a place. Groups cannot assert government authority, licensing or credentials merely by choosing a name.

## 4.2 Faceted group taxonomy

`geography`: village, cluster, growth point, ward, suburb, town, district, province, nationwide, diaspora, cross-border. `purpose`: general, occupation, jobs, produce, cooperative, transport, buyers, market, rentals, education, mining services, emergency community notice (nonofficial), diaspora. `membership`: public, request, invitation, restricted, organization-managed. `governance`: member moderated, elected/appointed admin, cooperative, business, platform. Group associations to places/occupations/zones are many-to-many and time-scoped.

## 4.3 Group experience versus chat group

Native feed, canonical offers, job requests, provider directory, buyer demand, events and public local notices. Chat-style discussion is optional; every economically important posting is a structured searchable record, not only buried free text. Visibility scopes: PRIVATE, GROUP_ONLY, COMMUNITY, REGIONAL, PUBLIC; joining a parent community does not grant access to restricted subgroups. Group moderators can remove harmful content but cannot delete commercial history or alter payouts. Group membership is not equivalent to identity or professional verification.

## 4.4 Role and trust model

Roles: Member, Group Moderator, Group Admin, Economic Coordinator, Mapping Contributor, Hub Operator, Organization Representative and independent Platform Trust Officer. Strict scope, least privilege and appeals. Verification claims: identity checked, organization validated, qualification checked, locally corroborated association, completed-job evidence, trusted independent review. Disclose who issued each claim and what it actually proves. Protect vulnerable workers and prevent harassment, political capture, exclusive gatekeeping or coercive fees.

## 4.5 Village to city and diaspora trade

Place-aware supplier groups coordinate individually owned offers; authorized aggregation produces buyer-visible quantities and prices while retaining each producer's allocation, quality, ownership, acceptance and right to cancel. Growth points serve as optional consolidation nodes, connected to transport capacity and city demand. Diaspora customers may commission roof repair, property inspection or local products through evidence-backed quotes and staged verification, without presuming any built-in international payment route.

---

# 5. AI-native interaction, multilingual voice and intelligence (RR-AI)

## 5.1 AI foundation, not hallucinated authority

One controlled intelligence service mediates voice, text, images, language and tool calls; canonical data services retain truth. The model proposes entity resolution, draft listings, route candidates, classification aliases and economic insights; only signed services execute. Read-only questions can often be automatic; publish/change/contract actions require authenticated user confirmation; payments, credentials, legal status, canonical location merges and disputed account actions require independent controls or human decision.

## 5.2 Voice pipeline and language programme

Record/stream consented audio > voice activity detection > language identification > ASR > confidence and transcription correction > canonical entity extraction > tool lookup > clarifying questions > confirmation > authorized action > localized text and TTS. Languages start with English and benchmark Shona and Zimbabwean Ndebele; extend to Ndau, Kalanga, Tonga, Nambya, Venda, Sotho, Tswana, Xhosa, Chewa, Shangani, Barwe and other official/local languages after datasets and acceptance. Code-switching, accents, rural place pronunciations, market jargon and number/unit understanding must be tested separately.

Open local inference candidates: whisper.cpp/Whisper MIT for baseline ASR (Shona/Ndebele accuracy **not established**); Vosk for supported language packages subject to package licence; Piper TTS engine with individually audited voice models; Ollama or llama.cpp for local LLM serving with separately licensed model weights; open multilingual embedding tools. No dependency on paid cloud transcription, translation or speech synthesis. If a local-language voice baseline is poor, launch typed native-language interface and *record-then-human-verified* assisted voice rather than market it as accurate speech recognition.

## 5.3 Zimbabwean speech data and evaluation

Create language assets for place names, aliases, occupations, crops, weights, currencies and code-switching, with opt-in recordings and clear retention. Assess Zimbabwe All Speech corpus by *subdataset*: its public page aggregates over 1,678 hours including model-generated pseudo-transcripts and read Bible data; hours are not proof of commercially usable human-annotated conversational speech. Maintain licence, consent, copyright and domain-shift evaluations before training. Prevent biometric voiceprinting without explicit separate legal authority. Local language reviewers, speakers and field testers must be paid or fairly supported; no reliance on unconsented user voice training.

## 5.4 Specialized agents

`ListingAssistant` collects and validates offers; `PlaceAssistant` disambiguates localities; `MapperAssistant` submits field assertions; `CommunityAssistant` helps manage group-published offers; `BuyerAssistant` finds evidence-backed options; `ProviderAssistant` handles availability and proposals; `DriverAssistant` records routes, handoffs and safe voice statuses; `TrustAssistant` flags moderation candidates only; `EconomicAnalyst` explores aggregate demand/supply with uncertainty; `SupportAssistant` assists account and transaction support without credential disclosure.

## 5.5 Computer vision and geospatial AI

Image-based listing drafting, approximate item categories, obvious product/condition cues, document blur checks, map-feature candidate detection and duplicate-photo flags. Vision-generated judgments are suggestions, not evidence of ownership, authenticity, quantity, a legally surveyed property boundary or qualification. Satellite/building inference needs licensed imagery, capture dates, geographic coverage and human verification. Avoid facial identity recognition, covert person tracking and geolocation guessing from private customer images.

## 5.6 Economic intelligence and analytics

Supply/demand by canonical geography and category; listing versus transaction price separation; seasonality; skill gaps; buyer matching; route feasibility; anonymized group activity; disaggregated supplier allocations; logistics costs; forecast bands when sufficient unbiased history. Expose data coverage metrics with every trend; missing rural listings are a coverage gap, not proof of economic inactivity. No assured prices or fabricated demand. Protect small communities from re-identification by thresholded aggregation and role-scoped reporting.

## 5.7 Model and action safety

Prompt injection and tool-output instructions are untrusted; no raw SQL tool for agents; bounded schema tools with allowlists; server-side auth on every invocation; audit actor, intent, input hash, idempotency key, model/version, decision and returned authority. Timeouts, retries, cost budgets, max token/audio lengths, fallback to deterministic forms, human escalation. Voice assistant while driving supports only safe short interactions; complex forms require stopping.

---

# 6. Free API, open-source and external-data policy (RR-FREE-01)

## 6.1 Binding rule and classification

**F0 — Open-source, self-hostable:** preferred for all mission-critical runtime operations. **F1 — Free licensed downloadable data:** approved after rights/provenance review and local snapshotting. **F2 — Public rate-limited free API:** development/reference only unless permitted production usage plus an immediate F0 fallback and proved quotas. **F3 — Free vendor tier requiring billing or with mutable quotas:** not mandatory for production. **X — Paid API, noncommercial-only model, forbidden bulk tiles, unverified licence:** excluded from baseline. Protocol compatibility is not service affordability.

## 6.2 Preferred technologies and API cost exposure

| Layer | Candidate | Class | Baseline use / restriction |
|---|---|---|---|
| App web | TypeScript, PWA standards | F0 | No licence cost; devices/data/hosting still cost |
| Server API | Fastify / equivalent open-source HTTP | F0 | JSON/OpenAPI internal public contract |
| Database | PostgreSQL + PostGIS | F0 | Production data and geometry on self-run infrastructure |
| Cache/queue | PostgreSQL job queue initially | F0 | Avoid separate infrastructure until scale requires |
| Files | Self-hosted object storage / local dev storage | F0 | Security, backups and disk costs remain |
| Search | PostgreSQL FTS + trigram; optional open index | F0 | Language/alias evaluation required |
| Vector search | pgvector if warranted, licensed self-host | F0 | Embeddings never substitute canonical IDs |
| Map UI | MapLibre GL JS | F0 | BSD; must supply own licensed tiles |
| Open geographic data | Geofabrik Zimbabwe OSM PBF | F1 | Attribution and ODbL conditions |
| Gazetteer | GeoNames data download | F1 | CC BY attribution; verify record quality |
| Government boundaries | Zimbabwe Geoportal licensed datasets | F1 conditional | Check true vintage, login, reuse scope; no assumed live sync |
| Hex geospatial index | H3 library | F0 | Apache 2; index, not legal boundary |
| Address encoding | Plus Codes local library | F0 | Compute locally; avoid paid Google geocoder APIs |
| Routing | Valhalla self-hosted | F0 | MIT engine; source route quality must be validated |
| Map tiles | Self-generated OSM vector tiles | F0/F1 | Map rendering allowed with attribution/ODbL compliance |
| Geocoding | Own gazetteer + optional self-host Nominatim | F0 | No autocomplete against public Nominatim API |
| 3D globe | CesiumJS local licensed data | F0 | Apache 2 runtime, not free ion hosting |
| Spatial format | OGC 3D Tiles | Open standard | Not itself a 3D data source |
| LLM inference | Ollama / llama.cpp locally | F0 engine | Model weights each need licence review; compute is not free |
| ASR | Whisper / whisper.cpp local | F0 engine/model subject to stated licence | Validate local languages before promise |
| TTS | Piper local | F0 engine | Voice-model licences and coverage differ |
| Agent connector | Model Context Protocol SDK/server | Open protocol | Self-host MCP servers; no paid provider assumption |
| Data analysis | Python, DuckDB, pandas, open plotting | F0 | Reproducible analytics and evidence |
| Observability | OpenTelemetry/Prometheus/Grafana OSS | F0 | Self-host retention and storage costs |

## 6.3 Explicit disallowed dependencies under the user constraint

Do **not** build core production features on: Google Maps paid geocoding or Places; Google Cloud Speech paid endpoint; Mapbox/MapTiler billed map requests; Cesium ion commercial hosting; commercial LLM/TTS/STT paid calls; paid SMS/USSD providers; WhatsApp Business message charges or group automation; paid identity verification/KYC; paid background-check APIs; subscription data vendors; imagery requiring purchase. Their open alternatives can be studied, but rollout remains blocked until a non-paid operational path is certified. External payment processing has real financial rails: the initial product supports negotiated terms and manual settlement evidence, *not* a promise of free automated transfers.

## 6.4 Public API policy caveats

The OSM public tile servers prohibit bulk offline tile caching; Nominatim public service imposes a maximum of 1 request/second and disallows autocomplete/systematic queries; public Overpass is unsuitable as a busy production backend. GeoNames web services have documented daily/hourly free quotas; prefer permitted datasets in a local gazetteer. Geofabrik distributes free OSM regional extracts, but ODbL obligations remain. Public demonstrations and free tiers have no committed SLA. A 'zero paid API' release still requires hosting and operational budgets and legal review.

## 6.5 Zero-vendor-billing controls

Machine-checkable integration registry (`name`, `endpoint`, `licence`, `class`, `fee_required`, `rate_limit`, `data_rights`, `fallback`, `owner`, `last_checked`); build failure on unauthorized new outbound domains; deployment egress allowlist; explicit zero paid API procurement policy; monthly dependency attestation; dataset terms snapshots; local load tests; egress alerts; immutable test fixtures for disconnected development. No marketing claim of 'free to operate nationally' is permitted.

---

# 7. REST APIs, internal events and MCP integration contracts (RR-API / RR-MCP)

## 7.1 API conventions

Use versioned JSON REST endpoints for client operations (`/v1/...`) and OpenAPI 3.x contracts; PostGIS spatial queries remain server-only. IDs are UUIDs or opaque stable identifiers; pagination, idempotency keys and rate limits are mandatory; errors include stable machine codes, translated end-user messages and correlation IDs. Public and private geodata require separate query models. All write endpoints validate authorization, ownership and state transitions. Long-running imports, GIS processing and AI jobs return tracked task IDs.

## 7.2 Proposed API domain boundaries

| API namespace | Illustrative operations | Auth boundary |
|---|---|---|
| `/v1/taxonomy` | categories, occupations, skills, services, aliases | Public reads; steward writes |
| `/v1/places` | place search, passport, landmarks, relations | Public safe reads; reviewed changes |
| `/v1/map-proposals` | submit assertion, inspect review status | Contributor-scoped writes |
| `/v1/parties` | account and provider profiles | Strict owner/private/public projections |
| `/v1/offers` | draft, publish, search, expire | Owner mutation only |
| `/v1/requests` | post and respond to demand | Owner/authorized respondent |
| `/v1/communities` | groups, membership, publish canonical offers | Group policy and membership checks |
| `/v1/quotes` | quote, revision, acceptance | Parties scoped to transaction |
| `/v1/orders` | agreements, milestones, completion | Transition guarded; auditable |
| `/v1/logistics` | hubs, routes, handoff and delivery | Task-scoped operator access |
| `/v1/verification` | claim and evidence review | Independently privileged role |
| `/v1/ai` | submit utterance/image, draft, confirm | Scoped agent action gateway |
| `/v1/analytics` | eligible aggregate measures | Suppression/consent and scoped access |
| `/v1/admin` | taxonomy revisions, source provenance, appeals | Privileged, least-privilege |

## 7.3 MCP architecture and servers

MCP is an **open tool interoperability protocol**, not a free inference provider, bank, messaging service or authorization bypass. Expose only bounded functions through internally hosted MCP servers; tool discovery never means tool execution is unrestricted. MCP resources may expose public taxonomies and non-sensitive public place catalogs; MCP prompts can encode tested local-language listing workflows. All user-specific tools require resource-scoped authorization, audience-bound tokens, confirmation where material, traceability and transport security. No token pass-through; no unconstrained SQL, shell, browser, private coordinates or financial tool to the model.

| Proposed MCP server | Example read tools | Example write tools / safety |
|---|---|---|
| `rr-taxonomy-mcp` | `search_occupations`, `resolve_service`, `get_aliases` | `propose_alias` steward queue only |
| `rr-geo-mcp` | `find_places`, `get_place_passport`, `nearby_hubs` | `propose_landmark` reviewed, not directly publish |
| `rr-skills-mcp` | `find_providers`, `get_public_profile` | `draft_profile_update` owner-confirmed |
| `rr-market-mcp` | `search_offers`, `find_requests`, `get_public_offer` | `draft_offer`, `publish_offer` with confirmation |
| `rr-community-mcp` | `list_joinable_groups`, `get_visible_posts` | `request_join`, `share_offer` visibility checked |
| `rr-route-mcp` | `get_verified_route`, `list_collection_points` | `submit_route_observation` review queue |
| `rr-logistics-mcp` | `get_authorized_assignment`, `get_handoff` | `record_handoff` signed actor, task scoped |
| `rr-analytics-mcp` | `get_coverage_metrics`, `aggregate_demand` | No raw private analytics exports |
| `rr-trust-mcp` | `get_public_badge_meaning` | `submit_evidence` only; no auto-verify |
| `rr-support-mcp` | `get_own_case_status`, `explain_policy` | `open_case` confirmed by owner |

## 7.4 Transaction protocol

The AI returns a structured *proposal* with canonical refs, source refs, uncertainty and required confirmations. Action Gateway validates JSON schema, token scope, account ownership, constraints, rate limits and state; a deterministic service commits one transaction against an idempotency key; it returns authoritative status. The model then reports the actual outcome. Repeated mobile audio or network retries cannot create duplicate binding orders. Sensitive events are auditable without retaining entire raw voice data by default.

## 7.5 Internal event names

`party.created`, `provider.capability_claimed`, `offer.drafted`, `offer.published`, `demand.created`, `quote.proposed`, `agreement.accepted`, `place.assertion_submitted`, `place.reviewed`, `group.membership_changed`, `transport.handoff_recorded`, `work.completed`, `review.published`, `ai.action_requested`, `ai.action_committed`, `moderation.decision_recorded`. Events carry immutable IDs, timestamps, actor context and schema version; avoid private location leakage in general event streams.

## 7.6 Explicit future Tengasell boundary

Rese Rese may become a Tengasell top-level domain/module. First certify Tengasell's actual identity/auth, listing, messaging, payments, search, media and event ownership. Integrate using versioned adapters; do not write directly into protected existing tables or merge repositories without owner authorization. Shared identity mapping must be explicit and reversible. Preserve existing auction, vehicle and seller semantics and production isolation. This document authorizes no repository or deployment change.

---

# 8. Canonical logical data architecture and integrity (RR-DATA)

## 8.1 Entity inventory by domain

| Domain | Entities |
|---|---|
| Identity | `Party`, `Person`, `Organization`, `UserAccount`, `PartyRole`, `ProviderProfile`, `ContactMethod`, `ConsentRecord` |
| Taxonomy | `Concept`, `Category`, `Occupation`, `Skill`, `ServiceDefinition`, `CommodityClass`, `ConceptLabel`, `ConceptRelation`, `Crosswalk`, `TaxonomyVersion` |
| Geography | `Place`, `PlaceName`, `PlaceGeometry`, `SpatialAssertion`, `PlaceRelation`, `LocationPassport`, `Landmark`, `EconomicZone`, `ZoneMembership`, `SourceRecord`, `ReviewDecision` |
| Community | `Community`, `Group`, `GroupPlaceLink`, `Membership`, `GroupPublication`, `CommunityRole`, `OrganizationMembership` |
| Marketplace | `ProviderCapability`, `MarketplaceOffer`, `OfferSpecification`, `StockObservation`, `WorkRequest`, `Proposal`, `QuoteVersion`, `Agreement`, `Order`, `Milestone`, `Review` |
| Logistics | `TransportProvider`, `Vehicle`, `TransportNode`, `TransportConnection`, `ServiceRoute`, `CollectionBooking`, `Handoff`, `DeliveryEvidence`, `FulfilmentEvent` |
| Trust/safety | `VerificationClaim`, `EvidenceRecord`, `RiskFlag`, `ModerationCase`, `Appeal`, `RegulatoryEligibility`, `AuditEvent` |
| Intelligence | `AIInteraction`, `IntentInterpretation`, `EntityCandidate`, `AIProposal`, `AIActionRequest`, `AIActionResult`, `ModelVersion`, `Evaluation`, `GeoInference`, `HumanCorrection` |
| Analytics | `MetricDefinition`, `CoverageSnapshot`, `FactEvent`, `ForecastScenario`, `ModelAssumption`, `ReportExport` |

## 8.2 Key cardinality rules

A party may operate multiple authorized provider profiles but every offer has exactly one accountable party. A provider has many skills, offers and service areas. A village may have zero accepted geometry; its names are not unique globally. A group can attach to many places and one listing can be published in many groups without duplication. Administrative parent links are acyclic and versioned. A review requires eligible completed activity and documented relationship. A delivery handoff changes custody only, not necessarily legal ownership. Historical facts preserve the canonical IDs applicable at the time, even if concepts are merged.

## 8.3 Provenance assertions

Every significant geographic and professional statement stores `(subject, predicate, value/object, evidence_source, reporter, observed_at, valid_from, valid_to, status, reviewer, decision)`; source snapshot includes licence, publisher, original URL, hash and retrieval time. States `REPORTED`, `CORROBORATED`, `VERIFIED`, `DISPUTED`, `SUPERSEDED`. A verified school location does not mean the attached village boundary is verified. Conflicting human and government sources are retained and mediated; no silent overwrites.

## 8.4 Privacy segmentation

Public-safe locations, approximate provider service areas, confidential home addresses, authorized customer delivery points, regulated site coordinates and live task tracking live under separate data access policies. Personal data minimization, consent, purpose binding, retention/deletion, account recovery, secure encryption and incident response are platform-wide. Aggregate analytics suppress cells and groups at re-identification risk. Zimbabwe privacy law, mining/transport/health credentials, employment safeguards and commercial regulations require qualified local legal review before launch.

## 8.5 Data migration and quality gates

Bulk imports land in quarantine with source rights and data quality checks: freshness, uniqueness by canonical ID (not only text names), out-of-range coordinates, suspected duplicate settlements, invalid geometries, impossible routing, broken lineage, stale credentials, PII leaks and unrecognized unit conversions. Human steward approves versioned publication. Every import must be reproducible and reversible. Backups and restore drills are mandatory before real user onboarding.

---

# 9. Rural-first UX, offline operation and community support

## 9.1 Form factors

**Web/PWA:** full marketplace plus 2D maps with text/list alternatives. **Lightweight mode:** minimal text, cheap images, queued voice notes, localized prompts. **Offline:** selected self-owned map package, cached place list, draft offers and map assertions with local encryption/sync conflicts. **Agent-assisted:** trained community agents help with consented, auditable submissions; they never own client accounts or proceeds. **SMS/USSD:** deferred because telecom integrations generally carry costs outside the free-only API rule; do not claim baseline availability.

## 9.2 Language first

Language choice at entry, easily switched. Display native script and localized audio guidance. Each user can have distinct recognition and output-language preferences. Translate classifications without translating canonical IDs. Confirmation screens for units, money, names and places; replay audio when needed. Users can type if voice fails. Accessibility for low literacy includes icons and short steps without patronizing design.

## 9.3 Community-agent governance

Agent identity, training, verified appointment, role scopes, user consent, zero covert fees, audit trails, account recovery separation, suspension and grievance channels. Assisted registration is tagged but does not imply more trust than self-registration. Mapping incentives reward audited quality rather than inflated counts. Local administrators cannot exclude competitors or assert government endorsement without evidence.

---

# 10. Security, legal, ethical and operational controls

## 10.1 Core controls

Strong auth and session isolation; owner/organization and group-scoped authorization; RLS or equivalent server enforcement; encryption in transit/at rest; secure secrets; immutable audit of material changes; restricted moderation; source and licence registry; explicit platform safety policy; backups, restore tests, disclosure reporting, abuse prevention, transaction dispute workflows and independent certification. No model-facing raw SQL, arbitrary runtime shell or secret-laden prompt context.

## 10.2 Special risk cases

**Mining:** underground work authorization, confidential geometry, mining claims/licences, hazard classification, safe job descriptions and qualified contractors only. **Health/legal/finance/security:** verify professional authority before represented as licensed. **Domestic work/care:** safeguard minors and vulnerable people, protect home coordinates. **Agriculture and livestock:** quarantine/biosecurity/transport requirements. **Cross-border logistics:** legal trade and customs; no evasion. **Environmental:** prevent publication of sensitive habitats. **Fraud:** report/review with appeal; machine suspicion is not guilt. **Community:** avoid exclusion, impersonation, public exposure and coerced economic arrangements. **Payments:** manual settlement proof does not equal independently verified transfer and is marked accordingly.

## 10.3 Compliance programme

Document Zimbabwe Cyber and Data Protection Act/2024 data-controller regulation applicability and licensing; provider/worker classification; sector-specific permits; consumer protection; taxes/records; user/agent consent; intellectual property; dataset ODbL/CDLA/CC-BY scope; cross-border processing; retention; minors; AI transparency. Cost of compliance is not waived by the free API policy. Obtain legal sign-off before a public launch; do not replace this with AI-generated legal advice.

---

# 11. Architecture decisions and non-negotiable standards

| ADR | Decision | Rationale / guardrail |
|---|---|---|
| ADR-001 | Single canonical economic graph in relational database | Prevent divergent identities across products |
| ADR-002 | Modular monolith first, service boundaries documented | Reduce deployment and ops overhead |
| ADR-003 | PostGIS for authoritative spatial storage | Stable spatial semantics, indexes and provenance |
| ADR-004 | Independent self-owned map/route endpoints | Respect public OSM server policies and free-API constraint |
| ADR-005 | Voice gateway abstracts ASR/TTS/LLM providers | Support language model replacement and local inference |
| ADR-006 | MCP server tools wrap typed application APIs | Do not give agents unrestricted access |
| ADR-007 | AI proposes, transactional services execute | Auditable decisions and idempotency |
| ADR-008 | Registered place can lack geometry | Include unmapped villages |
| ADR-009 | Group publication references canonical offer | Avoid double sale and inconsistent stock |
| ADR-010 | Public/private geographic projections | Protect households, drivers and regulated sites |
| ADR-011 | Native structured community groups with optional share links | Not dependent on charged WhatsApp API |
| ADR-012 | 2D first; 3D/AR/VR are derived views | Rural accessibility before spectacle |
| ADR-013 | Local-first development with isolated staging | Minimize deployment cost; production immutable until authorized |
| ADR-014 | No essential paid API in baseline | Testable provider/egress policy |


---

# 12. Dependency-ordered delivery programme: work packages and urgency

## 12.1 How tasks are governed

**URGENT P0:** Blocks foundational correctness, licence compliance, user safety or first true working transaction; start first. **HIGH P1:** Required for a meaningful, independently certified rural pilot. **PLANNED P2:** Scale, new economic sectors and intelligence after pilot proof. **VISION P3:** Advanced 3D/immersive and nationwide optimization, only after reliable evidence. Urgency is a *dependency priority*, not a promise of deployment date. Every work package receives a named owner, exact source/branch custody, evidence folder, acceptance report and independent review before being marked complete.

**Work unit format:** `ID — Specific name [Urgency]`; dependencies; owner discipline; deliverable; acceptance gate; risk/fallback. All tasks begin on isolated development infrastructure. Repos, production, domains, secrets and funds are outside current authorization.

## 12.2 Phase A: governance, architecture, licences and truth foundation

**RR-GOV-001 — Programme Charter and Scope Freeze [URGENT P0].** Depends: none. Owner: Programme Moderator + Product Owner. Deliverable: signed product glossary, outcomes, in/out boundaries, free API policy, source-of-truth statement, first pilot definition and architecture decision records. Acceptance: all pillars share one graph, no duplicate service definitions, no paid core API, phase authority named; rejected requirements tracked. Fallback: park unbounded features in future register.

**RR-GOV-002 — Open Technology, API and Data-Licence Register [URGENT P0].** Depends: GOV-001. Owner: Open-source / Legal Reviewer. Deliverable: machine-readable inventory of software, public endpoints, downloadable datasets, local model weights, commercial use terms, attribution and snapshots. Acceptance: each required dependency F0/F1 approved; no noncommercial-only model in a commercial runtime; egress policy tested; public OSM servers excluded from production. Fallback: substitute a local implementation.

**RR-GOV-003 — Zimbabwe Legal, Employment and Sector Controls [URGENT P0].** Depends: GOV-001. Owner: Zimbabwe legal adviser + Trust Lead. Deliverable: data-protection, consumer, labour, mining, regulated professional, agricultural, trade and child-safeguarding requirements. Acceptance: launch blockers and prohibited offerings signed off; appeals and privacy policy drafted; no unexplained personal-data processing. Fallback: restrict service classes.

**RR-ARC-001 — Canonical Domain and Integration Boundaries [URGENT P0].** Depends: GOV-001. Owner: Solution Architect. Deliverable: service/context map, UUID rules, transaction authority, public/private projections, ADR-001–014. Acceptance: independently reviewed sequence diagrams prove one listing and one place identity across Skills/Communities/GEO. Fallback: defer integration adapters rather than duplicating identity.

**RR-DEV-001 — Local-first Repository, CI and Evidence Harness [URGENT P0].** Depends: ARC-001, GOV-002. Owner: Platform Engineer. Deliverable: reproducible developer environment, lint/typechecks, test database, container definitions, seeded demo, security scan, local URL and documented commands. Acceptance: fresh clone starts offline with fixtures; automated tests reproducible; production untouched; no paid API credentials. Fallback: run reduced services locally.

**RR-DATA-001 — Canonical Schema, Temporal Relations and Migration Governance [URGENT P0].** Depends: ARC-001. Owner: Data Architect. Deliverable: ERD, Postgres/PostGIS schema, RLS design, migrations, invariants, merge redirects, event ledger, audit and rollback. Acceptance: duplicate names, no-geometry village, multi-role party, multi-group offer, boundary history, inventory ownership and failed atomic writes all pass. Fallback: reduce scope but never weaken invariants.

**RR-SEC-001 — Authorization, Privacy and Secrets Authority [URGENT P0].** Depends: ARC-001, DATA-001. Owner: Security Lead. Deliverable: RBAC/ABAC and object access, client scopes, encrypted sensitive locations, consent records, secrets and response projections. Acceptance: role-switch, enumeration, private-location and broken-object-auth tests fail closed; audit events emitted. Fallback: disable affected endpoint.

**RR-SRC-001 — Open Dataset Acquisition and Import Quarantine [URGENT P0].** Depends: GOV-002, DATA-001. Owner: Data Steward. Deliverable: signed source manifest and immutable snapshot mechanism for Zimbabwe Geoportal, Geofabrik, GeoNames and approved public references. Acceptance: old/uncertain geometry not mislabelled current, source date preserved, licence recorded, bad records quarantined and import reversible. Fallback: manual verified seed of pilot places.

**PHASE A GATE — G-A:** one authorized conceptual system, a reproducible local repository, legally reviewed approved free dependencies, a coherent schema, and auditable source handling. No public-user onboarding before G-A.

## 12.3 Phase B: national taxonomy and village-capable GEO foundations

**RR-TAX-001 — National Occupation and Service Dictionary [URGENT P0].** Depends: DATA-001. Owner: Taxonomy Lead. Deliverable: 24 browse families, initial canonical occupations, skills, customer tasks, aliases, multi-role relationships, sector tags and ISCO/ESCO crosswalk provenance. Acceptance: sampled village wording and urban job terms resolve to correct reusable concepts; no occupation created from an unchecked AI phrase. Fallback: allow searchable provisional descriptor without canonical publication.

**RR-TAX-002 — Goods, Produce, Units, Rental and Mining Sector Facets [URGENT P0].** Depends: TAX-001. Owner: Commerce Taxonomist. Deliverable: goods classification; standard/locally described units; grades, packaging, expiry; restrictions; equipment rental; verified mining-industry *service* facets. Acceptance: '20 crates' remains 20 crates unless validated conversion; unsafe/regulated goods gated; product and skill distinctions maintained. Fallback: limit to unregulated pilot categories.

**RR-GEO-001 — Canonical Zimbabwe Place Gazetteer [URGENT P0].** Depends: DATA-001, SRC-001. Owner: GIS Lead. Deliverable: versioned province/district/ward references and provisional villages, local aliases, landmarks, geometry status, source lineages. Acceptance: identical village names remain distinct; source confidence visible; no autogenerated exact household points. Fallback: region-based browse plus named local landmarks.

**RR-GEO-002 — Location Passport and Place Resolution API [URGENT P0].** Depends: GEO-001, SEC-001. Owner: GIS/API Lead. Deliverable: place lookup by text, alias, nearby public landmark, optional GPS, Plus Code/H3, uncertainty and no-coordinate state. Acceptance: user can register by village name alone; ambiguous village prompts for district/landmark; private address never returned in public API. Fallback: manual place correction task.

**RR-GEO-003 — Self-hosted Map Tiles, Gazetteer Search and Attribution [URGENT P0].** Depends: SRC-001, GEO-001, DEV-001. Owner: Geo Platform Lead. Deliverable: self-generated vector tiles, MapLibre web map, OSM/GeoNames attribution, own place autocomplete; offline package source. Acceptance: app still runs after disabling public OSM tiles, Nominatim and Overpass; licence notices shown; map/list parity. Fallback: text-only search remains available.

**RR-GEO-004 — Spatial Assertions, Review Queue and Map My Village [HIGH P1].** Depends: GEO-002, SEC-001. Owner: GIS Steward + Community Research. Deliverable: audio/text/photo landmark submissions, uncertain observations, independent review, dispute queue and versioned publication. Acceptance: two contradictory ward claims preserved; mapper cannot unilaterally verify official place; offline submission sync retries safe. Fallback: manual administrative review.

**RR-GEO-005 — Verified Routes, Hubs and Seasonal Accessibility [HIGH P1].** Depends: GEO-003, GEO-004. Owner: GIS / Logistics Lead. Deliverable: directional transport-node graph, collection hub registry, Valhalla integration, explicit route-source dates, mode and confidence. Acceptance: unmapped road returns 'unknown'; no invented ETA; demonstrable hub-to-market route only after evidence. Fallback: human-reported pickup instructions.

**PHASE B GATE — G-B:** any sampled village can be represented without a postal code or coordinate; no geographic assertion loses provenance; map and search work without paid/public production geocoding; regulated occupations remain gated.

## 12.4 Phase C: commerce, identity and grassroots community products

**RR-ID-001 — Multi-role Account and Provider Skills Passport [URGENT P0].** Depends: DATA-001, SEC-001, TAX-001. Owner: Identity/Product Lead. Deliverable: persons, organizations, consented provider profiles, capability claims, portfolios, verification distinctions, multi-area coverage. Acceptance: one person can be both buyer and multi-skill provider; no self-declared qualification renders as verified. Fallback: hide unverified badge, not provider identity.

**RR-MKT-001 — Unified Offer, Work Request and Search [URGENT P0].** Depends: ID-001, TAX-002, GEO-002. Owner: Marketplace Lead. Deliverable: typed drafts, publication, expiry, filters, category and location search, photo uploads and quote invitations. Acceptance: all offer types remain one canonical object family; unpublished drafts not visible; owners cannot mutate each other's offers; duplicate submissions idempotent. Fallback: category-limited pilot.

**RR-MKT-002 — Quotation, Agreement, Completion and Review [URGENT P0].** Depends: MKT-001, SEC-001. Owner: Transaction Lead. Deliverable: proposal versions, explicit acceptance, work/delivery milestones, disputes, eligible verified reviews and manual settlement-evidence label. Acceptance: an accepted quote freezes terms; every completion/review linked to work record; payment never asserted from a screenshot alone. Fallback: cash/manual settlement *declared*, not verified.

**RR-COM-001 — Canonical Community, Group and Membership Registry [URGENT P0].** Depends: ID-001, GEO-002. Owner: Community Lead. Deliverable: place-linked community identity, faceted group taxonomy, roles, member consent and visibility scopes. Acceptance: multi-village group and multiple groups in one village work; no group inherits private subgroup data; group admin cannot change verified credential. Fallback: group listing disabled until auth passes.

**RR-COM-002 — Structured Group Marketplace and Native Feed [HIGH P1].** Depends: COM-001, MKT-001. Owner: Community UX Lead. Deliverable: searchable group offers, jobs, regional announcements, referrals and moderation. Acceptance: shared offer keeps single owner/stock ID; deleting group post does not erase order history; visibility enforced. Fallback: group share links and public listings only.

**RR-TRUST-001 — Claims, Credentials, Reviews, Moderation and Appeals [URGENT P0].** Depends: SEC-001, ID-001, MKT-002. Owner: Trust & Safety. Deliverable: independent identity/profile/credential claims, business checks, abuse reports, evidence retention and appeal flows. Acceptance: group endorsements never become verified licences; minors/high-risk data protected; impersonation reports can be resolved with audit. Fallback: no regulated-category publication.

**RR-COM-003 — Community Agent and Cooperative Identity Program [HIGH P1].** Depends: COM-002, TRUST-001. Owner: Field Operations. Deliverable: agents, organization representatives, delegated consent, prohibited agent custody, quality scoring and fees policy. Acceptance: farmer retains account and stock; agent actions traceable; consent can be withdrawn. Fallback: self-service only.

**PHASE C GATE — G-C:** a full deterministic job/offer interaction and an authorized native group workflow can complete without AI, paid APIs, orphaned data or unauthorized writes.

## 12.5 Phase D: AI and multilingual voice, guarded by the deterministic core

**RR-AI-001 — Local Model Gateway and Model Licence Audit [URGENT P0].** Depends: GOV-002, DEV-001, SEC-001. Owner: AI Platform Lead. Deliverable: self-hosted inference abstraction for chosen LLM/embeddings/ASR/TTS, exact model weights/version/licence and resource budget. Acceptance: no paid model endpoint invoked; fallback deterministic UI works when local model is offline. Fallback: typed workflow without AI generation.

**RR-AI-002 — Structured Intent, Place and Taxonomy Entity Resolution [URGENT P0].** Depends: AI-001, TAX-001, GEO-002. Owner: Language/AI Lead. Deliverable: intent schemas, slot extraction, unit/currency/place ambiguity tests, retrieval-grounded recommendations. Acceptance: no guessed price, village or unit is persisted; ambiguous entities demand clarification. Fallback: guided form with manual selection.

**RR-AI-003 — AI Action Gateway with Permissioned Tool Contracts [URGENT P0].** Depends: AI-002, MKT-002, SEC-001. Owner: Security / Agent Engineer. Deliverable: typed tools, action classes, confirmation, actor scopes, no SQL, idempotency, prompt-injection tests and audit events. Acceptance: a forged tool output cannot publish or move funds; repeated prompts create one permitted transaction. Fallback: read-only assistant.

**RR-LANG-001 — English, Shona and Zimbabwean Ndebele Dataset and Baselines [URGENT P0].** Depends: AI-001, GOV-002. Owner: Speech Lead + Native-language Reviewers. Deliverable: representative consented benchmark, licence-checked public corpus subsets, domain lexicon and baseline WER/CER, place name and number accuracy by dialect/device/noise. Acceptance: no language marketed as supported without passing bounded end-to-end tasks; no unlicensed model training. Fallback: typed/native language and human-assisted voice note processing.

**RR-VOICE-001 — Local Voice Listing and Search Assistant [HIGH P1].** Depends: AI-003, LANG-001, MKT-001. Owner: Voice UX Lead. Deliverable: record/playback, recognition, clarification, spoken/text response, listing draft and confirmation in qualified languages. Acceptance: task works under poor microphone/network test; quantity and place verified before publication; voice failure yields text path. Fallback: async transcription/manual assistance.

**RR-MCP-001 — MCP Servers, OAuth Scope and Tool Isolation [HIGH P1].** Depends: AI-003, ARC-001. Owner: Integration Security Lead. Deliverable: taxonomy, geo, market, community and support servers exposing minimal declared tools/resources; protocol tests. Acceptance: user A cannot retrieve user B records; no token passthrough; direct host actions require authorized tool calls; logs redacted. Fallback: internal REST-only agent connector.

**RR-AI-004 — Community, Buyer, Provider and Support Agents [HIGH P1].** Depends: MCP-001, COM-002, TRUST-001. Owner: Agent Experience Lead. Deliverable: tested role-specific agent flows, citations to records, bounded context and human escalation. Acceptance: group-only information never revealed outside group; support agent cannot secretly reset credentials; responses show evidence/uncertainty. Fallback: search and human support.

**RR-AI-005 — Computer Vision and Geographic Inference Queue [PLANNED P2].** Depends: GEO-004, AI-001, SRC-001. Owner: Geospatial ML Lead. Deliverable: opt-in listing photo classifications, inferred feature candidates and confidence; human review. Acceptance: model-suggested building/road never directly becomes verified village/parcel; imagery licences checked. Fallback: community text/field mapping.

**PHASE D GATE — G-D:** a real user can complete a controlled voice and text task; all tool actions are permissioned; speech performance is measured separately for languages; when AI is disabled, deterministic commerce remains operational.

## 12.6 Phase E: rural transport, economic zones and AI analytics

**RR-LOG-001 — Transport Provider and Vehicle Service Profiles [HIGH P1].** Depends: GEO-005, ID-001, TRUST-001. Owner: Logistics Lead. Deliverable: declared route, mode, capacity, availability, driver qualifications/vehicle details where applicable. Acceptance: no vehicle suitability inferred from voice alone; user location not permanently tracked. Fallback: verified route descriptions and pickup phone/contact.

**RR-LOG-002 — Collection Hub and Chain-of-Custody [HIGH P1].** Depends: LOG-001, MKT-002, GEO-005. Owner: Fulfilment Lead. Deliverable: bookings, handoff actors, time, quantity/condition checks, evidence, exception reports, owner/shipper distinctions. Acceptance: producer ownership and group allocations survive multiple handoffs; missing proof generates unresolved status. Fallback: manual audited handoff.

**RR-LOG-003 — Driver Voice Assistant and Route Observation [HIGH P1].** Depends: LOG-002, VOICE-001, AI-003. Owner: Driver Experience Lead. Deliverable: short safe task-scoped commands, route/road-condition reports, pickup confirmation, offline message queue. Acceptance: no complex prompts while driving; GPS opt-in; no false all-weather road from one journey. Fallback: stop-and-submit form.

**RR-ECON-001 — Economic Zone and Producer Aggregation [HIGH P1].** Depends: COM-002, LOG-002, TAX-002. Owner: Rural Commerce Lead. Deliverable: functional market zones, multiple-village supply pools, buyer demand, supplier allocations and consent. Acceptance: individual supplier quantity and proceeds not lost; zone != official district designation. Fallback: direct one-to-one produce sale.

**RR-DATA-002 — Analytics Events, Coverage Metrics and Dashboards [HIGH P1].** Depends: DATA-001, MKT-002, GEO-004. Owner: Analytics Lead. Deliverable: privacy-aware aggregate supply/demand, event QA, coverage completeness, transaction vs listing-price separation. Acceptance: every geographic insight displays coverage/data freshness; small groups not re-identifiable. Fallback: reviewed CSV reports with no personal data.

**RR-ECON-002 — Demand, Opportunity and Seasonal Forecasting [PLANNED P2].** Depends: DATA-002, ECON-001. Owner: Data Scientist. Deliverable: baseline forecasting, uncertainty, drift monitoring, route/market scenarios. Acceptance: evaluated against held-out periods and geography; no fabricated 'current market price' when data missing. Fallback: descriptive statistics only.

**RR-DIAS-001 — Diaspora Procurement and Verified Local Work [PLANNED P2].** Depends: MKT-002, TRUST-001, LOG-002. Owner: Commerce Product Lead. Deliverable: remote requests, photo-evidenced milestones, international customer privacy and payment-neutral agreements. Acceptance: remote buyer can inspect evidence and dispute; no promise of free international payment APIs. Fallback: quotes and verified milestones only.

**PHASE E GATE — G-E:** at least one real partner-authorized rural-to-urban corridor has verified source, collection, provider ownership and buyer handoff data; dashboards report what is truly observed.

## 12.7 Phase F: specialized economies, national expansion and immersive GEO

**RR-SECOR-001 — Mining, Highlands, Water and Remote Site Access Profiles [PLANNED P2].** Depends: GEO-005, TRUST-001, GOV-003. Owner: Sector Safety Lead. Deliverable: safe permitted public site registry, regulated jobs/qualifications, offline terrain and site-gated logistics. Acceptance: underground GPS not assumed, mine confidentiality respected, no licence-gated offers without checks. Fallback: above-ground publicly safe area and inquiry-only listings.

**RR-SCALE-001 — Multi-province Expansion and Local-language Coverage [PLANNED P2].** Depends: G-E certification. Owner: Field Expansion Lead. Deliverable: region onboarding playbook, language and geography scorecards, mapping reviewers and support network. Acceptance: new regions pass same pilot quality gates; no map coverage represented as complete without evidence. Fallback: restrict to tested districts.

**RR-GEO-006 — Economic GEO Intelligence Console [PLANNED P2].** Depends: DATA-002, GEO-005, ECON-001. Owner: Geospatial Intelligence Lead. Deliverable: layered maps, filters, map-source traceability, geographic coverage gaps, demand/supply overlays and route confidence. Acceptance: small-area privacy tests, sources visible, no individual live tracking. Fallback: 2D public summaries only.

**RR-XR-001 — 3D Digital Terrain and Economic Twin Pilot [VISION P3].** Depends: GEO-006, GOV-002. Owner: Spatial Computing Lead. Deliverable: self-hosted CesiumJS pilot over verified public places, licensed terrain/3D Tiles, scenario metadata. Acceptance: source and hypothetical overlays distinguished; no paid ion/tiles; frame time and accessibility measured. Fallback: MapLibre 2D.

**RR-XR-002 — AR / VR Field Navigation and Scenario Exploration [VISION P3].** Depends: XR-001, GEO-005. Owner: XR Accessibility Lead. Deliverable: bounded safe public-hub AR experiment or VR economic-zone simulation. Acceptance: tested physical safety, no fake route guarantee, device accessibility and consent. Fallback: text/audio directions.

**RR-INT-001 — Tengasell Compatibility and Contract Audit [PLANNED P2; execution requires Owner authorization].** Depends: stable G-C, later Owner mandate. Owner: Tengasell Architect + Rese Rese Moderator. Deliverable: identity/listings/search/media/payments/notifications mapping and change-risk assessment. Acceptance: zero production mutation, exact-lineage custody and tested reversible adapters. Fallback: keep products separate.

**RR-INT-002 — Authorized Tengasell Integration [VISION P3; explicit separate release approval].** Depends: INT-001, owner go-ahead, independent certification. Owner: Integration Lead. Deliverable: approved shared navigation/identity/linkage through stable APIs, tests and rollback. Acceptance: native Tengasell features unaffected, staging first, exact versions, no unauthorized merges. Fallback: links between independently operated products.

## 12.8 Cross-cutting certification tasks (through every phase)

**RR-QA-001 — Canonical Data and Geographic Invariant Tests [URGENT P0].** Depends: DATA-001. Owner: Independent QA. Deliverable: versioned fixtures and DB/API tests for duplicated villages, no-coordinate places, overlapping jurisdictions, merge lineage, multi-skill identity, transaction ownership, group scopes, import rollback. Acceptance: no invariant bypass in normal and adversarial tests.

**RR-QA-002 — Multilingual Voice and Agent Red-Team Certification [HIGH P1].** Depends: VOICE-001, AI-003. Owner: Independent AI Tester. Deliverable: dialect/noise dataset, WER/CER, intent, number, monetary value, place, prompt-injection and hallucination reports. Acceptance: high-risk errors block launch; performance reported per language rather than averaged away.

**RR-QA-003 — Rural Physical UAT, Offline and Low-end Device Matrix [HIGH P1].** Depends: G-D, LOG-002. Owner: Field UAT Lead. Deliverable: actual 2G/3G/spotty data or simulated connectivity, Android budget devices, human agent path and accessibility evidence. Acceptance: multi-step job works and resumes; no silently lost voice drafts or duplicate orders.

**RR-QA-004 — Privacy, Licensing, Security, Load and Restore Certification [URGENT P0 before public release].** Depends: all release-facing components. Owner: Independent Security/Data Reviewer. Deliverable: penetration testing, MCP tool isolation, no-licence violations, documented egress deny, load envelope, backup restore, deletion and legal gates. Acceptance: P0 findings closed; production cost/API policy attested.

**RR-OPS-001 — Launch Runbooks, Incident Response and Owner Controls [URGENT P0 before public release].** Depends: QA-004, GOV-003. Owner: Operations Lead. Deliverable: on-call ownership, moderation coverage, legal notices, abuse/incident escalation, dashboards, outage/offline mode, data rollback and public source attributions. Acceptance: tabletop incident, compromised moderator, provider fraud and GIS corruption exercises passed.

**PHASE F GATE — G-F:** new sectors and immersive features are enabled only when legally and empirically supported; future Tengasell deployment remains a separate programme with distinct custody and authorization.

---

# 13. Release sequence, critical path and practical pilot plan

## 13.1 Gated development releases

| Release | Primary user-visible capability | Gate |
|---|---|---|
| R0 Architecture | Local sandbox, free stack, schema, source licences | G-A |
| R1 Geographic and taxonomy proof | Village-first location passport; browse, canonical categories | G-B |
| R2 Deterministic marketplace | Multi-skill profiles, offers, jobs, group publications, quotes | G-C |
| R3 AI-assisted marketplace | Governed agents, Shona/Ndebele tested voice, location clarification | G-D |
| R4 Rural trading corridor | Produce pool, trusted collection and transport handoff | G-E |
| R5 Regional expansion | More settlements, sectors, languages and coverage analytics | Regional recertification |
| R6 Spatial intelligence | Economic layers, 3D/AR/VR experiments with proof | G-F |
| R7 Tengasell | Explicitly authorized integration after independent audit | Separate owner approval |

## 13.2 Critical path

GOV-001/002/003 → ARC-001 → DATA-001 + SEC-001 + SRC-001 → TAX-001/002 + GEO-001/002/003 → ID-001 + MKT-001/002 + COM-001/002 → AI-001/002/003 + LANG-001/VOICE-001 → GEO-005 + LOG-001/002 → QA physical corridor → pilot acceptance. Build legal/privacy and source licences *before* importing arbitrary public maps or recruiting field users. Develop marketplace logic independent of AI so model failure never blocks confirmed orders.

## 13.3 Pilot geography selection criteria

Select one Zimbabwean town or growth point connected to several named villages, with functioning markets, local skills, an agricultural supply use case, at least one willing transport provider, low-connectivity variation and a trusted independent field coordinator. The actual district is an owner decision informed by permission, local partnerships and field research; do not assume we have validated one already. Include (a) a mapped village, (b) a village with only an approximate landmark and (c) a duplicate village-name case. Avoid a city-only pilot, which cannot test the primary promise.

## 13.4 Six certification journeys

1. **Rural voice seller:** spoken produce listing; unit clarification; local landmark; offline save; agent validation; publish to group; buyer discovery.
2. **Multi-skilled worker:** builder also repairs pumps; service radius; independently labelled credentials; employer job request; quote and completed review.
3. **Village-name conflict:** two locations share a name; disambiguate by district/school; disputed ward; no false precise GPS; reviewer resolution and history.
4. **Group privacy:** private cooperative discussion versus public listing; member removal; admin abuse attempt; masked private address.
5. **Supply chain:** multi-producer stock allocations; hub custody; transporter consent; delivery exception; manual settlement evidence without pretending automated payment.
6. **Remote work or regulated site:** legal mining-related repair request with authorization checks, no underground real-time GPS claims, safe public rendezvous fallback.

## 13.5 Initial outcome scorecard

Metrics: share of submitted places resolved correctly; proportion of village registrations without exact addresses; first relevant result; verified introductions; completed jobs; group-to-market conversions; user confirmed quantity and currency extraction; rural voice completion by language; offline sync success; listing duplicate rate; disputes, impersonation and private-location exposure; mapping review latency; geographic coverage known versus unknown; cost per successful task *including compute and human support*. Provisional initial targets require measured baselines; zero tolerated unauthorized money movements, false verified credentials and private coordinate leaks.

## 13.6 Suggested team responsibilities

Owner/Programme Moderator, Solution Architect, Domain/Database Engineer, GIS/Map Engineer, Taxonomy/Data Steward, Frontend/Mobile Engineer, AI/Voice Engineer with Zimbabwean language reviewers, Community Field Operations Lead, Marketplace/Logistics Engineer, Trust/Legal Reviewer, Independent QA/Security Specialist and UX/Accessibility Researcher. Small teams may combine roles for engineering, but release acceptance and sensitive verification require an independent reviewer. Assign an accountable named owner to each ID before implementation.

## 13.7 Milestones without unjustified calendar promises

Use explicit evidence gates, not unsupported launch dates. Work packages are sequenced; parallel work is allowed after contracts and security rules are frozen. Stop and remediate if one of the no-geometry village, private-location, language-number recognition, group ownership or idempotency tests fails. No deployment to production merely because source-level tests are green. Local UAT comes first; staging only by approval; public release after independent physical certification and legal readiness.

---

# 14. Testing, evidence and independent acceptance protocol

## 14.1 Evidence bundle per work package

Each specialist return must contain: task ID, owner, approved parent, exact git SHA/branch, change summary, schema/migration version, API contract change, tests and commands, screenshot/recording if user-facing, source and licence records if geospatial/AI, security/privacy impact, limitations, cost/egress proof, and acceptance recommendation. A Moderator issues **ACCEPT**, **REMEDIATE** or **BLOCK**, not a silent assumed merge. Feature source acceptance and live physical certification remain distinct.

## 14.2 Minimum test pyramid

Unit tests for normalization, unit conversion, auth checks and taxonomy; property tests for cyclic place hierarchies and idempotency; integration tests for signed APIs/MCP; migration/rollback and PostGIS index tests; adversarial AI prompts; private and group visibility tests; real-device voice/dialect tests; offline replays; geo-source change detection; load and restore drills; UAT with rural residents and low-cost phones. Run comparison baselines after any major model update to catch language regressions.

## 14.3 Foundational failure modes to prove safe

A village is not on OSM; two names collide; a resident misremembers a ward; an AI infers a price; an ASR model mistakes '15' for '50'; a driver submits the same voice status twice; an image is falsely assigned to a person; a moderator joins a private subgroup; stale market demand creates misleading recommendation; a public API returns a home coordinate; OSM tile endpoint fails; local model times out; a revoked agent attempts to act; an underground mine request asks for precise restricted site maps. Each must have a documented harmless outcome and user-facing recovery path.

## 14.4 Definition of done

An item is done only when the exact approved candidate passes domain invariants, authorization, independently reviewed evidence, supported language/geography tests as applicable, licence checks and the appropriate physical demonstration. 'AI generated the code' or 'UI looks polished' is not acceptance evidence. Time-to-first-result and accessible low-data alternatives are as important as visual sophistication.

---

# 15. Costs, sustainability and zero-paid-API realities

## 15.1 Free software versus operational expenditure

Free software reduces vendor lock-in and marginal request charges but does not eliminate GPUs/CPUs, SSD storage, backups, bandwidth, electricity, internet connectivity, field mapping work, language annotation, device testing, customer support, legal licensing, fraud and insurance risk. Under a strictly zero paid API requirement we can build and test locally and self-host, but cannot responsibly guarantee nationwide live service for zero total cost. A scale-out budget decision is separate from an API procurement exception.

## 15.2 Practical low-cost architecture choices

Start modular monolith + Postgres/PostGIS + one local model process + MapLibre with locally generated Zimbabwe tiles + own gazetteer + basic images. Use scheduled batch jobs for voice-note transcription and geographic processing where real-time processing is expensive. Favor deterministic search before embeddings; store thumbnails not oversized originals; run valuation models only when enough data supports them; manage cache and lifecycle storage. Do not bind an always-on national chatbot to a paid token-metered endpoint. Record real operating cost and capacity in the pilot.

## 15.3 Possible future revenue (not required at launch)

Optional verified business profiles, service subscriptions, approved promoted listings clearly labelled, organization tools, enterprise procurement, logistics coordination fees, aggregate economic planning tools, training matches and third-party software integrations. Essential rural discovery must not be destroyed by pay-to-rank. Subscription or transaction charges require transparent user terms and lawful payment rails. Monetizing personal location or voice data is explicitly rejected.

---

# 16. Risk register and mitigations

| Risk | Severity | Mitigation / go-no-go |
|---|---|---|
| False village coordinate/boundary | Critical | Place without geometry; assertion provenance; human review |
| Wrong quantity/currency in local-language ASR | Critical | Explicit user confirmation; independent language benchmark; fallback |
| Private household or mine position disclosure | Critical | Separate protected location projections; adversarial leak tests |
| Dependence on restricted 'free' public map API | High | Own licensed extracts, tiles, gazetteer and routing endpoints |
| Noncommercial model or dataset used commercially | High | Per-asset licence register; block deploy until approved |
| AI action changes order or money without authority | Critical | Schema tool gateway, server auth, transaction states and idempotency |
| Group gatekeeping / exploitation | High | Nonexclusive community identity, appeals, independent moderation |
| Fraudulent credentials and reviews | High | Claim-specific verification, traceable completed jobs and appeals |
| Rural network/device barriers | High | PWA low-data mode, offline drafts, agent-assisted registration |
| Poor Shona/Ndebele model quality | High | Per-dialect acceptance; typed language and human correction |
| Incomplete OSM roads imply false delivery reliability | High | Unknown route status, time-scoped observations, manual handoffs |
| Mining and cross-border legal violations | Critical | Sector-specific gates, authorized site roles and legal review |
| Unrealistic VR/AI infrastructure expense | Medium | 2D-first, self-host proof, postpone large 3D datasets |
| Vendor drift / licence changes | High | Versioned dependency registry, local snapshots, replacement adapters |
| Analytics interprets low platform coverage as low activity | High | Coverage denominators, uncertainty and non-representativeness flags |
| Local moderator compromised | Critical | Scoped permissions, dual review, audit, emergency revoke |
| Producer aggregation misallocates money or inventory | Critical | Supplier-level allocations and reconciliation; no implicit group ownership |
| Future Tengasell integration contaminates live systems | Critical | Adapter boundary, staging-only audit, explicit Owner authority |

---

# 17. Decisions required from the Owner before implementation

1. Confirm this document as **design authority**, with scope changes handled through versioned amendments rather than informal redesign.
2. Nominate the programme moderator and independent release certifier.
3. Select the initial pilot economic corridor after field availability and licensing research, rather than assume one town is representative of Zimbabwe.
4. Ratify the strict zero-paid-API dependency policy and separately acknowledge infrastructure/field-operation costs.
5. Approve the first permitted sector set (recommended: unregulated household services, digital work, common lawful goods/produce, local transport inquiries; legal review for other sectors).
6. Approve privacy and data collection policy, language recording consent and village mapping contributor rules.
7. Approve local-development repository custody and protected staging boundary, with no changes to Tengasell until a later specific authorization.
8. Decide whether manual settlement remains acceptable for pilot orders; do not presume payment automation exists.

---

# Appendix A. Canonical source and reference register

**Source verification date:** 28 October 2026. URLs and stated terms must be rechecked immediately before code import or public deployment. This register is evidence for technology choices, not legal advice or a claim that every dataset is complete.

1. PostgreSQL open-source licence — https://www.postgresql.org/about/licence/
2. MapLibre GL JS licence and open rendering — https://github.com/maplibre/maplibre-gl-js
3. H3 geospatial indexing — https://h3geo.org/docs/
4. Valhalla self-hostable routing and MIT licence — https://valhalla.github.io/valhalla/
5. Geofabrik free Zimbabwe OSM extract — https://download.geofabrik.de/africa/zimbabwe.html
6. OpenStreetMap data licence/attribution — https://www.openstreetmap.org/copyright
7. OSM public tile usage policy (no offline bulk download) — https://operations.osmfoundation.org/policies/tiles/
8. Nominatim public usage policy — https://operations.osmfoundation.org/policies/nominatim/
9. OpenStreetMap API usage policy — https://operations.osmfoundation.org/policies/api/
10. Overpass API public instance limits — https://wiki.openstreetmap.org/wiki/Overpass_API
11. Zimbabwe Geoportal dataset catalogue — https://zimgeoportal.org.zw/datasets/
12. Zimbabwe ADM0–ADM3 reference; check 2018-named source file — https://stagging.zimgeoportal.org.zw/datasets/zimbabwe-administrative-boundaries/
13. GeoNames data licence / downloadable data / free API quotas — https://www.geonames.org/export/
14. Google's open-location-code source (local Plus Codes library) — https://github.com/google/open-location-code
15. CesiumJS open-source licence and optional commercial ion — https://github.com/CesiumGS/cesium
16. OGC 3D Tiles specification — https://www.ogc.org/standards/3DTiles/
17. Whisper code and model MIT licence — https://github.com/openai/whisper
18. Piper engine (voice-model licences separately assessed) — https://github.com/rhasspy/piper
19. Ollama source MIT licence (model weights separately licensed) — https://github.com/ollama/ollama
20. Zimbabwe All Speech dataset and constituent source distinctions — https://huggingface.co/datasets/Teleagents/zimbabwe-all-speech
21. MCP architecture: tools/resources/prompts — https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/docs/2026-07-28/learn/architecture.mdx
22. MCP tool protocol — https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/server/tools.mdx
23. MCP authorization security and audience binding — https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/basic/authorization/security-considerations.mdx
24. Zimbabwe data-controller implementation guideline — https://www.veritaszim.net/node/7805
25. ILO ISCO-08 — https://ilostat.ilo.org/methods/concepts-and-definitions/classification-occupation/
26. ESCO occupations and skills — https://esco.ec.europa.eu/en/classification
27. Digital Earth Africa documentation — https://docs.digitalearthafrica.org/en/latest/
28. Google Open Buildings — https://sites.research.google/gr/open-buildings/
29. Microsoft ML building footprints — https://github.com/microsoft/GlobalMLBuildingFootprints
30. OpenMapTiles code licences and self-hosting context — https://github.com/openmaptiles/openmaptiles

# Appendix B. Acronyms and definitions

**API:** Application Programming Interface. **MCP:** Model Context Protocol; standardized agent tool/resource interface. **ASR/STT:** automatic speech recognition/speech-to-text. **TTS:** text-to-speech. **GIS:** geographic information system. **GNSS/GPS:** satellite-based positioning (generally unavailable underground). **H3:** hierarchical hexagonal spatial index. **PostGIS:** spatial extension to PostgreSQL. **ODbL:** Open Database License; common OSM data rights. **PWA:** progressive web app. **RLS:** row-level security. **UAT:** user acceptance testing. **VR/AR/XR:** virtual, augmented and extended reality. **Digital twin:** sourced, maintainable digital representation for observation/simulation; not an invented 3D picture. **Canonical ID:** stable identity independent of name. **Provenance:** documented source and review lineage. **Assertion:** specific claim with source and status. **Economic zone:** functional trading area, not automatically a legally designated government special economic zone. **Location Passport:** place identity presentation, not an additional independent map database. **Free API:** a protocol or endpoint that does not require a paid subscription for the specific allowed use; no assumption of unlimited hosted capacity.

# Appendix C. Architecture handoff and final authority

This master plan **consolidates** the earlier Rese Rese brainstorm, the **RR-NTB-01 National Taxonomy Blueprint**, and the **RR-AI-FOUNDATION-01** AI-native direction into one programme. It covers services, professions, informal work, commodities, buying and selling, agricultural aggregation, mining and remote work safeguards, rural place maps, groups, voice, AI intelligence, 3D digital twins, open APIs, MCP, and a gated development roadmap.

**Design disposition:** PROPOSED FOR OWNER ACCEPTANCE. **Implementation disposition:** NOT STARTED. No live database, repository, cloud service, private contacts, user account, economic group or Tengasell deployment has been created or modified by this document. Every specialized implementation must start from this authority, state exact dependencies, deliver testable evidence and obtain an independent acceptance disposition. This is the united plan; every future feature must explain which part of the single economic graph it extends.

**Guiding maxim:** *No legitimate worker invisible. No village excluded because it lacks a postcode. No economic claim mistaken for verified truth. No intelligent agent above accountable human authority.*
# RESE RESE
## RR-GEO-AI-01 — National Geospatial Intelligence Architecture

**Engineering specification:** v1.0 / Design candidate  
**Date:** 28 October 2026  
**Parent:** RR-MASTER-01 (National Master Development Plan 1.0)  
**Companion:** RR-AI-CORE-01 (Intelligence, Language & Agent Architecture)  
**Scope:** Nationwide Zimbabwe including cities, rural settlements, farms, growth points, mountains, mining settlements, border corridors and remote areas  
**Priority:** URGENT P0 architectural definition; no geographic dataset imported or production system changed  
**Operating constraint:** Free-to-use open APIs/open-source software and licence-approved geographic data, self-hosted for production-critical calls

> **Mission.** Make every lawful economic location discoverable even when streets, postcodes, coordinates, accessible roads and official village polygons are missing. Rese Rese GEO maintains geographic identity, source evidence, uncertainty, economic accessibility and trusted public landmarks across the entire marketplace; it does not promise complete mapping, private-location disclosure or continuous satellite/GPS monitoring.

## 1. Architectural decisions and national scope

| ID | Design decision | Non-negotiable consequence |
|---|---|---|
| GEO-ADR-001 | Canonical `Place` is independent of coordinates | Village can exist with name/landmark and unknown geometry |
| GEO-ADR-002 | Government geography, human settlement geography, computational indexing and functional economic geography are separate layers | Ward ≠ village ≠ H3 cell ≠ group ≠ economic corridor |
| GEO-ADR-003 | Every geographic fact is a versioned assertion with provenance | Satellite predictions, resident claims and official boundaries never silently become equal evidence |
| GEO-ADR-004 | Self-generated, permitted map assets for production | No dependence on public OSM tiles, Nominatim, Overpass or paid maps |
| GEO-ADR-005 | Place identity resolution may return multiple candidates or unresolved state | No fake precision or guessed district |
| GEO-ADR-006 | Exact private homes are not public map objects | Public interface displays appropriate safe area or meeting point |
| GEO-ADR-007 | Routing is mode-, direction-, time- and evidence-sensitive | Unknown seasonal access yields unknown ETA rather than invented travel time |
| GEO-ADR-008 | AI creates reviewable hypotheses, not authoritative geography | Independent GIS/civic validation required for accepted map change |
| GEO-ADR-009 | Offline map and location search derive from our owned/permitted assets | No prefetching of public OSM tile servers |
| GEO-ADR-010 | Underground and controlled sites use separately authorized local/site reference systems | Surface GNSS must not be claimed to work inside mines |
| GEO-ADR-011 | National schema from day one, coverage progressively verified | No false claim that every village is already mapped |

**Reference:** RR-NTB-01 defines the occupation, place, community, route, claim and economic-zone semantic contracts; this specification refines the geographic implementation. Rese Rese Skills and Communities use `place_id`, `service_area_id`, `zone_id`, public landmark references and authoritative visibility rules, never raw mutable place-name strings as the sole join keys.

## 2. Four connected geographies

1. **Administrative geography:** country (ADM0), provinces/metropolitan provinces (ADM1), districts (ADM2), wards (ADM3), local authority jurisdictions with separate lineage. Administrative changes are time-versioned.
2. **Human geography:** villages, hamlets, farm settlements, resettlement areas, growth points, townships, suburbs, markets, public clinics, schools, road junctions, collection points and locally understood landmarks. Customary-area relationships are recorded independently and only with supportable evidence.
3. **Computational geography:** WGS84 points/lines/polygons, geometry uncertainty, PostGIS indexes, H3 cell covering sets, offline-derived Plus Codes and searchable normalized aliases. Grid cells are not legal place boundaries.
4. **Functional economic geography:** provider coverage, market catchments, buyer corridors, pickup hubs, transport connections, seasonal road availability, cooperative areas and secure site-access zones. Areas may cross ward/district boundaries.

**Geographic coverage classes:** `AUTHORITATIVE_ADMIN`, `VERIFIED_PUBLIC_FEATURE`, `CORROBORATED_LOCAL`, `REPORTED_LOCAL`, `AI_INFERRED`, `DISPUTED`, `UNLOCATED`. These are evidence-state categories; an AI-inferred building cluster is never automatically a registered village or house address.

## 3. Logical spatial architecture

```text
 Licensed source registry --> ingest/stage --> validate/normalize --> candidate matches
       |                            |                    |
       |                        raw immutable        provenance / source hashes
       |                            |                    |
 Community/field submissions --> assertion queue --> human review/appeal
       |                                                 |
       +----------> Geo AI suggestions (NEVER direct authority)
                                                         |
                                                         v
                         [Canonical Places + Names + Geometry Versions]
                         [Admin edges + Landmark edges + Evidence records]
                         [Transport graph + Economic zones + Access reports]
                                         |
                   +---------------------+----------------------+
                   |                     |                      |
           [PostGIS + H3]         [Gazetteer text search]   [licensed tiles]
                   |                     |                      |
                   +--------- [Geo API / rr-geo-mcp] -----------+
                                         |
                 Skills / Communities / Market / Voice / Logistics
                                         |
                                GEO intelligence / 3D
```

The canonical database is PostgreSQL with PostGIS; simple search uses normalized names + PostgreSQL full-text/trigram. H3 and Plus Codes are computed spatial references, not independent geographic authorities. A separate semantic search index or geospatial warehouse can be added only after measured needs justify the complexity.

## 4. Canonical place ontology and identifiers

### 4.1 Entity definitions

| Entity | Meaning | One critical rule |
|---|---|---|
| `Place` | Stable named or recognized geographic object | No required coordinate |
| `PlaceName` | Language/alias/name associated with place | Names non-unique nationally |
| `PlaceGeometry` | Dated coordinate/line/polygon observation | Geometry not assumed a legal boundary |
| `PlaceRelation` | Typed place-to-place relation | Provenance + validity window |
| `AdministrativeUnit` | Referenced formal government division | Official evidence distinguished from user reports |
| `CustomaryArea` | Locally or legally recognized traditional jurisdiction | Separate from ward hierarchy |
| `Landmark` | Publicly appropriate navigation reference | Private homes not default public landmarks |
| `GeoAssertion` | Atomic claim about place/geometry/relationship | Source, review, uncertainty mandatory |
| `SourceRecord` | Source document or data origin | Dataset version, licence, timestamp and digest |
| `GeoReviewDecision` | Authorized review of assertion | Independent, auditable, reversible by later supersession |
| `ServiceArea` | Areas a provider declares/serves | Not a verified residence or road guarantee |
| `EconomicZone` | Market catchment, community area or corridor | Functional, not presented as legal authority |
| `TransportNode` | Pickup, hub, junction, safe meeting point | Source and access status |
| `TransportEdge` | Directed, mode-dependent connection | Availability/confidence/season/source |
| `OfflineRegion` | Permitted regional package manifest | Dataset licence + expiry + integrity |

### 4.2 Relational schema (logical, migrations not yet authored)

| Table | Required fields / constraints |
|---|---|
| `geo_places` | `place_id` UUID PK; `type_code`, `lifecycle`, `visibility`, `created_at`; optional safe anchor reference |
| `geo_place_names` | `name_id` PK, `place_id` FK, `language`, `name_type`, `label`, `normalized`, `source_id`; no global unique label |
| `geo_geometries` | `geometry_id`, `place_id`, `geom geometry(SRID 4326) nullable`, `geometry_type`, `precision_class`, `uncertainty_m`, `observation_time`, `valid_period`, `source_id`, `review_status` |
| `geo_admin_units` | `admin_id`, `place_id` FK, `admin_level`, `official_code nullable`, `dataset_version`, `valid_period` |
| `geo_place_relations` | `relation_id`, `subject_place_id`, `predicate`, `object_place_id`, `valid_period`, `assertion_id`, `status` |
| `geo_source_records` | `source_id`, `publisher`, `source_uri`, `dataset_version`, `licence`, `licence_snapshot_ref`, `retrieved_at`, `observed_at`, `checksum`, `access_class` |
| `geo_assertions` | `assertion_id`, subject, predicate, value or object ref, `source_id`, `submitter_actor`, `review_status`, `valid_period`, `created_at` |
| `geo_review_decisions` | `decision_id`, `assertion_id`, `reviewer_actor`, `decision`, `method`, `evidence_refs`, `decided_at` |
| `geo_service_areas` | `area_id`, `party_id`, `geometry_ref` or `place_set`, `travel_mode`, `declared_or_verified`, `valid_period` |
| `geo_transport_edges` | `edge_id`, `from_node`, `to_node`, `mode`, `access_class`, `season_status`, `route_evidence`, `valid_period`, `geometry_ref` |
| `geo_offline_packages` | `package_id`, `region`, `source_versions`, `licence_manifest`, `archive_hash`, `build_time`, `expiration`, `signed_manifest_ref` |

Raw input files and evidence references are separately stored in an access-controlled, versioned object repository. Avoid using a JSONB “misc” property in place of important typed relations, ownership constraints or provenance fields.

### 4.3 Geographic integrity constraints

* Immutable `place_id`; rename and merges preserve history through successor or redirect links. Active admin-parent graph must be acyclic and appropriately time-scoped.
* A `Place` may have zero or more geometries. No zero-coordinate substitute, fabricated centroid, or mandatory WGS84 coordinate in onboarding.
* Every geometry and administrative relationship used for public authoritative display must have an accepted assertion and eligible source.
* Individual home coordinates use separate private access rules and are not general `Landmark` records.
* PostGIS validity tests reject corrupt/self-intersecting authoritative polygons, invalid CRS assumptions and inconsistent lat/long order. Unresolved invalid source polygons stay in staging with diagnostics.
* Spatial indexes (GiST) and H3 calculated references are rebuildable projections; they cannot generate facts unavailable from the source record.
* If place merges are proposed, past transaction references remain resolvable and historical snapshots remain reconstructible.

## 5. Source-data inventory, licensing and ingestion

### 5.1 Candidate open sources

| Source | Intended data | Restriction / caveat |
|---|---|---|
| Zimbabwe Geoportal administrative dataset | ADM0–ADM3 government reference | Visible catalog update 2026 does not prove boundaries are 2026; source file name includes 2018; access/reuse verification required |
| Geofabrik Zimbabwe PBF | OSM roads, settlements, POIs and other mapped public features | OSM ODbL attribution/share-alike obligations; coverage incomplete |
| GeoNames downloadable data | Alternate populated-place names | Attribution and accuracy, licence and name conflicts need review |
| Community-contributed Rese Rese surveys | Villages, school/market aliases, public pickup points | Consent, provenance, validation, conflict-of-interest, no automatic OSM upload |
| Digital Earth Africa / permitted Earth observation | Environmental/land-use layers and change research | Variable asset-level licence and date; not a people tracker |
| Google Open Buildings / other permissibly licensed building detections | Candidate settlement density | Predictions may contain false positives and omissions; no household inference |
| Approved public site/road surveys | Seasonal access, transport hub location | Precise routes may be sensitive; check authority and disclosure |

**Source ledger fields:** `dataset_id`, `version`, `publisher`, `legal_holder`, `licence`, `attribution_text`, `download_url`, `last_seen`, `source_observed_on`, `file_sha256`, `spatial_extent`, `CRS`, `transform_history`, `reuse_constraints`, `commercial_use_verified`, `redistribution_verified`, `reviewer`, `status`. An import is refused if licensing and provenance cannot be established sufficiently for its intended use.

### 5.2 Reproducible ingest pipeline

1. Acquire from approved URL or consented field survey; snapshot source metadata and exact file checksum.
2. Verify file signature/checksum where available, scan malicious payloads, validate licence/attribution and record source date separately from retrieval date.
3. Store immutable raw copy and manifest; stage parsed features without assigning canonical `place_id` by name alone.
4. Normalize source CRS to WGS84 for published geometry; preserve source CRS and transformation pipeline. Validate geometry and administrative code ranges.
5. Create candidate entity matches based on multilingual names, type, geometry proximity and administrative context, with source-dependent uncertainty.
6. Detect duplicate and conflicting assertions; do not conflate a school named after a village with the village itself.
7. Send uncertain or authority-affecting claims to review queue; accept only per approved evidence policy.
8. Publish a versioned, atomically switchable geographic snapshot to search/tiles/offline builders.
9. Reconcile aliases, historical redirects and dependent service areas; produce data-quality and coverage reports.
10. Rebuild derived assets reproducibly from approved snapshot, enabling rollback to last accepted version.

Never scrape public OSM tiles to fabricate an offline source dataset. OSM public tile service expressly prohibits bulk download, prefetch and offline use. Prefer permitted OSM PBF extracts for self-generation.

## 6. Geographic provenance and review state machine

The core mechanism is `SourceRecord → GeoAssertion → ReviewDecision → PublishedProjection`.

**States:** `PROPOSED`, `SOURCE_CHECKED`, `CORROBORATED`, `ACCEPTED`, `DISPUTED`, `REJECTED`, `SUPERSEDED`, `REVOKED`. `AI_INFERRED` is evidence type, not an accepted verification status. Each accepted claim records its specific subject, predicate, effective time, reviewer and approved evidence; verification of a pin does not verify a whole village polygon.

### 6.1 Verification tiers by claim

| Claim | Evidence threshold (initial policy proposal) | What remains unproven |
|---|---|---|
| Public market exists | Legitimate public source or corroborated field observation | Legal ownership, opening hours, capacity |
| Village name is used | Independent local reports and contextual matches | Exact official boundary |
| Village lies in specific ward | Reliable administrative reference + reviewer | Customary area or current postal service |
| School location | Public map with field or official confirmation | Whether private households nearby are associated |
| Road exists | OSM/field trace and map review | Current vehicle suitability and seasonal passability |
| All-season transport | Repeated reliable observations with seasonal coverage | Guaranteed future service/ETA |
| Mine-site access | Explicit site authorization and approved site maps | Public or underground navigability |

No fixed “two submissions = verified” shortcut. Independent corroboration depends on source quality, contributor independence and claim risk. If evidence conflicts, publish safe uncertainty and initiate dispute review rather than suppress village access.

### 6.2 Place merge procedure

Retrieve all candidates and supporting assertions → compare place type, aliases, historic administrative affiliations and geometry confidence → check whether two real villages share a name → reviewer proposes merge with reasons → independent second approval for high-impact merge → store `redirect_place_id` and source lineage → update search projections and aliases → verify prior offers/orders/groups still resolve → issue rollback-capable publication event.

## 7. Location Passport and place resolution

### 7.1 Location Passport format

A passport is a versioned read projection of canonical place, alias, approved safe public anchor, administrative context, public landmarks and evidence. It is **not** a second source of geographic truth. Users can register by village name without latitude/longitude or postal code.

```json
{
  "place_id": "uuid",
  "display_name": "Village A (illustrative)",
  "place_type": "village",
  "geometry_status": "unlocated",
  "public_anchor": null,
  "administrative_context": [{"unit_id": "uuid", "status": "reported"}],
  "public_landmarks": [{"place_id": "uuid", "relation": "near", "status": "reported"}],
  "name_aliases": [{"label": "Local alternate spelling", "locale": "sn-ZW"}],
  "verification": {"name": "corroborated", "position": "unknown", "ward": "reported"},
  "source_refs": ["src-001"],
  "passport_version": 1
}
```

Fields shown are illustrative API shape, not verified actual village or geography.

### 7.2 Place-resolution API

`POST /v1/geo/resolve-place` accepts `query`, `locale_hint`, optional `district_hint`, `ward_hint`, `landmark_name`, `safe_approx_position` (only with location consent), and `max_candidates`. It returns a candidate list and may set `resolution_status = ambiguous | resolved | provisional | unresolved`. Each candidate includes `place_id`, distinguishing human-readable administrative and landmark context, source reference, geometry precision, and a structured explanation. Exact/private user GPS is never automatically returned to other users.

**Disambiguation cases:** same village name in distinct districts; village and school share a name; old and new names; mistyped Ndebele/Shona aliases; trading centre named after shop; disputed ward; place with no geometry; GPS position inconsistent with selected village; mobile user temporarily outside home locality. Never silently prefer a precise GPS coordinate over explicit user-selected village identity.

**Matching ladder:** exact canonical/alias → accent/orthographic/phonetic-aware search → administrative hints → landmark proximity where valid → approved geometry/H3 candidates → trusted population of candidate choices → ask user; never jump from unverified embeddings directly to `place_id` authority.

### 7.3 Plus Codes, PostGIS and H3

Plus Codes are computed **offline** from an already accepted coordinate, and may support shareable location references without named streets. They do not assert residence or legal address. H3 cells at resolutions 6–9 can provide increasingly local candidate and aggregate indexes (global average cell areas roughly 36.13, 5.16, 0.737 and 0.105 km², respectively). H3 cells are **not** village polygons, administrative divisions or route guarantees. Use PostGIS `ST_DWithin`, intersection and topology operations with appropriate index/CRS checks for actual geometry queries.

## 8. Public maps, geocoding and offline delivery

### 8.1 Self-hostable stack

| Capability | Proposed stack | Operational rule |
|---|---|---|
| Map display | MapLibre GL JS / licensed native variant | Client SDK free; supply self-hosted licensed tiles |
| Source extract | Geofabrik Zimbabwe `.osm.pbf` and approved layers | ODbL and other licences tracked |
| Vector tile build | Planetiler → MBTiles or PMTiles | Versioned build, attributed source, no public tile scraping |
| Tile delivery | PMTiles via controlled static/object storage or self-hosted tile server | CORS, range requests, caching and asset hash checks |
| Gazetteer autocomplete | Internal normalized alias index / local search | Not public Nominatim autocomplete |
| Fallback maps | Low-data raster snapshot or text/landmark list where licensed | Never require GPU-intensive map for core search |
| Offline packs | Region-scoped PMTiles/MBTiles and gazetteer subset | Built from rights-cleared source; expiry/revocation manifest |
| Route calculation | Self-hosted Valhalla or other OSM-compatible routing | Route facts depend on verified underlying road quality |

Public OSM tile servers prohibit bulk/offline downloads; Nominatim's public endpoint cannot be the production autocomplete backbone. Free hosted demo tiers and public community endpoints are never mandatory production dependencies. Self-hosting removes external API billing, not bandwidth, CPU, storage, monitoring or mapping staff costs.

### 8.2 Offline region manifest

Each package includes `region_id`, `build_id`, `dataset_versions`, `tile_schema`, `gazetteer_snapshot`, `min_zoom`, `max_zoom`, `file_hash`, `attribution`, `licence_notices`, `created_at`, `reviewed_until`, `package_bytes`. Client verifies integrity before swap, installs atomically and preserves last valid map. Private saved jobs/addresses do not get serialized into public map packages. Regional packs should be size-limited and downloaded only with user consent or a clear user-initiated choice.

### 8.3 Offline conflict handling

Field changes are queued as separately identified proposed assertions with immutable event IDs; reconnect sync passes current server actor rights and stale-version checks. If a place was merged or a landmark changed during disconnection, server returns a conflict bundle to review; it must not force-write old geometry over accepted current records. The client shows `locally saved`, `sync pending`, `submitted for review`, `accepted`, or `conflict`; do not equate upload with published verification.

## 9. Map My Village + geo AI pipeline

The goal is to turn local knowledge into reviewed structured geographic information using text, recorded voice, photos, optional consented GPS and existing public sources. The AI should minimize technical burden, not silently create facts.

| Stage | AI/processing step | Authority rule |
|---|---|---|
| Record | Consent, metadata, voice/photo and optional GPS | Owner chooses whether to share exact observations |
| Recognize | ASR/language model extracts candidate names and landmark relations | Output marked inferred from utterance |
| Match | Search canonical places, aliases and nearby landmarks | Can return multiple candidates |
| Compare | Detect possible duplicates, source conflicts and suspicious geometry | No auto-merge or automatic ward verification |
| Create | Proposed point/path/name/route assertion with source IDs | Review queue only |
| Review | GIS steward and local corroborators assess specific claims | Separate identity and conflict-of-interest checks |
| Publish | Versioned accepted location projection | Controlled server mutation with audit event |
| Improve | User-visible correction and appeal | Historical source and decisions retained |

**Computer-vision tasks later:** candidate public-road change, building-cluster pattern, land-cover change, possible public market structure, quality assessment of a consented photo. All visual predictions require valid asset licence, observation date, uncertainty, and review. Building footprints must not be equated with people, households or the existence of a named village.

**Imagery asset catalog:** STAC item/collection references and OGC Cloud Optimized GeoTIFF (COG) for supported remote-sensing sources; record acquisition date, resolution, publisher, sensor, spatial footprint, licence and model version. Do not run large-scale automated imagery downloads unless the source explicitly permits the intended use.

## 10. Routing, mobility and village-to-city accessibility

### 10.1 Transport graph

A route is an evidence-based sequence or connection between `TransportNode` records. Edges must carry direction, allowed mode/vehicle, seasonal state, surface confidence, restriction, source and effective period. A public map road does not imply operational transport service.

| Road/route state | User-facing treatment |
|---|---|
| `VERIFIED_PASSABLE` with mode/time evidence | May offer qualified routing estimate with timestamp and limitation |
| `REPORTED_PASSABLE` | Show as reported route; do not promise ETA |
| `SEASONAL_UNCERTAIN` | Route may be unavailable; request local confirmation |
| `UNMAPPED` | No routing estimate; show public landmarks and contact options |
| `RESTRICTED_ACCESS` | Do not share private/site routes without authority |
| `CLOSED_OR_DISPUTED` | Suppress unqualified directions; warn appropriately |

Routing estimates require a clearly specified mode, actual routable geometry and acceptable evidence freshness. Never infer travel time from straight-line distance or a guessed vehicle speed when the user expects real-world route truth.

### 10.2 Economic corridors and collection nodes

A producer's home location stays private; a separately published safe pickup hub can be visible to buyers or verified transporters. Multi-producer aggregation references canonical owner-specific quantities, custody handoffs and route connections. A group or hub does not automatically acquire inventory or payment rights. Logistics interfaces use explicit task-scoped location access and minimize historic tracking.

### 10.3 Mining, mountain, water and underground profiles

* **Mining:** site-specific access polygons, permit/induction claims, approved entrance and checkpoint nodes, legally authorized underground schematics where available. Surface GNSS does not locate workers underground; inertial/beacon/site systems are separate research and safety cases, not free nationwide GPS functionality.
* **Highland/valley:** terrain gradient, flood/landslide access observations and unverified trail status; no invented hiking/driving ETA.
* **Waterside/remote islands:** safe landing and pickup points with local route availability; public imagery cannot establish navigability.
* **Industrial/mineral goods:** services and legitimate trade only subject to sector licensing; no platform encouragement of unlawful mineral trade, unsafe mine entry or dangerous work.

## 11. GEO APIs and MCP adapter

| API / tool | Semantics | Default access |
|---|---|---|
| `GET /v1/geo/places/{id}` | Safe public place passport | Public for public fields |
| `POST /v1/geo/resolve-place` | Ambiguous name/landmark search | Public rate-limited or authorized based on inputs |
| `GET /v1/geo/places/{id}/nearby` | Public hubs, markets and providers by eligible data | Scope-limited, safe location precision |
| `GET /v1/geo/coverage` | GIS completeness and gaps | Aggregated public or reviewer detail |
| `POST /v1/geo/assertions` | Report landmark/place/route claim | Authenticated; proposal-only |
| `GET /v1/geo/assertions/{id}` | Own submission progress | Author/reviewer |
| `POST /v1/geo/reviews/{id}/decision` | Review an assertion | Independent geo steward scope |
| `GET /v1/geo/offline-regions` | Approved downloadable map packs | Rights/capacity policy |
| `GET /v1/geo/routes` | Evidence-based route options | Public/authorized as relevant; unknown when unsupported |
| `POST /v1/logistics/route-observations` | Consented route report | Task-limited actor; review queue |

`rr-geo-mcp` exposes bounded `find_places`, `get_public_place_passport`, `list_nearby_hubs`, `get_public_route_evidence`, `submit_place_assertion` and `get_own_assertion_status`. It cannot expose raw home coordinates, legal/private mine maps, unrestricted imagery buckets, `UPDATE geo_places`, or an automatic merge/verification tool. Its authentication must follow RR-AI-CORE-01 and the current MCP protocol.

### 11.1 Sample place resolution response

```json
{
  "resolution_status": "ambiguous",
  "query": "Village A",
  "candidates": [
    {"place_id": "uuid-1", "name": "Village A", "district_context": "District X", "geometry_status": "reported", "evidence_status": "corroborated"},
    {"place_id": "uuid-2", "name": "Village A", "district_context": "District Y", "geometry_status": "unlocated", "evidence_status": "reported"}
  ],
  "needed_clarification": ["district_or_landmark"],
  "selected_place_id": null
}
```

IDs and names above are illustrative. Schema output must not substitute map proximity for a user's explicit selection without confirmation.

### 11.2 Domain events

`geo.source_imported`, `geo.assertion_submitted`, `geo.assertion_reviewed`, `geo.place_created`, `geo.place_relation_published`, `geo.place_merged`, `geo.place_disputed`, `geo.geometry_superseded`, `geo.offline_region_built`, `geo.route_observation_reviewed`, `geo.transport_connection_changed`. Include IDs, schema version, actor and source IDs; public event streams exclude exact personal locations.

## 12. Spatial analysis and 3D / AR / VR extension

**GEO Discover (pilot):** text place lookup, responsive 2D map, public local hubs, service areas, practical list alternative. **GEO Intelligence (later):** multi-layer geographic filters, source/coverage overlays, access gaps, grouped supply and demand, corridor comparisons. **GEO Immersive (research):** optional 3D terrain and licensed photogrammetry/3D Tiles; scenario models for proposed hubs, routes and market regions; exploratory AR at approved public navigation points.

A 3D rendering must declare source date, vertical accuracy, spatial resolution and whether buildings/roads are measured or simulated. A digital twin of an individual village or mine requires appropriate survey detail and rights; it cannot be derived reliably from one low-resolution satellite view. VR is not required to complete a listing or find a provider.

The analytical plane should distinguish **economic coverage** (how many listed participants) from **economic activity** (what has actually occurred) and **mapping coverage** (how well places are known). Missing Rese Rese registrations do not prove a village has no skilled workers or commerce. Small-population areas require aggregate suppression and re-identification protection.

## 13. Geographic quality, privacy and security testing

| Test | Expected authoritative result |
|---|---|
| Same village name in two districts | Two distinct `place_id`s; clarification required |
| Village with no coordinates | User can register by name and landmark without fake point |
| Official admin dataset vintage conflict | Version visible; no false “2026 boundary” claim |
| Two contradictory ward claims | Both retained; disputed relationship not silently accepted |
| Private domestic-work provider home coordinate | Not exposed through public map, MCP, embeddings or tile pack |
| Public OSM tile outage/blocked requests | Self-generated tiles or text-only search still function |
| OSM public offline download prohibition | No client bulk download from OSM public tile host |
| H3 boundary mismatch | Index not treated as ward or village polygon |
| Incorrect/old road during rains | Unknown/inactive route; no fabricated travel guarantee |
| Offline field changes after village merge | Non-destructive conflict review and stable redirects |
| Automatic satellite building detection | Model output only; no invented household membership |
| Mine tunnel request | No assumed underground GNSS; requires site-authorized data |
| Place merge migration | Previous transactions, groups and reviews remain resolvable |
| Unlicensed source data | Import blocked and licences preserved in provenance ledger |
| Reviewer attempts self-approval | Conflict-of-interest and separation-of-duties rejection |

### 13.1 Metrics

Measure place resolution precision/recall and ambiguity outcomes by region and language; administrative and settlement coverage with explicit known/unknown denominators; reviewed map assertions per contributor; conflicts/duplicates; source freshness; route observations by season; offline package integrity; offline sync success; public-private coordinate leak tests; search latency; map accessibility; and operational cost per resolved place (including human review).

## 14. Work packages, owners, dependencies and acceptance

| Unit / urgency | Work scope | Dependencies | Acceptance |
|---|---|---|---|
| **RR-SRC-001 / URGENT P0** | Source + licence registry and immutable snapshots | GOV-002 | Source hashes, reuse rights and lineage review |
| **RR-GEO-001 / URGENT P0** | Canonical Zimbabwe place gazetteer, time-aware administrative references, alias registry | DATA-001, SRC-001 | Two same-name villages distinct; no fabricated geometry |
| **RR-GEO-002 / URGENT P0** | Location Passport and place resolution APIs | GEO-001, SEC-001 | Village-without-coordinate onboarding and privacy tests |
| **RR-GEO-003 / URGENT P0** | Permitted self-generated tiles, self-host search, MapLibre/text fallback | SRC-001, GEO-001, DEV-001 | Functions with public OSM tile/Nominatim disabled |
| **RR-GEO-004 / HIGH P1** | Geo assertion, community review, Map My Village, offline sync | GEO-002, SEC-001 | Disputes preserved; reviewed publication; no duplicate replay |
| **RR-GEO-005 / HIGH P1** | Transport hubs, seasonal edges, evidence-based routing | GEO-003/004 | Unknown/closed road does not produce false ETA |
| **RR-AI-005 / PLANNED P2** | Licensed imagery and AI candidate-detection pipeline | GEO-004, AI-001, SRC-001 | Independent false-positive and consent audit |
| **RR-GEO-006 / PLANNED P2** | Economic GEO intelligence console | DATA-002, GEO-005 | Source-qualified analysis, small-area privacy safeguards |
| **RR-XR-001 / VISION P3** | 3D terrain/economic twin research prototype | GEO-006 + rights/accuracy review | Visualization discloses uncertainty, source dates |
| **RR-XR-002 / VISION P3** | AR/VR navigation and scenario research | XR-001, GEO-005 | Public safe-site usability; no fictitious route accuracy |
| **RR-QA-GEO / cross-cutting** | Independent GIS/data/privacy/access certification | Every stage | Written evidence-linked acceptance/disposition |

**Gate G-GEO-P0:** canonical places independent of coordinates; populated official geography distinguished from reported local places; source provenance and licences traceable; internal place search works; privacy-protected Location Passport; self-owned map pipeline demonstrates no mandatory paid/public production geocoding.

**Gate G-GEO-P1:** Map My Village completes a verified field contribution, reviewer can dispute/appeal, offline sync is safe, transport corridor shows evidence-specific accessibility rather than fabricated metrics, and voice interactions pass through RR-AI-CORE-01.

## 15. Cross-specification contract with RR-AI-CORE-01

RR-AI-CORE-01 can call GEO only through signed, actor-scoped APIs and MCP adapters. Its language model may propose `PlaceName` alias, village, landmark, road or route observations, but it cannot set `verification_status = accepted` or insert official boundaries. A user-confirmed place selection is a user preference/association, not certification of the place's existence, legal affiliation or precise geometry.

RR-GEO-AI-01 supplies immutable IDs, candidate disambiguation, source/evidence metadata, geometry privacy, route validity and safe public summaries to all AI tools. Every agent-facing place or transport response includes whether it is verified, reported, inferred, disputed or unavailable. GEO must not require a functioning LLM for normal place search, a user registration, manual coordinate entry or map viewing.

A shared end-to-end pilot MUST verify: a Shona voice message names an unmapped village → AI returns multiple place candidates or unlocated state → user identifies a public landmark → provisional Location Passport is saved → listing draft created → explicit user confirmation → listing is published in eligible community feed → buyer sees safe service/collection location → transport connectivity remains unknown until verified. Every step must be reproducible without leaking a private home coordinate.

## 16. Outstanding field, legal and technical proofs

* Verify Zimbabwe Geoportal boundary vintage and actual download/redistribution rights; confirm any administrative updates independently.
* Select a real pilot area with a town/growth point, multiple rural settlements, at least one duplicate-name case, low data connectivity and consenting mapping partners.
* Agree public landmark selection, traditional authority relationship representation and local-mapper conflict policy with Zimbabwean geographic/legal advisors.
* Confirm open imagery asset licences and version metadata, and avoid unauthorized drones or mapping of private/security-sensitive sites.
* Benchmark low-end phones and storage sizes for offline gazetteer, tile packages, voice notes and synchronization.
* Obtain legally valid site-specific authorization before any underground mining or controlled industrial mapping.
* Establish route-source/season quality criteria before publishing travel estimates.
* Confirm data protection, safety and community moderation policies before recruiting field participants.

**Disposition:** DESIGN COMPLETE FOR SPECIALIST DISPATCH; **not geographically certified, no real village mapped or routes validated by this specification**.

## 17. Primary technical references and verification ledger

Verified/accessed 28 October 2026; source file rights, vintages and licences require a fresh check at import/build time.

1. Zimbabwe Geoportal datasets — https://zimgeoportal.org.zw/datasets/
2. Zimbabwe administrative boundaries catalog — https://stagging.zimgeoportal.org.zw/datasets/zimbabwe-administrative-boundaries/
3. Geofabrik Zimbabwe OSM extract — https://download.geofabrik.de/africa/zimbabwe.html
4. OpenStreetMap public tile usage policy — https://operations.osmfoundation.org/policies/tiles/
5. OpenStreetMap public vector tile policy — https://operations.osmfoundation.org/policies/vector/
6. Planetiler vector tile generator — https://github.com/onthegomap/planetiler
7. PMTiles archive and serving — https://github.com/protomaps/pmtiles
8. Valhalla routing engine — https://valhalla.github.io/valhalla/
9. H3 cell-size documentation — https://h3geo.org/docs/core-library/restable/
10. Open Location Code / Plus Codes spec — https://github.com/google/open-location-code
11. OGC STAC standard — https://www.ogc.org/standards/stac/
12. OGC COG standard — https://www.ogc.org/standards/ogc-cloud-optimized-geotiff/
13. RR-MASTER-01 v1.0 (project parent), §§ 3, 6–7, 12–13.

**End of RR-GEO-AI-01 v1.0.**
# National geospatial intelligence and village mapping

**Current authority:** [ADR-0004](../decisions/ADR-0004-evidence-based-geo.md), [Rust/TrailBase](technology-stack.md), [full GEO specification](../source/RR-GEO-AI-01.original.md).

## Four geographies
1. Administrative: province, district, ward, local authority, with versioned legal source.
2. Human: villages, growth points, settlements, suburbs, mines' public access areas, landmarks and community-recognized places.
3. Computational: location points, geometry with uncertainty, H3, Plus Codes, SQLite R*Tree, aliases.
4. Economic/mobility: hubs, service areas, seasonal routes, collection networks, corridors and overlapping economic zones.

## Place ontology
`Place`, `PlaceName`, `PlaceGeometry`, `SpatialAssertion`, `SourceRecord`, `ReviewDecision`, `LocationPassport`, `PlaceRelation`, `TransportNode`, `RouteObservation`, `EconomicZone`. IDs are permanent. Names can collide. Geometry can be absent. Source, observation date, licence, confidence, privacy and legal/jurisdictional boundaries are separate.

## Self-hosted GEO
Rights-checked Zimbabwe data extracts (e.g. OSM, approved administrative data, field observations); ETL quarantine; own place search; SQLite geometric index/H3O; Rust geometric refinement; self-hosted vector tiles and routing. Do **not** rely on public OSM tile/Nominatim/Overpass endpoints for production. Preserve attribution.

## Map My Village
Voice/text/photograph/GPS submissions create **provisional** assertions with provenance. AI extracts candidate place/road/landmark information but cannot finalize boundaries or merge two villages. Human independent review resolves conflicts; offline submissions are durable and idempotent.

## Safety
Exact private homes and continuous movement excluded from public GEO by default. Satellite detection is not address verification. Roads may be seasonal/unknown; no unsupported ETA. Underground mine navigation requires separate surveyed lawful evidence and should never be inferred from surface GPS.

## Future
2D low-data discovery -> regional GEO analysis -> licensed 3D terrain/digital twins -> physically evaluated AR/VR. All use the same authoritative place IDs.

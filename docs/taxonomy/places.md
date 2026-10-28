# Zimbabwe national place ontology

**Four layers:** administrative (country/province/district/ward), human (village/settlement/growth point/suburb/farm/landmark), computational (point, polygon, H3, Plus Code, uncertainty), and economic (service areas, hubs, routes, corridors, zones).

## Canonical entities
`Place`, `PlaceName`, `PlaceGeometry`, `SpatialAssertion`, `SourceRecord`, `ReviewDecision`, `LocationPassport`, `PlaceRelation`, `EconomicZone`.

- Multiple villages may have the same name and alternative spellings.
- A valid village record may have **no coordinates**.
- An approximate pin is not a household address or official village boundary.
- Customary jurisdiction and ward membership are separate relationships with sources.
- Roads and route accessibility are conditional and may be seasonally unknown.
- Personal home coordinates and live movement are private by default.
- Names, aliases, geometry, containment and source decisions are versioned; merges preserve historical redirects.
- A place discovered by satellite or AI is only inferred until independently reviewed.

The public-facing [Location Passport](../architecture/geo-ai.md) joins accepted facts with transparency about uncertainty. [Full GEO spec](../source/RR-GEO-AI-01.original.md).

# Canonical data ontology and integrity

**Primary authority:** TrailBase + SQLite. One stable canonical economic knowledge graph represented through relational entities and versioned relationships.

## Core entities
- **Identity:** Party, Account, ProviderProfile, Organization, ProviderCapability, CredentialClaim, VerificationEvidence.
- **Taxonomy:** BrowseCategory, Occupation, Skill, ServiceDefinition, Commodity, Unit, MultilingualAlias, ClassificationVersion.
- **Marketplace:** MarketplaceOffer, WorkRequest, Quote, Order, WorkCompletion, Review, StockAllocation, SettlementEvidence.
- **GEO:** Place, PlaceName, GeometryAssertion, SpatialAssertion, SourceRecord, ReviewDecision, LocationPassport, EconomicZone.
- **Communities:** Community, Group, GroupMembership, GroupPlaceLink, GroupOfferPublication, ModerationCase.
- **Logistics:** TransportHub, RouteConnection, RouteObservation, Consignment, CustodyEvent, DeliveryHandoff.
- **AI:** IntentInterpretation, EntityResolutionCandidate, AIProposal, ActionRequest, ActionResult, ModelVersion, EvaluationResult.

## Invariants
Permanent IDs independent of spelling; many-to-many category/skill/group/service mappings; non-unique place names; villages without geometry; explicit location uncertainty, source and time; membership visibility enforced server-side; one canonical offer shared to many groups; supplier-owned inventory through group aggregation; frozen accepted quotations; idempotent financial/physical mutations; source-backed verification, appeals and historic lineage.

## Storage constraints
Do not assume TrailBase has Postgres RLS or PostGIS. Prove equivalent access policy using actual TrailBase auth and Rust API, test database migrations/backup/restore and prevent multi-writer contention or duplicate writes. Derived search/vector/map/analytics indexes cannot become an alternative source of truth.

Read the [master plan](../source/RR-MASTER-01.original.md) for full entity taxonomy and the [GEO](geo-ai.md) / [AI](ai-core.md) specs for specialized contracts.

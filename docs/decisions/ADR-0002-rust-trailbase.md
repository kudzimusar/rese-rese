# ADR-0002 — Rust and TrailBase canonical backend
**Status:** Binding Owner technology decision · **Date:** 2026-10-28

## Decision
Use **Rust** for primary backend, domain logic and AI orchestration, with **TrailBase + SQLite** as the primary canonical database/backend platform. Older Postgres/PostGIS-first assumptions in the master plan and GEO/AI sources are superseded.

## Implementation candidates (not yet certified)
Axum/Tokio/Serde APIs; SQLite FTS5/R*Tree; Rust geo/geo-types/H3O; GEOS only if compatible; Martin and MapLibre; local AI inference; Dioxus to be physically evaluated on budget Android. Native TrailBase extensions, concurrency, migrations, backups, realtime, privacy and geographic performance require pinned-version proof before schema acceptance.

## Consequences
No second writable authoritative place/account database. Add spatial/search/analytics projections as needed. Preserve free-API constraint and existing domain invariants.

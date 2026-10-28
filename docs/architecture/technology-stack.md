# Technology baseline — Rust + TrailBase

**Binding:** Rust primary backend and TrailBase + SQLite canonical persistence. [ADR-0002](../decisions/ADR-0002-rust-trailbase.md).

| Layer | Binding or candidate | Qualification |
|---|---|---|
| Primary language | **Rust — binding** | Pinned toolchain, CI, property tests |
| Canonical data/backend | **TrailBase + SQLite — binding** | Auth, migrations, concurrency, restore, authorization |
| APIs | Axum / Tokio / Serde — candidate | Typed contracts, idempotency, permission checks |
| Geography | Rust geo, geo-types, H3O — candidate | Geometric correctness and rural indexing |
| Spatial indexes | SQLite R*Tree, bounding boxes, H3O — candidate | Exact/approximate query separation; load tests |
| Complex geometry | GEOS or compatible SQLite extension — candidate | Deployment compatibility and rights |
| Tiles | Martin, PMTiles/MBTiles — candidate | Legal self-hosted maps and offline packages |
| Maps | MapLibre — candidate | Accessibility and low-end Android tests |
| Text | SQLite FTS5 and alias dictionary — candidate | Multilingual occupation/place relevance |
| AI | Local Rust inference via mistral.rs/Candle — candidate | Commercial model licences, quality and performance |
| Voice | Local open ASR/TTS engines — candidate | Independent Shona/Ndebele evaluation |
| Interface | Dioxus to evaluate; alternative UI permitted | Real microphone/GPS/offline/accessibility proof |
| API integrations | REST + least-privilege MCP | No broad administrative model tools |

## Required qualification BEFORE migrations
1. Pin exact TrailBase and Rust versions, demonstrate repeatable local startup.
2. Prove auth and transactional guarantees, high-contention writes, idempotency, backups and restore.
3. Prove SQLite spatial/extensions and H3 correctness in actual TrailBase environment.
4. Benchmark 10k/100k offers, multi-village search, concurrent quote/stock operations and offline replay.
5. Preserve single authoritative party/place/owner store; derived search/map/analytics caches do not own truth.

Older source specs mention Postgres/PostGIS; those recommendations are superseded and must not silently return as primary persistence. No mandatory paid APIs. Free open-source software still has compute, hosting and maintenance costs.

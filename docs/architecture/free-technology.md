# Free API and open-source policy

**Binding rule:** No required paid API. Every essential user workflow must run with self-hostable implementations and approved free/open datasets, subject to actual compute/storage/bandwidth costs.

## Cost classification
- **F0:** self-hostable open-source implementation with commercially permitted licence (operational cost still exists).
- **F1:** open/downloadable dataset with permitted reuse and attribution.
- **F2:** hosted free tier or quota, optional enhancement only; never mandatory core production dependency.
- **F3:** paid/restricted/noncommercial-only dependency; blocked until separately approved and not part of free-core architecture.

Review exact product version and dataset/model rights before development or deployment. Maintain [source register](../governance/free-source-register.csv) with evidence, licence, scope, owner and fallback.

## Preferred candidates to qualify
TrailBase, SQLite, Rust, Axum/Tokio/Serde, SQLite FTS5/R*Tree, H3O, Rust geo, Martin, MapLibre, OSM regional extracts under ODbL, own gazetteer search, local inference and rights-checked open model weights.

## Restrictions
Public OpenStreetMap tiles, Nominatim and Overpass are not unlimited production infrastructure. Do not assume public routing providers or hosted free AI APIs offer commercial SLAs. Do not integrate noncommercial-only model weights into commercial production. No unreviewed training or sensitive data uploads. Evaluate licences and cost exposures before importing real data.

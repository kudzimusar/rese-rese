# Rese Rese documentation index

**Current stage:** Documentation foundation; application not implemented. **RR-GOV-001:** review candidate (`in_review`) — **no Owner sign-off or independent certification**. Start with [README](../README.md), [Documentation Authority](AUTHORITY.md), [coding-agent rules](../AGENTS.md), and [contributing guidance](../CONTRIBUTING.md).

| Domain | Current governing documents | Preserved source / status |
|---|---|---|
| Programme purpose and boundaries | [Programme charter — proposed](charter/programme-charter.md) · [Vision](charter/vision.md) | [Master plan](source/RR-MASTER-01.original.md) |
| Decisions and provenance | [Decision register](governance/decision-register.md) · [Seven ADRs](decisions/README.md) · [Authority order](AUTHORITY.md) | Binding ADRs take precedence over superseded historical implementation proposals |
| Specialist execution and acceptance | [Development authority](governance/development-authority.md) · [Change control](governance/change-control.md) · [Evidence](governance/evidence.md) | Documentation ≠ implemented ≠ independently certified |
| Unified architecture | [Overview](architecture/overview.md) · [Rust/TrailBase stack](architecture/technology-stack.md) · [Canonical data](architecture/canonical-data.md) | [Master plan](source/RR-MASTER-01.original.md), with PostgreSQL/PostGIS-first assumptions superseded |
| AI, languages and safe actions | [AI core](architecture/ai-core.md) · [REST/MCP](architecture/api-mcp.md) | [AI technical source](source/RR-AI-CORE-01.original.md) |
| Rural GEO, places and mapping | [GEO](architecture/geo-ai.md) · [Canonical data](architecture/canonical-data.md) | [GEO technical source](source/RR-GEO-AI-01.original.md) |
| Taxonomies | [Taxonomy index](taxonomy/README.md) | [Master plan](source/RR-MASTER-01.original.md) |
| Cost, rights and dependencies | [Free technology](architecture/free-technology.md) · [Technology/source register](governance/free-source-register.csv) | Licence and commercial-use clearance pending per asset |
| Planning and field selection | [57-work-package roadmap](roadmap/README.md) · [Task register](roadmap/tasks.csv) · [First-pilot selection framework](roadmap/pilot-selection.md) | **No actual first pilot location selected or approved** |
| Certification and release gates | [G-A–G-F](roadmap/release-gates.md) · [Evidence](governance/evidence.md) | No Production or Tengasell authorization |

**Read before implementation:** [Roadmap dependencies](roadmap/README.md) → applicable accepted [ADR](decisions/README.md) → [Development authority](governance/development-authority.md) → [task register](roadmap/tasks.csv) → relevant domain and preserved specification. Do not infer approved technology versions, legal certification, deployment rights or licence rights from a candidate document.

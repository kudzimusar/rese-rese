# Documentation authority and precedence
**Version:** 1.0 · **Date:** 2026-10-28 · **Status:** Current.

## Why this exists
The original source documents contain earlier PostgreSQL/PostGIS proposals superseded by the Owner's binding **Rust + TrailBase + SQLite** decision. Preserve those source documents as historical evidence; never treat their superseded technology choices as current.

## Order of authority
1. Explicit current Owner instruction and accepted change record.
2. Accepted architecture decision records (ADRs), newest superseding decision first.
3. Current normative files under `docs/architecture/`, `docs/taxonomy/`, `docs/governance/` and `docs/roadmap/`.
4. Complete original source plans under `docs/source/`, except where superseded.

## Binding decisions
Unified Skills/Communities/GEO core; Rust; TrailBase/SQLite canonical data; free/open core API access; source-backed rural geography; user-authorized AI operations; privacy and independent trust checks; no Tengasell or Production mutation without separate Owner approval.

## Update protocol
Every change to canonical entities, dependencies, API/MCP contracts, geographic authority, security, price/ownership semantics, cost/licence constraints or acceptance gates requires: (a) the relevant normative Markdown update, (b) task-register update, and (c) a new or superseding ADR for an architectural change, **in the same PR**. Do not rewrite preserved historical snapshots.

Documented ≠ implemented ≠ certified; acceptance requires independent evidence and an explicit disposition.

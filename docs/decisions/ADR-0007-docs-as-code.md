# ADR-0007 — Version-controlled documentation as the plan
**Status:** Binding Owner decision · **Date:** 2026-10-28

## Decision
`README.md`, `AGENTS.md`, current architecture/taxonomy/roadmap files, ADRs and tasks are maintained in Git. PDF/Word copies are optional exports, not the authoritative editing medium.

## Change control
A code change must update the relevant docs and task status in the same PR. Architecture revisions need an ADR and supersession note. Historical full source documents stay preserved in `docs/source/`. No status becomes accepted without test evidence and independent disposition.

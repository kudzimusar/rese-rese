# Rese Rese — Mandatory instructions for coding agents and specialists

This file is read **before every task**, together with `README.md` and `docs/AUTHORITY.md`.

## Governing authority
1. Explicit current Owner instruction.
2. Accepted ADRs in `docs/decisions/`, with supersession recorded.
3. `docs/AUTHORITY.md`, current normative architecture/taxonomy and approved task record.
4. Original full specs in `docs/source/` for historical detail not overruled above.

## Mandatory constraints
- Use **Rust** for primary application and domain services; **TrailBase + SQLite** for canonical application records. Rust `Axum`, `Tokio`, `Serde`, `geo`, `h3o`, `Martin`, open map rendering and local models are candidates, not automatically accepted runtime dependencies.
- Never recreate Postgres/PostGIS as the default authoritative database from historical specs. If advanced spatial functionality is necessary, propose a versioned SQLite-compatible solution and independent performance tests.
- No mandatory paid API usage. Every integration must document whether it is self-hosted, free/open data, or restricted free-tier, with a no-paid-API fallback. A free endpoint is not an unlimited production SLA.
- No unapproved GitHub/Production deploys, merges, cloud billing, account creation, direct Tengasell changes or data imports from unclear licences.
- Maintain canonical IDs for parties, places, concepts, offers and groups; multilingual labels/aliases are not identifiers.
- A village may have no coordinate; location accuracy, precision, source and confidence must remain independent. Never infer home coordinates or customary boundaries as fact.
- No raw model output may directly modify money, membership/identity, place verification, legal credentials or order terms. Require authorization, deterministic validation, appropriate confirmation and transaction result.
- Preserve ownership through producer aggregation, hub collection and delivery handoffs; group membership ≠ credential verification.
- Public location views must exclude private homes and movement traces by default; claims need traceable evidence and consent.

## Work-package protocol
- Identify authoritative task ID, urgency, dependencies and exact scope from `docs/roadmap/tasks.csv`.
- Keep changes on one branch and preserve source/branch/commit custody in the report.
- Use design → implementation → local tests → evidence → independent acceptance. Never claim completion solely because source code builds.
- Update normative docs and the task register **in the same pull request** as a relevant change. New architecture changes require an ADR, and when accepted must explicitly supersede older documents.
- Every capability must have a free/offline/no-AI fallback where feasible.
- For model, voice or geography claims, show evaluation data; no invented accuracy, latency, data coverage, provider availability or ETA.

## Final handoff template
Task ID; authority and exact parent SHA; branch/head; changed paths; migrations; commands executed and actual outputs; source/licence changes; risk/security analysis; tests/evidence; unresolved blockers; rollback; recommended ACCEPT/REMEDIATE/BLOCK (only a recommendation; owner/moderator decides).

## Mandatory Rese Rese programme-moderator operating skill

**For any moderation, specialist task assignment, programme-status, progress, PR review, acceptance or cross-agent handoff, read and apply [the Rese Rese Programme Moderator Skill](.agents/skills/rese-rese-programme-moderator/SKILL.md).** It inherits the Global Moderator's governance standard and is authoritative for the *moderation workflow*, subject to Owner instructions and accepted ADRs.

Before giving the Owner any substantive programme status, refresh:
- [Continuity and verified custody](docs/governance/moderator-continuity.md)
- [Progress percentages and gate methodology](docs/governance/progress-ledger.md)
- [Agent assignments and pending specialist returns](docs/governance/agent-register.csv)
- Live GitHub branches, pull requests, CI and [57-package registry](docs/roadmap/tasks.csv)

**Always report** the dated acceptance-coverage percentage (`accepted / baselined tasks`), active candidate PRs, responsible agents, blockers, current gate and next dependency-safe action. This metric is *not* an estimate of overall product feature completeness. Never invent agent execution, accepted work or background monitoring.

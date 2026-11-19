# Rese Rese — Two Global Agent Rules (Project Adoption v1.0)

**Status:** DRAFT adoption on existing moderator governance PR #2. This is process-only; it is not permission to integrate with TengaSell, change Rust/TrailBase decisions, deploy, migrate, spend, or override local ADRs.

**Canonical source:** `kudzimusar/11-11-tech@917c16f6d64d7f29d442fec562021dd11687228d` (central PR #16 still Draft).

**Downloadable files:** [Unified contract](https://github.com/kudzimusar/11-11-tech/blob/917c16f6d64d7f29d442fec562021dd11687228d/governance/global-moderator/AGENT-GOVERNANCE-UNIFIED-v1.0.md) | [Rule A Goal Fidelity](https://github.com/kudzimusar/11-11-tech/blob/917c16f6d64d7f29d442fec562021dd11687228d/governance/global-moderator/GOAL-FIDELITY-STAGE-CONTROL-v1.0.md) | [Rule B 11-11-GM-GC-001](https://github.com/kudzimusar/11-11-tech/blob/917c16f6d64d7f29d442fec562021dd11687228d/governance/global-moderator/STANDARD-v1.0.md) | [Download PDF](https://github.com/kudzimusar/11-11-tech/blob/917c16f6d64d7f29d442fec562021dd11687228d/governance/global-moderator/11-11-TECH-GLOBAL-AGENT-GOVERNANCE-v1.0.pdf).

## Mandatory local authority and gates

- Root `AGENTS.md` and `README.md` first, followed by `docs/AUTHORITY.md`, accepted ADRs and `docs/roadmap/README.md`.
- Preserve approved Rust + TrailBase/SQLite, free/open APIs, place privacy, and 57-package `docs/roadmap/tasks.csv` baseline; do not invent a different implementation sequence.
- Reuse the existing `docs/governance/moderator-continuity.md`, `docs/governance/progress-ledger.md`, `docs/governance/agent-register.csv`; do not duplicate existing registers.

## Every agent

Identify original goal, current approved stage and exact acceptance. Classify new findings as current/future/dependency/Owner change/safety. Use existing specialist and source branch. After real work, independently verify exact head, CI and pertinent runtime, issue one ACCEPT/REMEDIATE/BLOCK and record N/M or NOT_BASELINED. Update the current handoff after material changes and require successor to generate the next handoff. Never claim background agents were dispatched based on a written prompt.

**Review status:** Approval of project and central changes requires formal reviewer and Owner permissions. Distinguish Owner's operating directive from Git merge. No release authority is granted.

# Contributing to Rese Rese

Read [README](README.md), [AGENTS](AGENTS.md), [authority](docs/AUTHORITY.md), [change control](docs/governance/change-control.md) and applicable ADRs before editing.

## For any change
- Open a bounded task branch based on a registered `RR-...` work package.
- Preserve Rust and TrailBase foundations and the free API-only constraint.
- Update user-facing and architectural docs in the same PR as code/migration changes.
- Supply tests/evidence and explain any data schema, auth or licence changes.
- Do not modify Tengasell or any Production system as part of Rese Rese work without separate authorization.

Run `cargo test -p rr-docs-check` and `cargo run -p rr-docs-check -- --check` before a docs PR.

No project-wide reuse licence has been selected; request guidance before external code/data contribution or redistribution.
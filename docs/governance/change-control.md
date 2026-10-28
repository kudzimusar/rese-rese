# Development change control

## Source of truth
Git Markdown and task register are current; full original plans are preserved in `docs/source/`; ADRs explicitly supersede old architecture choices. PDF/Word files are optional derivative exports.

## Every work package
1. Identify `RR-...` ID, priority, dependencies, owner and accepted parent SHA.
2. Read README, AGENTS, ADRs and relevant technical specs.
3. Define exact scope, forbidden environments and interfaces.
4. Develop on a feature branch; update docs and machine-readable task status in the same PR.
5. Provide actual commands/results, migrations, security tests, licensing evidence, rollback and limitations.
6. Independent reviewer recommends acceptance; authorized Moderator issues ACCEPT, REMEDIATE or BLOCK.

Never confuse documented, source-complete, physically tested, certified and production-released. Do not touch Tengasell/Production without separate Owner authority.

# ADR-0006 — Independent Rese Rese building ground
**Status:** Binding programme boundary · **Date:** 2026-10-28

## Decision
Develop Rese Rese in its own `kudzimusar/rese-rese` repository. Future Tengasell integration requires a separate Owner-authorized contract and compatibility audit, migration plan, tests and independent certification.

## Consequences
Do not touch Tengasell branches, deployments, auctions, transactions, customers or Production in this programme. Design reusable adapters, but do not assume database compatibility or modify either system without explicit approval.

# ADR-0005 — AI does not own transactional authority
**Status:** Binding safety principle · **Date:** 2026-10-28

## Decision
AI interprets voice/text, resolves possible concepts, drafts and recommends. A validated, authorized Rust action gateway commits business operations through TrailBase. Models do not independently verify credentials, change money or ownership, publish unconfirmed orders, merge official places or grant membership privileges.

## Required safeguards
Typed tool contracts, per-user object authorization, confirmations, idempotency keys, source-backed outputs, audit evidence, prompt-injection resistance, recovery/fallback, model versioning and red-team evaluations. MCP tools have least privilege and never expose broad administrative database authority to customer agents.

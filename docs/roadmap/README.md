# Development Roadmap — 57 Named Work Packages

**Authority:** [Programme Charter (review candidate)](../charter/programme-charter.md), [Documentation Authority](../AUTHORITY.md), [decision register](../governance/decision-register.md), [ADRs](../decisions/README.md), and three immutable [source specifications](../source/README.md). The 49 inherited tasks plus eight Rust/TrailBase qualification tasks remain in the machine-readable [tasks.csv](tasks.csv); their identifiers and definitions are preserved.

**Current programme state (2026-10-28):** Documentation-only architecture. No Rese Rese marketplace runtime, canonical TrailBase migrations, verified national datasets, AI/voice service or live pilot implemented. `RR-GOV-001` is **in_review**, not accepted. All other work packages retain their recorded status; no dependent implementation is authorized by this roadmap alone.

## Controlled progression

```text
RR-GOV-001 (charter / authority; in_review)
      │
      ├── RR-GOV-002 (licence and open-source proof)
      ├── RR-ARC-001 (canonical contracts, ADR and domain boundaries)
      │          │
      │          └── RR-RUST-ARCH-01 (Rust workspace proof)
      │                       │
      │                       └── RR-TB-CORE-01 (TrailBase/SQLite authority proof)
      │                                   │
      │                                   └── RR-TB-GEO-01 (SQLite/H3 spatial proof)
      └── RR-GOV-003 (Zimbabwe legal / regulated-sector controls)
                      [cross-cutting; must close before affected pilot/public operations]
```

These arrows express **predecessor authority**, not a blanket permission to execute tasks. The source register records additional prerequisites, especially `RR-GOV-002` before `RR-DEV-001` and `RR-SRC-001`, and `RR-SEC-001` before protected production-facing flows. RR-GOV-003 must not be bypassed by an otherwise passing technical chain. Owner/Moderator dispatch, exact-SHA custody and acceptance evidence are separately required.

## Release programme and urgency

| Priority | Outcomes and leading tasks | Qualification gate |
|---|---|---|
| **URGENT P0** | Charter, technology rights, legal constraints, canonical contract, Rust/TrailBase proof, geo/data/security foundations | [G-A](release-gates.md) and source-specific proofs |
| **HIGH P1** | Village/landmark mapping, structured community commerce, local-language and transport/collection corridor, rural field testing | [G-B–G-E](release-gates.md) |
| **PLANNED P2** | Analytics, specialized industries, multi-province expansion and *separately approved* Tengasell compatibility audit | Rights, safety, independent evaluation |
| **VISION P3** | 3D/AR/VR research and separately approved integration implementation | [G-F](release-gates.md), new Owner mandate |

### Subsequent work, subject to individual acceptance

1. `RR-GOV-002` validates exact licences, self-hosting, source/data provenance, operational cost and fallbacks. `RR-GOV-003` evaluates Zimbabwe legal/privacy, labour, consumer, regulated sectors and child safeguarding.
2. `RR-ARC-001` freezes shared Party/Place/Offer/Group/transaction contracts. `RR-RUST-ARCH-01` demonstrates bounded reproducible Rust modules. `RR-TB-CORE-01` proves TrailBase transactions, auth, backup/restore and migrations. `RR-TB-GEO-01` tests rural geometry, no-coordinate villages, R*Tree/H3O and uncertainty. `RR-TB-LOAD-01`, `RR-SEC-001` and independent QA cannot be skipped.
3. `RR-DATA-001`, `RR-SRC-001`, `RR-TAX-001/002`, `RR-GEO-001/002/003` adapt inherited Postgres/PostGIS deliverable wording to the **binding Rust + TrailBase/SQLite ADR** rather than implementing incompatible historical schemas. Actual task definition changes, when justified, require their own governed PR.
4. `RR-ID-001`, `RR-MKT-001/002`, `RR-COM-001/002`, `RR-TRUST-001` establish canonical skills, commerce, groups, quotes and independent verification before AI-dependent mutations.
5. `RR-AI-RUST-01`, `RR-MCP-RUST-01`, `RR-AI-001/002/003`, `RR-LANG-001`, `RR-VOICE-001` require permitted local inference, scoped actions, separate Shona/Ndebele measurement and non-AI fallback.
6. `RR-GEO-005`, `RR-LOG-001/002`, `RR-ECON-001` and `RR-QA-003` demonstrate a consented village-to-town goods/work corridor; only then consider broader regions, analytics and separately gated advanced technology.

## First pilot selection — currently undecided

Use the [evidence-weighted pilot selection framework](pilot-selection.md) before recommending a town/growth point. It includes multiple villages, economic activity, transport, language, varied connectivity, data rights, trusted field staff, veto conditions and a 100-point scorecard. **No candidate is selected, cleared or approved.** Only a later recorded Product Owner decision and legal/field authorization can activate a pilot.

## Status and evidence discipline

- `proposed`: not an accepted implementation; `in_review`: candidate exists but awaiting independent assessment; `accepted`: explicit evidence-based Moderator/Owner disposition; `remediate`: bounded corrections required; `blocked`: unresolved custody/rights/safety/mandatory evidence.
- Every status transition cites an exact source SHA, branch, test/CI evidence, reviewed environment, legal/licence effects and final authority. See [Development Authority](../governance/development-authority.md) and [Evidence](../governance/evidence.md).
- Documentation present **does not** imply implementation, physical certification, public release, paid provider permission or Tengasell integration.
- **Production and Tengasell remain prohibited** without separate explicit Owner authorization under [ADR-0006](../decisions/ADR-0006-tengasell-boundary.md).

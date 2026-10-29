# Rese Rese Programme Progress Ledger

**Purpose:** Provide the Owner with a numerical, explainable, verifiable state of the entire programme at each substantive moderator interaction. No hidden progress or invented percent complete.

## 1. Definitions and formula

**Default programme percentage: work-package acceptance coverage.**

```text
Acceptance coverage (%) = 100 × number of formally ACCEPTED packages ÷
                          number of currently baselined packages

Current baseline denominator = 57
                          = 49 original work packages
                          + 8 Rust/TrailBase qualification work packages
```

Count a task **ACCEPTED** only if the current authority registers a disposition with exact SHA, tests and reviewer/Owner approval as required. Submitted code, draft PR, passing CI, working demo, an accepted design, merger and Production launch are separate attributes. Do not count a PR twice.

**Current candidate activity:** show `in review`, `blocked`, `in progress` and `unassigned` separately from the acceptance percentage. A pending candidate cannot enter the numerator.

**Vertical and gate readiness:** Skills, Communities, GEO, AI/Voice, Logistics, Platform/TrailBase, Trust/Security, Independent QA and G-A through G-F get discrete evidence-based statuses. Percentage of accepted packages must **not** be presented as product feature completion.

**Effort-weighted estimate:** `NOT BASELINED` until the Owner signs off task weights and estimation model. If later authorized, report it as a **second clearly labelled metric**, with weights, denominator, assumptions and date—not as a replacement for acceptance coverage.

## 2. Dated verified baseline — 2026-10-29

**Source:** `main` at `7207b57dd03bd362a007173828b0852a00882fb0`, `docs/roadmap/tasks.csv`, GitHub PR/Actions state.

| Metric | Result | Explanation |
|---|---:|---|
| Baseline work packages | 57 | 49 original + 8 technical qualification |
| Formally accepted in main register | 0 | `tasks.csv` lists all 57 as `proposed` |
| **Acceptance coverage** | **0.0%** | `0 ÷ 57 × 100` |
| Open candidate packages evidenced | 1 | RR-GOV-001 in open draft PR #1 |
| Candidate count relative to baseline | 1/57 (1.8%) | **Activity only**, not accepted progress |
| Independently verified blocked packages | Not established | Do not infer 0 real blockers from absence of entries |
| Gate G-A | In progress, NOT CERTIFIED | Governance candidate exists, foundational prerequisites open |
| Gate G-B through G-F | Not certified | No applicable physical acceptance evidence |
| Application implementation | Not started / not evidenced | No marketplace, TrailBase, GEO or AI runtime certification |
| Weighted overall completion | Not baselined | No approved estimates |

**PR #1 candidate:** [OPEN / DRAFT](https://github.com/kudzimusar/rese-rese/pull/1), head `58276f2ebe989532f8962bce2c89468813f0b2c0`, [documentation CI PASS](https://github.com/kudzimusar/rese-rese/actions/runs/40695727170), **NOT ACCEPTED / NOT MERGED** at this snapshot. Subsequent independent review: **BLOCK — OWNER_POLICY** for formal scope-freeze ratification; review ID `5691749101`.

**No declaration that the complete product is 0% built is intended.** A significant architectural documentation foundation exists, but the specific percentage is strictly the accepted-work-package ratio.

## 3. What the Owner should see every time

Use an always-visible concise status line in substantive Rese Rese progress/dispatch/review answers:

> As of YYYY-MM-DD (source: main SHA): **Accepted N/57 = XX.X%**; **under independent review:** N (IDs/PRs); **blocked:** N known/unknown; **gate:** G-A …; **active agents:** roles and task; **next:** named action.

Also state the **exact task and reason** for any blocker or delay, including Owner authority, unavailable provider, test harness, environment, source custody or licensing. Show the current accepted SHA and current unmerged candidate SHA separately.

When no fresh GitHub read is possible, publish **“last verified”**, date and evidence link; never imply a stale snapshot is live.

## 4. Updating these numbers

1. Read `docs/roadmap/tasks.csv` and compare to accepted moderator dispositions and PR evidence. A newly added task changes denominator **only after an authorized baseline change**.
2. Read `docs/governance/agent-register.csv` and active GitHub PRs; avoid counting the same task twice.
3. Read CI at the **actual PR head**; a PASS at a previous commit does not certify the new head.
4. Record changes in a dated summary when a work package is formally accepted, its status changes, a dependency becomes blocked, or the Owner changes scope.
5. Keep proof references and the calculation auditable. Never move work from proposed to accepted solely to increase the displayed percentage.

## 5. Current phase/vertical matrix

| Vertical | Verified implementation/certification status |
|---|---|
| Programme governance | Existing planning files; RR-GOV-001 candidate under review |
| Platform Rust / TrailBase | Rust documentation checker works; product backend not implemented |
| Skills and marketplace | Designed; not implemented or physically verified |
| Communities | Designed; not implemented or physically verified |
| National GEO | Designed; no verified gazetteer/Location Passport runtime |
| AI and multilingual voice | Specifications exist; no local inference or Shona/Ndebele UAT certification |
| Transport and economic zones | Planned; not implemented |
| Trust / security / legal | Principles documented; independent/legal acceptance pending |
| QA and release | Documentation CI passes; no product/physical release certification |

## 6. Record history

- **2026-10-29:** Created source-backed moderator baseline. Formal acceptance `0/57 = 0.0%`; RR-GOV-001 PR #1 candidate separate. This ledger itself is documentation-only until reviewed/merged.

- **2026-10-29:** Independent RR-GOV-001 source review recorded at PR #1, review ID `5691749101`. Disposition **BLOCK — OWNER_POLICY** pending explicit charter ratification; no substantive source defect identified; register remains 0/57 = **0.0% acceptance coverage**, G-A not certified. Do not count blocked Owner sign-off as failed product implementation. Current governance continuity changes are on draft PR #2, not merged to `main`.

Append a new dated row after future **verified** changes; retain prior snapshots for comparability.

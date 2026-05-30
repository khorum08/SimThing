# V7.8-MET-SCENARIO-ACCEPT-0 — Design-Authority/Product Ruling

**Reviewer:** Opus 4.8 (design authority — v7.8 track) + project-owner product direction.
**Date:** 2026-05-30.
**Decision:** **ACCEPT SCENARIOS; OPEN C-0 FIRST (Option B).** All three V7.8-MET-SCENARIO-0
named consumer scenarios are accepted (`NamedScenarioAccepted`). Per product priority — **close out
map batching first** — implementation opens only for **C-0 (Line C / M / atlas)**; **A-0 and B-0 are
queued (accepted, not opened)**. ClauseThing/L3 stays parked; FrontierV2-5 and ACT/EVENT/OBS/PIPE
remain unauthorized.

## Reviewed — code, not only the report

- `crates/simthing-spec/src/designer_admission/v7_8_line_scenarios.rs` — the three scenario claims,
  the gate-status machinery, and admission validation.
- `crates/simthing-spec/tests/v7_8_met_consumer_scenarios.rs` — 10/10 pass.
- Confirmed `simthing-sim/src/**` has no FrontierV2/atlas/E-11B/D-2/SEAD/ResourceFlow awareness
  (only the pre-existing `BoundaryRequest` core enum).

## VRAM budget set (product direction)

A typed, configurable VRAM budget was added to the Line C atlas claim and the C-0 gate (this is
scenario/gate metadata, **not** atlas implementation):

- `V78AtlasVramBudget` — **default ceiling 1.5 GiB** (`V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES =
  1_610_612_736`), **`configurable: true`**, **`architectural_hard_cap: false`**,
  **`multiplier_reporting_required: true`**.
- Design intent (documented on the type): the 1.5 GiB ceiling is the commodity-GPU *starting*
  budget; it is a profile/config parameter with **no architectural hard cap** — dedicated/headless
  servers and larger-VRAM cards raise `max_bytes` far beyond the default. Atlas occupancy is checked
  against the *active* budget (algebraic mask G=0 ≈ 1.0×; physical gutter G≥H ≈ 6.76×), never a
  constant. Admission rejects a Line C scenario whose budget is zero, non-configurable, hard-capped,
  or lacks multiplier reporting.

## Review answers

| # | Question | Finding |
|---|---|---|
| 1 | E/Line A scenario satisfies the nested-RF gate? | **Yes** — `depth_required = 4 (> 2)`, 1 faction / 100 planets / 1000 districts / 100000 factories, `flat_star_insufficient = true`. Genuine depth>2 fanout. **ACCEPT.** |
| 2 | T/Line B satisfies the hard-currency-ordering gate? | **Yes** — multi-transaction workload, sequential cross-band ordering required, discrete AccumulatorOp path insufficient at the declared contention scale, boundary/hot-pool contention declared. **ACCEPT.** |
| 3 | M/Line C satisfies the multi-theater-atlas gate? | **Yes** — `theater_count = 4 (> 1)`, single 32×32 insufficient, atlas required, VRAM budget now a typed configurable term, algebraic mask G=0 preferred / physical gutter G≥H fallback, full-tile protocol-oracle parity required. **ACCEPT.** |
| 4 | Accept all three? | **Yes** — all three move to `NamedScenarioAccepted`. |
| 5 | Parallel or prioritize? | **Prioritize C-0 first** (product: close out map batching). A-0/B-0 queued — no speculative parallel implementation. |
| 6 | Is C/M blocked by VRAM-budget acceptance? | **Resolved** — budget set (1.5 GiB default, configurable, no hard cap, reporting mandatory). C-0 still gated on the **§11 M-4 full-tile protocol-oracle-parity** implementation PR. |
| 7 | Is B/T blocked by the D-2a "defer until needed"? | The named scenario satisfies "needed," so the defer is lifted *in principle*; but by product priority **B-0 is queued, not opened.** |
| 8 | Is A/E blocked by flat-star sufficiency? | The fanout scenario demonstrates flat-star insufficiency at depth 4; accepted, A-0 queued. |
| 9 | Any overclaim of implementation authorization? | **No** — all `NamedScenarioProposed`/`implementation_authorized = false`; admission rejects status ≠ proposed and any authorized flag; implementer did not self-accept. |
| 10 | Production-track state per line | **A / E:** `NamedScenarioAccepted`; **A-0 queued.** **B / T:** `NamedScenarioAccepted`; **B-0 queued.** **C / M:** `NamedScenarioAccepted`; **C-0 OPEN** (gated on §11 M-4 protocol-oracle-parity PR; VRAM budget = 1.5 GiB default, configurable). |

## C-0 — opened gate (bounded scope)

| Step | Scope | Status |
|---|---|---|
| **C-0** | First §11-gate M-4 atlas slice: **full-tile protocol-oracle parity** (vs an exact per-tile-protocol CPU oracle, not corridor-t44 alone) + **VRAM-multiplier report against the active budget** (default 1.5 GiB, configurable). Algebraic tile-local mask G=0 preferred; physical gutter G≥H fallback. Opt-in, default-off; `request_atlas_batching` stays rejected until C-0 lands its gate. | **OPEN** |

C-0 is **opened for implementation**; this review does **not** implement M-4/M-4A. A-0 and B-0 are
**queued** (accepted scenarios, gates not opened).

## Guardrail confirmations (no authorization)

ClauseThing runtime, ClauseScript parser, FrontierV2-5, ACT-5/EVENT-3/OBS-5/PIPE-1, production
`SimSession` wiring, scheduler/cache, semantic WGSL beyond explicitly gated generic kernels, CPU
planner/urgency/commitment, Resource-Flow bypass, cross-entity/production movement writes,
production commitment emission, shared-pool tick writes, `simthing-sim` semantic awareness, and any
line implementation beyond C-0 — **all remain unauthorized**. A-0/B-0/atlas-runtime/E-11B/D-2a are
not implemented by this pass.

## Commands

| Command | Result |
|---|---|
| `cargo test -p simthing-spec --test v7_8_met_consumer_scenarios` | **PASS — 10/10** (incl. new VRAM-budget assertions) |
| `cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission` | **PASS — 25/25** |
| `cargo check --workspace` | **PASS** (pre-existing `simthing-driver` unused-import warning only) |

## Ruling

**ACCEPT SCENARIOS; OPEN C-0.** V7.8-MET-SCENARIO-0 sufficiently names the consumer scenarios for
E/Line A, T/Line B, and M/Line C; all three are recorded `NamedScenarioAccepted`. Per product
priority, **C-0 (atlas / map batching) opens first** with the VRAM budget set to **1.5 GiB default,
configurable, no architectural hard cap, multiplier reporting mandatory**, still gated on the §11
M-4 full-tile protocol-oracle-parity PR. **A-0 and B-0 are queued.** No ClauseThing/L3, FrontierV2-5,
ACT/EVENT/OBS/PIPE, or runtime widening is authorized. v7.8 constitution / production-track split
intact.

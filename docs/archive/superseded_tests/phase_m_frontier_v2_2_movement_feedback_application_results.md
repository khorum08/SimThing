# FrontierV2-2 — Own-Column Movement Feedback Application Results

## Base HEAD

`8b620e02781acd9a2f63dc0c0901eaa073ef2cdc` (post-FrontierV2-1 merge, pre-FrontierV2-2)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v2.rs` | Own-column shadow state, movement application, feedback summary hashing |
| `crates/simthing-driver/tests/phase_m_frontier_v2_2_movement_feedback_application.rs` | **New** — three-tick movement feedback fixture + 12 tests |
| `docs/tests/phase_m_frontier_v2_2_movement_feedback_application_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV2-2 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV2-2 row |
| `docs/workshop/field_policy_track.md` | FrontierV2-2 addendum |
| `docs/worklog.md` | Append-only milestone |

**No ClauseThing, no semantic WGSL, no default SimSession wiring, no phase closure declaration.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did FrontierV2-1 prove?** | Tick-aware movement/structural FixtureCandidate evolution across two ticks (`M1 != M0`, `S1 != S0`), tied to closed-loop feedback. Replay fingerprint `2d6e78a06d19736a`. |
| **2. What movement candidate fields are available?** | `FrontierV2MovementCandidate`: source_unit_id, delta_row, delta_col, route_code, dispatch_count; evolved via `build_evolved_movement_candidate`. |
| **3. What does “own-column movement application” mean in fixture/test-support terms?** | Apply movement delta to `FrontierV2OwnColumnShadow` (fixture-only row/col), classified as `OwnColumnShadowWrite` — not production simthing-sim state. |
| **4. How will the fixture prevent writes to another entity’s state?** | `validate_movement_write_target` rejects when `movement.source_unit_id != shadow.unit_id`; no simthing-sim writes. |
| **5. How will the updated position feed the next tick’s field route?** | Tick2 uses `source_seed_placement` with shadow row/col for faction-A seed instead of default (0,0). |
| **6. What counts as observed movement feedback?** | Shadow position changes after M1 application; tick2 mapping hash differs from tick1. |
| **7. What remains fixture-only?** | Shadow state, movement application, seed placement override, feedback application — driver test support only. |
| **8. Why this still does not implement ClauseThing or production runtime?** | No authoring front-end, no default SimSession wiring, no simthing-sim semantic state; shadow writes are test-only integers. |

## Fixture layout

```text
FrontierV2 admission (default-off, profile=FrontierV2)
  tick 0:
    GPU mapping + EML → GPU PIPE-0 → GPU ACT-2 → ResourceFlowAllocator
    → movement M0 (FixtureCandidate)
    → own-column shadow at (0, 0)
  tick 1:
    feedback-adjusted field/economy input
    GPU live route → movement M1
    apply M1 to own-column shadow (OwnColumnShadowWrite)
  tick 2:
    feedback-adjusted input + shadow source placement
    GPU live route
    verify mapping/proposal delta vs tick 1
```

Fixture ID: `frontier_v2_2_movement_feedback_application_v1`

## Tick 0 summary

**Live route: GpuVerified** — default source placement (0,0) and (7,7).

| Field | Value |
|---|---|
| Movement M0 | `delta_row=1`, `delta_col=6` |
| Shadow | `(0, 0)` unit_id=0 |
| Seeds | district_output_a=120 at (0,0), district_output_b=80 at (7,7) |

## Tick 1 movement candidate summary

**Live route: GpuVerified** — feedback-adjusted seeds (120/80 → 145/97), default placement.

| Field | Value |
|---|---|
| Movement M1 | `delta_row=2`, `delta_col=4` |
| Mapping hash | `10465360340528472241` |
| Resource route | allocator (code 1) |

## Own-column shadow write summary

| Field | Before | After | Classification |
|---|---|---|---|
| row | 0 | 2 | OwnColumnShadowWrite |
| col | 0 | 4 | OwnColumnShadowWrite |
| unit_id | 0 | 0 | same entity |
| tick_index | 0 | 1 | recorded |

Applied via `clamp(row + delta_row, 0, grid_size-1)` and same for col.

## Tick 2 source-placement summary

**Live route: GpuVerified** — faction-A seed at shadow position (2, 4); faction-B at (7, 7).

| Field | Value |
|---|---|
| Source placement A | (2, 4) from shadow |
| Seeds | district_output_a=170 at (2,4), district_output_b=97 at (7,7) |
| Mapping hash | `3848270009636632236` (differs from tick1) |
| Proposal/dispatch hash | unchanged vs tick1 (mapping change is primary proof) |

## Movement feedback delta summary

| Metric | Tick 1 | Tick 2 | Changed |
|---|---|---|---|
| Mapping hash | `10465360340528472241` | `3848270009636632236` | yes |
| Shadow position | (0, 0) pre-write | (2, 4) post-write | yes |
| Movement feedback delta hash | — | `18114721525200588216` | recorded |

Movement candidate → own-column shadow update → tick2 field route change proven.

## Structural candidate status

Structural candidates continue to be derived per tick as **FixtureCandidate** (same evolved builder as V2-1). Not the focus of V2-2; no production structural route.

## CPU oracle parity table

| Field | Tick 0 | Tick 1 | Tick 2 | Match |
|---|---|---|---|---|
| Movement candidate (oracle) | M0 | M1 | M2 | exact |
| Shadow update (oracle) | — | (2, 4) | — | exact |
| Tick2 source placement | default | default | (2, 4) | exact |
| Resource route code | 1 | 1 | 1 | exact |
| Allocator total | 332 | 332 | 332 | exact |
| Faction A / B | 199/133 | 199/133 | 199/133 | exact |
| Invalid routes | 0 | 0 | 0 | exact |
| Overflow flags | 0 | 0 | 0 | exact |

## Replay fingerprint design and value

Combined fingerprint XORs tick0/tick1 movement hashes, shadow before/after hashes, tick1/tick2 mapping hashes, movement feedback delta hash, and overflow flags.

**`6c01851a4afdfcbf`**

Two identical runs produce the same shadow hashes, tick2 mapping hash, and combined fingerprint.

## Coverage matrix

| Classification | Coverage |
|---|---|
| GpuVerified | tick0/tick1/tick2 resource routes, mapping+EML+FIELD_POLICY live pipe |
| FixtureCandidate | movement candidate evolution |
| FixtureOnly | closed-loop feedback, seed placement override |
| OwnColumnShadowWrite | movement application to shadow position |
| ReplayAccepted | — |
| NotImplemented | ClauseThing |
| Pending | production multi-tick runtime, production movement routes |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v2_2_movement_feedback_application -- --nocapture
→ 12 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v2_1_candidate_evolution -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v2_0_closed_loop_consumer -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v1_5_live_field_agent_route -- --nocapture
→ pass (regression)

cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture
→ pass (regression)

cargo check --workspace
→ pass
```

## Scans run

| Scan | Result |
|---|---|
| `rg "FrontierV2-2\|frontier_v2_2\|phase_m_frontier_v2_2\|OwnColumnShadowWrite\|movement feedback" crates docs` | fixture/report/docs present |
| `rg "ACT-5\|EVENT-3\|OBS-5\|PIPE-1" crates docs` | guardrail-only |
| Guardrail scan | no unauthorized widening |
| simthing-sim semantic scan | no matches |
| Self-acceptance phrase scan | no implementer closure in V2-2 report/docs |
| f64/Candidate C scan | no regression |
| scratch/tmp find | no artifacts deleted |

## Transient cleanup result

No scratch artifacts deleted; no E-phase evidence removed.

## FrontierV2 Relevance

1. **How does FrontierV2-2 build on FrontierV2-1?** Reuses evolved movement candidates and closed-loop feedback from V2-1, then applies M1 to fixture-only own-column shadow state and feeds updated placement into tick2.
2. **What own-column movement application did it prove?** Movement M1 updated shadow from (0,0) to (2,4) via `OwnColumnShadowWrite`; cross-entity writes rejected.
3. **How did the movement write affect the next tick?** Tick2 faction-A seed placed at shadow (2,4) instead of (0,0), producing mapping hash change vs tick1.
4. **How is cross-entity movement prevented?** `validate_movement_write_target` rejects mismatched unit_id; no simthing-sim state writes.
5. **What remains fixture-only?** Shadow state, movement application, seed placement override, entire closed-loop state.
6. **What remains not implemented?** ClauseThing, production scenario runtime, default SimSession wiring, production movement GPU routes, ACT-5/EVENT-3/OBS-5/PIPE-1.
7. **Why this is not ClauseThing?** No authoring front-end, no production commitment emission, driver test support only.
8. **Why this does not declare phase closure?** Implementer fixture reports statuses only; design-authority closure remains separate.

## Final verdict

**PASS** — FrontierV2-2 extended the default-off FrontierV2 closed-loop consumer with fixture-only own-column movement feedback application; movement candidate output updated own-column shadow position; the changed source placement fed the next tick; cross-entity movement writes were rejected; resource dispatch stayed through Resource Flow allocator; CPU oracle parity and replay reproducibility were recorded (fingerprint `6c01851a4afdfcbf`); docs and production plan were updated; ClauseThing was not implemented; no phase closure was declared; no default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, parallel fixture economy, shared-pool tick writes, simthing-sim semantic awareness, or ACT/EVENT/OBS/PIPE expansion was added; and V7.7 / Mapping ADR / Resource Flow ADR / FIELD_POLICY charter posture remained intact.

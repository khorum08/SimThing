# FrontierV2-0 — Multi-Tick Closed-Loop Self-AI Consumer Fixture Results

## Base HEAD

`25ecc493b432da054b967edae00bf32b38ffa926` (post-FrontierV1-5 design-authority ACCEPT, pre-FrontierV2-0)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v2.rs` | **New** — FrontierV2 closed-loop types, admission, feedback application |
| `crates/simthing-driver/tests/support/frontier_v1.rs` | Allow FrontierV2 profile in default-off and coupling validators |
| `crates/simthing-driver/tests/phase_m_frontier_v2_0_closed_loop_consumer.rs` | **New** — two-tick closed-loop fixture + 11 tests |
| `docs/tests/phase_m_frontier_v2_0_closed_loop_consumer_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV2-0 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV2-0 row |
| `docs/workshop/sead_self_ai_track.md` | §10 FrontierV2-0 addendum |
| `docs/worklog.md` | Append-only milestone |

**No ClauseThing, no semantic WGSL, no default SimSession wiring, no phase closure declaration.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did FrontierV1-5 prove?** | One live GPU-resident single-tick score→threshold→proposal→dispatch resource route; Resource Flow allocator dispatch; fixture-only `FrontierV1LiveSelfAiFeedbackCandidate`; fingerprint `1653b84847be2dd2`. |
| **2. What does the Forward Horizon require for FrontierV2?** | Multi-tick closed-loop where field-derived SEAD proposals drive movement/dispatch feeding back into field/economy next tick; consumer proof not hygiene; no implementer phase closure. |
| **3. Smallest multi-tick closed-loop consumer fixture?** | Two ticks: tick0 live route → feedback candidate → apply seed deltas → tick1 live route → verify delta. |
| **4. Which V1-5 feedback fields feed tick 1?** | `faction_a_allocation` / `faction_b_allocation` → `next_tick_seed_delta` on district outputs; `field_feedback_code` → structural candidate; dispatch/proposal codes preserved. |
| **5. Observed closed-loop behavior?** | Tick1 mapping hash differs from tick0 after feedback-adjusted seeds (visible field/economy input change). |
| **6. Structural/movement in V2-0?** | `FrontierV2MovementCandidate` and `FrontierV2StructuralCandidate` derived from feedback — **FixtureCandidate**, tied to closed-loop path. |
| **7. Remains ReplayAccepted or deferred?** | ClauseThing NotImplemented; no production runtime; atlas/mask/perception/etc. still rejected; ACT-5/EVENT-3/OBS-5/PIPE-1 not authorized. |
| **8. Why not ClauseThing / production runtime?** | Driver test fixture only; no simthing-sim semantic state; no default SimSession wiring; no scenario engine. |

## FrontierV2 Forward Horizon summary

FrontierV2-0 implements the named Forward Horizon consumer as a bounded two-tick fixture. It does not implement ClauseThing, declare phase closure, or reopen the SEAD ladder.

## Fixture layout

```text
FrontierV2 admission (default-off, profile=FrontierV2)
  tick 0:
    GPU mapping + EML → GPU PIPE-0 → GPU ACT-2 → ResourceFlowAllocator
    → FrontierV1LiveSelfAiFeedbackCandidate
    → movement/structural FixtureCandidates
  feedback application (fixture-only seed deltas)
  tick 1:
    GPU mapping + EML → GPU PIPE-0 → GPU ACT-2 → ResourceFlowAllocator
    → verify mapping/proposal delta vs tick 0
```

Fixture ID: `frontier_v2_0_closed_loop_consumer_v1`

## Explicit opt-in profile settings

| Setting | Value |
|---|---|
| `profile_name` | `FrontierV2` |
| `enabled_by_default` | `false` |
| `closed_loop_ticks` | `2` |
| `mapping_execution_profile` | `SparseRegionFieldV1` |
| `resource_flow_opt_in` | `FlatStarOptIn` |
| `resource_flow_execution_profile` | `FlatStarResourceFlow` |
| `grid` | 8×8 smoke |

## Tick 0 live-loop summary

**GpuVerified** — mapping + EML + PIPE-0 + ACT-2 + Resource Flow allocator; feedback candidate produced.

## Feedback candidate application

```text
tick1.district_output_a = tick0.base_a + faction_a_allocation / 8
tick1.district_output_b = tick0.base_b + faction_b_allocation / 8
```

Mechanism: **FixtureOnly** `next_tick_seed_delta` (no simthing-sim state).

Smoke: tick0 seeds 120/80 → tick1 seeds 145/97.

## Tick 1 live-loop summary

**GpuVerified** — same live GPU chain with feedback-adjusted seeds; mapping hash differs from tick0.

## Closed-loop delta summary

| Metric | Tick 0 | Tick 1 | Changed |
|---|---|---|---|
| Mapping hash | `14372289007418635547` | `10465360340528472241` | yes |
| Self-AI hash | stable | stable | no (same live pipe inputs from EML) |
| Proposal/dispatch hash | stable | stable | no |
| Closed-loop delta hash | — | `6685338266462229892` | recorded |

Mapping hash change satisfies the closed-loop consumer proof for V2-0.

## Structural/movement candidate status

| Candidate | Status |
|---|---|
| Movement (`FrontierV2MovementCandidate`) | **FixtureCandidate** |
| Structural (`FrontierV2StructuralCandidate`) | **FixtureCandidate** |
| Resource dispatch | **GpuVerified** (both ticks) |

## CPU oracle parity table

| Field | Tick 0 | Tick 1 | Match |
|---|---|---|---|
| Resource route code | 1 | 1 | exact |
| Allocator total | 332 | 332 | exact |
| Faction A / B | 199/133 | 199/133 | exact |
| Invalid routes | 0 | 0 | exact |
| Overflow flags | 0 | 0 | exact |
| Feedback delta (seeds) | base | +25/+17 | exact |

## Replay fingerprint design and value

Combined fingerprint XORs tick0/tick1 mapping hashes, self-AI hashes, feedback hash, closed-loop delta hash, and overflow flags.

**`0238c18ce3b559da`**

## Coverage matrix

| Classification | Coverage |
|---|---|
| GpuVerified | tick0/tick1 resource routes, resource flow, mapping+EML+SEAD live pipe |
| FixtureCandidate | movement, structural candidates |
| FixtureOnly | closed-loop feedback application |
| ReplayAccepted | — |
| NotImplemented | ClauseThing |
| Pending | production multi-tick runtime |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v2_0_closed_loop_consumer -- --nocapture
→ 11 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v1_5_live_self_ai_route -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v1_4_sead_route_replay -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_sead_pipe0_observer_event_pipeline -- --nocapture
→ pass (regression)

cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture
→ pass (regression)

cargo check --workspace
→ pass
```

## Scans run

| Scan | Result |
|---|---|
| FrontierV2 fixture/docs | present; fixture/test support only |
| ACT-5/EVENT-3/OBS-5/PIPE-1 | guardrail-only |
| Guardrail scan | no unauthorized widening |
| simthing-sim semantic scan | no matches |
| Self-acceptance phrase scan | no implementer closure in V2-0 report/docs |
| f64/Candidate C scan | no regression |

## Transient cleanup result

No scratch artifacts deleted; no E-phase evidence removed.

## FrontierV2 Relevance

1. **How does FrontierV2-0 consume FrontierV1-5?** Reuses live GPU PIPE-0 + ACT-2 + Resource Flow route and applies `FrontierV1LiveSelfAiFeedbackCandidate` as tick1 seed input.
2. **What multi-tick closed-loop behavior did it prove?** Two-tick loop: tick0 dispatch → feedback → tick1 field input change → tick1 live route re-run.
3. **How did feedback from tick 0 affect tick 1?** Faction allocation-derived seed deltas changed district outputs (120/80 → 145/97), producing a different mapping hash.
4. **What movement/dispatch/structural candidate was produced?** Movement candidate with row delta from faction imbalance; structural candidate with boundary_request_code from admission.
5. **What remains fixture-only?** Feedback application, movement/structural candidates, entire closed-loop state.
6. **What remains ReplayAccepted?** Nothing newly promoted beyond V1-5; structural/movement remain FixtureCandidate not production GpuVerified routes.
7. **Why not ClauseThing?** No authoring front-end, no production scenario engine, driver test support only.
8. **Why no phase closure?** Implementer fixture reports statuses only; design-authority closure remains separate.

## Final verdict

**PASS** — FrontierV2-0 created the first default-off multi-tick closed-loop self-AI consumer fixture; consumed the FrontierV1-5 feedback candidate; ran two ticks; showed tick-0 feedback changing tick-1 field mapping input; preserved Resource Flow allocator routing; recorded CPU oracle parity and replay fingerprint `0238c18ce3b559da`; kept structural/movement as FixtureCandidate; did not implement ClauseThing; did not declare phase closure.

# FrontierV2-3 — Structural BoundaryRequest Feedback Application Results

## Base HEAD

`aa70478563cb077532fc0782d4e7c71030764368` (post-FrontierV2-2 merge, pre-FrontierV2-3)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v2.rs` | BoundaryRequest shadow queue, structural feedback application, summary hashing |
| `crates/simthing-driver/tests/phase_m_frontier_v2_3_structural_feedback_application.rs` | **New** — three-tick structural feedback fixture + 12 tests |
| `docs/tests/phase_m_frontier_v2_3_structural_feedback_application_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV2-3 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV2-3 row |
| `docs/workshop/field_policy_track.md` | FrontierV2-3 addendum |
| `docs/worklog.md` | Append-only milestone |

**No ClauseThing, no production commitment emission, no semantic WGSL, no default SimSession wiring, no phase closure declaration.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did FrontierV2-2 prove?** | Movement M1 applied to own-column shadow `(0,0)→(2,4)`; tick2 source placement changed mapping hash. Replay fingerprint `6c01851a4afdfcbf`. |
| **2. What structural candidate fields are available?** | `FrontierV2StructuralCandidate`: proposal_code, boundary_request_code, route_code, dispatch_count; evolved via `build_evolved_structural_candidate`. |
| **3. What does “BoundaryRequest shadow application” mean?** | Enqueue fixture-only `FrontierV2BoundaryRequestShadow` when route_code == ThresholdEmitBoundaryRequest (code 2); classified `BoundaryRequestShadowWrite`. |
| **4. How will the fixture prevent production commitment emission?** | Shadow queue is test-only; admission rejects `cpu_commitment_emission`; no simthing-sim writes; invalid routes rejected. |
| **5. How will structural feedback feed the next tick’s route?** | `derive_next_tick_structural_feedback_code` from shadow → `apply_structural_feedback_to_config` as fixture-only seed delta modifier on tick2. |
| **6. What counts as observed structural feedback?** | Shadow `applied=true`; tick2 mapping hash differs from tick1. |
| **7. What remains fixture-only?** | BoundaryRequest shadow, structural feedback seed modifier, closed-loop state — driver test support only. |
| **8. Why this still does not implement ClauseThing or production runtime?** | No authoring front-end, no production commitment emission, no simthing-sim semantic state. |

## Fixture layout

```text
FrontierV2 admission (default-off, profile=FrontierV2)
  tick 0:
    GPU live route → structural S0
  tick 1:
    feedback-adjusted input → GPU live route → structural S1
    apply S1 to BoundaryRequest shadow (BoundaryRequestShadowWrite)
  tick 2:
    feedback + structural feedback seed deltas → GPU live route
    verify mapping delta vs tick 1
```

Fixture ID: `frontier_v2_3_structural_feedback_application_v1`

## Tick 0 summary

**Live route: GpuVerified** — default seeds (0,0) and (7,7).

| Field | Value |
|---|---|
| Structural S0 | evolved from live feedback |
| Seeds | district_output_a=120, district_output_b=80 |

## Tick 1 structural candidate summary

**Live route: GpuVerified** — feedback-adjusted seeds (145/97).

| Field | Value |
|---|---|
| Structural S1 | `boundary_request_code=5142`, `route_code=2` (ThresholdEmitBoundaryRequest) |
| Mapping hash | `10465360340528472241` |

## BoundaryRequest shadow write summary

| Field | Before | After | Classification |
|---|---|---|---|
| applied | false | true | BoundaryRequestShadowWrite |
| route_code | 0 | 2 | ThresholdEmitBoundaryRequest |
| boundary_request_code | 0 | 5142 | from S1 |
| proposal_code | 0 | from S1 | recorded |

## Tick 2 structural feedback summary

**Live route: GpuVerified** — tick1 feedback + structural feedback seed deltas.

| Field | Value |
|---|---|
| structural_feedback_code | derived from shadow (boundary + proposal + dispatch) |
| Mapping hash | `15921985127655497477` (differs from tick1) |
| Resource route | allocator (code 1) |

## Structural feedback delta summary

| Metric | Tick 1 | Tick 2 | Changed |
|---|---|---|---|
| Mapping hash | `10465360340528472241` | `15921985127655497477` | yes |
| Shadow applied | false → true | — | yes |
| Structural feedback delta hash | — | `3136016491576718370` | recorded |

## Movement status

Movement candidates derived per tick as **FixtureCandidate** only; not applied to own-column shadow in V2-3 (structural focus).

## CPU oracle parity table

| Field | Tick 0 | Tick 1 | Tick 2 | Match |
|---|---|---|---|---|
| Structural candidate (oracle) | S0 | S1 | S2 | exact |
| BoundaryRequest shadow | empty | applied | — | exact |
| structural_feedback_code | — | — | derived | exact |
| Resource route code | 1 | 1 | 1 | exact |
| Invalid routes | 0 | 0 | 0 | exact |
| Overflow flags | 0 | 0 | 0 | exact |

## Replay fingerprint design and value

Combined fingerprint XORs tick0/tick1 structural hashes, shadow before/after hashes, tick1/tick2 mapping hashes, structural feedback delta hash, overflow flags, and structural feedback code.

**`0ad0e0d7c80316ee`**

## Coverage matrix

| Classification | Coverage |
|---|---|
| GpuVerified | tick0/tick1/tick2 resource routes, live pipe |
| FixtureCandidate | structural/movement candidates |
| FixtureOnly | closed-loop feedback, structural seed modifier |
| BoundaryRequestShadowWrite | structural application |
| OwnColumnShadowWrite | — (not used in V2-3) |
| ReplayAccepted | — |
| NotImplemented | ClauseThing |
| Pending | production structural commitment |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v2_3_structural_feedback_application -- --nocapture
→ 12 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v2_2_movement_feedback_application -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v2_1_candidate_evolution -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v2_0_closed_loop_consumer -- --nocapture
→ pass (regression)

cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture
→ pass (regression)

cargo check --workspace
→ pass
```

## Scans run

| Scan | Result |
|---|---|
| FrontierV2-3 fixture/docs | present |
| ACT-5/EVENT-3/OBS-5/PIPE-1 | guardrail-only |
| Guardrail scan | no unauthorized widening |
| simthing-sim semantic scan | no FrontierV1/V2/FIELD_POLICY matches |
| Self-acceptance phrase scan | no implementer closure |
| f64/Candidate C scan | no regression |
| scratch/tmp find | no artifacts deleted |

## Transient cleanup result

No scratch artifacts deleted; no E-phase evidence removed.

## FrontierV2 Relevance

1. **How does FrontierV2-3 build on FrontierV2-2?** Reuses three-tick closed-loop live GPU route; replaces movement shadow application with BoundaryRequest shadow application and structural feedback into tick2 seeds.
2. **What BoundaryRequest shadow application did it prove?** S1 enqueued shadow record with `route_code=2`, `applied=true`, classified `BoundaryRequestShadowWrite`.
3. **How did the structural write affect the next tick?** Derived structural feedback code modified tick2 district seed deltas; mapping hash changed vs tick1.
4. **How is production commitment emission prevented?** Shadow queue is fixture-only; admission rejects commitment emission; invalid structural routes rejected; no simthing-sim writes.
5. **What remains fixture-only?** BoundaryRequest shadow, structural feedback modifier, entire closed-loop state.
6. **What remains not implemented?** ClauseThing, production commitment emission, default SimSession wiring, ACT-5/EVENT-3/OBS-5/PIPE-1.
7. **Why this is not ClauseThing?** No authoring front-end, no production commitment path — driver test support only.
8. **Why this does not declare phase closure?** Implementer fixture reports statuses only; design-authority closure remains separate.

## Final verdict

**PASS** — FrontierV2-3 extended the default-off FrontierV2 closed-loop consumer with fixture-only structural BoundaryRequest feedback application; structural candidate output enqueued a BoundaryRequest shadow record; structural context fed the next tick; invalid structural routes were rejected; no production commitment emission occurred; resource dispatch stayed through Resource Flow allocator; CPU oracle parity and replay reproducibility were recorded (fingerprint `0ad0e0d7c80316ee`); docs and production plan were updated; ClauseThing was not implemented; no phase closure was declared.

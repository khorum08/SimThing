# FrontierV2-4 — Combined Movement + Structural Feedback Loop Results

## Base HEAD

`9e4ed05a6a644da70cc4f1a8f79e80bae7264a50` (post-FrontierV2-3 merge, pre-FrontierV2-4)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v2.rs` | Combined feedback config, summary hashing |
| `crates/simthing-driver/tests/phase_m_frontier_v2_4_combined_feedback_loop.rs` | **New** — four-tick combined feedback fixture + 12 tests |
| `docs/tests/phase_m_frontier_v2_4_combined_feedback_loop_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV2-4 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV2-4 row |
| `docs/workshop/sead_self_ai_track.md` | FrontierV2-4 addendum |
| `docs/worklog.md` | Append-only milestone |

**No ClauseThing, no production commitment emission, no semantic WGSL, no default SimSession wiring, no phase closure declaration.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did FrontierV2-2 prove?** | Movement M1 → own-column shadow `(0,0)→(2,4)`; tick2 source placement changed mapping hash. Fingerprint `6c01851a4afdfcbf`. |
| **2. What did FrontierV2-3 prove?** | Structural S1 → BoundaryRequest shadow (`applied=true`); tick2 structural seed modifier changed mapping hash. Fingerprint `0ad0e0d7c80316ee`. |
| **3. What shared support already exists?** | `FrontierV2OwnColumnShadow`, `FrontierV2BoundaryRequestShadow`, apply helpers, seed placement, structural feedback derivations. |
| **4. How can both feedback paths coexist without conflict?** | Apply both shadows after tick1; tick2/tick3 use movement placement + `apply_combined_feedback_to_config` (closed-loop + structural modifier). |
| **5. How will cross-entity movement be prevented?** | `validate_movement_write_target` rejects mismatched unit_id. |
| **6. How will production commitment emission be prevented?** | BoundaryRequest shadow only; admission rejects `cpu_commitment_emission`; invalid structural routes rejected. |
| **7. What counts as observed combined feedback?** | tick2/tick3 mapping hashes differ from tick1; both shadows applied; deterministic replay fingerprint. |
| **8. What remains fixture-only?** | Both shadows, combined feedback modifier, closed-loop state — driver test support only. |
| **9. Why not ClauseThing or production runtime?** | No authoring front-end, no production commitment path, no simthing-sim semantic state. |

## Fixture layout

```text
tick 0: live GPU route → M0, S0
tick 1: feedback-adjusted → live route → M1, S1
        apply M1 → own-column shadow (OwnColumnShadowWrite)
        apply S1 → BoundaryRequest shadow (BoundaryRequestShadowWrite)
tick 2: combined feedback + movement placement → live route
tick 3: combined feedback continuation + movement placement → live route
```

Fixture ID: `frontier_v2_4_combined_feedback_loop_v1`

## Tick 0 summary

**GpuVerified** — default seeds; M0/S0 recorded.

## Tick 1 movement + structural application summary

**GpuVerified** — feedback-adjusted seeds (145/97).

| Application | Result | Classification |
|---|---|---|
| Movement M1 | shadow `(0,0)→(2,4)` | OwnColumnShadowWrite |
| Structural S1 | boundary=5142, route_code=2, applied=true | BoundaryRequestShadowWrite |
| combined_feedback_code | 6146 | derived from boundary shadow |

## Tick 2 combined feedback summary

**GpuVerified** — movement placement `(2,4)` + structural seed modifier + closed-loop feedback.

| Field | Value |
|---|---|
| Mapping hash | `11406265130728233769` (≠ tick1) |
| Seeds | combined district deltas at shadow placement |

## Tick 3 deterministic continuation summary

**GpuVerified** — same combined mechanism; mapping hash `6956700612629482482` (≠ tick2).

## Own-column shadow write summary

| Before | After |
|---|---|
| (0, 0) | (2, 4) |

## BoundaryRequest shadow write summary

| Field | Value |
|---|---|
| applied | true |
| route_code | 2 (ThresholdEmitBoundaryRequest) |
| boundary_request_code | 5142 |

## Combined feedback delta summary

| Metric | Tick 1 | Tick 2 | Tick 3 | Changed |
|---|---|---|---|---|
| Mapping hash | `10465360340528472241` | `11406265130728233769` | `6956700612629482482` | yes |
| Combined delta hash | — | — | `17749355492507662485` | recorded |

Baseline comparison to movement-only/structural-only fixtures deferred (optional per handoff); combined tick delta recorded.

## CPU oracle parity table

| Field | Tick 0–3 | Match |
|---|---|---|
| Movement/structural candidates | oracle-derived | exact |
| Movement shadow | (2, 4) | exact |
| BoundaryRequest shadow | applied, code 6146 | exact |
| Resource route | allocator | exact |
| Invalid routes | 0 | exact |
| Overflow flags | 0 | exact |

## Replay fingerprint design and value

Combined fingerprint XORs movement/boundary shadow hashes, tick2/tick3 mapping hashes, combined feedback delta hash, overflow flags, and combined feedback code.

**`dbb54b952f9face8`**

## Coverage matrix

| Classification | Coverage |
|---|---|
| GpuVerified | tick0–tick3 resource routes, live pipe |
| FixtureCandidate | movement/structural candidates |
| FixtureOnly | combined closed-loop feedback |
| OwnColumnShadowWrite | movement application |
| BoundaryRequestShadowWrite | structural application |
| ReplayAccepted | — |
| NotImplemented | ClauseThing |
| Pending | production movement/commitment |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v2_4_combined_feedback_loop -- --nocapture
→ 12 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v2_3_structural_feedback_application -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v2_2_movement_feedback_application -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v2_1_candidate_evolution -- --nocapture
→ pass (regression)

cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture
→ pass (regression)

cargo check --workspace
→ pass
```

## Scans run

| Scan | Result |
|---|---|
| FrontierV2-4 fixture/docs | present |
| ACT-5/EVENT-3/OBS-5/PIPE-1 | guardrail-only |
| Guardrail scan | no unauthorized widening |
| simthing-sim semantic scan | no matches |
| Self-acceptance phrase scan | no implementer closure |
| f64/Candidate C scan | no regression |
| scratch/tmp find | no artifacts deleted |

## Transient cleanup result

No scratch artifacts deleted; no E-phase evidence removed.

## FrontierV2 Relevance

1. **How does FrontierV2-4 build on V2-2 and V2-3?** Combines movement shadow application and BoundaryRequest shadow application in one four-tick fixture with shared live GPU route.
2. **What combined feedback loop did it prove?** Both shadows applied after tick1; tick2/tick3 use movement placement plus structural seed modifier plus closed-loop feedback together.
3. **How did movement and structural writes affect downstream ticks?** Movement changed source placement to (2,4); structural modifier changed district seeds; mapping hashes changed tick1→tick2→tick3.
4. **How are cross-entity movement and production commitment prevented?** Movement write validation; invalid structural route rejection; commitment emission admission rejection; no simthing-sim writes.
5. **What remains fixture-only?** Both shadows, combined feedback, entire closed-loop state.
6. **What remains not implemented?** ClauseThing, production commitment emission, default SimSession wiring.
7. **Why this is not ClauseThing?** Driver test support only; no production commitment path.
8. **Why this does not declare phase closure?** Implementer fixture reports statuses only.

## Final verdict

**PASS** — FrontierV2-4 combined fixture-only own-column movement feedback and BoundaryRequest structural feedback inside the default-off FrontierV2 consumer; both shadows fed downstream ticks; cross-entity movement writes and invalid structural routes were rejected; no production commitment emission occurred; resource dispatch stayed through Resource Flow allocator; CPU oracle parity and replay reproducibility were recorded (fingerprint `dbb54b952f9face8`); ClauseThing was not implemented; no phase closure was declared.

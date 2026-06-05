# FrontierV2-1 — Closed-Loop Movement/Structural Candidate Evolution Results

## Base HEAD

`ea0abb184fab8a168c4b429aa7863b9d07e96bc6` (post-FrontierV2-0 merge, pre-FrontierV2-1)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v2.rs` | Evolved candidate builders, `FrontierV2CandidateEvolutionSummary`, delta hashing |
| `crates/simthing-driver/tests/phase_m_frontier_v2_1_candidate_evolution.rs` | **New** — two-tick candidate evolution fixture + 11 tests |
| `docs/tests/phase_m_frontier_v2_1_candidate_evolution_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV2-1 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV2-1 row |
| `docs/workshop/field_policy_track.md` | FrontierV2-1 addendum |
| `docs/worklog.md` | Append-only milestone |

**No ClauseThing, no semantic WGSL, no default SimSession wiring, no phase closure declaration.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did FrontierV2-0 prove?** | First bounded two-tick closed-loop consumer: tick0 live GPU route → feedback candidate → tick1 seed delta → tick1 live route → mapping hash changed. Replay fingerprint `0238c18ce3b559da`. Movement/structural were static FixtureCandidates from tick0 feedback only. |
| **2. What remains FixtureCandidate?** | Movement and structural outputs — derived on CPU in fixture support, not GPU-resident production routes. V2-1 extends evolution across ticks but keeps the same classification. |
| **3. What movement/structural fields already exist in support code?** | `FrontierV2MovementCandidate` (source_unit_id, delta_row, delta_col, route_code, dispatch_count); `FrontierV2StructuralCandidate` (proposal_code, boundary_request_code, route_code, dispatch_count); V2-0 static builders; feedback fields from `FrontierV1LiveFieldAgentFeedbackCandidate`. |
| **4. What smallest feedback rule can make movement/structural candidates evolve across ticks?** | Tick-aware evolved builders: movement row/col deltas incorporate tick index, mapping hash nibble, urgency bucket; structural boundary/proposal codes incorporate tick index, mapping hash, allocator bucket. Tick1 also runs on feedback-adjusted seeds (different mapping hash). |
| **5. What should count as observed candidate evolution?** | `M1 != M0` and/or `S1 != S0` after two live ticks with closed-loop feedback; recorded in `candidate_delta_hash`. |
| **6. What remains fixture-only?** | Feedback seed application, evolved movement/structural candidates, entire closed-loop state, admission validators — driver test support only. |
| **7. Why this still does not implement ClauseThing or production runtime?** | No authoring front-end, no default SimSession wiring, no simthing-sim semantic state, no scenario engine; candidates are test-derived integers not production commitments. |

## Fixture layout

```text
FrontierV2 admission (default-off, profile=FrontierV2)
  tick 0:
    GPU mapping + EML → GPU PIPE-0 → GPU ACT-2 → ResourceFlowAllocator
    → FrontierV1LiveFieldAgentFeedbackCandidate
    → evolved movement M0 / structural S0 (FixtureCandidate)
  feedback application (fixture-only seed deltas)
  tick 1:
    GPU mapping + EML → GPU PIPE-0 → GPU ACT-2 → ResourceFlowAllocator
    → evolved movement M1 / structural S1 (FixtureCandidate)
    → assert M1 != M0 and S1 != S0
```

Fixture ID: `frontier_v2_1_candidate_evolution_v1`

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

## Tick 0 candidate summary

**Live route: GpuVerified** — mapping + EML + PIPE-0 + ACT-2 + Resource Flow allocator.

| Candidate | Value |
|---|---|
| Movement M0 | `delta_row=1`, `delta_col=6`, `route_code=3`, `dispatch_count=2` |
| Structural S0 | `boundary_request_code=5039`, `proposal_code` from admission, `dispatch_count=2` |
| Resource route | allocator (code 1) |
| Faction A / B | 199 / 133 |
| Allocator total | 332 |
| Seeds | district_output_a=120, district_output_b=80 |

## Tick 1 candidate summary

**Live route: GpuVerified** — same chain with feedback-adjusted seeds (120/80 → 145/97).

| Candidate | Value |
|---|---|
| Movement M1 | `delta_row=2`, `delta_col=4`, `route_code=3`, `dispatch_count=2` |
| Structural S1 | `boundary_request_code=5142`, evolved proposal_code, `dispatch_count=2` |
| Mapping hash | differs from tick0 (closed-loop field input change) |
| Resource route | allocator (code 1) |
| Faction A / B | 199 / 133 (same allocation; seed delta drives mapping change) |

## Candidate delta summary

| Metric | Tick 0 | Tick 1 | Changed |
|---|---|---|---|
| Movement row delta | 1 | 2 | yes |
| Movement col delta | 6 | 4 | yes |
| Structural boundary | 5039 | 5142 | yes |
| Movement hash | recorded | recorded | yes |
| Structural hash | recorded | recorded | yes |
| Candidate delta hash | — | `17700730053898035224` | recorded |

Both movement and structural candidates evolved (`M1 != M0`, `S1 != S0`). Evolution is tied to closed-loop feedback via tick index, mapping hash (changed by seed deltas), urgency, and allocator/field feedback codes.

## Movement/structural candidate classification

| Candidate | Status |
|---|---|
| Movement evolution (M0→M1) | **FixtureCandidate** |
| Structural evolution (S0→S1) | **FixtureCandidate** |
| Tick0/tick1 resource routes | **GpuVerified** |
| Closed-loop feedback | **FixtureOnly** |
| ClauseThing | **NotImplemented** |
| Phase closure | **NotDeclared** |

## CPU oracle parity table

| Field | Tick 0 | Tick 1 | Match |
|---|---|---|---|
| Movement candidate (oracle) | M0 fields | M1 fields | exact |
| Structural candidate (oracle) | S0 fields | S1 fields | exact |
| Resource route code | 1 | 1 | exact |
| Allocator total | 332 | 332 | exact |
| Faction A / B | 199/133 | 199/133 | exact |
| Invalid routes | 0 | 0 | exact |
| Overflow flags | 0 | 0 | exact |
| Feedback seed delta | base | +25/+17 | exact |

## Replay fingerprint design and value

Combined fingerprint XORs tick0/tick1 movement hashes, tick0/tick1 structural hashes, candidate delta hash, closed-loop delta hash, and overflow flags.

**`2d6e78a06d19736a`**

Two identical runs produce the same tick0/tick1 candidate hashes, candidate delta hash, and combined fingerprint.

## Coverage matrix

| Classification | Coverage |
|---|---|
| GpuVerified | tick0/tick1 resource routes, mapping+EML+FIELD_POLICY live pipe |
| FixtureCandidate | movement/structural candidate evolution |
| FixtureOnly | closed-loop feedback application |
| ReplayAccepted | — |
| NotImplemented | ClauseThing |
| Pending | production multi-tick runtime, production movement/structural routes |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v2_1_candidate_evolution -- --nocapture
→ 11 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v2_0_closed_loop_consumer -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v1_5_live_field_agent_route -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_field_policy_pipe0_observer_event_pipeline -- --nocapture
→ pass (regression)

cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture
→ pass (regression)

cargo check --workspace
→ pass
```

## Scans run

| Scan | Result |
|---|---|
| `rg "FrontierV2-1\|frontier_v2_1\|phase_m_frontier_v2_1\|candidate evolution" crates docs` | fixture/report/docs present |
| `rg "ACT-5\|EVENT-3\|OBS-5\|PIPE-1" crates docs` | guardrail-only; no next-number fixture authorization |
| Guardrail scan (default SimSession, scheduler, semantic WGSL, etc.) | guardrail-only in V2-1 report/docs |
| `rg "FrontierV1\|FrontierV2\|FIELD_POLICY\|RegionCell\|ArenaRegistry\|proposal\|ResourceFlow" crates/simthing-sim` | no simthing-sim semantic awareness |
| Self-acceptance phrase scan | no implementer closure in V2-1 report/docs |
| `rg "F64RoundDown\|SHADER_F64\|f64(\|sqrt_cr_c\|Candidate C\|native sqrt.*Exact" crates docs` | no Candidate C/f64/native exact sqrt regression |
| `find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \)` | no scratch artifacts deleted |

## Transient cleanup result

No scratch artifacts deleted; no E-phase evidence removed.

## FrontierV2 Relevance

1. **How does FrontierV2-1 build on FrontierV2-0?** Reuses the two-tick closed-loop live GPU route and feedback seed application from V2-0, then adds tick-aware evolved movement/structural FixtureCandidates that change across ticks.
2. **What candidate evolution did it prove?** Movement row/col deltas and structural boundary/proposal codes both changed from tick0 to tick1 (`M1 != M0`, `S1 != S0`).
3. **How is the evolution tied to closed-loop feedback?** Candidates incorporate feedback fields (faction imbalance, dispatch_count, field_feedback_code, allocator_total) plus tick index and mapping hash (which changes because feedback-adjusted seeds alter tick1 field input).
4. **What remains fixture-only?** Evolved candidates, feedback application, admission validators, entire closed-loop state — driver test support only.
5. **What remains not implemented?** ClauseThing, production scenario runtime, default SimSession wiring, production movement/structural GPU routes, ACT-5/EVENT-3/OBS-5/PIPE-1 ladder expansion.
6. **Why this is not ClauseThing?** No authoring front-end, no production commitment emission, no simthing-sim semantic state — only test-derived integer candidates.
7. **Why this does not declare phase closure?** Implementer fixture reports statuses only; design-authority closure remains separate.

## Final verdict

**PASS** — FrontierV2-1 extended the default-off FrontierV2 closed-loop consumer with fixture-only movement/structural candidate evolution across ticks; candidate changes were tied to closed-loop feedback; resource dispatch stayed through Resource Flow allocator; CPU oracle parity and replay reproducibility were recorded (fingerprint `2d6e78a06d19736a`); docs and production plan were updated; ClauseThing was not implemented; no phase closure was declared; no default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, parallel fixture economy, shared-pool tick writes, simthing-sim semantic awareness, or ACT/EVENT/OBS/PIPE expansion was added; and V7.7 / Mapping ADR / Resource Flow ADR / FIELD_POLICY charter posture remained intact.

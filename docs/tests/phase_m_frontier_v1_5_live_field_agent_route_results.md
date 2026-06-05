# FrontierV1-5 — Live GPU Field agent Route Run Toward FrontierV2 Results

## Base HEAD

`0fc5f840bd7f805f014675b0acc1923a21679796` (pre-FrontierV1-5)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/field_policy_v1_live_pipeline.rs` | **New** — shared GPU PIPE-0 + ACT-2 chain helpers for live Frontier fixture |
| `crates/simthing-driver/tests/support/frontier_v1.rs` | Live field agent types, feedback candidate, GPU execution hash |
| `crates/simthing-driver/tests/phase_m_frontier_v1_5_live_field_agent_route.rs` | **New** — live integrated field agent resource route fixture + 10 tests |
| `docs/tests/phase_m_frontier_v1_5_live_field_agent_route_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV1-5 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV1-5 row / status update |
| `docs/workshop/field_policy_track.md` | §10 FrontierV1-5 addendum |
| `docs/worklog.md` | Append-only milestone |

**No new semantic WGSL, kernel descriptor, AccumulatorRole, default SimSession wiring, scheduler/cache, simthing-sim semantic awareness, ACT-5/EVENT-3/OBS-5/PIPE-1, or FrontierV2 implementation.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did Opus accept as substrate?** | First-slice 8×8 mapping + reduction/`field_urgency` EML and flat-star Resource Flow allocation GPU-verified with exact CPU-oracle parity; routing guardrails honored; FIELD_POLICY ladder consolidated/closed; Phase E closes at FlatStarResourceFlow (design authority, not implementer memos). |
| **2. What remains unproven about the field agent loop?** | Before V1-5: score→threshold→proposal→dispatch was `ReplayAccepted` inside Frontier (descriptors consumed, route codes oracle-checked; no live GPU integrated run). |
| **3. What did the Forward Horizon banner add?** | Named next consumer **`FrontierV2`** — multi-tick closed-loop where field-derived FIELD_POLICY proposals drive movement/dispatch feeding back into field/economy; implementer fixtures may report statuses but may not declare phase closure; orient to consumer, not hygiene loops. |
| **4. How does FrontierV1-5 build toward FrontierV2 without implementing FrontierV2?** | Single-tick live GPU chain + fixture-only `FrontierV1LiveFieldAgentFeedbackCandidate` next-tick payload shape; no multi-tick loop, no default runtime wiring. |
| **5. What does one live GPU route require?** | GPU mapping+EML → field-derived observer rows → GPU PIPE-0 → GPU ACT-2 chain → resource dispatch via Resource Flow allocator → CPU oracle parity + replay fingerprint. |
| **6. Which accepted FIELD_POLICY V1 GPU pieces can be invoked directly?** | PIPE-0 observer→threshold→compact (`run_pipe0_gpu`); ACT-2 bucket→reduce→propose→consume→admit (`run_act2_chain_gpu`); landed descriptors consumed via `validate_field_policy_v1_consumed()`. |
| **7. Which outputs must be shaped as next-tick feedback candidates?** | `FrontierV1LiveFieldAgentFeedbackCandidate` (tick, source unit, route/proposal codes, dispatch count, allocator totals, faction allocations, field feedback code, overflow flags). |
| **8. What remains deferred or prohibited?** | FrontierV2 multi-tick loop; structural/movement live routes (ReplayAccepted); atlas, active mask, perception, source identity, nested E-11B/E-11B-5, D-2a, ClauseThing; ACT-5/EVENT-3/OBS-5/PIPE-1; default SimSession; scheduler/cache; semantic WGSL; CPU planner/urgency/commitment. |

## Opus §10 design-authority ruling summary

Substrate (mapping + flat-star RF) accepted; Phase E closes at FlatStarResourceFlow. Implementer `FrontierV1-ACCEPT-0`/`POSTACCEPT-0` are not closure. FIELD_POLICY field agent loop was honestly `ReplayAccepted` until a live GPU run. Forward Horizon names **FrontierV2** as the multi-tick consumer; V1-5 is the single-tick bridge producing a feedback-candidate shape without implementing FrontierV2.

## Forward Horizon / FrontierV2 guidance summary

FrontierV2 remains **NotImplemented** — the named multi-tick closed-loop consumer. V1-5 emits fixture-only feedback candidates that FrontierV2 can later consume. No phase closure declared.

## Live loop layout

```text
FrontierV1 admission (default-off)
  → GPU mapping (SparseRegionFieldV1) + reduction/EML readback (threat/urgency)
  → field-derived observer rows (2 rows, event_code 1)
  → GPU PIPE-0: score → threshold → compact (2 events)
  → GPU ACT-2: bucket → reduce → propose → consume → admit (proposal_code 1001)
  → GPU flat-star Resource Flow allocation (ResourceFlowAllocator route)
  → FrontierV1LiveFieldAgentFeedbackCandidate (FixtureOnly)
  → CPU oracle parity + combined replay fingerprint
```

Fixture ID: `frontier_v1_5_live_field_agent_route_v1`

## Explicit opt-in profile settings

| Setting | Value |
|---|---|
| `profile_name` | `FrontierV1` |
| `enabled_by_default` | `false` |
| `mapping_execution_profile` | `SparseRegionFieldV1` |
| `resource_flow_opt_in` | `FlatStarOptIn` |
| `resource_flow_execution_profile` | `FlatStarResourceFlow` |

## Mapping GPU status

**GpuVerified** — 8×8 smoke; CPU reference tolerance 1e-4; threat≈3958.344 urgency≈798.069.

## Resource Flow GPU status

**GpuVerified** — smoke allocator total 332; faction allocations 199/133; overflow 0.

## FIELD_POLICY V1 live execution status

| Component | Status |
|---|---|
| PIPE-0 GPU (score/threshold/compact) | **GpuVerified** (2 events, overflow 0) |
| ACT-2 GPU chain | **GpuVerified** (2 proposals, admission admitted) |
| Full integrated pipe | **GpuVerified** |
| Resource-dispatch route | **GpuVerified** (ResourceFlowAllocator) |
| Structural route | **ReplayAccepted** (not live-run in V1-5) |
| Movement route | **ReplayAccepted** (not live-run in V1-5) |

## Route status matrix

| Route | Code | Count | Status |
|---|---|---|---|
| Resource dispatch | 1 (ResourceFlowAllocator) | ≥1 | **GpuVerified** |
| Structural commit | 2 (ThresholdEmitBoundaryRequest) | 1 (oracle) | **ReplayAccepted** |
| Movement | 3 (OwnColumnsOnly) | 1 (oracle) | **ReplayAccepted** |
| Invalid routes | — | 0 | — |
| FrontierV2 | — | — | **NotImplemented** |

## FrontierV2-facing feedback candidate shape

```rust
FrontierV1LiveFieldAgentFeedbackCandidate {
    tick_index: 0,
    source_unit_id: 0,
    route_code: 1,           // ResourceFlowAllocator
    proposal_code: 1001,
    dispatch_count: 2,
    allocator_total: 332,
    faction_a_allocation: 199,
    faction_b_allocation: 133,
    field_feedback_code: 5001,
    overflow_flags: 0,
}
```

Status: **FixtureOnly** — not production runtime or simthing-sim state.

## CPU oracle parity table

| Field | GPU / fixture | CPU oracle | Match |
|---|---|---|---|
| Observer rows (event_code 1) | 2 | 2 (CPU threshold) | exact |
| PIPE-0 event count | 2 | 2 | exact |
| ACT-2 proposal count | 2 | chain verify | exact |
| Resource route code | 1 | 1 | exact |
| Allocator total | 332 | 332 | exact |
| Faction A / B | 199 / 133 | 199 / 133 | exact |
| Feedback candidate fields | see above | oracle | exact |
| Overflow flags | 0 | 0 | exact |
| Invalid route count | 0 | 0 | exact |

## Replay fingerprint design and value

Combined fingerprint XORs mapping hash, RF hash, field agent GPU execution hash, feedback candidate hash, route replay hash, and overflow flags.

**`1653b84847be2dd2`**

## Coverage matrix

| Classification | Coverage |
|---|---|
| GpuVerified | mapping, resource flow, PIPE-0, ACT-2, resource-dispatch route |
| PartialGpuVerified | — |
| ReplayAccepted | structural route, movement route |
| FixtureOnly | feedback candidate |
| OutOfScopeForV1-5 | — |
| Pending | FrontierV2 multi-tick consumer |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v1_5_live_field_agent_route -- --nocapture
→ 10 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v1_4_field_policy_route_replay -- --nocapture
→ pass (regression)

cargo test -p simthing-driver --test phase_m_frontier_v1_3_gpu_resource_flow -- --nocapture
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
| `rg "FrontierV1-5|frontier_v1_5|FrontierV2" crates docs` | fixture + docs present; FrontierV2 as future consumer |
| `rg "ACT-5|EVENT-3|OBS-5|PIPE-1" crates docs` | guardrail-only / negative references |
| Guardrail scan (default SimSession, scheduler, etc.) | guardrail-only in V1-5 docs |
| `rg "FrontierV1|FrontierV2|FIELD_POLICY|..." crates/simthing-sim` | no semantic awareness |
| Self-acceptance phrase scan on V1-5 report/docs | no implementer closure declarations |
| `rg "F64RoundDown|Candidate C|sqrt_cr_c" crates docs` | no regression |

## Transient cleanup result

No E-phase/E11 evidence deleted. No scratch `.log` artifacts removed from `docs/tests` (none required).

## M/E / Field agent / FrontierV2 Relevance

1. **How does FrontierV1-5 respond to Opus §10?** Executes the required one live GPU-resident integrated score→threshold→proposal→dispatch resource-dispatch route inside default-off FrontierV1, addressing the correction that replay alone was insufficient for the bounded smoke route.
2. **What was accepted before V1-5?** Mapping + flat-star RF GPU-verified; FIELD_POLICY ladder consolidated; route replay `ReplayAccepted` for full pipe inside Frontier.
3. **What live GPU field agent route did V1-5 prove?** Field-derived observer rows → GPU PIPE-0 (2 events) → GPU ACT-2 (proposal 1001, admission 5001) → GPU Resource Flow allocator dispatch.
4. **What FrontierV2-facing feedback candidate did V1-5 produce?** Fixture-only `FrontierV1LiveFieldAgentFeedbackCandidate` with route/proposal/dispatch/allocator/faction/feedback fields for a future multi-tick loop.
5. **What remains ReplayAccepted?** Structural and movement routes (not live-run in V1-5).
6. **Does V1-5 satisfy the single-tick field-as-policy proof requirement?** Yes for the bounded single-tick resource-dispatch smoke route.
7. **What remains non-blocking/deferred?** FrontierV2 multi-tick closed-loop; structural/movement live GPU routes; atlas, active mask, perception, ClauseThing, etc.
8. **Why this does not declare phase closure or implement FrontierV2?** This is a bridge fixture with honest statuses and fixture-only feedback artifacts; FrontierV2 status is NotImplemented; no acceptance/post-acceptance memo; design-authority closure remains separate from implementer reports.

## Final verdict

**PASS** — FrontierV1-5 executed one live GPU-resident integrated field agent resource-dispatch route inside the default-off FrontierV1 fixture; score→threshold→proposal→dispatch ran through accepted FIELD_POLICY V1 substrates without reopening the ladder; resource dispatch reached the Resource Flow allocator; a fixture-only feedback candidate shape was produced for the named FrontierV2 closed-loop consumer; CPU oracle parity and replay reproducibility were recorded (`1653b84847be2dd2`); structural/movement routes remained honestly ReplayAccepted; docs and production plan were updated; FrontierV2 was not implemented; no phase closure was declared.

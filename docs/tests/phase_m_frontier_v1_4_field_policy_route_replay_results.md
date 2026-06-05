# FrontierV1-4 — Integrated FIELD_POLICY V1 Route Replay Acceptance Results

## Base HEAD

`3343f5a` (post-FrontierV1-3 merge, pre-FrontierV1-4)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v1.rs` | `ReplayAccepted` status, route/FIELD_POLICY replay types, V1-4 replay summary builder |
| `crates/simthing-driver/tests/support/field_policy_v1_route_replay.rs` | **New** — FIELD_POLICY V1 PIPE-0/ACT-2 descriptor consumption |
| `crates/simthing-driver/tests/phase_m_frontier_v1_4_field_policy_route_replay.rs` | **New** — integrated route replay fixture + 9 tests |
| `docs/tests/phase_m_frontier_v1_4_field_policy_route_replay_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV1-4 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV1-4 row update |
| `docs/worklog.md` | Append-only milestone |

**No new WGSL, kernel descriptor, AccumulatorRole, default SimSession wiring, scheduler/cache, or simthing-sim semantic awareness.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did FrontierV1-3 GPU-verify?** | 8×8 mapping; reduction+EML; flat-star Resource Flow allocation; exact CPU oracle parity; fingerprint `7bacf7921b807bee` |
| **2. Still CPU-oracle-only after V1-3** | Route classification execution; full FIELD_POLICY PIPE-0 chain inside FrontierV1; final M/E acceptance review |
| **3. Reusable FIELD_POLICY V1 fixtures** | Landed PIPE-0 (`m_jit_field_policy_pipe0_observer_event_pipeline`) and ACT-2 (`m_jit_field_policy_act2_proposal_admission_records`) descriptors via `landed_jit_kernel_descriptors()`; FrontierV1 `classify_proposal_route()` + CPU oracles from V1-1 |
| **4. GPU-backed vs replay-accepted routing** | Route codes: **ReplayAccepted** via admission + CPU oracle; FIELD_POLICY observer→EML subset: **GpuVerified** via mapping tick; full PIPE-0 observer→event→compact: **ReplayAccepted** (consumed, not re-run) |
| **5. CPU oracle parity checks** | Resource/structural/movement route codes; route counts; event/proposal/admission counts; overflow=0; invalid routes=0 |
| **6. Replay fingerprint update** | Combined replay fingerprint: **`4382ec7ef93c9174`** (includes `field_policy_summary_hash`) |
| **7. Remains deferred/rejected** | Full PIPE-0 GPU re-run inside FrontierV1; GPU route dispatch execution; atlas, active mask, perception, nested E-11B, D-2a, ACT-5/EVENT-3/OBS-5/PIPE-1 |
| **8. M/E closure movement** | Integrates FIELD_POLICY V1 route replay after GPU mapping+RF; honest ReplayAccepted classification; ready for acceptance review |

## Fixture layout

```text
FrontierV1 admission (V1-0 validator)
  → validate_field_policy_v1_consumed() (PIPE-0 + ACT-2 descriptors)
  → GPU mapping (V1-2/V1-3 pattern) + reduction/EML
  → GPU flat-star Resource Flow allocation (V1-3 pattern)
  → build_route_replay_summary() (ReplayAccepted route codes)
  → build_field_policy_replay_summary() (event/proposal/admission counts)
  → build_frontier_v1_4_replay_summary()
```

Fixture ID: `frontier_v1_4_field_policy_route_replay_v1`

## Explicit opt-in profile settings

| Setting | Value |
|---|---|
| `profile_name` | `FrontierV1` |
| `enabled_by_default` | `false` |
| `mapping_execution_profile` | `SparseRegionFieldV1` |
| `resource_flow_opt_in` | `FlatStarOptIn` |
| `resource_flow_execution_profile` | `FlatStarResourceFlow` |

## Mapping GPU status

**GpuVerified** — unchanged from V1-2/V1-3.

## Resource Flow GPU status

**GpuVerified** — unchanged from V1-3 (smoke: 199/133/332).

## FIELD_POLICY V1 consumption/replay status

| Component | Status |
|---|---|
| PIPE-0 descriptor registered/admitted | **replay_accepted** (consumed) |
| ACT-2 descriptor registered/admitted | **replay_accepted** (consumed) |
| Reduction + EML (field_urgency) | **gpu_verified** |
| Full PIPE-0 GPU chain re-run | **replay_accepted** (not re-run; V1 evidence consumed) |

## Route replay status

| Route | Code | Count (smoke) | Status |
|---|---|---|---|
| Resource dispatch | 1 (ResourceFlowAllocator) | 1 | **replay_accepted** |
| Structural commit | 2 (ThresholdEmitBoundaryRequest) | 1 | **replay_accepted** |
| Movement | 3 (OwnColumnsOnly) | 1 | **replay_accepted** |
| Invalid routes | — | 0 | verified |
| Overflow flags | — | 0 | verified |

## CPU oracle parity table

| Field | Replay | CPU oracle | Match |
|---|---|---|---|
| `resource_route_code` | 1 | ResourceFlowAllocator | yes |
| `structural_route_code` | 2 | ThresholdEmitBoundaryRequest | yes |
| `movement_route_code` | 3 | OwnColumnsOnly | yes |
| `resource_route_count` | 1 | 1 | yes |
| `structural_route_count` | 1 | 1 | yes |
| `movement_route_count` | 1 | 1 | yes |
| `proposal_count` | 3 | 3 | yes |
| `event_count` | 1 | 1 | yes |
| `admission_count` | 1 | 1 | yes |
| `invalid_route_count` | 0 | 0 | yes |
| `route_overflow_flags` | 0 | 0 | yes |

## Replay fingerprint design and value

```text
FrontierV1GpuReplaySummary {
    mapping_summary_hash,         // GPU field FNV
    resource_flow_summary_hash,   // GPU RF FNV
    field_policy_summary_hash,            // FIELD_POLICY replay FNV (V1-4 addition)
    proposal_summary_hash,        // V1-1 CPU oracle
    route_summary_hash,           // V1-1 CPU oracle
    mapping_status = GpuVerified,
    resource_flow_status = GpuVerified,
    field_policy_routing_status = GpuVerified,
    field_policy_pipe_status = ReplayAccepted,
    route_status = ReplayAccepted,
}
```

Combined replay fingerprint: **`4382ec7ef93c9174`**

Prior fingerprints preserved: V1-2 `42b0455e4d0b59ac`, V1-3 `7bacf7921b807bee` (field_policy hash excluded when zero).

## Coverage matrix

| Substrate | Status |
|---|---|
| First-slice RegionCell mapping | **gpu_verified** |
| Reduction + EML (field_urgency) | **gpu_verified** |
| Flat-star Resource Flow allocation | **gpu_verified** |
| FIELD_POLICY PIPE-0 full chain | **replay_accepted** |
| Route classification execution | **replay_accepted** |
| Final M/E acceptance review | **pending** |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v1_4_field_policy_route_replay -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_3_gpu_resource_flow -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_2_gpu_replay_acceptance -- --nocapture
  → 9/9 PASS

cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture
  → 29/29 PASS

cargo check --workspace
  → PASS
```

| Test | Result |
|---|---|
| `frontier_v1_4_happy_path_field_policy_route_replay_runs` | PASS |
| `frontier_v1_4_route_replay_cpu_oracle_parity` | PASS |
| `frontier_v1_4_route_replay_reproducibility` | PASS |
| `frontier_v1_4_defaults_remain_disabled` | PASS |
| `frontier_v1_4_rejects_route_bypasses` | PASS |
| `frontier_v1_4_coupling_rejects_non_frontier_profile` | PASS |
| `frontier_v1_4_deferred_features_reject` | PASS |
| `frontier_v1_4_no_simthing_sim_semantic_awareness` | PASS |
| `frontier_v1_4_no_unauthorized_gpu_primitive` | PASS |

## Scans run

| Scan | Result |
|---|---|
| `FrontierV1-4\|frontier_v1_4\|phase_m_frontier_v1_4` in crates/docs | fixture + report present |
| `ACT-5\|EVENT-3\|OBS-5\|PIPE-1` in crates/docs | no next-number authorization |
| guardrail terms in report + active docs | guardrail-only |
| simthing-sim semantic markers | no matches |
| Candidate C/f64 regression | forbidden-term scan refs only in existing FIELD_POLICY tests |
| scratch/tmp cleanup | E-phase `.log` evidence retained |

## Transient cleanup result

No scratch/tmp artifacts removed.

## M/E Closure Relevance

1. **Builds on FrontierV1-3:** Adds FIELD_POLICY V1 route replay acceptance on top of GPU-verified mapping and Resource Flow allocation.
2. **Phase M proof:** GPU mapping + RF substrates remain verified; FIELD_POLICY observer→EML subset GPU-resident via mapping tick.
3. **Phase E proof:** Resource, structural, and movement proposals route only through accepted substrates; replay reproducibility recorded.
4. **Still pending:** Final FrontierV1 acceptance review; optional full PIPE-0 GPU re-run inside fixture (honestly replay_accepted today).
5. **Non-blocking deferred:** Atlas, active mask, perception, source identity, nested E-11B, D-2a, ClauseThing, CPU planner.
6. **Not a new FIELD_POLICY ladder:** Consumes PIPE-0 + ACT-2 landed descriptors; no ACT-5/EVENT-3/OBS-5/PIPE-1.

FrontierV1-4 integrates accepted FIELD_POLICY Field agent Proposal Pipeline V1 route replay into the default-off FrontierV1 fixture after mapping and Resource Flow GPU verification. It verifies that proposals route only through accepted substrates and records replay reproducibility without extending the FIELD_POLICY ladder, adding semantic WGSL, or adding default runtime behavior.

## Final verdict

**PASS** — FrontierV1-4 integrated accepted FIELD_POLICY V1 route replay into the default-off FrontierV1 fixture; preserved GPU-verified mapping and Resource Flow allocation; verified resource, structural, and movement proposal routes against accepted substrates; recorded replay reproducibility; honestly classified FIELD_POLICY PIPE-0 full chain and route execution as ReplayAccepted; added no default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, shared-pool tick writes, parallel fixture economy, or simthing-sim semantic awareness; updated docs and production plan; saved test results in `docs/tests`; kept V7.7 / Mapping ADR / Resource Flow ADR / FIELD_POLICY charter posture intact.

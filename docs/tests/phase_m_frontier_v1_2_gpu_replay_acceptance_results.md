# FrontierV1-2 — GPU-Resident Execution and Replay Acceptance Results

## Base HEAD

`3591d96` (post-FrontierV1-1 merge, pre-FrontierV1-2)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v1.rs` | GPU mapping spec, replay summary types, field hashing |
| `crates/simthing-driver/tests/phase_m_frontier_v1_2_gpu_replay_acceptance.rs` | **New** — GPU fixture + 9 tests |
| `docs/tests/phase_m_frontier_v1_2_gpu_replay_acceptance_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV1-2 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV1-2 row update |
| `docs/worklog.md` | Append-only milestone |

**No new WGSL, kernel descriptor, AccumulatorRole, default SimSession wiring, scheduler/cache, or simthing-sim semantic awareness.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. FrontierV1-1 proved** | End-to-end opt-in fixture wiring; CPU oracle parity; replay fingerprint `49d4c94ce1f52be5`; route classification; no default wiring |
| **2. CPU oracle outputs GPU must align with** | Mapping: GPU field values vs `cpu_horizon` reference (not V1-1 integer oracle); RF/routes/proposal counts: V1-1 CPU oracle |
| **3. Reusable GPU paths** | `FirstSliceMappingSession` + `SparseRegionFieldV1`; GPU reduction+EML via `diagnostic_readback_reduction_eml`; existing stencil/AccumulatorOp substrate |
| **4. CPU-oracle-only in V1-2** | Resource Flow allocation summary (pending GPU FrontierV1-3); SEAD full PIPE-0 chain not re-run; route classification remains CPU oracle |
| **5. Replay fingerprint** | Combined GPU replay fingerprint: **`42b0455e4d0b59ac`** |
| **6. Deferred/rejected** | Atlas, active mask, perception, source identity, nested E-11B/E-11B-5, D-2a, ClauseThing, ACT-N ladder, CPU planner |
| **7. M/E closure movement** | First GPU-resident mapping execution + replay for FrontierV1; RF GPU integration explicitly pending |

## GPU fixture layout

```text
FrontierV1 admission (V1-0 validator)
  → frontier_v1_mapping_field_spec() (8×8 SourceCappedNormalized)
  → FirstSliceMappingSession::open(SparseRegionFieldV1)
  → queue district seeds (0,0) + (7,7)
  → GPU tick (debug readback) + reduction/EML readback
  → hash_gpu_field_values()
  → CPU oracle for RF + routes (V1-1)
  → FrontierV1GpuReplaySummary
```

Fixture ID: `frontier_v1_2_gpu_replay_acceptance_v1`

## Explicit opt-in profile settings

| Setting | Value |
|---|---|
| `profile_name` | `FrontierV1` |
| `enabled_by_default` | `false` |
| `mapping_execution_profile` | `SparseRegionFieldV1` (explicit) |
| `resource_flow_opt_in` | `FlatStarOptIn` (explicit in skeleton) |
| Disabled profile control | `MappingExecutionProfile::Disabled` → no GPU execution |

## Mapping GPU execution summary

- 8×8 first-slice RegionCell field, `source_capped_normalized`, horizon 8
- District coupling seeds at corners (120.0 / 80.0)
- GPU field values match `cpu_horizon` CPU reference within 1e-4
- Reduction + EML executed on GPU (`gpu_reduction_eml_executed = true`)
- Status: **gpu_verified**

## Resource Flow execution status

| Component | Status |
|---|---|
| Flat-star allocation summary | **cpu_oracle_only** — FrontierV1-3 pending GPU integration |
| Route classification | **cpu_oracle_only** |
| Allocator routing contract | **verified** (ResourceFlowAllocator classification) |

Resource Flow GPU execution via SimSession flat-star path exists in repo but is not integrated into this FrontierV1 fixture slice without new wiring primitives. Not overclaimed.

## SEAD proposal routing summary

| Component | Status |
|---|---|
| Reduction + EML (field_urgency path) | **gpu_verified** (partial SEAD substrate) |
| Full PIPE-0 observer/event/proposal GPU chain | **not re-run** — V1 consumed, not extended |
| Route classification | **cpu_oracle_only** |

## CPU oracle parity table

| Field | GPU verified | CPU oracle only | Match |
|---|---|---|---|
| Mapping field values | yes | `cpu_horizon` reference | yes (≤1e-4) |
| Mapping summary hash | yes | — | replay-stable |
| Resource Flow allocation | — | yes | V1-1 oracle |
| Proposal/route counts | — | yes | V1-1 oracle |
| Overflow flags | — | yes | 0 on smoke |

## Replay fingerprint design and value

```text
FrontierV1GpuReplaySummary {
    mapping_summary_hash,      // GPU field to_bits FNV
    resource_flow_summary_hash,  // V1-1 CPU oracle
    proposal_summary_hash,       // V1-1 CPU oracle
    route_summary_hash,          // V1-1 CPU oracle
    overflow_flags,
}
```

Combined replay fingerprint: **`42b0455e4d0b59ac`**

## GPU coverage matrix

| Substrate | Status |
|---|---|
| First-slice RegionCell mapping | **gpu_verified** |
| Reduction + EML (field_urgency) | **gpu_verified** |
| Flat-star Resource Flow allocation | **cpu_oracle_only / pending_gpu (FrontierV1-3)** |
| SEAD PIPE-0 full chain | **cpu_oracle_only** (consumed, not extended) |
| Route classification | **cpu_oracle_only** |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v1_2_gpu_replay_acceptance -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_1_opt_in_fixture -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_0_scenario_skeleton -- --nocapture
  → 8/8 PASS

cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture
  → 29/29 PASS

cargo check --workspace
  → PASS
```

| Test | Result |
|---|---|
| `frontier_v1_2_happy_path_gpu_fixture_runs` | PASS |
| `frontier_v1_2_defaults_remain_disabled` | PASS |
| `frontier_v1_2_gpu_cpu_oracle_parity` | PASS |
| `frontier_v1_2_gpu_replay_reproducibility` | PASS |
| `frontier_v1_2_resource_dispatch_routes_through_allocator` | PASS |
| `frontier_v1_2_coupling_rejects_non_frontier_profile` | PASS |
| `frontier_v1_2_deferred_features_reject` | PASS |
| `frontier_v1_2_no_simthing_sim_semantic_awareness` | PASS |
| `frontier_v1_2_no_unauthorized_gpu_primitive` | PASS |

## Scans run

| Scan | Result |
|---|---|
| `FrontierV1-2\|frontier_v1_2\|phase_m_frontier_v1_2` in crates/docs | fixture + report present |
| `ACT-5\|EVENT-3\|OBS-5\|PIPE-1` | no next-number authorization |
| guardrail terms in report + active docs | guardrail-only |
| simthing-sim semantic markers | no matches |
| Candidate C/f64 regression | none |
| scratch/tmp cleanup | E-phase logs retained |

## Transient cleanup result

No scratch/tmp artifacts removed.

## M/E Closure Relevance

1. **Builds on FrontierV1-1:** Adds GPU-resident first-slice mapping execution and replay on top of V1-1 CPU-oracle fixture wiring.
2. **Phase M proof:** GPU executes bounded 8×8 RegionCell theater with explicit opt-in profile; field values match CPU stencil reference; replay reproducible.
3. **Phase E proof:** Resource Flow routing contract preserved via CPU oracle; GPU RF allocation honestly marked pending (FrontierV1-3).
4. **Still pending:** GPU flat-star Resource Flow integration in FrontierV1 fixture; full SEAD GPU chain end-to-end; production-doc M/E acceptance review.
5. **Non-blocking deferred:** Atlas, active mask, perception, source identity, nested E-11B, D-2a, ClauseThing.
6. **Not a SEAD ladder:** Consumes SEAD V1 + accepted mapping GPU substrate; no ACT-5/EVENT-3/OBS-5/PIPE-1.

FrontierV1-2 moves the named M/E closing vertical from CPU-oracle fixture wiring toward GPU-resident execution and replay acceptance. It consumes accepted Mapping, Resource Flow, and SEAD Self-AI substrates without extending the SEAD ladder or adding default runtime behavior. Resource Flow GPU allocation remains explicitly pending rather than overclaimed.

## Final verdict

**PASS** — FrontierV1-2 executed the default-off FrontierV1 fixture through GPU-resident accepted substrates where available; recorded GPU/CPU oracle parity and replay reproducibility; honestly classified Resource Flow allocation as CPU-oracle-only/pending GPU; preserved Resource Flow allocator routing, FrontierV1-only/default-off coupling, and all deferred-feature boundaries; added no default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, or simthing-sim semantic awareness; updated docs and production plan; and kept V7.7 / Mapping ADR / Resource Flow ADR / SEAD charter posture intact.

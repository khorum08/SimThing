# FrontierV1-3 — GPU Flat-Star Resource Flow Integration Results

## Base HEAD

`02e9033` (post-FrontierV1-2 merge, pre-FrontierV1-3)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v1.rs` | GPU RF summary types, hashing, replay summary `route_status`, flat-star weights |
| `crates/simthing-driver/tests/phase_m_frontier_v1_3_gpu_resource_flow.rs` | **New** — GPU RF fixture + 9 tests |
| `docs/tests/phase_m_frontier_v1_3_gpu_resource_flow_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV1-3 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV1-3 row update |
| `docs/worklog.md` | Append-only milestone |

**No new WGSL, kernel descriptor, AccumulatorRole, default SimSession wiring, scheduler/cache, or simthing-sim semantic awareness.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did FrontierV1-2 GPU-verify?** | First-slice 8×8 RegionCell mapping; reduction + EML readback; replay fingerprint `42b0455e4d0b59ac`; mapping field GPU/CPU parity |
| **2. Resource Flow pieces CPU-oracle-only in V1-2** | Flat-star allocation summary; route classification; full SEAD PIPE-0 chain |
| **3. Accepted flat-star GPU substrate** | E-11 flat-star path via `SimSession::open_from_spec` + `FlatStarOptIn` + `FlatStarResourceFlow`; `sync_resource_flow_if_enabled` + `run_resource_flow_bands`; E-11R bit-exact oracle in `e11_flat_star.rs` |
| **4. Fixture wiring required** | FrontierV1 admission → GPU mapping (V1-2) → separate flat-star GPU session (3 hosted cohorts: 1 allocator root + 2 faction leaves) → district-derived `allocator_total` as root intrinsic flow → weights 3:2 → GPU bands → hash |
| **5. CPU oracle parity checks** | `faction_a_allocation`, `faction_b_allocation`, `allocator_total`, overflow flags, route code = ResourceFlowAllocator (1); no shared-pool tick write; no parallel fixture economy |
| **6. Replay fingerprint update** | Combined GPU replay fingerprint: **`7bacf7921b807bee`** (V1-2 was `42b0455e4d0b59ac`) |
| **7. Remains deferred/rejected** | Route classification GPU execution; full SEAD PIPE-0 chain; atlas, active mask, perception, source identity, nested E-11B/E-11B-5, D-2a, ClauseThing, ACT-5/EVENT-3/OBS-5/PIPE-1, CPU planner |
| **8. M/E closure movement** | Closes Resource Flow GPU gap from FrontierV1-2; mapping + RF allocation GPU-verified inside default-off FrontierV1 fixture |

## GPU fixture layout

```text
FrontierV1 admission (V1-0 validator)
  → frontier_v1_mapping_field_spec() (8×8 SourceCappedNormalized)
  → FirstSliceMappingSession::open(SparseRegionFieldV1)
  → queue district seeds (0,0) + (7,7)
  → GPU tick (debug readback) + reduction/EML readback
  → hash_gpu_field_values()
  → SimSession::open_from_spec(flat_star_scenario(3), FlatStarOptIn + FlatStarResourceFlow)
  → sync_resource_flow_if_enabled + run_resource_flow_bands
  → E-11 oracle bit parity + u32 round vs V1-1 CPU oracle
  → hash_gpu_resource_flow()
  → FrontierV1GpuReplaySummary (resource_flow_status = GpuVerified)
```

Fixture ID: `frontier_v1_3_gpu_resource_flow_v1`

Flat-star topology note: E-11 flat-star D=2 uses the first explicit participant as allocator root; remaining hosted cohorts are faction leaves. FrontierV1 smoke uses **3 hosted cohorts → 2 faction leaves**.

## Explicit opt-in profile settings

| Setting | Value |
|---|---|
| `profile_name` | `FrontierV1` |
| `enabled_by_default` | `false` |
| `mapping_execution_profile` | `SparseRegionFieldV1` (explicit) |
| `resource_flow_opt_in` | `FlatStarOptIn` (explicit) |
| `resource_flow_execution_profile` | `FlatStarResourceFlow` (explicit) |
| Disabled profile control | `MappingExecutionProfile::Disabled`, `ResourceFlowOptInMode::Disabled`, `ResourceFlowExecutionProfile::DefaultDisabled` |

## Mapping GPU status

- **gpu_verified** (unchanged from V1-2)
- 8×8 first-slice RegionCell field, GPU field values match `cpu_horizon` reference within 1e-4
- Reduction + EML executed on GPU

## Resource Flow GPU execution summary

| Output | Value (smoke) | Status |
|---|---|---|
| `faction_a_allocation` | 199 | **gpu_verified** |
| `faction_b_allocation` | 133 | **gpu_verified** |
| `allocator_total` | 332 | **gpu_verified** |
| `resource_overflow_flags` | 0 | **gpu_verified** |
| `allocator_route_code` | 1 (ResourceFlowAllocator) | **gpu_verified** (admission contract) |

GPU path: accepted E-11 flat-star OrderBand allocation via existing accumulator Resource Flow ops — no new semantic WGSL or kernel descriptor.

## Resource Flow fixed-point / format contract

| Layer | Format | Parity contract |
|---|---|---|
| GPU substrate | `f32` column values (`allocated`, `weight`, `flow`) | Bit-exact vs `run_arena_allocation_oracle` before u32 conversion |
| FrontierV1 CPU oracle | `u32` integer arithmetic (`total * 3 / 5`) | GPU outputs converted via `v.round() as u32`; smoke fixture matches exactly |
| Coupling bonus | `mapping.cell_sum % 1000` added to district totals | CPU oracle derives `allocator_total`; GPU receives that total as root intrinsic flow |

Weights: `(3.0, 2.0)` via `frontier_v1_flat_star_weights()` — matches V1-1 3:2 integer split on smoke inputs.

## CPU oracle parity table

| Field | GPU | CPU oracle | Match |
|---|---|---|---|
| `faction_a_allocation` | 199 | 199 | yes |
| `faction_b_allocation` | 133 | 133 | yes |
| `allocator_total` | 332 | 332 | yes |
| `resource_overflow_flags` | 0 | 0 | yes |
| `allocator_route_code` | 1 | ResourceFlowAllocator | yes |
| E-11 oracle bit parity | yes | — | yes (per leaf) |
| Route classification execution | — | cpu_oracle_only | admission-enforced |

## Replay fingerprint design and value

```text
FrontierV1GpuReplaySummary {
    mapping_summary_hash,         // GPU field to_bits FNV
    resource_flow_summary_hash,     // GPU RF summary FNV (V1-3 change)
    proposal_summary_hash,          // V1-1 CPU oracle
    route_summary_hash,             // V1-1 CPU oracle
    overflow_flags,
    mapping_status = GpuVerified,
    resource_flow_status = GpuVerified,
    route_status = CpuOracleOnly,
}
```

Combined replay fingerprint: **`7bacf7921b807bee`**

Prior FrontierV1-2 fingerprint (`42b0455e4d0b59ac`) remains valid for the V1-2 fixture (CPU-oracle RF hash).

## GPU coverage matrix

| Substrate | Status |
|---|---|
| First-slice RegionCell mapping | **gpu_verified** |
| Reduction + EML (field_urgency) | **gpu_verified** |
| Flat-star Resource Flow allocation | **gpu_verified** |
| Route classification execution | **cpu_oracle_only** (admission + allocator route code enforced) |
| SEAD PIPE-0 full chain | **cpu_oracle_only** (consumed, not extended) |

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v1_3_gpu_resource_flow -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_2_gpu_replay_acceptance -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_1_opt_in_fixture -- --nocapture
  → 9/9 PASS

cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture
  → 29/29 PASS

cargo check --workspace
  → PASS
```

| Test | Result |
|---|---|
| `frontier_v1_3_happy_path_gpu_resource_flow_runs` | PASS |
| `frontier_v1_3_gpu_resource_flow_cpu_oracle_parity` | PASS |
| `frontier_v1_3_gpu_resource_flow_replay_reproducibility` | PASS |
| `frontier_v1_3_defaults_remain_disabled` | PASS |
| `frontier_v1_3_rejects_resource_flow_bypass` | PASS |
| `frontier_v1_3_coupling_rejects_non_frontier_profile` | PASS |
| `frontier_v1_3_deferred_features_reject` | PASS |
| `frontier_v1_3_no_simthing_sim_semantic_awareness` | PASS |
| `frontier_v1_3_no_unauthorized_gpu_primitive` | PASS |

## Scans run

| Scan | Result |
|---|---|
| `FrontierV1-3\|frontier_v1_3\|phase_m_frontier_v1_3` in crates/docs | fixture + report present |
| `ACT-5\|EVENT-3\|OBS-5\|PIPE-1` in crates/docs | no next-number authorization (stop/negative refs only) |
| guardrail terms in report + active docs | guardrail-only |
| simthing-sim semantic markers | no matches |
| Candidate C/f64 regression | forbidden-term scan refs only in existing SEAD tests |
| scratch/tmp cleanup | E-phase `.log` evidence retained (not scratch/tmp) |

## Transient cleanup result

No scratch/tmp artifacts removed. Existing E-phase `.log` files under `docs/tests/` retained per handoff policy.

## M/E Closure Relevance

1. **Builds on FrontierV1-2:** Adds GPU-resident flat-star Resource Flow allocation on top of V1-2 GPU mapping/replay; updates combined replay fingerprint to include GPU RF hash.
2. **Phase M proof:** GPU executes bounded 8×8 mapping plus flat-star OrderBand allocation through accepted substrates with explicit opt-in only.
3. **Phase E proof:** Resource dispatch remains routed through Resource Flow allocator; GPU faction allocations match CPU oracle exactly; no parallel fixture economy or shared-pool tick writes.
4. **Still pending before M/E close:** Full accepted substrate replay / acceptance review; route classification GPU execution (honestly cpu_oracle_only); full SEAD PIPE-0 GPU chain end-to-end; production-doc M/E closure sign-off.
5. **Non-blocking deferred:** Atlas, active mask, perception, source identity, nested E-11B, E-11B-5, D-2a, ClauseThing, CPU planner/urgency/commitment emission.
6. **Not a new SEAD ladder:** Consumes SEAD V1 + accepted mapping/RF GPU substrates; no ACT-5/EVENT-3/OBS-5/PIPE-1; named FrontierV1 integration slice only.

FrontierV1-3 closes the Resource Flow GPU gap identified by FrontierV1-2 by verifying flat-star allocation through the accepted Resource Flow allocator path inside the default-off FrontierV1 fixture. This moves Phase E toward closure without adding nested E-11B, D-2a, a parallel fixture economy, CPU planner behavior, or default runtime wiring.

## Final verdict

**PASS** — FrontierV1-3 GPU-verified flat-star Resource Flow allocation inside the default-off FrontierV1 fixture; matched the CPU oracle exactly; preserved Resource Flow allocator routing, FrontierV1-only/default-off coupling, and all deferred-feature boundaries; honestly classified route classification as CPU-oracle-only; added no default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, shared-pool tick writes, parallel fixture economy, or simthing-sim semantic awareness; updated docs and production plan; saved test results in `docs/tests`; kept V7.7 / Mapping ADR / Resource Flow ADR / SEAD charter posture intact.

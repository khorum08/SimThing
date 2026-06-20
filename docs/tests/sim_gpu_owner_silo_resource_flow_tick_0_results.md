# SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0 — resident GPU proof for admitted Owner silo flow

> **Lifecycle: PROBATION** — GPU participant accumulation over existing AccumulatorOp surfaces landed with scoped proof readback. Full owner-silo state mutation remains deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #786 — SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0
**Merge:** `3ef8fd03`
**Base:** `master` after PR #785 / SESSION-RESOURCE-FLOW-SILOS-HARDEN-0 (`faa84a67`)

## Current defect / deferral summary

| Slice | Status |
|---|---|
| Owner silo admission oracle (`evaluate_owner_silo_flow`) | PASS |
| Driver ResourceFlow materialization (explicit participants) | PASS |
| Invalid silo rejection before lowering | PASS |
| GPU participant surplus/deficit accumulation | PASS |
| Full owner-silo state mutation (reduce-up/disburse-down writes) | PARTIAL/deferred |
| Studio GPU dispatch | Deferred (out of scope) |

## Accumulator lowering model

- **simthing-spec** exports `owner_silo_flow_participant_inputs` from admitted scenarios.
- **simthing-driver** `compile_owner_silo_gpu_tick_plan` builds two generic `CompiledAccumulatorOpPlan` values (surplus sum + deficit sum) with explicit participant slots plus one aggregate slot.
- Reuses existing `AccumulatorOp` Sum-over-INPUT_LIST machinery — no owner-specific GPU primitive.

## Resident GPU tick ownership model

- **simthing-sim** owns `SimGpuAccumulatorTickState` initialized from driver-compiled plans.
- **simthing-gpu** reuses existing `AccumulatorOpSession` / existing WGSL.
- Proof integration lives in **simthing-driver** `owner_silo_gpu_tick.rs` (no upward sim dev-deps).

## CPU oracle comparison

| Fixture | Participant surplus sum | Participant deficit sum | Oracle reducible | Oracle resolvable | Oracle unresolved |
|---|---:|---:|---:|---:|---:|
| `owner_silo_balanced_flow` | 30 | 20 | 30 | 20 | 0 |
| `owner_silo_unresolved_deficit` | 5 | 50 | 5 | 15 | 35 |

GPU/CPU aggregate-slot outputs match participant sums on balanced flow corpus.

## Proof readback policy status

**PASS** — `SimGpuReadbackPolicy::ProofReadback` enables scoped readback; gate does not persist after proof tick (`owner_silo_gpu_tick_proof_readback_scoped`, `owner_silo_gpu_tick_readback_gate_restored`).

## None policy no-readback status

**PASS** — `SimGpuReadbackPolicy::None` returns `None` and does not enable debug readback (`owner_silo_gpu_tick_none_policy_does_not_enable_readback`).

## GPU adapter availability status

**REAL_ADAPTER_OBSERVED** — driver integration tests completed with live adapter (~2s GPU tick suite). If adapter unavailable, tests emit `GPU_TESTS_SKIPPED_NO_ADAPTER` and return without fake PASS.

## Full mutation vs projection/cache status

- GPU outputs are projection/cache/evidence only.
- Scenario authority (`SimThingScenarioSpec`) is not mutated by GPU proof ticks.
- `evaluate_owner_silo_flow` remains semantic truth for reduce-up/disburse-down totals.

## Driver/spec/sim/gpu boundary summary

| Crate | Role |
|---|---|
| simthing-spec | Oracle + explicit participant inputs + ingestion GPU readiness fields |
| simthing-driver | Accumulator plan compile + GPU integration proof |
| simthing-sim | Resident tick state + readback policy |
| simthing-gpu | Existing AccumulatorOp execution |
| simthing-mapeditor | No GPU dispatch (e10 guard) |

## Production synthesis update summary

- Corrected stale Generated Galaxy Authority deferral language.
- Added § SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0.
- Updated Next Production Rungs (full owner-silo state mutation deferred rung).

## Evidence lifecycle cleanup summary

**PASS** — live ledger preserved; new PROBATION results doc added; PRs #784/#785 prerequisites recorded; no DA promotion; no corpus fixture semantic changes.

## Specified-vs-implemented ledger

| Requirement | Status |
|---|---|
| Admitted flow lowers to AccumulatorOp | PASS |
| Invalid silo rejects before GPU | PASS |
| Explicit participants only | PASS |
| CPU/GPU participant accumulation parity | PASS |
| Scoped proof readback | PASS |
| None policy readback-free | PASS |
| No scenario authority mutation | PASS |
| Full state mutation | PARTIAL/deferred |
| No new WGSL/primitive/engine | PASS |
| Studio does not dispatch GPU | PASS |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS (72/72) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test session_resource_flow_silos` | PASS (20/20 + 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test session_resource_flow_silos` | PASS (6/6) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11/11) |
| `cargo check -p simthing-sim` | PASS |
| `cargo test -p simthing-sim` | PASS |
| `cargo check -p simthing-gpu` | PASS |
| `cargo test -p simthing-gpu` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/session_resource_flow.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/tests/session_resource_flow_silos.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-driver/src/owner_silo_accumulator_compile.rs` (new)
- `crates/simthing-driver/src/session_resource_flow_silos.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/owner_silo_gpu_tick.rs` (new)
- `crates/simthing-driver/tests/session_resource_flow_silos.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/sim_gpu_owner_silo_resource_flow_tick_0_results.md` (new)

## Deleted/archived artifacts

None.

## Deferred next rung recommendation

1. Studio ingestion/admission report display for owner-silo GPU readiness.
2. Full owner-silo state mutation (reduce-up/disburse-down writes to scenario authority).
3. Structural placement edit commands.
4. Planet/child-location admission.

## DA status

**PROBATION** — pending owner DA approval. No DA promotion in this PR.
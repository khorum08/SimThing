# ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 — Sum-over-INPUT_LIST canonical execution results

> **Lifecycle: PROBATION** — Sum-over-INPUT_LIST canonical proof landed; driver compile and sim tick seam behavioral; real-adapter GPU AO proof observed. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1  
**Base:** `master` after PR #759 / DA SPLIT ruling (`accumulator_driver_sim_convergence_0_results.md`)

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with CONVERGENCE-1 row |
| `docs/tests/accumulator_driver_sim_convergence_0_results.md` | DA RULING | Preserved — SPLIT source; referenced, not overwritten |
| `docs/tests/accumulator_driver_sim_convergence_1_results.md` | PROBATION | Created (this file) |
| `docs/tests/gpu_link_accumulator_smoke_0_results.md` | PROBATION | Preserved — historical baseline for deleted bespoke kernel |
| `docs/design_0_0_8_3_studio_production.md` | Living synthesis | Updated § ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 |
| `crates/simthing-gpu/src/accumulator_convergence.rs` | REJECTED | Deleted |
| `crates/simthing-driver/tests/accumulator_driver_sim_convergence_stub.rs` | REJECTED | Deleted |
| `crates/simthing-gpu/src/structural_link_accumulator.rs` | SUPERSEDED | Deleted after canonical proof |
| `crates/simthing-gpu/src/shaders/structural_link_accumulator.wgsl` | SUPERSEDED | Deleted after canonical proof |

No scratch logs, temp GPU dumps, or contradictory duplicate evidence left in tree.

## Why this is not hygiene

This rung decides whether terran-pirate horizon structural link coupling can run through the constitutional engine:

```text
scenario structural links
  -> driver-assembled canonical AccumulatorOp plan
  -> sim-owned execution seam
  -> semantic-free GPU operator (AO-WGSL-0)
```

instead of:

```text
Studio proof helper
  -> bespoke GPU kernel
  -> parallel runtime path
```

This is the reference instance for future Gu-Yang and PALMA convergence (STEAD §10).

## DA ruling ingestion

From `accumulator_driver_sim_convergence_0_results.md` (DA: SPLIT, 2026-06-18):

- Fence accepted: proof-only guard may remain until retired — **retired in this PR** via deletion after canonical proof.
- "AccumulatorOp infeasible without broad driver/sim redesign" — **overruled**; implemented bounded Sum-over-INPUT_LIST.
- Real gap was one bounded combine×source cell: **Sum over INPUT_LIST** — **implemented**.
- Documentation-as-code — **rejected and removed** (`accumulator_convergence.rs`, driver stub test).
- Convergence owed — **delivered** for RF/link surface 1.

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| Where is `CombineFn::Sum` limited to `SlotRange`? | Was limited in `cpu_oracle.rs` and `accumulator_op.wgsl`; extended to `SourceSpec::ConjunctiveCrossing` (INPUT_LIST encoding). |
| Where is INPUT_LIST defined and made GPU-resident? | `AccumulatorInputListTable` / `InputListRange` in `accumulator_op.rs`; uploaded via `AccumulatorOpSession::upload_ops_resolving_input_lists`. |
| Which INPUT_LIST combine paths already work? | `MIN_ACROSS_INPUTS` over INPUT_LIST; now `SUM` over INPUT_LIST added. |
| Minimal change for Sum-over-INPUT_LIST? | Mirror MIN gather loop in WGSL + CPU oracle for `COMBINE_SUM + SOURCE_INPUT_LIST`; no new primitive. |
| Driver compile links → INPUT_LIST rows? | `compile_structural_link_neighbor_sum_plan` in `simthing-driver/src/structural_link_accumulator_compile.rs` builds per-target `ConjunctiveCrossing` neighbor gathers. |
| Sim tick/boundary ownership? | `execute_accumulator_plan_tick_cpu` in `simthing-sim/src/accumulator_plan_tick.rs` seeds input col, zeros output col, calls `execute_ops_cpu`. |
| Studio prevented from runtime execution? | Proof helpers removed; app/load paths scanned; behavioral guard proves driver→sim path without Studio GPU dispatch. |
| Documentation-as-code removed? | `accumulator_convergence.rs`, `accumulator_driver_sim_convergence_stub.rs`, `accumulator_convergence_guards.rs` deleted. |
| structural_link_accumulator retirement? | Module + WGSL **deleted**; tests migrated to `accumulator_op_sum_input_list_convergence.rs`. |

## Sum-over-INPUT_LIST implementation summary

- **CPU oracle:** `crates/simthing-gpu/src/accumulator_op/cpu_oracle.rs` — `CombineFn::Sum` gathers `ConjunctiveCrossing` input lists.
- **WGSL:** `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` — `COMBINE_SUM` + `SOURCE_INPUT_LIST` gather loop (mirrors existing MIN path).
- **Plan type:** `CompiledAccumulatorOpPlan` in `crates/simthing-core/src/compiled_accumulator_plan.rs`.
- **Exact f32 integer contract:** values with `|v| <= 2^24` and integral represent exactly in f32; vertical seed `[10,20]→[20,10]` proven.

## AccumulatorOp CPU oracle proof

| Test | Result |
|---|---|
| `accumulator_op_sum_over_input_list_cpu_oracle_vertical_seed` | PASS |
| `accumulator_op_sum_over_input_list_cpu_oracle_chain` | PASS |
| `accumulator_op_sum_over_input_list_cpu_oracle_triangle` | PASS |
| `accumulator_op_sum_over_input_list_rejects_invalid_input_list_range` | PASS |
| `accumulator_op_sum_over_input_list_exact_integer_f32_contract` | PASS |
| `accumulator_op_sum_over_input_list_shader_contains_no_forbidden_semantic_terms` | PASS |

## AccumulatorOp GPU proof

| Test | Result |
|---|---|
| `accumulator_op_sum_over_input_list_gpu_vertical_seed_real_adapter_or_partial` | PASS — **REAL_ADAPTER_OBSERVED** |

GPU path: `AccumulatorOpSession::upload_ops_resolving_input_lists` + `tick()` on real adapter; output `[20.0, 10.0]` from `[10.0, 20.0]`.

## Driver compile/assembly proof

| Test | Result |
|---|---|
| `driver_compiles_vertical_seed_links_to_input_list_plan` | PASS |
| `driver_compiled_vertical_seed_plan_has_two_targets` | PASS |
| `driver_compiled_vertical_seed_plan_target_0_gathers_1` | PASS |
| `driver_compiled_vertical_seed_plan_target_1_gathers_0` | PASS |
| `driver_compiled_plan_uses_accumulator_op_sum_input_list` | PASS |
| `driver_compile_rejects_invalid_scenario_links` | PASS |
| `driver_compile_does_not_use_studio_or_bevy_state` | PASS |

## Sim tick/boundary ownership proof

| Test | Result |
|---|---|
| `sim_tick_executes_driver_compiled_vertical_seed_accumulator_plan` | PASS |
| `sim_tick_vertical_seed_outputs_20_10` | PASS |
| `sim_tick_owns_execution_boundary_not_studio` | PASS |
| `sim_tick_does_not_use_structural_link_accumulator` | PASS |

## Studio bypass guard

| Test | Result |
|---|---|
| `no_studio_runtime_loop_uses_structural_link_accumulator` | PASS |
| `studio_proof_helpers_do_not_run_as_runtime` | PASS |
| `studio_load_path_does_not_execute_accumulator_runtime` | PASS |
| `studio_remains_projection_and_proof_harness` | PASS |

## structural_link_accumulator retirement status

**Deleted** (preferred outcome):

- `crates/simthing-gpu/src/structural_link_accumulator.rs`
- `crates/simthing-gpu/src/shaders/structural_link_accumulator.wgsl`
- Mapeditor proof helpers `prove_gpu_link_accumulator_smoke_blocking`, `prove_runtime_vertical_seed_gpu_link_accumulator_blocking`

Historical PROBATION evidence preserved in `gpu_link_accumulator_smoke_0_results.md` only.

## Documentation-as-code removal status

| Artifact | Status |
|---|---|
| `accumulator_convergence.rs` | Deleted |
| `accumulator_driver_sim_convergence_stub.rs` | Deleted |
| `accumulator_convergence_guards.rs` | Replaced by `accumulator_convergence_1_guards.rs` (behavioral) |

## Gu-Yang / PALMA contract carried forward

Deferred per STEAD §10 — not implemented in this PR:

- **Gu-Yang falloff borders:** grid N4 adjacency → `saturating_flux_choke_threshold` + `structured_field_stencil` (bounded theater).
- **PALMA reach field:** grid N4 adjacency → `min_plus_stencil` + `w_impedance_compose` (bounded theater).

## Big-endian / portable byte-proof backlog

Deferred:

- Replace host-endian `bytemuck` byte evidence with explicit little-endian conversions (`i32::to_le_bytes`, `f32::to_bits().to_le_bytes`).
- Cross-platform byte-order evidence later.

## Forbidden-token scan

`accumulator_op.wgsl` and new Rust identifiers scanned — no forbidden semantic tokens (`route`, `predecessor`, `pathfinding`, `fleet`, `faction`, `border`, `pirate`, etc.).

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-gpu
cargo test -p simthing-gpu
cargo check -p simthing-driver
cargo test -p simthing-driver
cargo check -p simthing-sim
cargo test -p simthing-sim
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-core
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
git diff --check
```

## Windows / resource-limit notes

No paging-file or linker failures observed in this validation pass. GPU AO vertical-seed test observed on real adapter.

## Files changed (summary)

**Added:** `compiled_accumulator_plan.rs`, `structural_link_accumulator_compile.rs`, `accumulator_plan_tick.rs`, convergence test files, `accumulator_convergence_1_guards.rs`, this results doc.

**Modified:** `accumulator_op/cpu_oracle.rs`, `accumulator_op.wgsl`, `simthing-gpu/lib.rs`, driver/sim/mapeditor wiring, production doc, evidence index.

**Deleted:** `structural_link_accumulator.{rs,wgsl}`, `accumulator_convergence.rs`, documentation-as-code stubs/guards.

## Deferred work

- Gu-Yang / PALMA structural execution surfaces (STEAD §10 surfaces 2–3).
- Big-endian portable byte proofs.
- Full runtime terran-pirate play-out.
- RF/MF full execution, heatmap rendering, pathfinding, route/predecessor semantics.

## DA status

**PROBATION** — behavioral convergence evidence complete for RF/link surface 1; real-adapter GPU AO proof observed; awaiting owner sign-off. No new GPU primitive introduced.
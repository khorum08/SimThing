# GPU test inventory — ran on Intel iGPU (re-run required on discrete NVIDIA)

**Date:** 2026-06-03 · **Why:** until this date `GpuContext::new_blocking()` used
`PowerPreference::default()` (→ integrated), so **every GPU-dependent test ran on the Intel iGPU
(RaptorLake-S), never the discrete RTX 4080.** `GpuContext` is now fixed to **always select a discrete
GPU when present** (`crates/simthing-gpu/src/context.rs`).

## How to re-run (simplest path)

The fix is global: any test going through `GpuContext::new_blocking()` now picks the discrete adapter
automatically. So on the machine with the RTX 4080:

```
cargo test --workspace                       # GPU tests now route to the discrete adapter
SIMTHING_RUN_GPU_TESTS=1 cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu -- --nocapture
SIMTHING_RUN_GPU_TESTS=1 cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu -- --nocapture
```

**Verification gate:** the raw logs / parity reports must now report the adapter as the **NVIDIA RTX
4080**, not `Intel(R) RaptorLake-S`. A re-run that still names Intel did not pick up the fix.

## Priority — cross-adapter-sensitive (f32 `GpuVerified` tolerance; adapter rounding/FMA differs)

These are where a different adapter could shift results; re-prove tolerances on NVIDIA:
- `simthing-gpu` — `structured_field_stencil`
- `simthing-driver` — `dress_rehearsal_atlas_batch_0_pack_gpu` (EC-A2b GpuVerified), `structured_field_region_execution`,
  `structured_field_stencil_parent_eml`, `phase_m_first_slice_*` (runtime, product_fixture, product_commitment,
  map_residency, queue_write_hardening, scenario_spec, summary_validity), `phase_m_m5b/m5c/m5e_*` gradients,
  `phase_m_c0_m4_atlas_protocol_oracle`
- `simthing-sim` — f32 parity: `c1_threshold_scan_parity`, `c2_intent_accumulator_parity`,
  `c3_overlay_add_accumulator_parity`, `c4_overlay_orderband_parity`, `c5_weighted_mean_reduction_parity`,
  `c7_velocity_accumulator_parity`, `c8b_intensity_eml_parity`

## Robust (integer / `ExactDeterministic`; very likely adapter-independent — re-run to confirm, low risk)

- `dress_rehearsal_atlas_batch_0_store_gpu` (EC-A3-gpu bit-exact integer masked sums)
- `c6_exact_reduction_parity`, `c8c_transfer_accumulator_parity`, `c8d_emission_accumulator_parity`
- `phase_m_jit_sqrt_*` (sqrt candidate batteries, exact `mag2`/`mag` — already exhaustive-proof-backed),
  `phase_m_jit_grad0/grad1`, `phase_m_jit_exec0/exec1`, `phase_m_jit_prod0`, `phase_m_jit_evaleml_wgsl_prototype`

## Remaining GPU-touching targets (re-run via `cargo test --workspace`)

- `simthing-driver`: `phase_m_field_policy_obs0/obs2/obs3/obs4`, `phase_m_field_policy_event0/event1/event2`,
  `phase_m_field_policy_act0/act1/act2/act3`, `phase_m_field_policy_pipe0`, `phase_m_eml_gadget_2a_snapshot_copy`,
  `phase_m_eml_gadget_runtime_execution_gate`, `phase_m2_field_scheduler`, `phase_m_boundary_cadence_doctrine`,
  `phase_m_economy_field_policy_product_fixture`, `e11_arena_allocation`, `e11b_nested_fission_gap`,
  `e11b_nested_hierarchy_gpu`, `phase_ao_wgsl0_accumulator_op_performance`,
  `phase_t_b0_d2a_hard_currency_ordering`, `resource_economy_designer_ron_session`, `session_integration`
  (+ `tests/support/*` GPU fixtures consumed by the above: `e11_flat_star`, `e11_nested`, `gpu_exec0_fixture`,
  `mobility_gpu_kernel0/5_fixture`, `daily_economy_session`, `resource_economy_session`, `field_policy_v1_live_pipeline`)
- `simthing-sim`: `b4_world_summary_integrated`, `boundary_integration`, `c1_threshold_perf`, `c2_intent_perf`,
  `c5_legacy_weighted_mean_oracle`, `c8_full_pipeline_integration`, `c8a_eml_infrastructure`,
  `c_inf_legacy_oracle_harness`, `e1_emit_on_threshold_builder`, `e2a_resource_transfer_discrete_builder`,
  `e3_conjunctive_recipe_builder`, `e7_governed_by_planner_generalization`, `pivot_forward_remedial`,
  `s1..s6_*_sunset`
- `simthing-feeder`: `integration`

> **Note (perf tests):** `*_perf` / `phase_ao_wgsl0_*` may report different numbers on the discrete GPU;
> per `invariants.md`, perf claims require timestamp queries — treat any timing shift as diagnostic, the
> correctness asserts are what must still pass.

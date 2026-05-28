# Phase M-first-slice-R3 — GPU-Resident Readiness / Observability Parking — Test Results

**Date:** 2026-05-19  
**Base HEAD:** `b92460d6e6431edcc21c49557d239a8ea381c7d0` (M-first-slice-R2 merge)  
**Final commit SHA:** *(pending commit)*

## Summary

M-first-slice-R3 adds Opus/product readiness observability without expanding mapping runtime behavior. `FirstSliceReadinessReport` surfaces dispatch counts, GPU bridge cost-shape, budget preview, execution flags, and informational hot-path wall time. The GPU-resident hot path invariant (`reduction_stencil_readbacks == 0`) remains locked.

## Pass / fail table

| Test | Area | Result | Key result |
|---|---|---|---|
| R3-A | Readiness report hot path | **PASS** | Full shape: dispatches 1+8=9, bridge 3200 B / 102 writes, budget > 0 |
| R3-B | Debug readback explicit | **PASS** | field/reduction/eml Some; reduction_stencil_readbacks=0 |
| R3-C | Budget readiness | **PASS** | SingleGridNoAtlas 1.0×; over-budget rejects at spec layer |
| R3-D | No feature expansion | **PASS** | No atlas hooks, active mask, source_mask; simthing-sim map-free |
| R1/R2 + 0–10 | Prior suite | **PASS** | All 24 prior tests remain green |
| Bridge helper | accumulator_op_session_gpu_bridge | **PASS** | 2/2 (unchanged) |
| Workspace | Full regression | **PASS** | `cargo check --workspace` + `cargo test --workspace` green |

## Readiness report summary

Hot-path tick exposes `FirstSliceReadinessReport` on `FirstSliceMappingReport.readiness`:

```text
mapping_enabled=true, scheduled=true
source_setup_dispatches=1, propagation_dispatches=8, total_dispatches=9
reduction_executed=true, eml_executed=true, reduction_stencil_readbacks=0
field_values_present=false, parent_reduction_present=false, eml_output_present=false
grid_size=10, cell_count=100, n_dims=8, horizon=8
operator=source_capped_normalized
source_policy=caller_managed_one_shot_seed_then_zero
boundary_mode=zero, cadence=EveryTick
budget_estimate_bytes>0
gpu_bridge_bytes_copied=3200, gpu_bridge_slot_col_writes=102
hot_path_wall_ms_observed=informational only
```

## Hot-path cost-shape summary

| Signal | Value (10×10 first slice) |
|---|---|
| source_setup_dispatches | 1 (with seeds) |
| propagation_dispatches | 8 |
| total_dispatches | 9 |
| gpu_bridge_bytes_copied | 3200 (= 100 × 8 × 4) |
| gpu_bridge_slot_col_writes | 102 (= 100 resource + 2 parent weights) |
| budget_estimate_bytes | designer preview from session open |

## Posture summary

No atlas batching. No M-4A atlas masking. No active mask, perception, map residency, behavioral source policy, or source_mask. No semantic WGSL. `MappingExecutionProfile::default()` remains Disabled. simthing-sim remains map-free. Resource Flow / E-11B regression suite green.

## Known caveat

First-slice bridge uses queue writes for child resource values and parent weights. This is acceptable for the 10×10 first slice. Future multi-field/atlas scale should replace per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU fill kernel after a separate measured design step.

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/src/first_slice_mapping_runtime.rs` | `FirstSliceReadinessReport`, bridge counters, budget at open, wall-ms observability |
| `crates/simthing-driver/src/lib.rs` | Export `FirstSliceReadinessReport` |
| `crates/simthing-driver/tests/phase_m_first_slice_runtime.rs` | R3 tests A–D; posture check refinement |
| Docs (production plan, todo, worklog, workshop state, mapping guidance) | M-first-slice-R3 parking status |

## Commands run

```text
git status --short
git rev-parse HEAD
rustc --version
cargo --version

cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture
cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture
cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture

cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture
cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture
cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture
cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture

cargo check --workspace
cargo test --workspace
```

Toolchain: rustc 1.95.0, cargo 1.95.0.

## Final verdict

**PASS** — M-first-slice-R3 readiness pass landed; first-slice hot path remains GPU-resident through stencil, reduction, and EML, exposes Opus/product review counters, and no atlas or semantic mapping expansion landed.

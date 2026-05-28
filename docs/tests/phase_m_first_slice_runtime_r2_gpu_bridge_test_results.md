# Phase M-first-slice-R2 — GPU-Resident Layer 1→2→3 Bridge — Test Results

**Date:** 2026-05-19  
**Base HEAD:** `44020578c02f2b213dd10f5ec362f0cece84a75b` (M-first-slice-R1 merge)  
**Final commit SHA:** *(pending commit)*

## Summary

M-first-slice-R2 removes the hidden GPU→CPU→GPU staging before Layer 2/3. The hot path now copies the canonical stencil field directly into `AccumulatorOpSession` values buffer on GPU, writes resource/weight columns via queue writes, and executes SlotRange Sum + field_urgency EvalEML without reading back the stencil field for reduction preparation.

## Pass / fail table

| Test | Area | Result | Key result |
|---|---|---|---|
| R2-A | No hidden reduction readback | **PASS** | Hot path: report values None, reduction/eml executed, `reduction_stencil_readbacks=0` |
| R2-B | GPU bridge vs debug | **PASS** | Hot + diagnostic readback matches debug field/reduction/EML |
| R2-C | Two-tick GPU bridge | **PASS** | Two hot ticks; field matches CPU oracle; diagnostic reduction/EML finite |
| R2-D | Accumulator bridge helper | **PASS** | copy prefix + slot/col writes + SlotRange Sum in dedicated test |
| R2-E | Bounds validation | **PASS** | Copy/slot/col bounds reject cleanly |
| R2-G | Posture preserved | **PASS** | No atlas/active-mask/source_mask; simthing-sim map-free |
| R1 + 0–10 | Prior suite | **PASS** | All 20 prior tests remain green |
| Workspace | Full regression | **PASS** | `cargo check --workspace` + `cargo test --workspace` green |

## Hot-path no-hidden-readback summary

`run_reduction_and_eml(readback_report=false)` no longer calls `field_values_for_reduction()` or `readback_input_buffer`. It uses `bridge_stencil_field_to_accumulator()` (GPU zero → prefix copy from stencil input → queue writes for resource/weight columns). Report exposes `reduction_stencil_readbacks=0` on hot path.

## GPU bridge summary

Generic `AccumulatorOpSession` helpers: `zero_values_buffer`, `copy_values_prefix_from_buffer`, `write_slot_col_values`, `values_buffer()`. First-slice copies `cell_count * n_dims * sizeof(f32)` bytes from stencil canonical input to accumulator offset 0, then runs existing reduction + EvalEML ops unchanged.

## Two-tick persistence summary

Two consecutive hot-path ticks with different seeds preserve GPU field state; diagnostic readback after tick 2 matches two-tick CPU oracle. Diagnostic reduction/EML after tick 2 produces finite threat/urgency values.

## Posture summary

No atlas batching. No M-4A atlas masking. No active mask, perception, map residency, behavioral source policy, or source_mask. No semantic WGSL. No new EML opcode. `MappingExecutionProfile::default()` remains Disabled. simthing-sim remains map-free. Resource Flow / E-11B regression suite green.

## Files changed

| File | Change |
|---|---|
| `crates/simthing-gpu/src/accumulator_op/session.rs` | Generic GPU values-buffer bridge helpers + bounds errors |
| `crates/simthing-driver/src/first_slice_mapping_runtime.rs` | GPU-resident Layer 1→2→3 bridge; removed CPU reduction staging |
| `crates/simthing-driver/tests/phase_m_first_slice_runtime.rs` | R2 tests A/B/C/G |
| `crates/simthing-gpu/tests/accumulator_op_session_gpu_bridge.rs` | **New** — bridge helper tests D/E |
| Docs (production plan, todo, worklog, workshop state, mapping guidance) | M-first-slice-R2 status |

## Commands run

```text
git status --short
git rev-parse HEAD
rustc --version
cargo --version

cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture
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

**PASS** — M-first-slice-R2 GPU bridge landed; first-slice hot path now preserves GPU-resident field state through stencil, reduction, and EML without hidden CPU readback, while debug readback remains explicit and no atlas or semantic mapping expansion landed.

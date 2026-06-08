# Phase M-first-slice-R1 — GPU-State Ownership + No-Readback Correctness — Test Results

**Date:** 2026-05-19  
**Base HEAD:** `62190353360c7ec6d2ef6d5c66d8a317d59604db` (M-first-slice merge)  
**Final commit SHA:** `565f33e`

## Summary

M-first-slice-R1 remedial hardening fixes caller-managed source protocol on the no-readback hot path. GPU-resident seed writes, seed-only clears, and buffer copies preserve first-hop propagation without CPU readback. Hot-path reports no longer return placeholder parent/EML zeros. Invalid seeds reject cleanly.

## Pass / fail table

| Test | Area | Result | Key result |
|---|---|---|---|
| R1-A | Hot vs debug parity | **PASS** | Hot path + diagnostic readback matches debug field, reduction, EML |
| R1-B | First-hop propagation | **PASS** | Corner seed; neighbors non-zero; matches CPU caller-managed oracle |
| R1-C | Two-tick persistence | **PASS** | Tick 2 starts from tick 1 GPU state; matches two-tick CPU oracle |
| R1-D | Seed-only clear | **PASS** | Full field matches CPU oracle; neighbors preserved (no column-wide zero) |
| R1-E | Hot path report honesty | **PASS** | field/reduction/eml None; reduction_executed/eml_executed true |
| R1-F | Debug readback | **PASS** | field_values, reduction_parent_value, eml_output all Some |
| R1-G | Invalid seed | **PASS** | OOB row/col and non-finite values return errors; no panic |
| R1-H | Dispatch counts | **PASS** | With seeds: setup=1, propagation=H, total=H+1; clean skip: 0 |
| R1-J | Posture preserved | **PASS** | No atlas/active-mask/source_mask; simthing-sim map-free; default Disabled |
| 0–10 | Original first-slice suite | **PASS** | All 11 existing tests remain green |
| GPU helper | structured_field_stencil R1 | **PASS** | copy/write/zero/canonicalize helpers verified |
| Workspace | Full regression | **PASS** | `cargo check --workspace` + `cargo test --workspace` green |

## Hot-path parity summary

Identical seeds/spec: debug-readback tick and hot-path tick + `readback_canonical_field` + `diagnostic_readback_reduction_eml` produce matching field values (≤1e-4), parent reduction (≤0.01), and EML (≤1e-4).

## Two-tick persistence summary

Two consecutive hot-path ticks with different seeds preserve tick-1 GPU field state into tick-2 execution. Diagnostic readback after tick 2 matches the two-tick CPU caller-managed oracle (≤0.0001 per cell).

## Seed-clear protocol summary

Caller-managed protocol on GPU: `write_cell_values` → `dispatch_once` → `zero_cell_values` (seed slots only on output) → `copy_output_to_input` → configured horizon → `canonicalize_input_after_ping_pong`. No full host re-upload when GPU state is canonical. Seed-only clearing does not discard first-hop propagated values.

## Files changed

| File | Change |
|---|---|
| `crates/simthing-gpu/src/structured_field_stencil.rs` | Generic GPU buffer copy/write/zero/readback helpers |
| `crates/simthing-driver/src/first_slice_mapping_runtime.rs` | GPU-resident caller-managed protocol; seed validation; report honesty; dispatch counts |
| `crates/simthing-driver/tests/phase_m_first_slice_runtime.rs` | R1 tests A–H + J; updated queue_seeds/error handling |
| `crates/simthing-gpu/tests/structured_field_stencil.rs` | GPU helper integration test |
| Docs (production plan, todo, worklog, workshop state, mapping guidance) | M-first-slice-R1 status |
| `docs/tests/phase_m_first_slice_runtime_r1_no_readback_correctness_test_results.md` | This report |

## Commands run

```text
git status --short
git rev-parse HEAD
rustc --version
cargo --version

cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
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

**PASS** — M-first-slice-R1 no-readback correctness hardening landed; first-slice hot path preserves GPU-resident field state, seed-only clear does not discard first-hop propagation, hot/debug paths match under diagnostic readback, and no atlas or semantic mapping runtime expansion landed.

# Phase M Product-Facing First-Slice Scenario Fixture — Test Results

**Date:** 2026-05-28
**Base HEAD:** `605eb3fe0cf36ee0eeae4a7411349d93d3b43b83`
**Final commit SHA:** pending at report authoring
**Toolchain:** rustc 1.95.0 (59807616e 2026-04-14), cargo 1.95.0 (f2d3ce0bd 2026-03-21)

## Fixture summary

Phase M product-facing first-slice scenario fixture landed. It drives the accepted
GPU-resident first-slice runtime from a small product-style RegionFieldSpec/RON fixture:
one grid, source_capped_normalized, H<=8, caller-managed seed-only clear, dirty
scheduling, SlotRange Sum reduction, and parent field_urgency EvalEML.

The fixture is intentionally narrow: it is a test/RON fixture over the existing
`FirstSliceMappingSession`, not a scenario engine and not default `SimSession` wiring.

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/fixtures/first_slice_product_suppression_field.ron` | Product-style RegionFieldSpec fixture |
| `crates/simthing-driver/tests/phase_m_first_slice_product_fixture.rs` | Product fixture integration tests |
| `docs/accumulator_op_v2_production_plan.md` | Phase M product fixture landing note |
| `docs/workshop/mapping_current_guidance.md` | Active mapping guidance updated |
| `docs/workshop/workshop_current_state.md` | Current-state and decision-gate status updated |
| `docs/todo.md` | Phase M product fixture status added |
| `docs/worklog.md` | Landing entry added |
| `docs/adr/mapping_sparse_regioncell.md` | Landing note only; no classification change |
| `docs/tests/phase_m_first_slice_product_fixture_test_results.md` | This report |

## Commands run

```text
git status --short
git rev-parse HEAD
rustc --version
cargo --version

cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture

cargo check --workspace
cargo test --workspace
cargo test --workspace -j 1
```

`cargo test --workspace` first failed during parallel compilation with build-artifact
format errors for dependencies such as `ash`, `ron`, and `wgpu_core`. The workspace
test was rerun as `cargo test --workspace -j 1` and passed. No test assertion failed in
the first run before the compile-artifact failure.

## Pass / fail table

| Check | Result | Notes |
|---|---:|---|
| `phase_m_first_slice_product_fixture` | PASS | 7/7 |
| `phase_m_first_slice_runtime` | PASS | 28/28 |
| `region_field_spec_admission` | PASS | 10/10 |
| `accumulator_op_session_gpu_bridge` | PASS | 2/2 |
| `cargo check --workspace` | PASS | Existing warnings only |
| `cargo test --workspace -j 1` | PASS | Full workspace pass after serial rerun |

## Product signal summary

The product fixture queues a source into one 10x10 local theater grid. The field
propagates locally, reduces into the parent threat column, and the parent field_urgency
EvalEML interprets that pressure.

The same/equivalent field state is evaluated with two parent weight profiles:

```text
low urgency profile:  weights=(0.2, 0.1)
high urgency profile: weights=(0.9, 0.1)
```

Both threat and urgency are finite, parent threat is greater than zero, and
`high_urgency > low_urgency`.

## GPU-resident hot-path summary

The explicit `MappingExecutionProfile::SparseRegionFieldV1` hot path reports:

```text
source_setup_dispatches = 1
propagation_dispatches = 8
total_dispatches = 9
reduction_executed = true
eml_executed = true
reduction_stencil_readbacks = 0
field_values = None
reduction_parent_value = None
eml_output = None
```

Debug readback is used only after/for tests to inspect product signal and field sanity.

## Field sanity summary

The fixture verifies finite field values, CPU-oracle tolerance for the caller-managed
seed-only clear protocol, nonzero neighbor propagation from a corner seed, no NaN, and
no wraparound to the opposite corner.

## Posture summary

MappingExecutionProfile::default() remains Disabled. Spec presence alone does not execute
mapping. `request_atlas_batching` remains rejected at admission. PipelineFlags default
does not enable Resource Flow. simthing-sim remains map-free by source scan.

No atlas batching landed. No M-4A atlas masking landed. No active mask, perception, map
residency, behavioral source policy, or source_mask landed. No semantic WGSL landed.
Defaults unchanged.

## Known caveat

First-slice bridge uses queue writes for child resource values and parent weights. This is
acceptable for the 10x10 first-slice fixture. Future multi-field/atlas scale must replace
per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU
fill kernel after a separate measured design step.

## Final verdict

PASS — Phase M product-facing first-slice scenario fixture landed; the accepted
GPU-resident first-slice runtime is now exercised from a product-style RegionFieldSpec/RON
fixture with default-off behavior, explicit opt-in, finite field propagation, parent
reduction, field_urgency EvalEML, and no atlas or semantic mapping expansion.

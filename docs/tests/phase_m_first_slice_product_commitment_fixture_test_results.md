# Phase M First-Slice Product Commitment Fixture Test Results

Date: 2026-05-28

## Base

- Base HEAD: `74b6e9ed347711d97201930040fa3948d6dd6119`
- Final commit SHA: not known at report authoring; branch commit/merge records the final SHA.

## Files Changed

- `crates/simthing-driver/src/first_slice_mapping_runtime.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/phase_m_first_slice_product_commitment_fixture.rs`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/adr/mapping_sparse_regioncell.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/tests/phase_m_first_slice_product_commitment_fixture_test_results.md`

## Fixture Summary

The fixture reuses `first_slice_product_suppression_field.ron` and the landed
`FirstSliceMappingSession`. It drives one local 10x10 RegionField through explicit
`MappingExecutionProfile::SparseRegionFieldV1`, caller-managed one-shot seed then zero,
dirty scheduling, GPU field propagation, SlotRange Sum parent reduction, and parent
`field_urgency` EvalEML. A narrow opt-in fixture helper then registers one existing
AccumulatorOp threshold/EmitEvent scan over the parent urgency column.

The CPU only reads back the emitted event and diagnostic values for assertions. The
commitment decision is the GPU threshold crossing.

## Commands Run

| Command | Result |
|---|---|
| `git status --short` | PASS; pre-existing workshop report dirt and untracked local artifacts noted |
| `git rev-parse HEAD` | PASS; `74b6e9ed347711d97201930040fa3948d6dd6119` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 5/5 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 10/10 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 2/2 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS |

Existing warnings remain in unrelated crates/tests; no new failure or stop condition was hit.

## Product Signal Summary

Measured from the commitment fixture:

| Metric | Value |
|---|---:|
| `low_threat` | `9965.211` |
| `low_urgency` | `2003.0422` |
| `high_threat` | `9965.211` |
| `high_urgency` | `8978.689` |
| `threshold` | `5490.8657` |
| `event_count_low` | `0` |
| `event_count_high` | `1` |
| `dispatches` | `9` |
| `reduction_stencil_readbacks` | `0` |

Result: `low_urgency < threshold < high_urgency`.

## Event Emission Summary

- Default `MappingExecutionProfile::Disabled`: no mapping execution and no threshold event.
- Low-weight profile `(0.2, 0.1)`: urgency remains below threshold and emits no event.
- High-weight profile `(0.9, 0.1)`: urgency crosses threshold and emits exactly one event.
- Event kind: `0x53454144`.
- Event slot: parent slot `100`.
- Event column: parent urgency column `4`.
- Deterministic replay: high-profile event count, event payload, dispatch counts, and urgency tolerance passed across two equivalent runs.

## GPU-Resident Hot Path Summary

The production-shaped hot path returns no field values, no parent reduction value, and no
EML output in the tick report. Diagnostic readback is only used after the hot tick to
assert the product signal. `reduction_stencil_readbacks == 0`.

## Posture Summary

Phase M product commitment fixture landed.
It extends the product-facing first-slice fixture by using the existing threshold/event substrate over parent field_urgency, proving the FIELD_POLICY commitment path: GPU-resident field propagation -> parent reduction -> EvalEML urgency -> threshold event.
Low-weight profile stays below threshold; high-weight profile crosses and emits the expected event.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

## Known Caveat

First-slice bridge uses queue writes for child resource values and parent weights. This is acceptable for the 10x10 first-slice and commitment fixtures. Future multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU fill kernel after a separate measured design step.

## Final Verdict

PASS — Phase M product commitment fixture landed; the accepted GPU-resident first-slice runtime now drives a threshold/event commitment from parent field_urgency with low-profile no-event and high-profile event behavior, without introducing atlas, semantic WGSL, source_mask, perception, map residency, or CPU-side AI planning.

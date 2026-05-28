# Phase M First-Slice CommitmentSpec Test Results

Date: 2026-05-28

## Base

- Base HEAD: `194a102ec303b3a25afb31a1279438b8d5555743`
- Final commit SHA: not known at report authoring; branch commit/merge records the final SHA.

## Files Changed

- `crates/simthing-spec/src/spec/region_field.rs`
- `crates/simthing-spec/src/compile/region_field_admission.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/tests/region_field_spec_admission.rs`
- `crates/simthing-driver/src/first_slice_mapping_runtime.rs`
- `crates/simthing-driver/tests/fixtures/first_slice_product_commitment_field.ron`
- `crates/simthing-driver/tests/phase_m_first_slice_product_commitment_fixture.rs`
- `crates/simthing-driver/tests/phase_m_first_slice_runtime.rs`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/adr/mapping_sparse_regioncell.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/tests/phase_m_first_slice_commitment_spec_test_results.md`

## Fixture/Spec Summary

The first-slice commitment threshold/event binding is now authored in
`first_slice_product_commitment_field.ron` through `FirstSliceCommitmentSpec`:

- `source_formula_class = "field_urgency"`
- `parent_slot = 100`
- `urgency_col = 4`
- `threshold = 5490.8657`
- `direction = Upward`
- `event_kind = 0x53454144`

`compile_region_field_preview` admits the binding only after validating finite threshold,
nonzero event kind, `field_urgency` parent formula, matching reduction parent slot, current
first-slice urgency column, and the currently-supported upward direction.

## Commands Run

| Command | Result |
|---|---|
| `git status --short` | PASS; pre-existing workshop report dirt and untracked local artifacts noted |
| `git rev-parse HEAD` | PASS; `194a102ec303b3a25afb31a1279438b8d5555743` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 2/2 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS |

Existing warnings remain in unrelated crates/tests; no new failure or stop condition was hit.

## Authored Threshold And Event Kind

| Field | Value |
|---|---:|
| `threshold` | `5490.8657` |
| `event_kind` | `0x53454144` |
| `parent_slot` | `100` |
| `urgency_col` | `4` |

## Measured Product Signal

Measured from the authored CommitmentSpec fixture:

| Metric | Value |
|---|---:|
| `low_threat` | `9965.211` |
| `low_urgency` | `2003.0422` |
| `high_threat` | `9965.211` |
| `high_urgency` | `8978.689` |
| `event_count_low` | `0` |
| `event_count_high` | `1` |
| `dispatches` | `9` |
| `reduction_stencil_readbacks` | `0` |

Result: `low_urgency < authored_threshold < high_urgency`.

## Event Emission Summary

- Default `MappingExecutionProfile::Disabled`: no mapping execution and no threshold event.
- Low-weight profile `(0.2, 0.1)`: urgency remains below the authored threshold and emits no event.
- High-weight profile `(0.9, 0.1)`: urgency crosses the authored threshold and emits exactly one authored event.
- Deterministic replay: high-profile event count, event payload, dispatch counts, and urgency tolerance passed across two equivalent runs.

## GPU-Resident Hot Path Summary

The production-shaped hot path returns no field values, no parent reduction value, and no
EML output in the tick report. Diagnostic readback is only used after the hot tick to
assert the product signal. `reduction_stencil_readbacks == 0`.

## Posture Summary

Phase M CommitmentSpec fixture landed.
It moves the first-slice commitment threshold/event binding into a designer/spec-facing RON-admitted configuration while preserving the existing GPU-resident SEAD path: field propagation -> parent reduction -> field_urgency EvalEML -> Threshold + EmitEvent.
Low-weight profile remains below the authored threshold; high-weight profile crosses and emits the authored event.
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

PASS — Phase M CommitmentSpec fixture landed; the first-slice SEAD commitment threshold/event binding is now designer/spec-authored and admitted from RON, while the commitment decision remains GPU-resident through field_urgency -> Threshold + EmitEvent with no atlas, semantic WGSL, source_mask, perception, map residency, or CPU-side AI planning.

# Phase M FirstSliceScenarioSpec Test Results

Date: 2026-05-28

## Base

- Base HEAD: `ef878b68972b465faedc99cbabaf7ccc05c509f9`
- Final commit SHA: not known at report authoring; branch commit/merge records the final SHA.

## Files Changed

- `crates/simthing-spec/src/spec/first_slice_scenario.rs`
- `crates/simthing-spec/src/compile/first_slice_scenario_admission.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/ron.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-driver/src/first_slice_scenario_fixture.rs`
- `crates/simthing-driver/src/first_slice_mapping_runtime.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/phase_m_first_slice_scenario_spec.rs`
- `crates/simthing-driver/tests/fixtures/first_slice_product_commitment_scenario.ron`
- `crates/simthing-driver/tests/fixtures/first_slice_product_commitment_scenario_disabled.ron`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/adr/mapping_sparse_regioncell.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/tests/phase_m_first_slice_scenario_spec_test_results.md`

## Scenario/Spec Summary

`FirstSliceScenarioSpec` is a narrow scenario-level RON wrapper:

```text
FirstSliceScenarioSpec
  -> mapping_execution_profile (Disabled | SparseRegionFieldV1)
  -> region_field: RegionFieldSpec (with optional FirstSliceCommitmentSpec)
  -> compile_first_slice_scenario_preview
  -> CompiledFirstSliceScenarioPreview
  -> FirstSliceScenarioFixtureSession::open
  -> GPU-resident first-slice mapping + Threshold + EmitEvent commitment
```

Fixtures:

- `first_slice_product_commitment_scenario.ron` — authored `SparseRegionFieldV1`
- `first_slice_product_commitment_scenario_disabled.ron` — identical region field with `Disabled` profile

Commitment binding is taken only from the admitted scenario preview (no orphan external threshold).

## Commands Run

| Command | Result |
|---|---|
| `git status --short` | PASS; pre-existing workshop report dirt and untracked local artifacts noted |
| `git rev-parse HEAD` | PASS; `ef878b68972b465faedc99cbabaf7ccc05c509f9` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture` | PASS; 8/8 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS |

Existing warnings remain in unrelated crates/tests; no new failure or stop condition was hit.

## Authored Profile Summary

| Scenario fixture | `mapping_execution_profile` | Executes mapping |
|---|---|---|
| `first_slice_product_commitment_scenario.ron` | `SparseRegionFieldV1` | yes |
| `first_slice_product_commitment_scenario_disabled.ron` | `Disabled` | no (structure admits) |

`MappingExecutionProfile::default()` remains `Disabled`.

## Authored Threshold and Event Kind

From scenario-owned `RegionFieldSpec.commitment`:

| Field | Value |
|---|---|
| `threshold` | `5490.8657` |
| `event_kind` | `0x53454144` (`FIELD_POLICY`) |
| `parent_slot` | `100` |
| `urgency_col` | `4` |
| `source_formula_class` | `field_urgency` |

## Measured Low/High Urgency

| Metric | Value |
|---|---:|
| `low_threat` | `9965.211` |
| `low_urgency` | `2003.0422` |
| `high_threat` | `9965.211` |
| `high_urgency` | `8978.689` |
| `threshold` | `5490.8657` |
| `event_count_low` | `0` |
| `event_count_high` | `1` |

Result: `low_urgency < threshold < high_urgency`.

## Event Emission Summary

- Disabled scenario: admits, opens session, queues seed, performs no mapping (`scheduled == false`, `dispatches == 0`), emits no event.
- Low-weight profile `(0.2, 0.1)`: urgency below authored threshold, `event_count == 0`.
- High-weight profile `(0.9, 0.1)`: urgency crosses authored threshold, `event_count == 1`.
- Event kind: `0x53454144`; slot `100`; col `4`; value ≈ high urgency.
- Deterministic replay: same event count, payload, urgency tolerance, dispatch counts, and `reduction_stencil_readbacks == 0`.

## GPU-Resident Hot-Path Summary

SparseRegionFieldV1 authored scenario:

| Metric | Value |
|---|---:|
| `scheduled` | `true` |
| `source_setup_dispatches` | `1` |
| `propagation_dispatches` | `8` |
| `total_dispatches` | `9` |
| `reduction_executed` | `true` |
| `eml_executed` | `true` |
| `reduction_stencil_readbacks` | `0` |
| `field_values` | `None` |
| `reduction_parent_value` | `None` |
| `eml_output` | `None` |

Diagnostic readback used only for test reporting/verification.

## Posture Summary

Phase M FirstSliceScenarioSpec fixture landed.
It wraps the accepted first-slice RegionFieldSpec + CommitmentSpec in a scenario-level RON authoring shape that includes explicit MappingExecutionProfile.
Disabled scenarios admit as structure but do not execute. SparseRegionFieldV1 scenarios execute the GPU-resident first-slice path and emit the authored commitment event only when field_urgency crosses the authored threshold.
No CPU-side AI planner was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

Invalid specs reject: missing `region_field`, atlas request, invalid commitment threshold, zero event kind, wrong parent slot, wrong urgency col, wrong formula class, over-budget region field.

## Known Caveat

First-slice bridge uses queue writes for child resource values and parent weights. This is acceptable for the 10x10 first-slice scenario fixture. Future multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU fill kernel after a separate measured design step.

## Final Verdict

PASS — Phase M FirstSliceScenarioSpec fixture landed; the first-slice mapping + CommitmentSpec path is now driven from a scenario-level RON authoring shape with explicit MappingExecutionProfile, preserving default-off behavior and GPU-resident FIELD_POLICY commitment execution without atlas, semantic WGSL, source_mask, perception, map residency, default SimSession wiring, or CPU-side AI planning.

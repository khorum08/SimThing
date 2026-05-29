# Phase M M-5B-gradient — L3 Strategic Pressure Composition RON Fixture — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `91f5b0df9375360d295a5d3cfd8fef7197dd0d79` (M-5A merge SHA record)  
**Final commit SHA:** `af940c1` (M-5B-gradient implementation commit)  
**Lane classification:** Tier-1 fast lane (V7.7 §5 / `phase_m_gating_and_doc_policy.md`)  
**Verdict:** **PASS — M-5B-gradient landed**

---

## Pre-Edit Evaluation Summary

Inspected design docs and landed M-5A substrate before editing:

| Surface | Pre-edit shape |
|---|---|
| M-5A gradient | Single-target `Gradient { axis, output_col }`; per-direction stencil weights; CPU/GPU parity |
| L2 reduction | Existing `SlotRange Sum` via `ColumnAwareReductionCombine::Sum` |
| L3 composition | Existing EML gadget stack: `Ema`, `WeightedAccumulator` via `compile_eml_gadget_stack` |
| Commitment | Existing first-slice `Threshold + EmitEvent` via `FirstSliceMappingSession` |
| `MappingExecutionProfile` | Default `Disabled`; fixture opts into `SparseRegionFieldV1` |

Confirmed: L3 composition fixture could land as RON + driver test only — no new substrate, semantic WGSL, or simthing-sim changes required.

---

## Fixture Design Summary

Reference opt-in pattern (RON/test only):

| Layer | Fixture | Pattern |
|---|---|---|
| L1 scalar | `m5b_scalar_pressure_field.ron` | `SourceCappedNormalized` → `target_col: 0` |
| L1 gradient X | `m5b_gradient_x_field.ron` | `Gradient { axis: X, output_col: 1 }`, `horizon: 1` |
| L1 gradient Y | `m5b_gradient_y_field.ron` | `Gradient { axis: Y, output_col: 2 }`, `horizon: 1` |
| L2 | All three fields | `SlotRange Sum` reduction → parent cols 3/4/5 at slot 100 |
| L3 | `m5b_l3_composition_gadget_stack.ron` | 3× `Ema` (cols 3/4/5 → 13/14/15) + `WeightedAccumulator` → `composite_signal` col 6 |
| Commitment | `m5b_reference_scenario.ron` | Scalar field + `field_urgency` + GPU-resident threshold/event (calibrated to product fixture) |

No dual-output `GradientXY`. No new WGSL. No new EML opcode. No L1 cross-field coupling.

---

## Files Changed

- `crates/simthing-driver/tests/fixtures/m5b_scalar_pressure_field.ron`
- `crates/simthing-driver/tests/fixtures/m5b_gradient_x_field.ron`
- `crates/simthing-driver/tests/fixtures/m5b_gradient_y_field.ron`
- `crates/simthing-driver/tests/fixtures/m5b_l3_composition_gadget_stack.ron`
- `crates/simthing-driver/tests/fixtures/m5b_reference_scenario.ron`
- `crates/simthing-driver/tests/phase_m_m5b_gradient_l3_composition_fixture.rs`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`
- `docs/tests/phase_m_m5b_gradient_l3_composition_test_results.md` — this report

---

## Required Scans

**Scan 1 (dual-output / GradientXY):**
```bash
rg "GradientXY|dual-output|output_col_x|output_col_y" crates docs
```
**Result:** Matches in **docs only** as deferred/forbidden future optimization context. Test assertions verify fixtures contain no `GradientXY` / `output_col_x` / `output_col_y`. **No implementation path introduces active dual-output GradientXY.**

**Scan 2 (semantic WGSL):**
```bash
rg "gradient|threat|faction|AI|scarcity|migration|opportunity|supply|demand" crates/simthing-gpu/src/shaders
```
**Result:** **No semantic matches.** Only pre-existing generic tokens (`available`, `migration` comment in accumulator_op.wgsl unrelated to mapping semantics, `main` in snapshot.wgsl).

**Scan 3 (guardrails):**
```bash
rg "source_mask|source identity|atlas|M-4A|sqrt|L1 cross-field|production economy→mapping bridge|default SimSession mapping|CPU urgency|CPU-side AI planner" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_m5b_gradient_l3_composition_test_results.md
```
**Result:** **Guardrail-only** — matches state deferred, unauthorized, or not added; no new prohibited work introduced.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files found; **no scratch/tmp artifacts deleted.**

---

## Tests Run and Results

```bash
cargo test -p simthing-driver --test phase_m_m5b_gradient_l3_composition_fixture -- --nocapture
```
**Result:** **7 passed; 0 failed**

| Test | Result |
|---|---|
| `m5b_field_rons_admit_with_single_target_gradients` | PASS |
| `m5b_l3_gadget_stack_admits_with_ema_and_weighted_accumulator` | PASS |
| `m5b_l3_composition_oracle_is_deterministic_and_finite` | PASS |
| `m5b_reference_scenario_admits_and_default_profile_disabled` | PASS |
| `m5b_gradient_fields_gpu_parity_single_target` | PASS |
| `m5b_reference_scenario_gpu_commitment_path_no_cpu_emission` | PASS |
| `m5b_posture_no_new_substrate` | PASS |

**M-5A regression:**
```bash
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
```
**Result:** **18 passed; 0 failed**

```bash
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
```
**Result:** **25 passed; 0 failed**

```bash
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
```
**Result:** **28 passed; 0 failed**

```bash
cargo test -p simthing-driver --test phase_m_economy_sead_product_fixture -- --nocapture
```
**Result:** **6 passed; 0 failed**

```bash
cargo check --workspace
```
**Result:** **PASS** (pre-existing unused-import warning in `simthing-driver` only)

---

## Posture Attestation

No semantic WGSL, no default mapping wiring, no simthing-sim changes, no source-mask/source-identity work, no atlas/M-4A, no L1 coupling, no sqrt/new opcode, no production economy→mapping bridge; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-5B-gradient landed as a Tier-1 fast-lane L3 Strategic Pressure Composition fixture over the existing M-5A single-target Gradient substrate, using existing reduction and EvalEML/EML gadget composition with EMA smoothing, with one test report and compact status update only, no new substrate, no semantic WGSL, no dual-output GradientXY, no sqrt/new opcode, no L1 coupling, no source-mask/source-identity, no atlas/M-4A, no default mapping wiring, no simthing-sim changes, no production economy→mapping bridge, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

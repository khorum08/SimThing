# Phase M M-5D-gradient — Frame/Scenario-Level Gradient Sink Admission — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `bdbd80bc68064fa4f85c017340ebaeae3d2cf0f1`  
**Final commit SHA:** `4805e6d` (M-5D-gradient implementation commit)  
**Lane classification:** Tier-1 remedial admission-hardening pass (V7.7 §5)  
**Verdict:** **PASS — M-5D-gradient landed**

---

## Pre-Edit Evaluation Summary

| Surface | Pre-edit state |
|---|---|
| Single-spec admission | `Gradient` rejects `output_col == source_col` (same-pass read/write loop) |
| Frame/scenario admission | No cross-field gradient strict-sink enforcement |
| M-5B/M-5C fixtures | Valid by construction (scalar source col 0; gradients sink to cols 1/2; reduction/EML downstream only) |
| Constitutional rule | Codified in design note §3, `invariants.md`, production plan — enforcement gap only |

Confirmed: M-5D implementable as spec/admission/test hardening only; no runtime or substrate changes required.

---

## Implementation Summary

Added `validate_region_field_frame_gradient_sinks(fields: &[&RegionFieldSpec])` in `region_field_admission.rs`:

1. Collects gradient `output_col` values from all fields in a same-frame group.
2. Re-affirms per-field self-loop ban (`source_col == output_col` on gradient fields).
3. Rejects any field whose `source_col` matches any collected gradient sink column.
4. Error names both gradient producer and consumer field.
5. Does not inspect reduction child_col, parent_col, EML inputs, or threshold paths — only diffusion/stencil `source_col`.
6. Cross-tick coupling out of scope: validator checks one frame group at a time.

---

## Files Changed

- `crates/simthing-spec/src/compile/region_field_admission.rs` — frame-level validator
- `crates/simthing-spec/src/compile/mod.rs` — export
- `crates/simthing-spec/src/lib.rs` — re-export
- `crates/simthing-spec/tests/region_field_spec_admission.rs` — rejection + positive + cross-tick policy tests
- `crates/simthing-driver/tests/phase_m_m5b_gradient_l3_composition_fixture.rs` — M-5B frame validation test
- `crates/simthing-driver/tests/phase_m_m5c_gradient_need_signal_fixture.rs` — M-5C frame validation test
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`
- `docs/tests/phase_m_m5d_gradient_sink_admission_test_results.md` — this report

---

## Rejection and Positive Tests

| Test | Result |
|---|---|
| `m5d_rejects_normalized_field_using_gradient_output_as_source_col` | PASS |
| `m5d_rejects_gradient_field_using_another_gradient_output_as_source_col` | PASS |
| `m5d_rejects_base_field_using_gradient_output_as_source_col` | PASS |
| `m5d_reaffirms_gradient_self_loop_at_frame_level` | PASS |
| `m5d_admits_m5b_style_valid_gradient_sinks` | PASS |
| `m5d_admits_m5c_fixture_rons_under_frame_validation` | PASS |
| `m5d_validator_checks_same_frame_only_not_cross_tick` | PASS |
| `m5b_frame_gradient_sink_validation_admits` | PASS |
| `m5c_frame_gradient_sink_validation_admits` | PASS |

---

## Required Scans

**Scan 1 (dual-output / GradientXY):**
```bash
rg "GradientXY|dual-output|output_col_x|output_col_y" crates docs
```
**Result:** Docs-only deferred/forbidden context + test assertions. No active dual-output GradientXY implementation.

**Scan 2 (semantic WGSL):**
```bash
rg "gradient|threat|faction|AI|scarcity|migration|opportunity|supply|demand|need|routing|cost|price|labor" crates/simthing-gpu/src/shaders
```
**Result:** No new semantic matches. Pre-existing generic tokens only (`available`, `unit_cost`, unrelated `migration` comment). `structured_field_stencil.wgsl` unchanged.

**Scan 3 (guardrails / strict-sink rule):**
```bash
rg "source_mask|source identity|atlas|M-4A|sqrt|L1 cross-field|production economy→mapping bridge|default SimSession mapping|CPU urgency|CPU-side AI planner|ResourceEconomySpec.*mapping|economy.*mapping|within-frame|strict sink|same-frame" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_m5d_gradient_sink_admission_test_results.md
```
**Result:** Guardrail-only matches plus strict-sink / within-frame / same-frame rule documentation and enforcement description.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files; **no scratch/tmp artifacts deleted.**

---

## Tests Run and Results

```bash
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
```
**Result:** **25 passed; 0 failed**

```bash
cargo test -p simthing-driver --test phase_m_m5b_gradient_l3_composition_fixture -- --nocapture
```
**Result:** **9 passed; 0 failed**

```bash
cargo test -p simthing-driver --test phase_m_m5c_gradient_need_signal_fixture -- --nocapture
```
**Result:** **6 passed; 0 failed**

```bash
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
```
**Result:** **25 passed; 0 failed**

```bash
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
```
**Result:** **28 passed; 0 failed**

```bash
cargo check --workspace
```
**Result:** **PASS**

`phase_m_economy_sead_product_fixture` not run — change does not touch product-fixture commitment support.

---

## Posture Attestation

No semantic WGSL, no default mapping wiring, no simthing-sim changes, no source-mask/source-identity work, no atlas/M-4A, no L1 coupling, no sqrt/new opcode, no production economy→mapping bridge, no ResourceEconomySpec→mapping coupling; cross-field gradient strict-sink admission now rejects same-frame derivative feedback; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-5D-gradient landed as a Tier-1 admission-hardening pass; frame/scenario-level admission now rejects same-frame derivative feedback by treating Gradient output columns as strict sinks for diffusion/stencil source_col wiring, while preserving downstream reduction/EML/threshold consumption and legitimate cross-tick coupling, with M-5B/M-5C valid-sink fixtures green, rejection cases covered, one test report and compact status update only, no new substrate, no semantic WGSL, no dual-output GradientXY, no sqrt/new opcode, no L1 coupling, no source-mask/source-identity, no atlas/M-4A, no default mapping wiring, no simthing-sim changes, no production economy→mapping bridge, no ResourceEconomySpec→mapping coupling, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

# Phase M M-5D R1 — Grouped RegionField Frame Compile Helper — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `1a80ce6e7f15e8e224ad844653252808bc9a4751` (M-5D strict-sink validator merge)  
**Landed PR #278 HEAD:** `8a2177a7c28ed12c98e23dc5eddfdba07c14bb67`  
**Final commit SHA:** `6ed8e0d` (M-5D R1 evidence closure commit)  
**Lane classification:** Tier-1 admission-hardening evidence closure (V7.7 §5)  
**Verdict:** **PASS — M-5D R1 evidence closure landed**

---

## Implementation Summary

PR #278 added `compile_region_field_frame_preview(fields: &[&RegionFieldSpec])` in `simthing-spec`:

1. Calls `validate_region_field_frame_gradient_sinks(fields)` first (strict-sink / same-frame rejection).
2. Compiles each field with `compile_region_field_preview`.
3. M-5B and M-5C grouped fixture tests now use the helper as the safe default for multi-field admission.

No runtime, GPU, WGSL, or `simthing-sim` changes. Future grouped RegionField fixture/scenario authors should prefer `compile_region_field_frame_preview` over ad-hoc per-field compile when admitting same-frame field groups.

---

## Files Changed (PR #278)

- `crates/simthing-spec/src/compile/region_field_admission.rs` — `compile_region_field_frame_preview`
- `crates/simthing-spec/src/compile/mod.rs` — export
- `crates/simthing-spec/src/lib.rs` — re-export
- `crates/simthing-spec/tests/region_field_spec_admission.rs` — `m5d_compile_region_field_frame_preview_admits_valid_group`
- `crates/simthing-driver/tests/phase_m_m5b_gradient_l3_composition_fixture.rs` — grouped helper usage
- `crates/simthing-driver/tests/phase_m_m5c_gradient_need_signal_fixture.rs` — grouped helper usage
- `docs/workshop/m5_gradient_extraction_design_note.md` — M-5D enforcement status sync
- `docs/invariants.md` — M-5D enforcement reference
- `docs/accumulator_op_v2_production_plan.md` — helper mention
- `docs/worklog.md` — R1 entry

This remedial pass adds only the missing test report and compact status references (no code/runtime change beyond PR #278).

---

## Required Scans

**Scan 1 (grouped helper references):**
```bash
rg "phase_m_m5d_r1_gradient_frame_compile_helper_test_results|compile_region_field_frame_preview|validate_region_field_frame_gradient_sinks" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests
```
**Result:** Active docs and this report reference the strict-sink validator and grouped compile helper accurately.

**Scan 2 (dual-output / GradientXY):**
```bash
rg "GradientXY|dual-output|output_col_x|output_col_y" crates docs
```
**Result:** Docs-only deferred/forbidden context + test assertions verifying absence. No active dual-output GradientXY implementation.

**Scan 3 (semantic WGSL):**
```bash
rg "gradient|threat|faction|AI|scarcity|migration|opportunity|supply|demand|need|routing|cost|price|labor" crates/simthing-gpu/src/shaders
```
**Result:** No new semantic matches. Pre-existing generic tokens only (`available`, `unit_cost`, unrelated `migration` comment). `structured_field_stencil.wgsl` unchanged.

**Scan 4 (guardrails / strict-sink rule):**
```bash
rg "source_mask|source identity|atlas|M-4A|sqrt|L1 cross-field|production economy→mapping bridge|default SimSession mapping|CPU urgency|CPU-side AI planner|ResourceEconomySpec.*mapping|economy.*mapping|within-frame|strict sink|same-frame" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_m5d_r1_gradient_frame_compile_helper_test_results.md
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
**Result:** **26 passed; 0 failed**

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

`phase_m_economy_sead_product_fixture` not run — no product-fixture commitment support touched.

---

## Posture Attestation

No semantic WGSL, no default mapping wiring, no simthing-sim changes, no source-mask/source-identity work, no atlas/M-4A, no L1 coupling, no sqrt/new opcode, no production economy→mapping bridge, no ResourceEconomySpec→mapping coupling; grouped RegionField admission bundles gradient strict-sink validation before per-field preview compilation; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-5D R1 evidence closure landed; `compile_region_field_frame_preview` remains the safe grouped RegionField admission helper bundling strict-sink validation before per-field preview compilation, the missing `docs/tests` report was added, active guidance and production plan now reference the grouped-helper evidence, tests and cargo check are green, no runtime/GPU/WGSL/simthing-sim behavior changed, no production economy→mapping bridge was added, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture remains intact.

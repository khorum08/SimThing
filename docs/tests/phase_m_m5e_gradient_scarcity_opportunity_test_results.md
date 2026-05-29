# Phase M M-5E-gradient — Scarcity/Opportunity Composite Product Fixture — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `855a83402f5fbd520634927bf4c61b5cf49f934e`  
**Final commit SHA:** *(pending merge — see PR)*  
**Lane classification:** Tier-1 product fixture (V7.7 §5)  
**Verdict:** **PASS — M-5E-gradient landed**

---

## Pre-Edit Evaluation Summary

M-5E expresses the Product Scenario Selection Gate named scenario as RON/test fixture only:

| Requirement | Existing substrate | M-5E needs new code? |
|---|---|---|
| Single-target Gradient X/Y | M-5A | No |
| SlotRange Sum reductions | M-5B | No |
| EMA + WeightedAccumulator L3 | M-5B/C + EML-GADGET-1 | No |
| Grouped strict-sink admission | M-5D + R1 `compile_region_field_frame_preview` | No |
| Full-grid execution | First-slice default | No |
| Active mask / atlas / source-mask | Deferred gates | Not needed |

M-5E extends M-5C with a fourth field (logistics/supply-reach Gradient X) and a 4-input L3 composite producing `scarcity_opportunity_logistics_pressure`.

---

## Fixture Design Summary

| Layer | Fixture | Pattern |
|---|---|---|
| L1 scalar | `m5e_scarcity_opportunity_scalar_field.ron` | `scarcity_unmet_demand_field` — `SourceCappedNormalized` |
| L1 gradient X (price) | `m5e_scarcity_opportunity_price_gradient_x_field.ron` | `price_differential_gradient_x` — `Gradient { axis: X }` |
| L1 gradient Y (labor) | `m5e_scarcity_opportunity_labor_gradient_y_field.ron` | `labor_opportunity_gradient_y` — `Gradient { axis: Y }` |
| L1 gradient X (logistics) | `m5e_scarcity_opportunity_logistics_gradient_x_field.ron` | `supply_reach_logistics_gradient_x` — `Gradient { axis: X }` |
| L2 | All four fields | `SlotRange Sum` → parent cols 3/4/5/6 |
| L3 | `m5e_scarcity_opportunity_l3_stack.ron` | 4× `Ema` + `WeightedAccumulator` → `scarcity_opportunity_logistics_pressure` col 17 |

Integrated CPU-oracle tests tie L1 → L2 → L3. Optional GPU parity on Gradient X/Y fields. Monotonic scarcity response test included. No production multi-field runtime wiring.

Designer/spec-layer names (scarcity, opportunity, labor, price, logistics, supply reach) appear in RON/test only; `structured_field_stencil.wgsl` unchanged.

---

## Files Changed

- `crates/simthing-driver/tests/fixtures/m5e_scarcity_opportunity_scalar_field.ron`
- `crates/simthing-driver/tests/fixtures/m5e_scarcity_opportunity_price_gradient_x_field.ron`
- `crates/simthing-driver/tests/fixtures/m5e_scarcity_opportunity_labor_gradient_y_field.ron`
- `crates/simthing-driver/tests/fixtures/m5e_scarcity_opportunity_logistics_gradient_x_field.ron`
- `crates/simthing-driver/tests/fixtures/m5e_scarcity_opportunity_l3_stack.ron`
- `crates/simthing-driver/tests/phase_m_m5e_gradient_scarcity_opportunity_fixture.rs`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`
- `docs/tests/phase_m_m5e_gradient_scarcity_opportunity_test_results.md` — this report

---

## Required Scans

**Scan 1 (dual-output / GradientXY):**
```bash
rg "GradientXY|dual-output|output_col_x|output_col_y" crates docs
```
**Result:** Matches in **docs only** as deferred/forbidden context and in **test assertions** verifying absence. No active implementation path introduces dual-output GradientXY.

**Scan 2 (semantic WGSL):**
```bash
rg "scarcity|opportunity|labor|price|logistics|routing|need|demand|supply" crates/simthing-gpu/src/shaders
```
**Result:** **Zero semantic matches.**

**Scan 3 (guardrails):**
```bash
rg "active_mask|ActiveOnlyExperimentalNoHalo|RegionFieldMaskModeSpec|atlas|M-4A|source_mask|source identity|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|ResourceEconomySpec.*mapping|economy.*mapping" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_m5e_gradient_scarcity_opportunity_test_results.md
```
**Result:** **Guardrail-only** — matches state deferred, unauthorized, or not used by M-5E.

**Scan 4 (strict-sink / grouped admission):**
```bash
rg "compile_region_field_frame_preview|validate_region_field_frame_gradient_sinks|strict-sink|same-frame" crates/simthing-driver/tests crates/simthing-spec/tests docs/tests/phase_m_m5e_gradient_scarcity_opportunity_test_results.md
```
**Result:** M-5E test uses `compile_region_field_frame_preview` for 4-field same-frame admission; M-5D validator referenced in spec tests and prior reports.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files; **no scratch/tmp artifacts deleted.**

---

## Tests Run and Results

```bash
cargo test -p simthing-driver --test phase_m_m5e_gradient_scarcity_opportunity_fixture -- --nocapture
```
**Result:** **7 passed; 0 failed**

```bash
cargo test -p simthing-driver --test phase_m_m5c_gradient_need_signal_fixture -- --nocapture
```
**Result:** **6 passed; 0 failed**

```bash
cargo test -p simthing-driver --test phase_m_m5b_gradient_l3_composition_fixture -- --nocapture
```
**Result:** **9 passed; 0 failed**

```bash
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
```
**Result:** **26 passed; 0 failed**

```bash
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
```
**Result:** **25 passed; 0 failed**

```bash
cargo check --workspace
```
**Result:** **PASS**

---

## Posture Attestation

No semantic WGSL, no new WGSL, no atlas/M-4A, no active-mask admission, no source-mask/source-identity work, no default mapping wiring, no simthing-sim changes, no L1 coupling, no sqrt/new opcode, no production economy→mapping bridge, no ResourceEconomySpec→mapping coupling; M-5E uses existing M-5-gradient substrate with grouped strict-sink RegionField admission; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-5E-gradient landed as a Tier-1 full-grid scarcity/opportunity/logistics composite product fixture over existing M-5-gradient substrate.

# Phase M M-5C-gradient — Product-Facing Need/Routing Signal Fixture — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `d29e4eac997a1fb25ce50f75d7ca885f75ff5951` (M-5B R1 merge)  
**Final commit SHA:** `5e903a8` (M-5C-gradient implementation commit)  
**Lane classification:** Tier-1 fast lane (V7.7 §5)  
**Verdict:** **PASS — M-5C-gradient landed**

---

## Fixture Design Summary

Product-facing RON/test-only need/routing signal pattern over existing M-5A/M-5B substrate:

| Layer | Fixture | Pattern |
|---|---|---|
| L1 scalar | `m5c_need_signal_scalar_field.ron` | `unmet_demand_field` — `SourceCappedNormalized` |
| L1 gradient X | `m5c_need_signal_gradient_x_field.ron` | `price_differential_gradient_x` — single-target `Gradient { axis: X }` |
| L1 gradient Y | `m5c_need_signal_gradient_y_field.ron` | `labor_opportunity_gradient_y` — single-target `Gradient { axis: Y }` |
| L2 | All three fields | `SlotRange Sum` → parent cols 3/4/5 |
| L3 | `m5c_need_signal_l3_stack.ron` | 3× `Ema` + `WeightedAccumulator` → `routing_signal` col 16 |

Integrated CPU-oracle test (`m5c_integrated_need_routing_signal_is_finite_and_deterministic`) ties L1 → L2 → L3 in one artifact. Optional GPU parity on Gradient X/Y. No production multi-field runtime wiring, no CPU commitment, no ResourceEconomySpec→mapping coupling.

Designer/spec-layer names (scarcity, opportunity, price differential, labor) appear in RON/test only; `structured_field_stencil.wgsl` unchanged.

---

## Files Changed

- `crates/simthing-driver/tests/fixtures/m5c_need_signal_scalar_field.ron`
- `crates/simthing-driver/tests/fixtures/m5c_need_signal_gradient_x_field.ron`
- `crates/simthing-driver/tests/fixtures/m5c_need_signal_gradient_y_field.ron`
- `crates/simthing-driver/tests/fixtures/m5c_need_signal_l3_stack.ron`
- `crates/simthing-driver/tests/phase_m_m5c_gradient_need_signal_fixture.rs`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`
- `docs/tests/phase_m_m5c_gradient_need_signal_test_results.md` — this report

---

## Required Scans

**Scan 1 (dual-output / GradientXY):**
```bash
rg "GradientXY|dual-output|output_col_x|output_col_y" crates docs
```
**Result:** Matches in **docs only** as deferred/forbidden context and in **test assertions** verifying absence. No active implementation path introduces dual-output GradientXY.

**Scan 2 (semantic WGSL):**
```bash
rg "gradient|threat|faction|AI|scarcity|migration|opportunity|supply|demand|need|routing|cost|price|labor" crates/simthing-gpu/src/shaders
```
**Result:** **No new semantic matches.** Pre-existing generic tokens only (`available`, `unit_cost` in `accumulator_op.wgsl`, unrelated `migration` comment). `structured_field_stencil.wgsl` remains semantic-free.

**Scan 3 (guardrails):**
```bash
rg "source_mask|source identity|atlas|M-4A|sqrt|L1 cross-field|production economy→mapping bridge|default SimSession mapping|CPU urgency|CPU-side AI planner|ResourceEconomySpec.*mapping|economy.*mapping" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_m5c_gradient_need_signal_test_results.md
```
**Result:** **Guardrail-only** — matches state deferred, unauthorized, fixture-only, or not added.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files; **no scratch/tmp artifacts deleted.**

---

## Tests Run and Results

```bash
cargo test -p simthing-driver --test phase_m_m5c_gradient_need_signal_fixture -- --nocapture
```
**Result:** **5 passed; 0 failed**

| Test | Result |
|---|---|
| `m5c_need_signal_fields_admit_with_single_target_gradients` | PASS |
| `m5c_routing_signal_l3_stack_admits_with_ema_and_weighted_accumulator` | PASS |
| `m5c_integrated_need_routing_signal_is_finite_and_deterministic` | PASS |
| `m5c_gradient_fields_gpu_parity_single_target` | PASS |
| `m5c_posture_no_cpu_commitment_or_new_substrate` | PASS |

```bash
cargo test -p simthing-driver --test phase_m_m5b_gradient_l3_composition_fixture -- --nocapture
```
**Result:** **8 passed; 0 failed**

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
cargo check --workspace
```
**Result:** **PASS**

`phase_m_economy_field_policy_product_fixture` not run — change does not touch product-fixture support or commitment wiring.

---

## Posture Attestation

No semantic WGSL, no default mapping wiring, no simthing-sim changes, no source-mask/source-identity work, no atlas/M-4A, no L1 coupling, no sqrt/new opcode, no production economy→mapping bridge, no ResourceEconomySpec→mapping coupling; V7.7 / Mapping ADR / FIELD_POLICY GPU-resident default-off posture intact.

---

**PASS** — Phase M-5C-gradient landed as a Tier-1 fast-lane product-facing need/routing signal fixture over the existing M-5A/M-5B gradient substrate, using single-target Gradient X/Y, existing reductions, and EMA + WeightedAccumulator L3 composition, with one test report and compact status update only, no new substrate, no semantic WGSL, no dual-output GradientXY, no sqrt/new opcode, no L1 coupling, no source-mask/source-identity, no atlas/M-4A, no default mapping wiring, no simthing-sim changes, no production economy→mapping bridge, no ResourceEconomySpec→mapping coupling, and V7.7 / Mapping ADR / FIELD_POLICY GPU-resident default-off posture intact.

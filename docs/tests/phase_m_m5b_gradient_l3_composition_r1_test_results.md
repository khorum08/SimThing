# Phase M M-5B-gradient R1 — Integrated Fixture Evidence — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `a233252ee11209534f272c5a450a2618d3165776` (M-5B merge)  
**Final commit SHA:** `d3286a3` (M-5B-gradient R1 implementation commit)  
**Lane classification:** Tier-1 remedial evidence pass (V7.7 §5)  
**Verdict:** **PASS — M-5B-gradient R1 landed (integrated evidence path)**

---

## Defect Found

M-5B landed safely but proved scalar fields, Gradient X/Y, L3 stack oracle, and scalar commitment path mostly in separate tests; it did not tie scalar + GradientX + GradientY parent reductions into the L3 EMA + WeightedAccumulator stack in one coherent test artifact.

## Chosen Path

**Integrated evidence (preferred path).** Added `m5b_integrated_parent_columns_feed_l3_composite` using CPU oracles for L1 stencils and existing SlotRange Sum reduction semantics, feeding parent cols 3/4/5 into the compiled L3 gadget stack. No new substrate or production multi-field runtime wiring.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_m5b_gradient_l3_composition_fixture.rs` — integrated test + helpers
- `docs/workshop/mapping_current_guidance.md` — R1 evidence note
- `docs/workshop/workshop_current_state.md` — R1 evidence note
- `docs/accumulator_op_v2_production_plan.md` — R1 ladder status
- `docs/worklog.md` — append-only entry
- `docs/tests/phase_m_m5b_gradient_l3_composition_r1_test_results.md` — this report

---

## Required Scans

**Scan 1 (dual-output / GradientXY):**
```bash
rg "GradientXY|dual-output|output_col_x|output_col_y" crates docs
```
**Result:** Matches in **docs only** as deferred/forbidden context and in **test assertions** verifying absence. No active implementation path introduces dual-output GradientXY.

**Scan 2 (semantic WGSL):**
```bash
rg "gradient|threat|faction|AI|scarcity|migration|opportunity|supply|demand" crates/simthing-gpu/src/shaders
```
**Result:** **No new semantic matches.** Pre-existing generic tokens only (`available`, unrelated `migration` comment in `accumulator_op.wgsl`, `main` in `snapshot.wgsl`). `structured_field_stencil.wgsl` remains clean.

**Scan 3 (guardrails):**
```bash
rg "source_mask|source identity|atlas|M-4A|sqrt|L1 cross-field|production economy→mapping bridge|default SimSession mapping|CPU urgency|CPU-side AI planner" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_m5b_gradient_l3_composition_r1_test_results.md
```
**Result:** **Guardrail-only** — matches state deferred, unauthorized, or not added.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files; **no scratch/tmp artifacts deleted.**

---

## Tests Run and Results

```bash
cargo test -p simthing-driver --test phase_m_m5b_gradient_l3_composition_fixture -- --nocapture
```
**Result:** **8 passed; 0 failed** (includes new `m5b_integrated_parent_columns_feed_l3_composite`)

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

`phase_m_economy_sead_product_fixture` not run — R1 change does not touch commitment or product-fixture support.

---

## Posture Attestation

No semantic WGSL, no default mapping wiring, no simthing-sim changes, no source-mask/source-identity work, no atlas/M-4A, no L1 coupling, no sqrt/new opcode, no production economy→mapping bridge; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-5B-gradient R1 landed; the fixture evidence now ties scalar + GradientX + GradientY field outputs through parent reductions into the L3 EMA + WeightedAccumulator composite in one integrated test, with finite deterministic oracle parity, no new substrate, no semantic WGSL, no dual-output GradientXY, no sqrt/new opcode, no L1 coupling, no source-mask/source-identity, no atlas/M-4A, no default mapping wiring, no simthing-sim changes, no production economy→mapping bridge, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

# Phase M M-5A-gradient — Single-Target Gradient Operator + Per-Direction Stencil Weights — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `67835d60d97042b5bd7c567a53ebbc9c1f540572` (post Opus V7.7 gating constitution)  
**Final commit SHA:** *(to be recorded after push/merge)*  
**Verdict:** **PASS — M-5A-gradient landed**

---

## Pre-Edit Evaluation Summary

Inspected design docs and substrate before editing:

| Surface | Pre-edit shape |
|---|---|
| `RegionFieldOperatorSpec` | `Normalized`, `SourceCappedNormalized` only — no `Gradient` |
| Stencil params | Single scalar `gamma_neighbor`; isotropic neighbor average in WGSL/CPU |
| Output contract | One `source_col`, one `target_col`, one write per dispatch |
| `MappingExecutionProfile` | Default `Disabled` |
| Design note | M-5A staged as single-target two-pass; `GradientXY` dual-output deferred |

Confirmed: implementation could add per-direction weights and single-target gradient without dual-output kernel changes, semantic WGSL, or simthing-sim changes.

---

## Design Summary

| Layer | Change |
|---|---|
| WGSL / GPU | `FieldStencilParamsGpu` carries `weight_north/south/east/west`; stencil formula `next = α·center + Σ w·neighbor`; single `target_col` write preserved |
| Legacy operators | Normalized / SourceCappedNormalized map `gamma_neighbor` → equal weights `γ/4` (behavior preserved) |
| Spec | `RegionFieldOperatorSpec::Gradient { axis: X\|Y, output_col }` with admission: valid distinct cols, `target_col == output_col`, `horizon == 1`, no source_cap |
| Compiler | Gradient compiles weights: X → `(0,0,+0.5,-0.5)`, Y → `(-0.5,+0.5,0,0)`, `alpha_self=0` |
| CPU oracle | Same weight formula; boundary mode unchanged |

No dual-output `GradientXY`. No semantic WGSL names. No sqrt / new EML opcode.

---

## Files Changed

- `crates/simthing-spec/src/spec/region_field.rs` — `Gradient`, `GradientAxisSpec`
- `crates/simthing-spec/src/compile/region_field_admission.rs` — admission, compiled weights
- `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `spec/mod.rs` — exports
- `crates/simthing-gpu/src/structured_field_stencil.rs` — config weights, params, CPU oracle
- `crates/simthing-gpu/src/shaders/structured_field_stencil.wgsl` — generic per-direction weights
- `crates/simthing-driver/src/first_slice_mapping_runtime.rs` — compiled→GPU bridge
- `crates/simthing-spec/tests/region_field_spec_admission.rs` — M-5A admission tests
- `crates/simthing-gpu/tests/structured_field_stencil.rs` — M-5A CPU/GPU parity tests
- Driver test configs — zero weight field init (behavior unchanged)
- `docs/workshop/m5_gradient_extraction_design_note.md`
- `docs/workshop/workshop_current_state.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/tests/phase_m_m5a_gradient_single_target_test_results.md` — this report

---

## Required Scans

**Scan 1 (dual-output / GradientXY):**
```bash
rg "GradientXY|dual-output|two output columns|output_col_x|output_col_y" crates docs
```
**Result:** Matches in **docs only** as deferred/forbidden future optimization context. **No implementation/type/test path** introduces active dual-output `GradientXY`.

**Scan 2 (semantic WGSL):**
```bash
rg "gradient|threat|faction|AI|scarcity|migration|opportunity|supply|demand" crates/simthing-gpu/src/shaders/structured_field_stencil.wgsl
```
**Result:** **CLEAN — zero matches.**

**Scan 3 (guardrails):**
```bash
rg "source_mask|source identity|atlas|M-4A|sqrt|L1 cross-field|production economy→mapping bridge|default SimSession mapping|CPU urgency|CPU-side AI planner" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_m5a_gradient_single_target_test_results.md
```
**Result:** **Guardrail-only** — matches state not added, deferred, unauthorized, or stop conditions.

---

## Tests Run + Results

| Command | Result |
|---|---|
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | **18/18 PASS** (7 new M-5A admission tests) |
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **25/25 PASS** (8 new M-5A parity tests) |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | **28/28 PASS** |
| `cargo test -p simthing-driver --test phase_m_economy_sead_product_fixture -- --nocapture` | **6/6 PASS** |
| `cargo check --workspace` | **PASS** |
| `structured_field_stencil_ping_pong` | **N/A** — test file not present in repo; covered by `structured_field_stencil` ping-pong tests |
| GPU bridge (`accumulator_op_session_gpu_bridge`) | **OMITTED** — EvalEML execution assumptions unchanged |

### GPU Parity Highlights

- GradientX 3×3: center `(east−west)/2` CPU=GPU (max err < 1e-4)
- GradientY 3×3: center `(south−north)/2` CPU=GPU (max err < 1e-4)
- Normalized + SourceCappedNormalized: full-buffer CPU=GPU after weight refactor
- Single-target contract: only `target_col` written; other columns passthrough

---

## Transient Log Cleanup

Checked `docs/tests` for `*.log`, `*tmp*`, `*scratch*`.

**Result:** Historical `*_full.log` files preserved. No scratch/tmp files deleted.

---

## Posture Affirmations

- No semantic WGSL was added (shader uses generic `weight_north/south/east/west` only).
- No default SimSession mapping wiring was added.
- No `simthing-sim` semantics changed.
- No source-mask/source-identity work landed.
- No atlas/M-4A landed.
- No production economy→mapping bridge was added.
- Dual-output `GradientXY` not implemented.
- V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

## Final Verdict

**PASS** — Phase M-5A-gradient landed as a single-target generic Gradient axis extension over `StructuredFieldStencilOp`, with per-direction weights, CPU/GPU parity for GradientX and GradientY, preserved Normalized and SourceCappedNormalized behavior, active docs and production plan updated, no dual-output GradientXY, no semantic WGSL, no source-mask/source-identity work, no atlas/M-4A, no L1 field coupling, no sqrt/new opcode, no simthing-sim/default mapping changes, no production economy→mapping bridge, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

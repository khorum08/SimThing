# Phase M EML-GADGET-2D R1 — Hysteresis Compiler Truth + Parity Completion — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `6638f84afe72077a7bf2b074b3557b29ea021af3`  
**Final commit SHA:** `f5c454a`  
**Verdict:** **PASS — exact compiler parity path**

---

## Preflight Cleanup Summary

1. **`docs/tests/phase_m_eml_gadget_2d_hysteresis_test_results.md`**
   - Final SHA placeholder replaced with `6638f84afe72077a7bf2b074b3557b29ea021af3` (2D commit).
   - Compiler section corrected: initial 2D landing was spec/admission/oracle + safe stub emission; exact CMP/SELECT parity deferred to 2D R1.

2. **`docs/workshop/mapping_current_guidance.md`**
   - Removed stale orphaned tail (`stale next-step tail removed`, `Implementation stays unauthorized until 2A`, duplicate ladder text).
   - Replaced **Current EML-GADGET-2 status** with post-2D R1 accurate wording.

---

## Decision

**Path A (preferred): exact CMP/SELECT compilation** — implemented successfully.

`compile_hysteresis_nodes` now emits an exact postfix tree using existing `CMP_GE`, `CMP_LE`, `CMP_EQ`, `MUL`, and `SELECT` opcodes implementing:

```text
SELECT(on_to_off, off_value, SELECT(off_to_on, on_value, previous))
off_to_on = (previous == off_value) && (input >= on_threshold)
on_to_off = (previous == on_value) && (input <= off_threshold)
```

`eval_eml_postfix` extended for spec-layer parity only (matches `simthing-gpu` cpu_oracle CMP/SELECT semantics). Public `compile_eml_gadget` + `oracle_hysteresis` exported from `simthing-spec`.

---

## Design Summary

| Surface | Detail |
|---|---|
| Spec | `Hysteresis { input_col, previous_col, output_col?, on_threshold, off_threshold, off_value, on_value }` |
| Admission | finite thresholds/values; `on_threshold > off_threshold`; distinct cols |
| Compiler | 20-node exact CMP/SELECT tree; ≤ `MAX_EML_TREE_NODES` (32) |
| Oracle | high-activates state machine; exact equality at thresholds |
| Parity | compiled nodes vs `oracle_hysteresis` on off→on, on→off, deadband hold, threshold equality, non-default constants, stateful sequences |

No new opcode. No WGSL/GPU/simthing-sim changes. No runtime gadget execution or chained scheduling.

---

## Files Changed

- `crates/simthing-spec/src/compile/eml_gadget.rs` — CMP/SELECT node builders; `eval_eml_postfix` extension; exact `compile_hysteresis_nodes`; public `compile_eml_gadget`
- `crates/simthing-spec/src/compile/mod.rs`, `lib.rs` — export `compile_eml_gadget`, `oracle_hysteresis`
- `crates/simthing-spec/tests/eml_gadget_tier2_hysteresis.rs` — 16 tests (admission + oracle + compiled parity + opcode inspection)
- `crates/simthing-spec/tests/eml_gadget_tier2_temporal.rs`, `eml_gadget_tier2_bounded_feedback.rs` — deferred-list assertions updated (Hysteresis no longer deferred)
- `docs/tests/phase_m_eml_gadget_2d_hysteresis_test_results.md` — preflight SHA + compiler truth correction
- `docs/workshop/mapping_current_guidance.md`, `eml_gadget_library_design_note.md`, `workshop_current_state.md`, `accumulator_op_v2_production_plan.md`
- `docs/tests/phase_m_eml_gadget_2d_hysteresis_r1_test_results.md` — this report

---

## Required Scans

**Scan 1 (placeholders + stale language):**
```bash
rg "recorded by this push/merge|safe existing path pending full CMP/SELECT|stale next-step tail removed|Implementation stays unauthorized until 2A" docs/tests docs/workshop docs/accumulator_op_v2_production_plan.md crates/simthing-spec/src/compile/eml_gadget.rs
```
**Result:** CLEAN (after preflight + R1 implementation).

**Scan 2 (stale Hysteresis deferred wording):**
```bash
rg "Hysteresis remains conditional/deferred|Hysteresis \(conditional\).*deferred|next authorized slices.*2D" docs/workshop docs/accumulator_op_v2_production_plan.md
```
**Result:** CLEAN in authoritative next-step sections (historical PASS blocks may retain past-tense context).

**Scan 3 (guardrails):**
```bash
rg "new EML opcode|new WGSL|runtime gadget execution|chained OrderBand runtime scheduling|production economy→mapping bridge|default SimSession mapping|atlas/M-4A|CPU urgency|CPU-side AI planner" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_eml_gadget_2d_hysteresis_r1_test_results.md
```
**Result:** Guardrail-only — all matches are explicit non-authorizations / posture reaffirmations.

---

## Tests Run + Results

| Command | Result |
|---|---|
| `cargo test -p simthing-spec --test eml_gadget_tier2_hysteresis -- --nocapture` | **16/16 PASS** |
| `cargo test -p simthing-spec --test eml_gadget_tier2_bounded_feedback -- --nocapture` | **11/11 PASS** |
| `cargo test -p simthing-spec --test eml_gadget_tier2_temporal -- --nocapture` | **10/10 PASS** |
| `cargo test -p simthing-spec --test eml_gadget_tier1 -- --nocapture` | **14/14 PASS** |
| `cargo test -p simthing-spec --test resource_economy_authoring_preview -- --nocapture` | **8/8 PASS** |
| `cargo test -p simthing-driver --test phase_m_economy_sead_product_fixture -- --nocapture` | **6/6 PASS** |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | **28/28 PASS** |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | **11/11 PASS** |
| `cargo check --workspace` | **PASS** |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge` | **OMITTED** — no AccumulatorOp/EvalEML execution assumption changes |

GPU bridge omitted: 2D R1 is pure `simthing-spec` authoring/admission/compiler/oracle; no `simthing-gpu` code touched.

---

## Transient Log Cleanup

Historical `*_full.log` files under `docs/tests/` preserved as intentional evidence. No `*.tmp` or `*scratch*` files found for deletion.

---

## Posture Affirmations

- No runtime/code behavior changed outside `simthing-spec` authoring/admission/compiler/oracle.
- No new EML opcode / WGSL / GPU kernel added.
- No runtime gadget execution or chained OrderBand scheduling.
- No production economy→mapping bridge; no default SimSession mapping wiring.
- Acceleration and dense per-cell temporal memory remain deferred.
- V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

## Final Verdict

**PASS** — Phase M EML-GADGET-2D R1 landed; Hysteresis compiler emission now matches the CPU oracle using existing EvalEML CMP/SELECT primitives, with finite separated-threshold admission, stateful compiled-node parity, active docs and production plan corrected, no new opcode/WGSL/GPU/sim runtime behavior, no runtime gadget execution or chained scheduling, no production economy→mapping bridge, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

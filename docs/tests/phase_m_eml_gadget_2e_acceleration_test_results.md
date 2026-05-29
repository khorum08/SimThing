# Phase M EML-GADGET-2E — Acceleration Feasibility Gate + Implementation — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `a54be37176df2b7e81fb5fc735f5e55015923009` (2D R1 merge)  
**Final commit SHA:** `9d060867e9edc37c1f71d6031fc12a0b5412598c` (2E merge commit; expanded during 2D/2E parking preflight)  
**Verdict:** **PASS — explicit velocity-column Acceleration landed**

---

## Mandatory Production-Doc Preflight

Updated stale Hysteresis/2abc parking guidance in active Phase M EML-GADGET blocks:

1. **`docs/accumulator_op_v2_production_plan.md`**
   - Replaced stale "Hysteresis conditional/deferred" and "no Hysteresis" parking-packet implications with post-2D/2E accurate status.
   - Added consolidated parking-packet sentence noting 2D landed separately.
   - Added EML-GADGET-2E Acceleration entry; PR ladder updated to "landed through 2E".

2. **`docs/workshop/mapping_current_guidance.md`**
   - Updated Current EML-GADGET-2 status and next authorized step for 2E landed.
   - Removed stale "Acceleration and dense per-cell temporal memory remain deferred" from active posture blocks.

3. **`docs/workshop/workshop_current_state.md`**, **`docs/workshop/eml_gadget_library_design_note.md`**, **`docs/invariants.md`**
   - Synced Tier-2 ladder through 2E; position-history acceleration and dense per-cell memory remain separately gated.

No standalone remediation report created; preflight recorded here per handoff.

---

## Feasibility Decision

**IMPLEMENT (Path A)** — Acceleration fits safely as an explicit velocity-column Tier-2 gadget:

```text
acceleration = (current_velocity_col - previous_velocity_col) / dt   // dt defaults to 1.0
```

**Rationale:**
- Same substrate as VelocityMonitor: explicit `current_velocity_col` + `previous_velocity_col`, optional finite positive `dt`, optional `output_col`.
- Compiler reuses existing EvalEML `SLOT_VALUE`, `SUB`, optional `DIV`, `RETURN_TOP` — no new opcode.
- Position-history acceleration `(velocity_now - velocity_prev) / dt` requiring `previous_previous_col` was **not** implemented (would trigger STOP / dense temporal-memory gate).

**STOP conditions not triggered.**

---

## Implementation Summary

| Surface | Detail |
|---|---|
| Spec | `Acceleration { current_velocity_col, previous_velocity_col, dt?, output_col? }` |
| Admission | valid distinct velocity cols; finite `dt > 0` when provided |
| Compiler | delegates to `compile_velocity_monitor_nodes` (SUB + optional DIV) |
| Oracle | `oracle_acceleration(current, previous, dt)` with `dt = 1.0` default |
| Deferred list | `DEFERRED_GADGET_KINDS` emptied; position-history acceleration is a separate gate |

No hidden previous-value read. No automatic snapshot/copy scheduling. No dense per-cell memory. No runtime gadget execution or chained scheduling.

---

## Files Changed

- `crates/simthing-spec/src/spec/eml_gadget.rs` — `Acceleration` variant
- `crates/simthing-spec/src/compile/eml_gadget.rs` — admission, compile, oracle, deferred-list update
- `crates/simthing-spec/src/compile/mod.rs`, `lib.rs` — export `oracle_acceleration`
- `crates/simthing-spec/tests/eml_gadget_tier2_acceleration.rs` — **new** (11 tests)
- `crates/simthing-spec/tests/eml_gadget_tier2_temporal.rs`, `eml_gadget_tier2_bounded_feedback.rs` — deferred-list assertions
- `docs/accumulator_op_v2_production_plan.md`
- `docs/workshop/eml_gadget_library_design_note.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/invariants.md`
- `docs/worklog.md`
- `docs/tests/phase_m_eml_gadget_2e_acceleration_test_results.md` — this report

---

## Required Scans

**Scan 1 (stale Hysteresis / parking language):**
```bash
rg "Hysteresis remains conditional/deferred|no Hysteresis|Implementation stays unauthorized until 2A|stale next-step tail removed|safe existing path pending full CMP/SELECT" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests
```
**Result:** **CLEAN** in active authoritative docs. Historical test reports retain past-tense context only (explicitly non-authoritative).

**Scan 2 (guardrails):**
```bash
rg "new EML opcode|new WGSL|runtime gadget execution|chained OrderBand runtime scheduling|production economy→mapping bridge|default SimSession mapping|atlas/M-4A|CPU urgency|CPU-side AI planner|previous_previous_col|hidden previous-value read" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests
```
**Result:** **Guardrail-only** — matches are explicit non-authorizations, posture reaffirmations, or STOP-rationale context in historical reports. Active guidance states 2E landed without position-history / `previous_previous_col` inference.

---

## Tests Run + Results

| Command | Result |
|---|---|
| `cargo test -p simthing-spec --test eml_gadget_tier2_acceleration -- --nocapture` | **11/11 PASS** |
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

---

## Transient Log Cleanup

Checked `docs/tests` for `*.log`, `*tmp*`, `*scratch*` files.

**Result:** Historical `*_full.log` files preserved as intentional evidence. No scratch/tmp files deleted.

---

## Posture Affirmations

- No runtime/code behavior changed outside `simthing-spec` authoring/admission/compiler/oracle.
- No new EML opcode / WGSL / GPU kernel added.
- No runtime gadget execution or chained OrderBand scheduling.
- No production economy→mapping bridge; no default SimSession mapping wiring.
- Position-history acceleration and dense per-cell temporal memory remain separately gated.
- V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

## Final Verdict

**PASS** — Phase M EML-GADGET-2E Acceleration landed as a narrow explicit velocity-column Tier-2 EML gadget in `simthing-spec` authoring/admission/compiler/oracle only, with finite dt admission, compiled-node parity over existing EvalEML arithmetic primitives, active docs and production plan updated, no hidden temporal reads, no dense temporal memory, no new opcode/WGSL/GPU/sim runtime behavior, no runtime gadget execution or chained scheduling, no production economy→mapping bridge, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

# Phase M-2 — Field Scheduler + Dirty Macro-Region Skip — Test Results

**Date/time:** 2026-05-28  
**Base HEAD:** `8de55d6bafe9ec62dc389e5113a72815fdfaf3fb` (Phase M-1.1 merge, PR #220)  
**Branch:** `phase-m2-field-scheduler`  
**Final commit SHA:** `fd1b75d` (phase-m2-field-scheduler)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell

---

## Goal

Implement Phase M-2 generic cadence scheduler and dirty macro-region skip in `simthing-driver` without production mapping runtime or pass graph wiring.

---

## Files changed

| Area | Files |
|------|-------|
| Scheduler | `crates/simthing-driver/src/field_scheduler.rs`, `lib.rs` |
| Tests | `crates/simthing-driver/tests/phase_m2_field_scheduler.rs` |
| Docs | `accumulator_op_v2_production_plan.md`, `todo.md`, `worklog.md`, `workshop_current_state.md`, `mapping_current_guidance.md` |

No new WGSL. No pass graph changes. No `simthing-sim` changes.

---

## Scheduler decision summary

| Cadence (120 ticks) | Due count |
|---------------------|-----------|
| EveryTick | 120 |
| EveryN(4) | 30 |
| EveryN(10) | 12 |
| EveryN(60) | 2 |
| OnEvent (3 event ticks) | 3 |

Dirty skip fixture (tick=1, EveryN(4) not due): clean skipped; dirty_source, dirty_neighbor, residual, topology_changed, operator_changed, cadence_due (EveryTick field) all scheduled. **false_skip_count = 0**.

Report fixture: 10 regions, 4 scheduled, 6 skipped, skip_ratio = 0.6.

---

## Alternate square-size fixtures

Scheduler decisions identical across evidence-only grid descriptors:

| Grid | Scheduler outcome |
|------|-------------------|
| 5×5 | Dispatch (dirty_neighbor at tick 3) |
| 10×10 | Dispatch |
| 20×20 | Dispatch |

Grid size is not a runtime scheduler fence (M-3 admission concern).

---

## Commands run

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture` | **PASS** — 8/8 |
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** — 16/16 |
| `cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture` | **PASS** — 5/5 |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-spec --test eml_field_formula_admission -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |

---

## Pass/fail table

| Criterion | Status |
|-----------|--------|
| Generic cadence policy | **PASS** |
| EveryN(0) rejected | **PASS** |
| Cadence determinism/replay | **PASS** |
| Dirty macro-region skip | **PASS** |
| Zero false skips | **PASS** |
| Scheduler report metrics | **PASS** |
| Scheduled execution no-readback default | **PASS** |
| Readback test/debug only | **PASS** |
| Alternate square-size size-agnostic | **PASS** |
| No mapping runtime | **PASS** |
| No pass graph wiring | **PASS** |
| E-11B / Resource Flow preserved | **PASS** |
| Full workspace check/test | **PASS** |

---

## Important excerpts

- `execute_scheduled_stencil_regions` uses `StructuredFieldExecutionOptions::default()` (no readback).
- Skipped regions produce no `execute_configured` call; only dirty region executed in test E.
- `FieldSchedulerReport.false_skip_count` stays 0 in all fixtures.

---

## Final verdict

**PASS** — Phase M-2 generic cadence scheduler and dirty macro-region skip landed; scheduled execution uses the M-1.1 no-readback path by default; no false skips in dirty-skip tests; no production mapping runtime or pass graph wiring landed.

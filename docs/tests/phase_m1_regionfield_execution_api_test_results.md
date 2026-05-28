# Phase M-1 — Generic RegionField Execution API — Test Results

**Date/time:** 2026-05-28  
**Base HEAD:** `a498276dabfd0659e13375c1b3287cb13303975c` (docs cleanup merge, PR #218)  
**Branch:** `phase-m1-regionfield-execution-api`  
**Final commit SHA:** `74f4956` (phase-m1-regionfield-execution-api)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell

---

## Goal

Implement Phase M-1 generic natives: configured-horizon execution API on `StructuredFieldStencilOp`, generic debug/observability surface, and column-aware reduction convenience over existing `SlotRange` Sum — without production mapping runtime, pass graph wiring, or map/faction/AI semantics.

---

## Files changed

| Area | Files |
|------|-------|
| GPU primitive | `crates/simthing-gpu/src/structured_field_stencil.rs`, `lib.rs` |
| AccumulatorOp builder | `crates/simthing-core/src/accumulator_op_builder.rs`, `lib.rs` |
| Tests | `crates/simthing-gpu/tests/structured_field_stencil.rs`, `crates/simthing-driver/tests/structured_field_region_execution.rs` (new) |
| Docs | `accumulator_op_v2_production_plan.md`, `todo.md`, `worklog.md`, `workshop_current_state.md`, `mapping_current_guidance.md` |

No new WGSL. No production pass graph changes. No `simthing-sim` changes.

---

## Commands run

| Command | Result |
|---------|--------|
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** — 14/14 |
| `cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture` | **PASS** — 4/4 |
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
| Generic configured execution API with horizon guards | **PASS** |
| Debug report exposes required generic fields | **PASS** |
| Optional stats do not force readback unless requested | **PASS** |
| Column-aware reduction helper over existing SlotRange Sum | **PASS** |
| Helper output matches manual SlotRange Sum | **PASS** |
| Existing stencil GPU/CPU parity unchanged | **PASS** |
| No mapping runtime | **PASS** |
| No production pass graph wiring | **PASS** |
| No semantic/map-specific WGSL | **PASS** |
| `simthing-sim` remains map-free | **PASS** |
| Resource Flow / E-11B posture preserved | **PASS** |
| Active docs updated | **PASS** |
| Full workspace check/test | **PASS** |

---

## Important excerpts

- `execute_configured` rejects `steps > config.horizon` with `ExecutionHorizonExceedsConfig`.
- `collect_field_stats: false` leaves `field_max`, `field_l1_norm`, and `active_mask_ratio` as `None`.
- `column_aware_reduction_op` produces bit-identical `AccumulatorOp` to manual `SlotRange` Sum registration.
- Source scan: `passes.rs`, `session.rs`, and `simthing-sim` contain no `StructuredFieldStencilOp` / `RegionField` wiring.

---

## Final verdict

**PASS** — Phase M-1 generic execution API landed; StructuredFieldStencilOp remains opt-in and inert by default; column-aware reduction helper matches manual SlotRange Sum; no production mapping runtime or pass graph wiring landed.

# Phase M-1.1 — No-Readback Structured Field Execution Hardening — Test Results

**Date/time:** 2026-05-28  
**Base HEAD:** `fd1e612575241df333bdeb881203e880e975021c` (Phase M-1 merge, PR #219)  
**Branch:** `phase-m1-1-no-readback-hardening`  
**Final commit SHA:** `e709a7b` (phase-m1-1-no-readback-hardening)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell

---

## Goal

Remedial hardening after Phase M-1: make `execute_configured` GPU-resident by default (dispatch + debug metadata only), with explicit readback for tests/diagnostics and readback-derived stats.

---

## Files changed

| Area | Files |
|------|-------|
| GPU primitive | `crates/simthing-gpu/src/structured_field_stencil.rs` |
| Tests | `crates/simthing-gpu/tests/structured_field_stencil.rs`, `crates/simthing-driver/tests/structured_field_region_execution.rs` |
| Docs | `accumulator_op_v2_production_plan.md`, `todo.md`, `worklog.md`, `workshop_current_state.md`, `mapping_current_guidance.md` |

No new WGSL. No pass graph changes. No `simthing-sim` changes.

---

## API changes

- `StructuredFieldExecutionOptions.readback_values: bool` (default `false`)
- `StructuredFieldExecutionReport.values: Option<Vec<f32>>`
- `collect_field_stats: true` implies readback (stats are readback-derived)
- Low-level APIs unchanged: `dispatch_ping_pong`, `run_ping_pong`, `run_configured_horizon`, `readback_after_ping_pong`

---

## Commands run

| Command | Result |
|---------|--------|
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
| No-readback configured execution path | **PASS** |
| Values readback explicit | **PASS** |
| Field stats optional and readback-derived | **PASS** |
| Horizon guards on all paths | **PASS** |
| Column-aware reduction helper unchanged | **PASS** |
| Existing stencil parity green | **PASS** |
| No mapping runtime | **PASS** |
| No production pass graph wiring | **PASS** |
| Resource Flow / E-11B preserved | **PASS** |
| Active docs updated | **PASS** |
| Full workspace check/test | **PASS** |

---

## Important excerpts

- Default `StructuredFieldExecutionOptions`: `readback_values: false`, `collect_field_stats: false` → `values: None`, all stats `None`, dispatch proceeds.
- `readback_values: true`, `collect_field_stats: false` → `values: Some(...)`, stats remain `None`.
- `collect_field_stats: true` → readback forced, `field_max` / `field_l1_norm` / `active_mask_ratio` populated.
- Horizon guard: `steps: Some(8)` with `config.horizon = 4` returns `ExecutionHorizonExceedsConfig` on both no-readback and readback paths.

---

## Final verdict

**PASS** — Phase M-1.1 no-readback execution hardening landed; configured structured-field execution can dispatch without forced CPU readback; explicit readback/stats paths remain available; no production mapping runtime or pass graph wiring landed.

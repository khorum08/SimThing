# Phase M-2.1 — FieldScheduler API Hardening — Test Results

**Date/time:** 2026-05-28  
**Base HEAD:** `fef444a40a6d7e76a230a1f0a920886f7289d11b` (Phase M-2 merge, PR #221)  
**Branch:** `phase-m2-1-field-scheduler-api-hardening`  
**Final commit SHA:** `f349691` (phase-m2-1-field-scheduler-api-hardening)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell

---

## Goal

Remedial API hardening after Phase M-2: fix region identity collision and remove unsafe repeated same-op scheduled execution.

---

## Region identity summary

| Case | Result |
|------|--------|
| `FieldId(1)/RegionId(0)` + `FieldId(2)/RegionId(0)` | Both coexist (`regions.len() == 2`, `decisions.len() == 2`) |
| Re-register `FieldId(1)/RegionId(0)` with dirty state | One region remains; latest state wins; scheduled |

Scheduler identity key: **`(FieldId, FieldRegionId)`**.

---

## Scheduled visitor/executor summary

| API | Behavior |
|-----|----------|
| `visit_scheduled_regions` | Calls closure only for dispatch decisions; skipped regions not visited |
| `execute_scheduled_regions_with` | Caller-provided per-decision execution |
| `execute_single_scheduled_stencil_region` | At most one scheduled region per op; errors on multiple |

Removed: `execute_scheduled_stencil_regions` (unsafe multi-dispatch on one buffer).

Visitor test: 3 decisions (1 skip, 2 dispatch) → closure called exactly 2 times.  
Single-op guard: 2 scheduled regions → `ScheduledStencilExecutionError::MultipleScheduledRegionsForSingleOp { count: 2 }`.

GPU tests use `execute_single_scheduled_stencil_region` with `StructuredFieldExecutionOptions::default()` → `values == None`.

---

## Commands run

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** — 16/16 |
| `cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture` | **PASS** — 5/5 |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |

---

## Pass/fail table

| Criterion | Status |
|-----------|--------|
| Region identity uses (FieldId, FieldRegionId) | **PASS** |
| Same region ID under different fields coexists | **PASS** |
| Same field/region replacement works | **PASS** |
| No repeated same-op multi-region execution footgun | **PASS** |
| Skipped decisions skip visitor/execution | **PASS** |
| Existing cadence/dirty-skip tests green | **PASS** |
| No-readback default in GPU execution test | **PASS** |
| No mapping runtime / pass graph wiring | **PASS** |
| E-11B / Resource Flow preserved | **PASS** |
| Full workspace check/test | **PASS** |

---

## Final verdict

**PASS** — Phase M-2.1 FieldScheduler API hardening landed; scheduler region identity is keyed by (FieldId, FieldRegionId); scheduled execution orchestration no longer risks repeatedly advancing one StructuredFieldStencilOp for multiple regions; no production mapping runtime or pass graph wiring landed.

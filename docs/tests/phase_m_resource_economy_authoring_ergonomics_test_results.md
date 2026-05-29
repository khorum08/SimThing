# Phase M Resource Economy Authoring Ergonomics V1 — Test Results

Date: 2026-05-29

## Base

- Base HEAD: `2e137654bbab3b387a33b595bbd3c2b228e248d2`
- Final commit SHA: recorded at merge

## Files Changed

- `crates/simthing-spec/src/compile/resource_economy_admission.rs` — `ResourceEconomyAuthoringPreview`, `ResourceEconomyPreviewReport`, compile/preview helpers, simple static transfer-only net
- `crates/simthing-spec/src/compile/mod.rs` — module export
- `crates/simthing-spec/src/error.rs` — `ResourceEconomyAdmission`
- `crates/simthing-spec/src/lib.rs` — public exports
- `crates/simthing-spec/tests/resource_economy_authoring_preview.rs` — 8 spec/admission tests
- `crates/simthing-driver/tests/phase_m_resource_economy_authoring_ergonomics.rs` — 4 driver fixture + doctrine tests
- `crates/simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron` — softened description (example fixture framing)
- Docs: production plan, mapping guidance, workshop state, todo, worklog

## Preview / Diagnostic Design

| Component | Role |
|---|---|
| `compile_resource_economy_authoring_preview` | Wraps existing `compile_resource_economy`; builds structural preview report |
| `compile_game_mode_resource_economy_authoring_preview` | Compiles properties from `GameModeSpec`, then previews economy block |
| `ResourceEconomyPreviewReport` | `opt_in_mode`, counts, `order_bands`, `resources_bound`, transfer/recipe/threshold rows, `resource_flow_enabled`, `simple_static_nets`, `warnings` |
| `StaticPropertyNetPreview` | Diagnostic-only transfer-only net per property/role for one boundary (no CPU runtime execution) |
| Admission diagnostics | Reuses existing T-2 compile rejections (duplicate IDs, missing property/role, non-finite amounts/costs) |

No new runtime behavior. No CPU economy executor. No threshold event emission from preview.

## Commands Run

| Command | Result |
|---|---|
| `git rev-parse HEAD` | PASS; `2e137654bbab3b387a33b595bbd3c2b228e248d2` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-spec --test resource_economy_authoring_preview -- --nocapture` | PASS; 8/8 |
| `cargo test -p simthing-driver --test phase_m_resource_economy_authoring_ergonomics -- --nocapture` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_daily_economy_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_boundary_cadence_doctrine -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test resource_economy_designer_ron_session -- --nocapture` | PASS; 3/3 |
| `cargo test -p simthing-driver --test resource_economy_boundary_refresh -- --nocapture` | PASS; 5/5 |
| `cargo test -p simthing-driver --test resource_economy_compile -- --nocapture` | PASS; 8/8 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 3/3 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS (see full log) |

Full log: [`phase_m_resource_economy_authoring_ergonomics_full.log`](phase_m_resource_economy_authoring_ergonomics_full.log)

## Pass/Fail Table

| Test area | Result |
|---|---|
| Surplus fixture preview | PASS |
| Deficit fixture preview | PASS |
| Missing property diagnostic | PASS |
| Missing role diagnostic | PASS |
| Duplicate ID rejection | PASS |
| Non-finite amount/cost rejection | PASS |
| Doctrine posture preserved | PASS |
| Regressions (daily economy, boundary doctrine, designer RON, compile, first slice, region field, GPU bridge) | PASS |

## Surplus Fixture Preview Summary

- `opt_in_mode`: `TransferOnly`
- `transfer_count`: 2 (`bank_daily_income`, `daily_upkeep`)
- `recipe_count`: 1 (`daily_income`)
- `order_bands`: `[0, 1]`
- `resource_flow_enabled`: false
- Treasury static net: **+7** per boundary

## Deficit Fixture Preview Summary

- `transfer_count`: 2; `recipe_count`: 0
- `threshold_emit_count`: 1 (`low_storage_event`)
- `resource_flow_enabled`: false
- Treasury static net: **−6** per boundary
- No CPU event emission from preview path

## Invalid Authoring Diagnostics Summary

Existing T-2 compile errors surfaced through preview wrapper:

- Unknown source/target property includes transfer id + namespace/name
- Invalid role includes context + property + role label
- Duplicate authoring id rejected
- Non-finite transfer amount and recipe `unit_cost` rejected

## Doctrine / Posture Summary

- No `DailyResolutionBoundary` type added
- No calendar/pause semantics added to `simthing-sim`
- `MappingExecutionProfile::default()` == `Disabled`
- Resource Flow E-11 remains default-off (`ResourceFlowOptInMode::Disabled`, `use_accumulator_resource_flow=false`)
- Legible `day_index` / `ticks_per_day` / daily fixture labels allowed and unchanged
- No semantic WGSL, atlas, default mapping wiring, or CPU planner

## Final Verdict

**PASS — Phase M Resource Economy Authoring Ergonomics V1 landed; discrete ResourceEconomySpec authoring now has preview/diagnostic support while preserving abstract boundary doctrine, Resource Flow default-off posture, no DailyResolutionBoundary, no simthing-sim calendar/pause semantics, no runtime economy behavior changes, no semantic WGSL, and no atlas/default mapping wiring.**

# Phase M Boundary Cadence Doctrine — Audit Report

Date: 2026-05-29

## Base

- Base HEAD: `296b8126b1df806b41e1d395ff7f9adcaf4e9f9c` (Map Residency V1)
- Final commit SHA: `338bcf5` (branch `phase-m-boundary-cadence-doctrine-audit`)

## Scope

- **Type:** test + docs audit pass (no new runtime semantics)
- **Files changed:**
  - `crates/simthing-driver/tests/phase_m_boundary_cadence_doctrine.rs` — 6-test doctrine audit suite
  - `docs/accumulator_op_v2_production_plan.md`
  - `docs/workshop/mapping_current_guidance.md`
  - `docs/workshop/workshop_current_state.md`
  - `docs/todo.md`
  - `docs/worklog.md`
  - `docs/tests/phase_m_boundary_cadence_doctrine_audit.md` (this report)

## Existing Mechanisms Confirmed

| Mechanism | Location | Doctrine |
|---|---|---|
| `ticks_per_day`, `tick_in_day`, `boundary_reached`, `day_index` | `crates/simthing-feeder/src/dispatcher.rs` (`DispatchCoordinator::tick`) | `ticks_per_day=1` → every tick is a boundary; `ticks_per_day=N` → boundary after N substrate ticks; `day_index` advances at boundary cadence |
| `SimSession::run(max_days)` | `crates/simthing-driver/src/session.rs` | Host-driven loop; advances only when host calls `run`/`tick_one` |
| `BoundaryProtocol` + boundary hook | `crates/simthing-sim/src/boundary.rs` | CPU boundary sequence: GPU readback → hook → structural/lifecycle steps |
| `can_skip_empty_boundary` | `crates/simthing-sim/src/boundary.rs`, used in `session.rs` | Static-day fast path when no events/pending work |
| Discrete resource economy | `ResourceEconomySpec`, `CompiledResourceTransfer`, `CompiledResourceRecipe` | Daily banking via production recipes + discrete transfers |
| Summary-tier readback | Boundary GPU value readback + threshold events + first-slice summary/residency metadata | CPU consumes compact resolved outputs at boundary |

## Existing Tests Identified

| Test | Coverage |
|---|---|
| `simthing-feeder/tests/integration.rs::day_boundary_fires_on_ticks_per_day` | `ticks_per_day=4` → one boundary after four ticks |
| `simthing-feeder/tests/integration.rs::boundary_requests_reach_tree_maintainer` | `ticks_per_day=1` → every tick is a boundary |
| `simthing-sim/tests/boundary_integration.rs` | `ticks_per_day=1`, `=2` boundary cadence with full protocol |
| `simthing-driver/tests/resource_economy_designer_ron_session.rs` | `ticks_per_day: 1` + discrete economy RON |
| `simthing-driver/tests/resource_economy_boundary_refresh.rs` | Boundary refresh after structural boundary |
| `simthing-driver/tests/resource_economy_compile.rs` | Conservation-exact discrete transfer compile |

## New Tests Added

`crates/simthing-driver/tests/phase_m_boundary_cadence_doctrine.rs` (6 tests):

1. `doctrine_no_daily_resolution_boundary_primitive` — no forbidden runtime type
2. `doctrine_pause_is_host_non_advancement` — coordinator idle until host invokes tick
3. `ticks_per_day_one_boundary_every_tick` — N ticks → N boundaries, `day_index` advances each tick
4. `ticks_per_day_four_one_boundary_after_four_ticks` — 4 ticks → 1 boundary, `day_index` advances once
5. `host_pause_preserves_state_after_partial_advancement` — no tick call → frozen state
6. `daily_resource_economy_fixture_uses_ticks_per_day_one` — discrete economy fixtures cite daily cadence

## Commands Run

| Command | Result |
|---|---|
| `git status --short` | PASS |
| `git rev-parse HEAD` | PASS; base `296b8126b1df806b41e1d395ff7f9adcaf4e9f9c` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_boundary_cadence_doctrine -- --nocapture` | PASS; 6/6 |
| `cargo test -p simthing-driver --test phase_m_first_slice_map_residency -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_queue_write_hardening -- --nocapture` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_first_slice_summary_validity -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 3/3 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | See full log |

Full log: [`phase_m_boundary_cadence_doctrine_full.log`](phase_m_boundary_cadence_doctrine_full.log)

## Daily Cadence Doctrine Summary

- **day** = host/spec/calendar interpretation of a boundary index (`day_index`)
- **tick** = deterministic substrate advancement (`DispatchCoordinator::tick`)
- **boundary** = existing synchronization point (`boundary_reached` when `tick_in_day >= ticks_per_day`)
- `ticks_per_day=1` means every substrate tick is a day boundary
- `ticks_per_day=N` means N substrate ticks per day boundary

## Pause / Boundary Doctrine Summary

- The sim does not autonomously advance; no internal wall-clock scheduler drives simulation
- Pause/speed are host/UI orchestration: host chooses when to call `SimSession::run` / `tick_one`
- Pausing at a boundary is a coherent save/snapshot point (GPU values read back, boundary hook runs)
- No sim pause flag was added

## Resource-Economy Banking Doctrine Summary

- Clausewitz-style daily banking uses the **discrete resource economy** path:
  - `ResourceEconomySpec` production recipes
  - `CompiledResourceTransfer` into storage columns
  - upkeep transfers out
  - threshold/event checks (e.g. bankruptcy) via existing threshold substrate
- **Resource Flow Substrate E-11** remains continuous/high-frequency oriented and separately opt-in (`use_accumulator_resource_flow` default false)
- Existing fixtures use `ticks_per_day: 1` for daily cadence (`resource_economy_session.rs`, `resource_economy_designer_ron_session.rs`)

## CPU Boundary Discipline Summary

- CPU receives resolved summaries/events/metadata at boundary (GPU value readback, threshold events, boundary hook outputs)
- CPU must not scan dense RegionCell grids by default
- CPU must not recompute threat/urgency/economy state
- CPU must not emit AI commitments via if/else planner logic
- First-slice mapping: GPU-resident path resolves field/reduction/EML/threshold; SummaryValidity and Residency expose metadata; CPU boundary consumes compact resolved outputs

## Posture Summary

Preserved:

- V7.7 Mapping ADR approved; Phase M first-slice vertical proof accepted
- SummaryValidity V1 + V1-R1 parked; Queue-Write Scale Hardening V1 landed; Map Residency V1 landed
- `MappingExecutionProfile::default()` = Disabled; no default SimSession mapping wiring
- simthing-sim remains map-free; no semantic WGSL; no atlas/M-4A/perception/source_mask
- No new EML opcode; no CPU-side AI planner; no `DailyResolutionBoundary` runtime primitive
- Queue-write child resource scale caveat addressed for first-slice by generic bulk fill; parent scalar writes remain O(1)

## Final Verdict

**PASS** — Phase M Boundary Cadence Doctrine audit completed; Clausewitz-style daily resolution is expressible through existing boundary cadence machinery and discrete resource-economy authoring, with day/calendar/pause semantics kept at host/spec/boundary-handler layer and no new semantic DailyResolutionBoundary primitive introduced.

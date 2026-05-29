# Phase M Daily Economy Fixture V1 — Test Report

Date: 2026-05-29

## Base

- Base HEAD: `cdabea6471b6231d7eb3aaacb2b9029b147cc8ad` (Phase M Boundary Cadence Doctrine audit)
- Branch: `phase-m-daily-economy-fixture-v1`
- Final commit SHA: `a8cb12e`

## Files changed

- `crates/simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron` — surplus daily banking RON (`TransferOnly`, recipe + bank/upkeep transfers)
- `crates/simthing-driver/tests/fixtures/daily_economy_banking_deficit_scenario.ron` — deficit variant with `EmitOnThreshold`
- `crates/simthing-driver/tests/support/daily_economy_session.rs` — session helpers
- `crates/simthing-driver/tests/phase_m_daily_economy_fixture.rs` — 7-test fixture suite
- `crates/simthing-driver/tests/phase_m_boundary_cadence_doctrine.rs` — daily economy fixture cadence citation
- `docs/accumulator_op_v2_production_plan.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/tests/phase_m_boundary_cadence_doctrine_audit.md` — date typo fix (`2026-05-29`)
- `docs/tests/phase_m_daily_economy_fixture_test_results.md` (this report)
- `docs/tests/phase_m_daily_economy_fixture_full.log` — workspace test log

## Daily economy fixture design

Surplus game mode (`TransferOnly`):

- Properties: `treasury` (100), `producer` (10), `food`/`ore` (60 each), `upkeep_sink` (0)
- Recipe `daily_income`: conjunctive food+ore → `producer` (renewable daily production stock)
- Transfer `bank_daily_income`: producer → treasury (10/day)
- Transfer `daily_upkeep`: treasury → upkeep_sink (3/day)
- Net surplus: +7/day at `ticks_per_day=1`

Deficit game mode (`TransferOnly`):

- Properties: `treasury` (100), `producer` (2), `upkeep_sink` (0)
- Transfer bank: producer → treasury (2/day)
- Transfer upkeep: treasury → upkeep_sink (8/day)
- `EmitOnThreshold` `low_storage_event` on treasury falling through 95 (`0x4C4F5754`)

**Important:** C-8d `ResourceEmissionSpec` uses `ConsumeMode::EmitEvent` and is not hard-currency banking. Daily income is modeled as recipe production + discrete transfer, not emission.

## Commands run

| Command | Result |
|---|---|
| `git status --short` | PASS |
| `git rev-parse HEAD` | PASS; base `cdabea6471b6231d7eb3aaacb2b9029b147cc8ad` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_daily_economy_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_boundary_cadence_doctrine -- --nocapture` | PASS; 6/6 |
| `cargo test -p simthing-driver --test resource_economy_designer_ron_session -- --nocapture` | PASS; 3/3 |
| `cargo test -p simthing-driver --test resource_economy_boundary_refresh -- --nocapture` | PASS; 5/5 |
| `cargo test -p simthing-driver --test resource_economy_compile -- --nocapture` | PASS; 8/8 |
| `cargo test -p simthing-driver --test phase_m_first_slice_map_residency -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_queue_write_hardening -- --nocapture` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS (see full log) |

Full log: [`phase_m_daily_economy_fixture_full.log`](phase_m_daily_economy_fixture_full.log)

## Pass/fail table

| Test | Result |
|---|---|
| `daily_economy_ron_admits_and_compiles` | PASS |
| `one_day_surplus_banks_into_treasury` | PASS |
| `multi_day_accumulation_is_deterministic` | PASS |
| `multi_day_replay_matches_storage_trajectory` | PASS |
| `deficit_upkeep_emits_low_storage_threshold_event` | PASS |
| `sub_day_boundary_cadence_documented_not_daily_amount_scaling` | PASS |
| `posture_preserved_no_new_daily_semantics` | PASS |

## One-day banking summary

- `ticks_per_day=1`, `day_index=1` after one boundary-forced day
- Treasury after one day: `100 + 10 - 3 = 107` (within float tolerance)
- `use_accumulator_transfer=true`, `use_accumulator_emission=false`, `use_accumulator_resource_flow=false`

## Multi-day accumulation summary

- Five-day trace (both runs): `[107, 114, 121, 128, 135]`
- Final treasury: `135.0`
- Treasury persists in GPU values buffer across days without per-day re-upload

## Deficit / threshold event summary

- One-day deficit treasury: `100 + 2 - 8 = 94`
- Exactly one `low_storage_event` (`0x4C4F5754`) emitted by threshold substrate
- CPU test does not emit events via manual planner logic

## Boundary cadence summary

- Fixture uses existing `ticks_per_day=1` / `day_index` machinery
- No `DailyResolutionBoundary` primitive
- Sub-day cadence (`ticks_per_day=4`) documented as covered by boundary cadence doctrine; not reimplemented here

## CPU boundary discipline summary

- Tests read resolved treasury from GPU `read_values()` at boundary
- No dense RegionCell grid scan
- No CPU-side economy recomputation path
- Threshold events consumed from tick/boundary GPU readback

## Posture summary

- V7.7 Mapping ADR posture preserved
- `MappingExecutionProfile::default() == Disabled`
- `PipelineFlags::default().use_accumulator_resource_flow == false`
- No semantic WGSL, atlas batching, M-4A masking, active mask, perception, source_mask
- simthing-sim remains map-free
- No new EML opcode, no CPU-side planner

## Final verdict

**PASS** — Phase M Daily Economy Fixture V1 landed; Clausewitz-style daily banking is now proven through existing ticks_per_day=1 boundary cadence and discrete ResourceEconomySpec transfers, with resolved storage/events visible at the CPU boundary and no new DailyResolutionBoundary primitive, semantic WGSL, Resource Flow default, map runtime expansion, or CPU-side planner introduced.

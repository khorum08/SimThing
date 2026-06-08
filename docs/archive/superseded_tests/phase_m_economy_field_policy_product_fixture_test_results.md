# Phase M Economy + FIELD_POLICY Product Fixture V1 — Test Results

Date: 2026-05-29

## Base

- Base HEAD: `bea004f31418b56d4d83f75edf00523c0482b7d2`
- Final commit SHA: recorded at merge

## Files Changed

- `crates/simthing-driver/tests/support/economy_field_policy_product_fixture.rs` — Option A orchestration helpers
- `crates/simthing-driver/tests/phase_m_economy_field_policy_product_fixture.rs` — 6-test product fixture suite
- Docs: production plan, mapping guidance (stale “parked for review” corrected), workshop state, todo, worklog

## Integration Strategy

**Option A — test-level orchestration (preferred, landed).**

1. Run existing daily economy fixture at boundary (`SimSession`, discrete `ResourceEconomySpec`).
2. Read resolved treasury storage at boundary (CPU readback allowed).
3. Map treasury stress to authored EML weight profiles in test code:
   - treasury > 95 → low stress → `(0.2, 0.1)`
   - treasury ≤ 95 → high stress → `(0.9, 0.1)` (aligned with deficit fixture threshold)
4. Drive existing first-slice scenario commitment path (`FirstSliceScenarioFixtureSession` + GPU field/reduction/EML/Threshold+EmitEvent).

No production runtime bridge. No unified GameModeSpec. No SimSession mapping pass-graph wiring.

## Commands Run

| Command | Result |
|---|---|
| `git rev-parse HEAD` | PASS; `bea004f31418b56d4d83f75edf00523c0482b7d2` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_economy_field_policy_product_fixture -- --nocapture` | PASS; 6/6 |
| `cargo test -p simthing-driver --test phase_m_resource_economy_authoring_ergonomics -- --nocapture` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_daily_economy_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_boundary_cadence_doctrine -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture` | PASS; 9/9 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-driver --test phase_m_first_slice_summary_validity -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-driver --test phase_m_first_slice_map_residency -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-spec --test resource_economy_authoring_preview -- --nocapture` | PASS; 8/8 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 3/3 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS (see full log) |

Full log: [`phase_m_economy_field_policy_product_fixture_full.log`](phase_m_economy_field_policy_product_fixture_full.log)

## Pass/Fail Table

| Test | Result |
|---|---|
| 1 — fixtures admit/preview | PASS |
| 2 — surplus economy, no FIELD_POLICY commitment | PASS |
| 3 — deficit economy, one FIELD_POLICY commitment | PASS |
| 4 — deterministic replay | PASS |
| 5 — SummaryValidity / Residency not broken | PASS |
| 6 — posture preservation | PASS |
| Regressions | PASS |

## Surplus Economy + No-Commit Summary

- One boundary day on surplus fixture → treasury **107**
- Economy stress signal **0** (healthy storage)
- EML weights **(0.2, 0.1)**
- GPU first-slice path executes (9 dispatches, `reduction_stencil_readbacks == 0`)
- field_urgency ≈ **2003** < threshold **5490.8657**
- FIELD_POLICY event count **0**

## Deficit Economy + Commit Summary

- One boundary day on deficit fixture → treasury **94**
- Economy stress signal **1** (low storage)
- Economy substrate also emits `LOW_STORAGE` (`0x4C4F5754`) — separate from FIELD_POLICY
- EML weights **(0.9, 0.1)**
- field_urgency ≈ **8979** > threshold **5490.8657**
- FIELD_POLICY event count **1**, kind **0x53454144** ("FIELD_POLICY"), from Threshold+EmitEvent on parent slot 100 / urgency col 4

## GPU-Resident FIELD_POLICY Path Summary

- Existing `FirstSliceScenarioFixtureSession::tick_with_scenario_commitment` hot path
- Field propagation → parent reduction → field_urgency EvalEML → Threshold + EmitEvent
- No CPU urgency computation; diagnostic readback used for verification only

## CPU Boundary Discipline Summary

- CPU reads resolved treasury after discrete economy boundary sync
- CPU maps treasury to authored weight profiles (fixture orchestration)
- CPU does not emit FIELD_POLICY commitment events
- CPU does not recompute threat/urgency from field values

## Doctrine / Posture Summary

- No `DailyResolutionBoundary` type
- No calendar/pause semantics in `simthing-sim`
- `MappingExecutionProfile::default()` == `Disabled`
- Resource Flow E-11 default-off
- `request_atlas_batching` rejected at admission
- No semantic WGSL, atlas, default mapping wiring, or CPU planner
- Legible `day_index` / `ticks_per_day` naming unchanged

## Remaining Caveats

- Economy→FIELD_POLICY link exists only in test orchestration; not production session wiring.
- Weight mapping is fixture-authored (threshold 95.0 + two proven weight profiles), not a general policy engine.
- Cached first-slice ticks defer commitment scan (no events on cached summary); dirty refresh may re-execute hot path and emit if urgency crosses threshold.

## Final Verdict

**PASS — Phase M Economy + FIELD_POLICY Product Fixture V1 landed; a discrete ResourceEconomySpec boundary result now drives an opt-in first-slice FIELD_POLICY commitment fixture through existing GPU-resident EML/Threshold+EmitEvent mechanics, while preserving abstract boundary doctrine, Resource Flow default-off posture, no DailyResolutionBoundary, no simthing-sim calendar/pause semantics, no runtime economy behavior changes, no semantic WGSL, and no atlas/default mapping wiring.**

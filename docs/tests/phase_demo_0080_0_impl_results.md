# DEMO-0080-0 Implementation Results

**Date:** 2026-06-02

**Verdict:** PASS — headless Local Patrol Economy demo/export library helper implemented.

## Scope

Implemented `DEMO-0080-0` as a headless demo/export library helper in `simthing-driver`:

- Module: `crates/simthing-driver/src/demo_0080_0.rs`
- Tests: `crates/simthing-driver/tests/demo_0080_0.rs`
- Applies canonical `Control0080CommandBatch::canonical_run()`
- Runs existing `admit_control_0080_0` → schedule → `observe_gameplay_0080_0` path
- Emits deterministic observation export + companion `MOVEMENT|` demo export section
- **No CLI binary added**

## Files touched

| File | Action |
|---|---|
| `crates/simthing-driver/src/demo_0080_0.rs` | Created |
| `crates/simthing-driver/tests/demo_0080_0.rs` | Created |
| `crates/simthing-driver/src/lib.rs` | Updated exports |
| `docs/tests/phase_demo_0080_0_impl_results.md` | Created (this report) |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated ladder status |
| `docs/gameplay/demo_0080_0_opening_spec.md` | Implementation note |
| `docs/workshop/mapping_current_guidance.md` | Status line |
| `docs/worklog.md` | Top entry |

## Explicit opt-in / canonical batch / existing path

- Default demo surface is disabled/no-op; `Demo0080Gate::explicit_opt_in()` required.
- Uses `Control0080CommandBatch::canonical_run()` (`SetStepCount(3)`, `RunObservedScenario`, `ExportTranscript`).
- Path: `run_demo_0080_0` → `admit_control_0080_0` → `run_default_schedule_0080_0` → `observe_gameplay_0080_0` → `export_gameplay_0080_text`.
- No bypass of control, schedule, observation, or FIELD_POLICY.

## Day-to-day patrol and pirate movement record

Canonical demo run (`Demo0080Input::explicit_opt_in()`), one row per schedule step:

| step | patrol start | patrol end | patrol moved? | patrol source | patrol dest | pirate start | pirate end | pirate moved? | pirate source | pirate dest | source supply | source disruption | source local_security | destination supply | destination disruption | destination local_security | threshold? | event? | boundary? | production path? |
|---:|---|---|---|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---|---|---|---|
| 0 | source | destination | yes | source | destination | destination | source | yes | destination | source | 12 | 8 | 2 | 7 | 5 | 7 | yes | yes | yes | yes |
| 1 | source | source | no | — | — | source | destination | yes | source | destination | 7 | 6 | 4 | 7 | 5 | 7 | no | no | no | no |
| 2 | source | destination | yes | source | destination | destination | source | yes | destination | source | 7 | 9 | 4 | 3 | 9 | 7 | yes | yes | yes | yes |

Movement emerges from the existing FIELD_POLICY-sourced schedule path only; demo commands do not direct-move entities.

## Deterministic replay

`replay_demo_0080_0()` produces identical `demo_export`, `observation_export`, `movement_days`, and `deterministic_replay_checksum`.

## Tests run

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test demo_0080_0` | **18/18 PASS** |
| `cargo test -p simthing-driver --test control_0080_0` | **PASS** |
| `cargo test -p simthing-driver --test gameplay_0080_0` | **PASS** |
| `cargo test -p simthing-driver --test default_schedule_0080_0` | **PASS** |
| `cargo test -p simthing-driver --test production_path_0080_0` | **PASS** |
| Mobility substrate + FIELD_POLICY regression suites | **PASS** |
| `cargo check --workspace` | **PASS** (pre-existing warnings only) |

No tests skipped.

## Scope confirmations

| Item | Added? |
|---|---|
| CLI binary | **No** (existing `simthing` bin unchanged; no demo bin) |
| Direct movement command | **No** |
| External boundary request from demo | **No** |
| Player command loop | **No** |
| UI framework | **No** |
| Real-time loop | **No** |
| Global default schedule | **No** |
| New schedule implementation | **No** |
| Semantic/raw WGSL / new shader/GPU kernel | **No** |
| CPU planner / urgency / commitment | **No** |
| Hard currency / markets / trade / `ai_budget` | **No** |
| Nested Resource Flow | **No** |
| ClauseThing implementation | **No** |
| `simthing-spec` alteration | **No** |
| Invariant edits | **No** |
| Passive proof wrapper | **No** |
| Closed-ladder reopen | **No** |

Direct movement control, player command loop, UI framework, and real-time loop remain **CLOSED**.

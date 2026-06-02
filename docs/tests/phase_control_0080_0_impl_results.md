# CONTROL-0080-0 Implementation Results

**Date:** 2026-06-02

**Verdict:** PASS — bounded Local Patrol Economy command admission implemented.

## Scope

Implemented `CONTROL-0080-0` as a bounded command-admission layer in `simthing-driver`:

- Module: `crates/simthing-driver/src/control_0080_0.rs`
- Tests: `crates/simthing-driver/tests/control_0080_0.rs`
- Accepts tiny validated command vocabulary
- Writes only existing `DefaultSchedule0080Input` bounded values/config
- Invokes existing schedule → observation path after admitted commands
- Explicit opt-in/default-off; no direct movement, boundary emission, or SEAD bypass

## Files touched

| File | Action |
|---|---|
| `crates/simthing-driver/src/control_0080_0.rs` | Created |
| `crates/simthing-driver/tests/control_0080_0.rs` | Created |
| `crates/simthing-driver/src/lib.rs` | Updated exports |
| `docs/tests/phase_control_0080_0_impl_results.md` | Created (this report) |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated ladder status |
| `docs/gameplay/control_0080_0_opening_spec.md` | Implementation note |
| `docs/workshop/mapping_current_guidance.md` | Status line |
| `docs/worklog.md` | Top entry |

## Command vocabulary implemented

- `SetSourceDisruption`, `SetDestinationDisruption`
- `SetSourceSupply`, `SetDestinationSupply`
- `SetSourceLocalSecurity`, `SetDestinationLocalSecurity`
- `SetStepCount`, `SetPatrolDisruptionReduction`
- `RunObservedScenario`, `ExportTranscript`

Rejected command types: `DirectPatrolMove`, `DirectPirateMove`, `ExternalBoundaryRequest`, `CpuPlannerOrCommitment`.

## Value-bound validation

- Negative supply rejected
- Negative local security rejected
- Negative disruption rejected
- Negative patrol disruption reduction rejected
- Step count capped at 32

## Schedule / observation path

After admitted commands: `run_default_schedule_0080_0` → `observe_gameplay_0080_0` → `export_gameplay_0080_text`. Commands never move entities directly, never emit `BoundaryRequest`, never bypass SEAD.

## Deterministic replay

`replay_admit_control_0080_0()` produces identical transcript, text export, and checksum.

## Tests run

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test control_0080_0` | **18/18 PASS** |
| `cargo test -p simthing-driver --test gameplay_0080_0` | **PASS** |
| `cargo test -p simthing-driver --test default_schedule_0080_0` | **PASS** |
| `cargo test -p simthing-driver --test production_path_0080_0` | **PASS** |
| Mobility substrate + SEAD regression suites | **PASS** |
| `cargo check --workspace` | **PASS** (pre-existing warnings only) |

No tests skipped.

## Scope confirmations

| Item | Added? |
|---|---|
| Direct movement control | **No** |
| External boundary request from commands | **No** |
| Player command loop | **No** |
| UI framework | **No** |
| Real-time loop | **No** |
| Global default schedule | **No** |
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

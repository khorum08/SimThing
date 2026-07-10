# STUDIO-SIM-CLOCK-UI-0 Results

## Status
**PROBATION / proof-present** — Studio sim clock transport UI + headless hooks over `StudioSimClock`. No live session bridge (9.3).

## Identity
| Field | Value |
|---|---|
| Rung | `STUDIO-SIM-CLOCK-UI-0` (9.2) |
| Track | `0.0.8.6-studio-live-ops` |
| ORIENT-RECEIPT | see PR body |

## What changed
| Path | Role |
|---|---|
| `crates/simthing-mapeditor/src/studio_sim_clock_ui.rs` | Headless transport façade + readout |
| `crates/simthing-mapeditor/src/lib.rs` | Module + re-exports |
| `crates/simthing-mapeditor/src/app/mod.rs` | `StudioAppState.sim_clock_transport` |
| `crates/simthing-mapeditor/src/app/ui.rs` | Left-panel "Sim clock transport" controls |
| `crates/simthing-mapeditor/tests/studio_sim_clock_ui_0.rs` | Headless CI proofs |
| design + inventory + triage | Status stamp / ledger |

## Contract
UI is a projection over `StudioSimClockTransport` → `StudioSimClock`.
Commands: Pause, Play, 1×/2×/4×, max TPS draft applied via `set_max_tps` (clock validation).
Readout: paused/playing, rate, max TPS, effective TPS, tick index.
No second scheduler. No ScenarioSpec mutation. No SimSession bridge.

## Proofs
```text
cargo test -p simthing-mapeditor --test studio_sim_clock_ui_0
4 passed (pause/play, rates, max-tps validation, readout)
```

## Scope ledger
Implemented: transport UI + headless hooks. Deferred: 9.3 live bridge, 9.5 library modal pause. No product kernel/GPU/RF.

## seal_residue_risk
none

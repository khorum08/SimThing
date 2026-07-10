# STUDIO-SIM-CLOCK-UI-0 Results

## Status
**PROBATION** — transport UI + headless hooks over landed `StudioSimClock`. Not graduated.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | #1283 |
| branch | `studio-sim-clock-ui-0` |
| base | `master` |
| merge | NOT MERGED (DA-RESERVE / PROBATION) |

## What changed
- `studio_sim_clock_ui.rs` — `StudioSimClockTransport` / commands / readout (same path as UI)
- `app/ui.rs` — left-panel Sim clock transport (Pause, Play, 1×/2×/4×, max TPS, readout)
- `app/mod.rs` — `StudioAppState.sim_clock_transport`
- Headless tests: `studio_sim_clock_ui_0.rs` (6 load-bearing proofs)
- Design 9.2 PROBATION stamp; evidence index line

## Load-bearing proofs
| test | catches |
|---|---|
| `pause_action_freezes_clock` | Pause only flips UI while clock still schedules |
| `play_action_enables_clock` | Play fails to run underlying clock |
| `rate_actions_select_landed_clock_rates` | Wrong/divergent rate mapping |
| `invalid_max_tps_preserves_last_valid_value` | Bypass validation / corrupt prior TPS |
| `clock_readout_tracks_underlying_state` | Stale/duplicated presentation readout |
| `transport_actions_do_not_mutate_scenario_spec` | Accidental model-authority path |

## Scope Ledger
| | |
|---|---|
| Specified | Transport UI + programmatic hooks over StudioSimClock |
| Implemented | As above |
| Proxied | — |
| Deferred | 9.3 live bridge; 9.5 library modal pause; live observation |
| Out of scope | Kernel/sim/GPU/RF/EML/driver; clearance class; auto-Play |

## Conformance
Studio/Bevy presentation-only · existing clock reused · ScenarioSpec unchanged · no second scheduler · invalid TPS via clock API · no bridge

## Known gaps / next
`STUDIO-LIVE-SESSION-BRIDGE-0` wires scheduled ticks into loaded session. No desktop smoke claimed unless run.

## Graduation routing
PROBATION — DA/Owner; sticky DA-RESERVE(gate-wiring) expected (ui.rs/mod.rs outside substrate class).

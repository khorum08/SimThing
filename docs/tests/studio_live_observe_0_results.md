# STUDIO-LIVE-OBSERVE-0 Results

## Status
**PROBATION** — not complete; not graduated. Awaiting Owner/DA review.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1287](https://github.com/khorum08/SimThing/pull/1287) |
| branch | `studio-live-observe-0` |
| base | `master` |
| head_sha | `2920b1d996c2afaf523e79d9a4c080d21f529b7b` |
| merge | NOT MERGED |

## What changed
- `studio_live_observe.rs` — pure `StudioLiveObservationReadout` + `build_studio_live_observation_readout` over existing clock transport readout, live bridge readout, and `StudioSession` summary
- Left-panel **Live observation** section (near sim clock / bridge) showing clock, bridge, session identity, STEAD/RF, occupied cells
- Headless proofs: `crates/simthing-mapeditor/tests/studio_live_observe_0.rs` (10 tests)
- Evidence index + design 9.4 PROBATION row + test inventory

## Load-bearing proofs
| test | catches |
|---|---|
| live_observation_updates_while_running | observation stays stale while clock/bridge execute ticks |
| live_observation_freezes_on_pause | displayed live values changing while paused |
| observation_uses_clock_and_bridge_readouts_not_bevy_frame_count | frame-count/FPS becoming observation authority |
| observation_does_not_execute_ticks | open/refresh observation causing step_once or clock advance |
| observation_does_not_mutate_scenario_spec | observation path mutating ScenarioSpec |
| session_identity_and_stead_are_visible_in_observation | losing scenario identity / STEAD summary |
| bridge_error_or_unattached_state_is_reported | silent no-op when unattached or bridge error |
| no_new_gameplay_or_workshop_dependency_for_observation | workshop import or inventing gameplay summaries |
| tree_local_summary_is_projection_only | tree summary from planner instead of session projection fields |
| observation_tracks_rate_and_max_tps_from_clock | rate/TPS readout drift from transport |

Regressions kept green: `studio_live_session_bridge_0`, `studio_sim_clock_ui_0`, `studio_sim_clock_0`.

## Scope Ledger
| | |
|---|---|
| Specified | Live observation surfaces for clock + bridge + session; update/freeze; no tick/Spec mutation |
| Implemented | Pure readout compose + UI section + headless proofs |
| Proxied | Bridge executed counters projected from existing `StudioLiveSessionBridgeReadout` (no new driver API) |
| Deferred | 9.5 scenario library / modal pause; broader tree inspector |
| Out of scope | CPU planner; gameplay/RF attach; workshop; kernel/sim/WGSL; Auto-Play; new driver door |

## Conformance
- Studio presentation-only: YES
- Existing StudioSimClock / transport reused: YES
- Existing StudioLiveSessionBridge readout reused: YES
- Observation does not execute ticks: YES
- ScenarioSpec unchanged: YES
- Values update while running / freeze on pause: YES
- No CPU planner / new gameplay / workshop import: YES
- No authority/gate/dependency widening: YES

## Known gaps / next
- Boundary count omitted (not exposed by 9.3 bridge readout)
- Source kind is `generated` / `loaded scenario` (no separate clause-shaped enum on `StudioSessionSource`)
- Next: `STUDIO-SCENARIO-LIBRARY-UI-0` (9.5)

## Graduation routing
**PROBATION** — mapeditor presentation only; recommended light posture. Do not self-mark complete.

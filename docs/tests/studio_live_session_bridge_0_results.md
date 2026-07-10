# STUDIO-LIVE-SESSION-BRIDGE-0 Results

## Status
**PROBATION** — production clock→SimSession bridge. Not graduated.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | pending |
| branch | `studio-live-session-bridge-0` |
| base | `master` |
| merge | NOT MERGED |

## What changed
- `simthing-driver::SimSession::step_once` — one production hot-cycle (shared with `run`)
- `studio_live_session_bridge.rs` — open from StudioSession authority, consume scheduled ticks
- NonSend bridge resource + Send-safe readout on `StudioAppState`
- Update system drives bridge from wall elapsed + `StudioSimClock` (not frame authority)
- UI bridge status near clock transport
- Headless tests: 8 load-bearing proofs

## Load-bearing proofs
| test | catches |
|---|---|
| play_consumes_clock_scheduled_ticks_into_live_bridge | Play only advances clock UI |
| pause_freezes_live_bridge_execution | Pause still ticks SimSession |
| loaded_json_session_multiticks_under_play | Bridge only works for generated sessions |
| loaded_clause_session_multiticks_under_play | Clause hydrate not bridgeable (same attach path) |
| session_identity_and_stead_hold_across_bounded_play | Session/STEAD identity loss |
| bridge_uses_production_session_path_not_workshop_residue | Workshop quiet import |
| bridge_does_not_mutate_scenario_spec_from_ui_transport | UI mutates Spec |
| bridge_reports_open_or_tick_errors_without_silent_fallback | Silent no-op/TP fallback |

GPU: real adapter used when present; Unsupported surfaced (no silent TP fallback).

## Scope Ledger
| | |
|---|---|
| Specified | Clock→production SimSession multi-tick; identity/STEAD hold |
| Implemented | As above |
| Proxied | Clause multi-tick uses LoadedScenario path (same as post-clause hydrate) |
| Deferred | 9.4 observe; 9.5 library; RF/GameMode gameplay attach |
| Out of scope | Workshop import; kernel/sim/WGSL; clearance class |

## Conformance
Presentation-only UI · StudioSimClock schedules · SimSession::open+step_once executes · Spec unchanged by transport · no workshop import · no second scheduler

## Known gaps / next
STUDIO-LIVE-OBSERVE-0 for broader tree-local summaries. Structural shell strips product property maps for dense registry projection (Spec authority unchanged on StudioSession).

## Graduation routing
PROBATION — DA review; authority-adjacent driver step_once + mapeditor bridge.

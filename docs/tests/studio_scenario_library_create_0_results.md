# STUDIO-SCENARIO-LIBRARY-CREATE-0 Results

## Status
**DA-GRADUATED / COMPLETE** — merged [#1291](https://github.com/khorum08/SimThing/pull/1291) @ `e42a248f`.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1291](https://github.com/khorum08/SimThing/pull/1291) |
| branch | `codex/studio-scenario-library-create-0` |
| base | `master` |
| head_sha | `1b0640688c7db460a3af13eccf4b1c9c622103aa` |
| merge | `e42a248f` |

## What changed
- Activated Scenario Library Create tab with scenario ID input and blank-create action
- Neutral one-cell structural authority builder (`blank_minimal` provenance)
- Hydrate via `StudioSession::from_loaded_scenario`; adopt + scene rebuild on success only
- `live_bridge_reset_requested` → `bridge.detach()` on successful replacement
- Modal pause / no-autoplay preserved; fail-loud atomic create
- Headless proofs: `crates/simthing-mapeditor/tests/studio_scenario_library_create_0.rs` (12 tests)

## Load-bearing proofs
| test | catches |
|---|---|
| scenario_library_create_blank_produces_loadable_session | UI-only / non-authority create |
| scenario_library_create_session_has_valid_stead_and_links | broken STEAD / links on blank shell |
| scenario_library_create_preserves_scenario_authority_boundary | serializing view/bridge/Bevy state |
| scenario_library_create_save_load_roundtrip_preserves_identity | parallel I/O path / identity loss |
| scenario_library_create_failure_leaves_prior_session_intact | mutating prior session on invalid ID |
| scenario_library_create_keeps_modal_pause | create restoring Play |
| scenario_library_create_close_does_not_autoplay | close autoplay after create |
| scenario_library_create_does_not_tick_live_bridge | create scheduling live ticks |
| scenario_library_create_replaces_deferred_create_affordance | Create still deferred/disabled |
| scenario_library_create_has_no_tp_hardcodes | TP fixture/seed defaults |
| scenario_library_create_has_no_workshop_or_gameplay_dependency | workshop / gameplay residue |
| scenario_library_create_reports_errors_without_silent_fallback | silent empty-ID success |

Regressions kept green: `studio_scenario_library_ui_0`, `studio_live_observe_0`, `studio_live_session_bridge_0`, `studio_sim_clock_ui_0`, `studio_sim_clock_0`.

## Scope Ledger
| | |
|---|---|
| Specified | Minimal blank create, loadable session, authority roundtrip, modal pause/no-autoplay, fail-loud atomic replacement |
| Implemented | One-cell structural shell + session helper + Create UI + adoption/rebuild + bridge reset request |
| Proxied | `StudioSession::from_loaded_scenario` rebuilds document, admission, projections, hydration |
| Deferred | Rich templates; live-ops clearance class (9.7); polish/hardening (9.8) |
| Out of scope | Generator templates, driver/session APIs, kernel/sim/WGSL, workshop, gameplay/RF, CPU planner, gate/class |

## Conformance
- `SimThingScenarioSpec` sole authority; presentation never serialized
- Created session loadable with valid STEAD/links/structural projection
- Existing scenario I/O save/load identity holds
- Failed create leaves prior session intact; UI adopts success only
- Successful replace requests bridge detach before next live update
- Modal pause; create/close never autoplay or execute live ticks
- No TP/workshop/generator/gameplay residue

## Known gaps / next
- Blank uses minimal World structural shell (STEAD-valid); no template selector
- JSON/Clause load paths do not yet set `live_bridge_reset_requested` (create does; consider in 9.8)
- Next: `STUDIO-LIVE-OPS-CLASS-0` (9.7)

## Graduation routing
**DA PASS** — `DA-RESERVE(gate-wiring)` from append-only harness TSVs on `GATE_WIRING_PATHS`, not router/class edits. Blank authority + hydrate + atomic adopt + bridge detach + pause law hold. Pointer → `STUDIO-LIVE-OPS-CLASS-0`.

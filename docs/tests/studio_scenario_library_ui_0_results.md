# STUDIO-SCENARIO-LIBRARY-UI-0 Results

## Status
**DA-GRADUATED / COMPLETE** — merged [#1289](https://github.com/khorum08/SimThing/pull/1289) @ `d2493dc2`.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1289](https://github.com/khorum08/SimThing/pull/1289) |
| branch | `codex/studio-scenario-library-ui-0` |
| base | `master` |
| head_sha | `bba7504d0cf3a49009b9e6b805bac644a7deac2d` |
| merge | `d2493dc2` |

## What changed
- `studio_scenario_library_ui.rs` — presentation-only `StudioScenarioLibraryModel` (JSON / Clause / deferred Create)
- Blocking egui Scenario Library modal; top-row `Library...` affordance
- Modal pause via `StudioSimClockTransport` in UI pass + live-bridge update; close/I/O never autoplay
- JSON load/save and Clause open reuse existing production scenario_io / picker paths
- Headless proofs: `crates/simthing-mapeditor/tests/studio_scenario_library_ui_0.rs` (12 tests)

## Load-bearing proofs
| test | catches |
|---|---|
| scenario_library_open_pauses_clock | open without pausing transport |
| scenario_library_visible_freezes_live_bridge_execution | bridge ticks while modal visible |
| scenario_library_close_does_not_autoplay | close restoring Play |
| scenario_library_load_uses_existing_scenario_io | parallel JSON parser / serde in library model |
| scenario_library_save_writes_scenario_authority_only | save writing view model / Bevy / telemetry |
| scenario_library_load_preserves_stead_and_session_identity | identity / STEAD loss on load |
| scenario_library_clause_open_reuses_production_ingest_path | alternate Clause ingest path |
| scenario_library_clause_open_requires_explicit_resolver_when_needed | silent TP/default resolver |
| scenario_library_create_is_deferred_to_9_6 | Create implementing blank/template early |
| scenario_library_does_not_mutate_spec_without_load_or_save | modal tab/path mutating Spec |
| scenario_library_has_no_workshop_or_gameplay_dependency | workshop / gameplay residue |
| scenario_library_status_reports_io_errors_without_silent_fallback | silent I/O failure |

Regressions kept green: `studio_live_observe_0`, `studio_live_session_bridge_0`, `studio_sim_clock_ui_0`, `studio_sim_clock_0`.

## Scope Ledger
| | |
|---|---|
| Specified | Scenario library modal, current identity/status, JSON load/save, Clause open, modal pause, deferred Create |
| Implemented | Presentation model + egui modal + existing I/O action wiring + bridge-side pause guard + focused proofs |
| Proxied | Native picker UI remains a thin caller over existing injectable production action boundaries |
| Deferred | Blank/template creation (9.6), live-ops clearance class (9.7), hardening/polish battery (9.8) |
| Out of scope | Driver/session API changes, kernel/sim/WGSL, workshop, gameplay/RF attach, CPU planner, gate/workflow/class wiring |

## Conformance
- Studio modal state is presentation-only; `StudioSession.scenario_authority` remains authority.
- JSON save calls existing authority-only save; JSON load calls existing session rebuild path.
- ClauseScript open calls the existing production picker/ingest path with explicit resolver entries.
- Visible modal pauses `StudioSimClockTransport` and produces zero bridge ticks.
- Closing, loading, saving, Clause open, and reopening do not autoplay.
- Modal tab/path interaction does not mutate ScenarioSpec.
- Create is visibly deferred to `STUDIO-SCENARIO-LIBRARY-CREATE-0`.
- No TP defaults, alternate Clause parser, workshop dependency, CPU planner, or gameplay system.

## Known gaps / next
- No blank/template creation; next rung is `STUDIO-SCENARIO-LIBRARY-CREATE-0` (9.6).
- No dedicated live-ops clearance class until 9.7.
- No desktop interaction smoke claimed; headless modal model/action boundaries and Windows Studio build are proven.

## Graduation routing
**DA PASS** — `DA-RESERVE(gate-wiring)` from append-only `scripts/ci/anchor_reach_log.tsv` (+ inventory/triage) on GATE_WIRING_PATHS; not router/class/predicate edits. Modal pause law + authority I/O boundaries hold. Pointer → `STUDIO-SCENARIO-LIBRARY-CREATE-0`.

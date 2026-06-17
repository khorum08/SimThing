# SCENARIO-SAVELOAD-UI-0 — Studio scenario save/load UI

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added SCENARIO-SAVELOAD-UI-0 PROBATION row |
| `docs/tests/scenario_saveload_ui_0_results.md` | PROBATION | This report |
| `docs/tests/scenario_saveload_io_0_results.md` | PROBATION | Backend IO unchanged |
| `docs/tests/simthing_studio_config_0_results.md` | PROBATION | Config remains separate |
| `docs/0.8.3 Simthing Studio Production.md` | PROBATION | Standing Studio production synthesis updated |

## Scenario UI summary

- Left panel **Scenario (model authority)** section with path field, **Save Scenario**, **Load Scenario**
- Default path: `simthing-current.simthing-scenario.json`
- Action helpers: `crates/simthing-mapeditor/src/app/scenario_io.rs`
- Presentation state: `scenario_path_text`, `last_scenario_io_status` on `StudioAppState`

## Config-vs-scenario separation

- Scenario UI does not write `simthing-studio-config.json`
- Path validator rejects `simthing-studio-config.json` and `settings.ron`
- Studio config save-on-settings-close and save-on-app-exit unchanged

## Save action behavior

- Requires active `StudioSession`
- Calls `save_current_session_scenario_to_path`
- Status: `Scenario saved: <path>` or `Scenario save failed: <reason>`

## Load action behavior

- Calls `load_studio_session_from_scenario_path` with current generation profile as hint
- On success: `adopt_loaded_scenario_session` + `rebuild_galaxy_scene` + camera reset
- On failure: current session preserved

## Failure behavior

- Invalid path, missing file, STEAD/link validation errors surface in status panel
- Load failure does not clobber `state.session`

## Projection rebuild proof

- Load action tests verify hydration and view model rebuild from loaded authority
- UI load path triggers `rebuild_galaxy_scene` from loaded session view model

## Runtime vertical-test horizon compatibility note

The Save Scenario / Load Scenario UI is the first user-facing path for loading model authority. Future runtime vertical-test loading must target this SimThing-Spec authority layer, then attach GPU-resident execution/readiness surfaces — not Bevy state or Studio config.

## Broader constitutional validation commands

```text
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
```

## Tests added

`crates/simthing-mapeditor/src/app/scenario_io.rs` (14 tests):

- `scenario_path_defaults_to_simthing_scenario_suffix`
- `scenario_path_rejects_or_warns_non_scenario_config_path`
- `scenario_save_ui_requires_active_session`
- `scenario_save_ui_writes_simthing_scenario_file`
- `scenario_save_ui_does_not_write_studio_config`
- `scenario_save_ui_reports_success`
- `scenario_save_ui_reports_failure_without_panicking`
- `scenario_load_ui_loads_simthing_scenario_authority`
- `scenario_load_ui_rebuilds_hydration_projection`
- `scenario_load_ui_rebuilds_view_model`
- `scenario_load_ui_preserves_current_session_on_failure`
- `scenario_load_ui_reports_success`
- `scenario_load_ui_reports_failure`
- `scenario_io_status_is_presentation_only`
- `loaded_session_authority_is_loaded_scenario_not_synthetic_output`

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo check -p simthing-core
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
git diff --check
```

## Files changed

- `crates/simthing-mapeditor/src/app/scenario_io.rs` — new UI action helpers + tests
- `crates/simthing-mapeditor/src/app/mod.rs` — `StudioAppState` fields, `adopt_loaded_scenario_session`
- `crates/simthing-mapeditor/src/app/ui.rs` — Scenario section + save/load wiring
- `crates/simthing-mapeditor/src/scenario_io.rs` — compatibility-bridge comments
- `docs/0.8.3 Simthing Studio Production.md` — SCENARIO-SAVELOAD-UI-0 section
- `docs/tests/current_evidence_index.md` — PROBATION row
- `docs/tests/scenario_saveload_ui_0_results.md` — this report

## Deferred work

- Native file dialogs for scenario path selection
- Persisting `scenario_path_text` in studio config (optional future)
- Scenario-native loaded sessions without synthetic `GenerationRunOutput`
- Runtime vertical-test loading through scenario authority import path

## DA status

**PROBATION** — not DA-promoted. Owner approval required before CURRENT_EVIDENCE.
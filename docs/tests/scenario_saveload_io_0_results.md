# SCENARIO-SAVELOAD-IO-0 — SimThing-Spec scenario file IO

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added SCENARIO-SAVELOAD-IO-0 PROBATION row |
| `docs/tests/scenario_saveload_io_0_results.md` | PROBATION | This report |
| `docs/tests/simthing_studio_config_0_results.md` | PROBATION | Unchanged; config remains separate |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Scenario authority IO summary

- Module: `crates/simthing-mapeditor/src/scenario_io.rs`
- File suffix: `.simthing-scenario.json`
- Atomic write: `<target>.simthing-scenario.json.tmp` → rename/replace destination
- Save: `save_scenario_authority_to_path`, `save_current_session_scenario_to_path`
- Load: `load_scenario_authority_from_path`, `load_studio_session_from_scenario_path`
- Serde: wraps `serialize_scenario_authority` / `deserialize_scenario_authority` from `simthing-spec`

## Config-vs-scenario separation

- `simthing-studio-config.json` remains presentation-only (`schema_version = 1`, star/hyperlane/view/camera).
- Scenario files serialize whole `SimThingScenarioSpec` only.
- Save tests prove scenario JSON does not contain studio config keys, view model fields, or Bevy render metadata.

## STEAD validation coverage

- `deserialize_scenario_authority` calls `validate_stead_mapping_consistency`.
- Load rejects invalid map container ids, missing placements, and structural mismatches.
- Tests: `scenario_authority_load_validates_stead_mapping`, `scenario_authority_load_rejects_invalid_map_container_id`.

## Link validation coverage

- Added `validate_scenario_links` in `simthing-spec`; rejects hyperlane endpoints not present in structural placements.
- Wired into `deserialize_scenario_authority`.
- Tests: `scenario_authority_load_rejects_invalid_links` (spec + mapeditor).

## ID reservation / load safety

- `deserialize_scenario_authority` calls `reserve_simthing_ids_from_scenario`.
- Tests: `scenario_authority_load_reserves_simthing_ids`, `new_simthing_after_scenario_load_does_not_collide`.

## Projection rebuild proof

- `load_studio_session_from_scenario_path` rebuilds `StudioHydrationBoundary` and `StudioGalaxyViewModel` from loaded authority.
- `StudioSession::from_scenario_authority` added for non-generation load path.
- Tests: `loaded_scenario_rebuilds_studio_hydration_boundary`, `loaded_scenario_rebuilds_studio_view_model`, `loaded_scenario_projection_uses_structural_coords_not_render_coords`.

## Runtime vertical-test horizon compatibility note

Future runtime vertical-test loading must enter through SimThing-Spec scenario/runtime authority, then Studio projection, then GPU-resident execution/readiness surfaces — not view state, Bevy entities, or `simthing-studio-config.json`. Documented in `docs/design_0_0_8_3_studio_production.md` § SCENARIO-SAVELOAD-IO-0.

## Tests added

`crates/simthing-spec/src/spec/scenario.rs`:

- `scenario_authority_load_rejects_invalid_links`

`crates/simthing-mapeditor/src/scenario_io.rs` (20 tests):

- `scenario_authority_saves_whole_simthing_scenario_spec`
- `scenario_authority_save_does_not_write_studio_config`
- `scenario_authority_save_does_not_write_view_model`
- `scenario_authority_save_does_not_write_bevy_render_metadata`
- `scenario_authority_load_roundtrip_preserves_root_tree`
- `scenario_authority_load_roundtrip_preserves_structural_grid`
- `scenario_authority_load_roundtrip_preserves_map_container_binding`
- `scenario_authority_load_roundtrip_preserves_links`
- `scenario_authority_load_roundtrip_preserves_provenance`
- `scenario_authority_load_roundtrip_preserves_gridcell_children`
- `scenario_authority_load_validates_stead_mapping`
- `scenario_authority_load_rejects_invalid_map_container_id`
- `scenario_authority_load_rejects_invalid_links`
- `scenario_authority_load_reserves_simthing_ids`
- `new_simthing_after_scenario_load_does_not_collide`
- `loaded_scenario_rebuilds_studio_hydration_boundary`
- `loaded_scenario_rebuilds_studio_view_model`
- `loaded_scenario_projection_uses_structural_coords_not_render_coords`
- `studio_config_remains_presentation_only_after_scenario_io`
- `model_edit_then_save_preserves_authority_roundtrip`

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo check -p simthing-core
cargo test -p simthing-core
git diff --check
```

## Files changed

- `crates/simthing-spec/src/spec/scenario.rs` — `validate_scenario_links`, link validation in deserialize
- `crates/simthing-spec/src/spec/mod.rs`, `crates/simthing-spec/src/lib.rs` — exports
- `crates/simthing-mapeditor/src/scenario_io.rs` — new file IO layer + tests
- `crates/simthing-mapeditor/src/session.rs` — `from_scenario_authority`
- `crates/simthing-mapeditor/src/lib.rs` — module + re-exports
- `docs/design_0_0_8_3_studio_production.md` — SCENARIO-SAVELOAD-IO-0 section
- `docs/tests/current_evidence_index.md` — PROBATION row
- `docs/tests/scenario_saveload_io_0_results.md` — this report

## Deferred work

- Save Scenario / Load Scenario UI hooks (backend IO complete).
- Runtime vertical-test loading.
- Platform-specific scenario file directories.
- Native file dialogs.

## DA status

**PROBATION** — not DA-promoted. Owner approval required before CURRENT_EVIDENCE.
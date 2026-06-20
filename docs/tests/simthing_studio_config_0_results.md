# SIMTHING-STUDIO-CONFIG-0 — Studio config persistence

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added SIMTHING-STUDIO-CONFIG-0 PROBATION row |
| `docs/tests/simthing_studio_config_0_results.md` | PROBATION | This report |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Space-free alias synced |

## Config schema

`SimThingStudioConfig` (`schema_version = 1`):

- `settings_dialog`: visible flag + position
- `star_rendering`: falloff + render mode
- `hyperlane_rendering`: thickness/opacity falloff controls
- `view`: show_stars, show_hyperlanes, view_mode
- `camera`: optional orbit snapshot

File: `simthing-studio-config.json` in the current working directory.

## Load path

Startup calls `SimThingStudioConfig::load_at_startup()`. Missing file → defaults. Malformed JSON or unsupported schema → defaults + status warning. Valid file → validate once, apply to `EditorSettings` and `StudioAppState`, apply camera/view on scene setup.

## Validation policy

- Malformed JSON / unsupported `schema_version`: reject entire file, use defaults.
- NaN/Inf in numeric fields: reject entire file.
- Unrecognized `render_mode` or non-finite dialog position: reject entire file.
- Ordinary out-of-range star/hyperlane values: clamp to accepted bounds and record warning.

## Save triggers

- Settings window close via top-right X (shared close path).
- Settings window bottom Close button.
- App exit (`persist_settings_on_exit` also writes JSON).

Atomic write: `simthing-studio-config.json.tmp` → rename/replace destination.

## Reset behavior

Settings window Reset restores default star/hyperlane render values and dialog-controlled settings. Live render meta updates when a session is loaded. Reset does not modify `SimThingScenarioSpec` or generated galaxy data.

## Scenario authority separation

`simthing-studio-config.json` does not contain `SimThingScenarioSpec`, `root`, `structural_grid`, `placements`, or `scenario_id`. Scenario/model save/load remains a separate future rung.

## Future vertical-test compatibility note

Runtime vertical-test loading must synthesize or load SimThing-Spec scenario authority separately. Studio config only controls presentation preferences around that authority.

## Tests added

All in `crates/simthing-mapeditor/src/studio_config.rs`:

- `studio_config_defaults_are_valid`
- `studio_config_serializes_to_json`
- `studio_config_deserializes_from_json`
- `studio_config_roundtrip_preserves_star_settings`
- `studio_config_roundtrip_preserves_hyperlane_settings`
- `studio_config_roundtrip_preserves_settings_dialog_state`
- `studio_config_rejects_malformed_json`
- `studio_config_rejects_unsupported_schema_version`
- `studio_config_rejects_nan_or_infinite_values`
- `studio_config_clamps_or_rejects_out_of_range_values_according_to_policy`
- `startup_missing_config_uses_defaults`
- `startup_valid_config_applies_settings`
- `startup_invalid_config_uses_defaults_and_records_warning`
- `settings_x_close_saves_config`
- `settings_bottom_close_saves_config`
- `app_exit_saves_config`
- `settings_reset_restores_defaults`
- `settings_reset_does_not_modify_scenario_authority`
- `studio_config_does_not_serialize_simthing_scenario_authority`
- `studio_config_does_not_serialize_structural_grid`

## Commands run

- `cargo fmt --all`
- `cargo fmt --all -- --check`
- `cargo check -p simthing-mapeditor`
- `cargo test -p simthing-mapeditor`
- `cargo check -p simthing-spec`
- `cargo test -p simthing-spec`
- `cargo check -p simthing-core`
- `cargo test -p simthing-core`
- `git diff --check`

## Files changed

- `crates/simthing-mapeditor/src/studio_config.rs` (new)
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/src/settings.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/window.rs`
- `crates/simthing-mapeditor/src/app/camera.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/simthing_studio_config_0_results.md`

## Deferred work

Scenario/model save/load file IO, runtime vertical-test loading, platform app-data config directory, consolidating legacy RON `settings.ron` with JSON presentation config.

## DA status

**PROBATION** — not DA-approved. Owner sign-off required before CURRENT_EVIDENCE promotion.
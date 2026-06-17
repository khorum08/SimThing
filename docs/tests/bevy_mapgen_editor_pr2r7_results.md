# BEVY-MAPGEN-EDITOR-PR2R7 - live Settings dialog for star blur/falloff controls

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Owner feedback addressed

After PR2R6, the star presentation still needed visual iteration. This rung stops hardcoding every blur and
falloff correction and exposes live Studio controls so the owner can tune the view directly.

## Settings dialog behavior

- A small `⚙` gear-icon button appears with the existing top window controls.
- Tooltip text is `Settings`.
- Clicking `⚙` toggles the in-app Settings dialog; hidden dialogs keep their values and position.
- The dialog uses the same translucent dark-blue/frosted-glass frame as the main Studio panel.
- The title bar text is `Settings`.
- Top-right `X` and bottom `Close` both hide the dialog without destroying settings.
- Reopening restores the same values and in-session position. Values, visibility, and position also persist
  in the existing Studio settings RON.

## Drag/bounds behavior

- Drag starts only from the Settings title bar.
- The pure `clamp_dialog_rect_away_from_panels` helper clamps the dialog to the app viewport.
- When the main left control panel is visible, the dialog is clamped to the central area to the right of it.
- When the right status panel is visible, the dialog is clamped to the central area to the left of it.
- The same clamp is applied on resize before drawing, so the dialog remains visible and usable.

## Star render controls

The first group is `Star rendering` and exposes:

- `Base Star Blur Radius`
- `Falloff Distance`
- `Falloff Star Blur Radius`
- `Falloff Star Opacity`

`compute_star_falloff_visual(camera_depth_percent, settings)` uses 0% as near-camera and 100% as the far
horizon. At exactly `Falloff Distance`, blur reaches `Base Star Blur Radius * Falloff Star Blur Radius / 100`
and opacity reaches `Falloff Star Opacity / 100`. Beyond the falloff distance, blur and opacity continue a
gentle 1.0-to-0.75 taper toward the horizon.

## Live-update behavior

- Slider changes mutate Studio render metadata and the existing Bevy star material/transform update path.
- No galaxy regeneration, app restart, or MapGenerator rerun is required.
- Existing selection and highlight state remain valid.
- Hyperlane endpoints and stars still derive from shared PR2R4 render anchors.
- Star blur radius and opacity are render/editor metadata only.
- Structural gridcell coordinates remain authoritative.
- No MapGenerator output, topology, hyperlane graph, SimThing runtime semantics, future authoritative
  simulation fields, pathfinding, save/load, or live simulation behavior changed.

## Tests added or updated

- `settings_dialog_defaults_exist`
- `settings_dialog_open_close_preserves_values`
- `settings_dialog_close_icon_hides_dialog`
- `settings_dialog_close_button_hides_dialog`
- `settings_dialog_drag_clamps_to_viewport`
- `settings_dialog_drag_stops_at_left_panel_bounds`
- `settings_dialog_drag_stops_at_right_panel_bounds`
- `base_star_blur_radius_updates_render_meta`
- `falloff_distance_percent_updates_render_meta`
- `falloff_star_blur_radius_percent_updates_render_meta`
- `falloff_star_opacity_percent_updates_render_meta`
- `compute_star_falloff_visual_reaches_target_radius_at_falloff_distance`
- `compute_star_falloff_visual_reaches_target_opacity_at_falloff_distance`
- `settings_changes_do_not_regenerate_galaxy`
- `settings_changes_preserve_render_anchor_count`
- `settings_changes_preserve_hyperlane_anchor_coherence`
- `settings_persist_star_render_controls`

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo build -p simthing-mapeditor --bin simthing-studio
target\debug\simthing-studio.exe launch smoke
git diff --check
git diff --name-only master...HEAD
```

## Manual Windows check

This pass was implemented on Windows and validated with automated unit/model tests plus launch smoke. No
interactive DA visual approval is pre-filed.

| Check | Status |
|---|---|
| `⚙` gear icon/button appears and is visually consistent | Covered by implementation; pending DA visual confirmation |
| `⚙` tooltip says `Settings` | Implemented and covered by `settings_button_is_gear_icon_with_settings_tooltip`; pending DA visual confirmation |
| `⚙` opens Settings dialog | Implemented; pending DA visual confirmation |
| Settings dialog has frosted-glass style and title `Settings` | Implemented; pending DA visual confirmation |
| Dragging title bar moves dialog | Pure bounds tests PASS; pending DA visual confirmation |
| Dialog stops at left panel and right info panel bounds | Pure bounds tests PASS |
| Top-right close icon hides dialog | Model test PASS; pending DA visual confirmation |
| Bottom Close button hides dialog | Model test PASS; pending DA visual confirmation |
| Reopening restores values and position | Model/persistence tests PASS |
| Base Star Blur Radius changes star aura radius live | Render-meta test PASS; pending DA visual confirmation |
| Falloff Distance changes where falloff occurs live | Render-meta test PASS; pending DA visual confirmation |
| Falloff Star Blur Radius reaches configured percent at falloff distance | Helper test PASS |
| Falloff Star Opacity reaches configured opacity at falloff distance | Helper test PASS |
| Galaxy does not regenerate when settings change | Model test PASS |
| Stars/hyperlanes remain attached/coherent | Anchor-coherence tests PASS |
| Disc/Elliptical/Spiral generation still works | Existing editor generation regression test PASS |

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with PR2R6 and PR2R7 merge provenance |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r3_results.md` | PROBATION | Previous distance attenuation repair |
| `docs/tests/bevy_mapgen_editor_pr2r4_results.md` | PROBATION | Previous render-anchor coherence repair |
| `docs/tests/bevy_mapgen_editor_pr2r5_results.md` | PROBATION | Previous aura/distant-falloff tuning |
| `docs/tests/bevy_mapgen_editor_pr2r6_results.md` | PROBATION | Previous aura cap and horizon-falloff tuning |
| `docs/tests/bevy_mapgen_editor_pr2r7_results.md` | PROBATION | This report |

## Files changed

- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/window.rs`
- `crates/simthing-mapeditor/src/dialog.rs`
- `crates/simthing-mapeditor/src/panel_layout.rs`
- `crates/simthing-mapeditor/src/settings.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r7_results.md`

## Deferred features

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Hyperlane selection and edit workflows
- Shader-native uniform path if future polish needs GPU-side falloff rather than CPU-prepared material inputs
- DA-approved promotion from PROBATION to CURRENT_EVIDENCE

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

# BEVY-MAPGEN-EDITOR-PR2R9 - Base Star Blur Radius controls billboard and circle radius

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Owner screenshots / observed defect

Owner compared the same Studio session with `Base Star Blur Radius = 0.80` and `Base Star Blur Radius = 0.11`.
The rendered stars looked effectively unchanged. The intended control was live and persisted, but it did not
visibly govern the actual rendered star footprint.

## Diagnosis

- The Settings slider was saved and copied into `StudioGalaxyRenderMeta`.
- The value reached `compute_star_falloff_visual` and changed the helper's blur radius.
- The visible Bevy transform path still multiplied the small helper radius by each star's larger base
  generated scale, while the core star layer was sized independently from `Base Star Blur Radius`.
- Result: changing the slider affected stored metadata and some aura math, but the visible footprint was
  dominated by fixed star/base scale and the core layer. Low values did not produce a clearly smaller star.

PR2R9 fixes this by making the falloff radius helper feed the actual core/aura/circle radius used by Bevy
transforms.

## Implemented radius control path

- Added `StarRenderMode` with `Bloom / Starburst` and `Crisp Circle`.
- Added `StarRadiusVisual` and `compute_star_radius_visual`.
- `Base Star Blur Radius` controls near/max computed radius in both render modes.
- `Falloff Star Blur Radius` controls radius at the falloff distance:
  `radius = Base Star Blur Radius * Falloff Star Blur Radius / 100`.
- `Falloff Star Opacity` controls opacity/luminosity at falloff distance.
- `compute_star_distance_visual` now uses the computed radius for the rendered core/aura footprint.
- `sync_star_visuals_system` applies those computed radii directly through Bevy transform scale and updates
  material textures live.

## Crisp Circle mode behavior

- The Settings dialog exposes `Render mode`.
- `Bloom / Starburst` keeps the existing starburst core and aura textures.
- `Crisp Circle` swaps the core layer to a generated crisp circle texture.
- In `Crisp Circle`, the aura layer is hidden and the circle radius is governed by the same base/falloff
  radius controls.
- Selected and hovered stars keep radius/emphasis multipliers.
- Circles remain anchored to `StudioSystemRenderAnchor`.

## Live-update behavior

- Slider and mode changes update `StudioAppState`, persistent `EditorSettings`, the active
  `StudioGalaxyRenderMeta`, and Bevy material/transform state.
- No Generate call, app restart, entity identity loss, or MapGenerator rerun is required.
- Existing selection state remains view state and is not cleared by render settings changes.
- Hyperlane meshes are not rebuilt for radius-only settings changes; endpoints remain attached to shared
  render anchors.

## Structural-authority audit

- `Base Star Blur Radius` is presentation-only render metadata.
- Billboard/circle radius is not structural authority.
- Star radius, blur, aura, opacity, luminosity, render mode, and Bevy transforms are editor presentation
  metadata only.
- Hyperlane endpoints still use shared render anchors.
- Structural gridcell coordinates remain authoritative.
- No MapGenerator/topology/runtime/simulation authority changed.
- No `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`, `simthing-clausething`, or
  `simthing-mapgenerator/src` production code changed.
- No simulation, pathfinding, route, predecessor, movement-order, new-kind, or semantic GPU behavior was
  added.

## Tests added or updated

- `base_star_blur_radius_changes_computed_billboard_radius`
- `base_star_blur_radius_changes_computed_crisp_circle_radius`
- `falloff_star_blur_radius_reaches_expected_radius_at_falloff_distance`
- `falloff_star_opacity_reaches_expected_opacity_at_falloff_distance`
- `changing_base_star_blur_radius_marks_star_visuals_dirty`
- `settings_change_updates_star_visual_without_regenerating_galaxy`
- `crisp_circle_mode_uses_shared_render_anchor`
- `bloom_mode_uses_shared_render_anchor`
- `selected_star_radius_or_emphasis_exceeds_unselected`
- `hovered_star_radius_or_emphasis_exceeds_unhovered`
- `hyperlane_endpoints_remain_attached_to_render_anchors_after_radius_change`
- `star_render_mode_persists_if_settings_persistence_is_implemented`
- `settings_dialog_preserves_star_render_mode`
- `crisp_circle_texture_has_opaque_center_and_transparent_corner`

Existing guard coverage preserved:

- `shape_param_scoping_unchanged`
- `settings_changes_do_not_regenerate_galaxy`
- `settings_changes_preserve_render_anchor_count`
- `settings_changes_preserve_hyperlane_anchor_coherence`
- `camera_relative_lane_fade_still_present`

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

This pass was implemented on Windows. Automated model tests prove the radius and mode behavior; launch smoke
proves the Studio executable starts. Interactive DA visual confirmation is still pending.

| Check | Status |
|---|---|
| Set Base Star Blur Radius high and stars should grow | Helper/model tests PASS; pending DA visual confirmation |
| Set Base Star Blur Radius low and stars should shrink | Helper/model tests PASS; pending DA visual confirmation |
| Switch to Crisp Circle mode | Implemented and persisted; pending DA visual confirmation |
| Crisp Circle radius follows Base Star Blur Radius | Helper/model tests PASS |
| Falloff Distance shifts falloff depth | Existing helper tests PASS |
| Falloff Star Blur Radius reaches configured radius | Helper test PASS |
| Falloff Star Opacity reaches configured opacity | Helper test PASS |
| No Generate call when sliders/mode change | Model tests PASS |
| Hyperlanes remain attached to stars | Anchor tests PASS |
| Disc/Elliptical/Spiral generation still works | Existing generation regression tests PASS |
| Shape-param scoping remains fixed | Existing generation regression tests PASS |
| Studio executable starts | Launch smoke PASS |

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added PR2R9 PROBATION row |
| `docs/tests/bevy_mapgen_editor_pr2r7_results.md` | PROBATION | Previous live settings dialog |
| `docs/tests/bevy_mapgen_editor_pr2r8_results.md` | PROBATION | Previous stable billboard-instance pass |
| `docs/tests/bevy_mapgen_editor_pr2r9_results.md` | PROBATION | This report |
| Screenshots | Not retained | No new screenshot evidence kept |

## Files changed

- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/window.rs`
- `crates/simthing-mapeditor/src/dialog.rs`
- `crates/simthing-mapeditor/src/settings.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/starburst.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r9_results.md`

## Deferred features

- DA visual approval and promotion from PROBATION to CURRENT_EVIDENCE
- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Shader-native custom star material if later polish needs GPU-side radial falloff beyond Bevy material inputs

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

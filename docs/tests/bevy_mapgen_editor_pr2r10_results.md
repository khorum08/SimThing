# BEVY-MAPGEN-EDITOR-PR2R10 - Settings close and live hyperlane rendering controls

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added PR2R10 PROBATION row |
| `docs/tests/bevy_mapgen_editor_pr2r8_results.md` | PROBATION | Previous stable billboard-instance pass |
| `docs/tests/bevy_mapgen_editor_pr2r9_results.md` | PROBATION | Previous star radius and crisp-circle pass |
| `docs/tests/bevy_mapgen_editor_pr2r10_results.md` | PROBATION | This report |
| Screenshots | Not retained | No new screenshot evidence kept |

## Owner feedback addressed

Owner requested:

- Top-right Settings `X` should hide the dialog like the bottom `Close` button.
- Settings should expose live hyperlane render sliders after a separator:
  `Base Hyperlane Line Thickness`, `Base Hyperlane Opacity`, `Falloff Distance`,
  `Falloff Thickness`, and `Falloff Opacity`.

## Settings X close bug diagnosis/fix

- The dialog model already had separate `close_icon` and `close_button` entry points.
- PR2R10 makes both paths call the same shared model `close()` helper and the same UI persistence helper.
- Closing hides the dialog; it does not clear star settings, hyperlane settings, render mode, or dialog
  position.
- Reopening restores position and current slider values from `StudioAppState` / persisted Studio RON state.

## Hyperlane settings UI behavior

- The Settings dialog now has a separator between `Star rendering` and `Hyperlane rendering`.
- Hyperlane sliders update `StudioAppState`, `SettingsDialogModel`, persisted `EditorSettings`, and the active
  `StudioGalaxyRenderMeta`.
- Changes update render metadata live and do not call Generate, rerun MapGenerator, or rebuild generated
  topology.
- `Base Hyperlane Opacity = 0%` hides hyperlanes visually only. It does not delete or mutate generated
  hyperlane data.

## Hyperlane visual math

- `Base Hyperlane Line Thickness` is a percentage of the rendered nearest-camera star disc width.
- The accepted slider range is 1%-25%; computed base thickness is capped at 25% of nearest-camera star disc
  width.
- Minimum nonzero thickness is clamped to a small legible world-space value when opacity is nonzero.
- `Falloff Distance` is a percent of the camera far horizon where target values are reached.
- At that distance:
  - `effective_thickness = base_thickness * Falloff Thickness / 100`
  - `effective_opacity = base_opacity * Falloff Opacity / 100`
- After the falloff distance, PR2R10 clamps at the falloff target values rather than continuing to taper.

## Soft-edge gradient status

Implemented with visual-only triangle strip lane meshes:

- Central 80% of the line thickness is the core.
- Core vertices use the current effective opacity.
- Outer 10% on each side uses vertex alpha that falls to 0%.
- This is Bevy presentation geometry/material data only, not semantic WGSL and not simulation authority.

## Live-update behavior

- Hyperlane bucket meshes are rebuilt per frame from unchanged `HyperlaneRenderSegment` data and shared
  `StudioSystemRenderAnchor` endpoints.
- Existing selected star state and incident-lane highlight data remain valid.
- Hyperlane endpoints remain tied to shared render anchors.
- Hyperlane thickness and opacity are render/editor metadata only.
- Star radius and hyperlane thickness do not alter structural authority.
- Structural gridcell coordinates remain authoritative.
- No MapGenerator output, topology, generated hyperlane graph, SimThing runtime semantics, save/load
  behavior, or future authoritative simulation fields changed.

## Tests added or updated

- `settings_dialog_close_icon_hides_dialog`
- `settings_dialog_close_button_hides_dialog`
- `settings_dialog_bottom_close_hides_dialog`
- `settings_dialog_close_paths_preserve_values`
- `settings_dialog_close_paths_preserve_star_values`
- `settings_dialog_close_paths_preserve_hyperlane_values`
- `settings_dialog_reopen_restores_position_and_values`
- `hyperlane_settings_defaults_exist`
- `base_hyperlane_thickness_updates_render_meta`
- `base_hyperlane_opacity_updates_render_meta`
- `hyperlane_falloff_distance_updates_render_meta`
- `hyperlane_falloff_thickness_updates_render_meta`
- `hyperlane_falloff_opacity_updates_render_meta`
- `base_hyperlane_opacity_zero_hides_lanes`
- `base_hyperlane_opacity_nonzero_keeps_lanes_visible`
- `base_hyperlane_thickness_minimum_is_legible_nonzero`
- `base_hyperlane_thickness_max_is_at_most_25_percent_of_star_disc`
- `hyperlane_visual_reaches_falloff_thickness_at_falloff_distance`
- `hyperlane_visual_reaches_falloff_opacity_at_falloff_distance`
- `hyperlane_visual_core_fraction_is_80_percent`
- `hyperlane_visual_edge_falloff_is_10_percent_each_side`
- `hyperlane_settings_change_does_not_regenerate_galaxy`
- `hyperlane_settings_change_preserves_anchor_coherence`
- `hyperlane_settings_persist_roundtrip`
- `hyperlane_visual_strip_uses_transparent_edges_and_opaque_core`

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

This pass was implemented on Windows and includes automated model tests plus hidden executable launch smoke.
Interactive DA visual confirmation remains pending.

| Check | Status |
|---|---|
| Gear opens Settings | Existing UI path preserved; pending DA visual confirmation |
| X icon hides Settings like bottom Close | Model/helper tests PASS |
| Closing and reopening preserves settings values | Model tests PASS |
| Separator appears between Star rendering and Hyperlane rendering groups | Implemented; pending DA visual confirmation |
| Base Hyperlane Line Thickness changes lane thickness live | Render helper/model tests PASS; pending DA visual confirmation |
| Minimum thickness remains thin but nonzero | Helper test PASS |
| Maximum thickness is no more than 25% of nearest-camera star disc width | Helper test PASS |
| Base Hyperlane Opacity changes lane opacity live | Helper/model tests PASS |
| Opacity 0 hides all hyperlanes visually | Helper and visibility path tests PASS |
| Falloff Distance shifts the depth threshold | Helper tests PASS |
| Falloff Thickness changes thickness at falloff threshold | Helper test PASS |
| Falloff Opacity changes opacity at falloff threshold | Helper test PASS |
| Thick lines have soft edges | Vertex-alpha strip test PASS |
| Galaxy does not regenerate when sliders move | Model test PASS |
| Stars and hyperlanes remain attached/coherent | Anchor tests PASS |
| Disc/Elliptical/Spiral generation still works | Existing generation regression tests PASS |
| Studio executable starts | Launch smoke PASS |

## Files changed

- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/window.rs`
- `crates/simthing-mapeditor/src/dialog.rs`
- `crates/simthing-mapeditor/src/hyperlane_buckets.rs`
- `crates/simthing-mapeditor/src/settings.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r10_results.md`

## Deferred features

- DA visual approval and promotion from PROBATION to CURRENT_EVIDENCE
- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Custom shader material if later polish needs stronger camera-facing or screen-space line treatment

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

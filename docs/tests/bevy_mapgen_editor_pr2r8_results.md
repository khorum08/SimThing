# BEVY-MAPGEN-EDITOR-PR2R8 - instanced star billboard renderer with runtime size/opacity response

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Owner feedback addressed

PR2R7 exposed live star blur/falloff settings, but the rendered stars still used legacy spawn data that was
too easy to treat as a one-time visual artifact. PR2R8 makes stars explicit persistent billboard instances
bound to generated systems and shared render anchors, so settings changes affect only presentation and do not
rerun generation.

## Billboard-vs-particle decision

- Stars are stable generated-system billboards, not particles.
- Each billboard instance is prepared from the typed `StudioStarView` plus the matching
  `StudioSystemRenderAnchor`.
- The instance stores `system_id`, structural col/row reference, anchor position, base scale variation, and
  base intensity variation.
- Selection and hover are live Studio view state layered over the instance; they do not mutate the
  view model or the generated galaxy.

## Star billboard renderer design

- `prepare_star_billboard_instances` creates one instance per generated star when an anchor exists.
- The Bevy scene spawns aura and core quads from those instances and keeps the `GalaxyStar` component pointed
  at the stable instance data.
- `sync_star_visuals_system` reads the current camera position, live render settings, selected system, and
  hovered system each frame.
- Camera-relative depth is normalized by `normalized_billboard_camera_depth_percent`.
- `compute_star_distance_visual` maps depth + live settings to core scale, aura radius, core alpha,
  aura alpha, and luminosity.
- Bevy material color/emissive and transform scale are the runtime renderer inputs. These are
  presentation/uniform-backed material values, not SimThing simulation fields.

## Runtime settings behavior

- Base star blur radius changes the maximum aura radius.
- Falloff distance changes where the radius/opacity target is reached.
- Falloff star blur-radius percent changes the target aura radius at the falloff point.
- Falloff star opacity percent changes star luminosity/alpha at the falloff point.
- Settings changes update render metadata and Bevy material/transform state in place.
- No galaxy regeneration, app restart, MapGenerator rerun, or topology rebuild is required for settings-only
  changes.

## Structural-authority audit

- Structural gridcell coordinates remain authoritative.
- `StudioSystemRenderAnchor` is the shared render-only bridge for stars, picking, base hyperlane endpoints,
  and selected incident-lane highlights.
- Hyperlane endpoints and star visuals consume the same anchors.
- Bevy transforms, star billboard size, aura radius, opacity, luminosity, render height, and camera-depth
  response are render-only editor metadata.
- No runtime, GPU semantic authority, `simthing-spec`, driver, ClauseThing lowering, MapGenerator topology,
  pathfinding, save/load, live simulation, or producer behavior changed.

## Tests added or updated

- `star_billboard_instance_count_matches_system_count`
- `star_billboard_instances_use_render_anchors`
- `star_billboard_anchor_preserves_structural_coord_reference`
- `legacy_star_render_instances_wrap_billboard_instances`
- `star_distance_visual_far_is_smaller_than_near`
- `star_distance_visual_near_peak_luminosity_preserved`
- `star_distance_visual_reaches_settings_falloff_radius_at_falloff_distance`
- `star_distance_visual_reaches_settings_falloff_opacity_at_falloff_distance`
- `selected_star_visual_is_larger_or_brighter_than_unselected`
- `hovered_star_visual_is_larger_or_brighter_than_unhovered`
- `settings_update_changes_star_visual_without_regenerating_galaxy`

Existing guard coverage preserved:

- `shape_param_scoping_unchanged`
- `settings_changes_do_not_regenerate_galaxy`
- `settings_changes_preserve_render_anchor_count`
- `settings_changes_preserve_hyperlane_anchor_coherence`
- `hyperlane_anchor_coherence_unchanged`
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

This pass was implemented on Windows and validated with automated unit/model tests plus hidden launch
smoke. No interactive DA visual approval is pre-filed.

| Check | Status |
|---|---|
| Stars spawn as stable billboard instances, not particles | Model tests PASS |
| Billboard count matches generated system count | Model test PASS |
| Billboard positions use render anchors | Model test PASS |
| Billboard structural col/row references remain tied to anchors | Model test PASS |
| Runtime falloff settings change aura radius and luminosity | Helper/model tests PASS |
| Settings changes do not regenerate the galaxy | Model test PASS |
| Stars/hyperlanes/picking/highlight remain anchor-coherent | Existing anchor tests PASS |
| Near stars remain larger/brighter than far stars | Helper tests PASS |
| Selected/hovered stars remain visually emphasized | Helper tests PASS |
| Studio executable starts without Bevy ECS query panic | Launch smoke PASS |
| Owner visual confirmation of star/shading appearance | Pending DA visual confirmation |

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added PR2R8 PROBATION row |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r3_results.md` | PROBATION | Previous distance attenuation repair |
| `docs/tests/bevy_mapgen_editor_pr2r4_results.md` | PROBATION | Previous render-anchor coherence repair |
| `docs/tests/bevy_mapgen_editor_pr2r5_results.md` | PROBATION | Previous aura/distant-falloff tuning |
| `docs/tests/bevy_mapgen_editor_pr2r6_results.md` | PROBATION | Previous aura cap and horizon-falloff tuning |
| `docs/tests/bevy_mapgen_editor_pr2r7_results.md` | PROBATION | Previous live settings dialog |
| `docs/tests/bevy_mapgen_editor_pr2r8_results.md` | PROBATION | This report |

## Files changed

- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r8_results.md`

## Deferred features

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Hyperlane selection and edit workflows
- Shader-native custom pipeline if future polish needs GPU-side star falloff beyond Bevy material uniforms
- DA-approved promotion from PROBATION to CURRENT_EVIDENCE

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

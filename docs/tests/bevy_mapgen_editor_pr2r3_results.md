# BEVY-MAPGEN-EDITOR-PR2R3 — distance-attenuated stars and camera-depth lanes

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with PR2R3 probation row |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r2_results.md` | PROBATION | Previous visibility repair; now records PR #730/merge provenance in the ledger |
| `docs/tests/bevy_mapgen_editor_pr2r3_results.md` | PROBATION | This report |

## Observed Windows defects

DA live-run feedback after PR2R2:

- Stars were visible but far too large and bright at overview scale, merging dense regions into a cyan glow/cloud.
- Foreground/near hyperlanes still did not clearly read as lighter blue than camera-far lanes.

## Star shader/material diagnosis

The prior Studio renderer used one additive starburst quad per star. Scale and emissive were selected/hovered
aware, but they were not camera-distance attenuated and the texture still carried too much overview aura.
That made every far star render like a large glow sprite instead of a point.

## New distance attenuation model

- Added `StarDistanceVisual` and `star_distance_visual(camera_distance, selected, hovered, meta)`.
- Added render metadata for near/far star distance, core scale, aura scale, core alpha, aura alpha, and
  selected/hovered scale multipliers.
- Split star sprites into two Bevy render-only layers: `StarVisualLayer::Core` and `StarVisualLayer::Aura`.
- Per-frame star sync now reads the camera transform and applies distance attenuation to each layer.
- Far stars use a small core scale and near-zero aura alpha. Close/selected stars can bloom locally without
  forcing all overview stars into large cyan quads.

## Aura reduction behavior

- The core texture now uses a steeper radial falloff.
- The aura texture is separate, softer, and multiplied by the distance visual's aura alpha.
- Far aura alpha defaults to `0.008`; selected/hovered boosts are clamped to the configured near aura max.
- Dense overview fields should read as many crisp stars rather than one merged glow mass.

## Hyperlane camera-depth model

PR2R2's camera-relative hyperlane rebucketing remains in force:

- Each frame, lane segments are classified by camera-to-segment-midpoint distance.
- Near lanes use light blue and higher alpha.
- Mid lanes use muted blue and medium alpha.
- Far lanes use dark grey-blue and low alpha, clamped to a faint legible minimum.
- Selected incident lanes render in a separate visual-only overlay and override the base depth fade.

## Render-only authority note

- Structural gridcell coordinates remain authoritative.
- Bevy transforms are render-only.
- Star size, star aura, bloom, camera-depth alpha, and hyperlane color are render/editor metadata only.
- No runtime simulation, no pathfinding, no route/predecessor semantics, no semantic GPU.

## Shape-param scoping regression status

Shape-param scoping remains fixed:

- Elliptical generation does not submit spiral-only `arm_*` params.
- Disc generation does not submit spiral-only `arm_*` params.
- Spiral generation still submits spiral params.
- Dormant params remain visible as editor state but are not submitted for inactive shapes.
- Dormant params do not block generation.
- CLI/generator fail-closed validation remains intact for invalid submitted params.

## Tests added

- `star_distance_visual_far_is_small_point`
- `star_distance_visual_far_aura_is_minimal`
- `star_distance_visual_near_is_larger_than_far`
- `star_distance_visual_selected_is_larger_or_brighter_than_unselected`
- `star_distance_visual_aura_never_exceeds_configured_max`
- `star_render_preparation_count_matches_system_count`
- `star_visual_metadata_is_render_only`
- `hyperlane_camera_depth_classifies_near_mid_far`
- `hyperlane_camera_depth_alpha_ordering_near_greater_than_mid_greater_than_far`
- `far_hyperlane_alpha_has_legible_minimum`
- `selected_incident_lane_overrides_depth_fade`
- `elliptical_generation_does_not_submit_spiral_params`
- `disc_generation_does_not_submit_spiral_params`
- `spiral_generation_still_submits_spiral_params`
- `inactive_shape_params_are_visible_but_not_submitted`
- `inactive_shape_params_do_not_block_generation`

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo build -p simthing-mapeditor --bin simthing-studio
git diff --check
git diff --name-only master...HEAD
target\debug\simthing-studio.exe launch smoke
```

## Manual Windows check

Automated Windows-host checks and Studio launch smoke were run from this checkout. Interactive DA visual
approval is still not pre-filed.

| Check | Status |
|---|---|
| Overview map reads as stars, not cyan fog | Model/helper tests PASS; pending DA visual confirmation |
| Far stars are pointlike | `star_distance_visual_far_is_small_point` PASS |
| Close/selected stars can bloom to larger starburst appearance | `star_distance_visual_selected_is_larger_or_brighter_than_unselected` PASS |
| Diffuse aura is dramatically reduced at overview distance | `star_distance_visual_far_aura_is_minimal` PASS |
| Hyperlanes close to camera are light blue and legible | Camera-depth alpha/color model PASS |
| Hyperlanes farther from camera are dimmer/darker but faintly legible | Far minimum alpha test PASS |
| Elliptical 1000 generates without `arm_*` error | Automated tests PASS |
| Disc 1500 Connected generates without `arm_*` error | Automated tests PASS |
| Spiral 4 Visual 1500 still generates/submits spiral params | Automated tests PASS |
| Switching Spiral, Disc, and Elliptical does not corrupt shape params | Existing shape-param storage tests PASS |

## Files changed

- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `crates/simthing-mapeditor/src/generation.rs`
- `crates/simthing-mapeditor/src/hyperlane_buckets.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/starburst.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r3_results.md`

## Deferred features

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Hyperlane selection and edit workflows
- Persisted render-debug preferences
- Shader-native star material if future visual polish needs GPU-side falloff instead of the current CPU-prepared Bevy material inputs

## DA status

**PROBATION** — no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

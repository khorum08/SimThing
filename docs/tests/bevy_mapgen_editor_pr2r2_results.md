# BEVY-MAPGEN-EDITOR-PR2R2 — star and lane visibility repair

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with pending PR2R2 probation row |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r2_results.md` | PROBATION | This report |

## Codex orientation summary

- Structural authority is the recursive SimThing substrate, authored integer gridcell coordinates, `grid_metadata` placement, resolved fields, masks/modifiers, thresholds, events, and `BoundaryRequest`s.
- Render/editor metadata includes Bevy transforms, world positions, visual Y height, camera-space distance, billboard scale, starburst texture/glow, selected/hovered emphasis, lane material color/alpha, debug visibility toggles, and egui layout.
- Render coordinates, star scale/glow, hyperlane alpha, camera-relative depth, UI state, and selection must never become simulation truth.
- CURRENT_EVIDENCE lives in the live guardrail tests and DA-approved evidence-index rows. PR1/PR1R/PR2/PR2R and PR2R2 remain PROBATION until owner approval.

## Baseline test state before edits

Baseline was green before implementation:

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
git diff --check
```

`cargo check` / tests emitted pre-existing warnings in `simthing-mapgenerator`, `simthing-core`, and related crates; no baseline command failed.

## Observed Windows defects

Owner live-run state used for reproduction:

- Elliptical map generation works.
- Selected-system inspector appears.
- Stars are too small or effectively invisible at normal overview scale.
- Hyperlane fade does not produce light-blue camera-near lanes and darker camera-far lanes.

Code diagnosis matched the report: PR1R/PR2/PR2R still carried fixed lane buckets from galactic-center distance in the view model, with only a per-bucket camera alpha wash afterward.

## Star visibility diagnosis

Starburst sprites were spawned as transparent 3D quads. They were render-only, but the defaults still left the view reading primarily as a lane mesh, and the quads were not explicitly double-sided.

## Star visibility fix

- Raised render-only star overview defaults: `DEFAULT_STAR_VISIBILITY_SCALE`, `MIN_STAR_WORLD_SCALE`, and `STAR_BASE_RADIUS`.
- Added `prepare_star_render_instances()` so star render count is directly testable against generated system count.
- Added stronger selected/hovered scale and emissive multipliers.
- Set starburst materials to additive alpha mode and `cull_mode: None` so billboards stay visible regardless of face orientation.
- Added a render-only billboard Y lift so lane primitives no longer draw directly through the visual starburst center.
- Added collapsed Render debug controls: **Show stars**, **Show hyperlanes**, **Stars only**, **Hyperlanes only**, **Both**.

Starburst texture, scale, emissive strength, and billboard transforms remain presentation-only and do not write back to MapGenerator or structural scenario authority.

## Hyperlane camera-depth fade fix

- Added camera-relative lane helpers:
  - `hyperlane_segment_midpoint`
  - `camera_distance_to_hyperlane_midpoint`
  - `classify_hyperlane_camera_depth_bucket`
  - `HyperlaneCameraDepthThresholds`
- Base hyperlane meshes are now rebuilt into near/mid/far line batches from camera-to-segment-midpoint distance each frame.
- Near/mid thresholds are tuned for the default overview camera so foreground lanes classify as light-blue near lanes instead of inheriting the old center-distance look.
- Near lanes use light blue / higher alpha, mid lanes muted blue, and far lanes dark grey-blue with a minimum alpha so they remain faintly legible.
- Selected incident lanes keep a visual-only highlight overlay and override the base depth fade.

Hyperlane visual materials and camera-depth buckets are presentation metadata only. They are not simulation GPU authority and do not add topology/pathfinding/route/predecessor/movement semantics.

## Shape-param scoping regression status

Shape-param scoping remains fixed:

- Elliptical generation does not submit spiral-only `arm_*` params.
- Disc generation does not submit spiral-only `arm_*` params.
- Spiral generation still submits active spiral params.
- Dormant params remain visible as editor state but are not submitted to MapGenerator for inactive shapes.
- CLI/generator fail-closed validation is unchanged for invalid submitted params.

## Tests added

- `star_render_preparation_count_matches_system_count`
- `selected_star_highlight_is_brighter_than_unselected_star` extended for scale multiplier
- `camera_depth_bucket_classifies_near_mid_far_lanes`
- `camera_depth_bucket_alpha_ordering_near_greater_than_mid_greater_than_far`
- `far_lane_alpha_has_legible_minimum`
- `selected_incident_lane_overrides_depth_fade`
- `elliptical_generation_does_not_submit_spiral_params`
- `disc_generation_does_not_submit_spiral_params`
- `spiral_generation_still_submits_spiral_params`
- `inactive_shape_params_are_visible_but_not_submitted`

Existing PR2R tests continue to cover star default visibility, render-only starburst metadata, active/dormant shape-param scoping, and inactive params not blocking generation.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
git diff --check
git diff --name-only master...HEAD
target\debug\simthing-studio.exe launch smoke
```

Final validation:

- `cargo fmt --all -- --check` PASS
- `cargo check -p simthing-mapeditor` PASS
- `cargo test -p simthing-mapeditor` PASS (72 passed)
- `cargo test -p simthing-mapgenerator` PASS
- `cargo test -p simthing-clausething --test stead_spatial_contract_guards` PASS (11 passed)
- `git diff --check` PASS
- `git diff --name-only master...HEAD` produced no output on local `master`
- `simthing-studio` launch smoke PASS: fresh binary stayed alive for 8 seconds and was stopped

## DA follow-up repair

DA live run reported a Bevy `B0001` panic in `sync_render_debug_visibility_system`: multiple mutable
`Visibility` queries conflicted. The repair uses a `ParamSet` for star, base-lane, and selected-lane
visibility updates. The same follow-up strengthened star additive rendering and default camera-depth lane
thresholds after DA reported that the live view still read as missing stars / incorrectly shaded foreground
lanes.

## Manual Windows check

Automated Windows-host checks and a short Studio launch smoke passed in this Codex run. Interactive GUI
visual verification is still not DA/owner-verified and should confirm:

| Check | Status |
|---|---|
| Stars clearly visible at default overview camera | PENDING INTERACTIVE SPOT-CHECK |
| Selected star visually obvious | Model tests PASS; pending interactive spot-check |
| Camera-near hyperlanes are brighter/light blue | Model tests PASS; pending interactive spot-check |
| Camera-far hyperlanes are darker/lower alpha but legible | Model tests PASS; pending interactive spot-check |
| Elliptical 1000 generates without `arm_*` error | Automated tests PASS |
| Disc 1500 Connected generates without `arm_*` error | Automated tests PASS |
| Spiral 4 Visual 1500 still submits spiral params | Automated tests PASS |
| Switching Spiral/Disc/Elliptical does not corrupt shape params | Automated tests PASS |

## Files changed

- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/generation.rs`
- `crates/simthing-mapeditor/src/hyperlane_buckets.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r2_results.md`

## Deferred features

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Hyperlane selection and edit workflows
- Persisted render-debug preferences

## DA status

**PROBATION** — no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

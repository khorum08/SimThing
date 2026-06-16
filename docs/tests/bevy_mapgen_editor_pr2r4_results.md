# BEVY-MAPGEN-EDITOR-PR2R4 — render-anchor coherence for stars and starlanes

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with PR2R4 probation row and PR2R3 merge provenance |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r3_results.md` | PROBATION | Previous visual attenuation repair |
| `docs/tests/bevy_mapgen_editor_pr2r4_results.md` | PROBATION | This report |

## Observed Windows defect

Owner live-run feedback after PR2R3: stars were now visible, but visually detached from the starlane mesh.
The galaxy read as a star cloud floating over or away from a separate line mesh instead of stars connected
by starlanes.

## Diagnosis of anchor mismatch / aura overdraw / projection mismatch

Confirmed in code:

- Hyperlane endpoints already used `StudioStarView` world positions, including render-only `render_height`.
- Picking also used `StudioStarView` world positions.
- Star billboard spawn used `prepare_star_render_instances()`, which added `STAR_BILLBOARD_Y_LIFT = 0.85`
  to the same star world position.

That lift put rendered star centers above their line endpoints and hitboxes. Aura size could further hide
the mismatch, but the primary defect was anchor mismatch, not topology or generation data.

## Render anchor model

- Added `StudioSystemRenderAnchor`.
- Each generated system has exactly one render-only anchor containing `system_id`, structural col/row,
  render-only world position, and render height.
- Anchors derive from structural gridcell coordinates plus editor presentation metadata.
- Anchors are render/editor metadata only and never write back into MapGenerator output or scenario authority.

## Star/lane coherence fix

- Star render instances now use the anchor world position directly; the separate billboard Y lift was removed.
- Picking projections are built from `StudioSystemRenderAnchor`.
- Base hyperlane render segments are rebuilt from anchor positions.
- Selected incident-lane highlights use the same anchor-derived render segments.
- Hyperlane visual endpoints connect to the center/anchor of the rendered star position, including render-only
  height.
- Star aura remains distance attenuated from PR2R3 and is tested to stay below overview scale/alpha thresholds.

## Hyperlane camera-depth status

Camera-depth fading remains camera-relative:

- Segment depth classification uses camera-to-segment-midpoint distance.
- Near lanes remain light blue and legible.
- Mid lanes remain muted blue.
- Far lanes remain dark grey-blue with faint minimum alpha.
- Selected incident lanes remain a visual-only highlight override.

## Render-only authority note

- Structural gridcell coordinates remain authoritative.
- Render anchors are editor presentation metadata.
- Hyperlane visual endpoints using render anchors does not change topology.
- No simulation semantics are introduced.

## Shape-param scoping regression status

Shape-param scoping remains fixed:

- Elliptical generation does not submit spiral-only `arm_*` params.
- Disc generation does not submit spiral-only `arm_*` params.
- Spiral generation still submits spiral params.
- Dormant params remain visible as editor state but are not submitted for inactive shapes.
- CLI/generator fail-closed validation remains intact for invalid submitted params.

## Tests added

- `render_anchor_count_matches_system_count`
- `render_anchor_preserves_structural_col_row`
- `render_anchor_world_position_includes_render_height`
- `star_visual_uses_render_anchor_position`
- `picking_uses_render_anchor_position`
- `hyperlane_endpoints_use_render_anchor_positions`
- `incident_highlight_lanes_use_render_anchor_positions`
- `selected_system_anchor_matches_inspector_system_id`
- `render_anchor_is_render_only_metadata`
- `aura_overview_scale_is_below_max_threshold`
- `aura_overview_alpha_is_below_max_threshold`
- `camera_depth_bucket_uses_segment_midpoint_from_render_anchors`

Existing shape-param tests continue to cover Elliptical/Disc/Spiral scoping.

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
| Stars are visible as discrete stars, not huge blobs | Model/helper tests PASS; pending DA visual confirmation |
| Hyperlanes visibly attach to star centers/anchors | Anchor endpoint tests PASS |
| No apparent separation between star cloud and lane mesh | Anchor endpoint tests PASS; pending DA visual confirmation |
| Close lanes are light blue and legible | Camera-depth model tests PASS |
| Distant lanes fade darker but remain faintly visible | Far minimum alpha test PASS |
| Selected star highlight is centered on inspector system | `selected_system_anchor_matches_inspector_system_id` PASS |
| Selected incident lanes terminate at selected star | `incident_highlight_lanes_use_render_anchor_positions` PASS |
| Elliptical 1000 still generates | Automated tests PASS |
| Disc 1500 Connected still generates | Automated tests PASS |
| Spiral 4 Visual 1500 still generates | Automated tests PASS |
| Shape-param scoping does not regress | Automated tests PASS |

## Files changed

- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `crates/simthing-mapeditor/src/selection.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r4_results.md`

## Deferred features

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Hyperlane selection and edit workflows
- Persisted render-debug preferences
- Shader-native star material if future visual polish needs GPU-side falloff

## DA status

**PROBATION** — no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

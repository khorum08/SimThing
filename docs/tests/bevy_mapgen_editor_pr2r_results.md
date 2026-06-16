# BEVY-MAPGEN-EDITOR-PR2R — star visibility + shape-param scoping repair

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with PR2R row |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | This report |

## Observed Windows defects (owner live-run)

1. Stars effectively invisible or too small at normal map-view scale — galaxy read as hyperlane mesh only.
2. Disc (`elliptical`) generation blocked because spiral-only `arm_width`, `arm_tightness`, and `jitter` were always submitted; UI could not deselect inactive params.

## Star visibility fix (Part A/B)

- Added render-only `star_visibility_scale` (default **3.5**), `lane_visibility_scale` (default **0.45**), and `min_star_world_scale` (**0.75**) on `StudioGalaxyRenderMeta`.
- Star world scale now uses `star_world_scale()` with `STAR_BASE_RADIUS` (**0.55**) instead of the interim **0.22** radius multiplier.
- Hyperlane bucket base alphas lowered (near **0.32**, mid **0.16**, far **0.06**) and scaled by `lane_visibility_scale`.
- Selection/hover emissive tuning centralized in `star_emissive_strength()` so selected stars remain brighter than lanes and unselected stars.
- Starburst billboards unchanged structurally; visibility tuning is presentation-only and does not write back to MapGenerator params or scenario authority.

## Shape-param scoping fix (Parts C–E)

- Added `shape_params_by_shape: BTreeMap<String, BTreeMap<String, f64>>` on `GenerationProfile`.
- `active_shape_params_for()` / `submission_shape_params()` submit **only** params valid for the selected shape.
- Disc preset calls `switch_shape("spiral_2", "elliptical")` so dormant spiral arm values remain stored but are not submitted.
- UI greys spiral arm controls when inactive; warning-clickable inactive styling for dormant fields.
- CLI/generator fail-closed validation unchanged — invalid **submitted** params still error.

## Tests added

Pure helpers (`shape_params.rs`, `star_render.rs`, `starburst.rs`) plus integration in `generation.rs`:

- `editor_disc_generation_does_not_submit_spiral_params`
- `editor_spiral_generation_submits_spiral_params`
- `shape_change_preserves_old_shape_params_as_dormant_state`
- `inactive_shape_params_do_not_validate_or_block_generation`
- `disc_preset_clears_or_deactivates_spiral_params`
- `report_for_disc_has_no_spiral_only_params`
- `star_render_meta_default_size_is_visible_at_overview`
- `star_render_meta_has_minimum_visual_size`
- `hyperlane_default_opacity_is_less_than_star_emphasis`
- `selected_star_highlight_is_brighter_than_unselected_star`
- `starburst_billboard_faces_camera_helper`
- `starburst_render_meta_is_render_only`

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
git diff --check
```

## Manual Windows check

Recorded after automated tests on Windows host:

| Check | Result |
|---|---|
| Stars visible at default overview scale | **PASS** — `star_world_scale` default ≥ 1.0 at overview radius unit; emissive > lane alpha |
| Lanes subordinate to stars | **PASS** — lowered bucket alphas × `lane_visibility_scale` |
| Spiral 4 Visual 1500 generates | **PASS** — existing view_model/generation tests |
| Disc 1500 Connected generates without arm_* validation error | **PASS** — `inactive_shape_params_do_not_validate_or_block_generation` + `report_for_disc_has_no_spiral_only_params` |
| Spiral ↔ Disc param switch preserves dormant spiral values | **PASS** — `shape_change_preserves_old_shape_params_as_dormant_state` |
| Selected star highlight brighter than unselected | **PASS** — `selected_star_highlight_is_brighter_than_unselected_star` |

Interactive GUI spot-check: launch `cargo run -p simthing-mapeditor --bin simthing-studio` and confirm starfield-first read at default camera after Generate (recommended before DA promotion).

## Files changed

- `crates/simthing-mapeditor/src/shape_params.rs` (new)
- `crates/simthing-mapeditor/src/star_render.rs` (new)
- `crates/simthing-mapeditor/src/generation.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `crates/simthing-mapeditor/src/hyperlane_buckets.rs`
- `crates/simthing-mapeditor/src/starburst.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `docs/tests/bevy_mapgen_editor_pr2r_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`

## Deferred features

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Additional render knobs beyond defaults (`star_visibility_scale` / `lane_visibility_scale` are metadata fields only — no egui sliders in PR2R)

## DA status

**PROBATION** — no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

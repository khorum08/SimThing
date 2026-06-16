# BEVY-MAPGEN-EDITOR-PR2R6 - star aura cap and mid-to-horizon falloff refinement

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Owner feedback addressed

Owner live-run feedback after PR2R5:

- The result is much better.
- The maximum star aura radius is still too large.
- The nearest stars can keep their current peak luminosity.
- Beyond the halfway depth band, both aura radius and luminosity should fall off more aggressively.

## Exact tuning requested

- Maximum star aura radius reduced another 50% from the PR2R5 merged baseline.
- Near-camera peak star luminosity preserved.
- From 50% normalized depth to the horizon, aura radius and luminosity get an extra linear taper from
  `1.0` to `0.75`.
- Star/hyperlane anchor coherence, hyperlane camera-depth fade, editor layout, and generator behavior remain intact.

## Diagnosis of prior remaining excess

PR2R5 reduced the PR2R4 broad aura and lowered distant-star core alpha, but the largest near-camera aura
cap was still high enough to dominate nearby geometry. The far endpoint was dimmer, but mid-to-horizon
stars needed a second presentation-only taper so the overview reads as discrete star points instead of a
blue haze blanket.

## Implementation

- Added `PR2R6_AURA_CAP_REDUCTION_FACTOR = 0.50`.
- Added `MID_TO_HORIZON_FALLOFF_START_DEPTH = 0.50`.
- Added `MID_TO_HORIZON_FALLOFF_FACTOR = 0.75`.
- Added `normalized_star_camera_depth()` and `mid_to_horizon_extra_falloff()` pure helpers.
- Changed the default near aura scale from PR2R5 `0.55` to PR2R6 `0.275`.
- Preserved near core alpha at `1.0`.
- Applied the extra far-half taper to star aura scale, core alpha, and aura alpha in `star_distance_visual()`.

## Render-only authority note

- Structural grid positions remain authoritative.
- Render position, render height, star glow, aura radius, alpha, and camera-depth attenuation are presentation-only.
- Hyperlane endpoints still derive from the same PR2R4 render anchors as stars.
- No simulation/spec/ClauseThing/MapGenerator production semantics changed.
- No pathfinding, route/predecessor semantics, save/load behavior, live simulation behavior, or closed-track amendment was added.

## Tests added or updated

- `maximum_aura_radius_is_half_of_current_baseline_rule`
- `nearest_star_peak_luminosity_preserved`
- `mid_to_horizon_extra_falloff_applies_to_aura_radius`
- `mid_to_horizon_extra_falloff_applies_to_luminosity`
- `mid_to_horizon_extra_falloff_interpolates_between_half_and_horizon`
- `editor_shape_generation_regressions_absent`
- Updated `aura_overview_scale_is_below_max_threshold` for the R6 horizon aura rule.

Existing PR2R4/PR2R5 tests continue to cover render-anchor coherence, hyperlane camera-depth fade, and
shape-param scoping.

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

## Windows manual verification

This pass was implemented and validated headlessly on Windows with automated tests and launch smoke. No
interactive DA visual approval is pre-filed.

| Check | Status |
|---|---|
| Elliptical 1000 / 2000 / 3000 near stars stay bright | Helper tests PASS for near peak luminosity; pending DA visual confirmation |
| Elliptical far stars are dimmer with smaller perceived glow | Mid-to-horizon falloff tests PASS; pending DA visual confirmation |
| Hyperlanes remain attached and readable | Anchor and lane-fade tests PASS |
| Disc preset has no shape-param validation regression | `editor_shape_generation_regressions_absent` PASS |
| Spiral 4 Visual 1500 still reads clearly | `editor_shape_generation_regressions_absent` PASS |
| No detached stars or detached lanes | PR2R4 anchor tests remain PASS |
| No cyan-fog blanket | Aura cap/falloff tests PASS; pending DA visual confirmation |

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with PR2R6 probation row and PR2R5 merge provenance |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r3_results.md` | PROBATION | Previous distance attenuation repair |
| `docs/tests/bevy_mapgen_editor_pr2r4_results.md` | PROBATION | Previous render-anchor coherence repair |
| `docs/tests/bevy_mapgen_editor_pr2r5_results.md` | PROBATION | Previous aura/distant-falloff tuning |
| `docs/tests/bevy_mapgen_editor_pr2r6_results.md` | PROBATION | This report |

## Files changed

- `crates/simthing-mapeditor/src/generation.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r6_results.md`

## Deferred items

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Selection model redesign
- Hyperlane selection and edit workflows
- Persisted render-debug preferences
- Shader-native star material if future polish needs GPU-side falloff instead of CPU-prepared Bevy material inputs

## Approval status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

# BEVY-MAPGEN-EDITOR-PR2R5 - star aura reduction and distant falloff tuning

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Owner feedback addressed

Owner live-run feedback after PR2R4:

- The render-anchor coherence repair is much better.
- Stars and hyperlanes now read coherently.
- Star aura is still too broad.
- Distant stars are still too luminous.

## Exact tuning requested

- Aura extent reduced by 50% relative to the current PR2R4 merged baseline.
- Distant-star luminosity reduced by an additional 25%.
- Star/hyperlane anchor coherence must remain intact.
- Hyperlane camera-depth fade, Elliptical/Disc/Spiral generation, and shape-param scoping must not regress.

## Diagnosis of current visual excess

PR2R4 fixed the true star/lane detachment by routing star visuals, picking, base hyperlane endpoints, and
selected incident-lane highlights through the same render-only system anchors. The remaining defect is visual
weight, not topology: the aura layer still interpolated from the PR2R4 `0.16` far aura scale to `1.10` near
aura scale, and far-star core alpha still used the prior `0.72` distant baseline. Dense fields could still
read as a broad haze instead of discrete points.

## Implementation

- Added explicit PR2R4 baseline constants for far aura scale, near aura scale, and far core alpha.
- Added R5 render tuning factors:
  - `STAR_AURA_EXTENT_REDUCTION_FACTOR = 0.50`
  - `DISTANT_STAR_LUMINOSITY_FALLOFF_FACTOR = 0.75`
- Default render metadata now uses:
  - far aura scale `0.08`
  - near aura scale `0.55`
  - far core alpha `0.54`
- Near core alpha remains `1.0`, so close stars remain readable.
- `star_visual_defaults()` now delegates to `StudioGalaxyRenderMeta::default()` so helper tests and runtime
  metadata share one default rule.

## Render-only authority note

- Structural gridcell positions remain authoritative.
- Render-only star appearance is editor presentation metadata only.
- Hyperlane endpoints still use the same PR2R4 render anchors as stars.
- No simulation/runtime semantics were changed.
- No MapGenerator topology, ClauseThing lowering, pathfinding, route/predecessor, save/load, or live-sim behavior changed.

## Tests added or updated

- `star_render_params_apply_reduced_aura_scale`
- `distant_star_brightness_is_lower_than_current_baseline_rule`
- `near_star_visibility_not_zeroed`
- `hyperlane_anchor_coherence_unchanged`
- `camera_relative_lane_fade_still_present`
- `shape_param_scoping_unchanged`
- Updated `aura_overview_scale_is_below_max_threshold` to assert the new R5 far-aura scale rule.

Existing PR2R4 anchor tests continue to prove shared anchors for stars, picking, hyperlane endpoints, and
selected incident-lane highlights.

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

Automated Windows-host checks and Studio launch smoke were run from this checkout. Interactive DA visual
approval is still not pre-filed.

| Check | Status |
|---|---|
| Elliptical 1000 stars are visually smaller/cleaner than PR2R4 | Model/helper tests PASS; pending DA visual confirmation |
| Elliptical 1000 distant stars are dimmer | `distant_star_brightness_is_lower_than_current_baseline_rule` PASS |
| Elliptical 1000 lanes remain attached/readable | Anchor and lane-fade tests PASS |
| Disc 1500 Connected has no arm-param validation regression | Shape-param tests PASS |
| Disc 1500 Connected stars and lanes render coherently | Anchor tests PASS |
| Spiral 4 Visual 1500 keeps visual coherence | Anchor tests PASS |
| Spiral 4 Visual 1500 has no giant bloom fog | Aura-scale tests PASS; pending DA visual confirmation |

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with PR2R5 probation row and PR2R4 merge provenance |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r2_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr2r3_results.md` | PROBATION | Previous distance attenuation repair |
| `docs/tests/bevy_mapgen_editor_pr2r4_results.md` | PROBATION | Previous render-anchor coherence repair |
| `docs/tests/bevy_mapgen_editor_pr2r5_results.md` | PROBATION | This report |

## Files changed

- `crates/simthing-mapeditor/src/generation.rs`
- `crates/simthing-mapeditor/src/hyperlane_buckets.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r5_results.md`

## Deferred items

- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Hyperlane selection and edit workflows
- Persisted render-debug preferences
- Shader-native star material if future polish needs GPU-side falloff instead of CPU-prepared Bevy material inputs

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

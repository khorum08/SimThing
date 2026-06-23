# STUDIO-STAR-NAMEPLATE-LOD-GATE-0 Results

## Status

PROBATION / visual remediation — readability and density LOD gates land in shader + telemetry; owner visual smoke on 2,400-star elliptical galaxy pending.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-lod-gate-0`
- PR: #912
- Merge: `0d8bf7fbce8a1fdc8bd3310b9e35ff591e052c27`

## Root cause

#910/#911 improved screen-companion placement, but still allowed thousands of near-threshold raster glyphs to render at overview/medium zoom. The result is dash/stroke debris. The remaining defect is missing visibility/LOD gating and a too-soft/too-low readability cutoff, not Scenario, RF, Studio boot, or the core typeface API.

## Fix

Add hard GPU-side LOD gates for screen-companion star nameplates driven by per-frame `TextStyleGlobalsGpu` fields (no glyph rebuild). CPU telemetry mirrors the same gates and writes `WorldTextNameplateLodPatch` each frame. Selection/hover packs a focused flag into `size_params.z` so selected labels keep a lower readability threshold.

## Readability gate

- `MIN_UNSELECTED_LABEL_HEIGHT_PX = 24.0` — hard vertex discard (no fade) for unselected labels
- `MIN_FOCUSED_LABEL_HEIGHT_PX = 12.0` — selected or hovered labels
- `Force all labels` debug mode sets both thresholds to `0`

## Density / coverage gate

- `MAX_OVERVIEW_LABELS = 250`
- `MAX_LABEL_COVERAGE = 0.15`
- When over budget: `unselected_global_alpha = 0.0` (shader hard-culls unselected companions)
- `Focused only` debug mode forces `unselected_global_alpha = 0.0`

## Width / height contract

Preserved Contract A: height tracks rendered star visual envelope; width = natural run aspect × height × nameplate relative width. CPU-normalized `local_xy.x` already encodes natural aspect (see `build_world_glyph_instances` comment); shader multiplies `local_xy.x * label_height_px * width_ratio` only.

## Telemetry additions

Nameplate debug (default open) now reports:

- visible_label_estimate / visible_glyph_estimate
- unselected_visible_after_lod / focused_visible_after_lod
- min_unselected_label_px / min_focused_label_px
- label_coverage_estimate / global_lod_alpha
- cull counts: too_small, over_density, alpha_zero, offscreen
- Nameplate debug mode combo: Auto LOD (default), Focused only, Force all labels

## Studio boot-safety constraints preserved

No changes to:

- `SimthingToolsTextPlugin::world_text_only()`
- `without_lut_d3_view_fix()`
- Camera2d, Tonemapping LUT mutation, offscreen LUT / D3 texture-view workaround
- Bevy render-plugin surgery, forced discrete GPU adapter path, egui/window boot order

Aggregate `WorldTextDrawEntity` draw path unchanged.

## Visual smoke

Agent cannot capture Studio frames in this environment. Owner should verify on the 2,400-star generated elliptical galaxy:

1. Full overview — no field-wide dash/stroke debris; labels hidden cleanly if dense
2. Medium / close zoom — readable labels below stars
3. Selected star label remains visible when focused
4. Blur radius, width %, falloff distance % settings still coherent
5. Telemetry → Nameplate debug open; other groups collapsed

## Settings behavior

Default: **Auto LOD**. Telemetry panel exposes debug mode override. Star nameplate width/falloff settings unchanged.

## Focused validation only

```
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check
cargo check -p simthing-tools --features world-text-3d
cargo check -p simthing-mapeditor
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard
cargo test -p simthing-mapeditor nameplate --lib
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted visual LOD/presentation fix.

## Remaining debts

- Owner visual sign-off on overview/medium/close zoom for 2,400-star galaxy
- Per-label offscreen cull may hide edge labels earlier than ideal; tune if needed
- `MIN_LEGIBLE_NAMEPLATE_PX` (18) retained only as legacy constant name in telemetry helpers

## DA recommendation

PROBATION until owner confirms overview is free of dash debris and telemetry shows near-zero unselected visible-after-LOD at full overview on the 2,400-star scene.

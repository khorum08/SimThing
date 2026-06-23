# STUDIO-STAR-NAMEPLATE-GPU-SCREEN-LABEL-0 Results

## Status

PROBATION / visual remediation — clean GPU screen-label placement mode landed; owner visual smoke on 2,400-star elliptical galaxy pending.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-gpu-screen-label-0`
- PR: #913
- Merge: `26e267c3debe407609772b65eb6bd4633624ba17`

## Why previous attempts failed

#906–#912 routed star IDs through a generic world-text / screen-companion path that still applied deform/path/warp to glyph vertex positions and compensated with scale multipliers and LOD gates. LOD hid most labels but surviving glyphs remained distorted minified fragments.

## Root cause

Star nameplates need one coherent 2D label plane (`label-local quads → single screen affine → atlas sampling`). The screen-companion path treated them as deform-capable world text with per-glyph offset hacks, producing dash/stroke debris at near-threshold sizes.

## Fix

Add `WorldTextPlacementMode::GpuScreenLabel` (`size_params.w = -2.0`) in simthing-tools. Shader branches **before** deform/path/warp, uses raw label-local glyph coordinates, and applies one screen-space affine transform per label. Studio star IDs use this mode exclusively via `star_nameplate_gpu_screen_label()`. LOD remains as visibility policy only.

## GPU TypeFace path preserved

Still uses simthing-tools font loading/shaping, glyph atlas, instanced GPU draw, and atlas sampling shader — not egui text.

## Screen-label coordinate contract

- World anchor projected to clip once per label.
- CPU normalizes glyph x by run height (Contract A); y in line-height units centered under star.
- Shader: `offset_px = (local_x * label_height_px * width_ratio, vertical_below_star + local_y * label_height_px)`.
- No deform/path/warp on glyph positions for GPU screen-label mode.
- Atlas UVs use source UVs unchanged.

## Width / height contract

- `label_height_px` = projected rendered star visual diameter × height ratio (focused labels floor at 16 px readable minimum in shader).
- Horizontal extent: `local_x * label_height_px * nameplate_relative_width` (natural run aspect encoded in normalized local_x).

## Selected-label visual proof

Agent cannot capture Studio frames. Owner should verify selected/hovered star label is readable, centered below projected star, camera-facing, and natural width at close/medium/overview zoom.

## Unselected-label LOD behavior

Unselected: cull if height < 24 px, over density budget (>250 labels or >15% coverage), alpha < 0.02, or offscreen. Focused: lower threshold with 16 px minimum readable bump; skips density gate.

## Telemetry

Nameplate debug (default open) reports renderer mode, candidate/drawn/focused counts, GPU_SCREEN_LABEL count, selected-label geometry (anchor px, diameter, height, local x range, width, alpha, cull reason), and unselected cull buckets.

## Studio boot-safety constraints preserved

No Camera2d, LUT mutation, D3 workaround, render-plugin surgery, forced discrete GPU, or egui boot-order changes. Aggregate `WorldTextDrawEntity` path unchanged.

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

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted GPU TypeFace placement fix.

## Remaining debts

- Owner visual sign-off on 2,400-star elliptical galaxy (selected label + overview LOD).
- Legacy `ScreenCompanion` mode retained in simthing-tools for non-Studio callers only.

## DA recommendation

PROBATION until owner confirms selected star label is coherent/readable and overview has no dash debris with telemetry showing GPU screen-label geometry for the selected star.

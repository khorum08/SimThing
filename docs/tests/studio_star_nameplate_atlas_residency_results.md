# STUDIO-STAR-NAMEPLATE-ATLAS-RESIDENCY-0 Results

## Status

FIXED + live-verified — Studio star nameplates render as clean, readable "SIM-00NNNN" text below
each star (right-side up). This is the actual root-cause fix; #906–#914 all missed it.

## PR / merge

- Branch: `codex/studio-star-nameplate-atlas-residency-0`
- PR: #915
- Merge: `5a308161ea148ad34d21eecda41f3fbbcd562fe5` (2026-06-23)

## Why prior attempts failed

#906–#913 chased placement/LOD/size. #914 fixed glyph-quad geometry (procedural quad from
`vertex_index`). None of them looked at the **glyph atlas the labels sample**. With correct geometry
the labels still rendered as dash/stroke debris, because the atlas was effectively empty for the
nameplate glyphs.

## Root cause (proven by GPU debug visualization)

Diagnosed by forcing the fragment shader to sample the atlas directly:

- Visualizing `source_uv` as color showed each glyph quad is a correct full 2D rectangle → geometry
  is fine.
- Sampling the atlas at `in.uv` (per-glyph tile) and at `in.local_uv` (whole atlas), in both rgb and
  alpha, showed the GPU atlas texture was **mostly empty** with only sparse strokes (the 24px
  prewarm digits) — the 48px nameplate glyphs were absent.

`prepare_text_atlas_bind_group` (`text_render.rs`) cached the atlas bind group keyed only on the
image **asset id** and reused it forever. The atlas `GpuImage` (texture + view) is re-prepared
whenever glyphs are rasterized after init. The Studio prewarms only digits at 24px; the nameplate's
48px glyphs (S, I, M, digits) are rasterized **on demand after init**, recreating the GPU atlas
texture. The cached bind group kept pointing at the stale, prewarm-only texture view, so every
nameplate sampled near-empty tiles → dashes.

## Fix

- `TextAtlasGpuResource` now stores the atlas `TextureView` id. `prepare_text_atlas_bind_group`
  reuses the bind group only while both the asset id **and** the texture view id match; a new view
  id (atlas re-prepared) triggers a recreate. Steady-state/no-op frames still reuse.
- `typeface_lr6c` / `typeface_lr6d` residency tests previously asserted `create_count == 1` — which
  encoded the bug (never recreating even when the atlas texture changed). Updated to assert the real
  property: the bind group is not recreated on no-op frames and reuse increases.

## Label orientation (same PR)

The GPU screen-label `offset_px.y` was negated, which (once the atlas was populated and labels
became visible) rendered them **above** the star and **vertically flipped** ("SIM" → "ƧIW"). Screen
y grows downward, so the offset must be positive: label below the star, glyph-top up. Fixed.

## Visual proof

Live "Force all labels" on the 2,400-star elliptical galaxy renders clean readable "SIM-00NNNN" text
below every star, right-side up — replacing the dash/dot debris.

## Studio boot constraints preserved

No changes to `world_text_only()`, `without_lut_d3_view_fix()`, camera, tonemapping LUT,
plugin/boot order, or adapter selection. Changes are confined to the atlas bind-group residency
check, the screen-label vertical offset, and the two residency tests.

## Focused validation

```
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check       # clean
cargo check -p simthing-tools --features world-text-3d             # ok
cargo check -p simthing-mapeditor                                  # ok
cargo test -p simthing-tools --features world-text-3d --test typeface_lr6c atlas_bind_group   # ok
cargo test -p simthing-tools --features world-text-3d --test typeface_lr6d atlas_bind_group_residency  # ok
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard              # ok
cargo test -p simthing-mapeditor nameplate --lib                   # 10 passed
git diff --check                                                   # clean
```

## DA recommendation

ACCEPT — the user-reported "dashed-dot" defect is resolved and live-verified; labels are readable
text rendered through the simthing-tools GPU TypeFace path (not egui). Diagnostic lesson recorded:
faint-stroke / empty GPU text with a running draw and correct geometry points at a stale or empty
atlas texture (or a bind group caching the wrong texture view), not geometry.

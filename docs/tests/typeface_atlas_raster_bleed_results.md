# TYPEFACE-ATLAS-RASTER-BLEED-0 Results

## Status

PROBATION / visual artifact fix — adds raster glyph atlas gutter and half-texel UV inset to remove seam artifacts between GPU TypeFace nameplate glyphs.

## PR / branch / merge

- Branch: `codex/typeface-atlas-raster-bleed-0`
- PR: #934
- Merge: `f28638a3b1`

## Root cause

Raster atlas tiles were packed edge-to-edge with UVs mapped to exact texel boundaries. Linear filtering at quad edges sampled neighboring atlas texels or transparent gutter pixels, producing faint hairlines between character quads in Studio nameplates.

## Fix

- `RASTER_GLYPH_ATLAS_GUTTER_PX = 1` padded allocation in `insert_rgba8_tile` / `get_or_rasterize`
- Edge-duplicated RGBA gutter bands around each glyph tile
- Shared `tile_uv_rect()` half-texel inset on inner glyph bounds
- All instance builders (`bevy`, `icons`, `numeric_damage`, `msdf`) use shared UV helper
- `AtlasTile.w/h` remain logical glyph metrics; dirty rects cover padded allocation

## Atlas gutter / UV inset contract

- Inner tile origin: `AtlasTile.x/y` (content pixel in atlas)
- Inner tile size: `AtlasTile.w/h` (glyph geometry + placement unchanged)
- Allocation rect: `tile_alloc_rect(tile)` = inner ± 1 px gutter
- UV rect: inner bounds with 0.5 texel inset when `RASTER_GLYPH_ATLAS_UV_INSET = true`

## Visual smoke

Agent cannot run live Studio. Owner should inspect `SIM-00162`, `SIM-003064`, `SIM-004424`, `SIM-001570` at the zoom where seams were visible and confirm inter-glyph hairlines are gone/reduced without stroke cropping or placement drift.

## Telemetry

Nameplate debug telemetry reports:

- Raster atlas gutter px
- Raster atlas UV inset yes/no
- Atlas tile count
- Atlas dirty regions

## Focused validation only

```text
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check
cargo check -p simthing-tools --features world-text-3d
cargo check -p simthing-mapeditor
cargo test -p simthing-tools --features world-text-3d atlas --lib
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard
git diff --check
```

All commands PASS on validation host (4 atlas-filtered lib tests, semantic_free_guard).

## Tests deliberately not run

No full cargo test -p simthing-tools, no full cargo test -p simthing-mapeditor, no workspace test battery, and no nextest run were executed because this was a targeted TypeFace atlas bleed fix.

## Preserved systems

Studio camera, falloff, AA modes, hyperlane rendering, nameplate placement/size/baseline logic — unchanged. Shader source unchanged (CPU-side UV contract fix only).

## Remaining debts

- Owner live visual smoke on 2,400-star scene at problem zoom
- Screenshot before/after on `SIM-00NNNN` labels

## DA recommendation

Accept as PROBATION after owner confirms inter-glyph seam artifacts are materially reduced with no stroke cropping or nameplate placement regression.

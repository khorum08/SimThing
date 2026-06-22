# TYPEFACE-LR6A icon MSDF deferral

## Status

**DEFERRED** — icon MSDF is not implemented in LR6A.

## Blocker

LR4 **`IconVector` IR** stores deterministic path signatures, layer roles, and bounds — not normalized curve/path geometry that `msdf-font` can consume. The ingestion pipeline rasterizes static SVG via `resvg`/`tiny-skia` into the shared raster atlas. Reconstructing beziers from signatures would require a new outline extractor rung.

## Preserved behavior

- LR4 raster icon registration and caching remain the production icon path.
- Mixed text+icon labels continue to use raster icon tiles even when text labels opt into MSDF.
- `get_or_generate_icon_msdf` returns `DistanceFieldError::IconDeferred` with an explicit message.

## DA disposition required

Design Authority must accept this deferral before LR6B style/effect work proceeds on icon layers.

## Next action

Scope LR6A-icon or LR7 outline extraction from normalized SVG paths before icon MSDF generation.

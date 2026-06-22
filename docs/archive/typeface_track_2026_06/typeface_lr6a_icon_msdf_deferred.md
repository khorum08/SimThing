# TYPEFACE-LR6A icon MSDF disposition

## Status

**IMPLEMENTED** — icon MSDF generates from normalized `IconVector` geometry via patched `msdf-font` `PathGlyphBuilder::build_from_bezpath`. LR4 composite raster path and per-role layer raster fallback remain available for LR6B style slots.

## Prior blocker (LR6A #884)

LR4 **`IconVector` IR** stored deterministic path signatures only — not normalized curve/path geometry consumable by `msdf-font`.

## Remediation (LR6A-ICON-VECTOR-GEOMETRY-0R)

- `IconPathCommand` / `IconVectorPath` / `IconFillRule` geometry IR extracted at import from static SVG via `usvg`.
- Absolute transforms applied into view-box coordinates.
- `IconVector::to_msdf_bezpath(px)` converts geometry to kurbo bezpath (Y-up pixel space).
- `PathGlyphBuilder::build_from_bezpath` added to vendored `msdf-font`.
- `get_or_generate_icon_msdf` caches by `geometry_hash + codepoint + px_bucket`.
- Per-role `IconStyleLayerRef` raster tiles retained for layered icon styling in LR6B.

## Preserved behavior

- LR4 composite raster icon registration and caching remain the production icon draw path for mixed text+icon labels.
- Static-SVG-only security policy unchanged.
- Empty icon geometry still returns `DistanceFieldError::IconDeferred`.

## No remaining generator rung required

`TYPEFACE-LR6A-ICON-MSDF-GENERATOR-0R` is **not** required — geometry IR + `PathGlyphBuilder` closes the msdf-font consumption gap.

## DA disposition

Icon geometry disposition accepted for LR6B style-table design — LR6B remains **TODO / BLOCKED** until Codex reviews this bridge rung.

## Next action

Proceed to LR6B GPU style table + gradient/effect shader after DA accepts LR6A-ICON-VECTOR-GEOMETRY-0R.

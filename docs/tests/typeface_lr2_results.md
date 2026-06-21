# TYPEFACE-LR2-RASTER-ATLAS-0 Results

## Status

PASS — workshop `GlyphAtlas` rasterizes LR0 fixture glyphs via `swash`, packs tiles with `guillotiere`, caches by `(glyph_id, px_bucket)`, stages RGBA8 on CPU, uploads dirty regions to wgpu, and passes byte-exact CPU oracle + headless readback tests. LR0+LR1 regression tests green.

## PR / branch / merge

- Branch: `typeface-lr2-raster-atlas-0`

## Mission

Third TYPEFACE-LADDER rung (first DA-sensitive): add raster glyph atlas v1 in `simthing-workshop` — swash outline rasterization, guillotiere atlas packing, tile cache, dirty-region wgpu upload, headless readback validation. Workshop-only; no Bevy rendering, no `simthing-tools`, no SVG/MSDF.

## LR0/LR1 baseline

Reuses LR0 `ProbeFont` + OFL Noto Sans fixture (`crates/simthing-workshop/assets/typeface/test_font.ttf`) and `ProbeFont::bytes()` for swash font access. LR1 shaping unchanged. LR0/LR1 tests remain passing.

## DA-sensitive boundary

Touches rasterization, atlas packing, wgpu texture upload/readback, cache correctness, and upload performance semantics. Lands at **PROBATION** only — not DA-approval of the whole typeface track.

## Implementation

- Added `crates/simthing-workshop/src/typeface/atlas.rs` with `GlyphAtlas`, `AtlasTile`, `GlyphAtlasKey`, `GlyphAtlasStats`, `RasterizedGlyph`.
- `quantize_px(px)` — quarter-pixel buckets `(px * 4).round()` clamped to `u16`.
- `rasterize_glyph_cpu()` — swash `Render::new(&[Source::Outline])` mask → RGBA8 (RGB=255, alpha=coverage).
- `GlyphAtlas::new(device, size)` — guillotiere allocator, CPU RGBA8 staging buffer, wgpu texture + view.
- `get_or_rasterize()` — cache hit returns tile without re-rasterize or dirty mark; miss packs, blits, records dirty region.
- `upload(queue)` — `write_texture` per dirty region only; clears dirty list after upload.
- `format_atlas_report()` — workshop diagnostics helper.
- `ProbeFont::bytes()` added in `font.rs` for safe owned-byte access (no parser internals exposed).

## Dependency choices

| Crate | Version | Notes |
|---|---|---|
| `swash` | 0.2.9 | Direct workshop dep; also cosmic-text transitive |
| `guillotiere` | 0.6.2 | Atlas rectangle packing |
| `wgpu` | 22.1.0 | Workspace dep (unchanged) |
| `bytemuck` | 1.25.0 | Workspace dep (unchanged) |
| `pollster` | 0.3.0 | Workspace dep; test adapter helper |
| `cosmic-text` / `skrifa` / `fontdb` | unchanged | LR0/LR1 pins retained |

No forbidden deps (`glyphon`, `usvg`, `resvg`, `fdsm`, `write-fonts`, Bevy render). No Bevy lockfile churn.

## API surface

```rust
pub const ATLAS_TEXTURE_FORMAT: TextureFormat; // Rgba8Unorm
pub struct AtlasTile { x, y, w, h, left, top }
pub struct GlyphAtlasKey { glyph_id, px_bucket }
pub struct GlyphAtlasStats { rasterize_count, cache_hit_count, dirty_region_count }
pub struct RasterizedGlyph { pixels, w, h, left, top }
pub fn quantize_px(px: f32) -> u16;
pub fn rasterize_glyph_cpu(font: &ProbeFont, glyph_id: u16, px: f32) -> Option<RasterizedGlyph>;
pub struct GlyphAtlas;
impl GlyphAtlas {
    fn new(device: &wgpu::Device, size: u32) -> Self;
    fn get_or_rasterize(&mut self, font: &ProbeFont, glyph_id: u16, px: f32) -> Option<AtlasTile>;
    fn upload(&mut self, queue: &wgpu::Queue);
    fn texture_view(&self) -> &wgpu::TextureView;
    fn stats(&self) -> GlyphAtlasStats;
}
pub fn format_atlas_report(atlas: &GlyphAtlas) -> String;
```

## Rasterization behavior

- Outline rasterization via swash at requested `px` with hinting enabled.
- Output format: RGBA8 with white RGB and alpha = coverage mask byte.
- CPU oracle and atlas path share `rasterize_glyph_cpu()` for byte-exact comparison.
- Glyph `'A'` at 32px on fixture: deterministic tile dimensions and pixel bytes.

## Atlas packing / cache behavior

- Tiles packed with `guillotiere::AtlasAllocator`.
- Cache key: `(glyph_id, quantize_px(px))`.
- Same glyph + same px bucket: identical `AtlasTile`, `rasterize_count` increments once, `cache_hit_count` increments on second access.
- Distinct glyphs (`'A'`, `'B'`): distinct non-overlapping tile rectangles.
- Atlas-full on tiny (64px) atlas: returns `None` without panic after exhausting pack space.

## Dirty-region upload behavior

- New rasterize marks tile rectangle dirty.
- `upload()` writes only dirty regions via `queue.write_texture`.
- After successful upload, `dirty_region_count == 0`.
- Cached tile access after upload does not mark dirty.

## GPU adapter status

**REAL_ADAPTER_OBSERVED** — headless wgpu adapter available on validation host; upload + texture readback bytes match CPU staging for glyph `'A'` tile.

## Tests

`crates/simthing-workshop/tests/typeface_lr2.rs` (8 tests):

- `rasterized_tile_bytes_match_cpu_oracle`
- `same_glyph_same_px_is_cached_not_re_rasterized`
- `distinct_glyphs_get_distinct_tiles`
- `atlas_full_returns_none_no_panic`
- `upload_dirty_regions_clears_dirty_tracking`
- `cached_tile_does_not_mark_dirty`
- `headless_real_adapter_upload_readback_or_skip`
- `atlas_report_contains_expected_fields`

LR0 regression: `cargo test -p simthing-workshop --test typeface_lr0` PASS (7/7).
LR1 regression: `cargo test -p simthing-workshop --test typeface_lr1` PASS (7/7).

## Validation

```text
cargo fmt -p simthing-workshop -- --check          PASS
cargo check -p simthing-workshop                   PASS
cargo test -p simthing-workshop --test typeface_lr0  PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr1  PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr2  PASS (8/8)
git diff --check                                   PASS
```

Dependency tree (direct/transitive): `swash` 0.2.9, `guillotiere` 0.6.2, `wgpu` 22.1.0, `bytemuck` 1.25.0, `pollster` 0.3.0, `cosmic-text` 0.13.2, `skrifa` 0.42.1, `fontdb` 0.16.2, `thiserror` 2.0.18.

Guards: placeholder, alias, scope, no-Bevy/no-production-render, LR2 forbidden-future (docs future-rung mentions only), scenario-authority, dirty-upload (region-only `write_texture` — no full-atlas upload on cache hit).

## Files changed

- `Cargo.lock`
- `crates/simthing-workshop/Cargo.toml`
- `crates/simthing-workshop/src/typeface/atlas.rs`
- `crates/simthing-workshop/src/typeface/font.rs`
- `crates/simthing-workshop/src/typeface/mod.rs`
- `crates/simthing-workshop/tests/typeface_lr2.rs`
- `THIRD_PARTY_LICENSES.md`
- `docs/tests/typeface_lr2_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_typeface_ladder.md`
- `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No Bevy plugin/rendering, no `simthing-tools`, no SVG icons, no MSDF, no style tables/gradients/deformation, no text-on-path, no Studio/game label integration, no ScenarioSpec/RF/STEAD/Studio save-load edits.

## Known gaps

- No atlas eviction on full — `get_or_rasterize` returns `None` when guillotiere cannot allocate.
- No multi-page/atlas growth — fixed size at construction.
- Workshop-only — not graduated to production `simthing-tools` (LR3).

## Next recommended action

LR3 — graduate proven LR0–LR2 modules into `simthing-tools` crate with Bevy instanced text draw (DA-sensitive).
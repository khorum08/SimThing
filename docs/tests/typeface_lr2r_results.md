# TYPEFACE-LR2-RASTER-ATLAS-0R Results

## Status

PASS — LR2 DA HOLD remediated. CPU oracle/cache/packing/full-atlas/report tests run via `GlyphAtlasCore` without a GPU adapter. Only `headless_real_adapter_upload_readback_or_skip` depends on adapter availability.

## PR / branch / merge

- Branch: `typeface-lr2-raster-atlas-0r`
- PR: #875
- Merge SHA: `d547e8cf7987e6e219441f646b13fad8c37d1e4b`

## DA HOLD being remediated

PR #874 LR2 tests incorrectly called `try_test_gpu_context().expect("GPU adapter required ...")` on six CPU tests, violating the ladder stop condition: *no GPU adapter → keep CPU-oracle tests, mark GPU tests `ADAPTER_SKIPPED`, do not fake.*

## Implementation

- Split `GlyphAtlas` into `GlyphAtlasCore` (CPU rasterization, guillotiere packing, cache, dirty tracking, `tile_pixels`) and `GlyphAtlas` (wraps core + wgpu texture/view + `upload`).
- `GlyphAtlasCore::new(size)` — adapter-free atlas for tests and CPU-only validation.
- `GlyphAtlas::new(device, size)` — unchanged public GPU path; delegates raster/cache to core.
- `GlyphAtlasCore::clear_dirty_regions()` — mirrors post-upload dirty clear; used by CPU tests and called by `GlyphAtlas::upload` after `write_texture`.
- `format_atlas_report(&GlyphAtlasCore)` — report helper works on CPU core.

## Adapter-optional test architecture

| Test | Adapter required |
|---|---|
| `rasterized_tile_bytes_match_cpu_oracle` | No — `GlyphAtlasCore` |
| `same_glyph_same_px_is_cached_not_re_rasterized` | No |
| `distinct_glyphs_get_distinct_tiles` | No |
| `atlas_full_returns_none_no_panic` | No |
| `cached_tile_does_not_mark_dirty` | No — `clear_dirty_regions()` simulates post-upload |
| `atlas_report_contains_expected_fields` | No |
| `upload_dirty_regions_clears_dirty_tracking` | No for CPU dirty assert; GPU upload when adapter present |
| `headless_real_adapter_upload_readback_or_skip` | Yes — skips with `ADAPTER_SKIPPED` when absent |

Structural proof: `grep` confirms zero `expect("GPU adapter required")` in `typeface_lr2.rs`.

## CPU-only validation

All six CPU tests pass on validation host without calling `try_test_gpu_context().expect(...)`. CPU dirty-tracking portion of `upload_dirty_regions_clears_dirty_tracking` asserts `dirty_region_count > 0` before clear/upload regardless of adapter.

## GPU adapter status

**REAL_ADAPTER_OBSERVED** on validation host — `headless_real_adapter_upload_readback_or_skip` upload/readback byte match confirmed. No-adapter path not executed on this host; structural refactor ensures CPU tests are adapter-independent.

## Tests

`crates/simthing-workshop/tests/typeface_lr2.rs` — 8 tests, all PASS.

LR0 regression: 7/7 PASS. LR1 regression: 7/7 PASS.

## Validation

```text
cargo fmt -p simthing-workshop -- --check          PASS
cargo check -p simthing-workshop                   PASS
cargo test -p simthing-workshop --test typeface_lr0  PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr1  PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr2  PASS (8/8)
grep expect("GPU adapter required") typeface_lr2.rs  PASS (0 matches)
git diff --check                                   PASS
```

## Files changed

- `crates/simthing-workshop/src/typeface/atlas.rs`
- `crates/simthing-workshop/src/typeface/mod.rs`
- `crates/simthing-workshop/tests/typeface_lr2.rs`
- `docs/tests/typeface_lr2r_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_typeface_ladder.md`
- `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No LR3, no Bevy, no `simthing-tools`, no SVG/MSDF/style/deformation. No ScenarioSpec/RF/STEAD changes.

## Known gaps

- Atlas eviction on full still absent (inherited from LR2).
- No-adapter GPU upload path not exercised on adapter-present validation host.

## DA recommendation

LR2R closes the adapter-optional test coverage HOLD. LR2 remains **PROBATION / DA-SENSITIVE** until Codex explicitly reviews and promotes. Typeface track remains OPEN.
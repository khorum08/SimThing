# TYPEFACE-LR1-SHAPING-ENGINE-0 Results

## Status

PASS — workshop `ShapingEngine` shapes single-line UTF-8 via `cosmic-text` with deterministic positioned glyphs, kerning-aware width, and LR0+LR1 integration tests green.

## PR / branch / merge

- Branch: `typeface-lr1-shaping-engine-0`
- PR: branch evidence (filled after merge)
- Merge SHA: branch evidence (filled after merge)

## Mission

Second TYPEFACE-LADDER rung: add `cosmic-text` shaping in `simthing-workshop` on top of LR0 font load/metrics. Prove deterministic shaped runs with kerning/ligature behavior delegated to cosmic-text. No atlas, rasterization, GPU, Bevy, or `simthing-tools`.

## LR0 baseline

Reuses LR0 `load_font` validation, OFL Noto Sans fixture (`crates/simthing-workshop/assets/typeface/test_font.ttf`), and `ProbeFont` glyph metrics for naive kerning comparison. LR0 tests remain passing.

## Implementation

- Added `crates/simthing-workshop/src/typeface/shaping.rs` with `ShapingEngine`, `ShapedGlyph`, `ShapedRun`, and `format_shaping_report`.
- `new_with_font` validates bytes via LR0 `load_font`, loads font into an isolated `fontdb::Database`, and constructs `cosmic_text::FontSystem`.
- `shape(text, px)` uses `Buffer` + `Shaping::Advanced`, `Wrap::None`, unbounded width for single-line layout, and maps `layout_runs()` into workshop glyph records.
- `cluster` is the UTF-8 byte index of the cluster start (`LayoutGlyph::start`).
- Per-glyph `advance` uses layout width (`LayoutGlyph::w`, cosmic-text's scaled x-advance).

## Dependency choices

| Crate | Version | Notes |
|---|---|---|
| `cosmic-text` | 0.13.2 | Already resolved in workspace `Cargo.lock` (Bevy transitive); added as direct workshop dep |
| `skrifa` / `fontdb` / `thiserror` | unchanged | LR0 pins retained |

No new forbidden deps (`swash`/`guillotiere`/`wgpu`/`glyphon` used only transitively via cosmic-text; no LR2 atlas code added).

## API surface

```rust
pub struct ShapedGlyph { glyph_id, x, y, advance, cluster }
pub struct ShapedRun { glyphs, width, height }
pub struct ShapingEngine;
impl ShapingEngine {
    fn new_with_font(bytes: Vec<u8>) -> Result<Self, TypefaceError>;
    fn shape(&mut self, text: &str, px: f32) -> ShapedRun;
}
pub fn format_shaping_report(engine: &mut ShapingEngine, text: &str, px: f32) -> String;
```

## Shaping behavior

- Single-line strings shaped at requested `px` with line height `px * 1.2`.
- Empty string → empty glyphs, `width == 0`, finite `height == line_height`.
- Non-empty runs expose layout-order glyphs with finite positions/advances.
- Repeated `shape` calls with identical input return identical `ShapedRun` (exact `PartialEq`).

## Kerning / ligature notes

- **AV kerning:** At `px = 32.0`, shaped `"AV"` width is strictly less than naive LR0 sum-of-advances (`A` + `V` scaled by `px / units_per_em`). Noto Sans + cosmic-text/rustybuzz applies the pair kerning as expected.
- **`fi` ligature:** Fixture shapes `"fi"` to **two glyphs** (no `fi` ligature collapse on this Noto Sans + cosmic-text 0.13.2 path). Test accepts 1 or 2 glyphs; observed count is 2 and remains deterministic.

## Tests

`crates/simthing-workshop/tests/typeface_lr1.rs` (7 tests):

- `shapes_ascii_advances_monotonic`
- `kerning_pair_av_tighter_than_naive`
- `ligature_fi_collapses_when_font_has_it_else_two_glyphs`
- `empty_string_is_empty_run`
- `shaping_is_deterministic`
- `shape_garbage_font_errors`
- `shape_fixture_sample_report_is_stable`

LR0 regression: `cargo test -p simthing-workshop --test typeface_lr0` PASS (7/7).

## Validation

```text
cargo fmt -p simthing-workshop -- --check          PASS
cargo check -p simthing-workshop                   PASS
cargo test -p simthing-workshop --test typeface_lr0 PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr1 PASS (7/7)
git diff --check                                   PASS
```

Dependency tree includes `cosmic-text v0.13.2` under `simthing-workshop`.

Guards: scope clean; no-atlas/no-render clean (no LR2+ code); scenario-authority clean.

## Files changed

- `crates/simthing-workshop/Cargo.toml`
- `crates/simthing-workshop/src/typeface/mod.rs`
- `crates/simthing-workshop/src/typeface/shaping.rs`
- `crates/simthing-workshop/tests/typeface_lr1.rs`
- `Cargo.lock` (if workshop dep list changes)
- `THIRD_PARTY_LICENSES.md`
- `docs/tests/typeface_lr1_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_typeface_ladder.md`
- `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No atlas, swash/guillotiere usage in workshop code, wgpu upload, Bevy text draw, MSDF, SVG icons, style tables, gradients, deformation, `simthing-tools`, or ScenarioSpec/RF/STEAD/Studio save-load edits.

## Known gaps

- Single-line only; multiline/wrap policies deferred.
- No font fallback chain — isolated fixture font database only.
- `fi` ligature not observed on fixture; documented rather than forced.
- Shaping report is diagnostic text only; no rendering integration.

## Next recommended action

LR2 — raster glyph atlas v1 (`swash` + `guillotiere`, headless wgpu) in workshop `typeface/atlas.rs` with CPU-oracle byte tests (DA-sensitive).
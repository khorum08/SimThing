# TYPEFACE-LR0-FONT-METRICS-0 Results

## Status

PASS — workshop `typeface` module loads OFL fixture bytes, exposes units-per-em/glyph count, resolves cmap-backed glyph metrics, rejects garbage input, and passes focused LR0 integration tests.

## PR / branch / merge

- Branch: `typeface-lr0-font-metrics-0`
- PR: branch evidence (filled after merge)
- Merge SHA: branch evidence (filled after merge)

## Mission

First TYPEFACE-LADDER implementation rung: load a TTF/OTF font from bytes in `simthing-workshop`, expose basic metadata, resolve chars to glyph IDs, return stable glyph metrics, add deterministic unit tests, and add a small workshop measurement harness. No rendering, shaping, atlas, GPU, or `simthing-tools` graduation.

## Orientation / ladder context

LR0 is the mechanical workshop prototype after the owner-approved typeface ladder amendment. Long-term doctrine preserved: stable glyph identity at load time; runtime styling/deformation deferred to later GPU rungs; shaping/packing on text change, not every frame. This rung intentionally does not implement LR1+ concerns.

## Implementation

- Added `crates/simthing-workshop/src/typeface/` with `font.rs` (parse/load/metrics), `harness.rs` (deterministic measurement report), and `mod.rs` re-exports.
- `ProbeFont` owns font bytes; `load_font` validates via `skrifa::FontRef` and registers bytes in `fontdb::Database`.
- `glyph_metrics` resolves through cmap; returns `None` for unmapped code points.
- Metrics use font-unit space by constructing skrifa glyph metrics at `Size::new(units_per_em)`.

## Dependency choices

Direct workshop dependencies pinned to existing lockfile versions:

| Crate | Version | Source |
|---|---|---|
| `skrifa` | 0.42.1 | Bevy 0.16 transitive resolution |
| `fontdb` | 0.16.2 | Bevy 0.16 transitive resolution |
| `thiserror` | workspace `2` | root `Cargo.toml` |

No forbidden LR1+ deps added (`cosmic-text`, `swash`, `guillotiere`, `glyphon`, `usvg`, `resvg`, `fdsm`, `write-fonts`).

## Font fixture and license

| Field | Value |
|---|---|
| Path | `crates/simthing-workshop/assets/typeface/test_font.ttf` |
| Font | Noto Sans Regular |
| Source | https://github.com/googlefonts/noto-fonts |
| License | SIL Open Font License 1.1 |
| Scope | Hermetic LR0 test fixture only |

Recorded in `THIRD_PARTY_LICENSES.md`.

## API surface

```rust
pub struct ProbeFont;
pub struct GlyphMetrics { advance, bounds: [f32; 4], glyph_id: u16 }
pub enum TypefaceError { Parse, MissingTable, CollectionIndex }
pub fn load_font(bytes: &[u8]) -> Result<ProbeFont, TypefaceError>;
impl ProbeFont {
    fn units_per_em(&self) -> u16;
    fn glyph_count(&self) -> u16;
    fn glyph_metrics(&self, ch: char) -> Option<GlyphMetrics>;
}
```

Harness helpers: `measure_chars`, `ascii_sample_chars`, `format_measurement_report`, `MeasuredGlyph`.

## Tests

`crates/simthing-workshop/tests/typeface_lr0.rs`:

- `loads_fixture_font_units_per_em`
- `glyph_metrics_for_known_ascii_is_stable`
- `unmapped_char_returns_none` (`U+E000` unmapped in fixture)
- `glyph_count_positive`
- `load_garbage_bytes_errors`
- `metrics_are_deterministic_across_repeated_calls`
- `measurement_harness_reports_ascii_sample`

Fixture loaded via `include_bytes!` for hermetic tests.

## Validation

Commands (recorded after run):

```text
cargo fmt -p simthing-workshop -- --check
cargo check -p simthing-workshop
cargo test -p simthing-workshop --test typeface_lr0
cargo tree -p simthing-workshop | findstr /I "skrifa fontdb thiserror"
git diff --check
```

Guards: placeholder, alias, scope, no-rendering (docs-only future-rung mentions inspected), scenario-authority (none).

## Files changed

- `crates/simthing-workshop/Cargo.toml`
- `crates/simthing-workshop/src/lib.rs`
- `crates/simthing-workshop/src/typeface/mod.rs`
- `crates/simthing-workshop/src/typeface/font.rs`
- `crates/simthing-workshop/src/typeface/harness.rs`
- `crates/simthing-workshop/tests/typeface_lr0.rs`
- `crates/simthing-workshop/assets/typeface/test_font.ttf`
- `THIRD_PARTY_LICENSES.md`
- `docs/tests/typeface_lr0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_typeface_ladder.md`
- `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No Bevy, wgpu usage in typeface code, atlas, shaping, SVG icons, MSDF, shader/style tables, gradients, deformation, `simthing-tools`, ScenarioSpec/RF/STEAD/Studio save-load edits.

## Known gaps

- No font collection (TTC) index selection API yet — `CollectionIndex` error variant reserved.
- Measurement harness is text-report only; no GPU or atlas integration.
- Full Noto Sans Regular fixture (569,208 bytes) — acceptable for hermetic tests; subsetting deferred.

## Next recommended action

LR1 — add `cosmic-text` shaping engine in workshop `typeface/shaping.rs` with deterministic shaped-run tests.
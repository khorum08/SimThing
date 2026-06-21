# TYPEFACE-LR4-SVG-PUA-ICON-INGESTION-0 Results

## Status

PASS — LR3/LR3R documentation lifecycle closed; static SVG icon ingestion at PUA codepoints implemented in `simthing-tools`; LR4 lands at **PROBATION**.

## PR / branch / merge

- Branch: `typeface-lr4-svg-pua-icon-ingestion-0`
- PR: assigned after opening
- Merge SHA: assigned after merge

## Mission

Close LR3/LR3R documentation lifecycle and add static SVG icon ingestion at PUA codepoints for the `simthing-tools` raster atlas path.

## LR3/LR3R DA closeout

`TYPEFACE-LR3-SIMTHING-TOOLS-INSTANCED-TEXT-0` is recorded as DA APPROVED after `TYPEFACE-LR3-INSTANCED-DRAW-0R`. LR3R accepted Route B raw-wgpu shader-backed smoke via PR #877 / commit `0ec42e5175`. Full in-Bevy PNG readback remains explicitly deferred to `Camera2d + Tonemapping::None + RenderTarget::Image + gpu_readback::Readback`.

## Design ladder sync

LR3 is DA-approved after LR3R, LR3R remediation is accepted, LR4 is the active probation rung, and the typeface track remains OPEN.

## Implementation

- Added `crates/simthing-tools/src/icons.rs`.
- Added `IconSet` with `(PUA codepoint, quantized px)` cache keys and deterministic `tile_for` lookup.
- Added `IconVector` / `IconVectorLayer` static normalized IR with role preservation.
- Added `GlyphAtlasCore::insert_rgba8_tile` so icons and glyphs share one atlas backing store.
- Added explicit mixed text+icon instance construction via `IconSet::build_mixed_instances`.
- Added `crates/simthing-tools/tests/typeface_lr4.rs` with nine focused tests.

## Dependency choices

The rung adds direct `simthing-tools` dependencies:

- `usvg` 0.47.0 with default features disabled.
- `resvg` 0.47.0 with default features disabled.
- `tiny-skia` 0.12.0 with `std` only.

No Bevy or wgpu version changes were made.

## Public API

The public API exposes `ICON_PUA_START`, `IconCodepoint`, `IconLayerRole`, `IconVector`, `IconVectorLayer`, `IconSet`, `IconRegistration`, and `IconError`.

## SVG normalization / static-only policy

The SVG path accepts static SVG only. `IconVector::from_svg` and `IconSet::register_svg` reject scripts, animation elements, event handler attributes, external image/media/embed nodes, remote URLs, `file:` URLs, and `data:` URLs before `usvg` parse or atlas insertion. `usvg` then normalizes accepted shapes into absolute paths; `resvg` renders the normalized tree into a `tiny-skia` pixmap.

## Role-aware IconVector IR

`data-simthing-role` tags map to deterministic layer roles: `Primary`, `Secondary`, `Accent`, `Outline`, `Background`, and `Mask`. Missing role defaults to `Primary`; unknown roles return `IconError::UnknownRole`.

## PUA codepoint mapping

Icons use Supplementary PUA-A codepoints beginning at `0xF0000`. Codepoints outside `0xF0000..=0xFFFFD` return `IconError::CodepointOutOfRange`.

## Atlas integration

Icons rasterize through `resvg`/`tiny-skia` and insert into the same `GlyphAtlasCore` allocator/backing pixels used by font glyphs. There is no second icon texture or bind group.

## Mixed text+icon run behavior

`IconSet::build_mixed_instances` shapes the source string normally, reads each `ShapedGlyph.cluster` byte offset, and substitutes registered PUA source characters with their icon atlas tile. Font glyphs and icon glyphs return one combined `Vec<GlyphInstanceGpu>`.

## Tests

`crates/simthing-tools/tests/typeface_lr4.rs` — 9 tests, all PASS:

- `normalizes_static_svg_to_icon_vector`
- `rejects_dynamic_or_external_svg`
- `registers_svg_icon_tile`
- `icon_tile_bytes_deterministic`
- `same_icon_same_px_is_cached`
- `invalid_svg_errors_no_panic_no_atlas_mutation`
- `pua_codepoint_renders_in_mixed_run`
- `icon_and_glyph_share_one_atlas`
- `role_tags_are_preserved_in_icon_vector_ir`

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -- --check  PASS
cargo check -p simthing-tools                               PASS
cargo check -p simthing-workshop                            PASS
cargo test -p simthing-workshop --test typeface_lr0         PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr1         PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr2         PASS (8/8)
cargo test -p simthing-tools --test typeface_lr3            PASS (10/10)
cargo test -p simthing-tools --test semantic_free_guard     PASS (1/1)
cargo test -p simthing-tools --test typeface_lr4            PASS (9/9)
git diff --check                                            PASS
cargo tree -p simthing-tools                                PASS (usvg/resvg/tiny-skia present; Bevy/wgpu unchanged)
design ladder evidence rg checks                            PASS
placeholder guard                                           PASS for LR4 changes; existing unrelated pending text remains in studio diagnostic report
dynamic SVG guard                                           PASS by inspection; matches are rejection tests and static-only rejection code only
```

## Files changed

- `Cargo.lock`
- `THIRD_PARTY_LICENSES.md`
- `crates/simthing-tools/Cargo.toml`
- `crates/simthing-tools/src/atlas.rs`
- `crates/simthing-tools/src/icons.rs`
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-tools/tests/typeface_lr4.rs`
- `docs/design_typeface_ladder.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/typeface_lr4_results.md`
- `docs/workshop/studio_production_log.md`

Formatter-only updates were also applied to existing LR3 files required by `cargo fmt -p simthing-tools -- --check`: `crates/simthing-tools/src/bevy.rs`, `crates/simthing-tools/src/text_render.rs`, `crates/simthing-tools/src/wgpu_smoke.rs`, and `crates/simthing-tools/tests/typeface_lr3.rs`.

## Boundary / non-goals

No MSDF, style tables, gradients, deformation, text-on-path, TTF/OTF export, COLRv1, Studio/game label integration, ScenarioSpec/RF/STEAD changes, or Scenario Runtime + Save/Load reopening.

## Known gaps

MSDF icons, dynamic styling, icon-font export, imported icon-pack licensing, and Studio/game label integration remain deferred to later rungs.

## Next recommended action

After LR4, proceed to LR5 high-volume bench or the next owner-selected typeface rung.

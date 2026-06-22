# Third-party licenses

This file records vendored third-party code included in the SimThing workspace.

## jomini (text parsing path only)

| Field | Value |
|---|---|
| **Package** | jomini |
| **Upstream origin** | https://github.com/rakaly/jomini |
| **Vendored version** | v0.34.1 |
| **Vendored commit** | `fff00d8c7f8f06c084d776d1a2c98b34324e64ed` |
| **License** | MIT |
| **Copyright holder** | Nick Babcock (from upstream `Cargo.toml` `authors` at vendored commit; upstream `LICENSE.txt` omits the copyright line) |
| **Vendored path** | `crates/simthing-clausething/src/jomini/` |
| **License file** | `crates/simthing-clausething/src/jomini/LICENSE` |
| **Scope note (CT-0a)** | Only the Clausewitz/ClauseScript **text parsing path** is vendored and used: `TextTape` lexer/parser, DOM readers, scalar/encoding helpers, and text writer. Binary save format, envelope handling, melting, serde derive integration, and incremental `TokenReader` are **excluded**.

## cosmic-text (TYPEFACE-LR1 workshop shaping dependency)

| Field | Value |
|---|---|
| **Package** | cosmic-text |
| **Upstream origin** | https://github.com/pop-os/cosmic-text |
| **Version in workspace** | 0.13.2 (direct `simthing-workshop` dep; also Bevy transitive) |
| **License** | MIT OR Apache-2.0 |
| **Scope note (TYPEFACE-LR1)** | Workshop-only text shaping (`ShapingEngine`); no rendering/atlas integration in LR1 |

## swash (TYPEFACE-LR2 workshop rasterization dependency)

| Field | Value |
|---|---|
| **Package** | swash |
| **Upstream origin** | https://github.com/dfrg/swash |
| **Version in workspace** | 0.2.9 (direct `simthing-workshop` dep; also `cosmic-text` transitive) |
| **License** | MIT |
| **Scope note (TYPEFACE-LR2)** | Workshop-only glyph outline rasterization (`rasterize_glyph_cpu`); no Bevy/`simthing-tools` integration in LR2 |

## guillotiere (TYPEFACE-LR2 workshop atlas packing dependency)

| Field | Value |
|---|---|
| **Package** | guillotiere |
| **Upstream origin** | https://github.com/nical/guillotiere |
| **Version in workspace** | 0.6.2 (direct `simthing-workshop` dep) |
| **License** | Apache-2.0 OR MIT |
| **Scope note (TYPEFACE-LR2)** | Workshop-only atlas rectangle packing for `GlyphAtlas`; no production render path in LR2 |

## Noto Sans Regular (TYPEFACE-LR0 test fixture only)

| Field | Value |
|---|---|
| **Font** | Noto Sans Regular |
| **Upstream origin** | https://github.com/googlefonts/noto-fonts |
| **Fixture path** | `crates/simthing-workshop/assets/typeface/test_font.ttf` |
| **License** | SIL Open Font License 1.1 (OFL) |
| **Copyright holder** | Google LLC and contributors (see upstream OFL header) |
| **Scope note (TYPEFACE-LR0)** | Hermetic workshop test fixture for font load + glyph metrics only. **Not** the bundled default game font decision. Full upstream file committed for deterministic cmap/metrics tests.

## usvg (TYPEFACE-LR4 static SVG normalization dependency)

| Field | Value |
|---|---|
| **Package** | usvg |
| **Upstream origin** | https://github.com/linebender/resvg |
| **Version in workspace** | 0.47.0 (direct `simthing-tools` dep, default features disabled) |
| **License** | Apache-2.0 OR MIT |
| **Scope note (TYPEFACE-LR4)** | Static SVG parse/normalization for hand-authored icon assets only. Dynamic/external SVG behavior is rejected before runtime atlas insertion. |

## resvg (TYPEFACE-LR4 static SVG rasterization dependency)

| Field | Value |
|---|---|
| **Package** | resvg |
| **Upstream origin** | https://github.com/linebender/resvg |
| **Version in workspace** | 0.47.0 (direct `simthing-tools` dep, default features disabled) |
| **License** | Apache-2.0 OR MIT |
| **Scope note (TYPEFACE-LR4)** | Rasterizes already-validated static SVG icon vectors into the shared typeface atlas. No runtime SVG interpretation or external image loading. |

## tiny-skia (TYPEFACE-LR4 static SVG pixmap dependency)

| Field | Value |
|---|---|
| **Package** | tiny-skia |
| **Upstream origin** | https://github.com/linebender/tiny-skia |
| **Version in workspace** | 0.12.0 (direct `simthing-tools` dep with `std` only; Bevy also uses older transitive versions) |
| **License** | BSD-3-Clause |
| **Scope note (TYPEFACE-LR4)** | Provides the pixmap target for `resvg` icon rasterization before atlas insertion. |

## msdf-font (TYPEFACE-LR6 MSDF/SDF generation dependency)

| Field | Value |
|---|---|
| **Package** | msdf-font |
| **Upstream origin** | https://github.com/alexheretic/msdf-font |
| **Version in workspace** | 0.3.1 (direct `simthing-tools` dep, default features disabled) |
| **License** | MIT OR Apache-2.0 |
| **Scope note (TYPEFACE-LR6)** | Import-time TTF glyph MSDF/SDF generation for `DistanceFieldAtlasCore`. CPU oracle only; GPU owns sampling and edge reconstruction. |

## msdf-font (TYPEFACE-LR6A patched vendored copy)

| Field | Value |
|---|---|
| **Package** | msdf-font |
| **Upstream origin** | https://github.com/alexheretic/msdf-font |
| **Version in workspace** | 0.3.1 vendored at `vendor/msdf_font/` via `[patch.crates-io]` |
| **License** | MIT OR Apache-2.0 |
| **Patch** | Adds `GlyphBuilder::build_glyph_id(GlyphId)` for shaped-glyph MSDF without codepoint reverse lookup |
| **Scope note (TYPEFACE-LR6A)** | Minimal upstream-compatible patch; full crate vendored for deterministic builds |

## glam (TYPEFACE-LR6 msdf-font transitive dependency)

| Field | Value |
|---|---|
| **Package** | glam |
| **Upstream origin** | https://github.com/bitshifter/glam-rs |
| **Version in workspace** | 0.30.x (transitive via `msdf-font`) |
| **License** | MIT OR Apache-2.0 |
| **Scope note (TYPEFACE-LR6)** | Math types used internally by `msdf-font` outline processing. |

## linesweeper (TYPEFACE-LR6 msdf-font transitive dependency)

| Field | Value |
|---|---|
| **Package** | linesweeper |
| **Upstream origin** | https://github.com/alexheretic/linesweeper |
| **Version in workspace** | 0.3.0 (transitive via `msdf-font`) |
| **License** | MIT OR Apache-2.0 |
| **Scope note (TYPEFACE-LR6)** | Line-sweep geometry helper used internally by `msdf-font` MSDF generation. |

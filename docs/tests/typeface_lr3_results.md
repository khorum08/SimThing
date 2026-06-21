# TYPEFACE-LR3-SIMTHING-TOOLS-INSTANCED-TEXT-0 Results

## Status

PASS — `simthing-tools` production crate graduated from workshop LR0–LR2 core; Bevy `SimthingToolsTextPlugin` shapes/caches on label change only; instance buffers rebuild on change; semantic-free shader guard passes; workshop LR0/LR1/LR2 shim tests green. LR3 lands at **PROBATION** (DA-sensitive).

## PR / branch / merge

- Branch: `typeface-lr3-simthing-tools-instanced-text-0`

## Mission

1. Close LR2 DA documentation state (Codex approved LR2 after LR2R).
2. Graduate LR0–LR2 typeface core from `simthing-workshop` to `simthing-tools`.
3. Add Bevy instanced text draw foundation with changed-detection and semantic-free shader vocabulary.

## Design ladder sync

- LR2 → **DONE / DA APPROVED** after LR2R remediation (#874, #875).
- LR2R → **DA remediation accepted**.
- LR3 → **DONE / PROBATION** (this PR).
- Typeface track → **OPEN**.

## LR2 DA promotion

Codex DA approved LR2 raster atlas foundation after LR2R adapter-optional CPU test remediation. Evidence rows updated to DA APPROVED / remediation accepted. Whole typeface track remains OPEN.

## Crate move / workshop shim

- New crate: `crates/simthing-tools` (workspace member).
- Moved: `font.rs`, `shaping.rs`, `atlas.rs`, `harness.rs`, `bevy.rs`, `shaders/text_instanced.wgsl`.
- Workshop `typeface/mod.rs` is a thin re-export shim over `simthing_tools`; implementation files removed from workshop.
- Workshop depends on `simthing-tools`; LR0/LR1/LR2 tests unchanged and pass via re-exports.

## Implementation

- `SimthingToolsTextPlugin::new(font_bytes)` — Startup init, changed-label rebuild systems, diagnostics counters.
- `TextLabel { text, px, color }` — shapes/rasterizes on `Added`/`Changed` only.
- `TypefaceAtlas` — wraps `GlyphAtlasCore` CPU atlas for plugin raster cache.
- `TextRebuildDiagnostics` — `shape_rebuild_count`, `instance_rebuild_count` for no-per-frame-shaping proofs.
- `GlyphInstanceGpu` — `pos_size`, `uv_rect`, `color` instance layout (forward-compatible vec4 slots).
- `text_instanced.wgsl` — atlas alpha × instance color; semantic-free vocabulary.
- `ExtractComponent` path for render-world instance handoff (draw pipeline stub for LR3+ render phase).

## Dependency choices

| Crate | Version | Notes |
|---|---|---|
| `bevy` | 0.16.1 | Minimal features: asset/render/core_pipeline/sprite/image/png |
| `skrifa` / `fontdb` / `cosmic-text` / `swash` / `guillotiere` | LR0–LR2 pins | Moved to simthing-tools direct deps |
| `wgpu` | 22.1.0 | Workspace pin (workshop unchanged) |

No forbidden deps (`glyphon`, `usvg`, `resvg`, `fdsm`, `write-fonts`). No Bevy version bump.

## Public API

```rust
pub mod font; pub mod shaping; pub mod atlas; pub mod bevy;
pub use font::{load_font, GlyphMetrics, ProbeFont, TypefaceError};
pub use shaping::{ShapingEngine, ShapedGlyph, ShapedRun, format_shaping_report};
pub use atlas::{GlyphAtlas, GlyphAtlasCore, AtlasTile, GlyphAtlasStats, ...};
pub use bevy::{SimthingToolsTextPlugin, TextLabel, TypefaceAtlas, TextRebuildDiagnostics, GlyphInstanceGpu};
```

## Render path

- Label change → shape (cosmic-text) → rasterize missing glyphs into CPU atlas → build `GlyphInstanceGpu` buffer.
- No-op frames: `Or<(Added<TextLabel>, Changed<TextLabel>)>` filter prevents reshape/rasterize.
- Instance aggregate rebuilt only when labels change.
- WGSL shader registered via `load_internal_asset`; ExtractComponent prepares render-world handoff.

## Shader semantic-free proof

`semantic_free_guard` scans `src/` + `text_instanced.wgsl` for forbidden gameplay tokens; passes with technical allowlist for `dirty_region` atlas identifiers.

## Changed-detection / no-per-frame-shaping proof

- `label_change_rebuilds_instances_once` — counters increment on first build and text change only.
- `cached_label_noop_frame_does_not_reshape` — five no-op frames leave `shape_rebuild_count` unchanged.

## Atlas cache / no-per-frame-raster proof

- `atlas_cache_reused_after_initial_label_build` — `rasterize_count` stable across no-op frames after initial label build.

## GPU adapter status

**REAL_ADAPTER_OBSERVED** — validation host adapter available; smoke PNG written from plugin-built atlas/instances when adapter present.

## Visual smoke artifact

- Path: `docs/tests/typeface_lr3_smoke.png`
- Dimensions: 256×128 RGBA8
- Text: `SimThing` at 32px
- File size: 2370 bytes
- Git blob SHA: `393638ba293095f54eee2860d1661bb586e2fafc`

## Tests

`crates/simthing-tools/tests/typeface_lr3.rs` (7 tests) — all PASS.

`crates/simthing-tools/tests/semantic_free_guard.rs` (1 test) — PASS.

Workshop regression via `workshop_shim_still_passes_lr0_lr1_lr2_tests` — LR0 7/7, LR1 7/7, LR2 8/8 PASS.

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -- --check  PASS
cargo check -p simthing-tools                           PASS
cargo check -p simthing-workshop                        PASS
cargo test -p simthing-workshop --test typeface_lr0     PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr1     PASS (7/7)
cargo test -p simthing-workshop --test typeface_lr2     PASS (8/8)
cargo test -p simthing-tools --test typeface_lr3        PASS (7/7)
cargo test -p simthing-tools --test semantic_free_guard PASS (1/1)
```

## Files changed

- `Cargo.toml`, `Cargo.lock`
- `crates/simthing-tools/**` (new crate)
- `crates/simthing-workshop/Cargo.toml`, `src/typeface/mod.rs` (shim)
- `docs/tests/typeface_lr3_results.md`, `docs/tests/typeface_lr3_smoke.png`
- `docs/design_typeface_ladder.md`, `docs/tests/current_evidence_index.md`, `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No LR4 SVG, no MSDF, no style tables/gradients/deformation, no Studio/game label integration, no ScenarioSpec/RF/STEAD changes.

## Known gaps

- Full Bevy `Transparent2d` instanced draw command not wired (ExtractComponent + shader registered; render queue phase deferred).
- Smoke PNG uses adapter-gated instance/atlas composite proof (not full GPU shader readback yet).
- Atlas eviction / multi-page growth unchanged from LR2.

## DA recommendation

LR3 CPU changed-detection, crate graduation, and semantic-free shader guard are proven. Recommend **PROBATION** pending Codex review of render draw wiring and full GPU shader smoke readback. Do not promote LR3 to DA-approved until render phase completes.

## Next recommended action

LR4 — SVG icon ingestion at PUA codepoints (mechanical), or complete LR3 render draw phase if DA requires full GPU instanced draw before LR4.
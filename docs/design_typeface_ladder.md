# TYPEFACE-LADDER — granular implementation ladder (TTF/OTF + SVG glyph-atlas service)

> **Status: OPEN (owner-approved 2026-06-21).** Companion to `docs/design_simthing_typeface_track_proposal.md`
> (rationale + ecosystem survey). This doc is the **mechanical** ladder: one rung = one PR.
>
> **Owner decisions (locked):** renderer = **MSDF target with a glyphon raster milestone**; home = **prototype
> in `simthing-workshop`, graduate the production service to a new `simthing-tools` crate**.

## Roles & governance
- **DA / orchestrator:** Codex. Chooses rung order, reviews DA-sensitive rungs, writes the only DA sign-off.
- **Implementer:** Grok / Cursor Composer 2.5. Implements exactly one rung per PR, mechanically.
- **One rung = one PR.** PR title = the rung ID. Each rung lands at **PROBATION** with a result report; no rung
  is DA-approved until Codex reviews it; the *track* is DA-approved only after LR8 + a perf re-review.
- **DA-sensitive rungs** (Codex must review before merge): LR2, LR3, LR5, LR6 (anything touching wgpu/shaders/
  perf budgets). **Mechanical rungs** (Composer-merge after green CI): LR0, LR1, LR4, LR7, LR8 docs.

## Shared conventions (every rung)
- **Presentation-only.** No edits to `simthing-spec`/`simthing-driver`/`simthing-sim` scenario/RF/spatial code.
  Do not touch any DA-approved Scenario or STEAD track. No `ScenarioSpec`, RF, GPU-dispatch-into-sim.
- **Semantic-free GPU.** Shader/atlas text contains only glyph/atlas/SDF/instance vocabulary — no gameplay/
  map/faction/economy words. (A forbidden-token guard is added at LR3.)
- **GPU-resident in shape; CPU = build/oracle only.** Shaping + atlas packing happen on text change, never per
  frame. Static labels cache their instance buffer; per-frame work is GPU draw only. (See
  `docs/simthing-bevy-performance.md` — the "never compute in the draw/update path" rule that fixed Studio FPS.)
- **Determinism.** Same input font + string → same shaped run + same atlas tile bytes. Tests assert this.
- **Each rung adds:** `docs/tests/typeface_lr<N>_results.md` (PROBATION) + one row in
  `docs/tests/current_evidence_index.md`; updates `docs/design_typeface_ladder.md` rung status to DONE.
- **Per-rung validation (minimum):** `cargo fmt -p <crate> -- --check`, `cargo check -p <crate>`,
  `cargo test -p <crate> --test <rung tests>`, `git diff --check`. GPU rungs add a real-adapter run and record
  `REAL_ADAPTER_OBSERVED` or `ADAPTER_SKIPPED` honestly. Windows fmt path-length 206 → scoped fmt, recorded.
- **Test font fixture:** vendor one **OFL** font (e.g. a small Noto/DejaVu subset) at
  `crates/simthing-workshop/assets/typeface/test_font.ttf`; record it in `THIRD_PARTY_LICENSES.md`. OFL is
  permissive — this is a test fixture, **not** the still-open "bundled default game font" decision.

---

## LR0 — workshop prototype: font load + metrics + measurement harness  *(mechanical)*
**Crate/files:** `crates/simthing-workshop/src/typeface/mod.rs` (+ `font.rs`); test fixture above.
**Deps (workshop `Cargo.toml`):** add **direct** `skrifa`, `fontdb`, `thiserror` (already transitive — pin to
the versions Bevy 0.16 resolves in `Cargo.lock`; do not bump).
**Public API:**
```rust
pub struct ProbeFont { /* owns font bytes + skrifa FontRef-equivalent */ }
#[derive(Debug, Clone, Copy, PartialEq)] pub struct GlyphMetrics { pub advance: f32, pub bounds: [f32;4], pub glyph_id: u16 }
#[derive(Debug, thiserror::Error)] pub enum TypefaceError { #[error("font parse: {0}")] Parse(String), /* ... */ }
pub fn load_font(bytes: &[u8]) -> Result<ProbeFont, TypefaceError>;
impl ProbeFont { pub fn units_per_em(&self) -> u16; pub fn glyph_metrics(&self, ch: char) -> Option<GlyphMetrics>; pub fn glyph_count(&self) -> u16; }
```
**Steps:** load fixture bytes → `skrifa` font → expose metrics by char via cmap; return `None` for unmapped.
**Tests** (`crates/simthing-workshop/tests/typeface_lr0.rs`): `loads_fixture_font_units_per_em`,
`glyph_metrics_for_known_ascii_is_stable`, `unmapped_char_returns_none`, `glyph_count_positive`,
`load_garbage_bytes_errors`.
**Stop conditions:** if `skrifa`/`fontdb` versions in lock are incompatible with a direct dep declaration →
STOP, report the version, do not bump Bevy's transitive pins.
**Boundary:** no wgpu, no Bevy, no rendering.

## LR1 — shaping engine (cosmic-text)  *(mechanical)*
**Files:** `crates/simthing-workshop/src/typeface/shaping.rs`.
**Deps:** add `cosmic-text` (lock-pinned).
**Public API:**
```rust
#[derive(Debug, Clone, PartialEq)] pub struct ShapedGlyph { pub glyph_id: u16, pub x: f32, pub y: f32, pub advance: f32, pub cluster: usize }
#[derive(Debug, Clone, PartialEq)] pub struct ShapedRun { pub glyphs: Vec<ShapedGlyph>, pub width: f32, pub height: f32 }
pub struct ShapingEngine { /* cosmic_text FontSystem + cache */ }
impl ShapingEngine { pub fn new_with_font(bytes: Vec<u8>) -> Result<Self, TypefaceError>; pub fn shape(&mut self, text: &str, px: f32) -> ShapedRun; }
```
**Steps:** feed the fixture font into a `cosmic_text::FontSystem`; shape a single-line `&str` at a px size;
emit positioned glyph runs (kerning/ligatures applied by cosmic-text).
**Tests** (`typeface_lr1.rs`): `shapes_ascii_advances_monotonic`, `kerning_pair_AV_tighter_than_naive`,
`ligature_fi_collapses_when_font_has_it_else_two_glyphs`, `empty_string_is_empty_run`,
`shaping_is_deterministic` (shape twice → equal).
**Boundary:** no atlas, no GPU.

## LR2 — raster glyph atlas v1 (swash + guillotiere, headless wgpu)  *(DA-sensitive)*
**Files:** `crates/simthing-workshop/src/typeface/atlas.rs`.
**Deps:** add `swash`, `guillotiere`, `wgpu`/`bytemuck`/`pollster` (workshop already has wgpu stack).
**Public API:**
```rust
#[derive(Debug, Clone, Copy, PartialEq)] pub struct AtlasTile { pub x: u32, pub y: u32, pub w: u32, pub h: u32, pub left: i32, pub top: i32 }
pub struct GlyphAtlas { /* guillotiere allocator + CPU staging RGBA8 + wgpu::Texture */ }
impl GlyphAtlas {
  pub fn new(device:&wgpu::Device, size:u32) -> Self;
  pub fn get_or_rasterize(&mut self, font:&ProbeFont, glyph_id:u16, px:f32) -> Option<AtlasTile>; // cache key (glyph_id, px-bucket)
  pub fn upload(&mut self, queue:&wgpu::Queue);                 // dirty-region upload only
  pub fn texture_view(&self) -> &wgpu::TextureView;
}
```
**Steps:** rasterize a glyph outline (swash) to an alpha/RGBA bitmap; pack with `guillotiere`; stage into a CPU
buffer; upload dirty rows to a wgpu texture. Cache by `(glyph_id, quantized px)`. On atlas-full → return `None`
(LR-later eviction; record as a known gap).
**Tests** (`typeface_lr2.rs`): `rasterized_tile_bytes_match_cpu_oracle` (CPU-readback byte-exact vs swash
direct), `same_glyph_same_px_is_cached_not_re_rasterized` (alloc count unchanged on 2nd call),
`distinct_glyphs_get_distinct_tiles`, `atlas_full_returns_none_no_panic`, headless real-adapter upload+readback.
**DA focus:** byte-exact oracle; cache key correctness; **no per-call full-atlas upload** (dirty-region only).
**Stop conditions:** no GPU adapter in env → keep CPU-oracle tests, mark GPU tests `ADAPTER_SKIPPED`, do not fake.

## LR3 — `simthing-tools` crate + Bevy instanced text draw  *(DA-sensitive — graduation rung)*
**New crate:** `crates/simthing-tools/` (`Cargo.toml`, `src/lib.rs`); move the proven LR0–LR2 modules
(`font`, `shaping`, `atlas`) **out of workshop into `simthing-tools`** (workshop keeps only a thin
prototype/bench shim re-using the crate; workshop stays a non-production dep). Add `simthing-tools` to the
workspace members + `Cargo.toml` of consumers as needed.
**Files:** `crates/simthing-tools/src/{lib,font,shaping,atlas,bevy}.rs`; shader
`crates/simthing-tools/src/shaders/text_instanced.wgsl`.
**Deps:** `bevy = { version = "0.16", default-features = false, features=[...] }` (match Studio's feature set);
reuse atlas/shaping deps.
**Public API:**
```rust
pub struct SimthingToolsTextPlugin;            // Bevy Plugin
#[derive(Component)] pub struct TextLabel { pub text: String, pub px: f32, pub color: [f32;4] } // world or UI
#[derive(Resource)] pub struct TypefaceAtlas { /* GlyphAtlas + bind group */ }
// instance = vec4 pos/size + vec4 uv + vec4 color; one instanced quad draw per atlas.
```
**Shader (`text_instanced.wgsl`):** sample the atlas alpha; multiply by instance color; **semantic-free**
(struct names: `GlyphInstance`, `AtlasParams`, fields uv/pos/color/scale only).
**Steps:** shape `TextLabel` on change (changed-detection, NOT every frame) → build instance buffer → one
instanced draw using the atlas bind group. One UI label + one `Text2d`-style world label on screen.
**Tests** (`crates/simthing-tools/tests/typeface_lr3.rs`): `plugin_builds_headless_app`,
`label_change_rebuilds_instances_once` (instrument a rebuild counter; assert it does **not** increment on a
no-op frame), `instances_match_shaped_run_count`, plus a visual smoke PNG (headless render → committed under
`docs/tests/`). **Forbidden-token guard:** `crates/simthing-tools/tests/semantic_free_guard.rs` scanning the
wgsl + src for gameplay tokens (route/faction/economy/combat/planet/fleet/owner/...).
**DA focus:** the changed-detection (no per-frame shaping), the instanced single-draw, semantic-free shader,
workshop→tools move leaves no production dep on workshop.
**Stop conditions:** if the workshop→tools move would touch any non-typeface workshop code → STOP, scope to a
clean move PR first.

## LR4 — SVG icon ingestion (usvg + resvg) at PUA codepoints  *(mechanical)*
**Files:** `crates/simthing-tools/src/icons.rs`.
**Deps:** `usvg`, `resvg` (tiny-skia already in tree). Record MPL-2.0 in `THIRD_PARTY_LICENSES.md`.
**Public API:**
```rust
pub const ICON_PUA_START: u32 = 0xF0000; // Supplementary PUA-A
pub struct IconSet { /* codepoint -> raster tile in the SAME GlyphAtlas */ }
impl IconSet { pub fn register_svg(&mut self, codepoint:u32, svg:&str, px:f32, atlas:&mut GlyphAtlas) -> Result<AtlasTile, TypefaceError>; }
```
**Steps:** parse SVG (`usvg`) → rasterize (`resvg`+tiny-skia) at px → insert into the glyph atlas keyed by the
PUA codepoint; shaping passes PUA chars through as single glyphs so `"Sol \u{F0001} 42"` renders text+icon in
one run.
**Tests** (`typeface_lr4.rs`): `registers_svg_icon_tile`, `pua_codepoint_renders_in_mixed_run`,
`icon_tile_bytes_deterministic`, `invalid_svg_errors_no_panic`, `icon_and_glyph_share_one_atlas` (one bind).
**Boundary:** raster only (MSDF icons arrive with LR6).

## LR5 — high-volume bench + damage-text budget  *(DA-sensitive — perf gate)*
**Files:** `crates/simthing-tools/benches/` or `crates/simthing-tools/src/bin/typeface_bench.rs` +
`crates/simthing-tools/tests/typeface_lr5.rs`.
**Steps:** spawn **N animated labels** (scale/fade/rise) from one emitter; reuse cached shaped runs where text
is identical (damage numbers reuse glyph tiles); measure **CPU build time/frame** and **draw-call count**;
record on a real adapter.
**Binding budget (PASS):** **≥5,000** simultaneous animated labels at **≥60 FPS**, **CPU build < 1 ms/frame**,
**draw calls bounded** (single atlas bind; instanced). No per-glyph entities. No per-frame re-shape of
unchanged text.
**Tests:** `five_thousand_labels_cpu_build_under_budget` (assert measured CPU build < threshold),
`unchanged_label_text_does_not_reshape`, `bench_records_real_adapter_numbers`.
**DA focus:** the numbers are real, recorded, on a real adapter; the budget is met or the rung is HELD with a
specific bottleneck.

## LR6 — MSDF atlas (vector target) + SDF shader  *(DA-sensitive — graduation of scalability)*
**Files:** `crates/simthing-tools/src/msdf.rs`, `src/shaders/text_msdf.wgsl`.
**Deps:** add a **pure-Rust MSDF generator** — `fdsm` (generates MSDF from `ttf-parser`/outline data). If
`fdsm` cannot consume `skrifa`/`usvg` outlines directly, adapt outlines to its input; if that adaptation is
non-trivial → STOP and bring options to DA (do **not** add a C++ `msdfgen` build dependency without DA sign-off).
**Public API:** mirror `GlyphAtlas` as `MsdfAtlas` with the **same** `get_or_rasterize`-shaped API so the LR3
draw path swaps renderer behind one trait:
```rust
pub trait GlyphSource { fn tile(&mut self, key: GlyphKey) -> Option<AtlasTile>; fn texture_view(&self)->&wgpu::TextureView; }
```
**Steps:** generate one MSDF tile per glyph **and** per SVG-icon outline (icons share the MSDF atlas); a single
instanced SDF shader renders crisp at arbitrary scale (median-of-3 channels, screen-px-range AA). Swap the LR3
plugin to `GlyphSource = MsdfAtlas` behind a feature/flag; re-run the LR5 bench.
**Tests** (`typeface_lr6.rs`): `msdf_tile_deterministic`, `msdf_renders_crisp_across_scale_sweep` (visual PNG
at 8/32/128 px from ONE tile), `icons_and_glyphs_in_one_msdf_atlas`, `lr5_budget_still_met_or_better_with_msdf`.
**DA focus:** no C++ dep without sign-off; one atlas/one shader for glyphs+icons; LR5 budget preserved.

## LR7 — custom character set / icon-font manifest  *(mechanical)*
**Files:** `crates/simthing-tools/src/manifest.rs`; example manifest `assets/typeface/icons.ron`.
**Public API:** declarative manifest `{ codepoint: u32, svg_path: String, name: String }[]` → build-time bake
into the (MSDF) atlas; a stable `codepoint ↔ name` table.
**Tests** (`typeface_lr7.rs`): `manifest_bakes_all_icons`, `codepoint_table_is_stable` (golden),
`duplicate_codepoint_rejected`, `missing_svg_path_errors`.
**Owner input needed before this rung:** icon source set + reserved PUA range (still-open item #4).

## LR8 — Studio + game label seam  *(mechanical + DA docs)*
**Files:** `crates/simthing-mapeditor/src/app/labels.rs` (Studio consumer); a `DamageText`-style emitter
component in `simthing-tools`.
**Steps:** world-space entity-name labels (planet/system names) scaled by camera distance, fed from the
existing Studio view model (presentation-only, no authority); a damage-text emitter component. Studio smoke:
labels render; perf within the LR5 budget.
**Tests:** `studio_entity_labels_render_headless`, `labels_are_presentation_only_no_authority_touch`,
`damage_text_emitter_respects_lr5_budget`.
**Boundary:** labels read the Studio projection only; never mutate ScenarioSpec or any authority.

---

## Perf budget appendix (binding on LR5/LR6)
- ≥5,000 animated labels @ ≥60 FPS, CPU build < 1 ms/frame, bounded draw calls, single atlas bind, instanced.
- **Anti-patterns to reject in review:** per-glyph `Text2d`/entity spawning for high-volume text; per-frame
  shaping/raster of unchanged text; full-atlas re-upload per glyph; CPU work in the egui/Update draw path.

## Ladder status
| Rung | Title | DA-sensitive | Status |
|---|---|---|---|
| LR0 | workshop font load + metrics | no | TODO |
| LR1 | shaping engine | no | TODO |
| LR2 | raster glyph atlas v1 | **yes** | TODO |
| LR3 | simthing-tools crate + Bevy instanced draw | **yes** | TODO |
| LR4 | SVG icons at PUA codepoints | no | TODO |
| LR5 | high-volume bench + budget | **yes** | TODO |
| LR6 | MSDF atlas + SDF shader | **yes** | TODO |
| LR7 | icon-font manifest | no | TODO |
| LR8 | Studio + game label seam | no | TODO |

**Non-goals (whole track):** ScenarioSpec/RF/spatial changes, GPU dispatch into sim, persistent history,
pathfinding/combat/economy/fleet movement, new savefile format, DA promotion of any non-typeface row.

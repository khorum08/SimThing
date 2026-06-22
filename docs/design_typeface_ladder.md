# TYPEFACE-LADDER — granular implementation ladder (TTF/OTF + SVG glyph-atlas service)

> **Status: CLOSED / DA-APPROVED (Opus track closure 2026-06-22; closeout PR #897 `974ffcc7fc`).** Companion to `docs/design_simthing_typeface_track_proposal.md`
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

## LR0 — workshop prototype: font load + metrics + measurement harness  *(mechanical)* — **DONE / PROBATION**
**Status:** landed PR #872 (`c24de50cc`); result report `docs/tests/typeface_lr0_results.md`. Track remains OPEN — LR0 is not DA-approval of the whole typeface track.
**Crate/files:** `crates/simthing-workshop/src/typeface/mod.rs` (+ `font.rs`, `harness.rs`); test fixture above.
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

## LR1 — shaping engine (cosmic-text)  *(mechanical)* — **DONE / PROBATION**
**Status:** landed PR #873 (`b913e51ac`); result report `docs/tests/typeface_lr1_results.md`. Track remains OPEN — LR1 is not DA-approval of the whole typeface track.
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

## LR2 — raster glyph atlas v1 (swash + guillotiere, headless wgpu)  *(DA-sensitive)* — **DONE / DA APPROVED**
**Status:** landed PR #874 (`12dd92023`); remediation PR #875 (`d547e8cf7`); result reports `docs/tests/typeface_lr2_results.md`, `docs/tests/typeface_lr2r_results.md`. **Codex DA approved LR2 after LR2R adapter-optional remediation.** Track remains OPEN — LR2 approval is raster-atlas foundation only, not whole-track DA approval.
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
**Remediation:** `TYPEFACE-LR2-RASTER-ATLAS-0R` (PR #875) remediated DA HOLD on adapter-optional CPU test coverage — splits `GlyphAtlasCore` (CPU-only) from GPU-backed `GlyphAtlas`; **DA remediation accepted**; result report `docs/tests/typeface_lr2r_results.md`.

## LR3 — `simthing-tools` crate + Bevy instanced text draw  *(DA-sensitive — graduation rung)* — **DONE / DA APPROVED**
**Status:** landed PR #876 (`a4f8c7dfa`); DA remediation accepted via **TYPEFACE-LR3-INSTANCED-DRAW-0R** PR #877 (`0ec42e5175`). Route B raw-wgpu shader-backed smoke is accepted. Track remains OPEN — LR3 approval is not whole-track DA approval.
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

**LR3R closeout:** `TYPEFACE-LR3-INSTANCED-DRAW-0R` accepted Route B raw-wgpu shader-backed smoke as the LR3 draw remediation. Full in-Bevy PNG readback remains explicitly **DEFERRED**: `Camera2d + Tonemapping::None + RenderTarget::Image + gpu_readback::Readback`.

## LR4 — SVG icon ingestion (usvg + resvg) at PUA codepoints  *(mechanical)* — **DONE / ACCEPTED (#878)**
**Status:** `TYPEFACE-LR4-SVG-PUA-ICON-INGESTION-0` accepted/closed; PR #878 merge `990d6ce5ce804523564fe65e56725ece23a7a37d`; post-merge evidence commit `7c8cb1bd15`; result report `docs/tests/typeface_lr4_results.md`. Track remains OPEN — LR4 acceptance is not whole-track DA approval.
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

**Amendment folded into LR4:** LR4 includes static-SVG normalization plus a role-aware `IconVector` IR. The ingestion path accepts static SVG only, normalizes accepted shapes to deterministic path/layer records, rejects scripts, external images, animation/events, and remote resources, preserves optional `data-simthing-role` tags (`primary`, `secondary`, `accent`, `outline`, `background`, `mask`), and keeps deterministic layer/path ordering. Runtime never interprets SVG; it consumes rasterized atlas tiles and icon metadata only.

## LR5 — high-volume bench + damage-text budget  *(DA-sensitive — perf gate)* — **DONE / DA APPROVED**
**Status:** `TYPEFACE-LR5-HIGH-VOLUME-BENCH-BUDGET-0` **DA APPROVED** after LR5R/LR5S/LR5T (#879–#882). Final remediation: `TYPEFACE-LR5-NUMERIC-DAMAGE-LANE-0R` (PR #882, merge `c05baef87bc`). Result reports `docs/tests/typeface_lr5_results.md` through `docs/tests/typeface_lr5t_results.md`. Track remains OPEN — LR6 is active.
**Files:** `crates/simthing-tools/src/bench.rs`, `bevy.rs`, `numeric_damage.rs`, `text_render.rs`; `crates/simthing-tools/tests/typeface_lr5.rs`.
**Steps:** CPU harness plus Bevy-path aggregate versioning, dirty atlas sync, draw-entity sync gating, instance-buffer reuse; 5k no-op + fixed-width numeric damage binding profile recorded.
**LR5R remediation:** dirty aggregate rebuild; no-op draw sync/atlas sync/buffer recreate avoided; 5k labels @ avg no-op &lt;1 ms CPU update on validation host.
**LR5S remediation:** no-clone changed-label rebuild; segmented aggregate patching; numeric shape cache + digit prewarm; variable-width damage ~2.26 ms/frame (historical).
**LR5T remediation:** `NumericDamageLabel` + import-time glyph table; fixed-width `-####` lane bypasses cosmic-text per frame; 5k binding avg damage **~0.58 ms/frame** — accepted as final LR5 remediation.
**Tests:** LR5 direct harness + Bevy structural tests in `typeface_lr5.rs`; optional `#[ignore]` 5k binding profile.
**DA focus:** no-op and fixed-width numeric damage bindings met; LR5 closed for MSDF foundation work.

## LR6 — MSDF atlas (vector target) + SDF shader  *(DA-sensitive — graduation of scalability)* — **DONE / DA APPROVED for production glyph MSDF after LR6A**
**Status:** `TYPEFACE-LR6-MSDF-ATLAS-SHADER-0` (#883) foundation + `TYPEFACE-LR6A-PRODUCTION-MSDF-WIRING-0R` (#884) production glyph MSDF opt-in accepted. See `docs/tests/typeface_lr6_results.md`, `docs/tests/typeface_lr6a_results.md`.

## LR6A — production MSDF wiring  *(DA remediation)* — **DONE / PROBATION / ACCEPTED for glyph MSDF production opt-in**
**Status:** `TYPEFACE-LR6A-PRODUCTION-MSDF-WIRING-0R` (#884) — `TextLabelRenderMode` opt-in, shared-atlas MSDF packing, patched `build_glyph_id`. See `docs/tests/typeface_lr6a_results.md`.

## LR6A-ICON — icon vector geometry bridge  *(DA remediation before LR6B)* — **DONE / ACCEPTED**
**Status:** `TYPEFACE-LR6A-ICON-VECTOR-GEOMETRY-0R` (#885, `a3f7dcd30d`) — normalized `IconVector` geometry IR, icon MSDF from bezpath, per-role raster style-slot refs. See `docs/tests/typeface_lr6a_icon_geometry_results.md`.

## LR6B — GPU style table + gradient/effect shader  *(DA-sensitive)* — **DONE / DA APPROVED**
**Status:** `TYPEFACE-LR6B-GPU-STYLE-TABLE-EFFECTS-0` (#886) + buffer residency remediation `TYPEFACE-LR6B-STYLE-BUFFER-RESIDENCY-0R` (#887, `6117dd5fd5`). See `docs/tests/typeface_lr6b_results.md` and `docs/tests/typeface_lr6b_style_buffer_residency_results.md`. **DA APPROVED** after #887 (post-merge evidence `19ca9f8420`, index finalize `79ee674026`).

## LR6C — adaptive-tessellation glyph mesh + parametric deform  *(DA-sensitive)* — **DONE / DA APPROVED**
**Status:** `TYPEFACE-LR6C-ATLAS-RESIDENCY-DEFORM-0` (#888) + UV-sampling remediation `TYPEFACE-LR6C-DEFORM-UV-SAMPLING-0R` (#889, merge `913b148323`). See `docs/tests/typeface_lr6c_results.md` and `docs/tests/typeface_lr6c_deform_uv_sampling_results.md`. **DA APPROVED** after #889 (post-merge evidence `2e83ff80c8`, index finalize `581fe06f84`).

## LR6D — text-on-path + warp field / control lattice  *(DA-sensitive)* — **DONE / DA APPROVED**
**Status:** `TYPEFACE-LR6D-TEXT-ON-PATH-WARP-FIELD-0` (#890, merge `c0fb11c3cb`) + combined proof remediation `TYPEFACE-LR6D-COMBINED-MSDF-DEFORM-PROOF-0R` (#891, merge `ffc4bb6891`, post-merge evidence `6a32763bdd`). See `docs/tests/typeface_lr6d_results.md` and `docs/tests/typeface_lr6d_combined_msdf_deform_results.md`. **DA APPROVED** after combined MSDF/deform proof.

## LR7 — custom character set / icon-font manifest  *(mechanical)* — **DONE / DA APPROVED for manifest machinery**
**Status:** `TYPEFACE-LR7-ICON-FONT-MANIFEST-0` (#892, merge `ac320204eb`, post-merge evidence `be8dde2388`) — declarative RON manifest, stable name ↔ PUA codepoint table, fixture icon set baked through existing `IconSet` static-SVG path. See `docs/tests/typeface_lr7_results.md`. **DA APPROVED for manifest machinery** — production icon source set remains input debt. TTF/OTF export deferred optional LR7A.
**Files:** `crates/simthing-tools/src/manifest.rs`; fixture manifest `crates/simthing-tools/assets/typeface/icons/manifest.ron`.
**Public API:** `load_icon_manifest`, `bake_icon_manifest`, `IconManifest`, `IconManifestBake`.
**Tests** (`typeface_lr7.rs`): manifest load/bake, golden codepoint table, duplicate/range/path validation, role-layer preservation, mixed text+icon run, no runtime SVG dependency.

## LR8 — Studio + game label seam  *(mechanical + DA docs)* — **DONE / DA APPROVED**
**Status:** `TYPEFACE-LR8-STUDIO-LABEL-SEAM-0` (#893, merge `ec01da43c9`, post-merge evidence `22e7bfb9a4`) — `StudioTypefaceLabel` adapter, damage emitter, fixture manifest icon resolve, typeface component sync. **Shell mount remediation:** `TYPEFACE-LR8-STUDIO-PLUGIN-MOUNT-0R` (#894, merge `850a216a7a`) — `StudioTypefaceShellPlugin` in `run_studio()`. **DA APPROVED after #894.** See `docs/tests/typeface_lr8_results.md` and `docs/tests/typeface_lr8_studio_plugin_mount_results.md`.
**Files:** `crates/simthing-tools/src/studio_labels.rs`, `crates/simthing-mapeditor/src/studio_typeface_shell.rs`.
**Tests** (`typeface_lr8.rs`): spawn/sync, shell mount, manifest icon, damage path, GPU residency doc check.

## LR9 — dynamic style + animated + warped perf gate  *(DA-sensitive)* — **DONE / architecturally DA-approved (#896)**
**Status:** `TYPEFACE-LR9-FINAL-PERF-GATE-0` (#895, merge `c5b5faeab2`, post-merge evidence `7fa3a4dc91`) — structured scenario profiles. **Binding remediation:** `TYPEFACE-LR9-BINDING-PERF-EVIDENCE-0R` (#896, merge `bda6147c95`) — 5k flat/numeric + 256 warped binding profiles recorded. **Closeout blocker:** `TYPEFACE-CLOSEOUT-PERF-INVARIANT-0` — changed-label rebuild allocation invariant green. See `docs/tests/typeface_lr9_results.md`, `docs/tests/typeface_lr9_binding_perf_results.md`, `docs/tests/typeface_closeout_perf_invariant_results.md`. **Architecturally DA-approved after #896** — track remains OPEN pending closeout review.
**Files:** `crates/simthing-tools/src/lr9.rs`, `crates/simthing-tools/tests/typeface_lr9.rs`.
**Tests:** CI smoke at 1k/100 scale + `#[ignore]` 5k binding profiles; LR0–LR8 regressions retained.
**Boundary:** perf gate only — no track closure, no font export, no sim scope.

---

## Perf budget appendix (binding on LR5/LR6)
- ≥5,000 animated labels @ ≥60 FPS, CPU build < 1 ms/frame, bounded draw calls, single atlas bind, instanced.
- **Anti-patterns to reject in review:** per-glyph `Text2d`/entity spawning for high-volume text; per-frame
  shaping/raster of unchanged text; full-atlas re-upload per glyph; CPU work in the egui/Update draw path.

## Ladder status
| Rung | Title | DA-sensitive | Status |
|---|---|---|---|
| LR0 | workshop font load + metrics | no | **DONE / PROBATION** (#872) |
| LR1 | shaping engine | no | **DONE / PROBATION** (#873) |
| LR2 | raster glyph atlas v1 | **yes** | **DONE / DA APPROVED** (#874, #875) |
| LR3 | simthing-tools crate + Bevy instanced draw | **yes** | **DONE / DA APPROVED** (#876, #877 LR3R accepted) |
| LR4 | SVG icons at PUA codepoints | no | **DONE / ACCEPTED (#878)** |
| LR5 | high-volume bench + budget | **yes** | **DONE / DA APPROVED (#879, #880, #881, #882)** |
| LR6 | MSDF atlas + SDF shader | **yes** | **DONE / DA APPROVED for production glyph MSDF after LR6A** |
| LR6A | production MSDF wiring | **yes** | **DONE / PROBATION / ACCEPTED for glyph MSDF production opt-in** |
| LR6A-ICON | icon vector geometry bridge | **yes** | **DONE / ACCEPTED (#885)** |
| LR6B | GPU style table + gradient/effect shader | **yes** | **DONE / DA APPROVED (#886, #887)** |
| LR6C | adaptive-tessellation glyph mesh + parametric deform | **yes** | **DONE / DA APPROVED (#888, #889)** |
| LR6D | text-on-path + warp field / control lattice | **yes** | **DONE / DA APPROVED** (#890, #891 combined proof) |
| LR7 | icon-font manifest | no | **DONE / DA APPROVED for manifest machinery** (#892) |
| LR8 | Studio + game label seam + shell mount | no | **DONE / DA APPROVED** (#893 + #894) |
| LR9 | dynamic style + animated + warped perf gate | **yes** | **DONE / architecturally DA-approved (#896)** — closeout invariant remediated |

**Non-goals (whole track):** ScenarioSpec/RF/spatial changes, GPU dispatch into sim, persistent history,
pathfinding/combat/economy/fleet movement, new savefile format, DA promotion of any non-typeface row.

---

# TYPEFACE-LADDER-AMEND-DYNAMIC-STYLE-0 (DA amendment, owner-confirmed 2026-06-21)

Amends the ladder for realtime GPU styling, gradient nameplates, and glyph **deformation** (squash/stretch/
fold + EU4/Stellaris-style border-conforming curved/twisted labels). LR0–LR3 are unchanged in intent; this
amendment locks the LR3 buffer layout and inserts styling/deformation rungs after LR6.

## Owner decisions (locked)
- **Gradient scope:** **whole-label** gradient is primary (the diplomacy nameplate fades across the whole
  name); **per-glyph** gradient is a style flag. → LR3 instance buffer must carry label-local UV + label-rect.
- **Icon source:** **hand-authored SimThing SVGs first**; imported packs later behind a license review.
- **Font export:** **deferred** — runtime treats icons as PUA glyphs; a real `.ttf`/`.otf` exporter
  (`write-fonts`) is an optional later rung (LR7A), not on the critical path.
- **Morphing:** **IN**, but as **mesh deformation, never outline regeneration** (see doctrine below). Heavily
  used; must be performant.

## First handoff = the amend, then LR0
Per Codex Q6: this amendment lands first (docs only), then implementation starts at LR0 exactly as written.

## DEFORMATION DOCTRINE (binding — this is how morphing stays static-safe and fast)
Morphing **never** regenerates glyph outlines at runtime. The glyph identity + MSDF tile stay **static**; what
deforms is the **mesh the MSDF tile is painted onto**, evaluated in the **vertex shader**. This keeps
cosmic-text/skrifa/MSDF static (the owner's "must not undermine the underlying static-geometric packages") and
keeps crispness (each fragment still samples the static MSDF; AA uses derivative-based screen px-range so edges
stay sharp under stretch/curve).

- **Adaptive tessellation (budget knob):** a glyph defaults to **1 quad** (flat text, damage numbers → 2 tris,
  max throughput). A label that requests warp/curve tessellates its glyphs into an N×M grid so deformation has
  vertices to move. **You pay vertices only where warp is used.**
- **Tier 1 — parametric deform** (squash/stretch/skew/fold/pulse/scale): per-instance params (scale_x/y, shear,
  fold_axis, fold_amount, time_phase) applied in the vertex shader. Animatable from a global time uniform; no
  CPU per frame.
- **Tier 2 — path/region warp** (border-conforming names): label laid out along a **spline/path** (text-on-
  path) and/or a coarse **2D/3D warp field** (control-point lattice / Bézier patch defining the region the
  text fills). The spline/field mapping is computed on the **CPU once on text/path change and cached**; the
  per-vertex warp (position + tangent rotation + cross-curve bend, 3D-displaced if needed) runs in the vertex
  shader. EU4/Stellaris empire-name behavior = Tier-2 over tessellated glyph meshes.
- **Shader stays semantic-free:** style/deform inputs are `style_slot`, `deform_id`, `warp_id`, `effect_flags`,
  `time`, `local_uv`, `label_rect`, gradient stops, control points — never "diplomacy/owner/faction/border".
- **Perf rule:** flat labels stay 1-quad instanced; warped labels are fewer/persistent and cached; per-frame is
  GPU vertex transform only. Tessellation level + warp params are part of the LR9 budget.

## LR3 amendment (lock the instance/vertex layout now)
The LR3 instance/vertex format **must reserve**, even though styling lands later: `style_slot: u32`,
label-local UV + `label_rect`, a per-glyph affine + `deform_id`/`warp_id`, and an adaptive-tessellation hook
(default level 0 = 1 quad). This prevents an LR6x refactor of the draw path.

## Inserted / amended rungs
| Rung | Title | DA-sensitive | Notes |
|---|---|---|---|
| LR4 (expanded) | SVG icons at PUA codepoints **+ normalization/style-role IR** | **yes** | folds Codex "LR4A": static-SVG-only, normalize to paths, reject scripts/anim, optional role tags (fill/outline/accent), deterministic order |
| **LR6B** (new) | GPU style table + gradient/effect shader | **yes** | `GlyphStyle`{fill_mode, palette_a/b, gradient_id, effect_flags, alpha, outline_px, glow_px, time_phase} + `GradientStop`; whole-label gradient primary; layered icons = multiple style slots |
| **LR6C** (new) | adaptive-tessellation glyph mesh + parametric deform | **yes** | Tier-1 squash/stretch/skew/fold/pulse over static MSDF; derivative AA; flat default = 1 quad |
| **LR6D** (new) | text-on-path + 2D/3D warp field | **yes** | Tier-2 border-conforming curved/twisted nameplates; CPU path-layout cached on change; vertex-shader warp |
| LR8 (expanded) | Studio + game label seam **+ map-view style binding** | no | folds Codex "LR8A": nameplates read Studio projection (name/icons/overlay value/map-view mode) → write `LabelStyleRef`/style-buffer update; diplomacy red→blue gradient example; authority-boundary safe |
| **LR9** (new) | dynamic style + animated + **warped** perf gate | **yes** | 5k flat animated labels (damage text) AND a warped/curved nameplate set with tessellation, dynamic gradients, glow/pulse; bounded draw calls; CPU build/update < 1 ms/frame; adaptive-tessellation budget |
| LR7A (deferred) | SVG/manifest → `.ttf`/`.otf` exporter (`write-fonts`) | n/a | optional interchange asset; spec only if an external-font need appears |
| LR10 (deferred) | COLRv1 / variable-color export feasibility | n/a | export/interchange only; never the runtime path |

## Amended sequence
`LR0 → LR1 → LR2 → LR3(+layout lock) → LR4(+IR) → LR5 → LR6 → LR6B → LR6C → LR6D → LR7 → LR8(+style binding)
→ LR9`. Deferred/optional: LR7A, LR10. DA-sensitive: LR2, LR3, LR4, LR5, LR6, LR6B, LR6C, LR6D, LR9.

## Why this is not scope-bloat vs Codex's 15-rung proposal
Codex's concepts are accepted but compressed: LR4A→folded into LR4; LR6A→folded into LR6B (layers = style
slots); LR8A→folded into LR8; LR7A/LR10→deferred (not committed). The genuinely new committed work is the
styling rung (LR6B), the two deformation rungs (LR6C/LR6D — the owner's morphing requirement), and the perf
gate (LR9). Net committed rungs: 11; deferred optional: 2.

---

# TYPEFACE TRACK — DA CLOSURE (Opus, executive design authority, 2026-06-22)

**Decision: CLOSED / DA-APPROVED.** The full `cargo test -p simthing-tools` suite is green on a clean master
(`f6e0fb24fc`); the final closure blocker (`changed_label_rebuild_does_not_clone_old_instance_vec`) was
remediated in **PR #897 `974ffcc7fc`** (TYPEFACE-CLOSEOUT-PERF-INVARIANT-0 — replaced an icon-path
`extend_from_slice` memcpy with a move assignment; numeric/aggregate lanes unchanged; source-grep guard
retained). LR9 binding evidence (#896) stands: flat 5k noop avg 0.5037 ms, numeric noop avg 0.3260 ms,
warped 0.0683 ms.

**Rungs promoted PROBATION → CLOSED / DA-APPROVED:** LR0, LR1, LR2, LR3, LR4, LR5, LR6, LR6A, LR6A-ICON, LR6B,
LR6C, LR6D, LR7, LR8, LR9, plus the LR6B style-buffer-residency-0R and LR2R remediations.

**Doctrines verified preserved:** GPU-residency (style table, atlas bind group, MSDF, deform/path buffers all
persistent; numeric production authority GPU-resident); semantic-free shader (guard passes); no runtime SVG/
manifest reload; no bespoke *runtime* renderer (Route B was a headless test harness only). The architecture
delivers the owner's targets: EU4/EU5-style warp/bend/twist nameplates as vertex-shader deformation over static
MSDF (no CPU reshape), and double-use nameplates (progress/status bars) by animating GPU style slots
(fill_mode/outline_px/glow_px/gradient/effect_flags/time_phase) — both fully GPU-resident.

**Documented non-blocking debts (carried, not gating):**
1. Flat 5k max no-op spike 1.0086 ms — single-frame O(N) label-scan spike (noop rebuild delta = 0). Make the
   per-frame scan incremental/event-driven (dirty list) before scaling well beyond 5k.
2. 5k damage churn ~2.5 ms/frame — inherent changed-VALUE reshape cost, not the settled no-op budget.
3. Interactive Studio **window** smoke (live windowed FPS) — presentation debt; headless cannot cover it.
4. Production icon **source set** (art + licensing) — owner-content debt; LR7 manifest machinery + fixtures are
   complete and proven.

**Non-goals reaffirmed:** no ScenarioSpec/RF/spatial change, no GPU-dispatch-into-sim, presentation-only.
The typeface ladder is **closed**. Next track is selected by the project owner.

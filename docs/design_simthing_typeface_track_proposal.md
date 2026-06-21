# SimThing Typeface / Glyph-Atlas Service — production track proposal

> **Status: APPROVED TO OPEN (owner-confirmed 2026-06-21).** Owner decisions: **renderer = MSDF target with
> a glyphon raster milestone**; **home = prototype in `simthing-workshop`, graduate to a new production crate
> `simthing-tools`**. The `TYPEFACE-LADDER` may open at LR0. Each rung remains PROBATION + perf-gated until DA
> review.

## Goal

One native SimThing service that turns TTF/OTF fonts **and** SVG vector icons into a single GPU-resident,
codepoint-addressed glyph system, usable for four jobs from one pipeline:

1. **UI text** (Studio / egui and Bevy UI) — crisp at any DPI/zoom.
2. **World-space entity labels** — planet / system / fleet / owner names that scale with camera distance.
3. **High-volume realtime labels** — damage numbers, floating combat text: thousands on screen, animating
   (scale/fade/rise), must stay sub-millisecond on the CPU side.
4. **Vector icons** — SVG-injected symbols (resource glyphs, status icons, unit markers) addressed as if they
   were font characters.

The unifying trick: **icons are glyphs.** SVG icons are assigned **Unicode Private-Use-Area codepoints**
(the "icon-font" pattern, like Font Awesome / Nerd Fonts), so a label string like `"Sol ⟨planet-icon⟩ 42"`
flows through the *same* shaping → atlas → draw path as ordinary text. No separate icon system.

## Ecosystem survey (verified against `Cargo.lock`, Bevy 0.16.1)

**Already in the tree (transitive via Bevy text):** `cosmic-text`, `swash`, `skrifa`, `rustybuzz`,
`ttf-parser`/`owned_ttf_parser`, `fontdb`, `tiny-skia`, `guillotiere`, `ab_glyph`, `zeno`. This is the modern,
mature Rust text stack — the same one Bevy renders with. **We should build on it, not introduce a parallel
font stack.**

| Concern | Recommended crate(s) | Why | In-tree? |
|---|---|---|---|
| TTF/OTF parse + read | `skrifa` (+ `ttf-parser`) | Google Fonts' fontations reader; modern, fast, OTF/CFF + TTF | ✅ |
| Shaping (ligatures, kerning, complex scripts) | `cosmic-text` (uses `rustybuzz`) | de-facto Rust shaping/layout; Bevy's backend | ✅ |
| Glyph rasterization / outline access | `swash` (raster) / `skrifa` outlines | Bevy's rasterizer; outlines feed MSDF/SVG | ✅ |
| Atlas packing | `guillotiere` (or `etagere`) | dynamic rectangle packing on a wgpu atlas | ✅ (`guillotiere`) |
| SVG parse + simplify | `usvg` | RazrFalcon/linebender stack; the mature Rust SVG parser | ➕ add |
| SVG raster (atlas path) | `resvg` + `tiny-skia` | rasterize SVG → atlas tile at any resolution | ➕ `resvg` (tiny-skia ✅) |
| SVG → outline → MSDF (vector path) | `usvg` outlines + MSDF gen | feed icon outlines into the same MSDF atlas as glyphs | ➕ add |
| **High-volume GPU text** | **MSDF atlas + 1 instanced shader** (target) **or** `glyphon` (raster milestone) | see decision below | ➕ |

**High-volume renderer decision (this is the real fork):**

- **`glyphon`** = `cosmic-text` → wgpu atlas + buffered draw. Mature, drop-in, proven for "lots of text."
  *Raster*: glyphs are baked at a pixel size, so large scale-ups re-raster and animated scaling looks soft.
  Good for UI and static-size labels. Lowest risk.
- **MSDF atlas (multi-channel signed distance field) + one instanced quad shader** = the AAA approach.
  Each glyph/icon is stored **once** as an MSDF tile; a single shader renders it **crisp at any scale**, which
  is exactly what damage text (scaling/animating) and zoomable world labels need. Icons (SVG outlines) and
  font glyphs share the *same* MSDF atlas and *same* shader. Highest performance for high volume (one atlas
  bind, one instanced draw call, no per-size re-raster). MSDF generation is a pure-Rust offline/once step over
  glyph and SVG outlines.

**DA recommendation:** target **MSDF**, but de-risk by shipping a **`glyphon` raster milestone first** so we
have working Bevy text early and a measured baseline, then graduate to MSDF for the scalable/high-volume goal.
Same atlas/codepoint API both ways — the renderer swaps underneath.

## Architecture

```
simthing-tools  (new crate; semantic-free presentation utility)
  TypefaceDb        : load TTF/OTF (skrifa/fontdb) + register SVG icons at PUA codepoints (usvg)
  ShapingEngine     : str -> shaped glyph runs (cosmic-text)
  GlyphAtlas        : codepoint+size(or MSDF) -> atlas tile; guillotiere packing; wgpu texture (GPU-resident)
  TextMesh/Instances: shaped run -> instanced quads (pos, uv, color, scale) for Bevy
  simthing_tools_bevy     : Bevy plugin — atlas as a GPU resource, an instanced text material/draw, label components
```

- **GPU-resident by shape** (constitution): the atlas is a wgpu texture; labels are instanced quad buffers.
  CPU does shaping/packing only (oracle/build path), not per-frame per-glyph work.
- **Workshop-first**: prototype + measure in `simthing-workshop` (its mandate); the production service lives in
  its own `simthing-tools` crate (workshop is never a production dependency).
- **Studio reuse**: Studio entity labels and (optionally) egui font registration consume the same `TypefaceDb`.

## Proposed ladder — `TYPEFACE-LADDER` (LR0–LR8)

| Rung | Deliverable | Proof |
|---|---|---|
| **LR0** | New `simthing-tools` crate scaffold; `TypefaceDb` loads a TTF/OTF via `skrifa`/`fontdb`; enumerates glyphs/metrics. Workshop measurement harness. | unit: load font, query glyph metrics; determinism |
| **LR1** | `ShapingEngine`: `&str` → shaped glyph runs (cosmic-text), with kerning/ligatures; deterministic layout report. | unit: known string → expected advances/positions |
| **LR2** | `GlyphAtlas` v1 (raster): rasterize glyphs (swash) into a `guillotiere`-packed wgpu atlas; CPU-readback proof of tile placement; eviction policy. | unit + headless wgpu: atlas tile bytes match oracle |
| **LR3** | `simthing_tools_bevy` plugin: instanced-quad text draw in Bevy from a shaped run; one Bevy UI + one Text2d label on screen via the atlas. | Bevy headless: entity counts; visual smoke (PNG) |
| **LR4** | **SVG icon ingestion**: `usvg` parse + `resvg` raster of SVG → atlas tile; register icons at PUA codepoints; render `"text ⟨icon⟩ text"` in one run. | unit: SVG → tile; mixed run shapes/draws |
| **LR5** | **High-volume bench**: spawn N animated labels (scale/fade/rise); measure CPU shaping/build + draw-call count; establish the damage-text budget (target: 5–10k labels, sub-ms CPU, ≤ few draw calls). | bench harness + recorded numbers (real adapter) |
| **LR6** | **MSDF atlas** (vector target): generate MSDF for glyph + SVG outlines (pure-Rust MSDF over `skrifa`/`usvg` outlines); single instanced SDF shader; crisp at arbitrary scale; swap renderer behind the LR3 API. | unit: MSDF determinism; visual scale sweep PNG; re-bench vs LR5 |
| **LR7** | **Custom character set / icon font**: declarative manifest mapping PUA codepoints → SVG assets; build-time atlas bake; stable codepoint contract. | unit: manifest → atlas; codepoint stability guard |
| **LR8** | **Studio + game integration seam**: world-space entity-name labels (camera-distance scaled) and a damage-text emitter component driven by the service; egui font registration optional. | Studio smoke: planet labels render; perf within LR5 budget |

LR0–LR3 deliver working scalable Bevy text; LR4–LR5 add icons + the high-volume proof; LR6 is the MSDF
graduation for true scalability; LR7–LR8 productionize the icon-font + game labels.

## Performance requirements (binding on LR5/LR6)
- **Damage-text class:** ≥ 5,000 simultaneous animated labels at 60 FPS with **sub-millisecond** CPU build
  time per frame and a bounded draw-call count (single atlas bind; instanced).
- **No per-glyph entities** for high-volume text — instanced buffers only (per-glyph `Text2d` entities do not
  scale to damage-text volume; that is the trap to avoid).
- **No per-frame raster/shaping** for static labels — shape once on text change, cache the instance buffer;
  redraw is GPU-only. (Mirrors the Studio "never serialize/evaluate in the draw path" rule — see
  `docs/simthing-bevy-performance.md`.)
- MSDF atlas re-bake is an offline/asset-load step, never per frame.

## Constitutional alignment & boundaries
- **Presentation utility, not a sim engine.** This is a Studio/render service. It does **not** touch
  ScenarioSpec authority, RF/Accumulator, the spatial hierarchy, or any DA-approved track.
- **Semantic-free GPU.** The shader/atlas are generic glyph/SDF rendering — no gameplay/map/faction semantics
  in shader text (same rule as `accumulator_op` / `stead_spatial_contract` guards).
- **GPU-resident in shape; CPU is build/oracle only** (atlas pack + shaping), consistent with the
  Accumulator-Flow doctrine's CPU posture.
- **Workshop-prototyped, measured before promotion.** Each rung is PROBATION with a real-adapter perf number;
  no DA promotion until the LR5 budget is met.

## Decisions (owner-confirmed 2026-06-21)
1. **Renderer target — RESOLVED:** MSDF atlas target, with a **glyphon raster milestone** shipped first
   (LR2–LR5 raster, LR6 MSDF graduation behind the same LR3 API).
2. **Crate home — RESOLVED:** prototype + measure in `simthing-workshop`; graduate the production service to a
   **new `simthing-tools` crate**. (Workshop stays a non-production dependency; LR0 lands the prototype, the
   `simthing-tools` crate is created when the prototype is proven — by ~LR3.)

## Still-open (owner input before the relevant rung; do not block LR0)
3. **License posture (before LR4 SVG + any bundled font):** `resvg`/`usvg`/`cosmic-text` are MPL-2.0/Apache/MIT
   — confirm acceptable and add to `THIRD_PARTY_LICENSES.md`. Choose a bundled default font (e.g. an OFL font).
4. **Icon authoring (before LR7):** how icons are authored/sourced (hand SVGs vs an existing set) and the PUA
   codepoint range to reserve.

## Next action
**Open `TYPEFACE-LADDER` at LR0** — `simthing-workshop` prototype scaffold: load a TTF/OTF via `skrifa`/`fontdb`
and a measurement harness; add the track row to `docs/tests/current_evidence_index.md` at PROBATION. The
`simthing-tools` crate is created at the rung where the prototype graduates (target LR3).

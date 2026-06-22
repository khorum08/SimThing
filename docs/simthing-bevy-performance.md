# SimThing Bevy / egui / wgpu performance guide

> Durable reference for diagnosing and avoiding Studio (`simthing-mapeditor`) framerate problems.
> Born from STUDIO-FRAMERATE-REGRESSION-OPUS-AUDIT-0 (the Scenario-ladder 4 FPS collapse).

## The golden rule
**The Studio UI frame must be cheap and display cached state.** Anything expensive (canonical
serialization, ScenarioSpec parse, RF/report/candidate compile, file IO, tree walks, digest computation)
runs **only on events** (load / save / reopen-adopt / explicit Refresh / generation), never per frame.
ScenarioSpec is authority; the UI is a presentation cache. Keep it that way and most of these problems never
happen.

## Telemetry traps (read before trusting a number)

1. **Per-panel "closure" timers ≠ egui cost.** A timer wrapping `egui::Window::show(|ui| …)` measures only
   the closure that *queues* widgets. egui's real cost — galley **layout**, **tessellation**, and **paint** —
   happens when the context finishes, and gets charged to whichever wrapper bounds the egui run. A panel
   reading "226 ms" while a sibling reads "0.14 ms" is almost always **misattribution of egui paint**, not
   that panel doing 226 ms of logic. *Fix: also measure the full egui run/paint (`egui_paint_ms`).*
2. **Debug builds amplify egui 10–40×.** egui layout/tessellation is unoptimized-sensitive. Never conclude
   "GPU/render bottleneck" from a `debug/unoptimized` build — always compare the same UI in `--release`.
3. **Fifo present ≠ vsync-bound if FPS < refresh.** At 4 FPS under Fifo, you are CPU-bound; the present wait
   is not the cause.
4. **"Unexplained frame time" ≈ 0 with one panel ≈ frame total** means that panel's wrapper is swallowing the
   real (egui-paint) cost. Trust the *sum of independent* measurements, not one fat bucket.

## Top causes of Studio FPS collapse (in observed likelihood order)

| Cause | Symptom | Fix |
|---|---|---|
| Expensive Scenario/proof call in the per-frame draw or `Update` | egui pass = whole frame; status recomputes every frame | Move behind dirty-flag + event triggers; UI shows cached status |
| **Digest computed by per-frame canonical serialization** (the subtle trap) | UI shows "cache hit" yet frame still huge | Store the authority digest at load; pass it in; never serialize to decide whether to refresh |
| Large always-visible diagnostic text (egui paint) in debug | many `ui.label` lines; debug build; per-panel timer "lies" | Collapse panels by default; gate verbose telemetry; build release |
| Per-frame tree walks / `.clone()` of big structs in draw | Update or egui pass grows with scenario size | Compute once on event, cache the rows/strings |
| Un-dirty-gated mesh/material rebuilds | render-loop systems show ms cost | Dirty-gate; rebuild only on change |

## Mandatory patterns

- **Dirty + cached digest, never per-frame serialize.** Compute `authority_digest` once when the scenario is
  loaded/mutated and store it on the session/state. The per-frame decision compares two `u64`s. If you find
  yourself calling `to_canonical_json` / `serde_json::to_string` / a proof compile to *decide whether to
  refresh*, you have reintroduced the whole cost.
- **Collapse by default.** egui `CollapsingHeader` that is collapsed does **not** lay out its children — use
  it for diagnostics. An always-expanded 60-line panel pays full layout every frame.
- **One true isolation switch.** Keep a `minimal-egui` mode (a single FPS label; `studio_ui_system`
  early-returns) so anyone can instantly separate egui-paint cost from app/render cost. Hiding panels while
  still running `studio_ui_system` does **not** isolate egui.
- **Measure the egui run, not just closures.** Add an `egui_paint_ms` around the context pass end so future
  regressions are attributed correctly.

## wgpu / VRAM notes
- "Tracked VRAM" estimates only assets the Studio tracks (textures/meshes); render targets, swapchain, and
  bloom intermediates are untracked — do not read a small tracked number as "GPU is fine," and do not chase
  GPU until a release build with minimal-egui still shows the cost.
- Forcing a specific GPU adapter (e.g. NVIDIA) affects *render* throughput; it cannot fix a CPU-bound main
  thread burning the frame in egui or serialization.

## Diagnostic checklist (fast path)
1. Build `--release`. Re-measure. (Rules out debug amplification.)
2. Toggle **minimal-egui**. If FPS recovers → cause is egui paint/volume (collapse panels).
3. `grep` the per-frame draw/`Update` for `to_canonical_json | serde_json | read_to_string | compile_ |
   evaluate_ | authority_digest`. Any hit reachable per frame is the bug.
4. Add `egui_paint_ms`. If "Left panel: N ms" ≈ `egui_paint_ms`, the panel timer was misattributing.
5. Only after 1–4 are clean, look at renderer/wgpu.

## Constitutional reminder
Performance work is **presentation-only**. Do not move Scenario authority, save/load, RF/Accumulator, the
spatial gridcell hierarchy, or proof chains to "make it faster." Cache *when* you compute, never *what* the
proof computes.

## Case study: the left-panel ScrollArea FPS collapse (STUDIO-EGUI-PAINT-ISOLATION-0)

**Symptom:** left panel collapsed → 116 FPS; left panel expanded → 3.9 FPS, "Left panel: 240 ms".

**Cause:** the panel rendered its entire content tree (presets, generation fields, scenario/save-load,
camera) inside a single `egui::ScrollArea::vertical().show(...)`. An egui `ScrollArea` (and any
always-expanded panel) **lays out every child widget every frame** to compute content size — it only clips
*paint*, not *layout*. In a debug build that full layout pass dominates the frame.

**Fix (presentation-only):** wrap the heavy sections in `egui::CollapsingHeader::new(..).default_open(false)`.
A collapsed header does **not** lay out its children, so the per-frame layout cost drops to near-zero until a
section is opened. Keep only the always-needed controls (e.g. Generate, Camera) outside collapsing headers.

**Rules this established:**
- Never put a large, always-expanded widget tree in a per-frame panel/`ScrollArea`. Default heavy sections to
  collapsed; the user expands what they need.
- For genuinely long scrollable lists, use `ScrollArea::show_rows`/viewport culling so off-screen rows are not
  laid out — plain `.show()` lays out all children.
- The **collapse test** (toggle a panel and watch FPS) is the fastest way to attribute cost to a panel; it is
  a more reliable isolation than per-panel closure timers.

## Hard rule: never serialize/evaluate model state in the draw path

The save-load section regression (124 FPS collapsing to 1.9 FPS when the section was *displayed*) came from an
`else { refresh_status_if_needed(false) }` call left in the per-frame draw. Even a "cache-gated" refresh is a
trap: if the dirty flag is set (e.g. after Generate) and the gate decides to `Refresh`, you pay the **full**
canonical-serialize + STEAD/RF/report/candidate evaluation **every frame** the widget is visible.

**Rule:** expensive model evaluation (canonical serialization, proof/report chains, file IO) must be triggered
**only** by an explicit user action — a Refresh button, or the Load/Save/Reopen dialog handlers — never from a
widget's draw closure, and never on a timer for display. When state is dirty, the draw path shows a cached
value or a "click Refresh" hint; it does not compute. Drawing a panel must cost only egui layout, never a
serialize.

## Hard rule: a dirty-gate's INNER per-element key must include everything the outer gate keys on

The opposite failure of the save-load trap: a dirty-gate so aggressive it drops *legitimate* updates. Symptom:
a Settings slider (star radius / opacity / falloff) changed nothing on screen **until the mouse moved**
(`STUDIO-STAR-SETTINGS-REALTIME-0`, `app/picking.rs::sync_star_visuals_system`).

Two-level gating was the cause. The **outer** gate (`StarVisualSyncKey`) *included* the falloff settings, so the
system correctly *ran* on a settings change. But the **inner** per-entity gate (`StarVisualAppliedKey`) tracked
only `selected/hover/render_mode/depth_bucket/layer` — **not** the settings — so every star hit
`if *applied_key == visual_key { continue; }` and was skipped. The new settings only "took" when a camera move
changed each star's `depth_bucket`, breaking the per-star key. Moving the mouse = camera move = the only thing
that invalidated the inner key.

**Rules for multi-level dirty gates:**
1. **Every input that can change the output must appear in the key that actually short-circuits the write.** If
   the outer gate keys on setting `X` but the inner per-element key does not, an `X`-only change runs the
   system to no effect. Either add `X` (or a settings revision/hash) to the per-element key, or…
2. **…honor a one-frame `force` flag.** Capture the dirty flag before the loop (`let force = cache.dirty;`) and
   write `if !force && *applied_key == key { continue; }`. On the change frame every element re-applies; the
   flag resets at the end of the system, so steady-state frames keep the cheap per-element gate. This is the
   minimal, settings-complete fix (covers fields the per-element key omits) and preserves the perf gain.
3. **A "mark dirty" must reach the gate that does the work.** `apply_*_settings` here marked the star-visual
   cache dirty, but the gate that mattered for the *visible* change (the per-star material write) ignored it.
   Trace the dirty flag all the way to the write, not just to the system's entry guard.
4. **Test the realtime path, not just the skip path.** The dirty-gate tests proved "skips when unchanged" and
   "rebuilds when settings change" at the *outer* gate — and still passed while the *inner* gate silently
   dropped the update. Add a test that a settings change actually mutates the rendered material/transform with
   the camera held still.

Default redraw mode matters too: Studio runs Bevy's default `WinitSettings` (Continuous), so a correctly-gated
update appears within one frame. If you ever switch to `Reactive`/`desktop_app()` power-saving, the same class
of "updates only on input" bug appears for a *different* reason (no frame is produced until an event) — request
a redraw on the state change, or stay Continuous for an editor.

## Performance Telemetry window (STUDIO-PERFORMANCE-TELEMETRY-WINDOW-0)

Live FPS, frame-phase timings, VRAM estimates, diagnostic isolation toggles, and capture steps belong in the
on-demand **Performance Telemetry** window (top-right **Telemetry** button), not in the Settings dialog.
Settings retains only star/hyperlane preferences. Hiding telemetry does not stop collection — it is presentation
state only.

**Screenshot capture** is an explicit user action (Screenshot button in the Performance Telemetry window). It
writes `screenshot_{index:05}.png` to the current working directory via Bevy's `Screenshot::primary_window()` +
`save_to_disk`. Do not add per-frame or background screenshot timers.

## Render notes: headless pixel capture (LR3R / typeface smoke tests)

**Test-only.** These paths exist to prove shader-backed instanced text draws in CI; they do not affect shipped
runtime performance. The in-game render path is identical regardless of which smoke route the test uses.

### Route B (preferred): raw wgpu harness

For deterministic, fast pixel proof in headless CI:

1. Shape glyphs and build atlas pixels on the CPU (same data the Bevy plugin produces).
2. Spin a minimal wgpu device → pipeline → instanced draw → `copy_texture_to_buffer` readback.
3. Assert non-zero text pixels (`alpha > 0` and at least one RGB channel `> 0`) before writing the PNG artifact.

Route B is leaner than spinning a full Bevy `App` and does not depend on Bevy's image-output node.

### Route A (deferred fidelity nicety): in-Bevy image readback

When you need pixels through Bevy's real Core2d graph (not for perf — only for fidelity of evidence):

- Use a standard **`Camera2d`** with **`Tonemapping::None`**, **`RenderTarget::Image`**, and
  **`gpu_readback::Readback`** on that image handle.
- Let Bevy's built-in Core2d graph run end-to-end, including the node that resolves `ViewTarget.main_texture`
  back into the `Image`'s `GpuImage` texture.

**Never hand-roll the Core2d render graph** for offscreen capture. A custom offscreen camera that bypasses
`Camera2d` can queue draws correctly (Transparent2d phase, bind groups, instances) but **omit the
ViewTarget→Image write node**. The render pass executes; the `Image` stays `(0,0,0,0)`; readback copies the
cleared target. This is a self-inflicted fork bug, not a Bevy headless limitation.

### CI hygiene

- Do **not** pay multi-second `Readback` polling loops in CI when Route B is the authorized proof.
- Keep Bevy integration evidence to short queue-wiring probes (draw count, instance count, view count).
- Mark in-Bevy PNG capture as **DEFERRED** until a `Camera2d` + `Tonemapping::None` readback path is wired.

## Case study: the typeface mount black-screen (STUDIO-TYPEFACE-STARTUP-FIX-0R)

**Symptom:** after Cursor integrated the typeface (`simthing-tools`) render plugin into the live Studio
(LR8), the window started without panicking but rendered **pure black** — no egui at all (no left panel,
Generate button, gear, or close X). Disabling the typeface mount restored the full UI.

**Cause — a render plugin that mutates a shared GPU asset at app scope suppresses egui.**
`SimthingToolsTextPlugin::build` ran `fix_volume_image_view_descriptors` at `Startup`/`PostStartup`: it
walked **every** `Assets<Image>` entry and rewrote the tonemapping-LUT image to a **D3** texture view, then
fired `AssetEvent::Modified`. That fix exists only for the **offscreen** headless render path (text drawn
through a tonemapping camera, which needs the LUT bound as D3). On a **live window** there is no offscreen
camera — the mutation is pure collateral: it disturbs the prepared GPU views egui depends on, and `bevy_egui`
silently skips its pass (no panic, just black). A separate `TonemappingLutFixPlugin` did the same thing in
the render world (`RenderAssets<GpuImage>` / `FallbackImage.d3`) with the same effect.

**Why the first fix failed.** Putting `Tonemapping::None` on the Studio camera removed the LUT *bind-group
crash* but **not** the black screen — because the black screen was never the LUT mismatch; it was the
app-scope image mutation. Lesson: a plausible mechanism that "could" explain the symptom is not a diagnosis.
The deterministic signal was the bisect ("UI returns with the plugin OFF") — chase that, not a theory.

**Fix.** Gate the offscreen-only LUT fix behind `SimthingToolsTextPlugin::without_lut_d3_view_fix()`; the
Studio shell mounts with it OFF, the headless tests keep it ON (default). The camera returns to the known-good
default. All `simthing-tools` capability stays in the crate; only the not-yet-visible in-Studio mount is held
until the main/overlay-camera text path is built and verified on a real window.

**Rules to avoid a repeat:**
1. **A render/asset plugin is not "integrated" until it has run on a real window with egui mounted.** Headless
   queue-wiring probes (draw/instance/view counts) prove the GPU path, **not** egui coexistence. Smoke-test the
   actual window before calling an in-Studio mount done.
2. **Never mutate a shared `Image` / `GpuImage` / `FallbackImage` view descriptor at app scope on a live egui
   window.** If a render workaround is needed only for an offscreen/test path, **gate it to that path** (a
   plugin flag or an offscreen-camera filter) so it never touches the live presentation surface.
3. **A black screen with no panic is almost always egui's pass being skipped**, not the 3D scene failing.
   Suspect anything that touches camera views, `RenderAssets<GpuImage>`, `FallbackImage`, viewport rects, or
   the egui camera-view extraction — before suspecting the scene or the camera transform.
4. **Trust the bisect over the theory.** When "feature OFF ⇒ symptom gone" is reproducible, the cause is inside
   that feature's wiring; localize *which* system, don't merge a fix for an unproven mechanism.

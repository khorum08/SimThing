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

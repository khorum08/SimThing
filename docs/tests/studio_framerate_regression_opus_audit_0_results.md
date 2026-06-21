# STUDIO-FRAMERATE-REGRESSION-OPUS-AUDIT-0 Results

## Status
PROBATION audit. Root cause classified **B/C+E** (egui base/paint hotpath from the ladder-enlarged UI, amplified by a debug build) with a **D** component (per-panel timers misattribute egui paint as "Left panel: 226 ms"). Root cause **A (CPU-side Scenario eval in the frame path) is RULED OUT** — every expensive call is event-gated.

## PR / branch / merge
Branch `studio-framerate-regression-opus-audit-0`; docs-only.

## Mission
Find the precise code path explaining the post-Scenario-ladder collapse from 30–60 FPS to ~4 FPS, preserving all DA-approved tracks.

## Quick reorientation
Scenario Runtime + Save/Load track is DA-approved/closed; not reopened.

## Performance history
Pre-ladder ~30–60 FPS → post-ladder ~4 FPS (~230 ms/frame). Prior remediations (STATUS-CACHE-0, telemetry, dirty-gate, frame-phase) improved observability but did not restore FPS.

## Current telemetry summary
Frame ~230–250 ms; **egui/UI pass ≈ Left panel ≈ 226–233 ms**; Main Update ~0.05 ms; Settings window 0.14 ms; all render-loop systems (hyperlane rebuild, star/billboard sync, picking, VRAM) sub-1 ms; build debug; present Fifo; status shows "refresh pending" (cache not refreshing).

## Code paths reviewed
`app/ui.rs`, `app/mod.rs`, `scenario_runtime_saveload_ui.rs`, `scenario_io.rs`, `app/scenario_io.rs`, `app/galaxy_render.rs`, `hydration.rs`; spec/driver scenario IO + report-chain + candidate compile.

## Scenario ladder regression audit
The ladder added a large left-panel "Runtime / Candidate Save-Reopen" section and the telemetry passes added a ~60-line Settings "Performance" panel. This is a large net increase in egui widget/text volume. No new per-frame *Scenario evaluation* survives (below).

## Egui pass timing audit
The "Egui/UI pass" wrapper (~226 ms) and the "Left panel" wrapper (~226 ms) are nearly equal, while the Settings-window wrapper is 0.14 ms. The per-panel wrappers time only the **closure that queues widgets** (cheap); egui's galley **layout + tessellation + paint** run at context end and are attributed to whichever wrapper bounds the egui run. **"Left panel: 226 ms" therefore measures egui paint, not left-panel logic.** This is a measurement artifact (classification D) masking the real egui-paint cost (classification C).

## EguiPrimaryContextPass placement audit
`studio_ui_system` runs in `EguiPrimaryContextPass`. The wrapper time includes egui context run/tessellate; under Fifo at 4 FPS this is CPU layout/tessellation cost, not vsync wait (vsync would cap at ~60). Placement is not itself the bug; the bug is the volume of text laid out/tessellated every frame in a debug build.

## CPU-side Scenario evaluation search
Required grep output classified (mapeditor src):
- `app/mod.rs:277` `refresh_runtime_saveload_status_from_session` — **gated** (inside the `Refresh` decision branch only; dirty/force/digest-change).
- `app/ui.rs:1040` `canonical_json_from_loaded_scenario_authority` — **Save Candidate event handler** only.
- `app/scenario_io.rs:*`, `scenario_io.rs:*`, `app/ui.rs` reads — **load/save event handlers** only.
- `hydration.rs:*` serialize/deserialize — **tests/generation events**, not the frame.
- `scenario_runtime_saveload_ui.rs` compiles — only inside `build_studio_...` / `refresh_*` (reached only via the gated `Refresh` branch and explicit Refresh button).
**Decisive:** `StudioAppState::refresh_runtime_saveload_status_if_needed(false)` is called per-frame from the left-panel draw (`app/ui.rs` ~251/276 + `draw_runtime_candidate_saveload_controls` else-branch), but it passes **`authority_digest = None`** (`app/mod.rs:262`); with `dirty=false` the pure `runtime_saveload_refresh_decision` returns **UseCache** — no serialize, no digest, no proof. **No expensive Scenario work runs per frame.** Root cause A ruled out.

## UI section timing audit
Left-panel closure logic (widget queueing, cached-status `format!`s of small strings/u64s) is cheap; the 226 ms is egui paint attributed to it.

## Telemetry overhead audit
Settings "Performance" panel emits ~60 `ui.label` lines every frame (always visible, not collapsed). Each is a galley layout + tessellation. In debug this is the dominant new cost. No large `format!` over Debug structs or vector loops found, but the sheer count of always-visible text lines is the issue.

## Render-loop audit
Hyperlane/star/billboard/picking/VRAM all sub-1 ms and dirty-gated; not the cause.

## Build-profile audit
Build is debug/unoptimized. egui layout+tessellation is typically 10–40× slower unoptimized. Debug cannot be the *sole* explanation without a same-profile baseline, but it is a large amplifier of the enlarged-UI cost. A release sanity run is required (see remediation).

## True egui-off / minimal-egui isolation status
**Not implemented.** Current "Performance isolation" toggles hide stars/hyperlanes/panels but still run `studio_ui_system` and egui paint. A true egui-off / minimal-egui (single FPS label) mode does not exist; it is the decisive missing isolation.

## Root cause classification
**B/C with E amplification; D attribution artifact.** Not A. The egui run (text layout/tessellation/paint of the ladder-enlarged, always-expanded UI) consumes the frame in a debug build; the "Left panel" timer misattributes that paint cost.

## Recommended remediation track
**STUDIO-EGUI-PAINT-ISOLATION-0** (PROBATION):
1. Add a **true minimal-egui mode** (one tiny FPS label; no panels; `studio_ui_system` early-returns) to isolate egui paint from app logic — the decisive measurement.
2. Move/Add a timing wrapper around the actual **egui run/tessellate/paint** (e.g. measure `EguiContextPass` end-to-end and report `egui_paint_ms`) so "Left panel: 226 ms" stops misattributing.
3. Collapse the Settings "Performance" panel and the left-panel diagnostic sections **by default** (egui `CollapsingHeader` collapsed → children not laid out) and gate verbose telemetry behind a toggle.
4. Run **release** at identical UI and record FPS. If minimal-egui and/or release restores 30–60 FPS, the fix is compact-UI + release, touching **zero** DA logic.
Expected outcome: confirms C+E; remediation is presentation-only.

## Authority / non-authority boundary
Untouched. ScenarioSpec authority, save/load, RF/Accumulator, surface-gridcell hierarchy, owner doctrine all preserved. UI/Bevy/reports/GPU buffers remain non-authoritative.

## DA-approved baseline preservation
No DA-approved row demoted; no proof ladder reopened or weakened.

## Validation
`cargo check -p simthing-mapeditor`; `git diff --check`. Docs-only change.

## Evidence lifecycle and cleanup
New: this report + `docs/simthing-bevy-performance.md`. No reports deleted.

## Boundary / non-goals
No renderer rewrite, GPU dispatch, schema/save/RF changes, DA promotion.

## Files changed
`docs/tests/studio_framerate_regression_opus_audit_0_results.md`, `docs/simthing-bevy-performance.md`, `docs/tests/current_evidence_index.md`, `docs/design_0_0_8_3_studio_production.md`.

## Known gaps
Root cause B/C+E is evidence-strong but final proof needs the minimal-egui + release isolation run (STUDIO-EGUI-PAINT-ISOLATION-0), which this audit does not implement.

## Next recommended action
Execute STUDIO-EGUI-PAINT-ISOLATION-0.

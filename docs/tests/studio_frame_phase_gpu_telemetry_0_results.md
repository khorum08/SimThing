# STUDIO-FRAME-PHASE-GPU-TELEMETRY-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `studio-frame-phase-gpu-telemetry-0`
- PR: #859
- Merge SHA: `072f45fb031276913ee69082569e717f93747792`

## Mission

Expand Studio Settings Performance telemetry to distinguish build profile, GPU adapter/backend/present context, egui/UI pass timing, frame-phase timing, and VRAM estimate limitations. Add diagnostic render isolation controls without changing ScenarioSpec authority or reopening closed save/load tracks.

## Current observed performance

After STUDIO-RENDER-LOOP-DIRTY-GATE-0, Settings still reports approximately 4.2 FPS with instrumented render-loop systems collectively under 1 ms per frame.

## Existing telemetry interpretation

The currently instrumented render-loop systems do not account for the single-digit FPS. At approximately 4.2 FPS, total frame time is roughly 238 ms, while hyperlane rebuild, star visual sync, billboard sync, picking projection, and VRAM scan collectively report under 1 ms. The remaining bottleneck must be isolated in build profile, egui/UI pass, Bevy render sub-app, GPU present/swapchain, post-processing, fill-rate, or adapter/present-mode behavior.

## Build-profile telemetry

Settings Performance now shows `Build: debug/unoptimized` or `Build: release/optimized` via `cfg!(debug_assertions)` and displays a debug-build warning when applicable.

## GPU adapter/backend/present telemetry

Settings shows GPU adapter, vendor/device IDs, backend, device type, present mode, window resolution, and render scale (scale factor). Unavailable fields render as `unavailable`.

## Frame-phase telemetry

`StudioPerformanceTelemetry` records frame total, main Update pass, egui/UI pass, per-panel egui timings, instrumented render-loop sum, and estimated unexplained frame time. Render sub-app extract/prepare/queue/render/present phases are marked unavailable in this build.

## Egui/UI timing telemetry

`studio_ui_system` records total egui pass time plus left panel, galaxy status panel, and Settings window sub-timings with rolling averages.

## VRAM estimate limitation update

VRAM display now reads `Allocated VRAM estimate: <N> MB (tracked assets only)` with texture/mesh/buffer breakdown and explicit untracked lines for render targets/swapchain and bloom/postprocess intermediates.

## Diagnostic render controls

Settings Performance isolation section adds hide stars/hyperlanes, disable aura layer, force crisp stars, hide panels except Settings, freeze camera, plus `Apply Diagnostic Minimal Render` and `Restore Normal Render` buttons and manual capture steps.

## Authority / non-authority boundary

Telemetry and diagnostic controls are presentation-only. ScenarioSpec authority, hydration, and runtime proof chains are untouched.

## DA-approved baseline preservation

No DA-approved evidence rows were demoted. Scenario Runtime + Save/Load and parent capability-tree ladders were not reopened. Dirty gates from STUDIO-RENDER-LOOP-DIRTY-GATE-0 are preserved.

## Validation

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test studio_settings_performance_telemetry
cargo test -p simthing-mapeditor --test studio_render_loop_dirty_gate
cargo test -p simthing-mapeditor --test studio_frame_phase_gpu_telemetry
cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui
cargo test -p simthing-mapeditor --test studio_candidate_reopen_adopt
git diff --check
```

## Evidence lifecycle and cleanup

`STUDIO-FRAME-PHASE-GPU-TELEMETRY-0` is recorded as **PROBATION**. No DA promotion in this PR.

## Boundary / non-goals

No framerate fix, renderer rewrite, GPU dispatch, ScenarioSpec schema changes, or DA lifecycle promotion.

## Files changed

- `crates/simthing-mapeditor/src/studio_frame_phase_gpu_telemetry.rs`
- `crates/simthing-mapeditor/src/studio_performance_telemetry.rs`
- `crates/simthing-mapeditor/src/app/performance_telemetry.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/camera.rs`
- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_frame_phase_gpu_telemetry.rs`
- `crates/simthing-mapeditor/tests/studio_settings_performance_telemetry.rs`
- `docs/tests/studio_frame_phase_gpu_telemetry_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`

## Known gaps

- Render sub-app phase timing is unavailable through stable Bevy APIs in this slice.
- Unexplained frame time is an estimate (total − main Update − egui − instrumented render-loop); it does not yet attribute GPU present vs draw vs postprocess.
- Diagnostic minimal render does not yet hide all star layers beyond aura disable + crisp mode.
- Live FPS attribution requires manual capture workflow in the running client.

## Next recommended action

Run Settings capture steps on debug and release builds; compare frame total vs egui/UI pass vs unexplained time. If egui/UI dominates, profile panel draw paths. If unexplained dominates, profile Bevy render/present with release build and diagnostic minimal render preset.
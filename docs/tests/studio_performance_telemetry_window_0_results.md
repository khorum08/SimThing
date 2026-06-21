# STUDIO-PERFORMANCE-TELEMETRY-WINDOW-0 Results

## Status

PASS — presentation/UI feature + regression test. Performance telemetry split into dedicated window; #868 follow-up test added; camera zoom-in doubled; screenshot button implemented.

## PR / branch / merge

- Branch: `studio-performance-telemetry-window-0`
- PR: #869
- Merge SHA: `e857b0c621c8b8b5cc7c981d5b601a2978abcf16`

## Mission

Close the #868 follow-up regression test, move performance telemetry/diagnostics out of Settings into a dedicated Performance Telemetry window, add screenshot capture, and allow 200% closer camera zoom — without weakening #862/#864/#868 performance fixes or Scenario authority boundaries.

## Root cause / carried baseline

- **#862** — collapsed heavy left-panel sections by default.
- **#864** — Scenario status refresh removed from draw paths; event/Refresh only.
- **#868** — `force_resync` one-frame star visual reapply when settings dirty; per-star skip on steady state.

## Implementation

- Added `TelemetryDialogModel` and `telemetry_dialog` on `StudioAppState`.
- Top-right **Telemetry** button immediately left of Settings gear toggles movable **Performance Telemetry** window.
- Moved `performance_settings_section_lines`, frame-phase/GPU/VRAM/render-loop telemetry, capture steps, and diagnostic isolation controls from Settings into the telemetry window.
- Settings retains star/hyperlane sliders, Reset, and Close only.
- Telemetry window stays drawable when `performance_diagnostic_hide_panels` hides main panels so **Restore Normal Render** remains reachable.
- `star_visual_per_star_should_write` pure helper extracted for #868 skip decision.

## Realtime settings regression test

`tests/studio_star_settings_realtime.rs::settings_star_render_change_mutates_visual_with_camera_fixed`:

- Proves `force_resync=true` bypasses matching `StarVisualAppliedKey` (exact #868 failure mode).
- Proves `compute_star_distance_visual` output changes at fixed camera depth when blur/opacity settings change.

## Performance Telemetry window

- Title: **Performance Telemetry**; draggable title bar (Settings pattern); viewport/panel clamped.
- Top-right **X** and bottom-right **Close** hide window without stopping telemetry collection.
- Diagnostic label revised to **Hide main egui panels**.

## Screenshot behavior

- **Screenshot** button spawns `Screenshot::primary_window()` with Bevy `save_to_disk`.
- Filename: `screenshot_{index:05}.png` via `studio_screenshot::next_screenshot_filename` scanning CWD.
- `screenshot_*.png` added to `.gitignore`.

## Camera zoom change

- `CAMERA_MIN_ORBIT_DISTANCE = 12.5` (half prior 25.0 minimum).
- `camera_control::apply_scroll_zoom` used in `camera_control_system`.
- Test: `camera_scroll_zoom_allows_half_previous_minimum`.

## Scenario draw-path safety

No new per-frame `refresh_runtime_saveload_status_if_needed(false)`, `to_canonical_json`, or Scenario proof/serialize calls in telemetry/settings/screenshot draw paths. Screenshot IO only on explicit button click.

## Authority / non-authority boundaries

Performance Telemetry state, screenshot output, Settings state, Bevy entities, render materials, frame timings, and diagnostic flags are presentation/cache state only. ScenarioSpec remains serialized scenario authority.

## Validation

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --lib telemetry_button
cargo test -p simthing-mapeditor --lib telemetry_dialog_close
cargo test -p simthing-mapeditor --lib camera_scroll_zoom
cargo test -p simthing-mapeditor --lib next_screenshot_filename
cargo test -p simthing-mapeditor --test studio_star_settings_realtime
git diff --check
```

Draw-path guards: no new forbidden calls in `app/ui.rs` draw paths.

## Files changed

- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/camera.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `crates/simthing-mapeditor/src/camera_control.rs`
- `crates/simthing-mapeditor/src/dialog.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/src/studio_render_loop_dirty_gate.rs`
- `crates/simthing-mapeditor/src/studio_frame_phase_gpu_telemetry.rs`
- `crates/simthing-mapeditor/src/studio_screenshot.rs`
- `crates/simthing-mapeditor/tests/studio_star_settings_realtime.rs`
- `.gitignore`
- `docs/simthing-bevy-performance.md`
- `docs/workshop/studio_production_log.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/studio_performance_telemetry_window_0_results.md`

## Known gaps

- Screenshot button not validated headlessly (Bevy async capture); filename allocation unit-tested.
- Telemetry dialog position/visibility not persisted to studio config (optional per task).
- Integration tests for full `studio_frame_phase_gpu_telemetry` may hit linker PDB flakes on this Windows host.

## Next recommended action

Owner manual validation: Telemetry button placement, telemetry window toggle/close, screenshot file creation, Settings sliders still live-update stars without camera motion, closer zoom, Scenario status still refresh-on-event only.
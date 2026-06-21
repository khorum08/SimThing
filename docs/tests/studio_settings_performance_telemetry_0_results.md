# STUDIO-SETTINGS-PERFORMANCE-TELEMETRY-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `studio-settings-performance-telemetry-0`
- PR: pending
- Merge SHA: pending

## Mission

Add live FPS and allocated VRAM estimate counters to the bottom of the Studio Settings window so Studio performance can be measured in the running client without attempting framerate remediation in this PR.

## Performance context

Studio framerate remains very low after STUDIO-RUNTIME-SAVELOAD-STATUS-CACHE-0 removed per-frame runtime save/load proof recomputation. This PR adds presentation-only telemetry to distinguish CPU/UI overhead from render/GPU allocation pressure during subsequent remediation.

## FPS telemetry implementation

`FrameCountPlugin` and `FrameTimeDiagnosticsPlugin` are registered on the Studio app. `update_studio_fps_telemetry` reads smoothed FPS from `DiagnosticsStore` each frame. Settings UI shows `FPS: <value>` with one decimal place, or `FPS: warming up` before diagnostics are available.

## VRAM estimate implementation

`estimate_studio_allocated_vram_bytes` scans Bevy `Assets<Image>` and `Assets<Mesh>` at a 0.5s cadence (or immediately when scene rebuild marks `vram_dirty`). Texture bytes use CPU `Image::data` when present, otherwise mip-aware extent × format block-copy footprint. Mesh bytes sum vertex attribute byte slices and index buffers. `buffer_bytes_estimate` remains zero in this slice. UI label reads `Allocated VRAM estimate: <N> MB`.

## Settings-window placement

A **Performance** subsection is rendered at the bottom of the existing Settings dialog after hyperlane controls and before Reset/Close.

## Adapter/GPU identity display, if implemented

`StudioGpuIdentityInitPlugin::finish` copies render-subapp `RenderAdapterInfo` into telemetry once at startup. Settings shows `GPU: <adapter name> (<backend>)` when available.

## Authority / non-authority boundary

Telemetry is presentation diagnostics only. ScenarioSpec authority, runtime RF semantics, save/load behavior, and cached runtime save/load status are unchanged.

## Performance overhead guard

FPS reads one diagnostic value per frame. VRAM asset scans run at most every 0.5s unless a scene rebuild marks the estimate dirty. No per-egui-draw full asset rescan.

## Validation

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test studio_settings_performance_telemetry
cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui
cargo test -p simthing-mapeditor --test studio_candidate_reopen_adopt
git diff --check
```

## Evidence lifecycle and cleanup

`STUDIO-SETTINGS-PERFORMANCE-TELEMETRY-0` is recorded as **PROBATION**. No DA-approved rows were demoted and no closed Scenario Runtime + Save/Load tracks were reopened.

## Boundary / non-goals

No framerate remediation, GPU dispatch, persistent history, ScenarioSpec schema changes, or DA lifecycle promotion.

## Files changed

- `crates/simthing-mapeditor/src/studio_performance_telemetry.rs`
- `crates/simthing-mapeditor/src/app/performance_telemetry.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_settings_performance_telemetry.rs`
- `docs/tests/studio_settings_performance_telemetry_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`

## Known gaps

- VRAM is an in-app allocation estimate, not driver-wide total VRAM usage.
- Materials, render targets, pipeline caches, and explicit GPU buffers are not included (`buffer_bytes_estimate` is zero).
- GPU identity depends on render-subapp init timing; if unavailable, the GPU line is omitted.

## Next recommended action

Run Studio with a loaded scenario, open Settings, and record FPS + VRAM estimate while reproducing low-FPS cases. Use the measurements to target the next remediation track (scene rebuild frequency, asset retention, or render path profiling).
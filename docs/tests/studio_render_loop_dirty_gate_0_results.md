# STUDIO-RENDER-LOOP-DIRTY-GATE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `studio-render-loop-dirty-gate-0`
- PR: #858
- Merge SHA: `431f922a63e4d52c5a52991eb8d601825305678b`

## Mission

Instrument and dirty-gate Studio render-loop hyperlane mesh rebuilds, star visual material/scale updates, billboard orientation, and picking projection work. Expose per-system render-loop diagnostics in the Settings Performance section without changing ScenarioSpec authority, save/load behavior, RF/Accumulator semantics, or DA-approved evidence lifecycle.

## Performance root cause

The remaining low-FPS condition was driven by Studio render-loop work, not VRAM exhaustion: hyperlane meshes were rebuilt and replaced every frame, while star visual transforms/materials and billboard orientation were updated across all star visual entities every frame. This PR adds render-loop telemetry and dirty-gates the rebuild/update paths that do not need to run on unchanged frames.

## Hyperlane per-frame rebuild analysis

`sync_hyperlane_colors_system` previously called `build_hyperlane_render_segments` and replaced every hyperlane bucket mesh every frame. The system now quantizes camera position/orientation/view mode and hyperlane render settings into cache keys and skips mesh rebuilds when session revision, settings, and camera keys are unchanged.

## Star visual per-frame update analysis

`sync_star_visuals_system` previously rewrote transform, scale, textures, base color, and emissive for every star visual every frame. The system now dirty-gates on camera/selection/settings/session keys and uses per-entity `StarVisualAppliedKey` to skip redundant material writes when computed visuals are unchanged.

## Billboard sync analysis

`billboard_stars_system` previously called `look_at` on every `GalaxyStar` every frame. The system is instrumented and gated on quantized camera position changes; billboard cache resets on scene rebuild so new entities receive an initial orientation pass.

## Picking projection analysis

`star_pick_system` previously allocated a `Vec` and projected every render anchor every frame while the cursor was over the scene. The system is instrumented and caches screen projections, rebuilding only when camera, window size, anchor count, or session revision changes.

## New render-loop telemetry

`StudioPerformanceTelemetry` now records hyperlane rebuild count/timing and geometry counts, star visual sync count/timing, billboard sync count/timing, picking projection count/timing, render frame index, and VRAM scan last-ms. Values are presentation-only.

## Dirty-gate implementation

`StudioRenderLoopCaches` holds hyperlane, star visual, billboard, and picking cache state. `scene_render_revision` on `StudioAppState` bumps on session adopt. Scene rebuild and render-settings changes mark caches dirty through `mark_hyperlane_render_dirty` and `mark_star_visual_render_dirty`.

## Settings-window diagnostics

The Settings Performance section now includes a **Render loop diagnostics** subsection with hyperlane rebuild timing/counts, geometry counts, star visual sync, billboard sync, picking projection, and VRAM scan timing. Existing FPS and allocated VRAM estimate lines are preserved.

## ScenarioSpec authority preservation

No ScenarioSpec schema, admission, hydration, or authority paths were modified. Render-loop changes are Bevy presentation/cache behavior only.

## DA-approved baseline preservation

No DA-approved evidence rows were demoted. Scenario Runtime + Save/Load and parent capability-tree proof ladders were not reopened.

## Validation

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test studio_settings_performance_telemetry
cargo test -p simthing-mapeditor --test studio_render_loop_dirty_gate
cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui
cargo test -p simthing-mapeditor --test studio_candidate_reopen_adopt
git diff --check
```

## Evidence lifecycle and cleanup

`STUDIO-RENDER-LOOP-DIRTY-GATE-0` is recorded as **PROBATION**. No DA promotion in this PR.

## Boundary / non-goals

No GPU dispatch, persistent history, ScenarioSpec schema changes, save/load behavior changes, RF/Accumulator semantic changes, or DA lifecycle promotion.

## Files changed

- `crates/simthing-mapeditor/src/studio_render_loop_dirty_gate.rs`
- `crates/simthing-mapeditor/src/studio_performance_telemetry.rs`
- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/picking.rs`
- `crates/simthing-mapeditor/src/app/performance_telemetry.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/src/star_render.rs`
- `crates/simthing-mapeditor/tests/studio_render_loop_dirty_gate.rs`
- `crates/simthing-mapeditor/tests/studio_settings_performance_telemetry.rs`
- `docs/tests/studio_render_loop_dirty_gate_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`

## Known gaps

- Hyperlane/star camera quantization thresholds are a first slice; finer movement may still trigger rebuilds after crossing 0.5-world-unit steps.
- Billboard gating uses camera position only; large orientation-only moves without position change skip `look_at` (acceptable for current billboard model).
- Picking cache does not yet include view-mode as an independent key beyond session revision side effects.
- Live FPS improvement magnitude is not benchmarked in CI; diagnostics are the proof surface for this PR.

## Next recommended action

Run Studio with ~1000-star connected galaxy, confirm hyperlane rebuild count stays near zero while idling and rises only on camera/settings/session changes. If picking projection still dominates, tighten cache keys or reduce anchor projection frequency further.
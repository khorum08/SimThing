# STUDIO-ANTIALIASING-TEST-PATTERN-0 Results

## Status

PROBATION / diagnostic feature — adds a 3D geometry AA test pattern and Video Options Debug telemetry to prove MSAA affects geometry edges independently of TypeFace/star sprite aliasing.

## PR / branch / merge

- Branch: `codex/studio-antialiasing-test-pattern-0`
- PR: #933
- Merge: `25622ed74f`

## Purpose

Studio AA telemetry confirms selected mode and camera components, but the live galaxy scene is dominated by alpha sprites, GPU text, and glow — MSAA can be active with little obvious visual change. This track adds high-contrast 3D geometry strips and a triangle fan anchored camera-relative in the lower-left viewport for objective geometry-edge AA comparison.

## Test pattern implementation

- Toggle: **Show AA test pattern** in Video Options Debug and Render debug (default off)
- 12 thin diagonal strip meshes (6 angles × 2 widths: 0.18 / 0.08 world units) plus 1 triangle fan
- `StandardMaterial` unlit opaque on `Mesh3d` through primary Camera3d (MSAA-eligible geometry)
- Camera-relative placement via `pattern_root_transform` each frame; despawned when toggled off
- Independent of galaxy generation, scenario tree, and save/load

## Video Options Debug telemetry

Reports pattern visibility, geometry instance count, material label, full AA component state, MSAA sample count, mismatch flag, and expected visible effect notes when pattern is enabled.

## AA mode comparison

Owner should compare Off vs FXAA vs SMAA vs MSAA on the test pattern geometry, not on TypeFace labels or star sprites.

## MSAA proof

- **Telemetry proof:** MSAA sample count active on Camera3d when MSAA mode selected
- **Visual proof:** diagonal test geometry visibly smoother at MSAA 4x/8x versus Off (owner smoke)

## Visual/manual smoke

Agent cannot run live Studio. Owner checklist: toggle pattern, compare all AA modes, rapid switching, toggle off leaves no artifacts.

## Focused validation only

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor antialias --lib
cargo test -p simthing-mapeditor config --lib
git diff --check
```

All commands PASS on validation host (16 antialias-filtered tests, 34 config-filtered tests).

## Tests deliberately not run

No full cargo test -p simthing-tools, no full cargo test -p simthing-mapeditor, no workspace test battery, and no nextest run were executed because this was a targeted Studio AA diagnostic feature.

## Preserved systems

Studio boot/plugin order, Camera3d path, FXAA/SMAA/MSAA options, TypeFace, LUT avoidance, nameplate/hyperlane presentation — unchanged.

## Remaining debts

- Owner live visual smoke on 2,400-star scene
- Screenshot comparison Off vs MSAA 4x/8x on test pattern

## DA recommendation

Accept as PROBATION after owner confirms test pattern toggles cleanly and MSAA sample-count telemetry matches selected mode with visible geometry-edge improvement on the pattern.

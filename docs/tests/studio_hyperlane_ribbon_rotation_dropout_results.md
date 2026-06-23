# STUDIO-HYPERLANE-RIBBON-ROTATION-DROPOUT-0 Results

## Status

PROBATION / presentation robustness fix — hyperlane ribbon basis hardened against camera-angle degeneracy; invalid mesh rebuilds rejected.

## PR / branch / merge

- Branch: `codex/studio-hyperlane-ribbon-rotation-dropout-0`
- PR: (pending)
- Merge: (pending)

## Reproduction

Owner report: at certain 3D camera rotations hyperlanes disappear while stars and labels remain; nudging rotation restores lanes. Pattern matches a transient ribbon-basis degeneracy producing an empty/invalid bucket mesh that was committed and then cached by the dirty gate.

## Root cause

When lane direction aligns with the camera view axis, `lane_dir × view_dir` crosses near zero. The prior fallback could still yield unstable geometry in edge cases, and `sync_hyperlane_colors_system` always replaced live meshes even when a rebuild produced zero vertices for visible lanes. The quantized camera key (right/up only, 0.01 quantum) could then skip further rebuilds until orientation nudged.

## Fix

- New `hyperlane_ribbon.rs`: finite width-dir chain (cross → projected camera right → projected camera up → deterministic world-axis fallback).
- `build_hyperlane_bucket_mesh` returns per-bucket stats; `hyperlane_rebuild_is_valid` rejects NaN/empty rebuilds.
- Invalid rebuild: keep prior mesh, set `cache.dirty = true`, increment telemetry, log once per state.
- Camera key includes forward vector with hyperlane-only direction quantum `0.0025`.
- Dirty gate rebuilds while RMB rotation is active and on rotation end.
- Collapsed **Hyperlane debug** telemetry section in Performance Telemetry window.

## Width-dir robustness

`hyperlane_ribbon_width_dir` always returns a finite non-zero vector via layered fallbacks; unit tests cover lane parallel to camera forward/right/up and pure degenerate cross case.

## Mesh validity guard

`hyperlane_mesh_is_valid` / `hyperlane_rebuild_is_valid` reject buckets with segments but zero verts/indices or any NaN/Inf positions when base opacity > 0.

## Camera dirty-gate behavior

`HyperlaneCameraKey` now quantizes position, right, up, **forward**, and view mode. Rebuild forced during RMB orbit drag and for one frame after release.

## Telemetry

Hyperlane debug shows sync calls, rebuild/rejected counts, last/current camera keys, camera basis vectors, per-bucket segment/vertex/index counts, degenerate width-dir (fallback-handled), NaN/Inf verts, zero-length segments.

## Visual smoke

Agent cannot capture Studio locally. Owner should rotate through the previously failing angle on the 2,400-star galaxy and confirm lanes stay visible without nudging; incident highlight and hyperlane sliders should remain functional.

## Focused validation only

```
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor hyperlane --lib
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio hyperlane rendering fix.

## Preserved systems

No changes to simthing-tools, star/nameplate falloff, TypeFace renderer, Studio boot/plugin order, Camera3d path, or map generation.

Supersedes/fixes a residual edge case in BEVY-MAPGEN-EDITOR-PR2R11 (#740).

## Remaining debts

- Owner visual confirmation at the known dropout rotation on live Studio build.

## DA recommendation

PROBATION until owner confirms hyperlanes remain visible through the previously failing rotation without nudging and Hyperlane debug shows zero rejected rebuilds during normal orbit.

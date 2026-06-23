# STUDIO-HYPERLANE-RIBBON-CAMERA-BASIS-SYNC-0 Results

## Status

PROBATION / presentation robustness fix — hyperlane ribbon meshes now rebuild from the same-frame main-camera transform with stale-basis detection and guard.

## PR / branch / merge

- Branch: `codex/studio-hyperlane-ribbon-camera-basis-sync-0`
- PR: #927
- Merge: `07d6ce51db3fbdf3d695698ad61700f52281ca3b`

## Reproduction

Owner report (post-#925): hyperlanes still disappear at specific 3D camera rotations on the 2,400-star elliptical galaxy; nudging rotation restores them. Pattern matches mesh built from a stale camera basis while the frame renders with the updated camera.

## Root cause

`sync_hyperlane_colors_system` ran in `Update` and read `GlobalTransform` on `MainCamera`. Bevy propagates `GlobalTransform` in `PostUpdate`, so the hyperlane rebuild used **last frame's** camera basis while rendering used **this frame's** local `Transform` written by `camera_control_system`. After rotation stopped, the dirty gate could accept a quantized key built from the stale basis and skip further rebuilds until a nudge changed the key.

## Fix

- Move `sync_hyperlane_colors_system` to `PostUpdate` after `camera_control_system`.
- Read `Transform` on root `MainCamera` (same basis the renderer uses; no parent hierarchy).
- Store `last_mesh_build_basis` in `HyperlaneRenderCacheState`; compare each frame with `hyperlane_basis_mismatch_exceeds_epsilon` (0.35°).
- Extend `hyperlane_render_should_rebuild` with `basis_mismatch` trigger; increment `stale_basis_rebuild_count` when guard fires.
- Preserve #925 width-dir fallbacks, invalid mesh rejection, forward in camera key, and RMB orbit rebuild triggers.

## System ordering / camera-basis source

| System | Schedule | Camera source |
|---|---|---|
| `camera_control_system` | Update (chain) | writes `Transform` on `MainCamera` |
| `sync_hyperlane_colors_system` | PostUpdate `.after(camera_control_system)` | reads `Transform` → `hyperlane_camera_basis_from_transform` |

## Stale-basis guard

`HYPERLANE_BASIS_MISMATCH_REBUILD_EPSILON_DEG = 0.35`. When any axis (right/up/forward) differs between current camera basis and last successful mesh-build basis by more than epsilon, force rebuild even if the quantized camera key is unchanged.

## Telemetry proof

Hyperlane debug section now reports:

- Current vs mesh-build camera keys and basis vectors (right/up/forward)
- Basis mismatch angles (R/U/F) and `basis_mismatch_active`
- Frames since last rebuild, RMB orbit active, rotation delta since rebuild
- Stale-basis rebuild count

During normal rotation these should stay within epsilon; dropout angles should show nonzero mismatch only transiently until guard rebuilds.

## Visual smoke

Agent cannot capture Studio locally. Owner should rotate through the previously failing angle on the 2,400-star galaxy, stop without nudging, and confirm hyperlanes remain visible; incident highlights and Overhead/3D toggle should remain functional.

## Focused validation only

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor hyperlane --lib
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio hyperlane camera-basis synchronization fix.

## Preserved systems

- GPU TypeFace star nameplates, nameplate falloff/size/high-horizon fixes
- Star rendering, Studio boot/plugin order, `SimthingToolsTextPlugin::world_text_only()`
- `without_lut_d3_view_fix()`, Camera3d path, egui panels, selected-system panel
- #925 mesh validity guard and width-dir fallbacks (unchanged)

## Remaining debts

- Owner visual smoke on the known dropout angle after merge
- If residual dropout persists, capture telemetry snapshot at failure frame for epsilon tuning

## DA recommendation

Accept as PROBATION presentation fix pending owner visual confirmation on the 2,400-star reproduction case. Mark STUDIO-HYPERLANE-RIBBON-ROTATION-DROPOUT-0 ACCEPTED for width-dir/mesh-validity hardening; residual dropout addressed by this camera-basis sync rung.

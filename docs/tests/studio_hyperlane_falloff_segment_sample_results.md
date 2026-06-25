# STUDIO-HYPERLANE-FALLOFF-SEGMENT-SAMPLE-0 Results

## Status

PROBATION — targeted Studio hyperlane map-radius falloff sampling fix.

## PR / branch / merge

- Branch: `codex/studio-hyperlane-falloff-segment-sample-0`
- PR: (pending)
- Merge: (pending)

## Reproduction

On a generated galaxy with steep close camera angles and tight hyperlane falloff:

```text
Hyperlane Falloff Distance = 10%
Hyperlane Falloff Opacity = 0%
```

Long hyperlanes that cross near the map-radius view origin could disappear entirely when their segment midpoint lay outside the 10% plateau, even though a portion of the lane was near the camera/view origin. A slight camera tilt caused large pop-in/pop-out.

## Root cause

`hyperlane_map_radius_progress_percent` sampled only the XZ midpoint of each hyperlane segment. `build_hyperlane_bucket_mesh` applied that single progress value to the whole lane ribbon; when opacity reached zero the lane was skipped entirely (`!visual.visible`).

## Fix

Replaced midpoint-only falloff sampling with closest-point-on-segment sampling relative to `ctx.view_origin`. Preserved camera-depth bucket classification (still midpoint-based) and all ribbon/camera-basis paths from #925/#927.

## Closest-point falloff sample

Added `closest_point_on_segment_2d` and updated `hyperlane_map_radius_progress_percent` to project the view origin onto each lane segment before calling `map_radius_progress_percent`. Retained `hyperlane_midpoint_map_radius_progress_percent` for telemetry comparison.

## Falloff-cull telemetry

- `HyperlaneMeshStats.falloff_culled_segment_count` increments when a lane is skipped for `!visual.visible`.
- Aggregated as `hyperlane_falloff_culled_segment_count` in Performance Telemetry → collapsed **Hyperlane debug**:
  - `Hyperlane falloff sample: closest segment point`
  - `Sample midpoint progress %` / `Sample closest progress %` (first source segment)
  - `Falloff-culled segments: <n>`

## Camera-basis telemetry

Unchanged from #927. Hyperlane debug still reports basis mismatch (near zero while idle), invalid rebuild rejected count, NaN/Inf vertex count, and mesh-build camera key alignment.

## Visual smoke

Manual smoke (owner session): same generated galaxy and steep/zoomed angle as prior dropout reports.

Settings: Hyperlane Falloff Distance 10%, Hyperlane Falloff Opacity 0%.

Expected after fix:

1. Foreground-crossing hyperlanes remain visible at the previously bad angle.
2. Camera tilt no longer causes large hyperlane pop-in/pop-out from midpoint-only culling.
3. Distant hyperlanes still fade per slider.
4. Falloff Distance 50% / 100% behavior unchanged.
5. Stars and TypeFace nameplates unchanged.
6. Hyperlane camera-basis telemetry healthy (mismatch ~0 idle, rejected rebuilds stable, NaN/Inf verts 0).

## Focused validation only

- PASS — `cargo fmt -p simthing-mapeditor -- --check`
- PASS — `cargo check -p simthing-mapeditor`
- PASS — `cargo test -p simthing-mapeditor hyperlane --lib`
- PASS — `git diff --check`

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio hyperlane falloff-sampling fix.

## Preserved systems

- Camera-facing hyperlane ribbon basis (#925 width-dir fallbacks)
- Camera-basis synchronization (#927)
- Invalid mesh rebuild rejection
- Hyperlane AA settings, star rendering, GPU TypeFace nameplates
- Nameplate falloff and relative size
- Map-radius plateau model
- Studio boot/plugin order, `SimthingToolsTextPlugin::world_text_only()`, `without_lut_d3_view_fix()`
- No changes to `simthing-tools`, `simthing-spec`, `simthing-driver`, `simthing-sim`, `simthing-gpu`

## Remaining debts

- Whole-lane single-sample falloff (no per-vertex/per-fragment gradient along the ribbon).
- Bucket depth classification still uses camera distance to segment midpoint; may be revisited if ordering artifacts appear at extreme angles.

## DA recommendation

Accept as PROBATION presentation math fix pending owner visual confirmation on the known steep/zoomed reproduction. Falloff-cull telemetry should drop at the bad angle versus midpoint-only sampling.

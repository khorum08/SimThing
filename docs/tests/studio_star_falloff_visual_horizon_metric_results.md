# STUDIO-STAR-FALLOFF-VISUAL-HORIZON-METRIC-0 Results

## Status

PROBATION / presentation math fix — star and nameplate falloff use visual foreground-to-horizon progress.

## PR / branch / merge

- Branch: `codex/studio-star-falloff-visual-horizon-metric-0`
- PR: (pending)
- Merge: (pending)

## Root cause

Falloff sliders used `normalized_billboard_camera_depth_percent` (camera-distance ruler). That does not match the tilted 3D galaxy view where owners expect 0% at visual foreground and 100% at the central vanishing point.

## Fix

- `visual_horizon_falloff_progress_percent` projects anchors to screen space and measures progress from viewport bottom center to center.
- `sync_star_visuals_system` and nameplate telemetry use `star_falloff_progress_percent` (default visual horizon).
- GPU screen-label shader computes the same visual horizon progress for nameplate falloff alpha.
- Telemetry exposes metric, ruler endpoints, sample screen position, visual progress %, star alpha, label falloff alpha.

## Visual horizon metric

```text
base = viewport bottom center
vanishing = viewport center
progress = dot(screen - base, vanishing - base) / |vanishing - base|^2
progress_percent = clamp(progress, 0, 1) × 100
```

## Relative falloff formula

Unchanged from #921:

```text
effective_nameplate_falloff = min(star_falloff × relative / 100, star_falloff)
nameplate_alpha = star_alpha × label_ramp(effective, relative_transparency)
```

## Telemetry proof

Nameplate debug shows falloff metric, sample screen px, ruler base/vanishing px, visual progress %, star/label alphas.

## Visual smoke

Agent cannot capture Studio locally. Owner should verify 8% × 10% = 0.8% effective leaves only closest foreground labels at 0% relative transparency.

## Relative size preserved

No changes to #922 uniform relative size shader path.

## Focused validation only

```
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor nameplate --lib
git diff --check
```

If simthing-tools touched:

```
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check
cargo check -p simthing-tools --features world-text-3d
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio falloff-metric fix.

## Remaining debts

- Owner visual confirmation on 2,400-star galaxy (8%/10%, 50%/50%, camera tilt).
- GPU path always uses visual horizon; camera-distance debug is CPU/telemetry-only unless extended later.

## DA recommendation

PROBATION until owner confirms falloff tracks visual horizon and 0.8% effective culls distant labels at 0% relative transparency.

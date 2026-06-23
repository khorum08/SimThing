# STUDIO-FALLOFF-MAP-RADIUS-ORIGIN-STABILITY-0 Results

## Status

PROBATION / presentation math fix — stabilizes map-radius plateau falloff origin across camera height/angle.

## PR / branch / merge

- Branch: `codex/studio-falloff-map-radius-origin-stability-0`
- PR: #929
- Merge: `bcc4ba519b`

## Reproduction

On the 2,400-star elliptical galaxy, certain camera heights/angles caused nearby stars/nameplates to dissolve while farther map regions stayed bright. A slight camera nudge often corrected the falloff, indicating stale or inverted view-origin computation rather than a broken plateau model.

## Root cause

Two compounding issues:

1. **Viewport coordinate bug:** `viewport_bottom_center_map_origin` cast a ray from `(width * 0.5, 0.0)`. Bevy `Camera::viewport_to_world` uses y-down physical pixels (origin top-left), so y=0 is **top-center**, not bottom-center — inverting the falloff origin toward the horizon/far side at shallow pitch.

2. **Stale camera pose:** `update_map_radius_falloff_context_system` ran before `camera_control_system` and read `GlobalTransform`, which lagged the frame's local `Transform` written by camera control — one-frame origin drift during orbit/tilt.

## Fix

- Bottom-center ray now uses `(width * VIEWPORT_BOTTOM_CENTER_X_FRACTION, height * VIEWPORT_BOTTOM_CENTER_Y_FRACTION)` with y-down convention documented and unit-tested.
- Falloff context update moved to run immediately **after** `camera_control_system`; reads current-frame `Transform` via `GlobalTransform::from(*transform)`.
- Origin resolution adds bounds clamping, far-outside fallback to camera focus, context validity checks, and retention of the previous valid context when a frame computes an invalid result.
- Expanded Falloff debug telemetry section.

## Viewport coordinate convention

**y-down physical pixels** (Bevy `viewport_to_world`, origin top-left). Bottom-center = `(0.5 * width, 1.0 * height)`.

## Camera scheduling / stale context fix

Update chain order: `camera_control_system` → `update_map_radius_falloff_context_system` → star/nameplate/hyperlane sync. Same-frame camera `Transform` is used for origin ray cast.

## Origin validity and clamping

- Ray miss / t &lt; 0 → camera focus projected
- Hit outside map bounds by &gt; one map extent margin → camera focus projected
- Hit slightly outside bounds → clamp to AABB (`BottomCenterViewportRayClampedToBounds`)
- Invalid max distance → retain previous valid context or mark invalid (no bad GPU patch)

## Telemetry proof

Telemetry → **Falloff debug** (collapsed) reports viewport convention, bottom-center px, raw ray origin/direction, raw map-plane hit, origin source, clamp flag, final origin, bounds, max view distance, sample star map distance/progress, context frame, updated-after-camera flag, retained-context flag.

## Visual smoke

Agent cannot capture Studio locally. Owner should reproduce the prior bad camera angle, confirm Falloff debug shows bottom-center y ≈ viewport height (not 0), and verify near/foreground stars stay inside plateau without nudge correction.

## Slider preservation

All star/nameplate/hyperlane Settings sliders unchanged; map-radius plateau model unchanged.

## Focused validation only

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor falloff --lib
cargo test -p simthing-mapeditor nameplate --lib
cargo test -p simthing-mapeditor hyperlane --lib
git diff --check
```

All commands PASS on validation host (35 falloff-filtered lib tests).

## Tests deliberately not run

No full cargo test -p simthing-tools, no full cargo test -p simthing-mapeditor, no workspace test battery, and no nextest run were executed because this was a targeted Studio falloff-origin stability fix.

## Remaining debts

- Owner visual smoke at previously failing camera height/angle
- No map-radius debug ring overlay (optional, unchanged)

## DA recommendation

Accept as PROBATION after owner confirms the inversion/nudge bug is gone on the 2,400-star galaxy; promote STUDIO-FALLOFF-MAP-RADIUS-PLATEAU-0 plateau model as ACCEPTED baseline.

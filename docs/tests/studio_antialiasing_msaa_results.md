# STUDIO-ANTIALIASING-MSAA-0 Results

## Status

PROBATION / presentation settings feature — adds mutually exclusive MSAA 2x/4x/8x modes and expands Video Options Debug telemetry for FXAA/SMAA/MSAA active camera state.

## PR / branch / merge

- Branch: `codex/studio-antialiasing-msaa-0`
- PR: (pending)
- Merge: (pending)

## Purpose

Extend Studio antialiasing beyond FXAA/SMAA post-process modes with Bevy-native MSAA geometry-edge options, while preserving the stable Camera3d / TypeFace / egui render path. MSAA is expected to help hyperlane/star/ribbon mesh edges more than alpha-blended TypeFace glyph interiors.

## Implemented modes

- Off (default)
- FXAA
- SMAA Low / Medium / High / Ultra
- MSAA 2x / 4x / 8x

## Mutual exclusion behavior

Settings radio buttons over a single `StudioAntialiasingMode` enum. `apply_studio_antialiasing_mode` inserts either `Fxaa`, `Smaa`, or `Msaa` sample count on MainCamera and removes conflicting components. Off clears post-AA components and sets `Msaa::Off`.

## MSAA implementation

Bevy 0.16 `Msaa` component on the primary Camera3d (`Msaa::Off`, `Sample2`, `Sample4`, `Sample8`). Camera spawn seeds `Msaa::Off` so Off mode is truthful despite Bevy’s default `Sample4` on `Camera` required components. MSAA scope telemetry: **primary Camera3d component**.

## SMAA postprocess telemetry

Component presence proves Bevy has the SMAA postprocess component attached to the camera; it does not prove subjective image quality.

Telemetry distinguishes:

- SMAA selected: yes/no
- SMAA camera component present: yes/no
- SMAA preset: Low/Medium/High/Ultra/none
- SMAA expected active this frame: yes/no
- SMAA postprocess application: inferred active from Camera3d Smaa component
- SMAA pass timing: unavailable from current Bevy diagnostics

Bevy does not expose an isolated SMAA pass timer in current Studio telemetry; coarse frame total ms is surfaced when available.

## Video Options Debug telemetry

Extended collapsed **Video Options Debug** group reports selected mode, mode source, FXAA/SMAA/MSAA component state, MSAA sample count, post-AA active flag, mismatch detection, apply generation, last applied mode/frame, window scale factor, and coarse frame timing when available.

## Persistence

`antialiasing_mode` persists through existing `simthing-studio-config.json` and RON settings. New MSAA enum values serialize as `msaa_2x`, `msaa_4x`, `msaa_8x`. Invalid values normalize to Off with config warning.

## Visual/manual smoke

Agent cannot run live Studio. Owner should verify Telemetry → Video Options Debug on the 2,400-star elliptical scene across all nine modes, rapid switching, and restart persistence per task checklist.

## Focused validation only

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor antialias --lib
cargo test -p simthing-mapeditor config --lib
git diff --check
```

All commands PASS on validation host (13 antialias-filtered tests, 34 config-filtered tests).

## Tests deliberately not run

No full cargo test -p simthing-tools, no full cargo test -p simthing-mapeditor, no workspace test battery, and no nextest run were executed because this was a targeted Studio AA settings feature.

## Preserved systems

Studio boot/plugin order, Camera3d path, TypeFace world_text_only, LUT avoidance, map-radius falloff, nameplate/hyperlane presentation, egui panels — unchanged. No render-plugin surgery, Camera2d, or simthing-tools changes.

## Deferred modes

TAA remains deferred due ghosting/blur risk for moving camera and GPU TypeFace labels.

## Remaining debts

- Owner live visual smoke and screenshot comparison for MSAA on hyperlane/star edges
- SMAA pass timing would require render-graph instrumentation

## DA recommendation

Accept as PROBATION after owner confirms MSAA modes apply cleanly with Video Options Debug readouts matching Settings and no render regressions on the 2,400-star scene.

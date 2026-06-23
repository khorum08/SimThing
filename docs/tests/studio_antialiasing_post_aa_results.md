# STUDIO-ANTIALIASING-POST-AA-0 Results

## Status

PROBATION / presentation settings feature — adds mutually exclusive Bevy FXAA/SMAA Studio antialiasing modes.

## PR / branch / merge

- Branch: `codex/studio-antialiasing-post-aa-0`
- PR: (pending)
- Merge: (pending)

## Purpose

Reduce visible aliasing on distant stars, hyperlanes, and GPU TypeFace nameplates using Bevy-native post-process antialiasing without disturbing the stable Camera3d / TypeFace / egui render stack.

## Implemented modes

- Off (default)
- FXAA
- SMAA Low
- SMAA Medium
- SMAA High
- SMAA Ultra

## Deferred modes

TAA deferred due ghosting/blur risk for moving camera and GPU TypeFace labels.
MSAA deferred to a future track because it has a different geometry-edge cost profile and higher render-target cost.

## Mutual exclusion behavior

Settings UI uses radio buttons over a single `StudioAntialiasingMode` enum. `apply_studio_antialiasing_mode` inserts either `Fxaa` or `Smaa` on the MainCamera entity and removes the other component; Off removes both.

## Live settings behavior

Changing the Antialiasing radio selection applies immediately to the active Camera3d via `apply_antialiasing_settings` in the Settings dialog. A follow-up `sync_studio_antialiasing_system` keeps startup/config loads aligned. No galaxy topology, ScenarioSpec, TypeFace atlas, or nameplate rebuild occurs on AA change.

## Persistence

`antialiasing_mode` persists in `simthing-studio-config.json` and `EditorSettings` (RON). Legacy configs without the field default to Off. Saved on Settings close, app exit, and window persist path.

## Visual smoke

Agent cannot capture Studio locally. Owner should verify each mode on the 2,400-star galaxy: immediate apply, no black screen, egui remains readable, no dual FXAA+SMAA accumulation, restart restores selection.

## Focused validation only

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor antialias --lib
cargo test -p simthing-mapeditor config --lib
git diff --check
```

All commands PASS on validation host (7 antialias-filtered tests, 32 config-filtered tests).

## Tests deliberately not run

No full cargo test -p simthing-tools, no full cargo test -p simthing-mapeditor, no workspace test battery, and no nextest run were executed because this was a targeted Studio AA settings feature.

## Preserved systems

Studio boot/plugin order, Camera3d path, TypeFace world_text_only, LUT avoidance, map-radius falloff, nameplate/hyperlane presentation, egui panels — unchanged.

## Remaining debts

- Owner visual smoke across all six modes
- TAA / MSAA future tracks

## DA recommendation

Accept as PROBATION after owner confirms live FXAA/SMAA switching is stable on the 2,400-star galaxy with no render regressions.

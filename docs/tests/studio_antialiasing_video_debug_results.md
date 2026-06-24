# STUDIO-ANTIALIASING-VIDEO-DEBUG-0 Results

## Status

PROBATION / telemetry follow-up — adds collapsed Video Options Debug telemetry to confirm selected FXAA/SMAA mode and active Bevy Camera3d AA components.

## PR / branch / merge

- Branch: `codex/studio-antialiasing-video-debug-0`
- PR: (pending)
- Merge: (pending)

## Purpose

The owner could not tell whether Studio antialiasing settings were actually applied to the active Camera3d. FXAA appeared to work, but SMAA/MSAA confusion persisted in the UI. This track adds telemetry-only confirmation of selected mode vs live Bevy post-process components without changing AA behavior.

Video Options Debug confirms selected AA mode and active Bevy camera components. Visual quality still requires owner screenshot comparison.

## Video Options Debug telemetry

Telemetry → **Video Options Debug** (collapsed by default, near Falloff/Hyperlane debug) reports:

- Selected AA mode (Off / FXAA / SMAA Low–Ultra)
- Mode source: `current UI state`, `loaded studio config`, or `default fallback`
- Primary Camera3d entity id
- FXAA / SMAA component presence and SMAA preset
- Dual post-AA active flag
- AA settings generation, last applied mode, last applied frame
- GPU adapter / backend / window scale factor when available

## AA component audit

`update_studio_antialiasing_video_debug_system` queries `MainCamera` for `Fxaa` and `Smaa` each frame and compares against `StudioAntialiasingMode` via `antialiasing_component_state_mismatch`.

Expected states:

| Selected | FXAA | SMAA | Preset |
|---|---|---|---|
| Off | no | no | none |
| FXAA | yes | no | none |
| SMAA * | no | yes | matching preset |

## MSAA/TAA deferred status

MSAA and TAA are not implemented by this track. MSAA remains a future handoff; TAA remains deferred due ghosting/blur risk.

Telemetry explicitly prints:

```text
MSAA: deferred / not implemented in Studio AA mode
TAA: deferred / not implemented in Studio AA mode
```

## Mismatch detection

When selected mode and camera components diverge (including missing primary camera), telemetry shows **AA STATE MISMATCH** in yellow in the Telemetry dialog.

Unit tests cover mismatch when FXAA is selected but SMAA components remain.

## Persistence check

Mode source is set to `loaded studio config` on startup when JSON loads successfully; UI edits set `current UI state`. Restart persistence uses the existing #930 config path — owner should confirm selected mode and component telemetry match after reload.

## Visual/manual smoke

Agent cannot run live Studio on the 2,400-star elliptical scene. Owner checklist:

1. Off — no FXAA/SMAA components; MSAA/TAA deferred lines present
2. FXAA — FXAA yes, SMAA no, no mismatch
3. SMAA Low/Medium/High/Ultra — SMAA yes with matching preset, FXAA no
4. Rapid FXAA ↔ SMAA switching — no dual FXAA+SMAA persistence
5. Restart — selected mode and telemetry align

## Focused validation only

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor antialias --lib
cargo test -p simthing-mapeditor config --lib
git diff --check
```

All commands PASS on validation host (10 antialias-filtered tests, 34 config-filtered tests).

## Tests deliberately not run

No full cargo test -p simthing-tools, no full cargo test -p simthing-mapeditor, no workspace test battery, and no nextest run were executed because this was a targeted Studio AA telemetry follow-up.

## Preserved systems

Studio boot/plugin order, Camera3d path, TypeFace world_text_only, LUT avoidance, map-radius falloff, nameplate/hyperlane presentation, egui panels — unchanged. No MSAA/TAA implementation added.

## Remaining debts

- Owner live visual/manual smoke on 2,400-star scene
- Visual quality proof via screenshot comparison
- MSAA / TAA future tracks

## DA recommendation

Accept as PROBATION after owner confirms Video Options Debug readouts match Settings selections across all six modes with no render regressions.

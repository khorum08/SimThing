# STUDIO-STAR-NAMEPLATE-SETTINGS-VISIBILITY-0 Results

## Status

PROBATION / presentation cleanup — settings-driven nameplate visibility is the production default; debug overrides demoted.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-settings-visibility-0`
- PR: (pending)
- Merge: (pending)

## Context

After #913–#917, GPU screen-label nameplates render correctly through simthing-tools TypeFace, but the Telemetry “Nameplate debug mode” dropdown exposed Force All as a normal option. Force All bypassed settings-driven falloff and hid the production visibility path behind debug scaffolding.

## Fix

Rename production default `AutoLod` → `SettingsDriven`. In settings-driven mode, global density alpha is always 1.0 (no overview mass-hide); per-label visibility comes from Settings sliders and shader falloff. Debug modes (`FocusedOnlyDebug`, `ForceAllDebug`) moved under collapsed Telemetry “Debug overrides” with explicit warnings.

## Default visibility mode

`SettingsDriven` — startup default; not persisted (resets each session).

## Settings slider behavior

Documented in `star_nameplate_gpu_screen_label`:

- Relative width → natural text width scale (not fixed plate).
- Base transparency → alpha ceiling vs star opacity.
- Relative falloff distance → label falloff distance vs star falloff distance.
- Relative falloff transparency → label alpha target at label falloff.

Readability safety floor: unselected labels below 24 px projected height hard-cull.

## Debug override treatment

Telemetry → Nameplate debug → **Debug overrides** (collapsed, default closed). Combo includes Settings driven plus debug modes labeled “(debug)”. Force All shows yellow warning when active.

## Telemetry

Nameplate debug shows visibility mode, effective Settings slider values, drawn/cull counts, and `DEBUG OVERRIDE ACTIVE` when applicable. Global LOD alpha remains 1.0 in settings-driven mode.

## Studio boot constraints preserved

No changes to GpuScreenLabel shader placement, atlas residency, boot plugins, Camera3d, or simthing-tools glyph path.

## Visual smoke

Agent cannot capture Studio locally. Owner should verify default is Settings driven, sliders affect visibility live, and Force All is only under Debug overrides.

## Focused validation only

```
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor nameplate --lib
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio presentation cleanup.

## Remaining debts

- Owner visual confirmation that overview label count responds to falloff sliders, not Force All.
- Extreme overview may still show many labels if falloff sliders allow; readability floor is the only hard safety gate.

## DA recommendation

PROBATION until owner confirms Settings sliders predictably control nameplate visibility and debug overrides are clearly non-production.

# STUDIO-STAR-NAMEPLATE-SLIDER-AUTHORITY-0 Results

## Status

PROBATION / presentation policy fix — sliders govern default nameplate visibility; hidden LOD gates moved to optional debug modes.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-slider-authority-0`
- PR: (pending)
- Merge: (pending)

## Root cause

#919 renamed `AutoLod` to `SettingsDriven` and removed global density hide, but left `min_unselected_px = 24` in the default LOD patch and in the GPU screen-label shader path. The shader also treated `min_unselected_px < 0.5 && min_focused_px < 0.5` as `force_all_labels`, which skipped falloff/offscreen culls — conflating “no readability floor” with “ignore slider falloff.”

## Fix

- Default mode `AllLabelsSettingsDriven`: `min_unselected_px = 0`, `min_focused_px = 0`, `unselected_global_alpha = 1.0`.
- CPU gates return true for readability/density in default mode.
- Shader gpu_screen_label: apply readability/density only when thresholds `> 0.5`; always apply offscreen and `falloff_alpha` culls unless `ForceAllDebug` (negative sentinel).
- Restored visible Telemetry mode dropdown with `AutoLodDebug` as optional assist.
- `ForceAllDebug` bypasses offscreen/LOD culls but still respects Settings falloff alpha.

## Default visibility mode

`AllLabelsSettingsDriven` — “All labels — settings driven”.

## Slider authority

Settings sliders drive `WorldTextBillboard` base alpha, relative falloff distance/transparency, and width ratio; shader `falloff_alpha` culls at epsilon 0.02. No hidden 24px floor in default mode.

## Debug modes restored

- Auto LOD debug — 24px/16px floors + density cap
- Focused only debug — hide unselected
- Force all debug — bypass offscreen/LOD; falloff sliders still apply

## Telemetry proof

Shows mode, debug override yes/no, LOD patch min px / global alpha, Settings %, drawn/culled LOD/alpha/offscreen counts.

## Studio boot constraints preserved

Atlas residency, GpuScreenLabel placement, vertex_index quads, baseline fix, boot plugins unchanged.

## Visual smoke

Agent cannot capture Studio locally. Owner should verify slider sweeps on 2,400-star galaxy.

## Focused validation only

```
cargo fmt -p simthing-mapeditor -p simthing-tools -- --check
cargo check -p simthing-mapeditor
cargo check -p simthing-tools --features world-text-3d
cargo test -p simthing-mapeditor nameplate --lib
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio presentation policy fix.

## Remaining debts

- Owner visual confirmation that Base Transparency 0% hides all labels and falloff sliders fade predictably.
- Force-all debug negative sentinel is a shader contract; document if extended.

## DA recommendation

PROBATION until owner confirms sliders alone govern default visibility without hidden LOD surprises.

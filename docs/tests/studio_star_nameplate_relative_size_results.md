# STUDIO-STAR-NAMEPLATE-RELATIVE-SIZE-0 Results

## Status

PROBATION / presentation math fix — Nameplate Relative Width reinterpreted as uniform Nameplate Relative Size.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-relative-size-0` (deleted after merge)
- PR: #922 — https://github.com/khorum08/SimThing/pull/922
- Merge: `6d7e2f7e886ab15c417756a38b3d3381c3d81d97` on `master`

## Root cause

GPU screen-label shader multiplied `local_xy.x` by `label_height * width_ratio` while `local_xy.y` used only `label_height`, so the Settings slider stretched or squashed labels horizontally without scaling height.

## Fix

- Shader: `scaled_label_height_px = effective_label_height_px * relative_size`; both x and y offsets use `scaled_label_height_px`; gap scales with scaled height.
- Rust: `nameplate_scaled_label_height_px()` mirrors shader; telemetry width = run aspect × scaled height.
- UI renamed to **Nameplate Relative Size** with uniform-scale tooltip.
- Serialized field `relative_width_percent` retained for config compatibility.

## Size contract

```text
uniform_size_ratio = slider_percent / 100
label_height_px = projected_star_visual_diameter_px * uniform_size_ratio
label_width_px = natural_run_aspect * label_height_px
```

## Config compatibility

`relative_width_percent` / `nameplate_relative_width_percent` unchanged in serde; comments document historical name interpreted as relative size. TODO: rename with migration in a follow-up.

## Telemetry proof

Nameplate debug shows relative size %, sample label height px, natural run aspect, computed width px (run span × scaled height).

## Visual smoke

Agent cannot capture Studio locally. Owner should verify 50%, 100%, 200% uniform scaling and star blur radius still drives base label height.

## Falloff preserved

No changes to #921 effective falloff formula or relative falloff transparency behavior.

## Focused validation only

```
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check
cargo check -p simthing-tools --features world-text-3d
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor nameplate --lib
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio nameplate size-scaling fix.

## DA recommendation

PROBATION until owner confirms 50%/100%/200% scale height and width uniformly with no horizontal squashing.

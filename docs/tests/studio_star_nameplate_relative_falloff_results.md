# STUDIO-STAR-NAMEPLATE-RELATIVE-FALLOFF-0 Results

## Status

PROBATION / presentation math fix — nameplate falloff distance is a strict multiplier of star falloff distance.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-relative-falloff-0`
- PR: (pending)
- Merge: (pending)

## Root cause

CPU telemetry multiplied `relative_target_alpha` as a flat factor instead of using the shader distance-falloff curve. The shader applied star and label falloff as two independent curves without clamping effective distance to star falloff. `WorldTextBillboard::clamped()` forced `relative_falloff_percent` minimum 0.01, distorting small products.

## Fix

- `nameplate_effective_falloff_distance_percent(star, relative) = (star × relative / 100).min(star)`
- `star_nameplate_gpu_screen_label` stores effective distance in `relative_falloff_percent`
- Shader clamps `effective_falloff_at = min(style_params.z, star_falloff_at)` and applies star ceiling × label ramp
- CPU `nameplate_gpu_screen_label_falloff_alpha` mirrors shader for telemetry/cull counts
- Telemetry exposes star/relative/effective falloff %, sample depth, falloff alpha

## Relative falloff formula

```text
effective = min(star_falloff_distance × nameplate_relative_falloff_distance / 100, star_falloff_distance)
label_alpha = base_transparency × distance_falloff(star) × distance_falloff(effective, relative_transparency)
```

## Telemetry proof

Nameplate debug shows star falloff %, relative %, effective %, sample depth %, falloff alpha, past-effective cull count.

## Visual smoke

Agent cannot capture Studio locally. Owner should verify cases A–F from task brief.

## Dropdown / debug modes

Unchanged: default All labels — settings driven; falloff sliders apply in all modes including Force all debug.

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

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio nameplate falloff math fix.

## Remaining debts

- Owner visual confirmation of cases A–F on 2,400-star galaxy.

## DA recommendation

PROBATION until owner confirms effective falloff telemetry matches star × relative and 0% relative transparency hides labels past effective distance.

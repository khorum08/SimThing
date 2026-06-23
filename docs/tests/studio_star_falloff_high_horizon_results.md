# STUDIO-STAR-FALLOFF-HIGH-HORIZON-0 Results

## Status

PROBATION / presentation math fix — visual falloff vanishing point moved to high horizon at 25% from top.

## PR / branch / merge

- Branch: `codex/studio-star-falloff-high-horizon-0`
- PR: (pending)
- Merge: (pending)

## Root cause

Visual-horizon falloff (#923) used viewport center as the 100% vanishing point. Owner falloff intent reads as a high horizon (~25% from top), not screen center.

## Fix

- `STAR_FALLOFF_VANISHING_Y_FRACTION = 0.25` (centered horizontally at 0.5 width).
- Rust `VisualHorizonFalloffRuler` and GPU screen-label shader share the same fractions.
- Telemetry label updated to **Visual high horizon**.

## High horizon metric

```text
base      = (0.5 × width, 1.0 × height)
vanishing = (0.5 × width, 0.25 × height)
progress  = clamp(dot(screen - base, vanishing - base) / |ruler|², 0, 1) × 100
```

## Telemetry proof

Nameplate debug shows falloff metric, ruler base/vanishing px, sample visual progress %, effective falloff, alphas.

## Visual smoke

Agent cannot capture Studio locally. Owner should verify 10%/50%/100% falloff against bottom-center → high-horizon ruler.

## Relative falloff preserved

No changes to #921 effective multiplier formula.

## Relative size preserved

No changes to #922 uniform relative size path.

## Focused validation only

```
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo check -p simthing-tools --features world-text-3d
cargo test -p simthing-mapeditor nameplate --lib
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio visual falloff-ruler fix.

## Remaining debts

- Owner visual confirmation of high-horizon perspective on 2,400-star galaxy.

## DA recommendation

PROBATION until owner confirms 50% falloff reaches mid-way toward high horizon (not screen center) and 100% reaches ~25% from top.

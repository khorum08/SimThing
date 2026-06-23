# STUDIO-FALLOFF-MAP-RADIUS-PLATEAU-0 Results

## Status

PROBATION / presentation math fix — production Studio star/nameplate/hyperlane falloff now uses a map-plane radius plateau ruler; existing Settings sliders preserved.

## PR / branch / merge

- Branch: `codex/studio-falloff-map-radius-plateau-0`
- PR: (pending)
- Merge: (pending)

## Root cause

Star, nameplate, and hyperlane falloff used a visual high-horizon / screen-progress ruler (`StarFalloffMetric::VisualHorizon`). Sliders behaved like perspective-depth controls rather than answering “how much of the scenario map should remain fully visible from the current view origin?”

## Fix

- Added `crates/simthing-mapeditor/src/falloff_metric.rs` with `StudioMapRadiusFalloffContext`, view-origin resolution, map-corner max distance, `plateau_falloff_t`, and shared progress helpers.
- Default `StarFalloffMetric` → `MapRadiusPlateau`; visual-horizon metric retained as debug-only.
- Stars, nameplates, and hyperlanes share the same `range_progress` when plateau is active.
- Plateau curve: full base visibility inside slider plateau; linear fade to falloff targets across remaining map radius (no smoothstep).
- `simthing-tools` GPU screen labels: `WorldTextFalloffRulerPatch` + WGSL mode 2.0 for map-radius plateau parity on GPU nameplates.

## Map-radius metric

- **View origin:** bottom-center viewport ray ∩ map plane (y=0); fallbacks: camera focus projected → camera position projected → galaxy center.
- **Map max view distance:** max distance from view origin to AABB corners derived from render anchors.
- **Range progress:** `clamp(distance / map_max_view_distance, 0, 1)` per star/nameplate anchor / hyperlane segment midpoint.

## Plateau behavior

- Star Falloff Distance = plateau end as fraction of map radius; inside → base blur/opacity; beyond → linear lerp to falloff blur/opacity by map edge.
- Nameplate effective plateau = `min(star_falloff × relative_falloff, star_falloff)` on the same ruler; labels never outlive stars.
- Hyperlane Falloff Distance uses identical plateau semantics for thickness/opacity.
- Falloff Distance = 100% → no fade across map radius (plateau covers full range).

## Slider preservation

All existing Settings sliders unchanged (labels retained; tooltips updated):

- Stars: Base Star Blur Radius, Falloff Distance, Falloff Star Blur Radius, Falloff Star Opacity
- Nameplates: Nameplate Relative Size, Base Transparency, Relative Falloff Distance, Relative Falloff Transparency
- Hyperlanes: Base Hyperlane Line Thickness, Base Hyperlane Opacity, Falloff Distance, Falloff Thickness, Falloff Opacity

## Star falloff behavior

Unit tests prove base visibility inside plateau, falloff targets at map edge (100% progress), and aura reduction from mid progress to edge when plateau &lt; 100%.

## Nameplate falloff behavior

Relative multiplier formula preserved; GPU/CPU plateau alpha uses `world_text_plateau_falloff`; zero relative falloff transparency fades to invisible at map edge past effective plateau.

## Hyperlane falloff behavior

`compute_hyperlane_visual` and bucket mesh build take map-radius progress when plateau active; depth buckets still use camera distance for near/mid/far ordering only.

## Telemetry proof

Nameplate / render debug telemetry extended with:

- Falloff metric active (map radius plateau vs debug visual horizon)
- View origin x/z, map max view distance, origin source
- Sample range distance, range progress %, star/nameplate/hyperlane plateau %, sample alphas/thickness

High-horizon ruler overlay hidden unless `VisualHorizon` debug metric selected.

## Visual smoke

Agent cannot capture Studio locally. Owner should verify on the 2,400-star elliptical galaxy:

1. Star Falloff Distance 100% → all stars at base visibility across map
2. 50% → nearest half of map-radius range fully visible, then fade
3. 10% → only local region fully visible
4. Star 50% + Nameplate Relative Falloff 50% → nameplates fully visible for nearest 25% of map radius
5. Nameplate Relative Falloff Transparency 0% → labels fade toward invisible beyond effective plateau
6. Hyperlane Falloff Distance 50% → base thickness/opacity for nearest half, then fade
7. Camera move/rotate/zoom → falloff stable vs map-plane range, not screen-horizon drift
8. Nameplate Relative Size 50/100/200% uniform scale
9. Hyperlane rotation dropout remains fixed (#927)

## Focused validation only

```text
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check
cargo check -p simthing-tools --features world-text-3d
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor falloff --lib
cargo test -p simthing-mapeditor nameplate --lib
cargo test -p simthing-mapeditor hyperlane --lib
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard
git diff --check
```

All commands PASS on validation host.

## Tests deliberately not run

No full cargo test -p simthing-tools, no full cargo test -p simthing-mapeditor, no workspace test battery, and no nextest run were executed because this was a targeted Studio falloff-metric presentation fix.

## Remaining debts

- Owner visual smoke for 100% / 50% / 10% plateau cases after merge
- Optional map-radius debug overlay (origin marker + radius rings) not built; high-horizon overlay disabled for default metric

## DA recommendation

Accept as PROBATION presentation math fix after owner confirms map-radius plateau matches editor intent on the 2,400-star galaxy; promote prior visual-horizon/high-horizon metrics to ACCEPTED exploratory paths superseded by this default.

# MapGeneratorCLI — dense 2-arm spiral galaxy with galactic core, 3000px

**Classification: PROBATION visual evidence (DA sign-off pending).** Owner-requested render: a **2-arm**
spiral galaxy with **thick, dense arms** (not the stringy, evenly-spaced spiral_4 from the first run), a
**bright galactic core**, blue starlanes, same canvas as before (300×300 lattice, 3000×3000 px).

## Artifact
`docs/tests/mapgenerator_cli_spiral2_dense_3000.png` (3000×3000). `spiral_2`, **3000 stars**, ~5815 blue
starlanes, bright galactic-core glow over the inaccessible core void, one connected galaxy.

## Regeneration command
```
cargo run -p simthing-mapgenerator --bin mapgen -- \
  --shape spiral_2 --stars 3000 --lattice-edge 300 --seed 24680 \
  --shape-param arm_width=14 --shape-param arm_tightness=0.6 --shape-param jitter=2 \
  --max-hyperlane-distance 5 \
  --num-hyperlanes-min 1 --num-hyperlanes-max 8000 --num-hyperlanes 8000 \
  --no-partitions --cluster-count 4 --cluster-radius 500 \
  --hyperlanes base --hyperlane-color blue --draw-core \
  --png-size 3000 \
  --render-png docs/tests/mapgenerator_cli_spiral2_dense_3000.png
```

## How "thick and dense" was achieved (vs the stringy first spiral)
The spiral strategy already exposes `arm_width`, `arm_tightness`, and `jitter` shape params; the first
(spiral_4) run used the thin defaults (`arm_width=1`, `jitter=0`), giving stringy single-cell arms. Here:
- **`arm_width=14`** broadens each arm into a thick band (perpendicular scatter that widens outward, like a
  real grand-design spiral).
- **`arm_tightness=0.6`** gives a clean ~1.5-turn 2-arm sweep (not an over-wound coil).
- **`jitter=2`** breaks up the regularity so the band reads as a dense star field, not a line.
- **3000 stars** (vs 1500): density is the explicit ask — 1500 spread across thick arms looked wispy, so the
  star count was raised to pack the arms. Canvas dimensions are unchanged (300×300 lattice, 3000px).
- **`spiral_2`** concentrates all stars into 2 arms (vs 4), doubling per-arm density.

## Requirements verified
- **Reads as a 2-arm galaxy:** two distinct arms 180° apart, broad and dense.
- **Galactic core:** the default ~40-cell core void (inaccessible) rendered with a bright warm core glow
  (`--draw-core`).
- **Connected (no islands):** `connectivity: 1 component(s) … one interconnected galaxy` (connected-by-default;
  9 bridges, the longest crossing the inter-arm core gap to tie the two arms together).

## New capability (committed with this artifact)
- CLI **`--shape-param KEY=VALUE`** (repeatable) — sets any shape tuning param (`arm_width`, `arm_tightness`,
  `jitter`, …) into `params.shape.shape_params`. Production `main.rs`; no new strategy code (the spiral
  already read these params).

## Validation
`cargo fmt --all -- --check` clean · `cargo test -p simthing-mapgenerator` all green ·
`cargo test -p simthing-clausething --test stead_spatial_contract_guards` 11 green. Render/producer-only; no
closed lowerer / runtime / GPU / spec changes; no STEAD doctrine touched.

## DA status
**PROBATION — DA sign-off pending.** No approval pre-filed.

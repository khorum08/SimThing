# MapGeneratorCLI — 4-arm spiral, 1500 stars, algorithmic starlanes, 3000px

**Classification: PROBATION visual evidence (DA sign-off pending).** Owner-requested render: a 4-armed
spiral galaxy of 1500 stars on a 300×300 lattice, with **real algorithmic hyperlanes** (no wormholes) drawn
as blue lines, output at 3000×3000 for legibility.

## Artifact
`docs/tests/mapgenerator_cli_spiral_1500_starlanes_3000.png` (3000×3000, ~600 KB). **1814 base starlanes**
over 1500 stars (avg degree ≈ 2.4 — within Stellaris's typical 1–6 lanes/system range).

## Regeneration command
```
cargo run -p simthing-mapgenerator --bin mapgen -- \
  --shape spiral_4 --stars 1500 --lattice-edge 300 --seed 421500 \
  --num-wormhole-pairs 0 --num-gateways 0 \
  --max-hyperlane-distance 3 \
  --num-hyperlanes-min 1 --num-hyperlanes-max 5000 --num-hyperlanes 5000 \
  --hyperlanes base --hyperlane-color blue \
  --png-size 3000 \
  --render-png docs/tests/mapgenerator_cli_spiral_1500_starlanes_3000.png
```

## The starlanes are the genuine algorithm — not cosplay
The blue lines are the output of the same `generate_hyperlane_topology` used for production, run on the
**authored structural gridcell coordinates** (MAPGENCLI-TOPOLOGY-STEAD-0/-1):
- Candidate pairs = systems within `max_hyperlane_distance` (3) **Chebyshev on the authored `(col,row)`**.
- Candidates sorted shortest-first; selected greedily under a per-node **fanout cap of 4** (Stellaris-like
  local degree); duplicates / self-links rejected; deterministic for a fixed seed.
- **No wormholes, no gateways** (`--num-wormhole-pairs 0 --num-gateways 0`). No routes / predecessors /
  pathfinding; integer Chebyshev only, no sqrt.
- Each rendered segment connects two stars at their authored coords (+ sub-cell jitter), so a line on the
  image is a real adjacency between two real gridcell systems.

## Honest note on the arm-tracing look
The lanes follow the spiral arms rather than forming the dense *web* a typical Stellaris map shows. That is
**a property of the placement, not the lane algorithm**: `spiral_4` places stars on thin (≈1-cell-wide)
arms, so each star's nearest neighbours are *along* its arm. The nearest-neighbour algorithm therefore
links along the arm and the fanout cap saturates before reaching across the inter-arm gap. Widening
`--max-hyperlane-distance` (tried 8) adds only minor cross-links because along-arm neighbours stay closest.
A Stellaris-style web comes from *scattered* star fields (e.g. `elliptical`/`starburst`), where nearest
neighbours fan out in all directions. The spiral result here is the faithful algorithmic network for this
shape.

## New producer/render capabilities used (committed with this artifact)
- `--num-hyperlanes <N>` — sets the base-hyperlane target count (`num_hyperlanes_default`); previously only
  min/max were CLI-exposed, so the target stayed tiny (the old preset drew ~10 lanes).
- `--hyperlane-color {faint,blue,cyan,white}` — render-only starlane colour
  (`GalaxyPreviewOptions::hyperlane_rgba`; default `faint` preserves prior output).
- Preview line thickness and star radius now scale with `--png-size` (1.0× / 1-px at the canonical 1000px,
  so existing 1000px output is byte-identical), keeping large renders legible.

## Validation
`cargo fmt --all -- --check` clean · `cargo test -p simthing-mapgenerator` all green (20 files) · the 1000px
visual-preview tests are unaffected (scaling is a no-op at 1000px). Render-only / producer-only; no closed
lowerer, runtime, GPU, or spec changes; no STEAD doctrine touched.

## DA status
**PROBATION — DA sign-off pending.** No approval pre-filed.

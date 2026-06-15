# MapGeneratorCLI visual spiral 1500 — remediation results

**Classification:** PROBATION until DA approval

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr12_closeout_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr11_scale_envelope_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr10_gpu_compact_evidence_results.md` | CURRENT_EVIDENCE / LIVE GPU GUARDRAIL | Unchanged |
| `docs/tests/mapgenerator_cli_visual_spiral_1500.png` | PROBATION | New visual evidence; retain after DA approval as CURRENT_EVIDENCE if accepted |
| `docs/tests/mapgenerator_cli_visual_spiral_1500_results.md` | PROBATION | This report |
| `success_galaxy.png` (repo root) | Superseded | Removed; PR695 elliptical debug preview superseded by this spiral visual contract |

## Regeneration command

```text
cargo run -p simthing-mapgenerator --bin mapgen -- \
  --shape spiral_4 \
  --stars 1500 \
  --lattice-edge 300 \
  --seed 421500 \
  --hyperlanes base \
  --render-png docs/tests/mapgenerator_cli_visual_spiral_1500.png \
  --png-size 1000 \
  --jitter-stars \
  --no-grid
```

Equivalent shortcut:

```text
cargo run -p simthing-mapgenerator --bin mapgen -- --spiral-visual
```

## Generation parameters

| Field | Value |
|---|---|
| Shape | `spiral_4` |
| Star count | 1500 |
| Lattice edge | 300×300 |
| Seed | `421500` (`VISUAL_SPIRAL_1500_SEED`) |
| PNG path | `docs/tests/mapgenerator_cli_visual_spiral_1500.png` |
| PNG size | 1000×1000 |
| Background | Black (`#000000`) |
| Hyperlane source | Base bounded topology edges only (`HyperlanePreviewFilter::BaseOnly`) |
| Rendered base hyperlanes | 10 independent segments |
| Nebula rendering | Off by default (`draw_nebulas: false`) |
| Grid/core-mask debug | Off by default (`--no-grid` / `draw_core_mask: false`) |

## Star jitter algorithm (render-only)

Deterministic within-cell jitter per star:

```text
cell_pixel_width  = png_size * (1 - 2*margin) / lattice_edge
base = cell center from lattice (col, row)
jitter_axis = (hash(seed, system_id, axis) * 2 - 1) * 0.42 * cell_pixel_size
render = base + jitter
```

- Same seed + system id → same jitter.
- Jitter stays within ±42% of cell footprint.
- Hyperlane endpoints use the same rendered star coordinates.
- Scenario text, lattice coords, topology generation, and lowering are unchanged.

## Hyperlane rendering

- Each base topology pair is one dark gray segment (`[45,50,58,90]`).
- No polylines through system order, BFS/DFS, partition traversal, or cluster traversal.
- Bridge/special-route couplings classified for preview/report only; excluded from default preview.
- Producer-side classification in `coupling.rs`; not new grammar.

## Forbidden semantics scan

Diff limited to `crates/simthing-mapgenerator/**` and `docs/tests/**`. No changes to closed lowerer or runtime crates.

## Files changed

- `crates/simthing-mapgenerator/src/preview_png.rs`
- `crates/simthing-mapgenerator/src/coupling.rs`
- `crates/simthing-mapgenerator/src/visual_spiral.rs`
- `crates/simthing-mapgenerator/src/lib.rs`
- `crates/simthing-mapgenerator/src/main.rs`
- `crates/simthing-mapgenerator/tests/visual_preview.rs`
- `crates/simthing-mapgenerator/tests/preview_png.rs`
- `docs/tests/mapgenerator_cli_visual_spiral_1500.png`
- `docs/tests/mapgenerator_cli_visual_spiral_1500_results.md`

## Commands run

```text
cargo fmt --all
cargo test -p simthing-mapgenerator
cargo run -p simthing-mapgenerator --bin mapgen -- --shape spiral_4 --stars 1500 --lattice-edge 300 --seed 421500 --hyperlanes base --render-png docs/tests/mapgenerator_cli_visual_spiral_1500.png --png-size 1000 --jitter-stars --no-grid
```

## PNG retention after DA

Recommend retaining `docs/tests/mapgenerator_cli_visual_spiral_1500.png` as **CURRENT_EVIDENCE** after DA visual approval.

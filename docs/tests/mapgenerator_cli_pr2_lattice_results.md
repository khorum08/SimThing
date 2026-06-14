# MapGeneratorCLI PR2 — Deterministic Lattice Occupancy Core Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent audit; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority, after independent branch-source audit; Cursor PR2)** — deterministic RNG (SplitMix64), square integer lattice,
producer-side core mask, and one-system-per-cell occupancy with deterministic collision relocation. **No shape
generation, topology, scenario emission, lowering, runtime, GPU, simthing-sim, or FIELD-MOVIE-DATASET-0 work.**

## Track scope

0.0.8.6 MapGeneratorCLI PR2 adds the minimal placement substrate above PR1 params/registry. **0.0.8.2.5 MapGen
remains closed and is not reopened.**

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE | Unchanged (DA-approved PR1) |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| `mapgenerator_cli_pr2_lattice_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| RNG | `crates/simthing-mapgenerator/src/rng.rs` |
| Lattice + core mask | `crates/simthing-mapgenerator/src/lattice.rs` |
| Occupancy | `crates/simthing-mapgenerator/src/occupancy.rs` |
| Exports | `crates/simthing-mapgenerator/src/lib.rs` |
| Tests | `crates/simthing-mapgenerator/tests/lattice.rs`, `occupancy.rs` |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

No changes to `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`, `simthing-clausething`, or MapGen
lowering sources.

## RNG algorithm / seed stability summary

- Types: `MapGenSeed`, `MapGenRng`.
- Algorithm: **SplitMix64** (pinned; documented in `rng.rs`).
- Same seed ⇒ identical `next_u64` sequence; different seeds diverge immediately.
- No system entropy, thread RNG, or wall-clock seeding in deterministic paths.

## Square lattice summary

- Types: `LatticeCoord`, `SquareLattice`, `LatticeError`.
- Square edge (`width == height`); bounds checks; stable row-major `iter_coords`.
- Index ↔ coordinate round-trip via linear index.
- `SquareLattice::edge_from_scale` reads `lattice_size` or defaults to 200.

## Core mask summary

- Type: `CoreMask` — integer squared-distance mask from lattice center.
- `core_mask_from_scale` quantizes `core_radius`/`radius` floats to cell units (producer-side only).
- Masked cells excluded from occupancy; no declarative output or sim authority.

## Occupancy / collision relocation summary

- Type: `OccupancyGrid`, `OccupancyError`.
- `try_insert` rejects duplicates; `insert_or_relocate` / `insert_next` probe placeable cells in deterministic
  order with RNG-chosen start offset.
- One system per cell enforced via `BTreeSet` + stable insertion-order vector.
- `LatticeExhausted` when no placeable free cells remain.

## Dependency boundary

No new external dependencies. Still no `simthing-sim/gpu/driver/spec` (PR1 guard test unchanged and passing).

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
```

## Test results (2026-06-14 local validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-mapgenerator` | 37 passed (8 lattice + 8 occupancy + 18 params + 3 shape_registry) |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |

## DA sign-off status

**DA-APPROVED — 2026-06-14, executive design authority.** Independent branch-source audit (not relying on the
Cursor report): `rng.rs`/`lattice.rs`/`occupancy.rs`/`lib.rs` read in full from branch source. Confirmed:
RNG is pinned SplitMix64 with no system entropy (C3); lattice is integer/square with stable row-major
`iter_coords`; core mask is producer-side integer squared-distance quantization (Candidate-F respected — no
distance reaches the declarative output, which PR2 does not emit at all); occupancy enforces one-system-per-cell
(`BTreeSet` + ordered vec) with deterministic relocation (stable placeable order + pinned-RNG start + linear
probe), reproducible for a given seed+insertion-sequence (C2/C3). Forbidden-token scan of new src (`simthing_sim`/
`spec`/route/predecessor/pathfind/movement/wgsl/gpu/euclid/sqrt/hypot) returned NONE. Battery rerun locally on the
branch: `cargo fmt --all --check` clean; `cargo test -p simthing-mapgenerator` 37 passed; `mapgen_constitution_guards`
21 passed (closed 0.0.8.2.5 contract intact); `git diff --check` clean.

**Non-blocking DA notes (carried to later rungs — not PR2 defects):**
1. `OccupancyGrid::insert_relocated` rebuilds the full placeable list (O(cells)) per insertion → O(stars·cells)
   at scale. Fine for PR2 primitives and tiny placements; **revisit before the scale-envelope rung (PR11)** so the
   primitive does not become the 1000-star bottleneck.
2. `SquareLattice::cell_count` computes `edge*edge` (u32), which would panic on overflow for an absurd
   `lattice_size` (≥ 65536 — far outside the 200×200 envelope, and PR1 sets no upper bound). **Add an upper edge
   bound / saturating mul when a rung enforces the scale envelope**, to keep the "everything bounded" invariant honest.

## PR3 may proceed?

**Yes — DA approved PR2 (2026-06-14).** Next rung: `ShapeStrategy` trait + registry wiring with elliptical and static strategy
seams, still no emitter (`design_0_0_8_6_mapgenerator_cli_ladder.md` PR3).

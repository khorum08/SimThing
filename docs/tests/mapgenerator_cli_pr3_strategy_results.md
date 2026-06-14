# MapGeneratorCLI PR3 — Shape Strategy Registry Seam Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent branch-source audit; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority, after independent branch-source audit; Cursor PR3)** — `ShapeStrategy` trait, data-driven registry dispatch, and minimal in-memory
elliptical/static strategy seams. **No scenario emitter, topology, MapGen lowering, runtime, GPU, simthing-sim,
or FIELD-MOVIE-DATASET-0 work.**

## Track scope

0.0.8.6 MapGeneratorCLI PR3 adds the strategy seam above PR2 lattice/occupancy primitives. **0.0.8.2.5 MapGen
remains closed and is not reopened.**

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE | Unchanged (DA-approved PR1) |
| `docs/tests/mapgenerator_cli_pr2_lattice_results.md` | CURRENT_EVIDENCE | Unchanged (DA-approved PR2) |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| `mapgenerator_cli_pr3_strategy_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Strategy trait + context/output | `crates/simthing-mapgenerator/src/strategy.rs` |
| Elliptical strategy | `crates/simthing-mapgenerator/src/strategies/elliptical.rs` |
| Static / arbitrary_static strategy | `crates/simthing-mapgenerator/src/strategies/static_arbitrary.rs` |
| Strategy module root | `crates/simthing-mapgenerator/src/strategies/mod.rs` |
| Registry dispatch | `crates/simthing-mapgenerator/src/shape_registry.rs` |
| Exports + placement helper | `crates/simthing-mapgenerator/src/lib.rs` |
| Tests | `crates/simthing-mapgenerator/tests/strategy.rs`, `shape_registry.rs` |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

No changes to `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`, `simthing-clausething`, or MapGen
lowering sources.

## Strategy trait summary

- Types: `ShapeStrategy`, `ShapeStrategyContext`, `ShapePlacement`, `PlacedSystemSeed`, `ShapePlacementError`.
- Output is in-memory only: `id`, `LatticeCoord`, optional initializer bucket label.
- No links, field operators, initializers beyond bucket label, runtime overlays, or specs.

## Registry dispatch summary

- `ShapeRegistry::resolve_strategy(name)` — string-keyed lookup; not a baked Rust enum dispatch.
- Executable PR3 strategies: `elliptical`, `static`, `arbitrary_static` (shared static implementation).
- Descriptor-only entries (`ring`, `spiral_2`, `spiral_4`, …) remain advertised; execution returns
  `StrategyNotImplemented` listing executable names.
- Unknown shape ⇒ `UnknownShape` listing all registered descriptor names.
- `ShapeRegistry::place(...)` wires params + lattice + core mask + occupancy + RNG (+ optional explicit cells).

## Elliptical seam summary

- Collects lattice cells inside a producer-side ellipse envelope (float math quantized to cell inclusion).
- Fisher-Yates shuffle via `MapGenRng`; places `num_stars` systems through `OccupancyGrid::insert_or_relocate`.
- Respects core mask and one-system-per-cell; deterministic for same params+seed.

## Static / arbitrary seam summary

- Accepts explicit in-memory `LatticeCoord` slices (PR3 test seam — no file parsing).
- Validates bounds, core mask, and duplicate rejection via `OccupancyGrid::try_insert`.
- `static` and `arbitrary_static` share `StaticArbitraryStrategy`.

## Determinism summary

- Same params + seed ⇒ identical elliptical placement coordinates (tested).
- Different seeds diverge when candidate shuffle differs (tested).
- Static passthrough is order-stable; unknown/not-implemented errors are stable and descriptive.

## Dependency boundary

No new external dependencies. Still no `simthing-sim/gpu/driver/spec` (guard tests passing).

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
| `cargo test -p simthing-mapgenerator` | 53 passed (8 lattice + 8 occupancy + 18 params + 4 shape_registry + 15 strategy) |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |

## DA sign-off status

**DA-APPROVED — 2026-06-14, executive design authority.** Independent branch-source audit (not relying on the
Cursor report): `strategy.rs`, `strategies/{mod,elliptical,static_arbitrary}.rs`, and the `shape_registry.rs`/
`lib.rs` diffs read in full. Confirmed:
- **C10 seam genuine** — the shape SET is the data-driven `BTreeMap` descriptor registry; `ShapeRegistry::place`
  gates on the data-driven `contains()` first, then resolves an executable strategy, yielding `UnknownShape`
  (lists descriptors) vs `StrategyNotImplemented` (lists executables) — so descriptor-only shapes (`ring`,
  `spiral_2/4`) fail gracefully rather than crash. Adding a shape = descriptor + impl + one dispatch arm; the
  emitter/lowering contract is untouched (there is none yet).
- **Candidate-F respected** — `PlacedSystemSeed` carries integer `LatticeCoord` only; the elliptical seam does
  producer-side float ellipse math but uses a **squared** inclusion test (no `sqrt`) and emits only quantized
  cells; no Euclidean magnitude/distance reaches output (and PR3 emits nothing). Static seam uses `try_insert`
  (errors on collision rather than silently relocating explicit placements — correct for arbitrary layouts).
- **Determinism (C3)** — Fisher-Yates shuffle over stable row-major candidates via the pinned RNG.
- Forbidden-token scan of new strategy code (`simthing_sim`/`spec`/route/predecessor/pathfind/movement/wgsl/gpu/
  euclid/sqrt/hypot/distance) returned NONE in code; no new deps; `vanilla_pr1()` alias preserved so PR1 tests pass.

Battery rerun locally on the branch: `cargo fmt --all --check` clean; `cargo test -p simthing-mapgenerator`
53 passed; `mapgen_constitution_guards` 21 passed (closed 0.0.8.2.5 contract intact); `git diff --check` clean.

**New non-blocking DA notes (carried; not PR3 defects):**
1. **DRY/consistency risk** — `strategies::strategy_by_name` (match arms) and `executable_strategy_names()`
   (hardcoded `NAMES` vec) are two lists that must stay in sync. Harmless now (same small module), but **single-source
   them before PR8 fills the remaining 8 vanilla shapes**, or a descriptor/impl drift could mislist executables.
2. **Mode-gate asymmetry** — procedural validation rejects `shape="arbitrary_static"` but not `shape="static"`;
   the latter fails gracefully later at `ExplicitCellsRequired`. Harmless; consider unifying the gate.

## Whether PR4 may proceed

**Yes — DA approved PR3 (2026-06-14).** PR4 = declarative scenario emitter for tiny in-memory placements, still
no topology and no lowering.

## Carried-forward DA notes (from PR2 — not addressed in PR3)

1. `OccupancyGrid::insert_relocated` O(cells) rebuild per insertion — acceptable for PR3; revisit before scale-envelope rung.
2. `SquareLattice::cell_count` u32 edge² overflow — revisit before scale-envelope rung.

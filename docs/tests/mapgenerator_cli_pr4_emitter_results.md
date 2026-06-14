# MapGeneratorCLI PR4 — Declarative Scenario Emitter Results

> **Artifact lifecycle: PROBATION** (pending DA review — do not treat as CURRENT_EVIDENCE until DA approves).

## Verdict

**PASS pending DA review** — deterministic declarative scenario text emission from in-memory placements
(metadata, lattice, location blocks with inert integer positions and initializer references). **No links,
topology, field operators, MapGen lowering, runtime, GPU, simthing-sim, simthing-clausething dependency,
or FIELD-MOVIE-DATASET-0 work.**

## Track scope

0.0.8.6 MapGeneratorCLI PR4 adds the emitter seam above PR3 strategy placements. **0.0.8.2.5 MapGen remains
closed and is not reopened.**

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE | Unchanged (DA-approved PR1) |
| `docs/tests/mapgenerator_cli_pr2_lattice_results.md` | CURRENT_EVIDENCE | Unchanged (DA-approved PR2) |
| `docs/tests/mapgenerator_cli_pr3_strategy_results.md` | CURRENT_EVIDENCE | Unchanged (DA-approved PR3) |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| `mapgenerator_cli_pr4_emitter_results.md` | PROBATION | New (this report) |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Emitter | `crates/simthing-mapgenerator/src/emitter.rs` |
| Pipeline helper | `crates/simthing-mapgenerator/src/lib.rs` |
| Tests | `crates/simthing-mapgenerator/tests/emitter.rs` |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

No changes to `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`, `simthing-clausething`, or MapGen
lowering sources.

## Emitter contract summary

- Types: `ScenarioEmitter`, `ScenarioEmitterConfig`, `ScenarioText`, `ScenarioEmitError`.
- Inputs: `MapGeneratorParams`, `SquareLattice`, `ShapePlacement`.
- Output: byte-stable ClauseScript-shaped text only (`scenario { metadata … lattice … location … }`).
- Helper: `place_and_emit_scenario` chains PR3 placement + PR4 emission in-library (no external deps).

## Canonical output shape summary

```text
scenario = generated_<shape> {
    metadata = { generated_by, shape, seed, mode }
    lattice = { width = N height = N }
    location = system_000000 {
        initializer = "<ref>"
        position = { x = <col> y = <row> }
    }
}
```

Stable location names: `system_{id:06}`. Square lattice edge emitted for both width and height.

## Inert position summary

- Positions use integer lattice `col`/`row` as `x`/`y` only.
- No `z`, distance, radius authority, sqrt, hypot, normalize, or nearest-neighbor terms in output.

## Initializer-reference summary

- Uses `PlacedSystemSeed.bucket` when present; otherwise `example_rim_initializer` default.
- Reference strings only — no initializer definitions emitted.

## Forbidden-output scan summary

Focused tests assert emitted text contains no: `link`, hyperlane, `field_operator`, route/path/predecessor/
movement/border/frontline terms, RF/Movement-Front/PALMA/commitment/BoundaryRequest payloads, or Euclidean
authority tokens. `Cargo.toml` guard extended to reject `simthing-clausething` dependency.

## Dependency boundary

No new external dependencies. No `simthing-sim/gpu/driver/spec/clausething` crate dependency.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
```

Parse-only validation via `simthing-clausething` deferred to PR5 (no cross-crate dependency added).

## Test results (2026-06-14 local validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-mapgenerator` | 70 passed (17 emitter + 53 prior) |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |

## DA sign-off status

**Pending DA review.** Only the Design Authority writes DA sign-off. This report does not pre-file approval.

## Whether PR5 may proceed

**No — await DA review of PR4.** After DA approval, PR5 = generated tiny scenario through existing MapGen
parse/lowering surface, still no topology and no GPU unless DA explicitly scopes it.

## Carried-forward DA notes (not addressed in PR4)

1. PR2: `OccupancyGrid::insert_relocated` O(cells) rebuild per insertion — revisit before scale-envelope rung.
2. PR2: `SquareLattice::cell_count` u32 edge² overflow — revisit before scale-envelope rung.
3. PR3: `strategy_by_name` / `executable_strategy_names` should be single-sourced before PR8 fills remaining vanilla shapes.
4. PR3: procedural validation rejects `arbitrary_static` but not `static`; consider unifying the mode gate.

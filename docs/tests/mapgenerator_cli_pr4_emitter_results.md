# MapGeneratorCLI PR4 — Declarative Scenario Emitter Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent branch-source re-review of the remediation; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority, after independent branch-source re-review of the remediation; Cursor PR4)** — deterministic `static_galaxy_scenario` neutral-AST text emission
from in-memory placements. **No links, topology, field operators, MapGen lowering, front-end widening,
runtime, GPU, simthing-sim, simthing-clausething dependency, or FIELD-MOVIE-DATASET-0 work.**

## DA rejection and remediation

**DA rejected the original PR4 emitter (Opus, PR #678, not merged).** The first implementation emitted invalid
`hydrate_scenario`-style `scenario { metadata … lattice … location … }` text that neither `hydrate_scenario`
nor `mapgen_lattice` accepts.

**Remediation:** the emitter has been retargeted to the `static_galaxy_scenario` neutral-AST grammar consumed
by the closed `mapgen_lattice` reader (matching preserved baseline fixtures). **No front-end/lowerer widening
was performed.**

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
| `mapgenerator_cli_pr4_emitter_results.md` | CURRENT_EVIDENCE (DA-approved) | Updated (this report) |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Emitter (retargeted) | `crates/simthing-mapgenerator/src/emitter.rs` |
| Pipeline helper | `crates/simthing-mapgenerator/src/lib.rs` |
| Tests | `crates/simthing-mapgenerator/tests/emitter.rs` |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

No changes to `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`, `simthing-clausething`, or MapGen
lowering sources.

## Emitter contract summary

- Types: `ScenarioEmitter`, `ScenarioEmitterConfig`, `ScenarioText`, `ScenarioEmitError`.
- Inputs: `MapGeneratorParams`, `SquareLattice`, `ShapePlacement`.
- Output: byte-stable neutral-AST text — single root `<scenario_id> = { static_galaxy_scenario { … } …_initializer { … } }`.
- Helper: `place_and_emit_scenario` chains PR3 placement + PR4 emission in-library (no external deps).
- **Not emitted:** top-level `scenario =`, `metadata`, `lattice`, or `location` blocks.

## Canonical output shape summary

```text
generated_<shape> = {
    static_galaxy_scenario = {
        name = "MapGeneratorCLI <shape> seed <N>"
        random_hyperlanes = no

        system = {
            id = "0"
            name = ""
            position = { x = <col> y = <row> z = 0 }
            initializer = example_rim_initializer
        }
    }

    example_rim_initializer = {
        name = "Initializer Payload"
        planet = { count = 1 }
    }
}
```

Stable system ids: quoted decimal strings from `PlacedSystemSeed.id`. Initializer refs are barewords; sibling
`*_initializer` definition blocks are emitted for each unique initializer used.

## Inert position summary

- Positions use integer lattice `col`/`row` as `x`/`y` with `z = 0`.
- No distance, radius authority, sqrt, hypot, normalize, or nearest-neighbor terms in output.

## Initializer-reference summary

- Uses `PlacedSystemSeed.bucket` as bareword when present; otherwise `example_rim_initializer`.
- Emits one minimal synthetic sibling `*_initializer = { name planet }` block per unique bareword used.
- No initializer corpus interpretation or full library generation.

## Forbidden-output scan summary

Reworked tests assert emitted text contains no: `metadata`, `lattice`, `location`, quoted initializer refs,
`add_hyperlane`, `nebula`, `field_operator`, links, route/path/predecessor/movement/border/frontline terms,
RF/Movement-Front/PALMA/commitment/BoundaryRequest payloads, or Euclidean authority tokens. No
`simthing-clausething` dependency added.

## Dependency boundary

No new external dependencies. No `simthing-sim/gpu/driver/spec/clausething` crate dependency. No MapGen lowering
calls in PR4.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
```

Lowering proof deferred to PR5 (`parse_mapgen_neutral_document` → `mapgen_lattice` path).

## Test results (2026-06-14 remediation validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-mapgenerator` | 82 passed (29 emitter + 53 prior) |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |

## DA sign-off status

**DA-APPROVED — 2026-06-14, executive design authority (remediation re-review).** Independent branch-source
audit of commit `abe32048`: `emitter.rs` read in full and checked field-by-field against the
`mapgen_lattice.rs` reader (lines 188–246) and the preserved fixture `tiny_pentad_hub_slice_raw.clause`.
Confirmed the prior rejection is fully resolved:
- Emits a single-root `<id> = { static_galaxy_scenario = { name, random_hyperlanes=no, system{…} } <name>_initializer{…} }`.
- Each `system` block carries `id` (quoted scalar), `name=""`, `position = { x y z=0 }` (required x/y present),
  and `initializer` as a **bareword** (prior quoting defect fixed) that resolves to an emitted
  `*_initializer = { name planet{count=1} }` sibling definition — matching the reader's initializer map.
- The previously-rejected `metadata`/`lattice`/`location` blocks are **gone**; no `add_hyperlane`/`nebula`/
  `field_operator` (correct for locations-only PR4). Positions are inert integers (col→x, row→y) — Candidate-F clean.
- Duplicate-id and initializer-bareword validation guard malformed output; output is byte-stable/deterministic.
- Forbidden-token scan of `emitter.rs` returned NONE; no `simthing-*` dependency added (emitter is text-only and
  does not link the lowerer — lowering remains PR5's gate).

Battery rerun locally on the branch: `cargo fmt --all --check` clean; `cargo test -p simthing-mapgenerator`
82 passed (29 emitter + 8 lattice + 8 occupancy + 18 params + 4 registry + 15 strategy);
`mapgen_constitution_guards` 21 passed (closed 0.0.8.2.5 contract intact); `git diff --check` clean.

The grammar now structurally matches the closed reader + fixtures; PR5 must still *prove* it lowers via
`parse_mapgen_neutral_document` → `mapgen_lattice`/`mapgen_links` **without front-end changes** (its gate).

## Whether PR5 may proceed

**Yes — DA approved remediated PR4 (2026-06-14).** PR5 proves parse/lowering through the existing
`mapgen_lattice` / `mapgen_links` path without changing the closed front-end.

## Carried-forward DA notes (not addressed in PR4)

1. PR2: `OccupancyGrid::insert_relocated` O(cells) rebuild per insertion — revisit before scale-envelope rung.
2. PR2: `SquareLattice::cell_count` u32 edge² overflow — revisit before scale-envelope rung.
3. PR3: `strategy_by_name` / `executable_strategy_names` should be single-sourced before PR8 fills remaining vanilla shapes.
4. PR3: procedural validation rejects `arbitrary_static` but not `static`; consider unifying the mode gate.

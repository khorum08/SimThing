# MapGeneratorCLI PR8 — Vanilla Shape Registry + Single-Source Strategy Dispatch Results

> **Artifact lifecycle: PROBATION** (pending DA approval after independent branch-source audit).

## Verdict

**PROBATION — pending DA review.** Completes remaining vanilla shape registry descriptors and single-sources
executable strategy dispatch/names via `ShapeStrategyEntry` rows. All required procedural shapes are executable
and tested; generated procedural output parses and lowers lattice through existing closed surfaces. **Zero**
`crates/simthing-clausething/src/` changes. No topology/link/routing work, no new grammar, no route/path/
predecessor/movement/border/frontline semantics, field operators, RF, Movement-Front, PALMA, driver/GPU,
simthing-sim, new `SimThingKind`, Euclidean authority in output, or FIELD-MOVIE-DATASET-0 work.

## Track scope

0.0.8.6 MapGeneratorCLI PR8 closes the PR3 carried note (parallel descriptor/executable lists) and adds
executable placement for spiral_2/3/4/6, ring, bar, starburst, cartwheel, and spoked. **PR9** (nebula /
field_operator declarative producer) is **not** started in this PR.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr1_params_results.md` through `pr7` | CURRENT_EVIDENCE | Unchanged — preserved |
| `docs/tests/mapgenerator_cli_pr8_shape_registry_results.md` | PROBATION | New (this report) |
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Single-source registry | `crates/simthing-mapgenerator/src/shape_registry.rs`, `src/strategies/registry.rs` |
| Shared placement helpers | `crates/simthing-mapgenerator/src/strategies/common.rs` |
| Shape strategies | `src/strategies/{spiral,ring,bar,starburst,cartwheel,spoked}.rs`, `mod.rs` |
| Mode-gate validation | `crates/simthing-mapgenerator/src/params.rs` |
| Registry + shape tests | `crates/simthing-mapgenerator/tests/shape_registry.rs`, `pr8_shapes.rs` |
| Static integration test updates (mode gate) | `tests/{strategy,partition,special_routes}.rs` |
| Lowering smoke | `crates/simthing-clausething/tests/mapgenerator_cli_pr8_shape_registry_lower.rs` |
| Prior static lowering tests (mode gate) | `crates/simthing-clausething/tests/mapgenerator_cli_*_lower*.rs` (5 files) |
| Docs | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md`, `docs/design_0_0_8_1_clausething_production_track.md`, `docs/clausething/MapGeneratorCLI.md` |

## Single-source registry design summary

PR3 anti-pattern removed: no separate `executable_strategy_names()` list or `strategy_by_name` match ladder.

```text
strategies/registry.rs::vanilla_entries()
  → BTreeMap<String, ShapeStrategyEntry { descriptor, strategy: Option<&dyn ShapeStrategy> }>
  → ShapeRegistry::vanilla()
  → resolve_strategy / executable_names_sorted derive from entries
```

Adding an executable shape requires one registry row + tests — not a new match arm.

## Shape descriptor coverage table

| Shape | Executable (PR8) | Requires explicit cells | Key params |
|---|---|---|---|
| elliptical | yes | no | jitter, core_radius |
| static | yes | yes | coordinate_transform |
| arbitrary_static | yes | yes | coordinate_transform |
| spiral_2 | yes | no | num_arms, arm_tightness, arm_width, jitter |
| spiral_3 | yes | no | (same) |
| spiral_4 | yes | no | (same) |
| spiral_6 | yes | no | (same) |
| ring | yes | no | ring_radius, arm_width/band_width, jitter |
| bar | yes | no | bar_length, bar_width, jitter |
| starburst | yes | no | num_arms, jitter |
| cartwheel | yes | no | ring_radius, num_arms, jitter |
| spoked | yes | no | num_arms, core_radius, jitter |

## Shape executable coverage table

All 12 registered vanilla shapes are executable in PR8 (including prior elliptical/static/arbitrary_static).
Float sampling quantizes immediately to integer lattice cells; no Euclidean authority in emitted output.

## Mode-gate handling

**Fixed:** procedural mode now rejects both `static` and `arbitrary_static` (`ExplicitShapeInProceduralMode` /
existing `ArbitraryShapeInProceduralMode`). Explicit-cell placement uses `mode=arbitrary_static` with paths or
the in-memory explicit-cells API. Integration tests updated accordingly — no parser/import work required.

## Determinism / collision / core-mask test summary

`tests/pr8_shapes.rs`: same-seed stability, different-seed variation, one-system-per-cell, core-mask respect,
insufficient-cell fail-closed, per-shape structural tests (arm counts, annulus, bar axis, radial distribution),
forbidden-term emission scan. `tests/shape_registry.rs`: single-source registry invariants.

## Parse / lattice lowering proof

`mapgenerator_cli_pr8_shape_registry_lower.rs`: `spiral_4` and `ring` procedural scenarios emit
`static_galaxy_scenario`, parse via `parse_mapgen_neutral_document`, lower via `generate_mapgen_lattice_hierarchy`.

## Closed-source gate result

**PASS (expected)** — diff excludes forbidden closed `src/` paths.

## Forbidden semantics scan summary

No route/path/predecessor/movement/border/frontline/field_operator/RF/Movement-Front/PALMA/driver/GPU surfaces
in emitted procedural scenario text.

## Dependency boundary

- `simthing-clausething` dev-depends on `simthing-mapgenerator` for integration tests only.
- `simthing-mapgenerator` does **not** depend on forbidden crates.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_pr8_shape_registry_lower
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only master...HEAD
```

## Test results

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-mapgenerator` | PASS (168 tests) |
| `cargo test -p simthing-clausething --test mapgenerator_cli_pr8_shape_registry_lower` | PASS (3 tests) |
| `cargo test -p simthing-clausething --test mapgen_neutral_ast_parse` | PASS (8 tests) |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS (10 tests) |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | PASS (21 tests) |
| `git diff --check` | PASS |
| closed-src gate | PASS |

## Carried notes remaining

- `supports_shape` per-scale advertisement (corpus lever surface) remains descriptive in params/registry only —
  no runtime `supports_shape` dispatch table beyond registry lookup.
- PR11 arbitrary import / point-list parser not started.
- O(cells) relocation bound from PR7 ladder note still applies before galaxy-scale runs.

## DA sign-off status

**PROBATION — pending DA approval.** No executive sign-off yet.

## Whether PR9 may proceed

**No — await DA approval of PR8.** After DA approves this rung, **PR9** (nebula / field_operator declarative
producer) may proceed per ladder ordering.

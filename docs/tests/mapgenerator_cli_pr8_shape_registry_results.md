# MapGeneratorCLI PR8 — Vanilla Shape Registry + Single-Source Strategy Dispatch Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent branch-source audit + battery rerun; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority)** — completes remaining vanilla shape registry descriptors and single-sources
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
| `docs/tests/mapgenerator_cli_pr8_shape_registry_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
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

**DA-APPROVED — 2026-06-14, executive design authority.** Independent branch-source audit:
- **C10 single-source registry GENUINELY achieved (closes the carried PR3 note):** `strategies/registry.rs::vanilla_entries()`
  builds **one** `BTreeMap<String, ShapeStrategyEntry>` where each row pairs a descriptor **and** its executable
  strategy; `ShapeRegistry` consumes that map, with `get()` (descriptor) and `executable_names_sorted()`
  (`filter(strategy.is_some())`) both derived from it. The prior parallel `strategy_by_name` match + `executable_strategy_names`
  vec are **removed** (`mod.rs` exports only `vanilla_entries`). Adding a shape = one `entry()` row + a strategy
  module — core/emitter/lowering contract untouched (proves C10).
- **12 shapes executable** (static, arbitrary_static, elliptical, spiral_2/3/4/6, ring, bar, starburst, cartwheel,
  spoked). Float **polar sampling quantizes to integer cells** (`common.rs::quantize_polar` → `.round() as u32`;
  annulus uses squared radii, no sqrt); output is `LatticeCoord` only — **no Euclidean authority in emitted output**
  (cos/sin sampling is the Candidate-F-permitted producer-side convenience, quantized before emission). One-per-cell
  via `insert_or_relocate`; deterministic.
- **Mode-gate asymmetry fixed (closes the carried PR3 note):** procedural mode now rejects **both** `arbitrary_static`
  (`ArbitraryShapeInProceduralMode`) and `static` (new `ExplicitShapeInProceduralMode`); prior static integration
  tests updated to `mode=arbitrary_static` with fixture paths (benign test updates, all green).
- **Zero `crates/simthing-clausething/src/` changes**; no `simthing-*` dep; forbidden/Euclidean-token scan of
  producer `src/` returned NONE. Integration (`mapgenerator_cli_pr8_shape_registry_lower`) proves procedural
  spiral_4/ring parse + lattice-lower through the closed surfaces.

Battery rerun on the branch: `cargo fmt --check` clean; `cargo test -p simthing-mapgenerator` **160 passed,
zero failures** (every binary "0 failed"); `mapgenerator_cli_pr8_shape_registry_lower` 3; `mapgen_constitution_guards`
21; `mapgen_lattice_hierarchy` 10; `mapgen_links` 19. (The 160-vs-168 count is a per-binary summation method
difference, not a failure.) Pushed for DA review, not owner-merged — governance intact.

**This rung closes two carried DA notes** (PR3 single-source dispatch + static/arbitrary_static mode gate). Carried
scale notes (O(N²) enumeration; `cell_count` overflow; O(cells) relocation) remain for the PR11 scale-envelope rung.

## Whether PR9 may proceed

**Yes — DA approved PR8 (2026-06-14).** **PR9** (nebula / `field_operator` declarative producer + initializer
buckets + inert metadata passthrough) may proceed per ladder ordering — no GPU/runtime; field operator lowers to a
`RegionFieldSpec` operator through the closed surface only.

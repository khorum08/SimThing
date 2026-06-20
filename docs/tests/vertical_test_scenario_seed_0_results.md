# VERTICAL-TEST-SCENARIO-SEED-0 — Runtime vertical-test seed through Studio authority

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## GPU adapter evidence state

**REAL_ADAPTER_OBSERVED** — runtime vertical seed GPU upload/residency/validation integration tests executed on a real adapter in this environment (4 GPU tests passed; no adapter skips).

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added VERTICAL-TEST-SCENARIO-SEED-0 PROBATION row |
| `docs/tests/vertical_test_scenario_seed_0_results.md` | PROBATION | This report |
| `docs/tests/gpu_structural_validation_wgsl_0_results.md` | PROBATION | WGSL validation prerequisite |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This pass answers a concrete horizon question: can Studio load a real SimThing-Spec scenario resembling the prior runtime vertical test, rebuild editor projections from authority, and prove its structural graph becomes GPU-resident and GPU-validated? Yes — without resurrecting runtime execution or adding generic GPU scaffolding.

## Prior runtime vertical-test findings

| Source | Shape extracted | Deferred |
|---|---|---|
| `docs/tests/scenario_saveload_io_0_results.md`, `scenario_native_session_0_results.md` | Future vertical-test must enter through `SimThingScenarioSpec` → Studio projection → GPU surfaces | Full runtime load UI |
| `crates/simthing-mapeditor/src/scenario_projection.rs` `two_cell_scenario()` | Two gridcell Locations, canonical link, structural placements, cohort children | Was test-only, not vertical-test provenance |
| `crates/simthing-clausething/tests/ct_3b_4a_session_loop.rs` | `SimSession` RF/heatmap loop over gridcell Locations | Runtime execution — not resurrected |
| `crates/simthing-feeder/tests/integration.rs` | `WorldGpuState`, `BoundaryRequest` GPU substrate | GPU runtime state — not encoded in scenario |

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| Minimal prior vertical-test shape? | Two-cell structural grid with adjacency link and cohort payload children — the smallest Studio/GPU chain exercised by prior scenario-projection and GPU-structural passes |
| Representable as `SimThingScenarioSpec`? | World root, map container, 2 gridcells, structural frame/placements, canonical link, cohort children, provenance |
| Deferred runtime execution? | `SimSession` loop, RF/Accumulator, Movement-Front, heatmap, `WorldGpuState`, pathfinding, route/predecessor, sim loop |
| Load without Studio config/Bevy authority? | `load_scenario_authority_from_path` → `deserialize_scenario_authority` (STEAD + links + ID reserve) → `load_studio_session_from_scenario_path` → `StudioSessionSource::LoadedScenario` |
| Valid structural projection? | Yes — 2 locations, 1 link, structural coords (2,3) and (2,4) |
| Valid GPU upload packet? | Yes — `location_count = 2`, `link_count = 1` |
| GPU structural validation on real adapter? | **REAL_ADAPTER_OBSERVED** — `invalid_link_endpoint_count = 0`, `self_link_count = 0` |

## What was encoded in SimThingScenarioSpec

- `scenario_id`: `runtime_vertical_seed`
- World → map-container Location → two gridcell Locations
- Structural grid: 8×8 frame, `occupied_cells = 2`
- Placements: system 1 at (row=2, col=3), system 2 at (row=2, col=4)
- Canonical link: `1 → 2`
- Cohort child under each gridcell (payload substrate marker)
- Provenance: `source = VERTICAL-TEST-SCENARIO-SEED-0`

## What was deferred

- Live runtime vertical-test execution
- RF/Accumulator execution, Movement-Front execution
- Heatmap rendering, pathfinding, route/predecessor semantics
- Simulation loop, domain semantic WGSL, new SimThingKind
- **GPU-LINK-ACCUMULATOR-SMOKE-0** — now pulled by this seed (see `docs/tests/gpu_link_accumulator_smoke_0_results.md`)

## Fixture path / builder helper

- Fixture: `crates/simthing-mapeditor/tests/fixtures/runtime_vertical_seed.simthing-scenario.json`
- Builder: `runtime_vertical_seed_scenario_spec()` in `crates/simthing-mapeditor/src/runtime_vertical_seed.rs`
- Regenerate fixture: `cargo run -p simthing-mapeditor --example write_vertical_seed_fixture`

## Scenario authority validation

- STEAD validation passes on builder and fixture load
- Scenario link validation passes (one canonical non-self link)
- SimThing ID reservation runs on deserialize
- No render coordinate properties as authority

## Studio load/projection proof

- `load_studio_session_from_scenario_path` rebuilds hydration boundary (2 occupied cells), view model (2 stars, 1 hyperlane), structural projection
- `StudioSessionSource::LoadedScenario`; `generated_output` is `None`
- Structural coordinates used in view model; Studio config and Bevy state not authority

## GPU upload/validation proof

Loaded fixture → `StudioGpuStructuralUploadPacket` → GPU buffers → WGSL structural validation:

| Field | Expected | Observed |
|---|---|---|
| `location_count` | 2 | 2 |
| `link_count` | 1 | 1 |
| `invalid_link_endpoint_count` | 0 | 0 |
| `self_link_count` | 0 | 0 |

## Tests added

**simthing-mapeditor** integration test `tests/runtime_vertical_seed.rs` (25 tests):

- Scenario/spec: validity, serde, root tree, structural grid, links, map container, gridcell children, no render authority, builder/fixture equivalence
- Studio load/projection: scenario IO, hydration, view model, session source, structural coords, no studio config/bevy authority
- GPU: structural projection, upload packet, buffer residency, validation report, zero invalid endpoints, zero self-links
- Doc guards: production doc and evidence index mention VERTICAL-TEST-SCENARIO-SEED-0

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapeditor --test runtime_vertical_seed
cargo check -p simthing-gpu
cargo test -p simthing-gpu
cargo check -p simthing-core
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
git diff --check
```

## Windows resource-limit handling (PR #754 validation)

| Item | Detail |
|---|---|
| Failed command | `cargo test -p simthing-spec` (default parallel build/link) |
| OS/resource reason | Windows paging file too small for parallel linker jobs (linker exit code **1102**) |
| Serial/scoped rerun | `$env:CARGO_BUILD_JOBS=1; cargo test -p simthing-spec --lib` |
| Test binaries that ran on rerun | `simthing-spec` **lib** unit tests only (47 passed) |
| Test binaries that did **not** run in the failed parallel attempt | `simthing-spec` integration test binaries that failed to link under parallel jobs |
| PROBATION impact | **Unchanged** — required lib tests and all other PR #754 crate validations passed after serial rerun; evidence remains PROBATION pending DA approval, not downgraded to PARTIAL for this resource limit |

## Files changed

- `crates/simthing-mapeditor/src/runtime_vertical_seed.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/runtime_vertical_seed.rs`
- `crates/simthing-mapeditor/tests/fixtures/runtime_vertical_seed.simthing-scenario.json`
- `crates/simthing-mapeditor/examples/write_vertical_seed_fixture.rs`
- `crates/simthing-mapeditor/Cargo.toml`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/vertical_test_scenario_seed_0_results.md`

## Deleted/archived artifacts

None.

## Deferred work

Runtime vertical-test execution, RF/MF execution, heatmap rendering, pathfinding, route/predecessor semantics, live sim loop, Studio runtime GPU UI wiring.

## DA status

**PROBATION** — pending owner design-authority approval. GPU evidence: **REAL_ADAPTER_OBSERVED**.
# PALMA-PATH-3 Terran convoy / pirate fleet fixture results

Status: **PARTIAL / NUMERIC+GPU FIXTURE PASS** (2026-06-11; live tree proof moved to PATH-3R)

## Deliverable

8×8 Location field-sampling fixture proving a movable SimThing consumes numeric min-plus **D**
without a pathfinding engine, route object, or semantic GPU code:

- `crates/simthing-driver/tests/support/palma_terran_pirate_fixture.rs` — scenario helpers
- `crates/simthing-driver/tests/palma_path_3_terran_pirate_fixture.rs` — proof tests (PATH-3 + PATH-3R)

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Fixture semantics (names only)

| Name | Implementation |
|---|---|
| Destination station | `D=0` seed at `(0,0)` |
| Terran convoy | Movable at `(7,7)`; samples lowest finite neighbor **D** |
| Pirate fleet | Numeric `W` bump on anchor + N4 cells |
| Blockade | High numeric `W` on partial wall / perimeter bypass rows |
| Fuel shortage | Numeric `W` increment on distant cells (`x+y ≥ 10`) |

Min-plus band sees only **W** and **D**.

## PATH-3 proof coverage (numeric + GPU)

| Test | Claim |
|---|---|
| `terran_convoy_samples_lower_d_neighbor_without_route_object` | Convoy steps toward dest by min neighbor **D**; GPU parity |
| `pirate_blockade_and_fuel_shortage_change_sampled_neighbor_preference` | Numeric pressure raises sampled **D** |
| `clearing_blockade_and_moving_pirate_updates_field_and_sample` | **W** churn lowers convoy **D** and sampled step cost |
| `sampled_step_maps_to_generic_reparent_boundary_request` | Existing `BoundaryRequest::Reparent` targets sampled gridcell id — deterministic ids, no live tree |
| `gap_corridor_yields_lower_d_at_convoy_than_closed_gap` | Open gap lowers query **D** vs closed wall |

PATH-3 alone used deterministic ids without an admitted recursive SimThing tree. **Live tree proof is PATH-3R** — see [`palma_path_3r_simthing_tree_fixture_results.md`](palma_path_3r_simthing_tree_fixture_results.md).

No route object, predecessor table, pathfinding engine, movement policy, sqrt, simthing-sim changes, or ClauseThing runtime.

## Validation

- `cargo fmt --all -- --check` — PASS
- `cargo test -p simthing-driver --test palma_path_3_terran_pirate_fixture` — PASS
- `cargo test -p simthing-driver --test palma_path_min_plus_oracle` — PASS
- `cargo test -p simthing-gpu --test min_plus_stencil` — PASS

Not run: `cargo test --workspace`, broad driver suite, ClauseThing tests.

## Next rung

PALMA-PATH-3R — [`palma_path_3r_simthing_tree_fixture_results.md`](palma_path_3r_simthing_tree_fixture_results.md)

# PALMA-PATH-3 Terran convoy / pirate fleet fixture results

Status: **IMPLEMENTED / PASS** (2026-06-11)

## Deliverable

8×8 Location field-sampling fixture proving a movable SimThing consumes numeric min-plus **D**
without a pathfinding engine, route object, or semantic GPU code:

- `crates/simthing-driver/tests/support/palma_terran_pirate_fixture.rs` — scenario helpers
- `crates/simthing-driver/tests/palma_path_3_terran_pirate_fixture.rs` — proof tests (5 tests)

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

## Proof coverage

| Test | Claim |
|---|---|
| `terran_convoy_samples_lower_d_neighbor_without_route_object` | Convoy steps toward dest by min neighbor **D**; GPU parity |
| `pirate_blockade_and_fuel_shortage_change_sampled_neighbor_preference` | Numeric pressure raises sampled **D** |
| `clearing_blockade_and_moving_pirate_updates_field_and_sample` | **W** churn lowers convoy **D** and sampled step cost |
| `sampled_step_maps_to_generic_reparent_boundary_request` | Existing `BoundaryRequest::Reparent` targets sampled gridcell id — no new movement type |
| `gap_corridor_yields_lower_d_at_convoy_than_closed_gap` | Open gap lowers query **D** vs closed wall |

No route object, predecessor table, pathfinding engine, movement policy, sqrt, simthing-sim changes, or ClauseThing runtime.

## BoundaryRequest posture

Generic **`BoundaryRequest::Reparent`** exists in `simthing-feeder` (used by `simthing-sim::apply_structural_mutations`).
PATH-3 maps the sampled gridcell to that existing shape with deterministic fixture ids — it does **not** run a full Location/gridcell SimThing tree session or invent a movement engine.

## Validation

- `cargo fmt --all -- --check` — PASS
- `cargo test -p simthing-driver --test palma_path_3_terran_pirate_fixture` — PASS (5 tests)
- `cargo test -p simthing-driver --test palma_path_min_plus_oracle` — PASS (7 tests)
- `cargo test -p simthing-gpu --test min_plus_stencil` — PASS (1 test)

Not run: `cargo test --workspace`, broad driver suite, ClauseThing tests.

## Next rung

PALMA-PATH-4 (benchmark) — not started.

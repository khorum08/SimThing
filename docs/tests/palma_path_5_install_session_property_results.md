# PALMA-PATH-5 install/session property-column results

Status: **IMPLEMENTED / PASS** (2026-06-11)

Session RegionField min-plus band scheduling: **not wired** (ledgered below — does not block this rung’s property-column proof).

## Deliverable

Feed existing GPU `MinPlusStencilOp` from admitted Location/gridcell numeric W property columns; validate D against CPU oracle; write D back to property columns; sample D and map to generic `Reparent` on the PATH-3R admitted tree.

- `crates/simthing-driver/tests/support/palma_path_5_property_fixture.rs` — property registration, gather/writeback adapter, blocker ledger
- `crates/simthing-driver/tests/palma_path_5_install_session_property.rs` — 7 proof tests

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Fixture shape

```
World
└── Location (id raw 100)
    ├── GridCell × 64 (Custom kind, ids 1..64)
    │   └── property `palma/grid_traversal`: Named `w`, Named `d`
    └── Fleet convoy (id raw 9001) under start gridcell (7,7)
```

- Grid: **8×8** (PATH-3/3R fixture scale — not Stellaris-scale; PATH-4S owns performance).
- W seeded on each gridcell’s **`w`** sub-field via `compile_property` + `SimThing.properties`.
- Destination `(0,0)` seeds **`d = 0`** on property column before relaxation.
- Test-local adapter gathers row-major W from property columns → `pack_w_and_initial_d` → GPU stencil input (gridcells are per-slot SimThings, not a contiguous RegionField buffer).

## Proof coverage

| Test | Claim |
|---|---|
| `w_seeded_through_admitted_property_columns` | W round-trips property columns and `project_tree_to_values` shadow at registry offsets |
| `property_columns_use_named_w_and_d_roles` | Admitted layout exposes Named `w` / `d` sub-fields |
| `gpu_min_plus_from_property_gather_matches_cpu_oracle` | GPU `MinPlusStencilOp` parity vs CPU oracle on property-sourced W (`max abs err < 1e-4`) |
| `d_writeback_to_property_columns_matches_oracle` | D written to gridcell `d` columns + shadow resync matches CPU oracle |
| `movable_samples_d_from_property_columns` | Convoy samples lowest neighbor D via property reads only |
| `property_sample_maps_to_generic_reparent_on_admitted_tree` | Sample → `BoundaryRequest::Reparent` → live parent update (PATH-3R harness) |
| `path5_blocker_ledger_session_scheduling_not_wired` | Documents session/RegionField scheduling blockers |

## W source

**Admitted property columns** — `palma/grid_traversal` with Named sub-fields `w` and `d`, registered through `simthing_spec::compile_property` and attached to each gridcell SimThing. Numeric Terran/pirate/blockade composition reuses PATH-3 `build_location_w_field` values written into `w` only (fixture labels; no semantic GPU branches).

## D writeback

**Works in fixture** — `write_d_flat_to_properties` updates gridcell `d` sub-fields and resyncs shadow via `project_tree_to_values`. Not automatic SimSession tick writeback; explicit test-local scatter after GPU readback.

## Movable sampling

**Works** — reads `d` from neighbor gridcell property columns; no route object or pathfinding API.

## Reparent

**Exercised** — generic `BoundaryRequest::Reparent` through `apply_structural_mutations` on admitted tree (same as PATH-3R). No movement policy.

## CPU / GPU results

- CPU oracle: `cpu_min_plus_d_from_w` over property-gathered W, 64 iterations, cell-entry convention.
- GPU: existing `MinPlusStencilOp` ping-pong; parity **PASS** (`max_d_field_error < 1e-4`).
- Partial iterations are movement-front approximations, not exact full-map shortest paths (same as PATH-1–4S).

## Blocker ledger

| Item | Status |
|---|---|
| W/D as per-gridcell SimProperty columns | **PASS** — this rung |
| D writeback to property columns | **PASS** — test-local writeback + shadow sync |
| Movable D sampling via properties | **PASS** |
| Generic Reparent on admitted tree | **PASS** (PATH-3R harness) |
| RegionField spec operator for min-plus | **Not added** — reuses standalone `MinPlusStencilOp` |
| SimSession default tick schedules min-plus band | **Not wired** — fixture invokes op directly |
| Full install/session round-trip | **Not required** for this proof |
| Production movement policy | **Not landed** |

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-driver --test palma_path_min_plus_oracle
cargo test -p simthing-gpu --test min_plus_stencil
cargo build -p simthing-driver --test palma_path_5_install_session_property
target/debug/deps/palma_path_5_install_session_property-*.exe   # 7/7 PASS
```

`cargo test -p simthing-driver --test palma_path_5_install_session_property` hit a Windows elevation quirk on first spawn; direct test binary execution confirmed all tests pass.

## Boundaries preserved

No pathfinding engine, graph manager, movement engine, route object, predecessor table, semantic SEAD, ClauseThing runtime changes, simthing-sim semantic changes, semantic GPU branches, sqrt/magnitude, or production Dijkstra/A*.

**`cargo test --workspace` not run.**

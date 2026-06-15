# PALMA-PATH-3R admitted SimThing tree fixture results

Status: **IMPLEMENTED / PASS** (2026-06-11)

## Deliverable

De-cosplay PATH-3: prove Terran convoy ownership against a **live admitted recursive SimThing tree**
and apply generic `BoundaryRequest::Reparent` through existing structural harness — no movement engine.

- `crates/simthing-driver/tests/support/palma_terran_pirate_tree.rs` — admitted tree builder
- `crates/simthing-driver/tests/palma_path_3_terran_pirate_fixture.rs` — PATH-3R tests (3 tests)

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Admitted tree shape

```
World
└── Location (id raw 100)
    ├── GridCell × 64 (Custom kind, ids 1..64 by grid index)
    │   └── Fleet convoy (id raw 9001) parented to start gridcell (7,7)
    └── …
```

- Gridcells are **Location children** (`SimThingKind::Custom("GridCell")`).
- Terran convoy is an ordinary **`Fleet`** SimThing — fixture label only.
- **W** / **D** remain numeric scalar fields (flat arrays); not attached as property columns in this rung.

## Proof coverage

| Test | Claim |
|---|---|
| `admitted_location_gridcell_tree_maps_sample_to_reparent` | Gridcell ids are Location children; convoy parent is start gridcell; sampled neighbor maps to `Reparent { child: convoy, new_parent: gridcell }`; GPU parity unchanged |
| `reparent_request_updates_live_parent_if_supported` | `simthing_sim::apply_structural_mutations` applies Reparent; convoy parent changes; slot preserved |
| `fixture_ledgers_missing_reparent_application_if_not_supported` | Documents **no blocker** — harness supports Reparent on admitted trees |

## Blocker ledger

**None.** Driver tests build a minimal admitted tree and call `apply_structural_mutations` directly.
No full session/install pipeline required for this structural proof.

Not in scope (honest limits):

- W/D as per-gridcell SimProperty columns on live nodes
- Full driver session / install round-trip
- Movement policy or tick-loop commitment

## Validation

- `cargo fmt --all -- --check` — PASS
- `cargo test -p simthing-driver --test palma_path_3_terran_pirate_fixture` — PASS (8 tests)
- `cargo test -p simthing-driver --test palma_path_min_plus_oracle` — PASS (7 tests)
- `cargo test -p simthing-gpu --test min_plus_stencil` — PASS (1 test)

Not run: `cargo test --workspace`, broad driver suite, ClauseThing tests.

## Next rung

PALMA-PATH-4 (benchmark) — **IMPLEMENTED / PASS** — [`palma_path_4_benchmark_results.md`](palma_path_4_benchmark_results.md)

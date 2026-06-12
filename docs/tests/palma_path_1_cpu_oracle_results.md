# PALMA-PATH-1 CPU oracle results — min-plus Location field

Status: **IMPLEMENTED / PASS** (2026-06-11)

## Deliverable

CPU-only min-plus relaxation oracle over a 5×5 Location gridcell field:

- `crates/simthing-driver/tests/support/palma_min_plus_oracle.rs` — test-local helper
- `crates/simthing-driver/tests/palma_path_min_plus_oracle.rs` — proof tests

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Convention

Cell-entry min-plus (pinned destination seed):

```text
D_next[cell] = W[cell] + min_{neighbor ∈ N4(cell)} D_current[neighbor]
D[dest] = 0   each iteration
```

## Proof coverage

| Test | Claim |
|---|---|
| `palma_min_plus_uniform_grid_matches_manhattan_cost` | Uniform W=1 → D at convoy query = 8 |
| `palma_min_plus_pirate_blockade_corridor_raises_convoy_d` | Full-row blockade W=100 raises D vs clear |
| `palma_min_plus_clearing_blockade_lowers_d_field` | Clearing W lowers D back to baseline |
| `palma_min_plus_emits_scalar_field_only_not_route_object` | Output is `Vec<f32>` only — no route object |

Terran convoy / pirate fleet narrative in comments; **numeric W only** — no convoy movement, no GPU, no ClauseThing, no pathfinding engine.

## Validation

- `cargo fmt --all -- --check` — PASS
- `cargo test -p simthing-driver --test palma_path_min_plus_oracle` — PASS (4 tests)

Not run: `cargo test --workspace`, GPU tests, ClauseThing tests.

## Next rung

PALMA-PATH-2: bounded GPU/JIT min-plus stencil with CPU parity — not started.

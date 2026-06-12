# PALMA-PATH-1R CPU oracle results — hardened min-plus Location field

Status: **IMPLEMENTED / PASS** (2026-06-11, PALMA-PATH-1R remedial)

## Deliverable

CPU min-plus relaxation oracle (canonical logic in `simthing-gpu::min_plus_stencil`; driver tests wrap it):

- `crates/simthing-gpu/src/min_plus_stencil.rs` — CPU step/relaxation + shared helpers
- `crates/simthing-driver/tests/support/palma_min_plus_oracle.rs` — fixture builders
- `crates/simthing-driver/tests/palma_path_min_plus_oracle.rs` — proof tests (CPU + GPU parity hooks)

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Convention

Cell-entry min-plus (pinned destination seed):

```text
D_next[cell] = W[cell] + min_{neighbor ∈ N4(cell)} D_current[neighbor]
D[dest] = 0   each iteration
```

## PALMA-PATH-1R remedial

PATH-1 full-row blockade test was corrected: it proves **raised traversal cost**, not field bending (a full row spans the grid with no detour around the row itself).

Added:

- **Partial wall + gap (8×8)** — perimeter bypass rows blocked; scalar `D` prefers the gap corridor over high-`W` wall cells; closed gap raises query `D`.
- **INF/unreachable** — cut set isolates query cell; `D` stays `INF` while destination stays pinned at `0`.
- Renamed full-row test to `palma_min_plus_full_row_blockade_raises_traversal_cost`.

## Proof coverage

| Test | Claim |
|---|---|
| `palma_min_plus_uniform_grid_matches_manhattan_cost` | Uniform W=1 → D at convoy query = 8; GPU parity |
| `palma_min_plus_full_row_blockade_raises_traversal_cost` | Full-row W=100 raises D vs clear (not a bend proof) |
| `palma_min_plus_partial_wall_gap_bends_scalar_d_field` | Partial wall + gap bends D; closed gap costs more; GPU parity |
| `palma_min_plus_clearing_blockade_lowers_d_field` | Clearing W lowers D; GPU parity |
| `palma_min_plus_inf_blocked_query_stays_unreachable` | Blocked cut → query D=INF; GPU parity |
| `palma_min_plus_emits_scalar_field_only_not_route_object` | Scalar field only |
| `palma_min_plus_gpu_matches_cpu_on_uniform_grid` | GPU parity smoke |

Numeric W only in fixtures. No route object, pathfinding engine, ClauseThing, movement commitment, or sqrt.

## Validation

- `cargo fmt --all -- --check` — PASS
- `cargo test -p simthing-driver --test palma_path_min_plus_oracle` — PASS (7 tests)

Not run: `cargo test --workspace`, broad driver suite, ClauseThing tests.

## Next rung

PALMA-PATH-2: [`palma_path_2_gpu_min_plus_results.md`](palma_path_2_gpu_min_plus_results.md)

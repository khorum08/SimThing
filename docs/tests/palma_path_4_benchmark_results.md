# PALMA-PATH-4 min-plus Location field benchmark results

Status: **IMPLEMENTED / PASS — toy axis explorer** (2026-06-11)

Representative Stellaris-scale workload: **PALMA-PATH-4S** — [`palma_path_4_stellaris_scale_benchmark_results.md`](palma_path_4_stellaris_scale_benchmark_results.md)

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Environment

- **OS:** Windows 10 (10.0.26200), dev workstation
- **Profile:** `cargo test` debug+optimized (`test` profile)
- **GPU:** local WGPU adapter (same harness as PALMA-PATH-2)
- **Note:** absolute microseconds vary by CPU/GPU; cross-machine compare trends, not single numbers

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-driver --test palma_path_min_plus_oracle
cargo test -p simthing-gpu --test min_plus_stencil
cargo test -p simthing-driver --test palma_path_4_benchmark
PALMA_PATH_4_BENCH=1 cargo test -p simthing-driver --test palma_path_4_benchmark palma_path_4_benchmark_full_matrix -- --ignored --nocapture
```

Not run: `cargo test --workspace`, broad driver suite, ClauseThing tests.

## Tests

| Test | Role |
|---|---|
| `palma_path_4_dijkstra_baseline_matches_min_plus_when_relaxed_enough` | Dijkstra baseline matches CPU min-plus at 64 iterations (8×8 mixed W) |
| `palma_path_4_cpu_per_mover_wins_single_near_dest_query_on_large_grid` | Honest CPU-win case: one mover adjacent to dest on 128×128 beats full 8-iter field |
| `palma_path_4_benchmark_smoke_matrix` | CI-safe representative timings + GPU sample |
| `palma_path_4s_scenario_has_100_stars_and_150_fleets` | CI: 180×180 / 100 stars / 150 fleets shape + W compose |
| `palma_path_4s_stellaris_scale_benchmark` | Ignored Stellaris-scale matrix (`PALMA_PATH_4_BENCH=1`) |

Code:

- `crates/simthing-driver/tests/support/palma_path_4_benchmark.rs`
- `crates/simthing-driver/tests/palma_path_4_benchmark.rs`

## Baseline definitions

| Axis | Definition |
|---|---|
| **CPU per-mover baseline** | Test-local 4-neighbor Dijkstra with cell-entry `W` costs; one independent query per mover; early exit at destination. **Not** production API. |
| **CPU field oracle** | `simthing_gpu::cpu_min_plus_d_from_w` — fixed min-plus iterations (1/2/4/8), dest `D=0` each iter. |
| **CPU field sampling** | PATH-3 lowest-neighbor-`D` argmin per mover (microseconds per mover). |
| **GPU field cold** | `MinPlusStencilOp::new` + upload + `run_ping_pong` + readback (first tick). |
| **GPU field warm** | Reused op: upload + `run_ping_pong` + readback (subsequent tick). |
| **GPU setup** | Pipeline/buffer creation only (`MinPlusStencilOp::new`). |
| **W churn** | Deterministic pseudo-random bump on `churn_pct`% of cells before each timed tick. |

Numeric **W** only — uniform, blockade/gap corridor, pirate island bump, fuel gradient, mixed. No semantic GPU branches.

## Exactness / tolerance

- Dijkstra vs min-plus agreement checked at **64 iterations** on 8×8 (`±0.05` f32).
- Benchmark iterations **1–8** are **not** guaranteed exact shortest-path closure on large grids; they measure movement-front **field refresh** cost, not full tropical closure.
- GPU/CPU field parity remains PALMA-PATH-2 (`±1e-4` on D); not re-benchmarked exhaustively here.
- No `sqrt`/magnitude added.

## Timing table (representative rows, full matrix ~120 rows)

Units: **microseconds (µs)**. `cpu_mover` = per-query average; `cpu_field` = one field update; `cpu_sample` = per-mover sampling; break-even ≈ `cpu_field / cpu_mover` (movers below this favor per-query Dijkstra if field is not already maintained).

| grid | movers | churn | iters | scenario | cpu_mover | cpu_field | cpu_sample | gpu_setup | gpu_cold | gpu_warm | break_even |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---|
| 32 | 10 | 0% | 4 | uniform | 57.7 | 16.9 | 0.007 | 409 | 809 | 677 | <1 |
| 32 | 1000 | 0% | 8 | uniform | 59.2 | 35.2 | 0.005 | 606 | 1212 | 1067 | <1 |
| 32 | 10000 | 0% | 8 | uniform | 85.0 | 35.2 | 0.005 | — | — | — | <1 |
| 64 | 100 | 0% | 8 | uniform | 325 | 131 | 0.006 | 593 | 3704 | 1709 | <1 |
| 64 | 1000 | 0% | 8 | mixed | 454 | 733 | 0.046 | 1546 | 9031 | 2339 | 2 |
| 128 | 100 | 0% | 4 | uniform | 1887 | 1190 | 0.036 | 1507 | 3786 | 3357 | <1 |
| 128 | 10 | 0% | 4 | uniform | 979 | 334 | 0.020 | 1048 | 5467 | 2802 | <1 |
| 256 | 100 | 0% | 8 | uniform | 5346 | 2328 | 0.012 | — | — | — | <1 |

Full matrix: run ignored test locally (see Commands).

## Break-even / stowaway discussion

**When the maintained field wins (measured):**

- **Many movers, same tick:** At 100–10,000 movers on 32×64 grids, total `N × cpu_mover` (tens of ms) dominates one `cpu_field` (tens–hundreds of µs) even with negligible sampling. Break-even mover count is **often &lt;10** on tested configs (sometimes &lt;1 before rounding).
- **High W churn:** Field cost scales with grid×iterations, not mover count; per-mover Dijkstra must repeat search every tick. At 5–20% churn with 1000 movers, field amortization strengthens.
- **GPU warm path (64×64+):** After setup, warm GPU field updates can beat CPU field on sustained ticks (e.g. 64×64/100 movers/8 iter: CPU field ~131 µs vs GPU warm ~1.7 ms in one sample — GPU wins when CPU field exceeds ~2 ms or when piggybacking on an existing Location stencil pass; **cold** GPU path includes multi-ms setup/readback and loses on 32×32).

**When CPU per-mover wins (measured / constructed):**

- **Single (or few) near-destination queries on large grids:** `palma_path_4_cpu_per_mover_wins_single_near_dest_query_on_large_grid` — one mover at (1,0) on 128×128 with 8 field iterations: Dijkstra early-exit ≪ full-grid relaxation.
- **Small grids + cold GPU:** 32×32 GPU cold path (~0.8–8 ms) ≫ CPU field (~7–35 µs) when no amortized stencil session exists.
- **Low mover count + low iteration field not yet paid for:** If the Location does **not** already compute a heatmap band, paying full-grid relaxation for one query is wasteful vs spot Dijkstra.

**Not claimed:**

- Pathfinding is not “free” — only **stowaway** on an existing field pass is cheap-ish.
- Not production movement policy, session/install, or live gridcell SimProperty columns (PATH-3R caveat preserved).
- Not universal GPU victory; setup/transfer overhead is material on small grids and first tick.

## Fixed-iteration “good enough”

For movement-front **decisions** (argmin neighbor D), 4–8 iterations often suffice on 32–64 grids with uniform/low obstruction W; mixed blockade scenarios on 128+ may need more iterations for Dijkstra parity (oracle uses 64 iters in proof tests). Treat 1–2 iterations as deliberately approximate.

## Constitutional boundaries preserved

No pathfinding engine, graph manager, route object, predecessor table, movement policy, semantic SEAD, ClauseThing runtime, or simthing-sim semantic changes. Dijkstra is test-local only.

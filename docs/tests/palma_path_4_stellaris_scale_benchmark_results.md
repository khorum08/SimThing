# PALMA-PATH-4S Stellaris-scale fleet movement field benchmark results

Status: **IMPLEMENTED / METRICS REMEDIAL PASS** (2026-06-11; PALMA-PATH-4S-R)

Supersedes toy-shaped PALMA-PATH-4 samples as the **representative workload** for the stowaway-heatmap thesis. PATH-4 toy matrix remains in [`palma_path_4_benchmark_results.md`](palma_path_4_benchmark_results.md) for axis exploration.

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

**PALMA-PATH-4S-R:** removed mixed CPU-dest + GPU-warm “path eval” total and `max()` tick obscuring. Each strategy is reported side-by-side with explicit pressure-included and pressure-already-paid incremental costs.

## Environment

- **OS:** Windows 10 (10.0.26200), dev workstation
- **Profile:** `cargo test` (`test` profile)
- **GPU:** local WGPU adapter (PALMA-PATH-2 harness)
- **Seed:** `BENCH_SEED = 0x50414C4D41530004` (deterministic)

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-driver --test palma_path_min_plus_oracle
cargo test -p simthing-gpu --test min_plus_stencil
cargo test -p simthing-driver --test palma_path_4_benchmark
PALMA_PATH_4_BENCH=1 cargo test -p simthing-driver --test palma_path_4_benchmark palma_path_4s_stellaris_scale_benchmark -- --ignored --nocapture
```

Not run: `cargo test --workspace`, broad driver suite, ClauseThing tests.

## Scenario (measured, not simulated)

| Parameter | Value |
|---|---|
| Grid | **180×180** (32,400 cells) |
| Factions | **2** |
| Fleets per faction | **75** |
| Total fleets | **150** |
| Stars | **100** spaced cells |
| Distinct fleet destinations (default mix) | **75** |

Star placement: stratified **10×10 regions** + deterministic LCG jitter (no duplicates). Fleet placement: near faction-owned stars. Destination mix: 50% hostile / 30% frontier / 20% random star (benchmark labels only).

W composition: test-local pressure spread + numeric compose (hostile/friendly disks, blockade band, fuel gradient, star traffic, churn jitter). Min-plus/GPU see **flat `W`/`D` only**.

## Metric definitions (formulas)

All times in **microseconds (µs)** per simulated tick unless noted.

| Component | Formula |
|---|---|
| `pressure_reduction_us` | Timed `reduce_pressure_and_compose_w` |
| `cpu_per_fleet_total_us` | Σ 150× Dijkstra queries |
| `cpu_per_dest_fields_us` | Σ min-plus fields for **distinct** fleet destination stars |
| `cpu_sample_total_us` | 150× neighbor-`D` argmin on per-dest fields |
| `cpu_faction_fields_us` | **2** min-plus fields (faction rally stars) |
| `cpu_faction_sample_us` | 150× argmin on faction rally fields |
| `cpu_unique_dest_fields_us` | **150** min-plus fields (stress case) |
| `cpu_unique_sample_us` | 150× argmin on unique-dest fields |
| `gpu_single_field_*` | **One** primary rally-star `MinPlusStencilOp` — **not** 75 destination fields |
| `total_pressure_plus_*` | `pressure_reduction_us + strategy_path_cost` |
| `incremental_*_if_pressure_paid` | Strategy path cost only (pressure assumed sunk) |
| `total_pressure_plus_gpu_single_field_warm_us` | `pressure + gpu_warm_dispatch + gpu_readback` for **one** field |

**Removed (4S-R):** `path_eval_pressure_already_paid_us` mixing CPU dest fields + GPU warm; `max()` “total tick” hiding side-by-side strategies.

## Exactness

Iterations **1/2/4/8** are **movement-front approximations**, not guaranteed full-map shortest-path closure. f32 only; no `sqrt`/magnitude.

## Timing table — side-by-side strategy totals (µs, measured 2026-06-11)

`distinct_dests=75` for default fleet mix. GPU columns only when `churn=0%`, `iters∈{4,8}` (one rally field).

| churn | iters | pressure | +pressure+dijkstra | +pressure+cpu_dest | +pressure+cpu_faction | +pressure+cpu_unique | incr. dijkstra | incr. cpu_dest | incr. cpu_faction | incr. cpu_unique |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0% | 1 | 303 | 200777 | 18748 | 710 | 37906 | 200473 | 18445 | 407 | 37603 |
| 0% | 4 | 340 | 204666 | 43268 | 1473 | 86286 | 204326 | 42928 | 1134 | 85947 |
| 0% | 8 | 324 | 203474 | 81202 | 2437 | 160216 | 203150 | 80878 | 2113 | 159893 |
| 5% | 8 | 255 | 259648 | 137138 | 2357 | 237079 | 259393 | 136883 | 2102 | 236824 |
| 20% | 8 | 288 | 283556 | 113643 | 2342 | 216562 | 283268 | 113355 | 2054 | 216273 |

### GPU single-field (one rally star — not 75 fields)

| churn | iters | setup | cold dispatch | warm dispatch | readback | +pressure+gpu1_warm | incr. gpu1 (warm+readback) |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0% | 4 | 1903 | 12213 | 112 | 951 | 1403 | 1063 |
| 0% | 8 | 752 | 888 | 351 | 1166 | 1840 | 1517 |

Full 16-row matrix: run ignored test locally.

## Honest conclusions

### Measured, not simulated

All rows come from `palma_path_4s_stellaris_scale_benchmark` on the actual 180×180 `W` array, 100 stars, 150 fleet positions/destinations, Dijkstra baseline, CPU min-plus fields, and GPU `MinPlusStencilOp` where noted.

### Pressure/W composition is explicitly timed

**~0.25–0.35 ms/tick** here — small vs path evaluation but **never hidden**. Min-plus `D` may piggyback on the same numeric `W` buffer the movement/SEAD heatmap pass already maintains — **stowaway**, not free pathfinding.

### GPU timing is one field only

Warm dispatch **~0.1–0.4 ms** for **one** rally-star field. **Do not** compare this as a replacement for **75** CPU per-destination fields without a multi-field GPU batch (not implemented). Setup, cold path, and readback are reported separately.

### CPU per-fleet Dijkstra wins when…

- **Few movers** or **spot queries** near destination (PATH-4 counterexample).
- **Unique-destination stress (B3):** at churn 0%, 8 iter, incremental cpu_unique (**~160 ms**) approaches incremental Dijkstra (**~203 ms**).

### CPU faction rally fields (B2) — strongest amortization

At churn **0%**, **8** iter: incremental **~2.1 ms** vs Dijkstra **~203 ms** when fleets share strategic fronts.

### CPU per-destination fields (B1) — wins when distinct dest count is low enough

At churn **0%**, **8** iter: incremental **~81 ms** vs Dijkstra **~203 ms** with **75** distinct destinations.

### Partial iterations

1/2/4/8-iteration fields are movement-front decisions, not exact shortest-path closure on 180×180.

### Not production integration

PATH-4S is a **representative numeric W/D benchmark**. It is **not** install/session property-column integration. **PATH-5** (admitted Location/gridcell property columns) remains next. No production movement policy has landed.

## Constitutional boundaries

No pathfinding engine, graph manager, route object, predecessor table, movement policy, semantic SEAD runtime, ClauseThing runtime, simthing-sim semantic changes, or semantic GPU branches.

## Code

- `crates/simthing-driver/tests/support/palma_path_4_stellaris_scale.rs`
- `crates/simthing-driver/tests/palma_path_4_benchmark.rs` (`palma_path_4s_*` tests)

## Tests

| Test | Role |
|---|---|
| `palma_path_4s_scenario_has_100_stars_and_150_fleets` | CI shape + W compose sanity |
| `palma_path_4s_stellaris_scale_benchmark` | Ignored full matrix (`PALMA_PATH_4_BENCH=1`) |

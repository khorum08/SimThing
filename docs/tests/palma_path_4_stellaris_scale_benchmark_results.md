# PALMA-PATH-4S Stellaris-scale fleet movement field benchmark results

Status: **IMPLEMENTED / PASS** (2026-06-11)

Supersedes toy-shaped PALMA-PATH-4 samples as the **representative workload** for the stowaway-heatmap thesis. PATH-4 toy matrix remains in [`palma_path_4_benchmark_results.md`](palma_path_4_benchmark_results.md) for axis exploration.

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

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
| Distinct fleet destinations (default mix) | **75** (hostile/frontier/random star assignment) |

### Star placement

Stratified **10×10 regions** on the 180×180 grid — one star per region at region center + deterministic LCG jitter. No duplicates (asserted).

### Fleet placement

Each faction’s 75 fleets spawn near owned stars (home star + ±3 cell jitter).

### Destination selection (benchmark labels only)

Per fleet deterministic mix: **50%** hostile faction star, **30%** owned/frontier star, **20%** random star.

### W composition (numeric only)

Test-local **pressure/SEAD-reduction stand-in** then numeric compose:

1. Hostile fleet pressure disks (radius 3) at each fleet position
2. Friendly congestion disks on faction cluster samples
3. Two-pass neighbor spread/decay (movement-front reduction stand-in)
4. Base `W = 1 + pressure` (clamped)
5. Vertical blockade band at map midline (numeric high-`W` corridor)
6. Fuel/supply gradient on distant cells (`x+y` threshold)
7. Star traffic bump at star cells
8. Optional churn jitter on `churn_pct`% of cells

Min-plus / GPU code sees **flat `W`/`D` only** — no semantic branches.

## Baseline definitions

| Axis | Definition |
|---|---|
| **Pressure/SEAD reduction** | Timed `reduce_pressure_and_compose_w` (spread + compose). **Not free** — reported separately. |
| **CPU per-fleet baseline (A)** | 150× test-local Dijkstra (cell-entry `W`, early exit at dest star). |
| **CPU per-destination fields (B1)** | One min-plus field per **distinct** destination star among fleets; then sample all 150 fleets. |
| **CPU faction objective fields (B2)** | **2** min-plus fields (faction rally stars); sample all 150 fleets on faction field. |
| **CPU unique-destination worst case (B3)** | Force **150** unique destinations; one field each — amortization lower bound stress. |
| **GPU field (C)** | `MinPlusStencilOp` on **one** primary rally-star field: setup, cold dispatch (upload+dispatch), warm dispatch, **readback reported separately**. |
| **Sampling** | PATH-3 lowest-neighbor-`D` argmin per fleet. |

## Exactness

- Iterations **1/2/4/8** are **movement-front approximations**, not guaranteed shortest-path closure on 180×180.
- Do **not** claim exact routes unless comparing to Dijkstra or full relaxation on limited cases.
- f32 arithmetic; no `sqrt`/magnitude added.

## Timing table (µs — measured 2026-06-11)

`distinct_dests=75` for default fleet mix. GPU columns only on churn **0%**, iters **4/8** (primary rally field).

| churn | iters | pressure | cpu_fleet_total | cpu_fleet_avg | cpu_dest_fields | cpu_faction_fields | cpu_unique_fields | cpu_sample | gpu_setup | gpu_cold | gpu_warm | gpu_rb | total_w_pressure | path_dest_if_pressure_paid |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0% | 1 | 295 | 197069 | 1314 | 14617 | 389 | 29520 | 13 | — | — | — | — | 197364 | 14630 |
| 0% | 4 | 320 | 200644 | 1338 | 43811 | 1510 | 85137 | 17 | 2282 | 1243 | 1215 | 1005 | 200965 | 45043 |
| 0% | 8 | 274 | 212921 | 1420 | 83408 | 2179 | 166737 | 20 | 869 | 531 | 187 | 1140 | 213195 | 83615 |
| 5% | 8 | 290 | 271341 | 1809 | 147939 | 3975 | 193337 | 26 | — | — | — | — | 271631 | 147965 |
| 20% | 8 | 356 | 299870 | 1999 | 151193 | 4868 | 227233 | 47 | — | — | — | — | 300227 | 151240 |

Full 16-row matrix (churn 0/1/5/20 × iters 1/2/4/8): run ignored test locally.

## Break-even / stowaway discussion

### Pressure reduction is already required

Pressure compose + spread costs **~0.25–0.35 ms/tick** here — small vs path evaluation, but **explicitly counted**. When 150 fleets move, this stand-in represents work the Location must do anyway for SEAD/movement-front heatmaps. Min-plus **`D`** refresh can **piggyback** on the same numeric `W` buffer — it is **not** free pathfinding, but may be **free-ish** relative to a separate per-fleet planner farm.

### CPU per-fleet Dijkstra (A)

**~200–300 ms/tick** for 150 fleets on 180×180 (`~1.3–2.0 ms/fleet`). Scales linearly with fleet count. Still competitive for **very few** movers or **one** near-destination query (see PATH-4 counterexample test).

### Shared destination fields (B1) — wins at scale here

At churn **0%**, **8** iterations: **83 ms** for 75 distinct destination fields + **0.02 ms** sampling vs **213 ms** Dijkstra — **~2.5×** faster path evaluation **after** pressure is paid (`83615 µs` vs `212921 µs`).

Break-even vs Dijkstra (ignoring pressure): `83408 / 1420 ≈ 59` fleets with shared-dest amortization at 8 iterations.

### Faction objective fields (B2) — strongest amortization

**~2.2 ms** for 2 faction rally fields + negligible sampling vs **213 ms** Dijkstra when fleets share strategic fronts. This models Stellaris-like **shared war goals** — field approach most compelling.

### Unique destinations (B3) — honest weak case

**~167 ms** for 150 unique destination fields (8 iter) — approaches Dijkstra total (**213 ms**) without GPU batching. Reports **field amortization failure** when every fleet has a unique objective.

### GPU (C)

Single-field GPU warm dispatch **~0.2–1.2 ms** (churn 0%, 8 iter) vs **~83 ms** CPU for **75** CPU destination fields. **Do not** extrapolate to universal GPU victory: benchmark measures **one** GPU field; **75** distinct GPU fields would require multi-pass or batched stencil (not implemented). Readback **~1 ms** reported separately — hidden if fields stay GPU-resident.

### Total tick framing

| Strategy | churn 0%, 8 iter (µs) |
|---|---|
| pressure + Dijkstra | **213195** |
| pressure + per-dest fields + sample | **83615** (pressure already paid) + 274 ≈ **84k** incremental path |
| pressure + faction fields + sample | **~2200** path eval + 274 pressure |

## Cases where CPU wins

- **Few fleets / spot queries** near destination on large grids (PATH-4 counterexample).
- **150 unique destinations** — per-destination field count approaches per-fleet Dijkstra cost.
- **Cold GPU + many distinct GPU fields + readback** — setup dominates if not amortized.

## Constitutional boundaries

No pathfinding engine, graph manager, route object, predecessor table, movement policy, semantic SEAD runtime, ClauseThing runtime, simthing-sim semantic changes, or semantic GPU branches. All scenario naming is test-local.

## Code

- `crates/simthing-driver/tests/support/palma_path_4_stellaris_scale.rs`
- `crates/simthing-driver/tests/palma_path_4_benchmark.rs` (`palma_path_4s_*` tests)

## Tests

| Test | Role |
|---|---|
| `palma_path_4s_scenario_has_100_stars_and_150_fleets` | CI shape + W compose sanity |
| `palma_path_4s_stellaris_scale_benchmark` | Ignored full matrix (`PALMA_PATH_4_BENCH=1`) |

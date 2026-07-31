# FIELD-SWEEP-IR-PROBE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.4)
- Status: **PROBATION** — workshop-leaf disposable probe landed; STOP for orchestration/DA (required stall/memory counters unavailable; threshold miss → `ROUTE-SPECIALIZATION/JIT`)
- HD-RECEIPT: `e52f583c42e0`
- ORIENT-RECEIPT: `ea874cae36fb`
- orientation_rule_stamp: `3107f3671f1fac8f`
- Board dispatch: comment [`5137704490`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5137704490)
- expected_route: `DA-RESERVE(admitted-scope-router-gap)`
- Pointer: remains `FIELD-SWEEP-IR-PROBE-0` (no advance; no 5.5–5.8)

## Contract discharge

| Requirement | Result |
|---|---|
| Workshop-only IR probe (map + linear fold over gather) | PASS — `crates/simthing-workshop/src/field_sweep_ir_probe.rs` (+ `.wgsl`) |
| No production export / opcode / registration / reverse dep | PASS — workshop src + tests only; `simthing-gpu` via dev-dep tests |
| N4 bit-exact vs PALMA + Gu-Yang before timing | PASS — CPU + GPU; sparse-pulse Gu-Yang fixture |
| Fallbacks `MIN × INPUT_LIST` / `PRODUCT × INPUT_LIST + banded flux` | PASS — authored programs; no field-identity enum/tag/branch |
| N8 via throwaway workshop gather only | PASS — `WorkshopThrowawayN8`; engine N8 untouched |
| EML cap/stack facts re-derived from live impl | PASS — configured `MAX_EML_TREE_NODES=32`, `EML_STACK_MAX=32`; observed nodes=13 stack≤9 |
| Matched occupancy | PASS as matched work — same theater/degree/edges/iterations/column-reads |
| Stall/memory counters | **STOP** — no public stall / SM-occupancy / memory-counter door; timestamp query available for timing only; memory-shadow **not** inferred from timing |
| Threshold | **ROUTE-SPECIALIZATION/JIT** — IR retained; bespoke kernels are not final architecture |
| Corpus / scenarios | PASS — inline synthetic only |
| Birth / lifecycle | `0.0.8.7-rf-arena-modernization`, `dsu_survivals=0`, inventory `AUDIT`/`ledger-only` (no permanent-residue / renewal claim) |

## Aggregation method

- Warmup: 8; samples: 16 host wall-clock measurements wrapping upload + dispatch + readback (same envelope for generic and bespoke).
- Median = sample median; worst = sample max.
- Ratio = generic_median / bespoke_median (and worst/worst) per case; overall = max across PALMA-shaped and Gu-Yang-shaped N4 cases.
- Threshold: median ≤1.25× and worst ≤1.5× at matched work occupancy **and** counter evidence for memory-shadow. Counters unavailable ⇒ STOP; timing alone does not confirm memory-shadow.

## Raw measurement rows (adapter-pinned)

Adapter: **NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan**  
Theater: **32×32**  
Resource class label (measurement prose only): `legacy_fixed_32_stack`

| case | path | adjacency | degree dist (abbrev) | nodes | stack | col reads/edge | med µs | worst µs | edges/s (med) | stall/memory counters | counter status |
|---|---|---|---|---|---|---|---|---|---|---|---|
| min_x_input_list_n4_bespoke | bespoke_palma | GridN4_WENS | deg2/3/4 present | 7 | ≤9 | 1 | 688.7 | 828.7 | 5.76e6 | UNAVAILABLE | STOP(...) |
| min_x_input_list_n4_generic | generic_ir | GridN4_WENS | matched | 7 | ≤9 | 1 | 2976.1 | 3715.1 | 1.33e6 | UNAVAILABLE | STOP(...) |
| product_banded_flux_n4_bespoke | bespoke_guyang | GridN4_NSEW | matched | 13 | ≤9 | 4 | 677.2 | 889.1 | 5.86e6 | UNAVAILABLE | STOP(...) |
| product_banded_flux_n4_generic | generic_ir | GridN4_NSEW | matched | 13 | ≤9 | 4 | 1327.0 | 2613.1 | 2.99e6 | UNAVAILABLE | STOP(...) |
| product_banded_flux_n8_generic_cliff | generic_ir_n8_throwaway | WorkshopThrowawayN8 | higher degree | 13 | ≤9 | 4 | 724.8 | 1036.3 | 1.08e7 | UNAVAILABLE | STOP(...) |

### Ratios (generic / bespoke)

| case | median ratio | worst ratio |
|---|---|---|
| MIN × INPUT_LIST (PALMA-shaped) | **4.321×** | **4.483×** |
| PRODUCT × INPUT_LIST + banded flux (Gu-Yang-shaped) | **1.960×** | **2.939×** |
| Overall (max) | **4.321×** | **4.483×** |

## N8 cliff

- N4 edges: 3968; throwaway N8 edges: 7812 (≈1.97× edge inflation on identical 32×32 theater).
- Located without engine adjacency/admission changes; 5.6 owns engine N8.

## EML cap facts (configured vs observed)

| fact | configured (live) | observed (probe programs) |
|---|---|---|
| tree / program nodes | `MAX_EML_TREE_NODES = 32` | max total nodes across map+fold+post = 13 |
| stack | `EML_STACK_MAX = 32` | actual max stack depth ≤ 9 |
| resource class | legacy fixed-32-stack (prose; not amended) | label only — no engine `resource_class` enum invented |

## Threshold verdict / next-route

**`ROUTE-SPECIALIZATION/JIT`**

Generic N4 exceeds median ≤1.25× / worst ≤1.5×. Required stall/memory counters cannot be measured truthfully on public doors ⇒ **STOP** (do not invent memory-shadow from timing). The IR is retained as specification; this result does **not** authorize abandonment of the IR or permanent preservation of bespoke field kernels.

Next-route for orchestration/DA: admit specialization/JIT path at 5.5+ with IR retained; do not advance pointer in this landing; resolve counter-surface STOP before any memory-shadow claim.

## Proof commands (local)

```bash
cargo check -p simthing-workshop
cargo test -p simthing-workshop --test field_sweep_ir_probe_0
```

Final head / `tested_code_sha`: PR-body-bound only (this file does not self-hash).

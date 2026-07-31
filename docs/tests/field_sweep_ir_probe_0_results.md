# FIELD-SWEEP-IR-PROBE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.4)
- Status: **PROBATION** — Remand `5137964630` discharged; workshop-leaf disposable probe (test-only); STOP for orchestration/DA
- HD-RECEIPT: `e52f583c42e0`
- Remand: Board comment [`5137964630`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5137964630)
- Board dispatch: comment [`5137704490`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5137704490)
- expected_route: `DA-RESERVE(admitted-scope-router-gap)`
- Pointer: remains `FIELD-SWEEP-IR-PROBE-0` (no advance; no 5.5–5.8)

ORIENT-RECEIPT / orientation_rule_stamp / exact head SHAs are bound in the PR body and board STOP (this file does not self-hash).

## Contract discharge

| Requirement | Result |
|---|---|
| Workshop-only IR probe (map + linear fold over gather) | PASS — `crates/simthing-workshop/tests/support/field_sweep_ir_probe.rs` (+ `.wgsl`); compiled only by the integration test |
| No production export / opcode / registration / reverse dep | PASS — **no** `pub mod` / library API; `cargo check -p simthing-workshop` does not compile the probe; `simthing-gpu` via **dev-dep** tests only |
| N4 bit-exact vs PALMA + Gu-Yang before timing | PASS — CPU + GPU; sparse-pulse Gu-Yang fixture |
| Fallbacks `MIN × INPUT_LIST` / `PRODUCT × INPUT_LIST + banded flux` | PASS — authored programs; no field-identity enum/tag/branch |
| N8 via throwaway workshop gather only | PASS — `WorkshopThrowawayN8`; engine N8 untouched |
| EML cap/stack facts re-derived from live impl | PASS — configured `MAX_EML_TREE_NODES=32`, `EML_STACK_MAX=32`; observed max total nodes=13; **actual peak operand stack**=3 (postfix lowering); configured scratch capacity=32 reported separately; runtime eval model = `scratch_indexed_dag` (not claimed as stack depth) |
| Matched occupancy | **UNMEASURED** — no public SM-occupancy door; theater/degree/edges/iterations/column-reads are matched **work**, not occupancy |
| Stall/memory counters | **STOP** — no public stall / SM-occupancy / memory-counter door; timestamp query available for timing only; memory-shadow **not** inferred from timing |
| Threshold verdict | **DIAGNOSTIC_ONLY** — occupancy UNMEASURED ⇒ **no** `ROUTE-SPECIALIZATION/JIT` (or any threshold) claim from timing |
| Corpus / scenarios | PASS — inline synthetic only |
| Birth / lifecycle | `0.0.8.7-rf-arena-modernization`, `dsu_survivals=0`, inventory `AUDIT`/`ledger-only` (no permanent-residue / renewal claim) |

## Metric definitions (Remand 1)

| Metric | Definition |
|---|---|
| map/fold/post nodes | Length of each IR tree vector |
| actual peak operand stack | Peak live stack depth under **postfix lowering** of map/fold/post trees (`peak_operand_stack_depth`); planted left-fold proves node count ≠ peak (9 nodes, peak 2) |
| configured scratch capacity | Probe evaluator scratch slots (`32`); **not** stack depth |
| matched_occupancy | Always `UNMEASURED` while counters unavailable |
| dispatch time | Host wall-clock of GPU-resident dispatch (persistent buffers; no per-iter realloc/map/readback) |
| e2e time | Upload + dispatch (same persistent session/op); readback excluded from both timed regions |
| Gu-Yang generic path | Two GPU-resident dispatches (C then flux); **no** CPU merge/readback between stages; dispatch count published (=2) |

## Aggregation method

- Warmup: 8; samples: 16.
- Median = sample median; worst = sample max.
- Diagnostic ratio = generic_median / bespoke_median (dispatch); **not** a threshold verdict.
- Threshold adjudication requires measured occupancy **and** matched envelope; with occupancy UNMEASURED ⇒ `DIAGNOSTIC_ONLY(...)`.

## Raw measurement rows (adapter-pinned)

Adapter: **NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan**  
Theater: **32×32**  
Resource class label (measurement prose only): `legacy_fixed_32_stack`  
Threshold adjudication: `DIAGNOSTIC_ONLY(occupancy_UNMEASURED;no_threshold_verdict)`

| case | path | adjacency | nodes (m/f/p) | peak stack | scratch cap | col reads/edge | disp_n | disp med µs | disp worst µs | e2e med µs | edges/s (disp med) | occupancy | stall/memory | counter status |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| min_x_input_list_n4_bespoke | bespoke_palma | GridN4_WENS | 1/3/3 | 2 | 32 | 1 | 4 | 114.7 | — | 140.9 | — | UNMEASURED | UNAVAILABLE | STOP(...) |
| min_x_input_list_n4_generic | generic_ir | GridN4_WENS | 1/3/3 | 2 | 32 | 1 | 4 | 186.0 | — | 189.0 | — | UNMEASURED | UNAVAILABLE | STOP(...) |
| product_banded_flux_n4_bespoke | bespoke_guyang | GridN4_NSEW | 9/3/1 | 3 | 32 | 4 | 1 | 21.2 | — | 23.3 | — | UNMEASURED | UNAVAILABLE | STOP(...) |
| product_banded_flux_n4_generic | generic_ir | GridN4_NSEW | 9/3/1 | 3 | 32 | 4 | 2 | 262.4 | — | 265.3 | — | UNMEASURED | UNAVAILABLE | STOP(...) |
| product_banded_flux_n8_generic_cliff | generic_ir_n8_throwaway | WorkshopThrowawayN8 | 9/3/1 | 3 | 32 | 4 | 1 | 134.1 | — | 137.0 | — | UNMEASURED | UNAVAILABLE | STOP(...) |

### Diagnostic ratios (NOT threshold verdict)

| case | dispatch median ratio (generic/bespoke) |
|---|---|
| MIN × INPUT_LIST (PALMA-shaped) | ≈1.622× |
| PRODUCT × INPUT_LIST + banded flux (Gu-Yang-shaped) | ≈12.407× (generic=2 dispatches; bespoke=1; counts published) |

## N8 cliff

- N4 edges: 3968; throwaway N8 edges: 7812 (≈1.97× edge inflation on identical 32×32 theater).
- Located without engine adjacency/admission changes; 5.6 owns engine N8.

## EML cap facts (configured vs observed)

| fact | configured (live) | observed (probe programs) |
|---|---|---|
| tree / program nodes | `MAX_EML_TREE_NODES = 32` | max total nodes across map+fold+post = 13 |
| operand stack (postfix peak) | `EML_STACK_MAX = 32` | actual peak operand stack = 3 |
| scratch capacity (evaluator) | probe scratch = 32 | runtime model `scratch_indexed_dag` |
| resource class | legacy fixed-32-stack (prose; not amended) | label only — no engine `resource_class` enum invented |

## Threshold verdict / next-route

**`DIAGNOSTIC_ONLY(occupancy_UNMEASURED;no_threshold_verdict)`**

No architectural `ROUTE-SPECIALIZATION/JIT` conclusion is emitted from these timings. Required stall/memory/occupancy counters cannot be measured on public doors ⇒ **STOP** (do not invent memory-shadow from timing). Matched persistent-buffer timing envelope is diagnostic only until occupancy is measured.

Next-route for orchestration/DA: retain IR as specification; do not advance pointer; resolve counter-surface STOP before any threshold or memory-shadow claim. No merge / graduation / 5.5–5.8 in this landing.

## Proof commands (local)

```bash
cargo check -p simthing-workshop
cargo test -p simthing-workshop --test field_sweep_ir_probe_0
```

# FIELD-SWEEP-IR-PROBE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.4)
- Status: **PROBATION** — Remand `5138380302` (evidence rows + ingress) discharged; workshop-leaf disposable probe (test-only); STOP for orchestration/DA
- HD-RECEIPT: `e52f583c42e0`
- Remand 1: Board comment [`5137964630`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5137964630) (accepted/frozen)
- Remand 2: Board comment [`5138380302`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5138380302)
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
| EML cap/stack facts re-derived from live impl | PASS — configured `MAX_EML_TREE_NODES=32` compared to observed **per-tree** max=`9` (`max(map,fold,post)`); total program nodes=`13` reported as composition only; `EML_STACK_MAX=32`; peak operand stack=`3`; scratch=`32`; runtime model `scratch_indexed_dag` |
| Matched occupancy | **UNMEASURED** — no public SM-occupancy door; theater/degree/edges/iterations/column-reads are matched **work**, not occupancy |
| Stall/memory counters | **STOP** — no public stall / SM-occupancy / memory-counter door; timestamp query available for timing only; memory-shadow **not** inferred from timing |
| Threshold verdict | **DIAGNOSTIC_ONLY** — occupancy UNMEASURED ⇒ **no** `ROUTE-SPECIALIZATION/JIT` (or any threshold) claim from timing |
| Corpus / scenarios | PASS — inline synthetic only |
| Birth / lifecycle | `0.0.8.7-rf-arena-modernization`, `dsu_survivals=0`, inventory `AUDIT`/`ledger-only` (no permanent-residue / renewal claim) |

## Metric definitions

| Metric | Definition |
|---|---|
| map/fold/post nodes | Length of each IR tree vector |
| observed max tree nodes | `max(map_nodes, fold_nodes, post_nodes)` — compared to `MAX_EML_TREE_NODES` |
| total program nodes | `map+fold+post` — descriptive composition only; **not** compared to the per-tree cap |
| actual peak operand stack | Peak live stack depth under **postfix lowering** (`peak_operand_stack_depth`); planted left-fold proves node count ≠ peak (9 nodes, peak 2) |
| configured scratch capacity | Probe evaluator scratch slots (`32`); **not** stack depth |
| matched_occupancy | Always `UNMEASURED` while counters unavailable |
| dispatch / e2e time | Persistent buffers; GPU-resident dispatch; upload included in e2e; readback excluded |

## Aggregation method

- Warmup: 8; samples: 16.
- Median = sample median; worst = sample max.
- Diagnostic ratio = generic_median / bespoke_median (dispatch); **not** a threshold verdict.
- Threshold adjudication requires measured occupancy **and** matched envelope; with occupancy UNMEASURED ⇒ `DIAGNOSTIC_ONLY(...)`.

## Raw measurement rows (adapter-pinned, complete)

Adapter-pinned rerun at Remand-2 head. Threshold adjudication: `DIAGNOSTIC_ONLY(occupancy_UNMEASURED;no_threshold_verdict)`.

Machine-readable source lines (`FSIR_ROW`) emitted by `field_sweep_ir_probe_0_adapter_pinned_measurement` from `MeasurementRow::to_tsv_line()`.

### Complete rows (transcribed)

**1. min_x_input_list_n4_bespoke**
- adapter/backend: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan
- adjacency: GridN4_WENS · theater: 32x32 · degree_distribution: deg2:4,deg3:120,deg4:900
- nodes map/fold/post: 1/3/3 · peak_operand_stack: 2 · scratch_capacity: 32 · column_reads/edge: 1
- resource_class: legacy_fixed_32_stack · occupancy: UNMEASURED
- warmup: 8 · samples: 16 · dispatch_count: 4
- dispatch_med_us: 79.050 · dispatch_worst_us: 361.800 · e2e_med_us: 81.250 · e2e_worst_us: 364.100
- edges/s (dispatch med): 5.019608e7
- stall/memory: UNAVAILABLE(no_public_stall_or_sm_occupancy_or_memory_counter_door)
- counter_status: STOP(required_stall_memory_counters_unavailable;timestamp_query_available_timing_only;occupancy_UNMEASURED;memory_shadow_not_inferred_from_timing)
- timing_note: matched_envelope: persistent op/session; timed upload+GPU-resident dispatch; no per-iter realloc/readback; occupancy UNMEASURED

**2. min_x_input_list_n4_generic**
- adapter/backend: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan
- adjacency: GridN4_WENS · theater: 32x32 · degree_distribution: deg2:4,deg3:120,deg4:900
- nodes map/fold/post: 1/3/3 · peak_operand_stack: 2 · scratch_capacity: 32 · column_reads/edge: 1
- resource_class: legacy_fixed_32_stack · occupancy: UNMEASURED
- warmup: 8 · samples: 16 · dispatch_count: 4
- dispatch_med_us: 179.500 · dispatch_worst_us: 504.400 · e2e_med_us: 183.000 · e2e_worst_us: 508.200
- edges/s (dispatch med): 2.210585e7
- stall/memory: UNAVAILABLE(no_public_stall_or_sm_occupancy_or_memory_counter_door)
- counter_status: STOP(required_stall_memory_counters_unavailable;timestamp_query_available_timing_only;occupancy_UNMEASURED;memory_shadow_not_inferred_from_timing)
- timing_note: matched_envelope: persistent op/session; timed upload+GPU-resident dispatch; no per-iter realloc/readback; occupancy UNMEASURED

**3. product_banded_flux_n4_bespoke**
- adapter/backend: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan
- adjacency: GridN4_NSEW · theater: 32x32 · degree_distribution: deg2:4,deg3:120,deg4:900
- nodes map/fold/post: 9/3/1 · peak_operand_stack: 3 · scratch_capacity: 32 · column_reads/edge: 4
- resource_class: legacy_fixed_32_stack · occupancy: UNMEASURED
- warmup: 8 · samples: 16 · dispatch_count: 1
- dispatch_med_us: 19.900 · dispatch_worst_us: 168.100 · e2e_med_us: 22.000 · e2e_worst_us: 171.400
- edges/s (dispatch med): 1.993970e8
- stall/memory: UNAVAILABLE(no_public_stall_or_sm_occupancy_or_memory_counter_door)
- counter_status: STOP(required_stall_memory_counters_unavailable;timestamp_query_available_timing_only;occupancy_UNMEASURED;memory_shadow_not_inferred_from_timing)
- timing_note: matched_envelope: persistent ops; timed upload+GPU-resident dispatch (no readback); generic=2 dispatches (C then flux, no CPU merge); bespoke=1 horizon dispatch; counts published; occupancy UNMEASURED

**4. product_banded_flux_n4_generic**
- adapter/backend: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan
- adjacency: GridN4_NSEW · theater: 32x32 · degree_distribution: deg2:4,deg3:120,deg4:900
- nodes map/fold/post: 9/3/1 · peak_operand_stack: 3 · scratch_capacity: 32 · column_reads/edge: 4
- resource_class: legacy_fixed_32_stack · occupancy: UNMEASURED
- warmup: 8 · samples: 16 · dispatch_count: 2
- dispatch_med_us: 252.050 · dispatch_worst_us: 463.800 · e2e_med_us: 255.000 · e2e_worst_us: 468.000
- edges/s (dispatch med): 1.574291e7
- stall/memory: UNAVAILABLE(no_public_stall_or_sm_occupancy_or_memory_counter_door)
- counter_status: STOP(required_stall_memory_counters_unavailable;timestamp_query_available_timing_only;occupancy_UNMEASURED;memory_shadow_not_inferred_from_timing)
- timing_note: matched_envelope: persistent ops; timed upload+GPU-resident dispatch (no readback); generic=2 dispatches (C then flux, no CPU merge); bespoke=1 horizon dispatch; counts published; occupancy UNMEASURED

**5. product_banded_flux_n8_generic_cliff**
- adapter/backend: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan
- adjacency: WorkshopThrowawayN8 · theater: 32x32 · degree_distribution: deg3:4,deg5:120,deg8:900
- nodes map/fold/post: 9/3/1 · peak_operand_stack: 3 · scratch_capacity: 32 · column_reads/edge: 4
- resource_class: legacy_fixed_32_stack · occupancy: UNMEASURED
- warmup: 8 · samples: 16 · dispatch_count: 1
- dispatch_med_us: 110.700 · dispatch_worst_us: 117.700 · e2e_med_us: 113.650 · e2e_worst_us: 120.900
- edges/s (dispatch med): 7.056911e7
- stall/memory: UNAVAILABLE(no_public_stall_or_sm_occupancy_or_memory_counter_door)
- counter_status: STOP(required_stall_memory_counters_unavailable;timestamp_query_available_timing_only;occupancy_UNMEASURED;memory_shadow_not_inferred_from_timing)
- timing_note: N8 cliff diagnostic; occupancy UNMEASURED

### Compact table

| case | path | adjacency | degree dist | m/f/p | peak | scratch | col | warm/samp | disp_n | disp med/worst µs | e2e med/worst µs | edges/s med | occ |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| min_x…_bespoke | bespoke_palma | GridN4_WENS | deg2:4,deg3:120,deg4:900 | 1/3/3 | 2 | 32 | 1 | 8/16 | 4 | 79.05 / 361.80 | 81.25 / 364.10 | 5.02e7 | UNMEASURED |
| min_x…_generic | generic_ir | GridN4_WENS | deg2:4,deg3:120,deg4:900 | 1/3/3 | 2 | 32 | 1 | 8/16 | 4 | 179.50 / 504.40 | 183.00 / 508.20 | 2.21e7 | UNMEASURED |
| product…_bespoke | bespoke_guyang | GridN4_NSEW | deg2:4,deg3:120,deg4:900 | 9/3/1 | 3 | 32 | 4 | 8/16 | 1 | 19.90 / 168.10 | 22.00 / 171.40 | 1.99e8 | UNMEASURED |
| product…_generic | generic_ir | GridN4_NSEW | deg2:4,deg3:120,deg4:900 | 9/3/1 | 3 | 32 | 4 | 8/16 | 2 | 252.05 / 463.80 | 255.00 / 468.00 | 1.57e7 | UNMEASURED |
| product…_n8_cliff | generic_ir_n8_throwaway | WorkshopThrowawayN8 | deg3:4,deg5:120,deg8:900 | 9/3/1 | 3 | 32 | 4 | 8/16 | 1 | 110.70 / 117.70 | 113.65 / 120.90 | 7.06e7 | UNMEASURED |

### Diagnostic ratios (NOT threshold verdict)

| case | dispatch median ratio (generic/bespoke) |
|---|---|
| MIN × INPUT_LIST (PALMA-shaped) | ≈2.271× |
| PRODUCT × INPUT_LIST + banded flux (Gu-Yang-shaped) | ≈12.666× (generic=2 dispatches; bespoke=1; counts published) |

## N8 cliff

- N4 edges: 3968; throwaway N8 edges: 7812 (≈1.97× edge inflation on identical 32×32 theater).
- Located without engine adjacency/admission changes; 5.6 owns engine N8.

## EML cap facts (configured vs observed)

| fact | configured (live) | observed (probe programs) |
|---|---|---|
| expression-tree nodes | `MAX_EML_TREE_NODES = 32` | **max per tree** = `max(map,fold,post)` = **9** (Gu-Yang-shaped map) |
| program composition | — | total map+fold+post = **13** (descriptive only; not compared to per-tree cap) |
| operand stack (postfix peak) | `EML_STACK_MAX = 32` | actual peak operand stack = 3 |
| scratch capacity (evaluator) | probe scratch = 32 | runtime model `scratch_indexed_dag` |
| resource class | legacy fixed-32-stack (prose; not amended) | label only — no engine `resource_class` enum invented |

## Threshold verdict / next-route

**`DIAGNOSTIC_ONLY(occupancy_UNMEASURED;no_threshold_verdict)`**

No architectural `ROUTE-SPECIALIZATION/JIT` conclusion is emitted from these timings. Required stall/memory/occupancy counters cannot be measured on public doors ⇒ **STOP** (do not invent memory-shadow from timing).

Next-route for orchestration/DA: retain IR as specification; do not advance pointer; resolve counter-surface STOP before any threshold or memory-shadow claim. No merge / graduation / 5.5–5.8 in this landing.

## Proof commands (local)

```bash
cargo check -p simthing-workshop
cargo test -p simthing-workshop --test field_sweep_ir_probe_0
```

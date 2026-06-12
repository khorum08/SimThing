# PALMA-PATH-0 design results — semiring movement-front guide

Status: **DESIGN GUIDE / READY FOR REVIEW** (2026-06-11)

## Deliverable

- [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Scope

Docs-only orientation guide for min-plus traversal fields (**D**) over Location-owned gridcell **W**
impedance, stowaway on existing heatmap/stencil bands. No Rust code, no GPU, no ClauseThing.

## Authority

Anchored in `agents.md`, `simthing_core_design.md`, `invariants.md`, `design_0_0_8_1.md` §0/§2,
Resource Flow ADR, CT-3b+4a movement-front memo. PALMA paper §2.2, §2.3, §2.6, §3.2.4, §4.3.2, §5.2,
§6.6 — algebra only; §4.2 ARM/NEON excluded.

## Key decisions

| Item | Decision |
|---|---|
| Recurrence convention | Cell-entry: `D_next[c] = W[c] + min_{n∈N4} D[n]` |
| Destination seed | `D[dest] = 0` each iteration |
| Pathfinding engine | **Rejected** — field relaxation only |
| CPU routing | Oracle/fallback for small/static cases |
| GPU/JIT | Allowed later for bounded min-plus stencil (PATH-2) |
| Sqrt | Not required for min-plus; exact paths use `m_jit_sqrt_f_exact` if added later |

## Tests

None (docs-only). **`cargo test --workspace` not run.**

PALMA-PATH-0: **ACCEPTED / GUIDE**

PALMA-PATH-1R: **IMPLEMENTED / PASS** — [`palma_path_1_cpu_oracle_results.md`](palma_path_1_cpu_oracle_results.md)

PALMA-PATH-2: **IMPLEMENTED / PASS** — [`palma_path_2_gpu_min_plus_results.md`](palma_path_2_gpu_min_plus_results.md)

PALMA-PATH-3/4: not started.

# PALMA-PATH-2 GPU min-plus stencil results

Status: **IMPLEMENTED / PASS** (2026-06-11)

## Deliverable

Bounded GPU/JIT/WGSL min-plus neighbor-relaxation stencil with CPU oracle parity:

- `crates/simthing-gpu/src/min_plus_stencil.rs` — CPU oracle + `MinPlusStencilOp` ping-pong session
- `crates/simthing-gpu/src/shaders/min_plus_stencil.wgsl` — numeric min-plus step (no semantic code)
- `crates/simthing-gpu/tests/min_plus_stencil.rs` — compile + uniform-grid parity smoke
- `crates/simthing-driver/tests/palma_path_min_plus_oracle.rs` — CPU/GPU D-field parity on uniform, detour, clear, INF fixtures

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Behavior

Same convention as PATH-0/1R:

```text
D_next[cell] = W[cell] + min_{neighbor ∈ N4(cell)} D_current[neighbor]
D[dest] = 0   each iteration
```

- Interleaved grid buffers: `d_col=0`, `w_col=1`, `n_dims=2`
- Fixed iteration count (≤ `MIN_PLUS_MAX_ITERATIONS` = 64)
- `INF` sentinel policy shared by CPU and WGSL
- Ping-pong `input`/`output` storage buffers

## Exactness classification

f32 field arithmetic; GPU/CPU parity tolerance **1e-4** max absolute error on finite cells; both `INF` → zero error. **Not** exact-authority / Candidate F — min-plus does not use sqrt or magnitude.

## Proof coverage

| Surface | Claim |
|---|---|
| GPU smoke | WGSL compiles; uniform 5×5 matches CPU |
| Driver parity hooks | Uniform, 8×8 detour open/closed, cleared blockade, INF cut |

No route object, predecessor table, pathfinding engine, movement policy, semantic branches, or sqrt.

## Validation

- `cargo fmt --all -- --check` — PASS
- `cargo test -p simthing-driver --test palma_path_min_plus_oracle` — PASS (7 tests)
- `cargo test -p simthing-gpu --test min_plus_stencil` — PASS (1 test)

Not run: `cargo test --workspace`, broad driver suite, ClauseThing tests.

## Next rung

PALMA-PATH-3 (Terran convoy field sampling fixture) — not started; requires generic movement commitment path if sampling is proven.

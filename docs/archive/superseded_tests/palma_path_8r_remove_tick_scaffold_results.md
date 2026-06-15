# PALMA-PATH-8R — Remove traversal tick scaffold — Test Results

**Status: PASS** (2026-06-11)

## Scope

Remove public `tick()` / `tick_with_input()` from the traversal band API. Force explicit GPU-resident dispatch for production; retain diagnostic/compatibility only via named dispatch methods.

## API changes

| Removed | Replacement |
|---|---|
| `tick()` | `dispatch_shadow_column_compatibility` (explicit diagnostic only) |
| `tick_with_input()` | `dispatch_gpu_resident`, `dispatch_diagnostic_readback`, `dispatch_oracle_verification_*` |
| `TraversalFieldInput` | `TraversalFieldGpuInput` (production) + `TraversalFieldShadowColumnCompatInput` (compat) |

## Tests (targeted)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-gpu --test min_plus_stencil` | PASS |
| `cargo test -p simthing-driver --test palma_path_min_plus_oracle` | PASS |
| `cargo test -p simthing-driver --test palma_path_6_session_regionfield` | PASS |
| `cargo test -p simthing-driver --test palma_path_7_gpu_traversal_utility` | PASS |
| `cargo test -p simthing-driver --test palma_path_8_gpu_native_field_graph` | PASS |
| `cargo test -p simthing-driver --test palma_path_8r_remove_tick_scaffold` | PASS (5 tests) |

## Key assertions

- No public `tick` / `tick_with_input` on `TraversalFieldBandSession`
- `dispatch_gpu_resident` requires `TraversalFieldGpuInput`; no shadow D mutation
- Shadow compatibility only via `dispatch_shadow_column_compatibility`
- Oracle only via `dispatch_oracle_verification_gpu` / `_shadow_compat`
- GPU-resident D handle still exposed after explicit dispatch

## Boundaries preserved

No pathfinding engine, route object, movement engine, semantic SEAD, ClauseThing runtime, or simthing-sim semantic changes.

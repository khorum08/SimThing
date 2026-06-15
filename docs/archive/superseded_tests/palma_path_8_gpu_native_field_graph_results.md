# PALMA-PATH-8 — GPU-native field graph connection — Test Results

**Status: PASS** (2026-06-11)

## Scope

Connect the seated generic min-plus traversal utility to GPU-native W input and GPU-resident D output handles. No pathfinding engine, movement policy, or mandatory CPU readback.

## Implementation

| Surface | Change |
|---|---|
| `MinPlusTraversalInput` | `GpuFlatW`, `GpuInterleavedW`, `PackedCpuValues` (compat) |
| `MinPlusStencilOp` | GPU W scatter via `IndexedScatterOp`; D seed on GPU; `dispatch_traversal_from_input` |
| `MinPlusTraversalGpuOutputHandle` | Resident D buffer + ping-pong side |
| `TraversalFieldInput` | Driver band input enum mirroring GPU W sources |
| `TraversalFieldBandSession::dispatch_gpu_resident` | Production GPU-native dispatch |
| `TraversalFieldBandSession::dispatch_*` (diagnostic/oracle/shadow compat) | Explicit modes only — no `tick()` |
| `TraversalFieldBandSession::resident_d_output` | Downstream consumer handle accessor |

## Tests (targeted)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-gpu --test min_plus_stencil` | PASS |
| `cargo test -p simthing-driver --test palma_path_min_plus_oracle` | PASS |
| `cargo test -p simthing-driver --test palma_path_6_session_regionfield` | PASS |
| `cargo test -p simthing-driver --test palma_path_7_gpu_traversal_utility` | PASS |
| `cargo test -p simthing-driver --test palma_path_8_gpu_native_field_graph` | PASS (5 tests) |

## Key assertions

- `gpu_w_input_dispatches_without_shadow_gather` — poisoned shadow; `GpuFlatW` dispatch succeeds without shadow gather
- `gpu_resident_d_output_exposes_field_handle` — `output_handle()` / `resident_d_output()` non-zero buffer
- `shadow_column_input_remains_compatibility_mode` — `PackedCpuValues` path unchanged
- `diagnostic_readback_preserves_path7_visibility` — GPU W + diagnostic readback oracle parity
- `oracle_verification_preserves_cpu_parity` — GPU W + explicit CPU W oracle

## Deferred

- Downstream GPU threshold/EML consumer sampling D without CPU readback (no generic consumer wired in this PR)
- Automatic RegionField / `SimSession` pass-graph scheduling of traversal after upstream W passes

## Boundaries preserved

No pathfinding engine, route object, predecessor table, movement engine, semantic SEAD, ClauseThing runtime, or simthing-sim semantic changes.

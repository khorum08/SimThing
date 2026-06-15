# PALMA-PATH-9 — Downstream GPU consumer for resident D — Test Results

**Branch:** `codex/palma-path-9`  
**Scope:** First Fable-facing GPU greenfield consumer contract — compact probe over resident traversal D without full-D readback.

## Production path proved

```text
GPU W (FlatW)
  → TraversalFieldBandSession::dispatch_gpu_resident
  → resident_d_output() / MinPlusTraversalGpuOutputHandle
  → MinPlusTraversalDProbeOp::probe_resident_d
  → compact gathered + min_d readback (assertion only)
```

## New surface

| Item | Location |
|---|---|
| `MinPlusTraversalDProbeOp` | `simthing_gpu::min_plus_traversal_d_probe` |
| `MinPlusTraversalDProbeConfig` | from stencil config (`n_dims`, `d_col`, `inf_sentinel`) |
| `MinPlusTraversalDProbeResult` | `{ gathered, min_d }` |
| `cpu_probe_d_at_candidates` | CPU oracle for test comparison |
| WGSL | `simthing-gpu/src/shaders/min_plus_traversal_d_probe.wgsl` |

## Tests

| Test | Result |
|---|---|
| `resident_d_output_feeds_gpu_probe_without_full_d_readback` | PASS |
| `shadow_columns_not_required_for_downstream_gpu_probe` | PASS |
| `diagnostic_modes_remain_explicit` | PASS |
| `no_route_or_predecessor_constructs` | PASS |

## Targeted gates (PATH-9 handoff)

```text
cargo fmt --all -- --check
cargo test -p simthing-gpu --test min_plus_stencil
cargo test -p simthing-driver --test palma_path_min_plus_oracle
cargo test -p simthing-driver --test palma_path_7_gpu_traversal_utility
cargo test -p simthing-driver --test palma_path_8_gpu_native_field_graph
cargo test -p simthing-driver --test palma_path_8r_remove_tick_scaffold
cargo test -p simthing-driver --test palma_path_9_downstream_gpu_consumer
```

`cargo test --workspace` **not run** (per handoff discipline).

## Boundaries preserved

- No full D field readback on production path
- No route object, predecessor table, pathfinding engine, or movement policy
- CPU W gather / full-D readback remain explicit diagnostic/compatibility only
- No PALMA legacy aliases or public tick scaffold restored

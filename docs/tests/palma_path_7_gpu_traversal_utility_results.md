# PALMA-PATH-7 GPU traversal utility production seating results

Status: **IMPLEMENTED / PASS** (2026-06-11)

## Deliverable

Seat min-plus traversal as a **generic GPU field utility** (PALMA remains algebraic provenance in docs only).

| Layer | Symbol | Role |
|---|---|---|
| GPU op | `MinPlusTraversalFieldOp` (`simthing_gpu`) | Ping-pong min-plus dispatch + execution modes |
| Driver band | `TraversalFieldBandSession` (`simthing_driver::min_plus_traversal_field`) | Opt-in `FieldScheduler` band over admitted W/D columns |
| Legacy aliases | `PalmaMinPlusFieldBandSession`, … | PATH-6 test compatibility only |

Tests: `crates/simthing-driver/tests/palma_path_7_gpu_traversal_utility.rs` (6 tests)

## Execution modes

| Mode | CPU readback | Shadow/property D writeback | Use |
|---|---|---|---|
| `GpuResident` (**default**) | No | No | Production / Fable greenfield GPU consumers |
| `DiagnosticReadback` | Yes | Optional (`shadow_writeback=true`) | Debug, UI, transitional CPU consumers |
| `OracleVerification` | Yes + CPU compare | Optional | Tests / parity gates only |

Formula: `D_next[c] = W[c] + min_{n∈N4} D[n]`; destination seed `D[dest]=0`.

## Proof coverage

| Test | Claim |
|---|---|
| `traversal_utility_default_off_and_named_generically` | Opt-in; utility id `min_plus_traversal_field_v1` |
| `gpu_resident_mode_dispatches_without_cpu_readback_or_shadow_mutation` | Dispatch without CPU D mutation |
| `gpu_resident_op_exposes_resident_buffer_handle` | `resident_values_buffer(iterations)` after dispatch |
| `diagnostic_readback_mode_preserves_path5_path6_writeback` | Explicit diagnostic path matches PATH-5/6 |
| `oracle_verification_mode_preserves_cpu_parity` | Oracle err < 1e-4 |
| `default_execution_mode_is_gpu_resident` | Default mode is production GPU-resident |

PATH-6 updated: production tick uses `GpuResident`; diagnostic tests select modes explicitly.

## Blocker ledger

| Item | Status |
|---|---|
| GPU-resident D output handle | **PASS** — `MinPlusPingPongSide` + `resident_values_buffer` |
| Downstream GPU threshold consumer on D | **Deferred** — no generic consumer wired in this PR |
| Default `SimSession::tick` schedules band | **NOT WIRED** (PATH-6 limit preserved) |
| Pathfinding engine / movement policy | **NOT LANDED** |

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-gpu --test min_plus_stencil
cargo test -p simthing-driver --test palma_path_min_plus_oracle
cargo test -p simthing-driver --test palma_path_6_session_regionfield
cargo test -p simthing-driver --test palma_path_7_gpu_traversal_utility
target/debug/deps/palma_path_5_install_session_property-*.exe
```

**`cargo test --workspace` not run.**

## Fable handoff

Use `MinPlusTraversalFieldOp::dispatch_traversal_from_input` with `MinPlusTraversalExecutionOptions::gpu_resident(iterations)` or `TraversalFieldBandSession::dispatch_gpu_resident` with `TraversalFieldGpuInput`. Do **not** assume CPU shadow/property D is updated each dispatch. For debug/tests, use explicit `dispatch_diagnostic_readback`, `dispatch_shadow_column_compatibility`, or `dispatch_oracle_verification_*` — **no public `tick()`**.

No pathfinding engine, route object, movement engine, semantic SEAD, ClauseThing/simthing-sim changes, or sqrt/magnitude.

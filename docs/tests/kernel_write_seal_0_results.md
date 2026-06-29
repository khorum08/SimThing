# KERNEL-WRITE-SEAL-0 Results

## Status

**PROBATION** — resolved GPU column buffers sealed on `WorldGpuState`; external crates cannot reach write-capable `Buffer` handles. DA re-review required before DONE.

## PR / branch / merge

- Branch: `codex/kernel-write-seal-0`
- PR: https://github.com/khorum08/SimThing/pull/988
- Merge: `1ad2d72c3c` (master)

## What changed

- Sealed `values`, `previous_values`, `output_vectors`, and `previous_output_vectors` inside private `ResolvedGpuBuffers` on `WorldGpuState` (`pub(crate) resolved` for in-crate partial borrows only).
- Removed public `write_*` helpers; boundary/admission CPU shadow installs are named `install_resolved_*_at_boundary`.
- Added `ResolvedWriteAuthority` ZST (private `boundary_install()`); required by the full-column values install path (compile-time token, zero runtime cost).
- Public dispatch wrappers for external observation without exposing `&Buffer`: `dispatch_accumulator_threshold_scan`, `dispatch_indexed_scatter_from_resolved_values`, `encode_accumulator_orderband_into`.
- Migrated all call sites (feeder shadow upload, driver/clausething tests, accumulator parity tests) from direct buffer writes to install/dispatch APIs.

## Seal audit

| Check | Finding |
|---|---|
| Public `Buffer` fields on `WorldGpuState` | None — resolved columns are private inside `ResolvedGpuBuffers` |
| `write_values` / `write_previous_values` / `write_output_vectors` | Removed — no matches in workspace |
| External crate `state.resolved.*` or `write_buffer(&state.*values)` | None in `simthing-driver`, `simthing-feeder`, `simthing-sim`, `simthing-clausething` |
| Live tick GPU writes | In-crate `passes.rs` + accumulator sessions only (partial borrow via `state.resolved.*`) |
| Boundary CPU shadow install | `install_resolved_values_at_boundary`, `install_resolved_value_rows_at_boundary`, `install_resolved_previous_values_at_boundary`, `install_resolved_output_vectors_at_boundary`, `install_resolved_previous_output_vectors_at_boundary` |
| `compile_fail` doc on `ResolvedWriteAuthority` | External access to `state.resolved.values` does not compile |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `cargo check -p simthing-gpu -p simthing-feeder -p simthing-sim -p simthing-driver -p simthing-clausething` | Cross-crate compile after seal + call-site migration |
| `cargo test -p simthing-gpu --lib` (203 tests; includes `world_state::tests::write_read_values_roundtrip`) | In-crate pass pipeline + boundary install roundtrip |
| `cargo test -p simthing-sim --test s6_threshold_sunset --test c1_threshold_perf` | Threshold dispatch via sealed wrappers + boundary seeding |
| `ResolvedWriteAuthority` `compile_fail` rustdoc | External resolved-buffer field reach |

## Value parity

No resolved-value or behavior change intended. Refactor routes the same CPU shadow uploads and GPU pass bindings through sealed accessors; existing parity/oracle tests in `simthing-gpu --lib` green.

## Performance parity

Zero-cost by construction: private fields + `pub(crate)` grouping + ZST authority token (no runtime branch on hot path). In-crate passes use partial field borrows (`&state.resolved.values` alongside `accumulator_runtime.as_mut()`) with no added indirection.

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-gpu/src/world_state.rs` | Seal + boundary install API + dispatch wrappers + `compile_fail` doc |
| `crates/simthing-gpu/src/passes.rs` | In-crate partial borrows of sealed buffers |
| `crates/simthing-gpu/src/accumulator_op/world_summary.rs` | In-crate buffer binding |
| `crates/simthing-gpu/src/lib.rs` | Re-export `ResolvedWriteAuthority` |
| `crates/simthing-feeder/src/dispatcher.rs` | Shadow upload via install API |
| `crates/simthing-driver/src/simulation_fabric.rs` | Scatter via dispatch wrapper |
| `crates/simthing-driver/**` (tests + burn-in) | `install_resolved_*_at_boundary` migration |
| `crates/simthing-clausething/tests/**` | Same migration |
| `crates/simthing-sim/tests/**` (parity/sunset) | Same migration |
| `docs/tests/kernel_write_seal_0_results.md` | Evidence ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | Rung 3 OPEN → PROBATION |

**Not touched:** emission/participation seals, `simthing-kernel` crate extraction, `deny.toml`, new dependencies.

## Known gaps / next

- DA re-review: PROBATION → DONE.
- **`KERNEL-EMISSION-SEAL-0`** — next seal rung (threshold-crossing mint authority).
- Some legacy `simthing-sim` integration tests (`boundary_integration`, `c1_threshold_scan_parity`) fail to compile on master due to pre-existing AS-INDEX residue — unrelated to this rung; threshold sunset/perf paths used instead.

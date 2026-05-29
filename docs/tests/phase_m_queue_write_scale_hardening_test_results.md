# Phase M Queue-Write Scale Hardening V1 — Test Results

Date: 2026-05-29

## Base

- Base HEAD: `04afcea33fc5f3de3062cc4b36cdf7ef8cec8765` (SummaryValidity V1-R1 parking)
- Final commit SHA: recorded at merge

## Files Changed

- `crates/simthing-gpu/src/shaders/values_fill.wgsl` — generic column fill compute shader
- `crates/simthing-gpu/src/accumulator_op/session.rs` — `fill_slot_range_col`, bounds/finite validation
- `crates/simthing-gpu/tests/accumulator_op_session_gpu_bridge.rs` — fill helper parity/bounds tests
- `crates/simthing-driver/src/first_slice_mapping_runtime.rs` — bulk child resource fill + readiness counters
- `crates/simthing-driver/tests/phase_m_first_slice_queue_write_hardening.rs` — 4-test hardening suite
- `crates/simthing-driver/tests/phase_m_first_slice_runtime.rs` — updated bridge counter expectations
- `crates/simthing-driver/tests/phase_m_first_slice_product_fixture.rs` — updated bridge counter expectations
- `crates/simthing-driver/tests/e11_arena_allocation.rs` — allowlist generic `values_fill.wgsl`
- `crates/simthing-driver/tests/e11b_nested_*.rs` — same allowlist update

## Chosen Implementation Strategy

- Added generic substrate helper `AccumulatorOpSession::fill_slot_range_col(start_slot, count, col, value)`.
- `count == 1`: single scalar queue write.
- `count > 1`: one generic GPU fill dispatch (`values_fill.wgsl`) — required because row-major column slots are not contiguous in the values buffer.
- First-slice bridge uses one bulk fill for child resource column (100 slots) instead of 100 per-slot queue writes.
- Parent personality/weight columns remain 2 constant-size scalar queue writes (O(1)).

## Before / After Bridge Write-Shape

| Metric | Before | After (executed tick) |
|---|---|---|
| Child resource writes | 100 per-slot queue writes | 1 bulk col fill (`gpu_bridge_bulk_col_fills=1`, `gpu_bridge_bulk_fill_values=100`) |
| Parent scalar writes | 2 queue writes | 2 queue writes (`gpu_bridge_parent_scalar_writes=2`) |
| `gpu_bridge_slot_col_writes` | 102 | 2 (parent scalars only) |
| Stencil prefix copy | unchanged | unchanged (`gpu_bridge_bytes_copied=3200` at 10×10×8 dims) |

## Commands Run

| Command | Result |
|---|---|
| `git rev-parse HEAD` | PASS; `04afcea33fc5f3de3062cc4b36cdf7ef8cec8765` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 3/3 |
| `cargo test -p simthing-driver --test phase_m_first_slice_queue_write_hardening -- --nocapture` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_first_slice_summary_validity -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture` | PASS; 9/9 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS (see full log) |

Full log: [`phase_m_queue_write_scale_hardening_full.log`](phase_m_queue_write_scale_hardening_full.log)

## Pass/Fail Table

| Test | Result |
|---|---|
| 1 — generic fill helper parity/bounds | PASS |
| 2 — first-slice bridge uses bulk child resource fill | PASS |
| 3 — result parity (commitment low/no event, high/one event) | PASS |
| 4 — SummaryValidity fresh/cached sequence unchanged | PASS |
| 5 — scenario/product/runtime regressions | PASS |
| 6 — posture preservation | PASS |

## Hot-Path GPU-Residency Summary

- `reduction_stencil_readbacks == 0` preserved on hot path
- Stencil → accumulator prefix copy remains GPU-resident
- Child resource column populated via generic fill helper (no CPU threat/urgency rederivation)
- Threshold event behavior unchanged (low profile no event; high profile one event)

## SummaryValidity Regression Summary

Fresh → Cached age 1 → Cached age 2 → dirty refresh Fresh unchanged. Cached ticks report zero bridge bulk fills and deferred commitment scan.

## Posture Summary

Phase M Queue-Write Scale Hardening V1 landed.
The first-slice GPU bridge no longer uses per-child resource queue writes for the child resource column. It uses a generic bounded bulk/preinitialized fill path while preserving the GPU-resident stencil → accumulator → reduction → EML → threshold event flow.
Parent scalar weight writes remain constant-size and acceptable for the single-grid first-slice path.
No SummaryValidity behavior changed.
No CPU-side gameplay cache was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency expansion, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

## Remaining Caveat

V1 uses a generic GPU fill dispatch for strided column fills when count > 1 rather than a single contiguous buffer write. Parent weight/personality columns remain constant-size queue writes (O(1), not O(cell_count)). Revisit only if multi-parent/multi-field batching makes parent scalar writes measurable.

## Final Verdict

PASS — Phase M Queue-Write Scale Hardening V1 landed; first-slice child resource population no longer uses per-child queue writes, prior GPU-resident first-slice/SummaryValidity/SEAD behavior remains green, and no atlas, semantic WGSL, source_mask, perception, map residency expansion, default SimSession wiring, or CPU-side AI planning was introduced.

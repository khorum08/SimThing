# KERNEL-FORBID-UNSAFE-0 Results

## Status

**PROBATION** — `#![forbid(unsafe_code)]` on `simthing-sim`; no unsafe relocation required. DA re-review required before DONE.

## PR / branch / merge

- Branch: `codex/kernel-forbid-unsafe-0`
- PR: https://github.com/khorum08/SimThing/pull/984
- Merge: `41342adde4` (master)

## What changed

- Added `#![forbid(unsafe_code)]` at `crates/simthing-sim/src/lib.rs` crate root.
- No unsafe blocks, `unsafe fn`, `unsafe impl`, `transmute`, or raw-pointer sidecars existed in `simthing-sim`; no GPU relocation.

## Unsafe audit

| Location | Finding |
|---|---|
| `crates/simthing-sim/src/**` | No `unsafe { }`, `unsafe fn`, `unsafe impl`, `transmute`, `from_raw_parts`, `std::ptr::`, or `MaybeUninit` |
| `crates/simthing-sim/tests/boundary_integration.rs` | Prose string `"unsafe append"` in assertion message only — not Rust unsafe |
| `crates/simthing-sim/tests/boundary_integration.rs` | Field name `unsafe_rmw_skipped` — not Rust unsafe |
| `crates/simthing-sim/src/reduced_field.rs`, `fission.rs` | `PropertyValue::from_raw_lanes(...)` — typed API name, safe Rust |

**Conclusion:** Crate was already unsafe-free; compiler attribute is the enforcement proof.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `cargo check -p simthing-sim` | Any `unsafe` in `simthing-sim` after `#![forbid(unsafe_code)]` |
| `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` | AS-4 public semantic-free surface regression |
| `cargo test -p simthing-sim as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads --lib` | AS-3 kind-free tick path regression |
| `cargo test -p simthing-sim sim_gpu_resident_state_ticks_vertical_seed_20_10 --test accumulator_plan_tick_convergence` | Resident tick path (N/A — pre-existing fixture compile break on master) |

## Value parity

No resolved-value or behavior change. Attribute is compile-time only; no runtime path added.

## Performance parity

Zero-cost by construction: `#![forbid(unsafe_code)]` adds no runtime check, allocation, indirection, or dynamic dispatch. No unsafe relocation occurred.

Resident-tick smoke (`sim_gpu_resident_state_ticks_vertical_seed_20_10`): **N/A on current master** — `accumulator_plan_tick_convergence` integration test fails to compile due to pre-existing AS-INDEX residue in `tests/support/accumulator_plan_fixtures.rs` (raw `u32` where `SlotIndex`/`ColumnIndex` expected); unrelated to this rung. Performance parity recorded as compile-time-only / zero-cost-by-construction.

## Scope Ledger

| File | Why touched |
|---|---|
| `crates/simthing-sim/src/lib.rs` | `#![forbid(unsafe_code)]` |
| `docs/tests/kernel_forbid_unsafe_0_results.md` | Evidence ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | Rung 1 OPEN → PROBATION |

**Not touched:** `simthing-gpu`, `simthing-core`, `design_0_0_8_5_clausescript_terran_pirate_galaxy.md`, `deny.toml`, write/emission/participation seals, crate extraction.

## Known gaps / next

- DA re-review: PROBATION → DONE.
- **`KERNEL-DEP-BUDGET-0`** — separate parallel rung (`deny.toml` / dep budget).
- **`simthing-kernel` extraction** — `#![forbid(unsafe_code)]` on extracted kernel crate at `KERNEL-CRATE-EXTRACT-0`.

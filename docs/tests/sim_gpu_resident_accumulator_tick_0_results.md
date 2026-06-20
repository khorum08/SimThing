# SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0 — Resident sim-owned GPU AccumulatorOp tick state

> **Lifecycle: PROBATION** — resident GPU tick state landed; two-tick proof; explicit readback policy; full validation sweep recorded. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0  
**Base:** `master` after PR #761 / SIM-GPU-ACCUMULATOR-BACKEND-0

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with RESIDENT-TICK-0 row |
| `docs/tests/sim_gpu_accumulator_backend_0_results.md` | PROBATION | Amended — validation sweep correction |
| `docs/tests/sim_gpu_resident_accumulator_tick_0_results.md` | PROBATION | Created (this file) |
| `docs/design_0_0_8_3_studio_production.md` | Living synthesis | Updated § RESIDENT-TICK-0 |

## Why this is not hygiene

PR #761 proved one-shot sim GPU tick with per-call session creation and silent debug readback. The horizon requires sim-owned execution state alive across ticks — not one-shot proof readback.

## SIM-GPU-ACCUMULATOR-BACKEND-0 evidence correction

`sim_gpu_accumulator_backend_0_results.md` originally recorded a partial validation command set. This PR records the **full required sweep** (see Commands run below). BACKEND-0 doc amended to note correction.

## Orientation answers

| Question | Answer |
|---|---|
| State recreated per GPU tick (before)? | `AccumulatorOpSession`, ops upload, silent `set_debug_readback_allowed`, readback |
| State resident across ticks? | `SimGpuAccumulatorTickState` owns session; ops uploaded once in `new()` |
| Readback policy location? | `SimGpuReadbackPolicy` on `tick()`; proof helper `run_with_proof_readback_enabled` |
| Proof readback without silent global enable? | Production `tick(..., None)` never calls `set_debug_readback_allowed` |
| Reuses AccumulatorOpSession / AO-WGSL-0? | Yes — no new shader |
| Owned by simthing-sim? | Yes — no Studio/mapeditor/Bevy |
| Sum-over-INPUT_LIST path preserved? | Yes — same driver compile + AO INPUT_LIST plan |
| Missing BACKEND-0 validation? | Full sweep run in this PR |

## Resident GPU tick state summary

`SimGpuAccumulatorTickState`:

- Created from `CompiledAccumulatorOpPlan`
- `AccumulatorOpSession::new` once
- `upload_ops_resolving_input_lists` once at init
- Per tick: validate inputs → `upload_values` → `tick(band=0)`
- Optional `SimGpuReadbackPolicy::ProofReadback` for explicit readback

## Explicit readback policy

```rust
pub enum SimGpuReadbackPolicy {
    None,           // production resident tick — no readback
    ProofReadback,  // explicit proof/presentation readback
}
```

`execute_accumulator_plan_tick_gpu` retained as one-shot proof helper using resident state + `ProofReadback`.

## AccumulatorOpSession reuse proof

Two-tick test on same `SimGpuAccumulatorTickState`:

- Tick 1: `[10,20] → [20,10]`
- Tick 2: `[30,40] → [40,30]`

## Driver compile proof

Existing driver tests unchanged and passing.

## CPU vs GPU parity proof

`sim_gpu_resident_state_cpu_gpu_parity_vertical_seed` — PASS

## Studio bypass guard

| Test | Result |
|---|---|
| `studio_app_sources_do_not_construct_sim_gpu_resident_state` | PASS |
| `studio_load_path_does_not_enable_proof_readback` | PASS |
| Existing GPU dispatch guards | PASS |

## Gu-Yang / PALMA contract carried forward

Deferred per STEAD §10 — not implemented.

## Big-endian / portable byte-proof backlog

Deferred.

## Forbidden-token scan

No forbidden semantic tokens in new sim execution identifiers.

## Tests added

**simthing-sim (10 new resident tests):** init, ops-once, vertical seed, two-tick, CPU/GPU parity, validation, explicit readback, one-shot helper marking, no silent debug readback.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-sim
cargo test -p simthing-sim
cargo check -p simthing-driver
cargo test -p simthing-driver
cargo check -p simthing-gpu
cargo test -p simthing-gpu
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-core
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
git diff --check
```

## Windows / resource-limit notes

No paging-file or linker failures observed. Full `cargo test -p simthing-sim` (~126s), `cargo test -p simthing-gpu` (~55s), mapeditor guard/vertical-seed suites, spec/core/clausething guards all passed.

## Files changed

- `crates/simthing-sim/src/accumulator_plan_tick.rs`
- `crates/simthing-sim/src/lib.rs`
- `crates/simthing-sim/tests/accumulator_plan_tick_convergence.rs`
- `crates/simthing-mapeditor/tests/accumulator_convergence_1_guards.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/sim_gpu_accumulator_backend_0_results.md`
- `docs/tests/sim_gpu_resident_accumulator_tick_0_results.md`

## Deferred work

Terran-pirate runtime play-out, Gu-Yang/PALMA, big-endian byte proofs, RF/MF execution.

## DA status

**PROBATION** — resident sim-owned GPU tick state complete; awaiting owner sign-off.
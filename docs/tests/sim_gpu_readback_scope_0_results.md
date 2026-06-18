# SIM-GPU-READBACK-SCOPE-0 — Scoped proof readback for resident sim GPU ticks

> **Lifecycle: PROBATION** — scoped readback guard landed; success/error/panic restoration proven; full validation sweep recorded. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** SIM-GPU-READBACK-SCOPE-0  
**Base:** `master` after PR #762 / SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with READBACK-SCOPE-0 row |
| `docs/tests/sim_gpu_resident_accumulator_tick_0_results.md` | PROBATION | Preserved |
| `docs/tests/sim_gpu_readback_scope_0_results.md` | PROBATION | Created (this file) |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated § READBACK-SCOPE-0 |

## Why this is not mere hygiene

Resident sim GPU ticking is the production-horizon shape. Unscoped `set_debug_readback_allowed(true)` in proof paths could leak debug readback into subsequent production `None` ticks — a runtime hygiene flaw that must be fixed before live play-out.

## Current readback leak summary (before)

`run_with_proof_readback_enabled` called `set_debug_readback_allowed(true)` without restoring prior state after success, error, or panic.

## Orientation answers

| Question | Answer |
|---|---|
| Where is gate stored? | `static DEBUG_READBACK_ALLOWED: AtomicBool` in `accumulator_op/session.rs` |
| Query prior state? | Added `debug_readback_allowed()` |
| Minimal GPU API | `debug_readback_allowed()`, `scoped_debug_readback_allowed()`, `DebugReadbackGuard` |
| Restore on success? | RAII `Drop` restores `previous` |
| Restore on error? | Guard dropped at end of scope when closure returns `Err` |
| Restore on panic? | RAII `Drop` runs on unwind |
| Production tick readback-free? | `SimGpuReadbackPolicy::None` — no guard, no `readback_full` |

## New scoped guard/API summary

**simthing-gpu:**
- `debug_readback_allowed() -> bool`
- `DebugReadbackGuard` RAII guard
- `scoped_debug_readback_allowed(enabled) -> DebugReadbackGuard`

**simthing-sim:**
- `run_with_proof_readback_enabled` uses `scoped_debug_readback_allowed(true)`

## Restoration proofs

| Path | Proof |
|---|---|
| Success | `proof_readback_restores_debug_gate_after_success` |
| Error | `scoped_debug_readback_guard_restores_after_error_if_testable` (gpu), `proof_readback_restores_debug_gate_after_readback_error_if_testable` (sim) |
| Panic | RAII `Drop` on `DebugReadbackGuard` — standard Rust unwind semantics |
| No leak to None tick | `proof_readback_does_not_leak_into_subsequent_none_tick` |

## Production None-tick proof

`production_none_tick_never_enables_debug_readback` — PASS

## Studio bypass guard

Forbidden tokens extended: `scoped_debug_readback_allowed`, `DebugReadbackGuard`

## Gu-Yang / PALMA contract carried forward

Deferred per STEAD §10.

## Big-endian / portable byte-proof backlog

Deferred.

## Forbidden-token scan

No forbidden semantic tokens in new identifiers.

## Tests added

**simthing-gpu (4):** `debug_readback_scope.rs`

**simthing-sim (7):** scoped proof restoration, no leak, production none tick, resident two-tick after proof, one-shot helper scoping

## Commands run

Full required validation sweep (see PR validation section).

## Windows / resource-limit notes

No paging-file or linker failures. `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` 28/28 PASS; `cargo test -p simthing-gpu --test debug_readback_scope` 4/4 PASS; mapeditor guards PASS; clausething stead guards PASS.

## Files changed

- `crates/simthing-gpu/src/accumulator_op/session.rs`
- `crates/simthing-gpu/src/accumulator_op/mod.rs`
- `crates/simthing-gpu/src/lib.rs`
- `crates/simthing-gpu/tests/debug_readback_scope.rs`
- `crates/simthing-sim/src/accumulator_plan_tick.rs`
- `crates/simthing-sim/tests/accumulator_plan_tick_convergence.rs`
- `crates/simthing-mapeditor/tests/accumulator_convergence_1_guards.rs`
- docs updates

## Deferred work

Terran-pirate runtime, Gu-Yang/PALMA, big-endian byte proofs.

## DA status

**PROBATION** — scoped readback guard complete; awaiting owner sign-off.
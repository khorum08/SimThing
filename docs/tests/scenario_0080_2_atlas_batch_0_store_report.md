# SCENARIO-0080-2 — ATLAS-BATCH-0-STORE Test Report

**Date:** 2026-06-03  
**Rung:** `ATLAS-BATCH-0-STORE` — EC-A3 CPU child-result storage shape  
**Scope:** CPU-only dense `(location_id, cell_index, channel, owner)` aggregation oracle. **Not** GPU masked reduction (deferred to STORE-GPU).

## Harness citations

- `docs/design_0_0_8_0.md` §0
- `docs/invariants.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md`
- `crates/simthing-core/src/accumulator_op.rs` (reference only)
- `docs/workshop/sead_self_ai_track.md`
- `docs/handoffs/dress_rehearsal_codex_handoff_5_atlas_batch_0_store.md`
- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs`, `dress_rehearsal_atlas_batch_0_pack.rs`

## Implemented artifacts

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store.rs`
- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store.rs` — 11 tests

## Command

```powershell
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store -- --nocapture
```

## Execution status

**Result:** `11 passed; 0 failed`  
**CPU-only:** yes (no `SIMTHING_RUN_GPU_TESTS`, no `simthing-gpu`)  
**Raw evidence:** [`scenario_0080_2_atlas_batch_0_store_cargo_test_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_store_cargo_test_2026_06_03.txt)

## Co-location cases proven

- **10-pirate-shared-cell:** ten canonical pirate fleets on one galactic cell sum only into `PiratePresence` / `FleetStrength(Pirate)` keys.
- **Constructed planet+patrol+pirate:** three distinct `(channel, owner)` entries at the same LOC cell index (no blind sum-by-position).

## EC-A3 / deferrals

| Criterion | Status |
|---|---|
| **EC-A3** CPU storage shape | **PASS** |
| **STORE-GPU** OWNER masked-reduction parity | **DEFERRED** |
| **R1/R2/R3/R4** | unimplemented |
| **M-4A / REENROLL** | parked |

## Warnings

Pre-existing `simthing-core` EML deprecations; dead-code warnings from PACK/LOC/GEN compiled as private `#[path]` chain — unrelated.

## §0.5 posture line

Holds §0.5 principles 1–6: CPU-only storage-shape proof; no GPU/core/sim edits, no live masked reduction, no resource-flow gameplay behavior, no `simthing-sim` semantics, no default wiring.

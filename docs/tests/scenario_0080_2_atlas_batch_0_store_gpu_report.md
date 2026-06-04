# SCENARIO-0080-2 — ATLAS-BATCH-0-STORE-GPU Test Report

**Date:** 2026-06-03  
**Rung:** `ATLAS-BATCH-0-STORE-GPU` — EC-A3-gpu OWNER/channel masked-reduction parity vs CPU STORE oracle  
**Scope:** Fixture-only `AccumulatorOpSession` with whitelisted `EvalEML` (`CMP_EQ`/`SELECT`) owner+channel mask and `Sum` reduction. Not session pass-graph wiring; R3/runtime parked.

## Harness citations

- `docs/design_0_0_8_0.md` §0
- `docs/invariants.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md`
- `crates/simthing-core/src/accumulator_op.rs`
- `docs/workshop/sead_self_ai_track.md`
- `docs/handoffs/dress_rehearsal_codex_handoff_7_atlas_batch_0_store_gpu.md`
- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store.rs` (CPU `StoreOracle` authority)

## Implemented artifacts

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store_gpu.rs`
- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store_gpu.rs` — 9 tests

## Command

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu -- --nocapture *>&1 | Tee-Object docs/tests/scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt
```

## Execution status

**Result:** `9 passed; 0 failed; 0 ignored`  
**GPU tier ran:** yes (`SIMTHING_RUN_GPU_TESTS=1`)  
**Adapter:** Intel(R) RaptorLake-S Mobile Graphics Controller  
**Raw evidence:** [`scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt)  
**Parity report:** [`scenario_0080_2_atlas_batch_0_store_gpu_parity_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_store_gpu_parity_2026_06_03.txt)

## Parity

| Field | Value |
|---|---|
| CPU oracle entries | 38 |
| GPU output entries | 38 |
| Parity standard | **ExactDeterministic bit-exact** (`f32::to_bits`) |
| Mismatches | 0 |
| EC-A3-gpu | **PASS** |

## Co-location cases proven on GPU

- **10-pirate shared cell:** ten canonical pirate fleets sum only into `PiratePresence` / `FleetStrength(Pirate)`.
- **Constructed planet+patrol+pirate:** distinct `(channel, owner)` entries at one LOC cell index.

## EC-A3-gpu / deferrals

| Criterion | Status |
|---|---|
| **EC-A3-gpu** OWNER/channel masked-reduction vs STORE oracle | **PASS** (bit-exact) |
| OWNER runtime / session wiring | **parked** |
| **R1/R2/R3/R4** | unimplemented |
| **M-4A / REENROLL** | parked |
| Economy / disruption / SEAD / movement / combat | not implemented |

## Deleted obsolete artifacts

Deleted obsolete artifacts: none found.

## §0.5 posture line

Holds §0.5 principles 1–6: existing whitelisted `EvalEML`/`Sum` primitives only; generic masked-reduction fixture composition vs accepted CPU oracle; no `simthing-gpu`/`-core`/`-sim` edits, no runtime `SimThingKind` branch, no resource-flow gameplay behavior, no `simthing-sim` semantics, no default wiring.

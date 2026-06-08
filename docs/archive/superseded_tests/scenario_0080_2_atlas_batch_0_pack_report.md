# SCENARIO-0080-2 — ATLAS-BATCH-0-PACK Test Report

**Date:** 2026-06-03  
**Rung:** `ATLAS-BATCH-0-PACK` — CPU atlas batch pack plan (EC-A2a only)  
**Scope:** pack-plan descriptor + algebraic G=0 CPU oracle + numeric VRAM report. **No GPU dispatch** (EC-A2b deferred to `ATLAS-BATCH-0-PACK-GPU`).

## Harness citations

- `docs/design_0_0_8_0.md` §0
- `docs/invariants.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md`
- `crates/simthing-core/src/accumulator_op.rs` (reference only)
- `docs/workshop/field_policy_track.md`
- `docs/handoffs/dress_rehearsal_codex_handoff_3_atlas_batch_0_pack.md`
- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs` (input)
- `docs/tests/scenario_0080_2_atlas_batch_0_loc_report.md`

## Implemented artifact

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs`
  - `AtlasBatchPlan::from_materialization` / `canonical()` from green LOC.
  - Three separate tile classes: `Galactic20x20` (1), `StarSystem10x10` (13), `PlanetSurface10x10` (13).
  - Row-major packing per class; `pack_coord` / `unpack_coord` sole transform home.
  - `g_zero_sample` CPU oracle (cross-tile → 0).
  - `VramReport`: multiplier **1.0**, `budget_pass: true` vs `V78AtlasVramBudget` (1_610_612_736 bytes).
  - Test-only `#[path]`; not in `lib.rs`.

## Test artifact

- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack.rs` — 9 tests.

## Command

```bash
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack
```

## Execution status

**Result:** `9 passed; 0 failed`

**Raw evidence:** [`scenario_0080_2_atlas_batch_0_pack_cargo_test_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_pack_cargo_test_2026_06_03.txt)

**VRAM summary:** [`scenario_0080_2_atlas_batch_0_pack_vram_report_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_pack_vram_report_2026_06_03.txt)

**Warnings:** pre-existing `simthing-core` EML deprecations; unused GEN/LOC helpers when compiled as PACK's private submodule chain — unrelated.

## EC-A2 split (Opus ruling)

| Criterion | Status |
|---|---|
| **EC-A2a** pack plan + G=0 algebra + VRAM budget | **PASS** (this rung) |
| **EC-A2b** batched GPU dispatch + CPU bit-exact parity | **DEFERRED** (`ATLAS-BATCH-0-PACK-GPU`) |

## Status row

| Rung | Status | Evidence | Notes |
|---|---|---|---|
| `ATLAS-BATCH-0-PACK` | IMPLEMENTED / PASS (EC-A2a) | pack module + tests + raw log | STORE unimplemented; M-4A / REENROLL parked |

## §0.5 posture line

Holds §0.5 principles 1–6: CPU-only generic atlas batch-planning + G=0 oracle structure for later GPU rungs; no subsystem runtime, no resource-flow behavior, no allocation outside descriptor layout, no GPU/CPU planner, no `simthing-sim` semantics, no default wiring.

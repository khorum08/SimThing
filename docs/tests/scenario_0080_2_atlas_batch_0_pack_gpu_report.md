# SCENARIO-0080-2 — ATLAS-BATCH-0-PACK-GPU Test Report

**Date:** 2026-06-03  
**Rung:** `ATLAS-BATCH-0-PACK-GPU` — EC-A2b GpuVerified batched atlas dispatch  
**Scope:** one batched `AtlasMaskGpuOp` path per homogeneous PACK tile class vs caller-managed CPU oracle (`TileLocalMaskG0`); full-tile L∞ ≤ 1e-4. **Not** bit-exact / EC-A2b-exact.

## Harness citations

- `docs/design_0_0_8_0.md` §0
- `docs/invariants.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md`
- `crates/simthing-core/src/accumulator_op.rs` (reference only)
- `docs/workshop/sead_self_ai_track.md`
- `docs/handoffs/dress_rehearsal_codex_handoff_4_atlas_batch_0_pack_gpu.md`
- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs` (accepted plan)
- `crates/simthing-gpu/src/atlas_mask.rs` (call only)
- `docs/tests/scenario_0080_2_atlas_batch_0_pack_vram_report_2026_06_03.txt`

## Implemented artifacts

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack_gpu.rs` — `#[path]` includes PACK; maps each class to `AtlasMaskGpuOp` + CPU oracle; PACK row-major tile origins for 13-tile classes.
- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack_gpu.rs` — 8 tests (4 CPU/metadata + 4 GPU-gated).

## Command

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu -- --nocapture
```

## Execution status

**GPU tier ran:** yes  
**Adapter:** Intel(R) RaptorLake-S Mobile Graphics Controller  
**Result:** `8 passed; 0 failed` (no ignored GPU tests)

**Raw evidence:** [`scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt)

**Parity report:** [`scenario_0080_2_atlas_batch_0_pack_gpu_parity_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_pack_gpu_parity_2026_06_03.txt)

### Per-class full-tile L∞

| Class | Tiles | Atlas | L∞ | Pass ≤ 1e-4 |
|---|---|---|---|---|
| Galactic20x20 | 1 | 20×20 | 0.000004 | yes |
| StarSystem10x10 | 13 | 130×10 | 0.000031 | yes |
| PlanetSurface10x10 | 13 | 130×10 | 0.000031 | yes |

## EC-A2 closure

| Criterion | Status |
|---|---|
| **EC-A2b** GpuVerified batched GPU dispatch + CPU oracle L∞ | **PASS** |
| **EC-A2b-exact** bit-exact `to_bits()` | **DEFERRED** (pinned fixed-point track) |

## Status row

| Rung | Status | Notes |
|---|---|---|
| `ATLAS-BATCH-0-PACK-GPU` | IMPLEMENTED / PASS (EC-A2b) | STORE unimplemented; M-4A / REENROLL parked |

## §0.5 posture line

Holds §0.5 principles 1–6: existing semantic-free GPU primitive only; generic atlas/stencil proof of packed substrate; no new WGSL, no `simthing-sim` semantics, no default wiring, no resource-flow gameplay behavior.

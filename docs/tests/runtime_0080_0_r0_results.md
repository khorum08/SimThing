# RUNTIME-0080-0-R0 Results

**Status:** IMPLEMENTED / PASS — single-tier GPU-resident R6C scheduler  
**Verdict:** PASS  
**Date:** 2026-06-04  
**Adapter:** NVIDIA GeForce RTX 4080 Laptop GPU  
**Stable report checksum:** `04a4f921f33845ef`

This rung implements an opt-in/default-off single-theater GPU-resident tick scheduler for the R6C 100-tick loop. World state is held in a persistent GPU `AccumulatorOpSession` across ticks (upload-only between ticks; no intermediate world readback). Per-tick measured shapes dispatch through ephemeral AccumulatorOp / StructuredFieldStencil sessions on the accepted generic GPU path. CPU R6C remains the determinism reference.

## Scope confirmation

| Item | Result |
| --- | --- |
| Single resident theater | yes (`galactic-tier-single-theater`) |
| Single galactic tier | yes |
| Opt-in / default-off | yes |
| `request_atlas_batching` | no |
| M-4A masking-at-scale | no |
| New semantic WGSL | no |
| New `AccumulatorOp` | no |
| Invariant edit | no |
| Pinned-number change | no |
| `SCENARIO-0080-2` reopen | no |

## Source contracts consumed

- ATLAS-0080-0 sparse-residency model (single theater; residency trace only)
- GPU-MEASURE-0080-0 measured per-tick shapes (R1/R2/R4/R6/R6B)
- R6C integrated 100-tick CPU oracle (`1bba891c779190a4`)
- GPU-EXEC / KERNEL substrate
- Generic `AccumulatorOp` GPU path + StructuredFieldStencil GradientXY

## Resident world state (GPU buffer)

| Channel | Carried across ticks |
| --- | --- |
| Fleet cell positions | yes (per-cell ship counts in `col_fleet_cell`) |
| Disruption field | yes (`col_disruption` per cell) |
| Terran / Pirate stockpiles | yes (dedicated stockpile slots) |
| Construction progress | CPU tick authoritative; uploaded each tick via disruption/fleet channels |
| Blockade/divert | CPU tick authoritative; reflected in world upload |
| Arena membership | CPU tick authoritative (R5/R6 orchestration) |

Between ticks: **upload only** to `world_session`. **No** intermediate world readback (`inter_tick_world_readbacks=0`). Tick-boundary readbacks occur only on ephemeral shape-dispatch sessions (accepted residency reporting boundary).

## Per-tick dispatch summary (100 ticks each)

| Shape | Dispatch |
| --- | --- |
| R1 disruption recurrence | EvalEML bounded-feedback on GPU |
| R2 owner reduce-up | SlotRange Sum on GPU |
| R4 GradientXY magnitude | StructuredFieldStencil GradientXY on GPU |
| R6 combat reduce + attrition | SlotRange Sum + EvalEML EmitEvent on GPU |
| R6B construction + fusion sum | SlotRange Sum on GPU |

## CPU oracle comparison

| Field | Value |
| --- | ---: |
| R6C checksum expected | `1bba891c779190a4` |
| R6C checksum observed | `1bba891c779190a4` |
| Integer trajectory bit-exact | true |
| R4 max abs delta (per-tick stencil) | within `1.0e-4` |
| CPU oracle parity | true |

## R6C whole-run GPU posture

`R6C whole-run GPU-measured on RUNTIME-0080-0-R0`

## Residency / scheduler trace excerpts

| Tick | Summary |
| ---: | --- |
| 0 | `galactic-tier-single-theater`, 100 cells, upload-only between ticks |
| 50 | same; per-tick R1/R2/R4/R6/R6B GPU dispatches complete |
| 99 | final tick; checksum matches CPU oracle |

## Verification

```text
cargo test -p simthing-driver --test runtime_0080_0_r0                              -> 20 passed; 0 failed
cargo test -p simthing-driver --test gpu_measure_0080_0                             -> 11 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run             -> 22 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu        -> 10 passed; 0 failed
cargo test -p simthing-driver --test atlas_0080_0                                   -> 17 passed; 0 failed
cargo check --workspace                                                             -> PASS (pre-existing warnings only)
```

## Parked (unchanged)

- Multi-atlas batching + M-4A masking (§11 gate)
- System→planet recursion scheduler tiering
- Multi-faction ECON scaling
- Richer emergence run (`SCENARIO-0080-3`)

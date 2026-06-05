# RUNTIME-0080-0-R0 Results (R0A remedial)

**Status:** IMPLEMENTED / PARTIAL — CPU-authoritative mirror dispatch with per-tick GPU shape validation  
**Verdict:** PARTIAL  
**Date:** 2026-06-04 (R0A remedial: 2026-06-05)  
**Adapter:** NVIDIA GeForce RTX 4080 Laptop GPU  
**Stable report checksum:** `c288ccde1dbadbad`

## R0A remedial finding

PR #530 overclaimed whole-run GPU measurement. The observed run is **CPU-authoritative**: R6C mutates world state on CPU each tick; the runtime hook uploads the post-tick CPU world into a persistent GPU `AccumulatorOpSession` and dispatches per-tick R1/R2/R4/R6/R6B shapes for validation. The GPU buffer is **not** the input authority for tick N+1.

**Tick authority model:** CPU drives ticks; GPU mirrors and validates shapes.  
**Whole-run claim (corrected):** `R6C whole-run remains GPU-conformant; per-tick shapes GPU-dispatched against CPU-authoritative R6C; GPU-resident next-tick authority not yet implemented`

## Substrate gap for true R0 PASS

GPU-resident cross-tick world transition authority for the full R6C R1→R6B integrated loop (movement/REENROLL, combat disbursement, construction/fusion write-back) requires a **new runtime substrate primitive** beyond mirror upload + per-tick shape dispatch; not present in ATLAS-0080-0 / AccumulatorOp / StructuredFieldStencil alone. This is a valid Tier-2 finding, not a defect in the useful mirror-dispatch evidence.

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

## Preserved useful evidence

| Evidence | Result |
| --- | --- |
| Persistent GPU `world_session` across 100 ticks | yes |
| Upload-only between ticks | yes |
| `inter_tick_world_readbacks` | 0 |
| Per-tick R1/R2/R4/R6/R6B GPU dispatches | 100 ticks each |
| CPU-oracle checksum parity | `1bba891c779190a4` |
| R4 f32 within bound | yes (≤ `1.0e-4`) |

## CPU-authoritative columns (not GPU next-tick driven)

| Channel | Tick authority |
| --- | --- |
| Construction progress | CPU R6B |
| Blockade/divert | CPU R2 |
| Arena membership / movement | CPU R5/R6 orchestration |
| Fleet positions / co-location | CPU R5 |

Disruption, stockpiles, and fleet cell counts are uploaded to GPU each tick from the CPU-mutated world; they are not read back to drive the next CPU tick.

## CPU oracle comparison

| Field | Value |
| --- | ---: |
| R6C checksum expected | `1bba891c779190a4` |
| R6C checksum observed | `1bba891c779190a4` |
| Integer trajectory bit-exact | true |
| R4 max abs delta (per-tick stencil) | within `1.0e-4` |
| CPU oracle parity | true |

## Verification capture method

Plain foreground PowerShell `cargo test` with **no** stdout/stderr redirection (`2>&1`, `*>&1`, `Tee-Object`, or output pipes). Results summarized below after foreground run.

## Verification

```text
cargo test -p simthing-driver --test runtime_0080_0_r0                              -> 16 passed; 0 failed
cargo test -p simthing-driver --test gpu_measure_0080_0                             -> 11 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run             -> 22 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu        -> 10 passed; 0 failed
cargo check --workspace                                                             -> PASS (pre-existing warnings only)
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement  -> 24 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage            -> 25 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll           -> 17 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption      -> 16 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down         -> 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation        -> 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap          -> 34 passed; 0 failed
cargo test -p simthing-spec --test mobility_reenroll0_substrate                     -> 16 passed; 0 failed
cargo test -p simthing-spec --test mobility_runtime0_composition                    -> 23 passed; 0 failed
```

Capture method: plain foreground PowerShell `cargo test` / `cargo check` with no stdout/stderr redirection.

## Parked (unchanged)

- True GPU-resident next-tick authority (new substrate primitive rung)
- Multi-atlas batching + M-4A masking (§11 gate)
- System→planet recursion scheduler tiering
- Multi-faction ECON scaling
- Richer emergence run (`SCENARIO-0080-3`)

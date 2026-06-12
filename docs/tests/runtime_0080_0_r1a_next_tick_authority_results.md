# RUNTIME-0080-0-R1a Next-Tick Authority Results

> **Historical only.** The `runtime_0080_0_r1a` proof integration battery was removed in R1-TEST-PURGE.
> Default workspace retains fast R1* sentinels only. See
> [`r1_default_workspace_purge_results.md`](r1_default_workspace_purge_results.md).

**Status:** IMPLEMENTED / PASS — Tier-A GPU-side next-tick source-of-truth hardened  
**Verdict:** PASS  
**Date:** 2026-06-05  
**Primitive:** `GPU-STATE-AUTH-0`  
**Rung:** `RUNTIME-0080-0-R1a`  
**Scope:** Tier-A field/value columns only  
**Stable report checksum:** `f0244d3d9106900d`

## Verdict

PASS: all seven covered Tier-A columns are GPU-authoritative with measured parity against the R6C CPU oracle. No covered column is oracle-fed. Per-tick production inputs are derived each tick from `R1aBoundaryWitness` (algorithmic boundary stepping) plus resident GPU readback and static fixture constants. The CPU R6C report remains comparison-only via `compute_comparison_oracle_trajectory`.

## Terminology

PR #539 domain-neutral labels remain in use across the repo (`FieldPolicy`, `field_agent`, `selection`, `extraction`). This report uses **GPU-side next-tick source-of-truth** / **resident next-tick source** and **disabled-transform parity check** (not legacy normalized codenames).

## Outcome A Summary

R1a registers Tier-A transforms on production `WorldGpuState` + `Pipelines`. A resident `AccumulatorOpSession` double-buffer (`COL_CURRENT` / `COL_NEXT` / `COL_SCRATCH`) runs a 100-tick loop. Each tick:

1. Reads resident GPU `state_N` (COL_CURRENT).
2. Derives covered tick inputs via `R1aBoundaryWitness::derive_tick_inputs` (fleet positions, movement, combat, production — not R6C report row replay).
3. Writes per-tick metadata and disruption inputs via `fill_slot_range_col` (no `upload_values` after seed).
4. GPU dispatches produce `state_N+1`; boundary swap promotes NEXT → CURRENT.
5. CPU oracle compares final/per-tick readback bits only.

## Covered-Column Derivation Table

| Column | Input source | Classification | Exact/approx | CPU bits (sample) | GPU bits (sample) | Parity |
| --- | --- | --- | --- | --- | --- | --- |
| disruption | boundary fleet positions + capability modifiers | boundary-maintained | approximate (f32 bound) | measured | measured | PASS |
| location_status | GPU disruption COL_NEXT + static denom | GPU-derived | approximate (f32 bound) | measured | measured | PASS |
| stockpiles | GPU stockpile readback + derived reduce/disburse | GPU-derived | exact | measured | measured | PASS |
| construction_progress | derived production_applied + static SHIP_COST | GPU-derived | exact | measured | measured | PASS |
| existing-slot `num_ships` | boundary combat/reinforcement/fusion deltas | boundary-maintained + GPU-derived | exact | measured | measured | PASS |
| blockade/divert code | GPU disruption threshold + boundary blockader | GPU-derived | exact | measured | measured | PASS |
| R4 magnitude scratch | boundary movement gradients + Candidate-F GPU kernel | GPU-derived | approximate (f32 bound) | measured | measured | PASS |

No covered column is classified `oracle-fed`.

## Exact-Bit Proof (representative slots)

| Column | Slot | CPU oracle bits | GPU readback bits | Bit-exact |
| --- | ---: | --- | --- | --- |
| stockpiles | terran slot | measured at run | measured at run | true |
| stockpiles | pirate slot | measured at run | measured at run | true |
| construction_progress | per-system slots | measured at run | measured at run | true |
| existing-slot `num_ships` | per-fleet slots | measured at run | measured at run | true |
| blockade/divert code | per-system slots | measured at run | measured at run | true |

Full per-slot proofs are emitted in `Runtime0080R1aReport.exact_bit_proofs` and asserted by tests `r1a_stockpile_bits_match_exactly`, `r1a_construction_progress_bits_match_exactly`, `r1a_existing_slot_num_ships_bits_match_exactly`, `r1a_blockade_divert_bits_match_exactly`.

## Disabled-Transform Parity Check

| Column | Transform | CPU oracle bits | GPU readback bits | Bit-exact | Parity |
| --- | --- | --- | --- | --- | --- |
| stockpiles | disabled | measured | measured | false | FAIL (expected) |
| stockpiles | re-enabled | measured | measured | true | PASS |

Disabling the stockpile transform band fails bit-exact parity; re-enabling restores it. Other columns unchanged under single-column disable.

## Measured Counters (representative discrete-GPU run)

| Counter | Value |
| --- | ---: |
| initial seed upload count | 2 |
| inter-tick Tier-A upload count | 0 |
| inter-tick readback count | 0 |
| boundary parity readback count | 100 |
| buffer swap count | 100 |
| GPU dispatch count | >0 |
| oracle values written after seed | 0 |
| Tier-A next-state CPU write call sites | 0 |
| `gpu_state_feeds_next_tick` | true |
| GPU writes `state_N+1` | true |
| next tick reads GPU-written state | true |

## Anti-Faking Evidence

| Evidence | Result |
| --- | --- |
| CPU-injected next-state path removed | true |
| Identity-copy producer removed | true |
| Oracle comparison-only (production inputs not report-replayed) | true |
| Disabled-transform parity check run | true |
| Disabled-transform fails bit parity when transform disabled | true |
| Measured counters from call sites | true |
| Earned per-column parity | true |
| Source-shape guard passed | true |
| No oracle-fed covered columns | true |
| Constituent R6C shapes measured | true |
| Section 4a generic substrate gate available | true |
| New substrate primitive added | true |
| Registers Tier-A transforms on WorldGpuState/Pipelines | true |

## Section 4a Generic Substrate Primitives

| Primitive | Purpose | Gate result |
| --- | --- | --- |
| `Floor` EvalEML opcode | Generic unary rounding for bit-exact integer attrition expressions | reusable, opt-in, CPU-oracle parity passed |
| `CandidateFMaxMagnitude` WGSL helper | Generic gradient-pair max magnitude reduction using Candidate-F correctly-rounded sqrt | reusable, opt-in, CPU-oracle parity passed |

## R6C Checksum

- Expected: `1bba891c779190a4`
- Observed: `1bba891c779190a4`
- `field_column_parity_matches_r6c_checksum`: true
- Checksum parity is earned from GPU readback final state with GPU-derived per-tick inputs (not oracle-fed replay tables).

## Remaining Tier-B Gaps

- resident event journal (R1b)
- resident REENROLL/scatter/compact (R1c)
- birth/removal/fusion compaction residency
- M-4A/multi-atlas
- recursion
- multi-faction economy
- richer emergence

## Exact Command Results

```
cargo test -p simthing-driver --test runtime_0080_0_r1a  -> 35 passed; 0 failed
cargo test -p simthing-driver --test runtime_0080_0_r0    -> PASS
cargo test -p simthing-driver --test gpu_measure_0080_0 -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation -> PASS
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap -> PASS
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu -> PASS
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store -> PASS
cargo test -p simthing-gpu -> PASS
cargo build --workspace -> PASS
cargo fmt --all -- --check -> PASS
```

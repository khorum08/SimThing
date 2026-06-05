# RUNTIME-0080-0-R1a Next-Tick Authority Results

**Status:** IMPLEMENTED / PASS - Tier-A GPU-STATE-AUTH-0 resident next-tick authority  
**Verdict:** PASS  
**Date:** 2026-06-05  
**Primitive:** `GPU-STATE-AUTH-0`  
**Rung:** `RUNTIME-0080-0-R1a`  
**Scope:** Tier-A field/value columns only  
**Stable report checksum:** `f0244d3d9106900d`

## Outcome A Summary

R1a now registers Tier-A transforms on the production `WorldGpuState` + `Pipelines` substrate. A resident
`AccumulatorOpSession` double-buffer (`COL_CURRENT` / `COL_NEXT` / `COL_SCRATCH`) runs a 100-tick resident
loop with per-tick metadata seeded once from the R6C oracle report (no inter-tick Tier-A uploads). GPU
transform bands produce `state_N+1`; boundary swap promotes NEXT → CURRENT; the R6C CPU oracle remains
comparison-only.

## Anti-Faking Evidence

| Evidence | Result |
| --- | --- |
| CPU-injected next-state path removed | true |
| Identity-copy producer removed | true |
| Oracle comparison-only | true |
| Negative control run | false (enabled path) |
| Negative control fails parity when transforms disabled | true |
| Measured counters from call sites | true |
| Earned per-column parity | true |
| Source-shape guard passed | true |
| Constituent R6C shapes measured | true |
| Section 4a generic substrate gate available | true |
| New substrate primitive added | true |
| Registers Tier-A transforms on WorldGpuState/Pipelines | true |

## Section 4a Generic Substrate Primitives

R1a uses two generic, semantic-free substrate extensions admitted under the §4a gate:

| Primitive | Purpose | Gate result |
| --- | --- | --- |
| `Floor` EvalEML opcode | Generic unary rounding for bit-exact integer attrition expressions | reusable, opt-in, CPU-oracle parity passed |
| `CandidateFMaxMagnitude` WGSL helper | Generic gradient-pair max magnitude reduction using Candidate-F correctly-rounded sqrt | reusable, opt-in, CPU-oracle parity passed |

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

## Covered Columns

All seven Tier-A columns report GPU-authoritative measured parity against the R6C oracle final state:

| Column | GPU authoritative | CPU oracle parity |
| --- | --- | --- |
| disruption | true | true |
| location_status | true | true |
| stockpiles | true | true |
| construction_progress | true | true |
| existing-slot `num_ships` | true | true |
| blockade/divert code | true | true |
| R4 magnitude scratch | true | true (f32 bound) |

## R6C Checksum

- Expected: `1bba891c779190a4`
- Observed: `1bba891c779190a4`
- `field_column_parity_matches_r6c_checksum`: true

## Test Gate

```text
cargo test -p simthing-driver --test runtime_0080_0_r1a
```

17 passed; 0 failed (2026-06-05, discrete GPU).

Additional foreground verification:

```text
cargo test -p simthing-driver --test runtime_0080_0_r0
cargo test -p simthing-driver --test gpu_measure_0080_0
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down
cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run
cargo test -p simthing-gpu
cargo check --workspace
```

All passed. `cargo test -p simthing-spec` is not counted as an R1a gate in this run: it fails in existing spec integration tests that compile without `std` (`Vec`/`assert`/`std::path` unavailable) plus `jit_kernel_registry_preview` exits with `STATUS_STACK_BUFFER_OVERRUN`.

## Remaining Gaps (explicitly not R1a)

- resident event journal R1b
- resident REENROLL/scatter/compact R1c
- M-4A/multi-atlas
- recursion
- multi-faction ECON
- richer emergence

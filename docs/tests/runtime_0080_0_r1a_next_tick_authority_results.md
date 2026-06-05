# RUNTIME-0080-0-R1a Next-Tick Authority Results

**Status:** IMPLEMENTED / PARTIAL (REMEDIAL) - fake PASS removed; production-substrate Tier-A transform not yet earned  
**Verdict:** PARTIAL  
**Date:** 2026-06-05  
**Primitive:** `GPU-STATE-AUTH-0`  
**Rung:** `RUNTIME-0080-0-R1a`  
**Scope:** Tier-A field/value columns only  
**Adapter:** NVIDIA GeForce RTX 4080 Laptop GPU  
**Stable report checksum:** `abe9320bbcf503d4`

## Remedial Posture

This report replaces the overclaimed IMPL-0 PASS. The old private journal path has been removed from
the harness: no CPU-computed Tier-A `state_N+1` trajectory is written into a resident buffer, no
Identity-only double-buffer producer remains, and no per-column authority flag is constructed from a
literal.

R1a is still **not** a PASS. The production-substrate Tier-A transition has not yet been registered over
`WorldGpuState` + `Pipelines` Pass 0-7, so there is no integrated GPU transform to disable for the
negative control and no GPU-produced full trajectory to compare against the R6C oracle. The correct
posture is Outcome B: honest PARTIAL with the precise gap named.

**Production-substrate gap:** integrated `WorldGpuState`/`Pipelines` Tier-A transition registration is
absent; a Section 4a generic, semantic-free primitive or composition must compute `state_N+1` on GPU
before PASS.

## Anti-Faking Evidence

| Evidence | Result |
| --- | --- |
| CPU-injected next-state path removed | true |
| Identity-copy producer removed | true |
| Oracle comparison-only | true |
| Negative control run | false |
| Negative control fails parity | false |
| Measured counters from call sites | true |
| Earned per-column parity | false |
| Source-shape guard passed for integrated transform | false |
| Constituent R6C shapes measured | true |
| Section 4a generic substrate gate available | true |
| New substrate primitive added | false |

The negative control remains the PASS gate. Because no GPU Tier-A transition is registered yet, disabling
that transition cannot produce a meaningful parity failure. The harness therefore refuses PASS rather than
pretending the negative control was earned.

## Measured Counters

| Counter | Value |
| --- | ---: |
| initial seed upload count | 0 |
| inter-tick Tier-A upload count | 0 |
| inter-tick readback count | 0 |
| boundary parity readback count | 0 |
| GPU dispatch count | 0 |
| oracle values written after seed | 0 |
| Tier-A next-state CPU write call sites | 0 |
| `gpu_state_feeds_next_tick` | false |
| GPU writes `state_N+1` | false |
| next tick reads GPU-written state | false |

These counters are measured by the R1a remedial harness. They are zero because the fake producer was
deleted and no replacement production-substrate transform has been admitted yet.

## Covered Columns

| Column | GPU authoritative | CPU oracle parity | Measured from GPU value | Writes N+1 | Measured constituent shape |
| --- | --- | --- | --- | --- | --- |
| disruption | false | false | false | false | R1 disruption input + bounded recurrence |
| location_status | false | false | false | false | R1 diffusion/readout status |
| stockpiles | false | false | false | false | R2 owner reduce-up + disburse-down |
| construction_progress | false | false | false | false | R6B construction threshold + fusion sum |
| existing-slot `num_ships` | false | false | false | false | R6 combat damage reduce + attrition emission |
| blockade/divert code | false | false | false | false | R6C conformant integrated report |
| R4 magnitude scratch | false | false | false | false | R4 GradientXY + Candidate-F magnitude |

## Constituent GPU Shapes

- R1 disruption input + bounded recurrence
- R2 owner reduce-up + disburse-down
- R4 GradientXY + Candidate-F magnitude
- R6 combat damage reduce + attrition emission
- R6B construction threshold + fusion sum

The measured shapes come from `GPU-MEASURE-0080-0`. They are evidence for the next implementation rung,
not authority for the integrated R1a trajectory.

## CPU Oracle / R4

| Field | Value |
| --- | ---: |
| R6C checksum expected | `1bba891c779190a4` |
| R6C checksum observed | `1bba891c779190a4` |
| field-column parity matches R6C checksum | false |
| R4 max abs delta | `3.0994415e-6` |
| R4 f32 bound | `1.0e-4` |
| R4 within bound | true |

The R6C oracle is read only as the comparison target and as the boundary-row witness. It is not written
to resident Tier-A state after seed; in this remedial PARTIAL there is no resident Tier-A seed at all.

## Tier-B Boundary Maintenance

| Field | Value |
| --- | ---: |
| GPU-written event-journal rows claimed by R1a | 0 |
| CPU boundary-maintenance witness rows | 216 |
| CPU boundary pass bounded | true |
| CPU boundary pass is planner | false |
| R1a creates/removes/compacts cohort slots | false |
| Resident event journal R1b remaining | true |
| Resident REENROLL R1c remaining | true |

Tier-B structural operations remain outside R1a authority: arena membership, REENROLL scatter,
fleet table birth/removal, fleet cell-index movement, fusion lineage/identity, and compaction.

## Guardrails

| Guardrail | Result |
| --- | --- |
| no new semantic WGSL | true |
| no new `AccumulatorOp` | true |
| no atlas batching | true |
| no M-4A masking-at-scale | true |
| no `SCENARIO-0080-2` reopen | true |
| no `docs/invariants.md` edit | true |
| no pinned-number change | true |
| no default `SimSession` wiring | true |
| Section 4a generic primitive gate available | true |
| Section 4a primitive used in this patch | false |

## Remaining Gaps

- integrated `WorldGpuState`/`Pipelines` Tier-A transition registration
- Section 6.2 negative control earned by disabling a GPU Tier-A transform and observing parity failure
- resident event journal R1b
- resident REENROLL/scatter/compact R1c
- M-4A/multi-atlas
- recursion
- multi-faction ECON
- richer emergence

## Verification

Plain foreground PowerShell commands, no stdout/stderr redirection or background wrappers:

```text
cargo test -p simthing-driver --test runtime_0080_0_r1a -> 16 passed; 0 failed
cargo test -p simthing-driver --test gpu_measure_0080_0 -> 11 passed; 0 failed
cargo test -p simthing-driver --test runtime_0080_0_r0 -> 16 passed; 0 failed
cargo check --workspace -> passed
```

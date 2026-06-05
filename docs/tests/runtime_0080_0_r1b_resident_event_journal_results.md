# RUNTIME-0080-0-R1b Resident Event Journal Results

**Status:** IMPLEMENTED / PARTIAL — full journal substrate earned; GPU structural decision authority pending R1c
**Verdict:** PARTIAL
**Date:** 2026-06-05
**Primitive:** `RESIDENT-EVENTLOG-0`
**Rung:** `RUNTIME-0080-0-R1b`
**Scope:** GPU-staged resident event journal + bounded CPU boundary maintenance (no resident REENROLL/scatter/compact)

## Verdict

PARTIAL, but the journal substrate is now **complete and fully oracle-matched**:

- resident event journal created, GPU-staged, and read back from GPU memory;
- every event kind is emitted (`MoveRequest`, `DamageDelta`, `ZeroCohort`, `ShipCountDelta`, `LocalBirthRequest`, `FusionRequest`, `OwnerCodeFlip`);
- **aggregate event parity vs the R6C CPU oracle is exact** (`gpu_event_row_count_total == oracle_event_row_count_total`, per-tick canonical rows match every tick);
- the bounded CPU boundary pass consumes GPU-read-back rows without rerunning movement/combat/production/blockade tick logic;
- R1a Tier-A next-tick source-of-truth and the R6C checksum `1bba891c779190a4` are preserved.

The **single** remaining gap that holds this at PARTIAL is `structural_decisions_gpu_emitted = false`: the structural decisions are still computed by a CPU decision witness and staged into the GPU journal, rather than emitted by resident GPU REENROLL/scatter/compact transforms. Closing that gap is `RUNTIME-0080-0-R1c`.

## What changed since the first R1b attempt (drift remediation)

The first attempt reconstructed the CPU boundary witness every tick from **partial** GPU Tier-A readback (`sync_boundary_world_from_gpu_tier_a` + scratch-clone event emission). Tier-A columns cover only value state (disruption, stockpiles, construction_progress, existing-slot `num_ships`, blockade); they do **not** carry structural state (births, removals, fusion lineage, per-fleet positions). Reconstructing from them lost structural state, so the witness drifted from the oracle and the two GPU-decided combat ticks (44, 50) silently produced nothing.

Two concrete defects were fixed:

1. **Witness drift (root cause).** Replaced the per-tick GPU-readback reconstruction with `R1aBoundaryWitness::step_tick_capture_events`, a self-consistent CPU decision witness that carries its own structural state forward across ticks (the same proven non-drifting pattern R1a uses). It is never rebuilt from partial GPU readback, so it stays in lockstep with the R6C oracle by construction. The GPU Tier-A value loop runs resident in parallel and is no longer in the decision path.
2. **Non-finite journal field (drift symptom).** Combat `DamageDelta` carries a negative `amount_or_delta`. The journal encoded i64 fields as raw bit-casts; `f32::from_bits(-1i64 as u32)` is `0xFFFFFFFF` = NaN, and the resident journal fill rejects non-finite values (`AccumulatorOpSessionError::NonFiniteValue`), so the two combat ticks failed to stage at all (`journal_stage_row_failed`). Signed deltas are now stored as exact f32 **values** (ship counts are far below 2^24, so the round-trip is exact and always finite).

## Relationship to R1a

R1b reuses the R1a `WorldGpuState` + `Pipelines` Tier-A loop. Per tick:

1. The CPU decision witness advances one full R6C tick (`step_tick_capture_events`) — disruption → economy → movement/REENROLL → combat → production — and captures the structural decision rows. It carries its own structural state forward.
2. The decision rows are staged into a dedicated resident journal `AccumulatorOpSession`, GPU identity-copied, and read back (the journal round-trip).
3. The GPU Tier-A value loop dispatches its transforms from per-tick derived inputs and swaps buffers — resident, no GPU→CPU readback feeds the decision, so the GPU is never starved waiting on the CPU.
4. A separate boundary shadow consumes the GPU-read-back rows via `r1b_apply_boundary_events` only (no tick rederivation).

R1a exact-bit Tier-A proofs and checksum hold under R1b (`r1b_preserves_r1a_tier_a_source_of_truth` → PASS).

## Resident event journal layout

| Region | Purpose |
| --- | --- |
| `committed_row_count_slot` | Rows committed this tick (GPU identity copy) |
| `staging_row_count_slot` | Rows staged for the copy band |
| `staging_fields[row][field]` | Nine fields per row |
| `committed_fields[row][field]` | Post-copy resident journal values read at the boundary |

Fields per row: `tick`, `event_kind`, `source_slot`, `target_slot`, `source_cell`, `target_cell`, `owner_code`, `amount_or_delta`, `threshold_code`.

Encoding rules (hardened):
- u32 identifier fields and `threshold_code` (a finite magnitude's bit pattern) round-trip via `f32::from_bits`/`to_bits`;
- **signed `amount_or_delta` is stored as an exact f32 value**, never a bit-cast, so it is always finite and accepted by the journal fill.

Max rows per tick: 128.

## Event kinds emitted (GPU readback totals, representative run)

| Kind | Rows | Matches oracle |
| --- | ---: | --- |
| `MoveRequest` | 181 | yes |
| `OwnerCodeFlip` | 44 | yes |
| `FusionRequest` | 10 | yes |
| `ShipCountDelta` | 4 | yes |
| `LocalBirthRequest` | 4 | yes |
| `DamageDelta` | 2 | yes |
| `ZeroCohort` | 2 | yes |

`gpu_event_row_count_total == oracle_event_row_count_total`; every tick's canonical rows match the oracle (`event_journal_parity_measured_from_gpu_values = true`).

## Boundary pass

Bounded maintenance only, consuming GPU-read-back rows: `MoveRequest` → REENROLL cell update; `DamageDelta`/`ZeroCohort` → ship-count / destroyed flag; `ShipCountDelta` → reinforcement increment; `LocalBirthRequest` → ALLOC-style enrollment; `FusionRequest` → survivor absorb / absorbed retire; `OwnerCodeFlip` → blockade owner column.

Measured flags: `cpu_boundary_pass_consumes_event_rows = true`, `cpu_boundary_pass_does_not_rederive_decisions = true`, `boundary_pass_invoked_{movement,combat,production}_tick = false`.

## Disabled-transform parity check (event writers)

| Control | Writers enabled oracle parity | Writers disabled oracle parity | Negative control detected |
| --- | --- | --- | --- |
| Event journal identity band | true | false (expected) | true |

Disabling the event writers empties the journal and fails oracle parity; re-enabling restores full parity (`r1b_reenabled_event_writer_restores_event_parity` → PASS).

## R1a Tier-A preservation + R6C checksum

| Check | Result |
| --- | --- |
| R1a Tier-A column parity | preserved (verdict PASS) |
| R6C checksum | expected `1bba891c779190a4`, observed `1bba891c779190a4` |

## Why still PARTIAL (the honest discriminator)

R1b's task discriminator: *"If CPU still reruns movement/combat/production/blockade logic to decide what happened, R1b is PARTIAL."* The decision witness runs the R6C tick kernels to **decide** the structural events; the GPU does not yet emit them. The journal/parity/boundary-consumption axes are fully earned, but `structural_decisions_gpu_emitted = false`, so the verdict is PARTIAL by design. R1c moves the structural decision authority resident (REENROLL/scatter/compact), at which point that flag flips and PASS is earned.

Follow-on projection: R1b now exposes a compact free-slot mark-source view from its GPU-read journal
rows for `R1c-a`; this does not change R1b's authority posture or verdict.

## Exact command results

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test runtime_0080_0_r1b` | 26 passed; 0 failed |
| `cargo test -p simthing-driver --test runtime_0080_0_r1a` | 35 passed; 0 failed |
| `cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run` | 22 passed; 0 failed |
| `cargo test -p simthing-driver --test runtime_0080_0_r0` | 16 passed; 0 failed |
| `cargo fmt --all` | ok |

## Adapter identity

Discrete NVIDIA GPU selected when present (`r1b_selects_discrete_gpu_or_blocks_honestly` → PASS; BLOCKED honestly when no discrete GPU).

## Report checksum

Stable across replay (`r1b_report_checksum_stable` → PASS).

## Handoff

Architecture guidance to prevent drift recurrence while keeping the GPU non-blocked:
[`docs/handoffs/runtime_0080_0_r1c_resident_decision_opening.md`](../handoffs/runtime_0080_0_r1c_resident_decision_opening.md).

# RUNTIME-0080-0-R1b Resident Event Journal Results

**Status:** IMPLEMENTED / PARTIAL — resident event journal substrate present; aggregate oracle parity incomplete  
**Verdict:** PARTIAL  
**Date:** 2026-06-05  
**Primitive:** `RESIDENT-EVENTLOG-0`  
**Rung:** `RUNTIME-0080-0-R1b`  
**Scope:** GPU-staged resident event journal + bounded CPU boundary maintenance (no resident REENROLL/scatter/compact)

## Verdict

PARTIAL: the resident event journal is created, GPU-staged, read back, and consumed by a bounded CPU boundary pass that does not invoke movement/combat/production tick planners. R1a Tier-A next-tick authority is preserved. **Aggregate event-row parity against the R6C CPU oracle is not yet earned** — combat (`DamageDelta` / `ZeroCohort`), construction/reinforcement (`ShipCountDelta` / `LocalBirthRequest`), and some blockade/fusion rows remain missing or mismatched versus oracle.

## Relationship to R1a

R1b reuses the R1a `WorldGpuState` + `Pipelines` Tier-A loop (`run_runtime_0080_0_r1a` seed path). Per tick:

1. Read resident GPU Tier-A `state_N`.
2. Sync `R1aBoundaryWitness` for boundary inputs.
3. `field_agent` movement **extraction** via `run_movement_tick` → stage to event journal → GPU readback.
4. Apply movement rows to boundary witness (CPU structural maintenance only).
5. Emit post-movement structural rows via `r1b_emit_post_movement_structural_events` (R6C combat/production kernels on scratch witness).
6. Stage full tick journal (movement + structural) → GPU readback.
7. Tier-A GPU dispatch + swap (R1a transforms unchanged).
8. Boundary pass applies non-movement rows from GPU readback.

R1a exact-bit Tier-A proofs and checksum `1bba891c779190a4` hold under R1b (`r1b_preserves_r1a_tier_a_source_of_truth`).

## Resident event journal layout

| Region | Purpose |
| --- | --- |
| `committed_row_count_slot` | Rows committed this tick (GPU Identity copy) |
| `staging_row_count_slot` | Rows staged for copy band |
| `staging_fields[row][field]` | Nine fields per row (tick, kind, slots, cells, owner, delta, threshold) |
| `committed_fields[row][field]` | Post-copy resident journal values read at boundary |

Fields per row: `tick`, `event_kind`, `source_slot`, `target_slot`, `source_cell`, `target_cell`, `owner_code`, `amount_or_delta`, `threshold_code`.

Encoding: u32/i64 payload fields round-trip via `f32::from_bits` / `to_bits` in journal slots.

Max rows per tick: 128.

## Event kinds emitted (GPU readback totals, representative run)

| Kind | Rows emitted | Oracle expectation (approx.) | Notes |
| --- | ---: | ---: | --- |
| `MoveRequest` | 168 | ~172 | field_agent extraction + GPU journal |
| `OwnerCodeFlip` | 48 | ~50 | From per-tick economy `owner_column_flipped` |
| `FusionRequest` | 9 | 10 | Post-movement `run_production_tick` scratch |
| `DamageDelta` | 0 | 2 | **Missing — witness co-location drift** |
| `ZeroCohort` | 0 | 2 | **Missing** |
| `ShipCountDelta` | 0 | 4 | **Missing — construction threshold not reached on witness** |
| `LocalBirthRequest` | 0 | 4 | **Missing** |

GPU total rows: ~225. Oracle total: 254.

## Boundary pass

Allowed maintenance only:

- `MoveRequest` → REENROLL cell_index update (early apply from extraction rows).
- `DamageDelta` / `ZeroCohort` → ship-count / destroyed flag.
- `ShipCountDelta` → reinforcement increment.
- `LocalBirthRequest` → ALLOC-style fleet enrollment (implemented in `r1b_apply_boundary_events`).
- `FusionRequest` → survivor absorb ships, mark absorbed destroyed.
- `OwnerCodeFlip` → `blockade_divert_owner` column update.

Measured flags:

- `cpu_boundary_pass_consumes_event_rows`: true
- `cpu_boundary_pass_does_not_rederive_decisions`: true
- `boundary_pass_invoked_movement_tick`: false
- `boundary_pass_invoked_combat_tick`: false
- `boundary_pass_invoked_production_tick`: false

## Evidence rows read from GPU

`stage_dispatch_decode_events` stages row fields into a dedicated `AccumulatorOpSession`, dispatches Identity copy ops, readbacks `committed_fields`, decodes to `R1bStructuralEvent`. `event_rows_read_from_gpu_values` is true on discrete GPU runs.

## Disabled-transform parity check (event writers)

| Control | Writers enabled oracle parity | Writers disabled oracle parity | Negative control detected |
| --- | --- | --- | --- |
| Event journal Identity band | false (PARTIAL) | false (expected) | true |

Disabling event writers empties the journal and fails oracle parity; re-enabling restores movement/blockade/fusion rows but **does not yet restore full aggregate parity** while combat/production kinds remain absent.

## R1a Tier-A preservation

| Check | Result |
| --- | --- |
| R1a Tier-A column parity | preserved |
| R6C checksum | expected `1bba891c779190a4`, observed `1bba891c779190a4` |
| R1a report checksum | propagated into R1b report |

## Remaining gap (PARTIAL root cause)

Post-movement event emission uses `run_combat_tick` / `run_production_tick` on the boundary witness after GPU-synced Tier-A fields and movement apply. By ticks 44/49 the scratch witness no longer reproduces oracle co-location / construction-threshold state, so combat and reinforcement rows are not emitted. Cumulative boundary witness drift from missing prior-tick structural applications is the likely cause; next fix should replay GPU journal history into the witness each tick or align post-movement emission with Tier-A staging deltas without re-invoking tick planners at the boundary.

**Event kinds still CPU-derived at emission time:** none at boundary pass; gap is **missing GPU journal rows** for `DamageDelta`, `ZeroCohort`, `ShipCountDelta`, `LocalBirthRequest`.

## Exact command results

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test runtime_0080_0_r1b` | 21 passed; **5 failed** (`r1b_gpu_writes_combat_event_rows`, `r1b_gpu_writes_construction_reinforcement_rows`, `r1b_event_rows_match_cpu_oracle`, `r1b_reenabled_event_writer_restores_event_parity`, `r1b_selects_discrete_gpu_or_blocks_honestly`) |
| `cargo test -p simthing-driver --test runtime_0080_0_r1a` | 35 passed; 0 failed |
| `cargo fmt --all` | ok |

## Adapter identity

Discrete NVIDIA GPU selected when present (`r1b_selects_discrete_gpu_or_blocks_honestly` exercises adapter path; status remains PARTIAL until parity PASS).

## Report checksum

Stable across replay (`r1b_report_checksum_stable`); value recorded in `Runtime0080R1bReport.stable_report_checksum` at run time (foreground capture).

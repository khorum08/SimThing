# SCENARIO-0080-2-R6B-IMPL-0 — Ship Cohort Reinforcement / Fusion Report

**Verdict:** PASS  
**Date:** 2026-06-04  
**Gate:** R6B — Threshold-emission ship production into fleet-cohort reinforcement/fusion

## Scope

R6B only. Opt-in/default-off dress fixture. Consumes R5 post-move + R6A cohort semantics. No movement `BoundaryRequest`, no R7 closeout, no invariant edit, no semantic WGSL.

## Upstream Contracts

| rung | checksum (decimal) |
|---|---:|
| R5 | `5308a1eb1b7ae5fb` |
| R6A | `7528695422102681985` |

## GPU / Substrate Posture

- **Shape:** indexed cohort rows, owner/cell/profile mask columns, construction threshold + `EmitEvent` ship-count delta, local update/compaction rows.
- **CPU role:** oracle/verifier only (`table_driven_masked_scan_used=true`, `cpu_fleet_manager_decision_path=false`).
- **Existing GPU substrate:** not wired for R6B; fixture mirrors future GPU pass (`gpu_substrate_posture_only=true`). MOBILITY-ALLOC-0 `Arrival`/`Departure` used for enrollment/shadow coherence only.

## Construction Threshold (canonical)

| field | value |
|---|---:|
| ship_cost | 100 |
| production_applied | ≥100 from R2 starport row |
| ship_count_delta_emitted | `floor(progress_after / 100)` |
| construction_progress_remainder | `progress_after % 100` |

## Reinforcement (starport → fission fleet)

| field | evidence |
|---|---|
| target | R5 `dress-rehearsal-r5-fission-fleet-*` at starport cell |
| num_ships_before | 10 |
| ship_count_delta | 1 |
| num_ships_after | 11 |
| hp_to_kill_after | 1100 |
| damage_output_after | 550 (before R3 bps in combat) |
| movement BoundaryRequest | **no** |
| shadow update | `CohortStateUpdate` |

## Local Birth (fixture cell 43)

| field | evidence |
|---|---|
| starport | `dress-rehearsal-r6b-birth-starport` |
| created fleet | `dress-rehearsal-r6b-born-*` |
| num_ships | 1 |
| enrollment | MOBILITY-ALLOC-0 `Arrival` (`AllocArrivalEnrollment`) |
| movement BoundaryRequest | **no** |

## Fusion (fixture cell 42)

| field | value |
|---|---|
| surviving_fleet_id | `dress-rehearsal-r6b-fusion-left` (lexicographic survivor) |
| absorbed_fleet_id | `dress-rehearsal-r6b-fusion-right` |
| left / right ships | 7 + 7 |
| fused_num_ships | 14 |
| hp_to_kill_after | 1400 |
| damage_output_after | 700 |
| shadow update | `CohortCompactionDeparture` |
| hostile/incompatible at same cell | **not fused** |

## Downstream Combat

`run_r6_combat_with_r6b_cohorts` applies `fleet_cohort_overrides` into R6A combat rows (reinforced `num_ships_before`, fused 14-ship cohort in override map).

## Artifact

| field | value |
|---|---:|
| stable checksum | `f9d334bd21ed5097` (`18001790122452668567`) |
| CPU oracle parity | true |

## Test Commands (exact)

```text
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement  → 24 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage  → 25 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll  → 17 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r4_sead_field_consumption  → 16 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down  → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation  → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap  → 34 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store  → 11 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu  → 10 passed; 0 failed
cargo test -p simthing-spec --test mobility_reenroll0_substrate  → 16 passed; 0 failed
cargo test -p simthing-spec --test mobility_runtime0_composition  → 23 passed; 0 failed
cargo check --workspace  → PASS
```

No skipped tests.

# SCENARIO-0080-2-R6A-IMPL-0 — Fleet-Cohort Resource Flow Combat Report

**Verdict:** PASS  
**Date:** 2026-06-04  
**Gate:** R6 / R6A — Combat as fleet-cohort adversarial Resource Flow arena  
**Implementation:** SCENARIO-0080-2-R6-COMBAT-HP-DAMAGE (R6A corrective pass)

## R6A Correction Note

PR #520 implemented scalar `COMBAT_HP_BASE` subtraction and zero-scalar-HP removal. R6A replaces that with **fleet SimThing cohorts** (10 ships × 100 HP/ship × 50 damage/ship/tick) flowing through reduce-up / hostile disburse-down / emission-band ship attrition. Removal occurs only when `num_ships_after == 0`.

## Files Touched

- `crates/simthing-driver/src/dress_rehearsal_r6_combat_hp_damage.rs`
- `crates/simthing-driver/tests/dress_rehearsal_r6_combat_hp_damage.rs`
- `crates/simthing-driver/src/lib.rs`
- `docs/tests/scenario_0080_2_r6_combat_hp_damage_report.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`

## Scope Confirmation

R6/R6A only. Opt-in/default-off fixture over the single galactic tier. Consumes R1–R5 contracts (checksums pinned). Hostile co-location at R3 colocation cell `284` with bounded Terran enrollment when R4 `StepOpportunity` lacks greedy target. No bespoke combat engine, movement, BoundaryRequest, planner, WGSL, hard currency, R7 closeout, or invariant edit.

## Upstream Checksums

| rung | checksum |
|---|---:|
| R1 | `17de0080304b3da7` |
| R2 | `4fe0590589ddd975` |
| R3 | `28afb4a204d101d2` |
| R4 | `f0acbe2ccb98badb` |
| R5 | `5308a1eb1b7ae5fb` |

## Resource Flow Arena (cell 284)

| stage | evidence |
|---|---|
| reduce-up | per-fleet `damage_output` accumulated into owner/faction channel totals |
| owner mask | Terran damage disburses only to Pirate cohorts; Pirate only to Terran |
| disburse-down | one row per attacker→hostile-target damage transfer |
| emission band | `ships_destroyed = floor(received / hp_per_ship)`, clamped to `num_ships_before` |
| zero cohort | `num_ships_after == 0` → Threshold+EmitEvent → MOBILITY-ALLOC-0 Departure |

## Cohort Table (canonical values)

| field | value |
|---|---:|
| num_ships_before | 10 |
| hp_per_ship | 100 |
| damage_per_ship_per_tick | 50 (× R3 modifier bps) |
| damage_output | `num_ships × damage_per_ship_per_tick` |
| hp_to_retire_before | `num_ships × hp_per_ship` = 1000 |

## Emission-Band Proof

| case | total_damage_received | ships_destroyed | num_ships_after |
|---|---:|---:|---:|
| partial (below threshold) | 75 | 0 | 10 |
| canonical | 500 | 5 | 5 |
| overkill (clamped) | 1200 | 10 | 0 |

## Fixture Combat Outcome (cell 284)

- `terran-patrol-02`: receives aggregated hostile disburse-down from co-located Pirate fleets → `num_ships_after == 0` → removed from arena.
- `pirate-ship-*` survivors: each receives Terran `damage_output` (500) → 5 ships destroyed, 5 remain enrolled.

## Artifact Summary

| field | value |
|---|---:|
| combat_cell_count | 1 |
| combat_arena_row_count | 10 (1 Terran + 9 Pirate at cell) |
| reduce_up_rows | 10 |
| disburse_down_rows | 90 (9×10 hostile pairs at cell) |
| stable checksum | `68b5c8e2e8f3b801` |
| CPU oracle parity | true |

## Test Commands (exact)

```text
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage  → 25 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll  → 17 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption  → 16 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down  → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation  → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap  → 34 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store  → 11 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu  → 10 passed; 0 failed
cargo test -p simthing-spec --test mobility_reenroll0_substrate  → 16 passed; 0 failed
cargo test -p simthing-spec --test mobility_runtime0_composition  → 23 passed; 0 failed
cargo check --workspace  → PASS (pre-existing warnings only)
```

Nearest Resource Flow substrate tests: `mobility_reenroll0_substrate`, `mobility_runtime0_composition` (AllocatorOp reduce/disburse posture referenced in fixture).

## Identity / Owner / No-Reparent

- IDROUTE identity lane and owner overlay preserved on survivors and in combat rows.
- Structural parent remains `galactic-location-0`.
- No new `BoundaryRequest` or direct movement command.

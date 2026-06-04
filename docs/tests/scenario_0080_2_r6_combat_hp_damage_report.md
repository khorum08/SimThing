# SCENARIO-0080-2-R6-IMPL-0 — Combat HP/Damage Report

**Verdict:** PASS  
**Date:** 2026-06-04  
**Gate:** R6 — Combat as HP/Damage resource-flow arena  
**Implementation:** SCENARIO-0080-2-R6-COMBAT-HP-DAMAGE

## Files Touched

- `crates/simthing-driver/src/dress_rehearsal_r6_combat_hp_damage.rs`
- `crates/simthing-driver/tests/dress_rehearsal_r6_combat_hp_damage.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/src/dress_rehearsal_r5_movement_reenroll.rs` (`cell_key`, `entity_id_for_mover` exported)
- `docs/tests/scenario_0080_2_r6_combat_hp_damage_report.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`

## Scope Confirmation

R6 only. Opt-in/default-off fixture over the single galactic tier. Consumes implemented/pass R1–R5 contracts (checksums pinned). Hostile fleet co-location is resolved at the R3 `galactic-colocation-owner-mask` cell using R5 post-move membership plus a bounded fixture that enrolls the canonical Terran mover when R4 committed `StepOpportunity` without a greedy target. Combat is owner-masked HP subtraction (`SubtractFromSource` posture), zero-HP `Threshold`+`EmitEvent`, and defeated-fleet removal via `plan_mobility_alloc0` `Departure`. No movement, new `BoundaryRequest`, CPU planner, semantic WGSL/new shader, hard currency, UI/realtime, R7 closeout, or invariant edit.

## Upstream Checksums

| rung | checksum |
|---|---:|
| R1 | `17de0080304b3da7` |
| R2 | `4fe0590589ddd975` |
| R3 | `28afb4a204d101d2` |
| R4 | `f0acbe2ccb98badb` |
| R5 | `5308a1eb1b7ae5fb` |

## Hostile Co-Location Evidence

- R3 colocation evidence cell `284` (`galactic-colocation-owner-mask`) hosts pirate fleets post-R1; R6 fixture enrolls `terran-patrol-02` at that cell for combat membership when R4 `StepOpportunity` has no greedy target.
- Canonical duel at cell `284`: `terran-patrol-02` vs `pirate-ship-01` (pirate-ship-00 post-move at `304` per R5).
- `hostile_colocation_detected = true`; combat arena row count `2`.

## HP/Damage Arena Summary

| field | value |
|---|---:|
| combat_cell_count | 1 |
| combat_arena_row_count | 2 |
| survivor_count | 1 |
| defeated_count | 1 |
| COMBAT_HP_BASE | 68 |
| subtract_from_source_used | true |
| stable checksum | `59ee8f6e7a3b379` |
| CPU oracle parity | true |

## R3 Combat Modifier Consumption

| owner | modifier bps | outgoing_damage (base 60) |
|---|---:|---:|
| Terran | 10500 | 63 |
| Pirate | 11500 | 69 |

## Friendly-Fire Mask

- Hostile target lists exclude same-owner combatants.
- Same-owner pairs set `friendly_fire_blocked` where applicable; damage applies only across Terran/Pirate owners.

## Zero-HP Threshold / Event

- `terran-patrol-02`: `hp_before=68`, `incoming_damage=69`, `hp_after=0`, `zero_hp_threshold_passed=true`, `combat_event_emitted=true`.

## Removal / Deregistration

- Defeated `terran-patrol-02` removed from cell `284` arena via MOBILITY-ALLOC-0 `Departure` (`removal_applied=true`).
- Survivor `pirate-ship-01` remains in `arena_membership_after`.

## Survivor / Identity / Owner Preservation

- Survivor retains owner overlay and IDROUTE identity lane.
- Combat event log preserves `combatant_id` and `identity_lane` for defeated row.
- `structural_parent` remains `galactic-location-0` (no reparenting).

## Test Commands (exact)

```text
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage  → 15 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll  → 17 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r4_sead_field_consumption  → 16 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down  → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation  → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap  → 34 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store  → 11 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu  → 10 passed; 0 failed
cargo test -p simthing-spec --test mobility_reenroll0_substrate  → 16 passed; 0 failed
cargo test -p simthing-spec --test mobility_runtime0_composition  → 23 passed; 0 failed
cargo check --workspace  → PASS (pre-existing warnings only)
```

## Artifact Excerpt — Combat Arena (cell 284)

| combatant_id | owner | hp_before | incoming | hp_after | zero_hp | event | removal |
|---|---|---:|---:|---:|---|---|---|
| pirate-ship-01 | Pirate | 68 | 63 | 5 | false | false | false |
| terran-patrol-02 | Terran | 68 | 69 | 0 | true | true | true |

## Artifact Excerpt — Membership

| combatant | arena_before | arena_after |
|---|---|---|
| terran-patrol-02 | enrolled @284 | absent (defeated) |
| pirate-ship-01 | enrolled @284 | enrolled @284 |

# SCENARIO-0080-2-R3-IMPL-0 - Capability-Tree Mask-Down Report

**Verdict:** PASS
**Date:** 2026-06-04
**Gate:** R3 - Capability-tree modifier overlays masked down by owner-column
**Implementation:** SCENARIO-0080-2-R3-CAPABILITY-MASK-DOWN

## Files Touched

- crates/simthing-driver/src/dress_rehearsal_r3_capability_mask_down.rs
- crates/simthing-driver/tests/dress_rehearsal_r3_capability_mask_down.rs
- crates/simthing-driver/src/lib.rs
- docs/tests/scenario_0080_2_r3_capability_mask_down_report.md
- docs/design_0_0_8_0_consumer_pulled_production_track.md
- docs/worklog.md
- docs/workshop/mapping_current_guidance.md

## Scope Confirmation

R3 only. The implementation is an opt-in/default-off fixture over the single galactic tier. It
consumes the accepted R1 disruption heatmap, the implemented/pass R2 recursive allocation report,
and the ATLAS-BATCH-0 owner/channel layout. It resolves Terran/Pirate faction capability rows into
bounded read-side modifier overlays and masks them down by owner column. It adds no occupant
movement, reparenting, BoundaryRequest, SEAD action, GradientXY consumption, combat resolution, R4,
R5, R6, R7, new op, shader/WGSL, GPU requirement, default SimSession wiring, CPU planner, hard
currency/markets/trade/ai_budget, ClauseThing, UI, realtime loop, or invariant edit.

## Upstream Contracts Consumed

- R1 report consumed through `DressRehearsalR1Report`.
- R1 canonical checksum: `17de0080304b3da7`.
- R1 CPU oracle parity: true.
- R2 report consumed through `DressRehearsalR2Report`.
- R2 canonical checksum: `4fe0590589ddd975`.
- R2 CPU oracle parity: true.
- ATLAS-BATCH-0 owner/channel layout consumed for galactic co-location owner-mask evidence.

## Capability Trees

Terran faction SimThing `faction-terran` carries:

- `terran-patrol-suppression-doctrine` -> `patrol_suppression_multiplier` at 12000 bps.
- `terran-disruption-resistance` -> `disruption_decay_multiplier` at 11000 bps.
- `terran-defensive-logistics` -> `defensive_logistics_bonus` at 11000 bps.
- `terran-combat-bonus-placeholder` -> `combat_bonus_placeholder` at 10500 bps.

Pirate faction SimThing `faction-pirate` carries:

- `pirate-disruption-emission-doctrine` -> `pirate_emission_multiplier` at 12500 bps.
- `pirate-blockade-efficiency` -> `blockade_divert_multiplier` at 15000 bps.
- `pirate-raiding-logistics` -> `raiding_logistics_bonus` at 11000 bps.
- `pirate-combat-bonus-placeholder` -> `combat_bonus_placeholder` at 11500 bps.

All modifiers are clamped to the bounded 5000-20000 bps range and are read-side only.

## Resolved Modifier Overlays

| owner | modifier | source capability | multiplier_bps |
|---|---|---|---:|
| Terran | combat_bonus_placeholder | terran-combat-bonus-placeholder | 10500 |
| Terran | defensive_logistics_bonus | terran-defensive-logistics | 11000 |
| Terran | disruption_decay_multiplier | terran-disruption-resistance | 11000 |
| Terran | patrol_suppression_multiplier | terran-patrol-suppression-doctrine | 12000 |
| Pirate | blockade_divert_multiplier | pirate-blockade-efficiency | 15000 |
| Pirate | combat_bonus_placeholder | pirate-combat-bonus-placeholder | 11500 |
| Pirate | pirate_emission_multiplier | pirate-disruption-emission-doctrine | 12500 |
| Pirate | raiding_logistics_bonus | pirate-raiding-logistics | 11000 |

## Owner-Mask Application

R3 applies modifiers by owner/faction column, not by spatial parentage:

| source | owner | kind | cell | modifier | group |
|---|---|---|---:|---|---|
| terran-patrol-02 | Terran | patrol_fleet | 154 | patrol_suppression_multiplier | canonical-r1-owner-mask |
| pirate-ship-00 | Pirate | pirate_fleet | 284 | pirate_emission_multiplier | canonical-r1-owner-mask |
| system-0 | Terran | system | 322 | defensive_logistics_bonus | canonical-r2-owner-mask |
| system-10 | Pirate | system | 284 | raiding_logistics_bonus | canonical-r2-owner-mask |
| system-10 | Pirate | production-flow | 284 | blockade_divert_multiplier | canonical-r2-owner-mask |

Co-located owner-separated evidence on the same galactic cell:

| source | owner | kind | cell | x | y | modifier |
|---|---|---|---:|---:|---:|---|
| r3-colocated-terran-patrol | Terran | patrol_fleet | 284 | 4 | 14 | patrol_suppression_multiplier |
| r3-colocated-pirate-ship | Pirate | pirate_fleet | 284 | 4 | 14 | pirate_emission_multiplier |

The same spatial cell therefore holds distinct owner-modified entries. The owner column chooses the
modifier; the spatial parent remains `galactic-location-0`.

## Modified Signal Evidence

R1-style source rows:

| source | owner | channel | cell | base | modifier | bps | effective |
|---|---|---|---:|---:|---|---:|---:|
| terran-patrol-02 | Terran | PatrolSuppression | 154 | -15.000 | patrol_suppression_multiplier | 12000 | -18.000 |
| pirate-ship-00 | Pirate | PirateDisruption | 284 | 20.000 | pirate_emission_multiplier | 12500 | 25.000 |

R2-style economy/read rows:

| signal | owner | source contract | base | modifier | bps | effective | bounded |
|---|---|---|---:|---|---:|---:|---:|
| Terran-stockpile-reduce-up-read | Terran | R2 stockpile ledger reduced_in | 10.000 | defensive_logistics_bonus | 11000 | 11.000 | 11.000 |
| Pirate-stockpile-reduce-up-read | Pirate | R2 stockpile ledger reduced_in | 3.000 | raiding_logistics_bonus | 11000 | 3.300 | 3.300 |
| system-10-blockade-divert-read | Pirate | R2 diverted production row | 1.000 | blockade_divert_multiplier | 15000 | 1.500 | 1.500 |

`combat_bonus_placeholder` resolves as data only. It is not consumed by HP/Damage or combat
resolution in R3.

## Identity Evidence

- Capability tree checksum before and after: `c35f422d89e3a4af`.
- Occupant positions before and after R3 are equal.
- All owner-mask rows keep `structural_parent_before == structural_parent_after`.
- No BoundaryRequest is emitted.
- No SEAD action is emitted.
- No GradientXY is consumed.
- No combat event or HP delta is emitted.

## Artifact Summary

| field | value |
|---|---:|
| capability rows | 8 |
| modifier rows | 8 |
| owner-mask rows | 29 |
| modified R1 signal rows | 13 |
| modified economy rows | 3 |
| stable checksum | 28afb4a204d101d2 |
| CPU oracle parity | true |

## Test Commands And Results

`cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down`

Result: 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation`

Result: 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap`

Result: 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store`

Result: 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu`

Result: 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo check --workspace`

Result: PASS. Existing warnings remain in simthing-core (`EmlTreeMeta` deprecation / unused
`EmlConsumerKind`) and simthing-driver (`RF_CONTINUED_STATIC_512` unused import). No R3 error.

## Skipped Tests

None. STORE-GPU was available in this environment and passed.

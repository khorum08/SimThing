# SCENARIO-0080-2-R2-IMPL-0 - Recursive Allocation + Faction Economy + Blockade/Divert Report

**Verdict:** PASS
**Date:** 2026-06-04
**Gate:** R2 - Recursive allocation + faction economy + blockade/divert
**Implementation:** SCENARIO-0080-2-R2-RECURSIVE-ALLOCATION

## Files Touched

- crates/simthing-driver/src/dress_rehearsal_r2_recursive_allocation.rs
- crates/simthing-driver/tests/dress_rehearsal_r2_recursive_allocation.rs
- crates/simthing-driver/src/lib.rs
- docs/tests/scenario_0080_2_r2_recursive_allocation_report.md
- docs/design_0_0_8_0_consumer_pulled_production_track.md
- docs/scenarios/scenario_0080_2_r2_recursive_reduce_opening_spec.md
- docs/worklog.md
- docs/workshop/mapping_current_guidance.md

## Scope Confirmation

R2 only. The implementation is an opt-in/default-off fixture over the single galactic tier. It consumes
the accepted R1 disruption heatmap, implements M1 labor-to-production, M2 owner-masked reduce-up plus
deterministic disburse-down, and M3 blockade/divert as a production owner-column flip. It adds no
occupant movement, reparenting, BoundaryRequest, SEAD/GradientXY pathing, combat, new op, shader/WGSL,
GPU requirement, default SimSession wiring, CPU planner, market/currency layer, ClauseThing, UI, or
invariant edit.

## R1 Input Contract

- R1 report consumed through `DressRehearsalR1Report` structures.
- R1 canonical checksum: `17de0080304b3da7`.
- R1 `final_disruption` cell count: 400.
- R1 CPU oracle parity: true.
- R2 does not rebuild R1 recurrence, diffusion, or source semantics.

## M1 Production Economy

Each system gets one local pop/factory fixture row:

- pop cohort emits `+10` labor/tick through the IntrinsicFlow posture;
- factory recipe shape is `ConjunctiveCrossing(labor)` with `CrossingFormula{unit_cost:10}`;
- `SubtractFromAllInputs` consumes 10 labor;
- each system produces 1 production and leaves 0 local labor.

## M2 Recursive Allocation

Production reduces up into two owner-masked faction stockpiles, never through a blind owner merge:

| owner | before | reduced_in | after_reduce_up | disbursed_down | after_disburse_down |
|---|---:|---:|---:|---:|---:|
| Terran | 0 | 10 | 10 | 6 | 4 |
| Pirate | 0 | 3 | 3 | 2 | 1 |

Disburse-down is one deterministic subsidiarity sweep to starport deficits:

| owner | system | requested | disbursed | remaining_deficit |
|---|---|---:|---:|---:|
| Terran | system-0 | 2 | 2 | 0 |
| Terran | system-4 | 2 | 2 | 0 |
| Terran | system-8 | 2 | 2 | 0 |
| Pirate | system-10 | 2 | 2 | 0 |

## M3 Blockade/Divert

R2 reads R1 `final_disruption` at each system cell. The canonical pirate starport cell `system-10`
has disruption `100.000`, is blockaded, gates normal original-owner outflow to 0, and emits a diverted
production row to the blockader owner column:

| system | original_owner | blockader | before_owner_col | after_owner_col | production | parent_before | parent_after |
|---|---|---|---|---|---:|---|---|
| system-10 | Pirate | Pirate | Pirate | Pirate | 1 | galactic-location-0 | galactic-location-0 |

The cross-owner owner-column flip is proved by
`r2_divert_flips_production_owner_column_to_blockader`: a Terran system receives a R1-produced
pirate-channel disruption value of 100 and its production owner column changes from Terran to Pirate.
`r2_divert_is_owner_column_flip_not_reparenting` confirms the structural parent remains
`galactic-location-0`.

## Identity Evidence

- Occupant positions before and after R2 are byte-for-byte equal.
- No BoundaryRequest is emitted.
- No combat event or HP delta is emitted.
- No system interior 10x10 subtiles are materialized; R2 remains single galactic tier.
- No default SimSession pass graph is changed.

## Artifact Summary

| field | value |
|---|---:|
| system count | 13 |
| total production | 13 |
| diverted systems | 1 |
| total diverted production | 1 |
| total disbursed | 8 |
| stable checksum | 4fe0590589ddd975 |
| CPU oracle parity | true |

## Artifact Excerpt

### System Production Rows

| system | owner | effective_owner | cell | disruption | blockaded | labor | consumed | production | diverted | disbursed |
|---|---|---|---:|---:|---|---:|---:|---:|---:|---:|
| system-0 | Terran | Terran | 322 | 0.000 | false | 10 | 10 | 1 | 0 | 2 |
| system-4 | Terran | Terran | 338 | 0.000 | false | 10 | 10 | 1 | 0 | 2 |
| system-8 | Terran | Terran | 154 | 0.000 | false | 10 | 10 | 1 | 0 | 2 |
| system-10 | Pirate | Pirate | 284 | 100.000 | true | 10 | 10 | 1 | 1 | 2 |
| system-11 | Pirate | Pirate | 192 | 0.000 | false | 10 | 10 | 1 | 0 | 0 |
| system-12 | Pirate | Pirate | 196 | 0.000 | false | 10 | 10 | 1 | 0 | 0 |

### Top Affected Systems

| rank | system | original_owner | effective_owner | disruption | diverted | disbursed |
|---:|---|---|---|---:|---:|---:|
| 1 | system-10 | Pirate | Pirate | 100.000 | 1 | 2 |
| 2 | system-0 | Terran | Terran | 0.000 | 0 | 2 |
| 3 | system-4 | Terran | Terran | 0.000 | 0 | 2 |
| 4 | system-8 | Terran | Terran | 0.000 | 0 | 2 |
| 5 | system-1 | Terran | Terran | 0.000 | 0 | 0 |

## Test Commands And Results

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
`EmlConsumerKind`) and simthing-driver (`RF_CONTINUED_STATIC_512` unused import). No new R2 error.

## Skipped Tests

None. STORE-GPU was available in this environment and passed.

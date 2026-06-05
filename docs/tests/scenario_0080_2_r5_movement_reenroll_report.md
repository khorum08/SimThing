# SCENARIO-0080-2-R5-IMPL-0 — Movement / REENROLL Report

**Verdict:** PASS  
**Date:** 2026-06-04  
**Gate:** R5 — Movement via BoundaryRequest + REENROLL + mobility substrate  
**Implementation:** SCENARIO-0080-2-R5-MOVEMENT-REENROLL

## Files Touched

- `crates/simthing-driver/src/dress_rehearsal_r5_movement_reenroll.rs`
- `crates/simthing-driver/tests/dress_rehearsal_r5_movement_reenroll.rs`
- `crates/simthing-driver/src/lib.rs`
- `docs/tests/scenario_0080_2_r5_movement_reenroll_report.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`

## Scope Confirmation

R5 only. Opt-in/default-off fixture over the single galactic tier. Consumes implemented/pass R1–R4 contracts
(checksums pinned). Materializes `BoundaryRequest` rows only from R4 `StepOpportunity` rows that passed the
R4 threshold/event posture. Routes movement through `compose_mobility_runtime0` (ALLOC → REENROLL → IDROUTE →
ECON → OWNER). Starport→Fleet spawn uses `plan_mobility_alloc0` arrival (ALLOC) gated on R2 starport production.
No direct movement command, external `BoundaryRequest`, CPU planner, semantic WGSL/new shader, global default
schedule, SimSession pass-graph wiring, R6 combat, R7 closeout, hard currency, or invariant edit.

## Upstream Checksums

| rung | checksum |
|---|---:|
| R1 | `17de0080304b3da7` |
| R2 | `4fe0590589ddd975` |
| R3 | `28afb4a204d101d2` |
| R4 | `f0acbe2ccb98badb` |

## R4 Decisions Consumed

- `SitStill` movers → sit-still rows only; **no** `BoundaryRequest`.
- `StepOpportunity` movers with threshold passed → deterministic event + bounded `BoundaryRequest` + REENROLL move.
- R5 does **not** recompute the R4 composite field; destinations are taken from R4 `candidate_target_cell_index`.
- **R4 spatial-bias note:** R7 emergence narrative must not attribute movement to the R4 canonical tie-breaker bias.
  R5 consumes R4 decisions as the authority and does not re-derive movement from the bias field.

## BoundaryRequest Materialization

- One bounded request per qualifying `StepOpportunity` (threshold bits from R4 Candidate-F magnitude).
- `materialized_from_r4_step_opportunity = true` for all emitted requests.
- No externally scripted requests.

## Mobility Substrate

- Harness: `compose_mobility_runtime0` with `MobilityRuntime0HarnessConfig::opt_in_test_harness()`.
- Flat-star cell arenas: `parent_id = 0` (`galactic-location-0`), `key_id = cell_index`.
- REENROLL commits departure from source cell arena and arrival into destination cell arena.
- IDROUTE identity lane and OWNER faction columns preserved across moves.
- Structural parent remains `galactic-location-0` (no reparenting).

## Starport→Ship Fission

- Gated on first canonical R2 starport row with `has_starport && production_generated > 0`.
- New Fleet entity enrolled via ALLOC `Arrival` at starport cell; owner overlay recorded on fission row.
- **Status:** implemented/pass (not blocked).

## Artifact Summary

| field | value |
|---|---:|
| movement_row_count | 2 |
| boundary_request_count | 2 |
| sit_still_row_count | 0 |
| fission_row_count | 1 |
| stable checksum | `5308a1eb1b7ae5fb` |
| CPU oracle parity | true |
| mobility_substrate_admitted | true |

## Test Commands (exact)

```text
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

## Excerpt — Movement Row (sample)

| mover_id | source_cell | dest_cell | movement_applied | idroute before/after | owner before/after |
|---|---:|---:|---|---|---|
| pirate-ship-00 | (R4 cell) | (R4 target) | true | lane preserved | faction preserved |
| terran-patrol-02 | (R4 cell) | (R4 target) | true | lane preserved | faction preserved |

## Excerpt — Fission Row

| starport_id | trigger | new_fleet_id | enrolled_cell | fission_applied |
|---|---|---|---|---|
| first starport system | starport_production_generated | dress-rehearsal-r5-fission-fleet-{system} | starport cell | true |

# SCENARIO-0080-2-R7 - Dress Rehearsal Closeout & Claim-Boundary Report

**Verdict:** CLOSED / PASS after R6C integrated-run evidence
**Date:** 2026-06-04
**Gate:** R7 - CLOSE + closeout integrity + claim boundary
**Evidence anchor:** [`scenario_0080_2_r6c_integrated_run_report.md`](scenario_0080_2_r6c_integrated_run_report.md)
**Scope:** Closeout report over the implemented R1->R6C rehearsal. No invariant edit.

---

## 1. Ruling

`SCENARIO-0080-2` closes as a vertical proof and an integrated 100-tick observation run.

The original R7 docs-only closeout was reopened because the ladder had no implementation rung that
assembled R1 through R6B into one mutable world and observed which spec section 8.1 behaviors appeared
over time. R6C now fills that gap:

- one canonical R1/ATLAS seed;
- one mutable galactic-tier world state;
- 100 ticks of R1 disruption, R2 economy/blockade/divert, R3 overlays, R4 field reads, R5 movement,
  R6 combat, and R6B production/fusion;
- write-back of positions, ship counts, stockpiles, and disruption into the next tick;
- deterministic replay checksum `1bba891c779190a4`;
- CPU oracle parity true.

This closes R7 without changing the claim boundaries below.

---

## 2. Strongest Accurate Claim

`SCENARIO-0080-2` proves a vertical SimThing slice, mechanism-by-mechanism and in one integrated
multi-tick harness:

occupant-produced disruption field (R1) -> recursive owner-masked economy with blockade/divert (R2) ->
capability mask-down overlays (R3) -> FIELD_POLICY composite-field consumption with exact magnitude threshold
(R4) -> movement via `Threshold`/`EmitEvent` -> `BoundaryRequest` -> REENROLL on the mobility substrate
(R5) -> local combat as adversarial Resource Flow with emission-band ship attrition (R6/R6A) -> ship
production as threshold emission into fleet-cohort reinforcement/fusion (R6B).

Every stage reads the prior stage's emitted state in the integrated run. No bespoke combat, economy,
movement, or fleet-manager subsystem was introduced.

Required qualifiers:

- Single galactic tier, 13 systems on the 20x20 canonical field.
- Opt-in/default-off; no global default `SimSession` wiring.
- CPU oracle is the determinism reference.
- "Pathfinding" remains greedy local FIELD_POLICY movement, not route search or lookahead.
- R6C is GPU-conformant, but its GPU execution has not been measured.

---

## 3. Section 8.1 Emergence Detector Table

| Detector | Classification | First tick | Evidence |
|---|---:|---:|---|
| Pirate raiding waves toward weakly defended, high-value Terran systems | Emerged | 0 | `pirate_distinct_destinations=28` |
| Self-disruption migration | Partially emerged | 0 | Pirate field includes live disruption penalty and clean-target attraction |
| Patrol response to disruption | Not observed | - | Terran field consumed disruption and defensive logistics overlays, but no patrol-response detector fired |
| Patrol interception or co-location caused by movement | Emerged | 44 | R6 combat rows require moved-cell membership |
| Race equilibrium between Terran production and pirate attrition | Partially emerged | 49 | `race_curve_samples=101`; Terran grows from 3 to 7 ships while Pirates grow from 10 to 12 |
| Blockade/divert affecting economy over time | Emerged | 2 | R2 owner-column flip recomputed from live disruption and fleet positions |
| Combat caused by movement-produced co-location | Emerged | 44 | Combat rows carry `movement_produced_colocation=true` |
| Fleet attrition as cohort ship loss | Emerged | 44 | `ships_destroyed=floor(hostile_damage_received/hp_per_ship)` |
| Production reinforcing fleets | Emerged | 49 | `reinforcement_rows=4`, `birth_rows=4` |
| Friendly fleet fusion/cohort compaction | Emerged | 0 | `fusion_rows=10` |
| Front/standoff formation or persistent contested region | Partially emerged | 44 | Detector requires repeated movement-produced hostile co-locations |
| Self-sustaining pirate pressure loop | Partially emerged | 2 | Raid -> divert -> production/attrition feedback evidence is present but not a full strategic loop proof |
| Open-ended AI behavior | Not observed | - | R6C has no CPU planner, route search, or policy AI |
| Modder-facing expressibility | Emerged | 0 | R1-R6B appear as row/mask/reduce/disburse/threshold/emission-band traces |

Headline correction from the reopened report: the marquee questions are no longer "not yet run."
Raiding waves, blockade/divert over time, movement-produced combat, attrition, reinforcement, and fusion
now have integrated-run evidence. Patrol response, open-ended AI, and a fully self-sustaining strategic
pressure loop remain bounded claims, not hidden failures.

---

## 4. Numeric-Only Reconciliation

| Prior claim | Status after R6C |
|---|---|
| First-slice runtime R1/R2/R3 heatmap "accepted (numeric)" | Consumption-proven and integrated-run consumed: R1 heatmap feeds R2/R3/R4, then is recomputed from live fleet positions each tick |
| FrontierV1 "FIELD_POLICY route" registered descriptors but did not consume a field to act | Superseded for the field-to-action loop: R4/R5/R6C consume the field and produce movement; still no route search |
| Mapping first-slice heatmap was hand-seeded parity-only | Reconciled: R6C seeds once, then recomputes disruption from live positions for 100 ticks |
| "pathfinding solved" | Downgraded and bounded: movement is greedy local FIELD_POLICY stepping only |
| "economy generalized" | Bounded: multi-tick galactic-tier stockpile/blockade/divert is proven; nested depth remains parked |
| "combat production-ready" | Bounded: Resource-Flow combat proof in the rehearsal harness; production engine integration and GPU execution remain future work |

Remaining parked: system->planet nested depth beyond this galactic tier, sparse-residency scheduler /
M-4A, ClauseScript L3 authoring, hard currency, production default schedule wiring, and measured GPU
execution for R6C.

---

## 5. R4 Tie-Breaker Resolution

The reopened closeout required the richer integrated field to drop the sparse R4 fixture tie-breaker or
prove it dominated by the real field signal. R6C resolves that condition by using the non-degenerate
integrated field and movement rows whose field attribution comes from the composite field and exact
Candidate-F magnitude threshold. The R4 fixture tie-breaker is not the R6C movement claim.

---

## 6. GPU / Substrate Residency

**GPU posture:** `GPU-conformant; GPU execution not yet measured`

R6C uses row/mask/reduce/disburse/threshold/emission-band traces that conform to the GPU-resident
Accumulator posture, and the CPU oracle is the determinism reference used throughout the project. R6C did
not add semantic WGSL, a new shader, a new GPU primitive, or a CPU planner. Its limitation is measurement:
R6C itself has not been executed on the GPU.

Already measured on discrete GPU in this ladder: ATLAS-BATCH-0 STORE-GPU integer bit-exact owner/channel
storage, 10/10 tests.

---

## 7. Per-Rung Status

| Rung | Status | Evidence |
|---|---|---|
| ATLAS-BATCH-0 STORE | PASS | CPU storage shape, 11/11 |
| ATLAS-BATCH-0 STORE-GPU | PASS | GPU owner/channel store parity, 10/10 |
| R1 | PASS | Disruption heatmap, 34/34 |
| R2 | PASS | Recursive allocation + blockade/divert, 13/13 |
| R3 | PASS | Capability mask-down, 13/13 |
| R4 | PASS | FIELD_POLICY field consumption + exact sqrt, 16/16 |
| R5 | PASS | Movement + BoundaryRequest + REENROLL, 17/17 |
| R6/R6A | PASS | Fleet-cohort combat Resource Flow, 25/25 |
| R6B | PASS | Ship production, reinforcement, fusion, 24/24 |
| R6C | PASS | Integrated 100-tick run, 22/22 |
| R7 | CLOSED / PASS | This closeout |

---

## 8. Verification Rollup

Foreground verification on branch `codex/r6c-integrated-run`:

```text
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run              -> 22 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement   -> 24 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage             -> 25 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll            -> 17 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption       -> 16 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down         -> 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation         -> 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap           -> 34 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store             -> 11 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu         -> 10 passed; 0 failed
cargo test -p simthing-spec --test mobility_reenroll0_substrate                      -> 16 passed; 0 failed
cargo test -p simthing-spec --test mobility_runtime0_composition                     -> 23 passed; 0 failed
cargo test -p simthing-driver --test phase_m_frontier_v2_0_closed_loop_consumer      -> 11 passed; 0 failed
cargo check --workspace                                                              -> PASS (pre-existing warnings only)
```

---

## 9. Disposition

`SCENARIO-0080-2` is CLOSED / PASS as a vertical mechanism proof plus integrated observation run. The
human-facing companion report is
[`../gameplay/scenario_0080_2_pirate_gradient_pathfinding_results.md`](../gameplay/scenario_0080_2_pirate_gradient_pathfinding_results.md).

No invariant edit. No default schedule wiring. No CPU planner. No route search. No measured R6C GPU
execution claim.

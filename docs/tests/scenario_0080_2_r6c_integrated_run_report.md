# SCENARIO-0080-2-R6C - Integrated Multi-Tick Run Report

**Verdict:** IMPLEMENTED / PASS
**Date:** 2026-06-04
**Gate:** R6C - Integrated multi-tick run
**Scenario:** `SCENARIO-0080-2`
**Tick count:** 100
**Seed checksum:** `b2fa6b117f989011`
**Stable replay checksum:** `1bba891c779190a4`
**CPU oracle parity:** true
**GPU posture:** `GPU-conformant; GPU execution not yet measured`

---

## 1. Scope Confirmation

R6C assembles the accepted/implemented R1 through R6B dress-rehearsal rungs into one mutable
galactic-tier world and ticks it forward 100 times. It is opt-in/default-off and does not wire the
production `SimSession` default schedule.

Each tick runs:

1. R1 live disruption recurrence from current fleet positions.
2. R2 owner-masked economy, blockade, divert, and stockpile carry-forward.
3. R3 capability overlays.
4. R4 composite-field reads with exact magnitude threshold.
5. R5 greedy local movement through `BoundaryRequest` + REENROLL-shaped membership updates.
6. R6 combat only where movement produced hostile co-location.
7. R6B construction threshold, reinforcement, fleet birth, and compatible friendly fusion.
8. World write-back into the next tick.

No CPU planner, route search, semantic WGSL, new shader, new AccumulatorOp, hard currency, ClauseThing,
UI/realtime loop, or invariant edit was added.

---

## 2. Mutable World Evidence

R6C seeds the canonical ATLAS/R1 world once, then mutates a single in-memory world for 100 ticks.

- Fleet positions carry forward and become the next tick's R1/R4/R5/R6 inputs.
- Ship counts carry forward through combat attrition, production reinforcement, fleet birth, and fusion.
- Terran and Pirate stockpiles carry forward through R2 economy and R6B construction.
- Disruption carries forward through the bounded recurrence and is recomputed from live fleet positions.
- Combat rows are admitted only when R5 movement produced the co-location.

The deterministic replay checksum is stable across repeated runs: `1bba891c779190a4`.

---

## 3. Trace Excerpts

| Tick | Event |
|---:|---|
| 0 | Seeded canonical mutable world |
| 0 | First movement row emitted |
| 2 | First blockade/divert row emitted |
| 44 | First movement-produced combat row emitted |
| 49 | First production reinforcement row emitted |
| 100 | Final state: Terran ships=7, Pirate ships=12, Terran stockpile=356, Pirate stockpile=144 |

---

## 4. Race Curve Samples

| Tick | Terran ships | Pirate ships | Terran stockpile | Pirate stockpile | Blockaded systems |
|---:|---:|---:|---:|---:|---:|
| 0 | 3 | 10 | 0 | 0 | 0 |
| 10 | 3 | 10 | 37 | 13 | 0 |
| 20 | 3 | 10 | 73 | 27 | 0 |
| 30 | 3 | 10 | 109 | 41 | 0 |
| 40 | 3 | 10 | 144 | 56 | 1 |
| 50 | 5 | 11 | 179 | 71 | 1 |
| 60 | 4 | 11 | 215 | 85 | 0 |
| 70 | 4 | 11 | 250 | 100 | 1 |
| 80 | 4 | 11 | 285 | 115 | 1 |
| 90 | 4 | 11 | 321 | 129 | 0 |
| 100 | 7 | 12 | 356 | 144 | 1 |

Interpretation: the production/attrition race is partially visible over the 100-tick canonical run.
Terran production grows its fleet after tick 49, while Pirate pressure continues to produce blockade and
attrition events.

---

## 5. Section 8.1 Detector Table

| Detector | Classification | First tick | Evidence |
|---|---:|---:|---|
| Pirate raiding waves toward weakly defended, high-value Terran systems | Emerged | 0 | `pirate_distinct_destinations=28` |
| Self-disruption migration | Partially emerged | 0 | Pirate field includes a live disruption penalty and clean-target attraction |
| Patrol response to disruption | Not observed | - | Terran field consumed disruption and defensive logistics overlays |
| Patrol interception or co-location caused by movement | Emerged | 44 | R6 combat rows require moved-cell membership |
| Race equilibrium between Terran production and pirate attrition | Partially emerged | 49 | `race_curve_samples=101` |
| Blockade/divert affecting economy over time | Emerged | 2 | R2 owner-column flip is recomputed from live disruption and fleet positions |
| Combat caused by movement-produced co-location | Emerged | 44 | Combat rows carry `movement_produced_colocation=true` |
| Fleet attrition as cohort ship loss | Emerged | 44 | `ships_destroyed=floor(hostile_damage_received/hp_per_ship)` |
| Production reinforcing fleets | Emerged | 49 | `reinforcement_rows=4`, `birth_rows=4` |
| Friendly fleet fusion/cohort compaction | Emerged | 0 | `fusion_rows=10` |
| Front/standoff formation or persistent contested region | Partially emerged | 44 | Detector requires repeated movement-produced hostile co-locations |
| Self-sustaining pirate pressure loop | Partially emerged | 2 | Requires raid -> divert -> production/attrition feedback evidence |
| Open-ended AI behavior | Not observed | - | R6C has no CPU planner, route search, or policy AI |
| Modder-facing expressibility | Emerged | 0 | R1-R6B are represented as row/mask/reduce/disburse/threshold/emission-band traces |

---

## 6. Claim Boundaries

- Movement is greedy local SEAD stepping, not multi-step path search.
- R6C drops the sparse-field R4 tie-breaker claim boundary by using a non-degenerate integrated field and
  exact magnitude threshold as the movement authority.
- Combat is caused by movement-produced co-location in this run; it is still a rehearsal harness, not
  default production engine wiring.
- GPU execution is not measured for R6C. The posture is conformance, not a benchmark or parity claim.

---

## 7. Verification

```text
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run              -> 22 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement   -> 24 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage             -> 25 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll            -> 17 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r4_sead_field_consumption       -> 16 passed; 0 failed
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

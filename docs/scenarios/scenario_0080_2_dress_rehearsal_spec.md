# SCENARIO-0080-2 Dress Rehearsal — Scenario Specification

> **Status: PROVISIONAL scenario spec (2026-06-03, design authority).** Detail behind the rehearsal
> track; the rungs implement it. **Harness (cite on handoff):** `design_0_0_8_0.md` §0 (transient
> constitution); `invariants.md` (Scenario Proof); `design_0_0_8_0_consumer_pulled_production_track.md`
> §12–§12.5 (rehearsal design + retirement map); `workshop/mobility_and_transfer_allocation.md` §11
> (OWNER); `crates/simthing-spec/src/designer_admission/mobility_owner0.rs`. This file is the **concrete
> scenario** §12.x abstracts. Everything is SimThings + properties + overlays + `AccumulatorOp` (§0.1).

---

## 1. Scale — two distinct grids

- **ATLAS-BATCH-0 stress fixture (§12.3):** galactic 100×100, ~1000 stars — proves *batch allocation* of
  the primitive at scale. Static, no economy.
- **Economy rehearsal (this spec):** **13 live systems** (10 Terran + 3 Pirate) on a **20×20 galactic
  grid**, spaced per §2. Same Location/atlas primitive, small live scale so the economy and combat loop
  is legible. Surfaces/subgrids are **10×10** throughout. The grid is **20×20 (not tighter) so the
  galactic-tier heatmap has room for meaningful gradient falloff** between systems (§4.1) — a 13-system
  field on a cramped grid saturates and the gradient carries no direction.

## 2. Topology + placement constraints (deterministic map generator)

```
GameSession (root)
├── Terran faction        (+ techtree)         — stockpile, redistribution, disposition
├── Pirate faction        (+ techtree)         — stockpile, redistribution, disposition
└── WorldStateMap
    └── Galactic starmap (Location, 20×20)
        └── Star system (Location, 10×10)  ×13   ← occupies one galactic cell each
            ├── Starport (building, child of the system gridcell, at center cell (5,5))   [×4]
            └── Planet (Location, 10×10 surface)
                └── Planet surface (10×10)
                    ├── Factory district (building, a surface cell)
                    └── Pop cohort (building, a surface cell)
```

- **Terran:** 10 systems, each with 1 planet → 1 factory + 1 pop cohort.
- **Pirate:** 3 systems, each with 1 planet → 1 factory + 1 pop cohort (parallel economy).
- **Starports:** 3 in Terran systems, 1 in a Pirate system (4 total), each at the **center cell** of its
  star system's 10×10 subgrid; a **child of the star system gridcell** (not on the planet surface).
- **Placement rules (generator constraints, not fixed coords):** Terran systems ≥ **2–4 empty galactic
  cells** apart (room for galactic-tier gradient falloff, §4.1); each Pirate system within **1 empty
  cell** of a Terran system (so raiding is local). Deterministic placement, seeded.
- **Starting fleets (movers):** Pirate **10 ships**; Terran **3 patrol ships**. Sparse occupants,
  REENROLL between cells (R5). **Movement speed: Pirate 3 galactic cells/tick; Patrol 2 cells/tick** —
  as up-to-N greedy SEAD steps per tick (§8), **not** pathfinding.

## 3. SimThing kinds used

- `Location` (field primitive): galactic starmap, star system, planet, planet surface — each a gridded
  Location carrying its 2-D map (§12.2).
- Buildings (dense occupants, fixed): `Custom("Starport")`, `Custom("FactoryDistrict")`,
  `Custom("PopCohort")`.
- Movers (sparse, REENROLL): `Fleet` (patrol / pirate), owner-column distinguishes faction.
- `Faction` owner-entities under GameSession (Terran, Pirate). **No `StarSystem`/`Station`** (deprecated).
- `kind` is the install-time selector only — never a runtime branch (§0.1).

## 4. Arenas (everything is resource flow — §0.3)

| Arena | Source (IntrinsicFlow) | Consumer | Reduces up to |
|---|---|---|---|
| **labor** | Pop cohort: **+10 labor/tick** | Factory district | nets locally (planet/surface) |
| **production** | Factory: **10 labor → 1 production** (recipe) | Starport (100/ship); faction stockpile | Terran/Pirate faction stockpile (climbs) |
| **disruption** | Pirate/patrol presence (BoundedFeedback accumulate/decay) | patrol suppression | starmap heatmap; **gates flow at ≥100 (§6)** |

- **Factory recipe:** `ConjunctiveCrossing`(labor) → `CrossingFormula{unit_cost:10}` → emit `production`,
  `SubtractFromAllInputs` consumes 10 labor per 1 production. Existing AccumulatorOp machinery, no new op.
- **Per-owner channels at a cell:** co-located factory (production channel) + pop cohort (labor channel)
  stay distinct via the OWNER masked reduction (§12.4, EC-A3) — never merged.

### 4.1 Galactic-tier heatmaps (the 20×20 coarse field — why the grid is 20×20)

The galactic starmap (20×20) carries two **diffused heatmap channels**, reduced up from the systems and
spread by the stencil (gradient falloff):
- **`fleet_strength`** — per-owner (Terran / Pirate), the diffused presence×strength of fleets; a Terran
  fleet at a system radiates Terran strength to nearby galactic cells, decaying with distance (horizon H).
- **`disruption`** — the system-level disruption reduced up to the galactic tier (the raid heatmap).

This is the **coarse tier of the multi-resolution field** (§12.2): a fleet reads the galactic gradient of
`fleet_strength`/`disruption` **at its own system cell** to choose *which system* to move toward
(strategic pathing) — Pirate gradients ascend toward weakly-defended, high-value Terran systems; Terran
gradients ascend toward disrupted owned systems. The **diffusion horizon H is the strategic sight radius**
(§12.2): on a 20×20 grid the falloff spans several cells so the gradient actually points somewhere; a
tighter grid saturates. This galactic gradient **composes with** the fine in-system gradient (R4) —
multi-resolution, one local read, no map traversal.

## 5. Starport → ship (production sink + gated fission)

- A Starport carries a **production "need" of `−100 × queued_ship_count`** — every queued fleet adds a
  −100 demand. Production flowing into the starport pays the need down.
- **Multi-ship-per-tick via the OrderBand emission bands** (C-8d emission substrate): on the emission
  band, `CrossingFormula{unit_cost:100}` emits `floor(production / 100)` ships in one tick and subtracts
  `ships × 100` from accumulated production. So a starport sitting on 250 production with ≥2 queued emits
  **2 ships this tick**, carries 50 forward.
- **Ship emission = gated fission** (`instantiation = gated fission`, §11): each emitted ship instantiates
  a new `Fleet` SimThing at the starport's cell, owner-column = the starport's faction, enrolled into the
  disruption / combat / movement arenas. **This pulls the parked E-2B-5 fission-enrollment substrate** —
  add to the retirement map (R5-adjacent).

## 6. Disruption as blockade + production diversion (revised mechanic)

- Disruption accumulates on a location as before (BoundedFeedback). **At `disruption ≥ 100` the location
  is *blockaded*:** its outbound flow (production/labor) is **suppressed** — gated off from its normal
  reduction up the tree.
- **Diversion:** the blockaded location's production does not vanish — it is **diverted to the blockading
  side.** Mechanically, the production-outflow's destination **owner-column flips from owner → blockader**
  for that location (a `Threshold{≥100}`-gated mask on the OWNER masked reduction). A pirate-blockaded
  Terran system's production sums under the **Pirate** owner-column into the **Pirate** stockpile.
- Conformant: it is a threshold gate + an owner-masked re-route of an existing flow — no special-case
  logic, no new op (§0.1/§0.3).

## 7. Faction redistribution (subsidiarity clearinghouse — ECON)

- Each Faction owner-entity collects **surplus** production (Balance ledger) and **disburses to systems
  with a production deficit** (e.g. a starport with unmet ship-need) — the reduce-up / disburse-down
  sweep (§0.2). Pulls the ECON clearinghouse + Balance carryforward substrate.

## 8. Strategic dispositions (SEAD value decisions — R4)

The faction disposition is the masked-down weight vector the movers read off the heatmap (R4):

- **Pirate "fleet-overmatch need":** maintain `pirate_ships − terran_ships ≥ margin`. While ahead, pirates
  weight **raiding** (move toward Terran systems, raise disruption → blockade + divert production) to deny
  the Terran production advantage. Expressed as a faction-level need that biases pirate fleet gradients
  toward high-value, low-patrol Terran systems.
- **Terran "build-while-low-disruption":** weight **suppressing disruption** (patrol toward disrupted
  systems to push them back under 100) **and** out-producing (3 starports, ~3.3:1 production). Terran
  fleets weight toward disrupted owned systems.

**The core tension:** Terran has the production advantage (10 systems, 3 starports, ~10 prod/tick vs
pirate ~3/tick); Pirate has the fleet head start (10 vs 3), the blockade-divert lever, **and a speed
edge (3 vs 2 cells/tick)**. Pirates must raid fast enough to blockade+divert Terran production and hold
overmatch before Terran out-builds them; patrols are slower to respond, reinforcing the raiding lever.

**Movement speed — multi-step movement, NOT multi-step *pathfinding*.** A fleet takes **up to its speed
in greedy SEAD steps per tick** (Pirate 3, Patrol 2, galactic cells). Each sub-step is a **fresh local
gradient read + threshold** at the fleet's current cell (re-enroll → re-evaluate → step, or **stop early**
if the gradient flattens below threshold) — it is **never** a planned route, search, or lookahead
(`multi_step_pathfinding` stays rejected; §0.5 SEAD). This **generalizes the 0080-2 "single step per
tick"** to a per-faction speed. Per-step re-enrollment means a transiting fleet **can be intercepted in
an intermediate cell** (so speed is also exposure). The exact-sqrt gradient magnitude (R4) gates each
sub-step identically.

## 9. Economy reference numbers

| Quantity | Value |
|---|---|
| Pop cohort output | 10 labor / tick |
| Factory conversion | 10 labor → 1 production |
| Ship cost (patrol or pirate) | 100 production |
| Starport need | −100 × queued_ship_count |
| Terran factories / starports | 10 / 3 |
| Pirate factories / starports | 3 / 1 |
| Terran production (all fed) | ~10 / tick |
| Pirate production (all fed) | ~3 / tick |
| Blockade threshold | disruption ≥ 100 |
| Starting fleets | Pirate 10, Terran 3 |
| Pirate fleet speed | 3 galactic cells / tick (≤3 greedy SEAD steps) |
| Patrol fleet speed | 2 galactic cells / tick (≤2 greedy SEAD steps) |
| Terran system spacing | ≥ 2–4 galactic cells apart |

## 10. Rung mapping (which rung proves which part — §12.5)

| Part of this scenario | Rung |
|---|---|
| Topology, surfaces, building placement, atlas | **ATLAS-BATCH-0** |
| Disruption heatmap + **blockade ≥100 + production diversion** | **R1** (+ R1/R2 coupling for divert) |
| Labor/production economy, factory recipe, faction redistribution, starport need | **R2** |
| Faction techtree dispositions / bonuses | **R3** |
| Fleet pathing by disposition (overmatch / suppress) via exact-sqrt gradient | **R4** |
| Fleet movement (REENROLL + mobility) **and starport→ship fission (E-2B-5)** | **R5** |
| Combat resolution when fleets co-locate | **R6** |
| Close + closeout integrity | **R7** |

## 11. Open parameters (confirm before opening a gate)

- ~~Galactic grid size~~ **PINNED: 20×20** (2026-06-03) — for galactic-tier heatmap gradient falloff (§4.1).
- Pirate fleet-overmatch **margin** (how far ahead pirates try to stay).
- Disruption gain/decay rates, patrol suppression-per-ship, **and the galactic-tier diffusion horizon H**
  (sets both raid/suppress tempo and the strategic sight radius, §4.1).
- Whether labor also climbs to the faction (a galaxy-wide labor pool) or strictly nets locally
  (proposed: **labor nets locally; production climbs**).
- Starting ship placement (which cells the 10 pirate / 3 Terran fleets begin in).

# Pirate Gradient Pathfinding - What the Dress Rehearsal Actually Did

*A plain-language results report for SCENARIO-0080-2. Companion to the formal closeout
[`../tests/scenario_0080_2_r7_closeout_report.md`](../tests/scenario_0080_2_r7_closeout_report.md).*

This version records the R6C integrated run, not just the earlier single-pass R1-R6B mechanism chain.
The rehearsal now ticks one mutable world for 100 turns and observes which behaviors actually appear.

---

## The Setup

The map is a 20x20 starmap with 13 systems: 10 Terran and 3 Pirate. Terran owns the larger economy and
three starports; Pirates begin with the larger fleet and a blockade lever. Fleets, systems, factions,
ship cohorts, and grid cells are all represented as SimThings with properties and owner/channel overlays.

Combat, raiding pressure, economy, movement, and shipbuilding all use the same style of substrate:
row-shaped data, owner masks, reduce/disburse flows, threshold crossings, and emitted events. There is no
special combat engine, economy engine, or fleet manager hiding off to the side.

---

## What Happened Over The Run

**1. The raid pressure became a live field.** Pirate fleets produced disruption, patrol fleets suppressed
it, and the bounded recurrence carried that disruption forward. Unlike the first closeout, this was no
longer a one-shot hotspot only: fleet movement changed the next tick's disruption inputs.

**2. The economy reacted every tick.** Production flowed up into owner-separated Terran and Pirate
stockpiles, then down to systems. When disruption crossed the blockade line, production diverted by
owner-column flip to the blockader. The first blockade/divert event appeared at tick 2, and blockaded
systems continued appearing over the run.

**3. Doctrine shaped the field reads.** Terran and Pirate capability overlays changed how fleets valued
disruption, patrol pressure, opportunity, and combat. These overlays remained owner-masked data, not
planner decisions.

**4. Fleets moved by local FIELD_POLICY, not route search.** Each mover read the composite field at its own cell,
used exact magnitude thresholding, and stepped greedily to a neighboring cell when the field was strong
enough. The run produced broad Pirate raiding dispersion: `pirate_distinct_destinations=28`.

**5. Combat came from movement-produced co-location.** The first movement-produced hostile co-location
occurred at tick 44. Combat then ran as Resource Flow: damage reduced up by owner, disbursed down to
hostile cohorts, and converted into whole ships destroyed.

**6. Shipyards changed the fleet counts.** Construction crossed thresholds, reinforced existing compatible
fleets, and birthed new local fleets. The first reinforcement appeared at tick 49. Friendly compatible
fleets also fused into larger cohorts.

**7. The production race became visible but not final.** Terran ships grew from 3 to 7 by tick 100, while
Pirate ships grew from 10 to 12. The curve shows a real production/attrition race, but not a solved
strategic equilibrium.

---

## What Emerged

- Pirate raiding waves toward weakly defended, high-value Terran systems: emerged.
- Blockade/divert affecting economy over time: emerged.
- Movement-produced hostile co-location and combat: emerged at tick 44.
- Fleet attrition as cohort ship loss: emerged.
- Production reinforcement and fleet birth: emerged.
- Friendly fleet fusion/cohort compaction: emerged.
- Modder-facing expressibility: emerged; the whole run is row/mask/reduce/disburse/threshold machinery.

## What Partially Emerged

- Self-disruption migration: the Pirate field penalizes dirty targets and attracts cleaner ones, but this
  is still greedy local stepping.
- Race equilibrium: the race curve is real, but 100 ticks is evidence of pressure, not a final strategic
  balance proof.
- Front/standoff behavior: movement-produced combat appears, but persistent repeated fronts are only
  partial.
- Self-sustaining pirate pressure loop: raid -> divert -> production/attrition feedback is present, but
  not yet a full open-ended loop.

## What Did Not Emerge

- Patrol response to disruption did not trigger as a detector, even though Terran field rows consumed
  disruption and defensive logistics overlays.
- Open-ended AI behavior did not emerge and was not intended to: there is no CPU planner, policy AI,
  route search, or lookahead.
- Production-engine/default-schedule gameplay was not switched on. R6C is an opt-in harness.

---

## The Pathfinding Boundary

"Pathfinding" here means local field-following, not a route planner. A fleet can read the field under its
feet, decide whether the gradient is strong enough, and step to the best neighboring cell. It does not
plot a multi-step route, search around obstacles, or reason about future turns.

That boundary is important and useful: the rehearsal proves field-as-policy movement can produce raiding,
interception, and combat pressure without a planner. It does not claim a general pathfinding system.

---

## Durable Result

The R6C run is the first full SCENARIO-0080-2 rehearsal where the pieces are left running together.
Disruption feeds economy, economy and doctrine feed movement fields, movement changes future disruption
and combat opportunities, combat changes fleet cohorts, and production changes future combat capacity.

The game story is now visible in the substrate, while the claim remains bounded: single galactic tier,
opt-in harness, CPU-oracle primary, greedy local movement, and no measured R6C GPU execution yet.

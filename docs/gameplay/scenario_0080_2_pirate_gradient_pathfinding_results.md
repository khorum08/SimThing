# Pirate Gradient Pathfinding — What the Dress Rehearsal Actually Did

*A plain-language results report for SCENARIO-0080-2. Companion to the formal closeout
[`../tests/scenario_0080_2_r7_closeout_report.md`](../tests/scenario_0080_2_r7_closeout_report.md).*

> **Supersedes the 2026-06-02 pre-rehearsal version of this file.** That earlier report proved the
> disruption recurrence, compound field, dual-output gradient, and SEAD single-step movement at the
> **math / CPU-oracle layer** and explicitly flagged that they were *not yet proven through a full
> SimThing reduction* — naming this dress rehearsal as the successor. The rehearsal (R1–R6B) has now
> run; this version records what it actually demonstrated, and what it did not.

This document describes **what the simulation did**, in order, and is careful to separate **what genuinely
fell out of the rules** from **what was set up by hand to prove a mechanism worked**. Where something did
*not* happen yet, it says so plainly — that is a finding, not a failure to hide.

---

## The setup

A small corner of a galaxy: a **20×20 starmap** with **13 star systems** — 10 Terran, 3 Pirate. Terran
out-produces (10 systems, 3 starports, ~10 production/tick) but the Pirates start with the bigger fleet
(10 ships vs 3 patrols) and a blockade lever. Every actor — the galaxy, the factions, the systems, the
planets, the fleets, even the individual ships inside a fleet — is the **same kind of thing**: a SimThing
with properties and overlays. Nothing in here is a special "combat engine" or "economy engine" bolted on
the side. Combat, trade, raiding, and shipbuilding are all the *same* underlying operation — resources
flowing up and down a tree and crossing thresholds.

That uniformity is the whole point. The rehearsal exists to prove that an entire 4X-flavored vertical —
field, economy, doctrine, pathing, movement, combat, production — can be expressed as one substrate
without ever inventing a bespoke subsystem.

---

## What happened, step by step

**1. The raid lit up the map.** Ten Pirate ships sitting on one system pushed that cell's **disruption**
to its ceiling (100). Patrols, where present, pushed disruption *down*. The disruption spread to
neighboring cells with falloff. → *This is real arithmetic, but it was a hand-placed hotspot — one bright
spot on an otherwise dark map.*

**2. The economy felt it.** Production flowed up from factories into per-faction stockpiles, kept strictly
separate by owner (Terran money never silently merged with Pirate money), then was disbursed back down to
systems that needed it. The blockaded system crossed the disruption line and its production was **diverted
to the blockader** — the Pirates effectively stole a Terran system's output by changing *which owner the
flow counted for*, without moving or reparenting anything. → *Proven, cleanly, at one cell, for one tick.*

**3. The factions brought their personalities.** Terran and Pirate capability trees resolved into
modifier overlays — patrol-suppression doctrine, raiding logistics, a combat bonus — and those overlays
masked down onto the cells and ships of the owning faction only. → *Proven. Later consumed for real by
combat, where the Pirate combat bonus actually changed damage output.*

**4. A fleet read the field and decided.** A fleet looked at the galaxy gradient **at its own cell** — a
blend of disruption, patrol presence, opportunity, and its own faction's doctrine — measured the slope
with an exact, deterministic square-root, and asked one question: *is the pull strong enough to move?* If
yes, it picked the most attractive neighbor. → *This is the closest thing to genuine emergence in the
rehearsal: a real field read and a real threshold decision. With one honest asterisk (see below).*

**5. The fleet actually moved.** A "yes" decision turned into an event, the event into a boundary request,
and the request relocated the fleet — deregistering it from its old cell and enrolling it in the new one —
while keeping its identity and its owner stamp intact. No teleporting, no rewriting the tree. → *Real
movement, driven by the decision in step 4, not by a script.*

**6. Fleets fought as crowds of ships, not as hit-point bars.** Where a Terran fleet and Pirate fleets
shared a cell, combat ran as **resource flow**: each side's damage output (ships × per-ship damage, tuned
by the faction combat bonus) was pooled by owner, aimed only at the enemy, and the incoming damage was
converted into **whole ships lost** — 500 damage against 100-HP ships removes exactly 5 ships. A fleet is
only destroyed when its **last** ship dies, not when some abstract HP bar hits zero. → *The attrition math
genuinely emerges from the flow. The fight itself, though, was staged: the two sides were placed together
to prove the mechanism.*

**7. Shipyards refilled the fleets.** Production accumulated toward a build threshold; crossing it emitted
a **ship-count delta**. That delta found a compatible friendly fleet in the same cell and **grew it** (10
ships → 11, with hit-points-to-kill and damage output recomputed to match), or, if there was no suitable
fleet, **birthed a new one** locally. Two friendly fleets sharing a cell could **fuse** into one larger
cohort (7 + 7 → 14). All of this without any movement order. → *The reinforcement of a real, previously
spawned fleet is a genuine chain; the fusion and birth demonstrations used hand-placed fleets.*

---

## The honest asterisk: the tie-breaker

The canonical test field is **sparse** — essentially one bright hotspot. On a field that flat, a pure
gradient can point nowhere in particular. To keep the movement decision well-defined, R4 adds a tiny
deterministic spatial nudge (a function of the cell's position). **This nudge is a fixture tie-breaker,
not gameplay.** It must never be read as "the Pirates chose to go there because the simulation wanted them
to." A richer map with several competing hotspots should either remove the nudge entirely or prove the
real field signal overwhelms it. Until then, treat fleet *direction* in this rehearsal as "validly
decided" but not "strategically meaningful."

---

## What did **not** emerge (yet)

The marquee questions this scenario was built to eventually ask **were not answered**, because the
rehearsal is a chain of single-pass demonstrations wired together — not a clock ticking forward over many
turns:

- **Raiding waves** sweeping toward soft, rich Terran systems — *not shown.*
- **The race:** does Pirate fleet-overmatch hold, or does the Terran production edge out-build it? — *not
  shown; this needs many ticks.*
- **Interception:** two fleets meeting in transit and fighting because of where movement took them — *not
  shown; the one fight was staged.*
- **Fronts / standoffs** where patrol suppression balances Pirate disruption — *not shown.*
- **A self-sustaining pressure loop** (raid → disrupt → divert → build → raid again) — *not shown.*

None of these are broken. They are simply **not yet exercised**: the parts have each been proven to work;
they have not yet been left running together long enough to produce a story. That is the next scenario's
job — a multi-tick closed loop over a richer field.

---

## What this proved (the durable result)

- A complete vertical **mechanism chain** works, and **each stage genuinely consumes the previous stage's
  real output** — not a stand-in number copied by hand.
- **Combat, economy, disruption, movement, and shipbuilding are all the same substrate** (SimThings +
  overlays + accumulate/threshold/emit). No bespoke engine was added for any of them. For a modder, that
  means a new adversarial or economic system is "more SimThing," not new engine code.
- Fleets are **cohorts of ships**, and both **losing** ships (combat) and **gaining** ships (production)
  are the same kind of threshold-driven count change on that cohort.

This is the substrate doing what the constitution promises. The *game* — the emergent, open-ended pressure
between two factions — is the next thing to switch on, now that every part has been shown to run.

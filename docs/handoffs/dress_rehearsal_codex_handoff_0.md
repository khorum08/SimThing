# Codex Handoff 0 — SCENARIO-0080-2 Dress Rehearsal

**From:** Opus (design authority) · **To:** Codex (implementation) · **Date:** 2026-06-03
**This handoff:** orientation + the first production rung, **`ATLAS-BATCH-0-GEN`** (static map generator).

---

## 0. Read first — the context harness (cite these on every handoff back)

The project recently corrected a real drift (scenarios were proving *math in a vacuum* — never through a
real SimThing reduction; AI never consumed a heatmap). The constitution now has a **transient §0** that
carries forward forever, and a new **"Scenario Proof"** invariant. **Before writing code, read:**

1. **`docs/design_0_0_8_0.md` §0** — transient constitution: conformance; **all conflict is resource
   flow**; recursive allocation; **FIELD_POLICY = GPU-resident threshold crossings, no CPU planner** (§0.0, §0.5
   #4); **§0.5 harness discipline** (how to cite, how to self-check).
2. **`docs/invariants.md`** — **"Scenario Proof"**; **AccumulatorOp v2** + **Resource Flow Substrate**
   sections (the GPU-resident flow contract); FIELD_POLICY/JIT closure-posture.
3. **`docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5** — rehearsal design,
   EC1/EC2, nested-grid hierarchy, ATLAS-BATCH-0, the rung ladder + parked-inventory coverage. (§12.4
   links the OWNER design + parked `mobility_owner0.rs`.)
4. **`docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md`** — the **concrete scenario** (13 systems,
   factory/pop/starport economy, the numbers, disruption-as-blockade) and **§8.1 anticipated emergence**.
5. **`crates/simthing-core/src/accumulator_op.rs`** — the **GPU-resident Accumulator primitive**
   (`SourceSpec`/`CombineFn`/`GateSpec`/`ScaleSpec`/`ConsumeMode`) every arena compiles to.
6. **`docs/workshop/field_policy_track.md`** — the **FIELD_POLICY charter** (field-as-policy; threshold crossings
   → `BoundaryRequest`; no CPU planner).

**Self-check every diff against the §0.5 base principles** and say in one line that it holds them:
(1) everything is a SimThing — no subsystem outside the tree, no runtime `match kind`; (2) all conflict
is resource flow; (3) recursive allocation, emergent depth; (4) GPU threshold crossings, not a CPU
planner; (5) semantic-free `simthing-sim` + CPU-oracle bit-exact parity; (6) proven only through a real
reduction, opt-in/default-off. **If a change can't fit 1–6, stop and escalate — do not special-case.**

---

## 1. The SimThing process we are demonstrating (LOCKED)

The dress rehearsal proves **one** mechanism end-to-end through real SimThings — a per-boundary loop, no
bespoke engines:

```
emit → reduce-up → mask-down → diffuse → threshold → act
```

1. **Emit** — pop cohorts emit labor; factories convert labor→production (recipe); pirate/patrol presence
   emits disruption (BoundedFeedback) into per-owner cell channels.
2. **Reduce up** — labor/production/disruption reduce up the recursive tree (building → surface → planet
   → system → galaxy → faction stockpile); each tier keeps its 2-D map (§12.2).
3. **Mask down** — faction techtree dispositions/modifiers mask **down** by owner-column (OWNER §12.4).
4. **Diffuse** — disruption + fleet_strength diffuse to heatmaps per tier; `GradientXY` extracts the
   gradient; the galactic-tier heatmap (20×20) carries strategic falloff (§4.1).
5. **Threshold (FIELD_POLICY)** — a mover reads the **local** composite gradient at its own cell (patrol-presence
   × disruption × its masked disposition), takes the **exact-sqrt** magnitude, and a threshold crossing
   decides sit / move / engage / raid. **No CPU planner; no lookahead.**
6. **Act** — `Threshold`+`EmitEvent`→`BoundaryRequest`: movers REENROLL between cells; combat resolves at
   co-located cells (HP/Damage arena); starports emit ships (gated fission); disruption≥100 blockades a
   location and diverts its production to the blockader.

**The two hard exit criteria the rehearsal must hit (EC1/EC2):**
- **EC1:** the starmap holds a **non-trivial reduced disruption heatmap over real gridcell SimThings,
  produced by gameplay (not hand-seeded)**, vs a CPU oracle, emitted as an inspectable artifact.
- **EC2:** a mover's **FIELD_POLICY action is a function of the diffused heatmap gradient read at its own cell**,
  vs a CPU oracle — not a hand-seeded field or a registration-only stand-in.

**What we are watching for (emergent, not scripted — scenario §8.1):** pirate raiding waves,
self-disruption migration, patrol redistribution, blockade-divert ownership flips, and the headline
**race equilibrium** — does pirate fleet-overmatch hold or does the Terran ~10:3 production advantage
out-build it? The closing report (R7) documents which behaviors actually emerged.

---

## 2. The rung ladder (where this handoff sits)

Sequence (production track §12.5 retirement map — one parked phase proved per rung):

`Open (scenario admission)` → **`ATLAS-BATCH-0`** [GEN → LOC → PACK → STORE → CLOSE] → `R1` (disruption
heatmap) → `R2` (recursive economy) → `R3` (capability-tree) → `R4` (FIELD_POLICY + exact sqrt) → `R5` (movement
+ fission) → `R6` (combat) → `R7` (close + report).

**Opus authors/adjudicates each gate; Codex develops the IMPL rungs.** This handoff is the first IMPL rung.

---

## 3. THIS HANDOFF — `ATLAS-BATCH-0-GEN` (static map generator)

**Goal:** a **deterministic, seeded** generator that produces the fixed dress-rehearsal map **as a data
descriptor** (a fixture). **No GPU. No engine wiring. No SimThing instantiation. No economy. No
arenas.** Pure topology data the next rung (`ATLAS-BATCH-0-LOC`) turns into Location SimThings.

**Produce (per scenario spec §1–§3, §9):**
- Galactic grid **20×20**.
- **13 systems:** 10 Terran + 3 Pirate, placed deterministically with **Terran systems ≥ 2–4 empty
  galactic cells apart**, each **Pirate system within 1 empty cell of a Terran system**.
- Per system: a **planet** position (in the 10×10 system subgrid) with a **10×10 planet surface**; on the
  surface, a **factory district** cell and a **pop cohort** cell.
- **Starports:** 3 of the Terran systems and 1 Pirate system each get a starport at the **center cell
  (5,5)** of the system's 10×10 subgrid.
- **Starting fleets:** **10 pirate ships, 3 patrol ships**, placed deterministically at their owner's
  systems (proposed: at the faction's starport systems).
- Carry owner identity (Terran/Pirate) on every produced element as a plain field (owner-column comes later).

**Shape:** a `DressRehearsalMap` descriptor (galaxy dims; `Vec` of systems; per system its kind/owner,
galactic cell, planet/surface/building/starport cells; fleet placements). Plain Rust data + a seeded
constructor. Where it should live: a new module under `simthing-driver` (test/fixture support), not the
production binary.

**Exit criteria (EC-A1):**
- **Deterministic:** same seed → identical descriptor (assert two builds equal).
- **Constraints hold:** tests assert 13 systems (10+3), the spacing rules (≥2–4 apart; pirate within 1),
  counts (13 planets/factories/pops, 4 starports, 10+3 fleets), starport center placement, surfaces 10×10.
- **One test report** under `docs/tests/` with the §0.5 posture line; **one status-row** update.

**Stop conditions (escalate to Opus, do not implement around):**
- Needs the `Location` SimThing kind / slot allocation / GPU / atlas batching → that's the **LOC/PACK**
  rungs, not GEN. GEN emits *data only*.
- Any economy, arena, diffusion, movement, or owner-column logic → later rungs.
- Any `match kind`-driven behavior, semantic WGSL, default-on wiring, or CPU planner → prohibited (§0.5).
- Open params you do **not** need for GEN (overmatch margin, disruption/decay rates, diffusion horizon H,
  labor termination) — leave them out; they belong to R1–R5.

**Cite on handoff back:** the 6 harness links (§0) + the one-line self-check against §0.5 1–6.

---

*Next after acceptance:* `ATLAS-BATCH-0-LOC` — turn this descriptor into `Location` gridcell SimThings
with grid-placement slot allocation and the multi-channel cell (Opus authors the LOC contract).

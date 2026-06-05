# SCENARIO-0080-1 — Nested Starmap (Terran / Pirate Multi-Theater) Admission Packet

> **Status: ACCEPTED (design authority + product, 2026-06-02). OPENING = docs/design only; NO IMPLEMENTATION.**
> - This is the **second** 0.0.8.0 consumer-pulled scenario, after Local Patrol Economy (`SCENARIO-0080-0`, COMPLETE/PARKED).
> - It is the **named multi-theater scenario** the parked atlas gate's park-condition required, so it
>   deliberately opens **`ATLAS-0080-0`** (atlas production runtime / sparse-residency).
> - The pirate is admitted as a **full economy faction**, so it deliberately opens **`ECON-SCALE-0080-0`**
>   (Hybrid-Strata / faction-index ECON scaling).
> - All session wiring is **opt-in / default-off**. Default-on is a separate Tier-2 decision, not in scope.

---

## 1. Consumer pull

A bounded, **nested multi-theater** map in which **faction AI decisions derive from inherited composed
personality/policy overlays**, and **two factions (Terran patrol + Pirate) contend in resource flow**
across theaters. This single playable theater names, as its wiring, two parked substrates (atlas
sparse-residency nested mapping; multi-faction contended economy) and instantiates the inherited-overlay
decision model worked out in the design conversation of 2026-06-02.

It exercises capabilities Local Patrol Economy did **not**: nested multi-theater structure, atlas
residency, decision weights **sourced from broadcast faction overlays** (not local constants), ownership
**up-aggregation**, and **multi-faction adversarial resource flow**. It is a new track, not an extension
of the closed slice.

---

## 2. World structure (bounded)

- A **10×10 starmap** location simthing, added as a child of the game session simthing (opt-in).
- **10 of its grid cells (chosen by a deterministic seed)** are **starsystems**, each with its own **10×10
  subfield** of grid cells.
- **Each starsystem** has exactly **one** of its grid cells designated a **planet**, with its own **10×10
  submap**.
- Total ≈ 100 (starmap) + 10×100 (starsystem subfields) + 10×100 (planet submaps) ≈ **2,100 location
  simthings** — bounded and tractable; the atlas need is **sparse residency + nested theater
  management**, not raw cell count.
- **Faction owner simthings** (Terran, Pirate) are attached to the game session as **siblings of the
  starmap** location simthing — i.e. owner-relations, **not** spatial parents.

---

## 3. Ownership model

- Each location simthing carries an **owner overlay**.
- The owner overlay **inherits personality/policy weights** broadcast **down** from the owning faction's
  owner simthing via the proven **OWNER latched-modifier-overlay down-broadcast** substrate.
  "Personality" and "policy" are **authored overlays of numeric weights** (no `simthing-sim`
  `Personality`/`Gadget` type — invariant respected); they compose (`personality ⊕ policy`) by authored
  linear arithmetic.
- **Ownership up-aggregation (new rule):** if a starsystem's **planet** is owned by the Terran (patrol)
  faction, the **starsystem** is likewise owned by that faction. This is implemented as a **derived owner
  overlay** (the starsystem's owner = a function of its owned planet's owner) — **not** spatial
  reparenting and **not** owner-entity-as-spatial-parent (both remain stop conditions).

---

## 4. Factions & adversarial resource flow

- **Terran** = the patrol faction (continuity with the Local Patrol Economy patrol posture).
- **Pirate** = admitted here as a **full economy faction** (per product decision 2026-06-02), not merely a
  disruptor identity. When a pirate **enters a starsystem's grid**, it becomes an **adversarial
  participant in that starsystem's resource flow** — a contended, multi-faction economy interaction.
- This adversarial multi-faction resource flow is what opens **`ECON-SCALE-0080-0`** (Hybrid-Strata /
  faction-index ECON scaling), previously parked.

### 4.1 Initial conditions (this scenario instance)

Deterministically seeded; the 10 starsystems are the 10 starmap cells chosen by the scenario seed.

- **Star ownership:** **6 of the 10 stars are owned by the Terran (patrol) faction; the other 4 are
  neutral (unowned).** A star is Terran-owned because its planet is Terran-owned, propagated up by the
  ownership up-aggregation rule (§3); neutral stars have unowned planets.
- **Terran faction:** owns the 6 Terran stars; fields **3 Terran ships**, each initially sited at **3
  distinct stars chosen from its 6** (the remaining 3 Terran stars start without a ship).
- **Pirate faction:** **owns no stars** — it owns **only its 3 pirate ships**. Each **pirate ship starts
  at a distinct neutral star** (3 of the 4 neutral stars host a pirate ship at start).
- **Ships are mover simthings** with an owner overlay inheriting their faction's personality/policy
  weights (Terran ships ← Terran owner simthing; pirate ships ← Pirate owner simthing). A pirate ship
  that **enters a starsystem's grid** becomes an adversarial participant in that starsystem's resource
  flow (§4).

| Faction | Stars owned | Ships | Ship start locations |
|---|---|---|---|
| Terran (patrol) | 6 of 10 | 3 | 3 distinct Terran stars (of its 6) |
| Pirate | 0 | 3 | 3 distinct neutral stars (of the 4) |

---

## 5. Decision model (instantiates the 2026-06-02 design conversation)

Faction movers' decisions are produced by the existing **GPU-resident FIELD_POLICY frontier**
(`Threshold`+`EmitEvent`→`BoundaryRequest`) evaluated over a **composite gap-vector** state:
- per-channel gap = `current(space) − inherited_setpoint(space)` (setpoints arrive via the broadcast
  faction overlay);
- the composite is the **sum of the gap vectors** (e.g. supply/security gap + a bilateral relational
  gap); the resultant is the gradient the frontier evaluates.
- The composite and its **component terms are observable read-only** (the pattern the pirate score +
  `observe_gameplay_0080_0` already demonstrate).
- **Player orders, if later admitted, are a weighted overlay term on the action vector** — never a
  direct-move and never the hard-currency mechanism. Deferred to a later sub-slice.

No CPU planner / urgency / commitment; movement remains FIELD_POLICY-sourced.

---

## 6. Gates this scenario opens (docs/design; no implementation)

| Gate | What | Status after this PR |
|---|---|---|
| `SCENARIO-0080-1` | This scenario | **ACCEPTED** |
| `ATLAS-0080-0` | Atlas production runtime / sparse-residency nested mapping | **OPEN — docs/design gate; no implementation**. Spec: [`../production_paths/atlas_0080_0_opening_spec.md`](../production_paths/atlas_0080_0_opening_spec.md) |
| `ECON-SCALE-0080-0` | Multi-faction (Hybrid-Strata/faction-index) ECON scaling for adversarial resource flow | **OPEN — docs/design gate; no implementation**. Spec: [`../production_paths/econ_scale_0080_0_opening_spec.md`](../production_paths/econ_scale_0080_0_opening_spec.md) |
| `PRODUCTION-PATH-0080-1` | The scenario's opt-in production path (nested structure + owner overlays + inherited-overlay decision + adversarial RF) | **NOT YET OPENED** — opens only after the two substrate gates above have opening specs accepted |

**Invariant note:** opening `ATLAS-0080-0` is exactly the *named first slice* the invariant
*"No production mapping runtime without first-slice gating"* contemplates. The starmap wires into the
game session **opt-in / default-off** only; no default session pass-graph wiring (that would be Tier-2).
No invariant edit is required or made.

---

## 7. Bounds & stop conditions

Opt-in/default-off; bounded grid sizes (10×10 at each level, 10 starsystems, one planet each); deterministic
seeding and replay; **no** real-time loop; **no** UI framework; **no** player command loop (player orders,
if later, are a weighted overlay term); **no** direct movement control or externally-scripted boundary
request; **no** semantic/raw WGSL or new shader authored as semantic; **no** CPU planner/urgency/commitment;
**no** capture-as-reparenting; **no** owner-entity-as-spatial-parent; **no** ClauseThing; **no**
`simthing-spec` alteration for ClauseThing; **no** invariant edit; **no** global default schedule;
**no** default-on session wiring. Reversible. Crossing any line is a stop-and-escalate.

---

## 8. Sequence (proven scenario-first ladder)

1. **This PR (docs-only):** scenario admission + `ATLAS-0080-0` and `ECON-SCALE-0080-0` opening specs.
2. `ATLAS-0080-0` implementation (sparse-residency nested mapping, opt-in) — separate authorized PR.
3. `ECON-SCALE-0080-0` implementation (multi-faction contended RF, bounded) — separate authorized PR.
4. `PRODUCTION-PATH-0080-1` opening + implementation (nested structure + owner overlays + inherited-overlay
   decision + adversarial RF) — separate authorized PRs.
5. Schedule / observation / (optional) control / demo — reusing the proven `0080-0` pattern.
6. Closeout review.

---

## 9. Pointers
- Active constitution: [`../design_0_0_8_0.md`](../design_0_0_8_0.md)
- Production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Prior scenario (closed): [`scenario_0080_0_admission_packet.md`](scenario_0080_0_admission_packet.md); closeout: [`../tests/phase_local_patrol_economy_0080_closeout_results.md`](../tests/phase_local_patrol_economy_0080_closeout_results.md)
- Atlas opening spec: [`../production_paths/atlas_0080_0_opening_spec.md`](../production_paths/atlas_0080_0_opening_spec.md)
- ECON-scale opening spec: [`../production_paths/econ_scale_0080_0_opening_spec.md`](../production_paths/econ_scale_0080_0_opening_spec.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_scenario_0080_1_opening_review_results.md`](../tests/phase_scenario_0080_1_opening_review_results.md)

# 0.0.8.4 — ClauseScript Terran-Pirate Galaxy Production Track

> **Status: OPEN (production track, opened 2026-06-28, owner-authorized maximal scope).** Sits
> *beneath* [`simthing_core_design.md`](simthing_core_design.md) (permanent paradigm) and *beneath*
> the active constitution [`design_0_0_8_3.md`](design_0_0_8_3.md). This document is the authoritative
> 0.0.8.4 production design track and PR ladder. It opens the **first full-scenario native ClauseScript
> ingestion vertical**: a single authored `.clause` file that transpiles entirely to canonical
> `SimThingScenarioSpec` and runs live, STEAD-driven, through indefinite ticks in SimThing Studio.
>
> Detailed per-rung evidence lands in `docs/tests/*_results.md` and
> [`docs/tests/current_evidence_index.md`](tests/current_evidence_index.md).

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable, load-bearing — hold all of these in context every rung):**

1. [`design_0_0_8_3.md`](design_0_0_8_3.md) **§0** — the transient constitution (carry-forward doctrine, anti-flattening §0.6, exact-magnitude §0.7, STEAD §0.8, closed-lowering-layer gates §A).
2. [`simthing_core_design.md`](simthing_core_design.md) — the permanent paradigm (the one tree §2, RF arenas §5, overlays §6, the Movement-Front automaton §7, decisions-as-thresholds §8, drift detectors §9).
3. **This file** — the 0.0.8.4 canonical design file.
4. [`clausething/ClauseThing_Spec.md`](clausething/ClauseThing_Spec.md) §4 (the deep correspondence table) + §8 (sequencing hard problems) — the ClauseScript→SimThing isomorphism and its known gaps.
5. [`clausething/ct_2c_economic_category_memo.md`](clausething/ct_2c_economic_category_memo.md) — the modifier-key decoder, the `_mult`/`_add` inheritance asymmetry, and the movables doctrine (fleets/cohorts/leaders are ordinary SimThings).
6. [`stead_spatial_contract.md`](stead_spatial_contract.md) — the 8 STEAD invariants (mandatory for any MapGen / Location-grid / Movement-Front / Gu-Yang / PALMA / RF-over-gridcell work).

A rung handoff may add ≤3 rung-local links it directly consumes; rung-local links never accrete into this base.

**Established decisions — do NOT re-derive (one screen):**

- **Everything is a SimThing.** Owners are GameSession siblings, never spatial parents. Ownership = owner-columns + permanent identity overlays. Capture = column flip, never reparenting. (core §2, §6)
- **All conflict/opportunity/ambition/diplomacy is resource flow:** `accumulate → reduce up → settle locally → mask/disburse down → threshold crossings fire decisions`. No combat engine, no economy engine, no diplomacy engine, no AI engine. (core §1, §5; constitution §0.3)
- **Combat is HP/Damage economics** (constitution §0.3, core §5.3): damage = `SubtractFromSource` transfer; HP recovery = `governed_by`; death = a zero-crossing `Threshold` → `EmitEvent` → `BoundaryRequest`. **Owner bonuses disburse down as overlays on the HP/Damage columns.**
- **Diplomacy is influence/distrust economics** (core §2.1, §5.3): trust/influence is an ordinary flowing quantity reduced up and disbursed down; a stance change is a registered threshold crossing on the reduced trust/distrust column. No diplomacy subsystem.
- **Hull designs / ship classes are capability trees** (ClauseThing_Spec §4): one `Custom(...)` SimThing; progress = sub-fields; unlock = `Suspended`→`Permanent` overlay; cost = flow drain. Never a runtime `match kind`.
- **Decisions are GPU-resident threshold crossings — FIELD_POLICY, never a CPU planner** (core §8). The AI reads the front and acts when a personality-weighted pressure column crosses a named threshold. The CPU only consumes structural events at boundaries.
- **The map is the Movement-Front automaton** (core §7): bounded-horizon falloff (P1), one shared stencil (P2), attractor/threshold projection (P3). Gu-Yang `SaturatingFlux` for borders/chokes; PALMA for reach/impedance. The front *is* the route; fleet movement is gradient-following reparenting, never a route solver.
- **`simthing-sim` and WGSL are semantic-free.** ClauseScript never crosses into the sim crate; every modifier/category/trigger compiles away at hydration/admission. After ingestion, `grep` for "terran"/"pirate"/category names in any runtime artifact comes up empty (authoring ids/display strings only). (ClauseThing_Spec §7)
- **Exact magnitude gates route through Candidate F** (constitution §0.7). Native `sqrt`/`length`/`distance` are `ApproximateDiagnostic` only.
- **Closed lowering layers are closed** (constitution §A): a producer/front-end change makes zero edits to closed `crates/simthing-clausething/src/` lowerers *except* under a DA-authorized amendment. **This track carries two such owner-authorized amendments — see §1.**
- **No silent tier collapse; Deviation Record + Scope Ledger on every closure** (constitution §0.6). Parking a specified tier is a recorded, approved Deviation — never an implicit free pass.

---

## 1. Owner authorization & mandate (recorded verbatim by instruction)

The project Owner opened this track (2026-06-28) with **maximal authorization to expand the
ingest/authoring capability of SimThing Studio** toward the horizon goal of *full ClauseScript ↔
SimThing runtime fluency*, and ultimately ingesting and transpiling Paradox's full Stellaris base
configuration. The 0.0.8.4 scenario is **Objective A**: prove the capability of ingesting a complete,
native ClauseScript file (authored in-repo) and running it as a live SimThing simulation.

Two normally-gated expansions are **owner-authorized as in-scope for this track**:

1. **Closed-lowerer RF capacity amendment.** Raise/scale the RF participant & slot caps and the GPU
   slot/emission capacity so a galaxy-scale tree (1500 systems, ~250 owned, fleets, cohorts, factories)
   can admit / install / run. (Today: arena `max_participants` defaults of 8/16 in
   `simthing-driver/src/arena_registry.rs`; `SCENARIO_STRUCTURAL_INTEGER_MAX = 16_777_216` in
   `simthing-spec/src/spec/scenario.rs`; GPU slot/emission capacity fixed at session attach.) This is
   the **DA-authorized closed-lowerer capacity amendment** the 0.0.8.3 constitution §A named as the
   outstanding gate before galaxy-scale packs can install. It is opened here.

2. **The shipsize / `triggered_produces_modifier` modifier family.** ClauseThing_Spec §8 and the CT-2c
   memo §3 deferred this family (~69% of the `modifiers.log` key space) *"until a consumer names ships."*
   **This scenario names ships and fleets — it is exactly that consumer.** The family is pulled into
   scope here.

> **Owner stress-test instruction (recorded verbatim by the Owner's request, 2026-06-28):** *The Owner
> directed Claude to use the particularly hairy part of the ClauseScript modifier language — the
> `(category)_(resource)_(produces|upkeep|cost)_(add|mult)` underscore modifier-key chains, the
> `pop_category_*` / `_mod_pop_*`-style chains, `shipsize_*`/`ship_*` weapon/hull keys,
> `triggered_modifier { potential ... }`, `complex_trigger_modifier`, and `value:`/`ai_will_do` scripted
> modifier blocks — **liberally** when authoring this scenario, specifically to stress-test the intake
> stack and force the deep reduction to `EvalEML` opcode chains that proved difficult without it.* The
> authored `.clause` is therefore deliberately adversarial against the decoder, not minimal. The
> modifier catalogue the author must exercise is **Appendix A**.

---

## 2. Objective & acceptance

**Deliverable:** a single native ClauseScript file —
`crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause` — that:

1. **Describes a 1500-star disc galaxy consistent with the Studio galaxy generator.** The base lattice
   (disc shape, structural `(col,row)` placements, hyperlane topology, Stellaris-namespace star display
   names) is produced by `simthing-mapgenerator` (the closed producer) and embedded/referenced as the
   authored base; the ClauseScript adds the runtime overlay on top. The base **is byte-consistent with
   what Studio would generate** for the same seed/shape (proven by regenerating it).
2. **Transpiles fully to canonical `SimThingScenarioSpec` JSON.** After ingestion, *only* SimThing-Spec /
   ClauseThing scaffolding remains — no ClauseScript modifier/category/trigger concept survives into the
   runtime authority or any GPU artifact.
3. **Injects the Terran-Pirate runtime scenario at galaxy scale:**
   - **Terrans own 200 star systems** in a contiguous disc volume. Owning a star system = owning all child
     planet systems under it. Each owned system has **≥1 planet**, and each such planet has **≥1 factory
     building + ≥1 Terran population cohort** under its surface gridcell — which is what bestows ownership
     (owner-column + identity overlay flip on the system subtree).
   - **Pirates own 50 star systems** in a volume adjacent to the Terran volume, each with a planet carrying
     a factory + cohort.
   - **The remaining ~1250 systems are light-payload neutrals** (per Owner decision): each a gridcell
     Location with a minimal planet + 1×1 surface gridcell, **no** cohort/factory/owner — raidable/
     colonizable targets that give the STEAD fronts something to contest.
   - **Terran fleets:** **200 ships in 10 fleets**, distributed by the **60-40 rule** — 60% (6 fleets /
     ~120 ships) securing the border connections to Pirate systems; 40% (4 fleets / ~80 ships) in the
     interior and other borders.
   - **Pirate fleets:** **400 ships in 10 fleets**, distributed by the **80-20 rule** — 80% (8 fleets /
     ~320 ships) poised to raid/disrupt Terran space; 20% (2 fleets / ~80 ships) protecting Pirate space.
4. **Runs live through an indefinite number of ticks** in SimThing Studio, with **all decision-making
   made entirely by STEAD** — each faction perceiving threats and needs organically as Movement-Front
   pressure columns, and committing (attack / reinforce / raid / withdraw / fortify) only when a
   personality-weighted pressure crosses a registered threshold. No CPU planner anywhere.

**Acceptance (the Scope Ledger this track closes against, constitution §0.6):** every element of (1)–(4)
marked `implemented` / `proxied` / `deferred` / `parked` with evidence, plus a non-vacuous multi-tick
run on a real adapter where the contested Terran/Pirate border front measurably evolves and at least one
faction commitment fires from a threshold crossing (not a scripted event).

---

## 3. The transpilation pipeline (what gets built)

```
terran_pirate_galaxy.clause                     ← single authored native ClauseScript file
   │   (A) base: 1500-star disc static_galaxy_scenario   (simthing-mapgenerator, seed-pinned)
   │   (B) overlay: scenario-container ClauseScript       (owners, planets, factories, cohorts,
   │                                                        fleets, ships, combat/diplomacy economics,
   │                                                        front authoring, ai_will_do commitments)
   ▼
ClauseThing hydrate  (widened front-end; closed lowerers touched only under §1 amendments)
   │   • neutral-AST base → gridcell lattice hierarchy (galaxy → star → planet → 1×1 surface)
   │   • Owner SimThings as GameSession siblings  +  owner-columns  +  identity overlays
   │   • surface-tier gameplay children: factory buildings, population cohorts
   │   • shipsize/triggered-modifier decoder family (NEW §1.2): fleets, ships, hull capability trees
   │   • RF arenas:  HP/Damage (combat) · influence/distrust (diplomacy) · economy · suppression · disruption
   │   • Movement-Front L1/L2/L3:  Gu-Yang SaturatingFlux borders/chokes · PALMA reach · ai_will_do urgency
   ▼
SimThingScenarioSpec JSON   ← canonical save/load authority; only SimThing-Spec scaffolding remains
   │   + RF capacity amendment (§1.1): galaxy-scale participant/slot/emission caps
   ▼
Studio load → driver compile → SimSession resident GPU tick (indefinite)
   │   accumulate → reduce up → settle → disburse down → threshold crossings → BoundaryRequests
   ▼
Live STEAD simulation:  fronts propagate, borders settle/shift, fleets follow gradients (reparent),
                        combat resolves as HP/Damage, faction commitments fire from thresholds.
```

**Fleet homing decision (recorded design decision, grounded in core §2 law 1 + §7).** Immobile gameplay
children (factories, cohorts, buildings) home under the **planet 1×1 surface gridcell** (the mandated tier,
constitution-enforced). **Fleets are mobile spatial occupants of the star-system local grid** — they home
as children of the **star-system gridcell's local-grid Location** ("the Location where they physically
participate"), not a planet surface, and **movement is reparenting** to an adjacent star-system gridcell
down the desirability/threat gradient (core §7.2 "the front is the route"). This is a design decision, not
a re-derivation; it is flagged for DA confirmation at the Phase-6 rung and may be revised to a dedicated
"fleet berth" surface cell if review prefers.

---

## 4. The PR ladder

Each rung opens with the §0 harness header, self-checks its diff against the six base principles
(constitution §0.5 Rule 2), and lands a `docs/tests/*_results.md` report carrying a Scope Ledger.
Tier gates per constitution §0.5 Rule 7 and ClauseThing_Spec §5/§7. **`cargo test --workspace` is never
run**; each rung names its exact targeted tests.

### Phase 0 — Track opening & the capacity amendment (clears the galaxy-scale gate first)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 0.0 | `TP-TRACK-OPEN-0` | This document + evidence-index row + harness. Docs only. | Doc lands; ledger row honest (impl not started). | — |
| 0.1 | `TP-RF-CAPACITY-AMENDMENT-0` | **§1.1 amendment.** Make RF arena `max_participants`, GPU slot capacity, and emission capacity **scalable to galaxy scale** (budget-driven, not a magic constant — mirror `MapgenStructuralGridBudget` checked-`u128` discipline). No new `AccumulatorRole`, no semantic WGSL, no per-tick allocation. Pool growth at boundaries only (constitution §0.4). | Existing RF/admission tests stay green at small caps; a new galaxy-scale admission test installs an arena with participant counts ≥ the 250-owned + fleet load; `e10_resource_flow_admission` extended; CPU-oracle parity preserved. | **Tier-2** (touches closed lowerer + a binding cap) |
| 0.2 | `TP-SCALE-ENVELOPE-0` | Prove the **base 1500-star disc** lattice + topology admit/install at scale through the widened caps (placement/topology already tested at 1500 in `topology_stead.rs`; this proves *install*, not just placement). | Headless: generate 1500-star disc → lattice hierarchy → admit/install → compact GPU readback `is_none()` on a real adapter. | Tier-1 over 0.1 |

### Phase 1 — Base galaxy production (mostly reuse; Studio-consistency proof)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 1.0 | `TP-BASE-DISC-GEN-0` | Drive `simthing-mapgenerator` to emit the canonical **disc, 1500-star** base `static_galaxy_scenario` (seed-pinned) with Stellaris star-name corpus assignment. Capture the seed + params in the scenario metadata. | Byte-identical regeneration from recorded seed/params; `map_quality_status = PASS`; names assigned deterministically; the output is what Studio's Generate path produces for the same levers. | Tier-1 |
| 1.1 | `TP-BASE-EMBED-0` | Define how the base is carried in the single `.clause`: an embedded/`include`-style `static_galaxy_scenario { ... }` block consumed by the existing neutral-AST parser, with the Terran-Pirate overlay layered in the same file. | The combined `.clause` parses; base lattice round-trips identical to rung 1.0. | Tier-2 (combined-document grammar) |

### Phase 2 — Ownership: owners, planets, factories, cohorts (the scenario-container widening)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 2.0 | `TP-OWNER-SIBLINGS-0` | ClauseScript authoring of **Owner SimThings as GameSession siblings** (Terran, Pirate) with stockpile/policy/personality/capability columns. Lower to `Scenario → GameSession → {Owner, Owner, GalaxyMap}`. **No owner is a spatial parent.** | Hydrated tree has owners as GameSession children; STEAD roundtrip preserves owner-metadata-distinct-from-spatial-parentage (reuse `scenario_stead_map_roundtrip` pattern). | Tier-2 (first owner-as-sibling clause authoring) |
| 2.1 | `TP-OWNERSHIP-COLUMNS-0` | **Ownership = owner-column + permanent identity overlay** on each owned system subtree. Selecting the 200 contiguous Terran systems + 50 adjacent Pirate systems is an authoring-time contiguous-volume selection over the disc `(col,row)` layout (integer Chebyshev neighborhoods; **no Euclidean authority**). | 200 Terran + 50 Pirate systems carry the owner column; capture-as-column-flip proven by a unit flip test; neutrals carry no owner column. | Tier-1 over 2.0 |
| 2.2 | `TP-PLANET-SURFACE-PAYLOAD-0` | Each owned system: **≥1 planet gridcell → mandated 1×1 surface gridcell → ≥1 factory building + ≥1 cohort**. Light-payload neutrals: planet + surface, no children. Factory/cohort economy authored with **Appendix A modifier chains** (`pop_category_*` factory output, upkeep). | Surface-tier present and non-vacuous (no silent tier collapse, constitution §0.6); RF settles surface→planet→star→galaxy; ownership bestowed by presence of factory+cohort under owner column. | Tier-1 over 2.1 + 2c decoder |

### Phase 3 — Fleets, ships & the shipsize modifier family (the hairy-modifier stress core)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 3.0 | `TP-SHIPSIZE-DECODER-0` | **§1.2 amendment.** Extend the CT-2c longest-match modifier decoder with the **shipsize/`ship_*` family** (`shipsize_corvette_hull_add`, `ship_weapon_damage_mult`, `ship_fire_rate_mult`, `ships_upkeep_mult`, `country_naval_cap_add`, …) and **`triggered_modifier`/`complex_trigger_modifier`** gated forms. All lower to overlays (`Add` leaf-only / `Multiply` subtree-sweep) + `EvalEML` `ExactDeterministic` ≤32-node trees. **Ship classes lower to capability-tree `Custom(...)` SimThings.** | The decoder round-trips the **lab `modifiers.log` shipsize subset** (ignored, lab-only `CLAUSER_LAB_DIR` posture, no corpus committed); ambiguous segmentations rejected; every compiled EML tree matches the CPU oracle bit-exact. | **Tier-2** (new grammar family + the frontier silent-fidelity surface) |
| 3.1 | `TP-FLEETS-SHIPS-0` | Author **fleets as mobile star-system-grid occupants** and **ships as cohort-style children** with HP/Damage/upkeep columns. Place 10 Terran fleets (200 ships) by the 60-40 rule and 10 Pirate fleets (400 ships) by the 80-20 rule, homed at the relevant border/interior/raid-posture systems. | Fleet/ship counts and distribution match the rules exactly (counted in the hydrated spec); fleets reparent cleanly (re-enrollment via `FissionPolicy` + subtree-incremental arena refresh); upkeep is an RF obligation, not a fleet subsystem. | Tier-1 over 3.0 |

### Phase 4 — Combat as HP/Damage economics

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 4.0 | `TP-COMBAT-ARENA-0` | Author the **HP/Damage RF arena**: damage = `SubtractFromSource` transfer between opposing ships co-located in a system arena; HP recovery = `governed_by`; **owner combat bonuses (`ship_weapon_damage_mult`, etc.) disburse down as overlays on the Damage columns**; ship death = zero-HP `Threshold` → `EmitEvent` → `BoundaryRequest` removal (slot recycles, constitution §0.4). | A two-fleet contact resolves on GPU == CPU oracle bit-exact; a ship crossing zero HP fires removal at a boundary; owner bonus changes the resolved damage through the overlay path only. | Tier-1 over the accepted HP/Damage doctrine |

### Phase 5 — Diplomacy as influence/distrust economics

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 5.0 | `TP-DIPLOMACY-FLOW-0` | Author **influence/trust/distrust as RF lanes** (core §2.1, §5.3): each owner emits influence/distrust into touched assets; reduces up to the owner; a stance/hostility change is a **registered threshold crossing on the reduced trust/distrust column** (`AggregateAlertRegistration`-class) → `EmitEvent`. Terran↔Pirate baseline hostility seeded as an authored distrust intensity. No diplomacy subsystem. | A distrust accumulation crosses an authored threshold and emits a hostility commitment; trust math is bit-exact vs oracle; the influence round-trip lands on the owner through ordinary disbursement (subversion/foreign-sponsorship native, core §2.1). | Tier-1 over RF substrate |

### Phase 6 — STEAD fronts & fleet movement (the Movement-Front automaton, live)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 6.0 | `TP-FRONTS-AUTHORING-0` | Author the **Movement-Front L1/L2/L3** over the galaxy lattice: **threat front** (pirate raid pressure), **suppression front** (terran patrol/fleet presence), **disruption front** (raids feeding it). L1 = Gu-Yang `SaturatingFlux` `RegionFieldSpec` seeded by RF arena pressure (`ArenaPressureBindingSpec`); L2 = reduce up; L3 = `ai_will_do` urgency. Bounded horizon (P1), one shared stencil (P2), attractor/threshold (P3). | Fronts seed from arena pressure on-device (no readback); the contested Terran/Pirate boundary is a settling contour where suppression balances disruption; exact-magnitude gates route through Candidate F. | Tier-1 over landed Movement-Front (atlas/bounded-theater scheduling for the vast lattice) |
| 6.1 | `TP-PALMA-REACH-0` | **PALMA reach/impedance** over the fronts: impedance `W` composed from choke/threat fields → resident `D`. The reach field is what fleet movement gradients consume. No route object, no predecessor. | `D` field resident on a real adapter (compact probe); pathing is gradient-following only; forbidden route/path/predecessor tokens absent. | Tier-1 over seated PALMA |
| 6.2 | `TP-FLEET-MOVEMENT-0` | **Fleet movement = gradient-following reparenting.** A fleet steers proportionally down/up the local desirability/threat/reach gradient between star-system gridcells; movement is column updates + arena re-enrollment, **not** a route solver. Velocity uses an explicit previous-value column + copy band (EML has no previous-buffer read). | A fleet reparents toward higher raid-desirability (pirate) / toward border threat (terran) over multiple ticks; arena membership follows the move; bit-exact GPU==CPU on the gradient step. | Tier-2 (first live fleet movement; confirms the §3 fleet-homing decision) |

### Phase 7 — STEAD decisions: ai_will_do commitments (FIELD_POLICY, the capstone of "decisions by STEAD")

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 7.0 | `TP-COMMITMENTS-0` | Author each faction's **personality `ai_will_do`/`ai_weight` EML weight profiles** over its reduced L3 pressure columns (aggression/risk-tolerance sub-fields × pressure → urgency), firing commitments (**attack / reinforce / raid / withdraw / fortify**) as `Threshold` + `EmitEvent` → `BoundaryRequest` crossings. Pirate 80-20 and Terran 60-40 postures are **initial placement**, not ongoing scripted decisions; every subsequent decision is a threshold crossing. **No CPU planner, ever** (drift detector §9.4). | At least one commitment per faction fires *from a crossing over a resolved front column*, not a scripted timer; the CPU only consumes the structural event at a boundary; `grep` finds no CPU urgency traversal / commitment emission. | Tier-1 over the accepted FIELD_POLICY path |

### Phase 8 — Full transpile, live run, DA closure

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 8.0 | `TP-FULL-TRANSPILE-0` | Ingest the **complete single `.clause`** end-to-end → `SimThingScenarioSpec` JSON. Prove **only SimThing-Spec/ClauseThing scaffolding remains** (no ClauseScript/semantic token in the runtime authority or GPU artifacts). Canonical save/load roundtrip stable digest. | Full file transpiles; semantic-free scan passes; STEAD/link/tree/RF metadata survive roundtrip (reuse the closed save/load battery). | Tier-1 |
| 8.1 | `TP-LIVE-RUN-0` | **Indefinite-tick live run in Studio** (or headless driver at galaxy scale, with Studio load proven). Run the contested border for N ticks; the front evolves; combat resolves; a commitment fires. Capture the field-movie evidence. | Non-vacuous multi-tick run on a real adapter; border front measurably shifts; ≥1 STEAD commitment fires; no per-tick device/buffer creation; no CPU planner. | Tier-1 |
| 8.2 | `TP-DA-CLOSEOUT-0` | Scope Ledger over every §2 acceptance element; Deviation Records for anything proxied/deferred (e.g. galaxy-scale dense Movement-Front execution deferring to atlas scheduling is a bounded-theater Deviation, not a failure); DA review. | Complete Scope Ledger; DA sign-off (owner-channeled). | — |

---

## 5. Honest deferrals & Deviations (declared up front, constitution §0.6)

- **Dense galaxy-scale Movement-Front execution defers to atlas/bounded-theater scheduling.** A vast
  layout is admitted even where a dense execution profile defers — *layout admitted, execution requires
  atlas/tile scheduling*, never "the map is too large" (STEAD invariant 5). The live border run (8.1)
  proves the contested sub-volume as one or more bounded theaters; full-galaxy simultaneous dense fronts
  are an atlas Deviation, recorded, not silently flattened.
- **`from`/`root` dynamic scope chains** (ClauseThing_Spec §8 SCOPE-MEMO gate) are bounded to the CPU
  boundary or rejected; this scenario is authored to need only same-owner/same-scope triggers so no rung
  blocks on the event-payload substrate extension. Any construct that needs richer context is a recorded
  Deviation, not a silent CPU planner.
- **Balance / economic depth** beyond what drives the contested front is out of scope; this track proves
  the *mechanism and the ingestion*, not faction balance (ClauseThing_Spec §8 closing note).
- **Hull-design capability-tree depth** lands as the *shape* (ship class = `Custom(...)` SimThing); deep
  multi-tier tech trees are a follow-on consumer, not this track.

---

## 6. Appendix A — the hairy-modifier stress catalogue (Owner-mandated, §1)

The authored `.clause` must exercise these forms liberally (the Owner's explicit stress-test instruction).
Each lowers through the extended decoder to existing substrate only (overlays + `EvalEML` ≤32-node trees +
RF arenas), CPU-oracle bit-exact. Listing is the decoder's priority queue, not decoration.

- **Underscore modifier-key chains** `(category)_(resource)_(produces|upkeep|cost)_(add|mult)`, longest-match
  against registered sets: e.g. `pop_category_worker_minerals_produces_mult = 0.10`,
  `settlement_food_produces_add = 6`, `pop_factory_minerals_produces_mult`,
  `polity_energy_cost_add` (discrete-economy context only). `_add` lowers leaf-only; `_mult` sweeps the
  category subtree (the inheritance asymmetry, CT-2c §4).
- **Shipsize / `ship_*` family** (§1.2, newly admitted): `shipsize_corvette_hull_add`,
  `ship_weapon_damage_mult`, `ship_fire_rate_mult`, `ships_upkeep_mult`, `country_naval_cap_add` — combat
  economics + naval capacity as overlays on HP/Damage/upkeep columns.
- **`triggered_modifier { potential = { ... } <keys> }`** → `Suspended`→`Permanent`/`Transient` overlay +
  threshold/dissolve: e.g. `triggered_modifier { potential = { is_at_war = yes } ship_weapon_damage_mult = 0.15 }`.
- **`complex_trigger_modifier` (bool→number)** → `EvalEML` `SELECT`/threshold-count — admitted only where
  the underlying trigger reads a column (otherwise hard-error, ClauseThing_Spec §8).
- **`value:` scripted values** (base + math + modifier) → `EvalEML` formula trees, `ExactDeterministic`,
  ≤32 nodes, `@var` constant-folded.
- **`ai_will_do` / `ai_weight` blocks** with nested `modifier { factor … <trigger> }`: e.g.
  `ai_will_do = { weight = { base = 1 modifier = { factor = 5 is_at_war = yes } modifier = { factor = 3 has_border_threat = yes } } }`
  → urgency EML over a pressure column → `Threshold` crossing = commitment (Phase 7).
- **Diplomacy keys** as influence/distrust economics: `trust_growth_mult`, `opinion_add`, distrust `_mult`
  on the influence lane (Phase 5).

Any unknown/ambiguous form is a **spanned hard error with a suggested path** — never a silent guess
(CT-2c §8). The diagnostic stream is the backlog's priority queue toward full Stellaris fluency.

---

## 7. References

- Permanent paradigm: [`simthing_core_design.md`](simthing_core_design.md) (§2 the one tree, §5 RF arenas, §6 overlays, §7 Movement-Front automaton, §8 decisions, §9 drift detectors).
- Active constitution: [`design_0_0_8_3.md`](design_0_0_8_3.md) §0 (anti-flattening §0.6, exact-magnitude §0.7, STEAD §0.8), §A (closed-lowering gates + the named RF capacity amendment).
- ClauseScript↔SimThing isomorphism: [`clausething/ClauseThing_Spec.md`](clausething/ClauseThing_Spec.md) §4, §5 (tier ladder), §7 (guardrails), §8 (hard problems); decoder + movables: [`clausething/ct_2c_economic_category_memo.md`](clausething/ct_2c_economic_category_memo.md).
- MapGen producer + grammar: [`clausething/MapGeneratorCLI.md`](clausething/MapGeneratorCLI.md), [`clausething/MapGenThing.md`](clausething/MapGenThing.md); STEAD contract: [`stead_spatial_contract.md`](stead_spatial_contract.md).
- Studio authority + prior Terran-Pirate skeleton: [`design_0_0_8_3_studio_production.md`](design_0_0_8_3_studio_production.md), [`tests/terran_pirate_scenario_skeleton_0_results.md`](tests/terran_pirate_scenario_skeleton_0_results.md).
- Movement-Front operators: Gu & Yang (arXiv:2509.20797, `SaturatingFlux`), PALMA (arXiv:2601.17028), Wei/STEAD (arXiv:2602.01651), EML (arXiv:2603.21852).

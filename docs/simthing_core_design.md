# The SimThing Core Design — Permanent Paradigm Reference

> **Status: PERMANENT. This document is the paradigm itself, not a version of it.** It sits *beneath*
> the versioned constitution (`design_0_0_8_1.md` and successors): the constitution governs process,
> gating, and the current production track; **this document cements the immutable architecture every
> version, every PR, and every agent must build toward.** It is deliberately self-contained — no link
> needs to be followed to apply it. If any handoff, PR ladder, status row, or ancillary-service design
> conflicts with this document, this document wins, and the conflict is escalated to design authority.
> Changes to this file are Tier-2, design-authority-only, by addition — never silent weakening.
>
> **If you are a low-context agent: hold this file in context for the entire task.** Everything else
> is detail; this is the spine.

---

## 1. The SimThing Principle — the unitary vision

**Everything is a SimThing.** A SimThing is a recursive `{ properties, overlays, children }` node.
The entire simulation — the game session, factions, the world map, star systems, planets, grid cells,
fleets, cohorts, buildings, even arena-participant wrappers — is **one recursive tree of SimThings,
rooted in a single Session SimThing**, resident on the GPU as flat dense matrices.

The principle exists for exactly one reason: **conflict, opportunity, ambition, and extraction all
collapse into a single generic, GPU-resident mechanism.** Combat, economy, AI ambition, raiding,
trade, diplomacy, disruption — each is the *same* loop wearing a different label:

```
accumulate → reduce up the tree → mask → disburse down → threshold-crossings fire decisions
```

There is no combat engine, no economy engine, no AI engine, no pathfinding engine. There is one
`accumulate → reduce → mask → threshold` loop resolving all of them in the same GPU pass over the
same property columns. **Decisions are GPU-resident threshold crossings over resolved fields — never
a CPU planner.** The moment any behavior becomes a privileged structural special case (a bespoke node
shape, a runtime `match kind`, a subsystem beside the tree), it leaves the generic substrate, can no
longer be resolved as uniform GPU automata, and the unitary vision breaks. That is why conformance is
non-negotiable: it is not a style preference, it is the **precondition** for the whole simulation
being one GPU automaton instead of a federation of bespoke subsystems.

When a design seems to need special-case logic, the correct move is almost always **more SimThing**:
more properties, more overlays, more `AccumulatorOp` registrations — never a new subsystem.

**The substrate, not the game, is the product.** This genericity is foundational, not retrofitted:
from the earliest design (v4: *"semantic labels are read by the CPU semantic layer for display
only; the simulation never sees them"*) the kernel layer has been guarded against game semantics —
even the boundary layer is ticks and a monotonic counter, with "day" admitted only as legible
naming. The grand-strategy game is the engine's **first consumer, not its definition**: SimThing is
a general-purpose, GPU-resident simulation ontology engine. The same properties the game demanded —
semantic-free residency, bit-exact determinism, replay-pinned reproducibility, spec-layer ontology
that compiles away, and dynamics satisfying Anchor A's postulates — make every run a natively
annotated scientific artifact: **long-horizon field movies** (gradient heatmaps with full causal
sidecars and legal interventions) that constitute training and evaluation corpora for machine-
learned world models. This second mission is recorded here as *identity, not authorization* — its
consumers and current candidate technologies live in `workshop/field_world_model_horizon.md` and
open only by the normal consumer-pulled gate. Its operational force in this document is simple:
every binding constraint below now has **two products** depending on it. A semantic leak or a
determinism break poisons the corpus as surely as it breaks the game.

### 1.1 The two theoretical anchors

The paradigm is not folklore; it rests on two published results that every agent should internalize,
because they explain *why* the engine's constraints generalize where bespoke systems collapse:

**Anchor A — Movement-Front (Wei's *STEAD* concept): the cellular-automaton physics of the map.**
Zichao Wei, *On the Spatiotemporal Dynamics of Generalization in Neural Networks*
([arXiv:2602.01651](https://arxiv.org/abs/2602.01651)), derives
from three physical postulates that any system achieving lossless causal generalization is
necessarily a cellular automaton of locally-coupled cells iterated to convergence:

- **P1 Locality** — information propagates at finite speed; a cell's next state depends only on its
  neighborhood (its past light cone). Action-at-a-distance destroys causal structure.
- **P2 Symmetry** — one shared evolution rule at every cell and every tick; laws never depend on
  absolute coordinates. Only a translation-invariant rule generalizes beyond what it was tuned on.
- **P3 Stability** — the dynamics are dissipative: states converge to discrete attractors, so noise
  is projected back to legal values instead of accumulating; convergence itself signals
  "computation complete," and computation time adapts to how far the wave must travel.

SimThing's **Movement-Front system** (§7) is the engine-native realization of this result: gridcell
SimThings are the lattice, the one shared stencil kernel is the rule, the horizon-capped band cascade
is the light cone, and bounded operators + threshold projection are the attractor dynamics. Fronts —
disruption fronts, threat fronts, supply fronts, suppression waves — propagate, interfere, and settle
exactly as Wei's traveling waves do. When the constitution bans dense global diffusion, per-cell
bespoke rules, or unbounded recurrence, it is enforcing P1, P2, and P3 respectively.

**Anchor B — EML: one operator generates all behavior.** Andrzej Odrzywołek,
*All elementary functions from a single operator* (arXiv:2603.21852), proves that the single binary
operator `eml(x,y) = exp(x) − ln(y)`, with the constant 1, generates the entire elementary-function
repertoire — every formula becomes a uniform binary tree of identical nodes under the grammar
`S → 1 | eml(S,S)`, executable as an opcode stack on a single-instruction machine. This is the
constitutional justification for the engine's expression discipline: **any scripted interaction,
however complex, is encodable as an opcode stack over one fixed generic interpreter** (`EvalEML`),
so behavior is always *data* — postfix programs, gadget trees, column parameters — and never a new
kernel, opcode, or subsystem. The EML gadget library and the JIT shader compiler (§4) are the two
production surfaces of this anchor.

---

## 2. The one tree, and the Session root

Every SimThing in a running simulation is a descendant of one root **Session SimThing** (the
`GameSession` root). Its immediate children are siblings, not a hierarchy of engines:

```
Session (root)
├── Faction SimThings          (owner-entities: stockpiles, personality, policy overlays)
├── Species / registry SimThings
└── WorldStateMap SimThing     (the spatial spine)
    └── Galaxy grid → star systems → planets → planet surfaces → gridcells
        └── cohorts, fleets, buildings, pops … (leaf participants)
```

Two laws govern the tree's shape:

1. **The spatial tree expresses physical containment only.** A planet's spatial parent is its star
   system; a cohort's spatial parent is its location. Movement is the only thing that reparents.
2. **Owner-entities are never spatial parents.** Factions, species, and every other identity live as
   *sibling* children of the Session root. A planet changing hands is an **owner-column flip on the
   planet — never a reparenting, never a slot move.** The once-proposed "D=3 ownership node" (a
   structural faction tier inside the spatial tree) is the canonical rejected design; do not
   re-derive it. One spatial parent + N owner-columns, always.

`SimThingKind` is a topology label for spec/driver convenience. **Behavior never branches on kind at
runtime** — no `match kind` in any simulation path. New entity types are `Location` / `Cohort` /
`Custom(String)` carrying the right properties and overlays; new `SimThingKind` variants are not the
answer (the deprecated `StarSystem` / `Station` variants are the cautionary record).

### 2.1 Owner-entity fission — policy capture, succession, and civil war

Law 2 covers the capture of *assets*; this section covers the capture of the **owner itself**. When
the contested object is a polity's policy, cohesion, or existence — under **stress** (unrest,
defeat, fiscal exhaustion) or **inducement** (foreign sponsorship, ideological conversion, bribery)
— the generic mechanism is **owner-entity fission**, and it is how the engine models civil war,
secession, coups, and policy capture with zero special cases:

1. **Influence is an ordinary flowing quantity.** Any participant granted the property — domestic
   cohorts, foreign agents, anything — emits alignment/influence into the assets it touches. It
   reduces leaf → root like any resource and disburses back down onto the owning faction SimThing.
   The root round-trip makes the Session the adjudicator and makes **foreign-sponsored capture
   native**: a rival's influence seeded into your territory aggregates through the shared spatial
   reduction and lands on you through ordinary disbursement. Lobbying, regulatory capture,
   ideological conversion, and fifth columns are one flow with different sources.
2. **The trigger is an ordinary threshold.** Aggregate influence on the owner crosses a registered
   watch on the owner's post-reduction field (`AggregateAlertRegistration`-class) → `EmitEvent` →
   `BoundaryRequest`. Rebellion, revolution, separatism, and civil war are **property values
   crossing thresholds — never discrete flags, never special entity types.**
3. **The fission is of the owner entity — never the map.** At the boundary, the faction SimThing
   fissions through the existing `FissionTemplate` machinery (`clone_capability_children` hands the
   successor its inherited capability subtrees — tech tree, national ideas). The owned assets
   partition by their **per-asset alignment-intensity vector as a mass owner-column flip**: one new
   sibling node under the Session root plus N column flips. Per Law 2, no spatial reparenting
   occurs — the most violent political event in the simulation is structurally one of its cheapest.
4. **The burst announces itself.** A polity-scale fission re-resolves many memberships in one
   boundary. The influence **velocity** columns, computed every tick, predict the crossing before
   it arrives — slot pre-allocation and cascade preparation run on measured growth rates, never
   heuristics. Each resource-flow arena's declared `FissionPolicy` (`{Inherit, Reevaluate,
   Reject}`, §5) governs how the split polity's participants re-resolve.

**Provenance (recorded so this is never re-excavated from archives):** assembled across three
hard-earned workshops — capability-tree v1 (differentiation by intensity threshold; the
faction-fission inheritance hook), the E-11B reparenting analysis (empire collapse as fission
cascade; the arena-re-enrollment gap this design *avoids by construction* by keeping capture in
columns; velocity-driven predictive pre-allocation), and the policy-capture trigger pathway
(2026-06-10).

**The strategic toolkit this opens:** every participant — and every faction AI reading the fields —
gains a fourth vector beside fighting, trading, and allying: **subversion**. Emit influence to
capture a rival's policy weights or split its polity; defend with suppression and counter-influence
over your own assets; read the influence-velocity field as early warning of your own fracture.
Because allocation weights and threshold parameters are themselves reachable through this flow,
**the rules of the simulation are a contested object inside the simulation** — reflexivity is
endogenous to the substrate, not a bolted-on system.

---

## 3. SimProperty → Value: the load-bearing data model

All identity, resources, and state live in properties. This structure is **load-bearing for the
entire resource-flow accumulator resolution system** — the sub-fields below *are* the GPU columns the
accumulator reduces, masks, and disburses over, and *are* the cell-state columns the Movement-Front
automaton evolves. Get this wrong and nothing downstream works.

```
SimProperty   = identity (namespace + name — equality is on identity ONLY)
              + PropertyLayout (an ordered Vec<SubFieldSpec>)
              + optional behavior (decay, intensity, fission/fusion templates, on_expire)

SubFieldSpec  = role        SubFieldRole: Amount | Velocity | Intensity | Named(String) | Custom(String)
              + width       1 = scalar, N = vector of N floats
              + clamp       per-sub-field ClampBehavior (no property-level valid_range)
              + default
              + governed_by Option<SubFieldRole>   ← declared integration: this sub-field advances
                                                     by the governing role's value × dt each tick
              + reduction_override / soft_aggregate_guard / accumulator_spec (compile-time metadata)

PropertyValue = { data: Vec<f32> }   ← one flat float vector per (SimThing, property) instance;
                                       layout defined entirely by the registered SimProperty
```

The binding rules that make this safe at GPU scale:

- **One home for index arithmetic.** `stride()` is computed, never stored. Local offsets come from
  `PropertyLayout::offset_of(role)` only; global columns from `PropertyColumnRange::col_for_role`
  only. **No hardcoded `data[N]` anywhere, ever.** Overlays and transforms reference sub-fields
  **by role, not by column index**; the CPU prep pass resolves roles → columns; the GPU receives
  only resolved indices.
- **Integration is declarative.** `governed_by` is the only rate-of-change mechanism: Amount governed
  by Velocity, position governed by drift, HP governed by regeneration. Saturated values pin the
  governing rate to zero (no hidden velocity debt). The `Balance` carryforward pattern (below) is
  built on this same machinery.
- **Registry discipline.** Properties register once per session; columns are append-only; removal is
  tombstoning (`active=false`), never compaction — slot/column indices stay stable for the GPU.
- **Reduction is per-role.** Each sub-field resolves a `ReductionRule` (Sum for resources, Mean /
  WeightedMean for soft aggregates, etc.). Reduction aggregates **children's column values into the
  parent's columns** — this is the upward half of resource flow. Exact/conservation paths never use
  soft-aggregate combine functions; soft aggregates feeding hard thresholds require a guard.
- **Determinism is bit-exact.** Every exact claim carries CPU-oracle parity compared with
  `f32::to_bits()`, never approximate equality.

**Resources, identity, AI personality, and cell state are all just sub-fields.** A faction's food
stockpile, a planet's `faction_id` owner column, a gridcell's threat-front value, an AI's
`aggression` weight — identical machinery: a role in a layout, a float in a column, addressed only
through the layout. A Movement-Front gridcell's schema follows the same pattern, splitting its
columns between **local causal state** (the raw field values the automaton evolves) and **inferred
dynamics** (velocity/previous-value/pressure columns derived from them) — all ordinary sub-fields,
no special cell type.

---

## 4. GPU residency — the tree as dense matrices

The recursive tree flattens to **slots × columns**: one slot per SimThing (allocated by the
`SlotAllocator`, recycled through tombstone free-lists, never compacted mid-session), one column per
registered sub-field. A persistent `AccumulatorOpSession` owns the buffers for the whole session —
**no per-tick device or buffer creation, ever.** The tick is:

```
Pass 0   Snapshot: copy values → previous_values (hardware DMA, permanent)
Pass B   AccumulatorOp: the ONE unified gather/combine/gate/scatter kernel,
         dispatched once per OrderBand in ascending band order. It performs
         velocity integration, overlay application, all reductions, EML
         evaluation, transfers, allocation sweeps, and threshold-gated events.
Pass C   Event readback: GPU atomic counter + compact EmissionRecord buffer.
         Only structural events (fission, expiry, commitments) reach the CPU;
         pure numeric resolution never leaves the GPU.
```

**OrderBands are the scheduling primitive**: dependencies between operations are expressed as band
ordering (reduce in band N, interpret in band N+1), never as bespoke pass graphs. Cross-tree
propagation advances by later-band cascade.

**`simthing-sim` is semantic-free — permanently.** It never learns the words combat, economy, map,
faction, arena, gadget, or personality. All semantics live at the spec/driver/RON layer and **compile
away** to flat `AccumulatorOp` / overlay / threshold registrations before upload. Likewise WGSL: the
shader sees only floats and indices; gameplay concepts never enter shader text. Generic, semantic-free
substrate extensions are admissible (Tier-2, with CPU-oracle parity); semantic ones never are.

### 4.1 Scripted behavior is an opcode stack: EML gadgets and the JIT compiler

This is the single-operator universality of Anchor B (arXiv:2603.21852) made production-real, and it
is **the first tool every agent must reach for** when a feature seems to demand new compute:

- **`EvalEML` is the one generic expression interpreter.** Because a single primitive suffices to
  generate all elementary functions as uniform binary trees, *any* designer formula — urgency,
  desirability, decay, policy conditionals, personality weighting — compiles to a postfix opcode
  stack over the **fixed** `EvalEML` vocabulary and executes in the same unified kernel. The
  interpreter sees only floats and indices; the formula is data.
- **The EML gadget library** is the authoring layer over that fact: gadgets (FieldSampler,
  WeightedAccumulator, SoftStep, VelocityMonitor, Decay/EMA, BoundedFeedback, Hysteresis,
  Acceleration, …) are spec-layer macros that compile to postfix subgraphs — **no new WGSL, no
  per-gadget kernel, no new opcode**. Any complex scripted interaction is encoded as a gadget tree:
  temporal state uses explicit authored columns (current/previous/state/output) with a snapshot copy
  band, and every recurrent gadget carries a bounded-feedback admission contract (finite decay < 1,
  explicit clamp, no positive unbounded recurrence) — Anchor A's P3 stability applied to formulas.
- **The JIT shader compiler** (`ProductionKernelRegistryShell`, default-off) is the performance
  escape hatch on the same principle: a validated, semantic-free, straight-line kernel compiled from
  an expression tree, admitted only with pinned artifacts, exhaustive proof for exact authority
  (the Candidate-F `sqrt` precedent), and CPU-oracle parity. Approximate outputs never feed
  exact-authoritative state.

**The extension ladder for future agents, in order:** (1) express it as an EML gadget tree over the
existing interpreter; (2) if a genuinely new *generic* primitive is unavoidable, a semantic-free
`EvalEML` opcode / combine fn / kernel is a Tier-2 gate with bit-exact CPU-oracle parity; (3) a
scenario-specific or semantic op is **never** admissible. Reaching for a new subsystem before
exhausting (1) is the canonical drift this section exists to prevent.

Guardrails are two-layered: the designer/spec admission layer rejects unsafe *authoring* at import
with good diagnostics; the runtime enforces hard safety unconditionally as the last line. Guardrails
live there — never as special cases inside the kernel.

---

## 5. Resource flow arenas — one mechanism for everything

All simulation effects resolve through **resource flow**: per-tick vector values **reduced up** the
tree, then resources **disbursed back down**, inside *resource flow arenas*.

- **Reduce up:** each parent Sum-reduces its children's flow columns into its own (surplus/deficit),
  leaf → root, via ordinary reduction OrderBands.
- **Disburse down:** the root/faction stockpile partitions budget downward in a reverse-direction
  OrderBand sweep. Each intermediate SimThing is dual-role — contributes intrinsic flow upward,
  allocates received budget to its children downward. Writes land on independent per-participant
  columns: **no shared-slot contention, no GPU hot-pool allocator.**
- **An arena is the subtree where a masked flow nets to zero.** "Flat" vs "nested" is not a structural
  fork — a cell-local combat arena is simply the leaf-most settling depth of the one recursive
  hierarchy. Allocation is *always* recursive; settling depth is emergent.
- **Allocation policy is overlay weights, never a policy enum.** The allocator kernel reads weight
  columns; defaults are Demand-proportional; player intent, AI policy, interdiction, and scripted
  effects all compose as Add/Multiply/Set overlays on the weight columns (EML for conditionals).
- **`Balance` is the sole carryforward ledger.** Leaf residuals, allocator rounding residuals
  (O(ε × n_children) per step, deterministic, replay bit-exact), and zero-weight surplus all
  integrate into `Balance` via standard `governed_by`. No second budget state may exist.
- **Conservation:** discrete transfers are exact (`SubtractFromSource`; recipes via
  `MinAcrossInputs + SubtractFromAllInputs`); continuous allocation is approximate-deterministic.
  Hard currency never routes through continuous flow.

**All conflict is resource flow — and its spatial face is a Movement-Front.** Combat is an HP/Damage
arena (damage = `SubtractFromSource` transfer; HP recovery = `governed_by`; death = a zero-crossing
`Threshold` + `EmitEvent` → `BoundaryRequest` removal). Disruption is an accumulating-and-decaying
property arena whose values reduce up to the starmap as the heatmap — and whose lateral expression
across the gridcell lattice is a propagating disruption *front* (§7): patrols suppress it, pirates
feed it, and the contested boundary where suppression and disruption balance is exactly a traveling
wavefront settling toward an attractor. Trade, diplomacy, raiding, suppression — same law. Endgame
scale is never solved by prohibiting scale: participant caps are on *concurrent* participants, slots
recycle through the re-enrollment free-list, and pool growth happens only at boundaries.

---

## 6. Overlays — the universal modifier

**Every modifier in the system is an overlay on a SimThing.** An overlay is
`{ kind, source, affects, transform, lifecycle }` whose transform is a `PropertyTransformDelta`:
a list of `(SubFieldRole, Add|Multiply|Set)` pairs against a property, applied on overlay OrderBands
in the same unified kernel as everything else. There is no other modification mechanism. Concretely:

| What it looks like in a game | What it actually is |
|---|---|
| **Ownership / identity (permanent)** | Owner-columns (`faction_id` and friends) on the owned SimThing, plus **permanent identity overlays** stamping the owner-relation. The political map is overlays on the spatial tree, never nodes in it. Capture = column flip + refresh of the *faction* overlay layer; per-relation layers are independent (the species layer persists through capture). Modifier overlays are latched and blockade-immune (knowledge ≠ goods); flow is blockable. |
| **Policy / governance** | Overlays writing **weight columns** read by the allocation sweep, and Add/Multiply deltas on production/consumption sub-fields. A policy *is* its numeric pressure on the flow. |
| **AI personality** | Authored personality sub-fields (aggression, risk tolerance…) on the faction SimThing, applied as **EML weighting overlays** over reduced Movement-Front pressure fields. The AI has no other existence. |
| **User intervention / player controls** | `OverlaySource::Player` overlays — same transform machinery, same bands, same lifecycle. A player edict and an AI policy are structurally identical. |
| **Capability / tech trees** | Abstract trees that **resolve to modifier overlays + instantiation gates**; unlocking instantiates via gated fission. Capabilities never become runtime branches. |
| **Crises, events, scripted effects** | Transient overlays with declarative `DissolveCondition`s (property thresholds, tick timers, override), with any complex scripted logic encoded as an EML gadget tree (§4.1) — never bespoke event code. |

Lifecycle is declarative (`Permanent` / `Transient{conditions}` / `Suspended`); activation and
dissolution are boundary-protocol work, never mid-tick mutation. If a feature proposal cannot be
expressed as *properties + overlays + accumulator registrations + EML gadget trees*, the proposal is
wrong — escalate, don't special-case.

---

## 7. Mapping — the Movement-Front automaton over gridcell SimThings

The map is not a system; it is **more tree, run as a cellular automaton.** This is the
**Movement-Front system**: the engine-native realization of Anchor A (Wei,
[arXiv:2602.01651](https://arxiv.org/abs/2602.01651)), in which gridcell/location SimThings surface
their local conditions as a heat map whose values spill across the lattice as propagating,
falloff-shaped fronts.

> **Naming.** *Movement-Front* is the engine name — use it in code, spec, and tests. Wei's underlying
> concept is referred to in prose/design as **STEAD** — *SpatioTemporal Evolution with Attractor
> Dynamics* ([arXiv:2602.01651](https://arxiv.org/abs/2602.01651)) — deliberately **not** "SEAD",
> which carries an unrelated military connotation. Never write "SEAD" anywhere in the repo.

**A `Location` SimThing *is* a gridcell SimThing — the terms are interchangeable, and the spatial
identity is intrinsic.** Every SimThing of kind `Location` carries a **grid coordinate**: a position in
its parent's grid map of child cells. Owning that coordinate and participating in the **spatial arena**
is what `Location` *means* — it is **not** an optional "mapping-role" bolted onto an otherwise-ordinary
node. "Gridcell," "Location SimThing," and "spatial-arena participant" name the same thing; there is no
non-spatial `Location`. This is **not a new `SimThingKind`** — `Location` is the kind, and the gridcell
identity is what that kind already carries. The engine holds no fixed roster of grid kinds; *what* a
Location is — a star system, a planet-surface tile, a sector — is scenario data, never engine knowledge.
Future agents: treat "is a `Location`," "is a gridcell," and "occupies a coordinate in the spatial
arena" as one statement, and never demote a Location to a position-less abstract node.

**The parent owns the grid; its child Locations are the cells; the arena-pressure projection seeds the
field — this is the load-bearing column formation.** A parent SimThing lays out its child Location
SimThings positionally as a grid map. Each child Location occupies a `(width, height, col)` slot — a
**3-axis layout**: the spatial position of the cell (`width × height`) crossed with the per-cell
**column stack** (`col`) — so the GPU stencil walks neighbors as pure index arithmetic. The hard-won
integration this formation exists for is the **arena-pressure projection** (`ArenaPressureBindingSpec`):
a child Location is a resource-flow arena participant; its subtree reduces and disburses into its flow
columns (§3, §5); and an authored binding projects that resolved flow — `(arena, sub_field) →
(target_id, row, col)` — **onto the Location's own grid cell** as the RegionField seed. The projection
is **on-device and GPU-resident — no readback, no side-channel test map**: arena state *shapes the
field* directly, so the suppression / threat / supply pressure a Location accumulates *is* what seeds
its cell. The seeded `RegionField` is its own bounded column range (its `source_col` / `target_col`),
evolved by the Movement-Front stencil — distinct from the arena flow column it is *projected from*, not
a duplicate of it. The whole `RegionFieldSpec` carries the three-layer model in one struct: **L1** =
`pressure_binding` (this seed) + operator / horizon / `alpha_self` / `gamma_neighbor` (the falloff);
**L2** = `reduction` (cell → parent); **L3** = `parent_formula` (`ai_will_do` urgency) + `commitment`
(threshold → `CommitmentEffectSpec` → `BoundaryRequest`). (The parent also reduces its cells the way
every parent reduces every child — `SlotRange` over their columns — that L2 half.) Execution is opt-in
(`MappingExecutionProfile::SparseRegionFieldV1`, default `Disabled`).

**Layout scale and execution scale are decoupled — and SimThing models VAST spatial domains.** The
**gridcell-Location lattice LAYOUT** (each Location's structural `(col,row)`) is integer and sparse, so it
scales **freely** to far larger than any single reference — **200×200 is a *small* map; vast lattices
(thousands+ cells per edge) are anticipated** and lay out at full fidelity (occupied cells where the
scenario places them, unoccupied cells carrying ambient field). The **dense Movement-Front stencil
EXECUTION**, by contrast, is a **bounded local theater** (the implemented first slice is ≤ 10/32 cells per
edge — P1: dense-global diffusion over a vast grid is the permanently-rejected pattern). A vast lattice is
therefore covered by **many bounded theaters** — the multi-theater **atlas** rung — never by one giant
dense field; strategic awareness across theaters is **hierarchy (Layer 2)**, not a bigger stencil. **Never
shrink a layout to fit the theater cap, and never grow the theater to cover a vast layout:** the layout is
authoritative and unbounded; the stencil is a bounded window; the atlas tiles. (Lowerer: the gridcell
`(col,row)` is honored as authoritative layout at any edge — STEAD-PRIVILEGE-0 — while the Movement-Front
front honestly defers to the atlas above the bounded-theater edge.)

**A cell is shaped by its neighbors — falloff is the spatial arena's flow.** Exactly as a flow-arena
participant is shaped by the budget disbursed to it, a gridcell Location is **influenced by the falloff
of nearby gridcell Locations**: the stencil (§7.2; Gu-Yang) spreads each cell's value across its
bounded neighborhood, so a Location's resolved state is its own seeded value **plus the falloff reaching
it from neighboring cells**. The moving contour where opposing falloffs meet **is the front**. This is
the spatial-arena analogue of reduce-up / disburse-down: seed a cell from its subtree, let
bounded-horizon falloff carry it to its neighbors, and read the resulting gradient as the heat map.

**Base canonical grid dimensions are always square** (P2 symmetry has no preferred axis): the default
"medium" grid is **200×200**, scaling up — staying square — when cell density demands more than the
default holds. A grid's cells are occupied as the scenario requires; unoccupied cells carry ambient
field. (The superseded 200×150 is retired.)

### 7.1 The three postulates, enforced as engine law

The Movement-Front discipline is not a performance preference; it is Wei's three postulates as
binding constraints. Every mapping rule in `invariants.md` is one of them in disguise:

- **P1 Locality — the light cone is the horizon cap.** A cell's next state depends only on its
  stencil neighborhood; fronts advance at finite speed — H hops per tick within a band, and across
  cells by **later-band cascade, never same-band chaining**. Dense global diffusion as a
  strategic-awareness mechanism is **permanently rejected** (measured ~15× over budget): widening
  the horizon to "see further" is action-at-a-distance, and the cure is hierarchy (Layer 2), not a
  bigger light cone.
- **P2 Symmetry — one shared rule, every cell, every tick.** All cells evolve under the **same
  generic `StructuredFieldStencilOp` kernel with the same authored weights** — no per-cell bespoke
  rules, no coordinate-dependent logic, no semantic WGSL. This is what makes a rule learned/tuned on
  one region valid on every region and at every map scale; a per-cell special case breaks
  generalization exactly as the paper predicts.
- **P3 Stability — attractor dynamics, not raw accumulation.** Operators are stability-bounded
  (`normalized_stencil` / `source_capped_normalized`; raw additive blows up, clamped additive loses
  the gradient); recurrent formulas carry the bounded-feedback contract (finite decay < 1, explicit
  clamps); ping-pong buffers keep multi-hop propagation race-free; and **threshold crossings are the
  discrete projection** — continuous field noise below the threshold is dissipated, and only a real
  crossing becomes an event. A front *settling* (suppression balancing disruption, a contested
  boundary stabilizing) is the automaton converging to an attractor — and an unconverged race
  (production vs attrition still unresolved at tick 100) is simply a wave still traveling.

The automaton is also **adaptive in computation, exactly as the anchor predicts**: cadence tiers and
dirty macro-region skipping mean compute follows the wavefront — quiet regions cost nothing, and a
front crossing the whole map takes the ticks it takes. Sources are caller-managed one-shot seeds
(seed, then zero); horizon is capped (H ≤ 8 tactical, ≤ 16 gated).

### 7.2 The three layers

```
Layer 1 — the Movement-Front heat map (local, bounded falloff)
  StructuredFieldStencilOp evolves cell field columns (threat, disruption,
  suppression, supply reach, desirability) across the 2D lattice. Values SPILL
  ACROSS the map with falloff; the falloff gradient IS the signal, and the
  moving contour where opposing pressures meet IS the front.

Layer 2 — collection (reduce up)
  Cell columns Sum-reduce into parent columns (system → planet → faction →
  session) on an earlier OrderBand. Strategic awareness is hierarchy reduction,
  never a wider stencil (P1).

Layer 3 — interpretation (EvalEML at the parent)
  On a LATER band, the parent runs personality-weighted EML gadget trees (§4.1)
  over its reduced columns: aggression/risk-tolerance sub-fields × pressure →
  urgency/desirability. Band ordering is binding: reduce before interpret.
```

One Movement-Front heat map, three consumers — none with its own engine:

- **AI:** faction personality weights overlay the gradient field; commitments (attack, reinforce,
  withdraw, expand) fire as **`Threshold` + `EmitEvent` crossings** over the Layer-3 pressure
  columns — the P3 projection applied to strategy. There is no CPU map planner; the AI never
  traverses the grid. It *reads the front* and acts when a pressure crosses a named threshold.
- **Pathfinding / movement:** agents steer **proportionally down/up the local gradient** of the
  desirability/threat front (EML over neighbor cells); movement is column updates and arena
  re-enrollment, not a route solver. The front *is* the route: a supply front's gradient is the
  logistics path, a threat front's gradient is the avoidance path. Velocity-of-pressure uses an
  explicit previous-value column with a copy band (EML has no previous-buffer read) — the inferred-
  dynamics half of the cell schema (§3).
- **State:** the resolved field columns *are* the world state — disruption fronts, supply reach,
  contested boundaries — reduced up for display and strategy alike.

Perception (fog, stale intel, confidence, deception) is per-observer **filter fields over the true
front** — same machinery, bounded formulas, with a hard write-boundary: perceived columns never write
back into true columns; only explicit gameplay events through the `BoundaryRequest` path update
ground truth. Mapping is opt-in, bounded, default-off, and `simthing-sim` remains map-free: it sees
flat columns and opaque registrations.

**Production operators — the realized rule (Gu-Yang flux) and the reach utility (PALMA).** Two seated,
semantic-free GPU operators give the automaton its production form, each a generic
`StructuredFieldStencilOp`-family utility, not a new primitive or a semantic engine:

- **Gu-Yang `SaturatingFlux`** — an engineering ansatz *inspired by* Gu & Yang's hydrodynamic-limit
  results (arXiv:2509.20797), not a literal implementation — is the conservative, state-dependent
  stencil rule: a symmetric `(C_i + C_j)/2` flux with **zero-*flux*** (not zero-value) boundaries and a
  CFL cap (χ ≤ 0.25), so a front **saturates and chokes** at bottlenecks instead of blowing up. It is
  the same kernel, the same authored weights at every cell (P2), stability-bounded (P3) — and it is the
  operator that makes chokepoints and contested boundaries *emerge from the flow* rather than from a
  bespoke border service. The optional choke readout is one resident scalar column in the same dispatch.
- **PALMA** min-plus traversal (tropical algebra, arXiv:2601.17028) is the seated **reach/impedance
  utility** over the front: `D = W + min(N4 D)` is a *field*, not a route — it realizes "the front is
  the route" (§7.2) as the reach metric a supply/threat gradient implies. No sqrt, no predecessor, no
  path object; it is a generic GPU utility a Movement-Front consumer composes (impedance W from choke
  fields → D), never a pathfinding engine.

### 7.3 Trade-off geometry over the front — the Pareto-knee toolkit (deferred; guidance only)

**Status: not an opcode, not a gadget, not implemented anywhere — deferred under the consumer-pulled
discipline** (product deferral pre-dating this document; re-adjudicated 2026-06-09, horizon charter
§1.4: opens no consumer of its own). This section exists so the path is *known*, never re-derived
or re-excavated, when a consumer names it.

**The concept.** A faction's policy weights (allocation weight columns + threshold biases, §6) span
a trade-off front over its Layer-3 objectives — the personality-weighted pressure columns (§7.2).
The **knee** is the operating point of *least maximal change*: where a small reallocation toward
any objective forces a disproportionate deterioration in another, formalized as the **MCF** — the
max over objective pairs of sensitivity-norm ratios (Giovannelli/Raimundo/Vicente,
arXiv:2501.16993). A **knee event** is the front *kinking* — trade-off geometry sharpening past
threshold — and threshold cascades nucleate exactly at kinks. The internal case is already
doctrine: owner-entity fission (§2.1) is the intra-faction knee.

**Why it costs no new substrate (expressiveness, not a planted feature):** the objectives already
exist (Layer-3 reduced columns); the actions already exist (the weight columns overlays modify);
sensitivities are **measured, never derived analytically** — difference paired counterfactual runs
(bit-exact replay + one admitted overlay changed) or within-run policy dither; analytic derivatives
are unavailable regardless, because the dynamics are clamped/gated/nonsmooth, and the interesting
knees are precisely the nonsmooth ones. Over measured sensitivity columns the **MCF is plain
ratio/max algebra — an ordinary EML gadget tree** under the existing admission contract
(CPU-oracle parity, node cap, bounded feedback), and a knee event is an ordinary
`Threshold` + `EmitEvent` crossing on the MCF column. No new opcode, no new WGSL, no CPU planner.

**How Movement-Front consumers use it (when opened):**
- **Operating-point preference** — risk tolerance compiles to *distance-from-knee*: a cautious
  personality hedges toward the knee (the worst-case-protected point); an opportunist deliberately
  rides the steep face of the front.
- **Cascade early warning** — a diverging MCF marks where threshold chains will nucleate, pairing
  with the velocity columns as leading indicators (external betrayal knee / internal fission knee,
  §2.1).
- **Label generation** — measured knees are certification-grade labels; any learned estimator over
  them is `ApproximateDiagnostic` forever.

**What is actually missing (the gate):** the MCF gadget is one spec-layer admission away; the
**sensitivity-production harness** (paired-run differencing / dither) is the unbuilt part and stays
gated on a named consumer (intervention/dataset rungs, or a scenario that needs knee-aware policy).
Full adjudication lives in `workshop/field_world_model_horizon.md` §1.4 — link out, don't restate.

---

## 8. Time, decisions, and the CPU's only job

- **tick** = deterministic GPU substrate advancement; **boundary** = the synchronization point
  (`day_index` is a monotonic counter, not a calendar); the sim advances only when the host asks.
- **Decisions are GPU-resident threshold crossings** (`Threshold` + `EmitEvent` → `BoundaryRequest`)
  over resolved, masked fields — the FIELD_POLICY model, which is Movement-Front P3 applied to
  agency: a decision is the automaton's projection of continuous pressure onto a discrete attractor
  state. No CPU planner, no CPU urgency traversal, no CPU commitment emission, ever.
- **At a boundary the CPU consumes, never recomputes.** It reads resolved summaries, events, and
  metadata; applies structural results (fission, fusion, expiry, reparenting, re-enrollment) through
  `BoundaryProtocol`; and reads GPU-integrated values before any lifecycle decision. It must not
  re-derive economy/threat/urgency and must not scan dense grids by default.
- Structural change is boundary work: fission/fusion from property thresholds, slot scrubbing on
  add/remove, tombstoning whole-tree-scoped. The evaluator never mutates the tree.

---

## 9. The drift detectors — litmus tests for every change

A change is drifting from the paradigm the moment any answer below is "yes." Stop and escalate;
do not rationalize.

1. Am I adding a **runtime `match` on `SimThingKind`**, or a new kind variant, to get behavior?
2. Am I building a **subsystem beside the tree** (a combat/economy/AI/pathfinding service) instead of
   properties + overlays + registrations *on* the tree?
3. Am I making an owner/faction a **spatial parent**, or implementing capture as **reparenting**
   instead of a column flip?
4. Am I writing a **CPU planner** — any CPU code that traverses state to *decide* something the GPU
   should resolve as a threshold crossing?
5. Am I putting **gameplay semantics into WGSL or `simthing-sim`** (any map/faction/AI/scenario word
   below the spec layer)?
6. Am I creating a **second ledger** beside `Balance`, or hardcoding a column index outside
   `PropertyLayout`?
7. Am I adding a **new policy enum** where an overlay on a weight column is the answer?
8. Am I violating a **Movement-Front postulate**: widening a stencil horizon or adding global/
   same-band action-at-a-distance for awareness (P1 — use hierarchy reduction); writing per-cell or
   coordinate-dependent rules instead of the one shared kernel (P2); or authoring unbounded
   accumulation/recurrence without decay, clamp, and the bounded-feedback contract (P3)?
9. Am I proposing a **new opcode, kernel, or scripted-event subsystem** before expressing the
   behavior as an **EML gadget tree** over the existing `EvalEML` interpreter (§4.1's extension
   ladder)?
10. Am I claiming exactness **without bit-exact CPU-oracle parity**, wiring something **default-on
    without a gate**, or allocating GPU resources **per tick**?
11. Am I about to ship a **flattened proxy** for a specified recursive structure without an approved
    Deviation Record — or claim progress through documents instead of a real reduction under test?
12. Am I adding a **rebellion / civil-war / coup system, flag, or special entity type** — instead of
    an influence flow + aggregate threshold + owner-entity fission with an intensity-vector column
    partition (§2.1)?

**The six-line harness** (cite on every track, hold in context on every rung):

1. Everything is a SimThing — new behavior is SimThings + properties + overlays + registrations.
2. All conflict/opportunity/ambition/extraction is resource flow: accumulate → reduce → mask → threshold.
3. Allocation is recursive through the one tree; settling depth is emergent, never special-cased.
4. Decisions are GPU-resident threshold crossings — FIELD_POLICY, never a CPU planner; the map is the
   Movement-Front automaton (locality, symmetry, stability — arXiv:2602.01651).
5. `simthing-sim` and WGSL are semantic-free; behavior is EML opcode-stack data over one interpreter
   (arXiv:2603.21852); exact claims carry bit-exact CPU-oracle parity.
6. Proven only through a real reduction; opt-in / default-off; documents record progress, never constitute it.

When the PR ladders descend into allocator details, atlas batching, JIT kernels, spec admission, or
any other ancillary service — and they will — **this is the document you climb back up to.** Every
service exists only to keep one recursive, GPU-resident SimThing tree accumulating, reducing,
masking, crossing thresholds, and propagating Movement-Fronts. Build toward that, or escalate.

---

## References

- Zichao Wei, *On the Spatiotemporal Dynamics of Generalization in Neural Networks*
  ([arXiv:2602.01651](https://arxiv.org/abs/2602.01651)) — Wei's **STEAD** (*SpatioTemporal Evolution
  with Attractor Dynamics*) concept; the locality / symmetry / stability postulates and
  attractor-dynamics cellular-automaton architecture underlying the Movement-Front system (§1.1, §7).
- Andrzej Odrzywołek, *All elementary functions from a single operator* (arXiv:2603.21852) — the
  single-operator (`eml(x,y) = exp(x) − ln(y)`) universality result underlying the `EvalEML`
  interpreter, the gadget library, and the JIT compiler discipline (§1.1, §4.1).
- Giovannelli, Raimundo, Vicente, *Pareto sensitivity, most-changing sub-fronts, and knee solutions*
  (arXiv:2501.16993) — the least-maximal-change knee / MCF formalization behind the deferred §7.3
  toolkit.
- Gu & Yang, hydrodynamic-limit results (arXiv:2509.20797) — the inspiration for the `SaturatingFlux`
  conservative-flux stencil ansatz that gives the Movement-Front its saturation/choke dynamics (§7.2).
- *PALMA: A Lightweight Tropical Algebra Library for ARM-Based Embedded Systems* (arXiv:2601.17028) —
  the min-plus / tropical-algebra basis for the seated reach/impedance traversal utility over the
  front (§7.2).

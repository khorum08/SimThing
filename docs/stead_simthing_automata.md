# STEAD SimThing Automata (normative anchor)

**Status:** ANCHOR. Authored 2026-08-02 in an Owner design session, covering every change made
*after* the last graduated rung (5.9c `ARTIFACT-PROVENANCE-CONTAINMENT-0`, merged `2e3afacb`).

**Purpose.** This document is the bible for the SimThing-as-automaton work. It exists because the
design was settled in conversation across many exchanges, and conversation does not survive. Where
this document and a memory disagree, **this document wins**. Where this document and the ladder
(`design_0_0_8_7_rf_arena_modernization.md` §3b) disagree, **the ladder wins** — it is machine truth
and carries the graduation stamps.

**How to read it.** §1–§4 are the design. §5 is the shared-surface ledger and is the most useful
section for anyone implementing. §6 is what is genuinely new. §7 records **corrections and rejected
shapes** — read it before proposing anything, because several obvious-looking designs were tried and
found wrong, and re-proposing them costs a cycle each time.

---

## 1. The thesis

A SimThing should be a **minimum viable Wei automaton**: the base recursive stem cell carries, in its
root object, everything needed to emit information, receive information, and act on it. All
specialised SimThings inherit that capability. It is **inert by default** — a population cohort, an
empty gridcell, and a session root all carry the same machinery and pay nothing until activated.

The map is already a grid of gridcell-SimThings run as a cellular automaton (core design §7). A CA's
update rule is **local and closed**. Today the rule contains a CPU round-trip, which is a foreign
step inside what is supposed to be self-contained. Closing that loop is not exotic machinery; it is
**removing an interposition that was never part of the model**.

## 2. The four legs

| leg | meaning | state |
|---|---|---|
| **participate** | full member of spatial RF triads, reduce-up and disburse-down | **exists** |
| **act** | resolve incoming quantity against an authored cost | **planned** (6.1b CostBand) |
| **originate** | produce, hold, and project overlays to children | **2 of 3 exist** (hold, project) |
| **receive** | perceive events from its own subtree | **absent — nothing** |

A SimThing today can emit, act, and flow, but it **cannot listen**. That is a cell with an outbox
and no inbox, and it is the single reason Phase 6 cannot close as written.

## 3. Events are RF

The event system maps onto the RF triad's existing bidirectionality. This is the central structural
claim of the whole design:

- **reduce-up carries PERCEPTION** — what happened below
- **disburse-down carries DIRECTIVE** — what to do

Both channels are built and running. The event-aware stem cell therefore needs **no new transport**;
it needs a SimThing that knows how to participate in both directions.

### 3.1 Three delivery modes, three existing homes

| mode | trigger | mechanism | cost |
|---|---|---|---|
| **deficit-driven** | consumer surfaces a command deficit | disburse-down | free — rides the sweep |
| **standing** | value inherited, never consumed | resolution walk (the `resolve_owner` shape) | zero for inheritors |
| **predicate broadcast** | "all children of A with property B ≥ X" | one tree walk | paid by the initiator |

The third rides neither RF channel and must be named as its own mode — it is a **push by predicate**,
not a pull by deficit, and it is the only one that does not fall out of RF.

### 3.2 Conservation is not weakened

A directive that is **consumed** is a resource and conserves. A **standing** directive that is never
consumed is **not a resource** — it is an inherited property, *read* rather than *moved*. Modelling a
non-consumed value as a resource would put a non-conserving quantity inside the conservation judge,
which is an Invariant Set member. Two mechanisms, both already built, chosen by whether the thing is
consumed.

### 3.3 The cascade is bounded by the sweep, not by a parameter

Receive → originate → receive is a cycle. It cannot run away, and **no authored depth bound is
needed**, because:

- neither `owner_silo_disburse_down` nor `runtime_local_allocation` contains any convergence loop —
  both are **single pass**
- disbursement is **depth-banded**: `max_disbursement_band(layout)` →
  `band_layout.disburse_band(max_depth - 2, max_depth)`

Each direction runs **exactly once per generation**, so a full cycle takes one generation and is
bounded by `O(tree depth)`. The cycle is **paced by generations** — a directive disburses down this
generation and its effect reduces up next. That is how a CA paces itself, and an authored bound
would only add a parameter that can be set wrong.

## 4. CostBand

**`CostBand`** — one word, camel-humped. Canonical spelling in prose, code, and authoring.

A CostBand is **the definition of a resource sink**, not a mode a threshold opts into. The threshold
**is** the unit cost:

```
N = floor(V / C)     units afforded / destroyed
R = V − N·C          remainder, carried forward
V = N·C + R          exact, always
```

Conservation-preserving by construction, and pure algebra — one divide, one floor, one multiply-
subtract, no branching.

- **Every sink IS a CostBand.** There is no second sink mechanism. Stating it as an opt-in invites a
  rival sink to appear for something that "didn't need the quotient", which is how this capability
  fragmented the first time.
- **Booleans are a CostBand of depth 1.** This *removes* a branch rather than adding a case: there is
  no did-it-fire path, only how-many, with boolean as the degenerate `N ∈ {0,1}`.
- **Direction, not sign.** Production and destruction are the same algebra with opposite application.
  `output_coefficient < 0.0` **fails closed by admission** and must stay that way; the coefficient
  remains units-per-cost and an authored direction says accrete or deplete.
- **Observation is the base case.** A crossing costs nothing and consumes nothing — that is already
  observation. **Action is observation with a CostBand attached.** `VelocityAlert` is not a different
  species of event; it is a crossing nobody hung a CostBand on.

## 5. The shared-surface ledger (the piggyback)

Every surface below already exists and already earns its keep. The automaton adds a **second use**,
not a second mechanism. This table is the implementation map.

| surface | current use | shared second use |
|---|---|---|
| `PlacedParticipant` + `StructuralCoord` | spatial RF participation | **event spatial location — derived from origin, never stamped** |
| `resolve_owner` walk (6.0) | ownership resolution | **standing-directive inheritance** — same walk, absence means inherit |
| `is_ownership_crossing`, `OwnerChannelRfCrossingFlow.boundary_simthing_id` (6.0e) | STEAD crossing records | **staleness seeds and event boundary detection** |
| `reduce_owner_channel_rf` | RF surplus/deficit totals | **the `V` for CostBand — the sweep already summed it** |
| `owner_silo_disburse_down`, `runtime_local_allocation_from_disbursement` | resource disbursement | **directive delivery to command deficits** |
| `order_band`, `max_disbursement_band` | allocation ordering | **"closest automaton" determinism — authored, never tree-distance** |
| `SimThing.overlays`, `Overlay.affects` | policy/governance holding | **overlay hold and project — 2 of 3 legs already present** |
| `TransformOp {Add, Multiply, Set}` | static overlay math | **already a 3-instruction EML** (`ADD`, `MUL`, `LITERAL_F32`) |
| `accumulator_op.wgsl` EML interpreter (`var<storage, read>`) | accumulator programs | **all steering — compiles nothing; per-call instantiation is already free** |
| `BoundaryRequest` (7 verbs, sealed ingress) | structural mutation | **the complete action vocabulary — CostBand emits into it** |
| `BandCrossingDelta.threshold()` / `.post_value()` | crossing readout | **both CostBand operands, already sealed** |
| 13-stage boundary pipeline | allocation | **survives untouched in both resolution modes** |
| per-stage `_ms` telemetry | diagnostics | **6.2b's prerequisite measurement, available today** |

## 6. What is genuinely new

Only five things. Everything else is the ledger above.

1. **Reception.** A SimThing perceiving events from its subtree. Nothing exists.
2. **`Overlay.origin: SimThingId`, required.** Not new provenance — **60 construction sites already
   hold their originating node in scope and discard it**. Authored overlays originate from the
   ScenarioThing, and that is itself useful corpus. `OverlaySource` is retained and complementary:
   origin says *which node*, source says *what kind of will* (`Player` vs `Ai` on the same
   OwnerThing is a real distinction and is the intervention signal).
3. **Path routing.** Up to the common ancestor, then down — **so the overlay stack filters along the
   path**. Direct `affects` targeting bypasses every intermediate policy layer and is the thing that
   would break the model, not a cheaper version of it.
4. **The CostBand quotient.** The divide that drifted out. Both operands are already sealed.
5. **EML opcodes on overlays.** Giving an existing 3-instruction language its full instruction set.

## 7. Corrections and rejected shapes — READ BEFORE PROPOSING

Recorded so drift is detectable and rejected designs are not re-proposed.

**Rejected: opt-in-upward event sensitivity.** The DA proposed a sparse sensitivity property with
events propagating *up* to ancestors that declared interest. **Wrong derivation** — it was taken from
how CPU handlers subscribe to event kinds, i.e. from the system being replaced. The correct
derivation is from how RF moves: directives disburse **down**, effects reduce **up**. Both directions
already exist.

**Rejected: sink-vs-observation as a taxonomy.** The DA named six semantics as "OBSERVATIONS", making
a category with a fixed list. A taxonomy **enumerates**, so it invites "which category is my new
event?" and goes stale at the seventh semantic. Observation is the **base case**, not a sibling
category.

**Rejected: CostBand as an opt-in marker, default off.** Stating it as a flag invites a rival sink
mechanism later. It is the **definition** of a sink.

**Rejected: negative `output_coefficient` for destruction.** Admission fails closed on negatives
deliberately. Use **direction**, not sign.

**Rejected: an authored cascade depth bound.** Unnecessary — the sweep is single-pass and
depth-banded, so the cycle is already bounded by generation pacing.

**Rejected: parity needs an Invariant Set amendment for independent subtrees.** It does not. Parity
was always a property of *the tree being executed*; there was simply never more than one.

**Corrected: the flat `&mut [f32]` CPU shadow is not a single-generation defect.** It is flat *within*
a tree, which is correct. Mixed generations arise only if one shadow is forced to serve independently
ticking subtrees.

**Corrected: CostBand's placement.** Originally authored at 7.0. Separating the action *semantic*
from the action *transport* (6.2) across a phase boundary was a placement error. Phase 6 is the event
phase **end to end**: ingress → transport → action → resolution site.

## 8. Resolution site and the vendorized build

The **closed loop is the default execution model**; today's CPU-authoritative system is a
**vendorized build** derived from it (§4 Vendorized Build Principle). Built the other way round, the
incumbent stays load-bearing indefinitely and every capability is authored against the specialisation
first.

These are **not two systems** — one model, two resolution sites. Same math, same vocabulary,
different placement, which is why indistinguishability is provable rather than aspirational.

**Why it is smaller than it looks:** the current dispatch is mostly **identity re-attachment**, not
decision logic. `VelocityAlert`'s entire arm is a registry lookup rebuilding a struct — it exists only
because `ThresholdEvent {slot, col, value, event_kind}` is flat and the CPU puts SimThing identity
back. **A closed loop stays in slot space and never crosses that bridge, so those arms evaporate
rather than port.** `FissionTrigger` is the exception: it sizes an allocation, which stays at the
barrier in both modes. Seven of the 13 boundary stages are allocation machinery that survives
untouched.

**Riskiest assumption, test first:** slot-space identity must suffice for everything the closed loop
does. The moment something in-shader needs a `SimThingId`, the bridge returns and the mode difference
stops being mere placement.

**Prerequisite — measure before building.** Per-stage telemetry already exists. Readback over total is
the **upper bound** on what closing the loop can save; the allocation stages are the **floor** it can
never save. If allocation dominates, the architectural case must carry the work alone — legitimate,
but it must be known rather than assumed.

## 9. Governing laws

Defined in `design_0_0_8_7_rf_arena_modernization.md` §4; listed here so this document does not
restate and drift from them.

- **Vendorized Build Principle** — the general model is the default; specialisations are instances.
- **Definable Horizon Law** — no lifecycle may claim permanence; explicit removal is always ordinary.
- **Capability Binding Law** — a rung introducing a capability binds every downstream consumer, naming
  the forbidden fallback.
- **Purge Reconciliation Law** — a purge enumerates what it invalidates and reconciles it in the same PR.
- **Mechanisms-Not-Domains** — the engine names mechanisms, never domain activities.
- **Vendor Containment** — a derived crate never imposes its semantics beyond its own crate.
- **Transient Fixture Law** — a fixture that leaves its crate stops being transient.
- **Per-Tree Instantiation** — independent subtrees need no new engine mechanism.

## 10. Open questions

- **Do pressures/directives conserve?** Standing orders as stocks and one-shot commands as flows is
  expressible, but it makes command bandwidth finite and contested. That is either a wanted feature or
  an accidentally imported constraint, and should be decided deliberately.
- **Overlay EML program length.** Overlay application becomes O(program length); a one-node program
  stays static-priced, but an unbounded authored program applied per-slot-per-overlay in the hot loop
  is the failure mode and needs a cap.
- **Micro-subtree GPU contexts.** Per-tree cost is small (`SpecSessionState` is 14 fields of mostly
  empty collections; `SlotAllocator` needs no device), so thousands are feasible — *provided* they are
  CPU-only or share a context. Per-tree GPU contexts would disqualify the pattern.

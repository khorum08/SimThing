# StemThing Intrinsic Overlay Capability
## Working design for closing RF, STEAD/PALMA, CostBand, ActionBand, and Overlay into one recursive SimThing automaton

> **Status: WORKSHOP / UNANCHORED / OWNER-DIRECTION IN DEVELOPMENT.**
>
> This document is deliberately housed under `docs/workshop/`. It is the working surface for the proposed final closure of the base recursive **StemThing**: the thesis that the base SimThing is the sole semantic owner of overlay origination/emission, retention, reception, filtering/projection, activation/suspension, lifecycle, and dissolution, and that ordinary simulation action is expressed by that intrinsic overlay capability rather than by a peer event-execution or behavior subsystem.
>
> **This document does not amend the 0.0.8.7 ladder, does not supersede an anchor, and does not authorize implementation by itself.** Where it conflicts with an existing anchor or graduated rung, the existing authority wins until this workshop is adjudicated and promoted.
>
> Intended promotion path, if the design survives review:
>
> ```text
> docs/workshop/StemThing_Intrinsic_Overlay_Capability.md
>                     ↓ Owner / DA review
>              completed design candidate
>                     ↓ promotion
>>                           docs/
>                     ↓
>       doctrine anchors + 0.0.8.7 amendment
> ```
>
> Primary repository companions:
>
> - [`../stead_stemthing_unification.md`](../stead_stemthing_unification.md) — StemThing base recursive object, four-leg/lane unification, residency/derivation law.
> - [`../stead_simthing_automata.md`](../stead_simthing_automata.md) — minimum viable Wei automaton, four legs, events-as-RF, CostBand, overlay hold/project archaeology.
> - [`../multi-axis-ActionBand-STEAD.md`](../multi-axis-ActionBand-STEAD.md) — intrinsic GPU ActionBand, target/displacement/stakes, recursive GPU lowering, PALMA interaction, existing band-crossing reuse.
> - [`../simthing_core_design.md`](../simthing_core_design.md) — core GPU/RF/overlay/STEAD/PALMA architecture and authority laws.
> - [`../stead_spatial_contract.md`](../stead_spatial_contract.md) — spatial/STEAD/Movement-Front contract.
> - [`../design_0_0_8_7_rf_arena_modernization.md`](../design_0_0_8_7_rf_arena_modernization.md) — live workplan; ladder authority wins over this workshop.
>
> Primary implementation archaeology:
>
> - [`../../crates/simthing-core/src/simthing.rs`](../../crates/simthing-core/src/simthing.rs)
> - [`../../crates/simthing-core/src/overlay.rs`](../../crates/simthing-core/src/overlay.rs)
> - [`../../crates/simthing-core/src/automaton.rs`](../../crates/simthing-core/src/automaton.rs)
> - [`../../crates/simthing-core/src/property.rs`](../../crates/simthing-core/src/property.rs)
> - [`../../crates/simthing-kernel/src/overlay_prep.rs`](../../crates/simthing-kernel/src/overlay_prep.rs)
> - [`../../crates/simthing-sim/src/overlay_lifecycle.rs`](../../crates/simthing-sim/src/overlay_lifecycle.rs)
> - [`../../crates/simthing-sim/src/tree_mutation.rs`](../../crates/simthing-sim/src/tree_mutation.rs)
> - [`../../crates/simthing-sim/src/gpu_sync.rs`](../../crates/simthing-sim/src/gpu_sync.rs)
> - [`../../crates/simthing-driver/src/automaton_reception.rs`](../../crates/simthing-driver/src/automaton_reception.rs)
> - [`../../crates/simthing-feeder/src/work.rs`](../../crates/simthing-feeder/src/work.rs)
> - [`../../crates/simthing-feeder/src/patcher.rs`](../../crates/simthing-feeder/src/patcher.rs)
> - [`../../crates/simthing-spec/src/compile/overlay.rs`](../../crates/simthing-spec/src/compile/overlay.rs)

---

## 0. Executive thesis

The candidate closure is:

> **The StemThing is the simulation cell. All ordinary action is expressed by the StemThing originating, retaining, projecting, modifying, suspending, or dissolving admitted overlays. CostBand and ActionBand do not become peer executors; they authorize changes in intrinsic overlay state. Those overlays in turn deform ordinary SimThing properties, RF lanes, STEAD fields, PALMA impedance/valuation, and target conditions until the authored goal/lifecycle condition resolves.**

This does **not** mean every quantity becomes an RF-conserved resource and does **not** mean an overlay is a second mutable world-state object beside properties.

The intended distinction is:

```text
Property / governed value
    = ordinary state

RF
    = conserved / constrained flow and allocation

STEAD
    = propagated field state / pressure / causal circumstance

PALMA
    = reach / impedance / value-potential solve over admitted field state

ActionBand
    = unresolved target discrepancy and its lifecycle/progress bands

CostBand
    = exact quantization of affordable/executable work

Overlay
    = intrinsic actuation state emitted by a SimThing

BoundaryRequest
    = sealed structural consequence when numerical state requires tree mutation
```

Everything participates in the same recursive SimThing automaton, but these mechanisms keep their distinct mathematics.

### 0.1 The proposed complete recursive loop

```text
                            STEMTHING
                                │
                           participate
                                │
                  Properties / RF / STEAD state
                                │
                    reduce-up / disburse-down
                                │
                       PALMA field potential
                                │
                              act
                                │
                    ActionBand / CostBand
                                │
                   sealed threshold crossing
                                │
                    EML authorization/value
                                │
                            originate
                                │
                    intrinsic Overlay state
                                │
                 route / filter / inherit / apply
                                │
                             receive
                                │
              Property / RF / STEAD / target deformation
                                │
                         next generation
                                │
                                └──────────► STEMTHING
```

No CPU behavior tree, action dispatcher, movement executor, combat executor, event execution engine, or domain action taxonomy is required by this model.

### 0.2 Why this is a closure rather than a new subsystem

The repository already contains most of the pieces:

- `SimThing` already directly owns `overlays: Vec<Overlay>` and already has the base hold operation.
- standing overlays already inherit recursively without descendant copies;
- routed overlays already derive origin → common ancestor → target policy paths;
- RF disbursement already has a production route that terminates a delivered directive in `SimThing.overlays`;
- overlays already lower into property transforms consumed by the GPU AccumulatorOp path;
- CostBand already defines sink/action quantization;
- ActionBand is being implemented as the intrinsic `act` facility over the existing Phase-5 crossing machinery;
- generation pacing already prevents same-generation recursive convergence.

The proposed new architectural act is therefore **authority concentration**: move the remaining overlay lifecycle and action-emission semantics behind the StemThing germ rather than letting feeder/driver/sim layers retain peer overlay authorities.

---

## 1. Authority vocabulary used in this workshop

To prevent this working document from accidentally promoting hypotheses into law, statements are classified as follows.

| Tag | Meaning |
|---|---|
| **INHERITED LAW** | Already governed by an existing anchor / graduated mechanism. This workshop consumes it. |
| **OWNER-DIRECTED** | Owner has explicitly directed the architectural intent in the design session, but promotion still requires normal corpus process. |
| **WORKSHOP CANDIDATE** | Proposed interpretation/design that must survive DA/engineering review. |
| **RESEARCH CANDIDATE** | Promising physical optimization; not required for semantic closure. |
| **REJECTED** | Considered and deliberately excluded from the proposed architecture. |

---

## 2. StemThing anatomy: the four legs remain complete

**INHERITED LAW:** StemThing has four legs and no fifth leg:

```text
SimThing
 ├─ participate
 ├─ act
 ├─ originate
 └─ receive
```

Residency, derivation, RF participation, ActionBand, and overlay capability are lanes/behaviors flowing through these four legs, not additional anatomy.

The proposed overlay closure gives the four legs a particularly clean interpretation:

| Leg | Candidate closed-loop meaning |
|---|---|
| **participate** | expose ordinary property/RF/STEAD state to recursive reduce/disburse/field evaluation |
| **act** | ActionBand and CostBand determine unresolved discrepancy and executable amount |
| **originate** | emit/activate/update the StemThing's intrinsic overlays as the numerical actuation consequence |
| **receive** | receive routed/inherited overlays, filter them through policy/route law, and expose their admitted transforms to local property/RF/STEAD evaluation |

This recovers the old automaton archaeology in `stead_simthing_automata.md`: `originate` was already described as **produce, hold, and project overlays**; hold and project existed, while production/origination remained incomplete.

### 2.1 Default inertness

**INHERITED LAW / OWNER-DIRECTED:** the base recursive SimThing must remain default-inert.

The semantic presence of overlay capability on every StemThing must not imply a hot per-SimThing overlay object, a behavior loop, a CPU scheduler entry, or a mandatory per-generation scan.

Expected physical principle:

```text
no active overlay
    → no overlay instance row
    → no active binding span
    → no lifecycle work
    → no PALMA dirtying from overlay state
    → essentially zero hot-loop cost
```

The germ is universal; instantiated numerical state is sparse and pay-for-play.

---

## 3. Overlay is proposed as the intrinsic actuation language

**OWNER-DIRECTED / WORKSHOP CANDIDATE:**

> **An action is the activation, parameterization, modification, suspension, or dissolution of an admitted overlay whose numerical consequences are realized through ordinary SimThing substrate.**

A shorter but slightly less precise slogan is “overlays are action.” The preferred formulation is:

> **Overlays are the intrinsic actuation language/state of SimThing.**

The distinction matters because an overlay is not necessarily an instantaneous mutation. It may remain active over many generations and continuously deform values/weights/fields until its own lifecycle condition is satisfied.

### 3.1 CostBand and ActionBand stop at overlay authorization

Candidate responsibility split:

```text
ActionBand
    why / toward what / when a discrepancy should be reduced

CostBand
    how much executable work the cleared resources afford

Overlay
    what numerical actuation remains active as a consequence

RF
    constrained means / resource allocation

Property / STEAD
    resulting state and field
```

Thus a normal ActionBand or CostBand consequence should prefer:

```text
activate/update OverlayTemplate K with parameters P
```

over bespoke world mutation.

Structural effects remain exceptions in the existing sense: a numerical consequence can authorize a sealed `BoundaryRequest`, but the CPU structural boundary applies topology mutation without re-evaluating the action decision.

### 3.2 Action does not mean all state is RF-conserved

**REJECTED:** “everything is RF” if interpreted as “every property must obey RF conservation.”

The stronger and correct statement is:

> **Everything participates in the recursive SimThing flow architecture; only actually conserved/constrained quantities use RF conservation semantics.**

Examples that are not automatically conserved RF resources:

```text
location coordinate
political disposition
threat field
semantic axis coordinate
target coordinate
policy
ActionBand displacement
PALMA potential
```

Movement points, food stock, material stock, capacity, or a deliberately finite command bandwidth may be RF resources. A coordinate or semantic field is not converted into a resource merely because an overlay changes it.

---

## 4. Current overlay archaeology: what already belongs to the germ

This section records live repository behavior so the eventual design does not accidentally replace working substrate.

### 4.1 Retention/home already lives on `SimThing`

Current base object:

```rust
pub struct SimThing {
    ...
    pub properties: HashMap<SimPropertyId, PropertyValue>,
    pub resource_parent_edges: Vec<ResourceParentEdge>,
    pub overlays: Vec<Overlay>,
    pub children: Vec<SimThing>,
    ...
}
```

and:

```rust
pub fn add_overlay(&mut self, overlay: Overlay) {
    self.overlays.push(overlay);
}
```

This is already the correct semantic home: overlays are owned by the SimThing, not by a global event manager.

### 4.2 Routed attachment already terminates in the ordinary overlay store

`simthing-core::deliver_routed_overlay()`:

1. validates that origin and target belong to the supplied authority tree;
2. derives the origin path and target path;
3. constructs origin → common ancestor → target route identity;
4. applies dispatch-mint lifecycle admission where required;
5. resets `affects` to the target;
6. terminates by `target.add_overlay(overlay)`.

`deliver_deficit_directive()` and `deliver_standing_directive()` reuse this common primitive.

This is a strong precedent for the germ: delivery is **not** a second inbox. `SimThing.overlays` is the inbox/retained actuation state.

### 4.3 Standing inheritance and routed filtering already recurse fractally

`inherit_active_overlays()` extends the inherited `TransformStack` with each node's active overlays. Descendants receive effective standing state by recursion; no descendant copy is materialized.

`LiveOverlayRoutes` builds an ephemeral route view only when routed instructions exist. It derives active policy/governance filters from the current tree on each pass, so policy suspension/dissolution changes the effective route without redelivery.

This is already the fractal behavior sought by this workshop:

```text
ancestor active overlays
        ↓
child receives effective stack
        +
child local overlays
        ↓
grandchild receives effective stack
        +
...
```

### 4.4 RF → Overlay already exists

`simthing-driver::automaton_reception` constructs `CommandDeficit`s, runs them through existing owner-silo disburse-down and runtime-local allocation, and only delivers the attributable overlay when a conserved command unit was actually allocated.

Therefore one direction of the desired closed loop already literally exists:

```text
RF deficit / disbursement
        ↓
allocated directive
        ↓
Overlay retained by receiving SimThing
```

### 4.5 Overlay → RF already exists indirectly through property columns

An overlay does not need a special “enter RF arena” call. Overlay transforms write/deform the same property/weight/rate columns ordinary RF consumes.

Current physical path:

```text
SimThing.overlays
      ↓ CPU preparation today
build_overlay_deltas()
      ↓
per-slot ordered OverlayDelta ranges
      ↓
AccumulatorOp overlay OrderBands
      ↓
Property / RF input columns
      ↓
ordinary RF reduce / settle / disburse
```

This is an important semantic law for the final design:

> **An overlay affects RF by changing the numerical conditions under which the ordinary RF machinery executes; it does not invoke a separate RF API.**

### 4.6 Transform operations are already EML-shaped

`TransformOp` is already a singular admitted EML program representation. `Set`, `Add`, and `Multiply` are degenerate constructors, not a competing static execution language.

That strongly supports using overlay templates as the universal numerical actuator: richer overlay behavior can remain data/program state, not domain kernels.

---

## 5. Current authority fragmentation to be examined/reconciled

The semantic home is already correct; remaining lifecycle/ingress/compilation authority is distributed.

### 5.1 Player/AI feeder paths remain peer ingress machinery

Current feeder code has distinct `PlayerIntentOverlay` and `AiIntentOverlay` types and can:

```text
player / AI intent
       ├─ apply/fold transform as immediate GPU intent / CPU-shadow path
       └─ park overlay for boundary attachment
```

This is historical machinery that should be reviewed against the final germ. The likely desired end-state is:

> player, AI, script, ActionBand, and CostBand may differ in provenance and authorization, but they should not own distinct overlay-activation semantics.

This workshop does **not** yet authorize deletion; it identifies a probable convergence target.

### 5.2 Overlay lifecycle is still CPU boundary authority

`simthing-sim::resolve_overlay_lifecycle()` currently:

- walks the SimThing tree;
- checks `Transient` / `UntilDissolvedWith` conditions;
- reads GPU-fresh values through the CPU shadow;
- decrements `AfterTicks` counters;
- removes dissolved overlays from `node.overlays`;
- applies property `on_expire` effects to the CPU shadow.

That means current lifecycle authority is still approximately:

```text
GPU numerical state
       ↓ readback
CPU shadow
       ↓
CPU lifecycle evaluator
       ↓
remove/update Overlay objects
       ↓
recompile overlay tables
       ↓
GPU
```

If intrinsic overlay capability is promoted under the same GPU-authority law as ActionBand, this is the largest obvious peer authority to extract.

### 5.3 Activation and suspension are structural-boundary operations today

Current `BoundaryRequest` includes:

```text
AttachOverlay
ActivateOverlay
SuspendOverlay
```

and `simthing-sim::tree_mutation` directly rewrites `OverlayLifecycle` for activation/suspension.

This workshop asks whether ordinary numerical activation/suspension should instead become GPU-resident overlay state transitions, leaving CPU boundary work only for semantic persistence/structural storage changes.

### 5.4 Explicit generic overlay dissolution/removal should be re-audited

The corpus law says explicit removal is ordinary under the Definable Horizon doctrine, but the currently obvious `BoundaryRequest` vocabulary exposes attach/activate/suspend rather than a symmetrical generic overlay-removal verb.

Automatic lifecycle removal exists. The eventual germ should explicitly prove how an authored or action-resolved overlay can be dissolved without bespoke removal paths.

### 5.5 `OverrideReceived` handling requires archaeology before promotion

`overlay_lifecycle.rs` treats `OverrideReceived` as false during ordinary condition evaluation with a note that attachment handles it. The common routed-attachment primitive should be verified to actually own the promised override transition. This workshop does not assume the loop is already closed.

---

## 6. Candidate intrinsic overlay germ

The semantic StemThing capability should own five behaviors, not five services:

```text
OVERLAY GERM

originate
    activate/parameterize an admitted overlay

receive
    accept an attributable routed/inherited overlay

project
    expose effective overlay transforms into the SimThing's property/RF/STEAD lanes

lifecycle
    active ↔ suspended → dissolved under admitted conditions

collapse
    retire resolved transient numerical state, preserving only ordinary resulting world state
```

### 6.1 Proposed semantic law

> **Nothing outside the SimThing owns an overlay's active lifetime. External systems may author templates, supply directives, or apply genuinely structural storage changes at a boundary, but originate/receive/project/activate/suspend/dissolve are one intrinsic StemThing capability.**

This is a candidate law, not yet an anchor.

### 6.2 Collapse law

The ActionBand collapse principle generalizes naturally:

```text
overlay active
      ↓
ordinary state changes
      ↓
authored lifecycle/goal condition resolves
      ↓
overlay dissolves
      ↓
ordinary world state is the durable memory
```

Do not retain a completed-action object solely to remember that the action happened unless an authored persistent property/history demands that semantic memory.

### 6.3 Generation pacing remains authoritative

Overlay emission must preserve the one-generation recursive law:

```text
generation t
    ActionBand / CostBand crossing authorizes OverlayNext change

barrier / current-next swap

generation t+1
    new overlay participates normally
```

No same-generation emit → apply → re-cross → re-emit convergence loop.

---

## 7. Proposed GPU physical model

**WORKSHOP CANDIDATE:** semantic ownership by every StemThing does **not** imply a rich overlay object physically embedded in every hot row.

The preferred physical interpretation mirrors ActionBand:

> **StemThing owns overlay semantics; sparse GPU tables own active numerical overlay instances. CPU objects/names are semantic shadow and admission/persistence representation.**

Possible shape:

```text
OverlayTemplate[]
    immutable admitted shape
    EML program / transform program
    lifecycle predicate program
    projection/filter law
    target/binding schema
    parameter schema

OverlayInstance[]
    owner logical slot
    origin logical slot
    template id
    binding span
    parameter span
    lifecycle state
    generation stamp / temporal state where admitted

OverlayStateCurrent[]
OverlayStateNext[]

OverlayBinding[]
    instance → target slot/property/role or inherited/routed projection descriptor
```

### 7.1 Human-readable designations remain CPU semantic shadow

Consistent with ActionBand authority:

```text
GPU
    numeric ids
    logical slots
    columns
    EML programs
    lifecycle bits/state
    parameter vectors
    binding spans

CPU semantic shadow
    display name
    authoring name
    source file/span
    history / diagnostics
    UI labels
    persistence mapping
```

Human-readable overlay names must not become runtime dispatch keys.

### 7.2 Overlay taxonomy should not become GPU behavior taxonomy

Current `OverlayKind` / `OverlaySource` contain useful author-facing provenance such as Policy, Governance, Player, AI, System, Event.

A future intrinsic-germ implementation should examine whether GPU numerical behavior can instead be described by orthogonal admitted descriptors:

```text
projection/filter law
lifecycle law
transform EML
origin/target binding law
provenance metadata (shadow only where possible)
```

**WORKSHOP QUESTION:** Policy/Governance currently influence routed predicate composition and runtime dispatch-mint admission. Promotion should determine which distinctions are true numerical laws and which are merely semantic labels.

### 7.3 Sparse program/profile batching

The default physical target should preserve homogeneous GPU work:

```text
bucket by OverlayTemplate / EML program / binding shape
        ↓
execute large coherent batches
        ↓
instance parameters vary per row
```

Trivial `Set/Add/Multiply` overlay chains should be eligible for composition/fusion where exact authored order is preserved.

For sequential affine transforms:

\[
x' = a x + b
\]

with:

```text
Multiply(m) : a=m, b=0
Add(c)      : a=1, b=c
Set(c)      : a=0, b=c
```

compatible chains can be folded to one `(a,b)` pair without inventing a new semantic execution path. More general EML remains shared admitted program execution.

---

## 8. Alignment with RF, STEAD, PALMA, CostBand, and ActionBand

### 8.1 RF: constrained means

RF retains conservation authority. Overlay output can:

- change production/consumption rates;
- change allocator weights;
- change claim pressure/priority through admitted numerical lanes;
- activate/deactivate demand conditions;
- change constrained-flow topology only through the already lawful structural boundary when required.

An overlay does not bypass claim → clear → disburse.

### 8.2 STEAD: field composition

The intended field ordering is:

```text
ordinary SimThing / RF state
        +
active overlays
        ↓
resultant STEAD field / causal landscape
```

Overlays are ordinary contributors/deformers of the field. They are not post-hoc private ActionBand corrections that evade global field solution.

### 8.3 PALMA: exposes route/value over the composed field

The current design synthesis is:

> **STEAD describes/composes the field. Overlays deform it. PALMA resolves the lawful reach/impedance/value potential over that field. ActionBand consumes the resulting potential; it does not run its own graph search.**

Where a semantic factor materially affects route choice, prefer including that factor in PALMA's admitted impedance/terminal valuation before the potential is solved rather than applying a purely local greedy correction afterward.

### 8.4 ActionBand: unresolved discrepancy

ActionBand remains the intrinsic `act` facility. It observes current state and admitted target form, derives discrepancy/progress/stakes, consumes PALMA potential where topology exists, and uses the existing sealed Phase-5 crossing machinery.

Candidate closure:

```text
ActionBand band crossing
        ↓
EML result
        ↓
pre-admitted overlay emission/update binding
        ↓
OverlayStateNext
```

ActionBand does not become a peer lifecycle owner for the emitted overlay.

### 8.5 CostBand: executable amount

CostBand remains the exact sink/quantizer:

\[
N=\left\lfloor\frac{V}{C}\right\rfloor,
\qquad
R=V-NC.
\]

Candidate closure:

```text
RF clear V
    ↓
CostBand produces N
    ↓
parameterize / intensify / repeat overlay actuation by N units
```

CostBand determines how much actuation is affordable; Overlay expresses that actuation into ordinary state.

---

## 9. Movement as the literal worked example

Movement is not privileged architecture. It is useful because the generic vector/value/velocity model is unusually literal.

### 9.1 Location as ordinary property state

A location property can carry a vector coordinate and governed velocity:

\[
L_t = [x_t,y_t,\ldots],
\qquad
\dot L_t = 0
\]

while stationary.

The destination is an ActionBand target over that same admitted location topology.

### 9.2 Waypoints as ActionBand/threshold bands

The historical design intuition was that location CostBand bands could function as waypoints: movement accrues toward one milestone, crosses it, then progresses to the next.

Under the current sealed crossing machinery, the physically useful scalar observable is PALMA distance/potential to the waypoint:

\[
D_i(L)=\operatorname{PALMA}(L,W_i)
\]

and arrival is:

\[
D_i(L)\le r_i.
\]

Thus the semantic statement “waypoint is a band” remains valid while the Phase-5 crossing machinery continues to operate on an admitted ordered scalar projection.

### 9.3 Movement points fund velocity magnitude

Let cleared movement resource be `M`, unit movement cost `C`, and PALMA provide lawful local descent direction `u`.

\[
N=\left\lfloor M/C\right\rfloor
\]

and candidate velocity is:

\[
\dot L_t = v(N)\,\hat u_t.
\]

Then ordinary governed property integration advances location.

### 9.4 Intrinsic overlay actuation

Candidate final movement shape:

```text
ActionBand target = destination
       ↓
PALMA potential gives lawful local direction
       ↓
RF clears movement resource
       ↓
CostBand yields executable N
       ↓
activate/update MovementActuation overlay
       ↓
location velocity / movement demand lanes are deformed
       ↓
ordinary property/RF evaluation advances location
       ↓
waypoint/destination crossing
       ↓
overlay changes or dissolves
       ↓
ActionBand continues/collapses
```

There is no `MoveFleet()` numerical engine function in the generic architecture.

If location is expressed partly by structural parentage rather than only coordinate columns, numerical action may terminate in a sealed `BoundaryRequest::Reparent`; the structural boundary applies the authorized mutation without becoming movement authority.

---

## 10. Opportunity horizon and local-minimum findings

The workshop discussion distinguished two problems that must not be conflated.

### 10.1 Escape problem

```text
ultimate goal is already known
but locally composed valuation makes current neighborhood appear stationary
```

### 10.2 Opportunity-horizon problem

```text
a much better remote resolution exists
but the local actor has no information about it
```

The second is broader and should not be solved by giving ActionBand a private search tree.

### 10.3 Candidate Opportunity Horizon Principle

> **An ActionBand remains a local consumer of fields. Awareness of nonlocal opportunities is supplied by ordinary GPU field propagation. Increasing opportunity horizon means expanding/accelerating the admitted field's propagation or hierarchy, not increasing an agent-side search depth.**

### 10.4 Bellman/PALMA composition principle

For a target/value field satisfying a Bellman-style min-plus equation:

\[
J(x)=\min_{y\in N(x)}\left[W(x,y)+J(y)\right],
\]

a reachable nonterminal state on an optimal route has a lawful descending neighbor under ordinary positive impedance assumptions.

False local minima are introduced when a globally solved potential is subsequently deformed by an inconsistent purely local preference field.

Candidate guidance:

> **Semantic factors that materially alter route choice should, where feasible, enter the admitted PALMA edge/terminal valuation before the value field is solved.**

### 10.5 Opportunity-valued multi-source field

A promising formulation is to seed remote admissible resolutions with semantic opportunity value `Q_a(g)` and solve cost plus terminal value together:

\[
J_a(x)=\min_g\left[B_a(g)+\operatorname{Dist}_{W_a}(x,g)\right]
\]

where, for example,

\[
B_a(g)=-\lambda Q_a(g).
\]

This means a local descent of `J_a` can carry information propagated from a remote opportunity set without requiring an actor-owned graph search.

**Important correction:** assigning different terminal opportunity values intentionally changes the objective. It is not, by itself, the policy-invariant reward-shaping theorem of Ng/Harada/Russell.

---

## 11. Research evaluation: papers, mathematics, and disposition

This section records the external ideas evaluated during the workshop and how they are being used—or not used—in SimThing.

### 11.1 Potential-based reward shaping — useful analogy, corrected claim

**Paper:** Andrew Y. Ng, Daishi Harada, Stuart Russell, *Policy Invariance Under Reward Transformations: Theory and Application to Reward Shaping* (ICML 1999).

- Author bibliography / publication record: <https://aima.cs.berkeley.edu/~russell/resume.html>
- Common theorem form:

\[
F(s,s')=\gamma\Phi(s')-\Phi(s).
\]

**What is useful:** potential-difference shaping is a strong reminder that arbitrary local corrections can change behavior, while differences of a globally coherent potential have special invariance properties.

**What was rejected/corrected:** the theorem does **not** prove that setting PALMA terminal conditions to `-λ Q(g)` preserves some prior route/objective. Different terminal values intentionally make some resolutions more valuable than others.

**Promising but deferred:** a true potential/gauge reparameterization of a shortest-path/value solve may be useful as a preconditioner. For a state potential `h`:

\[
W_h(u,v)=W(u,v)+h(v)-h(u)
\]

with terminal adjustment:

\[
B_h(g)=B(g)-h(g).
\]

Along a path from fixed start `x`, the added potentials telescope, shifting all candidate route values by the same `-h(x)` term. This can preserve minimizers when admissibility/nonnegative-impedance requirements are satisfied.

Potential use: warm-start residual solving with `h ≈ J_{t-1}`. **Not required for overlay-germ closure; research-only until PALMA baseline exists and consistency constraints are proven.**

### 11.2 Hamilton–Jacobi / Bellman interpretation — retained as mathematical framing

The PALMA min-plus recursion is naturally understood as a discrete dynamic-programming/value-function relation:

\[
J_i=\min_j(W_{ij}+J_j).
\]

This is closely related to discrete Hamilton–Jacobi / Eikonal value propagation and tropical/min-plus algebra.

**Included because:** it explains why “the field is the route/value function” and why an ActionBand can consume a local descent without owning a predecessor tree or A* search.

**Caution:** SimThing PALMA is not automatically every form of continuous HJB control problem. The analogy is strongest where the admitted topology/cost model really satisfies the corresponding Bellman/min-plus fixed-point assumptions.

### 11.3 FIM — retained as a high-priority PALMA physical optimization

**Original paper:** Won-Ki Jeong, Ross T. Whitaker, *A Fast Iterative Method for Eikonal Equations*, SIAM Journal on Scientific Computing 30(5), 2008.

- DOI/publisher: <https://epubs.siam.org/doi/10.1137/060670298>
- PDF landing: <https://epubs.siam.org/doi/pdf/10.1137/060670298>

The method manages active nodes and iteratively updates only nodes not yet locally converged, avoiding expensive globally ordered structures. It was specifically designed for parallel/SIMD execution.

**GPU-oriented improvement:** Yuhao Huang, *Improved Fast Iterative Algorithm for Eikonal Equation for GPU Computing* (2021).

- arXiv: <https://arxiv.org/abs/2106.15869>

This work further emphasizes active/remedy sets that do not require a special global update ordering, making parallel GPU implementation practical.

**Why included:** FIM does not replace PALMA semantics. It is a candidate physical lowering for solving/repairing the PALMA/Eikonal-like value field while preserving local parallel updates.

The SimThing-specific opportunity is stronger than a one-shot solver because overlays provide exact dirty provenance:

```text
Overlay change
    ↓
known affected field binding/locus
    ↓
seed active PALMA tiles
    ↓
relax only causally affected regions
```

Instead of clearing and re-solving the entire field each generation, retain `J_t` and incrementally repair it into `J_{t+1}`.

### 11.4 SimThing FIM physical candidate: sparse active tiles, dense local math

**RESEARCH CANDIDATE:** do not copy textbook dynamic active lists literally if that would introduce atomic append ordering or nondeterministic queue semantics.

Prefer:

```text
TileActiveCurrent[]
TileActiveNext[]

active tile
    → dense regular N4/N8 PALMA relaxation in workgroup
    → if tile boundary changes materially, mark adjacent tile in Next

barrier
    → swap masks
```

This deliberately performs some redundant work inside a dirty tile to preserve coalesced memory access and GPU occupancy.

Expected hybrid execution posture:

```text
small dirty fraction
    → tiled FIM / incremental repair

large dirty fraction / cold start
    → dense PALMA sweep
```

The switch is a physical optimization only; both lower the same admitted PALMA semantics.

### 11.5 Multiscale FIM — retained as promising opportunity-horizon acceleration

**Paper:** Jingqi Zhang, Zihao Zhou, Lixin Ren, Junyuan Liu, Ying Li, Xiaowei He, *A parallel multiscale FIM approach in solving the Eikonal equation on GPU*, Computer-Aided Design 189 (2025), 103949.

- Publisher/DOI landing: <https://www.sciencedirect.com/science/article/pii/S0010448525001101>
- DOI: <https://doi.org/10.1016/j.cad.2025.103949>

The paper extends GPU-friendly single-grid FIM using multiscale V-cycles so long-range information can propagate faster than one fine-grid spacing per iteration.

**Why included:** it gives a much stronger, mathematically coherent answer to “expand the opportunity horizon” than making ActionBand perform bounded graph search or subgroup peeking.

Preferred interpretation:

```text
coarse relaxation of the SAME PALMA problem
        ↓
prolongate coarse value estimate
        ↓
fine-grid PALMA/FIM refinement
        ↓
final fine field remains authoritative
```

**Rejected interpretation:** blur/smooth the field and let the smoothed result authorize behavior that the fine field forbids. Multiscale should accelerate convergence to the authoritative fine solution, not replace it with an advisory approximation that can erase real chokepoints/prohibitions.

### 11.6 MeshFIM / irregular topology — retained as future evidence

**Paper:** Zhisong Fu, Won-Ki Jeong, Yongsheng Pan, Robert M. Kirby, Ross T. Whitaker, *A Fast Iterative Method for Solving the Eikonal Equation on Triangulated Surfaces*, SIAM Journal on Scientific Computing.

- Publisher: <https://epubs.siam.org/doi/10.1137/100788951>
- Open article mirror: <https://pmc.ncbi.nlm.nih.gov/articles/PMC3360588/>

**Why included:** it is evidence that the FIM idea is not intrinsically tied to one perfectly regular rectangular grid. SimThing should still prefer the simplest structured lowering for ordinary Location lattices, but irregular/derived topologies do not automatically invalidate the method family.

### 11.7 Reaction–diffusion / operator splitting — retained as execution analogy, not ontology proof

**Classic reference:** Alan Turing, *The Chemical Basis of Morphogenesis* (1952). Historical publication information is summarized by Cambridge at:

- <https://www.cambridge.org/core/books/alan-m-turing/morphogenesis/C7287DF8C5923FA02F7671FD6A9CEA1A>

The useful systems analogy is local reaction plus spatial diffusion producing complex pattern without central planning.

A closer GPU engineering reference is operator-split reaction/diffusion work, for example:

- *Accelerating the Finite-Element Method for Reaction-Diffusion Simulations on GPUs with CUDA*: <https://pmc.ncbi.nlm.nih.gov/articles/PMC7569852/>
- *Multi-Dimensional, Mesoscopic Monte Carlo Simulations of Inhomogeneous Reaction-Drift-Diffusion Systems on Graphics-Processing Units*: <https://pmc.ncbi.nlm.nih.gov/articles/PMC3323590/>

**Included lesson:** local pointwise “reaction” and spatial transport have different execution geometries and are profitably split into separate parallel operators.

Candidate SimThing analogy:

\[
\text{generation update}
\approx
\mathcal R_{overlay/EML}
\circ
\mathcal T_{RF/STEAD/PALMA}
\circ
\mathcal A_{ActionBand/CostBand}.
\]

**Rejected claim:** SimThing is literally a Turing reaction-diffusion system. RF conservation, target discrepancy, structural mutation, and min-plus routing are broader than classical reaction-diffusion PDEs. The analogy is architectural, not a proof of semantics.

### 11.8 Closed-loop feedback / PID — retained only as a design analogy

Movement and other ActionBands are closed-loop feedback controllers in the broad sense:

```text
measure discrepancy
    → determine affordable control effort
    → actuate via overlay
    → world changes
    → measure again next generation
```

Displacement can resemble a proportional term; velocity can resemble derivative information; an explicitly authored accumulated frustration/stakes state can resemble integration.

**REJECTED:** making PID universal ActionBand semantics. Not every ActionBand requires an integral term, derivative term, or PID tuning law.

### 11.9 Deterministic “thermal frustration” / barrier tolerance — retained as authorable overlay pattern, not core solver

A stalled ActionBand can detect something like:

\[
\|D\|>0,\qquad \dot D\approx 0
\]

and author a transient escalation overlay that increases willingness to pay or changes impedance weights over time.

This can model deterministic barrier-tolerance escalation:

```text
stall band crossing
    ↓
emit ExplorationPressure / urgency overlay
    ↓
ordinary field/RF valuation changes
    ↓
progress resumes
    ↓
overlay dissolves
```

**Correction:** this is not stochastic simulated annealing unless actual stochastic exploration/noise is present.

**Included because:** the intrinsic overlay emitter makes such adaptive response ordinary without adding an escape-planner subsystem.

### 11.10 Subgroup “horizon probing” — rejected as semantic architecture

Proposal evaluated: when stalled, use GPU subgroup shuffles to inspect neighbor/neighbor-of-neighbor candidate loci and overwrite the ActionBand target.

**REJECTED for semantics:**

1. subgroup lanes are not inherently topological neighbors; work mapping would have to make them so;
2. evaluating 8 candidates is not automatically the same cost as evaluating 1—register pressure, gathers, and arithmetic remain real;
3. most importantly, ActionBand would become a planner/search owner that silently mutates its own target based on local sampling.

Subgroup operations remain lawful **physical implementation tools** for a field kernel when useful. They are not part of ActionBand meaning.

### 11.11 Tropical/min-plus matrix view — retained, with an implementation warning

PALMA/Bellman relaxation has the semiring-linear-algebra form:

\[
D'_i=\min_j(W_{ij}+D_j).
\]

This can be viewed as min-plus/tropical matrix-vector multiplication.

**Included because:** it explains why PALMA belongs naturally in the same broad GPU numerical/table architecture as the rest of SimThing.

**Critical warning:** algebraically matrix-shaped does not imply that the ordinary spatial lowering should materialize a dense matrix or use conventional GEMM.

For N4/N8 structured lattices the likely efficient physical lowering is:

```text
neighbor stencil gather
+ local add
+ min reduction
+ ping-pong write
```

Dense tropical matrix multiplication may become interesting for coarse strategic graphs or other dense relationships. Ordinary tensor-core `FMA` units do not natively implement exact `min,+`; replacing authoritative min with softmin/log-sum-exp merely to reach MMA hardware would change semantics and is currently rejected.

---

## 12. Deeper GPU optimization opportunities exposed by intrinsic overlays

### 12.1 Overlay emission can become the dirty-set generator

If overlay templates/bindings are admitted and resident, activation already knows which fields/properties it can change.

Therefore:

```text
OverlayStateNext changes
        ↓
binding table names affected field loci/columns
        ↓
mark STEAD/PALMA tiles dirty directly
```

No global “what changed?” scan is required.

This is one of the strongest reasons FIM is included in this workshop: intrinsic overlay actuation and incremental PALMA repair are causally compatible by construction.

### 12.2 Temporal warm start

Retain the previous converged/settled PALMA potential:

```text
J[t]
 + localized overlay/field changes
        ↓
repair
        ↓
J[t+1]
```

Quiet regions should pay essentially zero PALMA work under the FIM posture.

### 12.3 Hybrid dense/sparse PALMA execution

FIM bookkeeping can lose to a regular dense sweep when nearly every tile is dirty or the field is being initialized.

Candidate runtime physical choice:

\[
\rho=\frac{\text{dirty tiles}}{\text{all admitted tiles}}
\]

```text
ρ small    → tiled FIM
ρ large    → dense PALMA sweep
```

The crossover is empirical and hardware/profile dependent. It must not become gameplay semantics.

### 12.4 Shared PALMA profiles / many right-hand sides

Actors should not automatically own unique full routing fields.

Many SimThings may share:

```text
same theater
topology
impedance projection profile
target/opportunity class
policy/personality program shape
```

Candidate optimization:

\[
D\in\mathbb R^{N_{cells}\times K_{profiles}}
\]

with coherent batches across a small number of shared profiles. Individual ActionBands bind to a profile and carry only truly instance-specific parameters/corrections.

This mirrors ActionBand program/profile batching and is likely more important than forcing literal dense GEMM.

### 12.5 Operator fusion on local overlay reaction

Overlay EML is pointwise/sparse; RF/STEAD/PALMA are transport/reduction/stencil-like. Treating these as operator phases suggests aggressive fusion within the local overlay reaction phase while keeping spatial/recursive propagation separate.

Candidate generation pipeline:

```text
1. current → previous snapshot where required
2. local active-overlay EML / transform reaction
3. RF / STEAD reduce-disburse-field operations
4. PALMA dense or FIM/multiscale relaxation
5. ActionBand target/progress evaluation
6. existing sealed crossings + EML
7. CostBand quantization / RF commitments
8. OverlayStateNext activation/update/lifecycle
9. generation barrier / swap
```

Exact OrderBand placement remains an engineering/admission problem; this workshop asserts only the dependency direction.

---

## 13. What is explicitly not being proposed

The following shapes are rejected or fenced from the intrinsic-overlay design:

1. **No fifth StemThing leg.** Overlay capability belongs to originate/receive and is driven by act/participate.
2. **No `ActionThing` peer entity.** ActionBand remains intrinsic to the acting SimThing.
3. **No overlay event manager/service.** The SimThing overlay store/table is the retained actuation surface.
4. **No CPU per-generation overlay evaluator** as final numerical authority.
5. **No CPU child/action scheduler** for recursive ActionBands.
6. **No human-readable OverlayKind/Action name as GPU dispatch key.**
7. **No second band-crossing machine.** Existing Phase-5 crossing substrate remains singular.
8. **No A* / predecessor / route-object simulation authority.** PALMA produces a field/potential.
9. **No ActionBand subgroup search semantic.** Subgroups may optimize field kernels only.
10. **No claim that all properties are RF resources.** Conservation remains scoped to conserved quantities.
11. **No smoothed/coarse PALMA field overruling the authoritative fine solution.** Multiscale is a solver acceleration.
12. **No dynamic GPU template authoring.** Overlay and ActionBand program shapes are admitted before execution; runtime activates/parameterizes admitted templates.
13. **No same-generation recursive convergence loop.** Current/next state and generation pacing remain mandatory.
14. **No assumption that FIM always beats dense PALMA.** Cold starts, small theaters, and field-wide changes may favor dense sweeps.
15. **No premature promise that structural hierarchy is automatically the correct multigrid hierarchy.** Geometry must justify restriction/prolongation relationships.

---

## 14. Candidate full StemThing germ

A useful semantic model for review—not yet a Rust layout—is:

```text
StemThing
│
├─ ordinary sparse Property state
│
├─ recursive structural children / parent relationships
│
├─ RF participation bindings
│
├─ intrinsic ActionBand facility (normally inert)
│
└─ intrinsic Overlay facility (normally inert)
    │
    ├─ admitted overlay template bindings
    ├─ active instance bindings
    ├─ receive / route / filter
    ├─ project to Property/RF/STEAD
    ├─ activate / suspend / parameterize
    └─ dissolve / collapse
```

Physical GPU state may be entirely out-of-line/sparse even though semantic ownership is intrinsic.

### 14.1 The recursive simulation identity

If the closure holds, a recursive SimThing tree is no longer a model operated by several engines. It is a family of identical inert-capable cells whose ordinary numerical relationships generate the simulation:

```text
state
  ↓
flow / field
  ↓
value potential
  ↓
discrepancy
  ↓
affordable action
  ↓
overlay actuation
  ↓
state
```

Specialized domain behavior is authoring over properties, templates, EML, RF lanes, field channels, targets, and thresholds—not a new runtime subsystem.

---

## 15. Candidate laws for eventual promotion

These are **workshop candidate laws**, intentionally not doctrine yet.

### 15.1 Intrinsic Overlay Ownership Law

> Every active overlay is semantically owned by a StemThing. No peer subsystem owns overlay lifetime or numerical execution.

### 15.2 Overlay Actuation Law

> Ordinary numerical action resolves by activating, parameterizing, modifying, suspending, or dissolving an admitted overlay. Direct numerical world mutation outside admitted property/RF/field/overlay/boundary surfaces is not a second action mechanism.

### 15.3 Sparse Inert Germ Law

> Overlay capability is universal in semantics but sparse in physical instantiation. An inactive StemThing incurs no hot overlay scan or mandatory per-instance program state.

### 15.4 RF Distinction Law

> Overlay actuation may deform RF inputs and claims, but only genuinely conserved/constrained quantities become RF resources. Semantic/value fields remain nonconserved unless separately authored otherwise.

### 15.5 Field-Before-Route Law

> STEAD plus active overlays compose the actionable field. PALMA solves the admitted reach/impedance/value potential over that composition. ActionBand consumes the solved potential rather than inventing a private route search.

### 15.6 Overlay Lifecycle Authority Law

> Overlay activation, suspension, temporal state, and dissolution are part of the intrinsic overlay facility. CPU state may remain semantic/persistence/structural shadow but may not become the ordinary numerical lifecycle evaluator once the GPU-resident form is admitted.

### 15.7 Generation-Paced Actuation Law

> An action authorized in generation `t` changes overlay next-state; the resulting overlay participates no earlier than the ordinary next generation/barrier. No same-generation recursive action convergence.

### 15.8 PALMA Physical-Lowering Law

> PALMA semantics are independent of solver posture. Dense relaxation, tiled FIM, multiscale FIM, and future lawful accelerators are interchangeable physical lowerings only if they preserve the admitted value-field semantics and deterministic authority.

### 15.9 Opportunity-Horizon Law

> Nonlocal opportunity awareness is a field-propagation responsibility, not an ActionBand search-depth responsibility.

### 15.10 Dirty-Provenance Law

> Where an admitted overlay binding identifies the field loci it can alter, overlay state transitions should directly seed incremental field work rather than requiring a population-wide or field-wide change-detection scan.

---

## 16. Falsifiers / conditions that would invalidate this closure

Promotion should fail or remand if implementation requires any of the following:

1. A CPU behavior/action/event dispatcher is necessary to interpret routine ActionBand/CostBand results.
2. A runtime action cannot be expressed as admitted overlay/property/RF/field/BoundaryRequest consequences without adding domain execution code.
3. Intrinsic overlay ownership requires a per-SimThing hot object allocation even for inactive SimThings.
4. Overlay lifecycle requires continuous CPU mirrors of GPU values/progress after a GPU authoritative representation is introduced.
5. Emitting an overlay requires dynamic creation of new shader/EML/template semantics at runtime rather than activating admitted data.
6. Overlay recursion creates same-generation convergence or an unbounded cascade.
7. A second threshold/band-crossing detector is introduced for overlay lifecycle or ActionBand emission.
8. FIM/multiscale changes the authoritative PALMA answer rather than only its solve posture, unless such approximation is separately admitted as non-authoritative.
9. ActionBand must search or retain predecessor/path graphs to function over PALMA.
10. Human-readable domain nouns become runtime behavior branches in core/kernel/GPU code.
11. A new overlay manager becomes the true owner while `SimThing` retains a ceremonial `overlays` field.
12. Overlay actuation can bypass RF conservation for a conserved resource.

---

## 17. Engineering questions deliberately left open

These are valid workshop questions, not permission to invent answers during implementation.

### 17.1 Overlay instance storage and capacities

- exact template/instance/binding table layout;
- maximum concurrent active overlay capacity and how it consumes residency resources;
- whether instance rows are globally packed or grouped by owner/template/profile;
- epoch rebind consequences for logical SlotIndex references.

### 17.2 Lifecycle lowering

- mapping current `DissolveCondition` vocabulary to GPU EML/threshold registrations;
- `AfterTicks` representation without CPU decrement loops;
- override semantics and deterministic ordering;
- whether activation/suspension is a lifecycle bit, a threshold state, or a degenerate ActionBand/overlay condition;
- structural/persistence readback requirements for semantically durable transitions.

### 17.3 Overlay routing representation

- standing inheritance lowering without duplicating transforms per descendant;
- routed instruction origin→LCA→target filtering in GPU-resident tables;
- whether route bindings are precompiled per epoch or derived from parent tables;
- preserving conjunctive policy predicate law distinct from ordinary sequential value transforms.

### 17.4 Overlay-to-field dirty provenance

- binding granularity: slot, cell, tile, property channel, or field profile;
- duplicate dirty activation handling;
- deterministic current/next tile masks;
- crossover criteria between FIM and dense solve.

### 17.5 Multiscale hierarchy

- whether existing spatial atlas hierarchy can provide lawful coarse grids;
- restriction/prolongation operators;
- handling obstacles/chokepoints without semantic smoothing;
- interaction with bounded theater and atlas scheduling.

### 17.6 Shared PALMA profile cardinality

- profile key definition;
- which overlays require actor-specific impedance vs shared profile deformations;
- when caching a derived opportunity field is cheaper than evaluating EML locally;
- capacity/budget accounting for profile fields.

### 17.7 Structural consequences

- which action results remain numerical overlays indefinitely;
- which must emit a sealed BoundaryRequest;
- how an overlay lifecycle relates to pending structural consequences and replay.

---

## 18. Suggested workshop/probe sequence before promotion

This is not a workplan amendment; it is a research sequence for making the anchor candidate complete.

```text
A. OVERLAY-GERM-ARCHAEOLOGY
   enumerate every attach/activate/suspend/dissolve/apply path
   prove which are semantic duplicates vs genuinely structural

B. GPU-OVERLAY-LIFECYCLE-PROBE
   one admitted transient overlay
   GPU owns active/current/next + PropertyReaches/AfterTicks equivalent
   CPU receives only sparse semantic lifecycle delta

C. ACTIONBAND→OVERLAY-PROBE
   existing sealed ActionBand crossing
   → fixed admitted overlay activation/update
   → next-generation ordinary property/RF consequence
   no direct state write

D. OVERLAY-DIRTY-PALMA-PROBE
   localized overlay changes impedance
   → exact dirty tile seed
   → warm-start tiled FIM repair
   compare bit/semantic result against dense PALMA oracle

E. DENSE↔FIM-CROSSOVER-PROBE
   cold / 1% / 10% / 50% / 100% dirty theater
   identify hardware crossover without semantic mode difference

F. MULTISCALE-FIM-PROBE
   same fine PALMA oracle
   multiscale V-cycle acceleration
   prove no coarse approximation becomes authority

G. FRACTAL-ROUTING-PROBE
   standing + routed + policy-conjunctive overlay behavior
   GPU table lowering across multi-depth StemThing tree

H. PROMOTION REVIEW
   delete/retire peer lifecycle/action authority only after equivalence is proven
```

The probe philosophy should follow existing SimThing practice: temporary witnesses prove or falsify the design and are reaped when their result is promoted into a type/admission/anchor.

---

## 19. Promotion criteria

This workshop is ready to move from `docs/workshop/` to `docs/` only when all of the following are true:

1. **Semantic closure is adjudicated:** Owner/DA agrees that overlays are the intrinsic SimThing actuation state and that ActionBand/CostBand ordinarily resolve into overlay changes.
2. **Authority is explicit:** the document states exactly what numerical overlay state is GPU authority and what remains CPU semantic/structural shadow.
3. **Lifecycle is total:** attach/receive, route/filter, activate, suspend, dissolve, explicit removal/override, and collapse each have one lawful home.
4. **RF boundaries are clear:** overlay deformation cannot bypass constrained-resource semantics.
5. **ActionBand integration is singular:** existing crossing/EML/emission surfaces are reused; no rival event mechanism.
6. **Default inertness is physically credible:** no population-wide overlay scan or mandatory hot storage.
7. **Recursive routing is bounded and deterministic.**
8. **FIM is correctly scoped:** a PALMA physical optimization with a dense oracle/equivalence story, not a new navigation semantic.
9. **Multiscale is correctly scoped:** convergence accelerator toward the fine authoritative field, not coarse behavioral authority.
10. **Current implementation migration is enumerated:** every feeder/sim/kernel overlay authority has a keep/migrate/delete disposition.
11. **Falsifiers remain green:** no CPU planner/lifecycle authority, domain dispatch key, second crossing machine, or same-generation recursive loop is required.
12. **The 0.0.8.7 amendment point is known:** promotion states exactly which current/future rung consumes the germ and which later rungs depend on it.

---

## 20. Working synthesis

The strongest current candidate is:

> **StemThing is a default-inert recursive automaton whose state is ordinary properties and field/resource participation; whose `act` leg is ActionBand plus CostBand; whose `originate` and `receive` legs are one intrinsic overlay capability; and whose nonlocal opportunity/navigation information is supplied by STEAD/PALMA fields rather than by actor-owned search.**

The intended GPU execution pattern is correspondingly uniform:

```text
local table/EML reaction
        ↓
recursive RF / STEAD transport
        ↓
PALMA min-plus value relaxation
        ↓
sparse ActionBand / CostBand threshold response
        ↓
OverlayStateNext
        ↓
generation swap
```

FIM is included because it can turn PALMA from a repeated full-field solve into **incremental causal field maintenance** driven directly by overlay dirty provenance while retaining massively parallel local relaxations. Multiscale FIM is included because it can expand the effective information horizon by accelerating propagation of the **same** value problem rather than granting ActionBand a planner/search tree. Potential-based reward shaping, Turing reaction-diffusion, and PID are retained only at the level their mathematics actually supports: useful structural analogies or possible preconditioning ideas, not proofs that SimThing's ontology follows from those literatures.

If this closure survives implementation scrutiny, the recursive SimThing ceases to be an object interpreted by a collection of simulation subsystems. The recursive SimThing family **is** the simulation: state produces fields; fields expose opportunities and constraints; discrepancies consume constrained means; resolved work emits intrinsic actuation; actuation changes state; and the next generation repeats the same rule everywhere.

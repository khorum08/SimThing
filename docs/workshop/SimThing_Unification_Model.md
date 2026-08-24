# SimThing Unification Model
## First architectural synthesis stub for the unified StemThing kernel

> **Status: WORKSHOP / SYNTHESIS STUB / NON-NORMATIVE.**
>
> This file is the first architectural drawing board for the **final SimThing unification model**. It is intentionally under `docs/workshop/` and does not supersede the 0.0.8.7 ladder, `simthing_core_design.md`, or any graduated anchor. The ladder remains machine truth. This document exists to assemble the now-landed pieces into one readable kernel model before the later composition/exclusivity/canonization rungs make that model constitutional.
>
> **Source posture:** written from the live `0.0.8.7` workplan after the ActionBand, OverlayThing, Field-Triad, CostBand, EML `EXP`/`LN`, contention, convergence, and Vendor-Door work had substantially landed. Statements below are tagged **LANDED**, **IN-FLIGHT / PHASE-11**, or **FORWARD GATE** where that distinction matters.

---

## 0. Executive shape

The target object is not a container around several engines. The target object **is the engine**.

> **A StemThing is a default-inert, recursively composable, GPU-resident simulation cell whose ordinary state, resource flow, field participation, action state, actuation state, contention, observation, and structural consequences are resolved through one shared generation protocol.**

The final architecture is therefore best read vertically:

```text
                     DESIGNER / EMBEDDER / CLAUSESCRIPT
                                  │
                                  │ authored data only
                                  ▼
                         ┌──────────────────┐
                         │   VENDOR DOOR    │
                         │ five verbs       │
                         │ Derive           │
                         │ Populate         │
                         │ Overlay          │
                         │ Bind             │
                         │ Run              │
                         └────────┬─────────┘
                                  │
                                  ▼
                    ┌──────────────────────────┐
                    │       STEMTHING          │
                    │ one recursive sim cell  │
                    │ one generation protocol │
                    └───────────┬──────────────┘
                                │
          ┌─────────────────────┼──────────────────────┐
          │                     │                      │
          ▼                     ▼                      ▼
     ordinary state           means                 fields
      Properties / RF       CostBand            STEAD / PALMA /
      owner / residency     ActionBand          Gu-Yang
          │                     │                      │
          └──────────────┬──────┴──────────────┬──────┘
                         │                     │
                         ▼                     ▼
                 Phase-5 crossings      constrained clearing
                         │                     │
                         └──────────┬──────────┘
                                    ▼
                         CrossingConsequence
                                    │
                  ┌─────────────────┼──────────────────┐
                  │                 │                  │
                  ▼                 ▼                  ▼
          ResidentNextWrite  RoutedOverlayDelivery  StructuralAuthorization
                  │                 │                  │
                  ▼                 ▼                  ▼
            ActionBandNext      receive/filter      BoundaryRequest
            OverlayThingNext    / lifecycle
                  │                 │
                  └──────────┬──────┘
                             ▼
                    generation boundary
                             │
                             └──────────────► next generation
```

The intended end-state is a **closed recursive automaton**, not a subsystem graph.

---

## 1. Architectural status map

This section records the workplan shape that this synthesis assumes.

### 1.1 Landed numerical substrate

**LANDED:**

- intrinsic RF participation from resource-bearing Properties + parent edge;
- object-owned row/slot semantics and logical-vs-physical identity split;
- one generic field-sweep mechanism for STEAD/PALMA/Gu-Yang authored instances;
- exact `EXP` and `LN` primitives behind sealed domain admission;
- arm-independent EML arithmetic semantics including the uniqueness/fusion rule;
- `CostBand` as the singular resource-sink mechanism;
- one Phase-5 `BandCrossingDelta` crossing surface;
- ActionBand admission, sparse GPU execution, Current/Next state, recursive composition, semantic shadow;
- ActionBand direct use of native RF/Gu-Yang realizable progress rather than private throughput solving;
- OverlayThing lifecycle on the existing GPU crossing path;
- one three-arm crossing-consequence substrate;
- one generic derived-span projection/invalidation substrate;
- fractal ancestor-overlay closure at large descendant cardinality;
- contention conservation judge and executed generic constrained clearing;
- AccumulatorOp convergence seal: legacy transfer/emission/RF allocation bypasses deleted;
- Vendor Door five-verb facade;
- ordinary-session FieldSweep execution seam;
- ordinary-session ActionBand execution ingress;
- authored EML exposed through the Vendor Door;
- power-law gadget composition through existing `EXP` + `LN` rather than a new `POW` opcode.

### 1.2 Still not constitutional closure

**FORWARD GATE:** the following are deliberately not claimed by this workshop stub as settled constitutional fact until their workplan gates graduate:

```text
11.3  unified-facility convergence witness
      proves StemThing-A / StemThing-B / ActionBand / OverlayThing carry load together

11.4  unified-ingress exclusivity
      proves no producer or domain can bypass the unified ingress chain

12.1  unrelated-domain portability proof

12.2  core canonization
      rewrites simthing_core_design.md around one StemThing kernel anatomy
```

Therefore this file may draw the intended composed kernel, but where a component still awaits a dedicated whole-system witness it is marked as such.

---

## 2. The StemThing root contract

### 2.1 Shape

A StemThing is a logical simulation identity bound at an epoch to a physical dense-matrix row.

```text
StemThing
    logical_id        : SimThingId
    logical_slot      : SlotIndex / admitted logical binding
    physical_row      : epoch-local binding only
    parent            : logical parent edge
    children          : recursive logical subtree

    Properties[]      : sparse authored / derived state dimensions
    owner binding     : sparse intrinsic property; absence means inherit
    RF membership     : derived from resource Properties + parentage
    anchors           : default-born observation loci

    facility bindings:
        CostBand
        ActionBand
        OverlayThing
        FieldSweep / Field Triad
        residency / granting vocabulary where admitted
```

Physical row identity is not durable semantics. An epoch rebind may scramble rows while logical identity and replay remain unchanged.

### 2.2 Protocol: the four semantic legs

The compact StemThing anatomy remains:

```text
participate
act
originate
receive
```

Interpreted through the landed facilities:

| Leg | Kernel meaning |
|---|---|
| `participate` | expose Property/RF/Field-Triad state; reduce and disburse recursively |
| `act` | evaluate ActionBand discrepancy and CostBand executable work |
| `originate` | emit resident or routed OverlayThing consequences / RF products |
| `receive` | receive disbursement, routed overlays, inherited standing state, stamped seam products |

OverlayThing is not a fifth leg. It is the intrinsic actuation state used by `originate` and `receive`.

### 2.3 Default inertness

The root must remain lightweight:

```text
no resource Property   → no RF work
no active ActionBand   → no ActionBand instance work
no active OverlayThing → no lifecycle work
no field registration  → no field sweep
no async seam          → no staleness field / transport work
no granting activity   → no granting census lanes
```

A resting StemThing is fundamentally still a row plus sparse structural metadata.

---

## 3. One generation: kernel-level view

The constitutional time unit is one **generation**.

A useful first approximation of the unified kernel schedule is:

```text
GENERATION t

  [0] read Current state / prior authoritative values
        │
        ▼
  [1] recursive RF accumulation / owner-channel grouping
        │
        ├─ reduce upward
        ├─ preserve seam holding balances
        └─ expose ordinary anchored quantities
        │
        ▼
  [2] apply resident effective OverlayThing projections
        │
        ├─ standing ancestor policy
        ├─ local transient actuation
        └─ EML-shaped values / weights / parameters
        │
        ▼
  [3] execute derived fields
        │
        ├─ STEAD      non-conserved signal propagation
        ├─ PALMA      min-plus potential / impedance
        └─ Gu-Yang    conservative realizable flux / stall
        │
        ▼
  [4] execute constrained clearing / disbursement
        │
        ├─ trivial clear and contested clear = one path
        ├─ authored resolution program
        ├─ unresolved claim U remains observable
        └─ results disburse downward
        │
        ▼
  [5] unified write door + Phase-5 crossings
        │
        ├─ anchored values mutate
        ├─ BandCrossingDelta derives in-pass
        └─ no second comparator/listener authority
        │
        ▼
  [6] CostBand / ActionBand EML resolution
        │
        ├─ CostBand computes affordable count/remainder
        ├─ ActionBand consumes native RF/Triad observables
        └─ authored EML computes desired consequences
        │
        ▼
  [7] CrossingConsequenceBinding
        │
        ├─ ResidentNextWrite
        ├─ RoutedOverlayDelivery
        └─ StructuralAuthorization
        │
        ▼
  [8] Next-state planes / stamped products complete
        │
        ▼
  ===== generation boundary =====
        │
        ├─ Current/Next swap
        ├─ boundary-only structural mutation
        ├─ CPU semantic identity reattachment where required
        └─ replay schedule / generation stamp recorded

GENERATION t+1
```

This is a workshop ordering diagram, not yet a replacement for the production OrderBand schedule. Later revisions should annotate exact landed OrderBands once 11.3/11.4 have proven the full chain as one composition.

---

## 4. Logical ingress points

The architectural goal is not merely to expose convenient APIs. It is to make **all legal simulation ingress converge on the same intrinsic facilities**.

### 4.1 External/domain ingress: Vendor Door

The public vendoring model is five verbs:

```text
Derive
    define specialization, owner seats, admitted authored laws, field roles

Populate
    build tree, Properties, resource values, queue shape, owner bindings

Overlay
    author settings / policy / directives / transient actuation

Bind
    bind thresholds, CostBands, ActionBands, field registrations, observations

Run
    initialize / execute paced or continuous generations / serialize / replay
```

No vendor owns a scheduler, simulation registry, planner, combat engine, economy engine, modifier engine, or alternative field executor.

### 4.2 Authoring-language ingress

ClauseScript remains the default designer-facing script language, but script vocabulary does **not** survive as runtime authority.

```text
ClauseScript
    modifier strings
    triggered/gated values
    scripted values
    scopes / iterators
    effects
    activities / staged progress
          │
          ▼
     authoring/lowering
          │
          ├─ Properties
          ├─ EML gadget/program identity
          ├─ RF claims / quantities
          ├─ CostBand
          ├─ ActionBand template
          ├─ OverlayThing template/instance binding
          └─ sealed BoundaryRequest authorization
```

The runtime knows mechanisms, never ClauseScript domain nouns.

### 4.3 Operator/player ingress

Operator directives are overlays / price injections, not a command-execution channel.

```text
player intent
    ↓
authored finite order-weight / target state
    ↓
OverlayThing / ActionBand input
    ↓
ordinary fields + RF + clearing
```

This preserves emergence: a strong order dominates ambient prices without bypassing the same physics and clearing used by autonomous actors.

### 4.4 Cross-tree ingress

Cross-tree consequences cannot directly mutate a foreign resident plane.

```text
source tree
   │ stamped product / routed overlay
   ▼
seam holding / transport record
   │
   ▼
destination receive ingress
   │
   ├─ preserves origin attribution
   ├─ preserves authored duration / lifecycle parameters
   ├─ destination establishes receiver-local deadline generation
   └─ ordinary filtering / routing applies
```

Foreign absolute deadlines and foreign physical row identities are invalid carriers.

---

## 5. EML: the kernel ISA

### 5.1 Role

EML is the numerical authoring language shared by the kernel facilities.

It is not one subsystem among many; it is the inspectable specification layer from which both generic interpretation and recognized fused lowerings derive.

```text
EML authored / sealed program
        │
        ├─ CPU reference lowering
        ├─ interpreted GPU lowering
        ├─ SSA/JIT lowering
        └─ recognized fused-kernel lowering

all lowerings implement ONE arithmetic meaning
```

### 5.2 Landed exact arithmetic posture

The arithmetic meaning is arm-independent:

- `ADD`, `SUB`, `MUL`, `DIV`: IEEE-754 single-rounding, no reassociation;
- `MIN`, `MAX`, bounded clamps: exact selections;
- `EXP`, `LN`: sealed exact primitives over admitted domains;
- unique single `MUL` feeding an `ADD`/`SUB`: fused by language definition;
- multiple `MUL` feeds to the same consumer: unfused; no tie-break exists.

This removed the need for open-ended per-consumer exactness policing.

### 5.3 Exact primitive door

`EXP` / `LN` consumers enter by one of two semantics:

```text
1. range-certified input
   → static proof, zero runtime guard cost

2. explicitly guarded semantics
   → clamp/select/MAX is part of the authored law
```

A guard is never retroactively called a proof of the unguarded value.

### 5.4 Gadget layer

The gadget library is authored vocabulary above the opcode set.

Representative classes now include:

```text
FieldSampler
SoftStep
WeightedAccumulator
VelocityMonitor
Decay
EMA
BoundedFeedback
Hysteresis
Acceleration
PowerLaw
```

A power law is authored as a **gadget**, but physically lowers through the existing generating pair:

\[
y = \exp(k\ln x)
\]

No `POW` opcode is required.

Canonical shape:

```text
SLOT_VALUE
CLAMP_BOUNDED     # certify / define positive LN domain
LN
LITERAL_F32 k
MUL
CLAMP_BOUNDED     # define/certify EXP range
EXP
RETURN_TOP
```

---

## 6. RF + Field Triad

The phrase **RF Triad** in this synthesis means the recursive resource-flow substrate together with the three canonical field-law families that consume/shape ordinary StemThing state.

### 6.1 RF: conserved/constrained allocation

RF handles resource participation, claims, grants, deficits, disbursement, and owner-channel segregation.

```text
child state
   ↓ reduce-up
owner/resource/scope bucket
   ↓
authored clearing
   ↓ disburse-down
child consequences
```

Ownership is intrinsic and sparse:

```text
explicit owner at node? yes → use it
                      no  → inherit nearest ancestor
no binding in valid tree → reserved neutral `unowned`
foreign/malformed target → fail closed
```

The resolved owner is not stamped onto all descendants.

### 6.2 STEAD: non-conserved propagated influence

STEAD is the field family for influence/urgency/actionability and other superposed non-conserved signals.

Properties are born as anchored observation loci unless explicitly `Unobserved { reason }`.

Bands quantize **reading**, never field propagation.

```text
continuous field
    ↓
anchored scalar projection
    ↓
band ladder / crossing
    ↓
ActionBand / OverlayThing consequence
```

### 6.3 PALMA: min-plus potential

PALMA computes a potential / impedance field.

```text
D_i = W_i + min_j D_j
```

`D` is a field, not a route object.

No predecessor map, A*, Dijkstra authority, CPU path planner, or path object is admitted in the simulation kernel.

### 6.4 Gu-Yang: conserved saturating flux

Gu-Yang owns physically realizable conservative channel flow and instantaneous saturation/stall.

It supplies the quantity PALMA and CostBand do not:

```text
PALMA     → where progress is desirable / lawful
Gu-Yang   → how much conserved progress can physically traverse now
CostBand  → how much sink/work is paid
ActionBand→ how progress relates to a target lifecycle
```

Signed flux is preserved. `abs(flux)` is not generic progress.

### 6.5 Comparative projections

Derived observables remain projections, never semantic services:

```text
dominance
margin
border identity change
contest / stall
authorized chokepoint projection
```

These become ordinary anchored columns and therefore can participate in the same crossing/action pipeline as any other state.

---

## 7. CostBand: the singular sink law

A CostBand is the definition of a resource sink.

For available value `V` and unit cost `C`:

\[
N = \left\lfloor\frac{V}{C}\right\rfloor
\]

\[
R = V - NC
\]

with:

\[
V = NC + R
\]

exact under the admitted arithmetic law.

### 7.1 Interpretation

```text
crossing without sink
    = observation

crossing + CostBand
    = action / executable work
```

There is no separate boolean action mechanism:

```text
boolean = CostBand depth 1
```

### 7.2 Distinction from unresolved claim

Keep two quantities separate:

```text
U = constrained demand requested but not granted
R = residual granted/available value below the next CostBand quantum
```

`U != R`.

This distinction is load-bearing in contention and temporal persistence.

### 7.3 CostBand is not channel capacity

In capacity-bearing transport:

```text
Gu-Yang / RF grant → physically available amount
CostBand            → quantized sink/work
```

CostBand never invents or replaces lane throughput.

---

## 8. ActionBand: intrinsic target lifecycle

ActionBand is an intrinsic, normally inert target-discrepancy facility.

Conceptually:

\[
d_t = g_t - x_t
\]

with authored target forms and optional admitted prior-generation state for derivative/velocity information.

### 8.1 What ActionBand owns

```text
target geometry / target condition
current discrepancy
progress state
subordinate ActionBand activation
bounded dependencies
crossing-driven lifecycle
EML desired consequence
```

### 8.2 What ActionBand does not own

```text
route search             → PALMA
physical channel flux    → Gu-Yang / RF
sink affordability       → CostBand
contention clearing      → generic RF clearing
actuation lifetime       → OverlayThing
structural mutation      → BoundaryRequest boundary
```

### 8.3 Current/Next discipline

ActionBand established the resident-state rule now reused by OverlayThing:

```text
StateCurrent  = read-only during generation
StateNext     = write target during generation
whole-plane swap at generation boundary
```

No per-row freshness selector.

### 8.4 Native Field-Triad progress

Where progress is constrained by conserved transport, the ActionBand consumes resident native observables rather than rerunning a solver.

Closed bound-source conceptually:

```text
None
RfGrant
GuYangAvailable
GuYangRealized
```

Desired progress may be authored by EML, but executable progress remains bounded by the selected native authority.

---

## 9. OverlayThing: intrinsic actuation state

OverlayThing is the runtime actuation/lifecycle facility of the StemThing.

```text
originate
receive
project
lifecycle
collapse
```

### 9.1 State model

```text
OverlayTemplate[]
OverlayInstance[]
OverlayStateCurrent[]
OverlayStateNext[]
OverlayBinding[]
```

Templates are session-admitted; instances are sparse; numerical lifecycle authority is GPU-resident.

### 9.2 Lifecycle

Legal resident lifecycle state is generation-paced.

```text
active ↔ suspended → dissolved
```

No permanence claim exists. Open-ended authored life is still bounded by a definable horizon such as `UntilDissolved` / `AtSessionEnd`.

### 9.3 Deadline generations

Fixed-duration lifecycle uses a deadline, not a decrement loop:

\[
g_{deadline}=g_{activation}+\Delta g
\]

Expiry compares against the owning tree's generation authority.

A routed overlay carries duration/provenance rather than a foreign absolute deadline; the receiver establishes its own resident deadline on arrival.

### 9.4 Ancestor residency

Subtree-wide overlays remain at the lawful ancestor.

```text
one OwnerThing policy
    ↓ inherited projection
100,000 descendants
```

not:

```text
100,000 independent overlay copies
```

The graduated span substrate resolves homogeneous descendants through `O(distinct profiles/spans)`, not `O(descendants)` semantic instances or hot ancestor walks.

### 9.5 Composition

Different semantic algebras remain distinct:

```text
SequentialTransform
    authored ordered numerical transforms

ConjunctiveRestriction
    descendant cannot loosen ancestor constraint

DeclaredParameterCombine
    binding-specific combine law
```

The physical span substrate does not choose the semantic combine rule.

---

## 10. CrossingConsequenceBinding: the common actuation ABI

The single Phase-5 crossing surface now feeds a common consequence substrate with three distinct arms.

```text
BandCrossingDelta
       ↓
CrossingConsequenceBinding
       │
       ├─────────────────────────────────────────┐
       │                                         │
       ▼                                         ▼
ResidentNextWrite                       RoutedOverlayDelivery
       │                                         │
       ├─ facility-local only                    ├─ origin required
       ├─ logical property/role identity         ├─ routing/filtering preserved
       ├─ no durable physical row                ├─ generation provenance
       └─ ActionBand/OverlayThing Next           └─ receive-leg ingress
       │
       └────────────────────────┐
                                │
                                ▼
                      StructuralAuthorization
                                │
                                └─ sealed BoundaryRequest
```

A `BoundaryRequest` is **authorization-only**, not a resident GPU state-plane write.

The shared consequence substrate must not be mistaken for permission to write another SimThing's resident Next plane directly.

---

## 11. Derived span projection and source-blind invalidation

The generic physical substrate introduced during the OverlayThing closure is broader than overlays.

### 11.1 Effective profile projection

```text
ancestor/local semantic state
         ↓ compose under consumer law
EffectiveProfile
         ↓
subtree span → profile id
```

Local divergence splits spans.

Dense per-row materialization is a disposable derived cache used only where measured beneficial.

### 11.2 Authoritative changed locus

Invalidation begins from **what authoritative locus changed**, not from which subsystem wrote it.

```text
ChangedLocus {
    SimThing logical identity
    PropertyId
    role/subfield
    optional binding/profile identity
}
```

There is deliberately no `change_source` discriminator.

### 11.3 DerivedDependencyIndex

```text
ChangedLocus
    ↓
frozen per-session reverse dependency index
    ↓
exact affected work:
    effective spans
    STEAD registrations
    PALMA registrations
    Gu-Yang registrations
    other admitted derived work
```

This is the natural substrate for later partial reconciliation / granting work without coupling that work to OverlayThing representation.

---

## 12. Contention and generic constrained clearing

Contention is not a separate simulation engine. It is the oversubscribed observation of ordinary constrained clearing.

### 12.1 Generic shape

```text
N ordinary claims
       ↓
reduce by owner/resource/scope
       ↓
authored clearing program
       ↓
allocation
       ↓
disbursement
       ↓
U = unresolved remainder of requested claim
```

Trivial and contested clear use the same mechanism.

### 12.2 Authored resolution law

The engine does not encode domain activity such as combat/trade/diplomacy.

The authored clearing score/law determines behavior:

```text
priority ordered
price / weight clearing
cooperative / proportional equality
later admitted numerical laws
```

The landed witness demonstrated outcome inversion with **only authored EML data changed**, no code branch change.

### 12.3 Proportional-within-band meaning

Where authored scores are bit-identical, the author has expressed indifference. Proportional allocation within that equal-score band follows conservation and order independence; selecting a winner would invent an ordering the author did not provide.

### 12.4 Temporal persistence / attrition-shaped consequences

A denied claim need not be resolved synchronously inside the same clear.

```text
unresolved U at generation t
      ↓
ordinary anchored observation / EML persistence valuation
      ↓
CostBand-funded consequence
      ↓
OverlayThing / later claim at t+1 or later
```

Same-generation:

```text
clear → persist → re-clear
```

is forbidden.

The simulation advances at generation speed.

---

## 13. Structural mutation boundary

Numerical authority is GPU-resident; true topology/storage mutations remain boundary work.

Representative sealed structural verbs remain finite and authored.

```text
AddChild
Remove
Reparent
Attach/activate/suspend overlay storage where genuinely structural
... existing closed vocabulary only
```

The important distinction is:

```text
numerical lifecycle state transition
    → resident Next-state write

cross-SimThing routed actuation
    → routed receive path

tree/topology/storage change
    → structural authorization / BoundaryRequest
```

CPU boundary code may apply a GPU-authorized structural request but may not re-evaluate whether the action should occur.

---

## 14. Ownership, residency, and StemThing-A

### 14.1 Intrinsic owner dimension

Owner identity is a sparse intrinsic Property dimension.

```text
absence = inherit
explicit binding at ancestor = entire subtree resolves naturally
reserved neutral owner = `unowned`
```

Effective owner query remains pure and total for valid admitted members.

### 14.2 Stable layout identity

Persistent RF layout is keyed by logical identities rather than transient physical rows or string sort order.

Session-local owner layout ids never cross a seam; seam transport carries canonical owner identity.

### 14.3 Residency tier vocabulary

StemThing-A residency uses authored, session-frozen price vectors rather than engine branching on tier names.

Conceptually:

```text
ResidencyTier
    lane set
    residency class
    adjacency participation
    churn class
    unit cost
```

The execution-facing draw shape does not carry the authored tier name, preventing runtime `match tier` semantics.

Capacity partition:

\[
free + in\_flight + occupied = capacity
\]

with exact checked accounting.

Residency pricing already reuses CostBand semantics rather than creating a rival pricing facility.

---

## 15. StemThing-B slot — intentionally provisional in this stub

The later granting/placement half of the StemThing unification should be drawn here only from graduated fact.

At this stub stage, the important architectural constraint is already established:

> **StemThing-B must consume the graduated logical-identity, residency-price-vector, CostBand, and derived-span/invalidation substrates rather than introduce a granting arena, allocator-local semantic taxonomy, or parallel reconciliation mechanism.**

Expected structural position in the kernel:

```text
StemThing-A
    capacity / price / residency vocabulary
           │
           ▼
StemThing-B
    granting / reconciliation / placement admission
           │
           ├─ uses CostBand unit-cost arithmetic
           ├─ uses ordinary RF/clearing where scarcity contends
           ├─ uses logical identity, never physical row semantics
           └─ seeds source-blind dirty loci for reconciliation
```

**FORWARD GATE:** replace this section with landed detail only after the relevant StemThing-B rungs graduate. 11.3 explicitly depends on that implementation before the four-facility convergence witness can be considered complete.

---

## 16. Full logical loop: one StemThing acting in a world

```text
                     ┌──────────────────────────────┐
                     │          StemThing           │
                     │   logical identity + row     │
                     └──────────────┬───────────────┘
                                    │
                             current Properties
                                    │
               ┌────────────────────┼────────────────────┐
               │                    │                    │
               ▼                    ▼                    ▼
           owner/RF           OverlayThing          field inputs
          participation        projections        / EML parameters
               │                    │                    │
               └──────────────┬─────┴──────────────┬─────┘
                              ▼                    ▼
                        constrained RF        Field Triad
                           clearing         STEAD/PALMA/Gu-Yang
                              │                    │
                              └─────────┬──────────┘
                                        ▼
                                  anchored state
                                        │
                                 Phase-5 crossing
                                        │
                            ┌───────────┴────────────┐
                            ▼                        ▼
                        CostBand                ActionBand
                   executable quanta       target discrepancy
                            │                        │
                            └───────────┬────────────┘
                                        ▼
                                  authored EML
                                        │
                                        ▼
                              CrossingConsequence
                       ┌────────────────┼────────────────┐
                       ▼                ▼                ▼
                 resident Next      routed overlay    structural auth
                       │                │                │
                       ▼                ▼                ▼
                 OverlayThing /      receive         BoundaryRequest
                 ActionBand Next      path
                       │                │
                       └──────────┬─────┘
                                  ▼
                          generation boundary
                                  │
                                  └──────────────► recurse
```

---

## 17. Full fractal loop: tree-scale simulation

Each StemThing executes the same rule recursively.

```text
                         parent StemThing
                              │
                 child products reduce upward
                              │
                              ▼
                    owner/resource/scope RF
                              │
                         parent policy
                              │
                              ▼
                   clearing / field response
                              │
                  consequences disburse down
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
          child A          child B          child C
             │                │                │
             └──── same StemThing protocol ────┘
```

A subtree can therefore be independently executed because its only external requirements are the admitted seam products and ancestor standing view.

Async execution does not weaken determinism: the integration schedule and generation stamps become recorded data.

---

## 18. Performance model

The unification is valuable only if it preserves the dense-sweep economics.

### 18.1 Pay-for-play facilities

```text
cost scales with active rows / admitted lanes / program shapes
not object-count × virtual dispatch
```

### 18.2 Already-paid work piggyback

Examples now structurally encouraged by landed design:

```text
Phase-5 crossing
    reused by CostBand, ActionBand, OverlayThing lifecycle

resident Gu-Yang output
    reused directly by ActionBand progress

canonical EML program identity
    shared semantically across facilities

subtree spans
    reused for effective projection and dirty invalidation

generation stamps
    reused for replay, seam provenance and lifecycle deadlines

logical identity bindings
    reused across epoch remap without hot indirection
```

### 18.3 Explicitly rejected hot-path regressions

```text
per-leaf standing-overlay stamping
per-generation ancestor walks
runtime-mutable dependency registry
CPU numerical lifecycle evaluator
CPU planner / path search
private ActionBand throughput solver
per-timed-overlay countdown mutation
per-template sim-readable telemetry table
physical-row durable binding
legacy RF/transfer/emission bypass
```

---

## 19. Observation and telemetry

Observation is intrinsic because anchored writes already derive crossings.

CPU observation uses coherent generation snapshots and is read-only relative to simulation authority.

Telemetry remains write-only with respect to the simulation unless the designer explicitly authors a Property that represents that value.

```text
fine telemetry
    → observation/snapshot lifecycle
    → Studio / corpus / LeWM

NOT

fine telemetry
    → hidden Property
    → simulation feedback
```

OverlayThing's graduated closure demonstrated fixed aggregate transition counters independent of template/population cardinality rather than a per-template hot table.

---

## 20. Replay and history

There is one replay/history authority.

```text
generation stamps
+ crossing deltas
+ seam integration schedule
+ structural remap history
+ existing ReplayFrame state checkpoints
```

ActionBand, OverlayThing and async seams extend this history; they do not create facility-specific replay engines.

An epoch rebind changes physical placement and records the remap without changing logical history.

---

## 21. The single-ingress law — target form

**FORWARD GATE: 11.4 mechanizes this.**

The intended exclusivity chain is:

```text
AUTHORED / VENDOR INPUT
        │
        ▼
Overlay + CostBand + ActionBand + authored EML
        │
        ▼
ordinary RF + Field-Triad resolution
        │
        ▼
generic constrained clearing / contention resolution
        │
        ▼
ONE SimThing execution path
```

No producer should exist without a non-test production consumer. No second execution path should reach domain resolution.

This remains a target statement in this workshop file until `UNIFIED-INGRESS-EXCLUSIVITY-0` is green.

---

## 22. Kernel pseudostructure

This is deliberately conceptual pseudocode, not a proposed Rust API.

```text
StemThingKernel {
    // identity / topology
    logical_identity
    parent_child_topology
    epoch_row_binding

    // ordinary state
    property_registry
    anchor_table
    owner_dimension
    residency_state

    // recursive resource participation
    owner_channel_rf
    seam_holding_accounts
    constrained_clearing

    // numerical ISA
    eml_program_registry
    exact_primitive_admission
    gadget_library

    // field participation
    field_sweep_registrations
    stead_instances
    palma_instances
    gu_yang_instances
    comparative_projections

    // action facilities
    costband_bindings
    actionband_templates
    actionband_instances
    overlay_templates
    overlay_instances

    // shared execution substrate
    facility_resident_planes
    crossing_consequence_bindings
    derived_span_profiles
    dependency_index

    // boundary and history
    structural_authorizations
    generation_stamp_authority
    integration_schedule
    replay_frames
}
```

The canonical implementation should continue deleting fields from this conceptual picture wherever a facility can be derived rather than stored.

---

## 23. Domain interpretation examples

The core does not name these domains; this section merely shows how the same mechanisms can be interpreted externally.

### 23.1 Movement-like process

```text
PALMA potential
+ Gu-Yang realizable lane flux
+ CostBand fuel/work
+ ActionBand destination discrepancy
→ OverlayThing actuation
→ structural relocation when authorized
```

### 23.2 Production-like process

```text
RF input claim
→ constrained clear
→ CostBand unit-cost count N
→ authored output coefficient
→ Property / OverlayThing consequence
```

### 23.3 Attrition-like process

```text
oversubscribed claims
→ authored clearing
→ unresolved U
→ EML persistence valuation
→ CostBand-funded later-generation OverlayThing consequence
```

### 23.4 Policy-like process

```text
one ancestor OverlayThing
→ effective subtree profile
→ changes RF weights / PALMA impedance / STEAD emission / Gu-Yang parameter envelope
→ ordinary field and clearing consequences
```

No new engine is required for any example.

---

## 24. Architectural invariants for later diagrams

Any later refinement of this file should preserve these distinctions unless a newer graduated rung explicitly changes them:

1. **Logical identity != physical row.**
2. **Observation != sink.** Observation is the crossing base case; CostBand attaches consumption.
3. **Unresolved claim `U` != CostBand remainder `R`.**
4. **PALMA potential != route object.**
5. **Gu-Yang throughput != CostBand affordability.**
6. **OverlayThing actuation != structural mutation.**
7. **Resident Next write != routed foreign delivery.**
8. **Routed delivery != structural authorization.**
9. **Semantic inheritance law != physical span projection substrate.**
10. **Telemetry != simulation state unless explicitly authored.**
11. **Generation pacing != bounded feedback gain.**
12. **Async integration != nondeterminism; schedule is recorded data.**
13. **Contention != a domain engine; it is oversubscribed generic clearing.**
14. **EML gadget != opcode.** A power law composes `EXP`/`LN`; it does not mint `POW`.
15. **Vendor Door != engine ownership.** The embedder facade owns no simulation state.

---

## 25. Diagram TODOs for the next revision

This first stub deliberately leaves room for a second pass after 11.3 / 11.4 evidence is available.

Planned additions:

```text
A. Exact production OrderBand / generation schedule
B. One full annotated GPU memory layout diagram
C. StemThing-A → StemThing-B granting/reconciliation diagram from graduated implementation
D. Vendor Door → kernel ingress map with exact public type names
E. Full RF channel / seam / clearing accounting diagram
F. Full FieldSweep map/fold/post + comparative projection diagram
G. Exact ActionBand target/binding state machine
H. Exact OverlayThing template/instance/lifecycle state machine
I. CrossingConsequenceBinding wire diagram with landed type names
J. One end-to-end ClauseScript authored example lowered entirely into the unified facilities
K. 11.3 four-facility convergence witness trace
L. 11.4 exclusivity proof map showing every retired bypass
M. 12.2 canonization mapping from this workshop synthesis into final `simthing_core_design.md`
```

---

## 26. Working synthesis

The best current compact statement of the architecture is:

> **SimThing is a recursive GPU automaton whose ordinary state lives in homogeneous Property lanes; whose constrained means reduce and disburse through intrinsic RF; whose world-awareness is the STEAD/PALMA/Gu-Yang Field Triad; whose numerical laws are EML; whose sink is CostBand; whose target lifecycle is ActionBand; whose actuation state is OverlayThing; whose conflicts are generic authored constrained clearing; whose observations are the same anchored crossing surface used by actions; whose state changes are generation-paced through facility-local Current/Next planes; and whose only exceptional work at the CPU boundary is identity reattachment, genuine structural mutation, persistence, and observation — never a peer simulation authority.**

The remaining work before this can become the final constitutional architecture is not to invent another facility. It is to prove **composition**, prove **exclusivity**, and then canonize the already-existing germ without preserving transitional prose or bypasses.

# SimThing Unification Model
## Post-graduation architectural guide and engineering review of the closed StemThing kernel

> **Status: WORKSHOP / POST-GRADUATION SYNTHESIS / NON-NORMATIVE.**
>
> This document is a readable engineering model of the architecture that graduated in SimThing
> `0.0.8.7`. The normative paradigm is [`../simthing_core_design.md`](../simthing_core_design.md),
> especially §1. This workshop guide does not supersede it. It exists to draw the whole machine,
> connect the landed implementation surfaces, and record performance / parallelization opportunities
> that remain after semantic closure.
>
> **Review basis:** live master after `CORE-CANONIZATION-0` (#1868) and its final stamp (#1869),
> plus direct inspection of the canonical core design and representative implementation surfaces for
> StemThing-B flow markets / constrained clearing, derived-span invalidation, the session loop, the
> Vendor Door, OverlayThing, and EML. The findings below distinguish **closed semantic law** from
> **physical implementation posture** and from **future capability horizon**.

---

# 0. Executive model

The final SimThing is not a container that calls several engines. **The SimThing is the engine.**

> **A StemThing is a default-inert recursive simulation cell with stable logical identity, homogeneous
> numerical Property state, intrinsic RF participation, one EML expression organ, the STEAD/PALMA/
> Gu-Yang Field Triad, CostBand sink arithmetic, ActionBand target lifecycle, OverlayThing actuation,
> and StemThing-A/B residency + recursive conserved-resource market capability.**

Every specialist — SessionThing, OwnerThing, GridcellThing, population cohort, fleet-like thing,
network-management thing, compute-node thing — derives from that same germ. Specialization supplies
admitted data; it does not mint a peer manager.

```text
                         AUTHORING / EMBEDDING
             ClauseScript or another front-end / Vendor Door
                                  │
                                  ▼
                      ┌───────────────────────┐
                      │       STEMTHING       │
                      │  one recursive germ  │
                      └───────────┬───────────┘
                                  │
             ┌────────────────────┼────────────────────┐
             │                    │                    │
             ▼                    ▼                    ▼
       Properties / RF           EML             Field Triad
       owner / residency     one numerical      STEAD / PALMA /
       market state             language           Gu-Yang
             │                    │                    │
             └──────────────┬─────┴─────┬──────────────┘
                            ▼           ▼
                         CostBand    ActionBand
                            │           │
                            └─────┬─────┘
                                  ▼
                           Phase-5 crossing
                                  │
                                  ▼
                    CrossingConsequenceBinding
                    ┌─────────────┼─────────────┐
                    ▼             ▼             ▼
              ResidentNext   RoutedOverlay   Structural
                  Write        Delivery      Authorization
                    │             │             │
                    ▼             ▼             ▼
              facility-local   receive /     BoundaryRequest
               Current/Next     filter
                    └─────────────┬─────────────┘
                                  ▼
                         generation boundary
                                  │
                                  └──────────────► recurse
```

The strongest compact identity is:

```text
SimThing
 = recursive state
 + recursive conserved-resource economics
 + propagated fields
 + target-seeking action
 + intrinsic actuation
 + one generation protocol
```

---

# 1. What 0.0.8.7 actually closed

## 1.1 Root closure

The canonized root contract has four semantic legs:

| leg | intrinsic meaning |
|---|---|
| `participate` | Property/RF state, reduce-up, disburse-down, Field-Triad participation |
| `act` | ActionBand discrepancy + CostBand executable work |
| `originate` | attributable OverlayThing state and ordinary products |
| `receive` | deficit-driven, standing, predicate-broadcast, routed and seam-delivered state |

StemThing-A and StemThing-B are **lanes of this object**, not fifth/sixth legs.

## 1.2 Authority closure

The graduated exclusivity law is now explicit:

```text
OverlayThing / CostBand / ActionBand / authored EML
                    │
                    ▼
              RF + Field Triad
                    │
                    ▼
       generic contention / resolution
                    │
                    ▼
            ONE SimThing execution path
```

No second production resolution path is sanctioned. 11.4 mechanized this with a constitutional
producer→production-consumer census and second-sink falsifiers.

## 1.3 Composition closure

11.3 demonstrated a real composed causal chain rather than coexistence:

```text
residency / grant
    N+0
      ↓
grant lane publication
    N+1
      ↓
ActionBand crossing + routed consequence
    N+2
      ↓
OverlayThing attachment
    N+3
      ↓
stable terminal state
    N+4
```

Each facility was separately neutralized while the other facilities remained live; the terminal stopped
moving. That is the important proof: the whole is causally composed.

## 1.4 Portability closure

12.1 proved an unrelated network-saturation domain through the five Vendor Door verbs, real GPU Field
Triad execution, serialization, restore and replay without lower-crate/domain-engine edits.

The Vendor Door remains a leaf and owns no simulation state.

---

# 2. The canonical StemThing anatomy

Conceptual pseudostructure only — this is not a proposed Rust struct:

```text
StemThingKernel {
    // logical ontology
    SimThingId
    parent / children
    specialization profile
    epoch-local row binding

    // resident numerical state
    Property lanes
    anchor metadata
    owner dimension
    StemThing-A residency state
    StemThing-B market/grant state

    // recursive economics
    owner-channel RF
    seam holding balances
    constrained claims / clearing
    CostBand bindings

    // numerical language
    EML program identities
    exact primitive admission
    gadget library

    // fields
    FieldSweep registrations
    STEAD
    PALMA
    Gu-Yang
    comparative projections

    // action / actuation
    ActionBand templates + instances
    OverlayThing templates + instances
    facility Current / Next planes

    // shared derivation
    crossing consequence bindings
    effective span/profile projection
    source-blind dependency index

    // boundary + replay
    generation authority
    IntegrationSchedule
    structural authorizations
    canonical replay state
}
```

The implementation should keep deleting conceptual members whenever they can be derived rather than
stored.

---

# 3. One generation — semantic schedule

The exact physical dispatch schedule is implementation-specific, but the semantic dependencies are now
stable:

```text
GENERATION N

Current state
    │
    ├─ owner / ancestor effective state
    ├─ RF accumulation and claims
    ├─ OverlayThing effective projection
    └─ field inputs
    │
    ▼
recursive reduce-up
    │
    ▼
EML valuation + constrained clearing
    │
    ├─ grant / unresolved U
    └─ disbursement
    │
    ▼
Field Triad
    ├─ STEAD observation / urgency
    ├─ PALMA potential / impedance
    └─ Gu-Yang signed realizable flux / stall
    │
    ▼
canonical write door
    │
    └─ Phase-5 BandCrossingDelta
             │
             ▼
       CostBand / ActionBand
             │
             ▼
   CrossingConsequenceBinding
       /          |          \
      /           |           \
 ResidentNext   Routed      Structural
    Write       Delivery   Authorization
      │           │            │
      └──────┬────┘            │
             ▼                 ▼
         Next planes      BoundaryRequest
             │                 │
============= generation barrier =============
             │
      swap / apply / record
             │
             ▼
GENERATION N+1
```

Generation pacing is itself the recursion bound. Same-generation clear→persist→re-clear, receive→emit
convergence, lifecycle countdown loops, and consequence re-entry are not part of the model.

---

# 4. Ingress map

## 4.1 Vendor Door

Five verbs, no sixth manager verb:

```text
Derive
    specialization, authored EML, offerings, Draw templates, field definitions

Populate
    tree, Properties, owner bindings, resource/capacity budgets, structural initial state

Overlay
    policy / settings / directives / transient actuation

Bind
    threshold registrations, CostBands, ActionBands, Field-Triad bindings, read-only observations

Run
    initialize, tick paced/continuous, serialize, restore/replay
```

## 4.2 ClauseScript

ClauseScript remains a designer-facing language, not a runtime VM.

```text
ClauseScript modifiers / scopes / values / effects
                  │
                  ▼
           admission/lowering
                  │
   ┌──────────────┼───────────────┐
   ▼              ▼               ▼
Properties       EML          OverlayThing
RF claims        CostBand     ActionBand
                  │
                  ▼
          closed StemThing kernel
```

Generated names and high-level script nouns compile away.

## 4.3 Cross-tree / detached subtree ingress

A provisioned descendant may become an independently executing subtree while retaining its grant.
Detachment is **not** release.

```text
ancestor tree                       detached child tree
     │                                     │
     │ stamped upward RF product           │
     ◄─────────────────────────────────────┤
     │                                     │
     ├────────────────────────────────────►│
     │ stamped standing / grant state      │
     │                                     │
     ▼                                     ▼
 one IntegrationSchedule / replayed integration relationship
```

No global manager is required semantically; the relationship is identity- and generation-stamped.

---

# 5. EML — one numerical ISA

`TransformOp` is now itself a singular admitted EML program representation. `Set`, `Add`, and `Multiply`
are constructors for degenerate programs, not rival value forms.

The language-level arithmetic law is arm-independent:

- `ADD`, `SUB`, `MUL`, `DIV`: IEEE single-rounding, no reassociation;
- `MIN`, `MAX`, bounded clamps: exact selections;
- `EXP`, `LN`: exact admitted primitives over sealed domains;
- one unique `MUL` feeding an `ADD`/`SUB`: fused by definition;
- multiple `MUL` feeds to that consumer: unfused, with no tie-break.

Physical arms are faithful lowerings:

```text
admitted EML
    ├─ CPU reference
    ├─ interpreted WGSL
    ├─ SSA/JIT
    └─ recognized fused kernel
```

`PowerLaw` is a gadget, not an opcode:

\[
y = \exp(k\ln x)
\]

and its authored shape guards both transcendental domains.

---

# 6. RF + Field Triad

## 6.1 RF

RF is the recursive constrained/conserved economic substrate:

```text
local amounts / deficits
      ↓ reduce-up
owner × resource × scope
      ↓ clearing
allocation / unresolved U
      ↓ disburse-down
local consequences
```

Owner identity is sparse and inherited; resolved identity is not stamped onto every descendant.

## 6.2 STEAD

STEAD represents non-conserved propagated signal/pressure/urgency. Properties are anchored by default
unless authoring explicitly declares them unobserved.

Bands quantize the **reading**, not the underlying field.

## 6.3 PALMA

PALMA provides cost-to-go / impedance / opportunity potential over admitted adjacency. `D` is a field,
not a route object. There is no simulation-authoritative predecessor tree or CPU A* path service.

## 6.4 Gu-Yang

Gu-Yang provides instantaneous signed conservative throughput, saturation and stall. It answers the
question PALMA and CostBand do not:

```text
PALMA     = where movement / flow is desirable
Gu-Yang   = how much conserved flow is realizable now
CostBand  = how much sink/work one executable quantum costs
ActionBand= how that progress relates to an authored target
```

Signed direction is semantic. `abs(flux)` is not generic progress.

## 6.5 Comparative observables

Dominance, margin, border identity change, contest/stall and chokepoint are derived anchored values,
not semantic services.

---

# 7. CostBand — the singular sink

For available value `V` and unit cost `C`:

\[
N=\left\lfloor\frac{V}{C}\right\rfloor,
\qquad
R=V-NC
\]

with exact accounting:

\[
V=NC+R.
\]

Observation is the base crossing. A sink is observation with CostBand attached. Boolean execution is
the depth-1 degenerate case.

Keep distinct:

```text
U = constrained demand requested but not granted
R = value below the next CostBand quantum
```

`U != R`.

CostBand never becomes channel capacity; Gu-Yang/RF owns physically available flow.

---

# 8. ActionBand — target lifecycle, not planner

ActionBand owns an admitted target discrepancy and generation-paced lifecycle.

It owns:

```text
target form
discrepancy
progress state
subordinate activation
bounded dependency spans
EML desired consequence
```

It does not own:

```text
routing             → PALMA
throughput          → Gu-Yang / RF
sink affordability  → CostBand
claim clearing      → constrained RF clearing
actuation lifetime  → OverlayThing
structural mutation → boundary
```

ActionBand Current is read-only within a generation; ActionBand Next is the resident write target;
whole-plane swap is the generation boundary.

Conserved progress uses a single admitted native bound source rather than private throughput solving.

---

# 9. OverlayThing — intrinsic actuation

OverlayThing is the StemThing's resident actuation/lifecycle facility:

```text
originate → route / inherit → receive → project → lifecycle → collapse
```

Four authoring families remain useful semantic descriptions:

- capability bestowals;
- standing policies;
- lifecycled transients;
- operator directives.

The numerical substrate does not branch on those labels.

## 9.1 Ancestor residency

A subtree-wide policy resides at the ancestor defining its scope. Descendant copies are not semantic
ownership.

## 9.2 Lifecycle

No `Permanent` and no `Never`. Fixed-duration state uses deadline generation, not decrement-per-tick.
Routed duration is rebased in the receiver's generation authority.

## 9.3 Composition

Two distinct algebras remain distinct:

```text
value transforms        = sequential / authored order
policy restrictions     = conjunctive / descendants cannot loosen ancestors
```

The physical span substrate does not decide the semantic combine law.

---

# 10. Crossing consequence ABI

One Phase-5 crossing feeds exactly three consequence classes:

```text
BandCrossingDelta
       ↓
CrossingConsequenceBinding
       │
       ├─ ResidentNextWrite
       │     facility-local only
       │     logical property / role identity
       │     no durable physical row
       │
       ├─ RoutedOverlayDelivery
       │     origin + target
       │     route/filter/pacing preserved
       │
       └─ StructuralAuthorization
             BoundaryRequest only
             never a resident-plane write
```

This is one of the strongest closure points in the architecture: numerical state, cross-node delivery,
and structural mutation are unified at the dispatch seam without being conflated.

---

# 11. Derived spans and source-blind invalidation

The 7.8a substrate is a generic physical mechanism, not OverlayThing semantics.

```text
semantic state / already-composed descriptor
           ↓
      EffectiveProfile
           ↓
 logical subtree span → profile id
```

Dense per-row materialization is an optional cache.

A changed value is identified by authoritative locus:

```text
ChangedLocus {
    logical SimThing id
    PropertyId
    SubFieldRole
    optional binding/profile narrowing
}
```

There is deliberately no writer-subsystem field.

`DerivedDependencyIndex` maps that locus to exact dependent work — effective spans, STEAD/PALMA/
Gu-Yang registrations and other admitted derived work — and freezes at session admission.

---

# 12. StemThing-B — recursive conserved-resource markets

This section replaces the provisional placeholder from the earlier version of this guide.

## 12.1 Identity

> **StemThing-B is the recursive conserved-resource market germ; VRAM Residency is the distinct
> engine-native market that first proves it.**

The canonical grammar is:

```text
admitted resource / capacity
        ↓
sealed specialization offering + Draw envelope
        ↓
descendant runtime claim
        ↓
recursive RF reduce-up
        ↓
inherited / EML effective clearing weight
        ↓
authored constrained clearing
        ↓
CostBand quantization
        ↓
grant / flow disbursement
        ↓
Gu-Yang conserved throughput / saturation
        ↓
PALMA potential / impedance / opportunity
        ↓
STEAD observations / bands
        ↓
ActionBand / OverlayThing response
        ↓
next generation
```

The objective is broader than malloc:

```text
domain management
      → authored conserved-resource markets
      → RF + CostBand + Field-Triad dynamics
      → deterministic replayable corpus
```

## 12.2 Authoring vocabulary

Current landed generic types include conceptually:

```text
ConservedOfferingSpec
    resource key
    unit cost
    default clearing weight

DrawEnvelopeTemplateSpec
    offering refs
    lifecycle trigger refs
    min / max quantity

SpecializationFlowMarketSpec
    existing specialization profile
    offerings
    Draw envelopes
```

A Draw is authorization to submit a bounded claim. It grants nothing.

## 12.3 Clearing weight

Unit price and clearing weight are orthogonal. An author may deliberately relate them via EML, but the
engine does not silently equate cost with priority.

Clearing weight is inherited / deformable state. Equal clearing scores express authored indifference.

## 12.4 Generic constrained clearing

Oversubscription is ordinary. Claims are grouped in full owner/resource/scope identity and scored by an
admitted EML program.

Within an exact score band, proportional integer allocation is work-conserving. Fractional residuals
use largest remainder; exact fractional ties rotate deterministically using stable logical identity plus
the granter's generation authority. Thus authored indifference does not permanently privilege one ID.

## 12.5 Grant lifecycle

A cleared grant is identity-keyed and schedule-recorded.

```text
accept
renew
partial revoke / release
fission partition
fusion transfer
terminal release
```

Detachment is deliberately **not** release. A detached independently executing subtree can remain
provisioned by its ancestor across the stamped seam.

Death/dissolution release; fission/fusion conserve exact quantity through partition/transfer.

## 12.6 Grant publication

11.2f closed an important implementation gap: accepted lifecycle facts now publish into ordinary
conserved-capacity lanes on the next boundary. ActionBand can observe a real grant-caused band crossing.
Replay realizes the recorded fact; it does not re-clear.

This is what turns a CPU grant record into ordinary StemThing state.

---

# 13. VRAM Residency — distinct physical specialization

VRAM residency is not dissolved into the generic market. It remains engine-native because extents and
row placement are kernel physics.

The authority split is:

```text
WHO / WHETHER / HOW MUCH
    StemThing-B market entitlement

WHERE
    VRAM residency placement physics
```

`ResidencyExtent` is checked, half-open and level-local by granter.

Two-stage fail-closed:

```text
provisional entitlement
     ↓
placement oracle
  ├─ no legal realization
  │      → typed refusal
  │      → no commit
  │      → U survives for later generation
  │
  └─ legal
         → commit placement
```

Already-committed overlap/out-of-bounds is not scarcity; it is structural corruption and hard-faults
after recording the exact fault.

Physical free-range/index structures may survive only downstream of market-decided entitlement.
Allocator order has no policy authority.

---

# 14. Growth entitlement and structural commitment

Post-initial growth cannot attach first and ask for residency afterward.

The landed ordinary path is:

```text
batch candidate claims before attach
        ↓
StemThing-B clear
        ↓
accepted grant identities
        ↓
VRAM placement
        ↓
VerifiedGrowthResidencyCommit
        ↓
fission / AddChild structural commit
        ↓
schedule record
```

Replay realizes recorded products and never re-clears.

The narrow initial-install door is explicitly separate: initial tree installation and same-root
continuation are allowed; presenting a different attached subtree as a new root is a typed bypass refusal.

---

# 15. Contention and generic resolution

Contention is simply the oversubscribed observation of constrained clearing.

```text
claims
  ↓
owner/resource/scope buckets
  ↓
authored EML score
  ↓
score bands
  ↓
proportional / weighted / priority-style allocation
  ↓
grants + U
```

Changing the authored scoring program can invert outcomes without code changes. No domain word such as
combat, trade, or queue class is required in the executor.

Denied demand can persist temporally:

```text
U at generation N
   ↓
anchored observation / EML persistence valuation
   ↓
CostBand-funded consequence
   ↓
later-generation claim / OverlayThing state
```

No same-generation retry solver.

---

# 16. Replay, schedules and the ML-corpus consequence

There is one history authority:

```text
GenerationStamp
IntegrationSchedule
ReplayFrame / canonical deltas
structural remap records
band crossings / lifecycle facts
```

Facilities extend that history; none creates a private replay engine.

The market/Field-Triad architecture means ordinary operation naturally yields a labeled trajectory:

```text
claims
clearing scores
accepted grants
U / CostBand R
signed Gu-Yang flux
stall / saturation
PALMA potentials
STEAD bands
ActionBand crossings
OverlayThing transitions
structural consequences
```

That is the intended ML-corpus consequence. **Corpus extraction must remain a projection of existing
authoritative history/observation, never a second telemetry authority feeding the simulation.**

---

# 17. Post-graduation engineering review

This section is intentionally not constitutional law. It records what the graduated semantics look like
when inspected as implementation.

## 17.1 Overall verdict

**I found no obvious new peer simulation authority in the reviewed closed-kernel surfaces.** The core
claims that were most vulnerable to architectural drift — one crossing surface, one constrained-clear
path, one OverlayThing lifecycle authority, one integration history, market-decided residency entitlement,
no allocation-order policy, and no second domain resolver — have concrete type/gate/deletion evidence.

The main remaining issues are of three different classes:

```text
A. completeness debt
   a canonical semantic exists, but one public lifecycle / opener path is incomplete

B. physical-placement debt
   correct exclusive semantics still execute in a CPU/allocation-heavy form

C. measured performance research
   semantics are complete; better lowerings can exploit locality / parallelism
```

Do not conflate B/C with a semantic failure.

---

# 18. Review finding A1 — generic admitted-field replay opener debt

**Classification: real completeness/API debt, already acknowledged by 12.1/12.2.**

The generic `open_replay_with_spec` path does not initialize admitted-field-sweep domains. The 12.1
portability proof remains exact by composing the canonical Run reader, existing apply operations and
`ReplayDriver` after ordinary reinitialization of the same domain.

That preserves authority, but the symmetric user-level lifecycle is incomplete:

```text
write / run admitted Field-Triad domain
             ↓
serialize
             ↓
open_replay_with_spec   ← gap for this domain class
```

Recommended repair: extend the canonical opener to reconstruct/finalize admitted field-sweep state using
the same dimension-finalization/compiler-callback authority already used at session open. No second replay
format and no wrapper-specific semantic path.

Priority: **high product/completeness, low semantic risk if implemented as pure composition**.

---

# 19. Review finding B1 — StemThing-B clearing is still CPU / allocation heavy

**Classification: largest post-graduation physical-performance opportunity.**

The canonical clearing semantics are correct and exclusive, but the current generic implementation is
host-shaped:

```text
BTreeMap supplies by scope
BTreeSet duplicate checks
BTreeMap scope → Vec<ScoredClaim>
per-claim EML score on CPU
comparison sort by score + SimThingId
score-band scans
fractional remainder Vec + sort
final grant Vec + sort
```

That is excellent reference/oracle code. It is not the physical form I would choose for millions of
claims across independently executing resource markets.

The important architectural fact is that the input is already naturally structured:

```text
persistent owner/resource/scope RF layout
+ stable logical SimThing identity
+ sealed EML score program
+ known generation authority
```

A future physical lowering can stay bit-identical while moving the work to GPU or a precompiled resident
plan.

### 19.1 Candidate GPU clearing pipeline

```text
resident claims already keyed by scope
       ↓
parallel EML score
       ↓
segmented canonical score ordering
       ↓
segmented requested-total reduction
       ↓
parallel exact integer base-share calculation
       ↓
segmented fractional-remainder selection
       ↓
deterministic tie rotation from (granter, generation)
       ↓
resident grants / U
```

Potential physical tools:

- radix sort / radix partition on sortable score bits + stable logical id;
- segmented scans/reductions per owner/resource/scope and score band;
- preallocated scratch sized at admission rather than `Vec`/tree allocation per generation;
- specialized lowering for common simple clearing EML programs while retaining one EML meaning.

**Fence:** there must still be one clearing authority. A GPU implementation replaces/lowers the canonical
executor; it does not become a second “fast” clearing path beside CPU semantics.

---

# 20. Review finding B2 — clearing-weight inheritance bypasses the span/profile substrate

**Classification: missed unification / scale opportunity.**

`resolve_effective_clearing_weights` currently walks the full CPU `SimThing` tree recursively and
materializes a `BTreeMap<SimThingId,f32>` for every node. The semantic rule is good: sparse overrides,
ancestor inheritance, ordinary EML deformation.

But 7.8a already landed the generic physical answer to exactly this class of problem:

```text
ancestor/local semantic change
      ↓
effective profile identity
      ↓
logical subtree span
      ↓
source-blind precise invalidation
```

OverlayThing consumes this model; StemThing-B clearing weights should as well.

Recommended lowering:

```text
admission:
    tier / offering default + sparse override programs
         → interned effective profile descriptors
         → subtree span assignments

generation:
    claims read profile/weight O(1)

on changed ancestor/local state:
    ChangedLocus
      → DerivedDependencyIndex
      → only affected span/profile rebuilt
```

Where clearing-weight EML depends on live numerical operands, fuse evaluation with claim scoring or evaluate
only dirty profiles/spans. Do not rematerialize a million-node map merely because one ancestor changed.

Priority: **high for million-child markets**.

---

# 21. Review finding B3 — authoring strings should not survive into hot market lookup

The admitted market API correctly uses human-readable `String` identifiers for offerings, Draws and lifecycle
triggers. The current runtime helpers also use `&str`, `BTreeMap<String,...>`, `Vec<String>` scans and
`BTreeSet<String>` trigger membership.

That is harmless at admission scale but expensive if authorization occurs every generation for large demand
sets.

Recommended physical split:

```text
CPU / authoring shadow
    offering_id: String
    draw_id: String
    trigger_id: String
            │ admission intern
            ▼
resident plan
    OfferingIndex
    DrawIndex
    TriggerBit / bounded trigger index
    prevalidated offering spans
```

Strings remain presentation/persistence identity. Hot execution consumes compact immutable indices.

This is the same successful pattern already used for logical-vs-physical owner/row identity.

---

# 22. Review finding B4 — compatibility clearing entry should be treated explicitly

The spec crate still exposes a compatibility `clear_constrained_claims(...)` entry that synthesizes granter 0 /
generation 0, while StemThing-B uses `clear_constrained_claims_at_generation(...)` with real remainder
authority.

The graduated 11.4 exclusivity census prevents a second production resolution route today, so this is **not a
current bypass finding**. It is nevertheless an attractive future footgun.

Recommended disposition at the next API-cleanup boundary:

- prove it is oracle/test/compatibility-only;
- annotate or narrow its export accordingly; or
- retire it if no live compatibility caller remains.

Do not let a vendor accidentally choose the generationless entry for a live market.

---

# 23. Review finding C1 — generic FieldSweep Gu-Yang tiling remains the clearest measured GPU debt

Earlier field-sweep measurement established an important asymmetry:

- generic/JIT PALMA could beat the retained bespoke path;
- generic Gu-Yang was several times slower than its bespoke stencil oracle because generated WGSL did naive
  global per-edge gathers while the bespoke stencil reused workgroup-local neighborhood data.

The 0.0.8.7 ladder did **not** weaken that threshold; later reconciliation retained legacy stencil shaders on
live-consumer proof rather than claiming this physical debt disappeared.

The natural post-closeout optimization is therefore still:

```text
GridOffsets N4/N8/radius
       ↓
physical lowering detects static local stencil
       ↓
workgroup tile + halo load
       ↓
canonical per-node fold order unchanged
       ↓
same FieldSweepRegistration semantics
```

Important fences:

- tiling is a physical container, never a new EML opcode or field-law tag;
- exact canonical neighbor/fold order remains unchanged;
- LinkGraph stays sparse and should not inherit dense-grid caps or forced tiling;
- select lowering by adjacency/execution shape and measurement, not “PALMA vs Gu-Yang” semantic names.

This likely offers one of the largest immediate GPU speedups available without architectural change.

---

# 24. Review finding C2 — make DerivedDependencyIndex actually skip field work

7.8a proves precise source-blind invalidation metadata and span rebuilding. The next performance question is
whether the ordinary FieldSweep scheduler consistently converts that knowledge into **dispatch elision and
localized repair**.

I did not find a graduated claim that every ordinary STEAD/PALMA/Gu-Yang session now skips unchanged
registrations/regions based on `DerivedDependencyIndex`.

Candidate staircase:

```text
0. unchanged registration
     → zero dispatch

1. local pointwise/stencil dirtiness
     → exact dirty tile / span work

2. PALMA propagating potential dirtiness
     → active-region incremental repair

3. Gu-Yang conservative dirtiness
     → affected conservative theater, native propagation law preserved
```

This is where the earlier workshop `OverlayFieldDependency` idea properly generalizes: invalidation is now
source-blind and can be fed by any authoritative changed locus.

---

# 25. Review finding C3 — PALMA incremental / multiscale remains a lawful physical program

The semantic architecture is now stable enough to revisit the previously fenced solver-performance program.

For PALMA specifically:

```text
ChangedLocus
    ↓
DerivedDependencyIndex
    ↓
dirty PALMA region
    ↓
tiled FIM / active-set repair
    ↓
optional multiscale coarse-to-fine acceleration
    ↓
authoritative fine PALMA D
```

The fine field remains authority.

For Gu-Yang, do **not** transfer FIM semantics by analogy. Dirty-region scheduling is broadly safe; multilevel
acceleration is only lawful if the registration represents a within-generation fixed point/equilibrium and
converges to the same fine conservative result. If each sweep represents finite-rate physical propagation,
coarse acceleration changes physics.

---

# 26. Review finding C4 — parallelize independent subtree execution, not semantics

The architecture deliberately supports independently executing subtrees and records their seam integration
schedule. That creates a large physical parallelization opportunity:

```text
subtree A generation N+4 ─┐
subtree B generation N+2 ─┼─ stamped products → parent integration schedule
subtree C generation N+7 ─┘
```

Possible host/GPU implementation strategies:

1. batch independent subtree roots into one large dispatch with root/range descriptors;
2. issue independent command buffers / queues where the backend benefits;
3. overlap field execution of one subtree with host-side structural preparation of another;
4. integrate only at recorded boundaries, preserving exact holding-account conservation.

The scheduler is physical. It may not become a semantic multi-session manager, choose outcomes, or invent a
second generation authority.

This is particularly attractive for network-node domains, where the tree topology naturally partitions into
many locally autonomous nodes.

---

# 27. Review finding C5 — boundary structural work can use prepare-parallel / commit-ordered execution

True structural changes remain at the boundary, correctly. At high fission/fusion/reparent/grant-placement
rates that boundary can become the serial floor.

VRAM placement is already granter-indexed and level-local. That allows safe parallel preparation:

```text
parallel by independent granter:
    inspect local free ranges
    build provisional placement / remap candidate
    validate extent constraints

canonical boundary order:
    commit accepted candidates
    record one IntegrationSchedule
    publish sparse remaps/deltas
```

The commit remains deterministic and single-authority; only independent preparation is parallel.

This is likely more valuable than attempting to move structural mutation itself into the GPU.

---

# 28. Review finding C6 — cross-facility fusion opportunities

The closed semantics permit physical fusion where no authoritative intermediate is being erased.

Promising cases:

### 28.1 RF claim scoring + clearing

Claims are already resident RF products. EML scoring can be evaluated in the same resident clearing pipeline
rather than copying claims to a separate host table.

### 28.2 Overlay ingress + FieldSweep map

If an overlay-deformed intermediate is not an independently anchored observable, fuse its EML deformation
into the field map lowering. If it **is** an anchored/crossed/corpus-visible value, materialize it normally.

### 28.3 Grant-lane publication + write-door crossing derivation

Grant facts already publish through ordinary state. Preserve the one generation of semantic latency, but
ensure the N+1 lane write and its Phase-5 crossing detection share the normal fused write door rather than an
extra scan.

### 28.4 EML program/profile interning

Same semantic program identity can share compiled artifacts under compatible execution shape / binding ABI.
Do not infer cross-instance common subexpressions merely from equal authored shape.

---

# 29. Review finding C7 — remeasure full-plane Current→Next carry at target scale

ActionBand measured whole-plane Current→Next carry through 4096 rows and correctly rejected speculative
compact active lists at that scale.

That evidence should not be silently extrapolated to tens of millions of resident facility rows.

At target scale, remeasure:

```text
rows
bytes copied
copy bandwidth
percentage of generation time
active fraction
```

Do **not** pre-author a sparse freshness-bit system; that was already rejected semantically. If full-buffer
copy becomes meaningful, look first at backend buffer-copy performance, layout coalescing, facility grouping,
or equivalent whole-plane-preserving physical techniques.

---

# 30. Review finding C8 — corpus extraction should be columnar/offline, never causal telemetry

The unified market + Triad architecture is now a particularly strong corpus generator. The next performance
step should be an **offline/read-only projection** of existing history rather than additional simulation state.

Potential output shape:

```text
[tree_id, generation, logical_id,
 owner/resource/scope,
 claim, clearing_score, grant, U, CostBand_R,
 STEAD bands,
 PALMA D,
 Gu-Yang net/gross/stall,
 ActionBand crossing,
 OverlayThing transition,
 structural consequence]
```

For ML scale, make this columnar/batched and optionally GPU-staged. But the dataset writer must remain a
consumer of authoritative replay/anchor state. It must never become another observer whose availability or
backpressure changes the simulation.

---

# 31. Explicit capability horizon — Vector CostBand

Scalar CostBand is closed. A separate known capability remains fenced:

> coordinated atomic common-depth commitment / persistent provisional holding across independently
> contested scarce lanes.

That is the Vector CostBand question.

It is **not** a defect in scalar CostBand, ActionBand or StemThing-B. Do not retrofit ad hoc transactions into
the market germ to cover it. If a real consumer requires all-or-nothing multi-resource commitment, reopen the
separate probe against the now-graduated 7.3 + 8.1 + 8.2 + StemThing-B semantics.

---

# 32. Architectural diagrams — final reference set

## 32.1 Full kernel

```text
                                    STEMTHING
                                       │
          ┌────────────────────────────┼────────────────────────────┐
          │                            │                            │
          ▼                            ▼                            ▼
   StemThing-A                    Properties                    StemThing-B
 logical residency               owner / state               flow-market germ
 placement identity                   │                            │
          │                            │                     claims / grants
          └──────────────┬─────────────┴─────────────┬──────────────┘
                         ▼                           ▼
                        RF                          EML
                  reduce / clear /                valuation
                    disburse                       laws
                         │                           │
                         └──────────────┬────────────┘
                                        ▼
                                   Field Triad
                           STEAD / PALMA / Gu-Yang
                                        │
                                   anchored state
                                        │
                                  Phase-5 crossing
                                        │
                           ┌────────────┴────────────┐
                           ▼                         ▼
                       CostBand                  ActionBand
                           │                         │
                           └────────────┬────────────┘
                                        ▼
                              CrossingConsequence
                       ┌────────────────┼────────────────┐
                       ▼                ▼                ▼
                 ResidentNext       Routed          Structural
                    Write           Delivery       Authorization
                       │                │                │
                       └───────┬────────┘                ▼
                               ▼                  BoundaryProtocol
                         OverlayThing /                 │
                         ActionBand Next                │
                               └──────────────┬──────────┘
                                              ▼
                                      generation barrier
                                              │
                                              └────► recurse
```

## 32.2 Resource-market loop

```text
ancestor / granter
       │ available conserved capacity
       ▼
ConservedOffering + Draw envelope
       │
       ▼
descendant claims
       │ reduce-up
       ▼
owner/resource/scope RF bucket
       │
       ├─ inherited EML clearing weight
       └─ authored clearing program
       │
       ▼
constrained clear
       │
       ├─ grant
       └─ U
       │
       ▼
CostBand / grant lifecycle
       │
       ▼
ordinary conserved lane publication
       │
       ▼
STEAD / PALMA / Gu-Yang observations
       │
       ▼
ActionBand / OverlayThing response
```

## 32.3 Residency specialization

```text
MarketGrantRecord
      │ provenance-verified entitlement
      ▼
ResidencyExtent proposal
      │
      ▼
level-local placement oracle
   ┌──┴────────────────────────────┐
   │                               │
 infeasible                      legal
   │                               │
 typed refusal / U             commit extent
   │                               │
 next generation              row / remap physics

committed overlap / out-of-bounds
             ↓
record exact corruption
             ↓
hard session fault
```

## 32.4 Detached subtree

```text
        Parent / manager StemThing
                  │
             cleared grant
                  │
                  ▼
            Child StemThing
                  │ detach
                  ▼
        self-executing subtree

Parent and child continue via:
    stamped RF products
    stamped standing/grant state
    holding-account conservation
    one IntegrationSchedule

Detachment != release.
```

## 32.5 Field/action loop

```text
Property / RF / Overlay state
           │
           ▼
     FieldSweep IR
   /       |       \
STEAD    PALMA    Gu-Yang
   \       |       /
    \      |      /
     anchored observables
             │
             ▼
      BandCrossingDelta
             │
      ┌──────┴───────┐
      ▼              ▼
   CostBand       ActionBand
      │              │
      └──────┬───────┘
             ▼
       OverlayThing Next
             │
             └────► next generation fields
```

---

# 33. Final distinction table

| do not conflate | distinction |
|---|---|
| logical identity / physical row | rows rebind; identity does not |
| observation / sink | crossing is observation; CostBand adds consumption |
| `U` / CostBand `R` | ungranted demand vs below-quantum value |
| PALMA / route object | potential field vs planner artifact |
| Gu-Yang / CostBand | realizable throughput vs sink price |
| grant entitlement / VRAM placement | WHO/HOW-MUCH vs WHERE |
| detachment / release | topology change does not terminate grant |
| OverlayThing state / structural mutation | resident numerical lifecycle vs boundary topology |
| ResidentNextWrite / RoutedDelivery | local facility state vs foreign receive path |
| semantic inheritance / span substrate | combine law vs physical projection |
| field invalidation / writer subsystem | changed locus determines dirtiness, not source label |
| telemetry / sim state | observation cannot become causal silently |
| EML gadget / opcode | PowerLaw composes EXP/LN; no POW opcode |
| contention / domain engine | oversubscription of generic clearing |
| CPU reference / second authority | oracle may remain; production resolution stays singular |
| physical scheduler / semantic manager | parallel execution cannot decide outcomes |

---

# 34. Recommended post-0.0.8.7 engineering sequence

This is a performance/completeness recommendation, not a new workplan.

```text
P0  close the admitted-field-sweep generic replay opener debt

P1  market hot-path census
    measure claim cardinalities, scope sizes, score-band sizes,
    CPU allocation/sort time, grant-lifecycle volume

P2  consume 7.8a spans for StemThing-B clearing weights
    eliminate whole-tree effective-weight materialization

P3  precompile / intern market vocabulary
    compact Offering/Draw/Trigger ids, preallocate scratch

P4  GPU / resident constrained-clearing lowering
    exact parity with canonical CPU oracle

P5  tiled static-grid FieldSweep gather
    first target: Gu-Yang measured global-gather debt

P6  wire source-blind dirtiness into dispatch elision / incremental fields

P7  parallel subtree execution + prepare-parallel structural boundary

P8  measured PALMA FIM / multiscale research
    only after dirty-dispatch measurements

P9  corpus projection / columnar export
    read-only over authoritative replay + anchors

P10 Vector CostBand only when a real atomic multi-resource consumer exists
```

The ordering matters: eliminate obvious O(N)/allocation hot paths and measured gather waste before adding
more sophisticated solvers.

---

# 35. Final synthesis

The graduated architecture can now be stated without future tense:

> **SimThing is a closed recursive simulation kernel. Each StemThing carries homogeneous Property state,
> intrinsic owner-aware RF participation, the EML numerical ISA, STEAD/PALMA/Gu-Yang fields, CostBand as
> the sole sink law, ActionBand as target lifecycle, OverlayThing as intrinsic actuation, StemThing-A as
> logical/physical residency discipline, and StemThing-B as the recursive conserved-resource market germ.
> All domain contention resolves through authored generic clearing, all ordinary action returns through the
> same crossing/consequence surfaces, and all asynchronous recursion is generation-stamped into one replay
> schedule. Specialized domains add authored data, not managers or engines.**

The post-graduation engineering task is therefore **not another unification refactor**. It is to make the
already-unified semantics execute closer to their ideal physical form:

```text
fewer CPU maps/sorts
fewer full-tree projections
fewer redundant field dispatches
greater GPU locality
greater independent-subtree parallelism
same bits
same authority
same generation law
```

That is the strongest sign that 0.0.8.7 actually closed the architecture: the remaining high-value work is
mostly optimization of one model rather than reconciliation of competing models.

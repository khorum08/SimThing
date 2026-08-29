# SimThing Unification Model
## Post-graduation architectural guide, Phase-13 remediation map, and engineering review

> **Status: WORKSHOP / NON-NORMATIVE / LIVE SYNTHESIS.**
>
> This document is a readable engineering model of the architecture that graduated through
> `CORE-CANONIZATION-0` and of the **Phase 13 legacy-convergence addenda now in flight**. The
> normative paradigm remains [`../simthing_core_design.md`](../simthing_core_design.md), especially
> §1. This guide does not supersede the ladder, the core design, or DA rulings.
>
> **State discipline used here:**
>
> - **GRADUATED** means landed and DA-graduated before Phase 13.
> - **PROBATION / IN FLIGHT** means implementation exists on an open review branch but is not yet
>   constitutional fact.
> - **MINTED / PLANNED** means the rung and exit law exist, but implementation is still future.
>
> The important correction from the first post-12.2 review is that several findings which looked like
> post-closeout optimization work have now been promoted into **pre-closeout convergence debts**. The
> guide therefore separates (a) the closed StemThing semantic kernel, (b) application/legacy surfaces
> that still need to converge onto it, and (c) physical performance work that remains a later track.

---

# 0. Executive model

The central architecture remains unchanged:

> **A StemThing is a default-inert recursive simulation cell with stable logical identity,
> homogeneous numerical Property state, intrinsic owner-aware RF participation, one EML expression
> organ, the STEAD/PALMA/Gu-Yang Field Triad, CostBand sink arithmetic, ActionBand target lifecycle,
> OverlayThing actuation, StemThing-A residency, and StemThing-B recursive conserved-resource market
> capability.**

Specialists such as SessionThing, OwnerThing, GridcellThing, population cohorts, fleet-like things,
network-management things, and compute-node things derive from the same germ. Specialization adds
admitted data; it does not create a peer runtime engine.

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

Compact identity:

```text
SimThing
 = recursive state
 + recursive conserved-resource economics
 + propagated fields
 + target-seeking action
 + intrinsic actuation
 + one generation protocol
```

Phase 13 does **not** replace this model. Its purpose is to make legacy/application surfaces and one
newly-landed StemThing-B implementation path conform more completely to it before closeout.

---

# 1. What 0.0.8.7 actually closed

## 1.1 Root and authority closure — GRADUATED

The canonized root contract has four semantic legs:

| leg | intrinsic meaning |
|---|---|
| `participate` | Property/RF state, reduce-up, disburse-down, Field-Triad participation |
| `act` | ActionBand discrepancy + CostBand executable work |
| `originate` | attributable OverlayThing state and ordinary products |
| `receive` | deficit-driven, standing, predicate-broadcast, routed and seam-delivered state |

StemThing-A and StemThing-B are lanes of this object, not additional legs.

The exclusivity chain is:

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

11.4 mechanized this with a constitutional producer→production-consumer census and second-sink
falsifiers. Phase 13.5 extends that **same census discipline** to the legacy estate; it does not mint a
second exclusivity checker.

## 1.2 Composition closure — GRADUATED

11.3 demonstrated a real causal composition:

```text
residency / grant      N+0
      ↓
grant lane publication N+1
      ↓
ActionBand crossing + routed consequence N+2
      ↓
OverlayThing attachment N+3
      ↓
stable terminal state N+4
```

Each facility could be neutralized independently while the others remained live, and the terminal
stopped moving. That is the load-bearing proof of composition.

## 1.3 Portability closure — GRADUATED

12.1 proved an unrelated network-saturation domain through the five Vendor Door verbs, real GPU
Field-Triad execution, serialization, restore and replay without a domain engine.

One read-side ergonomics/completeness debt remains: `open_replay_with_spec` does not directly open the
special admitted-field-sweep form. Phase 13.2 may fold that repair **only if the ClauseThing convergence
naturally passes through the same seam**. It is not permission for a second replay authority.

---

# 2. Canonical StemThing anatomy

Conceptual pseudostructure only:

```text
StemThingKernel {
    // logical ontology
    SimThingId
    parent / children
    specialization profile
    epoch-local row binding

    // resident numerical state
    Property lanes
    anchors
    owner dimension
    StemThing-A residency
    StemThing-B market / grant state

    // recursive economics
    owner-channel RF
    seam holding balances
    constrained claims / clearing
    CostBand bindings

    // numerical language
    EML programs
    exact primitive admission
    gadget library

    // field organ
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

The implementation should delete conceptual members whenever they can be derived rather than stored.
Phase 13.6 applies that rule directly to clearing-weight inheritance.

---

# 3. One generation — semantic schedule

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

Generation pacing is the recursion bound. There is no same-generation clear→persist→re-clear,
receive→emit convergence, or consequence re-entry.

Phase 13.6 strengthens this by deleting the compatibility clearing form that silently supplied a
synthetic generation to remainder tie rotation. Once that rung lands, the only ordinary clearing form
will require real generation authority explicitly.

---

# 4. Authoring and ingress boundaries

## 4.1 Vendor Door — GRADUATED

Five verbs, no sixth manager verb:

```text
Derive    specialization, EML, offerings, Draws, field definitions
Populate  tree, Properties, owner bindings, resource/capacity budgets
Overlay   policy / directives / transient actuation
Bind      CostBands, ActionBands, fields, observations
Run       initialize, tick, serialize, restore / replay
```

## 4.2 ClauseScript / ClauseThing boundary — PHASE 13 TARGET

ClauseThing is an **application-layer authoring system**. It is not part of the recursive kernel.

The intended one-way relationship is:

```text
ClauseScript / ClauseThing vocabulary
              │
              ▼
        hydrate / LOWER
              │
              ▼
 modern generic SimThing spec / admitted data
              │
              ▼
        closed StemThing kernel
```

Phase 13.2 makes this fence load-bearing:

```text
application converges onto kernel

NOT

kernel grows ClauseThing accommodations
```

The rung explicitly forbids edits to engine/spec/kernel/GPU/sim/driver/embedder semantics for the
ClauseThing convergence repair. If the application appears to need a kernel accommodation, the coding
lane must STOP and name the seam; the starting presumption is that the lowerer is stale.

This is an important interpretation of the three inherited red tests: they are **legacy lowering /
application-convergence evidence**, not proof that the StemThing germ requires game-specific semantics.

## 4.3 Authoring-ingress set — PHASE 13.5 TARGET

Phase 13.5 will census every way authored content becomes an admitted session, including at least:

```text
ClauseThing hydration
canonical JSON/interchange load
literal install
programmatic spec construction
```

Each path must be classified as canonical, interchange-with-stated-contract, or dated-deferred. The
purpose is to make statements such as “`.clause` reaches admission only through the ClauseThing lowerer”
mechanically checkable rather than tribal knowledge.

## 4.4 Cross-tree / detached subtree ingress — GRADUATED

A provisioned descendant may execute independently while retaining its grant. Detachment is not
release.

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

---

# 5. EML — one numerical ISA

`TransformOp` is a singular admitted EML program representation. `Set`, `Add`, and `Multiply` are
degenerate program constructors, not rival value forms.

Arithmetic meaning is arm-independent:

- `ADD`, `SUB`, `MUL`, `DIV`: IEEE single-rounding, no reassociation;
- `MIN`, `MAX`, bounded clamps: exact selections;
- `EXP`, `LN`: exact admitted primitives over sealed domains;
- one unique `MUL` feeding `ADD`/`SUB`: fused by definition;
- multiple `MUL` feeds: unfused.

Physical arms are faithful lowerings:

```text
admitted EML
    ├─ CPU reference
    ├─ interpreted WGSL
    ├─ SSA/JIT
    └─ recognized fused kernel
```

`PowerLaw` remains a gadget built from `EXP(k * LN(x))`, not a `POW` opcode.

---

# 6. RF + Field Triad

## 6.1 RF

RF is recursive constrained/conserved economics:

```text
local amounts / deficits
      ↓ reduce-up
owner × resource × scope
      ↓ clearing
allocation / unresolved U
      ↓ disburse-down
local consequences
```

## 6.2 STEAD

STEAD is non-conserved propagated signal/pressure/urgency. Bands quantize observations, not field
propagation.

## 6.3 PALMA

PALMA is min-plus potential/impedance/opportunity over admitted topology. `D` is a field, not a route
object.

## 6.4 Gu-Yang

Gu-Yang is signed conservative throughput, saturation and stall:

```text
PALMA      → where flow/progress is desirable
Gu-Yang    → how much conserved flow is realizable
CostBand   → how much a sink/work quantum costs
ActionBand → how progress relates to a target lifecycle
```

`abs(flux)` is not generic progress.

## 6.5 Gu-Yang performance debt — CORRECTED POSTURE

The prior edition of this guide described the historical gather-vs-tiled-stencil slowdown as if its
number were still current. Phase 13.4 correctly rejects that evidentiary posture.

The old measurement predates substantial kernel movement. Its instrument survives only in archived
workshop preserves. Therefore the live statement is now:

> **There is a plausible Gu-Yang global-gather locality debt, but its historical ratio is stale until
> current-head remeasurement succeeds or the archived instrument is formally recorded as stale.**

Phase 13.4 requires one of two ledgered outcomes:

```text
measured:<date>@<commit> + fresh number

OR

instrument-stale:<date> + remeasurement owed as performance-track first-rung precondition
```

This becomes a general performance-governance law: an inherited performance debt cannot charter a
future track without dated current-head measurement or a dated owed-measurement marker.

---

# 7. CostBand — singular sink

For available value `V` and unit cost `C`:

\[
N=\left\lfloor\frac{V}{C}\right\rfloor,\qquad R=V-NC
\]

with exact accounting `V = NC + R`.

Keep distinct:

```text
U = requested but ungranted constrained demand
R = value below the next CostBand quantum
```

`U != R`.

---

# 8. ActionBand — target lifecycle, not planner

ActionBand owns target form, discrepancy, progress state, bounded dependencies, subordinate activation,
and EML desired consequences.

It does not own routing, physical throughput, sink affordability, claim clearing, overlay lifetime, or
structural mutation.

```text
routing             → PALMA
throughput          → Gu-Yang / RF
sink affordability  → CostBand
claim clearing      → constrained clearing
actuation lifetime  → OverlayThing
structural mutation → boundary
```

Current is read-only during the generation; Next is the resident write target.

---

# 9. OverlayThing — intrinsic actuation

OverlayThing is the resident actuation/lifecycle facility:

```text
originate → route / inherit → receive → project → lifecycle → collapse
```

Subtree-wide policy resides at the lawful ancestor rather than being stamped onto every descendant.
Fixed-duration state uses deadline generation. There is no `Permanent` or `Never` semantic escape.

Two composition algebras remain distinct:

```text
value transforms    = sequential authored order
policy restrictions = conjunctive monotone restriction
```

---

# 10. Crossing consequence ABI

One Phase-5 crossing feeds three consequence classes:

```text
BandCrossingDelta
       ↓
CrossingConsequenceBinding
       ├─ ResidentNextWrite
       ├─ RoutedOverlayDelivery
       └─ StructuralAuthorization → BoundaryRequest
```

This keeps resident numerical state, cross-node delivery, and topology mutation unified at one seam
without conflating their authorities.

---

# 11. Derived spans and source-blind invalidation

The 7.8a substrate is generic physical machinery:

```text
semantic state / composed descriptor
           ↓
      EffectiveProfile
           ↓
 logical subtree span → profile id
```

A changed value is identified by authoritative locus rather than writer identity:

```text
ChangedLocus {
    logical SimThing id
    PropertyId
    SubFieldRole
    optional binding/profile narrowing
}
```

`DerivedDependencyIndex` maps loci to affected spans and derived work. Dense per-row materialization is
an optional cache.

This matters because **Phase 13.6 now promotes clearing-weight span consumption from recommendation to
workplan obligation**.

---

# 12. StemThing-B — recursive conserved-resource markets

## 12.1 Identity — GRADUATED

> **StemThing-B is the recursive conserved-resource market germ; VRAM Residency is the distinct
> engine-native market that first proves it.**

Canonical grammar:

```text
admitted resource / capacity
        ↓
sealed offering + Draw envelope
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
Gu-Yang throughput / saturation
        ↓
PALMA potential / opportunity
        ↓
STEAD observations / bands
        ↓
ActionBand / OverlayThing response
        ↓
next generation
```

Objective:

```text
domain management
      → authored conserved-resource markets
      → RF + CostBand + Field-Triad dynamics
      → deterministic replayable corpus
```

## 12.2 Draws and offerings — GRADUATED

A Draw is an admission-sealed claim envelope. It grants nothing. Unit price and clearing weight are
orthogonal.

## 12.3 Clearing — GRADUATED SEMANTICS, PHYSICAL REMEDIATION PENDING

Oversubscription is ordinary. Exact equal-score bands clear proportionally; fractional residuals use
largest remainder; exact remainder ties rotate under the granter's generation authority.

The semantic law is closed. Two implementation details are being remediated by 13.6:

1. inherited effective clearing weights currently use a recursive CPU tree walk + per-node `BTreeMap`;
2. a compatibility `clear_constrained_claims` form can synthesize granter/generation zero.

Neither represents a second **production** authority today, but both are poor final forms.

## 12.4 Grant lifecycle — GRADUATED

```text
accept
renew
partial revoke / release
fission partition
fusion transfer
terminal release
```

Detachment is not release. Death/dissolution release; fission/fusion conserve quantity exactly.

## 12.5 Grant publication — GRADUATED

Accepted lifecycle facts publish into ordinary conserved-capacity lanes on the next boundary. Replay
realizes the recorded fact and never re-clears.

---

# 13. VRAM Residency — distinct physical specialization

VRAM residency remains engine-native because extents and row placement are kernel physics.

```text
WHO / WHETHER / HOW MUCH → StemThing-B market entitlement
WHERE                    → residency placement physics
```

Ordinary placement infeasibility returns typed refusal and preserves `U`; committed overlap/out-of-bounds
is structural corruption and hard-faults after recording.

Free-range/index structures may survive downstream of entitlement but carry no grant policy.

---

# 14. Growth entitlement and structural commitment

Post-initial growth follows:

```text
candidate claims before attach
        ↓
StemThing-B clear
        ↓
accepted grant identities
        ↓
VRAM placement
        ↓
VerifiedGrowthResidencyCommit
        ↓
fission / AddChild commit
        ↓
schedule record
```

Replay never re-clears.

---

# 15. Replay, schedules, and ML-corpus consequence

There is one history authority:

```text
GenerationStamp
IntegrationSchedule
ReplayFrame / canonical deltas
structural remap records
band crossings / lifecycle facts
```

The market + Field-Triad architecture naturally yields labeled trajectories containing claims, scores,
grants, `U`, CostBand `R`, flux, stall, potentials, bands, crossings, overlay transitions and structural
consequences.

Corpus extraction must remain a read-only projection of authoritative history/observation, never a
second causal telemetry authority.

---

# 16. Phase 13 remediation map

Phase 13 was minted after post-canonization reviews found that the semantic kernel was closed but several
legacy/convergence surfaces were still not in the clean final form expected at track closeout.

## 16.1 13.1 `ADMISSION-PROVENANCE-TYPED-0` — PROBATION / IN FLIGHT

**Problem being fixed:** root admission refusals could collapse to `SpecError::ValidationFailedAt { site }`,
identifying where rejection happened without telling callers **which law** rejected **which element**.
This made the inherited ClauseThing reds diagnostically opaque and matched the standing
`ROOT-CONTRACT-ADMISSION-ERROR` promotion debt.

**Intended repair:** one domain-neutral existing-`SpecError` shape:

```text
AdmissionRefused {
    law_id: &'static str,
    element_path: String,
}
```

The current open implementation PR #1875 promotes exactly five root-install rejection sites while
preserving the existing wrapper chain:

```text
SpecError
  → InstallError::Spec
  → SessionError::Install
  → SimSession::open_from_spec caller
```

The proposed laws include registered resource-economy property, live host, live base-obligation
participant property, and standalone-overlay compile diagnostics. A planted wrong-law-id mutant makes
the public open-from-spec witness RED.

**Kernel-containment fence:** `law_id + element_path` is domain-neutral. No ClauseThing/Stellaris-specific
variant is allowed below the application crate.

**Guide posture:** treat this as imminent but not graduated until DA review/merge/stamp.

## 16.2 13.2 `CLAUSETHING-ADMISSION-CONVERGENCE-0` — MINTED / PLANNED

**Problem being fixed:** the two inherited ClauseThing GPU/oracle reds are now understood as lowerer-target
version skew: ClauseThing hydration still targets retired/changed engine shapes and test-side adaptation glue
has accumulated between hydration and admission.

**Repair law:** only the ClauseThing `hydrate_*` lowering layer may change. Existing assertion bodies remain
unmodified. The canonical ClauseScript corpus must parse, hydrate and admit on the modern substrate, while
negative fixtures continue refusing for the same named reasons.

Standing corpus witness target:

```text
all .clause files
    parse
      ↓
    hydrate
      ↓
    admit modern generic SimThing spec
```

**Hard fence:** zero kernel/spec/driver/GPU/sim/embedder accommodation. This is one-way application convergence.

**Optional fold:** the admitted-field-sweep generic opener debt may be repaired here only if the same seam is
naturally involved; otherwise it remains separate debt.

## 16.3 13.3 `STAR-NAMING-GOLDEN-TRUTH-0` — MINTED / PLANNED

**Problem being fixed:** the third inherited red is a stale canonical star-name golden whose truth status is
unknown.

Two lawful outcomes:

```text
upstream derivation drift was lawful
    → re-bless golden with provenance

OR

determinism/source regression
    → fix implementation
```

The rung must document the canonical entity-name derivation and future blessing procedure. It may not weaken
the oracle or invent a new name-source vocabulary.

This is a truth/provenance remediation, not a StemThing semantic facility.

## 16.4 13.4 `WORKSHOP-CORPUS-TRIAGE-0` — MINTED / PLANNED

**Problem being fixed:** closeout would otherwise default-delete workshop/performance artifacts that are the
only instruments for known future work, while some old benchmark numbers are too stale to justify new tracks.

Every reap-slated asset gets an explicit keep/delete/elevate/lease disposition with named consumer and date.
The future performance track is a named lease consumer for selected benchmark/replay/contention assets.

The Gu-Yang gather-vs-tiled-stencil debt receives special treatment:

```text
archived instrument works at current head
    → remeasure, record number + date + commit

instrument no longer works
    → record instrument-stale + date
    → make remeasurement first-rung precondition of performance track
```

**New governance law:** stale, unmeasured performance folklore cannot charter a track.

## 16.5 13.5 `LEGACY-SURFACE-CONVERGENCE-CENSUS-0` — MINTED / PLANNED

**Problem being fixed:** 11.4 proved exclusivity for the graduated core, but the wider legacy estate still lacks
one machine-readable map of producers and authoring ingress.

13.5 extends the existing constitutional census to:

```text
ClauseThing lowerers
MapGen generators
Studio scenario surfaces
all authoring-ingress paths
application-named vocabulary found in kernel crates
engine → ClauseThing dependency arrow
```

Every producer must map to unified ingress or a dated deferral. Every application-vocabulary-in-kernel row is
queued for later generalize/relocate work rather than normalized into kernel law.

**One gate, not two:** extending `constitutional_surface_check.sh` / `constitutional_surfaces.tsv` is the only
lawful checker shape.

**Important containment observation:** engine crates currently carry zero ClauseThing dependency; that
one-way dependency must remain true.

## 16.6 13.6 `CLEARING-WEIGHT-SPAN-UNIFICATION-0` — MINTED / PLANNED

This rung directly promotes two findings from the first engineering review.

### A. Clearing-weight span unification

Current transitional physical shape:

```text
resolve_effective_clearing_weights
    recursive CPU SimThing walk
    + BTreeMap<SimThingId, f32> materialization
```

Target shape:

```text
sparse authored default / overrides / EML deformation
        ↓
interned effective profile
        ↓
logical subtree spans (7.8a)
        ↓
source-blind invalidation
        ↓
claims consume effective weight without whole-tree rematerialization
```

Required proof is semantic neutrality: bit-identical weights across default, override, inheritance, and
EML-deformed cases; predecessor germ/11.2f/11.3 witnesses remain unchanged. The recursive implementation is
deleted rather than retained as a second path.

### B. Synthetic-generation clearing door deletion

Current compatibility form:

```text
clear_constrained_claims(...)
    → clear_constrained_claims_at_generation(...,
         granter = 0,
         generation = 0)
```

DA census found zero production consumers and only four test call sites in one test consumer. Phase 13.6 will
delete this form and migrate those calls to explicit `clear_constrained_claims_at_generation`.

This is stronger than “document compatibility-only”: the footgun becomes unrepresentable.

---

# 17. Revised engineering findings after Phase 13 mint

The earlier review should now be read with these dispositions.

| earlier finding | Phase-13 disposition |
|---|---|
| admitted-field-sweep generic opener gap | still real; optional 13.2 fold if same seam, otherwise remains debt |
| CPU allocation-heavy full constrained clearing | **not** a Phase-13 target; remains future performance track |
| clearing-weight full-tree materialization | **promoted to 13.6 convergence debt** |
| authored market strings in hot lookup | remains future performance work |
| synthetic-generation compatibility clearing entry | **promoted to 13.6 deletion** |
| Gu-Yang gather/tile old slowdown number | **cannot be cited as current; 13.4 remeasure or mark instrument stale** |
| source-blind dirtiness not yet guaranteed to skip all field work | future performance track |
| parallel detached-subtree scheduling | future performance track |
| prepare-parallel boundary work | future performance track |
| corpus columnar extraction | future performance track |
| Vector CostBand | separate capability horizon, unchanged |

This table is important: Phase 13 is a **convergence/closure addendum**, not a general performance campaign.

---

# 18. Remaining physical-performance opportunities after Phase 13

## 18.1 GPU/resident constrained clearing

The canonical semantics are correct but the generic executor is host-shaped: maps/sets, per-claim CPU EML
score evaluation, comparison sorts, score-band scans, fractional remainder arrays and result vectors.

A later faithful lowering can use:

```text
resident claims keyed by scope
       ↓
parallel EML scoring
       ↓
segmented deterministic ordering
       ↓
segmented requested-total reduction
       ↓
exact integer base shares
       ↓
segmented remainder selection
       ↓
tie rotation from real (granter, generation)
       ↓
resident grants / U
```

Phase 13.6 improves this future path by eliminating the synthetic-generation API and by putting effective
weights on the span substrate first.

## 18.2 Compact admitted market vocabulary

Human-readable offering/Draw/trigger strings should remain persistence/authoring identity but can intern to
compact immutable indices for hot execution.

## 18.3 FieldSweep locality

After 13.4 establishes a fresh measurement basis, static GridOffsets may justify a tiled workgroup lowering
with canonical fold order preserved. LinkGraph must remain sparse and should not inherit dense-grid assumptions.

## 18.4 DerivedDependencyIndex → dispatch elision

The dependency index should eventually skip unchanged registrations/regions rather than merely identify them.
Potential staircase:

```text
unchanged registration → zero dispatch
local stencil dirtiness → dirty tile/span only
PALMA dirtiness         → active-region repair
Gu-Yang dirtiness       → affected conservative theater
```

## 18.5 Independent subtree parallelism

Detached subtrees already carry the semantic ingredients for physical parallel scheduling: stable identity,
generation stamps, holding-account conservation and one integration schedule.

```text
subtree A N+4 ─┐
subtree B N+2 ─┼─ stamped products → canonical parent integration
subtree C N+7 ─┘
```

The physical scheduler cannot become a semantic manager.

## 18.6 Prepare-parallel structural boundary

Level-local residency/granter indexing allows independent placement/remap preparation in parallel while commit
order and schedule recording remain deterministic and singular.

## 18.7 Current→Next carry remeasurement

The previous 4096-row evidence correctly rejected speculative sparse-active machinery at that scale. It should
be remeasured rather than extrapolated to very large resident planes.

## 18.8 Offline corpus projection

ML datasets should be columnar/batched projections of canonical replay/anchor state, never simulation-readable
telemetry unless explicitly authored.

---

# 19. Phase 13 and kernel containment

The most important conceptual improvement in Phase 13 is not a new facility. It is a sharper boundary:

```text
APPLICATION LAYER
ClauseThing / MapGen / Studio nouns
          │
          │ one-way lowering / interchange
          ▼
GENERIC AUTHORING + ADMISSION
SpecError law/path provenance
Properties / EML / RF / CostBand / ActionBand / OverlayThing / fields
          │
          ▼
CLOSED STEMTHING KERNEL
no ClauseThing dependency
no application-named manager
no game-specific accommodation
```

Phase 13.1's typed admission provenance improves this boundary without naming the application. Phase 13.2
repairs the ClauseThing lowerer rather than changing the kernel. Phase 13.5 will census any remaining
application vocabulary that leaked downward and queue it for generalization/relocation.

This makes the recursive kernel more, not less, domain-agnostic.

---

# 20. Final distinction table

| do not conflate | distinction |
|---|---|
| closed kernel semantics / legacy application convergence | ClauseThing reds can exist while the kernel law remains valid |
| application lowerer / kernel accommodation | Phase 13.2 permits the former and presumptively forbids the latter |
| typed admission provenance / new domain error taxonomy | 13.1 extends existing `SpecError` with generic law+element identity |
| logical identity / physical row | rows rebind; identity does not |
| observation / sink | crossing is observation; CostBand adds consumption |
| `U` / CostBand `R` | ungranted demand vs below-quantum value |
| PALMA / route object | potential field vs planner artifact |
| Gu-Yang / CostBand | realizable throughput vs sink price |
| grant entitlement / VRAM placement | WHO/HOW-MUCH vs WHERE |
| detachment / release | topology change does not terminate grant |
| semantic inheritance / span substrate | combine law vs physical projection |
| whole-tree weight materialization / weight semantics | 13.6 changes the former, not the latter |
| synthetic generation compatibility / real granter generation | 13.6 deletes the former |
| stale benchmark number / current performance debt | 13.4 requires remeasurement or dated instrument-stale status |
| telemetry / sim state | observation cannot become causal silently |
| EML gadget / opcode | PowerLaw composes EXP/LN; no POW opcode |
| contention / domain engine | oversubscription of generic clearing |
| physical scheduler / semantic manager | parallel execution cannot decide outcomes |

---

# 21. Revised engineering sequence

The earlier post-0.0.8.7 sequence should now be split into **Phase 13 closeout convergence** and a later
performance track.

## 21.1 Phase 13 — current workplan

```text
13.1 typed root admission provenance
     ↓
13.2 ClauseThing lowerer convergence + corpus hydrate/admit witness
     ↓
13.3 star-name golden truth adjudication
     ↓
13.4 workshop/performance-instrument triage + fresh/stale measurement law
     ↓
13.5 legacy producer / authoring-ingress / application-vocabulary census
     ↓
13.6 clearing-weight span unification + synthetic-generation clear deletion
     ↓
track closeout
```

## 21.2 Post-closeout performance track — only after dated measurements

Likely candidates, subject to the 13.4 measurement law:

```text
P0 market hot-path census
P1 compact/intern admitted market vocabulary
P2 GPU/resident constrained-clearing lowering
P3 current-head FieldSweep locality measurements
P4 tiled static-grid lowering if measurement supports it
P5 source-blind dirty dispatch / incremental fields
P6 parallel detached-subtree scheduling
P7 prepare-parallel structural boundary
P8 measured PALMA active-set / multiscale research
P9 offline columnar ML-corpus projection
P10 Vector CostBand only for a real atomic multi-resource consumer
```

No stale number may be used to skip the measurement gate.

---

# 22. Final synthesis

The architecture should now be stated in two layers.

**Graduated kernel statement:**

> **SimThing is a closed recursive simulation kernel. Each StemThing carries homogeneous Property
> state, intrinsic owner-aware RF participation, the EML numerical ISA, STEAD/PALMA/Gu-Yang fields,
> CostBand as the sole sink law, ActionBand as target lifecycle, OverlayThing as intrinsic actuation,
> StemThing-A as logical/physical residency discipline, and StemThing-B as the recursive
> conserved-resource market germ. Domain contention resolves through authored generic clearing,
> ordinary action returns through the same crossing/consequence surfaces, and asynchronous recursion
> is generation-stamped into one replay schedule.**

**Phase-13 closeout statement:**

> **Before calling the track operationally clean, the remaining legacy/application surfaces must
> converge onto that kernel without teaching the kernel their vocabulary; root admission must expose
> typed generic law+element provenance; stale ClauseThing lowerings and star-name truth must be
> repaired/adjudicated; legacy ingress must be censused; performance debts must have dated evidence;
> and StemThing-B clearing-weight inheritance must consume the already-graduated span substrate while
> the synthetic-generation compatibility clearing form is deleted.**

The revised target physical form after Phase 13 is therefore:

```text
fewer application-specific seams
fewer opaque admission errors
no synthetic-generation clearing footgun
no full-tree clearing-weight rematerialization
better-governed performance evidence
same bits
same authority
same generation law
same domain-agnostic StemThing germ
```

That is a stronger closeout than the first post-canonization review envisioned: Phase 13 is not reopening the
architecture. It is forcing the remaining estate to live up to the architecture that 12.2 canonized.

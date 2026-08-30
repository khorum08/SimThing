# SimThing Unification Model
## Post-Phase-13 architectural guide and engineering completion review

> **Status: WORKSHOP / NON-NORMATIVE / POST-PHASE-13 REVIEW.**
>
> This document is the engineering synthesis of the unified SimThing architecture after the
> 0.0.8.7 core canonization and the completed Phase-13 convergence arc. Normative authority remains
> [`../simthing_core_design.md`](../simthing_core_design.md), the live workplan, and DA rulings.
>
> **Reviewed state:** Phase 13 is complete at **7/7**. 13.6
> `CLEARING-WEIGHT-SPAN-UNIFICATION-0` graduated in #1891 @ `3766a002`; 13.7
> `PERFORMANCE-DEBT-LEDGER-COMPLETION-0` graduated in #1893 @ `9cdb46e2`; the track is
> **PARKED-PENDING-REVIEW** with no active rung. The final structural certificate attached to the
> last structural implementation rung is **126 suites / 478 passed / 0 failed / 14 ignored**.
>
> The review distinguishes four things deliberately:
>
> 1. **semantic design closure** — whether one uniform StemThing model owns simulation meaning;
> 2. **repository convergence** — whether application/legacy surfaces point toward that model;
> 3. **physical execution placement** — whether the closed semantics execute in their ideal CPU/GPU form;
> 4. **future capability** — things such as Vector CostBand that are not defects in the closed scalar model.

---

# 0. Executive verdict

The post-Phase-13 tree is materially closer to the intended stem-cell architecture than the post-12.2
state.

> **The uniform StemThing design is semantically closed enough to be called the governing runtime
> architecture. It is not yet physically optimized enough to call execution placement finished, and
> the repository still contains explicitly censused authoring/legacy residue.**

The strongest compact model remains:

```text
SimThing
 = recursive field-resident state
 + recursive conserved-resource economics
 + one EML numerical language
 + STEAD / PALMA / Gu-Yang field laws
 + CostBand quantization
 + ActionBand target lifecycle
 + OverlayThing actuation
 + StemThing-A residency
 + StemThing-B resource-market sovereignty
 + one generation / replay authority
```

Phase 13 materially strengthened that statement by making the following formerly transitional shapes
unrepresentable or non-authoritative:

```text
site-only root admission errors       -> typed law + element provenance
ClauseThing test-side admission shims -> application lowerer emits canonical shape
three permanent workspace reds        -> zero-red certificate law
folk performance numbers              -> dated measurement / owed-measurement ledger
uncensused legacy authoring routes     -> one constitutional census
whole-tree clearing-weight walk       -> 7.8a span/profile projection
synthetic-generation clearing door     -> deleted
```

The remaining high-value engineering work is therefore mostly about **where the one model executes**,
not which model is authoritative.

---

# 1. Uniform StemThing anatomy

A StemThing is the default-inert recursive simulation germ. Specializations add admitted data; they do
not add managers or peer simulation engines.

```text
                              STEMTHING
                                  │
       ┌──────────────────────────┼──────────────────────────┐
       │                          │                          │
       ▼                          ▼                          ▼
  Properties / RF                EML                    Field organ
 owner / state / lanes     numerical semantics       STEAD / PALMA /
       │                          │                     Gu-Yang
       │                          │                          │
       ├──────────────┐           │           ┌──────────────┤
       ▼              ▼           ▼           ▼              ▼
 StemThing-A      StemThing-B   CostBand   ActionBand     observations
 residency       flow markets      │           │
       │              │            └─────┬─────┘
       └──────────────┴──────────────────▼
                                  Phase-5 crossing
                                         │
                              CrossingConsequenceBinding
                              ┌──────────┼──────────┐
                              ▼          ▼          ▼
                         ResidentNext  Routed    Structural
                            Write      Overlay   Authorization
                              │          │          │
                              └──────┬───┘          ▼
                                     ▼       BoundaryProtocol
                              generation barrier
```

The four semantic legs remain:

| leg | meaning |
|---|---|
| `participate` | Property/RF/field participation; reduce-up and disburse-down |
| `act` | CostBand and ActionBand resolution |
| `originate` | attributable OverlayThing / ordinary products |
| `receive` | inherited, deficit-driven, predicate and routed input |

StemThing-A/B are lanes of the germ, not additional legs.

---

# 2. One authority cycle

```text
GENERATION N

Current resident fields
    │
    ├─ inherited owner / effective state
    ├─ OverlayThing projection
    ├─ RF claims / balances
    └─ field operands
    │
    ▼
reduce / accumulate
    │
    ▼
EML valuation + constrained clear
    │
    ├─ grant
    └─ unresolved U
    │
    ▼
STEAD / PALMA / Gu-Yang
    │
    ▼
anchored writes + Phase-5 crossings
    │
    ▼
CostBand / ActionBand
    │
    ▼
ResidentNext / RoutedOverlay / StructuralAuthorization
    │
================ generation barrier ================
    │
Current <- Next; structural commits; schedule/replay record
    │
    ▼
GENERATION N+1
```

Generation pacing is still the recursion bound. Same-generation clear→persist→re-clear, receive→emit
convergence, retry loops, and consequence re-entry are not part of the model.

Phase 13.6 strengthened this law by deleting the last generationless compatibility clearing form. The
ordinary constrained-clear API now requires the real granter and real `GenerationStamp` used by the
remainder-rotation law.

---

# 3. What Phase 13 actually changed

## 3.1 13.1 — typed admission provenance

Root admission no longer collapses selected failures into a site-only `ValidationFailedAt`. The
production root path now carries:

```text
SpecError::AdmissionRefused {
    law_id,
    element_path,
}
    -> InstallError::Spec
    -> SessionError::Install
    -> SimSession::open_from_spec caller
```

This matters architecturally because the admission boundary now reports **which generic law rejected
which authoritative element** without inventing ClauseThing-specific kernel error vocabulary.

The application can interpret the refusal; the kernel does not learn the application.

## 3.2 13.2 — ClauseThing converges downward only

ClauseThing was proven to be an application-layer authoring system rather than an engine dependency.
The existing `hydrate_*` layer now emits the modern admitted registry/arena/property/field shape directly.
Five test-owned pre-open shims fell to zero.

```text
ClauseScript
    ↓ parse
RawDocument
    ↓ hydrate / lower
modern generic spec
    ↓ ordinary admission
StemThing kernel
```

The critical containment law is now structural:

```text
application -> engine        lawful
engine -> ClauseThing        forbidden
```

No engine/spec/kernel/GPU/sim/driver/embedder semantic accommodation was needed for the ClauseThing
repair. This is strong evidence that the unified substrate is genuinely generic rather than merely a
Stellaris-shaped framework with hidden application dependencies.

## 3.3 13.3 — zero-red becomes the standing baseline

The final inherited star-name red was proven to be serialization-spelling drift, not naming-semantic
regression. After provenanced re-blessing, the project reached its first recorded zero-failed structural
certificate.

The important outcome is not the name fix itself. It is the new operational rule:

> **A future structural certificate red is a STOP, not an inherited footnote.**

This removes a dangerous source of architectural ambiguity: a new integration failure can no longer hide
inside a tolerated red baseline.

## 3.4 13.4 + 13.7 — performance claims become measured debts

Performance work is no longer chartered from remembered ratios or review prose. The workshop corpus is
leased with named consumers, and production-code debts identified by engineering review have dated ledger
entries.

Current non-workshop debt rows are:

```text
constrained_clearing.rs
    CPU host-shaped BTreeMap/sort/CPU-EML clearing

flow_market.rs
    authored market strings on hot lookup paths

facility_resident_plane.rs
    Current->Next carry at target cardinality

derived_span_projection.rs
    dispatch-elision benefit not yet proven
```

These are **owed measurements**, not permission to optimize blindly.

The old Gu-Yang gather-vs-tiled number is explicitly stale. Its archived instrument no longer builds
against the modern typed kernel; remeasurement is therefore a first-rung precondition of the future
performance track rather than current evidence.

## 3.5 13.5 — the legacy estate is now bounded

The existing constitutional census was extended rather than duplicated. It now records both:

```text
A. transitional / legacy semantic mediation
B. authoring ingress routes
```

Sixteen rows were added, with future actions:

```text
remove          2
internalize     5
preserve        8
blocked         1
```

The engine→ClauseThing zero-dependency arrow is mechanically checked, including Cargo table-header
syntax that initially escaped the parser and caused a DA remand.

This is important epistemically: repository residue remains, but it is no longer unknown residue.

## 3.6 13.6 — clearing-weight inheritance finally becomes fractal physically

Before 13.6:

```text
SimThing tree
    ↓ recursive CPU child walk
BTreeMap<SimThingId, f32>
    ↓
claim weights
```

After 13.6:

```text
already-admitted OverlaySpanProjection logical directory
    ↓
sparse override boundaries
    ↓
maximal DerivedSpanProjection<f32>
    ↓
interned effective profiles / spans
    ↓
claim weight lookup by logical identity
```

The old resolver is deleted. There is no fallback participant map or second weight path.

This is one of the most important post-canonization improvements because StemThing-B now physically
reuses the same subtree-profile machinery already justified for OverlayThing rather than carrying a new
whole-tree semantic projection architecture.

The new dependency direction is also correct:

```text
simthing-spec -> simthing-kernel    accepted
simthing-kernel -> simthing-spec    forbidden
```

The authoring/admission layer may depend on the physical substrate it admits into. Physics must never
consult authoring at runtime.

---

# 4. StemThing-B after Phase 13

The market grammar remains:

```text
admitted conserved resource / capacity
        ↓
sealed offering + Draw envelope
        ↓
descendant claim
        ↓
RF reduce-up
        ↓
effective EML clearing weight
        ↓
authored constrained clear
        ↓
CostBand quantization
        ↓
grant / flow disbursement
        ↓
Gu-Yang throughput / saturation
        ↓
PALMA potential / impedance
        ↓
STEAD observation / bands
        ↓
ActionBand / OverlayThing consequence
```

Unit cost and clearing weight remain independent. Equal scores express authored indifference;
proportional-within-band clearing plus largest remainder and generation-rotated exact ties preserve
conservation without physical-order bias.

A grant remains an identity-keyed lifecycle relation. Detachment does not imply release. Death /
dissolution release; fission/fusion partition or transfer exactly.

VRAM residency remains distinct physics:

```text
WHO / WHETHER / HOW MUCH   = StemThing-B market
WHERE                      = residency placement
```

That distinction survived Phase 13 unchanged.

---

# 5. Material conformance to the uniform design

## 5.1 One-way semantic dependency

The strongest evidence for uniformity is not documentation but dependency direction:

```text
application authoring
       ↓
spec/admission
       ↓
kernel / GPU physics
```

13.2 and 13.5 prove the inverse application dependency is absent. 13.6 further moves runtime-like
clearing-weight projection *down* into the kernel instead of leaving a spec-owned runtime resolver.

That is exactly the direction a stem-cell architecture should evolve.

## 5.2 Shared substrate reuse

The 13.6 change is architectural evidence because it removed a duplicate physical pattern:

```text
old: StemThing-B-specific recursive inheritance walk
new: generic 7.8a span/profile projection
```

This is stronger than code reuse. It means descendants, overlays, clearing weights and later effective
subtree values can share one physical description of fractal inheritance.

## 5.3 One clearing generation authority

Deletion of `clear_constrained_claims(...)` closes an awkward semantic footgun. There is no longer an
ordinary form that silently invents granter 0 / generation 0 for tie rotation.

Generation is now explicit at the clearing boundary, matching the one recorded schedule.

## 5.4 Application vocabulary is contained, not eradicated

The repository is **not** yet vocabulary-pure. `simthing-spec::designer_admission` still contains
ClauseThing/ClauseScript parking terminology and other historical guardrail names. 13.5 deliberately
censused this instead of silently refactoring it.

That residue does not presently constitute a second runtime engine. But it means the statement
“everything in the repository is already domain-neutral” would still be false.

The correct statement is:

> **The runtime kernel is domain-neutral; remaining application-named authoring residue is bounded by a
> checked post-closeout worklist.**

## 5.5 Zero-red integration state

The final structural Phase-13 implementation passed:

```text
126 suites
478 passed
0 failed
14 ignored
```

That does not prove absence of all defects. It does remove the previous ambiguity in which genuine
integration regressions could be normalized as inherited baseline failures.

---

# 6. CPU privilege audit

The uniform model does **not** require “no CPU.” It requires that CPU placement not become a peer
semantic authority.

## 6.1 CPU roles that are architecturally legitimate

These remain appropriate CPU work unless measurements prove another placement better:

```text
authoring parse / hydration / admission
stable identity and semantic labels
true structural mutation at generation barriers
VRAM extent bookkeeping / remap commit
persistence and replay I/O
read-only presentation / observation
bit-exact CPU reference oracles
```

Moving these merely to satisfy a GPU-purity aesthetic would make the architecture worse.

## 6.2 CPU routes that are physically transitional

### Constrained clearing

The canonical executor still performs host-side:

```text
BTreeMap supply grouping
BTreeSet duplicate detection
claim cloning / Vec grouping
CPU EML scoring
score sorting
score-band scans
largest-remainder arrays and sorting
grant Vec construction
```

This is now the clearest remaining numerical CPU privilege. It is semantically lawful because it is the
**one** clearing authority, but physically it does not resemble the long-term resident GPU model.

Do not optimize it before measurement: 13.7 intentionally made current-head measurement a prerequisite.
If it is material, the correct future shape is a faithful lowering of the same algorithm using resident
scope layout, segmented reductions/sorts, exact integer apportionment and the same real generation
authority.

### Market string identity

Offering/Draw/trigger identities remain human-readable strings through several admitted helper lookups.
This is probably an admission-interning opportunity rather than a semantic problem. Measure whether these
lookups survive into generation-scale work before introducing compact indices.

### Current->Next carry

Whole-plane carry was previously measured at small cardinality and deliberately preferred over sparse
freshness machinery. That decision remains valid at its measured scale. It must be remeasured at target
cardinality rather than overturned from intuition.

### DerivedDependencyIndex dispatch elision

The repository has precise source-blind dependency knowledge. It has not yet proven that the ordinary
scheduler consistently converts that knowledge into avoided field work. This is a potentially large
performance opportunity, but again the newly minted ledger correctly requires measurement first.

---

# 7. Fresh 13.6 audit findings worth resolving before closeout

These are not currently DA-declared defects. They are questions exposed by reviewing the freshly landed
code while its context is still hot.

## 7.1 Dynamic clearing-weight invalidation must be explicitly proven

The new `clearing_weight_projection.rs` correctly consumes the 7.8a span representation, but its admitted
projection is currently constructed with:

```text
DerivedDependencyIndex::admit(Vec::new())
```

Therefore 13.6 proves **span/profile representation and static sparse inheritance**, but the new
specialization itself does not presently register changed-locus dependencies.

This is harmless if the complete runtime law is:

```text
all operands that can deform effective clearing weight
    are resolved before projection construction,
    and any such change reconstructs the projection through an already-authoritative caller
```

It is a real gap if the intended law is:

```text
live Property / Overlay / EML operands may change effective clearing weight
    and 7.8a source-blind invalidation is expected to update only affected spans automatically
```

Because the Owner design explicitly described clearing weight as dynamically deformable through ordinary
EML/OverlayThing state, this distinction should be **proved now**, not left to the future performance
track.

Recommended pre-closeout action: a bounded archaeology/witness decision, not an optimization campaign.
Either:

1. prove current dynamic deformation enters through an existing rebuild path and document the exact
   `ChangedLocus -> projection rebuild` chain; or
2. if that chain is absent, mint one narrow convergence rung to bind clearing-weight dependencies into the
   existing `DerivedDependencyIndex` without adding a new registry or scheduler.

This is the single strongest “context is fresh” candidate from this review.

## 7.2 The new span builder has sparse-K, not constant, costs

The new resolver eliminates O(descendant-count) traversal, which is the important win. But its current
construction does several operations quadratic in the number of overrides `K`:

```text
duplicate detection        scans prior overrides
for each boundary interval filters all admitted overrides
per interval               allocates + sorts an active Vec
```

So the implementation is independent of total descendant count but can approach roughly O(K^2 log K)
for many overlapping override scopes.

This is not evidence of a current performance problem because `K` is intentionally sparse and unmeasured.
It is, however, a more accurate complexity statement than “resolution is proportional to override
boundaries.”

Recommended action now: add this as an owed-measurement sub-debt to the performance ledger, rather than
silently optimizing it before a realistic override-cardinality workload exists.

## 7.3 Avoid panic-bearing lookup as production vocabulary

`ClearingWeightSpanProjection::effective_weight` is fail-soft (`Option<f32>`), while its public `Index`
implementation panics for an absent participant.

If the `Index` form is test/harness convenience only, keep production consumers on the typed/optional lookup.
If a production consumer reaches the panic-bearing form, narrow or remove that surface before closeout.
A malformed/foreign logical identity should fail at admission or through a typed runtime boundary, not turn
into a host panic merely because the new representation is dense by construction.

This is a small audit with high leverage because the type has just landed.

---

# 8. Legacy load-bearing code: what matters and what does not

## 8.1 Application-named designer-admission vocabulary

13.5 found real historical vocabulary in the engine authoring layer, including ClauseThing/ClauseScript
parking codes. These rows are now explicitly marked for later removal.

They are architectural residue, but not currently a performance bottleneck or runtime decision authority.
Removing them now would improve repository purity, not simulation throughput.

Unless archaeology shows one still controls runtime behavior, this is better left to the bounded legacy
refactor track rather than reopening the just-completed numerical arc.

## 8.2 Studio / MapGen adapters

Several Studio and MapGen surfaces are classified `internalize` or `preserve-as-compat`. Their risk is
mainly **authoring truth duplication**, not hot-loop performance.

The key invariant to preserve is that generated or imported content terminates in the same admitted
StemThing spec and does not create a second scenario/session authority.

## 8.3 CPU reference paths

CPU oracles are intentionally retained for EML/GPU parity. They should not be mistaken for legacy authority.
Deleting the oracle because production should be GPU-resident would damage the exactness proof architecture.

## 8.4 Structural CPU boundary

Likewise, residency placement and true topology changes remain CPU/kernel boundary physics. The optimization
opportunity is parallel **preparation**, not moving semantic authorization back onto CPU or inventing GPU
structural mutation.

---

# 9. Opportunities that should wait for the performance track

The following are important, but Phase 13 deliberately created the measurement law that says **do not act on
these yet**:

```text
GPU/resident constrained clearing
market-id interning
Current->Next carry alternatives
DerivedDependencyIndex dispatch elision
Gu-Yang tiled/static-grid lowering
PALMA incremental / multiscale repair
parallel independently executing subtrees
prepare-parallel structural boundary work
columnar offline ML-corpus extraction
```

The right next step is instrumentation and current-head measurement, not speculative implementation.

The performance track should preserve the same semantic fences:

```text
same bits
same EML meaning
same generation authority
same IntegrationSchedule
same logical identity
same clearing rule
same field law
```

Optimization is a physical lowering of the one model, never a second fast-path model.

---

# 10. Opportunities potentially worth seizing while this workplan context is fresh

Only three items clear the bar for possible pre-closeout attention.

## 10.1 Prove clearing-weight dynamic-deformation lifecycle

This is the highest-value item because it could distinguish a mere performance omission from an actual
semantic integration gap. Resolve §7.1 before closeout.

## 10.2 Audit and, if necessary, narrow the panic-bearing clearing-weight `Index` surface

This is local to the freshly landed type and cheap to reason about now.

## 10.3 Add the sparse-override build complexity to the dated performance ledger

Do not optimize it. Merely keep the newly discovered cost from disappearing when context is reaped.

I would **not** reopen the track now for GPU clearing, string interning, tiling, Current/Next compaction or
legacy Studio/MapGen cleanup. Phase 13 just established the rule that these must be measured or bounded by a
specific future consumer first.

---

# 11. Does this complete the design?

The answer depends on what “design” means.

### Semantic runtime design: **yes, substantially.**

There is one recursive StemThing model, one numerical expression language, one field family, one RF/clearing
semantics, one sink law, one action lifecycle, one actuation facility, one residency authority split, one
history and one generation law. Phase 13 removed several concrete shapes that contradicted or obscured that
uniformity.

### Repository-wide architectural hygiene: **not completely.**

13.5 deliberately records application-named authoring residue and a bounded remove/internalize worklist. The
runtime kernel need not wait for that cleanup to be semantically closed, but the repository is not yet a
perfectly pure expression of the final design.

### Physical execution design: **not complete.**

The most important numerical contention executor is still CPU-shaped. Some field locality and dirty-work
opportunities remain unmeasured. These are performance-placement questions, not rival semantic architectures.

### Future capability design: **intentionally incomplete.**

Vector CostBand / atomic heterogeneous scarce-lane commitment remains a separate capability horizon and
should stay separate until a real consumer proves the need.

So the best final statement is:

> **The SimThing semantic architecture is complete enough to close the unification workplan once the fresh
> clearing-weight dynamic-invalidation audit is resolved. The engine is not “finished”; rather, future work
> should now be constrained to measured physical lowerings, bounded legacy-authoring convergence, and genuinely
> new capabilities—without reopening the meaning of SimThing itself.**

---

# 12. Policy/value-field interpretation — non-normative design consequence

One reason this closure matters is that higher-level aggregates need not become peer state machines.
Everything numerically meaningful can participate in the uniform field substrate, with EML composing
observable/value projections and PALMA/Gu-Yang giving potential and conserved-flow law where appropriate.

A future policy/value model can therefore use ordinary field dimensions such as employment, income,
education, security, credit and maintenance to derive ephemeral aggregates such as blight, legitimacy or
investment attractiveness without storing those aggregates unless they possess independent persistence.

Likewise, intervention discovery can treat non-geographic economic/institutional relations as admitted
relational topology rather than inventing a policy planner:

```text
problem discrepancy
    ↓
PALMA opportunity / impedance over admitted causal topology
    ↓
CostBand / RF / Gu-Yang realizability
    ↓
ActionBand target lifecycle
    ↓
OverlayThing intervention
```

This remains brainstorming, not Phase-13 law. Its importance here is architectural: the completed StemThing
substrate is now coherent enough that such domains can be authored as data and field relations rather than
requiring another engine.

---

# 13. Final reference distinctions

| do not conflate | distinction |
|---|---|
| logical identity / physical row | row may rebind; identity persists |
| observation / sink | crossing observes; CostBand consumes |
| unresolved `U` / CostBand `R` | ungranted demand vs below-quantum value |
| PALMA / route object | potential field vs planner artifact |
| Gu-Yang / CostBand | realizable throughput vs sink price |
| grant entitlement / residency placement | WHO/HOW-MUCH vs WHERE |
| detachment / release | topology change does not terminate a grant |
| OverlayThing / structural mutation | resident actuation vs boundary topology |
| semantic inheritance / span projection | combine law vs physical representation |
| source-blind invalidation / dispatch elision | dependency knowledge vs proven saved work |
| CPU oracle / CPU authority | reference proof is legitimate; peer resolution is not |
| application vocabulary / kernel semantics | authoring compatibility may exist above a domain-neutral runtime |
| performance debt / performance fact | a debt needs dated measurement or owed-measurement provenance |
| physical lowering / second engine | optimization must preserve one semantic authority |

---

# 14. Completion synthesis

After Phase 13, the architecture is no longer best described as “a collection of facilities that have been
made compatible.” The stronger description is now supported by both implementation and deletion evidence:

> **SimThing is one recursive simulation kernel. Its state and observations are field-resident; EML defines
> numerical meaning; RF carries recursive conserved participation and clearing; STEAD/PALMA/Gu-Yang provide
> the field-law family; CostBand quantizes sinks; ActionBand resolves target discrepancy; OverlayThing is
> actuation; StemThing-A binds logical identity to physical residency; StemThing-B gives every descendant the
> same recursive conserved-resource market germ; and the generation boundary plus IntegrationSchedule is the
> single temporal authority. Application languages lower into this kernel and never pull application semantics
> back down into it.**

Phase 13's strongest contribution was to make the repository behave more like that sentence:

```text
less application accommodation
less duplicate inheritance machinery
less compatibility authority
fewer tolerated reds
more explicit dependency direction
more measured-performance discipline
```

If the fresh clearing-weight dynamic-invalidation audit closes without finding a missing semantic edge, I
would treat the **unification design itself as complete** and resist adding further architecture to this
workplan. The next gains should come from proving where the one model is expensive and lowering it more
faithfully onto the hardware—not from inventing another abstraction layer.

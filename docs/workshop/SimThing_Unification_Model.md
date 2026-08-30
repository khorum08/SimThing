# SimThing Unification Model
## Post-13.8 completion review, latent-boundary finding, and performance-track handoff

> **Status: WORKSHOP / NON-NORMATIVE / INDEPENDENT POST-GRADUATION REVIEW.**
>
> This document is the engineering synthesis of the unified SimThing architecture after
> `CLEARING-WEIGHT-DEFORMATION-LIFECYCLE-0` graduated in PR #1897 and was stamped by #1898.
> Normative authority remains [`../simthing_core_design.md`](../simthing_core_design.md), the live
> workplan, and DA rulings.
>
> **Reviewed state:** Phase 13 is complete at **8/8**; the workplan reports no active rung and the
> final 13.8 structural certificate is **127 suites / 479 passed / 0 failed / 14 ignored**. The DA
> ruling is Board comment `5470334375`.
>
> **Independent-review disposition:** 13.8 closes the lifecycle gap identified by the prior edition
> of this guide. A deeper review of the landed refresh algorithm, however, finds one narrow untested
> semantic edge: an override boundary whose current output equals its surroundings can be merged out
> of the effective span map and cannot later reappear when an ancestor/default operand changes. This
> does not reopen the StemThing architecture. It is a local conformance defect in the newly landed
> clearing-weight refresh implementation and should be repaired before final closeout and performance-
> track execution.

---

# 0. Executive verdict

The SimThing architectural model is now coherent and materially implemented:

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

13.8 correctly completed the prior review's five above-the-line remedies:

| prior remedy | landed 13.8 disposition |
|---|---|
| replace empty clearing-weight dependency index | real default/override `ChangedLocus -> SpanRoot` bindings freeze at admission |
| prove source-blind dynamic invalidation | `refresh` consumes the existing 7.8a `invalidate -> remap_range` lifecycle |
| close the chain at actual market behavior | changed weight flips the next-generation constrained-clear outcome and replays identically |
| remove panic-bearing lookup | `Index<&SimThingId>` is deleted; `effective_weight -> Option` remains |
| record sparse-override construction cost | one dated PERFORMANCE-TRACK owed-measurement row landed |

Those are substantive changes. The clearing-weight feature no longer owns a private tree walk, private
invalidation vocabulary, synthetic generation, panic lookup, or silent performance premise.

The remaining problem found here is much smaller than the prior 13.6/13.8 gaps, but it is semantic rather
than merely physical:

> **Effective-value equality is not derivation equality. A maximally merged effective span may hide a
> descendant transform boundary that must become visible after an ancestor input changes.**

One narrow remedial rung or bounded DA patch should close that edge. After that, this guide considers the
**SimThing Unification Model complete** and recommends moving to measured physical-performance work rather
than adding another architectural layer.

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
| `originate` | attributable OverlayThing and ordinary products |
| `receive` | inherited, deficit-driven, predicate and routed input |

StemThing-A and StemThing-B are lanes of this germ, not fifth and sixth semantic engines.

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

Generation pacing is the recursion bound. Same-generation clear→persist→re-clear, receive→emit
convergence, retry loops, and consequence re-entry are not part of the model.

The final Phase-13 implementation strengthens this in two ways:

1. the only ordinary constrained-clear entry requires the real granter and `GenerationStamp`;
2. a changed clearing-weight operand participates no earlier than its admitted refresh generation and
   affects an ordinary later-generation claim rather than a clearing-local retry path.

---

# 3. What Phase 13 now proves

## 3.1 Admission and application containment

Root refusal provenance is typed as generic law plus authoritative element. ClauseThing converges through
its own lowering layer rather than teaching the kernel application vocabulary. The checked dependency
arrow remains:

```text
application authoring
       ↓
spec / admission
       ↓
kernel / GPU physics

kernel -> ClauseThing = forbidden
```

The legacy census does not claim the repository is vocabulary-pure. It does make residual application
mediation and authoring ingress machine-visible instead of tribal.

## 3.2 One clearing generation authority

The generationless `clear_constrained_claims` compatibility form is gone. Largest-remainder rotation is
always denominated by the real granter and its generation authority.

## 3.3 Fractal clearing-weight representation

The old whole-tree resolver and participant `BTreeMap` were deleted. Clearing weights now use the same
logical subtree directory, maximal effective spans, effective profile identities, and source-blind
invalidation substrate that the OverlayThing closure established.

```text
OverlaySpanProjection logical directory
        ↓
sparse authored default / overrides
        ↓
DerivedSpanProjection<f32>
        ↓
maximal effective spans / profiles
        ↓
logical-id lookup
```

## 3.4 Dynamic deformation lifecycle

13.8 removes the empty dependency index and freezes real source bindings:

```text
default ChangedLocus  -> SpanRoot(root)
override ChangedLocus -> SpanRoot(override target subtree)
```

The landed lifecycle is:

```text
eager admission
    ↓
operand changes
    ↓
ChangedLocus
    ↓
DerivedDependencyIndex
    ↓
coalesced affected logical ranges
    ↓
DerivedSpanProjection::remap_range
    ↓
refreshed effective profiles
    ↓
next-generation claim / constrained clear
```

The 13.8 witness proves a child override change affects only its two-row subtree, leaves unrelated
profile identities stable, flips the ordinary generation-8 grant outcome, and replays identically.

## 3.5 Performance claims are debts, not folklore

The performance ledger now contains dated owed-measurement entries for:

```text
CPU host-shaped constrained clearing
market strings on hot lookup paths
Current->Next facility-plane carry at target cardinality
DerivedDependencyIndex dispatch elision
clearing-weight sparse-K construction / refresh cost
```

The historical Gu-Yang gather-vs-tiled result is explicitly stale because its archived instrument no
longer builds against the modern typed kernel. It may motivate remeasurement, not an implementation
conclusion.

---

# 4. Material conformance to the uniform design

13.8 is strongly conformant in the following respects.

## 4.1 Source-blind invalidation

The key is an authoritative numerical locus:

```text
(logical SimThing id, PropertyId, SubFieldRole, optional narrowing)
```

There is no writer-subsystem discriminator. The same changed state invalidates the same work whether it
originated through RF, OverlayThing, ActionBand, authoring, or another admitted route.

## 4.2 Shared physical substrate

Clearing weights consume the generic 7.8a machinery rather than introducing a clearing-specific dirty
map, registry, scheduler, or cache. This is the right implementation expression of the recursive germ.

## 4.3 Generation pacing and replay

The changed operand does not cause a same-generation retry. The refreshed weight is consumed by an
ordinary later-generation constrained clear. Repeating the same stamped inputs yields the same result.

## 4.4 Safe lookup vocabulary

The panic-bearing `Index` implementation is gone. Missing/foreign logical identity remains representable as
`None` and can be handled by the admitted boundary rather than becoming an untyped host panic.

## 4.5 Dependency direction

`simthing-spec -> simthing-kernel` remains doctrine-positive: admission may depend on the physics substrate
it admits into. `simthing-kernel -> simthing-spec` remains forbidden.

## 4.6 No new CPU semantic route

13.8 adds no production scheduler, cache, dynamic-deformation manager, alternate clear, or GPU/CPU split
authority. The standing witness proves the generic facility; it does not create a domain manager.

---

# 5. Independent review finding S1 — latent semantic boundaries can be erased

**Classification: semantic implementation defect in the new refresh path; not a performance debt and not a
new architectural requirement.**

The admission path correctly starts from every override subtree boundary. It evaluates each interval, then
maximally merges adjacent intervals when their *current effective value bits* are equal.

Conceptually:

```text
all semantic override boundaries
        ↓
evaluate current effective value per interval
        ↓
if adjacent values are equal, merge into one effective span
```

That merging is lawful as a compact **current-value representation**. It becomes unsafe when the merged
intervals represent different transfer functions over future ancestor inputs.

The landed `refresh` reconstructs values only at starts of **current effective spans**:

```text
for each current span intersecting the affected range:
    evaluate at current_span.start
    remap the whole overlap with that one result
```

It does not reintroduce override starts/ends that were merged away because they happened to produce the same
value at admission.

## 5.1 Minimal falsifier

Consider four logical rows with a child subtree covering rows `[1,3)`:

```text
root default weight = 1.0
child override       = Set(1.0)
```

At admission:

```text
root outside child = 1.0
child subtree      = 1.0
```

The current implementation lawfully compresses this to one effective span:

```text
[0,4) -> 1.0
```

Now change only the root/default operand to `2.0` and emit the default `ChangedLocus`.

The correct inherited semantics are:

```text
[0,1) -> 2.0
[1,3) -> 1.0   // child Set remains authoritative in its subtree
[3,4) -> 2.0
```

The current refresh sees only the one existing `[0,4)` span, evaluates at row `0`, obtains `2.0`, and remaps
the whole root range to `2.0`:

```text
[0,4) -> 2.0   // incorrect: the latent child Set boundary cannot reappear
```

The problem is general:

> **Equality at one input does not prove two derived functions are equivalent for future inputs.**

A `Set(1.0)` descendant and an inherited `1.0` environment have equal current outputs but different responses
to an ancestor change. Similar cases exist for nested non-commutative transform stacks that temporarily
converge numerically.

## 5.2 Why the 13.8 witness does not catch it

The 13.8 positive witness changes the child override locus itself. Its affected range begins at the child
subtree boundary, so `remap_range` splits the current parent span at that supplied range and behaves correctly.

The missing case is the opposite direction:

```text
ancestor/default change
    ↓
broad affected range
    ↓
previously collapsed descendant semantic boundary must re-emerge
```

The default-change witness also begins with three already-distinct effective spans, so it proves range-local
refresh over visible boundaries, not re-expansion of a hidden one.

---

# 6. Recommended immediate remediation

I recommend one final narrow remedial rung before closeout, at DA numbering discretion, such as:

```text
13.9 CLEARING-WEIGHT-SEMANTIC-PARTITION-0
```

This is not another unification design cycle. It makes the 13.8 implementation faithful to the already-
settled inheritance/deformation law.

## 6.1 Law

> Effective spans may merge equal current values for storage and lookup, but dynamic refresh must preserve or
> reconstruct every admitted derivation boundary capable of producing a different value after an upstream
> operand change. Current-value equality may never erase future derivation semantics.

## 6.2 Minimal lawful implementation shape

Retain the existing frozen override target ranges. For every affected range:

```text
affected-range start/end
    + every intersecting override subtree start/end
        ↓
sorted semantic windows
        ↓
resolved_weight_at(window.start) for each window
        ↓
range-local remap per semantic window
        ↓
ordinary adjacent-equal coalescing after evaluation
```

The resulting effective span map remains maximal. No per-descendant materialization is needed. No new
registry, scheduler, cache, writer discriminator, or clearing path is justified.

A second possible implementation is to retain a separate frozen semantic-partition descriptor at admission.
That is only physical metadata; the authoritative semantics remain the admitted override shapes. Reconstructing
from the already-frozen override ranges is probably the smaller repair.

## 6.3 Exit proof

The remedial proof should include all of these:

1. **Collapsed `Set` boundary reappears:** root default `1 -> 2`, child `Set(1)` remains `1`.
2. **Nested ancestor/child case:** changing an ancestor override preserves a deeper child's distinct transform.
3. **Re-coalescence:** when distinct semantic windows again produce equal values, effective spans collapse back
   to the maximal representation.
4. **Affected-only:** no descendant member scan; unrelated ranges and profile identities remain unchanged.
5. **Seals:** 13.6 matrix, germ, 13.8 lifecycle witness, integer apportionment, tie rotation, and replay remain
   semantically unchanged.
6. **Zero-red certificate:** the standing post-13.3 rule holds.

Until that falsifier is red-before/green-after, this guide cannot honestly certify the current implementation
as complete even though the governing architecture is complete.

---

# 7. Secondary fresh-code finding — proof instrumentation scans every span

`ClearingWeightSpanProjection::refresh` calls `unaffected_profile_samples` before remapping. That helper walks
all effective spans and allocates a sample vector so the post-refresh code can report whether unaffected profile
identities changed.

This is useful proof logic, but it means the production method's physical work is not strictly limited to the
reported affected spans:

```text
invalidation metric: spans_examined = affected current spans
actual refresh:       additionally scans all spans for proof samples
```

`logical_member_rows_scanned = 0` remains true, but the implementation performs an O(number-of-spans) audit on
every refresh. That global scan is not reflected by the headline selectivity metrics.

Because the code is fresh and the scan exists only to support a proof assertion, I would fold its cleanup into
the same narrow remediation:

- move the exhaustive unaffected-profile check to test-only proof code; or
- sample only bounded neighboring/unaffected profiles required by the witness; or
- expose a test-only profile snapshot API while leaving production refresh strictly affected-range-local.

This is not a separate architectural blocker, but it is better removed now than normalized as hidden runtime
cost and rediscovered during the performance track.

---

# 8. Consumer boundary after 13.8

The rung intentionally found and preserved **zero production dynamic-deformation callers**. Therefore the
landed truth is precise:

```text
proven:
    the generic lifecycle is representable and works end to end in the standing witness

not yet claimed:
    every ordinary shipped session automatically emits clearing-weight ChangedLocus events
    and invokes refresh
```

That is acceptable for an intrinsic capability whose first real domain consumer has not yet landed. It does
not justify manufacturing a manager or scheduler merely to create a caller.

The first production consumer must use the same ordinary chain:

```text
authoritative operand write
    ↓
ChangedLocus
    ↓
existing dependency index
    ↓
generation-paced refresh
    ↓
ordinary claim clear
```

It may not directly mutate a participant weight table, maintain a private cache, or call a clearing-specific
listener. This adoption obligation belongs with the first actual consumer, not the current unification closeout.

---

# 9. CPU, field-locality, and feeder posture

These areas remain sufficiently robust to support the refactored SimThing. They are not above-the-line
semantic blockers.

## 9.1 CPU constrained clearing

The one canonical executor remains host-shaped:

```text
BTreeMap/BTreeSet grouping and duplicate checks
CPU EML scoring
comparison sorts and score-band scans
largest-remainder arrays / ordering
grant Vec construction
```

This is a physical-placement debt, not a second semantics. The performance ledger now requires current-head
measurement before any GPU/resident lowering is chartered.

## 9.2 Field locality

STEAD, PALMA, and Gu-Yang share the admitted FieldSweep execution model. Static-grid tiling, dirty-region
repair, and dispatch elision remain physical lowerings. The old Gu-Yang ratio is stale and cannot be cited as
current evidence.

## 9.3 Feeder

The feeder is older-shaped transport around the closed kernel, not a peer simulation engine. Its lawful role
is:

```text
external / boundary intent transport
        ↓
logical identity + role resolution
        ↓
GPU intent deltas or parked structural products
        ↓
ordinary StemThing execution
```

CPU-shadow and direct-patch surfaces must remain subordinate and non-authoritative. Existing exclusivity and
legacy-surface censuses are the guards against a new production caller turning those helpers into a second
simulation ingress. Stale feeder comments are documentation debt, not a reason to reopen StemThing semantics.

---

# 10. Legacy residue

The runtime kernel is domain-neutral; the repository is not yet vocabulary-pure.

The 13.5 census records application-named designer-admission vocabulary, ClauseThing projection adapters,
Studio generation/hydration branches, MapGen authoring strategies, and four authoring ingress families. The
bounded worklist remains useful, but those rows are primarily authoring-truth and repository-hygiene concerns.

They do not currently alter the closed runtime model, and they should not delay the performance track once the
semantic-partition defect is repaired and the Owner completes closeout.

---

# 11. Performance-track handoff after the narrow repair

Once §5 is remediated, the unification track should stop adding architecture. The performance track should
begin with measurements over the dated debt inventory.

Recommended opening order:

```text
P0  restore/rebuild current-head Gu-Yang locality instrument
    and measure generic gather vs faithful local lowering

P1  measure CPU constrained clearing by claim count, scope count,
    score-band distribution, allocation volume, and EML program shape

P2  measure clearing-weight sparse-K admission and refresh separately
    after the semantic-partition fix

P3  measure whether market strings survive into generation-scale work;
    intern only if they do

P4  measure Current->Next carry at intended facility-plane cardinality

P5  prove the value of DerivedDependencyIndex dispatch elision
    before building incremental field machinery

P6  measure feeder readback/upload and intent-fold costs as transport,
    never as a semantic authority question

P7  only then evaluate GPU/resident clearing, tiled fields,
    incremental PALMA, subtree concurrency, and boundary preparation
```

Every lowering must preserve:

```text
same bits
same EML meaning
same generation authority
same IntegrationSchedule
same logical identity
same clearing law
same Field-Triad law
```

---

# 12. Does this close the SimThing Unification Model?

## Architectural design: **yes.**

The uniform meaning of SimThing is complete:

- one recursive germ;
- one field-resident state model;
- one EML numerical language;
- one RF/constrained-clearing market grammar;
- one STEAD/PALMA/Gu-Yang field family;
- one CostBand sink law;
- one ActionBand target lifecycle;
- one OverlayThing actuation facility;
- one StemThing-A residency split;
- one StemThing-B recursive resource-market lane;
- one generation and replay authority.

No additional architecture is suggested by this review.

## Current implementation: **not quite.**

The latent semantic-boundary defect in §5 can produce an incorrect descendant clearing weight after a broad
ancestor/default deformation. That is a real semantic edge and should not be deferred into a performance track.

## Closure condition

> Repair the latent-boundary case through the existing semantic partition and 7.8a remap vocabulary, remove or
> test-confine the global proof scan, preserve all prior seals, and obtain the zero-red certificate.

After that narrow repair, I would mark this workshop guide **complete**, authorize 0.0.8.7 closeout on the
Owner's call, and move to performance-track authoring without another unification review cycle.

---

# 13. CausalBand as a downstream consequence, not another kernel organ

The full SimThing composition now has a useful downstream designer-facing name:

> **CausalBand** is a full-SimThing causal field: STEAD is the substrate; EML couples and projects the
> dimensions; PALMA exposes opportunity/impedance; Gu-Yang constrains realizable conserved flow; RF and
> CostBand resolve scarce means and thresholds; ActionBand expresses target discrepancy; OverlayThing actuates
> change.

The CausalBand Atlas and its time-indexed corpus are downstream applications of the closed germ. They do not
require another simulation engine or another state authority. Frequently consumed aggregates such as blight
may be atlas-materialized derived channels while their truth remains entirely determined by admitted
underlying state and derivation.

This section remains workshop interpretation, not 0.0.8.7 normative law.

---

# 14. Final distinctions

| do not conflate | distinction |
|---|---|
| current-value equality / derivation equality | equal outputs today may diverge after an upstream change |
| effective span / semantic boundary | compact materialization may merge values; refresh must retain derivation topology |
| logical identity / physical row | row may rebind; identity persists |
| observation / sink | crossing observes; CostBand consumes |
| unresolved `U` / CostBand `R` | ungranted demand vs below-quantum value |
| PALMA / route object | potential field vs planner artifact |
| Gu-Yang / CostBand | realizable throughput vs sink price |
| grant entitlement / residency placement | WHO/HOW-MUCH vs WHERE |
| detachment / release | topology change does not terminate a grant |
| semantic inheritance / span projection | combine law vs physical representation |
| source-blind invalidation / dispatch elision | dependency knowledge vs proven saved work |
| CPU oracle / CPU authority | reference proof is legitimate; peer resolution is not |
| application vocabulary / kernel semantics | authoring compatibility may remain above a domain-neutral runtime |
| performance debt / performance fact | a debt needs dated measurement or owed-measurement provenance |
| physical lowering / second engine | optimization must preserve one semantic authority |

---

# 15. Completion synthesis

13.8 is a strong and necessary completion of the review's original lifecycle concern. It gives clearing weights
real source provenance, one generic invalidation authority, range-local refresh, safe lookup, generation-paced
claim consumption, and a dated performance debt rather than speculative optimization.

The independent code review nevertheless exposes one final implementation lesson:

```text
maximally compact current state
    must not erase
future semantic distinctions in the derivation graph
```

That lesson is fully compatible with the existing design. It needs no new field law, manager, cache, scheduler,
or executor. It requires only that clearing-weight refresh reconstruct the already-admitted semantic windows
inside an affected range before re-coalescing equal outputs.

Once that narrow conformance repair lands, the strongest final statement is justified:

> **SimThing is one recursive simulation kernel. Its state and observations are field-resident; EML defines
> numerical meaning; RF carries recursive conserved participation and clearing; STEAD/PALMA/Gu-Yang provide
> the field-law family; CostBand quantizes sinks; ActionBand resolves target discrepancy; OverlayThing is
> actuation; StemThing-A binds logical identity to physical residency; StemThing-B gives every descendant the
> same recursive conserved-resource market germ; and the generation boundary plus IntegrationSchedule is the
> single temporal authority. Application languages lower into this kernel and never pull application semantics
> back down into it.**

After the latent-boundary repair, further gains should come from measured physical lowerings of that one model,
not from another unification abstraction.

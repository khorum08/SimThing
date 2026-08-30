# SimThing Unification Model
## Final post-13.9 completion review and performance-track handoff

> **Status: WORKSHOP / NON-NORMATIVE / COMPLETE.**
>
> This document is the final engineering synthesis of the unified SimThing architecture after
> `CLEARING-WEIGHT-SEMANTIC-PARTITION-0` graduated in PR #1901 at `7df36050` and was stamped by
> PR #1902 at `389fe23b`. Normative authority remains
> [`../simthing_core_design.md`](../simthing_core_design.md), the live 0.0.8.7 workplan, and DA rulings.
>
> **Reviewed state:** every rung of 0.0.8.7 is DA-GRADUATED: the core ladder through 12.2 and nine
> Phase-13 addenda. The final DA ruling is Board comment `5471181522`; the final structural
> certificate is **128 suites / 481 passed / 0 failed / 14 ignored**.
>
> **Disposition:** the SimThing unification design is complete. The semantic defect identified by the
> preceding edition of this guide—the loss of equal-valued but derivationally distinct clearing-weight
> boundaries—has been repaired through the existing 7.8a span/remap vocabulary, with exact
> RED-before/GREEN-after evidence and re-coalescence. No further unification rung is recommended.
> Future work should be limited to measured physical lowerings, bounded legacy-authoring convergence,
> or genuinely new capability.

---

# 0. Executive verdict

The final architecture is no longer best described as several mechanisms that happen to cooperate.
It is one recursive simulation kernel:

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

The closed model has a single direction of semantic dependency:

```text
application authoring
        ↓
canonical lowering / admission
        ↓
uniform StemThing kernel
        ↓
physical CPU / GPU lowerings
```

There is no lawful reverse dependency in which the kernel consults ClauseThing, Studio, MapGen, a
manager object, a scheduler-owned policy, a CPU-only planner, or a second clearing route to determine
simulation meaning.

The workplan now supports three strong statements simultaneously:

1. **Composition:** the facilities actually run together through a generation-paced causal witness.
2. **Exclusivity:** no second production route may bypass the unified ingress and clearing chain.
3. **Fractal closure:** inherited state, overlays, clearing weights, fields, claims, grants, action and
   actuation all reuse the same logical tree/span/generation vocabulary rather than acquiring
   subsystem-specific walkers or histories.

The remaining work is therefore about **where and how efficiently the one model executes**, not which
model owns truth.

---

# 1. Canonical StemThing anatomy

A StemThing is the default-inert recursive simulation germ. A SessionThing, OwnerThing,
GridcellThing, population cohort, fleet-like thing, network-management thing, compute-node thing or
other specialist receives the same intrinsic mechanics. Specialization adds admitted data; it does not
add a peer manager, engine, allocator, planner or event executor.

```text
                                  STEMTHING
                                      │
       ┌──────────────────────────────┼──────────────────────────────┐
       │                              │                              │
       ▼                              ▼                              ▼
 Properties / owner / RF             EML                         Field organ
 resident state and lanes      one numerical ISA          STEAD / PALMA / Gu-Yang
       │                              │                              │
       ├───────────────┐              │              ┌───────────────┤
       ▼               ▼              ▼              ▼               ▼
 StemThing-A       StemThing-B     CostBand      ActionBand     observations
 residency        flow markets        │              │
       │               │              └──────┬───────┘
       └───────────────┴─────────────────────▼
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

The four semantic legs are:

| leg | intrinsic meaning |
|---|---|
| `participate` | hold Property/RF/field state, reduce upward and disburse downward |
| `act` | resolve CostBand crossings and ActionBand target discrepancy |
| `originate` | own and route attributable OverlayThing and ordinary products |
| `receive` | accept inherited, deficit-driven, predicate-broadcast and routed input |

StemThing-A and StemThing-B are lanes of the germ, not additional semantic legs.

The shared organs are:

```text
expression organ = EML
field organ      = RF + STEAD + PALMA + Gu-Yang
sink              = CostBand
execution target  = ActionBand
actuation         = OverlayThing
structure         = StemThing-A + BoundaryProtocol
resource market   = StemThing-B
history           = IntegrationSchedule + canonical replay
```

Every row is sparse and inert by default. The fact that a SimThing can participate in a market,
receive a policy, originate an overlay, host descendants or become an independently executing subtree
does not require active state until admitted data exercises that capability.

---

# 2. One authority cycle

```text
GENERATION N

Current resident state
    │
    ├─ inherited owner / effective profile
    ├─ OverlayThing projection
    ├─ RF amounts, claims and balances
    ├─ Field-Triad operands
    └─ changed-locus provenance
    │
    ▼
recursive reduce / accumulate
    │
    ▼
EML valuation + constrained clear
    │
    ├─ accepted grants / flows
    └─ unresolved U
    │
    ▼
STEAD / PALMA / Gu-Yang
    │
    ▼
anchored observations + Phase-5 crossings
    │
    ▼
CostBand / ActionBand
    │
    ▼
ResidentNextWrite / RoutedOverlayDelivery / StructuralAuthorization
    │
===================== generation barrier =====================
    │
Current <- Next
boundary commits / rebinds / lifecycle facts
IntegrationSchedule + replay record
    │
    ▼
GENERATION N+1
```

Generation pacing is the recursion bound. The model has no same-generation:

```text
clear -> persist -> re-clear
receive -> originate convergence
retry solver
consequence re-entry
structural mutation during field evaluation
```

A detached or independently scheduled subtree may run at a different generation, but every product is
stamped and integrated through the one schedule. Physical asynchrony never becomes semantic ambiguity.

---

# 3. What 0.0.8.7 and Phase 13 closed

## 3.1 Core closure through 12.2

The core ladder established:

- one recursive root contract;
- stable logical identity with epoch-rebindable physical placement;
- one Property/RF/overlay numerical substrate;
- EML EXP/LN and the full admitted numerical gadget path;
- one Field-Triad execution IR;
- CostBand as the sole sink/continuous-to-discrete quantizer;
- ActionBand as intrinsic target lifecycle rather than a planner;
- OverlayThing as intrinsic actuation rather than a peer event engine;
- StemThing-A residency and StemThing-B recursive conserved-resource markets;
- one crossing/consequence ABI;
- one generation and replay authority;
- one Vendor Door and one unified-ingress exclusivity gate.

11.3 proved the composed causal chain rather than merely asserting it:

```text
residency / grant fact N
    ↓
grant lane publication N+1
    ↓
ActionBand crossing and routed consequence N+2
    ↓
OverlayThing attachment N+3
    ↓
stable terminal state N+4
```

11.4 separately proved that no second production route may bypass that chain.

## 3.2 Phase-13 convergence sequence

Phase 13 did not redesign SimThing. It forced the remaining implementation estate to conform to the
already-canonized model.

| rung | closure contribution |
|---|---|
| 13.1 | typed root-admission provenance: generic law id + element path |
| 13.2 | ClauseThing lowering converges one-way onto the generic kernel; five test shims fall to zero |
| 13.3 | stale star-name golden adjudicated; project reaches the first zero-failed structural baseline |
| 13.4 | workshop/performance corpus triaged; stale benchmark folklore cannot charter work |
| 13.5 | legacy producers, authoring ingress and application vocabulary receive one checked census |
| 13.6 | whole-tree clearing-weight walk deleted; 7.8a spans become the sole representation; synthetic-generation clearing door deleted |
| 13.7 | non-workshop performance debts gain dated owed-measurement ledger rows |
| 13.8 | real clearing-weight `ChangedLocus` dependencies and affected-range refresh land; panic lookup removed |
| 13.9 | semantic override partitions are reconstructed during refresh so equal-valued latent boundaries can reappear correctly |

The certificate moved from:

```text
Phase-13 opening: 124 suites / 472 passed / 3 failed / 14 ignored
Phase-13 close:   128 suites / 481 passed / 0 failed / 14 ignored
```

The zero-red law is now standing: a future structural red is a STOP, not tolerated background noise.

---

# 4. Deep review of 13.9 `CLEARING-WEIGHT-SEMANTIC-PARTITION-0`

The cited DA ruling `5471181522` is for **13.9**, the follow-on remediation to 13.8.

## 4.1 The defect it repaired

13.8 correctly bound clearing-weight operands to the existing source-blind dependency index, but the
refresh implementation used the **current compressed effective spans** as the only recomputation
partition.

That lost derivational distinctions when two regions happened to have the same value.

Example:

```text
root default = 1.0
child override = Set(1.0)
```

The current effective field can lawfully compress to one physical span:

```text
[root subtree] = 1.0
```

But the derivation is not homogeneous. If the root default later becomes `2.0`, the correct result is:

```text
root + sibling  = 2.0
child subtree   = 1.0
```

The old refresh evaluated only the merged root span start and incorrectly wrote `2.0` across the child.

The governing distinction is:

```text
current-value equality != derivation equality
```

## 4.2 The landed repair

13.9 keeps the compact effective representation but restores semantic partitioning from the admitted,
frozen override ranges.

For every affected logical range it now forms windows from:

```text
affected start / end
+
start / end of every intersecting admitted override subtree
```

Then, for each resulting window:

```text
resolve active override composition at window start
    ↓
remap that exact window through existing DerivedSpanProjection::remap_range
    ↓
allow the existing adjacent-equal coalescer to merge equal results
```

No new partition registry, cache, scheduler, tree walk, row map or clearing authority was introduced.

## 4.3 Why the repair is general

StemThing subtree ranges are laminar: two subtree ranges are either disjoint or one contains the other.
Within an interval bounded by all intersecting subtree starts and ends, the set and order of active
overrides are constant.

Therefore:

```text
one evaluation at semantic-window start
=
correct effective value for every logical row in that window
```

This remains true for ordered `Set`, `Add`, `Multiply` and other admitted `TransformOp` composition,
because the active derivation stack—not the current output value—defines the window.

The repair is therefore not a fixture-specific patch. It restores the correct generic lowering:

```text
frozen derivation partition
        ↓ evaluate
compressed effective spans
```

rather than trying to infer derivation from compressed outputs.

## 4.4 Acceptance evidence

The exact previously identified S1 falsifier was captured RED-before and then made GREEN:

```text
admission physical spans = 1
root default 1.0 -> 2.0
root / sibling           = 2.0
child / grandchild       = 1.0
restore default to 1.0
physical spans return    = 1
```

A nested case separately proves that an equal-compressed leaf `Set` remains authoritative when its
ancestor multiplier changes and that the regions re-coalesce when the ancestor returns.

The locality evidence is also meaningful:

```text
S1 default change:
  affected ranges             1
  affected rows               4
  dirty / examined spans      1 / 1
  semantic windows rebuilt    3
  logical member rows scanned 0

nested target change:
  affected ranges             1
  affected rows               2
  dirty / examined spans      1 / 1
  semantic windows rebuilt    2
  logical member rows scanned 0
```

Production contains no all-projection `iter_spans` or `spans_in_range` walk in the clearing-weight
module. Broad unchanged-profile proofs were moved out of the production full-scan shape; the remaining
production probes are bounded immediate neighbours.

## 4.5 Preservation evidence

The 13.6 matrix, StemThing-B germ and 13.8 lifecycle witness remained byte-equal as predecessor seals.
The repair did not change:

```text
clearing arithmetic
eligibility
score ordering
epsilon policy
largest-remainder law
exact tie rotation
DecimalField exactness
generation authority
replay law
dependency direction
```

Focused 13.9 + 13.8 + matrix + germ + generic-span + constrained-clearing batteries all passed, and
the full workspace ended at 128/481/0/14.

## 4.6 Conformance verdict

13.9 satisfies every remedy requested by the preceding edition of this guide:

- semantic boundaries derive from admitted override structure, never observed equality;
- latent boundaries reappear under ancestor/default change;
- nested boundaries remain correct;
- equal results re-coalesce through the existing merge authority;
- production refresh is affected-range local;
- full proof scans do not ride the hot method;
- all predecessor semantics remain sealed.

**Result: PASS. The clearing-weight lifecycle is now materially and semantically conformant to the
uniform StemThing design.**

---

# 5. Material conformance to the uniform StemThing design

## 5.1 One derivation substrate

Before Phase 13, StemThing-B clearing weights used a recursive CPU tree walk and per-resolution
participant map.

The final shape is:

```text
OverlaySpanProjection logical directory
        ↓
frozen ChangedLocus dependency rows
        ↓
DerivedDependencyIndex
        ↓
affected logical ranges
        ↓
semantic override windows
        ↓
DerivedSpanProjection<f32>
        ↓
effective_weight(SimThingId) -> Option<f32>
```

Overlay inheritance and clearing-weight inheritance now share the same physical idea—logical subtree
ranges, effective profiles and source-blind invalidation—while retaining their own semantic combine laws.

## 5.2 One clearing authority

The generationless compatibility form is gone. Every ordinary constrained clear now receives genuine:

```text
ClearingRemainderAuthority {
    granter: SimThingId,
    generation: GenerationStamp,
}
```

Equal-score proportional clearing, exact integer largest remainder and exact-tie generation rotation
therefore belong to the same stamped authority as the rest of the simulation.

## 5.3 One source-blind change vocabulary

Clearing weights do not ask whether a Property, OverlayThing, ActionBand or another subsystem authored a
change.

They consume:

```text
ChangedLocus {
    logical identity
    PropertyId
    SubFieldRole
}
```

Identical authoritative changes invalidate identical derived work. No writer/source enum or clearing-
specific listener exists.

## 5.4 One application boundary

ClauseScript / ClauseThing, MapGen and Studio remain authoring applications or adapters. They lower into
ordinary admitted SimThing data.

```text
ClauseScript / authoring application
             ↓
canonical lowerer / interchange
             ↓
generic spec + admission
             ↓
closed StemThing kernel
```

The engine-to-ClauseThing dependency arrow is mechanically forbidden. Remaining application-named
vocabulary in `designer_admission` is censused legacy residue, not runtime authority.

## 5.5 One actuation route

CostBand and ActionBand crossing consequences still terminate in the same three-arm ABI:

```text
BandCrossingDelta
        ↓
CrossingConsequenceBinding
        ├─ ResidentNextWrite
        ├─ RoutedOverlayDelivery
        └─ StructuralAuthorization -> BoundaryRequest
```

OverlayThing remains the ordinary resident actuation/lifecycle facility. Structural mutation remains a
boundary authorization, not a GPU-local foreign write.

## 5.6 One history

All asynchronous products, grants, refusals, remaps, injections and lifecycle facts are generation-
stamped into the one `IntegrationSchedule` / canonical replay surface. No facility owns a private clock,
retry log or replay semantics.

---

# 6. StemThing-B in final form

StemThing-B is the recursive conserved-resource market germ:

```text
admitted conserved resource or capacity
        ↓
sealed offering + Draw envelope
        ↓
descendant claim
        ↓
recursive RF reduce-up
        ↓
effective EML clearing weight
        ↓
authored constrained clearing
        ↓
CostBand quantization
        ↓
grant / flow disbursement
        ↓
Gu-Yang throughput / saturation
        ↓
PALMA potential / impedance / opportunity
        ↓
STEAD observation / bands
        ↓
ActionBand / OverlayThing response
        ↓
next generation
```

The important settled distinctions remain:

```text
unit cost            != clearing weight
a Draw               != a grant
unresolved U         != CostBand remainder R
entitlement          != physical placement
detachment           != release
current value        != derivation identity
semantic partition   != compressed physical spans
```

VRAM residency remains a distinct engine-native market because extent placement, disjointness and remap
are kernel physics:

```text
WHO / WHETHER / HOW MUCH = StemThing-B entitlement
WHERE                    = VRAM residency placement
```

The same market grammar is available to domain-authored compute, bandwidth, storage, worker or other
conserved-resource markets without minting another manager subsystem.

---

# 7. CPU privilege and physical-placement audit

The uniform design does **not** require eliminating the CPU. It requires preventing CPU placement from
becoming a competing semantic authority.

## 7.1 Legitimate CPU responsibilities

These are architecturally appropriate unless measurement proves a better lowering:

```text
authoring parse / hydration / admission
semantic labels and stable identity bookkeeping
true structural mutation at generation barriers
VRAM extent placement and remap commit
persistence / replay I/O
read-only presentation and corpus extraction
bit-exact CPU reference oracles
```

Moving them merely for GPU purity would weaken clarity and proofability.

## 7.2 CPU constrained clearing

The canonical constrained-clear executor is still host-shaped:

```text
BTreeMap supply grouping
BTreeSet duplicate detection
claim grouping / cloning
CPU EML score evaluation
comparison sort into score bands
requested-total reduction
fractional-remainder arrays and sort
grant Vec construction
```

This is the most conspicuous remaining numerical CPU privilege. It is nevertheless semantically lawful
because it is the **one** clearing implementation, not a rival path.

Its ledgered status is correct:

```text
owed measurement first
then faithful physical lowering if material
```

A future GPU lowering must preserve exact scope segregation, score ordering, integer apportionment,
real granter generation, tie rotation and replay equivalence.

## 7.3 Clearing-weight projection

The final projection is CPU/kernel metadata, but it no longer scales with descendant count during
refresh. It scales with sparse override/semantic-window structure.

The remaining physical questions are already appropriate for the performance track:

```text
sparse-K construction complexity
broad ancestor refresh with many nested overrides
repeated local deformation and physical span fragmentation
whether an admitted compact index materially improves lookup
```

None changes clearing semantics.

## 7.4 Field locality

STEAD, PALMA and Gu-Yang already share the one FieldSweep meaning. The open question is physical:

```text
static GridOffsets -> tiled / workgroup-local lowering?
LinkGraph          -> sparse gather?
dirty registration -> dispatch elision?
PALMA              -> incremental / active-region repair?
```

The old Gu-Yang gather-vs-tiled number is stale because its archived instrument no longer builds against
the typed modern kernel. Rebuilding the instrument and measuring current head must precede any locality
track decision.

## 7.5 Feeder

The feeder remains older-shaped transport code, but it is subordinated to the unified model:

```text
external / player / AI intent
        ↓
logical identity + role resolution
        ↓
GPU IntentDelta or parked BoundaryRequest
        ↓
ordinary StemThing execution
```

The hot path can fold transforms into GPU intent deltas; structural requests remain boundary-only; the
schedule-owned grant lane is protected from generic writes. Existing CPU-shadow helpers and stale module
comments should remain fenced from becoming production semantic ingress, but they do not presently form a
second decision engine.

## 7.6 Structural boundary

Placement, reparenting, fission/fusion and registry-shape changes remain boundary physics. Performance may
parallelize preparation by independent granters while preserving one deterministic commit and schedule.
No optimization should turn structural authorization into unordered GPU mutation.

---

# 8. Legacy load-bearing code

## 8.1 Application-named admission vocabulary

Phase 13.5 identified ClauseThing/ClauseScript parking identities and other historical guardrails inside
`simthing-spec::designer_admission`.

They are real repository residue, but they are not a runtime performance path or a peer simulation engine.
Their correct disposition is the bounded post-closeout remove/generalize worklist already recorded by the
constitutional census.

## 8.2 Studio and MapGen adapters

Studio presets, hydration builders, session-source provenance and ClauseThing-local MapGen adapters remain
application mediation. Their risk is duplicated authoring truth, not hot-loop simulation authority.

The invariant is:

```text
all generated / imported content
        ↓
one admitted SimThing session shape
```

## 8.3 CPU oracles

CPU EML and field reference implementations are load-bearing proof assets, not legacy authority. They must
remain available for bit-exact CPU/GPU parity and mutation testing even when production work moves resident.

## 8.4 Initial residency exception

`install_initial_tree` retains the narrow initial bulk-install exception against the same admitted root.
It is not an attached-growth door. Later growth must continue through StemThing-B entitlement,
placement and boundary commit.

This is a legitimate, explicitly bounded bootstrap distinction—not a second allocator policy.

---

# 9. Fresh-context risks to carry forward — not closeout blockers

No further unification rung is warranted, but four implementation cautions should travel into future work.

## 9.1 Refresh failure semantics

`ClearingWeightSpanProjection::refresh` is a mutating, fallible operation. There is currently no production
dynamic-deformation caller, so no live session can rely on recoverable retry semantics.

Any future production wiring must choose one lawful contract explicitly:

```text
A. an error is generation/session-fatal and the projection is discarded

or

B. validation / staging is transactional and the old projection remains intact on error
```

A caller must not catch an error and continue from an implicitly partially refreshed projection. This is a
future wiring contract, not evidence of a current runtime defect.

## 9.2 Semantic-window metrics are not physical-write metrics

`semantic_spans_rebuilt` now truthfully counts semantic windows evaluated. It should not be interpreted as:

```text
number of changed values
number of final physical spans
number of GPU writes
```

Performance instrumentation should report these separately.

## 9.3 Dirty-generation fragmentation

Generic span merging preserves lifecycle metadata such as dirty generation. Repeated local
invalidate/restore sequences may therefore retain more physical spans than the value field alone would
require, even while effective values and profile identities remain correct.

This belongs under the existing `DerivedDependencyIndex` dispatch-elision debt. The performance track should
measure history-dependent span growth and decide whether barrier-time normalization or separate dirty
metadata is beneficial. It is a representation cost, not a semantic reopening.

## 9.4 Dynamic deformation production wiring

13.8/13.9 prove the complete mechanism:

```text
operand change
    -> ChangedLocus
    -> frozen dependency index
    -> semantic-window refresh
    -> next-generation claim consumption
```

They deliberately did not mint a new production caller merely to exercise it. A future domain that exposes
live policy/OverlayThing deformation must enter through this exact path; it may not rebuild privately or add
a listener/cache authority.

---

# 10. Performance-debt inventory and suggested track order

The current lease ledger already carries dated owed-measurement rows for the principal production debts:

```text
constrained_clearing.rs
    CPU host-shaped clearing

flow_market.rs
    authored market strings on hot lookup paths

facility_resident_plane.rs
    Current -> Next carry at target cardinality

derived_span_projection.rs
    dispatch-elision benefit and dirty-span behavior

clearing_weight_projection.rs
    sparse-K build / semantic-window complexity
```

The workshop ledger also preserves the FieldSweep/Gu-Yang instruments, with the old tiled comparison
correctly marked instrument-stale.

A disciplined performance track should begin with measurement rather than implementation.

## P0 — current-head instrument restoration and baseline

```text
rebuild or replace stale Gu-Yang locality instrument
measure CPU constrained clearing by scope/claim cardinality
measure market-string lookup survival into hot generations
measure Current->Next carry at target row/channel counts
measure dependency-index dispatch-elision opportunity
measure clearing-weight K/depth/window distributions
```

No optimization decision should precede this baseline.

## P1 — hot identity and layout census

Determine whether author-facing strings remain in per-generation work. If so, intern at admission into compact
immutable indices while preserving persistence/display identity.

## P2 — resident constrained-clearing prototype

Only if P0 shows material cost, implement a faithful GPU/resident lowering behind the existing CPU oracle:

```text
resident scope grouping
parallel EML scoring
segmented deterministic order
exact requested-total reductions
integer base shares
remainder selection
real generation tie rotation
resident grants / U
```

The CPU implementation remains the referee until parity is proven.

## P3 — FieldSweep locality

Use the fresh instrument to determine which topology classes benefit from:

```text
workgroup-tiled static grids
sparse link gathers
shared source profiles
warm-start / active-region repair
```

Do not force LinkGraph into a dense-grid lowering.

## P4 — dependency-driven dispatch elision

Turn precise changed-locus knowledge into skipped work:

```text
unchanged registration -> zero dispatch
local stencil change   -> dirty tiles/spans only
PALMA change           -> affected repair region
Gu-Yang change         -> affected conservative theater
```

## P5 — resident-plane carry

Remeasure full Current->Next copy versus sparse alternatives at actual target cardinality. The historical 4K-row
result remains valid only at its measured scale.

## P6 — subtree and boundary parallelism

After numerical hot paths are measured, evaluate:

```text
independent subtree execution with stamped integration
parallel placement/remap preparation by granter
single ordered structural commit
```

Physical schedulers may accelerate the model; they may not become semantic managers.

## P7 — corpus / CausalBand Atlas pipeline

The future CausalBand Atlas can be a read-only multichannel temporal projection of authoritative field and
replay state for LeWM/JEPA-style training. It must remain downstream of the one simulation truth and should
be engineered after the core hot-loop measurements, not as a second telemetry authority.

---

# 11. CausalBand as a non-normative consequence

CausalBand is the full-SimThing causal-field concept developed after the core design:

```text
STEAD substrate
+ PALMA potential / routing
+ Gu-Yang realizable flow / saturation
+ RF contention and conservation
+ CostBand quantization
+ ActionBand target lifecycle
+ OverlayThing actuation
+ EML coupling
```

Frequently consumed emergent aggregates such as blight can be materialized as derived atlas channels without
becoming independently writable Properties.

```text
authoritative local basis
    employment / income / education / security / credit / housing / ...
        ↓
CausalBand / EML projection
        ↓
materialized derived field channel: blight
        ↓
policy bands / visualization / ML corpus
```

The temporal multichannel atlas frames are the ML corpus; canonical replay supplies action conditioning,
identity, topology and exact transition ground truth. This is a future capability built *on* the completed
unified kernel, not another kernel organ required for closeout.

---

# 12. Completion decision

## 12.1 Semantic runtime design

**COMPLETE.**

There is one recursive object, one expression language, one field family, one constrained-market grammar,
one sink law, one action lifecycle, one actuation facility, one structural boundary, one history and one
generation law.

13.9 closes the final identified discrepancy between semantic derivation structure and compressed effective
representation.

## 12.2 Implementation conformance

**COMPLETE for the unification scope.**

The final tree proves:

- real source-blind clearing-weight dependencies;
- affected-range-only lifecycle;
- correct hidden-boundary re-emergence;
- nested override preservation;
- re-coalescence;
- zero member-row scan;
- no production all-span proof walk;
- safe optional participant lookup;
- unchanged clearing/replay/germ seals;
- a 128/481/0/14 structural certificate.

## 12.3 Physical optimization

**INTENTIONALLY OPEN.**

CPU constrained clearing, FieldSweep locality, Current/Next carry, string interning, dispatch elision,
sparse-K projection cost, subtree scheduling and structural preparation remain measurement-gated physical
lowerings.

They do not reopen the meaning of SimThing.

## 12.4 Repository hygiene

**BOUNDED, NOT ZERO.**

Application-named authoring residue and Studio/MapGen mediation remain, but every relevant surface is censused
and assigned a dated disposition. The runtime kernel is domain-neutral and uniformly governed; the remaining
application-specific authoring residue is explicitly bounded and censused.

## 12.5 Final recommendation

> **Mark the SimThing Unification Model complete. Proceed to 0.0.8.7 closeout on the Owner's explicit Board
> call, then author the performance track from the dated debt ledger without another unification review cycle.**

No additional architecture should be added to StemThing merely because a physical path is slow. The next
track's invariant is:

```text
same bits
same EML meaning
same logical identity
same generation authority
same IntegrationSchedule
same clearing rule
same field law
faster physical execution
```

A future reopening of unification is justified only by a concrete falsifier showing a second authority,
missing causal leg, or semantic non-equivalence—not by an optimization opportunity.

---

# 13. Final reference distinctions

| do not conflate | distinction |
|---|---|
| logical identity / physical row | rows may rebind; identity persists |
| current value / derivation identity | equal outputs may have different future response |
| semantic partition / effective span | derivation truth vs compressed representation |
| observation / sink | crossing observes; CostBand consumes |
| unresolved `U` / CostBand `R` | ungranted demand vs below-quantum value |
| PALMA / route object | potential field vs planner artifact |
| Gu-Yang / CostBand | realizable throughput vs sink price |
| grant entitlement / residency placement | WHO/HOW-MUCH vs WHERE |
| detachment / release | topology change does not terminate a grant |
| OverlayThing / structural mutation | resident actuation vs boundary topology |
| source-blind invalidation / dispatch elision | dependency knowledge vs proven work avoidance |
| CPU oracle / CPU authority | referee is legitimate; peer resolution is not |
| application vocabulary / kernel semantics | compatibility may live above a domain-neutral runtime |
| performance debt / performance fact | optimization requires dated measurement |
| physical lowering / second engine | faster execution must preserve one semantic authority |
| resident derived channel / authoritative Property | materialization does not grant an independent writer |

---

# 14. Final synthesis

> **SimThing is one closed recursive simulation kernel. Its state and observations live on the uniform
> field substrate; EML defines numerical meaning; RF carries recursive conserved participation and
> clearing; STEAD/PALMA/Gu-Yang provide the field-law family; CostBand quantizes sinks; ActionBand resolves
> target discrepancy; OverlayThing actuates change; StemThing-A binds logical identity to physical
> residency; StemThing-B gives every descendant the same recursive conserved-resource market germ; and the
> generation boundary plus IntegrationSchedule is the single temporal authority. Application languages
> lower into this kernel and never pull application semantics back down into it.**

Phase 13 made the implementation live up to that sentence by deleting opaque admission collapse, stale
application shims, tolerated red baselines, uncensused ingress, duplicate inheritance traversal,
synthetic-generation clearing, empty invalidation bindings, panic lookup and finally value-based loss of
semantic partitions.

The unification arc is therefore complete. The next gains should come from proving where this one model is
expensive and lowering it more faithfully onto the hardware—not from inventing another abstraction layer.

# StemThing Intrinsic Overlay Capability
## Engineering-review candidate for closing RF + Field Triad (STEAD / PALMA / Gu-Yang), full EML, CostBand, ActionBand, and Overlay into one recursive SimThing automaton

> **Status: PROMOTED SEMANTIC-CLOSURE ANCHOR (Owner-adjudicated 2026-08-10).**
>
> **OWNER RULING — promotion criteria 1 and 2 APPROVED:**
> **(1) Overlays are intrinsic StemThing actuation state** — with the Owner's clarifying gloss, which is
> the correct reading of the prohibition: the closure forbids PEER executors, never acting SimThings.
> Anything that acts — a user seat, a network controller, any would-be "action executor" — is itself a
> SimThing, which surfaces its auditability as ordinary columns, exposes its API as admitted
> lanes/templates/bands, and binds to the STEAD field for telemetry.
> **(2) The core SimThing stem cell is the sole owner, emitter, and possessor of overlays** — every
> overlay that disburses its values is attached to a SimThing somewhere up the tree.
>
> This document is the promoted anchor for the §22.A semantic closure. The §22.B field-solver
> performance program **remains workshop research** and is explicitly non-blocking. Anchor-table rows
> ride the rung-mint PR per sequencing. Rung minting follows the recorded order: after the 7.5a/7.5b
> ActionBand Field-Triad remediation stabilizes, before 8.2 hardens a second actuation path. Exact
> numbering belongs to the DA.
>
> **AMENDED INTO THE WORKPLAN (v4 unification sweep, 2026-08-13, #1743 @ `34db7956`).** The §23 probe
> sequence is superseded-in-part: ladder rows **7.6–7.9 plus the new 7.8a
> `DERIVED-SPAN-PROJECTION-INVALIDATION-0`** govern. v4 added (engineering-converged, Sol/Fable): the
> three-armed `CrossingConsequenceBinding` ABI (facility-local `ResidentNextWrite` /
> `RoutedOverlayDelivery` / `StructuralAuthorization`), the **Deadline Authority Law** and the
> **Routed Lifecycle Epoch Law**, the source-blind `DerivedDependencyIndex`, effective-profile
> interning with materialization-as-derived-cache, session-frozen facility residency accounting, the
> canonical EML program registry binding, and the write-only telemetry law. Where this document and
> those rows differ, **the rows govern**.
>
> Owner-approved working design, revised 2026-08-10 after engineering review and designer-language stress analysis. This document remains under `docs/workshop/` while the overlay closure is tested, reviewed, and prepared for promotion. It proposes the final semantic closure of the base recursive **StemThing**: the base `SimThing` is the sole semantic owner of overlay origination/emission, retention, reception, routing/filtering, projection, activation/suspension, lifecycle, dissolution, and collapse; ordinary numerical action is expressed by that intrinsic overlay capability rather than by a peer event-execution or behavior subsystem.
>
> **This document does not amend the 0.0.8.7 ladder, does not supersede an anchor, and does not authorize implementation by itself.** Existing anchors and graduated rungs remain authoritative until this workshop is adjudicated and promoted.
>
> The design explicitly incorporates the **full RF / Field-Triad authority** — STEAD, PALMA, and Gu-Yang / `SaturatingFlux` — plus the **full admitted EML direction**, including deterministic `EXP` / `LN` and their gadget-space consequences. It also anticipates the chosen designer-facing **ClauseScript** language: rich authored modifier/effect vocabulary must compile into this finite native capability set without creating a ClauseScript runtime, modifier manager, or semantic dispatcher.

Primary companions:

- [`../stead_stemthing_unification.md`](../stead_stemthing_unification.md)
- [`../multi-axis-ActionBand-STEAD.md`](../multi-axis-ActionBand-STEAD.md)
- [`../simthing_core_design.md`](../simthing_core_design.md)
- [`../stead_spatial_contract.md`](../stead_spatial_contract.md)
- [`../full_eml_unification.md`](../full_eml_unification.md)
- [`../EML_exp_ln_unification_expansion.md`](../EML_exp_ln_unification_expansion.md)
- [`../design_0_0_8_7_rf_arena_modernization.md`](../design_0_0_8_7_rf_arena_modernization.md)

Implementation archaeology:

- [`../../crates/simthing-core/src/simthing.rs`](../../crates/simthing-core/src/simthing.rs)
- [`../../crates/simthing-core/src/overlay.rs`](../../crates/simthing-core/src/overlay.rs)
- [`../../crates/simthing-core/src/automaton.rs`](../../crates/simthing-core/src/automaton.rs)
- [`../../crates/simthing-core/src/property.rs`](../../crates/simthing-core/src/property.rs)
- [`../../crates/simthing-kernel/src/overlay_prep.rs`](../../crates/simthing-kernel/src/overlay_prep.rs)
- [`../../crates/simthing-sim/src/overlay_lifecycle.rs`](../../crates/simthing-sim/src/overlay_lifecycle.rs)
- [`../../crates/simthing-sim/src/tree_mutation.rs`](../../crates/simthing-sim/src/tree_mutation.rs)
- [`../../crates/simthing-sim/src/gpu_sync.rs`](../../crates/simthing-sim/src/gpu_sync.rs)
- [`../../crates/simthing-driver/src/automaton_reception.rs`](../../crates/simthing-driver/src/automaton_reception.rs)
- [`../../crates/simthing-feeder/src/work.rs`](../../crates/simthing-feeder/src/work.rs)
- [`../../crates/simthing-feeder/src/patcher.rs`](../../crates/simthing-feeder/src/patcher.rs)
- [`../../crates/simthing-spec/src/compile/overlay.rs`](../../crates/simthing-spec/src/compile/overlay.rs)

Designer-language stress corpus/reference:

- [`../../scenarios/terran_pirate_galaxy.clause`](../../scenarios/terran_pirate_galaxy.clause)
- [`../clausething/ClauseThing.md`](../clausething/ClauseThing.md) — foreign-language reference only; not SimThing runtime doctrine

---

## 0. Executive thesis

The candidate closure is:

> **StemThing is the simulation cell. Ordinary action resolves by the StemThing originating, retaining, projecting, parameterizing, suspending, or dissolving admitted overlays. ActionBand and CostBand do not become peer executors; they authorize changes in intrinsic overlay state. Those overlays parameterize ordinary Properties, RF, and the complete Field Triad — STEAD, PALMA, and Gu-Yang — until authored lifecycle or target conditions resolve.**

This does **not** mean every quantity becomes RF-conserved, and it does **not** mean an overlay is a second world-state object beside properties.

```text
Property / governed value
    = ordinary state

RF
    = conserved / constrained claim → clear → disburse

STEAD
    = non-conserved propagated signal / influence

PALMA
    = min-plus potential / impedance / value field

Gu-Yang / SaturatingFlux
    = conserved realizable flux / saturation / stall over admitted topology

ActionBand
    = unresolved target discrepancy and lifecycle

CostBand
    = exact sink/work quantization

Overlay
    = intrinsic programmable actuation state

BoundaryRequest
    = sealed structural consequence when numerical state requires tree mutation
```

### 0.1 Complete recursive loop

```text
                              STEMTHING
                                  │
                             participate
                                  │
                  ordinary Property / RF state
                                  │
                         active Overlay inputs
                                  │
                    ┌─────────────┼─────────────┐
                    │             │             │
                  STEAD         PALMA        GU-YANG
             non-conserved     potential     conserved flux
                    └─────────────┼─────────────┘
                                  │
                       anchored field outputs
                                  │
                                  act
                                  │
                     ActionBand / CostBand
                                  │
                    existing sealed crossings
                                  │
                           full admitted EML
                                  │
                              originate
                                  │
                        OverlayStateNext
                                  │
                        generation boundary
                                  │
                              receive
                                  │
                  route / inherit / filter / apply
                                  │
              Property / RF / Field-Triad deformation
                                  │
                                  └────────────► next generation
```

No CPU behavior tree, action dispatcher, movement executor, combat executor, modifier evaluator, event engine, or domain action taxonomy is required by this model.

### 0.2 Closure by authority concentration

The repository already contains the pieces:

- `SimThing` already owns overlays;
- standing overlays already recurse by inheritance;
- routed overlays already derive origin → LCA → target routes;
- RF disbursement already terminates delivered directives in the receiving `SimThing` overlay store;
- overlays already lower into numerical columns ordinary GPU execution consumes;
- CostBand already defines exact sink/work quantization;
- ActionBand is intrinsic `act` over the existing Phase-5 crossing surface;
- generation pacing already forbids same-generation recursive convergence.

The architectural act is therefore **concentration**: remove remaining peer overlay authority from feeder/driver/sim lifecycle paths and make StemThing the one semantic home.

### 0.3 Designer-language closure criterion

ClauseScript remains the default designer-facing script language. Its authoring vocabulary may be extremely rich — generated modifier names, script values, conditional effects, scope-relative references, timed modifiers, iterators, and mutually dependent modifier networks — but none of those authored nouns should survive as runtime semantic authority.

The runtime closure test is:

```text
rich ClauseScript authoring
        ↓ admission / compilation
bounded templates + EML + bindings + RF/CostBand/ActionBand state
        ↓
ordinary StemThing execution
```

If a new authored construct requires a new domain runtime executor instead of lowering through admitted native capability, the closure has failed or the native capability boundary has genuinely been shown incomplete.

---

## 1. Review-status vocabulary

| Tag | Meaning |
|---|---|
| **INHERITED LAW** | Already governed by an existing anchor or graduated mechanism. |
| **OWNER-DIRECTED** | Owner-approved design direction pending promotion. |
| **WORKSHOP CANDIDATE** | Proposed semantic/engineering law under review. |
| **RESEARCH CANDIDATE** | Performance hypothesis; not required for semantic closure. |
| **REJECTED** | Deliberately excluded. |

---

## 2. StemThing anatomy: four legs remain complete

**INHERITED LAW:**

```text
SimThing
 ├─ participate
 ├─ act
 ├─ originate
 └─ receive
```

Overlay capability is not a fifth leg.

| Leg | Closed-loop meaning |
|---|---|
| **participate** | expose Property/RF/Field-Triad state to recursive evaluation |
| **act** | ActionBand resolves discrepancy; CostBand quantizes executable work |
| **originate** | activate/update/dissolve admitted overlays |
| **receive** | receive routed/inherited overlays and expose effective transforms locally |

### 2.1 Default inertness

Universal semantic capability does not imply universal hot state.

```text
no active overlay
    → no active overlay instance
    → no lifecycle work
    → no inherited-value recomputation
    → no field dirtiness
    → effectively zero overlay hot-loop cost
```

The germ is universal; active numerical state is sparse and pay-for-play.

---

## 3. Overlay is the intrinsic actuation language

**OWNER-DIRECTED / WORKSHOP CANDIDATE:**

> **An ordinary numerical action is the activation, parameterization, modification, suspension, or dissolution of an admitted overlay whose consequences are realized through ordinary SimThing substrate.**

Preferred slogan:

> **Overlays are the intrinsic actuation language/state of SimThing.**

### 3.1 ActionBand and CostBand stop at actuation authorization

```text
ActionBand = why / toward what / when
CostBand   = how much executable work is affordable
Overlay    = actuation state
RF         = constrained means
Field Triad= propagated consequence / accessibility / realizable transport
Property   = resulting durable state
```

A normal ActionBand/CostBand consequence should prefer:

```text
activate/update OverlayTemplate K with numeric parameters P
```

over bespoke domain mutation.

Structural consequences remain behind the existing sealed boundary.

### 3.2 Not every state is RF

Everything participates in the recursive SimThing architecture; only genuinely conserved/constrained quantities use RF conservation semantics.

### 3.3 Overlay is not every numerical intermediate

The closure must not over-expand Overlay until it becomes meaningless.

```text
EML scratch/result          = not automatically Overlay
RF claim quantity           = RF state, not automatically Overlay
CostBand quotient/remainder = CostBand state
PALMA D                     = field state
Gu-Yang flux                = field/RF state

persistent/external actuation
policy pressure
transient bonus/penalty
movement actuation
standing directive effect
lifecycle-bearing modifier
                             = Overlay candidates
```

A useful rule is:

> **Persistent or externally effective actuation should have an Overlay representation; pure internal numerical intermediates remain in their native mechanism.**

---

## 4. Current overlay archaeology

### 4.1 Semantic home already lives on `SimThing`

`SimThing` already stores overlays directly and exposes the base attach operation. This is the correct semantic ownership shape.

### 4.2 Routed attachment already terminates in the ordinary overlay store

`deliver_routed_overlay()` validates origin/target, derives the authority path, applies routing policy, and terminates at `target.add_overlay(...)`. Delivery is not a second inbox.

### 4.3 Standing inheritance already recurses fractally

Standing overlays are inherited down the tree rather than copied as independent semantic owners.

```text
ancestor overlay state
        ↓
child effective state + child-local overlays
        ↓
grandchild effective state + grandchild-local overlays
```

This is the basis for the stronger scale law in §15.

### 4.4 RF → Overlay already exists

An allocated command/directive already reaches a receiving SimThing as an overlay through ordinary RF disbursement/reception.

### 4.5 Overlay → RF already exists through numerical columns

An overlay changes the Property/rate/weight/pressure inputs that ordinary RF execution already reads. Overlay does not call a separate RF API.

### 4.6 Transform operations are already EML-shaped

`TransformOp` is already an admitted EML program representation; `Set/Add/Multiply` are degenerate forms, not a second transform language.

---

## 5. Current authority fragmentation

### 5.1 Feeder ingress remains duplicated by provenance

Player/AI/script/ActionBand/CostBand may differ in provenance and authorization, but should converge on one overlay activation semantics.

### 5.2 Overlay lifecycle remains CPU boundary authority today

Current lifecycle walks the tree, reads GPU-fresh values back through CPU shadow, evaluates dissolution/timers, removes overlays, and applies expiry effects.

This is the largest remaining peer numerical authority and should be treated as the engineering center of gravity of the semantic closure.

### 5.3 Activation/suspension remain structural-boundary mutations today

Promotion must distinguish true structural storage changes from ordinary GPU-resident lifecycle-state transitions.

### 5.4 Explicit generic dissolution/removal requires total proof

The promoted germ must have one lawful home for attach, receive, activate, suspend, dissolve, explicit remove/override, and collapse.

### 5.5 `OverrideReceived` remains an archaeology obligation

Do not assume the promised override path is complete until its actual attachment/lifecycle behavior is proven.

---

## 6. Candidate intrinsic overlay germ

```text
OVERLAY GERM

originate
    activate/parameterize admitted overlay state

receive
    accept attributable routed/inherited overlay state

project
    expose effective overlay parameters into Property/RF/Field-Triad lanes

lifecycle
    active ↔ suspended → dissolved

collapse
    retire resolved transient actuation state
```

### 6.1 Ownership law

> **Nothing outside the SimThing owns an overlay's active lifetime. External systems may author templates, supply directives, or perform genuine structural storage work, but originate/receive/project/activate/suspend/dissolve/collapse belong to the intrinsic StemThing capability.**

### 6.2 Collapse law

Resolved overlay state disappears unless an authored durable property/history explicitly records the consequence. World state is memory by default.

### 6.3 Generation pacing

```text
generation t:
    crossing authorizes OverlayStateNext

barrier

generation t+1:
    overlay participates normally
```

No emit→apply→re-cross→re-emit same-generation loop.

### 6.4 Pure-current versus next-state distinction

Within a generation, ordinary overlay transforms may read admitted **Current** values and compute a pure EML result. But activation, dissolution, parameter mutation, newly emitted actuation, and structural consequences write their lawful next-state/boundary destinations rather than recursively re-entering the current evaluation.

This distinction is load-bearing for designer-authored feedback:

```text
pure current transform     = DAG-shaped numerical evaluation
state-changing consequence = Current → Next edge
```

Cyclic authored behavior is therefore represented as time evolution, not evaluator recursion.

---

## 7. GPU physical target

Semantic ownership by every StemThing does not imply a rich object embedded in every row.

```text
OverlayTemplate[]
    admitted EML / lifecycle / projection / parameter shape

OverlayInstance[]
    owner slot
    origin slot
    template id
    parameter span
    lifecycle state
    generation stamp where required

OverlayStateCurrent[]
OverlayStateNext[]

OverlayBinding[]
    instance → target slot/property/role / field-parameter binding

OverlayFieldDependency[]
    overlay binding → affected field registration / locus / invalidation class
```

### 7.1 CPU semantic shadow

GPU sees ids, slots, bindings, state bits, parameters, and programs. CPU keeps names, source spans, presentation, persistence mapping, and sparse semantic/structural deltas. Human-readable kinds must not become numerical dispatch keys.

### 7.2 Sparse program/profile batching

Bucket by template/program/binding shape. Trivial ordered affine chains may fold to an equivalent `(a,b)` representation where exact authored semantics are preserved. General EML remains shared admitted program execution.

### 7.3 Persistent templates, parameterized instances

Designer-facing languages frequently express “apply modifier/effect X with duration/intensity/target Y.” The physical model should prefer one admitted immutable template plus sparse parameterized instances over runtime template minting.

```text
OverlayTemplate
    immutable admitted program/binding/lifecycle shape

OverlayInstance
    target/origin
    numeric params
    active state
    expiry/lifecycle state
```

This is also the natural ActionBand/CostBand emission target.

### 7.4 Semantic dissolve need not imply immediate allocation churn

An instance that becomes semantically dissolved may retire/recycle through the existing bounded residency/free-list discipline. High-frequency activate/suspend/dissolve semantics must not require heap-style allocation/deallocation on every authored state change.

---

## 8. Overlay × RF / Field Triad / ActionBand / CostBand

### 8.1 RF — constrained means

Overlay may modify demand pressure, production/consumption rates, allocation weights, priorities, and other admitted numerical inputs. It does not bypass claim→clear→disburse.

### 8.2 STEAD — non-conserved propagated influence

Overlay may parameterize emitter strength, attenuation, falloff, or other already-admitted STEAD inputs. It should feed the authoritative STEAD program before propagation rather than apply a private post-hoc correction when the semantic factor truly belongs in the field.

### 8.3 PALMA — route/value potential

Overlay may alter admitted edge/terminal valuation or other lawful `W` inputs. Route-relevant semantics should enter PALMA before solve where possible rather than create a local ActionBand correction inconsistent with the solved field.

### 8.4 Gu-Yang — conserved saturating flux

Overlay may parameterize demand, source/sink intensity, available capacity, conductance-like coefficients, participation, or other already-admitted Gu-Yang/RF inputs **within the registration's certified parameter envelope**.

Overlay does **not** compute private flux, saturation, congestion, or claimant arbitration.

```text
Overlay parameter change
        ↓
existing Gu-Yang/RF inputs
        ↓
existing conservative field execution
        ↓
realized flux / stall / contest / choke outputs
```

### 8.5 Gu-Yang remains instantaneous

> **Gu-Yang owns the instantaneous conservative fact. Persistence of stall/saturation belongs to ordinary authored state/overlay/property memory, not hidden Gu-Yang history or a saturation listener.**

### 8.6 Field-parameter certificate envelope

> **Overlay may vary admitted Field-Triad numerical parameters only within the structural/certified envelope under which the field program was admitted. It may not mutate canonical adjacency, ordering proof, conservation certificate, stability/χ certificate, or field-law identity.**

### 8.7 ActionBand

ActionBand observes current state, target discrepancy, and native Field-Triad outputs, then authorizes an overlay update through the existing crossing/EML/emission surfaces.

### 8.8 CostBand

CostBand remains exact sink/work quantization. Overlay expresses the resulting actuation intensity; it does not redefine sink arithmetic.

### 8.9 Designer-authored effect destinations remain finite

The chosen authoring language may expose hundreds of named effects, but runtime effect destinations should remain a small admitted set such as:

```text
activate/update/suspend/dissolve Overlay
write admitted PropertyNext
create/update ordinary RF claim/obligation
configure/update ActionBand state
configure/update CostBand state
emit sealed BoundaryRequest
telemetry/history through existing stamped surfaces
```

This is the effect-side analogue of compiling large generated modifier vocabularies into generic numerical semantics.

---

## 9. Full EML actuation space

Overlay semantics should exploit the full admitted EML language, not only affine transforms.

### 9.1 Bands versus continuous shaping

> **Bands decide semantic crossings/events; EML shapes continuous overlay intensity between crossings.**

### 9.2 ContinuousDecay

\[
x_{t+1}=x_t\exp(-\lambda\Delta t)
\]

Useful for bounded authored memory such as urgency, pressure, fear, mobilization, congestion memory, exploration pressure, and transient policy intensity.

### 9.3 Logistic

A stabilized logistic provides smooth bounded response instead of staircase state machines.

Use cases: scarcity response, mobilization, rationing, risk aversion, recruitment, willingness-to-pay, congestion aversion.

### 9.4 PowerLaw

\[
x^a=\exp(a\ln x)
\]

Useful for authored falloff, nonlinear scaling, substitution curves, economies/diseconomies, and positive scale transforms.

### 9.5 Softmax weighting

For candidate channels/options:

\[
p_i=\frac{\exp(\beta(z_i-z_{max}))}{\sum_j\exp(\beta(z_j-z_{max}))}
\]

Possible use: RF claim weights derived from STEAD attractiveness, PALMA impedance, Gu-Yang stall, opportunity value, policy, and commitment.

No `SelectionMode` enum is required.

### 9.6 Entropy

\[
H=-\sum_i p_i\ln p_i
\]

A generic concentration/diversity observable that vendors may feed back into overlays without the core assigning domain meaning.

### 9.7 Log-domain composition is authored semantics, never a silent optimization

`EXP(SUM(LN(x_i)))` may be an admitted positive/log-domain law where that is the intended semantics. It must never silently replace ordered Product semantics because f32 arithmetic results differ.

A promising Gu-Yang parameter pattern is positive conductance/scale parameterization in log space, clamped/certified to the admitted envelope.

### 9.8 Canonical bounded Gu-Yang feedback pattern

```text
Gu-Yang instantaneous stall s_t
        ↓
ordinary persistent overlay/property memory m_t
        ↓
m_next = m * EXP(-lambda*dt) + alpha*s
        ↓
Logistic bounded response
        ↓
overlay parameter
        ↓
PALMA W / RF demand / STEAD source / Gu-Yang input changes next generation
```

This preserves authority:

```text
Gu-Yang = instantaneous physical fact
EML     = temporal/nonlinear law
Overlay = actuation
PALMA/RF/STEAD = native consequence
```

### 9.9 Gated contributions should normally remain resident

A designer-authored triggered modifier should usually compile to a resident admitted template/instance whose numerical contribution is gated by an EML predicate, not repeated attach/remove churn every time the predicate flips.

```text
active contribution = predicate(Current) ? transform(Current) : identity
```

Lifecycle attach/dissolve remains available when the authored semantics genuinely require lifetime identity; simple conditionally active contribution should prefer numeric gating.

### 9.10 Bounded control flow lowers to dataflow

Designer-facing `if/else`, `switch`, bounded iterator predicates, and similar control flow should lower to admitted `SELECT`/gating/dataflow where exact semantics permit. Runtime need not carry a script program counter or general effect interpreter.

Bounded loop-like source constructs may be statically unrolled/gated only under an explicit admission cap. Unbounded runtime loops are not part of the intrinsic overlay ABI.

---

## 10. Overlay-to-Field execution integration

### 10.1 Overlay-to-Field ingress fusion — research-capable physical lowering

When an intermediate overlay-deformed value is not independently required as an authoritative observable, an implementation may fuse overlay EML with the already-admitted field ingress/map program rather than materialize a write/read pair.

This is physical fusion only. Exact EML semantics, routing/inheritance, certificates, and crossing identity must remain unchanged.

### 10.2 OverlayFieldDependency

Admission already knows which properties/field parameters an overlay can alter. Reuse that metadata as the invalidation source.

```text
OverlayState delta
        ↓
OverlayFieldDependency
        ↓
mark affected STEAD / PALMA / Gu-Yang work dirty
```

No global change-detection scan is required.

### 10.3 Invalidation classes

A physical scheduler may distinguish classes such as local pointwise, local stencil, propagating potential, and conservative-flux work. These are scheduler descriptors, not domain semantics.

### 10.4 Field-output-driven lifecycle

Lifecycle predicates may bind ordinary anchored outputs such as RF balance, STEAD pressure, PALMA distance, Gu-Yang realized flux/stall, ActionBand progress, or ordinary Properties through the singular Phase-5 crossing surface.

No separate lifecycle comparator or saturation listener.

---

## 11. Movement as a full-Triad worked witness

Movement remains a vendorized witness, not core semantics.

```text
Location Property
      ↓
ActionBand destination target
      ↓
STEAD + overlays compose local conditions
      ↓
PALMA gives lawful local potential descent
      ↓
Gu-Yang/RF gives signed realizable channel throughput where capacity-bearing
      ↓
CostBand quantizes movement/fuel sink
      ↓
ActionBand crossing
      ↓
MovementActuation overlay / sealed local structural consequence
      ↓
Location value/velocity changes
```

The key distinction is:

```text
PALMA   = desirable/lawful route
Gu-Yang = physically realizable transport now
CostBand= paid work
Overlay = actuation
```

---

## 12. Opportunity horizon, Gu-Yang, and apparent local traps

### 12.1 Opportunity-Horizon Principle

> **ActionBand remains a local field consumer. Nonlocal opportunity awareness is a field-propagation responsibility, not actor-side search depth.**

### 12.2 PALMA coherence

For Bellman-consistent positive-cost PALMA, a reachable nonterminal state should have a lawful descending neighbor once the field is converged. False traps often indicate stale/incomplete field information or omitted dynamic impedance rather than a need for a planner.

### 12.3 Gu-Yang as target-aligned local feasibility

Signed realized flux can provide something stronger than PALMA alone for conserved actions:

```text
PALMA gradient = attractive/lawful direction
Gu-Yang flux   = transport actually occurring in a signed direction
```

Where the same conserved transport is being solved, target-aligned signed Gu-Yang flux may be consumed as near-zero-marginal-cost feasibility/progress information.

### 12.4 Gu-Yang stall as missing dynamic impedance

If PALMA prefers a corridor but Gu-Yang reports saturation/stall, that is often not a true PALMA local minimum. It is evidence that dynamic capacity state is absent from current valuation.

```text
Gu-Yang stall
    ↓
EML bounded overlay response
    ↓
PALMA W changes locally
    ↓
PALMA repairs globally coherent value field
```

### 12.5 Opposed-flow opportunity information

Large gross flux with near-zero signed net progress is a useful generic contest/stall signal. Target-aligned net flux and stall magnitude may participate in EML valuation without redefining Gu-Yang.

### 12.6 Gu-Yang driving/dual-like potential archaeology probe

**RESEARCH QUESTION:** determine whether the landed Gu-Yang formulation already computes, consumes, or can expose a scalar node-level driving/pressure/dual-like potential whose gradient determines signed flux.

If such a quantity exists and is already paid for, test whether it can:

- seed or warm-start PALMA;
- act as a lower-bound/heuristic field;
- expose nonlocal capacity/opportunity structure;
- reduce duplicate route computation.

No such potential is assumed by this workshop. If archaeology says no, signed flux/stall reuse remains valuable independently.

### 12.7 Gu-Yang can expand information horizon only according to its own semantics

A local flux field may encode distant supply/demand consequences after they propagate through the conservative system. Whether multiscale acceleration is lawful depends on whether the registration is a within-generation fixed-point solve or finite-rate simulated transport. See §14.6.

### 12.8 Trap diagnostic hierarchy

```text
Actor appears trapped
       ↓
1. Is PALMA converged?
       no → repair solver / dirty propagation problem
       yes
       ↓
2. Is relevant conserved route physically saturated?
       yes → use native Gu-Yang flux/stall; feed bounded response back into valuation
       no
       ↓
3. Is remote opportunity outside current information horizon?
       yes → improve field propagation / hierarchy
       no
       ↓
4. Does authored coupled valuation create a true nonconvex/adversarial case?
       yes → later fenced navigation research
```

This diagnostic intentionally shrinks the future local-minimum problem before granting ActionBand any search semantics.

---

## 13. Opportunity-valued PALMA and potential reparameterization

A multi-source opportunity field may solve:

\[
J_a(x)=\min_g\left[B_a(g)+Dist_{W_a}(x,g)\right]
\]

with authored terminal opportunity values. Different terminal values intentionally change the objective.

### 13.1 Ng–Harada–Russell: retained correctly

Potential-based reward shaping is useful as an invariance warning and mathematical analogy. It does **not** prove that opportunity-valued terminal conditions preserve the old objective.

### 13.2 Exact PALMA gauge/reweighting candidate

For a potential `h`:

\[
W_h(u,v)=W(u,v)+h(v)-h(u)
\]

with matching terminal adjustment. Along a fixed-start path the potential terms telescope. Under the required admissibility/consistency conditions, this may preserve minimizers and act as a preconditioner.

Candidate use: `h ≈ J[t-1]` to flatten the residual problem before incremental PALMA repair.

### 13.3 Not a Gu-Yang optimization by default

A path-sum telescoping argument does not prove invariance of a local conserved flux field. Do not potential-shape Gu-Yang merely because the transform is valid for PALMA. Changing a Gu-Yang driving potential is semantics unless conservation-equivalence is separately proven.

---

## 14. Field-solver performance research program — separate promotion unit

> **This section is intentionally NOT a dependency of the semantic overlay closure.** It remains workshop/performance research even if the semantic closure promotes to doctrine.

### 14.1 FIM for PALMA

FIM is retained as a strong candidate physical lowering for incremental Eikonal/Bellman-like PALMA repair. It does not alter PALMA semantics.

### 14.2 Overlay dirty provenance as FIM seed

Overlay bindings know exactly which PALMA inputs changed. Use that as the active-set seed rather than scanning the full field.

### 14.3 Sparse active tiles, dense local math

Prefer deterministic current/next tile masks and regular workgroup-local stencils over semantic dependence on dynamic queue order.

### 14.4 Dense↔FIM crossover

Cold starts and high dirty fractions may favor dense sweeps. The solver posture is a hardware/profile optimization only.

### 14.5 Multiscale FIM

Use coarse-to-fine relaxation to accelerate propagation of the **same authoritative PALMA problem**. The final fine field remains authority; coarse approximations may not erase hard constraints or become gameplay truth.

### 14.6 Gu-Yang multilevel semantic test

FIM itself is not Gu-Yang. Dirty-region scheduling transfers readily; multiscale acceleration does not automatically.

> **If a Gu-Yang registration represents a within-generation equilibrium/fixed-point solve, multilevel acceleration may be lawful if it converges to the same fine conservative result. If each Gu-Yang sweep is itself finite-rate physical evolution, coarse acceleration would change information speed/physics and is forbidden.**

### 14.7 Dirty conservative theaters

Even where multiscale Gu-Yang is forbidden, overlay provenance can still avoid unchanged work by directly seeding dirty conservative loci/theaters while preserving native propagation rate.

### 14.8 Shared PALMA profiles

Do not solve one full field per actor when actors share topology/impedance/opportunity profiles. Profile cardinality should be admission-bounded and measured.

### 14.9 EXP/LN profile compression candidate

Actor heterogeneity may sometimes be expressed through a small set of admitted profile fields plus local EML weighting rather than one field per actor. This is a performance candidate only; exact route semantics may require distinct profiles where minimizers change.

### 14.10 Research performance staircase

```text
0. resident exact field value → direct bind
1. pointwise full EML shaping
2. overlay-driven dirty Field-Triad work
3. tiled PALMA FIM
4. multiscale PALMA FIM
5. Gu-Yang multilevel solve ONLY if fixed-point semantics permit
```

---

## 15. Ancestor residency, inheritance, and million-child scale

This is load-bearing for the target scale.

### 15.1 Ancestor Overlay Residency Law

> **An overlay whose semantic scope is a subtree is retained at the lawful ancestor that defines that scope, not stamped onto descendants. Parent-level policy/state overlays are naturally sparse: `OwnerThing`, `GameSession`, world/map parents, star/system parents, and other structural ancestors hold the authoritative instance; recursive inheritance/filtering projects its effective value downward.**

Examples:

```text
faction doctrine across 100,000 ships
    = one authoritative overlay at the faction/owner ancestor
    ≠ 100,000 leaf overlays

world-state modifier
    = one authoritative overlay at world/game-state ancestor
    ≠ population stamping
```

Correct filtering requires the overlay to remain at the ancestor whose subtree defines its scope.

### 15.2 Descendant materialization is not semantic ownership

A physical lowering may cache/materialize effective descendant parameters if that is equivalent and cheaper. Such caches are projections only; they do not become independent overlay instances, histories, lifecycles, or origins.

### 15.3 Inheritance hot-path law

The semantic inheritance model does **not** license a per-generation per-leaf ancestor walk.

Preferred physical shape:

```text
ancestor overlay set changes at boundary/epoch
        ↓
subtree-contiguous ranges / equivalent structural span
        ↓
compile or invalidate effective parameter plane/span
        ↓
hot generation reads O(1)-ish effective parameters
```

The exact representation is not law. The required property is that unchanged standing overlays do not cause `O(depth × descendants)` rewalks each generation.

### 15.4 Dirty inheritance propagation

Ancestor overlay changes should invalidate only the affected subtree/range and dependent parameter/field spans. This is the recursive-tree analogue of `OverlayFieldDependency`.

### 15.5 Broadcast-down authoring maps naturally to ancestor residency

ClauseScript's modifier language permits higher-level application of lower-granularity effects that broadcast downward. This does not justify leaf stamping. It reinforces the ancestor law: attach once at the semantic scope owner, then project/filter through the recursive tree.

A single authored doctrine/policy/world condition across a large subtree should remain one authoritative semantic overlay unless descendants genuinely diverge in lifecycle or parameters.

---

## 16. Designer-language closure: complex modifiers, scopes, and feedback

ClauseScript is the chosen designer-facing language. The runtime must be broad enough to host its most difficult semantics while remaining ClauseScript-blind after admission.

This section is therefore a **capability stress contract**, not a foreign-engine implementation plan.

### 16.1 Generated names compile away

Long modifier strings and generated modifier families are admission vocabulary, not runtime vocabulary.

```text
authored modifier/effect key
        ↓
grammar / registered vocabulary / source-context decode
        ↓
property/role + composition class + EML/template binding
        ↓
runtime numeric ids only
```

The intrinsic Overlay design must never require a runtime dictionary dispatch on arbitrary modifier strings.

### 16.2 Scope-relative targeting compiles to bounded bindings

Designer scripts commonly express effects relative to implicit scope, owner/root/source relations, ancestor/descendant context, or selected child sets. The runtime need not retain a foreign scope machine.

Admission should lower these forms into bounded recipes such as:

```text
Self
Ancestor-resident inheritance
Descendant subtree/span
Predicate-filtered admitted receiver span
Explicit admitted receiver set
RF-routed receiver
Structural/local-neighborhood binding
Cross-tree stamped receive product
```

Human-readable scope expressions compile away before numerical execution.

### 16.3 One logical action may project to many receivers

Iterator-shaped authoring must not imply a CPU loop attaching one overlay per receiver.

Prefer:

```text
one authoritative overlay instance
        + admitted projection/filter span
```

where receivers share semantics.

Split into independent instances only when descendants truly require independent parameters, provenance, lifecycle, or state.

This is the designer-language counterpart of the ancestor-residency scale law.

### 16.4 Active-state observability

The authoring language can test whether a modifier/effect state is active. Runtime predicates therefore need lawful numeric observability of admitted overlay state.

That must resolve through admitted template/instance ids or an equivalent compiled state lane, not CPU string lookup.

Conceptually:

```text
OverlayActive(template/instance binding) → numeric predicate input
```

### 16.5 Static variable/state vocabulary

Named authored variables should lower to admitted Property/sub-field state where they participate in simulation.

Dynamic construction of runtime variable/modifier names is incompatible with bounded admission unless the possible name family is statically enumerable and budgeted. Unbounded dynamic-name creation should fail closed.

### 16.6 Ordered effects and composition

Designer effect order may be semantically significant. This does not require an imperative runtime interpreter, but it does require admission to preserve ordering where later effects observe earlier authored effects.

Within one pure transform expression, authored operation order is preserved under the exact EML arithmetic law.

Where an authored statement changes persistent state, its visibility follows the admitted state/boundary ordering contract; the transpiler must not silently pretend batch-next semantics are equivalent to a source-language sequential dependency when they are not. Unsupported source semantics fail closed or require an explicit admitted staged lowering.

### 16.7 Gated/triggered modifiers

Conditionally active modifiers should normally become resident templates with pure numeric gating rather than attach/detach churn:

\[
y = p(x) ? F(x) : x
\]

or the appropriate identity contribution for the declared composition class.

This is especially important for large authored modifier networks because it keeps template identity stable and allows profile batching.

### 16.8 Timed modifiers and explicit removal

Authored duration/remove semantics map naturally to intrinsic overlay lifecycle:

```text
activation generation / timer state
lifecycle predicate
suspend/dissolve state
explicit remove/override binding
```

They must not require CPU timer decrements or a special script-side modifier store after GPU lifecycle authority is promoted.

### 16.9 Cyclic authored modifier graphs are temporal feedback

The most dangerous designer-language case is not a long modifier name but a dependency cycle:

```text
A modifies B
B changes a predicate on C
C modifies A
```

or an apparent self-reference.

StemThing must support such behavior as an ordinary discrete dynamical system:

\[
S_{t+1}=F(S_t)
\]

not as same-generation recursive evaluation:

\[
S_t=F(S_t).
\]

Canonical pacing:

```text
generation t
    Current properties / RF / fields / overlay state
        ↓
    pure EML transforms and native field/RF execution
        ↓
    crossings / lifecycle / action resolution
        ↓
    OverlayStateNext / PropertyNext / sealed boundary requests

---------------- generation barrier ----------------

generation t+1
```

The loop is the simulation. There is no modifier recalc cascade.

### 16.10 Within-generation dependency DAG

Admission should prove or construct an acyclic dependency graph for pure current-generation numerical evaluation.

A cycle is lawful only when at least one edge is a declared state transition across the generation boundary (or another already-admitted staged boundary).

This gives a precise rejection criterion for pathological authored self-reference:

```text
pure Current → Current algebraic cycle
    = reject/remand

Current → Next → next generation Current cycle
    = ordinary stateful feedback
```

### 16.11 Bounded feedback admission

Generation pacing bounds feedback **rate**, not feedback **gain**. Authored loops can still diverge or chatter.

Any emitted overlay that feeds a field/RF lane capable of driving its own future activation/intensity must obey the existing bounded-feedback posture: explicit clamps/finite ranges, admitted decay or other stabilizing law where required, and no unbounded positive recurrence silently admitted.

Hysteresis may be authored with separate activation/deactivation thresholds, decay, minimum lifetime, cooldown, or other ordinary state/band laws. No hidden scheduler cadence should be introduced to simulate stability.

### 16.12 Effect ABI remains closed and extensible by data

The authoring language may evolve indefinitely, but its runtime consequences should compile to finite native destinations:

```text
Overlay state
Property next-state
RF claim/obligation
ActionBand state
CostBand state
sealed BoundaryRequest
existing telemetry/history
```

New source vocabulary should normally require compiler/admission data or standard-library templates, not a new runtime semantic subsystem.

### 16.13 Staged progress constructs require no new engine

Long-running authored situations, projects, negotiations, explorations, and similar staged constructs should generally lower as:

```text
ordinary progress Property
+ prerequisite predicates
+ CostBand/RF work
+ ActionBand unresolved target where purposeful trajectory exists
+ threshold bands
+ Overlay approach/state actuation
+ terminal structural consequence only when truly structural
```

This is a capability claim, not a promise that every foreign construct is already mapped. A construct that cannot be expressed faithfully through these primitives must be identified explicitly rather than approximated by a hidden domain engine.

---

## 17. What is explicitly not proposed

1. No fifth StemThing leg.
2. No `ActionThing` peer entity. *(Owner gloss, adjudication 2026-08-10: this forbids PEER authority, not acting SimThings — an authored controller/actor of any kind is an ordinary SimThing acting through its own bands and overlays, auditable and STEAD-bound by construction.)*
3. No overlay manager/service as true owner.
4. No CPU per-generation overlay evaluator as final numerical authority.
5. No CPU child/action scheduler.
6. No human-readable kind/name as GPU dispatch key.
7. No second crossing machine.
8. No A*/predecessor/path-object simulation authority.
9. No actor-side subgroup search semantic.
10. No claim that all properties are RF resources.
11. No private Gu-Yang throughput/congestion model.
12. No mutation of field certificates/adjacency by overlays.
13. No coarse PALMA approximation overruling fine authority.
14. No dynamic GPU semantic template authoring.
15. No same-generation recursive convergence.
16. No silent Product→`EXP(SUM(LN))` replacement.
17. No silent sign erasure of Gu-Yang flux.
18. No per-leaf stamping of subtree-wide standing overlays.
19. No per-generation full ancestor-chain inheritance walk at population scale.
20. No ClauseScript/modifier/effect interpreter in the runtime.
21. No arbitrary runtime string dispatch for modifier/effect names.
22. No unbounded dynamic variable/template names.
23. No CPU iterator loop as the normal implementation of subtree-wide authored effects.
24. No pure same-generation algebraic cycle disguised as a modifier recalc loop.

---

## 18. Candidate promoted semantic laws

### 18.1 Intrinsic Overlay Ownership Law

> Every active overlay is semantically owned by a StemThing. No peer subsystem owns overlay lifetime or numerical execution.

### 18.2 Overlay Actuation Law

> Ordinary numerical action resolves through admitted overlay-state changes and ordinary Property/RF/Field-Triad/boundary consequences rather than a peer domain executor.

### 18.3 Sparse Inert Germ Law

> Overlay capability is universal in semantics but sparse in active physical state.

### 18.4 RF Distinction Law

> Overlay may deform RF inputs and claims, but only genuinely conserved/constrained quantities become RF resources.

### 18.5 Full Field-Triad Parameterization Law

> Overlay may parameterize admitted STEAD, PALMA, and Gu-Yang inputs while those mechanisms retain their native mathematical authority.

### 18.6 Field Certificate Envelope Law

> Runtime overlay variation must remain inside the admitted field registration's structural/conservation/stability envelope.

### 18.7 Overlay Lifecycle Authority Law

> Overlay activation, suspension, temporal state, dissolution, and collapse belong to the intrinsic overlay facility; CPU shadow may not remain ordinary numerical lifecycle authority after GPU promotion.

### 18.8 Generation-Paced Actuation Law

> Overlay next-state authorized in generation `t` participates no earlier than the ordinary next generation/barrier unless the value is part of an already-admitted pure current-generation transform that does not mutate overlay/state identity.

### 18.9 Opportunity-Horizon Law

> Nonlocal opportunity awareness is a field-propagation responsibility, not an ActionBand search-depth responsibility.

### 18.10 Dirty-Provenance Law

> Admitted overlay bindings should directly identify/invalidate dependent numerical work rather than rely on global change detection.

### 18.11 Ancestor Overlay Residency Law

> Subtree-scoped standing overlays are retained at the lawful ancestor whose scope they define and projected by recursive inheritance; stamping equivalent semantic overlay instances onto descendants is unlawful.

### 18.12 Inheritance Hot-Path Law

> Standing-overlay inheritance may be semantically recursive but may not require a per-generation `O(depth × descendants)` ancestor rewalk. Equivalent epoch/boundary-compiled effective parameters or another sparse invalidation-based lowering must provide hot reads without population-scale traversal.

### 18.13 Overlay Composition-Class Law

> Every overlay binding declares its composition class at admission. Sequential value transforms compose in authored order. Conjunctive restrictions compose monotonically and may not be weakened by a descendant. When multiple overlays bind one field/program parameter, the admitted per-parameter combine law is explicit; physical row/order accidents never choose semantics.

This closes the descendant-`Set`-weakens-ancestor-cap class of failure.

### 18.14 Derivation-to-Overlay-Template Law

> A derivation/tier capability grant includes the admitted overlay-template capability span the descendant may activate, alongside its other lane/residency capabilities. Derived specialists inherit lawful actuation vocabulary as data; vendors do not mint template scope per domain.

### 18.15 One-History / Crossing-History Law

> Overlay actuation history is recorded through the existing stamped crossing/schedule history surfaces. Overlay lifecycle must not mint a second replay/history mechanism.

### 18.16 Cross-Tree Actuation Seam Law

> Routed overlays derive origin→LCA→target within one authority tree. Cross-tree actuation enters another tree through the existing stamped product/receive seam; it does not bypass per-tree instantiation with a hidden global overlay route.

### 18.17 Gu-Yang Instantaneous-State Law

> Gu-Yang owns instantaneous conservative flux/stall facts; persistence belongs to separately admitted ordinary state/overlay memory.

### 18.18 Designer-Language Compilation Law

> Designer-facing semantic vocabulary, including ClauseScript modifier/effect names, scopes, scripted values, iterator forms, and control flow, must compile away at admission into bounded native templates, EML programs/predicates, projection bindings, Property/RF/CostBand/ActionBand state, or sealed structural requests. Source-language semantic nouns may not become runtime numerical authority.

### 18.19 Temporal Feedback Law

> Pure within-generation overlay/property/field transforms form an admitted acyclic numerical dependency graph. Cyclic authored semantics are represented by explicit Current→Next/staged state edges and recur only across an admitted generation/stage boundary; same-generation recursive modifier convergence is unlawful.

### 18.20 Bounded Projection Law

> Recursive authored application over descendants, scopes, selected receivers, or iterators lowers to bounded admitted projection/binding data. Population-scale semantic fanout may not be implemented as an unbounded CPU traversal or runtime dynamic semantic search.

### 18.21 Overlay-State Observability Law

> Active/suspended/lifecycle state required by authored predicates is exposed through admitted numeric state/bindings, not human-readable runtime lookup.

### 18.22 Static Runtime Vocabulary Law

> Runtime template, variable, property, and effect destinations are statically admitted/budgeted. Source-language dynamic names are rejected unless they resolve to a finite predeclared family.

---

## 19. Composition, derivation, routing, and authoring consequences

### 19.1 Composition classes are admission data

The core must not infer combine semantics from overlay kind or iteration order.

Typical classes:

```text
SequentialTransform
ConjunctiveRestriction
DeclaredParameterCombine
```

Exact Rust names are implementation-local; semantic class declaration is the law.

### 19.2 Ancestor restrictions cannot be loosened by descendants

Policy/governance-style predicates remain conjunctive/monotone. Ordinary value transforms may retain sequential authored semantics. The two categories must not be conflated.

### 19.3 Derivation closes into actuation vocabulary

Tier/capability price vectors should carry the set/span of overlay templates a derived child may activate. This makes “what the descendant can do” intrinsic admitted data.

### 19.4 Cross-tree actuation

Per-tree routing remains local. Inter-tree consequences cross the stamped seam as products/directives, then enter the destination tree through its receive leg and ordinary overlay routing/retention semantics.

### 19.5 History

A crossing/lifecycle event that matters for replay is represented by the existing stamped schedule/crossing history. Do not create an OverlayHistory service.

### 19.6 Authoring scopes become bindings, not runtime scope stacks

The compiler/admission layer may preserve rich source scope semantics while resolving them to stable relationships, spans, and selector bindings. Runtime overlay execution should consume only admitted structural/numeric references.

### 19.7 Ordered source semantics require explicit lowering proof

If a source-language effect depends on immediate visibility of an earlier state mutation, ordinary batch-next lowering is not automatically faithful. The authoring compiler must either:

- lower it into a proven equivalent pure EML/dataflow expression;
- use an already-admitted staged boundary with defined visibility;
- or reject/remand the construct.

A general imperative effect VM is not the answer.

---

## 20. Falsifiers

Promotion should remand if any of the following is required:

1. CPU behavior/action/event dispatch for routine ActionBand/CostBand outcomes.
2. Domain execution code for ordinary actions that should be expressible as admitted overlay/Property/RF/Field/boundary effects.
3. Per-SimThing hot overlay allocation even when inactive.
4. Continuous CPU lifecycle evaluation after GPU authority exists.
5. Runtime creation of new overlay semantics/EML/templates.
6. Same-generation recursive overlay cascades.
7. A second threshold/crossing detector.
8. Actor-owned path/predecessor search.
9. Human-readable domain names in core numerical dispatch.
10. Overlay bypass of RF conservation.
11. Overlay mutation of Gu-Yang/PALMA/STEAD proof metadata.
12. A private ActionBand/Overlay throughput or saturation solver beside Gu-Yang.
13. Coarse/multiscale solver state changing authoritative fine semantics without separate admission.
14. `abs(flux)` or equivalent sign erasure used as generic progress where signed target-relative flux is required.
15. Per-leaf stamping of one subtree-wide standing overlay at population scale.
16. Silent ordered-Product replacement by log-domain accumulation.
17. A per-generation `O(depth × descendants)` ancestor-chain inheritance walk for unchanged standing overlays.
18. Descendant overlay composition can weaken an ancestor conjunctive restriction because composition class was implicit.
19. A second overlay-specific replay/history mechanism is required.
20. Cross-tree overlay routing bypasses the stamped receive/product seam.
21. Runtime ClauseScript/modifier/effect string dispatch is required for numerical semantics.
22. A designer-authored cyclic dependency requires re-evaluating the same current-generation state until convergence.
23. An authored subtree/iterator effect requires a CPU per-receiver semantic loop as ordinary execution.
24. `has_modifier`-like predicates require CPU name lookup rather than admitted overlay-state observability.
25. Unbounded dynamic variable/modifier/template names are required at runtime.
26. Bounded source control flow requires a general script interpreter rather than finite admitted dataflow/staging.
27. Feedback into RF/Field-Triad lanes can amplify without admission bounds/clamps/stabilizing semantics.

---

## 21. Engineering questions left open

### 21.1 Overlay instance storage

- exact template/instance/binding table layout;
- capacity and residency pricing;
- grouping by owner/template/profile;
- logical-slot epoch rebind consequences;
- slot reuse after semantic dissolve.

### 21.2 GPU lifecycle extraction

This is the centerpiece engineering question:

- mapping existing dissolve conditions to GPU state/crossing semantics;
- `AfterTicks` without CPU decrement loops;
- override ordering;
- explicit remove/dissolve semantics;
- sparse semantic readback;
- indistinguishability versus the current CPU lifecycle oracle.

### 21.3 Ancestor inheritance lowering

- subtree-contiguous range use;
- effective-parameter plane/cache representation;
- dirty subtree invalidation;
- standing + local overlay composition;
- conjunction vs sequential transform lowering.

### 21.4 Cross-tree seam lowering

- product identity/stamp carried into receive-leg overlay ingress;
- replay ordering;
- origin provenance across tree boundaries.

### 21.5 OverlayFieldDependency

- dependency granularity: slot/cell/tile/property/field profile;
- duplicate dirty marks;
- current/next masks;
- dependency fanout admission bounds.

### 21.6 Designer-language dependency admission

- exact representation of the pure current-generation dependency DAG;
- cycle classification and diagnostics;
- projection/iterator fanout pricing;
- bounded dynamic-name families;
- ordered-effect equivalence/staging proof;
- mapping source `has_modifier`-like predicates onto admitted overlay-state lanes;
- deciding when a gated contribution should remain one persistent instance versus lifecycle attach/dissolve.

### 21.7 Field solver research questions

Remain in §14 and are not anchor blockers.

---

## 22. Two promotion units

The semantic closure and field-solver research have different proof standards and should not be anchored as one empirical dependency.

### 22.A Semantic closure — anchor candidate

Includes:

- StemThing overlay germ and ownership/lifecycle laws;
- RF + full Field-Triad parameterization;
- full EML actuation;
- ancestor residency/inheritance;
- composition classes;
- derivation→template capability;
- one-history and cross-tree seam;
- ActionBand/CostBand→Overlay actuation;
- designer-language compilation/feedback/projection closure;
- GPU lifecycle extraction/equivalence.

Evidence standard: archaeology, semantic equivalence, determinism, scale falsifiers, bounded admission, deletion of peer authority.

### 22.B Field-solver performance program — remains workshop research

Includes §14:

- FIM;
- multiscale FIM;
- warm-start/reweighting;
- dense↔sparse crossover;
- shared PALMA profiles;
- conditional Gu-Yang multilevel acceleration;
- profile compression.

Evidence standard: measurement and equivalence to the already-admitted field semantics.

**The semantic closure does not wait on the performance program.**

---

## 23. Suggested semantic rung/probe sequence

This is a proposal for DA review, not a workplan edit.

```text
A. OVERLAY-GERM-ARCHAEOLOGY
   enumerate every attach/activate/suspend/dissolve/apply route
   classify semantic duplicate vs genuine structural boundary

B. GPU-OVERLAY-LIFECYCLE-EXTRACTION
   retain CPU lifecycle as oracle
   GPU owns active/current/next + dissolve/timer state
   prove indistinguishability / replay equivalence
   retire CPU numerical lifecycle authority

C. ACTIONBAND→OVERLAY-ACTUATION
   existing ActionBand crossing
   → fixed admitted OverlayStateNext update
   → next-generation Property/RF consequence
   no direct domain executor

D. FULL-FIELD-TRIAD-OVERLAY-BINDING
   one overlay parameterizes STEAD, PALMA, and Gu-Yang admitted inputs
   certificate envelopes preserved
   no private solver

E. ANCESTOR-RESIDENCY / FRACTAL-ROUTING
   one ancestor policy over a large descendant subtree
   no leaf stamping
   no per-generation ancestor rewalk
   local overlays compose correctly
   conjunctive restriction mutant REDS

F. DESIGNER-LANGUAGE-FEEDBACK-CLOSURE
   generated modifier/effect names compile away
   gated modifier remains resident without attach churn
   has-modifier predicate reads admitted overlay state
   subtree iterator lowers to bounded projection
   Current→Current cycle REDS
   Current→Next temporal feedback loop runs deterministically

G. CROSS-TREE / ONE-HISTORY
   cross-tree actuation crosses stamped receive seam
   existing crossing/schedule history is sufficient for replay
```

Recommended sequencing relative to 0.0.8.7: after the ActionBand Gu-Yang remediation/semantic-shadow surfaces are stable and **before 8.2 outcomes-as-overlays hardens a second actuation path**. Exact numbering belongs to DA.

### 23.1 Lifecycle extraction deserves a dedicated rung

Do not hide `resolve_overlay_lifecycle()` removal inside a broad composition rung. Treat current CPU lifecycle as the oracle, prove GPU equivalence/indistinguishability, then retire the peer authority.

### 23.2 Designer-language probe should be adversarial, not broad-vocabulary busywork

The useful stress proof is not thousands of modifier names. It is a small deliberately difficult scenario containing:

```text
ancestor-scoped standing modifier
+ descendant-local modifier
+ generated long modifier key compiled to generic binding
+ gated/triggered modifier
+ timed modifier
+ has-modifier predicate
+ bounded subtree projection
+ RF/Field-Triad feedback
+ one lawful temporal cycle
+ one illegal same-generation algebraic cycle
```

This tests the semantic closure rather than parser breadth.

---

## 24. Separate performance research sequence

```text
P1. OVERLAY-DIRTY-PALMA
    exact dependency seed → tiled warm-start repair

P2. DENSE↔FIM CROSSOVER
    cold / sparse / medium / field-wide dirty cases

P3. MULTISCALE PALMA FIM
    same fine authoritative result, faster propagation

P4. GU-YANG DIRTY-THEATER
    avoid unchanged conservative work without changing propagation law

P5. GU-YANG DRIVING-POTENTIAL ARCHAEOLOGY
    determine whether a reusable scalar driver/dual-like field already exists

P6. CONDITIONAL MULTILEVEL GU-YANG
    only if registration semantics are fixed-point/equilibrium

P7. SHARED PROFILE / EML PARAMETRIC COMPRESSION
```

Nothing in this sequence blocks semantic promotion.

---

## 25. Promotion criteria

### 25.1 Semantic anchor readiness

The semantic closure is ready to move from `docs/workshop/` to `docs/` when:

1. Owner/DA agrees overlays are intrinsic StemThing actuation state.
2. StemThing is the one semantic owner of overlay lifetime.
3. Attach/receive/route/filter/activate/suspend/dissolve/remove/override/collapse each have one lawful home.
4. GPU vs CPU shadow authority is explicit.
5. ActionBand/CostBand resolve through one overlay actuation door rather than peer executors.
6. Full STEAD/PALMA/Gu-Yang authority is preserved.
7. Full EML is admitted only through the existing language/gadget laws.
8. Ancestor subtree overlays remain one authoritative instance, not descendant stamps.
9. Inheritance hot-path avoids `O(depth × descendants)` unchanged-tree rewalks.
10. Composition classes are admission-explicit.
11. Derived children receive overlay-template capability spans through derivation data.
12. Cross-tree actuation uses the stamped receive/product seam.
13. Existing crossing/schedule history is the one replay surface.
14. GPU lifecycle extraction has an equivalence/referee plan.
15. Default inertness remains credible at million-child scale.
16. Designer-facing ClauseScript modifier/effect vocabulary compiles to bounded native destinations without runtime semantic dispatch.
17. Active overlay state needed by authored predicates is numerically observable through admitted bindings.
18. Recursive/iterator authoring lowers to bounded projection rather than CPU population traversal.
19. Pure within-generation dependencies are acyclic; authored cycles become explicit temporal/staged feedback.
20. Dynamic authored names are bounded/static at runtime.
21. Ordered source semantics have an explicit equivalence/staging/rejection rule rather than accidental batch semantics.
22. No second crossing, planner, overlay manager, private flux solver, script VM, modifier manager, or CPU numerical lifecycle remains necessary.

### 25.2 Performance research is explicitly non-blocking

FIM/multiscale/shared-profile measurements do not gate the semantic anchor. They may promote later as measured physical lowerings.

---

## 26. Working synthesis

The strongest current architecture is:

> **StemThing is a default-inert recursive automaton whose durable state is ordinary Properties and whose participation is RF plus the complete Field Triad; whose `act` leg is ActionBand plus CostBand; whose `originate` and `receive` legs are one intrinsic overlay capability; and whose nonlocal opportunity information is supplied by propagated fields rather than actor-owned search. Subtree-wide policy/state actuation resides sparsely at the ancestor that owns its scope and is projected downward by recursive inheritance, never stamped across the population.**

The semantic execution loop is:

```text
state
  ↓
RF / Field-Triad participation
  ↓
ActionBand discrepancy + CostBand work
  ↓
full EML crossing/payload
  ↓
OverlayStateNext
  ↓
ancestor/local routing + composition
  ↓
state / RF / Field-Triad parameters
  ↓
next generation
```

The designer-language contract fits above, not inside, this loop:

```text
ClauseScript
rich semantic vocabulary
scope-relative authoring
generated modifier names
script values / conditions
bounded iterators
staged activities
cyclic authored dependencies
        ↓
admission / compilation
        ↓
finite Overlay templates + EML + bindings
RF / CostBand / ActionBand state
Property next-state / sealed boundary effects
        ↓
StemThing loop
```

The critical closure property is that even an ugly self-referential modifier network becomes **ordinary temporal feedback through StemThing** rather than a recursive modifier evaluator. Long names and rich scope syntax are an admission problem; persistence, actuation, propagation, conservation, and action remain native SimThing concerns.

The performance program is layered underneath this semantic closure rather than fused to it: overlay provenance may seed incremental PALMA/Gu-Yang work; FIM/multiscale may accelerate PALMA; Gu-Yang may expose free signed-feasibility or a reusable driving potential if archaeology proves one exists; and EXP/LN provide bounded temporal/nonlinear coupling between instantaneous field facts and overlay actuation. None of those optimizations changes the semantic owner or creates a new engine.

If this closure survives implementation scrutiny, future capability growth becomes admitted data — templates, EML, field parameters, RF lanes, targets, thresholds, derivation spans, and bounded projection bindings — rather than another architectural refactor.

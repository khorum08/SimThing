# StemThing Intrinsic Overlay Capability
## Engineering-review candidate for closing RF + Field Triad (STEAD / PALMA / Gu-Yang), full EML, CostBand, ActionBand, and Overlay into one recursive SimThing automaton

> **Status: WORKSHOP / ENGINEERING-REVIEW CANDIDATE / UNANCHORED.**
>
> Owner-approved working design, revised 2026-08-10. This document is deliberately housed under `docs/workshop/` while the overlay closure is tested, reviewed, and amended. It proposes the final semantic closure of the base recursive **StemThing**: the thesis that the base `SimThing` is the sole semantic owner of overlay origination/emission, retention, reception, routing/filtering, projection, activation/suspension, lifecycle, dissolution, and collapse; and that ordinary numerical action is expressed by that intrinsic overlay capability rather than by a peer event-execution or behavior subsystem.
>
> **This document does not amend the 0.0.8.7 ladder, does not supersede an anchor, and does not authorize implementation by itself.** Existing anchors and graduated rungs remain authoritative until this workshop is adjudicated and promoted.
>
> The design now explicitly incorporates the **full RF / Field-Triad authority** — STEAD, PALMA, and Gu-Yang / `SaturatingFlux` — and the **full admitted EML direction**, including deterministic `EXP`/`LN` and their gadget-space consequences. The earlier STEAD/PALMA-only framing is superseded in this workshop.

Primary repository companions:

- [`../stead_stemthing_unification.md`](../stead_stemthing_unification.md) — StemThing four-leg/lane unification and residency/derivation law.
- [`../stead_simthing_automata.md`](../stead_simthing_automata.md) — minimum viable Wei automaton; four legs; events-as-RF; overlay hold/project archaeology.
- [`../multi-axis-ActionBand-STEAD.md`](../multi-axis-ActionBand-STEAD.md) — intrinsic GPU ActionBand, now re-derived against the full Field Triad.
- [`../simthing_core_design.md`](../simthing_core_design.md) — core GPU/RF/Field-Triad/overlay authority.
- [`../stead_spatial_contract.md`](../stead_spatial_contract.md) — spatial substrate, PALMA/Gu-Yang field-sweep contract, comparative projections.
- [`../full_eml_unification.md`](../full_eml_unification.md) — `EXP`/`LN` admission design and full EML operator/gadget consequences.
- [`../EML_exp_ln_unification_expansion.md`](../EML_exp_ln_unification_expansion.md) — ActionBand/EML horizon synthesis and tiled execution candidates.
- [`../eml_gadget_library.md`](../eml_gadget_library.md) — reusable admitted numerical gadgets and bounded-feedback discipline.
- [`../design_0_0_8_7_rf_arena_modernization.md`](../design_0_0_8_7_rf_arena_modernization.md) — live workplan; ladder authority wins over this workshop.

Primary implementation archaeology:

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

---

## 0. Executive thesis

The candidate closure is:

> **The StemThing is the simulation cell. All ordinary numerical action is expressed by the StemThing originating, retaining, projecting, modifying, suspending, or dissolving admitted overlays. ActionBand and CostBand do not become peer executors; they authorize changes in intrinsic overlay state. Those overlays parameterize and deform ordinary properties, RF lanes, and the complete Field Triad — STEAD, PALMA, and Gu-Yang — through admitted EML until authored lifecycle/goal conditions resolve.**

This does **not** mean every quantity becomes RF-conserved and does **not** mean an overlay gains authority over the physics it influences.

The intended authority split is:

```text
Property / governed value
    = ordinary state

RF
    = conserved / constrained claims, clearing, disbursement, balances

STEAD
    = non-conserved propagated signal / influence / causal circumstance

PALMA
    = non-conserved min-plus potential / impedance / reach-value field

Gu-Yang / SaturatingFlux
    = conserved saturating flux / realizable channel throughput / stall

EML
    = admitted deterministic numerical law over those observables

ActionBand
    = unresolved target discrepancy and target-seeking lifecycle

CostBand
    = exact sink/work quantization over actually available value

Overlay
    = intrinsic actuation state / parameterization emitted by a SimThing

BoundaryRequest
    = sealed structural consequence when numerical state requires tree/topology mutation
```

Everything participates in one recursive SimThing automaton, but these mechanisms retain distinct mathematics and authority.

### 0.1 Complete recursive loop

```text
                              STEMTHING
                                  │
                            ordinary state
                                  │
                        participate / receive
                                  │
                 Properties + RF + active Overlays
                                  │
                    admitted EML parameterization
                                  │
                    FIELD-TRIAD / RF EXECUTION
               ┌──────────────────┼──────────────────┐
               │                  │                  │
            STEAD               PALMA             GU-YANG
      non-conserved field      min-plus D       conserved flux
        / influence            / impedance      / saturation
               │                  │                  │
               └──────────────────┼──────────────────┘
                                  │
                      anchored/resultant observables
                                  │
                                 act
                                  │
                       ActionBand / CostBand
                                  │
                    existing sealed threshold crossing
                                  │
                            admitted EML
                                  │
                              originate
                                  │
                         OverlayStateNext
                                  │
                        generation boundary
                                  │
                             route/filter
                                  │
                              receive
                                  │
                                 └──────────────► STEMTHING
```

No CPU behavior tree, action dispatcher, movement executor, combat executor, event execution engine, or domain action taxonomy is required by this model.

### 0.2 Why this is closure rather than a new subsystem

The repository already contains most pieces:

- `SimThing` already directly owns `overlays: Vec<Overlay>`.
- standing overlays already inherit recursively without descendant copies;
- routed overlays already derive origin → common ancestor → target policy paths;
- RF disbursement already has a production path that terminates a delivered directive in `SimThing.overlays`;
- overlays already lower into transforms consumed by GPU accumulator/order-band machinery;
- `TransformOp` is already EML-shaped rather than a rival static modifier language;
- PALMA and Gu-Yang already use generic `FieldSweepRegistration` machinery over admitted adjacency;
- CostBand already defines the singular sink/work quantization hinge;
- ActionBand is being implemented as intrinsic `act` over the existing Phase-5 crossing machinery;
- generation pacing already prevents same-generation recursive convergence.

The new architectural act is **authority concentration**: move remaining overlay lifecycle/origination authority behind the StemThing germ and make overlays the one intrinsic actuation layer over the native RF/Field-Triad substrate.

---

## 1. Authority vocabulary used in this workshop

| Tag | Meaning |
|---|---|
| **INHERITED LAW** | already governed by anchor / graduated mechanism; this workshop consumes it |
| **OWNER-DIRECTED** | explicit Owner architectural intent; promotion still requires corpus process |
| **WORKSHOP CANDIDATE** | proposed design interpretation requiring DA/engineering review |
| **RESEARCH CANDIDATE** | promising physical optimization; not required for semantic closure |
| **REJECTED** | considered and deliberately excluded |

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

Candidate closed-loop interpretation:

| Leg | Closed-loop meaning |
|---|---|
| **participate** | expose ordinary property/RF/Field-Triad state to recursive reduce/disburse/field execution |
| **act** | ActionBand identifies unresolved discrepancy; CostBand quantizes executable work from native resolved means |
| **originate** | activate/parameterize/update/dissolve intrinsic overlays as numerical actuation |
| **receive** | receive routed/inherited overlays, filter them through route/policy law, and expose admitted numerical effects to local property/RF/Field-Triad evaluation |

This recovers the old automaton archaeology: `originate` was already “produce, hold, project overlays”; hold/project existed while production/origination remained incomplete.

### 2.1 Default inertness

**INHERITED LAW / OWNER-DIRECTED:** overlay capability is universal semantically and sparse physically.

```text
no active overlay
    → no active overlay instance row
    → no active binding span
    → no lifecycle work
    → no field dirtiness
    → essentially zero hot-loop overlay cost
```

The germ is universal; numerical state is pay-for-play.

---

## 3. Overlay is the candidate intrinsic actuation language

**OWNER-DIRECTED / WORKSHOP CANDIDATE:**

> **An action is the activation, parameterization, modification, suspension, or dissolution of an admitted overlay whose consequences are realized through ordinary SimThing property/RF/Field-Triad substrate.**

Preferred concise form:

> **Overlays are the intrinsic actuation language/state of SimThing.**

An overlay need not be instantaneous. It may remain active for many generations and continuously parameterize values, weights, fields, demand, impedance, or capacity until its lifecycle resolves.

### 3.1 Responsibility split

```text
ActionBand
    why / toward what / when unresolved discrepancy should be reduced

RF + Gu-Yang
    what constrained/conserved means and channel throughput actually exist

CostBand
    how much sink/work those means buy

Overlay
    what actuation/parameterization remains active

STEAD / PALMA / Properties
    resultant non-conserved field, route/value, and ordinary state
```

A normal ActionBand/CostBand consequence should prefer:

```text
activate/update admitted OverlayTemplate K with parameters P
```

over bespoke world mutation.

Structural effects remain sealed boundary consequences.

### 3.2 “Everything is RF” is rejected as a conservation statement

**REJECTED:** converting location, political disposition, target, threat, ActionBand displacement, PALMA potential, or semantic axes into conserved RF merely because overlays affect them.

Correct statement:

> **Everything participates in one recursive SimThing flow architecture; only genuinely conserved/constrained quantities use RF/Gu-Yang conservation semantics.**

---

## 4. Current overlay archaeology: what already belongs to the germ

### 4.1 Retention/home already lives on `SimThing`

Current base storage includes `pub overlays: Vec<Overlay>` and `add_overlay`. The semantic home is therefore already the SimThing rather than a global event manager.

### 4.2 Routed attachment terminates in ordinary retained overlay state

`deliver_routed_overlay()` validates attributable origin/target authority, derives origin→LCA→target route identity, applies dispatch-mint admission where required, and terminates by adding the overlay to the target SimThing.

Delivery is not a peer inbox.

### 4.3 Standing inheritance and route filtering already recurse fractally

`inherit_active_overlays()` folds ancestor active overlays into inherited `TransformStack`; `LiveOverlayRoutes` re-evaluates live route policy instead of retaining a separate route cache. This is already the fractal projection shape sought here.

### 4.4 RF → Overlay already exists

`simthing-driver::automaton_reception` routes attributable command deficits through existing RF disbursement and only delivers the directive overlay when allocation is positive.

```text
RF deficit / clear / disburse
        ↓
allocated directive
        ↓
Overlay retained by receiving SimThing
```

### 4.5 Overlay → RF already exists through ordinary numerical lanes

Current physical path:

```text
SimThing.overlays
      ↓ CPU preparation today
build_overlay_deltas()
      ↓
ordered per-slot deltas
      ↓
AccumulatorOp / OrderBands
      ↓
Property / RF input columns
      ↓
ordinary RF execution
```

Candidate semantic law:

> **Overlay affects RF by changing admitted numerical conditions under which ordinary RF executes; overlay does not invoke a second RF API.**

### 4.6 Transform operations are already EML-shaped

`TransformOp` is an admitted EML program representation; `Set`, `Add`, `Multiply` are degenerate constructors. This is strong evidence that overlays should become a universal numerical actuator through shared EML rather than domain kernels.

---

## 5. Current authority fragmentation to reconcile

### 5.1 Player/AI feeder paths remain peer ingress machinery

`PlayerIntentOverlay` / `AiIntentOverlay` paths currently combine immediate patch/GPU intent with parked boundary attachment. Desired end state: provenance may differ, activation semantics should not.

### 5.2 Overlay lifecycle is still CPU boundary authority

Current `resolve_overlay_lifecycle()` walks the tree, reads GPU-fresh values through CPU shadow, evaluates dissolve conditions, decrements timers, removes overlays, and applies expire effects.

This is the largest obvious peer numerical authority to extract if GPU-resident overlay lifecycle is promoted.

### 5.3 Activation/suspension are boundary mutations today

Current `BoundaryRequest` includes attach/activate/suspend. Workshop question: ordinary numerical activation/suspension should likely become GPU current/next overlay state, reserving CPU boundary work for storage/structural persistence.

### 5.4 Explicit generic dissolution/removal needs one canonical door

Automatic lifecycle removal exists; explicit ordinary dissolve/removal needs to be proven as one intrinsic overlay lifecycle operation rather than an assortment of caller-specific mutations.

### 5.5 `OverrideReceived` requires archaeology

Current lifecycle code defers `OverrideReceived` semantics to attach. Promotion must prove the promised transition exists in one authoritative place.

---

## 6. Candidate intrinsic overlay germ

The semantic StemThing capability owns five behaviors, not five services:

```text
OVERLAY GERM

originate
    activate / parameterize an admitted overlay

receive
    accept attributable routed / inherited overlay state

project
    expose effective overlay EML/parameters into Property / RF / Field-Triad inputs

lifecycle
    active ↔ suspended → dissolved under admitted conditions

collapse
    retire resolved transient numerical state; ordinary world state is durable memory
```

### 6.1 Intrinsic ownership candidate law

> **Nothing outside the SimThing owns an overlay's active lifetime. External systems may author templates, supply directives, or apply genuinely structural storage changes at a boundary, but originate/receive/project/activate/suspend/dissolve are one intrinsic StemThing capability.**

### 6.2 Generation pacing

```text
generation t
    ActionBand / CostBand / RF consequence authorizes OverlayNext

barrier

generation t+1
    new overlay participates in ordinary RF/Field-Triad evaluation
```

No same-generation emit→apply→re-cross→re-emit convergence.

---

## 7. Proposed GPU physical model

Semantic ownership by every StemThing does not imply a rich overlay object in every hot row.

```text
OverlayTemplate[]
    admitted EML program(s)
    lifecycle predicate/program
    projection/filter law
    target/binding schema
    parameter schema
    field-dependency span

OverlayInstance[]
    owner logical slot
    origin logical slot
    template id
    binding span
    parameter span
    lifecycle state
    admitted temporal state where required

OverlayStateCurrent[]
OverlayStateNext[]

OverlayBinding[]
    instance → target slot/property/role/field parameter

OverlayFieldDependency[]
    overlay binding → affected RF/STEAD/PALMA/Gu-Yang registration/locus/profile
```

### 7.1 CPU semantic shadow

GPU owns ids, slots, EML programs, lifecycle bits/state, parameter vectors, field binding spans, and numerical execution. CPU owns authoring/display names, source spans, diagnostics, persistence mapping, categorical history, and sparse structural/lifecycle egress where required.

Human-readable overlay names must not be runtime dispatch keys.

### 7.2 Overlay taxonomy should not become GPU behavior taxonomy

`OverlayKind` / `OverlaySource` may remain useful provenance. Numerical behavior should prefer orthogonal admitted descriptors: projection law, lifecycle law, transform EML, origin/target binding, field-parameter binding, and route/filter law.

### 7.3 Sparse batching and exact local fusion

Bucket active instances by template/program/binding shape. Trivial affine chains may be folded when exact authored ordering and EML arithmetic semantics are preserved. More general transforms remain shared admitted EML execution.

---

## 8. Full RF / Field-Triad integration

This section is load-bearing. Overlay is the actuation/parameterization layer over native mechanisms; it never becomes a replacement for them.

### 8.1 RF: constrained means

Overlay may parameterize production/consumption rates, claim pressure, allocator weights, availability, or demand activation. It may not bypass claim→clear→disburse or mutate conservation balances directly outside native RF semantics.

### 8.2 STEAD: non-conserved field composition

Overlay may alter admitted source strength, attenuation/falloff parameters, contextual valuation, or other STEAD inputs.

```text
base STEAD inputs
    + admitted overlay parameterization
        ↓
existing STEAD field sweep
```

An overlay should normally alter the native input before the authoritative field is solved, not bolt a private local correction onto the solved field.

### 8.3 PALMA: route/value over the composed field

Overlay may alter admitted `W`, terminal opportunity value, access/impedance, or other PALMA input parameterization.

```text
base state + STEAD + overlays
        ↓
admitted PALMA W / terminal-value EML
        ↓
PALMA solve
        ↓
authoritative D / potential
```

Semantic factors that materially alter route choice should enter the admitted PALMA valuation before solve where practical, avoiding false local minima caused by post-hoc local deformation.

### 8.4 Gu-Yang / `SaturatingFlux`: conserved realizable throughput

Gu-Yang remains the native conserved-field authority. Overlay may parameterize admitted demand/source/sink intensity, availability, capacity factor, conductance weighting, or channel participation **inside the already-certified registration envelope**.

```text
base RF / channel state
    + admitted overlay parameterization
        ↓
existing Gu-Yang conservative sweep
        ↓
realized signed flux / gross flux / stall / saturation / sanctioned projections
```

Overlay may not compute a private throughput result, congestion model, or claimant arbitration law.

### 8.5 Overlay Field-Parameter Law — candidate

> **An overlay may parameterize admitted Field-Triad programs and ordinary numerical inputs but may not mutate structural adjacency, canonical-order proof, conservation/symmetry certificate, χ/stability certificate, or the semantic field law under which the registration was admitted. Runtime variation must remain inside the registration's admitted parameter envelope.**

This is especially important for Gu-Yang conductance/capacity variation.

### 8.6 Gu-Yang is instantaneous; persistence is authored state

> **Gu-Yang remains the instantaneous conservative mechanism. Persistence of saturation/stall belongs to admitted ordinary state/overlay state/EML memory and never to hidden Gu-Yang history or a saturation listener.**

If an author needs “stall for 20 generations,” that persistence is explicit state.

---

## 9. Full EML overlay actuation

The overlay architecture must consume the full admitted EML direction, not merely `Set/Add/Multiply`.

### 9.1 Arithmetic/primitive authority

Overlay EML inherits the same arm-independent arithmetic law as every other EML consumer: IEEE single-rounding, no reassociation, uniqueness-based MUL→ADD/SUB fusion, and pinned exact primitive semantics for admitted `EXP`/`LN`.

No overlay-specific arithmetic dialect is lawful.

### 9.2 ContinuousDecay

Canonical form:

\[
x_{t+1}=x_t\,\mathrm{EXP}(-\lambda\Delta t)
\]

Useful overlay state: urgency, fear, memory, mobilization, temporary efficiency, exploration pressure, disease pressure, political momentum, congestion memory.

This converts “persistent temporary effect” into ordinary state + EML rather than a lifecycle-type explosion.

### 9.3 Logistic / bounded smooth response

A stabilized logistic gadget provides bounded response in `(0,1)` without stair-step engine states.

Uses: scarcity→rationing intensity, threat→mobilization, unemployment→recruitment response, saturation-memory→route aversion, price differential→trade preference.

> **Bands decide discrete semantic events; EML shapes continuous overlay intensity between events.**

### 9.4 PowerLaw

\[
x^a=\mathrm{EXP}(a\,\mathrm{LN}(x))
\]

Uses: falloff, congestion penalty, economies/diseconomies of scale, influence attenuation, risk sensitivity, substitution curves. Domain guards/certificates remain the EML primitive-admission law, not overlay exceptions.

### 9.5 Softmax weighting

For candidate channels/edges/options:

\[
p_i = \frac{\mathrm{EXP}(\beta(z_i-z_{max}))}{\sum_j \mathrm{EXP}(\beta(z_j-z_{max}))}
\]

Useful for RF claim weights, supplier preference, migration pressure, target/profile weighting, or policy emphasis. `β` smoothly spans diversified effort to winner-take-most without an engine enum.

### 9.6 Entropy / LN meta-observable

\[
H=-\sum_i p_i\mathrm{LN}(p_i)
\]

This generic scalar may drive commitment persistence, diversification response, diagnostic output, or other EML feedback without the engine assigning semantic nouns.

### 9.7 Log-domain accumulation is a new authored law, never a hidden Product optimization

For positive multiplicative effects one may author:

\[
\log x' = \log x + \sum_i \log a_i
\]

but this is not bit-equivalent to ordered f32 multiplication and must not silently replace it.

A promising Gu-Yang parameterization is a positive conductance factor:

\[
c = \operatorname{ClampCertified}\left(\mathrm{EXP}(\ell_0 + \sum_i \Delta\ell_i)\right)
\]

where the final value remains inside the Gu-Yang registration's certified envelope.

### 9.8 Canonical Gu-Yang stall-memory response

Let `s_t` be instantaneous Gu-Yang stall and `m_t` explicit admitted memory:

\[
m_{t+1}=m_t\,\mathrm{EXP}(-\lambda\Delta t)+\alpha s_t
\]

Then bounded aversion/response:

\[
a_t=\sigma(k(m_t-m_0))
\]

and an overlay may parameterize next-generation PALMA impedance or RF demand with `a_t`.

Authority remains clean:

```text
Gu-Yang = instantaneous physical fact
EML     = temporal / nonlinear law
Overlay = actuation
PALMA   = route/value consequence
```

All field-seeded recurrent emissions must obey the existing bounded-feedback admission contract: finite decay where persistence is used, explicit clamps/bounds, no admitted positive unbounded recurrence.

---

## 10. Overlay lifecycle should ride already-hot RF / Field-Triad outputs

Field/RF outputs are ordinary anchored observables. Therefore lifecycle should prefer existing threshold machinery rather than a new scanner.

Examples:

```text
Blockade overlay
    dissolve when GuYangRealized > reopen threshold

Emergency rationing overlay
    dissolve when RF food balance > target

Evacuation overlay
    dissolve when PALMA D to safe locus reaches terminal bound

Exploration-pressure overlay
    dissolve when ActionBand progress resumes
```

Preferred path:

```text
already-hot RF / FieldSweep write
        ↓
existing Phase-5 crossing
        ↓
OverlayStateNext.active / suspended / dissolved
```

No `SaturationListener`, CPU lifecycle predicate loop, or second threshold engine.

---

## 11. Movement as full-Field-Triad worked example

Movement is not privileged architecture. It is a literal witness because location vectors, route potential, conserved movement throughput, and CostBand work can all be observed together.

```text
Location property / velocity state
        ↓
ActionBand destination target
        ↓
STEAD + overlays compose local circumstances
        ↓
PALMA gives lawful local potential descent
        ↓
Gu-Yang/RF gives signed realizable capacity-bearing throughput where applicable
        ↓
CostBand quantizes movement/fuel sink from the authoritative available value
        ↓
ActionBand crossing authorizes MovementActuation overlay
        ↓
location velocity / demand / structural ingress changes
        ↓
ordinary next-generation state
```

No movement-specific destination authority, path/predecessor object, congestion solver, saturation listener, or `MoveFleet()` execution engine is needed.

If structural parentage is authoritative placement, the numerical overlay/action ultimately authorizes the existing sealed structural boundary request.

---

## 12. Opportunity horizon and local-minimum findings

The workshop distinguishes four causes of an apparent trap.

### 12.1 Escape vs opportunity horizon

**Escape problem:** target is known but local effective behavior appears stationary.

**Opportunity-horizon problem:** a better remote resolution exists but its information has not propagated to the actor.

ActionBand should not acquire a private search tree for either.

### 12.2 Opportunity Horizon Principle — candidate

> **ActionBand remains a local consumer of native fields. Nonlocal opportunity awareness is a field-propagation responsibility. Increasing horizon means improving the admitted field's propagation, hierarchy, or shared profile solution, not increasing actor-side search depth.**

### 12.3 Bellman/PALMA composition

For:

\[
J(x)=\min_{y\in N(x)}[W(x,y)+J(y)]
\]

a converged reachable nonterminal value field under ordinary positive impedance has lawful descending local structure. Apparent traps often indicate stale/incomplete value propagation or omitted dynamic impedance rather than a need for actor search.

### 12.4 Opportunity-valued multi-source PALMA

A promising objective is:

\[
J_a(x)=\min_g[B_a(g)+\operatorname{Dist}_{W_a}(x,g)]
\]

with `B_a(g)` carrying authored opportunity value.

Assigning different terminal values intentionally changes the objective; it is **not** policy-invariant reward shaping.

---

## 13. Gu-Yang as opportunity-surface information

Gu-Yang may contribute to the opportunity-horizon solution more directly than a simple throughput cap.

### 13.1 Signed realized flux is local feasibility information

For conserved actions, target-aligned signed flux carries a stronger fact than PALMA alone:

```text
PALMA D gradient
    = desirable / low-cost local descent

Gu-Yang q
    = physically realizable conserved transport direction and magnitude now
```

If both agree, ActionBand can consume the native field result with almost no marginal routing computation. If PALMA points through a saturated edge while Gu-Yang shows zero/negative target-aligned flux, the route/value field is missing a live feasibility cost rather than proving a need for actor search.

### 13.2 Gu-Yang stall/contest/choke can become PALMA input through overlays

A lawful coupled path is:

```text
Gu-Yang realized/gross/net flux
        ↓
instantaneous stall / sanctioned contest/choke projection
        ↓
EML bounded temporal response
        ↓
Overlay parameterizes PALMA W
        ↓
PALMA repairs value field
```

This converts congestion-induced apparent traps into ordinary coupled-field evolution.

### 13.3 Gu-Yang driving-potential / dual-like archaeology probe

**ENGINEERING/RESEARCH QUESTION:** many conservative-flow formulations derive flux from a scalar pressure/driving potential. Do not assume this exists in Gu-Yang; inspect the landed law.

Probe questions:

```text
Does Gu-Yang already compute or imply:
    node pressure?
    source potential?
    residual / dual-like scalar?
    reduced-cost-like driver?
    another scalar whose difference determines signed flux?

If yes:
    is it already materialized?
    can PALMA consume it directly?
    can it warm-start / bound / precondition PALMA?
    can exposing it avoid a duplicate field solve?
```

A positive finding could surface nearly-free route-relevant information. A negative finding leaves signed flux/stall reuse intact.

### 13.4 Gu-Yang may shrink the true local-minimum problem

Diagnostic order for an apparent trap:

```text
1. Is PALMA converged/current?
      no → repair PALMA (dense/FIM/multiscale as lawful)

2. Is the conserved channel physically saturated or opposed?
      yes → consume Gu-Yang flux/stall and feed native field response next generation

3. Is remote opportunity outside current information horizon?
      yes → improve PALMA opportunity propagation / shared hierarchy

4. Does an actual authored nonconvex/adversarial valuation remain?
      yes → later fenced navigation research
```

A large fraction of “local minima” may therefore reduce to stale PALMA information, omitted Gu-Yang capacity information, slow opportunity propagation, or badly damped feedback.

### 13.5 Zero flux is not structural unreachability

`q == 0` can mean no demand, cancellation, temporary saturation, absent source, or closed capacity. Only an admitted capacity/reachability certificate may permanently prune PALMA. Temporary Gu-Yang state may still suppress unnecessary high-frequency repair until the dependency changes.

---

## 14. Research evaluation and disposition

### 14.1 Ng–Harada–Russell potential shaping: corrected use

**Paper:** Andrew Y. Ng, Daishi Harada, Stuart Russell, *Policy Invariance Under Reward Transformations: Theory and Application to Reward Shaping* (ICML 1999).

Reference: <https://aima.cs.berkeley.edu/~russell/resume.html>

Useful theorem form:

\[
F(s,s')=\gamma\Phi(s')-\Phi(s)
\]

**Retained insight:** arbitrary local shaping can change behavior; globally coherent potential differences have special invariance properties.

**Rejected overclaim:** opportunity-valued terminal conditions do not preserve an old objective; they intentionally define a new one.

### 14.2 Potential/gauge reparameterization as PALMA preconditioning

Research candidate:

\[
W_h(u,v)=W(u,v)+h(v)-h(u)
\]

with terminal adjustment:

\[
B_h(g)=B(g)-h(g)
\]

For fixed start, potential terms telescope along a path and can preserve minimizers under the required admissibility conditions.

Candidate use:

```text
h ≈ J[t-1]
    ↓
residual / reweighted PALMA problem
    ↓
dirty FIM / multiscale repair
```

This complements FIM rather than replacing it: reparameterization may make the residual easier; FIM reduces the region/iterations solved.

**Explicit rejection:** do not apply this transform to Gu-Yang flux merely because it telescopes on paths. Gu-Yang is a local conservative law, not a shortest-path objective. Changing its driver changes physical flux unless separately authored.

### 14.3 HJB/Bellman framing

PALMA's min-plus recursion is naturally related to discrete Hamilton–Jacobi/Eikonal/value-function propagation. Included as mathematical framing, not a claim that every SimThing field is a continuous HJB problem.

### 14.4 FIM

**Original:** Won-Ki Jeong, Ross T. Whitaker, *A Fast Iterative Method for Eikonal Equations*, SIAM J. Sci. Comput. 30(5), 2008.

- <https://epubs.siam.org/doi/10.1137/060670298>

**GPU-oriented improvement:** Yuhao Huang, *Improved Fast Iterative Algorithm for Eikonal Equation for GPU Computing* (2021).

- <https://arxiv.org/abs/2106.15869>

FIM is retained as a **PALMA physical-lowering candidate**, not a new semantic. SimThing should prefer deterministic active-tile/current-next lowering over semantic dependence on dynamic atomic queue order.

### 14.5 Multiscale FIM

**Paper:** Jingqi Zhang et al., *A parallel multiscale FIM approach in solving the Eikonal equation on GPU*, Computer-Aided Design 189 (2025), 103949.

- <https://www.sciencedirect.com/science/article/pii/S0010448525001101>
- <https://doi.org/10.1016/j.cad.2025.103949>

Retained because it accelerates long-range propagation of the **same PALMA problem** through coarse/fine V-cycle structure while leaving the fine field authoritative.

Rejected interpretation: coarse/smoothed field becomes behavior authority or erases real prohibitions/chokepoints.

### 14.6 Multiscale Gu-Yang: conditional research only

The multiscale/FIM **systems architecture** — hierarchy, dirty regions, current/next tiles, coarse correction — may transfer to Gu-Yang, but FIM mathematics does not.

Critical semantic test:

> **Is a given Gu-Yang registration a within-generation equilibrium/fixed-point solve, or is each local conservative sweep itself the intended finite-rate physical evolution?**

If the former, a conservative multilevel acceleration may be lawful if it converges to the exact fine-grid authority and preserves conservation/certificates.

If the latter, coarse propagation would change information/transport speed and therefore change simulation semantics. It is forbidden as a mere optimization.

### 14.7 Dirty conservative theaters — immediately promising

Even if multiscale Gu-Yang is rejected, FIM-inspired **causal work scheduling** remains promising:

```text
Overlay changes Gu-Yang parameter at locus
        ↓
known Gu-Yang dependency span
        ↓
mark conservative tiles/theater dirty
        ↓
run native Gu-Yang only where required by its own propagation law
```

This avoids recomputing unchanged conservative theaters without accelerating physical propagation.

### 14.8 MeshFIM / irregular topology

**Paper:** Zhisong Fu et al., *A Fast Iterative Method for Solving the Eikonal Equation on Triangulated Surfaces*.

- <https://epubs.siam.org/doi/10.1137/100788951>
- <https://pmc.ncbi.nlm.nih.gov/articles/PMC3360588/>

Retained as evidence that the FIM family is not intrinsically limited to rectangular grids. Ordinary structured Location lattices should still prefer the simplest structured lowering.

### 14.9 Reaction–diffusion/operator splitting

Turing-style reaction-diffusion is retained as an execution analogy, not ontology proof. The useful engineering split is local pointwise overlay/EML reaction versus RF/Field-Triad transport/relaxation.

### 14.10 PID / closed-loop control

Retained only as broad feedback analogy. PID is not universal ActionBand semantics.

### 14.11 Subgroup horizon probing

**REJECTED as semantic architecture.** Subgroup operations may optimize a field kernel but may not make ActionBand a local planner that samples neighbors and silently changes its target.

### 14.12 Tropical/min-plus matrix view

PALMA may be viewed as min-plus/tropical matrix-vector relaxation:

\[
D'_i=\min_j(W_{ij}+D_j)
\]

but structured N4/N8 lowerings should normally be stencil/tile kernels rather than dense GEMM. Softmin/log-sum-exp must not replace exact min merely to access tensor-core hardware.

---

## 15. Deeper GPU optimizations exposed by intrinsic overlays

### 15.1 Overlay state is the dirty-set generator for the full Field Triad

Because an admitted overlay binding already knows the parameters/loci it can modify:

```text
OverlayStateNext changes
        ↓
OverlayFieldDependency span
        ↓
mark only affected:
    RF lane(s)
    STEAD registration/loci
    PALMA profile/tile(s)
    Gu-Yang conservative theater/tile(s)
```

No global “what changed?” scan is required.

### 15.2 Invalidation classes

A physical descriptor may distinguish scheduler behavior without adding domain semantics:

```text
PointwiseOnly
LocalStencil
PropagatingPotential
ConservativeFlux
```

Exact names are implementation-local. The key is that overlay admission provides the dependency shape needed to seed native work.

### 15.3 Overlay-to-Field ingress fusion

If an overlay-deformed intermediate is not independently authoritative/observable, the local overlay EML and a `FieldSweepRegistration` map program may be faithfully fused:

```text
base column + overlay params
        ↓
fused exact EML ingress
        ↓
existing field fold/post
```

instead of materialize→barrier→reload.

This is physical-only and lawful only if exact EML arithmetic/order, route/filter resolution, field certification, and generation ordering are unchanged.

### 15.4 Temporal PALMA warm start

Retain `J[t]`, apply localized overlay/field changes, repair into `J[t+1]`. Quiet regions should pay essentially zero PALMA work under incremental posture.

### 15.5 Hybrid dense/FIM PALMA

```text
small dirty fraction → tiled FIM
large dirty fraction / cold start → dense PALMA
```

Crossover is empirical, not semantic.

### 15.6 Multiscale PALMA hierarchy

Coarse levels accelerate information propagation; fine PALMA remains authoritative. Structural SimThing hierarchy may be reused only where its geometry genuinely supplies lawful restriction/prolongation.

### 15.7 Shared PALMA profiles

Do not solve one full field per actor. Batch by admitted shared theater/topology/impedance/opportunity profile. Actor-specific EML may select among a finite admitted set of solved profiles; it may not algebraically synthesize an exact actor-specific optimum from fields that do not support that identity.

### 15.8 EXP/LN can reduce profile explosion — carefully

A smooth family of actor preferences can be mapped to a finite set of field-profile classes using EML/softmax-style weighting. The actual route still comes from an authoritative solved profile. This is a research performance strategy, not proof that a nonlinear family of shortest paths is linearly composable.

### 15.9 Gu-Yang → PALMA dirty-front coupling

```text
Gu-Yang stall/flux changes
        ↓
EML/overlay changes PALMA W locally
        ↓
known PALMA dependency tile becomes dirty
        ↓
FIM / multiscale FIM repairs only affected value region
```

This makes live congestion/capacity feedback affordable without actor-side search.

### 15.10 Operator-split generation posture

Candidate physical phase decomposition:

```text
1. snapshot previous plane where admitted
2. local overlay EML / parameter reaction
3. RF + STEAD + Gu-Yang native execution
4. PALMA dense/FIM/multiscale solve/repair
5. ActionBand target/progress evaluation
6. existing sealed crossings + EML
7. CostBand quantization / RF commitments
8. OverlayStateNext activation/update/lifecycle
9. barrier / swap
```

Exact OrderBand placement remains engineering/admission work; only dependency direction is asserted.

---

## 16. What is explicitly not proposed

1. No fifth StemThing leg.
2. No `ActionThing` peer entity or action service.
3. No overlay manager/service owning lifetime beside SimThing.
4. No CPU per-generation overlay numerical evaluator as final authority.
5. No human-readable overlay/action noun as GPU dispatch key.
6. No second threshold/crossing machine.
7. No A* / predecessor / path-object simulation authority.
8. No ActionBand subgroup-search semantics.
9. No claim all properties are RF resources.
10. No overlay-private PALMA, Gu-Yang, congestion, or throughput model.
11. No overlay mutation of FieldSweep structural/certificate metadata.
12. No hidden Gu-Yang stall history.
13. No coarse PALMA result overruling fine authority.
14. No multiscale Gu-Yang acceleration if it changes physical propagation speed.
15. No Ng-style potential transform applied to Gu-Yang as if flux were a shortest-path objective.
16. No `EXP(SUM(LN()))` rewrite silently replacing ordered Product semantics.
17. No dynamic GPU template/EML authoring.
18. No same-generation recursive convergence.
19. No assumption FIM always beats dense PALMA.
20. No assumption structural hierarchy is automatically a valid numerical multigrid hierarchy.

---

## 17. Candidate full StemThing germ

```text
StemThing
│
├─ ordinary sparse Property state
├─ recursive structural parent/children
├─ RF participation bindings
├─ intrinsic Field-Triad participation
│   ├─ STEAD bindings
│   ├─ PALMA profile/bindings
│   └─ Gu-Yang conservative bindings
├─ intrinsic ActionBand facility (normally inert)
└─ intrinsic Overlay facility (normally inert)
    ├─ admitted overlay templates
    ├─ sparse active instances
    ├─ receive / route / filter
    ├─ project to Property / RF / Field-Triad parameters
    ├─ full admitted EML response
    ├─ activate / suspend / parameterize
    ├─ lifecycle via ordinary anchored/crossing state
    ├─ dirty native dependency spans
    └─ dissolve / collapse
```

Physical GPU state may be entirely out-of-line/sparse while semantic ownership remains intrinsic.

### 17.1 Recursive simulation identity

```text
state
  ↓
RF + Field Triad
  ↓
opportunity / route / conserved feasibility
  ↓
ActionBand discrepancy
  ↓
CostBand executable work
  ↓
Overlay actuation
  ↓
state
```

Specialized domains are authoring over properties, EML, RF lanes, field registrations, targets, bands, and overlays — not new runtime engines.

---

## 18. Candidate laws for eventual promotion

### 18.1 Intrinsic Overlay Ownership Law

> Every active overlay is semantically owned by a StemThing. No peer subsystem owns overlay lifetime or numerical execution.

### 18.2 Overlay Actuation Law

> Ordinary numerical action resolves by activating, parameterizing, modifying, suspending, or dissolving an admitted overlay whose effects are realized through native Property/RF/Field-Triad/boundary surfaces.

### 18.3 Sparse Inert Germ Law

> Overlay capability is universal in semantics but sparse in physical instantiation. An inactive StemThing incurs no hot overlay scan or mandatory per-instance state.

### 18.4 Native Conservation Distinction Law

> Overlay may parameterize RF and Gu-Yang, but only genuinely conserved/constrained quantities use their conservation semantics. Overlay may not bypass or duplicate those laws.

### 18.5 Full Field-Triad Parameterization Law

> Overlay may parameterize STEAD, PALMA, and Gu-Yang only through admitted numerical inputs/bindings while each native mechanism retains its conservation/topology/stability authority.

### 18.6 Field-Parameter Certificate Law

> Runtime overlay variation may not mutate canonical adjacency/order, conservation/symmetry certificates, χ/stability certificates, or field-law identity; parameterization remains within admitted certified envelopes.

### 18.7 Field-Before-Route Law

> Semantics that materially change route choice should, where practical, parameterize the authoritative PALMA problem before solve rather than create a private post-hoc ActionBand correction.

### 18.8 Gu-Yang Instantaneous Law

> Gu-Yang owns instantaneous conserved flux/saturation. Persistent stall memory is explicit ordinary/overlay state under admitted EML, never hidden Gu-Yang or listener state.

### 18.9 Full EML Actuation Law

> Overlay numerical behavior uses the same admitted EML language, exact arithmetic, primitive-domain, and bounded-feedback laws as every other consumer. No overlay-specific numerical dialect exists.

### 18.10 Overlay Lifecycle Authority Law

> Activation, suspension, temporal state, and dissolution are intrinsic overlay capability. CPU may remain semantic/persistence/structural shadow but may not be the ordinary numerical lifecycle evaluator after GPU-resident lifecycle is admitted.

### 18.11 Generation-Paced Actuation Law

> Action authorized in generation `t` changes overlay next-state; the resulting overlay participates no earlier than the next ordinary generation/barrier.

### 18.12 Opportunity-Horizon Law

> Nonlocal opportunity awareness is a native field-propagation responsibility, not ActionBand search depth.

### 18.13 Gu-Yang Opportunity-Surface Law — candidate

> Signed Gu-Yang flux/stall/contest/choke may be consumed as native feasibility/opportunity observables and may parameterize PALMA/RF through admitted overlay EML; ActionBand may not reinterpret flux magnitude as progress without preserving native sign/target orientation.

### 18.14 PALMA Physical-Lowering Law

> Dense relaxation, tiled FIM, multiscale FIM, and future lawful accelerators are physical PALMA lowerings only if they preserve the admitted fine-field authority and deterministic semantics.

### 18.15 Dirty-Provenance Law

> An overlay transition should directly seed native RF/Field-Triad work using admitted dependency spans rather than require a global change-detection scan.

### 18.16 No Algebraic Semantic Substitution Law

> Algebraic relation does not authorize silent semantic substitution: Product is not LogAccumulate, Gu-Yang flux is not potential-shaped PALMA, and finite-rate transport is not equilibrium multigrid merely because a related transformation is mathematically convenient.

---

## 19. Falsifiers / remand conditions

Promotion should fail or remand if implementation requires any of the following:

1. CPU behavior/action/event dispatcher to interpret routine ActionBand/CostBand results.
2. Domain execution code because a routine action cannot be expressed through admitted overlay/property/RF/Field-Triad/boundary consequences.
3. Per-SimThing hot overlay object/state for inactive SimThings.
4. Continuous CPU mirrors for ordinary overlay lifecycle after GPU authority exists.
5. Runtime creation of new shader/EML/template semantics rather than activation of admitted data.
6. Same-generation recursive overlay/action convergence.
7. Second threshold/crossing detector.
8. Overlay bypass of RF conservation for a conserved resource.
9. Private ActionBand/overlay throughput/congestion/saturation model beside Gu-Yang/RF.
10. Runtime overlay modification of Gu-Yang/PALMA structural proof metadata.
11. Hidden stall history inside Gu-Yang or a saturation listener.
12. FIM/multiscale changes the authoritative PALMA result rather than solve posture.
13. Multiscale Gu-Yang changes intended physical propagation speed.
14. `abs(flux)` or other sign-erasing binding counts opposed Gu-Yang flow as positive target progress.
15. Potential/gauge transformation of Gu-Yang is treated as invariant without a conservation proof.
16. Log-domain accumulation silently replaces an existing ordered Product law.
17. Human-readable domain nouns become runtime behavior branches in core/kernel/GPU code.
18. A new overlay manager becomes the true owner while SimThing storage is ceremonial.

---

## 20. Engineering review questions

### 20.1 Overlay instance storage / capacity

- exact template/instance/binding/dependency table layout;
- maximum active overlay capacity and residency accounting;
- global packing vs owner/template/profile grouping;
- SlotIndex epoch remap behavior.

### 20.2 Lifecycle lowering

- mapping current `DissolveCondition` vocabulary to GPU EML/threshold registrations;
- `AfterTicks` representation without CPU decrement loop;
- explicit dissolve/remove/override semantics;
- activation/suspension current/next representation;
- durable lifecycle readback requirements.

### 20.3 Routing representation

- standing inheritance without per-descendant duplication;
- routed origin→LCA→target filter lowering;
- precompiled epoch route spans vs parent-table derivation;
- conjunctive policy predicate composition distinct from sequential value transform semantics.

### 20.4 Full-Field-Triad overlay bindings

- exact binding shape into STEAD/PALMA/Gu-Yang map/program parameters;
- certified runtime parameter envelopes;
- whether currently materialized Gu-Yang outputs expose all desired signed/gross/net/stall values;
- one authoritative conserved-progress bound source per binding where ActionBand/CostBand consumes flow.

### 20.5 Gu-Yang opportunity-surface reuse probe

- can target-aligned signed edge flux be bound directly as local feasibility/progress input?
- does landed Gu-Yang possess a node pressure/driving/dual-like scalar?
- if so, can that scalar seed/warm-start/bound PALMA without duplicate solve?
- can Gu-Yang stall/contest/choke parameterize PALMA W with zero duplicate field calculation?
- can Gu-Yang dirty frontier and PALMA FIM dirty frontier share dependency provenance?

### 20.6 Overlay-to-field dirty provenance

- dependency granularity: slot/cell/tile/channel/profile;
- deterministic duplicate activation handling;
- current/next masks;
- dense/FIM crossover;
- conservative-theater scheduling for Gu-Yang without changing physical propagation.

### 20.7 Multiscale hierarchy

- lawful PALMA restriction/prolongation operators;
- obstacle/chokepoint preservation;
- whether atlas hierarchy can be reused;
- for Gu-Yang, classify each candidate registration as equilibrium/fixed-point vs finite-rate evolution before considering multilevel acceleration.

### 20.8 Full EML actuation

- EXP/LN primitive availability on the target branch/rung at implementation time;
- domain-certified vs explicitly guarded call sites;
- bounded-feedback admission for field-seeded recurrent overlay emissions;
- JIT/program bucketing for logistic/decay/power/softmax shapes;
- log-domain authored laws where semantically intended.

### 20.9 Shared PALMA profile cardinality

- profile-key definition;
- actor-specific vs shared overlay deformations;
- finite profile class selection with EML;
- profile field budget/residency accounting.

### 20.10 Structural consequences

- which numerical overlays remain purely numerical;
- which resolve to sealed `BoundaryRequest`;
- pending structural request + overlay lifecycle replay semantics.

---

## 21. Suggested probe sequence before promotion

This is not a workplan amendment.

```text
A. OVERLAY-GERM-ARCHAEOLOGY
   enumerate every attach/activate/suspend/dissolve/apply/override path
   classify semantic duplicate vs genuinely structural

B. GPU-OVERLAY-LIFECYCLE-PROBE
   one admitted transient overlay
   GPU owns active/current/next + threshold/AfterTicks-equivalent lifecycle
   CPU receives sparse semantic delta only

C. ACTIONBAND→OVERLAY-PROBE
   existing sealed ActionBand crossing
   → fixed admitted OverlayStateNext activation/update
   → next-generation ordinary RF/Field-Triad consequence

D. FULL-TRIAD-OVERLAY-BINDING-PROBE
   one overlay parameterizes STEAD, PALMA W, and Gu-Yang input through admitted bindings
   prove no private solver and no certificate mutation

E. GU-YANG-OPPORTUNITY-SURFACE-PROBE
   signed opposed demand + target-aligned flux binding
   inspect whether driving/dual-like scalar exists and is reusable
   abs(flux) mutant RED

F. OVERLAY-DIRTY-TRIAD-PROBE
   overlay transition directly dirties only affected STEAD/PALMA/Gu-Yang dependency spans
   no global change scan

G. OVERLAY-DIRTY-PALMA-FIM-PROBE
   localized impedance change
   → exact tile seed
   → warm-start tiled FIM
   compare with dense PALMA oracle

H. DENSE↔FIM-CROSSOVER-PROBE
   cold / 1% / 10% / 50% / 100% dirty theaters
   derive physical crossover only

I. MULTISCALE-FIM-PROBE
   same fine PALMA oracle
   V-cycle acceleration
   no coarse behavioral authority

J. GU-YANG-MULTILEVEL-DISPOSITION
   classify finite-rate vs equilibrium semantics
   only equilibrium-like candidate may proceed to conservative multilevel proof

K. FULL-EML-OVERLAY-PROBE
   ContinuousDecay + Logistic stall-response
   Softmax RF weighting
   PowerLaw / positive log-domain parameter case
   prove primitive domain + bounded-feedback admission

L. FRACTAL-ROUTING-PROBE
   standing + routed + conjunctive policy behavior
   GPU table lowering across multi-depth tree

M. PROMOTION REVIEW
   retire peer lifecycle/action authority only after equivalence and falsifiers are proven
```

---

## 22. Promotion criteria

The workshop is ready to move to `docs/` only when:

1. Owner/DA adjudicates overlay as intrinsic StemThing actuation state.
2. GPU numerical authority vs CPU semantic/structural shadow is explicit.
3. Attach/receive/route/filter/activate/suspend/dissolve/remove/override/collapse have one lawful home.
4. RF and Gu-Yang conservation boundaries cannot be bypassed.
5. Full Field-Triad parameterization is explicit and certificate-safe.
6. Full EML/EXP/LN behavior is governed by the one EML language and bounded-feedback admission.
7. ActionBand integration uses the one existing crossing/emission surface.
8. Default inertness is physically credible.
9. Recursive routing is bounded/deterministic.
10. FIM is correctly scoped as PALMA physical optimization.
11. Multiscale PALMA retains fine authority.
12. Gu-Yang multilevel work is explicitly classified by semantic propagation law before optimization.
13. Gu-Yang opportunity-surface reuse has an engineering disposition, including dual/driving-potential archaeology.
14. Overlay dirty provenance covers all native RF/Field-Triad dependencies.
15. Current feeder/sim/kernel overlay authorities have keep/migrate/delete dispositions.
16. All falsifiers remain green.
17. Exact 0.0.8.7 amendment/consumer point is known.

---

## 23. Engineering-review synthesis

The strongest current candidate is:

> **StemThing is a default-inert recursive Wei-style automaton whose state is ordinary properties plus native RF/Field-Triad participation; whose `act` leg is ActionBand plus CostBand; whose `originate` and `receive` legs are one intrinsic overlay capability; and whose nonlocal opportunity, routing, and conserved-feasibility information is supplied by STEAD/PALMA/Gu-Yang rather than by actor-owned planners. Full EML is the singular programmable numerical law that shapes overlay intensity, memory, weighting, and field parameterization.**

The intended GPU posture is correspondingly uniform:

```text
local overlay / EML reaction
        ↓
RF + STEAD + Gu-Yang native execution
        ↓
PALMA min-plus value solve / repair
        ↓
sparse ActionBand / CostBand threshold response
        ↓
OverlayStateNext
        ↓
generation swap
```

FIM is included because it can turn PALMA from repeated full-field recomputation into incremental causal field maintenance driven by exact overlay dirty provenance while retaining massively parallel local relaxations. Multiscale FIM is included because it can expand effective information horizon by accelerating the same authoritative PALMA problem rather than giving ActionBand search depth. Ng–Harada–Russell is retained at the level actually supported: it motivates caution around local shaping and suggests exact potential/gauge reparameterization as a PALMA preconditioning research path, not a Gu-Yang flux transform. Gu-Yang contributes not only throughput limits but potentially already-paid opportunity information through signed realized flux, stall/contest/choke, and — if archaeology proves it exists — a reusable driving/dual-like scalar. `EXP`/`LN` complete the feedback loop by enabling deterministic decay, bounded logistic response, soft weighting, entropy, power laws, and positive log-domain parameterization without domain kernels.

If this closure survives engineering scrutiny, the recursive SimThing family **is** the simulation: state produces native fields and conserved flow; those fields expose opportunities, route/value, and feasibility; discrepancies consume constrained means; resolved work emits intrinsic actuation; actuation parameterizes the next native field/RF generation; and the same rule repeats everywhere.
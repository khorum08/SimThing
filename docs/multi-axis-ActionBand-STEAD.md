# Multi-Axis ActionBand + STEAD
## Intrinsic recursive event execution over STEAD value fields, PALMA routes, RF, EML, and CostBand

> **Status: OWNER-RULED DESIGN CANDIDATE — FOR FABLE 5 MAX + ENGINEERING REVIEW.**
>
> Owner ruling, 2026-08-08: **ActionBand is a first-class intrinsic SimThing facility.** It is part of the base recursive stem-cell definition, inert by default, and is the generic facility by which a SimThing listens to recursively produced state, compares that state with desired conditions, follows a lawful STEAD/PALMA route toward resolution, and emits executable consequences as authored bands are crossed.
>
> **Execution authority is GPU-only.** ActionBand numerical state and execution live entirely on the GPU. The CPU carries only the semantic shadow needed for human-readable designation, durable identity/persistence, diagnostics/presentation, and existing structural-boundary work. A CPU ActionBand evaluator, planner, scheduler, continuous mirror, or semantic decision path is forbidden.
>
> This document is intended to become the governing ActionBand anchor after DA review/admission. It does **not** by itself edit the 0.0.8.7 ladder or close the separate rollback/reconciliation ceremony required for already-graduated movement/7.1a work.
>
> The prior workshop disposition that attempted to infer ActionBand from already-landed domain witnesses is not the governing architectural premise of this document. The direction is inverted:
>
> **ActionBand is intrinsic. Domain behaviors are derived/vendorized uses of ActionBand.**
>
> In particular, physical movement is a useful spatial exemplar but never a peer core facility or the source from which ActionBand is generalized.

---

## 0. Executive definition

An **ActionBand** is an intrinsic, normally inert facility on every SimThing that represents an unresolved transition between:

1. the SimThing's **current coordinate** on one or more admitted STEAD value axes; and
2. a **desired target point, interval, region, or condition set** on those same observables.

The ActionBand derives the displacement between current and target state, observes whether that displacement is improving or worsening, evaluates the **stakes** of leaving the displacement unresolved, and uses admitted EML plus PALMA where topology exists to determine lawful progress toward the target.

Authored **bands** are threshold surfaces over ActionBand observables. Crossing a band emits an optional EML payload. That payload may:

- observe/check ordinary state;
- originate or modify ordinary overlays/directives;
- make ordinary RF claims;
- consume/transfer/reserve through the resource's existing semantics;
- invoke CostBand work/progress;
- authorize ordinary state changes;
- request an existing boundary-owned structural consequence; or
- activate one or more subordinate ActionBands on the **same intrinsic SimThing facility**.

Most ActionBands are expected to be depth 1 or depth 2. Richer multiband and recursively nested forms are lawful, but complexity is pay-for-play.

The complete conceptual loop is:

```text
recursive SimThing / RF state
          ↓
  resultant STEAD axes
          ↓
 current coordinate X(t)
          │
          │ compare against
          ▼
 target set G(t)
          ↓
 displacement D(t)
 velocity dD/dt
 stakes Σ(t)
          ↓
 EML valuation / deficiency resolution
          ↓
 PALMA-informed route where topology exists
          ↓
      ActionBand progress
          ↓
 authored band crossing
          ↓
 optional EML payload
          ↓
 existing RF / CostBand / overlay / boundary facilities
          ↓
 partial or terminal consequence
          ↓
 new ordinary SimThing state
          ↓
 next-generation recursive state
```

Everything through ActionBand evaluation, crossing, recursive subordinate evaluation, and numerical consequence authorization is GPU-authoritative. CPU interaction occurs only when a sparse semantic/structural boundary delta must be remembered, presented, persisted, or applied through an already-existing CPU-owned structural boundary.

An ActionBand is therefore not merely a selector in front of execution. **It is the SimThing's intrinsic event/action execution trajectory from unresolved condition to resolved condition.**

---

# 1. Constitutional placement: ActionBand is the SimThing `act` facility

The existing StemThing thesis remains unchanged: the base recursive SimThing carries all generic capabilities inert-by-default, and domain specialization activates or authors those capabilities rather than introducing peer engines.

ActionBand belongs inside that rule.

It is **not**:

- a fifth StemThing leg;
- an `ActionThing` entity type;
- an event manager;
- a behavior tree;
- a task graph;
- a destination manager;
- a movement engine;
- a combat engine;
- a production engine;
- a planner service;
- an administrative service;
- a CPU goal selector; or
- an independent lifecycle authority.

It is the intrinsic implementation of the SimThing's ability to **act on what it perceives**.

The base SimThing remains the sole point of iteration and failure. Recursive ActionBands never escape that ownership. A subordinate ActionBand is not another engine object with its own scheduler or authority; it is a recursively activated portion of the same ActionBand facility attached to the same SimThing.

This gives the system one failure surface:

```text
SimThing
 ├─ participate
 ├─ act  ← ActionBand
 ├─ originate
 └─ receive
```

ActionBand composes the already-existing mechanisms behind those legs rather than replacing them.

## 1.1 GPU numerical authority / CPU semantic shadow

ActionBand is **implemented entirely in the GPU numerical regime**.

The GPU owns, evaluates, and advances all ActionBand numerical authority, including as applicable:

```text
current coordinate / observed bindings
target numeric bindings
resultant STEAD inputs
displacement
velocity / delta-to-target
stakes
band thresholds and crossing state
EML payload execution
PALMA-derived route/impedance inputs
progress / CostBand interaction
recursive/subordinate ActionBand dependency state
RF claim quantities and numerical commitment state
terminal satisfaction / dissolve eligibility
```

No CPU copy of those values is authoritative, and no CPU loop is permitted to decide what an ActionBand wants, whether a band crossed, which subordinate discrepancy is active, or how far an ActionBand progresses.

The CPU owns a **semantic shadow**, not a second executor. That shadow may contain:

```text
stable logical identity
human-readable names/designations
authoring/debug/UI metadata
persistent categorical history
mapping from opaque GPU identities to semantic labels
sparse band/terminal/structural deltas after GPU execution
existing structural-boundary requests and their recorded application
```

Human-readable domain names exist only in that CPU semantic shadow / authoring-presentation layer. GPU ActionBand programs and descriptors operate on sealed numeric/logical bindings, opaque ids, offsets, ranges, thresholds, and admitted EML programs; they do not branch on strings, domain nouns, or human-readable action names.

The constitutional rule is:

> **GPU computes and decides ActionBand numerical state. CPU remembers and names the semantic consequences.**

A CPU semantic shadow may say that opaque ActionBand/program identity `317` corresponds in an authored scenario to “secure food supply” or “reach Orion IV.” That designation has no numerical authority and must never become a CPU-side dispatch key that selects behavior.

A structural consequence remains lawful when the GPU emits the sealed request/delta and the existing CPU boundary performs the mutation. The boundary executes structure; it does not re-evaluate the ActionBand decision.

This preserves the broader SimThing law:

> **Numerical state remains GPU-resident until a semantic or structural boundary crossing. CPU workload scales with semantic change rate, not SimThing count × generation count.**

---

# 2. The semantic coordinate model: Stellaris-style axes generalized

The useful lesson from the Stellaris ethics web is geometric, not domain-specific.

Stellaris presents several opposed value axes:

```text
Authoritarian  ←→ Egalitarian
Militarist     ←→ Pacifist
Xenophobe      ←→ Xenophile
Spiritualist   ←→ Materialist
```

The important abstraction is:

> **Complex state can be located as coordinates on a small number of logically chosen value axes rather than encoded as a catalog of named composite states.**

SimThing generalizes this to arbitrary admitted semantic/value axes.

Examples in a city simulation might include:

```text
scarcity        ←→ abundance
threat          ←→ safety
isolation       ←→ accessibility
degradation     ←→ physical quality
job insecurity  ←→ job security
housing pressure←→ housing abundance
```

Other domains may author different axes.

The engine does not know what these names mean. It knows admitted property/field bindings and the EML/STEAD laws that generate them. Those human-readable axis names are CPU semantic-shadow/authoring designations; the GPU sees only their admitted numeric bindings.

## 2.1 Resultant bipolar scalar law

Where an axis is truly bipolar, it is one bounded degree of freedom:

\[
x_k \in [L_k,U_k]
\]

with opposite semantic poles at the bounds or ends of the admitted range.

Ordinary state, falloff influence, policy/personality overlays, transient conditions, and other admitted transformations nudge the value toward one pole or the other.

Conceptually:

\[
x'_k = \operatorname{Bound}_k\left(x_k + \sum_i \Delta_{i,k}\right)
\]

or the corresponding admitted EML/field reduction law.

**The resultant is authoritative.**

If equal and opposite influences produce:

\[
+0.7 - 0.7 = 0
\]

then the receiving SimThing experiences that axis as `0`.

There is no intrinsic obligation to preserve hidden contest magnitude merely because strong opposing emitters happened to cancel. If the distinction between “quiet neutrality” and “strong opposed pressures whose resultant is zero” matters to a model, that distinction must be authored as another observable/axis rather than smuggled into the bipolar scalar.

This is deliberate. A SimThing can be neutral on one ActionBand while strongly displaced on another:

```text
border disposition = 0
trade imbalance    = +0.8
food security      = -0.5
job security       = -0.3
```

No universal equilibrium is implied.

### 2.1.1 This is not RF conservation

A bipolar semantic coordinate is **not automatically a conserved RF resource**.

“Moving toward one pole moves away from the other” describes one signed/bounded degree of freedom. It does not imply:

\[
\sum_s x_{s,k}=constant
\]

Actual RF conservation remains governed by the resource substrate. Semantic axes and conserved resources may interact, but they remain distinct authorities.

## 2.2 Strong axes, emergent semantics

Primitive axes should carry independent causal information. Higher-order named phenomena should normally emerge from combinations and trajectories through those axes.

For example, a city may author primitive values such as:

```text
housing pressure
housing condition
rent/land price
employment opportunity
labor availability
capital availability
consumer demand
material access
safety
pollution/environmental quality
education access
health access
amenity access
passenger accessibility
freight accessibility
network congestion
```

while terms such as:

```text
gentrification
blight
prosperity
food desert
suburbanization
commercial vitality
industrial decline
```

remain derived observations unless they contain genuinely independent causal state.

The same law applies outside cities.

---

# 3. Recursive SimThing state produces STEAD coordinates

A SimThing does not receive semantic labels by fiat. Its current coordinates arise from ordinary recursive state.

For SimThing `s`, define its recursively resolved state schematically as:

```text
R_s = {
    own ordinary properties,
    child reduce-up results,
    inherited/directive state,
    overlays,
    RF balances and unresolved pressure,
    topology observations,
    existing commitment/progress state,
    ordinary anchored observables
}
```

An admitted EML projection may produce one or more semantic coordinates:

\[
X_s(t) = [x_{s,0}(t),x_{s,1}(t),...,x_{s,K-1}(t)]
\]

with:

\[
x_{s,k}(t) = F_k(R_s(t))
\]

The coordinate is therefore a **projection of causal state**, not an independently authored domain label.

Because SimThings are recursive, this works at every scale:

```text
cohort
  ↓
residency
  ↓
district
  ↓
city
  ↓
region
```

or any other domain hierarchy.

## 3.1 STEAD propagation

A coordinate can become a STEAD field channel over admitted topology:

\[
\Phi(x,k)
\]

For emitter `e`, channel `k`, and locus `x`:

\[
\Phi_k(x)=\operatorname{Reduce}_{e\in E_k}
\left(A_{e,k}\,f_{e,k}(d(e,x))\right)
\]

where:

- `A_{e,k}` is the emitter amplitude/current coordinate contribution;
- `f` is the admitted propagation/falloff law;
- `d` is distance/impedance on the relevant topology; and
- `Reduce` is the admitted non-conserved field reduction law.

Different axes may use different topologies, falloff laws, cadences, and reductions. “One STEAD field” is conceptual; physically it may be several channel arrays/sweeps.

## 3.2 Derived compound fields

Frequently reused combinations may be cached as derived fields, but those caches are lowerings, not new semantic authorities.

For example:

```text
housing attractiveness
commercial investment pressure
healthcare accessibility
```

may be derived from primitive channels and cached when performance warrants it.

The authoritative meaning remains the primitive inputs plus admitted EML.

---

# 4. ActionBand target semantics

An ActionBand exists because a SimThing's current state differs from a desired state.

Let the current coordinate relevant to ActionBand `b` be:

\[
X_{s,b}(t)
\]

and let its desired target be a set:

\[
\mathcal G_{s,b}(t)
\]

The target may be:

- an exact point;
- a scalar threshold;
- an interval;
- a multidimensional region;
- an arrival locus/radius;
- an admitted predicate-defined condition set; or
- another EML-defined acceptable region.

Exact-point targets are therefore only one case.

Examples:

```text
food_security >= 0.6
housing_pressure in [-0.1, +0.1]
build_progress >= 1.0
fleet within arrival radius of Orion IV
capability.psionic_navigator >= 1
```

These names are explanatory CPU-shadow labels. The GPU representation is the corresponding admitted numeric target/property/topology binding.

## 4.1 Where targets come from

ActionBand owns no separate target manager.

A target may arise from ordinary SimThing state and authoring such as:

```text
baseline disposition / personality
standing conditions
scripted state
time
incoming directives
overlays
resource needs
deficits
explicit authored goals
```

These influences may establish or deform the desired state without becoming a privileged command channel.

The target itself may move over time:

\[
\mathcal G(t)
\]

which naturally represents changing priorities, orders, needs, prices, moving destinations, or evolving conditions without a retarget state machine.

## 4.2 Displacement

For point targets:

\[
D(t)=G(t)-X(t)
\]

For a target set, define the nearest lawful target projection conceptually as:

\[
D(t)=\Pi_{\mathcal G(t)}(X(t))-X(t)
\]

where `Π` is the admitted projection/distance-to-target operation appropriate to the axis/topology.

The exact norm is authored/domain-dependent. The core law is simply:

> **ActionBand acts on unresolved displacement between present state and an admitted desired state.**

## 4.3 Velocity

Velocity is useful ActionBand data:

\[
V_X(t)=X(t)-X(t-1)
\]

and, often more importantly:

\[
V_D(t)=D(t)-D(t-1)
\]

which indicates whether the discrepancy is closing or worsening.

Velocity should preferentially derive from already-available current/previous generation GPU planes rather than introduce an ActionBand-specific state handler. If longer history is required, that history remains ordinary SimThing state; only semantically durable history need cross into CPU shadow.

---

# 5. Stakes: consequence and urgency of unresolved displacement

**Displacement is not stakes.**

Displacement answers:

> How far is the current state from the desired state?

Stakes answer:

> How consequential or urgent is it that this displacement remain unresolved?

Let:

\[
\Sigma_{s,b}(t)=S_b\left(D, V_D, R_s, \Phi, overlays, reserves, deficits, history, ...\right)
\]

where `S_b` is ordinary admitted EML executed in the GPU numerical regime.

The default conceptual relationship may be thought of as:

\[
P_b \sim \Sigma_b \cdot \|D_b\|
\]

but this is **not** a mandated formula. EML may define richer or nonlinear relationships.

Examples:

- a large food-quality displacement with deep food reserves may have modest stakes;
- a smaller food-security displacement with zero reserves may have extreme stakes;
- an ideological disagreement can be large but low-stakes until other conditions make it consequential;
- a slowly worsening discrepancy may remain below threshold while the same discrepancy with sharply negative velocity crosses an escalation band.

Therefore:

> **Displacement supplies tension. Stakes supply urgency/consequence. Bands turn meaningful tension, urgency, or progress into emissions.**

Stakes themselves are ordinary GPU-resident ActionBand observables and may be used as band operands.

---

# 6. PALMA supplies the route where a route exists

ActionBand knows a current condition and a target condition. The means of reducing the displacement may require navigating an admitted topology.

Where that topology exists, PALMA provides the lawful route/impedance structure.

For physical movement this is intuitive:

```text
current spatial locus
    ↓
STEAD/PALMA impedance field
    ↓
target spatial locus
```

But the same principle applies to:

- LinkGraph relations;
- trade/supply accessibility;
- service access;
- social or diplomatic relation graphs;
- recursive tree relations;
- admitted progression ladders; or
- other topology already present in the simulation.

ActionBand does not mint a generic `ActionGraph`.

## 6.1 Route degeneracy

Not every ActionBand needs a nontrivial topology.

A depth-1 capability check such as:

```text
psionic_navigator >= 1
```

has no reason to invent a graph. Its “route” degenerates to ordinary state progress/observation toward the target predicate.

Thus PALMA is used **where target resolution is topological**. Direct threshold/property ActionBands remain lawful base cases.

## 6.2 Local minima and adversarial navigation are deliberately fenced

The base ActionBand definition does not attempt to settle:

- local-minimum escape;
- adversarial multi-field navigation;
- starvation/livelock of competing routes;
- iterative same-generation convergence; or
- coordinated Vector CostBand clearing across several independently contested RF arenas.

Those are explicitly fenced to the future `VECTOR-COSTBAND-PROBE-0` / related work.

This fence is scope discipline, not doubt about ActionBand's existence.

---

# 7. Intrinsic ActionBand anatomy

The semantic facility should remain small.

Conceptually, an ActionBand requires only:

```text
ActionBand
│
├─ observation/current-state binding
├─ target point/set binding
├─ derived displacement / velocity / stakes
├─ authored band descriptors
├─ optional EML payload per band
├─ optional subordinate ActionBand bindings
└─ lifecycle / terminal condition
```

The runtime representation may be more compact than this conceptual form, and its production numerical representation is GPU-resident. Human-readable names for any of these bindings exist only in the CPU semantic shadow/authoring layer.

## 7.1 Inert by default

Every SimThing has the **capability** to host ActionBands, but no hot work is owed merely because the capability exists.

An inactive SimThing should pay essentially the same cost as an unregistered property/threshold listener.

ActionBands become active only when authored/derived registrations exist.

This is required for city-scale and population-scale use.

## 7.2 ActionBand as listener registration

A SimThing “listens” for actionable conditions by registering an ActionBand against already-produced observables.

Conceptually:

```text
watch:
    property / STEAD / derived binding

target:
    desired point / set / predicate

bands:
    authored thresholds

payload:
    optional EML per crossing

lifecycle:
    continue / terminal / dissolve condition
```

There is no separate event subscription manager.

The sweep already produces the observed values. ActionBand rides those values and emits only when a meaningful band crossing occurs. Registration and evaluation are GPU-resident after admission; CPU-facing designations are descriptive shadow only.

---

# 8. Band semantics

The word **Band** is intentional.

A band is an authored threshold surface over any admitted ActionBand observable, carrying an optional EML payload.

Possible band operands include:

```text
displacement magnitude
one axis coordinate
stakes
closing/worsening velocity
route distance
PALMA impedance
resource accumulation
CostBand progress
construction completion
elapsed time
another admitted derived value
```

The core does not require every ActionBand to normalize progress to `[0,1]` or divide trajectories into equal slices.

Band segmentation is authored semantic structure.

## 8.1 Default depth

Expected common shape:

```text
0 bands   → inert
1 band    → ordinary event/action trigger
2 bands   → trigger + completion, or warning + action
N bands   → richer authored progression/escalation/auditing
```

Most ActionBands should be depth 1 or depth 2.

This common case should receive an explicit fast path in the physical lowering if measurement justifies it.

## 8.2 Crossing semantics

The baseline ActionBand behavior is edge/crossing driven:

```text
below band
   ↓ crosses
emit once
   ↓ remains beyond threshold
no duplicate emission solely for remaining there
```

A later recrossing may emit again according to the authored lifecycle/hysteresis law.

This preserves the existing threshold/crossing discipline and prevents held conditions from generating duplicate actions every generation.

If an author needs periodic milestones, they author multiple threshold bands or an explicit progress/time observable rather than an implicit “fire every tick while true” mode.

Band crossing authority is GPU-only. The CPU may receive a sparse crossing delta after the fact; it may not perform the crossing test independently.

## 8.3 EML payload

Every band may carry a tight admitted EML payload.

That payload can be trivial or specialized.

Examples:

```text
band crossed
→ emit telemetry only
```

```text
band crossed
→ check several state prerequisites
→ if satisfied, authorize consequence
```

```text
band crossed
→ compute resource claims
→ route them through ordinary RF
```

```text
band crossed
→ activate subordinate ActionBands representing unresolved deficiencies
```

The default implementation may be a simple threshold emission. Specialized uses may invoke richer EML reductions without changing ActionBand's core type.

All such payload evaluation is GPU-side. Human-readable payload/program designations are CPU-shadow metadata, not execution keys.

---

# 9. ActionBand is the point of execution

ActionBand does not stop at “candidate commitment.”

It owns the lifecycle of the unresolved transition.

For a multiband ActionBand:

```text
current state
   ↓
route/progress
   ↓
band crossing
   ↓
partial action
   ↓
new state/progress
   ↓
next band
   ...
   ↓
terminal band
   ↓
final consequence
   ↓
ActionBand dissolves
```

Physical movement is a simple example:

- one band may represent the amount of lawful progress a SimThing can traverse in one generation;
- crossing that band emits the spatial consequence;
- repeated generations move the current state closer to the target;
- arrival crosses the terminal band;
- terminal payload emits the final event;
- ActionBand is removed.

Construction is the same shape with a different progress observable.

Food acquisition may be depth 1.

A policy/capability gate may be depth 1 and emit no physical motion at all.

The common facility is the target-seeking, band-emitting lifecycle—not a movement-specific operation.

The entire numerical lifecycle is GPU-authoritative. A terminal structural request may cross to the CPU boundary for application, but CPU boundary code does not decide whether the terminal band was reached.

---

# 10. Multisource action: inherited structure, native semantics

The earlier Capability Tree work remains useful archaeology because it already established the semantic shape of heterogeneous prerequisites.

The important inherited findings are:

1. one transition may depend on several independent conditions;
2. those conditions need not share semantics;
3. progress and eligibility are distinct;
4. completed/paid progress should not be repeatedly erased merely because another prerequisite remains unresolved; and
5. consequence activation occurs when the required conjunction finally becomes executable.

The ActionBand design therefore does **not** reopen the question “can an action require several kinds of things?”

That is settled in shape.

The remaining hard problem is quantitative lowering when several scarce contested RF lanes must clear together.

## 10.1 Do not duplicate RF/property semantics inside ActionBand

ActionBand should bind to the simulation's existing semantic authorities.

A bound ordinary state/property check remains an observation.

A bound conserved RF resource remains a conserved RF resource.

A sink remains a CostBand.

A residency/capacity lane retains its residency/capacity semantics.

A transfer retains transfer semantics.

Time remains ordinary state unless authored as a scarce resource.

ActionBand must not invent a parallel “requirement resource” universe.

The initial design should therefore prefer:

```text
ActionBand band EML
    ↓
reads ordinary prerequisite/property state
    ↓
uses ordinary RF claims where scarcity is real
    ↓
uses existing CostBand semantics for sinks/progress
```

rather than a new engine-wide requirement taxonomy.

These reads, claims, and CostBand progress decisions remain GPU-resident numerical operations.

## 10.2 Braking / held progress

The inherited capability behavior supplies the default multisource progression law:

> **When one unresolved requirement blocks the next ActionBand band, already-resolved progress remains resolved according to the native semantics of its underlying binding. The parent ActionBand brakes rather than rewinding completed work.**

Conceptually:

```text
work            satisfied
capability A    satisfied
capability B    unresolved
energy          partially satisfied

→ parent cannot cross next executable band
→ satisfied state does not reset merely because another requirement remains open
```

Whether a scarce grant may remain held across generations, and under what fairness/livelock constraints, belongs to the later quantitative probe.

ActionBand semantics do not require every satisfied prerequisite to become a permanently siloed RF grant.

## 10.3 Multisource requirement as band payload

A depth-1 ActionBand may use its one band EML as the all-of gate.

Example:

```text
Goal: reach Shroud coordinate

band EML requires:
    psionic_navigator >= 1
    shroud_access >= 1
    required energy/fuel grant executable
```

`psionic_navigator` and `shroud_access` are ordinary capability/state observations.

Energy/fuel remains ordinary RF/CostBand.

The band fires only when the authored conjunction allows it.

This is multisource action without collapsing nonconserved prerequisites into RF.

---

# 11. Recursive ActionBands

ActionBand is structurally recursive because SimThing itself is structurally recursive.

An ActionBand band's EML payload may originate/register one or more **subordinate ActionBands** on the same SimThing's intrinsic ActionBand facility.

This is the generic mechanism for expressing a target whose unresolved deficiencies themselves require target-seeking action.

## 11.1 Parent unresolved-state model

Let parent ActionBand `P` have subordinate unresolved conditions with distances:

\[
u_i(t) \ge 0
\]

where:

\[
u_i(t)=0
\]

means the subordinate target is currently satisfied.

Then:

\[
U_P(t)=[u_0(t),u_1(t),...,u_n(t)]
\]

is an ordinary input to the parent's EML.

Parent progress may be:

\[
q_P(t)=F_P(D_P,V_{D_P},\Sigma_P,U_P,\Phi,R,...)
\]

The common all-of form is simply one authored case:

```text
if any u_i > 0:
    parent next band cannot execute
else:
    parent may advance
```

Other lawful EML compositions can represent substitution, quotas, weighted satisfaction, or other domain semantics without introducing ActionBand-specific boolean regimes.

All child state, parent reduction, and subordinate activation decisions are GPU-resident. The CPU shadow may name the parent/child relation for authoring or diagnostics but does not traverse or schedule it.

## 11.2 Nested discrepancies, not imperative tasks

This distinction is constitutional.

**Forbidden interpretation:**

```text
TaskNode {
    next_step
    retry_policy
    success_handler
    failure_handler
    child task scheduler
}
```

That is a behavior tree/planner wearing ActionBand vocabulary.

**Lawful interpretation:**

```text
Parent target discrepancy
  ├─ subordinate target discrepancy A
  ├─ subordinate target discrepancy B
  └─ subordinate target discrepancy C
```

There is no imperative “next.”

Current world state determines which discrepancies remain unresolved.

## 11.3 Recursion may be concurrent

Sibling subordinate ActionBands may progress independently in the same broader interval when their resources/topologies permit it.

A colonization target may simultaneously pursue:

```text
transport availability
population commitment
supply reserve
route/access
```

instead of hardcoding:

```text
A then B then C then D
```

The parent simply observes the unresolved vector.

This makes recursive ActionBand semantics naturally parallel and GPU-friendly.

## 11.4 Recurse and collapse

When a subordinate ActionBand reaches its terminal target:

```text
child terminal crossing
        ↓
ordinary consequence becomes world state
        ↓
child ActionBand dissolves
        ↓
parent later observes resulting ordinary state
```

Resolved child ActionBands do not become permanent “completed task” records merely to prove they once existed.

The world is the durable memory.

> **Resolved ActionBands collapse back into ordinary SimThing state.**

The parent itself collapses when its terminal target is satisfied and its terminal consequence has been emitted.

The CPU semantic shadow may retain a durable human-readable history if the design explicitly requires one, but that history is retrospective and non-authoritative; it cannot keep the completed ActionBand numerically alive.

## 11.5 Generation-paced recursion

Recursive ActionBands do **not** recursively execute to convergence inside one generation.

If a parent crossing emits a child registration in generation `t`, the child participates normally in the next admitted generation/barrier ordering.

Likewise, a child completion affects ordinary state; the parent observes that resolved state through the normal later-generation loop.

Forbidden:

```text
parent fires
→ spawn child
→ execute child immediately
→ spawn grandchild
→ execute grandchild immediately
→ converge in one dispatch
```

Required:

```text
generation t:
    parent crossing emits registration

barrier

generation t+1:
    child evaluates normally
```

This preserves determinism, bounds recursion, and keeps the Wei cellular-automaton cadence intact.

---

# 12. Multisource requirements and recursive ActionBands can collapse into one form

The recursive design gives an elegant unification.

A heterogeneous requirement may be represented in one of two physical ways while retaining one semantic model:

### Trivial requirement

If the condition has no independent lifecycle, it can lower directly into the parent band's EML:

```text
has_psionic_navigator >= 1
```

No child runtime object is owed.

### Stateful target-seeking requirement

If satisfying the condition requires its own target, STEAD interaction, resources, multiple bands, duration, or subordinate requirements, it may materialize as a subordinate ActionBand:

```text
Acquire Colony Transport
    target: required transport capacity available
```

Thus a complex parent can mix:

```text
inline capability checks
inline property predicates
native RF/CostBand requirements
materialized subordinate ActionBands
```

without a parallel planner or requirement engine.

All physically materialized subordinate ActionBands remain GPU-side numeric structures; the explanatory names above exist only in CPU shadow/authoring metadata.

## 12.1 Example: colonization

```text
COLONIZE ORION IV
│
│ target:
│   colony_state(OrionIV) = viable/established
│
├─ Acquire Colony Transport
│    target: transport_capacity >= required
│
├─ Establish Population Commitment
│    target: population_commitment >= required
│
├─ Stage Supplies
│    target: supply_reserve >= required
│
├─ Secure Access
│    target: route/access condition acceptable
│
└─ inline capability/state checks
```

The parent does not contain an authored procedural sequence.

It evaluates which deficiencies remain.

If transport, population, and supply can progress concurrently, they do.

When child consequences become ordinary state, the parent naturally sees a shrinking unresolved vector.

When the terminal parent condition is satisfied, the parent terminal band emits the colonization consequence and collapses.

## 12.2 Semantic route discovery remains EML territory

Physical movement has an obvious route topology.

More complex goals such as colonization, supply-chain construction, institutional change, or production expansion may require EML to determine **which deficiencies must be reduced** and which subordinate ActionBands should become active.

The engine does not need to pre-author a universal planning graph.

ActionBand provides the recursive target/discrepancy/band facility. EML supplies the authored semantic reduction that determines what remains unresolved.

---

# 13. Events and directives are ActionBand emissions/inputs

ActionBand gives the base SimThing a concrete event-execution meaning.

An event is often simply:

> **a meaningful ActionBand band crossing emitted from ordinary changing state.**

A directive is ordinary received state/overlay that may:

- deform the target;
- alter stakes;
- change a band threshold;
- affect a band's EML payload; or
- activate/register a new ActionBand.

The engine does not need an independent event taxonomy to understand the domain meaning.

Example:

```text
food_security falls below threshold
        ↓
registered ActionBand crosses band
        ↓
EML payload evaluates ordinary state/RF
        ↓
food-seeking response becomes actionable
```

This is the missing “listen and act” loop of the recursive SimThing stem cell.

All event detection and directive-driven numerical deformation occurs on GPU. CPU-readable event names are semantic-shadow projections of opaque band/program identities after the GPU has emitted a crossing.

---

# 14. Existing facilities retain their authority

ActionBand is a composition facility. It must not absorb or duplicate the semantics of the mechanisms it uses.

| Mechanism | Authority |
|---|---|
| **STEAD** | recursively produced/shared field coordinates and propagation |
| **PALMA** | topology-aware potential/impedance routing where a route exists |
| **EML** | authored valuation, target/deficiency reduction, band payload logic |
| **RF** | resource claims, conserved quantities, constrained clearing/disbursement |
| **CostBand** | exact resource-sink/work quantization and carried remainder |
| **Overlay** | ordinary policy/directive/transient deformation |
| **Threshold/crossing substrate** | detecting meaningful band crossings |
| **Boundary authority** | structural mutation that numerical execution may authorize but never perform directly |
| **GPU ActionBand** | numerical lifecycle of unresolved target displacement and band-emitted execution |
| **CPU semantic shadow** | human-readable designation, durable identity/history, diagnostics/presentation, sparse boundary deltas; never numerical decision authority |

This separation is load-bearing.

## 14.1 ActionBand + CostBand

ActionBand is an application/generalization of the threshold-emission idea that CostBand made useful for action.

CostBand remains:

\[
N=\left\lfloor\frac{V}{C}\right\rfloor
\]

\[
R=V-NC
\]

ActionBand may use CostBand output/progress as one of its band observables.

A one-band event handler is often the depth-1 case of this same pattern.

ActionBand must not invent another sink mechanism.

## 14.2 Structural consequences

An ActionBand EML payload may authorize a structural consequence, but the existing boundary still owns the mutation.

ActionBand must never directly relocate rows, mint structural identity, mutate topology behind admission, or bypass sealed boundary requests.

The CPU boundary applies the GPU-authorized request; it does not reinterpret its human-readable designation or re-decide the underlying ActionBand result.

---

# 15. Movement is a derived/vendorized ActionBand implementation

Physical movement is useful because it makes the target/displacement/route model easy to visualize:

```text
current spatial coordinate
        ↓
PALMA impedance route
        ↓
target coordinate
        ↓
movement-progress bands
        ↓
partial spatial consequences
        ↓
arrival terminal band
```

But “movement” is not a peer core action facility.

A movement implementation is lawful only insofar as it is a derived use of ActionBand and existing spatial/STEAD/PALMA/boundary facilities.

The same ActionBand semantics also cover:

```text
get food
build a door
complete research
acquire a capability
repair a machine
satisfy a service need
establish a colony
move a fleet to Orion IV
```

No one example defines the general case. These domain phrases are explanatory CPU-shadow labels; production GPU execution remains domain-nameless.

---

# 16. Worked examples

## 16.1 Get food — likely depth 1

```text
current:
    food_security = -0.7

target:
    food_security >= +0.2

stakes:
    function of displacement, reserves, worsening velocity, local conditions

band:
    actionable food deficit threshold

payload:
    EML reads food/access/price state
    ordinary RF claims where necessary
    CostBand consumption where necessary

terminal:
    target condition reached
    ActionBand dissolves
```

## 16.2 Build a door — depth 1 or several authored milestones

```text
current:
    build_progress = 0

target:
    build_progress >= 1

bands:
    0.25 optional milestone
    0.50 optional milestone
    0.75 optional milestone
    1.00 terminal
```

Or the author may choose a single terminal band if intermediate state has no gameplay consequence.

Each band payload may consume ordinary work/material CostBands or emit ordinary state changes.

## 16.3 Fleet to Orion IV — spatial vendorization

```text
current:
    spatial locus A

target:
    arrival set around Orion IV

route:
    PALMA over admitted spatial topology

band:
    lawful progress distance for this generation

payload:
    movement CostBand / ordinary boundary movement consequence

terminal:
    arrival predicate
    terminal event
    dissolve ActionBand
```

## 16.4 Shroud traversal with multisource requirements

```text
current:
    not at target Shroud coordinate

target:
    target coordinate reached

one executable band requires:
    psionic_navigator >= 1
    shroud_access >= 1
    required energy/fuel executable
```

The two capability checks are ordinary state observations.

Energy/fuel uses ordinary RF/CostBand.

If prerequisites are not met, the ActionBand remains unresolved/braked.

If acquiring one prerequisite itself requires extended action, that prerequisite may become a subordinate ActionBand instead of an inline predicate.

## 16.5 Colonization — recursive ActionBand

```text
parent target:
    colony established

subordinate discrepancies:
    transport capacity
    population commitment
    supplies
    route/access

inline checks:
    legal/capability prerequisites
```

Subordinate ActionBands may progress concurrently. They collapse when their consequences become ordinary state. The parent crosses its terminal band once its EML reduction determines all required discrepancies are sufficiently resolved.

The labels in all examples are explanatory semantic-shadow names only. Their production form is sealed GPU numeric bindings/programs.

---

# 17. Physical lowering and performance law

The semantic model is recursive. The hot implementation must not be forced to perform recursive pointer traversal.

> **Recurse semantically; flatten physically.**

Admission/JIT/lowering may compile recursive ActionBand structure into packed, contiguous evaluation data so long as semantics, generation ordering, band crossing behavior, and provenance are preserved.

The physical execution target is GPU-only. CPU is not a fallback executor for uncommon ActionBand shapes.

## 17.1 Sparse registration

The desired scaling law is:

\[
O(\text{active ActionBand registrations})
\]

not:

\[
O(\text{all SimThings} \times \text{all possible actions})
\]

Inactive SimThings owe no generic behavior loop.

## 17.2 Ride already-hot sweeps

ActionBand should evaluate against values already produced by the ordinary reduce/disburse/STEAD/field passes whenever possible.

The design target is approximately:

```text
value becomes available on GPU
    ↓
registered band compare on GPU
    ↓
no crossing → no additional work
    ↓
crossing → compact GPU emission
```

rather than a second world scan or CPU readback/re-evaluation.

The phrase “free evaluation” means **piggybacking on already-required memory traffic and paying only sparse compare/emission cost**, not literal zero instructions.

## 17.3 Depth-1/2 fast path

Because most ActionBands are expected to be depth 1 or depth 2, physical layout should optimize that case.

A plausible GPU descriptor family may contain:

```text
opaque observed binding
target/threshold parameters
band count / inline small bands
EML program id or offset
lifecycle flags/state
optional child/dependency span
```

This is an engineering sketch, not a frozen ABI.

No human-readable designation is part of the hot descriptor.

## 17.4 Inline trivial children

A subordinate condition such as:

```text
has_psionic_navigator >= 1
```

should not require a materialized child ActionBand if admission can faithfully inline it into the parent's EML payload.

Only children with independent lifecycle—own target, progress, bands, RF interaction, topology, duration, or recursive dependencies—need materialization.

This gives three physical cost tiers under one semantic model:

```text
inline predicate
    ↓ cheapest

materialized depth-1 ActionBand
    ↓ sparse watcher

multiband / recursive ActionBand
    ↓ only when genuinely needed
```

All three tiers execute on GPU.

## 17.5 Flatten recursive hierarchy

Authoring may be recursive:

```text
parent
 ├─ child A
 ├─ child B
 └─ child C
```

Physical lowering may compile this to a flat GPU descriptor/program region with resolved indices/offsets and dependency inputs.

The physical representation must not become semantic authority.

No runtime recursive call stack and no CPU child scheduler is required or permitted by the design.

## 17.6 Batch by program/binding shape

Many ActionBands will share identical:

```text
EML program
band schedule
axis basis
binding layout
```

while differing only in values/targets.

The executor should be free to batch these by program/profile/binding shape for coherent GPU access.

## 17.7 GPU residency is absolute for ActionBand numerical state

ActionBand numerical execution is not merely “GPU preferred”; **GPU is the authority**.

Continuous and discrete numerical ActionBand state such as:

```text
displacement
velocity
progress
stakes
band crossing state
subordinate unresolved state
EML intermediate/result values
claim quantities
terminal eligibility
```

remains GPU-resident.

The CPU must not maintain a continuous mirror, duplicate crossing evaluator, task scheduler, goal selector, or fallback action executor.

CPU-visible deltas are sparse and semantic/structural:

```text
band crossing designation for UI/history when needed
terminal event designation
structural consequence request
persistent categorical transition
logical identity/lifecycle bookkeeping required by existing boundary law
```

Even these are records of GPU-produced authority, not CPU recomputation.

Human-readable ActionBand, band, target, event, requirement, and domain designations live only in CPU semantic shadow/authoring metadata. GPU-side execution uses opaque admitted ids/bindings and numerical programs.

## 17.8 Parallel subordinate evaluation

Independent subordinate ActionBands are naturally parallel.

Sibling discrepancies may evaluate concurrently, and a parent reduction consumes their resolved/unresolved states according to generation pacing.

No serial procedural task order is implied by recursion.

---

# 18. Determinism and lifecycle

ActionBand must inherit SimThing determinism laws.

- ordering may never come from physical row order;
- field values and EML are evaluated under the admitted arithmetic semantics;
- threshold crossings are sealed/recordable where the surrounding substrate requires it;
- structural consequences remain behind recorded boundary authority;
- no same-generation recursive convergence is introduced;
- unresolved ActionBands have explicit lifecycle/horizon semantics;
- completed ActionBands collapse and do not leak permanent task-state residue;
- CPU semantic-shadow labels may never affect numerical ordering, target choice, crossing, or consequence selection.

A moving target or changing overlay is not an exception. It simply changes ordinary GPU input state for the next generation's displacement evaluation.

---

# 19. Candidate binding laws for DA review

The following are proposed as the normative ActionBand laws.

## 19.1 Intrinsic Action Law

> Every SimThing possesses the inert-by-default capability to host ActionBands. ActionBand is the base SimThing event/action execution facility, not a domain service and not a fifth StemThing leg.

## 19.2 Target-Displacement Law

> An ActionBand represents unresolved displacement between current recursively evaluated state and an admitted target point/set on the same observable/field basis.

## 19.3 Resultant Bipolar Axis Law

> A truly bipolar semantic axis is one bounded resultant degree of freedom. Opposing influences resolve into the current scalar; cancellation to zero is zero unless contest magnitude is separately authored as another observable. Semantic bipolarity does not imply RF conservation.

## 19.4 Stakes Law

> Stakes are the EML-derived consequence/urgency of leaving an ActionBand displacement unresolved. Displacement and stakes are distinct observables; velocity may affect stakes.

## 19.5 Authored Band Law

> Bands are authored threshold surfaces over admitted ActionBand observables. Crossing a band emits its optional EML payload. Band segmentation is semantic/authored, not fixed to normalized or equally spaced progress.

## 19.6 Native Semantics Law

> ActionBand binds to existing property, RF, CostBand, overlay, STEAD, PALMA, and boundary semantics; it does not create a parallel resource, prerequisite, sink, transfer, or structural-mutation universe.

## 19.7 Point-of-Execution Law

> ActionBand owns the lifecycle from unresolved target through partial band emissions to terminal resolution. It is not merely a goal selector in front of another action engine.

## 19.8 Recursive ActionBand Law

> An ActionBand band payload may activate subordinate ActionBands on the same intrinsic SimThing facility. Subordinate bands are nested target discrepancies, not imperative tasks; they may resolve concurrently, collapse when their target conditions become ordinary state, and never execute recursively to convergence inside one generation.

## 19.9 Multisource Collapse Law

> Heterogeneous multisource requirements may remain inline ordinary predicates/resources or materialize as subordinate ActionBands when satisfying them has an independent target-seeking lifecycle. The parent advances according to authored EML over the resulting unresolved state. Already-resolved requirements are represented by ordinary world state, not permanent completed-task records.

## 19.10 Semantic-Recursion / Physical-Flattening Law

> Recursive ActionBand structure is semantic authority. Admission/execution may inline, batch, or flatten it into non-recursive packed GPU data so long as semantics, crossing order, lifecycle, provenance, and generation pacing remain unchanged.

## 19.11 Generation-Pacing Law

> ActionBand emissions may affect later ActionBands and world state only through the admitted generation/barrier ordering. No parent→child→grandchild same-generation convergence loop is lawful.

## 19.12 Vendorization Law

> Domain behaviors such as physical movement are derived/vendorized uses of ActionBand. No domain implementation may become a peer core action facility or bypass ActionBand decision/execution semantics where ActionBand applies.

## 19.13 GPU Numerical Authority / CPU Semantic Shadow Law

> ActionBand numerical authority exists only on the GPU. Current/target values, displacement, velocity, stakes, band crossing, EML execution, recursive dependency evaluation, RF/CostBand numerical commitment state, progress, and terminal eligibility are GPU-resident and GPU-decided. CPU state is a semantic shadow containing human-readable designations, durable logical identity/history, diagnostics/presentation metadata, and sparse GPU-produced semantic/structural deltas. Human-readable labels may never become numerical dispatch keys or CPU-side decision authority. Structural CPU boundary work may apply a GPU-authorized request but may not re-evaluate the ActionBand that emitted it.

---

# 20. Explicitly fenced questions

The base ActionBand facility is considered semantically complete without resolving the following implementation/research questions:

1. **Vector CostBand atomicity:** exact efficient common-depth commitment across several independently contested scarce RF lanes.
2. **Holding/fairness:** how long provisional scarce grants may remain held while another requirement is unresolved without starvation or pathological hoarding.
3. **Local minima / adversarial navigation:** escape and competition behavior when PALMA/STEAD target descent is not trivially monotone.
4. **Optimal flattening:** the final packed GPU representation, inlining threshold, batching strategy, and whether depth-1/2 deserves dedicated fused lowering.
5. **Performance envelope:** exact memory and bandwidth cost under millions of active/inactive SimThings.

These questions are falsifiable engineering/research work. They do not reopen the constitutional existence or semantic role of ActionBand, and none permits a CPU fallback execution model.

---

# 21. Engineering/Fable review obligations

Review should attack the design at its actual load-bearing seams.

## 21.1 StemThing integrity

- Can ActionBand remain intrinsic to SimThing without creating an independent action object/service?
- Does recursive activation preserve one sole iteration/failure point?
- Does any proposed implementation secretly introduce a planner, task graph, destination registry, or domain action enum?

## 21.2 Semantic sufficiency

- Can target/displacement/stakes/bands express depth-1 event handling, multiband progress, and recursive target decomposition without a peer event engine?
- Are bipolar resultant axes sufficient when the author can create additional observables where hidden contest magnitude matters?
- Does target-set semantics avoid pathological exact-point oscillation?

## 21.3 Multisource compatibility

- Can ordinary state checks, RF lanes, and CostBand progress participate in one band payload without reclassifying them into a parallel requirement subsystem?
- Can the inherited “brake, do not rewind already-resolved progress” behavior be implemented without violating RF conservation?
- Which cases genuinely require future Vector CostBand work?

## 21.4 Recursive collapse

- Can trivial subordinate requirements be inlined while preserving exact semantics?
- Can stateful subordinate ActionBands materialize sparsely and dissolve without leaving task-history residue?
- Can siblings evaluate concurrently with only generation-paced parent observation?

## 21.5 Performance and GPU authority

- Can registration evaluation ride already-hot GPU field/reduction passes rather than require a second world scan?
- Is the common depth-1/2 case cheap enough for very large populations?
- Can recursive authoring lower to packed non-recursive GPU descriptors?
- Can identical EML/band shapes batch coherently?
- Can CPU traffic remain threshold/terminal/structural deltas only?
- Can every uncommon ActionBand shape remain GPU-executable without introducing a CPU fallback path?
- Does any proposed representation accidentally mirror continuous ActionBand state on CPU?
- Are human-readable designations absent from GPU decision/dispatch logic by construction rather than convention?

## 21.6 Determinism

- Does flattening preserve canonical semantic order?
- Are no decisions derived from physical row iteration order?
- Does recursive ActionBand activation obey the existing barrier/history laws?
- Can CPU semantic-shadow naming be proven incapable of changing GPU numerical behavior?

---

# 22. Falsifiers

The ActionBand design should be remanded if any of these are demonstrated.

### F1 — peer action authority is required

A broad class of ordinary SimThing actions cannot be expressed without introducing a second authoritative goal/event/action service beside ActionBand.

### F2 — recursion becomes an imperative planner

A necessary use requires persistent `next_step`/retry/task scheduling semantics that cannot be reconstructed from ordinary target discrepancies and world state.

### F3 — multisource semantics require duplication

A real multisource action cannot be expressed without ActionBand inventing a second resource/property classification system beside admitted RF/property/CostBand semantics.

### F4 — recursive state cannot collapse

Resolved subordinate ActionBands must remain permanently resident merely to preserve correctness rather than their consequences being represented by ordinary world state.

### F5 — performance requires population-wide action scans

The only workable implementation requires iterating all possible actions over all SimThings rather than sparse active registrations / compiled programs.

### F6 — semantic recursion cannot flatten

Correct recursive semantics require runtime pointer recursion or per-child CPU scheduling that cannot be compiled into deterministic packed GPU execution.

### F7 — band semantics are too narrow

Important target-seeking behavior cannot be represented by authored threshold surfaces plus EML payloads without introducing domain-specific event stages in core.

### F8 — topology requires a domain planner

A major target-resolution class cannot use an admitted topology/PALMA or direct-state base case and instead requires a privileged engine-side domain planner.

### F9 — CPU ActionBand authority is required

Correct behavior requires CPU-side ActionBand evaluation, continuous numerical mirroring, goal selection, band-crossing decisions, recursive child scheduling, or dispatch keyed by human-readable semantic designation. Any such requirement falsifies the intended GPU-native ActionBand architecture rather than licensing a fallback.

---

# 23. Implications for 0.0.8.7 review

This document is intended to fill the architectural space before movement or other derived actions are allowed to freeze as peer vocabulary.

The workplan reconciliation itself belongs to the DA/Owner rollback ceremony, not to this document, but the engineering dependency is clear:

```text
GPU-native ActionBand intrinsic door / semantics
        ↓
derived domain implementations
        ↓
movement as one vendorized spatial use
```

not:

```text
movement implementation
        ↓
infer/generalize ActionBand later
```

Any temporary placeholder/door used during rollback must therefore be a real GPU path through which the derived consumer is constructed or evaluated, not kabuki beside the old peer facility.

The already-proven gradient-derived authority substrate remains useful evidence: changing field/overlay state may redirect progress without editing a privileged destination/action identity. That substrate belongs underneath ActionBand rather than establishing movement as permanent peer vocabulary.

---

# 24. Core deliverable

The complete ActionBand conception is now:

```text
                     SIMTHING
                        │
                recursive state/RF
                        │
                        ▼
              resultant STEAD axes
                        │
              current coordinate X
                        │
            ┌───────────┴───────────┐
            │                       │
            │                 target set G(t)
            │              (disposition / state /
            │               time / directives /
            │                   overlays)
            │                       │
            └───────────┬───────────┘
                        ▼
                displacement D
                velocity dD/dt
                   stakes Σ
                        │
                        ▼
               EML deficiency logic
                        │
               PALMA route if needed
                        │
                        ▼
                   ACTIONBAND
                        │
              authored threshold bands
                        │
             ┌──────────┴──────────┐
             │                     │
       inline native          subordinate
       requirements           ActionBands
             │                     │
             └──────────┬──────────┘
                        ▼
                band EML payload
                        │
          ┌─────────────┼─────────────┐
          │             │             │
      ordinary RF    CostBand     overlays/events
          │             │             │
          └─────────────┼─────────────┘
                        ▼
               partial consequence
                        │
                 new world state
                        │
                        ▼
                later generation
                        │
              next band / collapse
                        │
                        ▼
                  terminal target
                        │
                terminal emission
                        │
                        ▼
               ActionBand dissolves

     ALL NUMERICAL AUTHORITY ABOVE: GPU
                        │
                        ▼ sparse semantic/structural deltas only

                 CPU SEMANTIC SHADOW
            names / durable identity / history
              UI / diagnostics / persistence
              existing structural boundary
          (never ActionBand decision authority)
```

The central architectural result is:

> **ActionBand is the fractally recursive, GPU-native event-execution facility of the base SimThing. It turns tension between current and desired STEAD state into generation-paced, band-emitted action using existing EML, PALMA, RF, CostBand, overlay, and boundary authorities. Complex goals recurse into subordinate discrepancies on the same intrinsic facility and collapse back into ordinary world state as those discrepancies resolve. The CPU never executes the ActionBand: it holds only the semantic shadow by which opaque GPU identities and GPU-produced boundary deltas become human-readable, persistent, and presentable.**

That is the specification Fable and engineering should review.
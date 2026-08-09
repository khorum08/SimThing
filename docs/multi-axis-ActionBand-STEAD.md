# Multi-Axis ActionBand + STEAD
## Intrinsic recursive GPU event execution over STEAD value fields, PALMA routes, RF, EML, and CostBand

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

# 0. Executive definition and physical feasibility verdict

An **ActionBand** is an intrinsic, normally inert facility on every SimThing that represents an unresolved transition between:

1. the SimThing's **current coordinate** on one or more admitted STEAD value axes; and
2. a **desired target point, interval, region, or condition set** on those same observables.

The ActionBand derives displacement between current and target state, observes whether that displacement is improving or worsening, evaluates the **stakes** of leaving the displacement unresolved, and uses admitted EML plus PALMA where topology exists to determine lawful progress toward the target.

Authored **bands** are threshold surfaces over ActionBand observables. Crossing a band evaluates an optional admitted EML program. The numerical result is routed through pre-admitted GPU bindings into ordinary RF, CostBand, overlay/directive, subordinate-ActionBand, or structural-request surfaces.

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
 GPU EML evaluation
          ↓
 pre-admitted emission bindings
          ↓
 existing RF / CostBand / overlay / boundary surfaces
          ↓
 partial or terminal consequence
          ↓
 new ordinary SimThing state
          ↓
 next-generation recursive state
```

Everything through ActionBand evaluation, crossing, recursive subordinate evaluation, and numerical consequence authorization is GPU-authoritative. CPU interaction occurs only when a sparse semantic/structural boundary delta must be remembered, presented, persisted, or applied through an already-existing CPU-owned structural boundary.

## 0.1 Feasibility under the all-GPU premise

The recursive ActionBand concept remains feasible **provided recursion is semantic, not a runtime call stack or dynamically allocated object tree**.

The physically lawful interpretation is:

> **Recursive ActionBand authoring lowers to a sparse, pre-admitted GPU dependency table plus compact mutable state. Parent/child recursion is unrolled across generations and represented by activation bits, indices/spans, parameter rows, and ordinary GPU buffers.**

Likewise, an EML band payload is **not** a CPU callback and should not become an imperative shader mini-language with arbitrary side effects.

The physically lawful interpretation is:

> **An EML payload is a bounded admitted numerical program. Its result is consumed by pre-admitted numeric emission bindings that write only to existing GPU-authoritative surfaces.**

Therefore the design does **not** require:

- runtime recursive function calls on GPU;
- CPU child scheduling;
- CPU event dispatch;
- GPU-side authoring of new EML bytecode;
- arbitrary pointer-linked ActionBand trees;
- per-band human-readable dispatch names;
- or an unbounded GPU heap of semantic action objects.

Those shapes would be disqualifying.

The design instead fits the existing SimThing execution direction: sealed bindings, bounded EML, packed GPU tables, sparse threshold emissions, generation pacing, and structural mutation only at the existing boundary.

## 0.2 “GPU matrix/table” means table-driven vectorized execution, not necessarily one literal dense GEMM

ActionBand should be designed for the GPU's table/vector execution model, but its semantics do not require that every operation be representable as one dense matrix multiply.

EML already includes comparison, `SELECT`, `FLOOR`, `MIN/MAX`, exact arithmetic, and admitted `EXP`/`LN`; RF and threshold emission also use sparse bindings and gathers. Therefore the physically accurate target is:

```text
packed SoA tables
+ sparse logical bindings
+ shared EML/JIT programs
+ GPU field/reduction passes
+ threshold compare/emission
+ compact next-state writes
```

rather than forcing every ActionBand operation into a literal dense matmul.

The important invariant is **GPU-resident, vectorizable, table-driven execution with no CPU semantic interposition**.

---

# 1. Constitutional placement: ActionBand is the SimThing `act` facility

The existing StemThing thesis remains unchanged: the base recursive SimThing carries generic capabilities inert-by-default, and domain specialization activates/authors those capabilities rather than introducing peer engines.

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
- a CPU goal selector; or
- an independent lifecycle authority.

It is the intrinsic implementation of the SimThing's ability to **act on what it perceives**.

The base SimThing remains the sole point of iteration and failure. Recursive ActionBands never escape that ownership. A subordinate ActionBand is not another engine object with its own scheduler or authority; it is a recursively activated portion of the same ActionBand facility attached to the same SimThing.

```text
SimThing
 ├─ participate
 ├─ act  ← ActionBand
 ├─ originate
 └─ receive
```

## 1.1 GPU numerical authority / CPU semantic shadow

ActionBand is implemented entirely in the GPU numerical regime.

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
recursive/subordinate dependency state
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

Human-readable domain names exist only in the CPU semantic shadow / authoring-presentation layer. GPU ActionBand programs and descriptors operate on sealed numeric/logical bindings, opaque ids, offsets, ranges, thresholds, and admitted EML programs; they do not branch on strings, domain nouns, or human-readable action names.

> **GPU computes and decides ActionBand numerical state. CPU remembers and names the semantic consequences.**

A structural consequence remains lawful when the GPU emits a sealed request/delta and the existing CPU boundary performs the mutation. The boundary executes structure; it does not re-evaluate the ActionBand decision.

---

# 2. The semantic coordinate model: Stellaris-style axes generalized

The useful lesson from the Stellaris ethics web is geometric, not domain-specific.

```text
Authoritarian  ←→ Egalitarian
Militarist     ←→ Pacifist
Xenophobe      ←→ Xenophile
Spiritualist   ←→ Materialist
```

The abstraction is:

> **Complex state can be located as coordinates on a small number of logically chosen value axes rather than encoded as a catalog of named composite states.**

SimThing generalizes this to arbitrary admitted semantic/value axes.

Examples might include:

```text
scarcity         ←→ abundance
threat           ←→ safety
isolation        ←→ accessibility
degradation      ←→ physical quality
job insecurity   ←→ job security
housing pressure ←→ housing abundance
```

The engine does not know what these names mean. It knows admitted property/field bindings and the EML/STEAD laws that generate them. Human-readable axis names are CPU semantic-shadow designations; the GPU sees admitted numeric bindings.

## 2.1 Resultant bipolar scalar law

Where an axis is truly bipolar, it is one bounded degree of freedom:

\[
x_k \in [L_k,U_k]
\]

Ordinary state, falloff influence, overlays, transient conditions, and other admitted transforms move the value toward one pole or the other.

Conceptually:

\[
x'_k = \operatorname{Bound}_k\left(x_k + \sum_i \Delta_{i,k}\right)
\]

**The resultant is authoritative.**

If equal and opposite influences produce:

\[
+0.7 - 0.7 = 0
\]

then the receiving SimThing experiences that axis as `0`.

There is no intrinsic obligation to preserve hidden contest magnitude. If the difference between quiet neutrality and strong cancelled pressure matters, it must be authored as another observable/axis.

A SimThing can therefore be neutral on one ActionBand and highly displaced on another:

```text
border disposition = 0
trade imbalance    = +0.8
food security      = -0.5
job security       = -0.3
```

### 2.1.1 Bipolarity is not RF conservation

A bipolar semantic coordinate is not automatically a conserved RF resource.

“Moving toward one pole moves away from the other” describes one bounded degree of freedom; it does not imply:

\[
\sum_s x_{s,k}=constant
\]

Actual conservation remains RF authority.

## 2.2 Strong axes, emergent semantics

Primitive axes should carry independent causal information. Higher-order named phenomena should normally emerge from combinations and trajectories through those axes.

City examples of primitive values might include housing pressure, condition, rent, employment access, labor availability, capital, demand, safety, pollution, services, accessibility, and congestion. Terms such as gentrification, blight, prosperity, food desert, suburbanization, or industrial decline remain derived observations unless they contain genuinely independent causal state.

---

# 3. Recursive SimThing state produces STEAD coordinates

A SimThing does not receive semantic labels by fiat. Its current coordinates arise from ordinary recursive state.

For SimThing `s`:

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

An admitted EML projection may produce semantic coordinates:

\[
X_s(t) = [x_{s,0}(t),x_{s,1}(t),...,x_{s,K-1}(t)]
\]

with:

\[
x_{s,k}(t) = F_k(R_s(t))
\]

The coordinate is a projection of causal state, not an independently authored label.

Because SimThings are recursive, this works at every scale.

## 3.1 STEAD propagation

A coordinate can become a STEAD field channel over admitted topology:

\[
\Phi(x,k)
\]

Conceptually:

\[
\Phi_k(x)=\operatorname{Reduce}_{e\in E_k}
\left(A_{e,k}\,f_{e,k}(d(e,x))\right)
\]

Different axes may use different topologies, falloff laws, cadences, and reductions. “One STEAD field” is conceptual; physically it may be several channel arrays/sweeps.

Frequently reused combinations may be cached as derived fields, but the authoritative meaning remains primitive inputs plus admitted EML.

---

# 4. Target, displacement, velocity, and stakes

An ActionBand exists because current state differs from a desired state.

Let:

\[
X_{s,b}(t)
\]

be the current coordinate relevant to ActionBand `b`, and:

\[
\mathcal G_{s,b}(t)
\]

be the target set.

The target may be an exact point, scalar threshold, interval, multidimensional region, arrival locus/radius, predicate-defined condition set, or another EML-defined acceptable region.

Examples, expressed here with CPU-shadow names only:

```text
food_security >= 0.6
housing_pressure in [-0.1, +0.1]
build_progress >= 1.0
fleet within arrival radius of Orion IV
capability.psionic_navigator >= 1
```

## 4.1 Target sources

ActionBand owns no separate target manager. Targets may arise from ordinary SimThing state and authoring:

```text
baseline disposition/personality
standing conditions
scripted state
time
incoming directives
overlays
resource needs
deficits
explicit authored goals
```

A target may move over time:

\[
\mathcal G(t)
\]

without requiring a retarget state machine.

## 4.2 Displacement

For point targets:

\[
D(t)=G(t)-X(t)
\]

For a target set:

\[
D(t)=\Pi_{\mathcal G(t)}(X(t))-X(t)
\]

where `Π` is the admitted projection/distance-to-target operation appropriate to the axis/topology.

## 4.3 Velocity

Useful quantities include:

\[
V_X(t)=X(t)-X(t-1)
\]

and:

\[
V_D(t)=D(t)-D(t-1)
\]

which indicate whether the discrepancy is closing or worsening.

Velocity should derive from already-available current/previous GPU planes whenever possible. No ActionBand-specific CPU history is owed.

## 4.4 Stakes

Displacement answers “how far from desired state?” Stakes answer “how consequential is it to remain unresolved?”

Let:

\[
\Sigma_{s,b}(t)=S_b\left(D,V_D,R_s,\Phi,overlays,reserves,deficits,history,...\right)
\]

where `S_b` is admitted GPU EML.

A useful conceptual relationship is:

\[
P_b \sim \Sigma_b \cdot \|D_b\|
\]

but no formula is mandated.

> **Displacement supplies tension. Stakes supply urgency/consequence. Bands turn meaningful tension, urgency, or progress into emissions.**

---

# 5. Band semantics and EML payloads

A band is an authored threshold surface over any admitted ActionBand observable.

Possible operands include:

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

Band segmentation is authored semantic structure. It is not required to normalize to `[0,1]` or use equal slices.

## 5.1 Default depth

Expected common shape:

```text
0 bands   → inert
1 band    → ordinary event/action trigger
2 bands   → trigger + completion, or warning + action
N bands   → richer authored progression/escalation/auditing
```

Most ActionBands should be depth 1 or depth 2.

## 5.2 Crossing semantics

Baseline behavior is edge/crossing driven:

```text
below band
   ↓ crosses
emit once
   ↓ remains beyond threshold
no duplicate emission solely for remaining there
```

A later recrossing may emit again according to authored lifecycle/hysteresis.

Band crossing authority is GPU-only.

## 5.3 EML payloads are pure numerical programs plus bound emissions

The phrase **EML payload** must not be interpreted as an imperative callback.

EML remains a bounded admitted numerical language. A band payload therefore has two conceptual pieces:

```text
1. EML program
   reads admitted GPU bindings
   computes scalar/bounded numeric results

2. emission-binding table
   routes those results to pre-admitted destinations
```

Examples of destinations include:

```text
ordinary property/result column
overlay numeric parameter/activation state
RF claim quantity
CostBand input/progress binding
subordinate ActionBand activation/parameter state
sealed structural-request buffer
telemetry/event emission buffer
```

EML does **not** need arbitrary mutation opcodes, dynamic pointers, string dispatch, or runtime graph construction.

The physical sequence is:

```text
band crosses
    ↓
execute admitted EML program on GPU
    ↓
obtain numeric result(s)
    ↓
apply fixed emission bindings
    ↓
write ordinary GPU next-state / claim / activation / request buffers
```

This is crucial to preserving EML as one deterministic language rather than turning each ActionBand into a mini-script VM.

### 5.3.1 Shared programs

Many bands may share the same EML program and emission shape. A descriptor stores opaque program/binding ids or offsets; the human-readable program name exists only in CPU semantic shadow.

The executor is free to bucket and JIT/fuse by program shape.

### 5.3.2 Multi-effect bands

A band may have several consequences without requiring one EML program to perform imperative side effects. A crossing may gate or parameterize a fixed emission bundle. Where distinct numerical values are needed, the physical lowering may use several result slots/program blocks or another faithful admitted lowering. The exact ABI remains engineering work.

---

# 6. GPU physical model: sparse tables, not recursive objects

This section is load-bearing for the all-GPU architecture.

The semantic model is recursive. The physical model is **flat, indexed, and sparse**.

A plausible implementation family is:

```text
ActionBandTemplate[]      immutable/admitted program shape
ActionBandInstance[]      sparse active numerical instances
BandDescriptor[]          thresholds + program/binding spans
DependencyBinding[]       parent/child template or instance relations
EmissionBinding[]         result destination bindings
StateCurrent[]            active/progress/crossing/parameter state
StateNext[]               next-generation state
```

This is an architectural sketch, not a frozen ABI.

## 6.1 Templates versus instances

A **template** contains immutable admitted structure that can be shared by many SimThings/instances:

```text
observation binding shape
target law/binding shape
band schedule
EML program ids/offsets
emission-binding spans
subordinate template spans
lifecycle law
```

An **instance** carries only mutable numerical state and parameter bindings needed for one active ActionBand:

```text
owning logical SimThing slot
template id
active state
current band/crossing state
progress/remainder where not already ordinary world state
target parameters where instance-specific
subordinate activation/result bits or spans
```

Human-readable names are not present in either hot structure.

This split is important because millions of semantically similar ActionBands can share one template while retaining different current/target values.

## 6.2 Sparse registration

“Every SimThing has ActionBand capability” does **not** imply fixed physical ActionBand storage for every possible action on every SimThing.

The desired scaling law is:

\[
O(\text{active/materialized ActionBand instances})
\]

not:

\[
O(\text{all SimThings} \times \text{all possible actions})
\]

A SimThing with no active ActionBands owns no hot instance rows.

## 6.3 Recursion becomes dependency state, not pointer recursion

Semantic authoring may say:

```text
Parent
 ├─ Child A
 ├─ Child B
 └─ Child C
```

Physical lowering represents this with stable indices/spans and state bits, for example:

```text
parent instance → dependency span [i..j)
dependency row  → child template/instance id + target/result binding
child instance  → own ordinary ActionBand row
```

No runtime recursive function call is needed.

No GPU thread follows a pointer-linked tree until completion.

No CPU scheduler walks children.

## 6.4 Generation pacing makes recursion much cheaper

The existing rule that recursion does not converge inside one generation is a major physical advantage.

A parent crossing in generation `t` does **not** need to execute the child immediately.

Instead:

```text
generation t:
    parent evaluates
    parent output sets child-next active/parameter state

barrier / state swap

generation t+1:
    child evaluates like any other active ActionBand
```

A child terminal result similarly becomes ordinary next-state observed by the parent later.

Therefore every active ActionBand can usually be evaluated as an independent GPU row against `StateCurrent`, writing only `StateNext` and ordinary output buffers.

Recursion is temporally unrolled by the generation model.

## 6.5 Child activation is not semantic object construction

When this document says an EML payload may “activate a subordinate ActionBand,” the base meaning is:

> **Set the next-generation active/parameter state of a pre-admitted subordinate template/slot.**

It does not mean:

```text
new ActionBand(...)
allocate arbitrary object
compile new EML
attach pointer child
run child now
```

For the common authored recursive case, subordinate shapes are known at admission and can be precompiled into the template/dependency table.

### 6.5.1 Parameterized child instances

A pre-admitted child template may receive instance parameters computed on GPU, such as a target locus, threshold, resource quantity, or relevant logical binding.

The GPU may therefore activate the same template with different parameter rows without inventing a new semantic program.

### 6.5.2 Truly new capacity

If active ActionBand instance capacity itself must grow, that is a **storage/structural capacity problem**, not a license for CPU action semantics.

The GPU may authorize a bounded structural/storage request through the existing boundary. The CPU boundary may materialize storage/identity according to ordinary law, but it does not choose the child semantic template or decide why it is needed.

Future residency/derivation machinery may make this cheaper. The base ActionBand law does not depend on arbitrary same-generation GPU heap growth.

## 6.6 Collapse is a state transition, not object destruction logic

When a child resolves, the GPU marks it terminal/inactive in next state and its ordinary consequences remain in world state.

A physical row may later be recycled under ordinary residency/allocation law, but correctness does not require a CPU “task cleanup” operation.

The semantic rule remains:

> **Resolved ActionBands collapse back into ordinary SimThing state.**

## 6.7 Deterministic activation cannot depend on atomic append order

A naive GPU append queue whose semantic ordering depends on racing atomic increments would violate existing ordering discipline.

Base recursion should therefore prefer deterministic shapes such as:

- preassigned subordinate slots;
- fixed dependency spans;
- stable logical keys;
- next-state activation bits;
- or another recorded/canonically ordered GPU emission structure.

If a future dynamic instance allocator uses append/compaction internally, its physical order may not become semantic order.

---

# 7. PALMA supplies the route where a route exists

ActionBand knows a current condition and a target condition. Reducing the displacement may require navigating admitted topology.

Where topology exists, PALMA provides the lawful route/impedance structure.

This includes physical space, LinkGraph relations, trade/supply accessibility, service access, recursive tree relations, admitted progression ladders, and other existing topology.

ActionBand does not mint a generic `ActionGraph`.

## 7.1 Route degeneracy

A depth-1 capability check such as:

```text
psionic_navigator >= 1
```

requires no graph. Its route degenerates to ordinary state observation/progress toward the target predicate.

PALMA is used where target resolution is topological; direct property/threshold ActionBands remain lawful base cases.

## 7.2 Local minima and adversarial navigation remain fenced

The base definition does not settle local-minimum escape, adversarial multi-field navigation, starvation/livelock, same-generation convergence, or coordinated Vector CostBand clearing across several independently contested RF arenas.

Those remain later probes. This is scope discipline, not doubt about ActionBand.

---

# 8. Multisource action: inherited structure, native semantics

Capability-tree archaeology established the important semantic shape:

1. one transition may depend on several independent conditions;
2. those conditions need not share semantics;
3. progress and eligibility are distinct;
4. completed/paid progress should not be erased merely because another prerequisite remains unresolved; and
5. consequence activation occurs when the required conjunction becomes executable.

ActionBand therefore does not reopen whether heterogeneous requirements can compose.

## 8.1 Do not duplicate RF/property semantics

ActionBand binds to existing authorities.

A property check remains an observation. A conserved RF lane remains RF. A sink remains CostBand. Residency/capacity retains its own semantics. Transfer remains transfer. Time remains ordinary state unless modeled as scarce.

The general shape is:

```text
band EML
   ↓
reads ordinary prerequisite/property state
   ↓
computes/gates native RF claims where scarcity is real
   ↓
uses existing CostBand semantics for sinks/progress
```

All numerical steps remain GPU-resident.

## 8.2 Braking / held progress

Default inherited law:

> **When one unresolved requirement blocks the next band, already-resolved progress remains resolved according to the native semantics of its underlying binding. The parent brakes rather than rewinding completed work.**

Whether scarce grants may remain held across generations, and under what fairness/livelock constraints, belongs to later quantitative work.

## 8.3 Multisource requirement as one-band payload

A depth-1 ActionBand may use one EML program as an all-of gate.

Example:

```text
Goal: reach Shroud coordinate

band EML reads:
    psionic_navigator >= 1
    shroud_access >= 1
    required energy/fuel grant executable
```

The capability checks are ordinary state observations; energy/fuel remains RF/CostBand.

No requirement is forced into RF merely because it participates in the same ActionBand.

---

# 9. Recursive ActionBands

ActionBand is structurally recursive because SimThing itself is structurally recursive.

A band may cause subordinate ActionBands to become active on the same SimThing's intrinsic ActionBand facility when unresolved deficiencies themselves require target-seeking action.

Under the GPU physical model, this means **activation of pre-admitted subordinate templates/instances**, not runtime construction of semantic action objects.

## 9.1 Parent unresolved-state model

Let a parent `P` observe subordinate unresolved quantities:

\[
u_i(t) \ge 0
\]

with `u_i=0` meaning the subordinate target is satisfied.

Then:

\[
U_P(t)=[u_0(t),u_1(t),...,u_n(t)]
\]

is an ordinary GPU input to parent EML:

\[
q_P(t)=F_P(D_P,V_{D_P},\Sigma_P,U_P,\Phi,R,...)
\]

The common all-of rule is only one authored case. EML may express substitution, quotas, weighted satisfaction, or other lawful combinations.

## 9.2 Nested discrepancies, not imperative tasks

Forbidden:

```text
TaskNode {
    next_step
    retry_policy
    success_handler
    failure_handler
    child task scheduler
}
```

Lawful:

```text
Parent target discrepancy
  ├─ subordinate target discrepancy A
  ├─ subordinate target discrepancy B
  └─ subordinate target discrepancy C
```

There is no imperative `next`. Current state determines which discrepancies remain unresolved.

## 9.3 Siblings may resolve concurrently

Subordinate ActionBands may progress independently where resources/topology permit.

A colonization target may simultaneously pursue transport, population commitment, supplies, and access. The parent observes their result vector instead of hardcoding `A then B then C`.

This is naturally parallel on GPU.

## 9.4 Recurse and collapse

```text
child terminal crossing
        ↓
ordinary consequence becomes world state
        ↓
child next-state becomes inactive
        ↓
parent later observes resulting ordinary state
```

Resolved children do not become permanent completed-task records.

## 9.5 Generation-paced recursion

Forbidden:

```text
parent fires
→ run child now
→ run grandchild now
→ converge in one dispatch
```

Required:

```text
generation t:
    parent sets child-next activation/parameters

barrier

generation t+1:
    child evaluates as an ordinary active row
```

This is both the semantic pacing law and the main reason recursive ActionBands remain practical as GPU tables.

---

# 10. Multisource requirements and recursive ActionBands collapse into one form

A heterogeneous requirement may lower in one of two ways while preserving one semantic model.

### Trivial requirement

If the condition has no independent lifecycle, inline it into parent EML:

```text
has_psionic_navigator >= 1
```

### Stateful target-seeking requirement

If satisfying the condition requires its own target, STEAD interaction, resources, multiple bands, duration, or subordinate requirements, activate a subordinate ActionBand template:

```text
Acquire Colony Transport
    target: transport_capacity >= required
```

Thus a parent can mix:

```text
inline capability checks
inline property predicates
native RF/CostBand requirements
materialized subordinate ActionBands
```

without a parallel planner or requirement engine.

## 10.1 Colonization witness

```text
COLONIZE ORION IV
│
│ target: colony_state(OrionIV) = established
│
├─ Acquire Colony Transport
├─ Establish Population Commitment
├─ Stage Supplies
├─ Secure Access
└─ inline capability/state checks
```

The authoring view is recursive. The GPU view may be one parent template, several child templates, stable dependency spans, sparse active instance rows, and next-state activation bits.

No human-readable branch exists in production execution.

## 10.2 Semantic route discovery remains EML territory

Complex goals such as colonization or supply-chain creation may require EML to determine which deficiencies remain and which pre-admitted subordinate templates should become active.

This does not require a universal planning graph.

EML computes deficiency/gating values; fixed subordinate-binding tables map those results to allowable child activations.

---

# 11. Events and directives are ActionBand emissions/inputs

An event is often simply a meaningful band crossing emitted from changing state.

A directive is ordinary received state/overlay that may deform the target, alter stakes, change a threshold, affect EML inputs, or activate an admitted ActionBand template.

```text
food_security falls below threshold
        ↓
GPU ActionBand crossing
        ↓
GPU EML evaluates ordinary state/RF
        ↓
fixed emissions make food-seeking response actionable
```

CPU-readable event names are semantic-shadow projections of opaque GPU identities after the GPU has emitted a crossing.

---

# 12. Existing facilities retain their authority

| Mechanism | Authority |
|---|---|
| **STEAD** | recursively produced/shared field coordinates and propagation |
| **PALMA** | topology-aware potential/impedance routing where a route exists |
| **EML** | pure admitted numerical valuation, target/deficiency reduction, band-result computation |
| **RF** | resource claims, conserved quantities, constrained clearing/disbursement |
| **CostBand** | exact resource-sink/work quantization and carried remainder |
| **Overlay** | ordinary policy/directive/transient deformation |
| **Threshold/crossing substrate** | detecting meaningful band crossings |
| **Emission bindings** | fixed routing from EML results to existing numerical output surfaces |
| **Boundary authority** | structural mutation that numerical execution may authorize but never perform directly |
| **GPU ActionBand** | numerical lifecycle of unresolved target displacement and band-emitted execution |
| **CPU semantic shadow** | names, durable identity/history, diagnostics/presentation, sparse boundary deltas; never numerical decision authority |

ActionBand composes these mechanisms. It does not absorb their semantics.

## 12.1 ActionBand + CostBand

CostBand remains:

\[
N=\left\lfloor\frac{V}{C}\right\rfloor
\]

\[
R=V-NC
\]

ActionBand may use CostBand output/progress as a band observable and may gate/parameterize CostBand through EML. It must not invent another sink mechanism.

## 12.2 Structural consequences

A band result may authorize a structural consequence, but only the existing boundary mutates structure.

GPU writes an opaque sealed request with numeric/logical bindings. CPU applies it; CPU does not reinterpret a human-readable label or re-decide the ActionBand result.

---

# 13. Movement is a derived/vendorized ActionBand implementation

Physical movement is a straightforward spatial specialization:

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

Movement is not a peer core action facility.

The same ActionBand semantics also cover getting food, building a door, completing research, acquiring a capability, repairing a machine, satisfying a service need, establishing a colony, or moving a fleet.

Those domain phrases are CPU-shadow labels only.

---

# 14. Worked examples

## 14.1 Get food — likely depth 1

```text
current: food_security = -0.7
target:  food_security >= +0.2
stakes:  function of displacement, reserves, velocity, local conditions
band:    actionable deficit threshold
EML:     reads food/access/price state
outputs: native RF claims / CostBand parameters
terminal: target reached → inactive next-state
```

## 14.2 Build a door — one or several authored bands

```text
current: build_progress = 0
target:  build_progress >= 1
bands:   0.25 / 0.50 / 0.75 / 1.00, or only terminal 1.00
```

Each band may run a shared EML program whose bound outputs drive native work/material CostBands or other state.

## 14.3 Fleet to Orion IV — spatial vendorization

```text
current: spatial locus A
target:  arrival set around Orion IV
route:   PALMA over admitted topology
band:    lawful progress for this generation
output:  numeric movement/boundary request
terminal: arrival predicate
```

## 14.4 Shroud traversal — multisource one-band gate

```text
one executable band reads:
    psionic_navigator >= 1
    shroud_access >= 1
    energy/fuel grant executable
```

The two capability checks are observations; energy/fuel remains RF/CostBand.

## 14.5 Colonization — recursive templates

The authoring tree may show transport, population, supply, and access as child ActionBands. The GPU implementation may flatten them into templates plus dependency spans and evaluate all active children concurrently across generations.

---

# 15. Performance implications of the GPU-only model

The GPU-only premise **strengthens** some parts of the design and constrains others.

## 15.1 Recurse semantically; flatten physically

> **Recursive ActionBand structure is semantic authority. Physical execution is a flat GPU dataflow.**

No runtime recursive call stack is required or permitted.

## 15.2 Ride already-hot sweeps

The intended path is:

```text
value becomes available on GPU
    ↓
registered ActionBand input gather/compare
    ↓
no crossing → no payload execution
    ↓
crossing → EML program + fixed emissions
```

rather than a second world scan or CPU readback.

“Free evaluation” means piggybacking on already-required memory traffic and paying sparse compare/emission cost, not literal zero instructions.

## 15.3 Depth-1/2 fast path

Because most ActionBands are expected to be depth 1 or 2, physical layout should favor inline threshold descriptors and shared EML program ids for the common case.

Deep recursive authoring should not force deep per-instance state.

## 15.4 Inline trivial children

Simple prerequisites should compile into parent EML. Only children with independent lifecycle require instance rows.

This creates three cost tiers:

```text
inline predicate
    ↓
materialized depth-1 ActionBand
    ↓
multiband / recursive ActionBand
```

All execute on GPU.

## 15.5 Program bucketing and JIT fusion

Divergent per-row EML programs would harm GPU coherence. Templates therefore provide a natural batching key.

The executor should be free to group active rows by:

```text
EML program id
band shape
binding layout
axis/field access shape
```

and execute a shared interpreter/JIT block over many instances.

This reuses the existing EML direction: one authored language with interpreted/JIT lowerings, rather than per-domain kernels.

## 15.6 Sparse gathers are likely the dominant cost, not recursion itself

Once recursion is flattened, the likely physical cost is gathering dispersed STEAD/RF/property inputs and writing sparse outputs.

Engineering should therefore prioritize:

- SoA layout;
- grouping by binding/program shape;
- locality-aware descriptor ordering;
- shared-ingress/workgroup-local caching where proven;
- compact active masks/lists;
- and avoiding duplicate field projections.

The recursion metadata itself can be very small.

## 15.7 Double-buffered state removes intra-dispatch dependency hazards

Use `StateCurrent` for all reads and `StateNext` for activations/progress/collapse writes where practical.

This makes parent/child generation pacing explicit and avoids read-after-write recursion inside one dispatch.

## 15.8 Continuous state remains GPU-resident

Displacement, velocity, stakes, progress, active bits, crossing state, child unresolved state, EML intermediates/results, claims, and terminal eligibility remain GPU-resident.

CPU-visible traffic is sparse semantic/structural output only.

## 15.9 Memory risk is bounded by active state, not authoring depth

A deep authored template graph does not require every node to be materialized for every SimThing.

Templates are shared; trivial nodes inline; stateful children materialize sparsely.

The practical risk is therefore active-instance cardinality and per-instance state width, not recursive syntax itself.

## 15.10 Unbounded dynamic recursion is deliberately rejected

ActionBand does not promise arbitrary runtime creation of previously unknown semantic programs or unbounded child fanout.

Any such requirement would undermine deterministic GPU-table execution and must be justified as new structural machinery rather than smuggled into “recursion.”

---

# 16. Determinism and lifecycle

ActionBand inherits SimThing determinism laws:

- ordering may never come from physical row order;
- EML uses admitted arithmetic semantics;
- threshold crossings are sealed/recordable where required;
- structural consequences remain behind recorded boundary authority;
- no same-generation recursive convergence exists;
- CPU semantic-shadow labels never affect numerical ordering or dispatch;
- child activation may not depend semantically on nondeterministic atomic append order;
- completed ActionBands collapse and do not leak permanent task-state residue.

A moving target or changing overlay simply changes GPU input state for the next generation.

---

# 17. Candidate binding laws for DA review

## 17.1 Intrinsic Action Law

> Every SimThing possesses the inert-by-default capability to host ActionBands. ActionBand is the base SimThing event/action execution facility, not a domain service and not a fifth StemThing leg.

## 17.2 GPU Numerical Authority / CPU Semantic Shadow Law

> ActionBand numerical authority exists only on the GPU. CPU state is a semantic shadow containing human-readable designations, durable logical identity/history, diagnostics/presentation metadata, and sparse GPU-produced semantic/structural deltas. Human-readable labels may never become numerical dispatch keys or CPU-side decision authority.

## 17.3 Target-Displacement Law

> An ActionBand represents unresolved displacement between current recursively evaluated state and an admitted target point/set on the same observable/field basis.

## 17.4 Resultant Bipolar Axis Law

> A truly bipolar semantic axis is one bounded resultant degree of freedom. Opposing influences resolve into the current scalar; cancellation to zero is zero unless contest magnitude is separately authored as another observable. Semantic bipolarity does not imply RF conservation.

## 17.5 Stakes Law

> Stakes are the EML-derived consequence/urgency of leaving an ActionBand displacement unresolved. Displacement and stakes are distinct observables; velocity may affect stakes.

## 17.6 Authored Band Law

> Bands are authored threshold surfaces over admitted ActionBand observables. Crossing a band evaluates its admitted EML program and bound emissions. Band segmentation is semantic/authored, not fixed to normalized or equally spaced progress.

## 17.7 EML Payload Purity / Bound Emission Law

> An ActionBand EML payload is a bounded numerical program, not an imperative callback. EML results reach the world only through pre-admitted emission bindings into existing GPU-authoritative property, overlay, RF, CostBand, subordinate-ActionBand, telemetry, or structural-request surfaces.

## 17.8 Native Semantics Law

> ActionBand binds to existing property, RF, CostBand, overlay, STEAD, PALMA, and boundary semantics; it does not create a parallel resource, prerequisite, sink, transfer, or structural-mutation universe.

## 17.9 Point-of-Execution Law

> ActionBand owns the lifecycle from unresolved target through partial band emissions to terminal resolution. It is not merely a goal selector in front of another action engine.

## 17.10 Recursive ActionBand Law

> A band result may activate subordinate ActionBands on the same intrinsic SimThing facility. Subordinate ActionBands are nested target discrepancies, not imperative tasks; they may resolve concurrently and collapse when their target conditions become ordinary state.

## 17.11 GPU Table Recursion Law

> Recursive ActionBand semantics lower to stable GPU indices/spans, shared templates, sparse instance state, and next-generation activation/result buffers. No runtime pointer recursion, CPU child scheduling, semantic dependence on atomic append order, or dynamic GPU authoring of EML programs is required or permitted by the base design.

## 17.12 Activation-Is-Not-Construction Law

> Activating a subordinate ActionBand means activating/parameterizing a pre-admitted GPU template/instance for a later generation. If additional storage must be materialized, that is ordinary structural/residency work; CPU boundary code may provide storage but may not choose or execute ActionBand semantics.

## 17.13 Multisource Collapse Law

> Heterogeneous multisource requirements may remain inline ordinary predicates/resources or materialize as subordinate ActionBands when satisfying them has an independent target-seeking lifecycle. Already-resolved requirements are represented by ordinary world state, not permanent completed-task records.

## 17.14 Generation-Pacing Law

> Parent/child ActionBand effects propagate through admitted generation/barrier ordering. No parent→child→grandchild same-generation convergence loop is lawful.

## 17.15 Semantic-Recursion / Physical-Flattening Law

> Recursive ActionBand structure is semantic authority. Admission/execution may inline, batch, JIT, or flatten it into packed non-recursive GPU data so long as semantics, crossing behavior, lifecycle, provenance, and generation pacing remain unchanged.

## 17.16 Vendorization Law

> Domain behaviors such as physical movement are derived/vendorized uses of ActionBand. No domain implementation may become a peer core action facility or bypass ActionBand decision/execution semantics where ActionBand applies.

---

# 18. Explicitly fenced questions

The base ActionBand facility is semantically complete without resolving:

1. **Vector CostBand atomicity:** exact efficient common-depth commitment across several independently contested scarce RF lanes.
2. **Holding/fairness:** how long provisional scarce grants may remain held while another requirement is unresolved without starvation or pathological hoarding.
3. **Local minima / adversarial navigation:** escape and competition behavior when PALMA/STEAD target descent is not trivially monotone.
4. **Optimal GPU ABI:** exact descriptor widths, state layout, template/instance split, inlining threshold, active-list representation, and batching/JIT strategy.
5. **Dynamic capacity growth:** the best GPU-resident/residency-backed mechanism when active ActionBand cardinality exceeds pre-granted capacity.
6. **Performance envelope:** exact memory/bandwidth cost under millions of active/inactive SimThings.

These are engineering/research questions. None licenses a CPU fallback execution model.

---

# 19. Engineering/Fable review obligations

Review should attack the design at its actual load-bearing seams.

## 19.1 GPU physical feasibility

- Can the common depth-1/2 case be represented as compact sparse GPU descriptors?
- Can shared templates amortize EML/band structure over many instances?
- Can all recursive dependencies be represented by stable indices/spans and current/next state rather than pointer recursion?
- Can child activation be performed as deterministic next-state writes without CPU scheduling?
- Can storage growth remain separate from semantic activation?

## 19.2 EML payload feasibility

- Can EML remain pure numerical evaluation while fixed emission bindings provide all required effects?
- Do existing EML/JIT resource limits permit useful band payloads without per-domain kernels?
- Is program bucketing sufficient to avoid pathological shader divergence?
- Does any required ActionBand use demand runtime authoring of new EML bytecode or arbitrary side effects? If yes, remand.

## 19.3 StemThing integrity

- Does recursive activation preserve one sole iteration/failure point?
- Does any implementation secretly introduce a planner, task graph, destination registry, domain action enum, or CPU event manager?

## 19.4 Multisource compatibility

- Can ordinary state checks, RF lanes, and CostBand progress participate in one EML/binding bundle without reclassification?
- Can brake-without-rewind behavior remain conservation-correct?
- Which cases truly require Vector CostBand?

## 19.5 Performance

- Can evaluation ride already-hot field/reduction passes?
- Are sparse gathers, not recursive metadata, the real dominant cost?
- Can SoA layout, template bucketing, and JIT fusion keep memory traffic coherent?
- Can CPU traffic remain sparse semantic/structural deltas only?

## 19.6 Determinism

- Does flattening preserve canonical semantics independent of physical row order?
- Can activation avoid semantic dependence on nondeterministic atomic append order?
- Does generation pacing fully eliminate same-dispatch recursive dependency?
- Can CPU semantic-shadow naming be proven incapable of changing GPU behavior?

---

# 20. Falsifiers

The ActionBand design should be remanded if any of these are demonstrated.

### F1 — peer action authority is required

A broad ordinary action class requires a second authoritative goal/event/action service beside ActionBand.

### F2 — recursion requires imperative scheduling

Correct behavior requires persistent `next_step`/retry/task scheduling rather than target discrepancies and world state.

### F3 — recursion cannot flatten

Correct semantics require runtime pointer recursion, CPU child scheduling, or same-generation recursive execution that cannot be expressed as stable GPU tables/current-next state.

### F4 — EML must become imperative

Useful ActionBands require arbitrary side-effecting EML, runtime bytecode creation, string/domain dispatch, or unrestricted memory mutation rather than pure EML plus bound emissions.

### F5 — multisource semantics require duplication

A real action cannot be expressed without ActionBand inventing a second resource/property classification system beside RF/property/CostBand.

### F6 — recursive state cannot collapse

Resolved subordinate ActionBands must remain permanently resident merely to preserve correctness rather than their consequences becoming ordinary world state.

### F7 — performance requires population-wide action scans

The only workable implementation iterates all possible actions over all SimThings rather than sparse active registrations/shared templates.

### F8 — GPU storage growth requires CPU semantic selection

Additional ActionBand capacity can be provided only if CPU code decides which semantic child/action to create. Storage growth itself is permitted; CPU semantic action choice is not.

### F9 — topology requires a domain planner

A major target-resolution class cannot use admitted topology/PALMA or direct-state base cases and instead requires a privileged domain planner.

### F10 — CPU ActionBand authority is required

Correct behavior requires CPU-side ActionBand evaluation, continuous numerical mirroring, goal selection, crossing decisions, recursive scheduling, or dispatch keyed by human-readable designation.

---

# 21. Implications for 0.0.8.7 review

This document fills the architectural space before movement or other derived actions are allowed to freeze as peer vocabulary.

The engineering dependency is:

```text
GPU-native ActionBand intrinsic door / tables / semantics
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

Any temporary placeholder/door used during rollback must therefore be a real GPU path through which the derived consumer is constructed/evaluated, not kabuki beside the old peer facility.

The already-proven gradient-derived authority substrate remains useful evidence. It belongs underneath ActionBand.

---

# 22. Core deliverable

The complete ActionBand conception is:

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
             │                (GPU templates /
             │                 sparse instances)
             └──────────┬──────────┘
                        ▼
             band EML numerical program
                        │
                        ▼
             fixed emission-binding table
                        │
          ┌─────────────┼─────────────┐
          │             │             │
      ordinary RF    CostBand     overlays/events/
                                  child-next activation
          │             │             │
          └─────────────┼─────────────┘
                        ▼
               partial consequence
                        │
                 GPU next state
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
                inactive/collapsed

     ALL NUMERICAL AUTHORITY ABOVE: GPU
                        │
                        ▼ sparse semantic/structural deltas only

                 CPU SEMANTIC SHADOW
            names / durable identity / history
              UI / diagnostics / persistence
              existing structural boundary
          (never ActionBand decision authority)
```

And the corresponding physical recursion is:

```text
AUTHORING / SEMANTIC VIEW
Parent ActionBand
 ├─ child A
 ├─ child B
 └─ child C

          ↓ admission / lowering

GPU PHYSICAL VIEW
shared templates
+ sparse instance rows
+ dependency spans
+ current/next activation state
+ shared EML program ids
+ fixed emission bindings

          ↓ generation t
parent writes child-next activation

          ↓ barrier

generation t+1
children evaluate as ordinary parallel ActionBand rows
```

The central architectural result is:

> **ActionBand is the fractally recursive, GPU-native event-execution facility of the base SimThing. It turns tension between current and desired STEAD state into generation-paced, band-emitted action using existing EML, PALMA, RF, CostBand, overlay, and boundary authorities. Recursive authoring does not imply recursive execution: subordinate ActionBands lower to stable GPU templates, dependency spans, sparse instance state, and next-generation activation. EML band payloads remain pure bounded numerical programs whose results flow through fixed admitted emission bindings. The CPU never executes the ActionBand; it holds only the semantic shadow by which opaque GPU identities and GPU-produced boundary deltas become human-readable, persistent, and presentable.**

That is the specification Fable and engineering should review.
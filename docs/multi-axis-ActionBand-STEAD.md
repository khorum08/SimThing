# Multi-Axis ActionBand + STEAD
## Intrinsic recursive GPU event execution over STEAD value fields, PALMA routes, RF, EML, and CostBand

> **Status: OWNER-RULED ANCHOR CANDIDATE — READY FOR DA PROMOTION + 0.0.8.7 PHASE-7 AMENDMENT.**
>
> Owner ruling, 2026-08-08: **ActionBand is a first-class intrinsic SimThing facility.** It is part of the base recursive stem-cell definition, inert by default, and is the generic facility by which a SimThing listens to recursively produced state, compares that state with desired conditions, follows a lawful STEAD/PALMA route toward resolution, and emits executable consequences as authored bands are crossed.
>
> **Execution authority is GPU-only.** ActionBand numerical state and execution live entirely on the GPU. The CPU carries only the semantic shadow needed for human-readable designation, durable identity/persistence, diagnostics/presentation, and existing structural-boundary work. A CPU ActionBand evaluator, planner, scheduler, continuous mirror, or semantic decision path is forbidden.
>
> **ActionBand band crossings are the existing Phase-5 sealed band-crossing/threshold mechanism.** ActionBand does not mint a second crossing detector, listener framework, event comparator, or parallel threshold state machine. Its band descriptors lower to the same anchored threshold registrations and fused write-door crossing derivation that already produce `BandCrossingDelta`.
>
> This revision incorporates the 2026-08-08 Fable 5 Max review. The remaining fenced questions are implementation/research questions, not open semantic holes in the base facility.
>
> The prior workshop disposition that attempted to infer ActionBand from already-landed domain witnesses is not the governing premise. The direction is inverted:
>
> **ActionBand is intrinsic. Domain behaviors are derived/vendorized uses of ActionBand.**
>
> Physical movement is a spatial witness and later vendorization of ActionBand, never a peer core facility or the source from which the general case is inferred.

---

## 0. Executive definition and physical feasibility verdict

An **ActionBand** is an intrinsic, normally inert facility on every SimThing that represents an unresolved transition between:

1. the SimThing's **current coordinate** on one or more admitted STEAD value axes; and
2. a **desired target point, interval, region, locus, reachability set, or other closed admitted target form** on those same observables/topologies.

The ActionBand derives displacement between current and target state, observes whether that displacement is improving or worsening when an admitted previous-generation plane exists, evaluates the **stakes** of leaving the displacement unresolved, and uses admitted EML plus PALMA where topology exists to determine lawful progress toward the target.

Authored **bands** are ordinary Phase-5 threshold registrations over ActionBand observables. Crossing a band evaluates an optional admitted EML program. The numerical result is routed through pre-admitted GPU emission bindings into ordinary RF, CostBand, overlay/directive, subordinate-ActionBand activation, telemetry, or sealed structural-request surfaces.

Most ActionBands are expected to be depth 1 or depth 2. Richer multiband and recursively nested forms are lawful, but complexity is pay-for-play and bounded at admission.

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
 closed target form G(t)
          ↓
 displacement D(t)
 optional velocity dD/dt
 stakes Σ(t)
          ↓
 EML valuation / deficiency resolution
          ↓
 PALMA-informed route where topology exists
          ↓
      ActionBand progress
          ↓
 existing sealed band crossing
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

### 0.1 Feasibility under the all-GPU premise

The recursive ActionBand concept remains feasible **because recursion is semantic, not a runtime call stack or dynamically allocated object tree**.

The physically lawful interpretation is:

> **Recursive ActionBand authoring lowers to a sparse, pre-admitted GPU dependency table plus compact mutable state. Parent/child recursion is unrolled across generations and represented by activation bits, stable indices/spans, parameter rows, and ordinary GPU buffers.**

Likewise, an EML band payload is not a CPU callback and does not become an imperative shader mini-language with arbitrary side effects.

> **An EML payload is a bounded admitted numerical program. Its result is consumed by pre-admitted numeric emission bindings that write only to existing GPU-authoritative surfaces.**

The design therefore does **not** require:

- runtime recursive function calls on GPU;
- CPU child scheduling;
- CPU event dispatch;
- GPU-side authoring of new EML bytecode;
- arbitrary pointer-linked ActionBand trees;
- per-band human-readable dispatch names;
- unbounded runtime child fanout; or
- an unbounded GPU heap of semantic action objects.

Those shapes are disqualifying.

### 0.2 GPU matrix/table means table-driven vector execution, not one literal dense GEMM

ActionBand is designed for the GPU's table/vector execution model, but its semantics do not require every stage to be one dense matrix multiply.

The physically accurate target is:

```text
packed SoA tables
+ sparse logical bindings
+ shared EML/JIT programs
+ existing GPU field/reduction passes
+ existing threshold compare/emission
+ compact current/next-state writes
```

The invariant is **GPU-resident, vectorizable, table-driven execution with no CPU semantic interposition**.

---

## 1. Constitutional placement: ActionBand is the SimThing `act` facility

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

### 1.1 GPU numerical authority / CPU semantic shadow

ActionBand is implemented entirely in the GPU numerical regime.

The GPU owns, evaluates, and advances all ActionBand numerical authority, including as applicable:

```text
current coordinate / observed bindings
target numeric bindings
resultant STEAD inputs
displacement
admitted velocity / delta-to-target
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

Human-readable domain names exist only in that CPU semantic shadow / authoring-presentation layer. GPU ActionBand programs and descriptors operate on sealed numeric/logical bindings, opaque ids, offsets, ranges, thresholds, and admitted EML programs; they do not branch on strings, domain nouns, or human-readable action names.

> **GPU computes and decides ActionBand numerical state. CPU remembers and names the semantic consequences.**

A structural consequence remains lawful when the GPU emits a sealed request/delta and the existing CPU boundary performs the mutation. The boundary executes structure; it does not re-evaluate the ActionBand decision.

---

## 2. The semantic coordinate model: Stellaris-style axes generalized

The useful lesson from the Stellaris ethics web is geometric, not domain-specific:

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

### 2.1 Resultant bipolar scalar law

Where an axis is truly bipolar, it is one bounded degree of freedom:

\[
x_k \in [L_k,U_k]
\]

Ordinary state, falloff influence, overlays, transient conditions, and other admitted transforms move the value toward one pole or the other.

Conceptually:

\[
x'_k = \operatorname{Bound}_k\left(x_k + \sum_i \Delta_{i,k}\right)
\]

**The resultant is authoritative.** If equal and opposite influences produce `0`, the receiving SimThing experiences that axis as `0`. There is no intrinsic obligation to preserve hidden contest magnitude. If quiet neutrality and cancelled strong pressure must differ, that difference is another authored observable/axis.

A SimThing can therefore be neutral on one ActionBand and highly displaced on another.

#### 2.1.1 Bipolarity is not RF conservation

A bipolar semantic coordinate is not automatically a conserved RF resource. Moving toward one pole and away from the other describes one bounded degree of freedom; it does not imply world-total conservation. Actual conservation remains RF authority.

### 2.2 Strong axes, emergent semantics

Primitive axes should carry independent causal information. Higher-order named phenomena should normally emerge from combinations and trajectories through those axes.

A city may author primitive values such as housing pressure, condition, rent, employment access, labor availability, capital, demand, safety, pollution, services, accessibility, and congestion. Terms such as gentrification, blight, prosperity, food desert, suburbanization, or industrial decline remain derived observations unless they contain genuinely independent causal state.

---

## 3. Recursive SimThing state produces STEAD coordinates

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

### 3.1 STEAD propagation

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

### 3.2 Axis/channel budget is admission-bounded

The semantic basis is open across authoring but **closed for a running session/theater**.

Each field theater/session admission declares the maximum admitted semantic/STEAD channel budget it will materialize. ActionBand templates bind only to channels admitted inside that budget. Runtime code may not mint new semantic axes or silently grow a field bundle because a new ActionBand asks for one.

Exceeding the declared axis/channel budget is a build/admission error or a storage-deferral question handled by existing residency/tiling law; it is never permission to allocate an unbounded side table.

No universal numeric constant is specified here. The law is that the budget is explicit, sealed at admission, and consumed by the same physical resource-accounting discipline as other GPU-resident tables.

Frequently reused combinations may be cached as derived fields, but their authoritative meaning remains primitive inputs plus admitted EML, and a cached derived field consumes the same declared channel budget.

---

## 4. Target, displacement, velocity, and stakes

An ActionBand exists because current state differs from a desired state.

Let `X_{s,b}(t)` be the current coordinate relevant to ActionBand `b` and `G_{s,b}(t)` its target representation.

### 4.1 Target sources

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

A target may move over time without requiring a retarget state machine.

### 4.2 Target forms are a closed admitted vocabulary

A generic “predicate-defined region” is too powerful to be a primitive target form because membership alone does not tell the GPU how to move toward the set. ActionBand therefore admits a **closed target-form vocabulary**. The exact Rust enum name is rung-local; the semantic variants are closed here:

| Target form | Meaning | GPU displacement/projection lowering |
|---|---|---|
| **Point** | exact scalar/vector coordinate | componentwise `G - X` |
| **ScalarBound** | `x >= t`, `x <= t`, or equivalent sealed comparator | zero when satisfied; signed scalar distance to bound otherwise |
| **Interval** | scalar `x in [lo,hi]` | zero inside; distance to nearest bound outside |
| **AxisAlignedBox** | multidimensional independent intervals | componentwise clamp/projection onto box |
| **LocusRadius** | admitted topology locus plus arrival radius | PALMA/topology distance to locus; zero inside radius |
| **PalmaReachableSet** | admitted reachable target set represented by a PALMA potential/distance field | use the sealed PALMA distance/potential already produced for that set |
| **EmlProjectedSet** | authored acceptable set not covered above | requires **both** an admitted membership predicate and an admitted displacement/distance projection program; predicate-only targets are illegal |

Every admitted target form must therefore provide two total GPU operations over its admitted domain:

```text
satisfied(X) -> bool/0|1
project_or_distance(X) -> bounded displacement/progress representation
```

No target form may require an iterative solver, CPU fallback, arbitrary graph search, or runtime semantic object construction.

New target forms are constitutional vocabulary additions and follow the normal DA/admission growth law; they do not appear ad hoc inside a vendor implementation.

### 4.3 Displacement

For a Point target:

\[
D(t)=G(t)-X(t)
\]

For the other closed forms, `D` is the deterministic projection/distance defined by the target-form admission above.

The important law is not one universal norm; it is that **the target form carries its own bounded, admitted GPU lowering, so no implementation agent has to invent `Π_G` at runtime.**

### 4.4 Velocity requires an admitted previous-generation plane

Useful quantities include:

\[
V_X(t)=X(t)-X(t-1)
\]

and:

\[
V_D(t)=D(t)-D(t-1)
\]

but velocity is lawful **only when the observed value has an admitted previous-generation plane or an explicitly admitted previous-plane derived representation**.

An ActionBand template that references velocity over an observable with no previous-generation representation fails admission. It may not cause:

- an ActionBand-specific CPU history cache;
- an implicit per-band GPU history allocation;
- a hidden second evaluation path; or
- an assumption that every derived EML value is automatically retained.

If a model needs velocity, it pays for the previous-plane representation explicitly at admission. Physical lowerings may optimize or recompute only when they preserve that admitted semantic source exactly.

### 4.5 Stakes

Displacement answers “how far from desired state?” Stakes answer “how consequential is it to remain unresolved?”

\[
\Sigma_{s,b}(t)=S_b\left(D,V_D,R_s,\Phi,overlays,reserves,deficits,history,...\right)
\]

where `S_b` is admitted GPU EML.

A useful conceptual relationship is `P_b ~ Σ_b * ||D_b||`, but no formula is mandated.

> **Displacement supplies tension. Stakes supply urgency/consequence. Bands turn meaningful tension, urgency, or progress into emissions.**

---

## 5. Band semantics and EML payloads

A band is an authored threshold surface over any admitted ActionBand observable.

Possible operands include displacement magnitude, one axis coordinate, stakes, admitted velocity, route distance, PALMA impedance, resource accumulation, CostBand progress, construction completion, elapsed time, or another admitted derived value.

Band segmentation is authored semantic structure. It is not required to normalize to `[0,1]` or use equal slices.

### 5.1 ActionBand crossings ARE the Phase-5 sealed crossing machinery

This is a binding anti-fragmentation law.

Phase 5 already landed:

- anchored emission-band ladders/threshold registrations;
- the fused write-door derivation of `BandCrossingDelta`;
- generation-stamped threshold/event egress;
- and the CostBand use of the same crossing operands.

**ActionBand bands compile to and ride those same threshold registrations.** The ActionBand template/instance identity and its EML/emission-binding span are metadata attached to the existing registration/crossing identity; crossing detection itself remains the one sealed mechanism.

The lawful shape is:

```text
ordinary anchored value/write
        ↓
Phase-5 fused crossing derivation
        ↓
BandCrossingDelta / sealed crossing identity
        ↓
ActionBand template/instance binding
        ↓
GPU EML + emission bindings
```

Unlawful shapes include:

```text
ActionBandCrossingDelta         // rival crossing record
ActionBandThresholdScanner      // rival comparator pass
ActionBandListenerManager       // listener framework beside anchors
CPU ActionBand crossing loop    // second authority
```

A second crossing detector is a constitutional violation even if it produces identical values.

### 5.2 Default depth and crossing behavior

Expected common shape:

```text
0 bands   → inert
1 band    → ordinary event/action trigger
2 bands   → trigger + completion, or warning + action
N bands   → richer authored progression/escalation/auditing
```

Baseline behavior is edge/crossing driven: a threshold crossing emits once; merely remaining beyond the threshold does not emit duplicates. Recrossing behavior follows authored lifecycle/hysteresis.

### 5.3 EML payload purity / bound emission law

The phrase **EML payload** must not be interpreted as an imperative callback.

EML remains the already-admitted bounded numerical ISA. A band payload has two conceptual pieces:

```text
1. EML program
   reads admitted GPU bindings
   computes bounded numerical result(s)

2. emission-binding table
   routes those result(s) to pre-admitted destinations
```

Destinations may include:

```text
ordinary property/result column
overlay numeric parameter/activation state
RF claim quantity
CostBand input/progress binding
subordinate ActionBand next-activation/parameter state
sealed structural-request buffer
telemetry/event emission buffer
```

EML does **not** gain arbitrary mutation opcodes, dynamic pointers, string dispatch, runtime graph construction, or runtime template minting merely because it is attached to an ActionBand band.

The physical sequence is:

```text
existing sealed band crossing
    ↓
execute admitted EML program on GPU
    ↓
obtain numeric result(s)
    ↓
apply fixed emission bindings
    ↓
write ordinary GPU next-state / claim / activation / request buffers
```

Many bands may share the same EML program and emission shape. The executor is free to bucket and JIT/fuse by program/binding shape.

A band may have several consequences without requiring imperative side effects: a crossing gates or parameterizes a fixed emission bundle. Distinct numerical outputs may use several result slots/program blocks or another faithful admitted lowering.

---

## 6. GPU physical model: sparse tables, bounded recursion, and one admission door

This section is load-bearing for the all-GPU architecture.

The semantic model is recursive. The physical model is **flat, indexed, sparse, admission-bounded, and domain-nameless**.

A plausible implementation family is:

```text
ActionBandTemplate[]      immutable/admitted program shape
ActionBandInstance[]      sparse active numerical instances
BandDescriptor[]          existing threshold registration binding + program spans
DependencyBinding[]       parent/child template or instance relations
EmissionBinding[]         result destination bindings
StateCurrent[]            active/progress/crossing/parameter state
StateNext[]               next-generation state
```

This is an architectural shape, not a frozen ABI.

### 6.1 Templates versus instances

A template contains immutable admitted structure shared by many instances:

```text
observation binding shape
closed target form + target binding shape
existing threshold-registration/band ladder bindings
EML program ids/offsets
emission-binding spans
subordinate template dependency span
lifecycle law
axis/channel span
max active subordinate count
```

An instance carries only mutable numerical state and parameter bindings required by one active ActionBand.

Human-readable names are not present in either hot structure.

### 6.2 Sparse registration

“Every SimThing has ActionBand capability” does **not** imply fixed physical storage for every possible action on every SimThing.

The desired scaling law is:

\[
O(\text{active/materialized ActionBand instances})
\]

not:

\[
O(\text{all SimThings} \times \text{all possible actions})
\]

A SimThing with no active ActionBands owns no hot instance rows.

### 6.3 Recursion becomes dependency state, not pointer recursion

Semantic authoring may say:

```text
Parent
 ├─ Child A
 ├─ Child B
 └─ Child C
```

Physical lowering represents this with stable dependency spans and state bits. No runtime recursive function call is needed. No GPU thread follows a pointer-linked semantic tree until completion. No CPU scheduler walks children.

### 6.4 Generation pacing temporally unrolls recursion

```text
generation t:
    parent evaluates
    parent output sets child-next active/parameter state

barrier / state swap

generation t+1:
    child evaluates like any other active ActionBand
```

A child terminal result similarly becomes ordinary next-state observed by the parent later.

Therefore every active ActionBand can normally be evaluated as an independent GPU row against `StateCurrent`, writing `StateNext` and ordinary output buffers.

### 6.5 Child activation is not semantic object construction

Activating a subordinate ActionBand means:

> **Set the next-generation active/parameter state of a pre-admitted subordinate template/slot.**

It does not mean `new ActionBand`, arbitrary object allocation, compiling new EML, attaching pointer children, or running the child immediately.

A pre-admitted child template may receive GPU-computed numeric parameters such as target locus, threshold, resource quantity, or logical binding.

If active instance capacity itself must grow, that is storage/residency/structural capacity work. The GPU may authorize a bounded structural/storage request through the existing boundary. The CPU may materialize storage/identity according to ordinary law, but it does not choose the child semantic template or decide why it is needed.

### 6.6 Bounded subordination is explicit at admission

Recursive semantics do not license unbounded runtime fanout.

Every ActionBand template declares a **maximum concurrently active subordinate count** and a closed dependency/template span at admission. The session's ActionBand instance capacity is likewise an admitted GPU-residency budget. A child activation outside the admitted span or beyond the template/session bound is impossible or fails closed; it never silently appends an arbitrary semantic object.

No universal numeric child cap is imposed here. The important laws are:

- the bound is explicit authored/admitted data;
- it is frozen for the running session/template;
- physical storage is budgeted before execution;
- runtime semantics cannot expand the vocabulary/fanout beyond the admitted span;
- exceeding pre-granted storage is a structural/residency event, not a semantic fallback to CPU.

This is the ActionBand form of the existing bounded-capacity discipline.

### 6.7 Deterministic activation cannot depend on atomic append order

Base recursion should prefer preassigned subordinate slots, fixed spans, stable logical keys, next-state activation bits, or another canonically ordered GPU emission structure.

If a future dynamic allocator uses atomics/compaction internally, physical append order may never become semantic order.

### 6.8 The admission door lives at session-build `simthing-spec`, not in the driver

ActionBand templates are authored data and enter the engine through **one session-build admission path in `simthing-spec`**, alongside the other authored/admitted SimThing structures. The admitted product is frozen into session state and lowered to domain-free numeric GPU template/descriptor tables before execution.

The exact Rust type/function name is the responsibility of the first 7.* implementation rung, but the location and authority are fixed:

```text
ClauseThing / direct spec authoring
        ↓
Scenario/SimThing spec data
        ↓
simthing-spec session-build ActionBand admission  ← THE ONE DOOR
        ↓
sealed ActionBand template admission product on session state
        ↓
kernel/GPU numeric table lowering
```

Forbidden alternatives:

- driver-owned ActionBand registry;
- runtime semantic template minting;
- CPU event-handler registration as ActionBand authority;
- a second ActionBand admission path in `simthing-sim` or `simthing-driver`;
- a vendor implementation defining a target form/template shape the session admission door cannot express.

Mid-session **activation/parameterization of already-admitted templates** is ordinary GPU state. Mid-session minting of a new semantic template/program is not.

---

## 7. PALMA supplies the route where a route exists

ActionBand knows a current condition and a target condition. Reducing displacement may require navigating admitted topology.

Where topology exists, PALMA provides the lawful route/impedance structure. This includes physical space, LinkGraph relations, trade/supply accessibility, service access, recursive tree relations, admitted progression ladders, and other existing topology.

ActionBand does not mint a generic `ActionGraph`.

### 7.1 Route degeneracy

A depth-1 capability/state check requires no graph. Its route degenerates to ordinary state observation/progress toward its admitted target form.

PALMA is used where target resolution is topological; direct property/threshold ActionBands remain lawful base cases.

### 7.2 Local minima and adversarial navigation remain fenced

The base definition does not settle local-minimum escape, adversarial multi-field navigation, starvation/livelock, same-generation convergence, or coordinated Vector CostBand clearing across several independently contested RF arenas.

Those remain later probes. This is scope discipline, not doubt about ActionBand.

---

## 8. Multisource action: inherited structure, native semantics, and the 8.x fence

Capability-tree archaeology established the important semantic shape:

1. one transition may depend on several independent conditions;
2. those conditions need not share semantics;
3. progress and eligibility are distinct;
4. completed/paid progress should not be erased merely because another prerequisite remains unresolved; and
5. consequence activation occurs when the required conjunction becomes executable.

ActionBand therefore does not reopen whether heterogeneous requirements can compose.

### 8.1 Do not duplicate RF/property semantics

ActionBand binds to existing authorities.

A property check remains an observation. A conserved RF lane remains RF. A sink remains CostBand. Residency/capacity retains its own semantics. Transfer remains transfer. Time remains ordinary state unless modeled as scarce.

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

### 8.2 Braking means preserve native resolved state; it does not pre-invent scarce holding

Default inherited law:

> **When one unresolved requirement blocks the next band, already-resolved progress remains resolved according to the native semantics of its underlying binding. The parent brakes rather than rewinding completed work.**

This applies immediately to:

- nonconsuming state/capability predicates;
- ordinary accumulated progress already represented as world state;
- ordinary scalar CostBand remainder/progress where the existing sink semantics already preserve it; and
- other native persistent state.

It does **not** authorize 7.* to invent persistent holding of independently contested scarce grants across several RF arenas.

### 8.3 Sequencing contract with 8.1 / 8.2 / Vector CostBand

The live 0.0.8.7 order places ActionBand before the generic contention/conservation rungs. The dependency is therefore explicit:

**During 7.***:

- ActionBand may emit ordinary RF claims through existing lanes;
- ordinary scalar CostBand sinks/progress are lawful;
- multisource bundles mixing state checks with ordinary scalar resource work are lawful where existing RF/CostBand semantics are sufficient;
- no new ActionBand-specific holding account or cross-arena transaction subsystem may land;
- a template whose required semantics depend on atomic common-depth commitment across multiple independently contested scarce lanes **fails/defer-closes admission as unsupported**, rather than guessing a hold policy.

**At 8.1 `CONTENTION-CONSERVATION-JUDGE-0`:**

- ActionBand-originated claims are ordinary RF claims and enter the judge's declared universe;
- the judge includes existing `in_flight`/seam balances exactly as already required;
- no ActionBand-specific conservation oracle is minted.

**At 8.2 `CONTENTION-ARENA-EXECUTED-0`:**

- ActionBand claims clear through the generic executed claim→clear→disburse path;
- trivial and contested clears remain one mechanism;
- unresolved claim `U` remains distinct from CostBand remainder `R`;
- generation pacing remains non-negotiable.

**After those prerequisites**, `VECTOR-COSTBAND-PROBE-0` may test atomic common-depth multi-lane commitment, persistent provisional holding, fairness/livelock, and the already-fenced local-minimum/adversarial-route questions.

This sequencing prevents ActionBand from silently becoming the transaction subsystem that 8.x exists to define.

### 8.4 Multisource requirement as one-band payload

A depth-1 ActionBand may use one EML program as an all-of gate over ordinary state plus currently executable native resource inputs.

Example:

```text
Goal: reach Shroud coordinate

band EML reads:
    psionic_navigator >= 1
    shroud_access >= 1
    scalar energy/fuel CostBand executable
```

The capability checks are observations; energy/fuel remains RF/CostBand.

No requirement is forced into RF merely because it participates in the same ActionBand.

---

## 9. Recursive ActionBands

ActionBand is structurally recursive because SimThing itself is structurally recursive.

A band may cause subordinate ActionBands to become active on the same SimThing's intrinsic facility when unresolved deficiencies themselves require target-seeking action.

Under the GPU physical model, this means **activation of pre-admitted subordinate templates/instances**, not runtime construction of semantic action objects.

### 9.1 Parent unresolved-state model

Let a parent `P` observe subordinate unresolved quantities `u_i(t) >= 0`, with `u_i=0` meaning that subordinate target is satisfied.

\[
U_P(t)=[u_0(t),u_1(t),...,u_n(t)]
\]

is an ordinary GPU input to parent EML:

\[
q_P(t)=F_P(D_P,V_{D_P},\Sigma_P,U_P,\Phi,R,...)
\]

The common all-of rule is only one authored case. EML may express substitution, quotas, weighted satisfaction, or other lawful combinations.

### 9.2 Nested discrepancies, not imperative tasks

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

### 9.3 Siblings may resolve concurrently

Subordinate ActionBands may progress independently where resources/topology permit. The parent observes their result vector instead of hardcoding `A then B then C`.

### 9.4 Recurse and collapse

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

### 9.5 Generation-paced recursion

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

## 10. Multisource requirements and recursive ActionBands collapse into one form

A heterogeneous requirement may lower in one of two ways while preserving one semantic model.

### Trivial requirement

If the condition has no independent lifecycle, inline it into parent EML.

### Stateful target-seeking requirement

If satisfying the condition requires its own target, STEAD interaction, resources, multiple bands, duration, or subordinate requirements, activate a subordinate ActionBand template.

Thus a parent can mix:

```text
inline capability checks
inline property predicates
native RF/CostBand requirements
materialized subordinate ActionBands
```

without a parallel planner or requirement engine.

### 10.1 Colonization witness

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

### 10.2 Semantic route discovery remains EML territory

Complex goals such as colonization or supply-chain creation may require EML to determine which deficiencies remain and which pre-admitted subordinate templates should become active.

EML computes deficiency/gating values; fixed subordinate-binding tables map those results to allowable child activations. There is no universal planning graph.

---

## 11. Events and directives are ActionBand emissions/inputs

An event is often simply a meaningful **existing sealed band crossing** emitted from changing state and bound to an ActionBand payload.

A directive is ordinary received state/overlay that may deform the target, alter stakes, change a threshold, affect EML inputs, or activate an admitted ActionBand template.

```text
food_security falls below threshold
        ↓
Phase-5 sealed GPU crossing
        ↓
ActionBand binding resolves opaque template/instance
        ↓
GPU EML evaluates ordinary state/RF
        ↓
fixed emissions make response actionable
```

CPU-readable event names are semantic-shadow projections of opaque GPU identities after the GPU has emitted a crossing.

---

## 12. Existing facilities retain their authority

| Mechanism | Authority |
|---|---|
| **STEAD** | recursively produced/shared field coordinates and propagation |
| **PALMA** | topology-aware potential/impedance routing where a route exists |
| **EML** | pure admitted numerical valuation, target/deficiency reduction, band-result computation |
| **RF** | resource claims, conserved quantities, constrained clearing/disbursement |
| **CostBand** | exact resource-sink/work quantization and carried remainder |
| **Overlay** | ordinary policy/directive/transient deformation |
| **Phase-5 threshold/crossing substrate** | the **only** band-crossing detector and `BandCrossingDelta` authority |
| **Emission bindings** | fixed routing from EML results to existing numerical output surfaces |
| **Boundary authority** | structural mutation that numerical execution may authorize but never perform directly |
| **GPU ActionBand** | numerical lifecycle of unresolved target displacement and band-emitted execution |
| **CPU semantic shadow** | names, durable identity/history, diagnostics/presentation, sparse boundary deltas; never numerical decision authority |

ActionBand composes these mechanisms. It does not absorb or duplicate their semantics.

### 12.1 ActionBand + CostBand

CostBand remains:

\[
N=\left\lfloor\frac{V}{C}\right\rfloor
\]

\[
R=V-NC
\]

ActionBand may use CostBand output/progress as a band observable and may gate/parameterize CostBand through EML. It must not invent another sink mechanism.

### 12.2 Structural consequences

A band result may authorize a structural consequence, but only the existing boundary mutates structure.

GPU writes an opaque sealed request with numeric/logical bindings. CPU applies it; CPU does not reinterpret a human-readable label or re-decide the ActionBand result.

---

## 13. Movement is a derived/vendorized ActionBand implementation

Physical movement is a straightforward spatial specialization:

```text
current spatial coordinate
        ↓
ActionBand target form: locus/radius or PALMA-reachable set
        ↓
PALMA impedance/potential
        ↓
existing sealed crossing(s)
        ↓
ActionBand EML / native CostBand
        ↓
partial spatial consequence
        ↓
arrival terminal band
```

Movement is not a peer core action facility.

### 13.1 Spatial vendorization inherits the proven 7.1 fences

When movement is reintroduced as a derived/vendorized ActionBand implementation, it must **re-prove rather than rediscover** the useful substrate constraints from the invalidated movement rung:

1. **No movement-specific destination authority.** A target coordinate/locus exists only as an ActionBand target parameter or field/overlay-derived target. No peer `Destination` object/registry/planner may select behavior.
2. **No predecessor/path object.** PALMA `D`/impedance is a field; the spatial ActionBand consumes local lawful progress. The core never materializes a multi-step route.
3. **Single local structural step at the spatial ingress.** For the N4 witness, a movement consequence must fail closed unless the emitted step is one admitted adjacent edge (`manhattan == 1` equivalent). A one-band emission may not encode a hidden multi-hop path/teleport.
4. **Ambiguous locus fails closed.** An input that resolves to multiple/no authoritative spatial loci is an admission/execution error; physical row or iteration order may never pick one.
5. **Sealed crossing provenance.** Spatial progress is driven from the same Phase-5 crossing identity/`BandCrossingDelta` surface; no raw CPU field read or movement-side crossing detector may become authority.
6. **Placement is not movement semantics.** Reparenting remains column/membership/structural-boundary work and makes zero incidental physical-row-relocation assumptions; movement ordering uses logical/authored identity, never physical row order.
7. **Consumptive movement uses CostBand.** Fuel/supply/movement-budget consumption is an ordinary sink; no bespoke movement-cost path.
8. **Ownership and overlays inherit landed stem-cell laws.** Any ownership flip is the existing root rebind, not per-participant stamping; any transit/outcome overlay carries real origin and lawful lifecycle rather than permanent movement state.

The spatial vendorization is successful only when deleting its human-readable “movement” designation from the CPU shadow would leave the GPU ActionBand mechanics unchanged.

### 13.2 Worked witnesses

The same ActionBand semantics cover, as explanatory CPU-shadow labels only:

```text
get food
build a door
complete research
repair a machine
satisfy a service need
establish a colony
move a fleet to Orion IV
```

No one witness defines the general case.

---

## 14. Worked examples

### 14.1 Get food — likely depth 1

```text
current: food_security = -0.7
target:  ScalarBound(food_security >= +0.2)
stakes:  EML(displacement, reserves, velocity if retained, local conditions)
band:    existing threshold registration for actionable deficit
EML:     reads food/access/price state
outputs: native RF claims / scalar CostBand parameters
terminal: target satisfied → inactive next-state
```

### 14.2 Build a door — one or several authored bands

```text
current: build_progress = 0
target:  ScalarBound(build_progress >= 1)
bands:   0.25 / 0.50 / 0.75 / 1.00, or only terminal 1.00
```

Each band may run a shared EML program whose bound outputs drive native work/material CostBands or other state.

### 14.3 Fleet to Orion IV — spatial vendorization

```text
current: spatial locus A
target:  LocusRadius(Orion IV, arrival radius)
route:   PALMA over admitted topology
band:    existing crossing on lawful local progress
output:  numeric spatial/boundary request
terminal: arrival target satisfied
```

### 14.4 Shroud traversal — multisource one-band gate

```text
one executable band reads:
    psionic_navigator >= 1
    shroud_access >= 1
    scalar energy/fuel CostBand executable
```

The capability checks are observations; energy/fuel remains RF/CostBand. If the semantics instead require atomic common-depth grants from several independently contested scarce arenas, that template is deferred until the Vector CostBand capability exists.

### 14.5 Colonization — recursive templates

The authoring tree may show transport, population, supply, and access as child ActionBands. The GPU implementation lowers them to admitted templates plus dependency spans and evaluates active children concurrently across generations, within declared subordination and axis budgets.

---

## 15. Performance implications of the GPU-only model

The GPU-only premise strengthens some parts of the design and constrains others.

### 15.1 Recurse semantically; flatten physically

> **Recursive ActionBand structure is semantic authority. Physical execution is a flat GPU dataflow.**

No runtime recursive call stack is required or permitted.

### 15.2 Ride already-hot sweeps

The intended path is:

```text
value changes through ordinary GPU write/reduction
    ↓
existing fused threshold/crossing derivation
    ↓
no ActionBand-bound crossing → no ActionBand work
    ↓
ActionBand-bound crossing → EML program + fixed emissions
```

This is stronger than “a cheap second compare”: where possible there is **no second compare at all**. ActionBand piggybacks the sealed crossing substrate that the write door already owes.

### 15.3 Depth-1/2 fast path

Because most ActionBands are expected to be depth 1 or 2, physical layout should favor shared inline band descriptors and shared EML program ids for the common case.

Deep recursive authoring should not force deep per-instance state.

### 15.4 Inline trivial children

Simple prerequisites compile into parent EML. Only children with independent lifecycle require instance rows.

```text
inline predicate
    ↓
materialized depth-1 ActionBand
    ↓
multiband / recursive ActionBand
```

All execute on GPU.

### 15.5 Program bucketing and JIT fusion

Templates provide a natural batching key. The executor may group active rows by EML program id, band/emission shape, binding layout, and axis/field access shape, then use interpreted or faithful JIT/fused lowerings under existing EML semantics.

### 15.6 Sparse gathers are likely the dominant cost, not recursion itself

Once recursion is flattened, the likely physical cost is gathering dispersed STEAD/RF/property inputs and writing sparse outputs. Engineering should prioritize SoA layout, binding/program grouping, locality-aware descriptor ordering, shared-ingress/workgroup caching where proven, compact active masks/lists, and avoiding duplicate field projections.

### 15.7 Current/next state removes intra-dispatch dependency hazards

Use `StateCurrent` for reads and `StateNext` for activations/progress/collapse writes where practical. This makes parent/child generation pacing explicit and avoids read-after-write recursion inside one dispatch.

### 15.8 Continuous state remains GPU-resident

Displacement, admitted velocity, stakes, progress, active bits, crossing state, child unresolved state, EML intermediates/results, claims, and terminal eligibility remain GPU-resident. CPU-visible traffic is sparse semantic/structural output only.

### 15.9 Memory risk is bounded by admitted active state, not authoring depth

Templates are shared; trivial nodes inline; stateful children materialize sparsely; subordination and axis counts are admission-bounded. The practical risk is active-instance cardinality and per-instance width, not recursive syntax itself.

---

## 16. Determinism and lifecycle

ActionBand inherits SimThing determinism laws:

- semantic ordering may never come from physical row order;
- EML uses admitted arithmetic semantics;
- ActionBand crossing detection is exactly the sealed Phase-5 threshold machinery;
- structural consequences remain behind recorded boundary authority;
- no same-generation recursive convergence exists;
- CPU semantic-shadow labels never affect numerical ordering or dispatch;
- child activation may not depend semantically on nondeterministic atomic append order;
- velocity may not read unadmitted history;
- target forms may not invoke runtime solvers outside their sealed lowering;
- template/axis/subordination capacities are explicit at admission;
- completed ActionBands collapse and do not leak permanent task-state residue.

A moving target or changing overlay simply changes GPU input state for the next generation.

---

## 17. Candidate binding laws for DA review

### 17.1 Intrinsic Action Law

> Every SimThing possesses the inert-by-default capability to host ActionBands. ActionBand is the base SimThing event/action execution facility, not a domain service and not a fifth StemThing leg.

### 17.2 GPU Numerical Authority / CPU Semantic Shadow Law

> ActionBand numerical authority exists only on the GPU. CPU state is a semantic shadow containing human-readable designations, durable logical identity/history, diagnostics/presentation metadata, and sparse GPU-produced semantic/structural deltas. Human-readable labels may never become numerical dispatch keys or CPU-side decision authority.

### 17.3 Target-Displacement Law

> An ActionBand represents unresolved displacement between current recursively evaluated state and a target expressed through the closed admitted target-form vocabulary.

### 17.4 Resultant Bipolar Axis Law

> A truly bipolar semantic axis is one bounded resultant degree of freedom. Opposing influences resolve into the current scalar; cancellation to zero is zero unless contest magnitude is separately authored as another observable. Semantic bipolarity does not imply RF conservation.

### 17.5 Stakes Law

> Stakes are the EML-derived consequence/urgency of leaving an ActionBand displacement unresolved. Displacement and stakes are distinct observables; velocity may affect stakes only when a previous-generation plane is admitted.

### 17.6 Authored Band Law

> Bands are authored threshold surfaces over admitted ActionBand observables. Band segmentation is semantic/authored, not fixed to normalized or equally spaced progress.

### 17.7 EML Payload Purity / Bound Emission Law

> An ActionBand EML payload is a bounded numerical program, not an imperative callback. EML results reach the world only through pre-admitted emission bindings into existing GPU-authoritative property, overlay, RF, CostBand, subordinate-ActionBand, telemetry, or structural-request surfaces.

### 17.8 Native Semantics Law

> ActionBand binds to existing property, RF, CostBand, overlay, STEAD, PALMA, crossing, and boundary semantics; it does not create a parallel resource, prerequisite, sink, crossing, transfer, or structural-mutation universe.

### 17.9 Point-of-Execution Law

> ActionBand owns the lifecycle from unresolved target through partial band emissions to terminal resolution. It is not merely a goal selector in front of another action engine.

### 17.10 Recursive ActionBand Law

> A band result may activate subordinate ActionBands on the same intrinsic SimThing facility. Subordinate ActionBands are nested target discrepancies, not imperative tasks; they may resolve concurrently and collapse when their target conditions become ordinary state.

### 17.11 GPU Table Recursion Law

> Recursive ActionBand semantics lower to stable GPU indices/spans, shared templates, sparse instance state, and next-generation activation/result buffers. No runtime pointer recursion, CPU child scheduling, semantic dependence on atomic append order, or dynamic GPU authoring of EML programs is required or permitted by the base design.

### 17.12 Activation-Is-Not-Construction Law

> Activating a subordinate ActionBand means activating/parameterizing a pre-admitted GPU template/instance for a later generation. If additional storage must be materialized, that is ordinary structural/residency work; CPU boundary code may provide storage but may not choose or execute ActionBand semantics.

### 17.13 Multisource Collapse Law

> Heterogeneous multisource requirements may remain inline ordinary predicates/resources or materialize as subordinate ActionBands when satisfying them has an independent target-seeking lifecycle. Already-resolved requirements are represented by ordinary world state, not permanent completed-task records.

### 17.14 Generation-Pacing Law

> Parent/child ActionBand effects propagate through admitted generation/barrier ordering. No parent→child→grandchild same-generation convergence loop is lawful.

### 17.15 Semantic-Recursion / Physical-Flattening Law

> Recursive ActionBand structure is semantic authority. Admission/execution may inline, batch, JIT, or flatten it into packed non-recursive GPU data so long as semantics, crossing behavior, lifecycle, provenance, and generation pacing remain unchanged.

### 17.16 Vendorization Law

> Domain behaviors such as physical movement are derived/vendorized uses of ActionBand. No domain implementation may become a peer core action facility or bypass ActionBand decision/execution semantics where ActionBand applies.

### 17.17 Bounded Subordination Law

> Every recursive ActionBand template declares a finite admitted subordinate/dependency span and maximum concurrently active subordinate count. Runtime semantics may activate only within that span. Exceeding physical capacity is a storage/residency event, never permission to mint new semantic children or fall back to CPU scheduling.

### 17.18 Axis Budget Law

> Every running field theater/session has an explicit admitted semantic/STEAD channel budget. ActionBands bind only to admitted channels; runtime ActionBand execution may not mint new axes or unbounded derived fields. Cached compound fields consume the same declared budget.

### 17.19 Single Crossing Surface Law

> ActionBand band crossings ARE the existing Phase-5 anchored threshold/`BandCrossingDelta` mechanism. ActionBand adds template/instance and emission bindings to that crossing identity; a second comparator pass, crossing record, listener framework, or CPU crossing evaluator is unlawful.

### 17.20 Closed Target Form Law

> Every ActionBand target uses one of the admitted target forms defined in §4.2, each of which supplies total GPU satisfaction and displacement/projection semantics. Predicate-only or open-ended target forms that require an implementation agent to invent a solver are inadmissible.

### 17.21 Previous-Plane Velocity Law

> Velocity is lawful only over an observable with an admitted previous-generation representation. Referencing velocity may not implicitly allocate history or create CPU/GPU side caches outside the admitted state model.

### 17.22 One Admission Door Law

> ActionBand semantic templates are admitted once at session build through `simthing-spec` and frozen into the session's sealed numeric template product. No driver registry, runtime template mint, or vendor-local admission authority exists beside that door.

### 17.23 8.x Scarce-Holding Fence

> Before 8.1/8.2 and the Vector CostBand follow-on, ActionBand may use ordinary RF claims and existing scalar CostBand semantics but may not invent persistent cross-arena scarce-grant holding or atomic multi-lane transaction semantics. Templates requiring that capability fail/defer closed until it is admitted.

---

## 18. Explicitly fenced questions

The base ActionBand facility is semantically complete without resolving:

1. **Vector CostBand atomicity:** exact efficient common-depth commitment across several independently contested scarce RF lanes.
2. **Holding/fairness:** persistent provisional scarce grants after the 8.x substrate exists; starvation/pathological hoarding falsifiers.
3. **Local minima / adversarial navigation:** escape and competition behavior when PALMA/STEAD target descent is not trivially monotone.
4. **Optimal GPU ABI:** exact descriptor widths, state layout, inlining threshold, active-list representation, and batching/JIT strategy.
5. **Dynamic capacity growth:** the best residency-backed mechanism when active ActionBand cardinality exceeds pre-granted capacity.
6. **Performance envelope:** exact memory/bandwidth cost under millions of active/inactive SimThings.

None licenses a CPU fallback execution model, a second threshold machine, an open target solver, or an ActionBand-owned transaction subsystem.

---

## 19. Engineering/Fable review obligations

### 19.1 Crossing integration

- Can every ActionBand band compile onto the existing Phase-5 threshold registration / fused write-door path?
- Is there exactly one `BandCrossingDelta` authority?
- Does any proposed ActionBand implementation add a second compare/scan/listener surface? If yes, remand.

### 19.2 Target admission

- Does every target fit one closed §4.2 form?
- Does every target form provide total satisfaction plus bounded displacement/projection semantics on GPU?
- Can `EmlProjectedSet` be admitted without iterative search or CPU fallback?

### 19.3 GPU physical feasibility

- Can the common depth-1/2 case be represented as compact sparse GPU descriptors?
- Can shared templates amortize EML/band structure over many instances?
- Can all recursive dependencies be represented by stable spans/current-next state rather than pointer recursion?
- Can child activation be deterministic and bounded without CPU scheduling?
- Can storage growth remain separate from semantic activation?

### 19.4 EML payload feasibility

- Can EML remain pure numerical evaluation while fixed emission bindings provide all required effects?
- Do existing EML/JIT resource limits permit useful band payloads without per-domain kernels?
- Is program bucketing sufficient to avoid pathological shader divergence?
- Does any required ActionBand use demand runtime authoring of new EML bytecode or arbitrary side effects? If yes, remand.

### 19.5 Multisource / 8.x compatibility

- Can ordinary state checks, RF lanes, and scalar CostBand progress participate without reclassification?
- Is brake-without-rewind restricted to native persistent semantics before held-grant machinery exists?
- Do ActionBand-originated claims enter 8.1/8.2 as ordinary RF claims with no special conservation/clearing path?
- Are unsupported atomic multi-arena bundles refused rather than guessed?

### 19.6 Performance and bounds

- Can evaluation ride already-hot write/crossing/field passes?
- Are axis and subordinate counts admission-bounded?
- Are sparse gathers, not recursive metadata, the real dominant cost?
- Can CPU traffic remain sparse semantic/structural deltas only?

### 19.7 Determinism

- Does flattening preserve canonical semantics independent of physical row order?
- Can activation avoid semantic dependence on nondeterministic append order?
- Does generation pacing fully eliminate same-dispatch recursive dependency?
- Can CPU semantic-shadow naming be proven incapable of changing GPU behavior?

---

## 20. Falsifiers

The ActionBand design should be remanded if any of these are demonstrated.

### F1 — peer action authority is required

A broad ordinary action class requires a second authoritative goal/event/action service beside ActionBand.

### F2 — recursion requires imperative scheduling

Correct behavior requires persistent `next_step`/retry/task scheduling rather than target discrepancies and world state.

### F3 — recursion cannot flatten

Correct semantics require runtime pointer recursion, CPU child scheduling, or same-generation recursive execution that cannot be expressed as stable GPU tables/current-next state.

### F4 — EML must become imperative

Useful ActionBands require arbitrary side-effecting EML, runtime bytecode creation, string/domain dispatch, or unrestricted mutation rather than pure EML plus bound emissions.

### F5 — multisource semantics require duplication

A real action cannot be expressed without ActionBand inventing a second resource/property classification system beside RF/property/CostBand.

### F6 — recursive state cannot collapse

Resolved subordinate ActionBands must remain permanently resident merely to preserve correctness rather than their consequences becoming ordinary world state.

### F7 — performance requires population-wide action scans

The only workable implementation iterates all possible actions over all SimThings rather than existing crossing-triggered sparse active registrations/shared templates.

### F8 — GPU storage growth requires CPU semantic selection

Additional ActionBand capacity can be provided only if CPU code decides which semantic child/action to create. Storage growth itself is permitted; CPU semantic action choice is not.

### F9 — topology requires a domain planner

A major target-resolution class cannot use a closed target form plus admitted topology/PALMA or direct-state base case and instead requires a privileged domain planner.

### F10 — CPU ActionBand authority is required

Correct behavior requires CPU-side ActionBand evaluation, continuous numerical mirroring, goal selection, crossing decisions, recursive scheduling, or dispatch keyed by human-readable designation.

### F11 — second crossing machine is required

Correct ActionBand behavior requires a crossing detector/threshold registry beside the Phase-5 sealed anchor/`BandCrossingDelta` machinery.

### F12 — target vocabulary must remain open

A required ordinary ActionBand target cannot be represented by the closed target forms without giving runtime code an unbounded projection/solver problem.

### F13 — bounded recursion is insufficient

A required ordinary ActionBand use needs semantically unbounded concurrent child fanout that cannot be admitted/budgeted ahead of execution.

### F14 — hidden history is required

A required ActionBand velocity/stakes use can be implemented only by silently retaining unadmitted prior state or introducing a CPU history cache.

---

## 21. 0.0.8.7 Phase-7 rewrite and anchor promotion plan

This document fills the architectural space previously occupied by movement-first 7.1/7.1a/7.2. The dependency direction is now:

```text
intrinsic ActionBand admission + sealed crossing binding
        ↓
GPU ActionBand execution + EML emissions
        ↓
recursive/multisource ActionBand composition
        ↓
spatial ActionBand witness/vendorization
        ↓
8.1 conservation judge
        ↓
8.2 generic constrained clear
        ↓
Vector CostBand / adversarial-local-minimum probe
```

### 21.1 Proposed replacement 7.* ladder

The DA may adjust exact numeric labels to fit ceremony, but the dependency decomposition should be preserved.

| Rung | Proposed ID | Binding scope | Exit proof |
|---|---|---|---|
| **7.1** | `ACTIONBAND-ADMISSION-DOOR-0` | Mint the one `simthing-spec` session-build ActionBand template admission door. Bind templates to the **existing Phase-5 threshold registrations/`BandCrossingDelta`** rather than a new crossing path. Land the closed target-form vocabulary, axis budget, bounded subordinate/dependency span, EML/emission-binding references, GPU-only/CPU-shadow type separation. No movement vocabulary. | A depth-1 synthetic ActionBand is admitted and bound to an ordinary anchored threshold; changing the observed value produces the existing sealed crossing and resolves the opaque ActionBand binding. A planted rival crossing path is impossible/red. Predicate-only target, unretained velocity, over-axis-budget, over-subordination, mid-session template mint all fail admission. |
| **7.2** | `ACTIONBAND-GPU-EXECUTION-0` | Land sparse GPU templates/instances, `StateCurrent/StateNext`, depth-1/2 fast shape, pure EML payload execution and fixed emission-binding lowering. All numerical authority remains GPU; CPU receives semantic/structural deltas only. Recurse-semantically/flatten-physically from birth. | CPU/interpreted/JIT or applicable faithful-lowering parity under existing EML semantics; no CPU evaluator/scheduler/mirror; one-node/depth-1 path measures against existing crossing cost; program/binding bucketing demonstrated; structural request is GPU-authorized and CPU-applied without re-evaluation. |
| **7.3** | `ACTIONBAND-RECURSIVE-COMPOSITION-0` | Land pre-admitted subordinate activation, dependency spans, collapse, concurrent sibling evaluation, generation pacing, inline trivial requirements, and native RF/scalar-CostBand multisource binding. **Explicitly fence persistent cross-arena scarce holding/atomic common-depth semantics to post-8.x Vector CostBand.** | Parent sets child-next state at generation `t`; child evaluates at `t+1`; child completion becomes ordinary state and collapses; parent later resolves. Siblings progress concurrently. Boolean/state+scalar CostBand multisource gate works. Any template requiring unadmitted atomic multi-arena hold fails/defer-closes. No runtime pointer recursion or semantic append order. |
| **7.4** | `ACTIONBAND-SPATIAL-VENDORIZATION-0` | Born-mortal/scenario-neutral spatial witness proving the old useful movement substrate **through ActionBand**, not as a peer movement facility. Re-prove §13.1 fences: ActionBand target only, PALMA field not path object, adjacent local step, ambiguity fail-closed, sealed crossing provenance, CostBand for consumption, logical identity/placement law. Production core gains no movement action vocabulary. | Field/overlay-only changes redirect the spatial ActionBand target-seeking result with zero movement-specific destination authority. Adjacent step/ambiguity mutants red. Grep/type proof: no peer movement planner/path/crossing facility in core; witness can be reaped without removing ActionBand capability. |
| **7.5** | `ACTIONBAND-SEMANTIC-SHADOW-0` | Bind generic stamped ActionBand crossings/terminal/structural deltas to existing CPU semantic shadow/readback surfaces. Human-readable designation is re-attached only after GPU authority, never used to dispatch. Retires any movement-specific readback obligation by making it an ordinary projection of ActionBand state. | Same opaque GPU ActionBand run with semantic labels changed produces bit-identical numerical results; generic readback reports stamped identity/designation after the fact; no continuous numerical mirror and no movement-specific authoritative readback path. |

### 21.2 Bind 8.1 and 8.2 in the same constitutional amendment

The Phase-7 amendment must reconcile downstream consumers under the Capability Binding Law:

- **8.1 `CONTENTION-CONSERVATION-JUDGE-0` BINDS 7.3:** ActionBand-originated RF claims are ordinary declared inputs to the conservation universe; no ActionBand judge; existing child+seam+parent/in-flight accounting remains sovereign.
- **8.2 `CONTENTION-ARENA-EXECUTED-0` BINDS 7.3:** ActionBand claims use the generic claim→clear→disburse path; no ActionBand-local clearing rule; unresolved `U` and CostBand `R` remain distinct; no same-generation re-clear.
- **Vector CostBand follow-on BINDS 7.3 + 8.1 + 8.2:** only this follow-on may admit coordinated atomic common depth / persistent provisional hold semantics across independently contested scarce lanes and the paired fairness/livelock falsifier.

### 21.3 Movement code is evidence, not architecture

Any existing/reverted movement implementation is archaeology. The durable constraints are §13.1. The new spatial witness starts from the ActionBand door and may reuse proven field/crossing/boundary substrate, but it may not preserve movement-shaped peer types merely because they existed previously.

---

## 22. Anchor promotion map and core deliverable

### 22.1 Recommended doctrine anchors

Use heading anchors; this document now uses the same `##`/`###` hierarchy as the existing doctrine table.

| Anchor id | Section | Trigger domains |
|---|---|---|
| `actionband-executive` | `## 0. Executive definition and physical feasibility verdict` | `core-0087,actionband` |
| `actionband-constitutional-placement` | `## 1. Constitutional placement: ActionBand is the SimThing \`act\` facility` | `kernel,sim,driver,actionband` |
| `actionband-eml-payload-purity` | `### 5.3 EML payload purity / bound emission law` | `kernel-eml,actionband` |
| `actionband-gpu-physical-model` | `## 6. GPU physical model: sparse tables, bounded recursion, and one admission door` | `kernel,gpu,actionband` |
| `actionband-binding-laws` | `## 17. Candidate binding laws for DA review` | `core-0087,actionband` |
| `actionband-vendorization-direction` | `## 13. Movement is a derived/vendorized ActionBand implementation` | `sim,movement,actionband` |
| `actionband-fenced-questions` | `## 18. Explicitly fenced questions` | `actionband` |

The DA should promote the document by changing its status from anchor candidate to the appropriate constitutional anchor status, adding these rows, and resyncing hashes in the same amendment. The old `actionband-meaning` anchor that points into the horizon workshop should be retired/repointed rather than left as a competing reachable meaning.

### 22.2 Core deliverable

```text
                     SIMTHING
                        │
                recursive state/RF
                        │
                        ▼
              resultant STEAD axes
             (admission-bounded channels)
                        │
              current coordinate X
                        │
            ┌───────────┴───────────┐
            │                       │
            │                closed target form G(t)
            │                       │
            └───────────┬───────────┘
                        ▼
                displacement D
          optional admitted velocity dD/dt
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
       EXISTING PHASE-5 THRESHOLD REGISTRATION
                        │
                 BandCrossingDelta
                        │
             band EML numerical program
                        │
             fixed emission-binding table
                        │
          ┌─────────────┼─────────────┐
          │             │             │
      ordinary RF    CostBand     overlays/events/
          │             │         child-next activation
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

The corresponding physical recursion is:

```text
AUTHORING / SEMANTIC VIEW
Parent ActionBand
 ├─ child A
 ├─ child B
 └─ child C

          ↓ simthing-spec session admission

GPU PHYSICAL VIEW
shared templates
+ bounded dependency spans
+ sparse instance rows
+ current/next activation state
+ shared EML program ids
+ fixed emission bindings
+ existing threshold registrations

          ↓ generation t
parent writes child-next activation

          ↓ barrier

generation t+1
children evaluate as ordinary parallel ActionBand rows
```

> **ActionBand is the fractally recursive, GPU-native event-execution facility of the base SimThing. It turns tension between current and desired STEAD state into generation-paced, band-emitted action using the existing Phase-5 crossing machinery plus EML, PALMA, RF, CostBand, overlay, and boundary authorities. Recursive authoring does not imply recursive execution: subordinate ActionBands lower to bounded admitted GPU templates, dependency spans, sparse instance state, and next-generation activation. EML band payloads remain pure bounded numerical programs whose results flow through fixed admitted emission bindings. The CPU never executes the ActionBand; it holds only the semantic shadow by which opaque GPU identities and GPU-produced boundary deltas become human-readable, persistent, and presentable.**

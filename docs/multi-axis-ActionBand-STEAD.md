# Multi-Axis ActionBand + STEAD
## Intrinsic recursive GPU event execution over STEAD value fields, PALMA potentials, Gu-Yang saturating flux, RF, EML, and CostBand

> **Status: OWNER-RULED ANCHOR CANDIDATE — REVISED FOR GU-YANG / FIELD-TRIAD DA REVIEW.**
>
> Owner ruling, 2026-08-08: **ActionBand is a first-class intrinsic SimThing facility.** It is part of the base recursive stem-cell definition, inert by default, and is the generic facility by which a SimThing listens to recursively produced state, compares that state with desired conditions, follows lawful Field-Triad state toward resolution, and emits executable consequences as authored bands are crossed.
>
> Owner corrective directive, 2026-08-09; incorporated 2026-08-10: the prior ActionBand design used STEAD and PALMA while omitting **Gu-Yang / `SaturatingFlux`**, the conserved member of the Field Triad. This revision does not patch Gu-Yang in as a name. It re-evaluates ActionBand execution with all three Field-Triad conservation classes present.
>
> **Result:** the omission was load-bearing. ActionBand does not own a throughput model, but when progress traverses a conserved, capacity-bearing channel, its physically realizable progress is bounded by the **existing Gu-Yang/RF flux result**. PALMA answers where the potential descends; Gu-Yang answers how much conserved quantity can traverse the admitted channel this generation and where that flow saturates; CostBand answers how much sink/work can be purchased from the quantity actually available. Where the Gu-Yang sweep has already materialized realized flux, ActionBand should consume that result rather than recompute throughput.
>
> **Execution authority is GPU-only.** ActionBand numerical state and execution live entirely on the GPU. The CPU carries only the semantic shadow needed for human-readable designation, durable identity/persistence, diagnostics/presentation, and existing structural-boundary work. A CPU ActionBand evaluator, planner, scheduler, continuous mirror, throughput calculator, or semantic decision path is forbidden.
>
> **ActionBand band crossings are the existing Phase-5 sealed band-crossing/threshold mechanism.** ActionBand does not mint a second crossing detector, listener framework, event comparator, saturation comparator, or parallel threshold state machine. Gu-Yang outputs are ordinary anchored field/property columns and therefore reach ActionBand through the same sealed crossing/observation substrate as other state.
>
> Companions: [`simthing_core_design.md`](simthing_core_design.md) (closed four-leg germ and Field-Triad authority), [`stead_spatial_contract.md`](stead_spatial_contract.md) (PALMA/Gu-Yang spatial contract and shared `FieldSweepRegistration`), and [`stead_stemthing_unification.md`](stead_stemthing_unification.md) (StemThing unification).
>
> Physical movement remains a spatial witness and vendorization of ActionBand, never a peer core facility. Local-minimum/adversarial navigation remains fenced; **saturation is not navigation**.

---

## 0. Executive definition and physical feasibility verdict

An **ActionBand** is an intrinsic, normally inert facility on every SimThing that represents an unresolved transition between:

1. the SimThing's **current coordinate** on one or more admitted STEAD/value/ordinary property observables; and
2. a **desired target point, interval, region, locus, reachability set, or other closed admitted target form** on those same observables/topologies.

The ActionBand derives displacement between current and target state, observes whether that displacement is improving or worsening when an admitted previous-generation plane exists, evaluates the **stakes** of leaving the displacement unresolved, and consumes the already-authoritative mechanisms needed to make lawful progress:

- **STEAD** for non-conserved propagated influence/state;
- **PALMA** for min-plus potential/impedance over admitted topology;
- **Gu-Yang / `SaturatingFlux`** for conserved realizable flux, saturation, stall, and channel throughput where progress is a conserved transport problem;
- **RF** for claims, clearing, and disbursement;
- **CostBand** for exact sink/work quantization;
- **EML** for bounded numerical projection/valuation; and
- the existing Phase-5 crossing substrate for band emission.

ActionBand does not replace any of them. It is the intrinsic `act` facility that **binds their resolved outputs to target-seeking lifecycle**.

Authored **bands** are ordinary Phase-5 threshold registrations over admitted ActionBand observables. Crossing a band evaluates an optional admitted EML program. The numerical result is routed through pre-admitted GPU emission bindings into ordinary overlay, RF, CostBand, subordinate-ActionBand activation, telemetry, or sealed structural-request surfaces.

Most ActionBands are expected to be depth 1 or depth 2. Richer multiband and recursively nested forms are lawful, but complexity is pay-for-play and bounded at admission.

The complete conceptual loop is:

```text
recursive SimThing / RF state
          ↓
  STEAD / ordinary state fields
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
 ┌──────────────── FIELD TRIAD ────────────────┐
 │ STEAD: propagated non-conserved influence    │
 │ PALMA: min-plus potential / lawful direction │
 │ Gu-Yang: conserved realizable flux / choke   │
 └─────────────────────┬────────────────────────┘
                       ↓
          native RF / CostBand feasibility
                       ↓
                ActionBand progress
                       ↓
           existing sealed band crossing
                       ↓
                GPU EML evaluation
                       ↓
          pre-admitted emission bindings
                       ↓
       overlays / RF / CostBand / boundary
                       ↓
          partial or terminal consequence
                       ↓
             new ordinary SimThing state
                       ↓
          next-generation recursive state
```

The critical Gu-Yang correction is that **ActionBand progress is not always synonymous with desired progress**. If resolution requires transporting a conserved quantity through a capacity-bearing topology, the amount that can physically advance this generation is the amount the native conserved-flow substrate realizes, not the amount ActionBand wishes to advance.

Everything through ActionBand evaluation, field consumption, crossing, recursive subordinate evaluation, and numerical consequence authorization is GPU-authoritative. CPU interaction occurs only when a sparse semantic/structural boundary delta must be remembered, presented, persisted, or applied through an already-existing CPU-owned structural boundary.

### 0.1 Feasibility under the all-GPU premise

The recursive ActionBand concept remains feasible because recursion is semantic, not a runtime call stack or dynamically allocated object tree.

> **Recursive ActionBand authoring lowers to sparse, pre-admitted GPU dependency tables plus compact mutable state. Parent/child recursion is unrolled across generations and represented by activation bits, stable indices/spans, parameter rows, and ordinary GPU buffers.**

Likewise, an EML payload is a bounded admitted numerical program, not a callback or shader-side semantic engine.

The design does **not** require runtime recursive GPU calls, CPU child scheduling, CPU event dispatch, GPU-side authoring of EML, arbitrary pointer-linked ActionBand trees, per-band string dispatch, unbounded runtime fanout, or a GPU heap of semantic action objects.

Gu-Yang integration also requires **no ActionBand-local flux solver**. The Field Triad already lowers through generic field-sweep machinery; ActionBand consumes its ordinary anchored outputs.

### 0.2 GPU matrix/table means table-driven vector execution, not one literal dense GEMM

The physically accurate target is:

```text
packed SoA tables
+ sparse logical bindings
+ shared EML/JIT programs
+ existing RF/reduction passes
+ existing FieldSweepRegistration passes
    - STEAD-class programs
    - PALMA min-plus programs
    - Gu-Yang conservative-flux programs
+ existing threshold compare/emission
+ compact current/next-state writes
```

The invariant is **GPU-resident, vectorizable, table-driven execution with no CPU semantic interposition**.

---

## 1. Constitutional placement: ActionBand is the SimThing `act` facility

The StemThing thesis remains unchanged: the base recursive SimThing carries generic capabilities inert-by-default, and specialization activates/authors those capabilities rather than introducing peer engines.

ActionBand is not a fifth StemThing leg, an `ActionThing`, event manager, behavior tree, task graph, destination manager, movement engine, combat engine, production engine, planner service, CPU goal selector, or independent lifecycle authority.

It is the intrinsic implementation of the SimThing's ability to **act on what it perceives**.

```text
SimThing
 ├─ participate  ← RF + Field-Triad participation
 ├─ act          ← ActionBand / CostBand
 ├─ originate    ← overlay/event emission
 └─ receive      ← inherited/routed overlay/RF perception/directive
```

The base SimThing remains the sole point of iteration and failure. Recursive ActionBands never escape that ownership.

### 1.1 GPU numerical authority / CPU semantic shadow

The GPU owns, evaluates, and advances all ActionBand numerical authority, including as applicable:

```text
current coordinate / observed bindings
target numeric bindings
STEAD inputs
PALMA potential/impedance inputs
Gu-Yang available/realized flux and saturation inputs
displacement
admitted velocity / delta-to-target
stakes
band thresholds and crossing state
EML payload execution
progress / CostBand interaction
recursive/subordinate dependency state
RF claim quantities and numerical commitment state
terminal satisfaction / dissolve eligibility
```

No CPU copy of those values is authoritative. No CPU loop may decide what an ActionBand wants, whether a band crossed, how much flux is available, which subordinate discrepancy is active, or how far an ActionBand progresses.

The CPU owns a semantic shadow: stable logical identity, human-readable names, authoring/debug/UI metadata, persistent categorical history, opaque-id-to-label mappings, sparse band/terminal/structural deltas, and existing structural-boundary application.

> **GPU computes and decides ActionBand numerical state. CPU remembers and names semantic consequences.**

---

## 2. The semantic coordinate model: Stellaris-style axes generalized

Complex state can be located as coordinates on a small number of causally meaningful value axes rather than encoded as a catalog of named composite states.

Examples:

```text
scarcity         ←→ abundance
threat           ←→ safety
isolation        ←→ accessibility
degradation      ←→ physical quality
job insecurity   ←→ job security
housing pressure ←→ housing abundance
```

The engine knows admitted numeric bindings and laws, not the human-readable meanings.

### 2.1 Resultant bipolar scalar law

Where an axis is truly bipolar, it is one bounded degree of freedom:

\[
x_k \in [L_k,U_k]
\]

and ordinary state, fields, overlays, and other admitted transforms move the single resultant value.

If equal and opposite influences produce `0`, the receiving SimThing experiences `0`. Hidden contest magnitude exists only if separately authored as another observable.

#### 2.1.1 Bipolarity is not RF conservation

A bipolar semantic coordinate is not automatically conserved. Actual conservation remains RF/Gu-Yang authority where the quantity is a conserved flow.

### 2.2 Strong axes, emergent semantics

Primitive axes should carry independent causal information. Higher-order named phenomena normally emerge from combinations and trajectories through those axes.

---

## 3. Recursive SimThing state produces STEAD coordinates

For SimThing `s`:

```text
R_s = {
    own ordinary properties,
    child reduce-up results,
    inherited/directive state,
    overlays,
    RF balances and unresolved pressure,
    topology observations,
    Field-Triad outputs,
    existing commitment/progress state,
    ordinary anchored observables
}
```

An admitted EML projection may produce semantic coordinates:

\[
X_s(t)=[x_{s,0}(t),...,x_{s,K-1}(t)]
\]

with `x_{s,k}(t)=F_k(R_s(t))`.

### 3.1 STEAD propagation

A coordinate can become a STEAD field channel over admitted topology. STEAD is the non-conserved Field-Triad class: propagated influence/signal whose superposition and attractor behavior are not conservation claims.

### 3.2 Axis/channel budget is admission-bounded

The semantic basis is open across authoring but closed for a running session/theater. ActionBand templates bind only to admitted channels. Runtime code may not mint new semantic axes or silently grow an unbounded field bundle.

This budget includes cached/reused Field-Triad outputs that become ActionBand observables.

---

## 4. Target, displacement, velocity, stakes, and realizable progress

An ActionBand exists because current state differs from desired state.

Let `X_{s,b}(t)` be the current coordinate and `G_{s,b}(t)` its admitted target representation.

### 4.1 Target sources

Targets may arise from baseline disposition/personality, standing conditions, scripts/time, incoming directives, overlays, resource needs/deficits, or explicit authored goals. ActionBand owns no target-manager service.

### 4.2 Target forms are a closed admitted vocabulary

| Target form | Meaning | GPU displacement/projection lowering |
|---|---|---|
| **Point** | exact scalar/vector coordinate | componentwise `G-X` |
| **ScalarBound** | scalar comparator | zero when satisfied; signed distance otherwise |
| **Interval** | scalar interval | zero inside; nearest-bound distance outside |
| **AxisAlignedBox** | multidimensional intervals | componentwise projection |
| **LocusRadius** | admitted topology locus + radius | topology/PALMA distance; zero inside radius |
| **PalmaReachableSet** | target set represented by PALMA potential | consume sealed PALMA potential |
| **EmlProjectedSet** | other authored acceptable set | requires both membership and admitted displacement/distance projection |

Every form supplies total GPU `satisfied(X)` and `project_or_distance(X)` operations. No target form may require CPU fallback, arbitrary graph search, or runtime semantic construction.

**Gu-Yang adds no target form.** Flux is an execution/feasibility observable over a conserved channel, not a target geometry. If a future proposal needs a target defined by flux topology itself rather than a target evaluated using flux state, that is a constitutional target-vocabulary addition and must be reviewed as such.

### 4.3 Displacement

For a Point target:

\[
D(t)=G(t)-X(t)
\]

Other target forms use their sealed projection/distance lowering.

### 4.4 Velocity requires an admitted previous-generation plane

\[
V_X(t)=X(t)-X(t-1),\qquad V_D(t)=D(t)-D(t-1)
\]

Velocity is lawful only when the observed value has an admitted previous-generation representation. No ActionBand-specific CPU history cache or implicit GPU history allocation is lawful.

### 4.5 Stakes

Stakes answer how consequential it is to remain unresolved:

\[
\Sigma_{s,b}(t)=S_b(D,V_D,R_s,\Phi,overlays,reserves,deficits,history,...)
\]

where `S_b` is admitted GPU EML.

> **Displacement supplies tension. Stakes supply urgency/consequence. Bands turn meaningful tension, urgency, progress, or saturation into emissions.**

### 4.6 Conserved progress is flux-bounded

ActionBand may compute or observe a desired progress quantity `q_desired`, but desired progress is not automatically executable.

When the transition requires moving a **conserved quantity through a capacity-bearing admitted topology**, Gu-Yang/RF owns the physically realizable transport:

```text
ActionBand target pressure / desired progress
        ↓
ordinary RF claim / field demand
        ↓
Gu-Yang conservative flux sweep
        ↓
available / realized flux + saturation/stall observables
        ↓
ActionBand consumes the result
```

Conceptually:

\[
q_{exec}(t) \le q_{flux}(t)
\]

where `q_flux` is native Gu-Yang/RF authority. No universal `min(...)` formula is mandated because some lanes expose available capacity while others expose already-cleared realized flux. The governing rule is stronger:

> **If native Gu-Yang/RF has already resolved realizable flux, ActionBand consumes that resolved quantity and may not reconstruct a competing throughput model.**

For non-conserved direct property progress, capability checks, semantic movement through non-capacity state, or other non-flux cases, Gu-Yang degenerates out of the path and imposes no cost.

---

## 5. Band semantics and EML payloads

A band is an authored threshold surface over any admitted ActionBand observable.

Possible operands include displacement magnitude, axis coordinate, stakes, admitted velocity, route distance, PALMA impedance, **available flux, realized flux, saturation/stall magnitude, choke pressure**, resource accumulation, CostBand progress, completion, elapsed time, or another admitted derived value.

Gu-Yang values need no new crossing mechanism: they are ordinary anchored field/property outputs and use Phase-5 crossings like everything else.

### 5.1 ActionBand crossings ARE the Phase-5 sealed crossing machinery

Phase 5 already owns anchored band ladders/threshold registrations, fused `BandCrossingDelta`, generation-stamped egress, and CostBand crossing operands.

Lawful:

```text
ordinary anchored value/write/Field-Triad output
        ↓
Phase-5 fused crossing derivation
        ↓
BandCrossingDelta
        ↓
ActionBand template/instance binding
        ↓
GPU EML + fixed emission bindings
```

Unlawful: `ActionBandCrossingDelta`, an ActionBand threshold scanner, a Gu-Yang saturation listener beside anchors, or a CPU crossing loop.

### 5.2 Default depth and crossing behavior

```text
0 bands → inert
1 band  → ordinary trigger
2 bands → trigger + completion / warning + action
N bands → richer authored progression
```

Baseline behavior is edge/crossing driven; recrossing follows authored lifecycle/hysteresis.

### 5.3 EML payload purity / bound emission law

An ActionBand EML payload is a bounded numerical program, not an imperative callback.

```text
EML program
  reads admitted GPU bindings
  computes bounded numerical result(s)

fixed emission-binding table
  routes result(s) to:
    ordinary property/result columns
    overlay numeric/activation state
    RF claim quantity
    CostBand input/progress
    subordinate ActionBand next-state
    sealed structural request
    telemetry/event buffers
```

EML does not gain arbitrary mutation opcodes, dynamic pointers, string dispatch, runtime graph construction, runtime template minting, or a private flux solver.

Many bands may share program and emission shape; faithful bucketed/JIT/fused lowerings are allowed.

---

## 6. GPU physical model: sparse tables, bounded recursion, and one admission door

The semantic model is recursive. The physical model is flat, indexed, sparse, admission-bounded, and domain-nameless.

```text
ActionBandTemplate[]
ActionBandInstance[]
BandDescriptor[]
DependencyBinding[]
EmissionBinding[]
StateCurrent[]
StateNext[]
```

ActionBand descriptors bind to existing Field-Triad output columns/registrations; they do not own copies of PALMA/Gu-Yang fields.

### 6.1 Templates versus instances

Templates carry immutable admitted observation/target shapes, threshold bindings, EML ids, emission spans, subordinate spans, lifecycle law, axis/channel span, and active-subordinate limits. Instances carry only mutable numerical state/parameters.

### 6.2 Sparse registration

The scaling law is `O(active/materialized ActionBand instances)`, not `O(all SimThings × all possible actions)`. A SimThing with no active ActionBand owns no hot instance row.

### 6.3 Recursion becomes dependency state, not pointer recursion

Semantic parent/children lower to stable spans and bits. No runtime recursive function calls or CPU scheduler are needed.

### 6.4 Generation pacing temporally unrolls recursion

```text
generation t:
    parent evaluates
    parent writes child-next activation/parameters

barrier

generation t+1:
    child evaluates as an ordinary active row
```

### 6.5 Child activation is not semantic object construction

Activating a child means activating/parameterizing a pre-admitted subordinate template/slot. Additional storage is ordinary residency/structural work, not permission for CPU semantic choice.

### 6.6 Bounded subordination is explicit at admission

Every template declares a finite subordinate/dependency span and max concurrently active subordinate count. Runtime semantics may activate only inside the admitted span.

### 6.7 Deterministic activation cannot depend on atomic append order

Prefer fixed spans, stable logical keys, preassigned slots, and current/next bits. Physical append order may never become semantic order.

### 6.8 The admission door lives at session-build `simthing-spec`, not in the driver

```text
ClauseThing / direct spec authoring
        ↓
Scenario/SimThing spec data
        ↓
simthing-spec session-build ActionBand admission
        ↓
sealed template product
        ↓
kernel/GPU numeric lowering
```

No driver registry, runtime semantic template mint, CPU handler registry, or vendor-local target authority exists beside this door.

---

## 7. The Field Triad supplies influence, route, and flux where each applies

The previous framing — “PALMA supplies the route where a route exists” — was incomplete. ActionBand operates over the existing **Field Triad**, whose members represent distinct conservation classes and therefore distinct execution questions.

| Field-Triad member | Conservation class | ActionBand question answered |
|---|---|---|
| **STEAD** | non-conserved signal / influence | what propagated state/pressure exists here? |
| **PALMA** | non-conserved min-plus potential | which local direction/potential reduces topological cost, and how far/impeded is the target? |
| **Gu-Yang / SaturatingFlux** | conserved saturating flux | how much conserved quantity can actually traverse this channel this generation, and where is it saturated/stalled? |

All three are ordinary generic field-sweep programs over admitted adjacency. PALMA and Gu-Yang already lower through `FieldSweepRegistration`; ActionBand composes their anchored outputs rather than creating an `ActionGraph` or `FluxThing`.

### 7.1 Route and flux degeneracy

A depth-1 direct state/capability check may require neither PALMA nor Gu-Yang.

A topological but non-conserved target may use PALMA without Gu-Yang.

A conserved capacity-bearing transition uses PALMA if direction/potential matters and Gu-Yang if realizable channel throughput matters. If topology has no meaningful capacity/saturation constraint, the Gu-Yang leg degenerates to no binding rather than imposing an artificial flow model.

### 7.2 PALMA remains the route/potential authority

PALMA `D` is a field, not a path. ActionBand consumes local potential/impedance; it never materializes predecessors, `came_from`, or route objects.

### 7.3 Gu-Yang remains the conserved-flow authority

Gu-Yang / `SaturatingFlux` is a conservative-flux field law over structural adjacency. It is not a border/frontline service and not an ActionBand-specific movement solver.

Its outputs — including realized/net/gross flux and sanctioned derived saturation/stall/choke observables where admitted — are ordinary fields/properties. ActionBand may observe them, gate on them, or use their resolved quantity as its progress bound.

The `stead_spatial_contract.md` 5.8 comparative projections remain authoritative examples: contest consumes Gu-Yang stall magnitude; chokepoint combines contested-border with PALMA-low-`D`. ActionBand does not rederive those facts privately.

### 7.4 Local minima and adversarial navigation remain fenced

This revision does **not** solve local-minimum escape, adversarial multi-field navigation, starvation/livelock, or same-generation convergence. Gu-Yang saturation can tell the actor that a channel is saturated; it does not choose a better route. Those navigation questions remain later probes.

---

## 8. Multisource action: inherited structure, native semantics, and the 8.x fence

ActionBand does not duplicate RF/property semantics. A property check remains an observation. A conserved RF lane remains RF. **A conserved capacity-bearing spatial/channel flow remains Gu-Yang/RF.** A sink remains CostBand. Residency/capacity retains its own semantics. Transfer remains transfer.

### 8.1 Do not duplicate native semantics

```text
band EML
   ↓
reads ordinary prerequisite/property/Field-Triad state
   ↓
computes/gates native RF claims where scarcity is real
   ↓
consumes Gu-Yang realized flux where channel throughput is authoritative
   ↓
uses CostBand for sink/work quantization
```

### 8.2 Braking means preserve native resolved state; it does not pre-invent scarce holding

Already-resolved progress remains resolved according to its native substrate. This does not authorize persistent holding of independently contested scarce grants across several RF arenas.

### 8.3 Sequencing contract with 8.1 / 8.2 / Vector CostBand

During 7.* ActionBand may emit ordinary RF claims, consume existing Gu-Yang/Field-Triad outputs, use scalar CostBand, and compose ordinary state. It may not invent ActionBand-specific holding or cross-arena transaction semantics.

At 8.1 ActionBand-originated claims enter the ordinary conservation universe. At 8.2 they clear through generic claim→clear→disburse. `U` remains distinct from CostBand remainder `R`. After those, Vector CostBand may test atomic common-depth multi-lane commitment and fairness/livelock.

### 8.4 Multisource requirement as one-band payload

One EML gate may mix nonconsuming predicates, existing Field-Triad observables, ordinary RF availability, and scalar CostBand feasibility. Nothing is forced into RF or Gu-Yang merely because it participates in one ActionBand.

---

## 9. Recursive ActionBands

ActionBand is structurally recursive because SimThing is structurally recursive. A band may activate subordinate ActionBands on the same intrinsic facility when unresolved deficiencies themselves require target-seeking action.

### 9.1 Parent unresolved-state model

Let `U_P(t)=[u_0,...,u_n]`, with `u_i=0` meaning subordinate target satisfied. Parent EML may consume `D`, velocity, stakes, `U`, STEAD, PALMA, Gu-Yang/RF observables, and ordinary state.

### 9.2 Nested discrepancies, not imperative tasks

There is no `next_step`, retry policy, success handler, failure handler, or child scheduler. There are only current unresolved discrepancies and ordinary world state.

### 9.3 Siblings may resolve concurrently

Subordinates progress independently where native resources/topology permit.

### 9.4 Recurse and collapse

Child terminal crossing → ordinary world consequence → child inactive next-state → parent later observes ordinary state.

### 9.5 Generation-paced recursion

No parent→child→grandchild same-generation convergence loop is lawful.

---

## 10. Multisource requirements and recursive ActionBands collapse into one form

Trivial conditions inline into parent EML. Stateful target-seeking conditions materialize as subordinate ActionBands. Native RF/Gu-Yang/CostBand remains native beneath both forms.

### 10.1 Colonization witness

A colonization parent may depend on transport, population commitment, staged supplies, access, and inline state checks. Any transport/supply leg that traverses a conserved capacity-bearing channel receives Gu-Yang throughput from the ordinary field/RF substrate rather than a colonization-specific transport solver.

### 10.2 Semantic route discovery remains EML territory

EML computes deficiency/gating values and selects among pre-admitted subordinate bindings. It does not construct a universal planning graph or flow solver.

---

## 11. Events and directives are ActionBand emissions/inputs

An event is often a meaningful existing sealed band crossing bound to an ActionBand payload. A directive is ordinary received state/overlay that may deform target, stakes, threshold, EML inputs, RF claim pressure, or subordinate activation.

CPU-readable names are after-the-fact semantic-shadow projections of opaque GPU identities.

---

## 12. Existing facilities retain their authority

| Mechanism | Authority |
|---|---|
| **STEAD** | recursively produced/shared non-conserved field coordinates and propagation |
| **PALMA** | topology-aware min-plus potential/impedance; route field, never path object |
| **Gu-Yang / `SaturatingFlux`** | **conserved saturating flux/throughput over admitted topology; authoritative bound on how much conserved quantity can traverse a capacity-bearing channel this generation, including admitted saturation/stall/choke observables** |
| **EML** | pure admitted numerical valuation, target/deficiency reduction, band-result computation |
| **RF** | resource claims, conserved quantities, constrained clearing/disbursement and channel identity |
| **CostBand** | exact resource-sink/work quantization and carried remainder |
| **Overlay** | ordinary policy/directive/transient deformation and actuation state |
| **Phase-5 threshold/crossing substrate** | the only band-crossing detector and `BandCrossingDelta` authority |
| **Emission bindings** | fixed routing from EML results to existing numerical output surfaces |
| **Boundary authority** | structural mutation that numerical execution may authorize but never perform directly |
| **GPU ActionBand** | numerical lifecycle of unresolved target displacement and band-emitted execution |
| **CPU semantic shadow** | names, durable identity/history, diagnostics/presentation, sparse boundary deltas; never numerical decision authority |

ActionBand composes these mechanisms. It does not absorb or duplicate their semantics.

### 12.1 ActionBand + CostBand

\[
N=\left\lfloor\frac{V}{C}\right\rfloor,\qquad R=V-NC
\]

CostBand quantizes a sink. It does not establish channel throughput. When `V` itself is supplied through a capacity-bearing conserved channel, Gu-Yang/RF may bound or realize `V` before CostBand quantizes it.

### 12.2 ActionBand + Gu-Yang

The lawful relationship is **consumer, not owner**:

```text
ActionBand/RF demand
    ↓
existing Gu-Yang/RF conserved-flow execution
    ↓
realized flux / saturation outputs
    ↓
ActionBand progress / band observables / CostBand input
```

If realized flux is already an ordinary property/field output from an already-hot sweep, ActionBand should bind to it directly. A second ActionBand throughput pass is a defect.

> **Gu-Yang remains the instantaneous conservative mechanism. Persistence of saturation/stall is authored ordinary state (for example admitted decay-memory state) and never hidden history owned by Gu-Yang, ActionBand, or a saturation listener.**

This complements §17.21's previous-plane law and §17.19's single-crossing-surface law. A request for "stall memory" therefore means an admitted state/overlay/property, not a new flux-history facility.

**Per-binding conserved-progress bound source.** No universal formula such as `min(RF_grant, flux_available, flux_realized)` is canonized, because different native lanes expose different already-resolved authorities. Instead each conserved-progress binding declares **exactly one** authoritative bound source from a closed vocabulary — semantically `None | RfGrant | GuYangAvailable | GuYangRealized`; the exact Rust spelling is rung-local. Admission rejects **double-bounding** (re-applying an RF grant against a realized flux that already includes it), **zero-bounding** (each authority assuming the other supplied the executable bound), and any vendor-local fifth source or implicit default invented outside the one ActionBand admission door. This is a binding descriptor, not a new resource taxonomy and not a target form.

**Signed progress, not magnitude.** Gu-Yang conservative flux is signed with respect to canonical edge orientation / target-relative progress. An ActionBand binding consumes flux **projected toward its authorized direction** while preserving the native canonical Gu-Yang sign/order authority; `abs(flux)` is not a generic progress lowering. Symmetric opposed demand on one channel must produce mutual stall/contest, never mutual positive progress.

**Bounded feedback on field-seeding emissions.** An ActionBand emission binding that writes into a lane seeding a field/RF recurrence is admitted under the existing bounded-feedback contract — finite decay where persistence is used, explicit bounds/clamps, no admitted positive unbounded recurrence. Generation pacing bounds update *rate*, not feedback *gain*; an authored `demand += k * stall` is a positive-feedback amplifier. This reuses existing EML/gadget admission law and mints no controller subsystem.

### 12.3 Structural consequences

A band result may authorize structure, but only existing boundary authority mutates it. CPU applies the sealed request without re-evaluating ActionBand, PALMA, or Gu-Yang.

---

## 13. Movement is a derived/vendorized ActionBand implementation

Physical movement is the canonical spatial witness because it can exercise all three Field-Triad classes without creating a movement engine.

```text
current spatial coordinate/property
        ↓
ActionBand target: locus/radius or PALMA-reachable set
        ↓
STEAD/overlays compose local conditions
        ↓
PALMA potential gives lawful local descent
        ↓
Gu-Yang/RF gives realizable conserved lane throughput / saturation
        ↓
CostBand quantizes consumptive movement work from available movement resource
        ↓
ActionBand consumes native outputs and emits actuation/structural consequence
        ↓
location advances / arrival terminal band
```

Movement is not a peer core action facility.

A useful conceptual decomposition is:

```text
PALMA      = direction / impedance
Gu-Yang    = realizable channel throughput / saturation
CostBand   = paid work quanta
ActionBand = target lifecycle / binding / band emissions
Overlay    = actuation/deformation
Property   = resulting location/velocity state
```

### 13.1 Spatial vendorization inherits the proven 7.1 fences

1. **No movement-specific destination authority.** Target exists only as ActionBand target/field/overlay state.
2. **No predecessor/path object.** PALMA `D` is a field.
3. **Single local structural step at spatial ingress.** N4 witness must fail closed unless an emitted structural step is adjacent.
4. **Ambiguous locus fails closed.** Physical row/iteration order cannot choose a target.
5. **Sealed crossing provenance.** No movement-side or Gu-Yang-side crossing detector.
6. **Placement is not movement semantics.** Reparenting/row binding remains structural law.
7. **Consumptive movement uses CostBand.** No bespoke movement-cost path.
8. **Capacity-bearing movement uses native Gu-Yang/RF flux.** No movement-local congestion, saturation, throughput, lane-capacity, or claimant arbitration model.
9. **Ownership and overlays inherit StemThing laws.** No per-participant owner stamping or permanent transit residue.

Deleting the human-readable word “movement” from the CPU shadow must leave GPU mechanics unchanged.

### 13.2 Worked witnesses

The same mechanics cover get food, build, research, repair, service access, colonization, and fleet movement. Only actions whose progress actually traverses a conserved capacity-bearing channel bind Gu-Yang.

---

## 14. Worked examples

### 14.1 Get food

Food-security displacement may emit ordinary food RF demand. If supply traverses a capacity-bearing logistics channel, Gu-Yang resolves realizable flow; CostBand then consumes/quantizes whatever sink semantics apply. The ActionBand does not plan logistics.

### 14.2 Build a door

Build progress may use one or several bands. Material/work sinks remain CostBands. A purely local stockpile→work conversion may need no Gu-Yang; a delivered-material channel may.

### 14.3 Fleet to Orion IV

```text
current: spatial locus A
target: LocusRadius(Orion IV, arrival radius)
route: PALMA potential
throughput: Gu-Yang/RF if the traversed lane is capacity-bearing
cost: movement/fuel CostBand
actuation: overlay/property velocity or sealed local structural step
terminal: arrival target satisfied
```

### 14.4 Shroud traversal

Capability checks are observations; scarce energy/fuel remain RF/CostBand. If no conserved channel topology is involved, Gu-Yang is absent from this ActionBand instance.

### 14.5 Colonization

Transport, population, supply, and access can be child ActionBands. Transport/supply children consume native flux when their underlying RF channel is capacity-bearing.

---

## 15. Performance implications of the GPU-only model

### 15.1 Recurse semantically; flatten physically

Recursive ActionBand structure is semantic authority; execution is flat GPU dataflow.

### 15.2 Ride already-hot sweeps — including Gu-Yang

The prior performance thesis only noticed write/crossing reuse. The Field Triad provides another major piggyback surface.

Preferred path:

```text
ordinary GPU RF / field evolution
    ↓
STEAD / PALMA / Gu-Yang FieldSweepRegistration work already owed
    ↓
ordinary anchored field/property outputs
    ↓
ActionBand binds those outputs as current/progress/flux observables
    ↓
existing fused threshold/crossing derivation
    ↓
no ActionBand-bound crossing → no ActionBand payload work
    ↓
ActionBand-bound crossing → EML + fixed emissions
```

Where Gu-Yang has already materialized **realized flux**, ActionBand may require no throughput computation at all. Its execution can collapse to a binding/read of that resolved field value plus the crossing/payload work it already owes.

This is stronger than “reuse a cheap flux helper.” The optimization target is **zero duplicate flux solve**.

### 15.3 Depth-1/2 fast path

Shared inline descriptors/program ids should dominate the common case.

### 15.4 Inline trivial children

Simple prerequisites inline into parent EML; only independent lifecycles materialize rows.

### 15.5 Program bucketing and JIT fusion

Bucket by EML program, emission shape, binding layout, and field-access shape. Field-Triad outputs should be grouped/reused by admitted profile rather than recomputed per ActionBand instance.

### 15.6 Sparse gathers are likely the dominant ActionBand cost — unless the field sweep has already paid them

Once recursion is flattened, the likely remaining cost is gathering dispersed property/RF/Field-Triad inputs and writing sparse outputs. Engineering should prioritize SoA layout, binding/program grouping, locality-aware descriptor order, shared ingress/workgroup caching where proven, compact active masks/lists, and zero duplicate field projections.

A Gu-Yang/PALMA output that is already resident can remove an entire class of ActionBand-side gathers or computation. Therefore performance measurement must separate:

```text
A. field solve cost already owed by world simulation
B. incremental ActionBand binding/gather cost
C. crossing/payload/emission cost
```

Charging A again to ActionBand would mismeasure the architecture.

### 15.7 Current/next state removes intra-dispatch dependency hazards

Use current-state reads and next-state activation/progress writes where practical.

### 15.8 Continuous state remains GPU-resident

Displacement, velocity, stakes, flux observations, progress, active bits, crossing state, child state, claims, and terminal eligibility remain GPU-resident.

### 15.9 Memory risk is bounded by admitted active state, not authoring depth

Templates are shared; trivial nodes inline; stateful children materialize sparsely.

### 15.10 Field-output consumption fast path

A production optimization is specifically admitted for review:

> **When PALMA/Gu-Yang/STEAD already produce the exact authoritative observable an ActionBand needs, the ActionBand implementation should bind directly to that resident column/registration and skip an equivalent EML/field recomputation.**

This is a physical optimization only; it may not change semantics, crossing identity, or generation ordering.

---

## 16. Determinism and lifecycle

ActionBand inherits SimThing determinism laws:

- semantic order never derives from physical row order;
- EML uses admitted arithmetic semantics;
- crossing detection is exactly Phase-5;
- structural consequences stay behind recorded boundary authority;
- no same-generation recursive convergence;
- CPU labels never affect numerical dispatch;
- activation never depends semantically on atomic append order;
- velocity never reads unadmitted history;
- target forms never invoke unsealed runtime solvers;
- Field-Triad outputs retain their own canonical adjacency/order/conservation proofs;
- ActionBand may not replace a native Gu-Yang result with a differently ordered/private flux calculation;
- completed ActionBands collapse.

---

## 17. Candidate binding laws for DA review

### 17.1 Intrinsic Action Law

> Every SimThing possesses the inert-by-default capability to host ActionBands. ActionBand is the base SimThing event/action execution facility, not a domain service and not a fifth StemThing leg.

### 17.2 GPU Numerical Authority / CPU Semantic Shadow Law

> ActionBand numerical authority exists only on the GPU. CPU state is a semantic shadow containing human-readable designations, durable logical identity/history, diagnostics/presentation metadata, and sparse GPU-produced semantic/structural deltas. Human-readable labels may never become numerical dispatch keys or CPU-side decision authority.

### 17.3 Target-Displacement Law

> An ActionBand represents unresolved displacement between current recursively evaluated state and a target expressed through the closed admitted target-form vocabulary.

### 17.4 Resultant Bipolar Axis Law

> A truly bipolar semantic axis is one bounded resultant degree of freedom. Cancellation to zero is zero unless contest magnitude is separately authored. Semantic bipolarity does not imply RF conservation.

### 17.5 Stakes Law

> Stakes are the EML-derived consequence/urgency of leaving displacement unresolved. Velocity may affect stakes only when a previous-generation plane is admitted.

### 17.6 Authored Band Law

> Bands are authored threshold surfaces over admitted ActionBand observables. Band segmentation is semantic/authored, not fixed to normalized or equally spaced progress.

### 17.7 EML Payload Purity / Bound Emission Law

> An ActionBand EML payload is a bounded numerical program, not an imperative callback. Results reach the world only through pre-admitted emission bindings into existing property, overlay, RF, CostBand, subordinate-ActionBand, telemetry, or structural-request surfaces.

### 17.8 Native Semantics Law

> ActionBand binds to existing property, RF, CostBand, overlay, **STEAD, PALMA, Gu-Yang**, crossing, and boundary semantics; it does not create a parallel resource, prerequisite, sink, crossing, transfer, flux, saturation, throughput, or structural-mutation universe.

### 17.9 Point-of-Execution Law

> ActionBand owns the lifecycle from unresolved target through partial band emissions to terminal resolution. It is not merely a goal selector in front of another action engine.

### 17.10 Recursive ActionBand Law

> A band result may activate subordinate ActionBands on the same intrinsic facility. Subordinates are nested target discrepancies, not imperative tasks; they may resolve concurrently and collapse when their targets become ordinary state.

### 17.11 GPU Table Recursion Law

> Recursive semantics lower to stable GPU indices/spans, shared templates, sparse state, and next-generation buffers. Runtime pointer recursion, CPU child scheduling, semantic atomic-append order, and dynamic GPU EML authoring are forbidden.

### 17.12 Activation-Is-Not-Construction Law

> Activating a subordinate means activating/parameterizing a pre-admitted GPU template/instance for a later generation. Additional storage is ordinary residency work, never CPU semantic choice.

### 17.13 Multisource Collapse Law

> Heterogeneous requirements remain inline ordinary predicates/resources or materialize as subordinate ActionBands when they have independent target-seeking lifecycle. Resolved requirements become ordinary world state, not permanent task records.

### 17.14 Generation-Pacing Law

> Parent/child effects propagate through generation/barrier ordering. No same-generation recursive convergence loop is lawful.

### 17.15 Semantic-Recursion / Physical-Flattening Law

> Recursive ActionBand structure is semantic authority. Admission/execution may inline, batch, JIT, or flatten it into packed GPU data so long as semantics, crossing behavior, lifecycle, provenance, and generation pacing remain unchanged.

### 17.16 Vendorization Law

> Domain behaviors such as physical movement are derived/vendorized uses of ActionBand. No domain implementation may become a peer core action facility.

### 17.17 Bounded Subordination Law

> Every recursive template declares finite subordinate/dependency span and maximum concurrently active subordinate count. Runtime semantics may activate only within that span.

### 17.18 Axis Budget Law

> Every running field theater/session has an explicit admitted semantic/Field-Triad channel budget. Runtime ActionBand execution may not mint new axes or unbounded derived fields.

### 17.19 Single Crossing Surface Law

> ActionBand band crossings ARE the existing Phase-5 anchored threshold/`BandCrossingDelta` mechanism. A second comparator pass, crossing record, listener framework, saturation listener, or CPU crossing evaluator is unlawful.

### 17.20 Closed Target Form Law

> Every target uses one admitted §4.2 target form with total GPU satisfaction and projection semantics. Flux/saturation observables do not silently create a new target form.

### 17.21 Previous-Plane Velocity Law

> Velocity is lawful only over observables with admitted previous-generation representation. Referencing velocity may not implicitly allocate history.

### 17.22 One Admission Door Law

> ActionBand semantic templates are admitted once at session build through `simthing-spec` and frozen into sealed numeric template products. No driver registry, runtime template mint, or vendor-local admission authority exists beside that door.

### 17.23 8.x Scarce-Holding Fence

> Before 8.1/8.2 and Vector CostBand, ActionBand may use ordinary RF claims, existing Gu-Yang/RF realized-flow semantics, and scalar CostBand but may not invent persistent cross-arena scarce holding or atomic multi-lane transaction semantics.

### 17.24 Field-Triad Native Progress Law

> **ActionBand consumes Field-Triad authority rather than reconstructing it. STEAD supplies non-conserved propagated state, PALMA supplies topology-aware potential/impedance, and Gu-Yang/RF supplies realizable conserved flux and saturation where progress traverses a capacity-bearing channel. When native Gu-Yang/RF has already resolved the available or realized flux, that value is the authoritative bound/input for ActionBand progress; a parallel ActionBand throughput, congestion, or saturation calculation is unlawful. CostBand remains the sink/work quantizer downstream of whatever conserved quantity is actually available.**

---

## 18. Explicitly fenced questions

The base ActionBand facility is semantically complete without resolving:

1. **Vector CostBand atomicity:** exact efficient common-depth commitment across several independently contested scarce RF lanes.
2. **Holding/fairness:** persistent provisional scarce grants after 8.x; starvation/pathological hoarding falsifiers.
3. **Local minima / adversarial navigation:** escape and competition behavior when field descent is not trivially monotone. **Gu-Yang saturation does not resolve this fence.**
4. **Optimal GPU ABI / fusion:** exact descriptor widths, state layout, inlining threshold, active-list representation, batching/JIT strategy, and the best way to bind already-hot PALMA/Gu-Yang outputs without duplicate gather.
5. **Dynamic capacity growth:** best residency-backed mechanism when active ActionBand cardinality exceeds pre-granted capacity.
6. **Performance envelope:** exact memory/bandwidth cost under millions of active/inactive SimThings, including the crossover between direct resident-field consumption and additional sparse gathers.

**Gu-Yang relationship: RESOLVED by this revision.** The former §18.7 fence is superseded by §§4.6, 7, 12.2, 13, 15.2/15.6/15.10, and binding law §17.24. What remains open is physical optimization, not semantic ownership.

> **Anchor-impact note:** `## 18. Explicitly fenced questions` is itself an anchored section (`actionband-fenced-questions`). This edit intentionally changes that anchor's content/rule stamp. DA/orchestration must sequence anchor resync and carried ORIENT-RECEIPT invalidation against the active 7.* rung; this document does not edit anchor tables or ladder rows.

---

## 19. Engineering/Fable review obligations

### 19.1 Crossing integration

- Does every ActionBand band use the existing Phase-5 threshold registration / fused crossing path?
- Do Gu-Yang saturation/stall bands remain ordinary anchored crossings?
- Is there exactly one `BandCrossingDelta` authority?

### 19.2 Target admission

- Does every target fit §4.2?
- Is Gu-Yang used only as observable/progress bound unless a future constitutional target-form addition is explicitly approved?

### 19.3 GPU physical feasibility

- Can common depth-1/2 cases remain compact sparse descriptors?
- Can Field-Triad bindings reference resident outputs rather than materialize ActionBand copies?
- Can recursion use stable spans/current-next state without CPU scheduling?

### 19.4 EML payload feasibility

- Does EML remain pure numerical evaluation?
- Does any payload try to implement a private throughput/congestion solver? If yes, remand.

### 19.5 Multisource / 8.x compatibility

- Are ActionBand RF claims ordinary claims?
- Are native Gu-Yang/RF realized-flow semantics preserved without pre-inventing 8.x holding?
- Are unsupported atomic multi-arena bundles refused rather than guessed?

### 19.6 Performance and bounds

- Can evaluation ride already-hot write/crossing/**Field-Triad** passes?
- When Gu-Yang has already produced realized flux, is duplicate ActionBand throughput work literally absent?
- Are performance measurements split into field cost already owed vs marginal ActionBand binding/payload cost?
- Are sparse gathers still dominant after resident field-output reuse?
- Can CPU traffic remain sparse semantic/structural deltas only?

### 19.7 Determinism

- Does flattening preserve canonical semantics independent of physical row order?
- Does ActionBand consume Gu-Yang outputs with the same canonical adjacency/order/conservation authority rather than reordering them?
- Does generation pacing eliminate same-dispatch recursive dependency?

---

## 20. Falsifiers

The ActionBand design should be remanded if any of these are demonstrated.

### F1 — peer action authority is required
A broad ordinary action class requires a second authoritative goal/event/action service beside ActionBand.

### F2 — recursion requires imperative scheduling
Correct behavior requires persistent task scheduling rather than target discrepancies and world state.

### F3 — recursion cannot flatten
Correct semantics require runtime pointer recursion, CPU child scheduling, or same-generation recursive execution.

### F4 — EML must become imperative
Useful ActionBands require arbitrary side effects, runtime bytecode creation, string/domain dispatch, or unrestricted mutation.

### F5 — multisource semantics require duplication
A real action cannot be expressed without a second resource/property classification beside native substrates.

### F6 — recursive state cannot collapse
Resolved subordinates must remain permanent task records merely for correctness.

### F7 — performance requires population-wide action scans
The only workable implementation iterates all possible actions over all SimThings.

### F8 — GPU storage growth requires CPU semantic selection
Additional capacity can be provided only if CPU code chooses which semantic action to create.

### F9 — topology requires a domain planner
A major target class requires a privileged domain planner rather than closed targets + native Field-Triad/direct-state machinery.

### F10 — CPU ActionBand authority is required
Correct behavior requires CPU-side evaluation, mirroring, goal selection, crossing decisions, recursive scheduling, flux calculation, or name-based dispatch.

### F11 — second crossing machine is required
Correct behavior requires a crossing detector beside Phase-5.

### F12 — target vocabulary must remain open
An ordinary target requires an unbounded runtime solver outside the closed target vocabulary.

### F13 — bounded recursion is insufficient
An ordinary use requires semantically unbounded concurrent child fanout.

### F14 — hidden history is required
A required velocity/stakes use needs unadmitted prior state or CPU history cache.

### F15 — private flux authority is required
Correct ordinary ActionBand behavior requires a throughput, congestion, lane-capacity, saturation, or claimant-allocation model beside native Gu-Yang/RF. If demonstrated, either the Field-Triad substrate is insufficient and must be escalated explicitly, or the ActionBand unification premise fails; silently adding the private model is not lawful.

---

## 21. 0.0.8.7 Phase-7 rewrite and anchor promotion plan

The already-authored dependency remains:

```text
ActionBand admission
    ↓
GPU execution
    ↓
recursive/native composition
    ↓
spatial vendorization
    ↓
semantic shadow
    ↓
8.1 conservation judge
    ↓
8.2 generic constrained clear
    ↓
Vector CostBand / adversarial-navigation probes
```

Gu-Yang does **not** justify reopening every 7.* rung by default. The semantic omission affects native composition, spatial vendorization, performance binding, and later overlay-actuation work. The least-disruptive proposal is to add remedial Field-Triad rungs unless exact implementation archaeology proves an earlier rung must be reopened.

### 21.1 Existing 7.* ladder disposition

| Rung | Existing ID | Gu-Yang disposition |
|---|---|---|
| **7.1** | `ACTIONBAND-ADMISSION-DOOR-0` | No target-vocabulary change required. Admission must merely be able to bind existing Gu-Yang/field outputs as ordinary observables. Reopen only if the admitted binding vocabulary physically forbids that. |
| **7.2** | `ACTIONBAND-GPU-EXECUTION-0` | No new executor required. Reopen only if GPU bindings cannot consume resident Field-Triad outputs without CPU mediation. |
| **7.3** | `ACTIONBAND-RECURSIVE-COMPOSITION-0` | Native-semantics text was incomplete but direction remains correct. Gu-Yang/RF realized flux is another native input; no ActionBand holding/transaction system. Prefer remedial proof rather than reopening unless 7.3 implementation hard-coded RF/CostBand while excluding field-output bindings. |
| **7.4** | `ACTIONBAND-SPATIAL-VENDORIZATION-0` | Spatial witness was semantically incomplete without capacity-bearing flux. Prefer a remedial spatial-flux witness; reopen 7.4 only if its landed public surface encodes a PALMA+CostBand-only movement law that must be changed. |
| **7.5** | `ACTIONBAND-SEMANTIC-SHADOW-0` | Semantic shadow itself does not own flux. The active rung may proceed only if DA is satisfied that its implementation does not freeze the incomplete PALMA-only execution story into readback/public vocabulary. |

### 21.2 Proposed remedial 7.* rungs for DA sequencing

The DA may renumber. Proposed shape:

| Proposed rung | ID | Scope | Exit proof |
|---|---|---|---|
| **7.5a** | `ACTIONBAND-FIELD-TRIAD-PROGRESS-0` | Bind ActionBand progress/band operands to existing STEAD/PALMA/Gu-Yang field outputs as native GPU inputs. Admit available/realized flux and saturation/stall/choke observables without new target forms, crossing surfaces, or flux solvers. Add resident-output fast path. | Synthetic conserved-channel ActionBand consumes a real Gu-Yang/`FieldSweepRegistration` output; reducing channel capacity reduces ActionBand progress without changing ActionBand EML/target; restoring capacity restores progress. Planted private-throughput mutant disagrees/REDS. No second field solve or CPU readback. |
| **7.5b** | `ACTIONBAND-SPATIAL-FLUX-WITNESS-0` | Re-prove movement/vendorization using full Field Triad: STEAD/overlays compose conditions, PALMA selects lawful local potential descent, Gu-Yang/RF bounds realizable lane throughput, CostBand quantizes movement sink, ActionBand emits ordinary actuation/structural consequence. | Same target and PALMA field with different Gu-Yang capacity produces different lawful per-generation progress while route identity remains unchanged; saturated lane stalls/limits throughput without invoking local-minimum navigation; no movement-local congestion model; removing human-readable movement labels leaves GPU result unchanged. |

If exact archaeology shows 7.3 or 7.4 cannot accept these bindings without changing their graduated public semantics, the DA should reopen the smallest affected rung rather than building compatibility shims. The Owner has explicitly authorized reopening 7.* where necessary.

### 21.3 Downstream 8.x impact

8.1/8.2 remain conceptually correct: Gu-Yang conserved flux does not replace the conservation judge or generic contention clearing. The remedial rungs should bind ActionBand-originated/consumed flux products into the same declared conservation universe and ordinary claim→clear→disburse path where applicable.

Vector CostBand remains downstream. Gu-Yang solves channel throughput, not atomic common-depth commitment across several independently contested scarce lanes.

### 21.4 Local-minimum fence remains untouched

Neither 7.5a nor 7.5b may design escape/search behavior. A saturated route is an observed physical condition; choosing a different opportunity horizon remains §18.3 / later-probe work.

---

## 22. Anchor promotion map and core deliverable

### 22.1 Recommended doctrine anchors

Existing heading anchors remain appropriate. This revision intentionally changes the content under several of them, especially `actionband-binding-laws`, `actionband-vendorization-direction`, and `actionband-fenced-questions`; DA must resync hashes/rule stamps.

| Anchor id | Section | Trigger domains |
|---|---|---|
| `actionband-executive` | `## 0. Executive definition and physical feasibility verdict` | `core-0087,actionband` |
| `actionband-constitutional-placement` | `## 1. Constitutional placement: ActionBand is the SimThing \`act\` facility` | `kernel,sim,driver,actionband` |
| `actionband-eml-payload-purity` | `### 5.3 EML payload purity / bound emission law` | `kernel-eml,actionband` |
| `actionband-gpu-physical-model` | `## 6. GPU physical model: sparse tables, bounded recursion, and one admission door` | `kernel,gpu,actionband` |
| `actionband-binding-laws` | `## 17. Candidate binding laws for DA review` | `core-0087,actionband` |
| `actionband-vendorization-direction` | `## 13. Movement is a derived/vendorized ActionBand implementation` | `sim,movement,actionband` |
| `actionband-fenced-questions` | `## 18. Explicitly fenced questions` | `actionband` |

No anchor-table or ladder-row edits are made by this document revision.

### 22.2 Core deliverable

```text
                         SIMTHING
                            │
                    recursive state/RF
                            │
                            ▼
                   FIELD TRIAD / state
          ┌─────────────────┼─────────────────┐
          │                 │                 │
       STEAD             PALMA            GU-YANG
 non-conserved        min-plus D       conserved flux
 influence/state      / impedance      / saturation
          └─────────────────┼─────────────────┘
                            │
                   current coordinate X
                            │
                 closed target form G(t)
                            │
                 displacement / stakes
                            │
                   EML deficiency logic
                            │
          ┌─────────────────┴─────────────────┐
          │                                   │
  direct/non-conserved progress       conserved-channel progress
          │                                   │
          │                           bounded by Gu-Yang/RF
          │                                   │
          └─────────────────┬─────────────────┘
                            │
                       ACTIONBAND
                            │
              EXISTING PHASE-5 CROSSING
                            │
                     BandCrossingDelta
                            │
                     EML + bindings
                            │
            ┌───────────────┼───────────────┐
            │               │               │
         overlays          RF           CostBand
            │               │               │
            └───────────────┼───────────────┘
                            │
                    GPU next state
                            │
                      later generation
                            │
                    next band / collapse

         ALL NUMERICAL AUTHORITY ABOVE: GPU
                            │
                            ▼ sparse semantic/structural deltas
                    CPU SEMANTIC SHADOW
```

The physical recursion remains shared templates + bounded dependency spans + sparse instance rows + current/next activation + shared EML ids + fixed emission bindings + existing threshold registrations. The Field Triad remains a resident substrate beneath that recursion, not copied into it.

---

## 23. Gu-Yang revision summary for DA

1. **§12 authority sentence:** Gu-Yang / `SaturatingFlux` owns conserved saturating flux/throughput over admitted topology and is the authoritative bound on how much conserved quantity can traverse a capacity-bearing channel this generation, including admitted saturation/stall/choke observables.
2. **Pathway change (§15.2/§15.6): YES.** Where a Gu-Yang sweep is already hot and already materializes the required available/realized flux, ActionBand should consume the resident output directly. The optimization target is zero duplicate throughput solve; measure only marginal ActionBand binding/gather/payload cost above the field work the world already owes.
3. **§17 law warranted:** new §17.24 `Field-Triad Native Progress Law` binds ActionBand to STEAD/PALMA/Gu-Yang native authority and forbids a private throughput/congestion/saturation model.
4. **Target form:** **no change.** Gu-Yang is an execution/progress constraint, not target geometry. **Band operands:** add available flux, realized flux, saturation/stall magnitude, and sanctioned choke/contest projections as ordinary anchored observables. Any future flux-defined target form is a constitutional vocabulary addition.
5. **§18.7:** **superseded/struck as an open semantic question.** The relationship is resolved; only ABI/fusion/performance remains fenced. This intentionally drifts anchored §18 and invalidates the current `actionband-fenced-questions` rule stamp until DA resync.
6. **Ladder proposal:** prefer remedial `ACTIONBAND-FIELD-TRIAD-PROGRESS-0` and `ACTIONBAND-SPATIAL-FLUX-WITNESS-0` in 7.* before 8.1/8.2. Reopen 7.3 or 7.4 only if archaeology proves their graduated implementation/public surface cannot consume native Gu-Yang outputs without semantic change. Do not edit ladder/anchors in this revision.

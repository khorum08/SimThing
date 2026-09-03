# The SimThing Core Design — Paradigm Reference

> **Status: STANDING — effectively permanent, deliberately amendable.** This document is the
> paradigm itself, beneath the versioned constitution. It states the architecture every version,
> PR, and agent builds toward. If a handoff, ladder row, status record, or ancillary design
> conflicts with it, stop and escalate to design authority.
>
> **Definable horizon.** This document claims no permanence. A graduated ruling that changes the
> paradigm amends this document in the same change; canonization consumes temporary anchors rather
> than leaving them as a second authority.

---

## 0. Spatial substrate: STEAD/Mapping is not optional

SimThing is a spatial simulator. A Location is a structural gridcell SimThing, and the map is a
lattice of those cells running the Movement-Front automaton. Spatial identity is intrinsic; it is
not render metadata or a detachable role.

The spatial contract has eight non-negotiable consequences:

1. The parent grid owns the arena; placements live in structural grid metadata.
2. Emitted integer col,row coordinates are honored as structure, never replaced by emission order.
3. Empty cells are ambient field, not absent ontology; the lattice is sparse and may be vast.
4. Heatmaps, fronts, falloff, PALMA, Gu-Yang and RF pressure are expressions over this substrate.
5. Layout admission and bounded execution-profile admission are different judgments.
6. A dense theater cap never shrinks, compacts, or invalidates structural layout.
7. Exact Euclidean gates route through Candidate F; exactness never licenses spatial erasure.
8. Runtime numerical law is translation-invariant and local; absolute coordinates never choose it.

The full contract and adjudications remain in
[stead_spatial_contract.md](stead_spatial_contract.md) and
[ClauseThingADR.md](adr/ClauseThingADR.md).

---

## 1. The SimThing Principle — one closed recursive stem-cell kernel

**Everything in simulation authority is a SimThing.** The SessionThing is the single admitted root.
Every descendant is the same recursive germ: logical identity and parentage, sparse Property state,
intrinsic ownership, overlay state, admitted lanes, children, and the capability to participate,
act, originate, and receive. Specialization adds admitted data; it does not create a privileged
runtime kind, manager, or adjacent engine.

The closed root contract is:

| anatomy | intrinsic meaning |
|---|---|
| **participate** | hold and exchange Property and RF state; reduce upward and disburse downward |
| **act** | resolve admitted ActionBand discrepancies and CostBand crossings |
| **originate** | own and route attributable OverlayThing state |
| **receive** | accept deficit-driven, standing, and predicate-broadcast directives |
| **StemThing-A lane** | stable logical residency, parent slotting, child-row/extent placement |
| **StemThing-B lane** | recursive conserved-resource markets, Draws, clearing and grants |
| **expression organ** | the one admitted EML ISA and gadget library |
| **field organ** | the RF/STEAD/PALMA/Gu-Yang Field Triad |

Every row is sparse and inert by default. Identity obeys the same law: the base SimThing carries only a compact tree-local id — realm, host, and execution metadata are O(1) per executing tree, never O(population), and higher-order identifiers are derived anatomy on specialized SimThings. The id's internal width is not frozen; widening is an Owner-ruled ABI/schema and migration change. A population cohort, empty gridcell, owner seat, and
SessionThing possess the same semantics without allocating active state for unused lanes. The
kernel is fractal: a provisioned descendant may execute as a subtree while remaining connected to
its ancestor by the same stamped product and grant seams.

All conflict, opportunity, ambition, extraction, residency and actuation enter one cycle:

    admit and bind
      → accumulate local state
      → reduce perception and claims up the tree
      → resolve ownership, overlays, fields, valuation and constrained clearing
      → disburse grants and directives down
      → cross admitted CostBand and ActionBand thresholds
      → apply the recorded boundary products
      → advance Current to Next and repeat

There is no combat engine, economy engine, AI planner, allocator service, pathfinder, overlay
manager, or second clearing path beside that cycle. More behavior means more admitted Property,
OverlayThing, EML, CostBand, ActionBand, RF, or Field-Triad data on the germ.

The Invariant Set is closed: conservation, determinism, CPU/GPU parity, boundedness, admission
totality, and residency/typing are the complete proof surface. Emergence witnesses demonstrate the
substrate; they do not mint new invariants.

### 1.1 Root identity, residency and recursive generation

Logical SimThing identity survives physical slot movement. StemThing-A gives the admitted root and
every child stable logical slot identity plus epoch-rebindable physical placement. The physical row
is execution geometry, never semantic identity.

The **initial-residency distinction** has one narrow exception. SlotAllocator.install_initial_tree admits initial bulk
installation, and may continue installing against the **same already admitted structural root**.
It is not an ordinary growth door. Presenting a different subtree or root is the typed
InstallInitialTreeAttachedGrowthBypass refusal. After initial residency, attached growth enters
through the ordinary StemThing-B entitlement, placement, and structural-commit path.

Each tree has one generation authority. Current state is read during generation N; authorized
Next-state and boundary products participate no earlier than their admitted later generation.
Generation pacing is the recursion bound: no same-generation receive/originate convergence loop,
re-clear, retry, or consequence re-entry exists.

### 1.2 Shared-surface ledger and the one integration schedule

The closed germ reuses one set of already-authoritative surfaces:

| surface | role in the germ |
|---|---|
| Property columns and role registry | resident numerical state and resolved bindings |
| resolve_owner and owner-keyed RF buckets | inherited identity, local conservation and segregation |
| reduce-up / disburse-down | perception and consumed directives |
| OverlayThing + ActionBand | attributable actuation and sealed consequence dispatch |
| EML programs | valuation, transforms, predicates and field laws |
| CostBand | the sole sink and continuous-to-discrete quantizer |
| FieldSweepRegistration | the one map/fold/post execution IR for the Field Triad |
| BoundaryProtocol | structural mutation at the barrier |
| IntegrationSchedule + canonical replay deltas | the one history and replay order |

Both asynchronous carrier directions are generation-stamped: upward RF products and downward
standing/directive products. RoutedOverlayProduct likewise carries the source generation and
required origin while remaining in slot space inside the closed loop. IntegrationSchedule is the
single append-only ordering record for products, grants, refusals, remaps, injections and lifecycle
facts; no facility may mint its own history, clock, retry log, or checkpoint authority.

Staleness is observable state, not host-side truth. The age of a stamped product or field sample is
represented as an ordinary STEAD field column and may be thresholded like any other observation.
Observer lag follows the admitted backpressure policy and never waits for or perturbs the sim.

### 1.3 Events are RF

Events use the germ's bidirectional RF shape: **reduce-up carries perception; disburse-down carries
directive.** There are exactly three delivery modes:

| mode | trigger and existing home |
|---|---|
| **deficit-driven** | a consumer surfaces a command deficit; delivery rides ordinary disbursement |
| **standing** | a non-consumed value is inherited through the same absence-means-inherit resolution shape |
| **predicate broadcast** | one bounded tree walk, paid by the initiator, selects matching descendants |

A consumed directive is a conserved resource. A standing directive is a Property read and makes no
conservation claim. Predicate broadcast is named separately because it is the only mode that does
not fall out of RF transport.

Overlay.origin is required. It answers which SimThing originated the act; OverlaySource answers
what kind of will produced it. Spatial provenance is derived from origin through admitted placement
and StructuralCoord, never stamped as a rival coordinate. Within a tree, routing is origin to common
ancestor to target so every intermediate policy layer can filter it. The closed loop carries slot
identity; SimThingId is reattached only at the admitted boundary when required.

### 1.4 Unified ingress exclusivity

The closed-kernel claim is enforced, not aspirational:

1. OverlayThing, CostBand, ActionBand and mapping EML EXP/LN are the only unified ingress to
   RF/Triad resolution.
2. RF/Triad resolution is the only ingress to domain contention and resolution.
3. The unified SimThing object is the only execution path for simulation resolution.

A graduated producer must have a non-test production consumer or a structured dated deferral.
No second production sink may reach resolution, including a renamed, wrapped, or alternate-call-form
bypass. Composition proves the facilities can work together; unified-ingress exclusivity separately
proves nothing bypasses them.

### 1.5 Section-12 market grammar — StemThing-B

StemThing-B is the recursive conserved-resource market germ:

    admitted resource or capacity
      → descendant claims authorized by sealed Draw templates
      → recursive RF reduce-up (branch-attributed pressure, once per edge)
      → typed eligible pressure — raw P or born Gu-Yang serviceable F —
        selected once into AllocatorWeight (neutral identity law)
      → child-share EML on live parent columns → AllocatedFlow
      → exact quantization Q at the terminal integration band → canonical T_s
      → the child consumes that same T_s as its own supply
      → unresolved U → optional sealed deformation → next-generation demand
      → Gu-Yang throughput/saturation and PALMA potential/impedance/opportunity
        observed on the executed flow (born field authorities, applied once)
      → STEAD observations and bands
      → ActionBand, CostBand-funded, or OverlayThing consequence
      → next-generation state

A Draw is a specialization-profile claim template, not a grant. Its offerings, lifecycle triggers
and finite quantity envelope seal at admission; instances mint no new Draw vocabulary. Unit price
and clearing weight are distinct. Oversubscription is ordinary. Equal scores clear proportionally
unless the author supplies an additional deterministic rank; discrete remainder is
largest-remainder and exact ties rotate under the granter's generation authority. Unmet demand U is
revalued next generation and is not CostBand remainder R.

Residency entitlement is market-decided, then physical placement fails closed in two stages.
Ordinary infeasibility keeps the quantity in U and records a typed refusal; committed overlap or
out-of-bounds placement is an invariant breach. Detachment alone does not release a grant.
Death/dissolution releases; fission/fusion partition or transfer exactly; termination is explicit.
No allocator path exists outside a SimThing disbursing admitted capacity through RF.

### 1.5.1 The germ self-consumption law (Owner-ruled 2026-08-31)

A recursive StemCell germ facility is necessarily its own consumer: its emission at one tree level
IS its intake at the next. Every germ facility is therefore input/output symmetrical in its
constrained-product vocabulary — the settlement/emission product type is IDENTICAL to the
supply/intake type, one type under at most two role names. Any adapter, conversion, or projection
between a germ's output port and its own input port one level down is a STOP-grade defect: an
adapter at the germ replicates at every level of every tree in every domain. Enforcement is the
type identity itself, never a guard scan.

Scope: facilities every StemThing carries fractally (the section-12 market germ is the archetype).
One-way non-germ doors — oracles, replay/persistence, observation egress — have external consumers
by design and are out of scope. Corollaries: the recursion is the consumer, so the canonical
witness for a germ facility is recursive self-consumption across at least two tree levels and the
exact-consumer obligation is discharged intrinsically; the root's intake port binds authored
supply and the leaf's settlement port binds actuation — same ports everywhere, self-bound in the
interior; a seam to a detached subtree carries the SAME type, so self-consumption survives
detachment; a zero-production-consumer posture on a germ candidate is probation scaffolding whose
consumer of record is the germ itself one level down. Mint record: Board #1332 comment 5483829845.

### 1.6 Anchor lifecycle and repoint law

A doctrine anchor is an index into in-tree canonical law, not a second constitution. A pending
anchor binds newly landed vocabulary to the rung that minted it until the declared canonization
consumer runs. CORE-CANONIZATION-0 is the exact consumer for the 0.0.8.7 pending worklist.

Canonization and repoint are one atomic change: first home the graduated law in truthful canonical
prose, then repoint the anchor to that section, set lifecycle canonical, and resync its content hash.
A post-canonization design-authority anchor uses lifecycle `until:<RUNG-ID>` naming its
CONSUMING rung: healthy while that rung is open, stale the moment it graduates without the atomic
repoint, orphaned if the rung is absent or superseded. An orphaned interim anchor is reaped only through the existing authorized-deletion ledger. Track
closeout admits no pending anchor. This is the exact **anchor lifecycle/repoint law**; it creates no second
rung registry.

### 1.7 The theoretical anchors

Movement-Front applies Wei's locality, symmetry and stability postulates: local finite-speed
propagation, one translation-invariant rule, and dissipative convergence to admitted attractors.
Gridcell SimThings are the lattice; field sweeps are the shared rule; bounded operators and threshold
projection provide stability.

EML applies the single-operator construction eml(x,y) = exp(x) − ln(y). Authoring lowers into the
one closed opcode-stack interpreter and gadget library, so complex behavior stays data rather than
becoming a bespoke kernel or opcode.

### 1.8 Admission is part of the anatomy

Every invariant climbs to the highest enforceable rung:

1. type boundary — the illegal state is unrepresentable;
2. admission hard error — invalid content never reaches runtime;
3. guard or source scan — executable residue where a higher rung cannot yet express the law;
4. prose and DA judgment — the irreducible ontology residue.

The compiler admits code; hydration/session build admits content. Runtime receives resolved numeric
columns, identities and sealed registrations, not game concepts. A recurring detector is a promotion
target, not a reason to accumulate more tests.

The Necessity Test is exact: retain a test only when it catches a regression not already caught by a
type boundary, production admission error, or existing canonical integration path. Durable exceptions
are the narrow terminal classes: compile-fail seals, CPU/GPU parity, deterministic golden artifacts,
doc-named invariants, escaped bugs, and active-rung live proofs.

### 1.8.1 The kernel crate, the cross-crate seal law, and residue-as-tripwire

The kernel crate owns closed numerical vocabulary, admission proofs, stable identities and sealing
constructors. Higher crates may compose or present those products but may not forge them. Public raw
constructors, bare column indices, caller-selected proof tokens, and cross-crate shadow authorities are
promotion defects.

Rust cannot type-check WGSL, so exact GPU claims retain a bit-exact CPU oracle and per-opcode
qualification. Source scans remain valid only for the semantic residue that cannot yet be sealed.

---

## 2. The one tree: Scenario wrapper, GameSession root, owners, and spatial containment

Scenario is authoring input; SessionThing is the runtime root — PER EXECUTING TREE: a
provisioned subtree, or a sub-leaf tree of one, may itself execute as a root through a
TreeExecutionContext binding (root binding, realm, generation authority, local schedule, local
registry/residency context, seam attachment to its ancestor/granter) while remaining
grant-connected. Rootness is never SimThingKind mutation, semantic reparenting, duplicated owner
authority, or a second root object. Spatial containment answers where a
thing is. Ownership answers whose policy and RF identity it inherits. The two relations never replace
one another: owners are not spatial parents, capture is not reparenting, and a container may host
participants from many owners.

Owner is the intrinsic sparse Property dimension OWNER_CHANNEL_PROPERTY_ID. Absence means inherit,
so an inert tree stores no redundant owner copies. resolve_owner returns Result: it is total for
valid admitted members and fails closed for foreign or malformed identity. The reserved neutral
unowned owner is a real identity; it is not Option::None and authored content may not collide with it.
Resolution is pure and never materialized down the subtree.

RF bucket identity includes OwnerRef, ResourceKey and ScopeId. Ownership remains singular per
SimThing while participation is multi-owner; alliance is an authored relation between owners, never
a second ownership meaning.

Fission and fusion are ordinary boundary changes. Policy capture and succession emerge from
Property/RF thresholds and partitioned state; they do not introduce rebellion entities or a civil-war
subsystem. Stable logical slot and owner identity survive admitted remap and replay.

---

## 3. SimProperty → Value: the load-bearing data model

SimProperty is typed metadata; Value is the resolved resident numerical state. Properties are sparse
on SimThings, but admitted layout maps each participating role to a typed ColumnIndex. Runtime
behavior reads columns and registered roles, never authored names or hardcoded offsets.

Identity has three independent axes:

- logical SimThing identity, stable through physical movement;
- Property identity and role layout, resolved at admission;
- physical slot/row, an epoch-bound execution placement.

Confusing those axes is a defect. Replay and IntegrationSchedule carry logical identities and remaps;
physical rows are regenerated through the authoritative binding chain.

Sequential value transforms apply in authored order and may be last-wins-capable. Predicate
selection is a different algebra: restrictions compose conjunctively and monotonically, so a
descendant cannot loosen an ancestor restriction. Composition class is admission data, never inferred
from overlay kind or iteration order.

---

## 4. GPU residency — StemThing-A and the EML expression organ

The recursive tree is flattened physically into dense GPU columns while remaining recursive
semantically. StemThing-A owns stable logical residency, root installation, child allocation,
extent placement, remap, and binding-table freshness. Capacity is the conserved RF quantity;
contiguous extent is a kernel-minted placement result. Free-list or buffer mechanics are downstream
physics, not allocation policy.

The slot conservation judge is exact:

    free + in_flight + occupied = capacity

Extent disjointness and bounds are a separate placement judge. Physical relocation occurs only at
the owning generation barrier and records the canonical remap. No physical row becomes semantic
identity and no session silently rebinds a stale admitted plan.

### 4.1 Complete EML ISA and library

All scripted numerical behavior lowers to the one admitted EML expression registry, opcode-stack
interpreter and grown gadget library. Overlay values have one singular form: an opaque admitted EML
value. There is no static-versus-computed tag, overlay-local interpreter, second program table, or
caller-selected evaluation mode.

The complete language includes the admitted EXP and LN primitives. ExactPrimitiveAdmission is their
sole exact-primitive door: domain proof or an admitted guard precedes execution, the opcode set and
program cap remain closed, and each CPU, interpreted-GPU and SSA-JIT arm is a faithful lowering of
one arm-independent arithmetic meaning. ADD, SUB, MUL and DIV carry specified binary32 rounding;
MIN, MAX and clamps are exact selections; the unique multiply feeding an ADD or SUB is fused, while
multiple multiply inputs are unfused. EXP and LN retain their certified qualification.

The gadget library is the authoring surface for formulas, including field maps, folds, predicates,
gates and bounded feedback. A new domain formula adds data to that library; it does not add an opcode,
shader, evaluator, cache, or evidence architecture.

---

## 5. Resource flow arenas — participate, market, CostBand and ActionBand

RF is the conserved circulation of the germ. Local values accumulate, owner/resource/scope buckets
reduce upward, admitted constrained clearing resolves oversubscription, and grants or directives
disburse downward. Local settlement occurs before bubbling; settling depth is emergent from the one
tree and authored OrderBand, never a domain special case.

Resource classes declare their algebra and conservation posture at admission. One homogeneous lane
has one meaning; no amount/weight/identity conflation, no same-generation re-clear, and no rival
ledger beside the canonical balance and schedule.

### 5.1 CostBand — the sole resource-sink mechanism

**CostBand** is the definition of a resource sink, not an optional threshold mode. The threshold C
is unit cost for available value V:

    N = floor(V / C)
    R = V - N*C
    V = N*C + R

N is the number of units afforded or destroyed and R is carried forward. Every sink is a CostBand;
no threshold/sink split or rival mechanism exists. Booleans are CostBand depth 1. Direction expresses
accretion versus depletion; a negative output coefficient remains inadmissible. A crossing with no
CostBand observes without consuming.

### 5.2 ActionBand — the act facility

ActionBand is intrinsic recursive actuation over admitted numerical axes. Target, displacement,
velocity, stakes, EML payload and bounded subordinate state compile into sparse GPU tables. Semantic
recursion is physically flattened and generation-paced; activation changes resident state, not the
ontology or template inventory.

The target vocabulary is closed and admitted. ActionBand crossings use the one Phase-5 sealed
crossing surface and existing consequence dispatch. Field-Triad values retain native authority:
PALMA supplies potential/descent when routing applies; Gu-Yang/RF supplies signed realizable
throughput and stall; CostBand prices a sink when one is authored. None is reconstructed inside
ActionBand, and CostBand never becomes route or capacity authority.

---

## 6. Overlays — the intrinsic OverlayThing closure

OverlayThing is the germ's sole numerical actuation language. Every active overlay is owned by a
SimThing; there is no peer manager, service, ActionThing, lifecycle executor, or second dispatch
path. The four authoring families share this one surface:

1. **capability bestowal** — admitted template vocabulary a derived descendant may activate;
2. **standing policy** — ancestor-resident inherited restrictions or parameter shaping;
3. **lifecycled transient** — bounded active state with explicit dissolution or session horizon;
4. **operator directive** — attributable Player/AI will entering through the same routing door.

Overlay.origin is required, OverlaySource remains complementary, and subtree-scoped overlays reside
once at the lawful ancestor. Equivalent per-leaf stamping is unlawful. Rich source scopes compile
away into bounded bindings; runtime semantic string dispatch and population-scale CPU walks are not
admitted.

Overlay lifecycle has exactly UntilDissolved and AtSessionEnd. There is no permanence variant and no
Never dissolve condition. Activation, suspension, Current/Next state, dissolution and collapse use
the intrinsic facility and the ordinary generation boundary. A routed duration is rebased against
the destination tree's generation authority; foreign absolute deadlines are not transported.

Overlay numerical values use the singular admitted EML form from §4.1. Sequential transforms retain
authored order. Policy and selector predicates compose conjunctively. Same-generation numerical
dependencies are an admitted DAG; cyclic behavior crosses explicit Current-to-Next state with bounded
feedback. Replay-relevant lifecycle facts extend the one schedule and delta history.

---

## 7. Mapping — the Movement-Front automaton over gridcell SimThings

Mapping is the spatial expression of the same germ. Every gridcell is a Location SimThing with
structural coordinates, sparse Property state, overlays, RF participation and field registrations.
There is no map service beside the tree.

### 7.1 The Field Triad and born observables

One FieldSweepRegistration map/fold/post IR hosts the complete Triad:

- **STEAD** anchors resident fields and exposes magnitude, intensity, velocity, generation and
  staleness through ordinary band crossings.
- **PALMA** owns min-plus potential, reach and impedance over admitted GridOffsets or LinkGraph
  topology. It produces a field, not a route/predecessor object.
- **Gu-Yang SaturatingFlux** owns signed conservative throughput, saturation and choke behavior under
  the admitted symmetry and stability certificate.

Comparative net flux, gross flux, stall, dominance, margin, contest, border bands and chokepoint
projections are born as sealed EML projections over co-located anchored Triad columns when their
admission conditions hold. They are ordinary numerical observables, not optional services or
host-authored interpretations. The derived anchor table is their sole observation surface.

### 7.2 Movement-Front execution

The shared rule has three layers:

1. local field sweeps propagate pressure over admitted adjacency;
2. hierarchy reduction carries strategic perception upward without widening the stencil;
3. later-band EML interpretation crosses ActionBands and CostBands into ordinary consequences.

The front is the route. Movement descends local PALMA potential, advances only by signed native
Gu-Yang/RF realizable throughput, consumes CostBand-priced resources where authored, and repeats on
the next generation until the target condition is satisfied. No route solver, border polygon,
congestion model, predecessor object, saturation listener, or CPU planner acquires authority.

Per-observer perception is a filter field over truth. Perceived columns never write true columns;
only admitted boundary products update ground truth.

---

## 8. Time, decisions, and the CPU's only job

A tick advances deterministic resident state. A boundary is the synchronization point requested by
the host. Decisions are GPU-resident threshold crossings over resolved fields. The CPU consumes
sealed summaries and applies structural products; it never re-derives threat, economy, urgency,
clearing, field direction, or actuation.

Structural change—fission, fusion, add/remove, reparent, expiry, remap and re-enrollment—occurs at the
boundary through the sealed protocol. The evaluator does not mutate tree structure.

The **Vendorized Build Principle** makes the closed loop the general default. A CPU-authoritative build
is a derived resolution-site instance only when it shares the same registrations, math, invariants,
admission and history. A parallel CPU implementation competing with the resident substrate is not
vendorization.

### 8.1 Door symmetry

**Door symmetry:** a vendor-facing surface that writes a lifecycle artifact must read/reopen that
artifact through the same surface. The read side re-exports the existing authority and exact types;
it does not add a sixth verb, facade-owned state, wrapper result, or alternate replay semantic.

The canonical Run read side is the composition of read_spec_replay_file, apply_spec_snapshot,
apply_spec_delta and ReplayDriver application. The generic open_replay_with_spec entry is also
reachable through Run, but it does **not** cover admitted-field-sweep domains. That coverage gap is
accepted dated debt from the 2026-08-29 PORTABILITY-PROOF-0 ruling, Board 5460089932; closeout must
disposition it. This canonization does not claim or implement the missing generic-opener coverage.


### 8.2 Asynchronous subtrees — non-foreclosure laws (Owner-ruled 2026-08-30)

Local asynchronous execution is already law and substrate: generation authority is per executing
tree and is not a global barrier; integration never waits for a lagging child; determinism is
relative to each tree's recorded schedule. Remote-host execution is a FUTURE capability; these laws
exist so nothing built today forecloses it.

1. **Realm is ambient, never carried.** An executing tree holds one TreeRealmId (stable semantic
   namespace surviving migration and restart), one execution incarnation (the single active writer;
   a partition or lease law is required for any other arrangement), and transient host-locator
   metadata that is never semantic. Migration preserves realm and local ids but changes incarnation;
   a fork, counterfactual branch, or concurrent copy mints a NEW realm; stale-incarnation seam facts
   fail closed. Host address, process id, thread id, GPU adapter, device, queue, or endpoint never
   enters base SimThing anatomy.
2. **Realm qualification lives in seam and durable cross-tree relationship vocabulary** — seam
   facts and receipts, schedule provenance, foreign grant relationships, foreign overlay
   origin/provenance, and any durable reference whose referent remains in another tree — for EVERY
   seam-visible local identity. A foreign reference may lower to a destination-local proxy with its
   realm-qualified provenance recoverable. Raw local ids never cross, compare, merge, hash, or
   persist as foreign identity.
3. **The process-global id allocator is a local-construction implementation, not cross-tree
   identity law** (remote-allocation debt, dated 2026-08-30 in the constitutional census). No code
   may rely on raw ids being process- or host-globally unique. A remotely executable tree owns its
   local-id allocation authority and high-water state; on detachment either a new realm preserves
   inherited local ids (presumptive) or a same-realm partition uses explicit non-overlapping
   allocation leases — recorded, not yet chosen.
4. **Seam recording is never distributed atomic commit.** The source tree records an immutable seam
   EMISSION at its own generation; transport may retry, delay, or reorder without changing fact
   identity; the target tree records INTEGRATION at its own generation; both records correlate by
   one immutable SeamFactId (source realm, seam id, source generation, source ordinal). A content
   key alone cannot distinguish a transport duplicate from lawful multiplicity.
5. **Seam delivery classes:** CONSERVED transfer (no loss, no duplicate realization, in-flight and
   holding-account conservation, retry-idempotent); STANDING VIEW (a newer complete view supersedes,
   staleness explicit); OBSERVATION (overwrite, throttle, and coalescing lawful, never feeding
   simulation truth). A conserved or simulation-authoritative fact riding a lossy observer surface
   is a violation.
6. **Generation stamps are opaque outside their tree authority.** Cross-tree arithmetic on raw
   generation values is lawful only across a declared common-lineage, common-cadence seam; other
   seams carry an admitted temporal relation or destination-observed age. Migration never resets a
   generation counter. Boundary-synchronized execution remains the opt-in form.
7. **One clearing home per conserved resource scope per generation.** Cross-tree claims travel to
   the home as stamped seam products; grants return through the seam; a provisioned child
   subdivides its already-granted local budget autonomously; moving a clearing home is a recorded
   structural authority transfer, never concurrent dual clearing.
8. **No synchronous ancestor RPC in the local hot loop.** An executing subtree advances from
   resident state and pre-granted resources; missing or late seam input appears as staleness,
   unavailable capacity, unresolved U, or an authored local refusal, blocking only the dependent
   operation and never the subtree's unrelated generations.
9. **Global semantic MUTABLE authority is forbidden; immutable shared artifacts are lawful.**
   Trees may share a physical device, pipeline cache, or compiled-shader artifact when
   ABI-compatible; they never share a mutable semantic registry, scheduler, or singleton.

Exit statement of ASYNC-SUBTREE-NON-FORECLOSURE-0: local async support is proven; remote execution
remains future and is merely not foreclosed; every known blocker above is fenced here or dated in
the constitutional census; nothing in resident clearing may assume one process, host, device,
clock, allocator, or schedule.

### 8.3 Resident RF market clearing — Phase-14 closure

Phase 14 closes the RF market as one recursively self-consuming operator, not as a clearing
subsystem beside RF. Each StemThing receives descendant demand, reduces branch-attributed pressure,
binds already-resolved eligible continuous flow, settles exact constrained supply, disburses it, and
repeats the same germ at the next level and generation. Roots and leaves are degenerate bindings of
that interior germ. There is one market, one generation-relative schedule, one exact settlement
authority in production, and no peer clearinghouse, feedback manager, or second quantity universe.

#### 8.3.1 The mirror cycle and identical recursive ports

Continuous output at level `n` is direct continuous intake at level `n+1`. Exact settlement output
`T_s` is the identical exact recursive supply intake `T_s`: role names may be aliases or
conversion-free views only. The canonical product carries logical semantic-row identity, claimant,
exact granted quantity, exact unresolved `U`, generation, and integration band. It does not acquire
physical row, device handle, host location, or a role-specific payload copy. Detached seams may add
realm, incarnation, generation, retry, and transport envelopes, but must preserve the original
economic payload byte-for-byte.

Unresolved `U` at `N` re-enters the same first-order recursive demand authority at `N+1`; it is not
CostBand `R`, impairment, contention shortfall, or delivery shortfall, and it does not require an
authored persistence overlay to exist. Demand/Draw authorization metadata may remain distinct from
the demand quantity, but must not mint a second quantity authority. No same-generation reweight and
re-clear convergence loop is lawful.

#### 8.3.2 Receive, do not recompute

Exact settlement receives lawful request/cap, the existing resident `AllocatedFlow` (or a proved
equivalent), exact available `T_s`, generation and claimant authority, integration band, commitment
semantics, and the existing hard-precedence decision. Settlement never walks descendants, evaluates
policy, queries routes or congestion, runs STEAD/PALMA/Gu-Yang, estimates private flux or urgency,
or owns a clearing-specific field plane, cache, weight table, solver, registration, or pressure
state. Upstream RF allocation owns branch pressure and authored deformation; already-resolved PALMA
and Gu-Yang effects are consumed, never applied again. Hard precedence remains distinct from
continuous share, and physical order, atomics, workgroup shape, or floating comparison outside the
admitted exact law never decide an exact grant.

#### 8.3.3 Surface-reuse and settled-code census

Before any RF-market binding is added, every proposed role receives exactly one disposition:
`REUSE AS-IS`, `REUSE WITH BINDING`, `REHOME EXISTING AUTHORITY`, or `MISSING — STOP`. A semantic
`MISSING` requires Owner/DA ruling; it is not permission to invent an adapter. Already-settled
market-adjacent code is classified `CONFORMING`, `IN-FLIGHT FIX`, `DATED CONSTITUTIONAL DEBT`, or
`NOT IN SCOPE` (one-way oracle, replay, observation, or structural boundary). Any debt records its
provenance, owner, future consumer, and retirement condition in the existing constitutional census,
never a sibling registry.

The census mechanically protects the recursive ABI, resident/host disbursement, grant lifecycle and
holding-account history, Draw quantity authority, unresolved recurrence, and seam carriage. A
`GrantRow -> ChildSupplyRow` adapter, paired role newtypes joined by `From`/`Into`, runtime
shape-matching, or a seam economic translator is a constitutional failure even if all currently
visible fields happen to be copied.

#### 8.3.4 Resident authority, replay live head, and CPU division

`ClearingExecutionPosture` is orthogonal to scheduling `ExecutionPosture`; paced and continuous
scheduling each admit either `ResidentRequired` or explicit `CpuVendorizedOracle` without changing
market semantics. `ResidentRequired` is the ordinary production default. Its complete
adapter/backend, driver, feature, compiler, shader/compiler, dependency-lock, workgroup,
subgroup-independence, and ABI tuple is qualified at admission; a mismatch fails typed before
execution and never falls back to CPU.

The graduated resident exact result is appended on-device to an admission-bounded segment that is
the authoritative live head of the one `IntegrationSchedule`, then the identical `T_s` bytes enter
the N+1 recursive intake in queue order. Both happen before host readback, CPU grant reconstruction,
vector append, replay drain, or structural handling. The host `Vec` is asynchronous durable/replay
materialization of that same schedule, not a second live history. A live reservation is never
dropped, coalesced, overwritten, or redirected; capacity exhaustion is the typed
`ReplayEgressExhausted` session fault unless an explicitly admitted synchronous-durability posture
exists. While admitted capacity remains, replay egress cannot make N+1 wait.

The existing CPU filter oracle is the vendorized form: the same registrations, math, invariants,
admission, generation/history semantics, and exact products, selected explicitly for diagnostics,
tests, and vendorization proof. It has no hidden ordinary-session caller and no fallback edge from
resident admission or execution. CPU responsibility in production remains asynchronous observation,
replay/persistence materialization, and genuine structural consequences. Only an exact grant that
authorizes such a consequence may project to a sparse sealed `BoundaryRequest`; ordinary economic
continuation stays resident.

#### 8.3.5 Binding falsifiers

The binding is RED if it creates a peer market/settlement/feedback path; recomputes field or policy
state in clearing; makes authored policy necessary for neutral lawful pressure; double-applies
PALMA/Gu-Yang; replaces additive pressure by max/tropical pressure; double-counts branch aggregate
and descendants; lets physical placement decide grants; creates a second history, demand universe,
generation source, or live head; changes `T_s` across a seam; translates settlement output into a
second recursive type; hides or conflates `U`; requires synchronous ancestor RPC or all-tree
barriers; assumes raw ids are globally unique; shares mutable clearing authority across trees; lets
host materialization delay admitted N+1 work; silently falls back to CPU; or moves structural tree
mutation onto the GPU. These are binding failures, not optimization opportunities.

This section is the durable Phase-14 home for the RF Market Core workshop laws. Section 1.5.1 remains
the germ seal; sections 1.2 and 8.2 remain the one-schedule and asynchronous-tree authorities.

### 8.4 Recursive constrained-resource filter — Phase-15 closure

> **KNOWN REMAND (2026-09-03, engineering cross-rung review; DA notice — this block is a
> truth marker, not replacement semantics).** Sections 8.4.1-8.4.3 contain graduated
> explanatory claims presently under `RECURSION-AXIS-CONFORMANCE-0` /
> `RESIDENT-FILTER-SUBSTRATE-BINDING-0` correction: exact settlement is an exact
> CONSTRAINED PROJECTION (feasible set = actual conserved supply + request caps +
> precedence), not a mere quantization of the downward flow, and the resident spatial
> recursion is NOT yet literal (the current interior intake is a same-scope loopback
> under one granter). Use the ACTIVE LADDER rows 15.5-15.7 plus the unification review
> (docs/0_0_8_7_SimThing_Unification.md sec 7) for implementation truth until the
> corrective rows graduate, at which point this block is deleted and the sections
> rewritten.

Phase 15 closes the explanatory gap left by the graduated RF market without adding a runtime
quantity, response program, settlement rule, or execution authority. The market is the resident
recursive filter already admitted by section 8.3. Its continuous projection and frozen exact
projection are distinct stages of one operator, not peer markets.

#### 8.4.1 One recursively composable filter

A StemThing evaluates one recursively composable constrained-resource filter. Its upward projection
is the sufficient response of its subtree to scarcity. Its downward projection is the resource flow
allocated over each child edge. The child consumes that same edge resource as its own supply. Exact
settlement is the deterministic quantization Q of that flow into canonical identity-bearing
possession; it is not a second market.

`RecursiveResourceFilterRuntime` is a conversion-free type alias for the existing
`ResidentClearingRuntime`, which remains the sole ordinary production R->Q executor.
`evaluate_recursive_resource_filter_oracle` is an item alias for the existing generalized
`run_arena_allocation_oracle`; it is the CPU reference view of R and contains no wrapper arithmetic.
The frozen exact CPU oracle remains Q. An alias or view is lawful only when Rust type/function
identity and every output bit are unchanged; it cannot allocate storage, translate an economic
payload, select policy, or become a callable peer authority.

#### 8.4.2 Spatial and temporal recursion axes

Spatial continuous recursion is literal edge identity:
`x_(p->v) = AllocatedFlow[v] = S_v`. The parent emits `x` and the child reads that same resident cell
as its incoming supply; no disbursement report, copied role payload, or host intermediary belongs
between the two. Spatial exact recursion is likewise literal identity:
`Q(x_(p->v)) = T_s(p->v) = ResidentRecursiveSupplyIntake[v]`. The settlement and intake names remain
conversion-free views of the canonical identity-bearing product.

Temporal recursion remains exactly
`d_effective(v,N+1) = d_authored(v,N+1) + U(v,N)`. It crosses the single Current-to-Next mint once;
same-generation clear/reweight/re-clear is forbidden. An admitted persistence deformation may
transform U only inside that mint under the section 8.3 generation authority: authored policy
deforms and the substrate creates. The valuation -> CostBand -> Overlay consequence chain remains
beside the mint, never feeds demand, and cannot become a persistence market, carry lane, or
migration adapter.

#### 8.4.3 Born sufficient statistic

The sufficient-statistic presumption is closed as a theorem. Before R, the already-authoritative
eligibility projection selects the lawful pressure or serviceable-flow source exactly once and
binds one scalar `AllocatorWeight`; simultaneous `(P,F)` response storage would duplicate upstream
provenance. The upward statistic is the existing direct-child `weight_sum/P_up`. The downward state
is the incoming `AllocatedFlow`, that sum, and each direct child's eligible weight. A subtree query
is answered from this born statistic; it never authorizes a host descendant walk or reconstruction.

Normalized scarcity response is implicit in `weight/weight_sum`: scaling all eligible weights by a
common positive scalar preserves the emitted flow bits. For exact Q, the existing request cap,
`AllocatedFlow`, hard precedence, identity, supply, and generation are necessary and sufficient.
There is no runtime lambda, response curve, shadow price, scarcity column, or richer response tuple.
For each frozen admitted program family the resident representation is O(1) per node; recursion
changes row count, not the number of response fields or operators.

PALMA route/impedance and Gu-Yang serviceability influence a causal quantity upstream once; R
consumes the sealed born eligibility value and never solves either field. RF execution-stage order,
hard precedence, smooth continuous share, and exact residue are four distinct authorities and may
not be encoded into one another. Physical row placement, workgroup shape, dispatch partition,
epoch, realm, host scheduling, and observation timing cannot alter R, Q, or T_s.

#### 8.4.4 Peer-authority deletion census

Phase-15 closure uses a symbol-keyed public/runtime census, not prose disappearance. The audited
pre-cutover peer set contained five symbols:
`apply_owner_silo_runtime_disburse_down_cpu`, `compile_owner_silo_disburse_down_plan`,
`compile_owner_silo_disburse_down_plan_from_owner_view`,
`evaluate_owner_silo_disburse_down_with_rf_source`, and
`runtime_local_allocation_from_owner_silo_disburse_report`. Closure deletes the unused scenario
compiler wrapper and internalizes the owner-view compiler plus report-to-local-allocation helper.
The two still-required public report/oracle witnesses remain quarantined compatibility proof
surfaces, so `N_peer_runtime_authorities` decreases from 5 to 2. Two conversion-free canonical
aliases were added, three public/runtime nouns were removed, and `N_new_economic_authorities = 0`.

The separately frozen CPU compatibility doors remain exactly
`clear_constrained_claims_at_generation`, `clear_reduced_owner_channels`,
`clear_reduced_owner_channels_at_generation`, `clear_stamped_owner_channels`, and
`produce_runtime_rf_next_generation_demands`. Their constitutional census and caller census are
the retirement mechanism. They are vendorized oracle vocabulary, do not define the architecture,
and may not acquire an ordinary resident caller, fallback edge, or sixth door. Structural grant
recorders remain one-way consequence authorities after Q and are excluded from the peer-market
metric; they cannot translate or reinject an economic product.

---

## 9. The drift detectors — litmus tests for every change

Stop and escalate when a change does any of the following:

1. branches on runtime SimThing kind or gameplay nouns;
2. builds a manager, planner, allocator, map, overlay, action, combat or economy subsystem beside the tree;
3. makes an owner a spatial parent or capture a reparent;
4. lets CPU traversal decide what a resident threshold should resolve;
5. writes semantic nouns into simthing-sim or WGSL;
6. creates a rival ledger, schedule, history, generation source, crossing surface or clearing path;
7. hardcodes a column, physical row, allocation order or authored string as numerical authority;
8. violates Movement-Front locality, symmetry, stability or bounded feedback;
9. adds an opcode or kernel before expressing the behavior in admitted EML data;
10. claims exactness without the exact admission/oracle contract;
11. flattens a specified recursive structure without an approved deviation;
12. mints decision effects outside the sealed crossing and consequence ingress;
13. stamps inherited owner or subtree overlay state onto every descendant;
14. lets install_initial_tree admit attached growth after initial residency;
15. bypasses unified ingress or adds a producer with neither a production consumer nor dated deferral;
16. writes a lifecycle artifact through a vendor door that cannot reopen it.
17. couples two executing trees by anything other than stamped seam facts;
18. compares, merges, or hashes raw local ids across executing trees;
19. binds two trees' generation clocks outside a declared common-lineage seam;
20. adds identity, realm, host, or device weight to base SimThing anatomy;
21. rides a conserved or simulation-authoritative seam fact on a lossy observer surface.
22. inserts an adapter between a germ facility's emission and its own intake one level down.

### 9.1 Rejected rival shapes

The following shapes are canonically rejected:

- opt-in upward event subscriptions copied from CPU handler taxonomies;
- observation versus sink as a fixed event taxonomy;
- CostBand as an optional marker or negative coefficient as destruction;
- an authored cascade-depth limit instead of generation pacing;
- per-tree parity as a new invariant rather than parity of the executed tree;
- direct Overlay.affects routing that bypasses intermediate policy;
- descendant Set/value semantics reused for conjunctive policy selection;
- flat owner stamps, Option-none ownership, or foreign identity aliasing to unowned;
- per-leaf materialization of ancestor-resident overlays;
- an overlay-local EML evaluator, history, clock, lifecycle manager or convergence loop;
- PALMA route objects, Gu-Yang reconstruction inside ActionBand, or CostBand as throughput authority;
- a second allocator or grant manager outside the StemThing-B RF market;
- a producing-side-only vendor lifecycle door.

The six-line harness is:

1. Everything is a SimThing: behavior is admitted data on the recursive germ.
2. Conflict, opportunity, ambition, extraction and residency flow through RF and constrained clearing.
3. Allocation is recursive through one tree; settling depth is emergent.
4. Decisions are resident crossings; the map is a local, symmetric, stable Movement-Front automaton.
5. Runtime numerical code is semantic-free EML and Field-Triad data with exact claims oracle-qualified.
6. Proof uses a real reduction; documents record graduated truth and never constitute execution.

---

## SimThing tools crate — presentation/support services

crates/simthing-tools imports, stages and renders presentation artifacts: fonts, shaping, glyph and
distance-field atlases, SVG icons, style/deformation/path/warp tables and Studio labels. It is not
simulation authority and may not introduce gameplay semantics into GPU shaders.

---

## References

- Zichao Wei, On the Spatiotemporal Dynamics of Generalization in Neural Networks
  ([arXiv:2602.01651](https://arxiv.org/abs/2602.01651)).
- Andrzej Odrzywołek, All elementary functions from a single operator
  ([arXiv:2603.21852](https://arxiv.org/abs/2603.21852)).
- [0.0.8.7 RF arena modernization](design_0_0_8_7_rf_arena_modernization.md) — graduated source
  rulings and rung stamps.
- [STEAD spatial contract](stead_spatial_contract.md) — spatial and field-sweep invariants.
- [StemThing unification](stead_stemthing_unification.md) — StemThing-A and Section-12 market grammar
  provenance.
- [Full EML unification](full_eml_unification.md) and
  [EML gadget library](eml_gadget_library.md) — complete ISA, exact admission and library law.
- [ActionBand STEAD](multi-axis-ActionBand-STEAD.md) — intrinsic actuation and Field-Triad binding.
- [Intrinsic overlay capability](stemthing_intrinsic_overlay_capability.md) — overlay closure and
  composition provenance.

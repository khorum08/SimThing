# STEAD StemThing Unification (design anchor)

> **Status: DESIGN ANCHOR (Owner-directed 2026-08-03; DA-authored, Fable).** This document is the
> normative anchor for the **StemThing unification** — the refactor phase in which allocation,
> registration, derivation, and layout cease to be subsystems and become lanes on the one base
> recursive SimThing. It binds design conversation the way `stead_simthing_automata.md` bound
> Phase 6 before its rows existed. It does **not** open a rung, alter sequencing, or amend any
> phase row; ladder rows are minted by DA amendment under the usual gates, subject to the
> sequencing constraints in §10. Where this document and the core design conflict, the core design
> wins and the conflict escalates.
>
> Companions: [`simthing_core_design.md`](simthing_core_design.md) (the paradigm),
> [`stead_simthing_automata.md`](stead_simthing_automata.md) (the four legs; normative for Phase 6),
> [`eml_n4_expansion_digest.md`](eml_n4_expansion_digest.md) (field-sweep provenance),
> [`stead_spatial_contract.md`](stead_spatial_contract.md) (spatial law; CostBand definition §5).

---

## 0. Verdict — why this justifies a refactor phase

Owner-directed review of the unification arc (field-sweep IR → intrinsic ownership → four legs →
CostBand → resolution sites → malloc-as-RF → tiling → derivation surface → descendant census →
rootness) concludes the phase is **justified**. Four benefit classes, each grounded in a landed or
in-flight mechanism:

1. **Architecture.** The last un-unified mechanism dissolves. The corpus has no combat engine, no
   pathfinding engine, no border service; after this phase it has no allocator, no registrar, and
   no derivation framework. Every one of these is the same deliverable the track has landed five
   times: a deletion, with a lane left where a subsystem stood.
2. **Capability.** Admission-by-conservation (§4), shape-anticipating pre-allocation (§6),
   ontogeny as ground-truth corpus columns (§6), and the learned-policy-as-data loop extended to
   the engine's own resource management.
3. **Function.** Elastic vendoring (a new domain is authored tier rows + draws, zero engine code),
   recursive sandboxing as an engine-native memory behavior, and the CPU boundary's allocation
   stages shrinking from "the floor the closed loop can never save" to an ordinary lane.
4. **Concentration.** One object carries every capability; iteration, debugging, review, and
   regression proof concentrate at the point where the admission substrate is already densest.
   This is the Owner's stated goal — a single point of management for a human and for an
   agent-led regime alike — and its honest cost is named in §9: a single point of iteration is a
   single point of regression, acceptable only because it sits where enforcement is strongest.

**Why now and not earlier:** the unification was inexpressible until this track built its parts.
The CostBand (6.1b) is the quantizer between continuous flow and discrete grants; generation
stamps (6.1) are the determinism-by-recording seam; the contention arena (8.2) is the resolution
mechanism; resource classes + the field-sweep JIT (5.7) are the layout vehicle. None existed at
track opening. The phase consumes them; it invents almost nothing.

---

## 1. The thesis — the StemThing

**The StemThing is the base recursive SimThing carrying every capability as an inert-by-default
lane — including the capability to hold memory, grant memory, and derive descendants.** A
specialized SimThing is not a type: it is a **derived descendant produced through the ordinary RF
process under conservation law** — a draw on its parent's derivation surface that simultaneously
prices its lanes, its memory shape, and its slot dynamism. All governance of the simulation lives
inside the family; what remains outside is physics (§8).

The test every piece of this phase must pass:

> **Malloc arrives as a lane, not a leg.** If integrating memory or derivation awareness requires
> a fifth capability on the base object, the unification has failed. The four legs (participate,
> act, originate, receive) are complete; residency and derivation are *lanes flowing through
> them*, not new anatomy.

The identity that makes the phase coherent — the malloc finding and the tiling finding are one
finding viewed from two sides:

> **A granted slot budget IS a contiguous block IS a tile.** The RF quantity, the memory geometry,
> and the reconciliation unit are the same object. Disbursing residency capacity down the tree and
> laying the tree out as nested contiguous ranges are one act.

---

## 2. The residency lane

Slots are a conserved, discrete, contested resource. The lane maps onto the four legs with no new
mechanism:

| Leg | Residency semantics |
|---|---|
| **participate** | residency demand/balance are ordinary columns; claims reduce up, capacity disburses down through owner channels |
| **act** | the slot draw is a CostBand — `N = floor(V/C)` grants N rows at unit cost C = rows; `R` carries forward |
| **originate** | allocation policy is overlay weights on the residency lane (priority, reservation, eviction bias) — authored data, never an allocator rule |
| **receive** | a grant is a directive disbursing down; a growth need is a deficit surfacing up — the existing delivery modes, unchanged |

**The CostBand is the necessary adapter, which is why this was impossible before 6.1b.** Slots are
hard currency; RF flow is continuous and approximate-deterministic (O(ε × n) allocator
conservation). Those worlds may never touch directly — `V = N·C + R` is the quantizer that floors
a continuous demand accumulation into a discrete, exact grant with a carried remainder. The reason
malloc resisted unification for the project's whole history is that this primitive did not exist
as a first-class mechanism until the CostBand was canonized.

**Closures that fall out (no new machinery):**

- **Pre-allocation becomes `governed_by`.** Core design §2.1's mandate — slot pre-allocation on
  measured growth rates, never heuristics — stops being a special CPU obligation: the velocity
  column drives the demand column, the deficit surfaces ahead of the fission burst, the budget
  arrives before the crossing.
- **Defragmentation becomes a threshold registration.** The CostBand remainder `R` *is* the
  fragmentation: granted block = used rows + slack, exactly `V = N·C + R` in memory form. Slack is
  a column; an authored threshold on it fires epoch compaction at a boundary; the Definable
  Horizon Law forbids slack accreting forever. The allocator's GC heuristic becomes an ordinary
  authored crossing.
- **Allocation storms are already absorbed.** Slot claims for the same `{owner, residency, scope}`
  bucket coalesce by addition (integers, lossless) under the 6.2 queue — a subtree fissioning at
  100× queues one coalesced claim. Backpressure is cardinality-bounded by authoring, exactly as
  6.2 established for every other product.

---

## 3. Grant determinism and placement stratification

- **Slots are hard currency.** Residency transfers use discrete-exact semantics
  (`SubtractFromSource` class) only; the continuous approximate allocator path is forbidden for
  slots. `free + occupied = capacity` holds **exactly**, every boundary, judged by the
  8.1-class conservation oracle. A slot leak is a conservation violation, not a bookkeeping bug.
- **Determinism by recording, not by waiting.** The grant schedule — which claims were granted
  which ranges at which generation — is recorded data, exactly as 6.1 records the integration
  schedule. Replay the schedule, reproduce bit-exactly.
- **Priority is authored, never iteration order.** The legacy LIFO free-list order is a hardcoded
  rule the corpus's own law (8.2: "which claimant is served first is an AUTHORED rule, never
  vector iteration order") already condemns. It retires into an authored resolution rule over the
  contention arena; slot contention is ordinary contention.
- **Placement stratification.** Claims and grants recurse freely at any depth, any time. **Placement
  changes only at the generation barrier of the level that owns the containing block.** A subtree
  subdivides its granted range autonomously — in-shader, from pre-granted budget; it never
  relocates its own block. Relocation (epoch compaction) is boundary structural work of the
  granter, with a **recorded remap** so replay and the shadow survive. Memory settles at barriers
  the way flow settles in arenas: allocation is always recursive; settling depth is emergent.

---

## 4. Admission by conservation

Today an expansion cap is a rung-2 admission hard-error: validation code checks
`max_participants` at session build. Under the derivation/residency lanes **a cap is a budget**:
exceeding it does not fail a check — the draw floors to zero because `V` is exhausted. The illegal
state is not merely unrepresentable; it is **unreachable by arithmetic**. No validation code
exists to maintain, bypass, or drift past.

This is the admission ladder's own directive ("encode every invariant at the highest rung that
can express it") taken above the type boundary: quantitative admission migrates from the spec
compiler into conservation law. The split it induces is principled and permanent:

- **Quantitative admission** (caps, budgets, fanout, expansion) → in-family, enforced by
  conservation.
- **Structural and type admission** (schema validity, sealed constructors, spanned content
  rejection) → stays at the boundary. The gate that judges what may become family must stand
  outside the family.

---

## 5. The derivation surface

**Derivation-as-CostBand is a restoration, not an invention.** The 6.1b archaeology found the
primal band was authored as `num_to_produce × cost_to_produce` — production of units was the
original mechanism, and it drifted apart. The derivation surface is that mechanism applied to the
ontology itself: **deriving a specialized SimThing is a draw on the parent's derivation CostBand,
where the thing produced is a member of the family.** Producing a cohort unit and deriving a
specialist are one operation at different tiers; the special case stops existing, exactly as
booleans became depth-1 CostBands.

**A tier is a price vector, never a category:**

```
tier = { lane set, residency_class, adjacency participation, churn class, unit cost }
```

Drawing at a tier simultaneously answers what the descendant can do (lanes), what memory shape it
receives (tile dimensions — spatial block or compact row), and how its slots behave (dynamism
class). The tier does not *cause* those properties; they are its *components*.

**Enumerate shapes, never instances; author pathways, never a manifest.**

- Vendors enumerate their entities **by name, in their own crate** — that vocabulary never reaches
  the engine (semantic-free core; vocabulary compiles away).
- The shape ladder underneath is small and closed at design time. The canonical worked example: a
  galaxy worldmap (150×150 gridcell children), a star system (10×10), a planet (10×10), and a
  planet surface (10×10) are **one tier — spatial container — drawn four times at different N**
  (22,500 / 100 / 100 / 100). Scale is a draw quantity, never a shape property; `10×10` sits
  inside one standard dense theater while `150×150` admits as layout and defers dense execution to
  the atlas — the existing layout-vs-execution-profile law doing its job.
- A full grand-strategy domain needs on the order of **four tiers** (spatial container, compact
  participant, compact policy holder, granting root) against dozens of named entity types and
  tens of thousands of instances. That ratio is the health test.
- **Smallest-fit, not hand-assignment.** Designers declare what an entity *needs* (lanes); the
  engine computes the residency class and tile shape that fits — the same deterministic
  smallest-fit discipline `EmlResourceClass` established.
- **Banding gates derivation; it never catalogues descendants.** Threshold ladders decide *when*
  derivation fires; there is no band-per-possible-descendant.

**Derivation capacity is the budget's meaning.** `V` on the derivation lane is remaining
derivation capacity: drawing specializes and depletes; a terminally-specialized SimThing is one
whose capacity is exhausted — not forbidden by rule, empty by arithmetic. Replenishment is
authored (`governed_by` inflow for self-renewing lineages; none for terminal ones), and under the
Definable Horizon Law capacity itself carries an authored horizon, never implicit permanence.

**Narrowing falls out of budget, not permission.** A descendant inherits the *entire* tier ladder;
what limits it is what it was granted. There is no access-control layer, no capability masking,
no per-level whitelist — conservation does the narrowing.

---

## 6. Descendant census — awareness as perception

Because the tier set is closed at design time, the subtree census is a **fixed-width column
vector**: counts, churn, and growth velocity per tier, Sum-reduced up like any other columns,
generation-stamped (6.1), staleness derived and visible (6.3). This is the only lawful form of
descendant awareness:

> **Census is perception, never a directory.** A node knows its subtree's aggregate composition,
> slightly historical, through banded reduce-up — its light cone. No tree walk, no global registry
> query, no synchronous view. P1 applies to self-knowledge exactly as it applies to threat fields.

What the census endows:

- **Shape-anticipating pre-allocation.** §2.1's velocity-driven pre-grant generalizes from "how
  many slots" to "what shaped blocks," because the census carries tier composition.
- **Live budget governance.** Per-tier caps enforced as budgets (§4) with the census as the
  observable side.
- **Ontogeny as corpus ground truth.** Speciation rates, tier distributions, schema evolution
  become replayable corpus columns — a world model observing not just world state but world
  *schema* dynamics. No natural corpus offers this axis.
- **The learned-policy loop closes over the substrate itself.** Allocation and derivation policy
  are authored data (weights, priorities, thresholds); a policy learned from the corpus deploys as
  data with zero engine change. Measure → learn → author → no code — extended to the engine's own
  resource management.

---

## 7. Rootness as the granting tier — and where layout lands

**Rootness is a capability tier, not a type.** Per-Tree Instantiation already states it in
CPU-struct form ("a subtree IS a root SimThing tree and gets its own allocator, session state,
shadow, and generation counter by instantiation"); `GameSession` already documents itself as
"authority marker only; NOT a runtime engine singleton." This phase makes it intrinsic: every
StemThing carries, inert, the capacity to hold an arena, run a census, and grant budgets. Activate
it and any SimThing is a subtree root. **A child root's arena IS its granted block** — nested
arenas are nested contiguous ranges; sandbox subtrees, vendored builds, and high-churn experiments
purged wholesale at horizon are all "a SimThing with its rootness lanes active." Micro-subtrees
share the device context (per-tree GPU contexts are disqualifying, per the automata doc §10);
the arena hierarchy is ranges within one physical arena.

**Layout enters as `residency_class` — the sibling of `resource_class`.** A closed, authored,
admission-checked set (compact row; spatial block; granting arena), selected smallest-fit,
executed as **JIT layout variants of the one field-sweep IR**. `FIELD-SWEEP-SINGLE-PATH` must
never fire: tiles are a *where*, not a *what*, and the 5.7 safety template governs every layout
move — *it changes WHERE values load from, never the ORDER they fold*, so `CanonicalOrderProof`
and bit-exactness survive relocation, tiling, and packing alike.

Current distance (measured this session, recorded so the phase scopes honestly):

| Component | Status |
|---|---|
| Compact row per SimThing (`slot-major`, contiguous) | **byte-literal today** — the micro-tile already exists |
| Spatial field tiles + double-buffer + masked tombstones | built |
| SRAM tiling in sweeps | scheduled (`FIELD-SWEEP-TILED-GATHER-0`) |
| Subtree-contiguous child blocks | **this phase** — turns tree reduction from scattered `INPUT_LIST` gathers into coalesced range folds |
| Per-block reconciliation (partial shadow) | **forced by this phase** — granted ranges are the dirty-tracking unit; the full-shadow upload retires |
| Tree seams | range seams, never halos — trees are not lattices |
| MMA / f16 / tensor hardware | refused: unreachable via wgpu and barred by the determinism law; the hand-rolled canonical-order contraction over contiguous ranges captures the memory-traffic win deterministically |

**The map is a derivation pattern.** A galaxy is the root drawing a spatial-tier subtree whose
residency class is a 2D block: the lattice materializes as the grant, and the STEAD field sweeps
what derivation placed. MapGen becomes a derivation program; the Movement-Front automaton runs
over memory the derivation surface shaped. This is Wei's postulates applied to memory itself:
locality (claims resolve against locally pre-granted budgets; the global pool is touched only at
barriers), symmetry (one authored grant rule at every arena, never per-site allocation logic), and
stability (exact conservation plus bounded budgets — growth is a threshold crossing, never a
surprise).

---

## 8. What dissolves, what remains — the fixed points

| Stratum | Disposition |
|---|---|
| Allocation/derivation/registration **policy** | in-family: lanes, CostBands, authored rules, budgets |
| The allocator, the registrar, the derivation framework **as services** | **dissolved** — no such subsystem exists after this phase |
| The **executor** (buffers, dispatch, placement mechanics) | remains outside as physics: the kernel interprets, owns memory per the cross-crate seal law, and holds zero policy |
| Structural/type **admission** | remains at the boundary: the immune system stands outside the body it screens |
| The **base case** | remains the embedder handshake: physical VRAM capacity is a fact about the host, granted to the root at instantiation (the Vendor Door's Populate declares it as data); the recursion bottoms exactly one handshake outside the family, which is what makes the self-reference well-founded rather than circular |

External **code** persists; external **authority** does not. The kernel becomes to the family what
silicon is to a program. And the self-reference discipline is the same stratification as §3: the
root may describe and govern its subtree's memory; it may never relocate itself mid-flight — its
own placement belongs to its granter's barrier, which for the session root is session scope.

---

## 9. Binding laws and drift detectors

**Laws (numbered; every rung of the phase cites the ones it touches):**

1. **Lane, not leg.** No fifth capability on the base object; residency and derivation flow
   through the existing four.
2. **Slots are hard currency.** Discrete-exact transfers only; `free + occupied = capacity`
   exactly, judged at every boundary.
3. **Admission by conservation.** A cap is a budget; no validation code owns what arithmetic
   already forbids.
4. **Placement stratification.** Self-description always; self-relocation never; placement changes
   only at the granter's barrier, with a recorded remap.
5. **Grant determinism by recording.** Recorded grant schedule + authored priority; no
   iteration-order allocation anywhere.
6. **A tier is a price vector.** Open set of authored bundles, domain-free, smallest-fit; the
   engine never branches on tier.
7. **Rootness is a tier, not a type.** No service nodes.
8. **Census is perception.** Banded, stamped, aggregate, stale-visible; never a directory.
9. **Layout is a residency class of the one IR.** No bespoke kernels; WHERE never ORDER.
10. **The base case is one handshake wide.** The embedder grants the root arena as data; nothing
    else external holds authority.

**Drift detectors (stop and escalate on any "yes"):**

1. Am I writing `match tier` — or naming a tier with a domain noun?
2. Am I minting an `AllocatorThing`, `RegistrarThing`, or any entity type that *owns* granting?
3. Is my tier-to-entity ratio approaching 1:1 (a taxonomy wearing a budget)?
4. Am I answering "what exists?" with a tree walk or global query instead of census columns?
5. Am I giving derivation capacity implicit permanence, or slots a continuous-flow path?
6. Am I relocating a block outside its granter's barrier, or without a recorded remap?
7. Am I hand-assigning a residency class where smallest-fit should compute it?
8. Am I writing a new layout kernel instead of a JIT residency-class variant of the one IR?
9. Am I building an access-control layer where budget narrowing already governs?
10. Am I validating a quantity a conservation law could make unreachable?

---

## 10. Phase disposition and sequencing constraints

- **Consumes (must be landed):** 6.0 owner channels; 6.1 stamps; 6.1b CostBand; 6.2 coalescing
  queue; 8.1 conservation judge; 8.2 contention arena (slot contention resolves there — the
  arena's authored-rule law is what retires LIFO).
- **Must precede:** 11.1 `EMBEDDER-INTERFACE-0`. The five verbs must express this phase as data —
  **Populate** declares the root arena and slot budgets; **Derive** declares tier rows and draws.
  A door frozen before this phase ships unable to express the capability, and retrofitting it
  violates the containment law at the one surface vendors touch.
- **Exemplar falsifier (for 11.2):** a vendor domain standing up **many named entity types from
  few authored tiers**. An exemplar needing a tier per entity proves the derivation surface has
  become a taxonomy, and fails regardless of green checks.
- **Success signal (arithmetic, per the track's own convention):** net deletion — allocator-specific
  code paths retire; the CPU boundary's allocation stages shrink from the closed-loop cost floor
  to an ordinary lane; no new scan surface is minted (the phase should *retire* allocation-shaped
  validation, per §4).
- **Grant-site ruling to make early, with measurement:** barrier-resolved grants from arena
  resolution, with pre-disbursed ranges for in-shader minting (the hybrid). Claims reduce up
  during ticks; grants disburse at the barrier; a subtree mints mid-generation only from its
  pre-granted range. First-run determinism is trivial under this shape; measure before admitting
  anything more exotic.

## 11. Open questions

1. Does derivation capacity conserve across fission — does a successor inherit remaining `V` by
   the intensity-vector partition, or re-draw? (The `FissionPolicy` analogue for the derivation
   lane; decide deliberately, not by default.)
2. Census staleness bounds for pre-grant under forced lag — the 6.3 soak is the natural home for
   proving pre-allocation stays ahead of bursts at N generations of lag.
3. Epoch-compaction cadence: threshold-fired only, or threshold plus an authored floor cadence?
4. The exact Populate/Derive schema for budgets and tier rows (11.1 design work, bound above).
5. Whether the census vector wants velocity lanes per tier from birth, or grows them on demand —
   the fixed-width commitment makes this a design-time choice.

## References

- [`simthing_core_design.md`](simthing_core_design.md) — §1.2 admission ladder; §2.1 fission and
  velocity-driven pre-allocation; §3 registry discipline; §5 arenas and hard-currency law; §7
  Movement-Front; §9 drift detectors.
- [`stead_simthing_automata.md`](stead_simthing_automata.md) — the four legs; events-are-RF;
  Per-Tree Instantiation; resolution sites; governing laws.
- [`stead_spatial_contract.md`](stead_spatial_contract.md) — §5 CostBand definition; layout vs
  execution-profile admission.
- [`eml_n4_expansion_digest.md`](eml_n4_expansion_digest.md) — field-sweep IR provenance;
  `FIELD-SWEEP-SINGLE-PATH`; the WHERE-not-ORDER safety template; preservation invariants.
- `design_0_0_8_7_rf_arena_modernization.md` — 5.4–5.8, 6.0–6.3, 8.1–8.2, 11.1–11.2 rows; the
  governing laws (§4) this document extends to memory and derivation.

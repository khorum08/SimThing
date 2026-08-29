# STEAD StemThing Unification (design anchor)

> **Status: DESIGN ANCHOR (Owner-directed 2026-08-03; DA-authored, Fable).** This document is the
> normative anchor for the **StemThing unification** — the refactor phase in which allocation,
> registration, derivation, and layout cease to be subsystems and become lanes on the one base
> recursive SimThing. It preserves the design provenance now canonized in
> `simthing_core_design.md`. It does **not** open a rung, alter sequencing, or amend any
> phase row; ladder rows are minted by DA amendment under the usual gates, subject to the
> sequencing constraints in §10. Where this document and the core design conflict, the core design
> wins and the conflict escalates.
>
> Companions: [`simthing_core_design.md`](simthing_core_design.md) (the paradigm and four-leg germ),
> [`eml_n4_expansion_digest.md`](eml_n4_expansion_digest.md) (field-sweep provenance),
> [`stead_spatial_contract.md`](stead_spatial_contract.md) (spatial law; CostBand definition §5).
>
> **Amended 2026-08-03 (decorrelated DA review — Codex 5.6 Sol: REMAND FOR TARGETED DESIGN
> AMENDMENT; all eight amendments adopted, both ladder obligations bound).** The remand corrected a
> category error (quantity/geometry/identity conflation, §1), surfaced a constitutional blocker
> (slot-identity stability vs. compaction, §3.1 — **blocking before any row is minted**), and
> resolved an internal contradiction (tier vocabulary, §5). The refactor phase, the StemThing
> thesis, and the deletion deliverable stand unchanged.
>
> **Amended 2026-08-03 (second — DA disposition DESIGN-ADMITTED, remand closed; Owner rulings
> recorded).**
>
> **Amended 2026-08-04 (STEMTHING-CENSUS-TIER2-SESSION @ `a3183b17`): the §3.1 HARD HOLD is
> DISCHARGED.** The census ran (single pass, post-Phase-6, per the Owner timing ruling):
> `scripts/ci/stemthing_slot_census.tsv` — 25 artifact rows; accessor-consumer universe **51/51** assigned (compiler-harvested under the
> strict CENSUS-note filter; two earlier counts of 61/62 were contaminated by unrelated bevy/mapgen
> deprecation warnings — corrected 2026-08-04), analysis-added carriers named, grep residue fully
> dispositioned, **zero `BLOCKER`, zero `ORDER-PIN`** —
> and **shape (a) is RULED from that evidence**: stable logical `SlotIndex`, per-epoch physical
> binding, rebind only via the existing `AnchorLocusRemap` door (5.2). The Tier-2 amendment is
> landed in core design §3 (slot-identity law) and §4. StemThing-A rows are minted: **6.4
> `SLOT-LOGICAL-IDENTITY-0`**, **6.5 `RESIDENCY-TIER-VOCABULARY-0`**; the §7.1 movement clause is
> bound onto the 7.1 row. Workplan integration only — implementation flows through the normal
> DA→orchestrator→coder regime. The pointer stays `none`; **the flip to 6.4 is the Owner's act.**
>
> Sol's REMAND is lifted; ladderization ~~remains on **HARD HOLD pending §3.1**~~ (discharged, see
> above). Three
> Owner rulings recorded in place: census timing (§3.1 — post-Phase-6, single pass), phase
> placement and the movement interaction (§7.1, §10 — split insertion; StemThing-A precedes the
> Phase 7 movement rungs), and the mid-session tier door (§5 — chartered as future owner-gated
> design; the freeze is law until it exists). §0 wording normalized per DA editorial.

---

## 0. Verdict — why this justifies a refactor phase

Owner-directed review of the unification arc (field-sweep IR → intrinsic ownership → four legs →
CostBand → resolution sites → malloc-as-RF → tiling → derivation surface → descendant census →
rootness) concludes the phase is **justified**. Four benefit classes, each grounded in a landed or
in-flight mechanism:

1. **Architecture.** The last un-unified mechanism dissolves. The corpus has no combat engine, no
   pathfinding engine, no border service; after this phase it has no allocator **service**, no
   registrar **policy/service**, and no derivation framework (the registry *substrate* remains as
   sealed admitted metadata — §8). Every one of these is the same deliverable the track has landed
   five times: a deletion, with a lane left where a subsystem stood.
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

The relation that makes the phase coherent — stated precisely, because its first form was a
category error (header amendment). A scalar free-count is not a contiguous range: two arenas with
identical free capacity can differ radically in allocatability. Quantity, geometry, and identity
are three things, and the unification is the weaker, architecturally stronger statement:

> **Capacity is the conserved RF quantity. A contiguous extent is the sealed placement result the
> granter's kernel-owned placement boundary mints when an exact draw succeeds. The extent is
> execution geometry, not the conserved quantity itself.** Allocation *policy* dissolves into RF;
> placement remains kernel physics (§8). What survives of the original identity is the part that
> mattered: disbursing capacity down the tree **authorizes** nested extent-minting at each
> granter's boundary, so the domain hierarchy and the memory hierarchy remain expressions of one
> act — quantity flowing as RF, geometry minted where placement truth lives.

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
- **Defragmentation becomes a threshold registration — on geometry, not remainder.** The CostBand
  remainder `R` is unallocated *quantity*; `R = 0` can coexist with a heap fragmented into
  unusable singletons. The compaction trigger therefore fires on **placement telemetry the kernel
  reports as ordinary columns** (largest free extent, extent count) — kernel-derived geometric
  observables, RF-thresholded. The Definable Horizon Law binds both: neither slack quantity nor
  fragmentation geometry may accrete without an authored horizon. The allocator's GC heuristic
  still becomes an ordinary authored crossing; its operand is geometry.
- **Allocation storms are already absorbed.** Slot claims for the same `{owner, residency, scope}`
  bucket coalesce by addition (integers, lossless) under the 6.2 queue — a subtree fissioning at
  100× queues one coalesced claim. Backpressure is cardinality-bounded by authoring, exactly as
  6.2 established for every other product.

---

## 3. Grant determinism and placement stratification

- **Slots are hard currency, and the partition names the seam.** Residency transfers use
  discrete-exact semantics (`SubtractFromSource` class) only; the continuous approximate allocator
  path is forbidden for slots. The exact invariant is
  **`free + in_flight + occupied = capacity`** — the in-flight term is the seam holding account
  for grants issued but not yet delivered, the same lesson 6.2 canonized (the holding account
  belongs in the universe being judged). Judged by the 8.1-class conservation oracle every
  boundary. A slot leak is a conservation violation, not a bookkeeping bug.
- **Extent disjointness is its own judge.** Quantity conservation cannot prove two children were
  not granted overlapping ranges. Minted extents must be provably **disjoint and bounded by the
  granter's own extent** — a placement oracle standing beside the conservation judge, with its
  own planted-defect red.
- **Determinism by recording, not by waiting — one history surface.** The grant schedule — which
  claims were granted which extents at which generation — and every compaction remap **extend the
  canonical 6.1 recorded-schedule surface**. A second history mechanism is forbidden. Replay the
  one record, reproduce bit-exactly.
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

### 3.1 The slot-identity decision — constitutional blocker, ruled before any row is minted

Core design registry discipline states: slots recycle through tombstone free-lists, are **never
compacted mid-session**, and slot/column indices stay stable for the GPU. Epoch compaction with
physical relocation contradicts this under the current meaning of `SlotIndex` — and per this
document's own precedence clause, the core design wins until amended. **Silent reinterpretation is
inadmissible.** Two lawful shapes:

- **(a) Logical/physical split (recommended).** `SlotIndex` becomes stable **logical identity**
  for the lifetime of the SimThing; physical row binding happens at boundary upload, exactly where
  boundary sync already rebuilds and re-uploads slot-bearing artifacts. Compaction re-derives
  bindings at an epoch boundary with a recorded remap; **between epochs there is zero per-access
  indirection** — bindings are baked into the uploaded artifacts. Every authoritative handle
  remains valid across relocation, which is the only shape under which compaction is expressible
  at all.
- **(b) Stability retained.** No mid-session compaction; reclamation only through tombstone
  recycling as today; elastic geometric reclamation deferred to session boundaries.

Shape (a) requires a **Tier-2 core-design amendment** restating the stability law as *stable
within an epoch; rebindable only at a recorded boundary remap*, together with an **enumeration of
every slot-bearing artifact and its rebind path** (accumulator registrations, INPUT_LIST tables,
adjacency tables, arena descriptors, shadow addressing, emission records). Until that ruling
lands, compaction language elsewhere in this document is design intent, not authorization.

---

## 4. Admission by conservation

Today an expansion cap is a rung-2 admission hard-error: validation code checks
`max_participants` at session build. Under the derivation/residency lanes **a cap is a budget**:
exceeding it does not fail a check — the draw floors to zero because `V` is exhausted. The illegal
state is not merely unrepresentable; it is **unreachable by arithmetic**. No validation code
exists to maintain, bypass, or drift past.

This is the admission ladder's own directive ("encode every invariant at the highest rung that
can express it") taken above the type boundary — **narrowed to its true scope**:

> **Fungible scarcity is admitted by conservation; structural shape and execution safety remain
> boundary admission.**

- **In scope** (fungible, conserved, budget-shaped): capacity, expansion allowance, reservations,
  fanout budgets.
- **Out of scope, permanently** (numbers appear, but the constraint is structural): EML stack
  bounds, alignment, address width, adjacency symmetry and conductance certificates, layout
  geometry, schema validity, sealed constructors, spanned content rejection. These stay at the
  boundary — the gate that judges what may become family must stand outside the family, and a
  numeric appearance in a constraint does not make it a budget.

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
- **Tier vocabulary is open across authoring, frozen at session admission, closed engine
  vocabulary beneath.** Authoring may mint tier bundles freely (an open set — no taxonomy
  pressure and no engine branching); **session admission freezes the session's tier set** —
  finite, admission-checked, and the exact moment the ladder's budget arithmetic becomes
  statically solvable; underneath, the engine vocabulary (residency classes, lane semantics) is
  small, generic, and closed, and authored tiers only *compose* it. This resolves open-vs-closed
  without contradiction: open to the designer, fixed for the session, closed in the engine.
  **Owner ruling (2026-08-03): a mid-session tier door is chartered as a future, separately
  designed, owner-gated capability** — dynamic ontogeny (runtime minting of genuinely new tier
  definitions) is worth pursuing for the oblique capability and corpus richness it may expose,
  provided new definitions pass **session-admission-grade validation at an epoch boundary** (the
  injection-corruption guard is the same admission substrate, applied later). Until that door is
  designed, the freeze is law, census width is session-fixed, and "schema evolution" in the corpus
  sense means **usage evolution over the admitted vocabulary, not vocabulary invention** (Sol's
  sharpening, adopted).
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

Because the session's tier set is frozen at admission (§5), the subtree census is a
**fixed-width-per-session column vector**: counts, churn, and growth velocity per tier,
Sum-reduced up like any other columns, generation-stamped (6.1), staleness derived and visible
(6.3). Census lanes exist **only on nodes with granting active** — sparse, inert-by-default
economics; the cost is never `O(nodes × authored tiers)`. This is the only lawful form of
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

### 7.1 Movement never moves memory — the binding clause on the Phase 7 rungs

Core law already states that movement is the **only** spatial reparenting and that it executes as
**column updates and arena re-enrollment** — a parent-pointer flip plus membership changes, zero
row relocation. The residency design preserves this by construction, through the tier's churn
class:

- **Static-class SimThings** (gridcells, spatial containers) never move; their
  subtree-contiguous blocks and coalesced reductions never degrade.
- **Mobile SimThings** (the churn class) are allocated from **churn arenas — never embedded
  inside a spatial container's block** — and are gathered by arena membership (`INPUT_LIST`),
  which is exactly how the RF column structure already sustains them today. Reparenting flips
  columns and memberships and **cannot fragment a spatial block**, because the mover's rows were
  never inside one.

**Binding forward clause (Owner ruling 2026-08-03, motivating the split insertion in §10):** the
Phase 7 movement rungs must make **zero physical-placement assumptions**, must express
reparenting entirely as column/membership updates against the tier vocabulary, and must never
implement movement as row relocation. Movement settling *before* this vocabulary exists is the
hazard the split insertion removes.

**Rootness is a capability tier, not a type — and granting root ≠ session root.** Per-Tree
Instantiation already states it in CPU-struct form ("a subtree IS a root SimThing tree and gets
its own allocator, session state, shadow, and generation counter by instantiation");
`GameSession` already documents itself as "authority marker only; NOT a runtime engine
singleton." This phase makes it intrinsic: every StemThing carries, inert, the capacity to hold
an arena, run a census, and grant budgets. Activate it and any SimThing is a **relative
granting/execution root for its subtree** — and nothing more: it acquires **no
Scenario/GameSession authority**. The ontological session topology (core design §2) is untouched
by this phase, and an active granting node is never a GameSession-equivalent. A future coder who
reinterprets every granting node as a session authority has left the design. **A child root's arena IS its granted block** — nested
arenas are nested contiguous ranges; sandbox subtrees, vendored builds, and high-churn experiments
purged wholesale at horizon are all "a SimThing with its rootness lanes active." Micro-subtrees
share the device context (per-tree GPU contexts are disqualifying, per the automata doc §10);
the arena hierarchy is ranges within one physical arena.

**Layout enters as `residency_class` — the sibling of `resource_class` — and the lawful claim is
narrower than "layout is a residency class of the IR."** The field-sweep IR's proven universality
is **compute**: `adjacency × map × fold × post` executing over whatever layout the residency class
prescribes. So: **one admitted compute IR executes over class-specific layouts, and no layout may
acquire a bespoke semantic compute kernel** — `FIELD-SWEEP-SINGLE-PATH` must never fire, and the
5.7 safety template governs every layout move (*changes WHERE values load from, never the ORDER
they fold*, so `CanonicalOrderProof` and bit-exactness survive relocation, tiling, and packing).
**Placement mechanics — allocation, relocation, compaction, range partitioning — are not field
sweeps.** They are policy-free kernel physics behind the placement boundary (§1, §8). The IR
governs computation over geometry; the placement boundary governs the geometry itself.

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

**The grant materializes storage, never structure.** Structural coordinates and topology are
authored/admitted truth (spatial contract §3): a derivation draw authorizes **storage for** an
authored lattice — it does not author the lattice, and insufficient residency **defers or tiles
execution, never shrinks or reinterprets the authored layout**. (The earlier phrasing "the
lattice materializes as the grant" is withdrawn as drift-adjacent to the layout-vs-execution
law.) With that held, MapGen remains a derivation program in the storage sense: the root draws a
spatial-tier subtree, the grant materializes its block, and the STEAD field sweeps what admission
placed. This is Wei's postulates applied to memory itself:
locality (claims resolve against locally pre-granted budgets; the global pool is touched only at
barriers), symmetry (one authored grant rule at every arena, never per-site allocation logic), and
stability (exact conservation plus bounded budgets — growth is a threshold crossing, never a
surprise).

---

## 8. What dissolves, what remains — the fixed points

| Stratum | Disposition |
|---|---|
| Allocation/derivation/registration/enrollment **policy** | in-family: lanes, CostBands, authored rules, budgets |
| The allocator and the derivation framework **as services** | **dissolved** — no such subsystem exists after this phase |
| The registry **substrate** (property layouts, column ranges, threshold registrations, admitted EML programs) | **remains** as sealed admitted structural metadata at the boundary. The honest claim is that registration *policy* dissolves — no service, no policy authority — **not** that every registrar substrate disappears; the broader claim would require a mechanism this document does not yet design |
| The **executor** (buffers, dispatch, placement mechanics) | remains outside as physics: the kernel interprets, owns memory per the cross-crate seal law, and holds zero policy |
| Structural/type **admission** | remains at the boundary: the immune system stands outside the body it screens |
| The **base case** | remains the embedder handshake: physical VRAM capacity is a fact about the host, granted to the root at instantiation as **exact byte/page budgets per residency class** (the Vendor Door's Populate declares them as data; row counts derive only after the admitted layout is known); the recursion bottoms exactly one handshake outside the family, which is what makes the self-reference well-founded rather than circular |

External **code** persists; external **authority** does not. The kernel becomes to the family what
silicon is to a program. And the self-reference discipline is the same stratification as §3: the
root may describe and govern its subtree's memory; it may never relocate itself mid-flight — its
own placement belongs to its granter's barrier, which for the session root is session scope.

---

## 9. Binding laws and drift detectors

**Laws (numbered; every rung of the phase cites the ones it touches):**

1. **Lane, not leg.** No fifth capability on the base object; residency and derivation flow
   through the existing four.
2. **Slots are hard currency.** Discrete-exact transfers only;
   `free + in_flight + occupied = capacity` exactly, judged at every boundary; minted extents
   provably disjoint and bounded by the granter's extent (the placement oracle).
3. **Fungible scarcity by conservation; structural shape at the boundary.** A cap is a budget and
   no validation code owns what arithmetic already forbids — but a numeric appearance in a
   structural constraint does not make it a budget.
4. **Placement stratification.** Self-description always; self-relocation never; placement changes
   only at the granter's barrier, with a recorded remap.
5. **Grant determinism by recording.** Recorded grant schedule + authored priority; no
   iteration-order allocation anywhere.
6. **A tier is a price vector.** Domain-free, smallest-fit; open across authoring, **frozen at
   session admission**, closed engine vocabulary beneath; the engine never branches on tier.
7. **Rootness is a tier, not a type — and never session authority.** No service nodes; a granting
   root is relative to its subtree and is not a GameSession-equivalent.
8. **Census is perception.** Banded, stamped, aggregate, stale-visible; lanes only where granting
   is active; never a directory.
9. **One compute IR over class layouts; placement is physics.** No layout acquires a bespoke
   semantic compute kernel (WHERE never ORDER); allocation, relocation, compaction, and
   partitioning are not field sweeps.
10. **The base case is one handshake wide — denominated in bytes.** The embedder grants the root
    arena as exact byte/page budgets per residency class; row counts are derived only after the
    admitted layout is known. Nothing else external holds authority.
11. **One history surface.** Grant schedules and compaction remaps extend the canonical recorded
    schedule; a second history mechanism is forbidden.
12. **Quantity is not geometry.** Conserved capacity and minted extents are distinct objects with
    distinct judges; conflating them is the category error this document was remanded to remove.

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
10. Am I validating a *fungible* quantity a conservation law could make unreachable — or,
    conversely, converting a structural constraint into a "budget" because it contains a number?
11. Am I treating the CostBand remainder as fragmentation, or conserved quantity as geometry?
12. Am I minting an extent anywhere but the granter's kernel-owned placement boundary?
13. Am I recording grants or remaps anywhere but the canonical schedule surface?
14. Am I reinterpreting `SlotIndex` without the §3.1 Tier-2 ruling — or treating a granting node
    as a session authority?
15. Am I shrinking or reinterpreting an authored lattice because residency was insufficient,
    instead of deferring or tiling execution?

---

## 10. Phase disposition and sequencing constraints

- **Blocking pre-decision:** the §3.1 slot-identity ruling (Tier-2 core-design amendment with the
  slot-bearing-artifact enumeration) must land **before any row of this phase is minted**. It is
  the one place this anchor currently conflicts with standing law, and the conflict resolves by
  amendment, never by reinterpretation. **Owner ruling on census timing (2026-08-03): the census
  runs as a single pass after Phase 6 closes** — surfaces stop moving first; the Tier-2 ruling
  window is therefore the Phase 6 → 7 gap. Per Sol: the enumeration must be a **tree-derived
  exhaustive census**, not the illustrative §3.1 list. Two artifacts are pre-flagged as the likely
  hard cases: canonical fold order (must be provably pinned to *logical* identity, or rebinding
  breaks bit-exactness) and replay records (any physical-row keying forces the remap into the
  canonical history surface).
- **Shared window with the EML completion rungs (Owner-approved 2026-08-03):** rungs
  `5.10`–`5.12` (`PrimitiveDomain` door machinery, full-domain `EXP`, `LN` — see
  [`full_eml_unification.md`](full_eml_unification.md) §10) dispatch in the same Phase 6 → 7 gap. **SUPERSEDED (Owner sequencing ruling, 2026-08-04, stamped in
  board comment 5182422593): the streams are SERIAL, not parallel — 5.11 → 5.12 graduate FIRST; the
  Owner's pointer flip to 6.4 (StemThing-A dispatch) follows 5.12's graduation.** The earlier
  parallel-window language is withdrawn; the census/Tier-2 DA work completed inside the gap as
  planned, but StemThing-A *implementation* queues behind the EML completion rungs. The primitives are
  deliberately **not** StemThing components; the dependency runs the other way and later:
  **StemThing-B's derivation-pricing and capacity-depletion curves are horizon consumers of
  `EXP`/`LN`/`POW`**, authorable as ordinary gadgets when B's rungs land.
- **Integration sequencing (Owner ruling 2026-08-03 — Phase 6 lands as stated; the trigger chain
  is tree-state, never calendar):**
  1. Phase 6 completes under its existing rows (6.2, 6.2b, 6.3 + any unstamped ladder cells) —
     coder-lane work, no StemThing act occurs during it.
  2. **6.3's graduation stamp is the trigger.** The DA then runs the census (single pass, per the
     ruling above) and authors the §3.1 Tier-2 slot-identity amendment; the constitutional
     pointer-flip is an **Owner act**, as is the shape-(a)/(b) decision if the census forces it.
  3. The amendment's merge lifts the HARD HOLD; the DA mints the StemThing-A rows into §3b in the
     same window, before any Phase 7 row is dispatched.
  4. StemThing-A implements; Phase 7 movement lands **under the §7.1 clause**; 8.1/8.2 land;
     StemThing-B rows are minted and implemented; Phase 11's door follows.
- **StemThing-B forward-bind (v4 unification sweep, 2026-08-13, #1743):** B's partial
  reconciliation consumes the **7.8a `DERIVED-SPAN-PROJECTION-INVALIDATION-0` span/invalidation
  substrate** — never any consumer's representation. The B-mint session inherits this bind.
- **Split insertion (Owner ruling 2026-08-03, placement inside 0.0.8.7):**
  **StemThing-A** lands in the **Phase 6 → 7 gap** — the census, the §3.1 Tier-2 ruling, the
  residency/tier vocabulary, and the §7.1 movement clause — so allocation/tiling awareness exists
  **before the movement regime settles**. **StemThing-B** — the granting arena, authored-priority
  retirement of LIFO, allocator-service deletion, placement oracle — lands **after 8.2**, keeping
  its true dependencies. The must-precede-11.1 constraint is unchanged and satisfied by B.
- **Consumes (must be landed):** 6.0 owner channels; 6.1 stamps; 6.1b CostBand; 6.2 coalescing
  queue; 8.1 conservation judge; 8.2 contention arena (slot contention resolves there — the
  arena's authored-rule law is what retires LIFO). The placement oracle (extent disjointness) is
  new referee work this phase owes alongside the 8.1 judge.
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

## 12. StemThing-B — the recursive conserved-resource market germ (Owner design session 2026-08-24, RATIFIED)

> StemThing-B is not "malloc generalized." It is the recursive conserved-resource market germ; VRAM Residency is the distinct engine-native market that first proves it.

> domain management → authored conserved-resource markets → RF + CostBand + Field Triad dynamics → replayable ML corpus

**The market grammar** (one chain, already-intrinsic surfaces only): admitted conserved resource/capacity → descendant claims/Draws → recursive RF reduce-up → EML valuation / effective clearing weight → authored generic constrained clearing → CostBand quantization → grant/flow disbursement → Gu-Yang conserved throughput/saturation → PALMA potential/impedance/opportunity over admitted topology → STEAD anchored pressure/observation/bands → ordinary ActionBand/OverlayThing response → next-generation state.

Every SimThing carries this germ; SessionThing / NetworkManagementThing / TeamManagerThing / ComputeNodeThing and every other domain-management derivative instantiate their own conserved-resource market over descendants **as authored data** — never as manager subsystems, never as additional core facilities. VRAM Residency remains a **distinct engine-native facility** (extents, placement oracle, disjointness, relocation — kernel physics); its entitlement side consumes the germ. Triad observations are ordinary numerical outputs, not semantic services; a degenerate domain need not invent topology.

**Ratified laws (Owner, 2026-08-24):**

- **Clearing weight** is ordinary authored clearing input, never implicitly derived from tier unit cost (authors may derive one from the other through EML; the engine never equates them). It is an ordinary effective SimThing value: tier-authored default, sparse inherited overrides (ancestor projects over subtree), dynamically deformable through admitted EML/OverlayThing. Claims consume the effective weight and carry no policy vocabulary.
- **Ties and remainder:** equal clearing scores clear proportionally unless the author supplied an additional deterministic ranking. Discrete remainder is largest-remainder and work-conserving; exact fractional ties rotate deterministically over canonical logical identity under the owning granter StemThing generation authority — no permanently privileged ID order. Residual unmet demand is ordinary `U` (never CostBand `R`), re-valued next generation; **no residency-specific retry loop; no same-generation retry.**
- **Draws:** a Draw is the specialization-profile-level claim template — admitted offering(s), lifecycle trigger(s), and a bounded quantity envelope from which runtime claims may be emitted. Draw vocabulary seals at session admission with the profile; instances author no new Draws; EML/OverlayThing/ActionBand state determines actual claim quantity within the envelope. **A Draw grants nothing by itself** — it only authorizes an ordinary RF claim, which may clear or remain `U`. Strict offering reference: no per-type parameter deltas (a delta is a shadow tier). No new vocabulary object: the existing specialization profile gains offering references and a Draw template.
- **Oversubscription is ordinary.** Admission certifies model validity (envelopes finite and well-formed, offerings exist, triggers admitted, root arena declared) and never requires sum-of-envelopes ≤ capacity.
- **Placement (residency), two-stage fail-closed:** a cleared entitlement is provisional until the owning boundary placement oracle proves a legal, disjoint, in-bounds realization. Infeasibility is an ordinary typed placement refusal — nothing commits, the quantity stays `U`, the refusal rides the one recorded schedule. Overlap or out-of-bounds observed in already-committed authoritative placement state is an invariant breach and hard-faults the session. *Ordinary infeasibility must not crash the sim; committed corruption must not be disguised as `U`.* Unchanged placements need no global per-generation re-proof.
- **Grant lifecycle:** detachment does **not** inherently release a grant — a detached/spooled child (independently executing subtree) remains provisioned, observed, revalued, communicated-with, revocable and renewable by its ancestor through the existing stamped seam. Death/dissolution normally releases; fission/fusion partition or transfer exactly; **grant termination is an explicit lifecycle fact, never implied by topology.**
- **Confinement (Owner R3):** no allocator path exists that is not a SimThing disbursing resources through the RF arena; scope is slots/extents and authored capacity lanes — registry column append and GPU buffer sizing remain infrastructure behind the placement boundary.

**Implementation fences (Owner, enactment approval 2026-08-24):**

- **F1:** the non-residency witness must exercise the FULL RF + CostBand + Triad + replay market grammar — not merely a second allocation lane.
- **F2:** ML-corpus production must arise from existing authoritative observation/replay surfaces and may not create a second telemetry/history authority. Corpus-format/export optimization is a dated horizon deferral (2026-08-24), not silent scope.

**Load-bearing witness:** a granter provisions a child; the child becomes an independently executing subtree; the same market relationship remains conserved and observable across the stamped seam with no global manager — defined at the landed level (detached subtree in-session, identity-keyed grant records, stamped seam); multi-session spooling is a SessionThing-derivative horizon (dated 2026-08-24).

**Rung set (minted into the 0.0.8.7 ladder §3b):** 11.2a `STEMTHING-B-FLOW-MARKET-GERM-0`; 11.2b `STEMTHING-B-VRAM-RESIDENCY-0`; 11.2c `STEMTHING-B-GROWTH-ENTITLEMENT-SEAM-0`; 11.2d `STEMTHING-B-ALLOCATOR-RETIREMENT-0`; 11.2e `VENDOR-DOOR-GRANTING-SURFACE-0` (facade-only, no sixth verb, recursive-granting witness, STOP-and-reopen fence). The core-design §4 residency text is amended in the same delta: entitlement is market-decided; free-lists are downstream physical machinery.

## References

- [`simthing_core_design.md`](simthing_core_design.md) — §1.2 admission ladder; §2.1 fission and
  velocity-driven pre-allocation; §3 registry discipline; §5 arenas and hard-currency law; §7
  Movement-Front; §9 drift detectors.
- [`stead_spatial_contract.md`](stead_spatial_contract.md) — §5 CostBand definition; layout vs
  execution-profile admission.
- [`eml_n4_expansion_digest.md`](eml_n4_expansion_digest.md) — field-sweep IR provenance;
  `FIELD-SWEEP-SINGLE-PATH`; the WHERE-not-ORDER safety template; preservation invariants.
- `design_0_0_8_7_rf_arena_modernization.md` — 5.4–5.8, 6.0–6.3, 8.1–8.2, 11.1–11.2 rows; the
  governing laws (§4) this document extends to memory and derivation.

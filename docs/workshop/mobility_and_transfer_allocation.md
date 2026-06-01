# Mobility and Transfer Allocation — Workshop Findings
## Spatial Mobility, Reparenting Re-Enrollment, and Slot Allocation at Scale

**Date:** 2026-05-31
**Source:** Opus 4.8 design authority session — architectural analysis of the next
named-scenario territory
**Status:** Workshop findings — not an ADR, not an implementation gate, not an
authorization. No production code from this document.
**Companion docs:**
- `docs/reviews/e11b_nested_hierarchy_gpu_readiness_review.md` — E-11B GPU readiness
- `docs/reviews/e11b_nested_dynamic_enrollment_readiness.md` — E-11B-5 deferral rationale
- `docs/adr/mapping_sparse_regioncell.md` — Mapping ADR
- `docs/design_v7_8_production_track.md` — v7.8 M/E/T lines current state
- `docs/design_v7_8.md` — constitution and operating doctrine

---

## 1. Current substrate state (as of 2026-05-31)

All three v7.8 promoted M/E/T implementation lines are closed for their current named
scenarios:

| Line | Status |
|---|---|
| **A / E-11B** | A-0 ACCEPTED — static nested D=3/D=4 materialization, per-parent contiguous SlotRange, reserved-gap exclusion, D=3/D=4 bit-exact GPU/CPU parity. FlatStar remains bounded production posture. |
| **B / D-2a** | B-0 ACCEPTED — authored `order_band` → existing AccumulatorOp `GateSpec::OrderBand(n)`; deterministic boundary schedule; exact CPU-oracle parity. Line B/T CLOSED at narrow smoke level. |
| **C / M-4** | C-0/C-1/C-2 ACCEPTED — packed-atlas GPU path + 2000-star scale model + bounded algebraic-G=0 designer admission. Map batching CLOSED at designer surface. Atlas production runtime / sparse-residency scheduler is a separate later gate. |

**Parked behind separate named scenarios (not opened):**
E-11B-5 (nested dynamic enrollment), atlas production runtime, mixed-kind multi-band
ordering, ClauseThing/L3, FrontierV2-5, ACT/EVENT/OBS/PIPE.

The static nested layout (A-0) proves the GPU path. Dynamic enrollment — entities joining or
leaving arenas after session open — is the frontier. Spatial mobility is the concrete
product scenario that makes dynamic enrollment concrete.

---

## 2. What this document is about

The closed A-0 slice handles **born-into-place** nested participants: the empire → sector →
cell → fleet topology materializes at session open; fission (E-2B-5) handles the birth of
new sub-entities under existing parents. Neither handles an entity **moving** from one parent
to another mid-session.

This is the spatial mobility gap. It is the architectural territory that any "fleet movement"
or "empire dynamics" named scenario must navigate. This document maps the terrain: the
underlying tensions, the constitutional framing, the concrete mechanism gaps, and the open
design decisions.

---

## 3. Constitutional framing: two orthogonal purposes of the SimThing tree

The SimThing constitution already states:

```
The spatial tree is the physical map.        (unchanged)
The political map is overlays on that tree.  (unchanged)
```

The SimThing tree serves **two orthogonal purposes simultaneously**:

| Purpose | What changes | What stays stable |
|---|---|---|
| **Spatial containment** | Fleet moves: parent changes | Slot identity, political columns |
| **Political ownership** | Planet changes faction: `faction_id` column changes | Spatial parent (still under its star) |

**The critical distinction:** political changes are **column updates**, not reparenting events.
An empire collapsing, a planet changing hands, a faction restructuring — these are overlay
updates on a stable spatial tree. **They do not trigger reparenting.** Only actual spatial
movement (a fleet crossing into a new cell) is a reparenting event.

This distinction is architecturally load-bearing. If political changes were modeled as
reparenting, the reparenting-triggered re-enrollment mechanism would be flooded with false
positives. The substrate supports the orthogonal model; the gap is not in the model — it is
in the **wiring**: the reparenting boundary mutation currently has no hook into arena
enrollment.

### 3.1 The canonical spatial operations

| Operation | Substrate action | Arena enrollment effect |
|---|---|---|
| Fleet moves cell | Reparenting: spatial parent changes | Origin cell arena: deregister fleet children. Destination cell arena: register fleet children. |
| Planet changes ownership | Column update: `faction_id` on the planet SimThing | None — spatial parent unchanged, no reparenting. |
| Empire collapses | Overlay restructure: political hierarchy columns | None — spatial tree untouched. |
| Fleet spawns ship class | Fission: new child born under fleet parent | E-2B-5 / E-11B-5 path (already designed). |

### 3.2 Combat as a cell-level arena (constitutional)

Two fleets under the same grid cell are **siblings**. Their shared parent — the cell — mediates
all interaction. Combat is a resource arena owned by the cell with fleets as participants.
Damage flows are `SubtractFromSource` transfers; HP recovery is `governed_by` integration;
a ship class crossing zero HP fires `Threshold + EmitEvent` → boundary removal.

This is constitutionally clean: siblings interact only through parents. The cell is the
allocator; fleets are the leaves. Cell arenas are **flat-star within the cell** — the nesting
(empire → sector → cell) lives **above** the cell, not inside it.

**Implication:** the spatial mobility mechanism operates on flat-star arenas, not nested ones.
E-11B-5 (nested dynamic enrollment) is the wrong frame for fleet movement.

---

## 4. The primary gap: reparenting-triggered arena re-enrollment

### 4.1 What is currently wired

The reparenting boundary mutation moves a node in the SimThing tree. It does **not** update
arena participation tables. The GPU allocation plan for the origin cell and the destination
cell are built at session open (or updated via fission) and are **not rebuilt on reparenting**.

### 4.2 What is missing

When Fleet X reparents from Cell A to Cell B:

1. Fleet X's children must be **deregistered** from Cell A's combat arena.
2. Fleet X's children must be **registered** in Cell B's combat arena.
3. Both Cell A and Cell B arena allocation plans must be **rebuilt**.
4. The entire operation must be **atomic at boundary time** with GPU/CPU oracle parity.

None of this is currently wired. This is a new class of boundary operation: a
**reparenting-triggered arena re-enrollment**, distinct from fission (birth), expiry
(death), and static enrollment (session open).

### 4.3 Why E-11B-5 is the wrong frame

E-11B-5 as scoped addresses **nested interior allocator invalidation on dynamic enrollment**
— specifically, a SimThing being born mid-session and joining its parent's nested Resource
Flow arena. This is a fission-triggered operation.

Fleet movement is structurally different:
- The entity **already exists** with a stable slot identity.
- It is **not being born** — it is being **relocated**.
- The **arena it is leaving** must deregister it, not just ignore it.
- The **arena it is joining** must handle a non-contiguous arriving slot.
- Cell arenas are **flat-star**, not nested — E-11B-5's nested interior machinery is not
  needed.

The correct frame is **reparenting as a first-class arena operation** — an operation that is
as fundamental as fission and expiry, with its own preflight/commit protocol, its own
atomicity discipline, and its own slot accounting.

---

## 5. The five architectural tensions — resolution map

The mobility/ownership work surfaced five tensions. All five are now resolved; this section is the
map from each to its resolution. (Detailed mechanisms in the linked sections.)

| Tension | What it was | Resolved by |
|---|---|---|
| **A — static enrollment vs. mobility** | Session-open rosters assume stable topology; mobility needs *bilateral* re-enrollment (leave one arena, join another). | REENROLL bilateral prepare/commit (E-2B-5 extended) for the protocol; bulk two-stage accounting absorbs the move burst (§12.3). |
| **B — SlotRange contiguity vs. non-contiguous arrivals** | The reduction needs contiguous children; an arriving entity's slot is non-contiguous by construction; compaction is banned. | Slab/block model: a parent owns a pre-formatted contiguous block; arrivals claim slices in reserved headroom; whole-block reclaim, no compaction (§12.2). |
| **C — identity boundary (named vs. fungible)** | One slot per named SimThing is overcomplete for fungible mass units. | The **SimThing is the named unit; fungible members are a `count` column on it** (a cohort is a SimThing, individuals are a scalar within). Homogeneity kept by fission-on-partial-change (§11.11). |
| **D — reactive vs. predictive allocation** | Reactive LIFO degrades under growth bursts. | Bulk two-stage accounting absorbs bursts *reactively* (§12.3); prediction demoted to optional smoothing (§7.3) — not load-bearing. |
| **E — static arena sizing vs. dynamic reality** | `max_participants` is a static guess. | Two-sided: block-size knob on the supply side (§12.2) + exact bulk accounting of actual demand (§12.3). |

---

## 6. Slot contiguity — the resolved approach

Contiguity is handled by the **slab/block model** (§12.2): a parent owns a pre-formatted contiguous
block sized by a declared `max_fleet_density`; an arriving entity claims the next free **slice** in
that reserved headroom; reclamation is whole-block at a boundary event — **no compaction, no
indirection list, no mid-session reordering** (the v7.7/E-11B-4 invariants hold).

**Fungible members are a `count` column on their named SimThing, not separate slots** — a ship class
carries `fighter_count`/`fighter_hp_pool`; a pop cohort carries a population count (§11.11). The
identity boundary is the SimThing; individuals below it are a scalar. The indirection-buffer option
is **not needed and not pursued** (it would require a new GPU primitive; the slab + Hybrid Strata
cover every case).

---

## 7. Slot allocation at scale: per-parent free lists and predictive expansion

### 7.1 Current LIFO model and its limits

Current tombstone policy: dead cohort's slot pushed onto a global LIFO stack; new cohorts
claim LIFO. Adequate for steady-state churn. Breaks in two regimes:

- **Net-positive burst** (productive empire phase): tombstone stack empty; every new cohort
  needs a fresh slot; headroom exhaustion triggers `rebuild_for_slots` on every boundary.
- **Mass-collapse** (endgame empire destruction): tombstone flood of potentially hundreds
  of cohorts under many parents simultaneously; slot fragmentation across parent-kind
  boundaries.

### 7.2 Per-parent free lists (generational tombstoning)

**Finding:** Per-parent free lists are consistent with the SimThing model and should be the
default tombstone policy for the next allocation iteration.

When a cohort dies, its slot returns to its **parent's free list**, not a global stack.
New cohorts under that parent draw from the parent's free list first. This keeps cohort
slots spatially clustered under their parent — beneficial for reduction topology (children
tend to be contiguous), dirty-skip effectiveness, and active mask efficiency.

**Parent removal compaction:** when a parent SimThing is removed, its entire slot block is
returned to a **global kind-pool** that assigns blocks to new parents of the same kind. This
is safe because parent removal is a boundary event and all slot references under that parent
are already being cleaned. It does not require mid-session slot compaction — only a block
return at boundary time.

**Overflow:** parents that exceed their initial block size (more fission children than the
initial gap reservation supports) spill to the global kind-pool for extension. The policy
for this extension is a design decision (see D4).

This is a **self-contained change to the slot allocator** with no dependency on reparenting
re-enrollment or predictive expansion. It can land as an independent improvement.

### 7.3 Predictive expansion from the state field

**Finding:** The state field already contains the information needed for exact predictive
slot pre-allocation. This is more powerful than fixed headroom.

| State column | Allocation signal |
|---|---|
| `Velocity` | Current growth/decay rate → integrate forward by horizon to get expected cohort count at tick T |
| `Balance` | Net-positive or net-negative arena → aggregate demand signal |
| Pressure columns | Demand aggregated across children → parent-level slot pressure |
| Urgency columns | Pre-collapse signal → begin releasing that empire's slot reservations proactively |

**Wire:** The driver already reads these columns at every boundary (threshold detection,
summary readback, B-4 infrastructure). The allocator becomes a **new consumer of the same
readback** — subscribing to velocity and pressure summaries the same way the boundary
protocol subscribes to threshold events.

**Cascade prediction:** an empire whose balance columns are deeply negative, urgency columns
maxed, and resource flow running a large deficit is signaling collapse in the state field
ticks before threshold crossings fire. The allocator can observe this signal and begin
releasing that empire's slot reservations proactively — converting a reactive mass-tombstone
flood into a smooth, predicted release.

**Scope:** Driver-internal first (no spec surface). Spec-declared allocation profiles (a
designer-facing capability that declares the projection horizon and which columns to observe)
are a later, named-scenario gate.

**Reclassification (see §12):** the literature review demotes this finding. A bulk two-stage
accounting allocator (NVIDIA 2019) absorbs the burst *reactively* in one boundary pass, so
prediction is no longer load-bearing for feasibility. Predictive expansion survives only as an
*optional smoothing refinement* (pre-reserve a block before a known burst; proactively release
blocks on a predicted-collapse signal), reading **aggregate** demand signals only — never
per-entity. Build the bulk reactive allocator first; treat prediction as a later nicety.

---

## 8. Design decisions — status

| # | Decision | Status |
|---|---|---|
| **D1 — Reparenting trigger scope** | How the boundary protocol knows a mutation needs arena re-enrollment. | **Spec-layer rule:** automatic when an enrollment selector reads `parent_id`/`spatial_parent`; declared opt-out for purely column-driven arenas. A spec-admission decision, not a runtime heuristic. (Only owner-relation that *aggregates* — faction — re-enrolls; overlay relations are column writes, §11.11.) |
| **D2 — Arriving-entity contiguity** | How a moving entity lands contiguously. | **Resolved:** slab/block model — claim a slice in the parent's reserved block; no compaction, no indirection buffer (§6, §12.2). |
| **D3 — Predictive allocator** | Whether/how to pre-allocate from state-field signals. | **Parked (demoted):** bulk two-stage accounting absorbs bursts reactively (§12.3); prediction is optional smoothing only — do not sequence it ahead of the reactive allocator. |
| **D4 — Per-parent free lists** | Default slot-reuse policy. | **Resolved:** per-parent generational free lists + whole-block kind-pool reclaim on parent removal (§12.2; ALLOC track). Self-contained; lands early. |
| **D5 — Identity boundary** | Where named identity ends and fungible-aggregate begins. | **Resolved:** the SimThing is the named unit; fungible members are a `count` column on it (cohort = SimThing, §11.11). |

---

## 9. Recommended implementation sequence

**Not yet authorized. Requires a named scenario, D1-D5 resolved, and a separate Opus
implementation gate.** This sequence assumes a scenario with a bounded theater, declared
`max_fleet_density`, ship-class cohorts as named entities, and fighters as aggregate columns.

| Step | Scope | Dependencies | Notes |
|---|---|---|---|
| 1 | **Per-parent free lists + kind-pool compaction** (D4) | None — self-contained | Improves all high-churn scenarios; no reparenting or arena changes. |
| 2 | **Predictive expansion wire** (D3) | B-4 summary readback (already landed) | Driver reads velocity/pressure columns; projects forward; pre-allocates ahead of burst. No GPU changes. |
| 3 | **Reparenting-triggered arena re-enrollment** (D1 + D2 with gap reservation) | D5 identity boundary declared; D4 landed | Extend reparenting boundary mutation to call E-2B-5 prepare/commit enrollment protocol for origin and destination arenas. Flat-star cell arenas only; nested arena reparenting is a later gate. Gap reservation sized to `max_fleet_density` declared in spec. |
| 4 | **First scenario slice** | D1/D2/D4/D5 done | Single cell arena, two fleets, reparenting across cells. Exercises the full reparenting re-enrollment path in the simplest case. No atlas, no active mask, no indirection buffer. |
| 5 | **Indirection buffer** (if needed) | Gap reservation VRAM cost unacceptable at target scale | New GPU primitive. Requires sandbox probe before admission. |

**Architectural note on step 3:** The E-2B-5 prepare/commit protocol is the template. The
extension is: `prepare_reparenting_enrollment(entity, origin_arena, dest_arena)` verifies
origin deregistration and destination slot assignment (gap reservation or contiguous
extension); `commit_reparenting_enrollment` executes both atomically with generation bumps
on both arena registries. Both arena plans rebuild in the same boundary pass.

---

## 10. Open questions for the scenario design pass

These are **scenario/product parameters** (not substrate questions — those are settled in §11–§12).
They set tunables and bounds for a named scenario:

1. **Max fleet density / `max_factions_per_cell`** — sizes the block reservation and the leaf channel
   count `c` (§11.10).
2. **Sector supply vs. cell combat** — whether the sector→cell edge is a Resource-Flow coupling or
   just tree structure (i.e. where the masked flow balances under subsidiarity, §11.4).
3. **Endgame entity/cohort count** — sizes the kind-pool and the slab high-water budget.
4. **Doctrine uniformity** — whether routing/targeting weights are uniform across owners or
   per-owner (a weight-column authoring choice).

---

## 11. The identity/ownership overlay — directing flows by property, not by structure

*Scope: §11.1–§11.3 the local routing case (combat, D=2 masked); §11.4–§11.14 the global ownership
architecture (session clearinghouse, owner-relations, subsidiarity economy, latched modifier
overlays, Hybrid Strata, pop cohorts, the fixed-point/conservation-band keystone, selective routing,
and faction-index stability). One mechanism throughout: identity is a property read by a masked
reduction, never tree structure.*

This section describes how **identity/ownership** (faction, role, allegiance) directs the universal
up-sweep/disburse flow dynamic according to each SimThing's *properties and roles* — generically and
semantic-free. "Combat" is one designer-authored expression of this overlay; "supply," "tribute,"
and "taxation" are others. The substrate knows none of those words. **Identity is a property/column
read by a masked reduction — never tree structure.**

### 11.1 The masked reduction (D=2)

The faction split is a **masked reduction**, not a sub-tree: the substrate reads a property *inside*
the reduction rather than grouping by it. The cell is an **allocator SimThing with an identity-routing
overlay**; fleets are flat siblings (D=2) each carrying an `identity` column. Per tick the overlay
does:

1. **Masked gather** (new composition, landed primitives only): for each identity `F` present,
   compute per child `masked_fp = firepower · (identity == F)` via an `EvalEML` `select`/`CMP_EQ`
   mask, then a contiguous `SlotRange Sum` of that into a per-identity column on the cell. "Sum the
   enemies" expressed as *mask-then-sum*, not *group-then-sum*. Cost ≈ **two extra bands total,
   independent of `k`** (one band writes `k` masked scratch columns per child; one band runs the `k`
   Sums into `k` cell columns).
2. **Directed disburse** (existing disburse band, new routing EML): each fleet computes
   `incoming = Σ_F hostile(self.identity, F) · faction_fp[F] · targeting_weight`, an EML reading the
   child's own `identity` and the cell's per-identity columns. For `k=2`: `select(self==A, fp_B,
   fp_A)·w` — a handful of nodes.
3. **Integration (unchanged):** apply `incoming` to the fleet HP column with `IntegrateWithClamp`
   clamped at 0; a zero-crossing fires `Threshold + EmitEvent` → boundary removal (landed E-7R +
   C-1). *(Damage = a `SubtractFromSource`/clamped-integration expression a designer authors; the
   substrate sees only a flow and a threshold.)*

No new WGSL, no new `AccumulatorRole`, no new `CombineFn` — `EvalEML` (select mask), `Sum`, and
`ScaleSpec::ByColumn` already exist. Determinism holds: masked Sum is a fixed-order contiguous Sum
of masked values → bit-exact. "Siblings interact only through the parent" is preserved — the cell
mediates entirely (masked sums + directed disburse); fleets never touch each other directly.

### 11.2 Properties of the masked-reduction model

Because identity is a property, not structure:

- **Identity change is a column write.** Capture/defection flips the `identity` column; next tick the
  masked reduction counts the SimThing under its new identity. No slot move, no re-enrollment, no
  re-projection — consistent with §3.2 ("the political map is overlays on the spatial tree").
- **No per-identity contiguity.** One flat child block per cell (the normal A-0 append), not `k`
  blocks — the single-key cell block that A-0 + REENROLL already handle. The identity dimension adds
  zero new contiguity burden.
- **The cell allocator is always-on.** Combat is simply what the masked sums compute when a hostile
  child is present; a hostile arrival is an ordinary child append and the overlay sees its `identity`
  immediately. There is no separate arena to instantiate or tear down — the conflict lifecycle *is*
  the cell's child-membership lifecycle (fission / arrival / expiry), which is landed.

### 11.3 Scale: `k` bounded by the routing formula, not by grouping

"Route to enemies by identity" is conceptually a dense `D = H·F` over an N×N diplomacy matrix, but
in practice the masked gather and directed disburse operate over **only the identities present in
the cell** (`k`, typically 2, rarely >4). The global N×N matrix never reaches the GPU. The bound is
now clean and designer-facing: **`max_factions_per_cell` is "how many hostility terms fit in the
directed-disburse EML"** (≤16 nodes, no transcendentals) — an *EML node budget*, not a slab size.
Over-cap cells reject visibly at admission.

### 11.4 Local vs. global routing: where a masked flow balances

§11.1–§11.3 cover **local** routing (combat): all participants are contiguous siblings under one
cell, so a masked reduction reaches them in place at D=2. But the *same overlay mechanism* must also
serve **global** relations — an empire's economy, a species' bonuses — whose members are scattered
galaxy-wide and are **not** contiguous under any single parent. The masked-in-place trick does not
reach them. The governing question is:

> **Does the identity relation align with the spatial reduction topology?**
> **Aligned** (co-located: combat) → mask in place, D=2. **Misaligned** (scattered: economy) → the
> masked reduction must *climb* the spatial spine to where the relation's members finally meet.

An **"arena" is just the subtree over which a masked flow balances; its root is wherever supply meets
demand.** Combat balances at the cell; a self-sufficient system balances at the system; a fully
global economy balances at the session root. One mechanism, different termination depth.

### 11.5 The session clearinghouse topology — owner-entities as session-descendants

Identity must never be a **spatial-containment parent** — if `empire → planet` were a tree edge, the
planet would have no spatial position and **capture would become spatial reparenting** (a structural
mutation for an event where nothing moved). So the spatial tree (the **worldstatemap**) carries no
empire node. Instead:

```
GameSession (root SimThing — the political↔spatial clearing hub)
├── Faction A           owner-entity: capability trees, policies, stockpile, effective overlay set
├── Faction B
├── …
├── SpeciesRegistry     grouping node (peer of the factions)
│   ├── Species X        owner-entity (a designed species instance)
│   └── Species Y
└── WorldStateMap       pure spatial containment
    └── sector → system → cell → holding / pop-cohort  (carry owner-columns)
```

An **owner-entity** (a faction, a species) is a **session-descendant** holding per-owner state; it is
**never** a spatial parent of the things it owns. Its only path to its scattered members is **up to
the session and down the worldstatemap spine, masked by the owner-column** — which is exactly the
"climb to where the relation balances" of §11.4, with the session as the universal bridge where
per-owner state meets the spatial tree. **Capture is a column flip** on a holding that stays exactly
where it is in the map; **no reparenting, ever.**

### 11.6 Owner-relations generalized — a SimThing has one parent + N owner-columns

"Belonging to multiple factions" is really **multiple owner-relations**. A SimThing has **one spatial
parent** (its cell) plus **N owner-columns** (`faction_owner`, `species_owner`, …), each referencing
an owner-entity.

- **Subscription is automatic from column presence.** A fleet carries `faction_owner` only → the
  species mask never matches it; it gets no species overlays *with no special wiring*. A population
  cohort carries both → it gets both. Nothing declares "fleets ignore species"; they simply lack the
  column the species mask keys on.
- **Species ≡ faction, structurally.** A species blueprint instance is an owner-entity dispersing a
  masked overlay down the spine — identical mechanism to a faction, differing only in the owner-column
  and which kinds subscribe. The `SpeciesRegistry` is a grouping node, a **peer of the factions**, not
  a new system. Do not build a "species system."
- **Capability trees, abstract.** An owner-entity hosts a set of **capability trees**; the substrate
  sees only: *a capability tree resolves (via the up-flow + unlock thresholds) into ① modifier
  overlays and/or ② instantiation gates (a bitmask of which blueprints the owner may instantiate).*
  "Ship-design," "buildings/districts," "tech," and "species-design" are **instances** of this one
  abstraction — keep them abstract; their contents are ClauseThing semantics.
- **Instantiation = gated fission.** Building a unit/structure, or instantiating a designed species,
  is **fission of a blueprint gated by the capability bitmask**, stamping the new SimThing's
  owner-columns. A designed species fissions under the `SpeciesRegistry` and *thereby becomes a new
  owner-entity* — an owner's capability tree gating the creation of new owner-entities. All landed
  primitives (fission + gates).

### 11.7 Two disciplines on the spine: blockable resources vs. blockade-immune modifiers

The session-hub disperses two kinds of per-owner state down the worldstatemap spine, masked by
owner-column, with **different temporal disciplines that must not be conflated**:

| | **Resource flow** (economy) | **Modifier overlay** (tech / policy / species) |
|---|---|---|
| Cadence | **per-tick** circulation | **latched** — pushed on change (`DirtyOnly`), then persists |
| Subsidiarity | yes — balances locally (§11.4) | no — applies everywhere unconditionally |
| Blockade | **cuts the flow** (the heatmap signal) | **immune** — knowledge is not goods |
| Lands in | stockpile / consumption | the SimThing's **overlay filter** |

**Subsidiarity economy.** Resolve supply/demand at the **lowest spatial node where they balance**;
escalate only the residual. A self-sufficient system never escalates; a deficit system pulls from its
sector; a deficit sector pulls from the owner's global pool (the session). **Distance = tree-distance
to the balancing node; a blockade = a cut tree edge** (the deficit below it can no longer be served
from above). The session-hub is the degenerate case where nothing balances until the root.

**The decision heatmap is a pure flow signal.** A blockaded holding produces at its *full modified*
rate but its surplus cannot export and its needs cannot import. The **potential-vs-realized gap is a
flow shortfall** (stuck surplus here / unmet deficit there) — a scalar field over the map whose
gradient is the actionable decision frontier ("where my full-tech output is throttled by a cut
edge"). This feeds the landed **M5 gradient / scarcity-opportunity** sink. **Modifiers scale the
magnitude of what is stuck; they do not create or attenuate the signal.**

**Modifiers are blockade-immune (resolve → disperse → apply).** A tech/policy/species bonus is
*knowledge*, not goods — a besieged holding still benefits from it.
- **Resolve** (at the owner-entity): compose capability trees + policies + traits into an effective
  overlay set (+ capability bitmask) via the **C-4 OrderBand overlay compiler** (Add/Multiply/Set with
  designer-set priorities). Unconditional modifiers fold into a compact effective vector; **conditional
  modifiers** (e.g. species/terrain-gated) disperse *with a predicate*, applied per-holding via a
  masked overlay (apply-if the predicate matches).
- **Disperse**: owner-entity → session → down the spine, masked by owner-column, `DirtyOnly`
  (pushed only on change, then latched). Not a per-tick cost.
- **Apply**: lands in the SimThing's **overlay filter**, where it **stacks** (C-4) with intrinsic
  overlays; capability unlocks land as **gate flags**.

**The overlay filter is layered per owner-relation, each layer refreshed independently.** A pop
cohort owned by faction-A and species-X has a faction layer and a species layer. **Capture (faction
flip A→B) refreshes the faction layer only; the species layer persists** — a conquered population
keeps its species under new management. A merged filter would wrongly drop the species bonuses on
capture, so the layering is load-bearing.

### 11.8 Frontier V1 / A-0 re-rooting (fix, not break)

The principle does not break Frontier V1 — it **re-roots** it. **Frontier V1 is the `k=1` degenerate
case:** the session holds one faction-entity + the map; the "flat-star economy" is session →
faction-entity, fed by the map. Behavior is unchanged; the faction is re-described as a session-child
*entity*, not a spatial parent. **A-0's nested D=4 hierarchy is the worldstatemap spine** (`sector →
system → cell` was always legitimately spatial); the only correction is that the top edge is
`session → worldStateMap`, with the faction sort at the hub and factions/species as session-children.
The accepted A-0 reduction machinery is reused verbatim — it was only ever *mis-labeled* at the top.

### 11.9 Classification, admission bounds, and the substrate/semantics line

- **Conservation class is a property of the authored quantity, not the overlay.** Soft rates (damage
  → HP, clamp-at-0) are Resource-Flow (`GpuVerifiedApproximate`); exact discrete quantities (spent
  munitions, hard currency) are Line B (`SubtractFromSource`, bit-exact). **The overlay must not
  silently mix classes in one arena/tick** (a real substrate guardrail).
- **Admission bounds:** `max_factions_per_cell` = directed-disburse EML node budget (§11.3);
  deterministic fixed op-order for any multi-term Sum → bit-exact; deterministic tie-break
  (`authoring_id`) if selective routing is admitted; per-owner-relation overlay layering with
  independent `DirtyOnly` refresh keys.
- **Open substrate question (one remaining):** recursive composition and band ordering across depths
  — the **OrderBand depth budget** when several circulations (Band Alpha hard, Band Beta soft,
  modifier-down, economy-up, research-up, threshold) interleave on one spine. This needs the
  `owner_band_budget_audit` against `max_orderband_depth` at the target spatial depth. *(Routing
  expressiveness → §11.13; conservation-class mixing → §11.12; per-owner aggregation scale → §11.10.)*
- **Substrate vs. semantics line.** *Substrate:* owner-relations as columns; owner-entities as
  session-descendants; masked overlay dispersal; capability-tree resolution → overlays + instantiation
  gates; gated fission; the per-relation-layered, independently-refreshed overlay filter; subsidiarity
  balancing; C-4 composition. *Designer/ClauseThing:* what a "flow" means (damage, food, money,
  morale), tree contents/prereqs/costs, what's buildable, species traits, composition priorities, and
  every combat-fidelity concern (armor, ammo-as-model, initiative, focus-fire, mitigation,
  damage-types, overkill, heal-vs-damage ordering). **Do not re-derive any of the latter as substrate
  work.**

### 11.10 Hybrid Strata — local anonymous channels → dense root (resolves the scale ceiling)

Aggregating a per-owner quantity up the spine faces a dilemma: a **dense** globally-indexed N-wide
vector at every node is a plain elementwise Sum but pays O(N) everywhere; a **sparse** local
representation is cheap but its up-merge is a keyed merge by global faction id that `SlotRange Sum`
cannot do (it would need GPU indirection). **Stratify, and resolve the index mapping on the CPU at
enrollment** — the GPU only ever does fixed-width elementwise Sums, so **no new GPU primitive and no
sandbox probe.**

- **Leaf stratum (RegionCell, StarSystem):** allocate a small fixed set of **anonymous local
  channels** (Channel 0..`c-1`, `c = 4`). The GPU treats them as opaque columns; the meaning lives in
  a CPU `faction_id → channel_idx` binding computed at enrollment. **This is the *same* local identity
  layer the combat masking uses (§11.1–§11.3): the leaf channel count and `max_factions_per_cell` are
  one tunable, not two.**
- **Dense stratum (toward the root):** when a flow climbs past the point where the present-set
  exceeds `c`, it transitions to a **dense N-wide vector** (column F = global faction F). Because that
  layout exists only at the top, the O(N) cost is confined to a few high nodes.

**The cap is on the present-set *union* at a node** (= union of its children's), which grows
monotonically up the tree — so the compact→dense frontier is the level where `|present-set| > c`. Make
it **cardinality-driven, not level-fixed**: a contested sector (or a besieged capital with 5+ owners)
slides the frontier down gracefully; a quiet region keeps it high. The bottom-up presence pass
computes the frontier for free.

**Correctness pivot — a parent-imposed shared binding.** Elementwise Sum across channels is valid
**only if all merged children agree what each channel means.** So channel assignment is **imposed by
the parent and inherited by children**, via a two-pass enrollment (all CPU, deterministic, only on
change):

1. **Bottom-up presence gather:** each node's present-set = union of children's (also detects the
   dense frontier where `|union| > c`).
2. **Top-down channel assignment:** each node binds Channel 0..`c-1` from its present-set **sorted by
   `faction_id`** (never arrival order — or replay diverges); children inherit the parent's binding
   for shared factions.

The per-tick GPU pass is then a pure bottom-up elementwise Sum over aligned channels. **The keyed
merge is not eliminated — it is relocated to CPU enrollment and bounded to binding-change events**
(faction enters/leaves a node), which fold into the REENROLL boundary work already paid.

**Required mechanisms (all CPU, all existing paths):**
- **Deterministic binding** (sort by `faction_id`) → bit-exact replay.
- **Resync on binding change** → the A-0/REENROLL generation-bump-and-resync; no new path.
- **Hysteresis on re-pack** → don't compact channels the instant a faction departs (leave it zeroed);
  re-pack only under pressure, to avoid resync churn on transient transits.
- **Promotion rule** → a node whose union exceeds `c` promotes to the dense stratum; the promotion
  point *is* the frontier from pass 1.

**Stated precondition:** the dense layer stays cheap **iff contestation is spatially local** (the
`|present-set| > c` frontier stays near the top). This holds in any realistic galaxy — N owners
cannot be physically co-present in every cell — so the O(N) mitigation is sound; record it as the
assumption.

### 11.11 Pop cohorts — the canonical multi-owner SimThing

A population cohort is a **SimThing** (not packed sub-columns on a planet); its members are a scalar
`count` property *inside* it. This is the identity boundary of §5 Tension C, fixed: the cohort is the
named unit, individuals are fungible-within.

- **Homogeneity invariant.** A cohort is homogeneous in every owner-relation it carries (exactly one
  `faction_owner`, one `species`). Partial change (a subset defects/assimilates) **splits a new
  cohort via gated fission** — never a mixed cohort. This is what keeps the single-SimThing
  representation valid (no sub-aggregate targeting).
- **Aggregating relation vs. overlay relation — the test.** A relation needs Hybrid-Strata channels
  (§11.10) **only if it pools flow up to a clearinghouse**. `faction_owner` does (the economy) →
  channels. **`species`, `blueprint_owner`, tech, policy do *not* — they only broadcast a value
  overlay *down*** and apply locally; they **never spawn an arena column**. The test: *pools up →
  channel; broadcasts down only → overlay mask.*
- **Pops as the canonical layered filter.** Conquest = `faction_owner` column write → faction overlay
  layer refreshes, species layer persists (§11.7). The species/blueprint overlays mask the cohort's
  output **first** (local, §11.7 ordering), then the faction-masked reduction aggregates the modified
  flow **second** — only the second touches channels.
- **Scale note.** Cohorts dominate the entity count, so the 34k soak is pop-dominated; ALLOC's slab +
  per-parent free lists + bulk accounting are sized for pop churn (birth/growth/migration/death).

### 11.12 The fixed-point keystone and conservation bands

**Keystone:** conserved quantities and any **structural-decision variable** (a value that gates
escalation, capture, fission) are **I64 fixed-point**, never float. This is the determinism floor
for those columns (float drift across GPU/CPU is the I8 parity threat). Continuous *rate* flows
(repair, degradation, morale drift) may stay float — they are downstream and drive no structural
transition.

**Strict band separation** enforces the soft/hard split in the OrderBand schedule (no new primitive —
a scheduling + admission discipline):

- **Band Alpha (hard, first):** exact discrete/fixed-point transfers (money, ammunition, conserved
  resources) via bit-exact integer reductions / `SubtractFromSource`. Subsidiarity balance tests live
  here, so `supply ≥ demand` is **exact and jitter-free**.
- **Band Beta (soft, second):** continuous float flows. It **reads the finalized Band-Alpha state**;
  the dependency is **one-directional (Alpha → Beta), never the reverse** — a float can never corrupt
  an exact integer. They never share a pass.

A quantity declares its class at admission; the compiler bands it; cross-class reads are Alpha→Beta
only. (This is the conservation-class guardrail at the designer/scenario layer.)

**Subsidiarity escalation stability (distinct from determinism).** Even with exact integers, a node
at the balance point can thrash (escalate/de-escalate every tick → resync churn). A
`SoftAggregateGuard::Hysteresis` **deadband** (escalate only when the deficit exceeds the node's local
supply by a quantized margin; de-escalate only well below it) anchors escalation depth. The deadband
is integer-quantized (deterministic) and its margin is a designer-facing tunable.

### 11.13 Selective routing — the bracketed reduction (single-winner argmax)

Proportional `child_share` is a *spread*; some routing is *selective* (triage to the single most
desperate consumer). This is expressed with **existing primitives only** — no `argmax` primitive:

1. **Up-sweep:** `SlotRange Max` over a **packed composite key** = `(deficit << k) | (~slot_id & mask)`
   — deficit in the high bits, inverted hardware `slot_id` in the low bits. The maximum is **unique by
   construction** (highest deficit, lowest slot_id), so the deterministic tie-break is built in — no
   separate tie-break pass.
2. **Down-sweep:** broadcast `key_max`; each child sets `is_winner = (my_key == key_max)` (an exact
   equality, safe because `Max` is an *exact* CombineFn — `key_max` is bit-identical to the winner's
   key) and multiplies its routing weight by `is_winner`.

**Precondition:** the keyed quantity must be on the **hard/fixed-point band** (§11.12) — argmax over a
soft float would need quantization first. Scope: this delivers **k=1**; top-k is a bounded iteration
(find max, exclude, repeat) if a scenario ever needs it.

### 11.14 Faction-index stability — the generational slot registry

The N-wide global faction index (the §11.10 dense layer) is treated **exactly as an ALLOC slab**:

- Faction index slots are **never mutated mid-tick** — static across a GPU execution tick.
- An eliminated faction becomes a **Ghost Node**: its slot persists, zeroed, contributing 0 flow. This
  **preserves global-index alignment** (every node's column F still means faction F).
- Index reclaim/reassignment (e.g. a rebellion taking a new index) happens **only at a synchronous
  CPU Session Boundary Break** with the A-0/REENROLL generation bump — new factions take the
  lowest-free ghost slot (deterministic). N is the live-faction cap; ghost exhaustion → admission
  rejection or a larger N. A faction born mid-cycle waits until the next boundary break for its index
  (a one-cycle latency).

---

## 12. Slot allocation at scale — slab tracking (Gallatin) + bulk accounting (NVIDIA)

Two peer-reviewed GPU-allocator papers reclassify the §5 tensions. Neither is a drop-in — both
carry design choices that conflict with SimThing invariants — but each contributes a transferable
*principle* to a different layer of the allocator, and together they compose cleanly with SimThing's
deterministic boundary protocol.

**Sources:** Gallatin: A General-Purpose GPU Memory Manager (McCoy & Pandey, PPoPP 2024,
`10.1145/3627535.3638499`); Throughput-Oriented GPU Memory Allocation (Gelado & Garland, NVIDIA,
PPoPP 2019, `10.1145/3293883.3295727`).

### 12.1 The two-layer decomposition

| Layer | Question | Paper | Contribution |
|---|---|---|---|
| **Tracking** | *Which* specific slots? | Gallatin | Slab hierarchy (segment → block → slice); block-granular reclamation; lowest-free-first packing. |
| **Accounting** | *How many* slots? | NVIDIA 2019 | Two-stage allocation decoupling accounting from tracking; bulk admission of a whole burst in one step. |

SimThing's deterministic boundary protocol is the **coordinator** that uses both as *architectural
patterns*, not as concurrency code.

### 12.2 Tracking layer (Gallatin) — dissolves Tension B

Gallatin partitions DRAM into **segments** (16 MB) → **blocks** (4096 × slice) → **slices** (the
allocation unit), reclaims a segment only when fully empty, and uses a van Emde Boas tree for
O(log log U) successor search to **always allocate the lowest-ID free segment** (packing low,
minimizing fragmentation).

Transfers to SimThing:

- **Block-granular reclamation → dissolves Tension B (contiguity).** A parent/key owns a
  **pre-formatted contiguous block**; arrivals claim **fixed-size slices within already-contiguous,
  pre-reserved headroom**. This converts the unsolvable "insert contiguously into a packed array"
  into the solved "claim the next free slice in my block." Contiguity stress moves from *every
  arrival* (pervasive) to *block-boundary events only* (rare: block exhaustion, parent death). The
  §7.2 dead-parent fragmentation worry dies at the same time. **No compaction of live slices ever**
  — respects the no-compaction invariant.
- **Lowest-free-first → bounds the high-water mark** (§7.1 burst growth), keeping the active region
  dense for active-mask/dirty-skip. "Smallest free ID" is a **deterministic policy**; run serially
  at boundary time it is fully replay-stable (respects I8).
- **Tunable block size → the Tension E supply-side knob.** Block size is a declared
  throughput/reuse tradeoff (bigger = fewer allocator ops + more headroom, but more VRAM slack).

**Do NOT adopt** the vEB structure itself (its asymptotic win matters at billions of slices; a
2-level hierarchical popcount bitmask suffices for SimThing's bounded, VRAM-budgeted slot universe),
nor in-kernel malloc, nor lock-free races (both conflict with boundary-time-decision + determinism).

### 12.3 Accounting layer (NVIDIA 2019) — demotes Tension D

NVIDIA's allocator **decouples accounting (how many) from tracking (which ones)** and makes the
contended accounting step massively parallel via a **bulk semaphore** (many simultaneous acquires
served as one aggregated op), with deferred reclamation delegated to blocked threads. 16.56× over
CUDA 9, low fragmentation. **It does no prediction** — it absorbs the burst when it arrives.

The reframe: the §7.1 burst is *not* a burst problem, it is a **serialization problem**. The doc
assumed reactive allocation can't handle bursts → therefore predict (D3). NVIDIA shows reactive
allocation handles bursts fine *if accounting is bulk*. So **prediction is demoted from requirement
to optional smoothing** — and that removes the workshop's riskiest idea (an allocator that subscribes
to simulation velocity columns, crossing a layer boundary).

Transfers to SimThing (architecture, not the concurrency primitives — the bulk semaphore is a GPU
race-resolution tool that would reinject nondeterminism; take the **two-stage decoupling**):

- **Accounting (one deterministic bulk pass):** at each boundary compute
  `net_demand[block] = births + arrivals − deaths` in a single arithmetic pass over the event set;
  one reservation per block. Burst handling becomes **O(blocks), not O(arrivals)**.
- **Tracking (deterministic, local):** hand out contiguous slices from each block's reserved run
  (Gallatin layer).
- **Batched reclamation:** net deaths against births *in the same pass* — a slice freed by a death
  is reusable by a birth in the same burst; a balanced-churn burst allocates **zero** new blocks.

> **WGSL feasibility — DO NOT attempt to port NVIDIA's bulk semaphore to WGSL.** The accounting
> layer above is **CPU/driver code by construction** — it runs serially in the boundary handler,
> not in a shader. There is nothing in §12.3 that compiles to WGSL, and that is deliberate
> (determinism / I8 / replay parity). Future agents must not "GPU-ify" it. Three reasons, in order
> of finality:
>
> 1. **We would reject it on determinism grounds regardless of capability.** The bulk semaphore is
>    an atomic-race-resolution tool; which workgroup wins is scheduler-dependent → nondeterministic
>    slot assignment → breaks replay and CPU/GPU parity. This alone disqualifies it (and fails the
>    §13 `*_determinism` / `*_parity` gates by construction).
> 2. **It is not portably expressible in core WGSL anyway (effectively CUDA-only).** The mechanism
>    depends on **warp/subgroup aggregation** (no subgroup ops in core WGSL; the `subgroups`
>    extension is optional, non-portable, and variable-width), **64-bit atomics** (WGSL has only
>    `atomic<u32>`/`atomic<i32>`; no 64-bit, no float atomics), **grid-level cooperative sync** (WGSL
>    has only `workgroupBarrier`/`storageBarrier`, no device-wide barrier), and **forward-progress
>    guarantees for blocking** (WebGPU explicitly does *not* guarantee inter-workgroup forward
>    progress — a blocking semaphore / spin-wait can **deadlock**, which kills the "deferred
>    reclamation on blocked threads" trick). A naive 32-bit-atomic port collapses to a serialized
>    counter — exactly the contention NVIDIA was escaping — so it buys nothing.
> 3. **SimThing has no accounting bottleneck to solve.** NVIDIA needed the semaphore because
>    *millions of GPU threads* call `malloc` concurrently. SimThing's accounting set is **per-block**
>    (hundreds–thousands), enumerated serially on the driver from the boundary event list. The CPU
>    pass is already optimal at SimThing's scale; there is no concurrency to parallelize away.
>
> **If GPU-side accounting is ever genuinely needed, the WGSL-native primitive is a deterministic
> prefix-sum (scan), never a semaphore:** write per-requester demand → exclusive scan → each
> requester's offset is its scan result, total is the reduction → write at `base + offset`
> (contiguous, non-overlapping, order-independent, bit-stable). SimThing already owns the building
> block (the `SlotRange Sum` reduction family). This stays generic non-semantic WGSL with CPU-oracle
> parity. But per reason 3, even this is unnecessary today — do not build it speculatively.

### 12.4 Where SimThing beats both papers

Gallatin's accepted weakness is **delayed reclamation** (can't free a segment until its last slice
is gone — a general allocator can't know when a region is logically dead). SimThing **does** know:
**parent removal is a deterministic boundary event.** SimThing reclaims a dead block *promptly* at
the boundary it dies — prompt block reclamation a racy general-purpose allocator structurally
cannot achieve. The deterministic boundary protocol turns both papers' patterns strictly cleaner
than they are in their own settings.

### 12.5 Net effect on the five tensions (post-literature)

| Tension | Status after §11–§12 | Mechanism |
|---|---|---|
| **B — contiguity** | **Dissolved** | Pre-formatted slab blocks; claim slices within reserved contiguous headroom; block-granular reclaim; no compaction (Gallatin). |
| **D — predictive** | **Demoted to optional** | Bulk two-stage accounting absorbs the burst reactively; prediction survives only as smoothing (NVIDIA). Park D3. |
| **A — re-enrollment** | **Burst half absorbed; protocol half already solved** | Bulk accounting nets moves/births/deaths in one pass; atomic prepare/commit is E-2B-5. |
| **E — sizing** | **Two-sided framing** | Supply-side block-size knob (Gallatin) + demand-side exact bulk accounting (NVIDIA). |
| **C — identity boundary** | **Resolved** | The SimThing is the named unit; fungible members are a `count` column on it (cohort = SimThing); homogeneity kept by fission-on-partial-change (§11.11). |

### 12.6 The single-key cell block

Identity is a column read by a masked reduction (§11), not a grouping — so there is **one flat block
per cell** and identity change is a column write, not a slot move. The allocator surface is the
single-key cell block that A-0 (contiguous append) + REENROLL (spatial arrival) + the Gallatin/NVIDIA
slab+bulk model already cover. The identity dimension imposes **no new allocator structure** — only
`k` bounded per-identity columns on the cell (§11.3, §11.10).

---

## 13. Testing battery tracks — performance-led, with guardrails at the designer/scenario layer

These are **battery designs**, not authorizations. They are gated behind a named scenario and a
separate Opus implementation gate (§14).

**Framing (read this first).** The implementation track is **performance-led and disciplined** — its
job is to hit real throughput/scale bars, not to grind an open-ended invariant checklist. Correctness
is *not* enforced by making every implementation PR re-prove a long list of invariants (that path
burns implementers in hygiene loops); it is enforced **structurally at the designer/scenario admission
layer** — the spec rejects ill-formed configurations at import time, so the implementer inherits a
sound substrate rather than re-deriving it. Strong guardrails are most optimal at that
designer/scenario-facing layer; relax them toward it.

So each track has:
- a **performance battery** — the disciplined implementation target (scale soak, bounded per-tick
  cost, no thrash, burst absorption); this is what an implementer works to green;
- a small **substrate floor** — the few non-negotiables that *cannot* be delegated to the designer
  layer because they are intrinsic substrate correctness: **determinism / replay parity (I8),
  no-compaction, no owner-entity as a spatial parent**. Kept minimal — a floor, not a checklist;
- a set of **designer/scenario admission guardrails** — the richer correctness rules (subscription,
  blockade-immunity, capture-is-a-column-flip, layering, conservation-class, bounds). These are
  listed so the **designer-admission layer** implements them once as import-time rejections; they are
  **not** a per-implementation grind.

The five tracks follow the dependency order: **ALLOC** (deterministic slab + bulk allocator) is the
foundation; **REENROLL** (spatial mobility / bilateral re-enrollment) builds on it; **IDROUTE** (the
D=2 *local* identity-routing overlay — masked reduction + directed disburse, §11.1–§11.3) builds on
both; **ECON** (session clearinghouse + subsidiarity *global* routing + Hybrid Strata + fixed-point
conservation bands + faction-index slab, §11.4–§11.14) builds on ALLOC+REENROLL; **OWNER**
(owner-relations + latched modifier overlays + pop cohorts, §11.6–§11.7, §11.11) builds on ECON.
Scale target throughout: **34,000 AI entities** (the project's stated AI-entity target) as the soak
ceiling. The split mirrors the architecture: IDROUTE proves the *local/aligned* masked routing, ECON
proves *global/misaligned* routing through the hub, OWNER proves the *latched, blockade-immune*
overlay discipline layered on top.

### Track ALLOC — deterministic slab + bulk-accounting allocator

*Entry gate:* none beyond A-0 (landed). This is the §9-step-1/2 foundation and can be exercised
first. *Maps to:* Tensions B, D, E.

**ALLOC-GUARDRAILS (designer/scenario admission — spec-layer rejections, not a per-PR grind; ★ = substrate floor that must hold in-track):**

| Battery test | Must prove |
|---|---|
| `alloc_block_contiguity_invariant` | After any alloc/free/migrate sequence, every key's block hands out a contiguous slice run; `verify_child_contiguity` holds for all interior nodes. |
| ★ `alloc_no_live_slice_moves` | No live slice ever changes slot address mid-session (no compaction). |
| `alloc_block_granular_reclaim` | Parent/key removal returns the whole block to the kind-pool; no live slice touched; block reusable by a new same-kind key. |
| ★ `alloc_bulk_accounting_determinism` | Same boundary event **multiset** → identical slot assignment regardless of event arrival order (canonicalized ordering); replay bit-exact. |
| `alloc_net_churn_zero_growth` | A boundary with `births == deaths` under one block allocates **zero** new blocks (batched reclamation nets in one pass). |
| `alloc_lowest_free_first` | Allocation always fills the lowest free slice; high-water mark is a deterministic function of live-set history. |
| `alloc_kindpool_no_fragmentation_accrual` | N collapse/regrow cycles do not monotonically grow wasted-slot count (block reclaim prevents accrual). |
| ★ `alloc_cpu_gpu_parity` | Post-allocation arena layout produces bit-exact GPU/CPU oracle results (I8). |

**ALLOC-PERF (the disciplined implementation target):**

| Battery test | Must prove |
|---|---|
| `alloc_burst_absorption_O_blocks` | N simultaneous arrivals resolved in O(blocks) accounting steps; wall-time scales sub-linearly in N vs. a serial-claim baseline (regression-asserted speedup). |
| `alloc_high_water_bound` | Under net-positive→net-negative churn, buffer growth stays within a declared % of live-set peak. |
| `alloc_collapse_fragmentation_ratio` | Mass parent removal (empire collapse) leaves live-set density above a declared floor; wasted-slot ratio bounded. |
| `alloc_scale_soak_34k` | 34,000 entities, sustained churn, M ticks: steady-state allocation throughput and bounded buffer; no resize thrash. |

### Track REENROLL — reparenting / bilateral re-enrollment

*Entry gate:* ALLOC green (★ floor + PERF). *Maps to:* Tension A (and B via **spatial** slice
migration — a fleet moving between *cell* blocks; this is the only slice migration in the design,
identity change is a column write, §11.2). *Builds on:* the E-2B-5 prepare/commit protocol.

**REENROLL-GUARDRAILS (designer/scenario admission — spec-layer rejections, not a per-PR grind; ★ = substrate floor that must hold in-track):**

| Battery test | Must prove |
|---|---|
| `reenroll_atomic_bilateral` | Deregister-origin + register-destination commit in one boundary pass; any failure → **zero** partial mutation (E-2B-5R discipline extended to two arenas). |
| ★ `reenroll_slice_migration_contiguous` | Migrating entity lands contiguously in destination block; origin block stays contiguous; no compaction. |
| `reenroll_generation_commit_only` | Both arena registries bump generation **only** on successful commit. |
| ★ `reenroll_replay_determinism` | Same movement sequence + seed → identical layout + generation trajectory. |
| ★ `reenroll_cpu_gpu_parity_post_move` | Bit-exact parity after movement sequences (at the spatial arena's nested depth). |
| `reenroll_political_is_column_not_reparent` | A `faction_id`/ownership change does **not** trigger spatial reparenting; only spatial movement does (Tension C orthogonality). |

**REENROLL-PERF (the disciplined implementation target):**

| Battery test | Must prove |
|---|---|
| `reenroll_burst_moves_bulk` | A burst of simultaneous moves is absorbed via bulk accounting in O(affected-blocks), not O(moves). |
| `reenroll_scale_soak_34k_mobility` | 34,000 entities with continuous inter-cell movement: bounded per-boundary cost; no resize thrash. |

### Track IDROUTE — identity-routing overlay (D=2 masked reduction + directed disburse)

*Entry gate:* ALLOC + REENROLL green (the cell's single flat child block is the only allocation
surface; **there is no faction slab and no slice migration** — faction change is a column write).
*Maps to:* the §11 overlay. *Class:* per-quantity — a soft flow (e.g. damage→HP) is Resource-Flow
(`GpuVerifiedApproximate`); the overlay must not silently mix it with a hard-currency quantity.

**IDROUTE-GUARDRAILS (designer/scenario admission — spec-layer rejections, not a per-PR grind; ★ = substrate floor that must hold in-track):**

| Battery test | Must prove |
|---|---|
| `idroute_no_self_routing` | Adversarial routing never delivers a flow to a child of the *same* identity (the mask excludes self); cooperative routing delivers only to same-identity children. |
| ★ `idroute_masked_sum_correct` | Per-identity masked Sum (`firepower·(identity==F)` then contiguous Sum) equals the exact per-identity total; GPU/CPU bit-exact. |
| `idroute_flow_conservation_clamped` | Total directed flow applied = total routed, modulo clamp-at-0; parity within the authored quantity's class tolerance. |
| `idroute_k_way` | 2/3/4-identity arenas: directed disburse equals the k-term routing-predicate evaluation. |
| `idroute_relation_is_routing_variant` | Adversarial / cooperative / directed are the *same* masked-gather+disburse machine with different relation predicates; each asserted. |
| `idroute_identity_change_is_column_write` | Flipping a child's `identity` column re-routes it the **next** tick with **no slot move, no re-enrollment, no plan rebuild beyond the column read**. |
| `idroute_threshold_removal` | A child whose HP crosses 0 fires `Threshold + EmitEvent` → deterministic boundary removal. |
| `idroute_max_identities_reject` | An arena exceeding `max_factions_per_cell` (the directed-disburse EML node budget) rejects visibly at admission, no silent degrade. |
| `idroute_no_conservation_class_mix` | The overlay rejects (or isolates by band) an arena that routes a hard-currency quantity and a soft Resource-Flow quantity in the same tick without an explicit class boundary. |
| ★ `idroute_multi_term_sum_determinism` | Any multi-term routing Sum accumulates in a sorted, fixed op order → bit-exact replay; equal-weight tie-break is deterministic (`authoring_id`). |
| `idroute_argmax_single_winner` | Selective routing (triage) delivers the whole flow to exactly one child via the bracketed reduction (`Max` over a packed `(deficit<<k)\|~slot_id` key + equality match) — §11.13. |
| ★ `idroute_argmax_packed_key_unique` | The packed composite key makes the `Max` winner **unique by construction** (deterministic tie-break built in, no second pass); the argmax quantity is on the hard/fixed-point band. |

**IDROUTE-PERF (the disciplined implementation target):**

| Battery test | Must prove |
|---|---|
| `idroute_d2_masked_dispatch_scale` | Many cells (each k≈2) dispatch the D=2 masked-gather + directed-disburse within the existing AccumulatorOp pipeline; the masked gather adds ≈2 bands **independent of `k`**. |
| `idroute_concentration_one_cell` | A single contested cell with N-thousand co-located children (a decisive battle) maintains GPU occupancy and bounded per-tick cost — distinct from the spread soak. |
| `idroute_scale_soak_34k` | 34,000 entities across many simultaneous contested cells: bounded per-tick cost; bit-stable replay. |

### Track ECON — session clearinghouse + subsidiarity economy (global routing)

*Entry gate:* ALLOC + REENROLL green. *Maps to:* §11.4–§11.5, §11.7–§11.8, §11.10, §11.12, §11.14. *Class:*
per-quantity (soft Resource-Flow for rates; Line B for exact discrete). *Builds on:* A-0 nested
reduction as the worldstatemap spine.

**ECON-GUARDRAILS (designer/scenario admission — spec-layer rejections, not a per-PR grind; ★ = substrate floor that must hold in-track):**

| Battery test | Must prove |
|---|---|
| ★ `econ_rooting_no_spatial_owner` | The session's children are owner-entities + worldstatemap; **no owner-entity is a spatial-containment parent of its holdings.** Holdings' spatial parent is their cell. |
| `econ_capture_is_column_flip` | Ownership change flips a holding's owner-column with **no spatial reparenting** and no slot move. |
| `econ_subsidiarity_balances_lowest_node` | Supply/demand resolves at the lowest spatial node where it balances; a self-sufficient subtree **never escalates** to the session. |
| `econ_blockade_is_cut_edge` | Cutting a tree edge isolates the subtree below: its surplus cannot export and its deficit is not served from above — deterministically. |
| `econ_flow_shortfall_field` | The potential-vs-realized gap is a per-holding scalar field feeding the **M5 gradient/scarcity-opportunity** sink; its gradient is the decision frontier. |
| ★ `econ_circulation_parity` | Up-sweep (production/needs) + hub clearing + down-sweep (disbursement) is bit-exact GPU/CPU at the spatial depth. |
| `econ_session_is_simthing` | The clearing hub is itself a SimThing (root); per-owner state (stockpiles, vectors) is gathered from owner-children and dispersed down the spine masked by owner-column. |
| ★ `econ_shared_binding_merge_correct` | Elementwise channel Sum at a parent is correct **only** under a parent-imposed shared `faction_id→channel` binding; a child with a divergent binding is re-mapped (CPU enrollment) or rejected — never silently summed (§11.10). |
| ★ `econ_channel_binding_deterministic` | The leaf channel binding is a deterministic function of the present-set **sorted by `faction_id`** (not arrival order) → bit-exact replay. |
| `econ_channel_count_equals_max_factions_per_cell` | The leaf channel count `c` and `max_factions_per_cell` are the **same tunable**; combat masking and leaf-economy aggregation share one local identity layer (§11.10). |
| `econ_channel_rebind_resync` | A binding change (faction enters/leaves a node) rebuilds the affected GPU ops via the A-0/REENROLL generation-bump-and-resync; hysteresis prevents re-pack churn on transient transits. |
| ★ `econ_balance_test_fixed_point` | All escalation/balance-decision columns are I64 fixed-point (never float); `supply ≥ demand` is exact and jitter-free → bit-exact escalation depth (§11.12). |
| `econ_escalation_deadband_no_thrash` | A quantized `SoftAggregateGuard::Hysteresis` deadband stops escalate/de-escalate thrashing at the balance point; escalation depth is stable across ticks. |
| ★ `econ_band_alpha_before_beta` | Hard fixed-point transfers (Band Alpha) execute **before** soft float flows (Band Beta); Beta reads finalized Alpha state; the dependency is one-directional (Alpha→Beta), never shared in one pass (§11.12). |
| `econ_no_class_mix_in_pass` | A quantity declares hard/soft class at admission; the compiler bands it; an arena mixing classes in one pass is rejected at the designer/scenario layer. |
| ★ `econ_faction_index_static_during_tick` | The N-wide faction index is immutable across a GPU tick; an eliminated faction is a zeroed **Ghost Node** preserving global-index alignment; reclaim only at a CPU Session Boundary Break (§11.14). |
| `econ_faction_reclaim_deterministic` | A new faction takes the lowest-free ghost index at the boundary break (deterministic); ghost exhaustion → admission rejection or larger N. |

**ECON-PERF (the disciplined implementation target):**

| Battery test | Must prove |
|---|---|
| `econ_local_clears_cheap` | Self-sufficient subtrees cost O(subtree-depth), not O(full-spine) — subsidiarity reduces average circulation depth vs. the global-clearinghouse degenerate case. |
| `econ_dense_frontier_stays_local` | The compact→dense frontier (`|present-set| > c`) stays near the top under spatially-local contestation; dense N-wide storage is confined to O(few high nodes), not O(all nodes). |
| `econ_leaf_is_fixed_width_sum` | Leaf-stratum aggregation is a fixed-`c`-width elementwise `SlotRange Sum` with **no GPU indirection**; the `faction_id→channel` mapping is resolved entirely in CPU enrollment. |
| `econ_scale_soak_34k` | 34,000 owned entities across a deep spatial tree with mixed blockades/shortages and a moving contestation frontier: bounded per-tick cost; bit-stable replay; M5 field stable. |

### Track OWNER — owner-relations + modifier overlays (latched, blockade-immune)

*Entry gate:* ECON green (★ floor + PERF). *Maps to:* §11.6–§11.7, §11.11. *Class:* modifiers are **knowledge overlays**
(blockade-immune), distinct from the ECON resource flow. *Builds on:* C-4 OrderBand overlay compiler;
landed fission + gates.

**OWNER-GUARDRAILS (designer/scenario admission — spec-layer rejections, not a per-PR grind; ★ = substrate floor that must hold in-track):**

| Battery test | Must prove |
|---|---|
| `owner_subscription_by_column_presence` | A SimThing receives an owner-relation's overlays **iff** it carries that owner-column; a fleet (faction only) gets no species overlays with no special wiring. |
| `owner_species_equals_faction_mechanism` | A species owner-entity disperses a masked overlay by `species_owner` via the *same* path as a faction by `faction_owner`; `SpeciesRegistry` is a session-peer grouping node, not a separate system. |
| `owner_modifier_blockade_immune` | A blockaded/besieged holding still receives full owner modifier overlays (knowledge ≠ goods); only the ECON resource flow is cut. |
| `owner_layered_filter_capture_refresh` | A pop cohort with faction + species layers: capture refreshes the **faction layer only**; the **species layer persists** (conquered pop keeps its species). |
| `owner_latched_dirtyonly_dispersal` | Modifiers push down the spine only on owner-set change or owner-column change (`DirtyOnly`); they persist between pushes — **not** a per-tick flow. |
| `owner_stacking_composition` | Multiple owner layers + intrinsic overlays compose via C-4 (Add/Mul/Set, designer priorities) deterministically. |
| `owner_conditional_predicate_apply` | A predicated modifier (e.g. species/terrain-gated) applies at a holding **iff** its predicate matches the holding's properties (masked overlay). |
| `owner_capability_tree_resolves` | A capability tree resolves into ① modifier overlays and/or ② instantiation gates; tree contents stay opaque to the substrate. |
| `owner_instantiation_is_gated_fission` | Building/instantiating = fission of a blueprint **gated by the capability bitmask**, owner-columns stamped; a designed species fissions into a new owner-entity under the registry. |
| `owner_modifier_before_production_order` | Modifier-down is band-ordered **before** the production it modifies; deterministic interleave of modifier-down / economy-up / research-up / threshold. |
| ★ `owner_cohort_homogeneity_via_fission` | A cohort stays homogeneous in every owner-relation; partial defection/assimilation **splits a new cohort via gated fission**, never a mixed cohort (§11.11). |
| `owner_multi_relation_masking_per_cohort` | A cohort with `faction_owner` + `species` receives both overlays, composed (C-4); each owner-relation masks independently. |
| `owner_overlay_relation_no_channel` | A down-broadcast overlay relation (species, blueprint, tech, policy) applies a value mask locally and **never spawns an arena/aggregation column**; only flow-pooling relations (faction economy) get Hybrid-Strata channels (§11.11 test). |

**OWNER-PERF (the disciplined implementation target):**

| Battery test | Must prove |
|---|---|
| `owner_dirtyonly_amortized` | Steady-state (no owner-set changes) incurs **zero** modifier dispersal cost; cost is paid only on unlock/policy/capture events. |
| `owner_band_budget_audit` | The interleaved circulations (modifier-down + economy-up + research-up + threshold) fit `max_orderband_depth` at the target spatial depth — a hard, checkable ceiling. |
| `owner_scale_soak_34k` | 34,000 entities across multiple owner-relations (faction + species) with periodic unlocks and captures: bounded cost; bit-stable replay. |

### How the tracks gate (and how they explicitly do *not*)

1. **The implementation track is the performance battery.** An implementer works to the `*-PERF`
   bars (scale soak, bounded per-tick cost, no thrash, burst absorption). That is the deliverable.
   It is **not** an open-ended correctness checklist — do not grind invariants for their own sake.
2. **The substrate floor (★) is the only correctness the implementation owns in-track**, because it
   is intrinsic and cannot be delegated: **determinism / replay parity (I8), no-compaction, no
   owner-entity as a spatial parent.** A handful of ★ tests, not a hygiene exercise. Anything whose
   speed comes from nondeterministic races (in-kernel malloc, lock-free atomics as the authoritative
   assigner) fails the ★ parity/determinism tests by construction — the literature's *architecture*
   (slab tracking, two-stage accounting) is admissible, its concurrency primitives are not.
3. **Everything else is a designer/scenario admission guardrail**, enforced once at the spec layer as
   import-time rejection (the `DesignerAdmissionDiagnosticCode` pattern): subscription, blockade-
   immunity, capture-is-a-column-flip, per-relation layering, conservation-class separation,
   `max_factions_per_cell`, block size, `max_fleet_density`, targeting/routing policy. The implementer
   inherits these as a sound substrate; they are **not** re-proven per PR. **Strong guardrails are
   most optimal at this designer/scenario-facing layer — relax them toward it.**
4. **One standing prohibition** (it's a parity-floor consequence, not a hygiene rule): do not GPU-ify
   the accounting layer or port NVIDIA's bulk semaphore to WGSL — see §12.3. Accounting is CPU/driver
   code by construction; the only admissible GPU-side variant, if ever needed, is a deterministic
   prefix-sum scan.

---

## 14. What this is not

- **Not an authorization.** Reparenting re-enrollment, the identity-routing overlay, the session
  clearinghouse / subsidiarity economy, owner-relations and latched modifier overlays, the slab/bulk
  allocator, per-parent free lists, predictive expansion, and the §13 batteries are all **parked
  behind a named scenario**. No implementation gate is open.
- **Not a rearchitecture.** The AccumulatorOp v2 substrate, Resource Flow arenas,
  faction/planet/fleet identity model, Phase T hard-currency path, and everything currently
  landed are correct and stay as designed. The additions are additive.
- **Not a criticism of E-11B scope.** E-11B was correctly scoped for static nested
  materialization. E-11B-5 was correctly deferred — the named scenario this document
  outlines would be the first concrete case that opens E-11B-5 or its reparenting analog.
- **Not the only future direction.** This document focuses on spatial mobility and slot
  allocation. The atlas production runtime / sparse-residency scheduler, mixed-kind
  hard-currency ordering, and ClauseThing/L3 are separate future gates with their own
  workshop territory.

---

## 15. Revision log

| Date | Change |
|---|---|
| 2026-05-31 | Initial workshop findings. Integrated with post-A-0/B-0/C-closure repo state. |
| 2026-05-31 | Explored (and **retired**) a D=3 structural faction tier and a `(cell,faction)` two-key slab for combat/ownership; both replaced by the D=2 masked-reduction model now in §11 — recorded here only for traceability, do not re-derive. Added §12 (Gallatin/NVIDIA allocation literature) and the first §13 battery tracks. |
| 2026-05-31 | Added a WGSL-feasibility callout to §12.3 and a cross-reference from the §13 determinism gate: **do not port NVIDIA's bulk semaphore to WGSL.** Accounting is CPU/driver code by construction; the semaphore is rejected on determinism grounds and is not portably expressible in core WGSL (no subgroup ops, no 64-bit/float atomics, no device-wide barrier, no forward-progress guarantee for blocking — effectively CUDA-only); SimThing has no accounting bottleneck at its scale. Only admissible GPU-side variant if ever needed is a deterministic prefix-sum scan. |
| 2026-06-01 | **Global ownership architecture — expanded §11 (§11.4–§11.9).** Established: (1) *local vs. global routing* — an "arena" is the subtree where a masked flow balances; aligned relations (combat) mask in place at D=2, misaligned relations (economy) climb the spine. (2) *Session clearinghouse topology* — `GameSession` root with faction-entities, a `SpeciesRegistry`, and the `worldStateMap` as **sibling children**; owner-entities are session-descendants, **never spatial parents**; capture = column flip, never reparenting. (3) *Owner-relations generalized* — one spatial parent + N owner-columns; subscription is automatic from column presence; **species ≡ faction structurally**; capability trees are abstract (resolve → modifier overlays + instantiation gates); **instantiation = gated fission**. (4) *Two disciplines on the spine* — per-tick **blockable** subsidiarity resource flow (blockade = cut edge; potential-vs-realized shortfall → M5 gradient heatmap) vs. **latched, blockade-immune** modifier overlays (knowledge ≠ goods; `DirtyOnly`; per-owner-relation **layered** overlay filter where capture refreshes the faction layer only and the species layer persists). (5) *Frontier V1 / A-0 re-rooting* — FV1 is the `k=1` case; A-0's nested hierarchy is the worldstatemap spine; reused verbatim, only re-labeled at the top. Added §13 Tracks **ECON** (clearinghouse + subsidiarity) and **OWNER** (owner-relations + modifiers), extended the cross-track invariants (no spatial owner-parent, capture-is-column-flip, modifier blockade-immunity, per-relation layering) and added a `owner_band_budget_audit`. Corrected the earlier (chat-only) claim that blockade *attenuates* a modifier: modifiers are blockade-immune; the heatmap is a pure flow shortfall. |
| 2026-06-01 | **Reframed §13 — dropped the "durability" requirement to prevent hygiene/token-burn loops.** The implementation track is now **performance-led and disciplined** (the `*-PERF` bars are the deliverable), with a minimal **substrate floor** (★: determinism/replay parity I8, no-compaction, no owner-entity as a spatial parent) as the only correctness the implementer owns in-track. All richer correctness rules are reframed as **designer/scenario admission guardrails** enforced once at the spec layer (`DesignerAdmissionDiagnosticCode` import-time rejections), not re-proven per PR — per the standing directive that strong guardrails are most optimal at the designer/scenario-facing layer. Renamed `*-DUR (durability)` → `*-GUARDRAILS (designer/scenario admission)`, retitled §13, rewrote the gating rules, and marked the ★ floor rows. No change to the architecture; only how the work is framed and gated. |
| 2026-06-01 | **Resolved Gap #1 (keyed-merge / N-wide scale ceiling) — added §11.10 Hybrid Strata.** Leaf stratum (RegionCell/StarSystem) uses a small fixed set of **anonymous local channels** (`c=4`, the *same* layer as combat masking — leaf channel count ≡ `max_factions_per_cell`); the `faction_id→channel` binding is resolved on the **CPU at enrollment**, so the GPU does only fixed-width elementwise `SlotRange Sum` — **no new GPU primitive, no sandbox probe.** Flows transition to a **dense N-wide vector** only past the present-set-union cap, confining O(N) to a few high nodes (precondition: spatially-local contestation). Correctness pivot: a **parent-imposed shared channel binding** (two-pass CPU enrollment — bottom-up presence union, top-down assignment sorted by `faction_id`); the keyed merge is **relocated to CPU enrollment and bounded to binding-change events** (folds into REENROLL), not eliminated. Cutoff is **cardinality-driven, not level-fixed** (handles a contested sector/besieged capital). Added ECON guardrails (`econ_shared_binding_merge_correct` ★, `econ_channel_binding_deterministic` ★, `econ_channel_count_equals_max_factions_per_cell`, `econ_channel_rebind_resync`) and PERF (`econ_dense_frontier_stays_local`, `econ_leaf_is_fixed_width_sum`). |
| 2026-06-01 | **Resolved Gaps #2–#6 (§11.11–§11.14), all without new GPU primitives.** **#6 (multi-owner pops, §11.11):** a pop cohort is a SimThing with a `count`-within (Tension C resolved); homogeneity kept by fission-on-partial-change; the *aggregating-vs-overlay test* — only flow-pooling relations (faction economy) get Hybrid-Strata channels, while species/blueprint/tech are **down-broadcast overlay masks that never spawn an arena column**. **Keystone + #4/#5 (§11.12):** conserved/decision quantities are **I64 fixed-point**; **Band Alpha (hard/exact, first) → Band Beta (soft/float, second, one-directional read)** gives conservation-class separation; subsidiarity balance test on Alpha is exact; a quantized `Hysteresis` deadband stops escalation thrash. **#2 (§11.13):** selective/argmax routing via a **bracketed reduction** — `Max` over a packed `(deficit<<k)|~slot_id` key (unique winner + deterministic tie-break in one pass) + equality match; no `argmax` primitive. **#3 (§11.14):** the N-wide faction index is a **generational slab** — Ghost-Node zeroing keeps global-index alignment, reclaim only at a CPU Session Boundary Break. Collapsed §5 to a resolution map and §8 to a status table; terse-ified §6; pruned §10 to scenario parameters; §11.9 now lists one open question (the OrderBand depth budget — `owner_band_budget_audit`). Added IDROUTE/ECON/OWNER battery rows for each closure. |

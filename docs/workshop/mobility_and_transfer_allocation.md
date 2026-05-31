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

## 5. The five underlying architectural tensions

These are the tensions that any reparenting re-enrollment implementation must navigate.

### Tension A: Static enrollment assumption vs. spatial mobility

The entire current substrate is built on the assumption that **session-open topology is
stable** for enrollment purposes:
- Arena participant rosters are built at session open.
- Enrollment selectors resolve once (E-10).
- Slot ranges are assigned contiguously.
- The GPU allocation plan is uploaded once and patched only by fission.

E-2B-5 (fission dynamic enrollment) extends this to handle birth. But fission is a one-way
door: entities are born and assigned slots; they do not move. Spatial mobility requires the
substrate to handle **bilateral re-enrollment**: an entity simultaneously leaving one arena
and joining another. This is a generalization of the fission pattern, not just an extension.

### Tension B: SlotRange contiguity as load-bearing primitive vs. non-contiguous arrivals

The GPU allocation kernel uses `SourceSpec::SlotRange { start, count, col }` — a contiguous
slot span. This is **architecturally correct and must not be compromised**: no compaction, no
indirection lists, no mid-session slot reordering (all rejected by the v7.7 constitution and
E-11B-4 enforcement).

But when Fleet X arrives at Cell B, its slot was assigned at session open (or fission time)
under Cell A. That slot is **non-contiguous with Cell B's participant block by construction**.
Cell B's allocation plan cannot simply absorb it with a contiguous SlotRange.

Three resolution paths exist (see §6).

### Tension C: Entity identity boundary — named slot vs. fungible aggregate

The current model assigns **one slot per named SimThing participant**. This is correct for
entities with persistent identity (fleets, ship classes, factions, planets). It is
**overcomplete** for fungible mass units (individual fighters within a class, individual
soldiers within a garrison cohort) where what matters is the aggregate count and HP pool, not
individual identity.

For fungible units, the correct representation is **aggregate columns on the parent**: a ship
class carries `fighter_count` and `fighter_hp_pool` as columns; damage allocation is a formula
over those columns, not a per-fighter slot reduction. This eliminates the slot churn problem
for the fungible tier entirely — and also makes the fleet movement problem easier, because
the unit of mobility is the ship class (named, has identity) not the fighters within it.

This is partly a game design decision (where is the identity boundary?), but it has
substrate implications: the slot budget at session open is computed from spec declarations,
and the spec must declare whether a given kind is a named participant or an aggregate column.

### Tension D: Reactive slot allocation vs. predictive allocation

The current allocator is **reactive**: tombstoned slots are reclaimed LIFO; buffer resize
fires when slot count exceeds capacity. This works well for steady-state churn but degrades
during net-positive growth bursts (productive empires, fleet buildups) and endgame
mass-collapse events.

The **state field already contains predictive information**:
- `Velocity` columns carry current growth/decay rates.
- `Balance` columns carry net flow state per arena.
- Pressure columns aggregate demand across children.

The allocator is currently a consumer of the **spec** (static declarations) and **boundary
events** (structural mutations). Making it also a consumer of **state field summaries** (via
the B-4 summary/readback infrastructure) would enable predictive pre-allocation. This is a
new consumer cross-cutting the layer but is architecturally consistent with how the boundary
protocol already uses state summaries for threshold detection.

The tension: the allocator knowing about simulation state is a new dependency. It must be
driver-internal (the spec does not declare allocation profiles) or spec-declared (explicit
allocation profiles surfaced to designers). Both paths are viable; driver-internal is lower
blast radius.

### Tension E: Per-cell arena sizing as static declaration vs. dynamic reality

Arena `max_participants` is a static designer declaration. For cell arenas with mobile
fleet participants, the real capacity need depends on how many fleets converge on a cell
during a battle. Declaring too high wastes VRAM; declaring too low forces a scenario-time
rejection.

The state field contains the information to derive this dynamically: velocity columns of
approaching fleets, integrated over expected travel time, give the projected arrival count.
This is the same predictive pattern as Tension D, applied to arena capacity rather than slot
count. Tension E is a manifestation of Tension D at the arena level.

---

## 6. Slot contiguity resolution options

For cell-level flat-star arenas where fleets move in and out, three options exist:

| Option | Mechanism | VRAM cost | New GPU surface | Viability |
|---|---|---|---|---|
| **Gap reservation** | Reserve N slots per cell at session open for arriving fleets; `max_fleet_density` declared in spec | Proportional to `max_fleet_density × cell_count` | None — uses existing gap pool | **Viable** if fleet density per cell is bounded and declared in spec |
| **Indirection buffer** | Per-arena index list (non-contiguous SlotRange with indirection) | Low per-arena | New WGSL + new GPU primitive | Requires sandbox probe before admission per v7.6/v7.7 constitution |
| **Aggregate columns** | Fleet carries aggregate columns (count, HP pool, weight); no per-cohort cell arena participation | Eliminates problem for fungible units | None | **Correct** for fungible units; requires identity boundary decision (Tension C) |

**Design authority position (not yet an authorization):**
- For **fungible sub-fleet units** (fighter cohorts): aggregate columns are the right model
  and should be declared explicitly in the scenario spec. This eliminates the contiguity
  problem for that tier.
- For **named ship classes** that participate in cell arenas individually: gap reservation
  is the preferred first path, scoped to a scenario with declared `max_fleet_density`. The
  indirection buffer is a later improvement only if gap reservation VRAM cost is
  unacceptable at the scale declared by the scenario.
- The indirection buffer is a **Tier-2 substrate change** requiring a sandbox probe and
  Opus review before any admission; it is not a v1 option.

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

---

## 8. Open design decisions

These must be resolved before any implementation gate opens. They are not blocked on each
other except where noted.

**D1 — Reparenting trigger scope**
How does the boundary protocol know that a reparenting mutation requires arena re-enrollment?

- *Option 1 (automatic):* automatic for any arena whose enrollment selector reads
  tree-position or parent-identity; declared opt-out for purely column-driven arenas.
- *Option 2 (declared):* arena spec declares `reparenting_enrollment: Auto | Opt-Out`.

**Recommendation:** Automatic for the default case (spatial arenas); declared opt-out for
political/overlay arenas. The enrollment selector is already the right predicate — if a
selector reads `parent_id` or `spatial_parent`, reparenting must trigger re-enrollment.
This is a spec-layer decision, not a runtime heuristic.

**D2 — Slot contiguity resolution for arriving fleets**
Gap reservation vs. indirection buffer vs. aggregate columns (see §6). Must be resolved
before implementation gate. Depends on D5 (identity boundary for ship-class cohorts).

**Recommendation:** Resolve D5 first. If ship-class cohorts are aggregate columns (not
named slots in cell arenas), the contiguity problem shrinks significantly. If ship classes
participate individually, declare `max_fleet_density` in the scenario spec and use gap
reservation. Do not open the indirection buffer path without a sandbox probe.

**D3 — Predictive allocator wire scope**
Which state field columns drive pre-allocation, what the projection horizon is, and whether
the allocation profile is driver-internal or spec-declared.

**Recommendation:** Driver-internal first, reading velocity columns via B-4 summary
readback, projecting forward by a configurable fixed horizon (default 10 ticks). Spec-
declared allocation profiles are a later capability. No spec surface change in the first
slice.

**D4 — Per-parent free list policy**
Confirm per-parent generational tombstoning as the default slot reuse policy. Specify:
- Initial block size policy (declared in spec or heuristic).
- Kind-pool compaction protocol on parent removal.
- Overflow pool policy for parents exceeding their initial block size.
- Global kind-pool fragmentation mitigation (periodic consolidation vs. lazy).

**Recommendation:** This decision is independent of D1/D2/D3 and can land earlier as a
self-contained allocator improvement. The only dependency is that the per-parent free list
must integrate cleanly with the reparenting de-registration protocol (when a fleet leaves
a cell, its slots return to the fleet parent's free list, not the cell's free list).

**D5 — Fungible unit identity boundary**
Where does individual named-entity identity end and aggregate columns begin for each
entity kind in the target scenario?

Candidates:
- Fleet, ship class → named SimThing, individual slot, full lifecycle identity.
- Fighter within a ship class, individual soldier within a garrison cohort → aggregate
  column on the parent class.

**This is partly a game design decision.** The substrate accommodates either, but the schema
must be declared before session-open slot budgeting can be done. If a ship class's fighters
are aggregate columns, those columns must be declared in the `ExplicitParticipantSpec` for
the ship class, not as child SimThings.

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

These cannot be answered from architecture alone — they require product/scenario input.

1. **Max fleet density per cell** — bounds gap reservation VRAM cost and determines whether
   gap reservation is viable or the indirection buffer is needed.
2. **Identity boundary for ship-class cohorts** — are individual ship classes named
   SimThings with their own slots in cell arenas, or are they aggregate columns on the
   fleet? (D5)
3. **Sector supply vs. cell combat** — are sector-level supply arenas separate from
   cell-level combat arenas, or does supply flow through the same cell arena? Determines
   whether the sector → cell edge is a Resource Flow coupling or just tree structure.
4. **Endgame fleet/cohort count** — bounds the predictive expansion projection horizon
   and the kind-pool sizes.
5. **Fleet doctrine uniformity** — is the weight column assignment (how a fleet shares
   damage with its ship classes) uniform across all fleets or fleet-specific? Determines
   whether Policy A (Inherit) covers arrival enrollment or Policy B (Reevaluate) is needed.
6. **Political arena enrollment** — do faction-scoped arenas (e.g., empire-wide resource
   production) enroll participants by spatial position or by `faction_id` column? This
   determines whether an ownership change triggers D1's re-enrollment predicate or is
   handled entirely as a column update.

---

## 11. What this is not

- **Not an authorization.** Reparenting re-enrollment, per-parent free lists, and predictive
  expansion are all **parked behind a named scenario**. No implementation gate is open.
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

## 12. Revision log

| Date | Change |
|---|---|
| 2026-05-31 | Initial workshop findings. Integrated with post-A-0/B-0/C-closure repo state. |

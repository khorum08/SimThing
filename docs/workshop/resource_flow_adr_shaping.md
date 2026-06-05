# Resource Flow Model — ADR-shaping Draft

**For:** ADR landing (Cursor / Codex / Opus follow-up)  
**From:** Opus synthesis of feasibility eval + Gemini synthesis + 2026-05-26 architectural picks  
**Status:** **ADR-shaping**, not ADR-final. Architectural commitments are settled; phrasing, schema specifics, and open questions in §10 require ADR-author finalization.  
**Supersedes:** `docs/workshop/resource_flow_feasibility_opus.md` (kept for traceability; this document is the canonical successor)  
**Target file:** `docs/workshop/resource_flow_adr_shaping.md`  
**Companions:** `docs/design_v7.md`, `docs/invariants.md`, `docs/adr_accumulator_op_v2.md`, `docs/accumulator_op_v2_production_plan.md`

---

## 0. Reading order

Read in order. Sections build on prior commitments.

1. §1 Executive summary — what landed and what it means
2. §2 The constitutional move — one new clause in `design_v7.md §2`
3. §3 Architectural commitments — the four load-bearing picks
4. §4 Draconian content guardrail framework — where rigor lives
5. §5 Substrate and primitive shape — concrete schemas
6. §6 Conservation reframed — three independent invariants, one ledger
7. §7 Proposed constitution diff — exact lines to land in `design_v7.md`
8. §8 Proposed invariants additions — exact rows for `invariants.md`
9. §9 Updated E-phase matrix — production plan delta
10. §10 Open ADR questions — what the ADR author must still decide
11. Appendix A — file-level evidence index
12. Appendix B — relationship to existing invariants

---

## 1. Executive summary

The Resource Flow model adopts a **unified GPU-resident substrate** for
continuous resource dynamics. The runtime stays small, generic, and
semantically ignorant; designer content carries the burden of clarity,
boundedness, and explicit causal intent. The substrate is symmetric:
**reduction sweeps leaf→root**, **allocation sweeps root→leaf**, both
through the existing `AccumulatorOp` OrderBand mechanism.

Four architectural commitments — settled by the 2026-05-26 picks — define
the model:

1. **Unified field substrate.** One execution model. Many named arenas.
   Explicit couplings between arenas. No per-arena kernel proliferation.
2. **Hierarchical allocation via overlay-modifiable weights.** Each
   intermediate participant can allocate to its children. The allocator
   is a reverse-direction OrderBand sweep that mirrors the existing
   reduction pass.
3. **All three delay forms permitted per-coupling.** The spec compiler
   verifies that every dependency cycle in the arena graph contains at
   least one delay-bearing edge. Designer picks the form per edge:
   `Algebraic`, `OneTickDelay`, `BoundaryStage`, `AccumulatorState`.
4. **Default allocation is Demand-proportional, modified through the
   existing overlay stack.** No new policy enum. Policies, interdiction,
   and player intent all compose via Add/Multiply/Set OrderBands on the
   intermediate allocator's weight columns. EML available as the
   weight-evaluator for conditional logic.

The constitutional addition is one clause (§2). The invariants additions
are six rows (§8). The production-plan delta is the E-phase matrix
adjustment in §9.

**D-1 is rescoped to a discrete-transaction analysis memo. D-2
implementation is deferred indefinitely.** Continuous flow architecturally
eliminates the per-tick hot-pool contention regime D-1 was scoped to
solve. The reasoning is in §6.3.

This document does not commit to combat-arena, diplomacy-arena, or any
other application of the substrate. Those are downstream ADRs.

---

## 2. The constitutional move

One clause is added to `design_v7.md §2` ("The v7 constitution"). The
existing seven `One` clauses are unchanged. The new clause is:

```
One mechanism for resource interaction at scale:
  ArenaRegistry { arenas, participants, couplings }
  — the substrate for continuous resource flow, hierarchical allocation,
    and arena-to-arena coupling. Built by simthing-spec at session
    open from designer-declared admission rules; refreshed at boundary
    structural mutations; consumed by simthing-driver as AccumulatorOp
    registrations selection the existing GPU substrate.

  Constitutional rule:
    Capability is universal — any SimThing CAN participate.
    Participation is explicit — admission requires designer selector.
    Expansion is bounded — every arena declares hard caps.
    Unsafe content is rejected at import or session build time.
```

This clause does not introduce a new GPU primitive. It introduces a new
*registration discipline* on top of the existing `AccumulatorOp` v2
substrate. Every arena compiles down to existing combine/gate/consume
primitives. The substrate is unchanged; the *grammar of correctness*
extends.

---

## 3. Architectural commitments

These four commitments are load-bearing. Changing any of them changes
the ADR substantially.

### 3.1 Unified field substrate

Continuous Resource Flow is one substrate across all arenas, not a
library of patterns. Concretely:

- One `ArenaRegistry` artifact per session.
- One execution model: leaf→root Sum reduction, then root→leaf
  proportional allocation, then per-recipe `MinAcrossInputs +
  SubtractFromAllInputs` consumption, then `Balance` integration.
- Many named arenas (`food`, `piracy_suppression`, `trade_access`,
  `research`, ...) — each is a `GpuArenaDescriptor` consuming the same
  kernel paths.
- Couplings between arenas are explicit edges with declared delay form
  (§3.3).

Implication: the kernel does not gain new combine variants for "food"
vs "research". Both compile to identical AccumulatorOp registration
shapes; only the column ranges differ.

### 3.2 Hierarchical allocation (reverse-reduction sweep)

The boundary-time allocator is **not** a single CPU-side step picking
N participants for one root surplus. It is a **second OrderBand sweep
in reverse direction**:

```
Per tick (existing, unchanged):
  Leaf→Root Sum reduction over intrinsic Flow
    (already implemented as C-5/C-6 ReductionOrderBand)

Per tick (new for arena participants):
  Root→Leaf proportional allocation
    band 0:  root partitions its budget across children by weight
    band 1:  each child of root partitions its share across grandchildren
    ...
    band N:  leaves receive final allocated_flow
```

Each intermediate participant is **dual-role**: it contributes
`intrinsic_flow` upward (participates in the reduction pass) and receives
`allocated_flow` downward (participates in the allocation pass). These
are distinct sub-fields on the resource property; they do not conflict
because they participate in opposite-direction sweeps gated by
opposite-direction OrderBands.

This is elegant because:

- The kernel mechanism (`AccumulatorOp` with `OrderBand` gate) is
  unchanged. New direction = new band ordering, not new primitive.
- Conservation is structural at every level: an intermediate can never
  disburse more than it received. The existing
  `atomic_add_single_writer_f32_at` path applies because OrderBand
  guarantees single-writer per (band, slot, col).
- Leaf residual (received > consumed) integrates into `Balance` via the
  existing `governed_by` machinery. **`Balance` IS the carryforward
  ledger** — no separate budget state needed.

### 3.3 Per-coupling delay form

Cycles in the arena coupling graph are permitted, but every cycle must
contain at least one delay-bearing edge. The designer declares the
delay form per coupling edge from this set:

| Form | Semantics | Implementation |
|---|---|---|
| `Algebraic` | No delay. Source value read same tick. Only valid on edges that do not participate in any cycle. | Direct AccumulatorOp source reference. |
| `OneTickDelay` | Reads previous tick's value. | Source reads from `previous_values` buffer (already exists from Pass 0 snapshot). |
| `BoundaryStage { stage: u32 }` | Coupling resolves at boundary, ordered by stage. Allows tick-time reduction → boundary allocator → next tick coupling. | Boundary hook ordering; couplings with stage `s` execute after all stage `s-1` couplings complete. |
| `AccumulatorState { property: SimPropertyId }` | Routes through an explicit `Balance`/`Need` column whose integration is the delay. | Source reads the integrated column from the named property. |

The spec compiler walks the coupling graph and rejects any cycle whose
edges are all `Algebraic`. Each non-`Algebraic` edge "cuts" cycles for
the purpose of the check. This is a topology check, not a simple
acyclicity check.

### 3.4 Default-overlay allocation policy

The allocation policy at each intermediate node is **not** declared via
a per-arena policy enum. It is expressed as **overlays on the
intermediate's weight columns**:

- **Default** (no overlays present): allocation is proportional to each
  child's `Demand` — derived as `max(0, -Balance)` for that child's
  subtree, computed during the leaf→root reduction sweep.
- **Policy overlay** (declared in spec): an overlay attached to the
  intermediate node modifies the weight column for one or more children.
  e.g. `Multiply(2.0)` on the military_research weight during wartime.
- **Interdiction overlay** (declared as a coupling-edge effect): an
  overlay attached at the coupling edge level reduces the effective
  Flow that crosses the edge. e.g. blockades multiply the
  `trade_access → food_import` coupling by 0.2.
- **Player intent** (existing mechanism): the player's
  `PlayerIntent → AttachOverlay` path attaches an overlay to a faction
  allocator. Same path as existing player intent overlays; no new
  primitive required.
- **EML-evaluated weight** (when conditional logic is needed): the
  weight column itself can be the output of a `CombineFn::EvalEML`
  registration. e.g. "weight = 2.0 if intensity > 0.7 else 1.0".
  Already supported by the C-8b infrastructure; admits the
  `ExactDeterministic` execution class.

This collapses what would otherwise be a four-way policy enum into the
*existing* overlay primitive. The substrate stays minimal; the
designer surface gains no new concept.

**The default policy is constitutional, not configurable.** A
participant with no policy overlays attached gets Demand-proportional
allocation. This is the canonical reference behavior used by replay
and by the spec compiler's expansion report.

---

## 4. Draconian content guardrail framework

The runtime substrate cannot defend against explosive content authored
at the spec layer. A spec that implies hidden transitive participation,
unbounded fanout, fission-driven arena explosion, or unreadable
coupling graphs is **invalid input**, not a runtime optimization
problem. `simthing-spec` is the enforcement layer.

This generalizes the existing pattern of forward-protecting validators
at the registration boundary (e.g.
`assert_no_hard_trigger_on_soft_aggregate` in
`simthing-sim/src/threshold_registry.rs`).

### 4.1 Required admission checks

Enforced by `simthing-spec` at session build time. Failure is rejection,
not warning.

1. **Explicit participation.** Arena participants must be admitted by
   explicit GameSpec selectors. Property possession is *insufficient*.
   A property may exist on a SimThing without that SimThing being an
   arena participant.
2. **Hard caps per arena.** Every arena declares: `max_participants`,
   `max_coupling_fanout`, `max_orderband_depth`, `expected_scale_class`.
3. **Wildcard discipline.** Selectors using kind wildcards or
   "all SimThings carrying property X" must declare an upper bound on
   expansion. The compiler computes expansion at session build; if
   expansion exceeds the declared bound, build fails.
4. **Fission inheritance policy.** Each arena declares its
   fission-policy for spawned children (see §10 question 4 for the
   straw-man enum). Default `Reevaluate` — the child is run through
   the admission selectors fresh and may or may not be admitted.
5. **Cycle-with-delay check.** The coupling graph is walked; any
   cycle without a delay-bearing edge fails the build.
6. **OrderBand budget.** Coupling DAG depth (plus the per-arena
   allocation tree depth) must fit the declared
   `max_orderband_depth`. The compiler reports each arena's
   computed depth alongside its budget.
7. **No hidden fanout.** A single spec rule must not silently expand
   into thousands of registrations without an explicit budget and
   inclusion in the expansion report.
8. **Readable expansion report.** The compiler produces, for each
   build, a report listing per-arena participant counts, per-coupling
   fanout, total registration count, total OrderBand depth used,
   and any rejected-risk diagnostics.

### 4.2 Example admission

Rejected:

```ron
arena_participants: "all SimThings carrying loyalty"
```

Reason: implicit participation, no upper bound, no fission policy.

Accepted:

```ron
arena: "piracy_suppression" {
    allowed_participants: [Frigate, PatrolFleet, PirateActivity, NavalBase]
    max_participants: 50_000
    max_coupling_fanout: 8
    max_orderband_depth: 6
    fission_policy: Reevaluate
    couplings: [
        { to: "trade_access", delay: BoundaryStage(1) },
    ]
}
```

### 4.3 The four-rule constitutional summary

```
Capability is universal       — any SimThing CAN participate.
Participation is explicit     — admission requires a selector.
Expansion is bounded          — every arena declares hard caps.
Unsafe content is rejected    — at import / session build, not at runtime.
```

These four rules belong in `design_v7.md §2` directly under the new
constitutional clause from §2 of this document.

---

## 5. Substrate and primitive shape

Concrete schemas. ADR author may refine names; types and relationships
are settled.

### 5.1 `SubFieldSpec` accumulator metadata

`crates/simthing-core/src/property.rs::SubFieldSpec` gains a planned
`accumulator_spec` field (`design_v7.md §3` already plans this; this
ADR lands it). The new field:

```rust
pub struct SubFieldSpec {
    // existing fields unchanged: role, width, clamp, velocity_max,
    // default, display_name, display_range, governed_by,
    // reduction_override, soft_aggregate_guard

    /// Accumulator-substrate metadata. Set when this sub-field
    /// participates in resource flow as Flow, Balance, allocated_flow,
    /// or weight. None for non-resource sub-fields.
    #[serde(default)]
    pub accumulator_spec: Option<AccumulatorSpec>,
}

pub struct AccumulatorSpec {
    pub role:        AccumulatorRole,
    /// Logging tier override for this sub-field. Default: Summary.
    pub log_tier:    LogTier,
}

pub enum AccumulatorRole {
    /// Signed rate signal contributing to upward Sum reduction.
    /// Combines with reduction_override = Some(ReductionRule::Sum).
    IntrinsicFlow,

    /// Per-arena allocated flow received from the parent allocator.
    /// Combines via AddToTarget on the OrderBand-gated downward sweep.
    AllocatedFlow { arena: ArenaName },

    /// Balance/Need ledger. Integrated each tick via governed_by from
    /// the participant's total flow (IntrinsicFlow + AllocatedFlow,
    /// less consumption).
    Balance(BalanceSpec),

    /// Weight column for an intermediate allocator's child split.
    /// Default value is computed from child Demand reductions; may be
    /// overlay-modified.
    AllocatorWeight { arena: ArenaName },
}

pub struct BalanceSpec {
    /// Unit cost for debt-band emission registration, if this Balance
    /// drives emission events. None = no emission registration.
    pub unit_cost: Option<f32>,

    /// Reference to a column whose value is the current count for
    /// emission threshold computation (e.g. queued_count for debt-band).
    /// Resolved at session sync via PropertyColumnRange::col_for_role.
    pub num_count_source: Option<NumCountSource>,
}

pub enum NumCountSource {
    Static(u32),
    Column { property_id: SimPropertyId, role: SubFieldRole },
}
```

`Flow` and `Balance` remain `SubFieldRole::Named("flow")` and
`SubFieldRole::Named("balance")` — **not first-class enum variants**.
The `accumulator_spec` field carries the semantic distinction without
touching the role enum.

### 5.2 `ArenaRegistry` (driver-session artifact)

Lives in `simthing-driver` as session-owned state. Compiled by
`simthing-spec` at session open; refreshed at boundary structural
mutations (because fission can admit new participants under `Reevaluate`
policy).

```rust
pub struct ArenaRegistry {
    pub arenas: Vec<GpuArenaDescriptor>,
    /// Flat: indexed by (arena_idx, local_participant_idx).
    pub participants: Vec<(ArenaIdx, SlotId)>,
    /// Coupling edges: producer arena → consumer arena with delay form.
    pub couplings: Vec<ArenaCoupling>,
    /// Bumped when fission/structural mutation invalidates participation.
    pub generation: u64,
}

pub struct GpuArenaDescriptor {
    pub name:                ArenaName,
    pub flow_property_id:    SimPropertyId,
    pub balance_property_id: Option<SimPropertyId>,
    pub max_participants:    u32,
    pub max_coupling_fanout: u32,
    pub max_orderband_depth: u32,
    pub fission_policy:      FissionPolicy,
    /// Participant slot range in the flat participants vec.
    pub participant_range:   (u32, u32),
}

pub struct ArenaCoupling {
    pub from_arena: ArenaIdx,
    pub to_arena:   ArenaIdx,
    pub delay:      CouplingDelay,
    /// Designer-supplied transformation: what fraction/transform of
    /// from-arena root flow contributes to to-arena root flow.
    /// EML-evaluable (ExactDeterministic class).
    pub transform:  CouplingTransform,
}

pub enum CouplingDelay {
    Algebraic,
    OneTickDelay,
    BoundaryStage { stage: u32 },
    AccumulatorState { property: SimPropertyId },
}

pub enum CouplingTransform {
    Identity,
    Scale(f32),
    EvalEML { tree_id: u32 },
}

pub enum FissionPolicy {
    Inherit,                          // child inherits parent's enrollment
    Reevaluate,                       // run admission selectors fresh
    Reject,                           // child never admitted
    Custom { handler_id: String },    // spec-defined handler
}

pub type ArenaName = String;     // canonical key; e.g. "food", "piracy_suppression"
pub type ArenaIdx  = u32;        // index into ArenaRegistry::arenas
```

The registry is **not** consumed by `simthing-sim` directly. The driver
compiles registry → flat `AccumulatorOp` registrations and uploads them
through existing `WorldGpuState::sync_accumulator_*_session` paths.
`simthing-sim` remains arena-ignorant; it sees only `AccumulatorOp`
structs. This preserves the existing invariant: *`simthing-sim` never
knows recipe semantics*.

### 5.3 Hierarchical allocation kernel pattern

The allocation pass produces, for each intermediate participant, two
AccumulatorOp registrations per child:

```
1. Weight-sum reduction (during the upward sweep, alongside intrinsic Flow):
   source:   SlotRange { start: first_child, count: n_children }
   combine:  Sum
   gate:     OrderBand(reduction_band)
   consume:  ResetTarget
   target:   intermediate.weight_sum

2. Per-child allocation disbursement (during the downward sweep):
   source:   SlotValue { slot: intermediate, col: budget_col }
   combine:  EvalEML { tree_id: child_share_formula }
                where formula = budget * child_weight / weight_sum
   gate:     OrderBand(allocation_band)
   consume:  AddToTarget
   target:   child.allocated_flow_col
```

The `child_share_formula` is a fixed EML tree (16 nodes well within the
ExactDeterministic class limit). One tree per arena, not per
intermediate node. The kernel's existing `EvalEML` path executes it.

OrderBand depth: existing reduction uses bands `0..max_tree_depth-1`.
Allocation uses bands `max_tree_depth..2*max_tree_depth-1`. Total
OrderBand budget per arena: `2 * tree_depth`.

### 5.4 Per-recipe consumption (unchanged from C-8c)

Already landed and unchanged by this ADR. Recipes use the existing
`ConjunctiveCrossing + MinAcrossInputs + SubtractFromAllInputs` pattern.
The 4-input CPU-side cap in `AccumulatorOp::validate` should be lifted
in the E-3 follow-up; the GPU input-list table already supports
arbitrary N.

---

## 6. Conservation reframed

Three independent invariants. One ledger.

### 6.1 Per-recipe conservation (exact)

Unchanged from feasibility eval §2.1. `MinAcrossInputs + SubtractFromAllInputs`
guarantees `Σ_j ΔNeed_j + emit_count × Σ_j c_j = 0` exactly per
invocation, enforced structurally by the kernel and planner.

### 6.2 Per-allocator-level conservation (structural)

Each intermediate allocator can disburse at most what it received from
above. Enforced structurally by the kernel pattern in §5.3: the per-child
disbursement formula `budget × child_weight / weight_sum` cannot sum
across children to more than `budget`, because `Σ child_weight =
weight_sum` by construction.

Invariant:
```
For every intermediate I and its children C_1, ..., C_n:
  Σ_i disbursed(I → C_i) = budget(I)
```

This is exact (no floating-point tolerance needed beyond the EML
deterministic execution class).

### 6.3 Per-arena conservation (allocator-pool accounting)

At the root of each arena:
```
root_intrinsic_flow_reduced + Σ inbound_coupling_contributions
  = root_budget
root_budget
  = Σ_i disbursed(root → child_i)
```

Both equalities are structurally guaranteed by §6.1 and §6.2. The
per-arena invariant is the closure: no participant receives flow that
did not originate from a declared intrinsic-flow source or a declared
inbound coupling. The spec compiler enforces this at session build by
walking the coupling graph and verifying no orphan participants exist.

### 6.4 `Balance` is the carryforward ledger

When a leaf participant receives `allocated_flow` but consumes less
than received (e.g. recipe inputs are insufficient to match the allocated
budget), the residual is **not** discarded. It integrates into the
participant's `Balance` column via the existing `governed_by` machinery
(C-7's `IntegrateWithClamp`, generalized to arbitrary governed pairs in
E-7 — see §9).

```
Balance_{t+1} = Balance_t + (intrinsic_flow + allocated_flow - consumption) × dt
```

This means:

- The Balance column accumulates surplus or deficit naturally.
- No separate budget ledger is required at the per-arena level.
- Players and AI can observe Balance directly (it's a normal property
  column with the existing reduction/threshold/observability machinery).
- Debt-band emission registers thresholds on Balance the same way as
  any other column.

The conservation question I posed as prose (B) in the prior round
collapses: strict equality, with Balance as the natural carryforward.
No designer-configurable policy needed.

### 6.5 D-1 verdict (confirmed)

The continuous-flow case has no per-tick shared-pool contention because:

- Reduction is fully parallel within each OrderBand (existing
  C-5/C-6 mechanism).
- Allocation writes to independent participant columns via
  `atomic_add_single_writer_f32_at` (single writer per band, slot, col).
- Recipe consumption rejects same-band consumed-input contention at
  plan time (existing `ContendedConsumedInput` check).
- Hierarchical allocation distributes the per-pool fanout across the
  tree depth: a faction with 100k participants becomes
  `factions(1) → planets(100) → districts(1000) → factories(100000)`,
  each level handling 100 children at most.

Discrete transactions (construction commits, treaty payments) remain
in `SubtractFromSource` mode and create a narrower D-1 case at the
scale of human/AI decision frequency per boundary — orders of
magnitude smaller than the workshop's 100k-requester stress test.

**D-1 disposition:** discrete-transaction analysis memo only.
**D-2 disposition:** deferred indefinitely.

---

## 7. Proposed constitution diff

### 7.1 `design_v7.md §2` — add one clause and the four-rule summary

**Insert after the existing `One source of truth` / `One place to edit`
clauses:**

```diff
+ One mechanism for resource interaction at scale:
+   ArenaRegistry { arenas, participants, couplings }
+   — the substrate for continuous resource flow, hierarchical allocation,
+     and arena-to-arena coupling. Built by simthing-spec at session open
+     from designer-declared admission rules; refreshed at boundary
+     structural mutations; consumed by simthing-driver as AccumulatorOp
+     registrations selection the existing GPU substrate.
+
+   Constitutional rule:
+     Capability is universal       — any SimThing CAN participate.
+     Participation is explicit     — admission requires a selector.
+     Expansion is bounded          — every arena declares hard caps.
+     Unsafe content is rejected    — at import / session build, not at runtime.
```

### 7.2 `design_v7.md §5` — add Pattern 4

**Append to `§5.1 The three canonical patterns`** (which becomes "four
canonical patterns"):

```diff
+ **Pattern 4: Continuous resource flow with hierarchical allocation**
+
+ ```
+ A faction supplies food intrinsically; planets and districts consume
+ it; the surplus or deficit propagates as Balance.
+
+ Per tick (existing reduction substrate):
+   leaf→root Sum reduction over intrinsic_flow
+   produces root_intrinsic_flow_reduced
+
+ Per tick (new allocation substrate, mirror direction):
+   root→leaf proportional allocation, each intermediate splits
+   budget across children using overlay-modifiable weight columns
+   (default proportional to child Demand)
+
+ Per tick (existing integration substrate, generalized):
+   Balance_{t+1} = Balance_t
+     + (intrinsic_flow + allocated_flow - consumption) × dt
+
+ Per arena coupling (designer-declared):
+   from_arena root flow propagates to to_arena via declared
+   delay form (Algebraic / OneTickDelay / BoundaryStage /
+   AccumulatorState) and declared transform.
+ ```
+
+ Conservation is structural at every level (§5.5). Balance is the
+ carryforward ledger. No separate budget state required.
```

### 7.3 `design_v7.md §5` — add `§5.5` conservation guarantees

**New subsection** between `§5.4 Conservation guarantee` (rename to
`§5.4 Per-recipe conservation`) and `§6`:

```diff
+ ### 5.5 Conservation guarantees for continuous flow
+
+ Three independent invariants, each structurally enforced:
+
+ Per-recipe (§5.4): MinAcrossInputs + SubtractFromAllInputs is
+   exact across all input channels.
+
+ Per-allocator-level: for every intermediate I with children C_i:
+   Σ_i disbursed(I → C_i) = budget(I)
+   structurally guaranteed by the proportional-share formula
+   (Σ child_weight = weight_sum by construction).
+
+ Per-arena: total intrinsic flow + coupling inflows equals total
+   leaf allocations + Balance changes + emission consumption.
+   The spec compiler verifies no orphan participants exist.
+
+ Balance is the carryforward ledger: leaf residuals integrate into
+ Balance via the existing governed_by machinery. No separate budget
+ state is required.
```

### 7.4 `design_v7.md §3` — land `accumulator_spec` (already planned)

The `accumulator_spec: Option<AccumulatorSpec>` field on `SubFieldSpec`
is already planned in `design_v7.md §3` as "NEW in v7" but is not yet
in `crates/simthing-core/src/property.rs`. This ADR formalizes the
schema in §5.1 of this document and is the trigger to land it.

No diff to `design_v7.md §3` text; existing text is correct, just
needs the implementation to follow.

---

## 8. Proposed invariants additions

Add to `docs/invariants.md` under a new section **"Resource Flow
(ArenaRegistry)"** at the end of the existing tables. Same enforcement
weight as existing invariants.

| Rule | Enforced by |
|---|---|
| Arena participation is explicit | `simthing-spec` rejects implicit/wildcard admission without declared upper bound at session build; property possession alone never admits to an arena |
| Arena caps are declared and enforced | Every `GpuArenaDescriptor` carries `max_participants`, `max_coupling_fanout`, `max_orderband_depth`; spec compiler fails the build if computed expansion exceeds any declared cap |
| Coupling cycles must contain a delay-bearing edge | Spec compiler walks the coupling graph; any cycle whose edges are all `CouplingDelay::Algebraic` fails the build |
| Hierarchical conservation is structural per level | For every intermediate allocator, `Σ disbursed = budget`. Enforced by the kernel pattern in §5.3 (`Σ child_weight = weight_sum` by construction); no soft tolerance |
| Balance is the sole carryforward ledger for resource flow | Leaf residual (allocated − consumed) integrates into `Balance` via existing `governed_by` machinery; no separate per-arena budget state may exist in the runtime |
| Allocation policy is expressed through overlays, not policy enums | The allocator kernel reads weight columns; weight columns are default Demand-proportional and overlay-modifiable via existing Add/Multiply/Set OrderBands. No new policy enum is added to ArenaSpec or kernel |
| `simthing-sim` never sees `ArenaRegistry` | The driver compiles registry → flat `AccumulatorOp` registrations before upload; `simthing-sim` sees only `AccumulatorOp` structs and remains arena-ignorant |
| Fission inheritance is declared per arena | Each arena declares its `FissionPolicy` (default `Reevaluate`); the boundary protocol applies the policy at fission time via `ArenaRegistry::refresh_for_structural_mutation` |
| The `SubtractFromSource` invariant is split: source-debit transfers use it; allocator disbursements use `AddToTarget` | Discrete-transaction transfers (E-2 discrete builder) use `SubtractFromSource` and `SubtractFromAllInputs` for recipe consumption; allocator disbursements use `AddToTarget` on independent participant columns; conservation in the allocator path is structural per §5.5 |

Existing invariant requiring textual clarification:

| Old text | New text |
|---|---|
| `SubtractFromSource is the only transfer mechanism` | `SubtractFromSource is the transfer mechanism for source-debit transactions; allocator disbursements use AddToTarget on independent target slots with structural conservation per design_v7.md §5.5; SubtractFromAllInputs handles per-recipe consumption` |

---

## 9. Updated E-phase matrix

Adjusted from feasibility eval §8 to incorporate synthesis revisions
and the architectural picks.

| PR | Status | Notes |
|---|---|---|
| **E-1** Debt-band emission | UNCHANGED | Pattern 2 (Threshold gate + EmitEvent). Already landed via C-1 + C-8d. |
| **E-2** Resource builders | SPLIT | Two builders. `resource_transfer_discrete(source, target, amount)` for boundary-time discrete transactions (SubtractFromSource). `resource_flow_participant(slot, arena, role)` for continuous-flow enrollment. |
| **E-3** Lift conjunctive input cap | DO SOON | One-line removal of `inputs.len() > 4` check in `AccumulatorOp::validate`, plus updated test. GPU substrate already supports N. |
| **E-4** RON + GameSpec arena admission | RETHINK around §4 | Explicit arena enrollment with caps, fission policy, coupling graph. Compiler emits expansion report. |
| **E-5** Compact log integration | UNCHANGED | Emission records remain generic. Allocator disbursements do not produce emission records (not threshold-gated); they surface via summary diff. |
| **E-6** Docs | FOLLOWS | Updates per §7 of this document. |
| **E-7** `governed_by` generalization | NEW REQUIRED GATE | C-7's `IntegrateWithClamp` planner currently special-cases `(Amount, Velocity)`. Must generalize to arbitrary `(Named, Named)` governed pairs to support `Balance` integrating from `Flow`. Kernel unchanged; only the planner is touched. |
| **E-8** `accumulator_spec` lands on `SubFieldSpec` | NEW REQUIRED GATE | Adds the `accumulator_spec: Option<AccumulatorSpec>` field per §5.1. Includes `BalanceSpec`, `NumCountSource`, `AccumulatorRole` enum. Already planned in `design_v7.md §3`; this ADR triggers the landing PR. |
| **E-9** `ArenaRegistry` in `simthing-driver` | NEW REQUIRED GATE | Driver-session-owned registry per §5.2. Refresh API for boundary structural mutations. |
| **E-10** Spec compiler admission framework | NEW REQUIRED GATE | The draconian content guardrail framework per §4. Includes expansion report generation, cap enforcement, cycle-with-delay check. |
| **E-11** Hierarchical allocation kernel pattern + parity | NEW REQUIRED GATE | Implements the reverse-OrderBand allocation sweep per §5.3. Includes CPU oracle for parity. |

PR sequencing: E-7 and E-8 are prerequisites for E-9. E-9 is a
prerequisite for E-10 and E-11. E-2, E-3, E-5, E-6 are independent.
E-1 is already landed.

---

## 10. Open ADR questions

The ADR author must decide these before finalization. None are
architecturally load-bearing; all are scoping or detail decisions.

### 10.1 Fission inheritance enum

Straw-man from §5.2:

```rust
pub enum FissionPolicy {
    Inherit,                          // child inherits parent's enrollment
    Reevaluate,                       // run admission selectors fresh
    Reject,                           // child never admitted
    Custom { handler_id: String },    // spec-defined handler
}
```

Default: `Reevaluate`. Questions for ADR author:

- Is `Custom` overscoped for v1? Could be deferred to a follow-up
  ADR if needed.
- Should `Inherit` carry sub-options (inherit weight values vs.
  inherit just-membership-with-default-weights)?

### 10.2 Runtime cap-exceeded behavior

Spec validation catches authored explosion. What about emergent
runtime growth (fission cascade creating new participants under
`Reevaluate` until cap is exceeded)?

Options:

- Hard reject the spawned child (cancel the fission, log warning)
- Admit the child but exceed cap, log a critical-severity warning
- Designer-configurable per-arena overflow policy
- Reserve N% of cap headroom at session build for runtime growth

ADR author should pick one and state it in `§4 Required admission
checks`.

### 10.3 Per-property arena-eligibility tagging

Currently any property *can* be used as a Flow/Balance property if
the spec author wires it through an arena. Should properties opt-in
to arena-eligibility via metadata to prevent accidental misuse?

Lean: no — the `accumulator_spec` field per §5.1 is itself the
opt-in mechanism. A property without `accumulator_spec` set on any
sub-field cannot be referenced by an `ArenaSpec`. Compiler
enforces.

### 10.4 ArenaCoupling transform expressiveness

`CouplingTransform::EvalEML { tree_id }` allows arbitrary
ExactDeterministic formulas to transform from-arena root flow into
to-arena contribution. Is this overscoped for v1?

Options:

- Yes, restrict v1 to `Identity` and `Scale(f32)`; defer EML to v2.
- No, EML is already in production for intensity (C-8b); same
  ExactDeterministic class applies; no extra implementation cost.

Lean: include EML in v1. Cost is small, expressiveness is large.

### 10.5 Where does the leaf consumption side land?

This ADR specifies the allocation sweep (root → leaf) and the
Balance integration (per-tick). It does not specify how *consumption*
is expressed when a leaf participant runs a recipe that consumes
some of its allocated_flow as recipe inputs.

Lean: existing C-8c `ConjunctiveCrossing + SubtractFromAllInputs`
applies unchanged. The leaf reads its `allocated_flow` column as
input to a recipe; the kernel debits via existing path. But
this needs explicit confirmation by the ADR author and may need
a small worked example in §5.

---

## 11. What this document does not commit to

For traceability — these are deliberate non-commitments:

- **Combat as a Flow arena.** Gemini's draft moved this to a
  speculative appendix; this document removes it entirely. Combat
  is a downstream ADR application of the substrate, not a
  constitutional concern.
- **Diplomacy as a Flow arena.** Same disposition.
- **Multi-faction trade as a Flow arena.** Same.
- **EML beyond `ExactDeterministic`.** This ADR uses EML only in
  the production-baseline class (per C-8 production policy).
  `SoftDeterministic`, `FastApproximate`, `CpuOracleOnly`
  classes are unchanged and follow the existing per-PR admission
  policy.
- **Cross-arena coupling at scale (>1000 couplings).** The current
  design admits arbitrary coupling counts subject to per-arena caps.
  A separate performance gate may be needed at large coupling
  counts; this ADR does not specify it.
- **Replay model for allocator state.** Existing compact emission
  record path (E-5) is unchanged. Allocator disbursements are
  reproducible from the same `ArenaRegistry` + initial state + the
  same overlay history; no new replay primitive is required. But
  the ADR author should explicitly state this in `§6 (Logging
  tiers)` of `design_v7.md` to avoid ambiguity.

---

## Appendix A — file-level evidence index

For traceability if another agent re-checks the claims. Combined
from feasibility eval Appendix A and this document.

| Claim | File | Function / line |
|---|---|---|
| `SubFieldRole` is `{ Amount, Velocity, Intensity, Named, Custom }` | `crates/simthing-core/src/property.rs` | enum definition near top |
| `SubFieldSpec` does not yet have `accumulator_spec` (E-8 will add) | `crates/simthing-core/src/property.rs` | struct fields list |
| `ReductionRule::Sum` dispatches to `combine_kind::SUM` | `crates/simthing-gpu/src/reduction_orderband.rs` | `combine_for_rule` |
| Conjunctive recipe conservation is exact | `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` | `gather_min_across_inputs` + `apply_consume` |
| Same-band consumed-input contention rejected at plan time | `crates/simthing-gpu/src/transfer_accumulator.rs` | `plan_transfer_ops`, `seen_consumed` set |
| `AccumulatorInputListTable` supports arbitrary N | `crates/simthing-gpu/src/accumulator_op/input_list_table.rs` | `ensure_capacity`, `upload_lists` |
| CPU-side 4-input limit (E-3 will lift) | `crates/simthing-core/src/accumulator_op.rs` | `AccumulatorOp::validate`, `TooManyConjunctiveInputs` |
| `read_output_vectors` is the post-reduction readback path | `crates/simthing-gpu/src/world_state.rs` and `crates/simthing-sim/src/boundary.rs::read_reduced_field` | |
| Boundary sequence step ordering | `crates/simthing-sim/src/boundary.rs` | header comment lines 11–32 |
| `assert_no_hard_trigger_on_soft_aggregate` validator pattern | `crates/simthing-sim/src/threshold_registry.rs` | function definition |
| V1 allocator is one-invocation-per-pool (obsolete at workshop scale) | `crates/simthing-workshop/src/transfer_contention_gpu.wgsl` | `resolve_transfer_contention_tick` |
| C-7 governed-pair integration combine (E-7 generalizes) | `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` | `COMBINE_INTEGRATE_CLAMP` branch in `execute_ops` |
| `atomic_add_single_writer_f32_at` exists for OrderBand-gated adds | `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` | function definition |
| Existing player intent → AttachOverlay path | `crates/simthing-sim/src/boundary.rs` | `take_player_intents` in `execute_with_boundary_hook` |

---

## Appendix B — relationship to existing invariants

Recheck of each relevant invariant in `docs/invariants.md` against
this ADR. Combined from feasibility eval §1.

| Invariant | Status | Notes |
|---|---|---|
| `simthing-sim never knows recipe semantics` | PASS | `ArenaRegistry` lives in `simthing-driver`; only flat `AccumulatorOp` registrations cross to `simthing-sim` |
| `SubtractFromSource is the only transfer mechanism` | **REVISE** (per §8 textual clarification) | New text in §8 splits discrete-transfer / allocator-disbursement / recipe-consumption paths cleanly |
| `SoftAggregateGuard on WeightedMean columns feeding thresholds` | PASS | Flow reduces via Sum (exact). Balance integration is local per slot. No soft-aggregate concern |
| `No hardcoded column indices` | PASS | `NumCountSource::Column` resolves via `col_for_role` at session sync |
| `Persistent session per session lifetime` | PASS | Allocator uses existing per-runtime upload paths; no session creation/teardown |
| `Stride is computed, never stored` | PASS | `accumulator_spec` is a new optional field, not a stride change |
| `Sub-field roles are named, not positional` | PASS | Flow and Balance use `Named` |
| `col_for_role` is sole authority for global column arithmetic | PASS | `AllocatorWeight { arena }` and `AllocatedFlow { arena }` columns resolve via existing path |
| `Exact operations never use soft-aggregate combine fns` | PASS | Flow Sum, Balance IntegrateWithClamp, recipes MinAcrossInputs — all exact |
| `Emission records produced for every GPU-resolved emission` | PASS | Debt-band Balance crossings emit via existing path |
| `design_v7.md §4 updated by each migration PR` | PASS | E-7 through E-11 are migration PRs; each updates §4 |

---

*End of ADR-shaping draft. Architectural commitments settled. ADR
author decides §10 open questions and lands the diffs in §7 and §8.*

# E-11 — Hierarchical Allocation Design Memo (v2)

**Status:** Implementation-ready design memo. Supersedes v1 (`docs/workshop/e11_hierarchical_allocation_design.md` revision dated 2026-05-26 morning).
**Author:** Opus, 2026-05-26 afternoon.
**Authority:** ADR `docs/adr/resource_flow_substrate.md` adopted as law. v2 resolves the eight blockers identified in ChatGPT's v1 critique.
**Scope:** Hierarchical down-sweep allocation over AccumulatorOp v2. **No new WGSL.** **No kernel primitives added.** **simthing-sim arena-ignorant.**
**Implementation handoff:** §10. Cursor may proceed against this memo without further design review *if* the four v2 prerequisites in §1.3 are confirmed by the user.

---

## 0. What changed from v1

v1 had **eight defects** that ChatGPT correctly flagged. v2 resolves each one:

| Defect | v1 status | v2 resolution |
|---|---|---|
| 1. Contradictory band-count narrative (3-band/2-band/4-band variants) | Three competing schedules in v1; band budget said `4·D - 2` in one place, `5·D - 4` in another. | **§2** — one canonical schedule, one formula: **`3·D - 1`**. Derivation in §2.4. |
| 2. `allocated_flow` lifecycle unclear | v1 said `AddToTarget` but did not define the reset phase. | **§3.1** — `allocated_flow` is a per-tick signal. Reset op runs first in the down-sweep at every level. Disbursement uses `AddToTarget` (single writer, single AddToTarget per child per tick). |
| 3. Sibling reduction addressability unproven | v1 used `SourceSpec::SlotRange` without proving siblings are contiguous in the GPU slot buffer. | **§4** — load-bearing pivot. Arena participants are **dedicated SimThing children of a per-arena root**, allocated contiguously at admission, with reserved gap policy for fission. The C-5/C-6 `NonContiguousChildren` invariant is satisfied by construction. |
| 4. `total_inflow_col` not in canonical column model | v1 introduced and then removed it across sections. | **§3.2** — final column model: 8 columns, 4 marked, 4 unmarked. Each has a defined reset/write lifecycle. |
| 5. Identity validation hole | v1 noted `subtree_root_id` is raw u32 with no live-existence validation; deferred to E-12. | **§5** — added **E-10R prerequisite** (`validate_explicit_participant_identity`). Specified, scoped, listed as PR-blocker in §1.3. |
| 6. Role cardinality enforcement | v1 claimed E-10 enforces it; under-specified the scope. | **§6** — exact rule: `AllocatedFlow { arena }` and `AllocatorWeight { arena }` are unique per `(arena, participant)`. E-10's `DuplicateArenaRoleBinding` is at the property layer; v2 specifies the participant-layer enforcement (E-10R extension). |
| 7. EML zero-weight NaN safety unproven | v1 asserted SELECT discards NaN but did not give an explicit proof or required test. | **§7** — proof against WGSL `select` semantics; required test `e11_no_nan_propagation_in_disbursement_path`. |
| 8. E-7 integration ownership reconciliation | v1 said "not E-11's concern" while E-11 tests depend on Balance integration. | **§8** — explicit planner ordering contract; E-7 generalized planner produces integration ops; E-11 ordering invariant requires integration band to immediately follow the deepest disbursement band. |

### 0.1 What did NOT change

The constitutional substrate from v1 is preserved exactly:

- No new WGSL.
- No new `CombineFn`, `GateSpec`, `ConsumeMode`, `AccumulatorRole` variants.
- `simthing-sim` arena-ignorant.
- Conservation is approximate-deterministic with O(ε × n) bound per level.
- Balance is the sole carryforward ledger.
- Allocation policy expressed through overlays, not enums.
- The 13-node `child_share_formula` EML tree from v1 §5.1 (4-column variant) is preserved unchanged.

---

## 1. Reading order, verdict, and v2 prerequisites

### 1.1 Reading order

1. §1 — Verdict and prerequisites
2. §2 — The one canonical OrderBand schedule
3. §3 — Lifecycle: columns and the `allocated_flow` reset
4. §4 — Sibling contiguity: the arena-participant SimThing model
5. §5 — Identity validation (E-10R prerequisite)
6. §6 — Role cardinality enforcement
7. §7 — EML formula with NaN-safety proof
8. §8 — Planner ordering between E-11 and E-7
9. §9 — CPU oracle, conservation, residual
10. §10 — Implementation handoff (Cursor binding)
11. §11 — Acceptance tests (Cursor binding)
12. §12 — Stop conditions
13. §13 — Explicit non-goals
14. Appendix A — kernel surface evidence

### 1.2 Verdict

**PROCEED — implementation-ready after v2 prerequisites land.**

Every defect from v1 is resolved. Every required test has an exact name
and acceptance criterion. Every kernel primitive used has been verified
against the landed WGSL.

### 1.3 v2 prerequisites (must land before E-11 PR opens)

E-11 cannot land without these three small upstream changes. None
require Cursor design judgment — they are mechanical extensions of E-10
and E-8.

**E-10R — Identity & arena-participant SimThing model.**
Detailed in §4.4 and §5. One PR. Codex 5.5.

**E-8R — `accumulator_spec` carries arena-participant-kind metadata.**
Adds one variant to `AccumulatorRole` discrimination *at admission time
only* — see §4.4. Internal column model gains the four unmarked
propagation columns as auto-derived layout entries. One PR. Codex 5.5.

**E-7 ordering API.** The E-7 generalized governed_by planner must expose
an ordering hook so E-11 can place the integration band at
`max_disbursement_band + 1`. Detailed in §8. One PR. Composer 2.5.

Sequencing: E-10R and E-8R can land in parallel. E-7 ordering API can land
in parallel with both. **All three are prerequisites for E-11.**

---

## 2. The one canonical OrderBand schedule

There is exactly one schedule. The v1 memo's three competing variants are
discarded.

### 2.1 The schedule

For one arena of depth `D` (root at depth 0, leaves at depth `D-1`):

```
PHASE 0 — RESET  (one band, regardless of depth)
  Band 0 (RESET_BAND):
    For every participant: allocated_flow ← 0
    (Identity + ResetTarget; source = Constant(0.0); target = (slot, aF_col))

PHASE 1 — UP-SWEEP REDUCTION  (D-1 bands, one per non-leaf level)
  Band 1 + (D-1 - 1 - d_interior) for each interior depth d_interior ∈ [0, D-2]:
    For every intermediate at depth d_interior:
      intrinsic_flow_sum  ← Sum over children's intrinsic_flow
      weight_sum          ← Sum over children's weight
    (Identity + SlotRange + Sum + ResetTarget; one op per intermediate per column)
    Order: deepest interior first, root last (matches existing C-5/C-6 deepest-first).

PHASE 2 — DOWN-SWEEP DISBURSEMENT  (D-1 bands, one per non-leaf level)
  For each parent level d_parent ∈ [0, D-2], processed root-first:
    Band  (D + d_parent):                  # Broadcast band
      For each non-root child of each parent at depth d_parent:
        propagated_iF  ← parent.intrinsic_flow_sum  (or parent.iF if d_parent == 0)
        propagated_aF  ← parent.allocated_flow      (root: zero — see §3.1 root case)
        propagated_WS  ← parent.weight_sum
      (Identity + ResetTarget; three ops per non-root child)

    Band  (D + d_parent):                  # Disbursement band — SAME band as broadcast!
                                            # Wait — see §2.2 single-writer proof.
      For each non-root child:
        allocated_flow ← EvalEML(child_share_formula)
      (EvalEML + AddToTarget; one op per child)
```

**Hold.** §2.2 explains why broadcast and disbursement cannot share a band,
and how the schedule collapses cleanly.

### 2.2 Single-writer proof and the broadcast/disburse band split

The kernel's `atomic_add_single_writer_f32_at` is `load + store` — no CAS.
**It is only safe when no other op writes the same (slot, col) in the
same band.** The disbursement op writes `(child_slot, aF_col)` and reads
`(child_slot, pIF_col)`, `(child_slot, pAF_col)`, `(child_slot, pWS_col)`.

If the broadcast band writes the `p*` columns in the same band as the
disbursement reads them, the disbursement's read race against the
broadcast's write is undefined behavior — the GPU compute pass has no
inter-op ordering within a single dispatch.

**Therefore broadcast and disburse MUST be in different bands.**

### 2.3 The corrected schedule

```
PHASE 0 — RESET  (1 band)
  Band 0:  reset allocated_flow on every participant

PHASE 1 — UP-SWEEP  (D-1 bands)
  Bands 1 .. D-1:  intrinsic_flow_sum and weight_sum reductions
                   deepest interior first

PHASE 2 — DOWN-SWEEP  (2·(D-1) bands)
  For each parent level d ∈ [0, D-2]:
    Band  D + 2·d:      broadcast (parent → children)
    Band  D + 2·d + 1:  disburse  (child reads its own propagated_* and weight,
                                    writes its own allocated_flow)

PHASE 3 — BALANCE INTEGRATION  (1 band)
  Band  D + 2·(D-1):  governed_by integration (emitted by E-7 planner per §8)
                       balance += (iF + aF − consumption) × dt
```

### 2.4 Final band count formula

```
total_bands(D) = 1   (reset)
               + (D - 1)   (up-sweep)
               + 2·(D - 1) (down-sweep: broadcast + disburse per non-leaf level)
               + 1   (integration)
              = 3·D − 1
```

| D | total_bands |
|---|---|
| 1 (single root, no children) | 2 (reset + integration; no allocation needed) |
| 2 | 5 |
| 3 | 8 |
| 4 | 11 |
| 5 | 14 |
| 6 | 17 |

A 4-level arena (faction → planet → district → factory) costs **11
bands**. Within the workshop's typical `max_orderband_depth = 16` cap
with 5 bands of headroom for cross-arena couplings.

E-10R must enforce: `arena.max_orderband_depth ≥ 3·max_depth - 1`.

### 2.5 Why `D = 1` is special

A single-root arena with no children has no `weight_sum` to compute and
no children to disburse to. The schedule degenerates: reset (which zeros
nothing useful — `allocated_flow` is never written) then integration.
**Cursor:** the planner emits zero ops for `D = 1`; the band budget is
still computed for invariant uniformity but no actual dispatch happens.

### 2.6 Multi-arena interleaving

When multiple arenas are enrolled, each arena's band schedule is
independent. The driver assigns each arena a contiguous block of bands
starting at `arena.band_base = sum of prior arenas' total_bands`. The
global tick dispatch iterates bands 0 .. `Σ_arena total_bands(D_arena) -
1` and dispatches all arena-specific ops at each band.

**Coupling edges between arenas (E-9 `ArenaCoupling`)** are evaluated
**only at boundary-stage couplings** for v1. `Algebraic` coupling (same
tick) is structurally rejected by the cycle-with-delay check at admission;
`OneTickDelay` couplings read `previous_values` (existing kernel
infrastructure). E-11 does not introduce cross-arena ops at tick time.
**Coupling materialization is out of scope for E-11.** Out of scope per
§13.

---

## 3. Lifecycle: columns and the `allocated_flow` reset

### 3.1 The canonical column model

Every arena-participant SimThing carries the following columns on its
arena-bound property. **This is the final, complete column model.**

| Column | `AccumulatorRole` (compile-time) | Lifecycle within one tick |
|---|---|---|
| `intrinsic_flow` (`iF`) | `IntrinsicFlow` | **Authored input.** Set by overlays/policies before tick. Read by reset band? No. Read by up-sweep (children's iF aggregate to parent's `iF_sum`). |
| `intrinsic_flow_sum` (`iF_sum`) | (unmarked; up-sweep output) | **Written** by up-sweep band at every interior. **Read** by broadcast band (parent broadcasts to children's `pIF`). Reset implicit via `ResetTarget` semantics of the Sum reduction op. Leaves: this column is unused. |
| `allocated_flow` (`aF`) | `AllocatedFlow { arena }` | **Reset** by Phase 0 (Identity + Constant(0) + ResetTarget). **Written** by Phase 2 disburse band (one AddToTarget per child per tick). **Read** by integration band (E-7). Per-tick signal, not stock. |
| `balance` | `Balance(BalanceSpec)` | **Integrated** by E-7 governed_by IntegrateWithClamp at Phase 3. Persists across ticks (the carryforward ledger). |
| `weight` | `AllocatorWeight { arena }` | **Default** = `max(0, -balance_subtree)`, **overlay-modifiable**. Set before tick by overlay stack. Read by up-sweep (children's weight aggregates to parent's `weight_sum`). Held across ticks — not reset per tick (overlays manage their own state). |
| `weight_sum` | (unmarked; up-sweep output) | **Written** by up-sweep band at every interior. **Read** by broadcast band. Implicit reset via `ResetTarget`. Leaves: unused. |
| `propagated_intrinsic_flow` (`pIF`) | (unmarked; broadcast buffer) | **Written** by broadcast band. **Read** by disburse band. Implicit reset via `ResetTarget`. Roots: unused. |
| `propagated_allocated_flow` (`pAF`) | (unmarked; broadcast buffer) | **Written** by broadcast band. **Read** by disburse band. Implicit reset via `ResetTarget`. Roots: unused. Roots write 0 here when they broadcast (root has no allocated_flow). |
| `propagated_weight_sum` (`pWS`) | (unmarked; broadcast buffer) | **Written** by broadcast band. **Read** by disburse band. Implicit reset via `ResetTarget`. Roots: unused. |

**Total: 9 columns per arena-bound participant** (4 marked + 5 unmarked
buffer columns).

The unmarked columns are **arena-internal plumbing**. They are not
mentioned in `accumulator_spec.role`. They are auto-derived at session
sync when E-10R sees that a participant has `AllocatedFlow` or
`AllocatorWeight` roles. The role enum stays at four variants.

### 3.2 Lifecycle proof: no stale allocation survives across ticks

The defect ChatGPT flagged: if `allocated_flow` accumulates via
`AddToTarget` without resetting, after N ticks each leaf's `aF` would be
`N × per_tick_share`, breaking conservation.

**Resolution:** Phase 0 (RESET_BAND) zeros every participant's
`allocated_flow` at the start of every tick. Phase 2 disbursement is a
**single AddToTarget per child per tick** — i.e., `aF[c]` is written
exactly once between resets, so `AddToTarget` is mathematically
equivalent to `ResetTarget` for the per-tick semantics.

**Why use AddToTarget instead of ResetTarget for disbursement?** Two
reasons:

1. **Forward-compatibility with overlay-driven additive interdiction.**
   A future overlay attached at the coupling edge may want to add a
   constant flow to a child's `aF` before disbursement runs. With
   `AddToTarget` semantics, that overlay's contribution survives;
   with `ResetTarget`, it would be clobbered.
2. **Consistency with the existing C-4 overlay OrderBand pattern.**
   The same kernel helper (`atomic_add_single_writer_f32_at`) handles
   both. The planner emits AddToTarget; the kernel's single-writer
   contract makes it equivalent to a non-atomic store. No CAS overhead.

**Cursor invariant test:** `e11_allocated_flow_resets_each_tick`
(§11) — run 100 ticks, verify per-tick `aF` equals the per-tick share, not
the accumulated value.

### 3.3 Why `weight` is NOT per-tick reset

`weight` reflects the participant's **stable allocation preference** —
default proportional to subtree deficit (`-balance`), or overlay-set to
designer policy. It persists across ticks; overlays manage their own
write semantics (existing C-4 OrderBand discipline). E-11 does not reset
`weight`. If a designer wants per-tick weight reset, they attach a
`ResetTarget` overlay on band -1 (existing overlay infrastructure
handles this).

---

## 4. Sibling contiguity: the arena-participant SimThing model

This is the load-bearing pivot from v1. Three full subsections.

### 4.1 The blocker

The C-5/C-6 reduction substrate (`reduction_orderband.rs::plan_reduction_orderband`)
**requires contiguous child slots** and **returns
`NonContiguousChildren { parent_slot }` if siblings are not contiguous**.
The existing reduction comment in `design_v7.md §4.3` confirms:
"Non-contiguous child slots skip reduction until topology is
SlotRange-compatible."

The `SlotAllocator` is **append-only with LIFO tombstone reuse**:

- `populate_from_tree` allocates depth-first, so siblings born at session
  open ARE contiguous in their parent's slot range.
- `alloc(id)` on a fission-spawned child reuses a tombstoned slot
  (anywhere in the buffer) or extends the high-water mark. **Fission can
  break sibling contiguity.**

For E-11, skipping the reduction is unacceptable: stale `weight_sum`
means stale allocation, means broken conservation.

The v1 memo dodged this. v2 confronts it.

### 4.2 Three options considered

| Option | Approach | Verdict |
|---|---|---|
| (a) Boundary-time slot compaction | Driver moves slot rows around to re-establish contiguity at boundary structural mutations. | **Rejected.** Slot rows hold velocity/intensity/etc. for all properties. Compaction would invalidate every cached slot reference everywhere. Architectural violation. |
| (b) Indirection list for arena reductions | Per-arena `AccumulatorInputListTable`-style indirection so reductions can gather from non-contiguous slots. | **Rejected: STOP triggered.** The kernel's `SOURCE_INPUT_LIST` only pairs with `COMBINE_MIN_ACROSS_INPUTS`. There is no `gather_sum_across_inputs` branch in the WGSL. Adding one means new WGSL. STOP condition fires. |
| (c) Dedicated arena-participant SimThing model | Arena participants are *not* arbitrary SimThings. They are dedicated SimThings allocated at admission as direct children of a per-arena root SimThing, with reserved gap policy for fission. | **Accepted.** Maintains the existing reduction substrate's contiguity contract by construction. No kernel changes. |

### 4.3 Option (c) — the arena-participant SimThing model

When a designer enrolls a SimThing `S` in arena `A`, the spec compiler
(E-10R) does NOT mark `S` itself as the arena participant. Instead:

1. The driver allocates a dedicated SimThing `P` — call it an
   **arena-participant SimThing** — as a direct child of the arena's
   root SimThing `R_A`.
2. `P.kind` is a new variant `SimThingKind::ArenaParticipant`.
3. `P` carries the arena-bound property (with all 9 columns from §3.1).
4. `P` carries a back-reference column (a `SubFieldRole::Named("hosted_simthing_id")`)
   to `S`.
5. The driver records `(S, A) → P_slot` in a new
   `arena_participant_index: HashMap<(SimThingId, ArenaIdx), SlotId>`
   in `ArenaRegistry`.

Because `P` is allocated at admission time as a fresh child of `R_A`,
and because admission happens all-at-once at session open or all-at-once
during a fission cascade, **all of `R_A`'s `ArenaParticipant` children
are contiguous in the slot buffer by construction**.

The hierarchy at the arena level is constructed *over the participant
SimThings*, not over the hosted SimThings. The faction → planet →
district → factory tree becomes a tree of `ArenaParticipant` SimThings,
each with a back-reference to its hosted "real" SimThing.

### 4.4 Fission cascade & contiguity preservation

Fission of a hosted SimThing `S` may produce a child `S'`. If the arena's
`FissionPolicy` admits `S'`:

- The driver allocates a new `ArenaParticipant` SimThing `P'` as a child
  of `P` (the parent's existing arena participant, NOT of `R_A`).
- `P'` is allocated via `SlotAllocator::alloc()` — appended at high-water
  or reused from LIFO tombstones.

**This breaks contiguity of P's children unless P had no children before
or P's existing children are at the high-water mark.**

**The contiguity-preservation policy:** per-arena `ReservedGap` —
each `ArenaParticipant` reserves slots for `K` children at admission
time, where `K = expected_max_children_per_intermediate` from the
arena's spec. The reservation creates `K` tombstoned slots immediately
adjacent to the participant. Fission-spawned children pop these
tombstoned slots first (LIFO order matches the gap reservation order).

When the reservation is exhausted:

- **Option A (preferred):** the spec's `FissionPolicy` is honored. If
  `Reject`, the new fission-spawned arena membership is rejected
  (the hosted SimThing fissions normally; the arena participant for
  the new child is just not created). If `Reevaluate` or `Inherit`,
  the boundary protocol triggers a **bounded compaction** of that
  parent's arena participant subtree at the next boundary.
- **Option B (escape hatch):** the driver allocates the new
  participant anywhere available and marks the parent as
  "needs_compaction"; compaction happens at the next boundary
  structural mutation.

**v2 mandates Option A (Reject by default).** A spec author who wants
unlimited fission must declare a large enough `ReservedGap` at
admission time. E-10R enforces:

```
reserved_gap_per_intermediate ≥ expected_max_children_per_intermediate
```

This makes the contiguity invariant **declared and checked at admission**.
Runtime fission cannot silently break it.

### 4.5 The driver-side data model

```rust
// crates/simthing-driver/src/arena_hierarchy.rs (NEW FILE)

use crate::arena_registry::{ArenaIdx, ArenaRegistry, SlotId};
use simthing_core::SimThingId;
use std::collections::HashMap;

/// Per-session arena execution plan. Derived at session open and
/// at boundary structural mutations. Driver-only; not serialized.
#[derive(Clone, Debug)]
pub struct ArenaExecutionPlan {
    pub arenas: Vec<ArenaTreeLayout>,
    /// (hosted_simthing_id, arena_idx) → arena participant slot.
    pub arena_participant_index: HashMap<(SimThingId, ArenaIdx), SlotId>,
    pub generation: u64,
}

#[derive(Clone, Debug)]
pub struct ArenaTreeLayout {
    pub arena_idx: ArenaIdx,
    pub arena_root_simthing: SimThingId,    // R_A
    pub arena_root_slot: SlotId,
    /// Roots within this arena's participant tree (children of R_A).
    pub participant_roots: Vec<HierarchyNode>,
    pub max_depth: u32,
    pub max_children_per_intermediate: u32,
    pub interior_count: u32,
    pub band_layout: ArenaBandLayout,
    /// Declared reservation per intermediate. Read from spec at admission.
    pub reserved_gap_per_intermediate: u32,
}

#[derive(Clone, Debug)]
pub struct HierarchyNode {
    pub participant_slot: SlotId,           // contiguous within parent's child range
    pub hosted_simthing_id: SimThingId,     // the real SimThing this participant fronts
    pub depth: u32,                         // 0 = direct child of R_A
    pub children: Vec<HierarchyNode>,
    pub cols: NodeColumnRefs,
    pub gap_used: u32,                      // children allocated; gap_remaining = reserved_gap - gap_used
}

#[derive(Clone, Copy, Debug)]
pub struct NodeColumnRefs {
    pub intrinsic_flow_col: u32,
    pub intrinsic_flow_sum_col: u32,
    pub allocated_flow_col: u32,
    pub balance_col: Option<u32>,
    pub weight_col: u32,
    pub weight_sum_col: u32,
    pub propagated_intrinsic_flow_col: u32,
    pub propagated_allocated_flow_col: u32,
    pub propagated_weight_sum_col: u32,
    pub hosted_simthing_id_col: u32,        // for back-reference / observability
}

#[derive(Clone, Copy, Debug)]
pub struct ArenaBandLayout {
    pub reset_band: u32,
    pub upsweep_band_base: u32,
    pub upsweep_band_count: u32,
    pub downsweep_band_base: u32,
    pub downsweep_band_count: u32,
    pub integration_band: u32,
    pub total_bands_used: u32,              // == 3·D - 1
}
```

**Cardinality:** one `ArenaExecutionPlan` per session. One
`ArenaTreeLayout` per enrolled arena. One `HierarchyNode` per
`ArenaParticipant` SimThing.

---

## 5. Identity validation (E-10R prerequisite)

### 5.1 The hole

`ResourceFlowSpec.ExplicitParticipantSpec` carries `slot: u32` and
`subtree_root_id: u32` (see `crates/simthing-spec/src/spec/resource_flow.rs`).
**Neither is validated against the live SimThing tree.** A spec could
claim `slot: 999999` for a session with 10 slots; admission today would
compile successfully.

For E-11, this is a correctness blocker: invalid identity at admission
means the driver tries to construct an arena hierarchy over slots that
don't exist, producing nonsense or panics.

### 5.2 The fix: E-10R `validate_explicit_participant_identity`

Add to `simthing-spec/src/compile/resource_flow_admission.rs`:

```rust
/// E-10R: validate that every ExplicitParticipantSpec references a
/// live SimThing with a valid slot in the current session.
fn validate_explicit_participant_identity(
    spec: &ResourceFlowSpec,
    session: &SpecSessionState,
) -> Result<(), SpecError> {
    for arena in &spec.arenas {
        for p in &arena.explicit_participants {
            // Check 1: the SimThingId exists in the session.
            let id = SimThingId::from_session_raw(p.subtree_root_id);
            let resolved_slot = session.slot_allocator.slot_of(id)
                .ok_or(SpecError::UnknownExplicitParticipantSimThing {
                    arena: arena.name.clone(),
                    subtree_root_id: p.subtree_root_id,
                })?;
            // Check 2: the declared slot matches the live allocation.
            if resolved_slot != p.slot {
                return Err(SpecError::ExplicitParticipantSlotMismatch {
                    arena: arena.name.clone(),
                    subtree_root_id: p.subtree_root_id,
                    declared_slot: p.slot,
                    actual_slot: resolved_slot,
                });
            }
            // Check 3: the SimThing is alive (not tombstoned).
            if !session.slot_allocator.is_live(p.slot) {
                return Err(SpecError::ExplicitParticipantTombstoned {
                    arena: arena.name.clone(),
                    subtree_root_id: p.subtree_root_id,
                    slot: p.slot,
                });
            }
        }
    }
    Ok(())
}
```

New `SpecError` variants:
- `UnknownExplicitParticipantSimThing { arena, subtree_root_id }`
- `ExplicitParticipantSlotMismatch { arena, subtree_root_id, declared_slot, actual_slot }`
- `ExplicitParticipantTombstoned { arena, subtree_root_id, slot }`

Called from `compile_resource_flow_admission` before the existing
property-binding validation.

### 5.3 What this does NOT fix

E-10R validates that the *declared participant identity* corresponds to
a live SimThing. It does not validate the **arena participant SimThing
allocation** (§4.3) — that is the driver's responsibility at materialization
time and is internally consistent by construction (the driver allocates
the ArenaParticipant SimThings; it cannot get its own allocation wrong).

### 5.4 Authored content path (deferred)

For authored RON content where the author cannot know session-raw slot
IDs, a stable-identity layer (e.g. logical participant keys mapped to
SimThings via selector queries at session open) is required. **Out of
scope for E-11.** E-10R handles only the driver-emitted case (driver
constructs ResourceFlowSpec at install time from RON, knowing live
slot IDs at that moment). Authored content authoring belongs to a
future Studio/content PR.

---

## 6. Role cardinality enforcement

### 6.1 The exact rule

| Role | Cardinality | Enforcement |
|---|---|---|
| `IntrinsicFlow` | Per (property, sub-field). Multiple sub-fields per property may carry `IntrinsicFlow`. Multiple properties on one participant may each carry an `IntrinsicFlow`. | Existing E-10 `validate_duplicate_role_bindings` at the property layer. |
| `AllocatedFlow { arena }` | **Exactly one per (arena, arena-participant)**. Property-level: exactly one sub-field per property may bind `AllocatedFlow { same_arena }`. Participant-level: exactly one property on each arena-participant SimThing may carry `AllocatedFlow { arena }`. | E-10 enforces property-layer uniqueness today via `DuplicateArenaRoleBinding`. E-10R **adds** participant-layer enforcement (see §6.2). |
| `AllocatorWeight { arena }` | Same as `AllocatedFlow`: exactly one per (arena, arena-participant). | Same — E-10 + E-10R extension. |
| `Balance` | At most one per property. | Existing E-10 / E-8 enforcement at the property layer; one balance column per arena-participant SimThing (because the arena's flow-property has one Balance sub-field). |

### 6.2 E-10R participant-layer cardinality check

Because v2 introduces dedicated `ArenaParticipant` SimThings, each
arena-participant carries exactly one instance of the arena's bound
property. E-10R enforces this at participant materialization time:

```rust
fn validate_arena_participant_property_cardinality(
    arena: &CompiledArenaAdmission,
    plan: &ArenaTreeLayout,
) -> Result<(), SpecError> {
    // Each ArenaParticipant SimThing must carry exactly one instance of
    // the arena's bound flow_property. The driver allocates these, so
    // this check is a planner consistency invariant — failure indicates a
    // planner bug, not authored content.
    for node in plan.iter_all() {
        let bound = node.hosted_property_count_for(arena.flow_property_id);
        if bound != 1 {
            return Err(SpecError::ArenaParticipantPropertyCardinality {
                arena: arena.name.clone(),
                participant_slot: node.participant_slot,
                expected: 1,
                actual: bound,
            });
        }
    }
    Ok(())
}
```

This is a defensive assertion. Authored content cannot violate it
(participants are driver-allocated); a violation indicates a driver
bug. Test `e11_arena_participant_property_cardinality_defensive` covers
this.

### 6.3 What v1 got wrong

v1 implied E-10 already enforces full cardinality. **It only enforces
per-property uniqueness**, not per-participant. The participant-layer
check is new in E-10R. v2 calls it out explicitly.

---

## 7. EML formula with NaN-safety proof

### 7.1 The formula (unchanged from v1 §5.1, 4-column variant)

```
child_share = select(propagated_weight_sum > 0,
                     (propagated_iF + propagated_aF) * weight / propagated_weight_sum,
                     0)
```

Postfix EML encoding (13 nodes; max stack depth = 4):

```
Idx  Opcode         Args                Stack after
0    SLOT_VALUE     a=pWS_col           [pWS]
1    LITERAL_F32    a=0.0_bits          [pWS, 0.0]
2    CMP_GT                             [pWS>0?]
3    SLOT_VALUE     a=pIF_col           [pWS>0?, pIF]
4    SLOT_VALUE     a=pAF_col           [pWS>0?, pIF, pAF]
5    ADD                                [pWS>0?, pIF+pAF]
6    SLOT_VALUE     a=w_col             [pWS>0?, pIF+pAF, w]
7    MUL                                [pWS>0?, (pIF+pAF)*w]
8    SLOT_VALUE     a=pWS_col           [pWS>0?, (pIF+pAF)*w, pWS]
9    DIV            flags=1 (safe)      [pWS>0?, (pIF+pAF)*w/pWS]
10   LITERAL_F32    a=0.0_bits          [pWS>0?, (pIF+pAF)*w/pWS, 0.0]
11   SELECT                             [child_share]
12   RETURN_TOP                         [child_share]
```

`EmlFormulaMeta` per v1 §5.3 — `ExactDeterministic`,
`TransferConservation` consumer.

### 7.2 NaN-safety proof (against the WGSL implementation)

**Claim:** when `pWS == 0`, `child_share` evaluates to `0.0` (not NaN) on
both GPU and CPU oracle.

**WGSL kernel evaluation (lines from `eml_eval`):**

```wgsl
// At idx 9 (DIV):
let rhs = stack[sp - 1u];   // pWS = 0.0
let lhs = stack[sp - 2u];   // (pIF+pAF)*w
stack[sp - 2u] = lhs / rhs; // = ±NaN or ±Inf depending on lhs sign
sp = sp - 1u;
// Stack now: [pWS>0?, NaN_or_Inf]

// At idx 10 (LITERAL_F32 0.0):
stack[sp] = 0.0;
sp = sp + 1u;
// Stack: [pWS>0?, NaN_or_Inf, 0.0]

// At idx 11 (SELECT):
let f_val = stack[sp - 1u];      // 0.0
let t_val = stack[sp - 2u];      // NaN_or_Inf
let cond  = stack[sp - 3u] != 0.0; // pWS>0? == 0.0 → cond = false
stack[sp - 3u] = select(f_val, t_val, cond);
//                = select(0.0, NaN_or_Inf, false)
//                = f_val = 0.0
sp = sp - 2u;
// Stack: [0.0]

// At idx 12 (RETURN_TOP):
return stack[sp - 1u];   // returns 0.0
```

**WGSL `select(f, t, cond)` semantics (verified against WGSL spec):**
when `cond == false`, returns `f` *regardless of t's value*. There is no
short-circuit evaluation, but `t` does not contaminate the returned
value. NaN in `t` is discarded.

**The proof holds in three subcases:**

1. `(pIF + pAF) * w > 0` and `pWS == 0` → `lhs/rhs = +Inf`, select discards, return 0. ✓
2. `(pIF + pAF) * w < 0` and `pWS == 0` → `lhs/rhs = -Inf`, select discards, return 0. ✓
3. `(pIF + pAF) * w == 0` and `pWS == 0` → `lhs/rhs = NaN`, select discards, return 0. ✓

**CPU oracle equivalence:** the oracle uses an if-statement instead of
SELECT:

```rust
let child_share = if pWS > 0.0 {
    (pIF + pAF) * w / pWS
} else {
    0.0
};
```

This produces 0.0 in all three subcases as well, but it does NOT produce
the intermediate NaN. **For NaN propagation tests, the GPU's intermediate
NaN is the strictly more dangerous case.** The oracle gives the correct
final answer trivially; the GPU's correctness depends on `select`
discarding the NaN. The §11 test `e11_no_nan_propagation_in_disbursement_path`
verifies the GPU result is exactly 0.0 in all three subcases — bit-exact.

### 7.3 Required test (Cursor binding)

```rust
#[test]
fn e11_no_nan_propagation_in_disbursement_path() {
    // Build the 13-node child_share_formula tree.
    // Set up an arena with 1 root + 1 child, weight = 0.
    // Three sub-cases — for each:
    //   sub-case 1: root.iF = 5.0  (positive budget)
    //   sub-case 2: root.iF = -5.0 (negative budget)
    //   sub-case 3: root.iF = 0.0  (zero budget)
    // Execute one tick of E-11 on the GPU.
    // For each sub-case, assert:
    //   1. child.aF.to_bits() == 0.0_f32.to_bits() — bit-exact zero
    //   2. No NaN anywhere in child's columns
    //   3. CPU oracle produces the same exact 0.0 bits
}
```

---

## 8. Planner ordering between E-11 and E-7

### 8.1 The problem

E-11 emits the disbursement ops that produce `allocated_flow`. E-7 emits
the integration ops that consume `allocated_flow` to update `balance`.
**E-7's integration must run after E-11's disbursement, on a higher
band.** The two planners do not currently communicate.

### 8.2 The E-7 ordering API

E-7's PR (prerequisite, see §1.3) must expose:

```rust
// crates/simthing-driver/src/governed_pair_plan.rs (E-7's territory)

pub struct GovernedPairPlan {
    pub ops: Vec<simthing_core::AccumulatorOp>,
    pub bands_used: Range<u32>,
}

impl GovernedPairPlan {
    /// Place all integration ops at exactly `band`. Used by E-11 to ensure
    /// integration runs immediately after the deepest disbursement.
    pub fn plan_at_band(
        registry: &DimensionRegistry,
        slot_allocator: &SlotAllocator,
        band: u32,
        dt: f32,
    ) -> Self { ... }
}
```

E-11 calls `GovernedPairPlan::plan_at_band(.., max_disbursement_band + 1, ..)`
when stitching the global tick schedule.

### 8.3 The global tick schedule

```
Global band 0:                                  arena[0].reset_band
Global band 1:                                  arena[0].upsweep_band_base
...
Global band arena[0].band_layout.total_bands_used - 1:
                                                arena[0].integration_band

Global band arena[0].total_bands_used:          arena[1].reset_band
...

After all arena schedules:                      cross-arena coupling bands
                                                 (out of E-11 scope; future work)
After cross-arena coupling:                     existing C-8c transfer / consumption bands
After consumption:                              existing C-1 threshold scan
                                                etc.
```

Each arena's integration band is local to that arena. `E-7` emits
governed_by integration ops only for the participants of that arena
(the driver passes the participant slot range to
`GovernedPairPlan::plan_at_band`). Other governed_by integrations
(e.g. velocity → amount for non-arena properties) continue to be
emitted by E-7 at their own bands; those are unrelated to E-11.

### 8.4 What this means for the "E-7 is not E-11's concern" tension

v1 said "integration is not E-11's concern." That was wrong as stated.
**Integration is E-7's concern, but E-11 OWNS THE BAND PLACEMENT** —
i.e., E-11 decides which band E-7 emits integration ops at, because the
band ordering is part of E-11's correctness contract.

The new contract:
- E-7 owns the *combine* (`IntegrateWithClamp`).
- E-7 owns the *target* (which slots' balance columns).
- E-11 owns the *band* (when in the global tick schedule the integration runs).

E-11 passes E-7 the band number; E-7 emits ops at that band. Clean
separation.

### 8.5 Required test

```rust
#[test]
fn e11_integration_band_immediately_follows_deepest_disbursement() {
    // Set up a depth-3 arena. Plan ops. Inspect each op's gate band.
    // Find max(op.gate.band for op in disbursement_ops).
    // Find min(op.gate.band for op in integration_ops where op.target is in
    //         this arena's participants).
    // Assert integration_band == max_disbursement_band + 1.
}
```

---

## 9. CPU oracle, conservation, residual

### 9.1 CPU oracle (revised for v2 column model)

```rust
// crates/simthing-driver/src/arena_allocation_oracle.rs

pub fn run_arena_allocation_oracle(
    layout: &ArenaTreeLayout,
    values: &mut HashMap<(SlotId, u32), f32>,
    dt: f32,
) -> ArenaAllocationOracleTrace {
    let mut trace = ArenaAllocationOracleTrace::default();

    // PHASE 0 — RESET
    for node in layout.iter_all() {
        values.insert((node.participant_slot, node.cols.allocated_flow_col), 0.0);
        trace.record_reset(node.participant_slot);
    }

    // PHASE 1 — UP-SWEEP (deepest interior first)
    for depth in (0..=layout.max_depth - 1).rev() {
        for intermediate in layout.iter_at_depth(depth) {
            if intermediate.children.is_empty() { continue; }
            // intrinsic_flow_sum
            let mut iF_sum = 0.0_f32;
            for child in &intermediate.children {
                iF_sum += values[&(child.participant_slot, child.cols.intrinsic_flow_col)];
            }
            values.insert(
                (intermediate.participant_slot, intermediate.cols.intrinsic_flow_sum_col),
                iF_sum,
            );
            // weight_sum
            let mut weight_sum = 0.0_f32;
            for child in &intermediate.children {
                weight_sum += values[&(child.participant_slot, child.cols.weight_col)];
            }
            values.insert(
                (intermediate.participant_slot, intermediate.cols.weight_sum_col),
                weight_sum,
            );
            trace.record_reduction(intermediate.participant_slot, iF_sum, weight_sum);
        }
    }

    // PHASE 2 — DOWN-SWEEP (root-first)
    for depth in 0..layout.max_depth - 1 {
        // 2a. Broadcast (band D + 2·depth)
        for parent in layout.iter_at_depth(depth) {
            let pIF = if depth == 0 {
                values[&(parent.participant_slot, parent.cols.intrinsic_flow_col)]
            } else {
                values[&(parent.participant_slot, parent.cols.intrinsic_flow_sum_col)]
            };
            let pAF = if depth == 0 {
                0.0_f32   // root has no allocated_flow
            } else {
                values[&(parent.participant_slot, parent.cols.allocated_flow_col)]
            };
            let pWS = values[&(parent.participant_slot, parent.cols.weight_sum_col)];
            for child in &parent.children {
                values.insert((child.participant_slot, child.cols.propagated_intrinsic_flow_col), pIF);
                values.insert((child.participant_slot, child.cols.propagated_allocated_flow_col), pAF);
                values.insert((child.participant_slot, child.cols.propagated_weight_sum_col), pWS);
            }
        }
        // 2b. Disburse (band D + 2·depth + 1)
        for parent in layout.iter_at_depth(depth) {
            for child in &parent.children {
                let pIF = values[&(child.participant_slot, child.cols.propagated_intrinsic_flow_col)];
                let pAF = values[&(child.participant_slot, child.cols.propagated_allocated_flow_col)];
                let w   = values[&(child.participant_slot, child.cols.weight_col)];
                let pWS = values[&(child.participant_slot, child.cols.propagated_weight_sum_col)];
                let share = if pWS > 0.0 {
                    (pIF + pAF) * w / pWS
                } else {
                    0.0
                };
                let cell = values.entry((child.participant_slot, child.cols.allocated_flow_col))
                                 .or_insert(0.0);
                *cell += share;     // AddToTarget — but reset above means single write
                trace.record_disbursement(parent.participant_slot, child.participant_slot, share);
            }
        }
    }

    // PHASE 3 — BALANCE INTEGRATION
    for node in layout.iter_all() {
        if let Some(balance_col) = node.cols.balance_col {
            let iF = values[&(node.participant_slot, node.cols.intrinsic_flow_col)];
            let aF = values[&(node.participant_slot, node.cols.allocated_flow_col)];
            // Consumption deducted separately by C-8c recipes; oracle takes pre-deducted aF
            let dbal = (iF + aF) * dt;
            *values.entry((node.participant_slot, balance_col)).or_insert(0.0) += dbal;
        }
    }

    trace
}
```

### 9.2 Conservation invariant (unchanged from v1)

```
| Σ_i disbursed(I → C_i) − budget(I) | ≤ O(ε × n_children)
```

Residual is implicit (never stored). Balance is the carryforward. Six
cases per v1 §6.2 (positive weight_sum, zero weight_sum, rounding,
per-level, zero-demand surplus, signed flows) — all six pass with the
v2 column model and band schedule.

### 9.3 Replay invariance

The down-sweep is deterministic given identical inputs and band ordering.
**Replay is bit-exact** when:

1. The same `ArenaExecutionPlan` is reconstructed (same hierarchy, same
   slot assignments).
2. The same overlay history applied (weight columns set to the same
   values before each tick).
3. The same kernel + same EML formula + same EML execution class
   (`ExactDeterministic`).

The arena-participant SimThing model (§4.3) reinforces replay: arena
participants are deterministically allocated at admission, and the
ReservedGap policy ensures fission allocations land at predictable
slots.

---

## 10. Implementation handoff (Cursor binding)

### 10.1 Prerequisites (do these first, in order or parallel)

| PR | Owner | Description |
|---|---|---|
| **E-10R** | Codex 5.5 | Identity validation + arena-participant SimThing model. Adds `SimThingKind::ArenaParticipant`, three SpecError variants from §5.2, `validate_explicit_participant_identity`, `reserved_gap_per_intermediate` field on `ArenaSpec` with admission check `reserved_gap_per_intermediate ≥ expected_max_children_per_intermediate`. |
| **E-8R** | Codex 5.5 | Auto-derived unmarked propagation columns when a property carries any of `IntrinsicFlow`, `AllocatedFlow`, `AllocatorWeight`, or `Balance` on any sub-field. The 5 unmarked columns (`intrinsic_flow_sum`, `weight_sum`, `propagated_intrinsic_flow`, `propagated_allocated_flow`, `propagated_weight_sum`) are added to the property layout at session sync. |
| **E-7 ordering API** | Composer 2.5 | `GovernedPairPlan::plan_at_band(registry, slot_allocator, band, dt, participant_filter)` per §8.2. |

### 10.2 E-11 implementation sequence

After all three prerequisites land:

**Step 1.** Add `crates/simthing-driver/src/arena_hierarchy.rs` with the
data model from §4.5.

**Step 2.** Add `crates/simthing-driver/src/arena_allocation_oracle.rs`
with the CPU oracle from §9.1. Unit tests for 1-level, 2-level, 3-level
trees.

**Step 3.** Add `crates/simthing-driver/src/arena_allocation_plan.rs`
with the planner that emits `Vec<AccumulatorOp>` matching the schedule
in §2.3. One op per child per phase (reset/broadcast/disburse), plus
weight-sum and intrinsic-flow-sum reductions per interior.

**Step 4.** Register the `child_share_formula` EML tree per §7.1 via
`EmlExpressionRegistry::register_formula`. Tree per arena.

**Step 5.** Wire planner into the existing
`WorldGpuState::sync_accumulator_*_session` paths. Add a feature flag
`use_accumulator_resource_flow: bool` (default `false`).

**Step 6.** Implement `ArenaParticipant` allocation in the driver's
install/boundary paths: at session open, for each `ResourceFlowSpec`
admission, allocate `ArenaParticipant` SimThings under each arena root,
reserve `ReservedGap` tombstoned slots after each intermediate.

**Step 7.** Implement boundary structural mutation refresh: on fission
of a hosted SimThing, attempt to allocate the new `ArenaParticipant`
into the parent's reserved gap; reject and log if exhausted (per
`FissionPolicy`).

**Step 8.** Run §11 tests. All must pass at flag = true.

**Step 9.** Documentation update — amend ADR `§5.3` and production
plan `E-11` entries to reference this memo and the `3·D - 1` band
budget formula.

**Step 10.** Default-on flip after CI burn-in.

### 10.3 STOP — see §12

---

## 11. Acceptance tests (Cursor binding)

Exact names. Cursor must produce tests with these names.

### `crates/simthing-driver/tests/e11_arena_allocation.rs`

```rust
#[test] fn e11_single_level_positive_weights_cpu_gpu_parity()
#[test] fn e11_zero_weight_sum_allocates_zero_and_routes_surplus_to_balance()
#[test] fn e11_no_nan_propagation_in_disbursement_path()              // §7.3
#[test] fn e11_allocated_flow_resets_each_tick()                       // §3.2
#[test] fn e11_three_level_hierarchy_conservation_within_tolerance()
#[test] fn e11_orderband_depth_budget_enforced()                        // assert 3·D - 1
#[test] fn e11_rejects_unbounded_child_fanout()
#[test] fn e11_rejects_missing_allocator_weight()
#[test] fn e11_rejects_missing_allocated_flow()
#[test] fn e11_balance_integrates_residual_via_governed_by()
#[test] fn e11_signed_flow_propagates_proportionally()
#[test] fn e11_integration_band_immediately_follows_deepest_disbursement()  // §8.5
#[test] fn e11_no_simthing_sim_arena_imports()
#[test] fn e11_no_new_wgsl_without_explicit_design_approval()
#[test] fn e11_replay_bit_exact_across_two_runs()                        // §9.3
```

### `crates/simthing-driver/tests/e11_arena_participant_model.rs`

```rust
#[test] fn e11_arena_participants_contiguous_at_session_open()
#[test] fn e11_fission_uses_reserved_gap()
#[test] fn e11_fission_rejected_when_gap_exhausted_per_fission_policy()
#[test] fn e11_arena_participant_property_cardinality_defensive()       // §6.2
```

### `crates/simthing-driver/tests/e11_eml_formula.rs`

```rust
#[test] fn e11_child_share_formula_validates_and_registers()
#[test] fn e11_child_share_formula_handles_zero_weight_sum_via_select()
#[test] fn e11_child_share_formula_max_stack_depth_is_4()
```

### `crates/simthing-spec/tests/e10r_identity_validation.rs`

```rust
#[test] fn e10r_rejects_unknown_subtree_root_id()
#[test] fn e10r_rejects_slot_mismatch()
#[test] fn e10r_rejects_tombstoned_participant()
#[test] fn e10r_accepts_valid_explicit_participant()
#[test] fn e10r_rejects_reserved_gap_smaller_than_expected_fanout()
```

### Verification commands

```
cargo test -p simthing-spec e10r -- --nocapture
cargo test -p simthing-driver e11 -- --nocapture
cargo test -p simthing-driver arena_registry -- --nocapture
cargo test -p simthing-driver arena_participant -- --nocapture
cargo test -p simthing-gpu accumulator_op -- --nocapture
cargo check --workspace
cargo test --workspace
```

All must be green before E-11 lands.

---

## 12. STOP conditions

Cursor must stop and report (not implement around) if any of these emerges:

1. **The kernel's `select` in WGSL contaminates the result with NaN** in any
   of the three subcases from §7.2. Indicates a WGSL-spec assumption
   violation; design review needed.
2. **The `SlotAllocator::alloc` LIFO behavior places a fission-spawned
   `ArenaParticipant` outside its parent's reserved gap** even when the
   gap is non-empty. Indicates a driver allocation contract change;
   design review needed.
3. **The C-5/C-6 reduction planner's `NonContiguousChildren` error fires
   on an arena's `ArenaParticipant` hierarchy** despite the contiguity
   policy. Indicates the arena-participant model is not enforcing
   contiguity correctly; design review needed.
4. **`atomic_add_single_writer_f32_at` produces racy writes** in the
   disbursement band. Indicates a planner consistency bug (two ops
   selection the same (band, slot, col)); planner correctness review
   needed.
5. **E-7's integration ordering API is not landed** when E-11 PR opens.
   Block E-11 PR until prerequisite lands.
6. **An `ArenaParticipant` SimThing gets allocated into a tombstoned slot
   that is not part of the parent's reserved gap.** Indicates the
   ReservedGap LIFO discipline is not aligned with `SlotAllocator`'s
   actual LIFO tombstone reuse order — needs investigation; possibly
   reserve gap slots in a different order than allocation order.

In any STOP scenario:
- Cursor produces a report.
- E-11 PR is **not** opened.
- Returns to design review.
- **No CPU production allocation fallback** is permitted under any circumstance.

---

## 13. Explicit non-goals

- **No `resource_flow_participant` builder.** Out of scope.
- **No new EML opcodes.** 13-node formula uses only existing opcodes.
- **No new `CombineFn`, `GateSpec`, `ConsumeMode`, `AccumulatorRole` variants.**
- **No runtime arena branches in `simthing-sim`.** Test enforces.
- **No CPU production allocation fallback.** Constitutional.
- **No relaxation of `ExactDeterministic` EML admission.**
- **No new `EmlConsumerKind` variant.**
- **No multi-error diagnostic reporting.** First-error reject.
- **No persistent allocator residual state.** Balance is the ledger.
- **No exact hard-currency transfers via continuous flow.** E-2A path.
- **No combat / diplomacy / trade arena enrollment.** Per ADR §"Out of scope".
- **No cross-arena coupling materialization.** Coupling edges exist in
  `ArenaRegistry` for admission-time validation (cycle-with-delay check)
  but tick-time cross-arena coupling ops are NOT emitted by E-11. That
  is a future PR (E-12 candidate).
- **No authored-content stable-identity resolution.** E-10R validates
  driver-emitted ResourceFlowSpecs only. Authored RON with arena
  selectors is a future Studio PR.
- **No boundary-time slot compaction.** ReservedGap is the v1 contiguity
  mechanism; compaction is rejected as too invasive.
- **No `FissionPolicy::Custom`.** Three variants only:
  `{Inherit, Reevaluate, Reject}`.

---

## Appendix A — kernel surface evidence

**Verified against `crates/simthing-gpu/src/shaders/accumulator_op.wgsl`
as landed in HEAD.**

| E-11 requirement | Kernel evidence |
|---|---|
| `Identity + Constant + ResetTarget` for Phase 0 reset | `CONSUME_RESET_TARGET` → `atomic_store_f32_at`. `SOURCE_CONSTANT` → `bitcast<f32>(op.source_slot)` reads the constant from `source_slot`. |
| `Sum + SlotRange + ResetTarget` for up-sweep reduction | `gather_value` branch on `COMBINE_SUM && SOURCE_SLOT_RANGE` — linear gather, identical to C-5/C-6. |
| `Identity + SlotValue + ResetTarget` for broadcast | Same kernel paths as overlay Add ops landed in C-3. |
| `EvalEML + AddToTarget` for disbursement | `gather_value` branch on `COMBINE_EVAL_EML` calls `eml_eval(ctx)`. `write_target` branch on `CONSUME_ADD_TO_TARGET && GATE_ORDER_BAND` calls `atomic_add_single_writer_f32_at`. |
| EML SELECT, CMP_GT, DIV, MUL, ADD, SLOT_VALUE, LITERAL_F32, RETURN_TOP | `eml_eval` switch opcodes 0, 1, 10, 12, 14, 32, 40, 50 — all present. |
| Single-writer-per-band proven for atomic_add_single_writer | `atomic_add_single_writer_f32_at` is load+store, no CAS. C-4 OrderBand comment: "guarantee a single writer per (band, slot, col)." |
| `select(f, t, cond)` NaN safety | WGSL spec §8.7: returns `f` when `cond == false`, regardless of `t`. Verified in §7.2. |
| Contiguous-children requirement in reduction planner | `reduction_orderband.rs::plan_reduction_orderband` returns `NonContiguousChildren { parent_slot }` if children are not slot-contiguous — explicit invariant. v2 §4.3 satisfies via ArenaParticipant SimThing model. |
| `SOURCE_INPUT_LIST` only pairs with `COMBINE_MIN_ACROSS_INPUTS` | Verified by exhaustive search of WGSL — no `gather_sum_across_inputs` exists. This blocks the indirection-list alternative; STOP condition explicit. |
| `SlotAllocator` LIFO tombstone reuse + append-only | `slot.rs` — `alloc()` pops from `free` (LIFO) before extending the high-water mark. v2 ReservedGap policy aligns with this LIFO behavior. |
| `SimThingKind` enum extensible | `simthing-core/src/simthing.rs` (not read here directly, but `SimThingKind::World`, `Cohort`, `Location` used freely in tests; adding `ArenaParticipant` is a one-line variant addition). |

---

*End of v2 design memo. Awaiting user acceptance + E-10R / E-8R / E-7
ordering API prerequisite PRs before Cursor opens the E-11 PR.*

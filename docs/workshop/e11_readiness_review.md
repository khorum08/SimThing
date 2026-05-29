> Restored during Phase M-JIT-DOC-CLOSEOUT R1 because this file documents stalled E11 / Resource Flow restart evidence and must remain available until E11 is explicitly resumed or superseded.

# E-11 — Final Readiness Review

**Date:** 2026-05-26  
**Status:** Prerequisites **PASS** — E-11 allocation execution **authorized** via narrowed handoff.  
**Authority:** [`e11_hierarchical_allocation_design.md`](e11_hierarchical_allocation_design.md) (Opus v2)  
**Implementation binding:** [`e11_implementation_handoff.md`](e11_implementation_handoff.md)

---

## Verdict

**PROCEED to E-11 allocation execution** using the narrowed handoff. No further prerequisite remedials required.

E-10R3 cleared the gap-block collision that blocked multi-participant arenas with nonzero `reserved_gap_per_intermediate`. The landed stack satisfies Opus v2 substrate constraints for SlotRange reductions, exclusive gap pools, internal column plumbing, and post-allocation integration band placement.

E-11 proper remains **unimplemented** — hierarchy planner, CPU oracle, EML registration, and `e11_*` acceptance tests are the next PR scope, not this review.

---

## Landed prerequisite stack

| ID | Scope | Landed | Tests |
|----|-------|--------|-------|
| **E-10R** | Driver identity preflight + reserved-gap admission sizing | Yes | `e10r_*` (driver), `e10_*` (spec) |
| **E-10R2** | `SimThingKind::ArenaParticipant`, scaffold, index | Yes | `e10r2_*` |
| **E-10R3** | Arena-local gap block + `ResourceFlowSlotOverflow` | Yes | `e10r3_*` |
| **E-8R** | Deterministic arena-internal plumbing columns | Yes | `e8r_*` |
| **E-7R** | `plan_governed_integration_at_band` | Yes | `e7r_*` |
| **E-11 design** | Opus v2 memo accepted | Yes | — |

---

## Review checklist

### 1. ArenaParticipant sibling slots are genuinely contiguous in allocator slot space

**PASS**

- Participants are allocated as consecutive tree children under each arena root; `allocator.alloc` assigns depth-first slots without interleaving gap slots between siblings.
- `arena_participant_sibling_slots` + `slots_are_contiguous` enforce the SlotRange precondition.
- **Evidence:** `crates/simthing-driver/src/arena_participant.rs` (`materialize_arena_participants`, `slots_are_contiguous`); `e10r2_arena_participants_contiguous_at_session_open`; `e10r3_participant_siblings_remain_contiguous_with_reserved_gaps`.

**Limit (E-11 scope):** Scaffold materializes **flat** explicit participants under `R_A`. Nested hierarchy (faction → planet → district) is E-11 `arena_hierarchy.rs` work, not a prerequisite gap.

---

### 2. Reserved gap slots never overlap participant sibling SlotRange

**PASS**

- E-10R3 allocates `N × K` gap slots in a separate HWM block after the contiguous participant sibling block.
- Gap pools are split deterministically per parent; gap slots are not tree children and not in the sibling SlotRange.
- **Evidence:** `SlotAllocator::reserve_exclusive_gap_block`; `e10r3_reserved_gap_pools_do_not_overlap_participant_sibling_range`; `e10r3_each_parent_gets_expected_gap_count`.

---

### 3. Gap consumption cannot disturb the original sibling SlotRange

**PASS**

- `try_alloc_participant_child_in_gap` claims exclusively reserved slots via `claim_exclusive_slot`; sibling participant slots are unchanged.
- **Evidence:** `e10r3_gap_consumption_preserves_sibling_slotrange`; `e10r2_reserved_gap_consumed_before_non_gap_tombstones` (global LIFO tombstones not used when pool non-empty).

**Limit:** Fission path that attaches new `ArenaParticipant` tree nodes under a parent is E-11 Step 7; gap **slot** claim API is ready.

---

### 4. Resource Flow materialization cannot exceed `scenario.n_slots` after arena roots, participants, and gaps

**PASS**

- Install step 4b runs `materialize_arena_participants`, then rejects with `InstallError::ResourceFlowSlotOverflow` when `allocator.capacity() > scenario.n_slots`.
- Capacity check accounts for arena roots, participant nodes, and reserved gap block slots (all extend HWM).
- **Evidence:** `crates/simthing-driver/src/install.rs` (step 4b); `e10r3_resource_flow_materialization_respects_scenario_slot_capacity`.

---

### 5. E-8R internal columns match the Opus v2 column model

**PASS** (plumbing columns; marked roles are authored)

E-8R appends six unmarked internal columns in canonical order when any arena-bound `AccumulatorRole` is present:

| Internal column | E-8R role name |
|-----------------|----------------|
| Up-sweep iF aggregate | `intrinsic_flow_sum` |
| Up-sweep weight aggregate | `weight_sum` |
| Broadcast buffer iF | `propagated_intrinsic_flow` |
| Broadcast buffer aF | `propagated_allocated_flow` |
| Broadcast buffer weight sum | `propagated_weight_sum` |
| Hosted back-reference | `hosted_simthing_id` |

Marked columns (`IntrinsicFlow`, `AllocatedFlow`, `AllocatorWeight`, `Balance`) remain on **author-defined** sub-field names; E-11 planner resolves column indices via `PropertyLayout::offset_of` after `expand_arena_internal_columns`.

**Note:** Opus v2 §0 defect table says "8 columns, 4 marked, 4 unmarked"; §3.1 lifecycle table lists nine logical columns including `hosted_simthing_id`. **Implement §3.1 + E-8R** (4 marked + 6 unmarked plumbing when all roles enrolled).

**Evidence:** `crates/simthing-core/src/arena_layout.rs`; wired in `simthing-spec/src/compile/property.rs`.

---

### 6. E-7R can place balance/flow integration at the exact post-E-11 band

**PASS**

- `plan_governed_integration_at_band(pairs, n_slots, band, participant_filter)` emits `ORDER_BAND` ops with `gate_a == band`.
- Integration band for depth `D`: `D + 2·(D−1) = 3·D − 1` (e.g. D=4 → band 11).
- **Evidence:** `crates/simthing-gpu/src/velocity_accumulator.rs`; `e7r_plan_at_band_places_all_ops_on_requested_band`; `e7r_balance_flow_integration_can_run_after_e11_band`.

**Handoff note:** E-11 driver must discover `GovernedPair` column indices per participant slot from expanded layouts and pass participant slot filter to E-7R.

---

### 7. E-10 admission plus E-10R preflight reject unsafe authored specs before runtime

**PASS** (identity, gap sizing, property-layer role binding)

- **E-10 admission:** arena graph, caps, coupling cycles, `DuplicateArenaRoleBinding`, implicit participation forbidden.
- **E-10R preflight (driver):** unknown participant, slot mismatch, tombstoned participant, `ReservedGapTooSmall`.
- **Evidence:** `resource_flow_admission.rs`, `resource_flow_preflight.rs`, `e10_*`, `e10r_*`.

**Deferred to E-11 plan build (not a prerequisite remedial):**

- `max_orderband_depth ≥ 3·max_depth − 1` band budget check (memo §2.4).
- Participant-layer `AllocatedFlow` / `AllocatorWeight` cardinality per `(arena, participant)` (memo §6.2) — enforce at plan compile with `e11_rejects_missing_*` tests.

---

### 8. `simthing-sim` remains arena-ignorant

**PASS**

- No `ArenaRegistry`, `ArenaParticipant`, or `resource_flow` imports in `simthing-sim`.
- `SimThingKind::ArenaParticipant` is not in `SimThingKindTag`; fission templates cannot spawn arena participants.
- **Evidence:** grep over `crates/simthing-sim`; `e10r2_arena_participant_kind_does_not_cross_into_simthing_sim`.

---

### 9. No new WGSL, new AccumulatorRole variant, CPU allocator fallback, or E-11 execution has landed

**PASS**

- Production WGSL: `accumulator_op.wgsl` + `snapshot.wgsl` only (`crates/simthing-gpu/src/shaders/`).
- `AccumulatorRole` remains four variants; E-8R adds layout columns only.
- No CPU production allocation fallback; no E-11 planner/oracle modules in tree.
- **Evidence:** repo scan; prerequisite PRs #155–#157.

---

## Stop conditions — review pass (none triggered)

| Stop condition | Result |
|----------------|--------|
| SlotRange cannot express reduction groups without compaction | **Not triggered** — sibling contiguity proven (items 1–3) |
| EvalEML cannot express share formula without NaN risk | **Not triggered at review** — kernel `select` + memo §7.2 proof; E-11 must register formula and run `e11_zero_weight_sum_allocates_zero_no_nan` |
| OrderBand budget exceeds arena `max_orderband_depth` | **Not triggered at review** — enforce at E-11 plan build |
| Integration cannot follow allocation via E-7R | **Not triggered** — E-7R band API landed |
| ArenaParticipant scaffold leaks into `simthing-sim` | **Not triggered** |
| Implementation requires new WGSL or CPU production allocator | **Not triggered** |

---

## Known E-11 scope (not prerequisite failures)

These are **expected missing work** for the E-11 PR, documented in the narrowed handoff:

1. `arena_hierarchy.rs` — `ArenaExecutionPlan`, `ArenaBandLayout`, nested `HierarchyNode`
2. `arena_allocation_oracle.rs` — CPU oracle §9.1
3. `arena_allocation_plan.rs` — `Vec<AccumulatorOpGpu>` emitter for §2.3 schedule
4. `child_share_formula` EML registration §7.1
5. Session sync wiring + `use_accumulator_resource_flow` flag (default off)
6. Full `e11_*` acceptance test suite
7. Band budget admission at plan build time
8. Fission → attach `ArenaParticipant` child under parent in SimThing tree (Step 7)

---

## Authorization

| Question | Answer |
|----------|--------|
| Prerequisites complete? | **Yes** |
| Remedial required before E-11? | **No** |
| E-11 allocation authorized? | **Yes** — follow [`e11_implementation_handoff.md`](e11_implementation_handoff.md) |
| May Cursor implement allocation ops in this PR? | **No** — this review is docs-only |

---

## Verification commands (prerequisite regression)

```
cargo test -p simthing-driver e10r2 -- --nocapture
cargo test -p simthing-driver e10r3 -- --nocapture
cargo test -p simthing-driver e10r -- --nocapture
cargo test -p simthing-core e8r -- --nocapture
cargo test -p simthing-gpu e7r -- --nocapture
cargo check --workspace
cargo test --workspace
```

All green at review time (`master` post PR #157).

# E-11B — Nested Hierarchy GPU Readiness Review

**Status:** Accepted (design/readiness review, docs-only)  
**Date:** 2026-05-27  
**Scope:** Post–E-11 flat-star audit for nested hierarchy GPU execution / materialization  
**Authority:** [`e11_hierarchical_allocation_design.md`](../workshop/e11_hierarchical_allocation_design.md) (Opus v2), [`e11_readiness_review.md`](../workshop/e11_readiness_review.md), [`e11_implementation_handoff.md`](../workshop/e11_implementation_handoff.md)

---

## Executive summary

E-11 landed a **production flat-star D=2 GPU vertical slice** with depth-generic planner, CPU oracle, E-7R integration placement, and E-10R3 gap isolation. The **execution substrate is largely ready for D≥3** without new WGSL or new `AccumulatorRole` variants. What is missing is **nested hierarchy materialization** in the production path (`build_execution_plan` always calls `build_flat_star_layout`), **D≥3 GPU parity**, **fission tree wiring** (`refresh_fission_participant_child` exists but is unwired), and **nested burn-in/soak** modes.

**Recommendation:** **Defer E-11B as the default next implementation gate**, but **authorize a narrowed implementation ladder** when nested Resource Flow is an explicit product requirement. E-11B does **not** appear to require new WGSL if per-parent contiguous child slot layout is enforced at materialization time. **E-2B enrollment compilation is higher priority** if dynamic Resource Flow participant growth is on the near-term roadmap; **D-2a boundary transaction scheduling** is higher priority if discrete hard-currency ordering is the immediate production need.

**Next gate depends on product priority:** E-11B implementation ladder, E-2B enrollment compilation, or D-2a discrete scheduling — not all three in parallel.

---

## 1. Current-state audit

| Layer | Flat-star (landed) | Nested (E-11B gap) |
|-------|-------------------|-------------------|
| Spec admission | `ArenaSpec`, `ExplicitParticipantSpec` | Same types; nested topology not expressed in production fixtures |
| Participant scaffold | `materialize_arena_participants` — flat siblings under arena root | No nested `ArenaParticipant` tree at session open |
| Execution layout | `build_flat_star_layout` only in `build_execution_plan` | `build_custom_layout` exists for tests; not wired to SimThing tree |
| Band planner | `ArenaBandLayout::for_depth` — `3·D − 1` bands | Depth-generic; D=2 used in production |
| Op planner | `plan_arena_allocation` — SlotRange Sum reductions | Same API; requires contiguity per parent |
| CPU oracle | `run_arena_allocation_oracle` — tree walk | D=3 tested via hand-built layout |
| GPU path | Session sync + dispatch proven D=2 | No D≥3 GPU parity |
| Fission | Gap claim API (`try_alloc_participant_child_in_gap`) | `refresh_fission_participant_child` unwired |
| Flag | `use_accumulator_resource_flow` default **false** | Unchanged |

**Constitutional posture preserved:** AccumulatorOp v2 substrate; Resource Flow as driver/spec registration over flat AccumulatorOps; `simthing-sim` arena-ignorant; no CPU production allocation fallback; no boundary-time slot compaction; hard-currency transfers separate from Resource Flow; Phase T complete.

---

## 2. Landed E-11 flat-star behavior

Production path (`crates/simthing-driver/src/arena_hierarchy.rs`):

- `build_flat_star_layout` treats first arena-root sibling as depth-0 root; remaining siblings as depth-1 leaves.
- Forces `max_depth = 2` when multiple siblings exist.
- `build_execution_plan` **always** invokes flat-star layout per arena.

Verified by:

- `e11_single_level_positive_weights_cpu_gpu_parity` — D=2 CPU/GPU
- `e11_resource_flow_flag_uploads_and_dispatches_flat_star_ops` — session path
- `e11_burn_in_*`, `e11_burn_in_scenarios_*`, `e11_resource_flow_soak_*` — flat-star only guards (`assert_flat_star_only_no_nested_claims`)
- `ResourceFlowSoakMode::FlatStarOptIn` — opt-in CI soak

Band schedule (Opus v2, implemented):

- Band 0: reset internal + `allocated_flow`
- Bands 1..D−1: up-sweep SlotRange Sum on `intrinsic_flow` / `weight`
- Per level d: broadcast `D+2d`, disburse `D+2d+1`
- Integration: band `3·D − 2` via `plan_governed_integration_at_band` (E-7R)

---

## 3. Missing nested hierarchy materialization

### 3.1 What flat-star v1 does not provide

1. **Multi-level `HierarchyNode` trees** from live SimThing / `ArenaParticipant` topology at session open.
2. **Per-parent contiguous child slot groups** beyond the single arena-root sibling block.
3. **Dynamic hierarchy rebuild** after fission attaches children under intermediate nodes.
4. **Production fixtures** with D≥3 authored arenas (only `build_custom_layout` in unit tests).
5. **GPU verification** for D≥3 conservation and integration-band ordering under nested fanout.

### 3.2 Required for real nested execution

| Work item | Module | Notes |
|-----------|--------|-------|
| Nested tree builder | `arena_hierarchy.rs` | Walk materialized participants; emit `HierarchyNode` forest per arena |
| Layout policy | `arena_participant.rs` + admission | Define slot assignment so each parent's children are contiguous |
| Fission wiring | `arena_participant.rs` + session | Call `refresh_fission_participant_child`; bump registry generation; re-plan |
| Session sync | `arena_allocation_sync.rs` | Unchanged API if plan shape is valid |
| Tests | `e11_arena_allocation.rs` + burn-in | D=3/4 GPU parity, nested fission, reserved-gap preservation |

---

## 4. SlotRange / contiguity proof requirements

### 4.1 Semantics (landed)

`SourceSpec::SlotRange { start, count, col }` gathers linear slot span `[start, start+count)` at column `col`. Used with `CombineFn::Sum` + `ResetTarget` for up-sweep reductions.

Planner derives range from direct children only:

```rust
// arena_allocation_plan.rs — child_range(parent)
start = parent.children[0].participant_slot
count = parent.children.len()
```

### 4.2 Can SlotRange express nested reductions without compaction?

**Yes, per parent group** — each interior node needs its direct children contiguous in slot space. Nested hierarchy does **not** require global slot compaction if:

- Each parent's children occupy a contiguous block (local contiguity).
- Gap-allocated fission children remain in the **arena-local gap block** (E-10R3), separate from sibling SlotRange under arena root.
- Fission children that must participate in a parent's reduction are either (a) pre-admitted into the contiguous sibling block, or (b) excluded from that parent's SlotRange until a hierarchy rebuild compacts/reassigns — **policy choice for E-11B-1**.

**No** — if a parent mixes arena-root siblings with gap-block children in the same reduction group without contiguous slot assignment. Opus v2 rejected indirection lists and boundary compaction; E-11B must enforce layout at materialization, not runtime repair.

### 4.3 Proof obligations before implementation

1. Document and test **nested admission slot assignment** algorithm (depth-first contiguous subtrees vs per-level blocks).
2. Prove `HierarchyNode::verify_child_contiguity` passes for all interior nodes in authored D=3/D=4 fixtures.
3. Prove E-10R3 gap block remains outside every participant sibling SlotRange after nested admission.
4. Prove fission gap claim does not insert slots into an active SlotRange (extend `e11_reserved_gap_fission_preserves_slotrange` to nested parents).

---

## 5. OrderBand budget review

| Depth D | Total bands | Integration band | Deepest disburse |
|---------|-------------|------------------|------------------|
| 2 | 5 | 4 | 3 |
| 3 | 8 | 7 | 6 |
| 4 | 11 | 10 | 9 |

- `ArenaBandLayout::for_depth` and `max_disbursement_band` are **depth-generic**.
- `e11_orderband_depth_budget_enforced` rejects plans exceeding `max_orderband_depth` on game mode.
- `e11_integration_band_immediately_follows_deepest_disbursement` — **D=2 fixture only**; D=3 band math unit-tested (`band_layout_d3_integration_follows_deepest_disburse`).

**E-11B requirement:** Extend integration-ordering test to D=3/D=4 before GPU parity. No new bands or WGSL expected if D fits within authored `max_orderband_depth`.

---

## 6. CPU oracle and GPU parity gaps

### 6.1 CPU oracle — nested hierarchy

**Yes — depth-generic and tree-structured.**

`run_arena_allocation_oracle` (`arena_allocation_oracle.rs`):

- Phase 0: reset all nodes
- Phase 1: up-sweep deepest interior first
- Phase 2: down-sweep broadcast/disburse per depth
- Phase 3: integrate balance on all nodes

`e11_multi_level_hierarchy_cpu_oracle_parity` validates D=3 via **`build_custom_layout`** with hand-assigned slots — not scaffold materialization.

### 6.2 GPU parity gaps

| Test | Status |
|------|--------|
| D=2 CPU/GPU parity | **Landed** |
| D=3 CPU oracle | **Landed** (custom layout) |
| D=3 CPU/GPU parity | **Missing** — explicit E-11B gate |
| D≥3 session-path dispatch | **Missing** |
| Nested burn-in / soak | **Missing** (`FlatStarOptIn` only) |

---

## 7. Fission / reserved-gap implications

### 7.1 E-10R3 behavior (landed, valid for nested)

- Participant siblings: contiguous block under arena root.
- Gap block: `N × K` slots via `reserve_exclusive_gap_block` **after** sibling block.
- Per-parent pools: non-overlapping slices; LIFO consume.
- Gap slots **do not** overlap arena-root sibling SlotRange.

**Valid under nested fission** provided gap children are not counted in a parent's SlotRange until hierarchy rebuild defines them as contiguous participants (or intermediate policy excludes them from reductions).

### 7.2 Unwired fission path

`refresh_fission_participant_child` exists in `arena_participant.rs` but is **never called** from session/install/fission hooks. E-11B must wire:

1. Gap claim on fission.
2. SimThing tree attachment under correct parent.
3. `ArenaRegistry` / execution plan refresh (generation-keyed pattern analogous to T-4 resource economy).

### 7.3 ArenaParticipant scaffold and nested layout

**Partial support:**

| Capability | Nested-ready? |
|------------|---------------|
| Flat explicit participants under arena root | Yes |
| Contiguous sibling block | Yes |
| Reserved gap pools per parent | Yes |
| Nested SimThing parent/child at admission | **No** — all participants are arena-root children |
| Build `HierarchyNode` from tree | **No** — production uses flat-star only |

**Conclusion:** Scaffold supports **flat admission + gap isolation**. Nested non-flat layout requires **new materialization policy**, not a new scaffold type.

---

## 8. E-7R integration placement

`plan_governed_integration_at_band` (E-7R) emits `IntegrateWithClamp` on `GateSpec::OrderBand(integration_band)`.

`plan_arena_allocation` passes:

- `integration_band = 3·D − 2`
- Optional `participant_filter` restricting to arena participant slots

**Can still run after deepest disbursement for D≥3:** Yes — band math is depth-generic; integration band is defined as deepest disburse + 1. **Needs D=3+ integration-ordering test on GPU path before sign-off.**

---

## 9. Stop conditions

Do **not** implement E-11B if any of the following would be required:

| Stop | Assessment |
|------|------------|
| New WGSL kernel | **Not required** for nested SlotRange + OrderBand sweep on existing AccumulatorOp execute pipeline |
| New `AccumulatorRole` variant | **Not required** — same ResetTarget / Sum / EvalEML / IntegrateWithClamp roles |
| CPU production allocation fallback | **Reject** — CPU oracle remains test-only |
| Boundary-time slot compaction | **Reject** — layout must be correct at materialization |
| Default-on `use_accumulator_resource_flow` | **Reject** — remain opt-in per scenario |
| Hard-currency through Resource Flow | **Reject** — Phase T path separate |
| `simthing-spec` in `simthing-sim` | **Reject** |

**Conditional stop:** If nested admission cannot guarantee per-parent child contiguity without indirection lists or compaction, **halt E-11B** and revisit Opus v2 layout ADR before any GPU work.

---

## 10. Priority vs E-2B and other gates

| Gate | Blocker / dependency | Production value | Substrate risk |
|------|---------------------|------------------|----------------|
| **E-11B nested GPU** | None technical; product must need D≥3 Resource Flow | Nested continuous allocation scenarios | Low if layout policy lands first; no WGSL |
| **E-2B enrollment compilation** | **Blocked** — enrollment compile not in scope | Dynamic `resource_flow_participant` growth | Medium — new compile path |
| **D-2a discrete scheduling** | D-1 memo landed | Cross-band hard-currency ordering | Low — driver-only |
| **Phase T follow-on** | Complete | Transfer/emission opt-in burn-in | None |

**Priority recommendation:**

1. **If dynamic enrollment is on the roadmap** → **E-2B enrollment compilation first** (unblocks participant growth; E-11B nested trees are more valuable once enrollment can add participants).
2. **If discrete treaty/construction ordering is urgent** → **D-2a first** (D-1 recommendation).
3. **If authored nested Resource Flow scenarios are required now** → **Proceed with narrowed E-11B ladder below**.
4. **Otherwise** → **Keep E-11B deferred**; continue flat-star opt-in soak and Phase T default-off posture.

E-11B is **not higher priority than E-2B** when dynamic enrollment is a prerequisite for realistic nested scenarios. E-11B is **higher priority than D-2** for continuous-flow nested allocation product goals.

---

## 11. Recommended implementation ladder (if E-11B authorized)

| Step | Scope | Deliverable |
|------|-------|-------------|
| **E-11B-1** | Nested hierarchy materialization in `simthing-driver` | `build_nested_layout` from SimThing tree; layout policy doc; contiguity proofs |
| **E-11B-2** | Nested execution plan using existing AccumulatorOp bands | Wire `build_execution_plan` to nested builder; flag-gated |
| **E-11B-3** | D=3 / D=4 CPU–GPU parity tests | `e11_multi_level_hierarchy_cpu_gpu_parity`; session dispatch |
| **E-11B-4** | Nested fission + reserved-gap preservation | Wire `refresh_fission_participant_child`; hierarchy rebuild tests |
| **E-11B-5** | Docs + nested burn-in update | `ResourceFlowSoakMode::NestedOptIn`; production plan E-11B Done |

**This PR does not implement the ladder.**

---

## 12. Tests required before implementation

| Test | Purpose |
|------|---------|
| `e11_nested_layout_materialization_contiguous_per_parent` | Scaffold → `HierarchyNode` tree; all interior nodes pass `verify_child_contiguity` |
| `e11_multi_level_hierarchy_cpu_gpu_parity` | D=3 minimum; GPU readback vs oracle |
| `e11_d4_band_integration_ordering` | Integration band = deepest disburse + 1 |
| `e11_nested_reserved_gap_fission_preserves_slotrange` | Gap claim under intermediate parent |
| `e11_nested_fission_wires_refresh_participant_child` | Session hook calls refresh API |
| `e11_nested_orderband_budget_d3_d4` | Reject over-budget depth on game mode |
| `e11_nested_burn_in_conservation` | 100-tick nested fixture; opt-in flag |
| `e11_nested_soak_opt_in` | Extend soak mode; no flat-star-only guard |

Existing flat-star tests remain regression baseline; do not remove `assert_flat_star_only_no_nested_claims` until nested mode is explicit.

---

## 13. Verdict

| Question | Answer |
|----------|--------|
| What is missing from E-11 flat-star v1? | Nested materialization, D≥3 GPU parity, fission wiring, nested burn-in/soak |
| Required for nested GPU execution? | Nested layout builder + per-parent contiguity policy + E-11B-3 parity tests |
| ArenaParticipant supports nested layout? | **Partially** — flat admission + gaps yes; nested tree materialization no |
| SlotRange without compaction? | **Yes per parent** if materialization enforces local contiguity |
| E-10R3 valid under nested fission? | **Yes** if gap slots stay outside active SlotRanges |
| E-7R after deepest disbursement? | **Yes** — depth-generic; needs D≥3 GPU test |
| CPU oracle models nested? | **Yes** — D=3 tested with custom layout |
| Tests before implementation? | See §12 |
| E-11B vs E-2B priority? | **E-2B first** if dynamic enrollment needed; **E-11B first** if static nested scenarios are the product target |

**Final recommendation:** **Defer E-11B as default next gate.** **Authorize narrowed E-11B-1…B-5 ladder** when nested Resource Flow is explicitly prioritized. **No production code in this review.** No constitutional stop conditions triggered.

---

## References

- [`e11_hierarchical_allocation_design.md`](../workshop/e11_hierarchical_allocation_design.md)
- [`e11_readiness_review.md`](../workshop/e11_readiness_review.md)
- [`e11_implementation_handoff.md`](../workshop/e11_implementation_handoff.md)
- [`d1_discrete_transaction_contention_memo.md`](d1_discrete_transaction_contention_memo.md)
- Code: `arena_hierarchy.rs`, `arena_participant.rs`, `arena_allocation_plan.rs`, `arena_allocation_oracle.rs`, `velocity_accumulator.rs` (E-7R)

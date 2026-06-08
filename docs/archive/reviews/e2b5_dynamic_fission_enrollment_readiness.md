# E-2B-5 — Dynamic Fission Enrollment Readiness Review

**Status:** Accepted (design/readiness review, docs-only)  
**Date:** 2026-05-27  
**Scope:** Post–E-2B static enrollment audit for fission-spawned Resource Flow arena enrollment  
**Authority:** [`e2b_resource_flow_enrollment_compilation_readiness.md`](e2b_resource_flow_enrollment_compilation_readiness.md), [`e11b_nested_hierarchy_gpu_readiness_review.md`](e11b_nested_hierarchy_gpu_readiness_review.md), [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md), [`e11_implementation_handoff.md`](../workshop/e11_implementation_handoff.md)

---

## Executive summary

E-2B static enrollment (E-2B-1…4) is **landed**. E-2B-5 dynamic fission enrollment is the **boundary-time path** that enrolls fission-spawned hosted SimThings into Resource Flow arenas after session open, without manual `ExplicitParticipantSpec` authoring or full session rebuild.

**Critical flat-star finding:** `try_alloc_participant_child_in_gap` / `refresh_fission_participant_child` place new `ArenaParticipant` nodes **under the parent participant** in the **E-10R3 gap block**, which is **outside** the arena-root **participant sibling SlotRange** used by E-11 flat-star. Gap enrollment preserves sibling contiguity (proven by `e11_reserved_gap_fission_preserves_slotrange`) but **does not add the fission child to flat-star leaf disbursement**. For flat-star allocation participation, v1 must **append a new arena-root sibling participant** with a contiguous slot extension — not route through the gap primitive alone.

**Policy choice (v1):** **Policy A — inherit-only dynamic enrollment** with **flat-star-safe append semantics**. Parent arena membership is inherited; fission child is admitted as a new arena-root sibling when capacity and contiguity allow. **`Reevaluate` selector re-run is deferred (Policy B).** **`Reject`** when cap or contiguity extension fails.

**E-11B:** **Not required** for Policy A flat-star append path. **Required** if product wants gap-block nested participant children to receive hierarchical allocation disbursement (E-11B scope).

**Recommendation:** **Implement E-2B-5 with narrowed Policy A ladder** (driver hook + arena-root append + registry/scaffold/sync). Defer Policy B (`Reevaluate`) and gap-only enrollment for allocation. **Cursor implementable** without Opus unless product demands Policy B selector semantics at fission time.

---

## 1. Current-state audit

| Layer | Static E-2B (landed) | E-2B-5 gap |
|-------|------------------------|------------|
| Enrollment at session open | `resolve_resource_flow_enrollment` + install §4b | **None** at fission boundary |
| Fission in sim | `BoundaryProtocol` spawns child SimThing; driver `react_to_fission_clones` for capability trees | No Resource Flow hook |
| Gap primitive | `try_alloc_participant_child_in_gap`, `refresh_fission_participant_child` in `arena_participant.rs` | **Unwired** |
| ArenaRegistry | `refresh_subtree` bumps generation only; **no `admit_participant` at runtime** | No dynamic admit API |
| E-11 flat-star | `build_flat_star_layout` — arena-root sibling block only | New gap slots **excluded** from sibling SlotRange |
| E-10R3 | Gap block isolated from participant siblings | Preserved if gap path used |
| Replay | Fission lineage in `BoundaryProtocol` / replay driver | No Resource Flow enrollment events |
| Flag | `use_accumulator_resource_flow` default **false** | Unchanged |

**Constitutional posture preserved:** AccumulatorOp v2 substrate; driver/session artifacts; no WGSL; no new `AccumulatorRole`; no CPU production fallback; no boundary slot compaction; `simthing-sim` arena-ignorant; hard-currency separate from Resource Flow; E-11B deferred by default.

---

## 2. Dynamic enrollment definition

**E-2B-5 dynamic fission enrollment** is the driver/session behavior that, on a successful fission boundary outcome:

1. Identifies fission-spawned **hosted SimThings** (world-tree children, not capability clones only).
2. Determines which **Resource Flow arenas** the **parent** was enrolled in at session open.
3. **Admits** the child to those arenas (subject to `FissionPolicy`, caps, contiguity).
4. Updates **`ArenaParticipantScaffold`**, **`ArenaRegistry`**, and (when flag enabled) **re-syncs E-11 flat-star ops**.

It is **not** static selector resolution at install (E-2B-2). It is **not** E-11B nested hierarchy materialization. It is **not** default-on Resource Flow.

---

## 3. Fission policy semantics

| Policy | E-2B-5 v1 participation | Rationale |
|--------|-------------------------|-----------|
| **Inherit** | **Primary v1 path.** Child inherits parent's arena set; admit if cap/contiguity allow. | Deterministic; no selector re-run; matches Policy A default. |
| **Reject** | Do not enroll child; log/report rejection. | Already used for gap exhaustion in `try_alloc_participant_child_in_gap`. |
| **Reevaluate** | **Deferred (Policy B).** Would re-run `EnrollmentSelectorSpec` for child subtree. | Selector scope, dedup, and replay ordering risk; defer until Opus/product signs Policy B. |

**Authored default today:** `FissionPolicySpec::Reevaluate` on `ArenaSpec`. **E-2B-5 v1 implementation should treat enrolled arenas as `Inherit` for fission enrollment** until Policy B lands, **or** map `Reevaluate` → inherit-only subset (documented constant) to avoid silent selector runs.

---

## 4. Gap-slot allocation and contiguity model

### 4.1 What E-10R3 gap primitive does

- `materialize_arena_participants` reserves per-parent gap pools in the **arena-local gap block** (separate from arena-root sibling span).
- `try_alloc_participant_child_in_gap` pops from pool; on exhaustion with `Reject`, errors; with `Inherit`/`Reevaluate`, falls back to `allocator.alloc()` (**contiguity risk**).
- `refresh_fission_participant_child` attaches `ArenaParticipant` **under parent participant node**, not under arena root.

### 4.2 Flat-star constraint

`build_flat_star_layout` uses `arena_participant_sibling_slots(root, arena_root_id)` — **direct children of arena root only**. Gap slots are **provably outside** active sibling SlotRange (`e11_reserved_gap_fission_preserves_slotrange`).

### 4.3 v1 contiguity model (Policy A append)

For **flat-star allocation participation**:

1. Append new `ArenaParticipant` as **child of arena root** (same as session-open materialization).
2. Allocate slot **immediately after** last sibling in contiguous block (`last_sibling + 1` if contiguous extension holds; else **Reject**).
3. **Do not** insert gap-slot children into sibling SlotRange.
4. Preserve E-10R3 gap block unchanged for future E-11B nested fission.

**Stop condition preserved:** no boundary-time slot compaction; no indirection lists; gap stays outside sibling SlotRange.

---

## 5. ArenaRegistry generation / sync model

### 5.1 Current API gaps

- `ArenaRegistry::refresh_subtree(changed_root)` — generation bump only; no participant admit/remove.
- No `admit_participant_runtime(arena_idx, slot, subtree_root)` on live registry.
- `ArenaParticipantScaffold` has no public helper to append arena-root sibling + update `ArenaParticipantIndex`.

### 5.2 Required E-2B-5 sync sequence (after successful admit)

```
Boundary fission outcome (driver)
  → detect parent arena memberships (scaffold.index / registry.participants)
  → append arena-root participant (Policy A) OR reject
  → registry.admit_participant + generation++
  → scaffold.index/report update
  → if use_accumulator_resource_flow:
        build_execution_plan (flat-star)
        sync_resource_flow_accumulator
```

Mirror existing **`react_to_fission_clones`** placement in `SimSession` boundary handler — sibling hook `react_to_fission_resource_flow_enrollment`.

### 5.3 OrderBand depth

Adding a flat-star leaf may increase `max_depth` only when sibling count goes from 1→2+ (already D=2). Adding leaf **n+1** to existing D=2 star **does not** increase band depth — still D=2. Re-sync must re-validate `max_orderband_depth` cap (unchanged band count).

---

## 6. Replay determinism model

### 6.1 Requirements

- Fission enrollment decisions must be **pure functions** of: boundary outcome (parent/child ids), pre-fission registry/scaffold, arena `FissionPolicy`, caps, and slot allocator state.
- **No** wall-clock, random, or iteration-order-dependent admission.
- Replay must reconstruct same participant set: either **record enrollment in spec/boundary snapshot** or **re-derive from fission lineage + static enrollment** deterministically.

### 6.2 Gap vs replay today

- `FissionLineageRecord` in replay driver reconstructs spawned children.
- Resource Flow has **no** enrollment delta in replay snapshot today.

### 6.3 E-2B-5f test obligations

- Record/replay boundary with fission + Resource Flow flag opt-in → same `arena_registry.participants` and allocation oracle totals at tick N.
- Bit-exact for `ExactDeterministic` paths where applicable (mirror T-5 / E-11 burn-in patterns).

---

## 7. Relationship to E-11 flat-star and E-11B

| Question | Answer |
|----------|--------|
| Can flat-star consume updated participant set without E-11B? | **Yes**, if new participants are **arena-root siblings** with contiguous slot extension. |
| Does gap-only enrollment work with flat-star allocation? | **No** — gap children excluded from sibling SlotRange. |
| When is E-11B required? | Nested participant trees, gap-block children in reductions, D≥3 GPU parity. |
| E-11B default | **Remain deferred** unless product prioritizes nested allocation over flat-star fission leaves. |

---

## 8. Stop conditions

| Stop condition | E-2B-5 posture |
|----------------|----------------|
| New WGSL | **Forbidden** |
| New `AccumulatorRole` variants | **Forbidden** |
| CPU production allocation fallback | **Forbidden** |
| Boundary-time slot compaction | **Forbidden** |
| Indirection-list SlotRange | **Forbidden** |
| `simthing-sim` arena awareness | **Forbidden** |
| Default-on Resource Flow | **Forbidden** |
| Gap-only path required for flat-star allocation | **Stop** — use append path or defer |
| Policy B without Opus review | **Defer** |

---

## 9. Recommended implementation ladder

| Step | Scope | Notes |
|------|-------|-------|
| **E-2B-5a** | Policy constants + docs: v1 = Inherit-only; Reevaluate deferred | Map authored `Reevaluate` → inherit for fission enrollment until Policy B |
| **E-2B-5b** | `react_to_fission_resource_flow_enrollment` in `SimSession` — detect parent memberships from fission outcome | Parallel to `react_to_fission_clones` |
| **E-2B-5c** | Arena-root sibling append + contiguous slot check ( **not** gap-only for flat-star) | Reuse materialization patterns from `materialize_arena_participants` |
| **E-2B-5d** | `ArenaRegistry` runtime admit + scaffold index update | Extend builder or add narrow runtime API |
| **E-2B-5e** | Generation bump + `sync_resource_flow_if_enabled` on boundary | Only when flag opt-in |
| **E-2B-5f** | Replay + 100-tick burn-in with fission fixture | Opt-in flag; conservation oracle |

**Defer:** Policy B selector re-run; gap-only enrollment for allocation; E-11B nested wiring.

---

## 10. Required tests (before E-2B-5 landed)

### 10.1 Unit / driver

- Parent enrolled in two arenas → fission child admitted to both (inherit).
- `max_participants` exceeded → reject child admission.
- Contiguity extension impossible → reject (no compaction).
- Gap pool untouched when using append path.
- Registry generation increments; untouched arenas unchanged.

### 10.2 Integration

- Fission during session with `use_accumulator_resource_flow` opt-in → re-sync uploads ops including new leaf.
- Flag default false → enrollment scaffold/registry may update but no GPU sync requirement beyond existing flag-off behavior.
- `e11_reserved_gap_fission_preserves_slotrange` regression remains green.

### 10.3 Replay / burn-in

- Fission + 100-tick flat-star conservation with dynamic enrollment.
- Replay from snapshot reproduces participant count and final allocated flows.

---

## 11. Docs update requirements (post-implementation)

- `docs/accumulator_op_v2_production_plan.md` — E-2B-5 status, Policy A semantics
- `docs/todo.md`, `docs/worklog.md`, `docs/workshop/workshop_current_state.md`
- Cross-link from [`e2b_resource_flow_enrollment_compilation_readiness.md`](e2b_resource_flow_enrollment_compilation_readiness.md) §7

---

## 12. Final recommendation

| Verdict | Detail |
|---------|--------|
| **Implement** | **Yes — narrowed Policy A (inherit + arena-root append)** |
| **Defer** | Policy B (`Reevaluate` selector); gap-only allocation path; E-11B |
| **Opus review** | **Optional** — only if product requires Policy B or gap-nested allocation semantics |
| **E-11B required?** | **No** for flat-star fission leaf append; **yes** for gap-nested disbursement |

### Review question answers (summary)

| # | Answer |
|---|--------|
| 1 | Boundary-time admit fission child to parent's arenas with registry/scaffold/sync update |
| 2 | **Inherit** v1; **Reject** on failure; **Reevaluate** deferred |
| 3 | All arenas where parent hosted SimThing is enrolled |
| 4 | **Inherit** — no selector re-run in v1 |
| 5 | **Arena-root sibling append** for flat-star; gap primitive for nested (E-11B) only |
| 6 | **Partially** — correct for E-10R3 nested topology, **insufficient alone** for flat-star leaves |
| 7 | Runtime admit + `generation++` + `sync_resource_flow_if_enabled` |
| 8 | **Yes** with append path; **no** with gap-only path |
| 9 | Gap exhausted → `Reject` (or defer enrollment); append path → reject if cap/contiguity fail |
| 10 | Deterministic function of fission outcome + registry state; replay tests required |
| 11 | **No** new WGSL / roles / CPU fallback / simthing-sim awareness |
| 12 | Ladder E-2B-5a…f above |

**Next gate depends on product priority:** E-2B-5 implementation, E-11B, D-2a, Resource Flow default-on, or deferral.

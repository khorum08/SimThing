# E-11B — Nested Dynamic Enrollment Readiness Review

**Status:** Accepted (design/readiness review, docs-only)  
**Date:** 2026-05-27  
**Scope:** Post–E-11B kickoff / E-11B-4 fission/gap hardening audit for whether E-11B should proceed to nested dynamic enrollment  
**Audience:** Cursor implementation handoff, GPT review, production plan maintainers  
**Prior art:** [`e11b_nested_hierarchy_gpu_readiness_review.md`](e11b_nested_hierarchy_gpu_readiness_review.md), [`d2a_boundary_transaction_scheduling_readiness.md`](d2a_boundary_transaction_scheduling_readiness.md), [`resource_flow_limited_scenario_class_posture.md`](../resource_flow_limited_scenario_class_posture.md)

---

## Executive summary

E-11B kickoff landed **static nested D=3/D=4 GPU parity** over existing AccumulatorOp v2 OrderBand bands. E-11B-4 hardened **reserved-gap preservation**: gap-resident fission children stay outside active child `SlotRange`, do not become allocation leaves by accident, and attaching them to nested topology without admission **rejects** with `HierarchyError::NonContiguousChildren` — no slot compaction.

Flat-star **Policy A dynamic fission enrollment** (E-2B-5) remains the only landed dynamic Resource Flow path: arena-root sibling append with preflight/commit atomicity. It does **not** generalize to nested interior parents without a new, narrowly scoped admission gate.

**Finding:** A **constitutionally safe nested dynamic enrollment v1** is feasible **without** Policy B Reevaluate, selector re-run, wildcard expansion, new WGSL, new `AccumulatorRole` variants, CPU fallback, slot compaction, or indirection-list `SlotRange` — **if and only if** v1 is defined as **explicit nested admission under an already-enrolled parent that preserves per-parent child contiguity or rejects visibly**. It is **not** “every gap child becomes an allocation leaf on fission.”

**Recommendation: B — defer nested dynamic enrollment implementation** until a **named product scenario** requires nested Resource Flow growth beyond static materialization. If authorized, proceed via the **narrow E-11B-5 ladder** (docs/policy → preflight → commit → generation/sync → parity → replay/burn-in). **Do not send to Opus** unless requirements expand to Policy B, selector re-run, compaction, or substrate redesign.

| Option | Verdict |
|--------|---------|
| **A. Implement narrow nested dynamic enrollment now** | **Not recommended by default** — no concrete nested dynamic product scenario blocked today |
| **B. Defer nested dynamic enrollment** | **Recommended** — static nested + fission/gap hardening sufficient for current posture |
| **C. Send to Opus** | **Not required** — narrow v1 fits existing AccumulatorOp substrate |

**Next gate depends on review recommendation:** narrow E-11B nested dynamic enrollment (only after product names a scenario), pause E-11B, narrow D-2a, or simthing-spec/RON rebuild.

---

## 1. Current-state audit after E-11B-4

| Layer | E-11B kickoff (landed) | E-11B-4 (landed) | Nested dynamic enrollment (gap) |
|-------|------------------------|------------------|--------------------------------|
| Static nested layout | `build_nested_layout` wired when `has_nested_participants` | Unchanged | No runtime growth path |
| GPU parity | D=3/D=4 CPU/GPU parity (`e11b_nested_hierarchy_gpu`) | Parity preserved after safe gap claim | No dynamic admission tests |
| Gap pools | E-10R3 reserved per interior parent | `reserve_gap_pools_for_parent_slots`, diagnostics | Gap claim ≠ layout admission |
| Fission attach | `refresh_fission_participant_child` (gap slot + tree child) | Proven gap stays outside `SlotRange`; non-admitted attach breaks contiguity → reject | Unwired to session dynamic enrollment |
| Flat-star dynamic | E-2B-5 Policy A arena-root append | Flat-star regression unchanged | Different topology target |
| Execution plan | `build_execution_plan` chooses nested vs flat-star | `NonContiguousChildren` without compaction | No `prepare_*` / `commit_*` nested gate |
| Registry generation | Bumps on E-2B-5 commit | Snapshot/replay helpers for gap pools | No nested admission generation model |
| Flag / posture | `use_accumulator_resource_flow` default **false** | Confirmed in tests | Unchanged |

**Test evidence (inspected):**

- `e11b_nested_hierarchy_gpu.rs` — 12/12 PASS (D=3/D=4 static nested GPU parity)
- `e11b_nested_fission_gap.rs` — 13/13 PASS (gap preservation, contiguity rejection, replay)
- `e2b5_dynamic_fission_enrollment.rs` — Policy A flat-star dynamic enrollment
- `e2b5_dynamic_enrollment_soak.rs` — atomicity + resync under soak

**Constitutional posture preserved:** AccumulatorOp v2 substrate; Resource Flow as driver/spec registration; `simthing-sim` arena-ignorant; no CPU production allocation fallback; no boundary-time slot compaction; hard-currency transfer separate from Resource Flow; Phase T complete; `FlatStarResourceFlow` remains bounded production posture.

---

## 2. Definition of nested dynamic enrollment

### 2.1 What it means after E-11B kickoff + E-11B-4

**Nested dynamic enrollment** is the driver/session mechanism by which a **new Resource Flow allocation participant** joins an **already-enrolled nested hierarchy** after session open — typically in response to fission — while:

1. Preserving **per-parent contiguous child `SlotRange`** semantics for all active interior nodes.
2. Keeping **E-10R3 reserved-gap slots** outside active child spans until (if ever) explicitly admitted.
3. Bumping **`ArenaRegistry` generation** and triggering **Resource Flow re-sync** only on successful commit.
4. Remaining **explicit and opt-in** — not implied by `ResourceFlowSpec` presence alone.

It is **not** static nested materialization at session open (E-11B kickoff). It is **not** gap-only fission topology attach (`refresh_fission_participant_child`) which E-11B-4 treats as non-participating in allocation layout.

### 2.2 Safe v1 definition (recommended if implementation is authorized)

> **Nested dynamic enrollment v1** = explicit admission of a fission-spawned (or otherwise eligible) child under an **already-enrolled nested interior or arena-root parent**, using a **preflight → commit** gate that either extends the parent's **active allocation children** with **contiguous slot assignment** or **rejects with no partial mutation**.

v1 **does not** mean:

- Re-running E-2B enrollment selectors at boundary time
- Policy B Reevaluate (re-plan all weights on topology change)
- Wildcard / dynamic selector expansion
- Automatic conversion of every gap-pool child into an allocation leaf
- Slot compaction or indirection-list `SlotRange`
- Global Resource Flow default-on

### 2.3 Relationship to flat-star Policy A (E-2B-5)

| Aspect | Flat-star Policy A | Nested dynamic enrollment v1 |
|--------|-------------------|-------------------------------|
| Parent target | Arena-root sibling block | Nested interior parent (or arena root in nested tree) |
| Slot strategy | `try_alloc_contiguous_after` last arena-root sibling | Same **pattern**, scoped to parent's **active child** contiguous block |
| Gap pool | Not used for allocation leaves | Gap remains **non-leaf** until explicit nested admission |
| Session hook | `react_to_fission_resource_flow_enrollment` | Requires **new** nested-scoped react or branch |
| Layout impact | Extends flat-star D=2 leaves | Extends nested `HierarchyNode` subtree; re-validates contiguity |

---

## 3. Gap child vs active allocation child semantics

| Concept | Tree / scaffold | Hierarchy layout | SlotRange participation |
|---------|-----------------|------------------|-------------------------|
| **Active allocation child** | `ArenaParticipant` in scaffold index; in parent's SimThing children | Present as `HierarchyNode` leaf (or interior) under parent | Included in parent's `active_child_slots()` / reduction `SlotRange` |
| **Reserved-gap child (topology only)** | May exist in SimThing tree after `refresh_fission_participant_child`; slot from gap pool | **Excluded** from `HierarchyNode` allocation tree | **Outside** active child span and arena-root sibling range |
| **Explicitly admitted nested child** | Same as active child after commit | Added to layout on next `build_execution_plan` | **Inside** parent's contiguous active child block |

**E-11B-4 proof:** `e11b_nested_gap_child_does_not_become_allocation_leaf` — gap claim without admission does not change allocation leaves. `e11b_nested_reserved_gap_child_stays_outside_active_child_slotrange` — gap slots stay outside active span.

**Answer to review Q6:** A gap child does **not** become an allocation leaf automatically. Only **explicit nested admission** (future E-11B-5 gate) may promote a child into the active allocation layout — and only when contiguity can be preserved.

**Critical nuance:** E-10R3 gap slots live in an **exclusive gap block**, typically **not** contiguous with the parent's active child span. Therefore **activating an existing gap-slot child in-place** into `SlotRange` will **break contiguity** (proven by `e11b_nested_rejects_noncontiguous_active_children_without_compaction`). v1 admission must therefore prefer **contiguous extension after the last active child** (new slot) or **reject**, not blind promotion of arbitrary gap slots.

---

## 4. Feasibility without Policy B

### 4.1 Review question: Can nested dynamic enrollment avoid Policy B Reevaluate?

**Yes, for v1.** Policy B implies re-evaluating allocation weights and re-running reduction semantics across the tree when topology changes. v1 can mirror E-2B-5 Policy A:

- Admit child with inherited/default weight columns already seeded at participant creation.
- Re-plan AccumulatorOp bands from updated layout on next sync.
- Do **not** re-run enrollment selectors or wildcard expansion.

**When Policy B would be required:** Product demands **dynamic re-weighting** or **selective re-enrollment** of existing participants when nested topology changes. That is **out of v1 scope** and would trigger **Opus escalation** if mandatory.

### 4.2 Review question: Selector re-evaluation required?

**No for v1.** Static E-2B enrollment resolves selectors at session install. Flat-star dynamic enrollment uses **`FissionOutcome` pairs** directly — no selector re-run. Nested v1 should use the same **pair-driven admission** (or an explicit driver API), scoped to nested parents already in the registry.

Selector re-run or wildcard expansion → **Opus stop condition**.

---

## 5. Feasibility without compaction / indirection SlotRange

### 5.1 Can parent active child contiguity be preserved?

**Yes, by rejection.** E-11B-4 established the enforcement model:

- `HierarchyNode::verify_child_contiguity` / `build_execution_plan` → `HierarchyError::NonContiguousChildren`
- No boundary-time slot compaction (constitutional reject)
- No indirection-list `SlotRange` (Opus v2 reject)

**Admission rule for v1:** `prepare_nested_gap_activation` (or equivalent) must verify that the post-admission active child slots form a contiguous range **before** commit. If the only available slot is a non-contiguous gap-block slot, **reject visibly** — same as E-2B-5R atomicity discipline.

### 5.2 Can nested dynamic enrollment work without compaction?

**Yes, narrowly:** allocate **next contiguous slot after last active child** under the nested parent (parallel to `prepare_dynamic_arena_root_append`), register in scaffold/registry, append to SimThing tree, rebuild layout. Gap pool may still hold **other** fission children that remain non-leaves.

**No, if product requires:** inserting children into the middle of a slot range, reordering siblings, or promoting non-adjacent gap slots — those require compaction or indirection → **halt / Opus**.

---

## 6. Registry generation and sync model

### 6.1 Required behavior after nested admission (v1)

Mirror E-2B-5 / E-2B-5R:

| Step | Requirement |
|------|-------------|
| Preflight | `prepare_nested_*` — no mutation of tree, scaffold index, allocator committed state, or registry |
| Commit | `commit_nested_*` — atomic: allocator → `ArenaRegistry::admit_participant_runtime` → tree → scaffold index |
| Generation | Bump **`ArenaRegistry.generation`** only if ≥1 admission committed in batch |
| Rejection | Failed admission leaves **zero** partial state (E-2B-5R precedent) |
| Sync | Session Resource Flow sync keyed on generation; re-build `ArenaExecutionPlan` via `build_execution_plan` |
| Reporting | Extend or parallel `DynamicFissionEnrollmentReport` with nested parent slot, arena idx, admission/rejection reasons |

### 6.2 What must **not** happen

- Generation bump on rejected preflight
- GPU sync when `use_accumulator_resource_flow` is false (unchanged)
- Inference of GPU execution from `ResourceFlowSpec` presence alone
- Hard-currency transfer side effects

---

## 7. Replay determinism model

### 7.1 Required guarantees

1. **Same fission sequence + same seed → same gap pool snapshots** (E-11B-4: `e11b_nested_replay_same_seed_same_gap_state`).
2. **Same admission sequence → same registry generation trajectory** and same nested layout shape.
3. **Rejected admissions are deterministic** — same reason codes/strings for same state.
4. **No partial mutation on reject** — gap exhaustion precedent (`e11b_nested_gap_exhaustion_rejects_without_partial_mutation`).

### 7.2 Replay scope for v1

- Record admissions/rejections in session report (inspectable like `last_resource_flow_dynamic_enrollment_report`).
- Rebuild layout from committed scaffold + tree state; do not cache stale nested plans across generation.
- GPU parity tests must pass after dynamic admission sequences (D=3/D=4 minimum).

---

## 8. Required tests before implementation

Do **not** implement nested dynamic enrollment until these exist (or are explicitly scoped in the implementation PR):

| Test | Purpose |
|------|---------|
| `e11b_nested_dynamic_admission_extends_contiguous_active_children` | Successful admission preserves `verify_child_contiguity` |
| `e11b_nested_dynamic_admission_rejects_noncontiguous` | Gap-slot-only or broken span → reject, no partial state |
| `e11b_nested_dynamic_admission_bumps_generation_on_commit_only` | Generation discipline |
| `e11b_nested_dynamic_admission_d3_cpu_gpu_parity` | Post-admission oracle vs GPU |
| `e11b_nested_dynamic_admission_d4_cpu_gpu_parity` | D=4 coverage |
| `e11b_nested_dynamic_admission_replay_determinism` | Same sequence → same layout + gap snapshot |
| `e11b_nested_dynamic_admission_flag_off_no_gpu_sync` | Flag default false respected |
| `e11b_nested_dynamic_admission_flat_star_regression` | E-2B-5 flat-star paths unchanged |
| `e11b_nested_dynamic_admission_no_simthing_sim_arena_imports` | Constitutional |
| `e11b_nested_dynamic_admission_no_new_wgsl` | Constitutional |

Existing suites remain regression baseline: `e11b_nested_hierarchy_gpu`, `e11b_nested_fission_gap`, full RF-T / E-2B ladders.

---

## 9. Stop conditions / Opus triggers

**Halt and recommend Opus review** if nested dynamic enrollment is judged to require any of:

| Trigger | v1 assessment |
|---------|---------------|
| Policy B Reevaluate | **Not required** for v1; **Opus if mandatory** |
| Selector re-run / wildcard expansion | **Reject** for v1 |
| New WGSL | **Not required** — existing OrderBand ops |
| New `AccumulatorRole` variants | **Not required** |
| CPU production allocation fallback | **Reject** |
| `simthing-sim` arena awareness | **Reject** |
| Boundary-time slot compaction | **Reject** |
| Indirection-list `SlotRange` | **Reject** |
| Global Resource Flow default-on | **Reject** |
| Hard-currency through Resource Flow | **Reject** |
| Automatic admission of all gap children as leaves | **Reject** — explicit gate only |

**Conditional halt:** If product requires **in-place gap-slot promotion** into `SlotRange` without contiguous extension, v1 cannot proceed without compaction or indirection → **defer or Opus**.

---

## 10. Review questions — concrete answers

| # | Question | Answer |
|---|----------|--------|
| 1 | What would nested dynamic enrollment mean after kickoff + E-11B-4? | Explicit post-open admission of allocation participants under enrolled nested parents, with preflight/commit, generation bump, and layout re-plan — **not** static materialization or gap-only attach. |
| 2 | Reserved-gap activation only, or selector re-evaluation? | **Explicit admission gate** under known parent; **no** selector re-run for v1. Gap-only attach remains non-leaf. |
| 3 | Without Policy B? | **Yes** for v1 — inherit Policy A weight seeding; no Reevaluate. |
| 4 | Without compaction / indirection? | **Yes** if admission uses contiguous extension or rejects; **no** if product needs non-adjacent gap promotion. |
| 5 | Preserve parent active child contiguity? | **Yes** — enforce via preflight + `NonContiguousChildren` on failure (already landed). |
| 6 | Gap child → allocation leaf automatically? | **No** — only after explicit nested admission; gap claim alone is topology-only. |
| 7 | Registry generation + RF sync after admission? | Bump generation on successful commit; rebuild execution plan; flag-gated GPU sync — mirror E-2B-5R. |
| 8 | Replay guarantees? | Deterministic gap snapshots, admission outcomes, generation trajectory; no partial reject mutation. |
| 9 | Tests required before implementation? | See §8 — dynamic admission, parity, replay, flag-off, regressions. |
| 10 | Continue, pause, or Opus? | **Pause (defer)** by default; **narrow implement** via E-11B-5 ladder when product names a scenario; **not Opus** unless triggers in §9 fire. |

---

## 11. Recommendation

### Verdict: **Defer (B)** — authorize narrow ladder when product prioritizes

**Rationale:**

1. **No named product scenario** today requires nested Resource Flow growth beyond static D=3/D=4 materialization plus gap-isolated fission attach.
2. **FlatStarResourceFlow** remains the accepted bounded production posture; nested dynamic enrollment expands blast radius without a burned-in product fixture class.
3. **Substrate is ready** for a narrow v1 — E-11B-4 proved contiguity enforcement and gap isolation; E-2B-5 provides the preflight/commit template.
4. **Risk is semantic, not GPU** — conflating gap attach with allocation admission is the primary failure mode; v1 must keep the E-11B-4 distinction explicit.

**Do not send to Opus** unless requirements mandate Policy B, selector re-run, compaction, or indirection.

### Candidate implementation ladder (if product authorizes — not this PR)

| Step | Scope |
|------|-------|
| **E-11B-5a** | Docs + policy constants; nested dynamic enrollment opt-in flag/mode |
| **E-11B-5b** | `prepare_nested_gap_activation` preflight (contiguity check, no mutation) |
| **E-11B-5c** | `commit_nested_gap_activation` — commit only when contiguity preserved |
| **E-11B-5d** | Generation bump + nested dynamic enrollment report + sync hook |
| **E-11B-5e** | D=3/D=4 nested dynamic admission CPU/GPU parity tests |
| **E-11B-5f** | Replay + nested burn-in extension |

---

## 12. Docs update requirements

When nested dynamic enrollment **implementation** lands (future PR):

- Update `accumulator_op_v2_production_plan.md`, `todo.md`, `worklog.md`, `workshop_current_state.md`
- Extend `resource_flow_limited_scenario_class_posture.md` only if a new explicit scenario class is accepted
- Add test report under `docs/tests/` for GPU-visible runs

**This review PR:** docs-only updates per handoff wording; no production code.

---

## 13. Priority vs other gates

| Gate | Relative priority | Notes |
|------|-------------------|-------|
| **Pause E-11B** | **Default** | Static nested + gap hardening sufficient until product pull |
| **Narrow E-11B-5** | When named nested dynamic scenario exists | Reuses E-2B-5 patterns; no WGSL |
| **D-2a** | When named hard-currency multi-transaction scenario exists | Independent track |
| **simthing-spec/RON rebuild** | When authoring track opens | Independent |
| **Continued flat-star soak** | Ongoing confidence | No conflict |

E-11B nested dynamic enrollment is **lower urgency** than flat-star production soak and **not** a prerequisite for Phase T or D-2a.

---

## 14. Verdict table

| Question | Answer |
|----------|--------|
| Implement nested dynamic enrollment now? | **No (defer)** unless product explicitly prioritizes |
| Narrow v1 feasible without Policy B / compaction / WGSL? | **Yes** |
| Gap child automatic allocation leaf? | **No** |
| Opus required? | **No** for v1; escalate if Policy B or compaction mandated |
| Next gate | Product choice: **E-11B-5 ladder**, **pause E-11B**, **D-2a**, or **spec/RON rebuild** |

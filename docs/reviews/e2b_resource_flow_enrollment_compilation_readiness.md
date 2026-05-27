# E-2B — Resource Flow Enrollment Compilation Readiness Review

**Status:** Accepted (design/readiness review, docs-only)  
**Date:** 2026-05-27  
**Scope:** Post–E-11 flat-star audit for `resource_flow_participant` enrollment compilation  
**Authority:** [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md), [`e11_implementation_handoff.md`](../workshop/e11_implementation_handoff.md), [`e11b_nested_hierarchy_gpu_readiness_review.md`](e11b_nested_hierarchy_gpu_readiness_review.md), [`transfer_emission_registration_ownership_opus_review.md`](transfer_emission_registration_ownership_opus_review.md)

---

## Executive summary

E-2B **enrollment compilation** is the missing compile/install bridge that turns **authored Resource Flow participation intent** into **session artifacts** (`ArenaRegistry`, `ArenaParticipantScaffold`, E-11 allocation sync) without manual test-fixture hacks (`fill_explicit_participants`).

**Much of the driver substrate is already landed:** E-10 admission compile, E-9 `ArenaRegistry` materialization, E-10R preflight, E-10R2 participant scaffold, E-10R3 gap blocks, and E-11 flat-star execution. What remains is primarily **authored enrollment resolution** (stable selectors → live `ExplicitParticipantSpec`), optional **wildcard expansion**, and optional **dynamic fission enrollment**.

**E-2B does not require E-11B nested hierarchy GPU.** Flat-star static enrollment compilation is sufficient for the first vertical slice. **E-11B remains deferred by default.**

**Recommendation:** **Implement E-2B with a narrowed ladder** — Cursor can land **E-2B-1…E-2B-4** (static session-open enrollment) directly. **Send E-2B-1 selector semantics to Opus** only if the product requires a novel selector algebra beyond reusing `InstallTargetSpec` patterns; otherwise Cursor may proceed. **Defer E-2B-5** (dynamic fission enrollment) unless fission-spawned participants are an immediate product requirement. **Revise** the original production-plan `resource_flow_participant` AccumulatorOp builder — enrollment is expressed through the E-11 allocator path, not per-participant `ResetTarget`/`AddToTarget` op sets.

**Next gate depends on review recommendation:** E-2B implementation ladder, E-11B (if nested Resource Flow prioritized), D-2a discrete scheduling, or Opus review of selector authoring.

---

## 1. Current-state audit

| Layer | Landed | E-2B gap |
|-------|--------|----------|
| Spec types | `ResourceFlowSpec`, `ArenaSpec`, `ExplicitParticipantSpec`, `WildcardAdmissionSpec` | No authored **selector** for enrollment; RON authors must supply session-raw `slot` + `subtree_root_id` |
| Spec compile | `compile_resource_flow_admission` — caps, coupling, property bindings, wildcard cap validation | No compile-time **selector → participant list** expansion |
| Driver preflight | `validate_resource_flow_preflight` — live slot/identity/tombstone checks | Runs only on pre-filled `explicit_participants` |
| Participant materialization | `materialize_arena_participants` — `ArenaParticipant` SimThings, index, E-10R3 gaps | Only processes authored `explicit_participants`; no selector query |
| Registry materialization | `compile_and_materialize_resource_flow` → `ArenaRegistry` | Same — admits only explicit list |
| Session install | `install.rs` §4b: preflight → scaffold → registry | Works when participants pre-filled; no RON-native enrollment |
| Execution | E-11 flat-star GPU path via `sync_resource_flow_accumulator` | Ready once enrollment artifacts exist |
| Fission | `try_alloc_participant_child_in_gap`, `refresh_fission_participant_child` | **Unwired**; `ArenaRegistry::refresh_subtree` is generation bump only |
| Builder | E-2A `resource_transfer_discrete` landed | E-2B `resource_flow_participant` **not implemented** (superseded architecturally) |
| Tests | E-10/E-10R/E-11 suites; `fill_explicit_participants` in flat-star fixtures | No end-to-end **RON selector → session open → allocation** test |

**Constitutional posture preserved:** AccumulatorOp v2 substrate; Resource Flow as driver/spec registration over flat AccumulatorOps; `simthing-sim` arena-ignorant; no CPU production allocation fallback; no boundary-time slot compaction; `use_accumulator_resource_flow` default **false**; hard-currency transfers separate from Resource Flow; E-11B deferred.

---

## 2. Definition of E-2B enrollment compilation

### 2.1 What E-2B means in the current codebase

E-2B **resource_flow_participant enrollment compilation** is the **compile + install pipeline** that:

1. Accepts **authored Resource Flow participation intent** (who enrolls in which arena).
2. Resolves that intent against the **live session** at install time (slots, SimThing ids, contiguity).
3. Materializes **driver/session artifacts**: `ArenaParticipantScaffold`, `ArenaRegistry`, and (when flag enabled) E-11 allocation ops.
4. Optionally supports **runtime growth** via fission policy (`Reevaluate` / gap claim).

It is **not** the E-11 allocation kernel itself (landed). It is **not** discrete hard-currency transfer (E-2A / Phase T). It is **not** default-on Resource Flow.

### 2.2 Original production-plan E-2B vs evolved architecture

The production plan described:

```rust
pub fn resource_flow_participant(
    slot: SlotId,
    arena: ArenaName,
    role: AccumulatorRole,  // IntrinsicFlow | AllocatedFlow | AllocatorWeight
) -> AccumulatorOpSet
```

That builder was intended when continuous flows used per-participant AccumulatorOp enrollment. **E-11 superseded this:** continuous allocation is compiled from `ArenaRegistry` + hierarchy layout into banded `AccumulatorOp` registrations via `plan_arena_allocation`. Property roles (`IntrinsicFlow`, `AllocatedFlow`, `AllocatorWeight`) remain on `SubFieldSpec.accumulator_spec`; enrollment binds **hosted SimThings** to arenas via participant scaffold + registry, not via a standalone op-set builder.

**E-2B should be reframed:** enrollment compilation = **selector/admission compile + driver materialization**, not necessarily landing the legacy `resource_flow_participant` builder. A thin documentation alias or internal helper mapping participant slots to role metadata may still be useful for tests, but it is not the production enrollment path.

### 2.3 Answers to review questions

| # | Question | Answer |
|---|----------|--------|
| 1 | What does E-2B mean? | Compile authored arena participation → session `ArenaRegistry` + `ArenaParticipantScaffold` + E-11 sync inputs |
| 2 | Spec, driver, or both? | **Both.** Spec adds/resolves enrollment selectors; driver materializes at session open |
| 3 | Static, dynamic, or both? | **Phase 1: static at session open** (required). **Phase 2: dynamic fission** (optional E-2B-5) |
| 4 | Mapping into artifacts | See §4 |
| 5 | E-10R / E-10R2 / E-10R3 | See §5 |
| 6 | Missing after E-11 flat-star | Selector compile, RON path, wildcard expansion, fission wiring, end-to-end test |
| 7 | Requires E-11B? | **No** for flat-star static enrollment. **E-11B remains deferred.** |
| 8 | WGSL / roles / CPU fallback / sim awareness? | **No** to all four |
| 9 | Implementation ladder | See §9 |
| 10 | Required tests | See §10 |

---

## 3. Spec authoring surface proposal

### 3.1 Problem

`ExplicitParticipantSpec` today requires:

```rust
pub struct ExplicitParticipantSpec {
    pub slot: u32,
    pub subtree_root_id: u32,  // session-local SimThingId raw
}
```

RON authors and content packs **cannot know** session-raw slot ids or runtime-assigned SimThing ids at authoring time. Tests work around this with `fill_explicit_participants` after scenario tree allocation (`e11_flat_star.rs`).

### 3.2 Proposed E-2B-1 shape (narrow)

Add an **authored enrollment selector** on `ArenaSpec`, sibling to (not replacing) explicit participants for driver/test injection:

```rust
/// Resolved at session install against Scenario + live SimThing tree.
#[serde(default)]
pub enrollment: Option<EnrollmentSelectorSpec>,

pub enum EnrollmentSelectorSpec {
    /// Reuse install-target resolution (AllOfKind, ScenarioListed, SessionRoot children, …).
    InstallTarget(InstallTargetSpec),
    /// Explicit list only — for tests and driver-emitted specs (current behavior).
    ExplicitOnly,
}
```

**Default for new RON:** `InstallTarget(AllOfKind { kind: "Cohort" })` or scenario-listed keys — product choice at E-2B-1.

**Compile rule:** at install time, driver resolves selector → `Vec<ExplicitParticipantSpec>` with live `(slot, subtree_root_id)`, then runs existing E-10R preflight + materialization unchanged.

### 3.3 Wildcard admission (deferred sub-slice)

`WildcardAdmissionSpec` exists (`max_expansion`, `expanded_count`) but **no selector query** expands wildcards into participants. Soak/burn-in explicitly avoid wildcard admission today.

**E-2B scope choice:** static `InstallTarget` resolution first; wildcard selector expansion as **E-2B-2b** or post-E-2B-4 follow-on unless product requires it immediately.

### 3.4 Opus gate

If enrollment selectors need arena-specific query algebra (property filters, capability-tree membership, cross-arena dedup rules), **Opus review before E-2B-1 code** is prudent. If reusing `InstallTargetSpec` resolution is sufficient for v1, **Cursor may implement without Opus.**

---

## 4. Driver/session materialization proposal

### 4.1 Mapping authored enrollment → artifacts

| Step | Input | Output | Module (existing / new) |
|------|-------|--------|-------------------------|
| **Resolve** | `ArenaSpec.enrollment` + live `Scenario` | `Vec<ExplicitParticipantSpec>` per arena | **New:** `compile_enrollment_participants` in `simthing-driver` or `simthing-spec` install compile |
| **Preflight** | Resolved explicit list + `SlotAllocator` | Hard reject on mismatch/tombstone | **Existing:** `validate_resource_flow_preflight` |
| **Scaffold** | `ResourceFlowSpec` + registry + root + allocator | `ArenaParticipantScaffold` (index, gap pools, reports) | **Existing:** `materialize_arena_participants` |
| **Registry** | Compiled admission | `ArenaRegistry` + expansion report | **Existing:** `compile_and_materialize_resource_flow` |
| **Execution plan** | Registry + scaffold + tree | `ArenaExecutionPlan` (flat-star D=2) | **Existing:** `build_execution_plan` |
| **GPU sync** | Plan + flag | Uploaded allocation ops | **Existing:** `sync_resource_flow_accumulator` |

### 4.2 `ArenaSpec` → `ExplicitParticipantSpec`

- **Selector resolution** produces `(hosted SimThingId, slot)` pairs.
- Each pair becomes `ExplicitParticipantSpec { slot, subtree_root_id: id.raw() }`.
- Driver creates one `SimThingKind::ArenaParticipant` child per pair under arena root (E-10R2).
- `ArenaParticipantIndex.by_host_and_arena` maps `(hosted_id, arena_idx) → participant_slot`.

### 4.3 `ExplicitParticipantSpec` → `ArenaRegistry`

- `compile_resource_flow_admission` copies explicit pairs into `CompiledArenaAdmission`.
- `materialize_arena_registry` calls `builder.admit_participant(arena_idx, slot, SimThingId::from_session_raw(subtree_root_raw))`.
- Caps (`max_participants`, fanout, orderband) enforced at build (E-9).

### 4.4 `ArenaParticipantScaffold` → resource-flow sync

- `build_execution_plan` reads sibling participant slots under arena root (flat-star).
- `plan_arena_allocation` + `sync_resource_flow_accumulator` upload E-11 ops when `use_accumulator_resource_flow` is enabled.

### 4.5 Install ordering (unchanged)

From `install.rs` §4b — properties first, capability trees attached, allocator populated, **then** Resource Flow:

1. Resolve enrollment selectors (E-2B **new**)
2. `validate_resource_flow_preflight`
3. `materialize_arena_participants`
4. `compile_and_materialize_resource_flow`

Slot overflow checks after scaffold allocation remain.

---

## 5. Relationship to E-10R / E-10R2 / E-10R3

| Gate | What it solved | E-2B relationship |
|------|----------------|-------------------|
| **E-10R** | Live identity preflight — slot matches SimThing, not tombstoned | **Reuse as-is** after selector resolution |
| **E-10R2** | Dedicated `ArenaParticipant` SimThings; sibling contiguity; allocation reports | **Reuse as-is**; enrollment compilation feeds it resolved explicit list |
| **E-10R3** | Reserved gap blocks per parent participant; fission gap claim API | **Reuse for static enrollment**; dynamic fission wiring is E-2B-5 |

E-10R/E-10R2/E-10R3 assumed the driver **already knew** explicit participants. E-2B fills the **authored → explicit** gap.

---

## 6. Relationship to E-11 flat-star and E-11B

| Item | Relationship |
|------|--------------|
| **E-11 flat-star** | **Execution consumer** of enrollment artifacts. E-2B unblocks RON-native scenarios that run E-11 without `fill_explicit_participants`. |
| **E-11B nested GPU** | **Not required** for E-2B v1. Nested hierarchy materialization is orthogonal; flat-star enrollment + flat-star execution is the correct first slice. |
| **Default-on Resource Flow** | Still blocked. E-2B is necessary but not sufficient; burn-in + product gate remain. |

---

## 7. Fission / dynamic enrollment implications

### 7.1 Current state

- `FissionPolicySpec` (`Inherit`, `Reevaluate`, `Reject`) is authored and stored in `ArenaRegistry`.
- `try_alloc_participant_child_in_gap` and `refresh_fission_participant_child` exist in `arena_participant.rs` but are **not called** from session/boundary hooks.
- `ArenaRegistry::refresh_subtree` bumps generation for matching `subtree_root` but **does not admit** new participants.

### 7.2 E-2B-5 scope (optional)

Dynamic enrollment requires:

1. Boundary hook on fission spawn → evaluate arena `fission_policy`.
2. For `Reevaluate`: run enrollment selector scoped to new subtree; admit if cap allows.
3. Claim gap slot (E-10R3) or reject per policy.
4. Increment `ArenaRegistry.generation`; re-run `build_execution_plan` + sync (may require flat-star re-layout if sibling count changes — **contiguity check**).

**Recommendation:** defer E-2B-5 unless product needs runtime participant growth in near term. Static enrollment (E-2B-1…4) unblocks most authored content.

---

## 8. Stop conditions

E-2B implementation must **stop** if any of the following would be required:

| Stop condition | E-2B posture |
|----------------|--------------|
| New WGSL | **Forbidden** — use existing E-11 op planner |
| New `AccumulatorRole` variants | **Forbidden** — use existing roles on flow property sub-fields |
| CPU production allocation fallback | **Forbidden** |
| Boundary-time slot compaction | **Forbidden** |
| `simthing-sim` imports `ArenaRegistry` / arena semantics | **Forbidden** |
| Routing hard-currency transfer through Resource Flow | **Forbidden** |
| Default-on `use_accumulator_resource_flow` | **Forbidden** — separate product gate |
| E-11B nested hierarchy as prerequisite | **Not required** — do not block E-2B on E-11B |

---

## 9. Recommended implementation ladder

| Step | Scope | Owner | Notes |
|------|-------|-------|-------|
| **E-2B-1** | Spec authoring shape: `EnrollmentSelectorSpec` on `ArenaSpec` (or equivalent) | Cursor (Opus if selector algebra non-trivial) | Reuse `InstallTargetSpec` where possible |
| **E-2B-2** | Compile/admission validation: resolve selectors → explicit participants; reject empty/over-cap | Cursor | New compile pass before E-10R preflight |
| **E-2B-3** | Driver materialization: wire resolution into `install.rs`; remove test-only `fill_explicit_participants` from production path | Cursor | Existing scaffold/registry/sync unchanged |
| **E-2B-4** | Session-open enrollment test: RON `GameModeSpec` → install → E-11 flat-star allocation (flag opt-in) | Cursor | Mirror `e11_flat_star` without manual slot fill |
| **E-2B-5** | Fission/dynamic enrollment scaffold | Cursor (later) | Wire `refresh_fission_participant_child`; extend registry admit API |
| **E-2B-6** | Docs + burn-in update | Cursor | Add opt-in scenario with selector enrollment; keep soak flat-star guards unless product expands |

**Do not implement** the legacy `resource_flow_participant` AccumulatorOp-set builder unless a separate design review proves per-participant ops are needed alongside E-11 (unlikely).

---

## 10. Required tests before E-2B is considered landed

### 10.1 Spec / compile (E-2B-1/2)

- Selector resolves `AllOfKind` / `ScenarioListed` to correct participant count.
- Over-cap rejection (`max_participants`).
- Empty resolution rejection (`ImplicitParticipation`).
- Duplicate hosted SimThing in same arena rejection (if rule adopted).
- Wildcard cap validation unchanged (no regression).

### 10.2 Driver materialization (E-2B-3)

- Resolved participants pass E-10R preflight.
- `ArenaParticipantScaffold.index` maps all hosted ids.
- Sibling contiguity preserved (`slots_are_contiguous`).
- Slot overflow → `ResourceFlowSlotOverflow` / `InstallError`.

### 10.3 End-to-end (E-2B-4)

- `GameModeSpec` RON with selector enrollment → `SimSession::open_from_spec` → `install_spec_state`.
- Flag opt-in → `sync_resource_flow_accumulator` uploads ops.
- CPU/GPU parity or oracle conservation on flat-star fixture (reuse E-11 tests pattern).
- Flag default **false** — opt-in explicit in test.

### 10.4 Regression

- All existing E-10, E-10R, E-11, E-11R, soak, Phase T suites green.
- `cargo check --workspace`; `cargo test --workspace`.

### 10.5 E-2B-5 (if in scope)

- Fission spawn under `Reevaluate` admits child when gap available.
- Cap exceeded → reject per `FissionPolicy::Reject`.
- Registry generation bump triggers re-sync.

---

## 11. Docs update requirements (post-implementation)

When E-2B code lands (not this review PR):

- `docs/accumulator_op_v2_production_plan.md` — E-2B status, revised builder definition
- `docs/todo.md` — unblock E-2B; update default-on Resource Flow gate
- `docs/worklog.md` — E-2B land entry
- `docs/workshop/workshop_current_state.md` — enrollment compilation landed
- `docs/adr/resource_flow_substrate.md` — authored enrollment selector cross-ref if ADR amended

---

## 12. Final recommendation

| Verdict | Detail |
|---------|--------|
| **Implement** | **Yes** — narrowed **E-2B-1…E-2B-4** static session-open enrollment compilation |
| **Defer** | **E-2B-5** dynamic fission enrollment; **E-11B** nested GPU; legacy `resource_flow_participant` op-set builder |
| **Opus review** | **Optional before E-2B-1** if selector semantics exceed `InstallTargetSpec` reuse; **not required** for driver materialization (E-2B-3/4) |
| **Cursor implementable** | **Yes** for E-2B-1…4 if E-2B-1 adopts `InstallTargetSpec` pattern |

**Priority vs other gates:**

1. **E-2B static enrollment** — highest if authored Resource Flow content is the near-term goal (unblocks RON scenarios without manual slot injection).
2. **D-2a discrete scheduling** — if hard-currency ordering is urgent (D-1 memo).
3. **E-11B nested GPU** — only if nested Resource Flow is explicitly prioritized (E-11B review defers by default).

**Bottom line:** The substrate is **ready for E-2B static enrollment compilation**. The gap is narrow and well-bounded. Proceed with E-2B-1…4 implementation; keep E-11B deferred; send selector authoring to Opus only if the product rejects `InstallTargetSpec` reuse.

# Resource Flow Default-On Readiness Review

**Status:** Accepted (design/readiness review, docs-only)  
**Date:** 2026-05-19  
**Scope:** Post–E-2B-5 soak audit for whether `use_accumulator_resource_flow` may move from global default-off toward any default-on posture  
**Authority:** [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md), [`accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md), [`e2b_resource_flow_enrollment_compilation_readiness.md`](e2b_resource_flow_enrollment_compilation_readiness.md), [`e2b5_dynamic_fission_enrollment_readiness.md`](e2b5_dynamic_fission_enrollment_readiness.md), [`e11b_nested_hierarchy_gpu_readiness_review.md`](e11b_nested_hierarchy_gpu_readiness_review.md), E-2B-5 soak gate (PR #178)

---

## Executive summary

Resource Flow GPU execution is **technically proven** for a **narrow, opt-in slice**: flat-star D=2 allocation, static E-2B enrollment, Policy A dynamic fission enrollment (E-2B-5/E-2B-5R), and controlled soak burn-in. **`use_accumulator_resource_flow` remains default false** and should stay that way globally.

**Recommendation: B — allow limited scenario-class default-on readiness to proceed; reject global default-on (D).**

Mirror the **Phase T precedent** (`ResourceEconomyOptInMode`): explicit scenario/game-mode opt-in for Resource Flow GPU sync, **not** flipping `PipelineFlags::default()`. Global default-on would enable GPU allocation for every session with authored `ResourceFlowSpec`, including paths that are **not burned in** (nested hierarchy, Policy B fission, couplings at scale, product scenarios).

**Next gate:** implement **limited scenario-class Resource Flow opt-in flagging** (T-6 analogue for Resource Flow), expand burn-in for flagged scenarios only, then re-evaluate global default-on only after product-scale soak.

---

## 1. Current-state audit

| Layer | Landed posture | Default-on relevance |
|-------|----------------|----------------------|
| **Runtime substrate** | AccumulatorOp v2; E-11 flat-star allocation via existing kernel | No new WGSL required for flat-star |
| **Flag** | `PipelineFlags::use_accumulator_resource_flow` default **false** (`simthing-sim`) | Global default-on = change this default |
| **Session sync** | `sync_resource_flow_if_enabled` → `sync_resource_flow_accumulator` when flag true | Flag-off clears GPU resource-flow ops |
| **Static enrollment** | E-2B-1…4: selectors → explicit participants at install | Proven in compile/session tests |
| **Dynamic enrollment** | E-2B-5 Policy A inherit + arena-root sibling append | E-2B-5R atomic prepare/commit |
| **Diagnostics** | `last_resource_flow_dynamic_enrollment_report` on boundary | Visible rejections/admissions |
| **Execution layout** | `build_flat_star_layout` only in production `build_execution_plan` | Nested D≥3 not production-wired |
| **Fission policy runtime** | `Reevaluate` mapped to inherit-only for enrollment | Policy B selector re-run **deferred** |
| **Hard currency** | Phase T transfer/emission on discrete AccumulatorOp paths | Separate from Resource Flow |
| **simthing-sim** | Arena-ignorant; no `ArenaRegistry` import | Preserved |

**Constitutional posture preserved:** no WGSL changes; no new `AccumulatorRole` variants; no CPU production allocation fallback; no boundary-time slot compaction; no indirection-list SlotRange replacement; E-11B deferred by default.

---

## 2. Definition of default-on candidates

### 2.1 What “default-on Resource Flow” would mean in this repo

Flipping or effectively enabling **`use_accumulator_resource_flow`** so that sessions with authored `ResourceFlowSpec` **plan and upload E-11 allocation AccumulatorOps** and **dispatch `run_resource_flow_bands`** without an explicit opt-in step.

Concrete effects when enabled:

1. `install_spec_state` / boundary hooks call `sync_resource_flow_if_enabled`.
2. `WorldGpuState` uploads combined flat-star (today) allocation op plans.
3. GPU bands execute per tick (when session loop dispatches resource-flow bands).
4. Dynamic fission admissions trigger conditional re-sync (E-2B-5R).

When flag is **false** (current default):

- Registry/scaffold/enrollment state may still be populated at compile/install.
- GPU resource-flow accumulator is **cleared**; no allocation dispatch.
- Dynamic enrollment updates driver artifacts only (E-2B-5R flag-off soak proven).

### 2.2 Candidate postures (distinct)

| Candidate | Meaning | Precedent in repo |
|-----------|---------|-------------------|
| **Global default-on** | `PipelineFlags::default().use_accumulator_resource_flow == true` | **Not used** for transfer/emission (T-6 kept global false) |
| **Scenario-class default-on** | Named scenario/game-mode fixture sets flag true at open (CI + product scenarios) | `ResourceFlowSoakMode::FlatStarOptIn`, `ResourceEconomyOptInMode`, dynamic enrollment soak fixtures |
| **Opt-in-by-spec default-on** | `GameModeSpec` declares Resource Flow execution mode (e.g. `FlatStarOptIn`) applied in `SimSession::open_from_spec` | T-6 `ResourceEconomyOptInMode` on `ResourceEconomySpec` |
| **Explicit opt-in only (status quo)** | Every session must set flag true manually or via test harness | Current production posture |

### 2.3 Which path has actually been burned in

| Path | Burn-in evidence | Scope |
|------|------------------|-------|
| **Static E-2B enrollment** | `resource_flow_enrollment_compile`, `resource_flow_enrollment_session` | Selector → explicit participants; no GPU required |
| **E-11 flat-star D=2** | `e11_burn_in`, `e11_burn_in_scenarios`, `e11_resource_flow_soak` (1000-tick, resync) | Opt-in flag true in tests; CPU/GPU oracle parity |
| **E-2B-5 Policy A dynamic enrollment** | `e2b5_dynamic_fission_enrollment` (21 tests) | Registry/scaffold/sync; GPU in subset |
| **E-2B-5R atomicity** | `e2b5_registry_rejection_*`, `e2b5_max_participants_*`, session report tests | Failure leaves no partial state |
| **Dynamic enrollment soak** | `e2b5_dynamic_enrollment_soak` (12 tests, 100–1000 ticks) | Opt-in GPU path; replay; resync; atomic rejections |

**Not burned in under default-on or broad scenario coverage:**

- Production `SimSession::open_from_spec` with Resource Flow flag true for real game modes
- Multi-arena coupling graphs at product scale
- Nested hierarchy GPU (E-11B)
- Policy B `Reevaluate` selector re-run at fission
- Gap-only nested fission allocation participation
- Wildcard/dynamic selector expansion beyond static E-2B fixtures
- Cross-session product-scale soak (many hosted counts, long campaigns)

---

## 3. Explicit exclusions (unchanged deferrals)

| Exclusion | Why it blocks global default-on |
|-----------|--------------------------------|
| **E-11B nested hierarchy GPU** | Production `build_execution_plan` always flat-star; D≥3 GPU parity not proven |
| **Policy B Reevaluate** | Authored default on `ArenaSpec`; runtime maps to inherit-only; selector re-run unimplemented |
| **Gap-only nested fission allocation** | E-10R3 gap path preserves SlotRange but **excludes** flat-star leaves; not used for Policy A |
| **Wildcard / unbounded dynamic expansion** | Spec compiler caps exist; runtime wildcard admission not product-soaked |
| **Cross-arena coupling at scale** | Coupling delay forms compiled; no large coupling burn-in |
| **Hard-currency via Resource Flow** | Constitutionally separate; Phase T discrete paths remain |
| **Global default-on without product soak** | Would activate GPU path for all authored specs including uncovered topology |

---

## 4. Remaining risks

1. **Silent no-op when spec populated but flag false** — Authors may assume Resource Flow runs because registry/scaffold exist. Enrollment and GPU sync are decoupled by design; needs explicit spec-level signaling (see §6).
2. **Flat-star-only production layout** — Default-on globally would imply hierarchical allocation works; it does not in production wiring.
3. **Dynamic enrollment + flag-on resync** — Generation-keyed replan is tested in soak, but not across full session tick loops with real fission cadence in product scenarios.
4. **f32 approximate-deterministic conservation** — Acceptable for continuous flow per ADR; must not be conflated with exact discrete transfer semantics if default-on blurs mental model.
5. **Reevaluate authored default** — Scenarios authored with `Reevaluate` do not get Policy B behavior; inherit-only may surprise designers until Policy B lands or spec defaults change.
6. **Two-flow registry dimension mismatch** — Soak fixed session open with multi-property registry; product install paths must keep `n_dims` consistent (soak lesson from two-arena fixture).
7. **Replay / spec snapshot** — Dynamic enrollment reports retained on session; full LDJSON replay of enrollment + GPU frames not product-gated.

---

## 5. Telemetry / reporting requirements before default-on

Before any scenario-class or global default-on implementation:

| Telemetry | Purpose |
|-----------|---------|
| **Resource Flow sync report** | Already in `ResourceFlowSyncReport` — surface arenas_planned, total_ops, n_bands, enabled in session diagnostics |
| **Dynamic enrollment report** | `last_resource_flow_dynamic_enrollment_report` — admissions/rejections/generation delta per boundary |
| **Expansion report at install** | Per-arena participant counts, caps, rejection diagnostics (E-10) |
| **Flag source attribution** | Log whether flag enabled via global default, game-mode opt-in, or test override |
| **Soak-style parity metrics** | max_abs_error, replay_bit_exact in scenario CI for opted-in fixtures |
| **Flat-star guard** | Assert no nested GPU claims in opt-in scenarios (`assert_flat_star_only_no_nested_claims` pattern) |

Default-on without these surfaces risks silent misconfiguration (spec present, wrong layout class, or rejections only in debug).

---

## 6. Authored Resource Flow with flag false

**Current behavior (correct):**

- Install compiles registry, scaffold, enrollment resolution.
- GPU resource-flow ops **cleared**; `accumulator_resource_flow_active == false`.
- Dynamic fission enrollment **still updates** registry/scaffold when Policy A applies (E-2B-5R).
- No GPU sync on successful admission unless flag true.

**Recommendation:** Treat this as **intentional staging** — spec compilation ≠ GPU execution. For product UX, add **spec-level opt-in** (see ladder) so authors declare execution intent. Optional future: driver warning when `resource_flow` populated, flag false, and scenario expects GPU disbursement.

**Do not** auto-enable GPU sync from spec presence alone without an explicit opt-in gate (same lesson as T-6 for transfer/emission).

---

## 7. Should default-on be rejected in favor of limited scenario flagging?

**Yes, for global default-on.** **No, for all progress** — limited scenario-class flagging should proceed.

Phase T concluded: global transfer/emission flags stay false; **`ResourceEconomyOptInMode`** enables execution per game mode. Resource Flow should follow the same pattern:

- **Reject D (global default-on)** until product-scale soak covers nested/coupling/fission-policy surfaces or those paths remain explicitly out of scope with hard guards.
- **Accept B (limited scenario-class default-on readiness)** — implement opt-in flagging parallel to T-6, expand burn-in for flagged scenarios only.

---

## 8. Recommendation

**Chosen: B — limited scenario-class default-on readiness may proceed; global default-on remains deferred.**

| Option | Verdict |
|--------|---------|
| **A.** Explicit opt-in only | Safe baseline; status quo — acceptable if product has no near-term Resource Flow scenarios |
| **B.** Limited scenario-class default-on | **Recommended next implementation gate** |
| **C.** Default-on flat-star/static only, not dynamic fission | Too narrow without opt-in machinery; dynamic path is now soaked — better expressed as scenario fixture flags |
| **D.** Global default-on | **Rejected** — insufficient burn-in coverage |

Rationale: Substrate slice is green for **opt-in flat-star + static/dynamic enrollment** in CI. Uncovered paths (E-11B, Policy B, couplings, product scale) are too broad for a global flag flip. T-6 provides a proven template.

---

## 9. Candidate implementation ladder (if B approved)

**Not in this PR.** Narrow sequence:

| Step | Scope | Notes |
|------|-------|-------|
| **RF-T1** | `ResourceFlowOptInMode` (or equivalent) on `GameModeSpec` / `ResourceFlowSpec` | Mirror `ResourceEconomyOptInMode`: `Disabled`, `FlatStarOptIn`, future `NestedOptIn` gated on E-11B |
| **RF-T2** | `SimSession::open_from_spec` applies opt-in → `proto.flags.use_accumulator_resource_flow` | Generation-keyed sync on install; reject inconsistent modes at compile |
| **RF-T3** | Scenario fixtures + CI markers | Named scenarios in driver tests; no global default change |
| **RF-T4** | Expanded burn-in for flagged scenarios | Product-like hosted counts; multi-session replay |
| **RF-T5** | Default-on readiness re-review | Revisit C/D only after RF-T4 green |

**Explicit non-goals in RF ladder:** no WGSL; no new `AccumulatorRole`; no CPU fallback; no simthing-sim arena awareness; no E-11B unless separately gated; no Policy B; no global `PipelineFlags` default change in RF-T1…T4.

---

## 10. Required tests before any default-on code lands

| Test obligation | Target |
|-----------------|--------|
| Flag default false invariant | `PipelineFlags::default().use_accumulator_resource_flow == false` |
| Opt-in scenario enables GPU | Session open with opt-in spec → sync → ops > 0, active true |
| Non-opt-in scenario stays off | Populated resource_flow spec + Disabled mode → active false |
| Flat-star-only guard | No nested claims in opt-in scenarios |
| E-2B-5 + opt-in resync | Dynamic admission → generation bump → conditional sync |
| Replay determinism | Same seed → same enrollment + parity frames (soak pattern) |
| Rejection atomicity | Cap/contiguity/registry reject — no partial state (E-2B-5R) |
| Regression | `e11_resource_flow_soak`, `e2b5_dynamic_enrollment_soak`, enrollment compile/session |

Stop and escalate to Opus if implementation appears to require: new WGSL, new roles, CPU fallback, simthing-sim arena imports, slot compaction, E-11B, Policy B, hard-currency via Resource Flow, or global default-on without RF-T4 soak.

---

## 11. Docs update requirements (post-review)

- `accumulator_op_v2_production_plan.md` — RF default-on review status; next gate RF-T1 or continued opt-in burn-in
- `todo.md` — pivot next gate per recommendation B
- `worklog.md` — review landed entry
- `workshop_current_state.md` — review + preserved default false

---

## 12. Review question index

| # | Answer (short) |
|---|----------------|
| 1 | Enable GPU E-11 sync/dispatch via `use_accumulator_resource_flow` |
| 2 | Three distinct candidates; **scenario-class / spec opt-in** recommended, not global |
| 3 | Static E-2B, E-11 flat-star, E-2B-5/5R, soak — all opt-in tested |
| 4 | E-11B, Policy B, gap-only nested, wildcards at scale, couplings, product scenarios — **not covered** |
| 5 | Silent flag-off, flat-star-only wiring, Reevaluate semantics, dimension mismatch, replay gaps |
| 6 | Sync report, enrollment report, expansion report, flag attribution, soak parity metrics |
| 7 | Spec stages driver state; GPU cleared; intentional — add spec opt-in |
| 8 | **Yes** reject global default-on; **proceed** with limited scenario flagging |
| 9 | RF-T1…T5 ladder above |
| 10 | Table in §10 |

---

## References

- [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md) — constitutional caps, approximate conservation, explicit participation
- [`accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md) — E-2B/E-11/T-6 gates
- E-2B-5 soak — PR #178, `e2b5_dynamic_enrollment_soak` (12 tests)
- [`e2b5_dynamic_fission_enrollment_readiness.md`](e2b5_dynamic_fission_enrollment_readiness.md) — Policy A vs gap-only
- [`e11b_nested_hierarchy_gpu_readiness_review.md`](e11b_nested_hierarchy_gpu_readiness_review.md) — nested deferral
- [`e2b_resource_flow_enrollment_compilation_readiness.md`](e2b_resource_flow_enrollment_compilation_readiness.md) — static enrollment

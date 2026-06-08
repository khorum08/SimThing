# Resource Flow Limited Scenario-Class Production Posture Review

**Status:** Accepted (design/readiness review, docs-only)
**Date:** 2026-05-27
**Scope:** Post-RF-T5 audit for whether limited scenario-class `FlatStarResourceFlow` may be treated as the current bounded production Resource Flow posture. This is not global default-on implementation.
**Authority:** [`resource_flow_global_default_on_rereview.md`](resource_flow_global_default_on_rereview.md), [`resource_flow_default_on_readiness_review.md`](resource_flow_default_on_readiness_review.md), [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md), [`accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md), [`todo.md`](../todo.md), [`worklog.md`](../worklog.md), [`workshop_current_state.md`](../workshop/workshop_current_state.md), RF-T5 verification artifact `resource_flow_scenario_class_burn_in_test_results.md` (inspected before artifact cleanup), and RF-T4/RF-T5 driver suites.

**Production code changes in this review:** **None.**

---

## Executive Summary

RF-T1 through RF-T5 completed the narrow Resource Flow enablement ladder:

- RF-T1: spec-level `ResourceFlowOptInMode::FlatStarOptIn`.
- RF-T2: expanded explicit opt-in burn-in.
- RF-T3: product-like opt-in soak and telemetry.
- RF-T4: `ResourceFlowExecutionProfile::FlatStarResourceFlow` scenario-class enablement with `ScenarioClassDefaultOn` attribution.
- RF-T5: scenario-class burn-in / telemetry soak for the RF-T4 path.

**Recommendation: A - accept limited scenario-class `FlatStarResourceFlow` as production-ready in bounded form.**

This means `FlatStarResourceFlow` can be treated as the current production Resource Flow posture for scenario classes that fit the burned-in flat-star slice. It does **not** authorize global default-on.

| Option | Verdict |
|--------|---------|
| **A. Accept limited scenario-class `FlatStarResourceFlow` as production-ready in bounded form** | **Chosen** |
| **B. Require additional RF-T5-style soak before production posture** | Not required for the bounded flat-star slice |
| **C. Pivot to E-11B nested hierarchy GPU** | Deferred unless nested scenarios become the product priority |
| **D. Pivot to D-2a boundary transaction scheduling** | Deferred unless discrete hard-currency ordering becomes urgent |
| **E. Pivot to simthing-spec/RON/Designer guardrail rebuild** | Deferred; useful future product work, not blocking this posture |
| **F. Reconsider global default-on** | **Rejected** |

Global `PipelineFlags::default().use_accumulator_resource_flow` remains **false**. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence over scenario-class enablement.

---

## 1. Current-State Audit After RF-T5

| Layer | Post-RF-T5 posture |
|-------|--------------------|
| Runtime substrate | AccumulatorOp v2 remains the production runtime substrate. |
| Resource Flow ownership | Driver/spec registration over flat AccumulatorOps; no new primitive. |
| Global flag | `PipelineFlags::default().use_accumulator_resource_flow == false`. |
| Spec presence | Populated `ResourceFlowSpec` stages registry/scaffold but does not imply GPU execution. |
| Spec opt-in | `ResourceFlowOptInMode::FlatStarOptIn` remains valid and takes precedence. |
| Scenario-class enablement | `ResourceFlowExecutionProfile::FlatStarResourceFlow` enables the same flat-star path with `ScenarioClassDefaultOn` telemetry. |
| Covered execution | E-11 flat-star, E-2B static enrollment, E-2B-5 Policy A dynamic enrollment, E-2B-5R atomicity/resync. |
| Artifacts | `ArenaRegistry` and `ArenaParticipant` remain driver/session artifacts. |
| simthing-sim | Arena-ignorant; no Resource Flow semantic ownership. |
| Hard currency | Remains separate from continuous Resource Flow. |

**Constitutional posture preserved:** no global default-on; no WGSL changes; no new `AccumulatorRole` variants; no CPU production allocation fallback; no boundary-time slot compaction; no indirection-list `SlotRange` replacement; E-11B deferred by default; Policy B Reevaluate selector re-run deferred; designer/RON/spec guardrail rebuild deferred.

---

## 2. Evidence Summary: RF-T1 Through RF-T5

| Gate | Evidence |
|------|----------|
| **RF-T1** | `ResourceFlowOptInMode::{Disabled, FlatStarOptIn}`; `open_from_spec` applies explicit opt-in; default stays disabled; wildcard/nested claims rejected for flat-star. |
| **RF-T2** | Burn-in fixtures for static, skewed-weight, dynamic fission, two-arena, disabled populated spec, wildcard rejection, resync, and replay paths. |
| **RF-T3** | Product-like explicit opt-in soak; `ResourceFlowOptInTelemetryReport`; flag-source attribution; 128/256 participants; dynamic fission cadence; multi-arena; replay; rejection telemetry; repeated resync. |
| **RF-T4** | `ResourceFlowExecutionProfile` on `GameModeSpec`; `FlatStarResourceFlow` profile maps to `ScenarioClassDefaultOn`; populated spec without profile stays inactive; spec `FlatStarOptIn` precedence preserved. |
| **RF-T5** | Scenario-class burn-in mirrors RF-T3 via `FlatStarResourceFlow`: static 128/256, dynamic fission, multi-arena no coupling, multi-session replay, rejection telemetry, repeated resync, default inactive, transfer/emission unaffected, no WGSL, no simthing-sim arena imports. |

The inspected RF-T5 test report recorded local GPU execution, targeted RF-T5 + RF-T4 + RF-T3/RF-T2/RF-T1 regressions, E-2B/E-11 regressions, `cargo check --workspace`, and `cargo test --workspace` passing.

---

## 3. Production-Ready Bounded Path

The following path is production-ready in bounded form:

1. A product/scenario class explicitly selects `ResourceFlowExecutionProfile::FlatStarResourceFlow`.
2. The authored Resource Flow shape is flat-star D=2 and passes the existing flat-star guardrails.
3. Participation comes from static E-2B enrollment or E-2B-5 Policy A dynamic fission inheritance.
4. Dynamic enrollment uses E-2B-5R prepare/commit atomicity and visible diagnostics.
5. GPU execution uses existing AccumulatorOp v2 flat registrations, with no new WGSL or roles.
6. Telemetry records `ResourceFlowFlagSource::ScenarioClassDefaultOn` and execution profile name.
7. Spec `FlatStarOptIn` remains valid and wins when both spec opt-in and scenario-class profile are present.

This is the current bounded production Resource Flow posture. It is a scenario-class/profile posture, not a global runtime default.

Product/scenario classes that may use it now:

- Flat-star allocation scenarios with bounded participant counts similar to RF-T5 static 128/256 and smaller fixtures.
- Flat-star scenarios with Policy A inherit-only dynamic fission enrollment.
- Multi-arena scenarios without coupled hierarchy semantics, where each arena independently stays in the flat-star covered shape.
- Scenarios that accept continuous-flow approximate-deterministic conservation and do not require exact hard-currency transaction semantics.
- Scenarios that can rely on existing `ScenarioClassDefaultOn` telemetry, sync counts, dynamic admission/rejection counts, and parity/replay burn-in metrics.

---

## 4. Explicit Exclusions / Blocked Paths

| Path | Status |
|------|--------|
| Global `PipelineFlags::default().use_accumulator_resource_flow = true` | **Rejected** |
| GPU execution from `ResourceFlowSpec` presence alone | **Forbidden** |
| Nested hierarchy GPU / E-11B | Deferred by default |
| Policy B `Reevaluate` selector re-run | Deferred |
| Wildcard or dynamic selector expansion beyond bounded flat-star fixtures | Blocked |
| Boundary-time slot compaction | Forbidden |
| Indirection-list replacement for `SlotRange` | Forbidden |
| New WGSL or new `AccumulatorRole` variants | Forbidden |
| CPU production allocation fallback | Forbidden |
| `simthing-sim` importing or owning `ArenaRegistry`, `ArenaParticipant`, or Resource Flow semantics | Forbidden |
| Hard-currency transfer through continuous Resource Flow | Forbidden |
| Coupling-heavy product graphs | Not production-ready under this posture |
| Immediate designer/RON guardrail redesign inside the RF track | Deferred to future simthing-spec rebuild |

Product/scenario classes that must remain blocked:

- Nested hierarchy D>2 allocation scenarios.
- Scenarios requiring fission-time selector re-evaluation instead of Policy A inheritance.
- Wildcard-heavy or unbounded dynamic expansion scenarios.
- Coupled arena graphs that rely on cross-arena delay semantics at product scale.
- Exact hard-currency transaction scenarios.
- Scenarios whose designer-facing authoring safety depends on guardrail UX not yet rebuilt.

---

## 5. Telemetry and Diagnostics Status

RF-T5 telemetry is sufficient to diagnose the accepted bounded scenario-class path:

- Flag source distinguishes `DefaultDisabled`, `SpecFlatStarOptIn`, `ScenarioClassDefaultOn`, and test override cases.
- Execution profile name records `FlatStarResourceFlow` vs `DefaultDisabled`.
- Telemetry reports planned arenas/participants, total ops, bands, sync count, generation movement, dynamic admissions/rejections, max absolute error, and replay bit-exactness where burn-in reports are attached.
- Negative-path fixtures prove populated specs remain inactive under the default profile.
- RF-T5 explicitly checks that scenario-class Resource Flow does not enable transfer/emission flags.

Telemetry is **not** sufficient to justify global default-on, nested hierarchy production, Policy B, wildcard expansion, or coupling-heavy products. Those paths need new gate-specific observability and soak.

---

## 6. Risk Register

| Risk | Current disposition |
|------|---------------------|
| Authors confuse populated `ResourceFlowSpec` with execution | Mitigated by default-disabled telemetry and docs; product UX still needs future guardrail work. |
| Scenario-class enablement overclaimed as global default-on | Mitigated by explicit recommendation A and repeated flag-default invariant. |
| Flat-star constraints drift as product scenarios grow | Stop condition: require E-11B or new review if nested/coupled/wildcard behavior is needed. |
| Dynamic enrollment surprises under fission | Policy A inherit-only is covered; Policy B remains deferred and must not be implied. |
| Approximate continuous conservation mistaken for exact transfer | Hard-currency transfer remains separate; exact discrete transfer uses Phase T paths. |
| Telemetry gaps during operations | Bounded path has enough attribution and sync/admission/rejection data; production polish can improve reporting without changing semantics. |
| Stale historical test artifacts obscure current state | RF-T5 local test artifacts are deleted in this PR after inspection; formal reviews and active docs remain. |

---

## 7. Recommendation A/B/C/D/E/F

**Chosen: A - accept limited scenario-class `FlatStarResourceFlow` as production-ready in bounded form.**

Rationale:

- RF-T4 implemented the scenario-class enablement surface without changing the global flag default.
- RF-T5 soaked that exact path with product-like fixtures and diagnostics.
- The bounded path preserves every v7.5 stop condition.
- Additional RF-T5-style soak would be useful only as ongoing confidence, not as a blocker for this bounded production posture.

**Explicitly reject F.** Global default-on is still rejected because uncovered paths remain: E-11B, Policy B, wildcard/dynamic selector expansion, coupling-heavy graphs, hard-currency semantics, and designer/RON guardrail gaps.

---

## 8. Next-Gate Options

| Gate | Recommendation |
|------|----------------|
| **RF-T6 production docs / telemetry polish** | Best default next implementation gate after Recommendation A. Tighten product docs, scenario-class naming, operational telemetry presentation, and regression routing without changing runtime behavior. |
| **E-11B nested hierarchy GPU** | Choose only if nested static Resource Flow scenarios become the product priority. |
| **D-2a boundary transaction scheduling** | Choose only if discrete hard-currency ordering/contention becomes urgent. |
| **simthing-spec/RON/Designer guardrail rebuild** | Choose when product authoring UX becomes the priority; not required for bounded runtime posture. |
| **Continued soak** | Optional confidence work for larger flat-star participant counts; not required for Recommendation A. |
| **Global default-on** | Do not choose now. Reconsider only after new review evidence covers excluded paths or permanently hard-blocks them. |

Additional RF work worth doing before global default-on is reconsidered:

- E-11B or hard rejection of nested claims at all global-enable boundaries.
- Policy B Reevaluate implementation or explicit product-level exclusion.
- Coupling-heavy soak with delay forms, if global posture would cover coupled arenas.
- Designer/RON guardrail rebuild so authors cannot accidentally request uncovered semantics.
- Broader telemetry for product operations, including clear execution-intent vs staged-spec diagnostics.

---

## 9. Required Tests Before Next Implementation

For RF-T6 docs/telemetry polish:

- Keep RF-T4/RF-T5 scenario-class suites green.
- Keep RF-T1/RF-T2/RF-T3 opt-in regressions green.
- Keep E-2B enrollment and E-2B-5 dynamic enrollment soak green.
- Keep E-11 flat-star soak green.
- Keep `PipelineFlags::default().use_accumulator_resource_flow == false` guarded.
- Keep populated spec + default profile inactive coverage.
- Keep spec `FlatStarOptIn` precedence coverage.
- Keep no-WGSL/no-new-role/no-simthing-sim-arena-import static guards.

For E-11B, D-2a, or simthing-spec/RON rebuild, write a fresh gate-specific test plan before implementation. The accepted bounded posture must not be broadened accidentally by those gates.

---

## 10. Stop Conditions / Opus Escalation Triggers

Stop and recommend Opus review if any next step requires:

- Global `PipelineFlags::default().use_accumulator_resource_flow = true`
- Execution inferred from `ResourceFlowSpec` presence
- E-11B nested GPU as a prerequisite for the bounded flat-star posture
- Policy B Reevaluate selector re-run
- Wildcard or dynamic selector expansion
- New WGSL
- New `AccumulatorRole` variants
- CPU production allocation fallback
- `simthing-sim` arena awareness
- Boundary-time slot compaction
- Indirection-list `SlotRange` replacement
- Hard-currency transfer through Resource Flow
- Immediate designer/RON guardrail redesign inside the RF track

Any of those would exceed Recommendation A and require a separate design/review gate.

---

## 11. Docs Update Requirements

This review updates:

- `docs/accumulator_op_v2_production_plan.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/workshop/workshop_current_state.md`

Required posture wording:

Resource Flow limited scenario-class production posture review landed. No production code changes. RF-T1 through RF-T5 remain landed. Limited scenario-class `FlatStarResourceFlow` is accepted as the current bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new `AccumulatorRole` variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild remains deferred to the future simthing-spec rebuild track. Next recommended gate is RF-T6 production docs/telemetry polish, with E-11B, D-2a, simthing-spec/RON rebuild, or continued soak available by product priority.

---

## 12. Review Question Index

| # | Question | Answer |
|---|----------|--------|
| 1 | What exactly is now production-ready after RF-T5? | Limited scenario-class `FlatStarResourceFlow` for flat-star D=2 Resource Flow using E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment. |
| 2 | What remains explicitly not production-ready? | Global default-on, spec-presence inference, E-11B, Policy B, wildcard/dynamic selector expansion, coupling-heavy graphs, hard-currency via Resource Flow, slot compaction, indirection `SlotRange`, CPU fallback, simthing-sim arena awareness. |
| 3 | Does `FlatStarResourceFlow` scenario-class enablement meet v7.5 boundaries? | **Yes**, in bounded form; it uses existing AccumulatorOp registrations, preserves explicit participation/caps, and keeps Resource Flow out of simthing-sim. |
| 4 | Is current telemetry enough to diagnose scenario-class RF behavior? | **Yes** for the bounded path; not for global/nested/coupled/Policy B paths. |
| 5 | Is global default-on still rejected? | **Yes**. |
| 6 | What product/scenario classes may use `FlatStarResourceFlow` now? | Flat-star, bounded, non-hard-currency continuous-flow scenarios using static E-2B and/or Policy A dynamic enrollment, including independently flat multi-arena no-coupling cases. |
| 7 | What product/scenario classes must remain blocked? | Nested, Policy B, wildcard/unbounded, coupling-heavy, exact hard-currency, and guardrail-dependent authoring surfaces. |
| 8 | What additional RF work before global default-on is reconsidered? | E-11B or hard nested exclusion, Policy B decision, coupling soak if in scope, designer/RON guardrails, broader product telemetry, and a new global-readiness review. |
| 9 | Should the next implementation gate be RF-T6, E-11B, D-2a, or simthing-spec/RON rebuild? | Default next gate: RF-T6 production docs/telemetry polish. Choose E-11B, D-2a, or simthing-spec/RON rebuild only by product priority. |
| 10 | What tests/docs are required before that next gate? | Preserve RF-T1..T5, E-2B, E-2B-5, E-11, flag-default, inactive-default, precedence, no-WGSL/no-role/no-simthing-sim guards; update production plan/todo/worklog/current state. |

---

## References

- Prior global/default-on re-review: [`resource_flow_global_default_on_rereview.md`](resource_flow_global_default_on_rereview.md)
- Prior readiness review: [`resource_flow_default_on_readiness_review.md`](resource_flow_default_on_readiness_review.md)
- Substrate ADR: [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md)
- Production plan: [`accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md)
- RF-T5 implementation: [`resource_flow_scenario_class_burn_in.rs`](../../crates/simthing-driver/src/resource_flow_scenario_class_burn_in.rs)
- RF-T5 tests: [`resource_flow_scenario_class_burn_in.rs`](../../crates/simthing-driver/tests/resource_flow_scenario_class_burn_in.rs)
- RF-T4 tests: [`resource_flow_scenario_class_default_on.rs`](../../crates/simthing-driver/tests/resource_flow_scenario_class_default_on.rs)

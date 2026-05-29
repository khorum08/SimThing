# SimThing Todo Log

Current parking state: **`simthing-spec` PRs 1–11 complete**; v6 Opus P0 (O2/B3/I1) complete;
**AccumulatorOp v2 Phases A–B** complete through B-3 (#95); **Phase C** in progress — C-1 (#97–#98),
**C-2** (#99–#100), **C-3** (#105–#107), **pivot-forward policy + B-4I** (#108),
**C-INF runtime/oracle** (#109), **pivot-forward remedial** (#111), and
**C-4 overlay OrderBand** (#118), **C-5 soft reductions** (#122–#123), **C-6 exact reductions** (#124),
**S-4 reduction sunset** (#126), **C-7 velocity** (#127), **C-8 EML block** (#129–#137),
**S-2 intensity sunset** (#138), **S-3 overlay sunset** (#141), **S-6 threshold sunset**,
**S-5 velocity sunset**, and **S-1 intent sunset** (`6b9bf8f`) landed.
`master` @ **`6975b93`** (post-sunset cleanup #143).

**Reduction flags (default true):** `use_accumulator_reduction_soft` +
`use_accumulator_reduction_exact` (both required). AccumulatorOp is the sole production
reduction path after **S-4**; legacy `reduction.wgsl` deleted.

**EML + intensity flags (default true):** `use_accumulator_eml` +
`use_accumulator_intensity`. Legacy `intensity_update.wgsl` deleted after **S-2**;
EvalEML is the only production intensity path.

**S-4 landed:** legacy reduction shader/pipeline/fallback removed; AccumulatorOp covers all
reduction rules; CPU oracle retained for test golden only.

**Phase M SummaryValidity V1-R1 hygiene + parking verification (completed):** Full targeted first-slice verification + workspace check green. Runtime status confirmed driver-owned (`FirstSliceSummaryStatus`). Designer policy remains in spec. All V7.7 guardrails intact. Parked.

**S-2 landed:** legacy intensity shader/pipeline deleted; EvalEML intensity only.

**Workshop entry point:** [`docs/workshop/workshop_current_state.md`](workshop/workshop_current_state.md)

**Pivot posture:** AccumulatorOp v2 is the production runtime. Legacy reduction (S-4),
legacy intensity (S-2), legacy overlay (S-3), legacy threshold (S-6), legacy velocity (S-5),
and legacy intent (S-1) are **deleted**. Snapshot is the only retained non-Accumulator
operation. See
[`docs/workshop/pivot_forward_implementation_policy.md`](workshop/pivot_forward_implementation_policy.md).

**Parking synthesis:** [`docs/design_v7.md`](design_v7.md) — AccumulatorOp v2 target architecture.
Historical v6.5 parking: [`docs/design_v6.5.md`](design_v6.5.md).

**Tests:** `cargo test --workspace` green after S-6/S-5/S-1 (450+ passed, ignored perf gates).
AccumulatorOp module: **72** gpu `accumulator_op` unit tests; `reduction_orderband` (6);
C-1/C-2/C-3 parity (26) + C-4 parity/cache (16) + S-3 overlay sunset (5) + C-5 reduction (11) + C-6 exact (10) +
C-INF-2 harness (2) + pivot-forward remedial (3) + B-4 world summary integrated (2).

**Cursor handoff:** AccumulatorOp v2 Phase C migrations + pivot-forward infrastructure (see table below).

**Canonical AccumulatorOp v2 progress:** `docs/accumulator_op_v2_production_plan.md` ·
`docs/adr_accumulator_op_v2.md` · `docs/design_v7.md` · `docs/worklog.md` ·
`docs/workshop/pivot_forward_implementation_policy.md`

**Canonical spec progress (v6 parking):** `docs/design_v6.5.md` ·
`docs/workshop/simthing_spec_progress_log.md` (PR ledger) · `docs/worklog.md` (session notes)

### AccumulatorOp v2 — Phases A–B (2026-05-19)

| PR | GitHub | Commit | Scope |
|----|--------|--------|-------|
| **A-4** | #90 | `cb33006` | Soft-aggregate tolerance — Opus audit, `SoftAggregateGuard`, threshold validator |
| **B-1** | #91 | `afff3b6` | `AccumulatorOpSession` persistent buffers + bootstrap kernel |
| **B-1 fix** | #92 | `f167e5c` | Scale encoding, contention rejection, clamped transfer, provisional readback tiers |
| **B-2** | #93 | `02e40eb` | EmitEvent, atomic emission count, overflow reporting, CPU oracle emissions |
| **B-2 fix** | #94 | `2633970` | Always gate wildcard contention validation |
| **B-3** | #95 | `d9fabf9` | Optional `TIMESTAMP_QUERY` plumbing, `last_pass_time_us()`, feature-detected fallback |

**Earlier A-phase:** A-1 docs (#86–#87), A-2 types (#88), A-3 EML registry (#89).

**B-phase complete through B-3:** kernel subset + Always wildcard validation + optional execute-pass timestamps (instrumentation only).

### AccumulatorOp v2 — Phase C (migration, feature-flagged)

| PR | GitHub | Scope |
|----|--------|-------|
| **C-1** | #97 | Pass 7 threshold scan → AccumulatorOp `Threshold` + `EmitEvent`; S-6 deleted legacy shader/pipeline and defaulted accumulator path |
| **C-1 refine** | #98 | Single-submission pipeline integration; Opus perf reframe (`docs/workshop/c1_perf_reframe_memo.md`); no-regression readback gate |
| **C-2** | #99 | Intent delta application → `COMBINE_AFFINE_INTENT`; S-1 deleted legacy shader/pipeline and defaulted accumulator path |
| **C-2 refine** | #100 | `finish_intent()` timestamp; `TickGpuError::AccumulatorThresholdReadback`; registry growth clears accumulator sessions |
| **Pivot-forward Fixes** | #102 | Fixes 1–6: narrow contention validator, encode all combine/source stubs, `Threshold+None`, single-submit reduction, atomic WGSL values |
| **C-3** | #105 | Overlay Add → AccumulatorOp; `use_accumulator_overlay_add` (default false); Add-only batches |
| **C-3 refine** | #106 | Mixed Add/Mul/Set → full legacy Pass 3 fallback (no split-mode) |
| **C-3 OrderBand** | #107 | Per-cell OrderBand sequencing for exact f32 Add order; multi-band dispatch fix |
| **C-4** | #118 | Full Add/Mul/Set overlay → AccumulatorOp OrderBand compiler; dirty/cached rebuild |
| **C-4 remedial** | #120 | Lifecycle/fission/cache hardening; combined C-1/C-2/C-4 path; consume-mode regressions |
| **S-3** | #141 | — | Legacy overlay deleted; AccumulatorOp OrderBands sole overlay path |
| **Pivot-forward + B-4I** | #108 | `2aa630e` | Pivot-forward policy; production `SlotSummaryGpu`; C-INF scaffolds |
| **C-INF-1 + C-INF-2** | #109 | `2f95c6d` | `WorldAccumulatorRuntime` on `WorldGpuState`; legacy oracle harness + tests |
| **Pivot-forward remedial** | #111 | `632d656` | Authoritative flags; `WorldSummaryRuntime`; oracle tolerance rename |
| **C-5** | #122 | Mean/WeightedMean soft reductions → `ReductionSoft` on `output_vectors` |
| **C-5 remedial** | #123 | Depth-interleaved soft/exact reduction; WeightedMean dependency tests |
| **C-6** | #124 | `a414a62` | Sum/Max/Min/First exact reductions; `use_accumulator_reduction_exact` |
| **S-4** | #126 | `208e5a2` | Legacy reduction deleted; AccumulatorOp sole path; flags default on |
| **C-7** | #127 | — | GovernedPair velocity → AccumulatorOp `IntegrateWithClamp`; dt in tick params |
| **C-8a** | #129 | — | EML infrastructure; persistent GPU program table |
| **C-8a remedial** | #130 | — | Program-table accounting, admissibility hardening |
| **C-8b** | #131 | — | Intensity → EvalEML; `use_accumulator_intensity` default on |
| **C-8b remedial** | #132 | — | Intensity op upload cache invalidation |
| **C-8c** | #133 | — | Transfer substrate + input-list table |
| **C-8c remedial** | #134 | — | Same-band consumed-input contention rejection |
| **C-8d** | #135 | — | Emission substrate (ExactDeterministic baseline) |
| **C-8d remedial** | #136 | — | Emission op signature + max_emit rejection |
| **C-8 completion gate** | #137 | — | Full C-8 all-flags integration |
| **S-2** | #138 | — | Legacy intensity deleted; EvalEML only |
| **S-6** | `6b9bf8f` | — | Legacy threshold scan deleted; AccumulatorOp threshold mandatory for threshold workloads |
| **S-5** | `6b9bf8f` | — | Legacy velocity deleted; AccumulatorOp velocity mandatory for governed velocity workloads |
| **S-1** | `6b9bf8f` | — | Legacy intent deleted; AccumulatorOp intent mandatory for pending intent workloads |
| **E-1** | #144 | — | **Done** — EmitOnThreshold builder; threshold+EmitEvent registration API over C-1 substrate |
| **E-2A** | #146 | — | **Done** — `resource_transfer_discrete` exact discrete transfer builder |
| **E-3** | #147 | — | **Done** — `conjunctive_recipe` builder + N-input cap lift |
| **E-3R** | #148 | — | **Done** — throttle hint metadata hardening before E-4 |
| **E-7** | #149 | — | **Done** — `governed_by` planner generalization to arbitrary Named pairs |
| **E-8** | #150 | — | **Done** — `accumulator_spec` compile-time metadata on `SubFieldSpec` |
| **E-9** | #151 | — | **Done** — `ArenaRegistry` driver session artifact |
| **E-9R** | #152 | — | **Done** — participant_range contiguity hardening |
| **E-10** | #153 | — | **Done** — Resource Flow admission framework + expansion report |
| **E-10R** | — | — | **Done** — driver participant identity preflight + reserved-gap admission |
| **E-10R2** | — | — | **Done** — ArenaParticipant SimThing scaffold + contiguity/gap tests |
| **E-10R3** | — | — | **Done** — arena-local gap block reservation + post-materialize capacity check |
| **E-8R** | — | — | **Done** — arena-internal plumbing columns at property compile |
| **E-7R** | — | — | **Done** — `plan_governed_integration_at_band` ordering API |
| **E-11 design** | — | — | **Accepted** — Opus v2 memo landed |
| **E-11 review** | — | — | **Done** — readiness review PASS; handoff published |
| **E-11** | `8a628ca` | #159 | **Done** — flat-star D=2 vertical slice; `e11_*` 14/14; flag default false |
| **E-11R** | `8939fc6` | #160 | **Done** — sync errors, scope honesty, session-path test |
| **E-11 burn-in** | `ae75d8e` | #161 | **Done** — controlled flat-star scaffold; `e11_burn_in_*` 4/4; flag default false |
| **E-11 burn-in scenarios** | — | — | **Done** — named fixtures + `e11_burn_in_scenarios_*` 6/6; flag default false |
| **E-11 CI soak** | — | — | **Done** — opt-in soak `e11_resource_flow_soak` 6/6; `ResourceFlowSoakSummaryReport` |
| **T-1** | — | #165 | **Done** — `resource_economy` authoring types + RON roundtrip 12/12 |
| **T-2** | `986bc99` | #166 | **Done** — `compile::resource_economy` validation + expansion report |
| **T-3** | `05f8b10` | #167 | **Done** — `resource_economy_compile` materialization + stable emission `reg_idx` |
| **T-4** | `92733c2` | #168 | **Done** — session integration + boundary refresh; generation-keyed skip; flag-off populated-spec rejection |
| **T-5** | `91bdae3` | #169 | **Done** — boundary refresh / replay / 100-tick conservation burn-in |
| **T-6** | `3294e6f` | direct commit | **Done** — limited opt-in scenario flagging; global transfer/emission flags remain default false |
| **Phase T RON smoke addendum** | — | — | **Done** — designer-authored `resource_economy` RON fixture exercises deserialize/compile/install/open_from_spec |
| **E-11B kickoff** | — | — | **Done** — nested D=3/D=4 static Resource Flow hierarchy materialization with GPU parity coverage |
| **E-11B-4 fission/gap hardening** | — | — | **Done** — nested reserved-gap preservation tests + diagnostics (`e11b_nested_fission_gap` 13/13) |
| **E-11B nested dynamic enrollment readiness** | — | — | **Done** — [`e11b_nested_dynamic_enrollment_readiness.md`](reviews/e11b_nested_dynamic_enrollment_readiness.md); defer by default |
| **E-11B pause checkpoint** | — | — | **Done** — E-11B paused after kickoff + E-11B-4 + readiness review; E-11B-5 not authorized without named scenario |
| **Continued flat-star soak** | — | — | **Done** — `resource_flow_flat_star_continued_soak` 12/12; 512 static @ 1000 ticks (local GPU) |
| **Product-priority vertical slice selection** | — | — | **Done** — [`product_priority_vertical_slice_selection.md`](reviews/product_priority_vertical_slice_selection.md); **Recommendation F:** pause |

**Next recommended action:** **Pause implementation** and gather product requirements per selection review. Do not open D-2a, E-11B-5, spec/RON rebuild, new vertical slice, or additional soak until product names a scenario and re-selects track A–E.

**Deferred tracks (require named scenario before authorization):**

1. **Narrow D-2a** — named multi-transaction hard-currency ordering scenario
2. **Narrow E-11B-5** — named nested dynamic Resource Flow scenario
3. **simthing-spec/RON/Designer guardrail rebuild** — authoring track intentionally opens
4. **New scenario-driven vertical slice (A)** — product brief + FlatStarResourceFlow / Phase T fixture plan
5. **Additional flat-star soak (E)** — specific evidence gap named
6. **Resource Flow global default-on** — remains deferred and rejected

**E-11B track status:** **Paused (not abandoned).**

**Open design warnings (preserve):**
- **Product-priority vertical slice selection checkpoint landed.** No production code changes. Continued flat-star Resource Flow soak remains landed and green. FlatStarResourceFlow remains the accepted bounded production posture. **Recommendation F:** pause implementation and gather product requirements. E-11B remains paused; E-11B-5 requires a named nested dynamic Resource Flow scenario. D-2a remains deferred until a named hard-currency ordering scenario exists. simthing-spec/RON/Designer guardrail rebuild remains deferred. Global flag default false. Resource Flow separate from Phase T. Next implementation gate depends on review recommendation after product names a scenario.
- **Continued flat-star Resource Flow soak checkpoint landed.** FlatStarResourceFlow remains the accepted bounded production posture. The checkpoint adds confidence/observability only and does not expand Resource Flow semantics. E-11B remains paused; E-11B-5 is not authorized without a named nested dynamic Resource Flow scenario. D-2a remains deferred until a named hard-currency scenario needs sequential cross-band ordering. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec FlatStarOptIn remains supported and takes precedence. Resource Flow remains separate from Phase T hard-currency transfer/recipe/emission. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant and spec-free. Designer-facing spec/RON guardrail rebuild remains deferred.
- **E-11B paused** after nested static GPU parity, fission/gap hardening, and nested dynamic enrollment readiness review. Nested D=3/D=4 static hierarchy materialization remains landed and GPU-parity covered. Nested reserved-gap children remain non-leaf unless explicitly admitted by a future nested enrollment gate. Nested dynamic enrollment is deferred until a named product scenario requires it. Future E-11B-5 must be narrow: explicit nested admission, contiguous extension or visible rejection, generation/sync reporting, replay/parity tests. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Policy B Reevaluate remains deferred. D-2a remains deferred until a named hard-currency product scenario needs sequential cross-band ordering. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Resource Flow remains separate from hard-currency transfer. Designer-facing spec/RON guardrail rebuild remains deferred.
- **E-11B nested fission/gap hardening landed:** reserved-gap children preserve active child SlotRange and do not become allocation leaves unless explicitly admitted by a future nested enrollment gate. D=3/D=4 nested GPU parity remains green for active trees after safe gap claims. FlatStarResourceFlow remains the accepted bounded production posture. E-11B remains an explicit nested extension and does not make Resource Flow global default-on. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Policy B Reevaluate remains deferred. D-2a remains deferred until a named hard-currency product scenario needs sequential cross-band ordering. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Resource Flow remains separate from hard-currency transfer. Designer-facing spec/RON guardrail rebuild remains deferred.
- **E-11B nested hierarchy GPU slice landed:** nested D=3/D=4 static Resource Flow hierarchy materialization now has GPU parity coverage.
- **D-2a boundary transaction scheduling readiness review landed:** [`d2a_boundary_transaction_scheduling_readiness.md`](reviews/d2a_boundary_transaction_scheduling_readiness.md). No production code changes. Phase T remains complete. Phase T designer/RON smoke addendum remains landed. Hard-currency transfer remains exact discrete AccumulatorOp transfer/recipe/emission. Resource Flow remains separate from hard-currency transfer. Bounded `FlatStarResourceFlow` posture remains unchanged. Global Resource Flow default-on remains deferred. **Recommendation: defer D-2a implementation** — current same-band collision rejection is sufficient for shipped workloads; `order_band` wiring gap documented. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains spec-free and semantic-free. Designer/RON/spec guardrail rebuild remains deferred.
- Transfer/emission registration ownership: **Phase T complete.** T-6 landed limited opt-in scenario flagging for resource economy transfer/emission. Global transfer/emission flags remain default false. Only explicitly opted-in scenarios enable AccumulatorOp transfer/emission paths. T-5 burn-in remains green. No WGSL changes. No CPU fallback. `simthing-sim` remains spec-free and semantic-free. Hard-currency transfers remain on exact discrete AccumulatorOp transfer paths, not Resource Flow.
- **Phase T designer/RON smoke addendum landed:** a designer-authored `resource_economy` RON fixture now exercises `deserialize_game_mode_ron` → compile/install/`open_from_spec`. Transfer, recipe, and emission authoring remain explicit `ResourceEconomySpec` content. `ResourceEconomyOptInMode` remains default disabled. Global transfer/emission flags remain default false. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains spec-free and semantic-free. Resource Flow bounded `FlatStarResourceFlow` posture remains unchanged. Full simthing-spec/RON/Designer guardrail rebuild remains deferred to its own future track.
- **RF-T6 landed:** production docs / telemetry polish for bounded `FlatStarResourceFlow` posture. [`resource_flow_limited_scenario_class_posture.md`](resource_flow_limited_scenario_class_posture.md) documents accepted scenario classes, blocked paths, telemetry fields, flag-source interpretation, and operator/debug checklist. Limited scenario-class `FlatStarResourceFlow` remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild remains deferred to the future simthing-spec rebuild track.
- **Resource Flow limited scenario-class production posture review landed:** [`resource_flow_limited_scenario_class_production_posture.md`](reviews/resource_flow_limited_scenario_class_production_posture.md). No production code changes. RF-T1 through RF-T5 remain landed. **Recommendation A:** limited scenario-class `FlatStarResourceFlow` is accepted as the current bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild remains deferred to the future simthing-spec rebuild track. Next gate depends on product priority: RF-T6 production docs/telemetry polish (recommended), E-11B, D-2a, simthing-spec/RON rebuild, or continued soak.
- **RF-T5 landed:** scenario-class Resource Flow burn-in / telemetry soak. `ResourceFlowExecutionProfile::FlatStarResourceFlow` was soaked through product-like static, dynamic, multi-arena, replay, rejection, and resync scenarios. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence. Scenario-class enablement records distinct flag-source telemetry and execution-profile attribution. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild remains deferred to the future simthing-spec rebuild track.
- **RF-T4 landed:** limited scenario-class Resource Flow default-on implementation.
- **Resource Flow global/default-on readiness re-review landed:** [`resource_flow_global_default_on_rereview.md`](reviews/resource_flow_global_default_on_rereview.md). No production code changes. **Recommendation B:** proceed to RF-T4 limited scenario-class default-on; global default-on rejected (D). RF-T1 scenario-class opt-in, RF-T2 opt-in burn-in expansion, and RF-T3 product-like opt-in soak + telemetry remain landed. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant.
- **RF-T3 landed:** product-like opt-in Resource Flow soak and telemetry surfacing. FlatStarOptIn scenarios now emit/record flag-source, sync, arena, participant, generation, dynamic admission/rejection, and parity/replay metrics. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant.
- **RF-T2 landed:** limited opt-in scenario burn-in expansion for Resource Flow. Only explicitly authored `FlatStarOptIn` scenarios enable GPU Resource Flow execution. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant.
- **RF-T1 landed:** limited scenario-class Resource Flow opt-in flagging. `ResourceFlowOptInMode` on `ResourceFlowSpec` enables `FlatStarOptIn` per authored scenario/game mode. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU Resource Flow execution. E-11 flat-star path, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment are reused. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant.
- **Resource Flow default-on readiness review landed:** [`resource_flow_default_on_readiness_review.md`](reviews/resource_flow_default_on_readiness_review.md). No production code changes. **Recommendation B:** limited scenario-class default-on readiness may proceed; global default-on rejected. E-2B static enrollment, E-2B-5 Policy A, E-2B-5R atomicity, and dynamic enrollment soak remain landed. `use_accumulator_resource_flow` remains default false. E-11B remains deferred by default. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant.
- **Resource Flow dynamic enrollment soak landed:** [`resource_flow_dynamic_enrollment_soak.rs`](../crates/simthing-driver/src/resource_flow_dynamic_enrollment_soak.rs) + [`e2b5_dynamic_enrollment_soak.rs`](../crates/simthing-driver/tests/e2b5_dynamic_enrollment_soak.rs) (PR #178). E-2B-5R remained atomic under soak.
- **E-2B static enrollment compilation landed:** Resource Flow enrollment selectors resolve to explicit participants at session install ([`EnrollmentSelectorSpec`](../crates/simthing-spec/src/spec/resource_flow.rs), [`resolve_resource_flow_enrollment`](../crates/simthing-driver/src/resource_flow_enrollment.rs)). No legacy `resource_flow_participant` AccumulatorOp builder. E-10R/E-10R2/E-10R3 scaffold and E-11 flat-star execution reused unchanged.
- **E-11B readiness review landed:** nested hierarchy GPU execution/materialization current-state audit ([`e11b_nested_hierarchy_gpu_readiness_review.md`](reviews/e11b_nested_hierarchy_gpu_readiness_review.md)). No production code changes. E-11B deferred by default.
- **D-1 memo landed:** discrete-transaction contention current-state audit ([`d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md)). No production code changes.
- Shared-input transfer contention: C-8c rejects same-band consumed-input contention; T-2 compile rejects same-band authoring collisions.
- Soft/Fast EML: future-gated; production admits `ExactDeterministic` only.

**E-2B:** **Done (static E-2B-1…4 + E-2B-5 Policy A + E-2B-5R + soak).**

**E-11B-1:** **Done** — explicit nested participant materialization landed. `ExplicitParticipantSpec.parent_subtree_root_id` enables static nested Resource Flow participant authoring. `materialize_arena_participants` builds nested `ArenaParticipant` topology and preserves per-parent child contiguity. Narrow static materialization fix for provisional deep-arena / mapping hierarchy authoring only — no mapping runtime, dynamic nested enrollment, WGSL, new roles, CPU fallback, Policy B, or slot compaction. E-11B-5 remains paused.

**E-11B RON smoke:** **Done** — static nested participant RON smoke landed. `parent_subtree_root_id` remains an optional static authoring field. RON-authored D=3/D=4 explicit nested participant specs materialize into nested `ArenaParticipant` topology and reach `build_nested_layout`. Flat-star Resource Flow authoring remains backwards-compatible when `parent_subtree_root_id` is omitted. Pending mapping/location work may use static deep hierarchy materialization later, but no mapping runtime behavior was implemented.

**RegionCell field-intelligence sandbox:** **Reverted** — PR #197 removed after external concept validation. Implementation remains parked after E-11B-1 and E-11B RON smoke. Static deep hierarchy authoring via `parent_subtree_root_id` remains landed. The sparse RegionCell field-intelligence sandbox was reverted after validating the concept externally; no sandbox test/prototype remains in the production repo. Mapping/location architecture remains provisional. Do not implement mapping/location runtime until the mapping doc is ready. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. `simthing-sim` remains arena-ignorant and spec-free.

**SEAD field-intelligence sandbox:** **Reverted** — source archived at `docs/workshop/archive/sead/sead_sandbox_code_preserve.rs`; decision-gate results at `docs/tests/sead_field_intelligence_sandbox_test_results.md`. Evidence informs approved Mapping ADR.

**SEAD strategic horizon / velocity / PF-skip sandbox:** **Reverted** — PR #202 probe merged then reverted to parked state. SEAD strategic horizon / velocity / PF-skip feasibility sandbox completed and was reverted to parked state. The sandbox source and decision-gate results are preserved in `docs/workshop` and `docs/tests`. No sandbox test/prototype remains in the production test suite. Overall probe verdict: **PARTIAL**. Mapping/location architecture remains provisional. Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution.

**SEAD operator toolkit sandbox:** **Reverted** — PR #204 probe merged then reverted to parked state. SEAD operator toolkit sandbox completed and was reverted to parked state. The sandbox source and decision-gate results are preserved in `docs/workshop` and `docs/tests`. No sandbox test/prototype remains in the production test suite. Overall probe verdict: **PARTIAL**. Mapping/location architecture remains provisional. Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario. No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, new WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution.

**SEAD tensor/stencil WGSL prototype sandbox:** **Reverted** — PR #206 probe merged then reverted to parked state. SEAD tensor/stencil WGSL prototype sandbox completed and was reverted to parked state. The sandbox source, WGSL prototype(s), notes, and decision-gate results are preserved in `docs/workshop` and `docs/tests`. No sandbox test/prototype remains in the production runtime/test suite. Overall probe verdict: **PARTIAL**. Mapping/location architecture remains provisional. Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario. No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, production WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution.

**SEAD tensor/stencil WGSL refinement sandbox:** **Reverted** — PR #208 probe merged then reverted to parked state. Preserved in `docs/workshop` and `docs/tests`. Workshop prototype promoted to live `StructuredFieldStencilOp` under V7.6 (separate PR).

**V7.6 constitution pivot:** **Done** — V7.6 supersedes V7.5 development guardrails where SEAD tensor/stencil and EML admission probes proved the old constraints too restrictive. The old "no new WGSL" guardrail is replaced with "no semantic/map-specific WGSL." Generic deterministic tensor kernels are allowed when bounded, replayable, opt-in, and semantic-free. The old EML whitelist posture is replaced with designer/RON-facing admission for deterministic bounded field formula classes. `StructuredFieldStencilOp` is promoted as a live generic GPU primitive. It is not mapping runtime. Default tactical horizon H≤8; execution cannot bypass configured horizon; ping-pong for H>1; caller-managed source policy; active mask provisional pending halo semantics. Column-aware reduction into parent EML input columns is the approved hybrid bridge. Mapping/location architecture remains provisional.

**V7.6 StructuredFieldStencilOp guardrail hardening:** **Done** — enforces execution horizon constraints, fixes source-cap test indexing, clarifies caller-managed source policy, adds clamp-boundary parity, marks active mask provisional. No production pass graph wiring; Resource Flow defaults unchanged.

**V7.7 Mapping ADR:** **Done** — [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md) approved at architecture level; surfaced in [`design_v7_7.md`](design_v7_7.md); invariants updated. Phase M generic natives unlocked. No mapping runtime authorized.

**Docs cleanup (pre-Phase M):** **Done** — superseded artifacts archived; authoritative mapping guidance: ADR + V7.7 + invariants + [`mapping_current_guidance.md`](workshop/mapping_current_guidance.md).

**Phase M-1:** **Done** — generic `StructuredFieldStencilOp::execute_configured` execution API and column-aware reduction convenience over existing `SlotRange` Sum.

**Phase M-1.1:** **Done** — no-readback configured execution path.

**Phase M-2:** **Done** — generic cadence scheduler and dirty macro-region skip helper.

**Phase M-2.1:** **Done** — FieldScheduler region identity `(FieldId, FieldRegionId)`; visitor-based scheduled execution; single-op guard.

**Phase M-3:** **Done** — RegionFieldSpec RON + mapping admission framework. RegionFieldSpec is designer/spec structure only and compiles/previews to generic substrate configs. MappingExecutionProfile remains default Disabled; spec presence alone does not enable execution.

**Phase M-first-slice:** **Done (opt-in)** — `FirstSliceMappingSession` in `simthing-driver`; explicit `MappingExecutionProfile::SparseRegionFieldV1` only; not default session wiring. RegionField VRAM budget preview in `simthing-spec`. See [`phase_m_first_slice_runtime_test_results.md`](tests/phase_m_first_slice_runtime_test_results.md). No atlas. No M-4A atlas masking. simthing-sim remains map-free.

**Phase M-first-slice-R3:** **Done (opt-in parking)** — GPU-resident readiness/observability pass; `FirstSliceReadinessReport` for Opus/product review. See [`phase_m_first_slice_runtime_r3_readiness_test_results.md`](tests/phase_m_first_slice_runtime_r3_readiness_test_results.md). Repo parked at “M-first-slice GPU-resident runtime landed; ready for Opus/product review.” No atlas. Defaults unchanged.

**Phase M product fixture:** **Done (opt-in)** — product-facing first-slice scenario fixture landed. It drives the accepted GPU-resident first-slice runtime from a small product-style RegionFieldSpec/RON fixture: one grid, source_capped_normalized, H<=8, caller-managed seed-only clear, dirty scheduling, SlotRange Sum reduction, and parent field_urgency EvalEML. The fixture proves default-off behavior, explicit SparseRegionFieldV1 opt-in, GPU-resident hot path with reduction_stencil_readbacks=0, finite propagated field values, and personality/weight-sensitive urgency. No atlas batching, M-4A atlas masking, active mask, perception, map residency, behavioral source policy, source_mask, semantic WGSL, simthing-sim map awareness, or default changes landed. Known caveat preserved: first-slice bridge uses queue writes for child resource values and parent weights; acceptable for the 10x10 first-slice fixture, not a future atlas/multi-field scale design.

**Phase M product commitment fixture:** **Done (opt-in)** — product-facing SEAD commitment fixture landed. It extends the first-slice product fixture by using the existing threshold/event substrate over parent field_urgency, proving GPU-resident field propagation -> parent reduction -> EvalEML urgency -> threshold event. Low-weight profile stays below threshold; high-weight profile crosses and emits the expected event. No CPU-side AI planner, atlas batching, M-4A atlas masking, active mask, perception, map residency, behavioral source policy, source_mask, semantic WGSL, simthing-sim map awareness, or default changes landed. Known caveat preserved: first-slice bridge uses queue writes for child resource values and parent weights; acceptable for the 10x10 first-slice and commitment fixtures, not a future atlas/multi-field scale design.

**Phase M CommitmentSpec fixture:** **Done (opt-in)** — first-slice commitment threshold/event binding moved into a designer/spec-facing RON-admitted configuration while preserving the existing GPU-resident SEAD path: field propagation -> parent reduction -> field_urgency EvalEML -> Threshold + EmitEvent. Low-weight profile remains below the authored threshold; high-weight profile crosses and emits the authored event. No CPU-side AI planner, atlas batching, M-4A atlas masking, active mask, perception, map residency, behavioral source policy, source_mask, semantic WGSL, simthing-sim map awareness, or default changes landed. Known caveat preserved: first-slice bridge uses queue writes for child resource values and parent weights; acceptable for the 10x10 first-slice and commitment fixtures, not a future atlas/multi-field scale design.

**Phase M FirstSliceScenarioSpec fixture:** **Done (opt-in)** — wraps the accepted first-slice RegionFieldSpec + CommitmentSpec in a scenario-level RON authoring shape with explicit MappingExecutionProfile. Disabled scenarios admit as structure but do not execute. SparseRegionFieldV1 scenarios execute the GPU-resident first-slice path and emit the authored commitment event only when field_urgency crosses the authored threshold. No CPU-side AI planner, default SimSession wiring, atlas batching, M-4A atlas masking, active mask, perception, map residency, behavioral source policy, source_mask, semantic WGSL, simthing-sim map awareness, or default changes landed. Known caveat preserved: first-slice bridge uses queue writes for child resource values and parent weights; acceptable for the 10x10 first-slice scenario fixture, not a future atlas/multi-field scale design.

**Phase M FirstSliceScenarioSpec-R1 hygiene:** **Done (opt-in)** — clarifies public/test-only boundary, hardens scenario budget estimate error propagation, documents prior crash/build-run history with final clean verification. No scope expansion.

**Phase M Map Residency V1:** **Done (opt-in)** — first-slice residency status/reporting; metadata only; cached commitment scan deferred. See [`tests/phase_m_first_slice_map_residency_test_results.md`](tests/phase_m_first_slice_map_residency_test_results.md).

**Phase M Boundary Resolution Doctrine R2:** **Done (docs terminology correction)** — restores legible tick/boundary/day_index/ticks_per_day vocabulary; day/calendar remains host/spec interpretation only. See [`tests/phase_m_boundary_resolution_doctrine_r2_terminology_test_results.md`](tests/phase_m_boundary_resolution_doctrine_r2_terminology_test_results.md).

**Phase M abstract boundary-resolution + example economy:** **ACCEPTED (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — abstract boundary doctrine accepted; Daily Economy Fixture V1 accepted as example/product fixture only; `ResourceEconomySpec` (discrete banking) vs Resource Flow E-11 (continuous, default-off) distinction accepted; the eleven future-agent guardrails made **binding** in [`invariants.md`](invariants.md) ("Boundary resolution (abstract cadence)"). Condition C-1: "no day semantics in simthing-sim" means no calendar arithmetic / `Calendar`/`Pause` types / `DailyResolutionBoundary` — not the legible `day`/`day_index` naming. **Next implementation step: resource-economy authoring ergonomics, or an economy+SEAD product fixture — not a `DailyResolutionBoundary`-by-another-name packet, not the M-4 atlas packer.** See [`reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md`](reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md).

**Phase M Boundary Resolution Doctrine audit:** **Done (docs+test audit)** — abstract deterministic tick/boundary cadence confirmed via existing substrate machinery; no `DailyResolutionBoundary` primitive; day/calendar/pause remain host layer only; example discrete banking may use resource economy not Resource Flow by default. See [`tests/phase_m_boundary_cadence_doctrine_audit.md`](tests/phase_m_boundary_cadence_doctrine_audit.md).

**Phase M Boundary Resolution Doctrine R1:** **Done (docs terminology correction)** — active docs reframe tick/boundary cadence as abstract substrate machinery; daily/Clausewitz semantics demoted to example fixture language. See [`tests/phase_m_boundary_resolution_doctrine_r1_test_results.md`](tests/phase_m_boundary_resolution_doctrine_r1_test_results.md).

**Phase M Daily Economy Fixture V1:** **Done (opt-in product/example fixture)** — example fixture showing one-boundary-as-one-day discrete banking at `ticks_per_day=1`; not canonical SimThing semantics. See [`tests/phase_m_daily_economy_fixture_test_results.md`](tests/phase_m_daily_economy_fixture_test_results.md).

**Phase M Queue-Write Scale Hardening V1:** **Done (opt-in)** — first-slice child resource column population uses generic `AccumulatorOpSession::fill_slot_range_col` bulk fill instead of O(cell_count) per-slot queue writes; parent scalar writes remain O(1). SummaryValidity and SEAD commitment behavior unchanged. See [`tests/phase_m_queue_write_scale_hardening_test_results.md`](tests/phase_m_queue_write_scale_hardening_test_results.md).

**Phase M SummaryValidity V1-R1 hygiene:** **Done (opt-in)** — runtime summary status driver-owned; spec retains policy only. See [`tests/phase_m_first_slice_summary_validity_r1_boundary_hygiene_test_results.md`](tests/phase_m_first_slice_summary_validity_r1_boundary_hygiene_test_results.md).

**Phase M SummaryValidity V1:** **Done (opt-in)** — bounded first-slice summary validity policy/status so a clean or skipped RegionField reports fresh, cached, zero-initial, or unavailable parent summary metadata without rerunning dense propagation or CPU-side threat/urgency rederivation. GPU retains authoritative parent summary columns on cached skips; hot path preserves `reduction_stencil_readbacks == 0`. Cached commitment threshold scan deferred (metadata-only on skip). No CPU-side AI planner, default SimSession wiring, atlas batching, M-4A atlas masking, active mask, perception, map residency system, behavioral source policy, source_mask, semantic WGSL, simthing-sim map awareness, or default changes landed. See [`tests/phase_m_first_slice_summary_validity_test_results.md`](tests/phase_m_first_slice_summary_validity_test_results.md).

**Phase M first-slice vertical proof:** **ACCEPTED (Opus/product 2026-05-28, PASS WITH CONDITIONS)** — full chain scenario RON → opt-in profile → GPU-resident field propagation → parent reduction → field_urgency EvalEML → Threshold + EmitEvent commitment accepted as complete for the single-grid opt-in SEAD path. **Map Residency V1 observability landed; next substrate slice TBD by product — NOT the M-4 atlas packer.** See [`reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md`](reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md).

**Phase M-first-slice-R2:** **Done (opt-in remedial)** — GPU-resident Layer 1→2→3 bridge. See [`phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md`](tests/phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md).

**Phase M-4 design note:** **Done; isolation policy ratified 2026-05-28** — see [`mapping_current_guidance.md`](workshop/mapping_current_guidance.md) and [`reviews/m4_m4a_first_slice_oversight_opus_review.md`](reviews/m4_m4a_first_slice_oversight_opus_review.md). **M-4A ratified** (algebraic tile-local mask G=0 preferred for homogeneous square batches; physical gutter fallback; local-bounds deferred; §11 binding gate). **Atlas batching remains Provisional and unimplemented** — `request_atlas_batching` rejected at admission until a §11-gate-passing M-4 PR; admissible only after a named multi-theater scenario + approved VRAM budget. **M-4 atlas implementation is NOT next.** First-slice runtime landed and hardened (R1/R2/R3), **accepted by Opus as a stable base**, not default-on. The product-facing first-slice scenario fixture (Option 3, single grid, no atlas) is now landed.

**V7.6 StructuredFieldStencilOp:** **Live, opt-in, hardened, inert by default** — not mapping runtime; not wired into production passes.

**Mapping optimization remedial sandbox:** **Reverted** — evidence archived; decision-gate summary [`mapping_optimization_remedial_sandbox_test_results.md`](tests/mapping_optimization_remedial_sandbox_test_results.md). Verdict **PARTIAL+** (incorporated into ADR).

**Mapping optimization toolkit sandbox:** **Reverted** — evidence archived; decision-gate summary [`mapping_optimization_toolkit_sandbox_test_results.md`](tests/mapping_optimization_toolkit_sandbox_test_results.md). Verdict **PARTIAL** (incorporated into ADR).

**Next (immediate):** **Pause implementation (F)** — park until mapping direction is finalized enough to define the next narrow substrate slice, or product names a concrete non-mapping scenario per [`product_priority_vertical_slice_selection.md`](reviews/product_priority_vertical_slice_selection.md). Global default-on remains deferred. **Mapping/location direction remains provisional** — must not trigger templates, simthing-spec/RON/Designer rebuild, or runtime mapping work until product finalizes direction.

**Implementation posture:** AccumulatorOp is the production runtime substrate. Do not
reintroduce runtime legacy oracle/fallback peers; tests use CPU/golden or AccumulatorOp
baselines only.

**Implementation:** `simthing-driver::SpecSessionState` owns spec runtime
state; `simthing-driver::install` compiles a `GameModeSpec` against a
`Scenario`, clones capability trees per owner with fresh `OverlayId`s and
`EffectTarget` overlay placement, installs scripted events as definition +
N instances, and populates spec state. `SimSession::open_from_spec(scenario, &game_mode)`
is the RON-driven entry point. After fission with cloned capability subtrees,
`react_to_fission_clones` registers new capability instances and thresholds.
`BoundaryProtocol::execute_with_boundary_hook` invokes handlers after GPU readback;
`simthing-sim` remains spec-free.

**`by_overlay` + `overlay_hosts`:** per-clone overlay → entry map and overlay → host
SimThing map on `CapabilityTreeInstance`. Handler resolves activate/suspend targets
via `overlay_hosts`; GPU overlay-prep uses overlay **placement** on the host tree.

### Opus session complete (2026-05-23, O2 + B3 + I1)

All three Opus P0 items shipped. No P0 code work outstanding.

**Landed (Opus, PRs #65–#67):**

| ID | PR | Commit | Scope |
|----|-----|--------|-------|
| **O2** | #65 | `2f2a7b5` | Replay v3 — `SpecSnapshot`/`SpecDelta`, `spec_replay.rs`, `open_replay_with_spec`, logical-key invariant |
| **B3** | #66 | `defb42c` | Precise `requires_boundary_tick` — 6 conditions; zero-alloc `has_*_in` predicates on `ThresholdRegistry` |
| **I1** | #67 | `6b8de81` | `preview_install` / `install_atomic` / `apply_install_preview`; `SlotAllocator: Clone`; ADR Accepted |

**Earlier Opus commits (`2eff1e0`–`8904522`):**

| ID | Commit | Scope |
|----|--------|-------|
| **O1b** | `2eff1e0` | Handler `emit_activation` uses per-clone ids from `instance.by_overlay` |
| **EffectTarget** | `8da4be9`, `7febdd1` | ADR Accepted; `Owner` default; `overlay_hosts` + host overlay placement |
| **S5** | `dcc74cc` | Disable Approach C when fission clones capability subtrees |
| **S5 follow-up** | `1253a97` | Fission clone overlay-id re-stamp; `react_to_fission_clones` |
| **O4** | `8904522` | Per-owner scripted event instances; `EventSpec.install` |

**Deferred / tabled:** B2 tighter incremental topology for fission clone internal edges;
`ScopeRef::Owner` and cross-owner scripted events; mid-session install atomicity (GPU resync);
atomic spec hot-reload with `SpecSessionState` preservation; scenario RON expansion;
`simthing-studio` GUI; E0 base economic system; **production migration of workshop spikes**
(EML / AccumulatorOp WeightedMean — ADR or specialized path required first).

### `simthing-workshop` — isolated viability spikes (non-production)

> **Important:** `crates/simthing-workshop` holds **targeted architectural experiments**
> and report artifacts. It has **no dependents** in the workspace and **does not reflect
> production GPU/simulation code**. Passing a workshop gate is **not** a claim that
> production should adopt that path without an ADR. Production WeightedMean lives in
> `simthing-gpu` reduction via AccumulatorOp; production intensity uses EvalEML through
> AccumulatorOp (legacy `intensity_update.wgsl` deleted in S-2).

**Crate README:** `crates/simthing-workshop/README.md`

| Spike | PRs | Status | Production implication |
|-------|-----|--------|----------------------|
| **EML Phase 5 intensity** | #71–#74 | **PASS** — bit-exact vs CPU; warm EML ~1.2–1.5× hardcoded at 100k | Optional numeric backend research only; not general EML |
| **WeightedMean AccumulatorOp parity** | #75, #77 | **LOOSE_TOLERANCE** at 100k (`WEAK_PASS_REQUIRES_ADR`); production-shape fixture **BIT_EXACT** | Do **not** replace production reduction without tolerance ADR or bit-exact fix |
| **Report artifacts** | #72, #76, #77 | `tests/eml_phase5_reports_hardened.txt`, `tests/workshop_full_reports.txt`, `tests/weighted_mean_reports.txt` | Raw test captures; not spec |

**Workshop tests (17):** 8 EML + 9 WeightedMean integration tests. `100k` markdown reports
written under `target/workshop/` (gitignored).

**Next (workshop, optional):** doc sync with `docs/eml_integration_guidance.md` /
`docs/workshop/multichannel_accumulator_test_battery.md` gate results; Sonnet D1/D2 modder
guide & examples per `docs/workshop/workshop_current_state.md` §3 (parallel, production docs).

**Known risks (remaining):**

- **Mid-session install** — `apply_install_preview` on a *running* session needs GPU resync and slot reallocation. Deferred per I1 ADR.
- **Spec hot-reload** — preserving in-flight `SpecSessionState` (cooldowns, selections) across re-install needs replay-style state merging.
- **O1c ruled out** — dimension sync after install not the blocker.

**Worktree:** clean for tracked files. Untracked `.claude/worktrees/`,
`demo.replay.ldjson`.

**ADRs fully current:** `spec_session_state_replay.md` → Accepted (O2); `install_clone_then_commit.md` → Accepted (I1 new file).

---

## Done

### V6 simulation core (`f39fe6d`)

- [x] Add `OverlayLifecycle::Suspended { when_activated }`.
- [x] Keep suspended overlays out of CPU evaluator and GPU overlay-delta prep.
- [x] Add boundary-time `ActivateOverlay` and `SuspendOverlay` requests.
- [x] Record overlay activation/suspension in the boundary delta log.
- [x] Replay overlay activation/suspension transitions.
- [x] Add `active` attribution to observability overlay contributions.
- [x] Ensure suspended overlays do not force empty-boundary work.
- [x] Add `FissionTemplate::clone_capability_children` (serde default `false`).
- [x] Clone capability containers on opted-in fission templates.
- [x] Allocate fresh IDs and slots for cloned capability subtrees.
- [x] Copy cloned capability shadow rows.
- [x] Remap cloned overlay `affects` from parent owner to spawned owner.
- [x] Pre-grow boundary slot headroom for cloned capability subtrees.

### Capability-container kind parameterization (PR #38, `a8aab5b`)

- [x] Add `FissionTemplate::capability_container_kinds: Vec<String>` with
      `#[serde(default)]` — empty vec when field omitted.
- [x] Remove hardcoded `"tech_tree" | "national_ideas" | "talent_tree"`
      checks from `simthing-sim` production code.
- [x] Shared `is_capability_container(kind, container_kinds)` in
      `fission.rs`; `boundary.rs` imports it for pre-grow headroom.
- [x] **Option A semantics:** `clone_capability_children: true` with empty
      `capability_container_kinds` clones nothing — no sim-crate fallback list.
- [x] Thread `&ft.template.capability_container_kinds` through
      `execute_fission` → `clone_capability_children` and through
      `projected_fission_slots` pre-grow estimation.
- [x] Serde compatibility test:
      `fission_template_deserializes_without_capability_container_kinds`
      (`simthing-core`).
- [x] Fission unit test: `clone_capability_children_empty_kinds_clones_nothing`.
- [x] Update existing clone/headroom tests to populate
      `capability_container_kinds` explicitly.
- [x] Doc addenda in `design_v6.md` and `capability_tree_v1.md`; agent briefing
      sync in `agents.md`.

### `simthing-spec` PR 1 — authoring-only scaffold (PR #46, `7eb48dc`)

- [x] Crate + workspace membership; depends on `simthing-core` only (PR 1).
- [x] `GameModeSpec`, `DomainPackSpec`, capability RON structs, `PropertySpec` /
      `OverlaySpec` placeholders.
- [x] Generic `SpecDiagnostics`, `SpecVersion`, `DisplayMeta`, logical keys.
- [x] RON loaders + lightweight `validate_capability_tree`.
- [x] PR 1 tests (`tests/pr1_spec.rs`, `validate` unit tests).
- [x] Reverted exploratory builder/boundary/threshold code from PR #45.

---

## Next

### `simthing-spec` (revised PR ladder — historical)

> **Historical.** PRs 2–11 complete. Current owners: see top of this file and
> `docs/design_v6.5.md` §5.

Authoritative spec: `simthing-spec — Master Implementation Handoff` (2026-05-22).
All PRs sequenced deliberately; do not skip ahead.

- [x] **PR 2** — property + overlay spec compiler (`compile/property.rs`,
      `compile/overlay.rs`, `compile/context.rs`). Landed 2026-05-22.
      `PropertySpec` expanded with `description` + `sub_fields`; empty
      `sub_fields` defaults to `PropertyLayout::standard(0)`. `OverlaySpec`
      expanded with `targets_property`, `sub_field_deltas`, `lifecycle`,
      `kind`, `source`. `compile_property` checks `id_of` before
      `register` (no panic on duplicate), validates `governed_by`
      against the same layout. `compile_overlay` parses `"ns::name"`,
      validates property existence, validates each sub-field role
      against the target's layout. New errors:
      `DuplicateProperty`, `UnknownProperty`, `InvalidGovernedByRole`,
      `InvalidSubFieldRole`, `InvalidPropertyReference`. Tests:
      `tests/pr2_compile.rs` — 11 passing (all 7 acceptance criteria
      from the handoff + 4 supplementary).
- [x] **PR 3** — `CapabilityTreeBuilder`. Landed 2026-05-22.
      `runtime/capability_definition.rs` defines `CapabilityTreeDefinitionId`
      (atomic newtype), `CapabilityTreeDefinition` (shared, immutable,
      `entries` / `by_threshold` / `by_overlay` lookups), `CapabilityDefinition`
      (per-entry with parallel `overlay_ids` / `effect_keys`), `CapabilityPrereq`
      (resolved `property_id` / `role` / `col` / `min_value`), and a
      placeholder `CapabilityUnlockRegistration` (PR 4 moves to feeder).
      `compile/capability.rs::CapabilityTreeBuilder::build` runs validation,
      registers one `SimProperty` per category with one `Named(entry.id)`
      sub-field each (`ReductionRule::Max` forced via `reduction_override`),
      constructs the template `SimThing` (`Custom(tree_kind)`), compiles
      each effect into a `Suspended { when_activated: ... }` `Overlay`,
      resolves prereqs (cross-category supported via `"ns::name"` strings),
      and emits one `CapabilityUnlockRegistration` per `Threshold` entry
      (`PlayerSelection` produces none). `ActivationMode` gains `OnPrereqMet`;
      `validate.rs` rejects it as an authored default plus `Limited(n != 1)`
      and self-referential prereqs. Tests:
      `tests/pr3_capability_builder.rs` — 16 passing (all 11 acceptance
      criteria from the handoff + 5 supplementary).
- [x] **PR 4** — capability unlock registration bridge. Landed 2026-05-22.
      `CapabilityUnlockRegistration` (with `Serialize/Deserialize` derives)
      lives in `simthing-feeder::capability`; `simthing-spec` re-exports it
      via `runtime::capability_definition` (placeholder removed) and gains
      a `simthing-feeder` dep. `simthing-sim::threshold_registry` adds
      `ThresholdSemantic::CapabilityUnlock { sim_thing_id, property_id,
      sub_field }` (with `Serialize/Deserialize` derives on the whole enum)
      plus `ThresholdBuilder::build_with_capability_unlocks(root, dim_reg,
      allocator, velocity_alerts, capability_unlocks)` and a
      `push_capability_unlocks` helper. The path is full-rebuild only; B2
      append-only integration deferred. Skipping behavior matches velocity
      alerts: inactive properties / unallocated sim_things / missing roles
      silently skip. `simthing-feeder/Cargo.toml` picks up `serde`. Tests:
      `simthing-feeder/src/capability.rs` (1), `threshold_registry.rs`
      tests (4 — 3 acceptance + 1 supplementary), and the GPU integration
      `capability_unlock_fires_in_boundary_integration_test` in
      `simthing-sim/tests/boundary_integration.rs` (uses a Permanent
      overlay attached to the cap tree to push progress across the
      threshold mid-Pass-3 — `submit_player_intent` doesn't work for
      this because intent_deltas apply BEFORE Pass 0's snapshot, so
      previous == current and the crossing isn't visible).
      All 5 handoff acceptance criteria met + 1 supplementary.
- [x] **PR 5** — capability runtime state + boundary handler
      (`boundary/capability_handler.rs`). Called by session coordinator,
      not embedded in `BoundaryProtocol`. Landed 2026-05-22 with Path A
      for `max_active`: `CapabilityCategorySpec.max_active` is now
      `Option<MaxActivePolicy>` with `Limited { count, replacement }`, and
      `ReplacementPolicy::SuspendOldest` is the supported v0 replacement.
      `CapabilityTreeDefinition` now carries category definitions; entries
      carry authored activation mode, `progress_col`, and `research_cost`.
      Added per-faction runtime state, notifications, diagnostics, and the
      boundary handler for threshold activation, failed-prereq reset into
      `OnPrereqMet`, fixpoint sweeps, player selection, per-faction active
      state, and `Limited(1)` sibling suspension. Tests:
      `tests/pr5_capability_handler.rs` — 10 passing acceptance tests.
- [x] **PR 6** — preview + mutual exclusivity completion
      (`preview/capability_preview.rs`). Landed 2026-05-22. Adds
      definition-only CPU preview for capability effects with per-overlay
      breakdowns and combined net deltas. `CapabilityDefinition` now carries
      compiled `effect_transforms` parallel to overlay/effect keys so preview
      does not need the template SimThing. Adds full national-ideas
      activate-switch verification by feeding PR 5 handler requests through
      real structural overlay activation/suspension. Tests:
      `tests/pr6_capability_preview.rs` — 5 passing acceptance tests.
- [x] **PR 7** — Script IR. Landed 2026-05-22. Replaces
      `spec/script_stub.rs` with canonical `ScriptExpr` / `ScriptPredicate`
      authoring IR, `PropertyKey`, `ScopeRef`, and a CPU evaluator over
      `DimensionRegistry + shadow + n_dims`. Supports constants, property
      reads, arithmetic, min/max, clamp, numeric gates, comparison predicates,
      boolean composition, serde round-trips, and hard evaluation errors for
      unknown property/role, bad slots/columns, division by zero, and invalid
      clamps. No EML, parser, trigger/effect compiler, or event system yet.
      Tests: `tests/pr7_script_ir.rs` — 10 passing acceptance/scaffold tests.
- [x] **PR 8** — trigger/effect/event compiler. Landed 2026-05-22 as a
      conservative typed-template slice: `TriggerSpec`, `EffectSpec`, and
      `EventSpec` compile into `CompiledTrigger`, `CompiledEffect`, and
      `ScriptedEventDefinition`. Simple threshold triggers resolve
      property/role/column against `DimensionRegistry`; predicate triggers
      preserve PR 7 `ScriptPredicate`; effects compile to boundary request
      templates for remove / activate overlay / suspend overlay. No event
      runner, threshold registry upload, parser, or EML. Tests:
      `tests/pr8_event_compiler.rs` — 7 passing scaffold tests.

### Parking notes / next candidates

- [x] **PR 9** — scripted event boundary handler. Landed 2026-05-22.
      `boundary/event_handler.rs` with `ScriptedEventBoundaryHandler`,
      `ScriptedEventBoundaryContext`, `ScriptedEventDiagnostic`,
      `ScriptedEventDiagnosticKind`. Predicate triggers only (threshold triggers
      deferred to GPU-path PR); cooldowns and priority ordering implemented.
      Missing slot targets push `UnresolvedEffectTarget` diagnostic. Eval errors
      push `TriggerEvalError` diagnostic. All 8 acceptance tests pass in
      `tests/pr9_event_handler.rs`.
- [x] **PR 10** — scripted-event GPU threshold path. Landed 2026-05-22.
      Adds `simthing_feeder::ScriptedEventTriggerRegistration` and
      `ScriptedEventTriggerEvent`; adds
      `ThresholdSemantic::ScriptedEventTrigger { event_id }` arm plus
      `ThresholdBuilder::build_with_scripted_event_triggers` and
      `ThresholdRegistry::extract_scripted_event_triggers` in
      `simthing-sim`; adds `ScriptedEventDefinition::to_trigger_registration`
      in spec. `ScriptedEventBoundaryHandler::handle_tick` now takes a
      `&[ScriptedEventTriggerEvent]` slice and fires threshold-triggered
      events under unified cooldown/priority gating with predicate-triggered
      events. New diagnostic variant: `UnknownEventId` for stale registrations.
      Bumps `simthing_core::Direction` with `Copy + PartialEq + Eq` derives.
      11 acceptance tests in `tests/pr10_scripted_event_thresholds.rs`.
- [x] **PR 11 Track A (Opus)** — session/driver assembly merged `01fb572`
      (2026-05-22). ADR: `docs/adr/pr11_track_a_session_assembly.md`.
      Driver-owned `SpecSessionState`, multi-tree-safe capability keys, generic
      post-readback boundary hook in sim, external threshold registration
      plumbing, `SimSession::install_spec_state`, GPU E2E unlock → handler →
      overlay → next-tick value change. **311** tests at landing.
- [x] **PR 11 Track B (Composer)** — mechanical prep merged PR #47 (`392992f`,
      2026-05-22): B5 release smoke check; B2 `EventKey: From<&str>`/`From<String>`;
      B1 `Display` for capability/scripted-event diagnostics; B3
      `append_capability_unlocks` / `append_scripted_event_triggers`;
      B4 docs addenda in `design_v6.md` and `capability_tree_v1.md`.
- [x] Assemble session/driver ownership for capability tree instances and
      runtime state maps. Driver storage is keyed by
      `(owner_id, definition_id, tree_thing_id)`; temporary one-instance maps
      are passed into the PR 5 handler to preserve current handler API while
      avoiding the session-level multi-tree footgun.
- [x] Clean up PR 5's temporary `simthing-spec -> simthing-sim` /
      `simthing-spec -> simthing-gpu` threshold dependencies. Done 2026-05-22.
      Approach: introduced `simthing-feeder::CapabilityUnlockEvent` as the
      resolved-event shape spec consumes; renamed handler entry point to
      `handle_capability_unlock_events`; added
      `ThresholdRegistry::extract_capability_unlocks` in `simthing-sim` as the
      bridge for callers that hold raw `ThresholdEvent`s. Spec production deps
      are now `simthing-core` + `simthing-feeder` only; `simthing-gpu` /
      `simthing-sim` remain as dev-dependencies for PR 6 integration tests.
- [ ] B2 append-only capability/scripted-event external registration integration
      remains deferred. Track A full rebuilds include external registrations;
      append-only handling for newly cloned capability trees is a later
      optimization/design item.
- [ ] Replay v3 for spec session state remains deferred. Existing structural
      overlay activations replay through the boundary delta log, but capability
      runtime state, scripted-event cooldowns, diagnostics, and notifications
      are not serialized yet.

**Known divergences between handoff doc and PR 1 code (Opus must resolve):**

Historical notes below were written before PRs 2-8 landed. Several are now
resolved; keep this section as archaeology until the handoff docs are folded
into the current code.

1. `CapabilityCategorySpec` has no `id` field — handoff §1.4 references one;
   actual struct identifies category by `property_namespace::property_name`.
   `CategoryKey { namespace, name }` in `keys.rs` already captures this.
   **Resolution:** add `id: String` to the struct and thread it through, OR
   accept that category id = `namespace::name` (matching `CategoryKey`).

2. `MaxActivePolicy` in `spec/capability.rs` is `Limited { count: usize }` — no
   `replacement: ReplacementPolicy` field; no `ReplacementPolicy` enum. Handoff
   §1.4 requires both. **Resolution:** add `ReplacementPolicy` enum and
   `replacement` field in PR 2/5 when needed.

3. `ActivationMode` is missing the `OnPrereqMet` arm — the comment says "will be
   added in later PRs." Handoff §1.3 defines all three arms.
   **Resolution:** add `OnPrereqMet` to the enum in PR 3; extend `validate.rs`
   to reject it as an authored default.

4. `CapabilitySpec.research_cost: f32` vs handoff `research_cost: ResearchRateSpec`
   — the struct also has a separate `research_rate: ResearchRateSpec` field,
   which is unused. **Resolution:** PR 3 builder reads `research_cost: f32` as
   the literal threshold value. The `research_rate` field is a vestige of an
   earlier design; either remove it or leave it unused. Do not rename `research_cost`
   (serde-breaking).

5. `PropertySpec` is a stub (`id`, `namespace`, `name`, `display_name` only) — no
   layout, no sub-field specs, no decay, no clamp, no governed_by. PR 2's
   `compile_property` enforces layout validity, so the struct must grow.
   **Resolution:** expand `PropertySpec` with at least a `sub_fields` layout
   description before writing the compiler, OR keep `compile_property` minimal
   (namespace+name registration with a default layout) and accept simpler tests.

6. `OverlaySpec` is a stub (`id`, `display_name` only) — no `targets_property`,
   `sub_field_deltas`, or `lifecycle`. PR 2's `compile_overlay` needs these.
   **Resolution:** expand `OverlaySpec` with those fields, or scope PR 2's
   `compile_overlay` to the standalone (non-capability) overlay use-case and
   note that capability overlays are built inline by the PR 3 builder.

7. `DimensionRegistry::register` panics on duplicate `namespace+name` — `compile_property`
   must check `registry.id_of(ns, name).is_some()` and return
   `Err(SpecError::DuplicateProperty(...))` before calling `register`.
   Add the error variant to `error.rs`.

8. No `registry.set_reduction_rule` method exists — handoff prose mentions it but the
   correct implementation is to set `reduction_override: Some(ReductionRule::Max)` on
   each `SubFieldSpec` when constructing the `SimProperty`, before calling `register`.
   `ReductionRule::Max` and `SubFieldSpec::reduction_override` both exist.

9. `SpecError` needs more variants for PR 2/3: at minimum `DuplicateProperty`,
   `OnPrereqMetAuthoredDefault`, `UnknownPrereqEntry`, `UnknownPrereqCategory`,
   `UnknownProperty`, `UnsupportedMaxActive`. Add as needed per PR.

10. `CapabilityTreeDefinitionId` type does not exist — needs to be defined in PR 3
    (likely a newtype wrapping `CapabilityTreeKey` or a `u32` index).

**Confirmed working (no surprises):**
- `OverlayId::new()` ✓ (atomic counter in `ids.rs`)
- `col_for_role` ✓ (method on `PropertyColumnRange` in `registry.rs`)
- `SubFieldRole::Named(String)` ✓
- `OverlayLifecycle::Suspended { when_activated: Box<OverlayLifecycle> }` ✓
- `ReductionRule::Max` ✓ (`reduction.rs`; `SubFieldSpec::reduction_override: Option<ReductionRule>`)
- `ThresholdSemantic` (5 arms; PR 4 adds `CapabilityUnlock`) ✓
- `CapabilityTreeKey`, `CategoryKey`, `CapabilityEntryKey`, `CapabilityEffectKey` ✓ (`keys.rs`)
- `SpecDiagnostics`, `SpecError`, `SpecResult<T>` ✓
- `simthing-feeder` has no `capability.rs` yet — PR 4 creates it ✓
- **212 tests passing**, 1 ignored, zero warnings ✓

### Performance and spec layer

- [x] **Priority 1 — activated overlay GPU integration test.** Landed
      2026-05-22. `activated_suspended_overlay_appears_in_gpu_delta_and_affects_values`
      in `crates/simthing-sim/tests/boundary_integration.rs`. Proves the full
      Suspended → Permanent transition: suspended overlay is GPU-inert (Pass 3
      filter), `BoundaryRequest::ActivateOverlay` flips lifecycle, boundary
      gpu_sync rebuilds Pass 3 deltas, next tick's Pass 3 applies the overlay
      to `values` (0.5 → 0.75 via Multiply(1.5)).
- [x] **Priority 2 — capability fission replay test.** Landed 2026-05-22.
      `replay_fission_with_cloned_capability_subtree_reconstructs_full_payload`
      in `crates/simthing-sim/tests/boundary_integration.rs`. Drives a faction
      fission with `clone_capability_children: true` and
      `capability_container_kinds: ["tech_tree"]`; verifies the
      `FissionOccurred { node }` payload carries the full cloned tech_tree
      subtree (2 nested levels), and `ReplayDriver` reconstructs the spawned
      faction, its cloned tech_tree, and the tech_tree's child, with slots
      allocated for every node and lineage round-tripped.
- [x] **Priority 3 — serde default for `clone_capability_children`.** Landed
      2026-05-22. `fission_template_deserializes_without_clone_capability_children`
      in `crates/simthing-core/src/property.rs`. Pre-V6 JSON/RON without the
      field deserializes as `false` (safe default — no capability cloning
      runs without explicit spec-layer opt-in).

### Performance and spec layer (V6 guardrails complete — B2 done)

- [x] **Priority 4 — B2 fission-growth Approach A (targeted value upload).**
      Landed 2026-05-22. `WorldGpuState::rebuild_for_slots` now preserves
      existing GPU contents via `copy_buffer_to_buffer` (values,
      previous_values, output_vectors, previous_output_vectors). Fission /
      AddChild / final-capacity pre-grow no longer force a full shadow
      flush. New `DispatchCoordinator::upload_row_range` coalesces
      contiguous dirty slots into single `queue.write_buffer` calls in
      `gpu_sync`. Regression guard:
      `fission_beyond_initial_headroom_grows_gpu_state` now asserts
      `!full_value_upload && value_rows_uploaded == 1` for a single
      fission across a growth boundary.
- [x] **Priority 4 — B2 Approach B (append-only threshold registry,
      2026-05-22).** `ThresholdBuilder::append_subtree` /
      `append_lineage` and `WorldGpuState::append_thresholds` push new
      registrations at the tail of the existing GPU buffer (preserving
      event_kind indices) when boundary mutations are limited to pure
      fission spawning. `boundary.rs` detects the eligible case (no
      fusions, no expiry, no add/remove, no dimension/config change)
      and skips the full tree walk. `fission_stress` `boundary_gpu_sync_ms`:
      ~7 → ~3.8 ms (~3 ms saved); upload bytes ~2.5 MB → ~1.0 MB;
      ms_per_sim_day unchanged (within noise on this machine).
      Regression guard:
      `fission_beyond_initial_headroom_grows_gpu_state` now asserts
      `threshold_regs_uploaded == 2` for a single fission (1 new
      FissionTrigger + 1 new FusionTrigger), proving the append path
      writes only deltas instead of rebuilding the registry.
- [x] **Priority 4 — B2 Approach C (incremental reduction topology,
      2026-05-22).** New `simthing-gpu::TopologyState` is the canonical
      source for the CSR `Topology`; `gpu_sync.rs` takes it by `&mut`
      so the full-rebuild path refreshes it and the append path
      (mirroring Approach B's eligibility predicate) patches it
      in-place via `add_child(parent_slot, child_slot)`. The
      `SlotAllocator`'s monotonically-increasing index guarantee
      makes the new child the highest slot in the world, so appending
      to the parent's child list preserves the ascending-slot
      invariant without re-sorting. Determinism safety verified by
      two new unit tests in `simthing-gpu::reduction`
      (`topology_state_flatten_matches_build_topology` and
      `topology_state_incremental_add_child_matches_full_rebuild`)
      that prove byte-identical CSR output AND bit-identical CPU
      oracle reduction. Integration test adds
      `reduction_edges == 3` and `reduction_depths == 4` assertions.
      `fission_stress` `boundary_gpu_sync_ms`: ~3.8 → ~2.0 ms.
- [ ] **Scenario format expansion.** Full RON tree/registry/shadow seeds —
      behind the GPU performance path.
- [ ] **Map-scale representation doc spike.** Evaluate sidecars only if
      benchmarks show tree-representation pressure.
- [ ] **`simthing-studio` designer GUI** — tabled; depends on `simthing-spec`.

---

## Notes

### Architecture boundaries (unchanged)

- Suspended overlays are CPU-visible and GPU-free until activated.
- Capability cloning is opt-in per `FissionTemplate` and defaults to `false`.
- Cohort/location fission is unaffected unless a template opts in.
- No WGSL shader changes were required for V6 or PR #38.

### `capability_container_kinds` contract (PR #38)

| Field | Role |
|---|---|
| `clone_capability_children: bool` | Gates whether fission runs the clone path at all. |
| `capability_container_kinds: Vec<String>` | Opaque `Custom(name)` labels to match against parent children. |

Studio/RON authors own the strings via **`simthing-spec`** (planned). Simulation
never interprets "tech tree" vs "national ideas" — it only compares `SimThingKind::Custom(name)`
to the template list. Modders add `"racial_abilities"` (or any label) in RON;
no simulation recompile.

**Faction fission RON example:**

```ron
FissionTemplate(
    child_kind:                 Faction,
    fusion_intensity_threshold: 0.8,
    fusion_scar_coefficient:    0.05,
    resolution_label:           "separatism",
    clone_capability_children:  true,
    capability_container_kinds: [
        "tech_tree",
        "national_ideas",
        "talent_tree",
        "racial_abilities",
    ],
)
```

### Doc references

- **Current state:** `docs/design_v6.5.md`
- Simulation spec: `docs/design_v6.md` (incl. implementation addenda)
- Capability trees: `docs/capability_tree_v1.md` (incl. addendum §11)
- **Spec-layer handoff (canonical):** `docs/workshop/simthing_spec_progress_log.md`
- Workshop index: `docs/workshop/README.md`
- **Workshop spikes (non-production):** `crates/simthing-workshop/README.md` — EML / WeightedMean gates only
- Historical worksheet: superseded; see `docs/workshop/simthing_spec_progress_log.md`
- Source workshop Q&A (archived): `docs/workshop/archive/capability_tree_studio_workshop.md`
- Historical workshop (archived): `docs/workshop/archive/tech_tree_decisions.md`
- Agent map: `docs/agents.md`

### Spec-layer dependency graph (PR 11 complete)

```text
simthing-core
    ↑
simthing-feeder   ← CapabilityUnlockRegistration, CapabilityUnlockEvent,
                    ScriptedEventTriggerRegistration, ScriptedEventTriggerEvent
    ↑         ↑
simthing-spec     simthing-sim   ← ThresholdSemantic, extract_*,
(production:      (production)     BoundaryHookContext, external threshold regs
 core + feeder
 only)
    ↑
simthing-driver   ← SpecSessionState, install_spec_state (wired)

simthing-studio   ← deferred GUI
```

### Recommended session order

1. ~~Priority 1 (activated overlay GPU proof)~~ — Done 2026-05-22, PR #39.
2. ~~Priority 2 (capability fission replay)~~ — Done 2026-05-22, PR #39.
3. ~~Priority 3 (`clone_capability_children` serde default)~~ — Done 2026-05-22, PR #39.
4. ~~Priority 4 — B2 Approach A (targeted value upload)~~ — Done 2026-05-22, PR #40.
5. ~~Priority 4 — B2 Approach B (append-only threshold registry)~~ — Done 2026-05-22, PR #41.
6. ~~Priority 4 — B2 Approach C (incremental reduction topology)~~ — Done 2026-05-22, PR #43.
7. ~~**PR 11 Track B** — mechanical prep~~ — Done PR #47, `392992f`.
8. ~~**PR 11 Track A** — session/driver assembly~~ — Done `01fb572`, parked `9e63718`.
9. ~~Composer Phase 0 + Phase 1 ADRs + O3~~ — Done through `c3f3556` (PRs #49–51).
10. ~~Composer S3 + S4~~ — topology full-rebuild guard; capability instance reverse map (PR #52, `7914528`).
11. ~~**O1** — RON-driven session installation~~ (PR #53, `6ba4e0d`). 320 tests.
12. ~~Post-O1 doc parking sync~~ (PR #54, `7eb015a`).
13. ~~Codex evaluation doc sync~~ (PR #55, `04867b1`).
14. ~~O1b E2E test (Cursor)~~ — landed; **green** after `2eff1e0`.
15. ~~O1b handler fix~~ — `2eff1e0`.
16. ~~S5 Approach C disable~~ — `dcc74cc`.
17. ~~S5 fission-clone instance registration~~ — `1253a97`.
18. ~~EffectTarget ADR + implementation~~ — `8da4be9`, `7febdd1`.
19. ~~O4 per-owner scripted events~~ — `8904522`.
20. ~~**Opus P0 O2** — Replay v3~~ — Done PR #65, `2f2a7b5`.
21. ~~**Opus P0 B3** — Precise boundary-skip classification~~ — Done PR #66, `defb42c`.
22. ~~**Opus P0 I1** — Install clone-then-commit~~ — Done PR #67, `6b8de81`.
23. Scenario format expansion / map-scale representation — tabled.
24. `simthing-studio` GUI — tabled.
25. E0 base economic system — tabled (separate design space).
26. ~~**Workshop EML Phase 5 + WeightedMean parity spikes**~~ — Done PRs #71–#77; non-production gates only.
27. Sonnet D1/D2 modder guide & examples — open (see `workshop/workshop_current_state.md` §3).

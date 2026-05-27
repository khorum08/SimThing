# SimThing Workshop — Current State

**Purpose:** Single synthesis of **active workshop docs**, **production migration state**,
and **documentation routing**. Read this first when picking up GPU migration or workshop work.

**Last updated:** 2026-05-19  
**Master HEAD:** E-2B-5R dynamic fission enrollment atomicity hardening (pending merge)  
**Verification (last recorded):** [`e2b5r_dynamic_fission_enrollment_atomicity_test_results.md`](../tests/e2b5r_dynamic_fission_enrollment_atomicity_test_results.md) — PASS; resource-flow flag default false

---

## 1. Executive summary

Two parallel tracks:

| Track | Status | Canonical docs |
|-------|--------|----------------|
| **V6 spec / driver / session** | **Parked complete** — PRs 1–11, Opus P0 (O2/B3/I1) shipped | `design_v6.5.md`, `simthing_spec_progress_log.md` |
| **AccumulatorOp v2 / design v7** | **Active** — Phases A–B done; C-1–C-8 landed; S-1/S-2/S-3/S-4/S-5/S-6 legacy passes deleted | `design_v7.md`, `accumulator_op_v2_production_plan.md`, `pivot_forward_implementation_policy.md` |

**Production direction:** AccumulatorOp v2 is the GPU execution path.
Legacy reduction is deleted (S-4). Legacy intensity is deleted (S-2). Legacy overlay is deleted (S-3). Legacy threshold is deleted (S-6). Legacy velocity is deleted (S-5). Legacy intent is deleted (S-1). Snapshot is the only retained non-Accumulator operation.

**E-11 status:** **Done (flat-star vertical slice)** — PR #159. **E-11R** PR #160. **Burn-in scaffold** PR #161. **Burn-in scenarios** PR #162. **Controlled opt-in CI soak** landed. **E-11B readiness review landed** — nested hierarchy GPU deferred pending product priority ([memo](../reviews/e11b_nested_hierarchy_gpu_readiness_review.md)). `use_accumulator_resource_flow` **default false**. No new WGSL; `simthing-sim` remains arena-ignorant.

**E-2B status:** **Done (static E-2B-1…4 + E-2B-5 Policy A + E-2B-5R).** Dynamic fission enrollment via arena-root sibling append with atomic prepare/commit ([memo](../reviews/e2b5_dynamic_fission_enrollment_readiness.md), [E-2B-5R test report](../tests/e2b5r_dynamic_fission_enrollment_atomicity_test_results.md)). Failed dynamic enrollment cannot leave partial tree/scaffold/registry state. Boundary reports retained on session. E-11B deferred. `use_accumulator_resource_flow` **default false**.

**Next gates:** **Resource Flow dynamic enrollment soak / opt-in scenario burn-in** (recommended); **E-11B**, **D-2a**, **Resource Flow default-on** per product priority.

**Open design gates (not sunset):** Phase T complete. E-2B static + dynamic enrollment done. E-11B deferred by default. `use_accumulator_resource_flow` remains default false. Hard-currency transfers remain separate from Resource Flow.

**C-8 complete:** EML infrastructure, intensity, transfer, and emission are GPU-resident through AccumulatorOp. TransferConservation remains ExactDeterministic only. Emission tolerance remains future-gated and isolated from transfer/hard thresholds.

**S-2 complete:** Legacy `intensity_update.wgsl` deleted. `IntensityBehavior` routes through AccumulatorOp EvalEML only; `use_accumulator_intensity` defaults on; disabling intensity with registered `IntensityBehavior` panics at boundary validation.

**S-3 complete:** Legacy `transform_application.wgsl` and the overlay pipeline/bind-group branch are deleted. Add/Multiply/Set overlays route solely through AccumulatorOp OrderBands; `use_accumulator_overlay_add` defaults on and disabling it with active overlay deltas rejects the workload.

---

## 2. AccumulatorOp v2 migration state

### Landed AccumulatorOp migration / sunset state

**Default-on today:** reduction soft/exact · EML · EvalEML intensity · threshold · velocity · intent.

**Default-off / pending integration:** transfer · emission.

| ID | PRs | What |
|----|-----|------|
| **A-4** | #90 | Soft-aggregate tolerance policy (`SoftAggregateGuard`) |
| **B-1–B-3** | #91–#95 | `AccumulatorOpSession`, kernel subset, timestamps |
| **B-4I** | #108 | Production `SlotSummaryGpu` (32 B/slot, group checksums) |
| **C-1** | #97–#98 | Threshold scan → AccumulatorOp; single-submit integration |
| **C-2** | #99–#100 | Intent affine → AccumulatorOp |
| **C-3** | #105–#107 | Overlay Add-only + OrderBand exact f32 order foundation |
| **C-4** | #118 | Full Add/Multiply/Set overlay OrderBand compiler + dirty cache |
| **C-4 remedial** | #120 | Structural lifecycle/fission/cache hardening + consume-mode regressions |
| **S-3** | #141 | Legacy overlay shader/pipeline deleted; AccumulatorOp OrderBands sole overlay path |
| **C-5** | #122 | Mean / WeightedMean soft reductions → `ReductionSoft` on `output_vectors` |
| **C-5 remedial** | #123 | Depth-interleaved soft/exact reduction per depth bucket |
| **C-6** | #124 | Sum / Max / Min / First exact reductions; full AccumulatorOp path when soft+exact on |
| **S-4** | #126 | Legacy `reduction.wgsl` deleted; AccumulatorOp sole reduction path; flags default on |
| **C-7** | #127 | GovernedPair velocity integration → AccumulatorOp `IntegrateWithClamp`; dt via tick params |
| **C-8a** | #129 | EML infrastructure: execution classes, registry, persistent GPU program table, EvalEML interpreter (ExactDeterministic only); `use_accumulator_eml` (default **true**) |
| **C-8a remedial** | #130 | Node-count accounting, unchanged boundary skip, empty-upload generation bump, HardThreshold gate, PARAM validation, CpuOracleOnly debug registration |
| **C-8b** | #131 | Intensity migration: `use_accumulator_intensity`, `IntensityBehavior` → EvalEML, boundary sync upload, tick placement after velocity |
| **C-8b remedial** | #132 | Intensity op upload cache keys on `IntensityEmlOpPlanSignature` (EML generation + world/op-plan shape); slot growth and entry/layout changes force op reupload; `replace_formula_if_changed` avoids boundary EML table churn when formulas unchanged |
| **C-8c** | #133 | Transfer substrate: `use_accumulator_transfer`, persistent `AccumulatorInputListTable`, `MinAcrossInputs` + `SubtractFromAllInputs`, GPU dispatch after intensity/before overlay; `TransferConservation` = `ExactDeterministic` only |
| **C-8c remedial** | #134 | Planner rejects same-band consumed-input contention; validates unit costs and single-source `output_scale`; defensive source debit clamp; input-list generation bump on nonempty→empty clear |
| **C-8d** | #135 | Emission substrate: `use_accumulator_emission`, `EmissionRegistration` planner, `IdentityFloor` / `Constant` / `EvalEML` ExactDeterministic, GPU dispatch after transfer/before overlay; `EmissionRecordGpu { reg_idx, emit_count }` unchanged; stable `reg_idx` via `combine_b`; overflow observable |
| **C-8d remedial** | #136 | Emission op-plan signature includes `reg_indices`, `constant_value_bits`, `max_emit`; EvalEML tree IDs derived/validated from formula variant; `max_emit` explicitly rejected until shader clamp implemented |
| **C-8 completion gate** | #137 | Full C-8 all-flags integration test; persistent table/op reuse; [`s2_legacy_intensity_sunset_inventory.md`](s2_legacy_intensity_sunset_inventory.md) |
| **S-2** | #138 | Legacy `intensity_update.wgsl` + Pass 2 pipeline deleted; EvalEML intensity only; `use_accumulator_intensity` + `use_accumulator_eml` default **true** |
| **S-6** | `6b9bf8f` | Legacy `threshold_scan.wgsl` + Pass 7 pipeline deleted; AccumulatorOp threshold mandatory for registered thresholds |
| **S-5** | `6b9bf8f` | Legacy `velocity_integration.wgsl` + Pass 1 pipeline deleted; AccumulatorOp velocity mandatory for governed pairs |
| **S-1** | `6b9bf8f` | Legacy `intent_delta.wgsl` + intent pipeline deleted; AccumulatorOp intent mandatory for pending intents |
| **E-1** | #144 | `emit_on_threshold` builder in `simthing-core`; compiles to C-1 threshold+EmitEvent ops; re-registration helpers |
| **E-2A** | #146 | `resource_transfer_discrete` builder; exact SubtractFromSource discrete transfer |
| **E-3** | #147 | `conjunctive_recipe` builder; lifted CPU N≤4 cap; C-8c MinAcrossInputs + SubtractFromAllInputs |
| **E-3R** | #148 | `throttle_hint_max_per_tick` metadata hardening; E-4 must not treat as GPU cap |
| **E-7** | #149 | `governed_by` planner generalization; arbitrary `(Named, Named)` pairs via `IntegrateWithClamp` |
| **E-8** | #150 | `accumulator_spec` on `SubFieldSpec`; compile-time Resource Flow metadata only |
| **E-9** | #151 | `ArenaRegistry` driver session artifact; explicit admission + subtree refresh |
| **E-9R** | #152 | `participant_range` contiguity hardening via arena-major canonicalization |
| **E-10** | #153 | Resource Flow admission framework; expansion report; bad-spec hard rejection |
| **E-10R** | — | Driver participant identity preflight + reserved-gap admission |
| **E-8R** | — | Arena-internal plumbing columns at property compile |
| **E-7R** | — | `plan_governed_integration_at_band` for E-11 integration placement |
| **E-11** | #159 | Flat-star D=2 GPU slice; nested GPU deferred |
| **E-11R** | #160 | Sync errors + session-path test + scope honesty |
| **E-11 burn-in** | #161 | Controlled flat-star scaffold; flag default false |
| **E-11 burn-in scenarios** | #162 | Named fixtures + `e11_burn_in_scenarios_*` 6/6 |
| **E-11 CI soak** | — | Opt-in soak `e11_resource_flow_soak` 6/6; flag default false |
| **T-1** | #165 | `resource_economy` authoring types + RON roundtrip 12/12 |
| **T-2** | #166 | `compile::resource_economy` validation + expansion report 19/19 |
| **T-3** | #167 | `resource_economy_compile` materialization + stable reg_idx 11/11 |
| **T-4** | #168 | Session integration + boundary refresh; generation skip; flag-off reject 8/8 |
| **T-5** | #169 | Boundary refresh / replay / 100-tick conservation burn-in 13/13 |
| **T-6** | `3294e6f` | Limited opt-in scenario flagging; global transfer/emission defaults remain false |
| **D-1** | (pending merge) | Discrete-transaction contention memo; D-2 deferred |
| **E-11B readiness** | (pending merge) | Nested hierarchy GPU audit; implementation deferred by default |
| **E-2B static enrollment** | (pending merge) | Selector → explicit participants at install |
| **E-2B-5** | merged (`a740845`) | Policy A dynamic fission enrollment |
| **E-2B-5R** | (pending merge) | Atomicity + visible boundary diagnostics |
| **Pivot-forward** | #102, #108 | Policy doc, encode fixes, atomic WGSL values |
| **C-INF-1/2** | #109 | `WorldAccumulatorRuntime` on `WorldGpuState`; legacy oracle harness |
| **Remedial** | #111 | Authoritative flags clear stale sessions; `WorldSummaryRuntime` for integrated B-4 summary |

### Runtime shape (post #109/#111)

```text
WorldGpuState
  accumulator_runtime: Option<WorldAccumulatorRuntime>
    intent_session / threshold_session / overlay_session / reduction_soft_session / velocity_session / intensity_eml_session / transfer_session / emission_session
    overlay_compile_cache: Option<OverlayCompileCache>    (C-4 dirty/cached planner)
    summary: Option<WorldSummaryRuntime>                  (B-4 from world values)
  accumulator_overlay_add_active / _bands                 (cached dispatch; survives session take)
  accumulator_reduction_soft_active / _bands              (C-5 cached dispatch)
  accumulator_reduction_exact_active                      (C-6 full path; no legacy fallback)

BoundaryProtocol flags → sync clears or ensures families
  use_accumulator_reduction_soft + use_accumulator_reduction_exact (exact requires soft)
Dispatcher → take/put sessions; encode world summary after Accumulator passes when active
```

**Overlay policy (C-4):** the compatibility flag `use_accumulator_overlay_add`
now routes full Add/Multiply/Set batches through AccumulatorOp OrderBands using
the canonical `build_overlay_deltas` output. S-3 deleted legacy Pass 3; the flag
defaults true and disabled overlay workloads reject instead of falling back.

**Reduction policy (S-4):** `use_accumulator_reduction_soft` + `use_accumulator_reduction_exact`
default **true** and must both be enabled. Production tick: copy `values` → `output_vectors`,
then AccumulatorOp reduction bands. No legacy `reduction.wgsl`. Topology planning
(`child_starts`, `depth_bucket_ranges`, `plan_reduction_orderband`) preserved. Non-contiguous
child slots skip reduction upload until topology is planner-compatible.

**S-4 landed:**
- Legacy `reduction.wgsl` deleted.
- Legacy reduction pipeline, bind group, `skip_soft_columns`, and exact-fallback branch deleted.
- AccumulatorOp covers Mean, WeightedMean, Sum, Max, Min, First.
- Two-buffer semantics preserved (`values` → `output_vectors`).
- THRESH_BUF_OUTPUT semantics unchanged.

**Feature flags (authoritative after #111 + sunsets):** empty workload flag-off sync clears
stale sessions; disabling accumulator families with real deleted-legacy workloads rejects
explicitly instead of falling back or silently skipping work.

### Open migration work

| Priority | ID | Owner | Blocks |
|----------|-----|-------|--------|
| Design | **D-1** discrete-transaction memo | Opus | **Done** — contention audit; D-2 deferred ([memo](../reviews/d1_discrete_transaction_contention_memo.md)) |
| Design | **E-11B** nested hierarchy readiness | Opus | **Done** — audit; defer by default ([memo](../reviews/e11b_nested_hierarchy_gpu_readiness_review.md)) |
| Design | **E-2B** enrollment compilation readiness | Opus | **Done** — audit ([memo](../reviews/e2b_resource_flow_enrollment_compilation_readiness.md)) |
| Implementation | **E-2B** static enrollment E-2B-1…4 | Composer 2.5 | **Done** — selector install + session tests |
| Design | **E-2B-5 / E-2B-5R** dynamic fission enrollment | Cursor | **Done** — Policy A + atomicity hardening ([memo](../reviews/e2b5_dynamic_fission_enrollment_readiness.md), [E-2B-5R test report](../tests/e2b5r_dynamic_fission_enrollment_atomicity_test_results.md)) |
| Implementation | **Phase T** transfer/emission registration ownership | Cursor (Codex 5.5 + Composer 2.5) | **Complete** — T-1 through T-6 landed; default-off / explicit opt-in production posture ([Opus memo](../reviews/transfer_emission_registration_ownership_opus_review.md)) |
| Infra | Test-harness cleanup | Optional | Runtime legacy oracle/fallback peers are gone; remaining cleanup is test-only ergonomics |

### Sunset targets (S-phase)

| S-PR | After | Deletes | Status |
|------|-------|---------|--------|
| S-1 | C-2 default-on | Legacy intent pass | **Done (`6b9bf8f`)** |
| S-2 | C-8b default-on | Legacy intensity (`intensity_update.wgsl`) | **Done (#138)** |
| S-3 | C-3 + C-4 | Legacy overlay prep | **Done (#141)** |
| S-4 | C-5 + C-6 | Legacy reduction passes + `reduction.wgsl` | **Done (#126)** |
| S-5 | C-7 | Legacy velocity | **Done (`6b9bf8f`)** |
| S-6 | C-1 default-on | Legacy threshold scan (Pass 7) | **Done (`6b9bf8f`)** |

---

## 3. V6 spec layer (parked)

**Complete:** `simthing-spec` PRs 1–11; Opus P0 O2 (replay v3), B3 (boundary skip), I1 (install atomicity).

**Next (optional, Sonnet/Composer):** modder guide refresh (D1), RON examples (D2), capability-tree preview docs (D3). **E0 base economy deferred.**

**Ledger:** [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md)  
**Parking synthesis:** [`../design_v6.5.md`](../design_v6.5.md)

---

## 4. Workshop crate (`simthing-workshop`)

Isolated viability spikes — **not production code**, no workspace dependents.

| Spike | Status | Implication |
|-------|--------|-------------|
| EML Phase 5 intensity | PASS at 100k | Research only; not general EML path |
| WeightedMean AccumulatorOp | LOOSE_TOLERANCE at 100k; BIT_EXACT on production-shape fixture | Do not replace production reduction without ADR |
| Multichannel battery | Spec in `multichannel_accumulator_test_battery.md` | Pivot-readiness gates for workshop only |

See `crates/simthing-workshop/README.md` and `todo.md` § workshop spikes.

---

## 5. Tests (AccumulatorOp focus)

| Suite | Count | Notes |
|-------|-------|-------|
| `simthing-gpu` `accumulator_op` | 63 | Includes `WorldSummaryRuntime` and C-4 consume-mode unit tests |
| C-1 parity | 2 | incl. fission stress |
| C-2 parity | 11 | incl. combined C-1/C-2 |
| C-3 parity | 13 | incl. combined C-1/C-2/C-3 |
| C-4 parity/cache | 16 | Add/Mul/Set parity, lifecycle/fission/cache, high-density guards |
| C-5 reduction | 15 | `reduction_orderband` (6) + legacy oracle (2) + parity/guards (11) |
| C-6 exact reduction | 8 | Sum/Max/Min/First parity vs CPU oracle golden |
| S-4 sunset | 4 | Shader absent, all-rules golden, no CPU production reduction, combined path |
| S-1 sunset | 4 | Shader absent, default-on, disabled rejection, AccumulatorOp mandatory |
| S-5 sunset | 4 | Shader absent, default-on, disabled rejection, CPU/golden parity |
| S-6 sunset | 4 | Shader absent, default-on, disabled rejection, threshold mandatory |
| C-7 velocity | 8 | IntegrateWithClamp bit-exact vs CPU/golden oracle; legacy velocity pass deleted in S-5 |
| C-INF-2 harness | 2 | intent + threshold oracle smoke |
| Pivot-forward remedial | 3 | authoritative flags |
| B-4 world summary integrated | 2 | intent + overlay orderbands |

```powershell
cargo test -p simthing-gpu accumulator_op
cargo test -p simthing-gpu overlay_orderband
cargo test -p simthing-sim --test c1_threshold_scan_parity --test c2_intent_accumulator_parity --test c3_overlay_add_accumulator_parity
cargo test -p simthing-sim --test c4_overlay_orderband_parity
cargo test -p simthing-sim --test c5_legacy_weighted_mean_oracle --test c5_weighted_mean_reduction_parity
cargo test -p simthing-sim --test c6_exact_reduction_parity
cargo test -p simthing-sim --test c7_velocity_accumulator_parity
cargo test -p simthing-sim --test s1_intent_sunset --test s4_reduction_sunset --test s5_velocity_sunset --test s6_threshold_sunset
cargo test -p simthing-gpu reduction_orderband
cargo test -p simthing-sim --test c_inf_legacy_oracle_harness --test pivot_forward_remedial --test b4_world_summary_integrated
cargo check --workspace
```

---

## 6. Active workshop documents

| Document | Role |
|----------|------|
| **This file** | Current-state synthesis and routing |
| [`pivot_forward_implementation_policy.md`](pivot_forward_implementation_policy.md) | Active migration doctrine; legacy runtime fallbacks are deleted, CPU/golden oracles are test-only |
| [`slot_summary_b4_design.md`](slot_summary_b4_design.md) | Accepted B-4 summary tier design |
| [`c1_perf_reframe_memo.md`](c1_perf_reframe_memo.md) | Accepted C-1 perf gate reframe (no 5× readback claim) |
| [`c4_overlay_orderband_compiler_design.md`](c4_overlay_orderband_compiler_design.md) | Accepted C-4 overlay OrderBand design |
| [`c5_weighted_mean_reduction_design.md`](c5_weighted_mean_reduction_design.md) | Accepted C-5 design — soft-reduction migration |
| [`c8_eml_transfer_intensity_design.md`](c8_eml_transfer_intensity_design.md) | Accepted C-8 design — execution-class taxonomy + staged delivery (C-8a/b/c/d). C-8 baseline = `ExactDeterministic` only; substrate future-prepped for `SoftDeterministic` / `FastApproximate` / `CpuOracleOnly` classes |
| [`multichannel_accumulator_test_battery.md`](multichannel_accumulator_test_battery.md) | Workshop benchmark spec |
| [`simthing_modder_object_guide.md`](simthing_modder_object_guide.md) | Modder-facing authoring surface |
| [`simthing_base_economic_system_working_doc.md`](simthing_base_economic_system_working_doc.md) | Provisional economic substrate (E0 deferred) |
| [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md) | V6 spec implementation ledger |

---

## 7. Archived workshop documents

Superseded handoffs and historical Q&A live in [`archive/`](archive/). **Do not implement from archived files.**

| Archived | Was | Read instead |
|----------|-----|--------------|
| `simthing_spec_sonnet_opus_handoff.md` | Opus P0 / Sonnet backlog routing (2026-05-23) | This file §3 · `todo.md` · progress log |
| `capability_tree_studio_workshop.md` | 2026-05-22 studio Q&A | `design_v6.5.md` · progress log |
| `tech_tree_decisions.md` | 2026-05-21 workshop decisions | progress log · `capability_tree_v1.md` |
| `soft_aggregate_tolerance_audit.md` | A-4 Opus audit (pre-implementation) | `adr_accumulator_op_v2.md` · landed `SoftAggregateGuard` |
| `chatgpt_implementation_review.md` | 2026-05 perf/arch review | Historical; many items since addressed |

Full manifest: [`archive/SUNSET.md`](archive/SUNSET.md)

---

## 8. Top-level doc map (outside workshop/)

| Document | Role |
|----------|------|
| [`../todo.md`](../todo.md) | Priority table, PR ledger, open items |
| [`../worklog.md`](../worklog.md) | Session-by-session landing notes |
| [`../design_v7.md`](../design_v7.md) | **Active** GPU + economic spec (supersedes v6 §10) |
| [`../design_v6.5.md`](../design_v6.5.md) | V6 spec/session parking synthesis |
| [`../design_v6.md`](../design_v6.md) | V6 mechanics (overlays, fission, boundary) |
| [`../accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md) | 33-PR migration ladder |
| [`../adr_accumulator_op_v2.md`](../adr_accumulator_op_v2.md) | AccumulatorOp ADR |
| [`../agents.md`](../agents.md) | Agent map and crate conventions |

---

## 9. Read order for new agents

**GPU / AccumulatorOp migration:**

1. This document
2. `pivot_forward_implementation_policy.md`
3. `design_v7.md` §2–§4 (constitution + pass migration table)
4. `accumulator_op_v2_production_plan.md` — find your PR section
5. `todo.md` + recent `worklog.md` entries
6. Code: `accumulator_op/runtime.rs`, `world_state.rs`, `boundary.rs`, `passes.rs`, `dispatcher.rs`

**Spec / session / modder work:**

1. `design_v6.5.md`
2. `simthing_spec_progress_log.md`
3. `simthing_modder_object_guide.md`
4. `todo.md` § Sonnet D1/D2

---

## 10. Migration handoff template

Every future C-family PR must include:

```text
Pivot posture:
  AccumulatorOp path is the intended production path.
  Legacy path is oracle/fallback only.

Sunset target:
  S-<n> — <old pass deletion>

Legacy interaction allowed:
  oracle / fallback / none

Legacy interaction forbidden:
  no new features · no optimization · no semantic expansion
```

See `pivot_forward_implementation_policy.md` §4 for full policy.

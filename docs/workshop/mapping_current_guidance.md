# Mapping Current Guidance

Authoritative decision:

- [`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md)

Constitutional surfacing:

- [`../design_v7_7.md`](../design_v7_7.md)
- [`../design_v7_6.md`](../design_v7_6.md)
- [`../invariants.md`](../invariants.md) — Mapping (Sparse RegionCell) rows

Active read order (authoritative path for agents):

1. [`phase_m_gating_and_doc_policy.md`](phase_m_gating_and_doc_policy.md) — **read first: which lane is your change, and how much doc does it need**
2. `docs/invariants.md` (binding constraints)
3. The compact status table below (where each slice is)
4. `docs/accumulator_op_v2_production_plan.md` (PR ladder)
5. `docs/workshop/eml_gadget_library_design_note.md`; `docs/workshop/m5_gradient_extraction_design_note.md`
6. The single test report for the slice you're touching

Historical/superseded artifacts live under `docs/workshop/archive/`. Do not treat archived files as active authority. **Per-policy:** the verbose per-slice narrative blocks further down this file are superseded by the status table; `docs/worklog.md` is the append-only history. Collapse verbose blocks when you touch this file (don't grow them).

## Phase M — compact status (single source of truth)

`Lane`: T1 = fast-lane (one PR + one test report + one row), T2 = gated (design-review → acceptance → impl). See the gating policy.

| Slice | Lane | Status | Notes |
|---|---|---|---|
| First-slice runtime R1/R2/R3 | T2 | accepted | GPU-resident stencil→reduction→EML→threshold; opt-in, default-off |
| Product fixture chain (economy + SEAD) | T2 | accepted | economy→SEAD link stays `tests/support` fixture-only |
| Boundary resolution doctrine (tick/boundary/day) | T2 | accepted | legible names preferred; no calendar/pause sim semantics |
| EML-GADGET-1 (Tier-1 stateless gadgets) | T2 | accepted | FieldSampler / WeightedAccumulator / SoftStep |
| EML-GADGET-2 (temporal: 2A–2E) | T2 | landed | explicit-column memory; bounded-feedback admission; Acceleration via explicit velocity col |
| **M-5A-gradient** (single-target Gradient op + per-direction weights) | **T1** | **landed** | [`phase_m_m5a_gradient_single_target_test_results.md`](../tests/phase_m_m5a_gradient_single_target_test_results.md) |
| **M-5B-gradient** (L3 composition RON fixture) | **T1** | **landed** | [`phase_m_m5b_gradient_l3_composition_test_results.md`](../tests/phase_m_m5b_gradient_l3_composition_test_results.md); R1 integrated evidence: [`phase_m_m5b_gradient_l3_composition_r1_test_results.md`](../tests/phase_m_m5b_gradient_l3_composition_r1_test_results.md) |
| **M-5C-gradient** (need/routing signal product fixture) | **T1** | **landed** | [`phase_m_m5c_gradient_need_signal_test_results.md`](../tests/phase_m_m5c_gradient_need_signal_test_results.md) |
| **M-5D-gradient + R1** (gradient strict-sink admission + grouped frame compile helper) | **T1** | **landed** | strict-sink validator: [`phase_m_m5d_gradient_sink_admission_test_results.md`](../tests/phase_m_m5d_gradient_sink_admission_test_results.md); grouped helper: [`phase_m_m5d_r1_gradient_frame_compile_helper_test_results.md`](../tests/phase_m_m5d_r1_gradient_frame_compile_helper_test_results.md) |
| **M-5E-gradient** (scarcity/opportunity/logistics composite product fixture) | **T1** | **landed** | [`phase_m_m5e_gradient_scarcity_opportunity_test_results.md`](../tests/phase_m_m5e_gradient_scarcity_opportunity_test_results.md) |
| Dual-output `GradientXY`; L1 coupling; dense per-cell temporal | T2 | deferred | separate gate each |
| **Shader/software deterministic sqrt / SEAD observer** | **T2** | **SEAD-V1 consolidated** | SEAD-OBS-1: [`phase_m_sead_obs1_overlay_score_admission_results.md`](../tests/phase_m_sead_obs1_overlay_score_admission_results.md). SEAD-OBS-2: [`phase_m_sead_obs2_multilayer_overlay_score_results.md`](../tests/phase_m_sead_obs2_multilayer_overlay_score_results.md). SEAD-OBS-3: [`phase_m_sead_obs3_fixed_point_score_results.md`](../tests/phase_m_sead_obs3_fixed_point_score_results.md). SEAD-OBS-4: [`phase_m_sead_obs4_threshold_event_results.md`](../tests/phase_m_sead_obs4_threshold_event_results.md). SEAD-EVENT-0: [`phase_m_sead_event0_compaction_results.md`](../tests/phase_m_sead_event0_compaction_results.md). SEAD-PIPE-0: [`phase_m_sead_pipe0_observer_event_pipeline_results.md`](../tests/phase_m_sead_pipe0_observer_event_pipeline_results.md). SEAD-EVENT-1: [`phase_m_sead_event1_code_bucketing_results.md`](../tests/phase_m_sead_event1_code_bucketing_results.md). SEAD-EVENT-2: [`phase_m_sead_event2_bucket_reductions_results.md`](../tests/phase_m_sead_event2_bucket_reductions_results.md). SEAD-ACT-0: [`phase_m_sead_act0_numeric_proposals_results.md`](../tests/phase_m_sead_act0_numeric_proposals_results.md). SEAD-ACT-1: [`phase_m_sead_act1_phase_e_proposal_consumer_results.md`](../tests/phase_m_sead_act1_phase_e_proposal_consumer_results.md). SEAD-ACT-2: [`phase_m_sead_act2_proposal_admission_records_results.md`](../tests/phase_m_sead_act2_proposal_admission_records_results.md). SEAD-ACT-3: [`phase_m_sead_act3_economic_fixture_records_results.md`](../tests/phase_m_sead_act3_economic_fixture_records_results.md). SEAD-ACT-4: [`phase_m_sead_act4_economic_fixture_validation_corpus_results.md`](../tests/phase_m_sead_act4_economic_fixture_validation_corpus_results.md). SEAD-V1: [`phase_m_sead_v1_consolidation_results.md`](../tests/phase_m_sead_v1_consolidation_results.md). **SEAD-V1-CONSOLIDATE-0:** SEAD self-AI fixture ladder consolidated into Proposal Pipeline V1. OBS/EVENT/PIPE/ACT evidence accepted through ACT-2 as V1 core; ACT-3/ACT-4 retained as supporting Economic V1 fixture evidence. Stop ACT-N/EVENT-N expansion. FrontierV1 is the next opt-in/default-off M/E closure scenario. Charter: [`sead_self_ai_track.md`](sead_self_ai_track.md). F artifact hash `e2e9e27601ee2e13`. |
| **Frontier V1 — named closing scenario (M + E)** | **T2** | **ACCEPTED — M/E closed** | Single-theater strategic vertical that closes Phase M and Phase E on accepted substrates: one ≤32×32 grid (first-slice mapping, no atlas/mask), two flat-star depth-2 faction economies (FlatStarResourceFlow, no nested E-11B), mobile cohorts self-directing on the field via exact F-magnitude + SEAD self-AI, commitments via Threshold/EmitEvent. One opt-in `FrontierV1` profile, **default Disabled**. **FrontierV1-ACCEPT-0:** FrontierV1 accepted as M/E closing vertical. **FrontierV1-POSTACCEPT-0:** FrontierV1 accepted and M/E closed for bounded scope. No FrontierV1-5 or ACT/EVENT/OBS/PIPE ladder expansion authorized. Future work requires a separately named production gate (current: Pause F). Reports: [`phase_m_frontier_v1_0_scenario_skeleton_results.md`](../tests/phase_m_frontier_v1_0_scenario_skeleton_results.md), [`phase_m_frontier_v1_1_opt_in_fixture_results.md`](../tests/phase_m_frontier_v1_1_opt_in_fixture_results.md), [`phase_m_frontier_v1_2_gpu_replay_acceptance_results.md`](../tests/phase_m_frontier_v1_2_gpu_replay_acceptance_results.md), [`phase_m_frontier_v1_3_gpu_resource_flow_results.md`](../tests/phase_m_frontier_v1_3_gpu_resource_flow_results.md), [`phase_m_frontier_v1_4_sead_route_replay_results.md`](../tests/phase_m_frontier_v1_4_sead_route_replay_results.md), [`phase_m_frontier_v1_acceptance_review_results.md`](../tests/phase_m_frontier_v1_acceptance_review_results.md), [`phase_m_frontier_v1_post_acceptance_roadmap_results.md`](../tests/phase_m_frontier_v1_post_acceptance_roadmap_results.md). Charter: [`sead_self_ai_track.md`](sead_self_ai_track.md). |
| Atlas / M-4A; source-mask (`M-5`); economy→mapping bridge | T2 | deferred/prohibited | see prohibition list in gating policy |
| **M-4A Atlas Readiness Gate** | **T2** | **deferred** | no concrete product need beyond M-5-gradient substrate; [`phase_m_m4a_atlas_readiness_gate_results.md`](../tests/phase_m_m4a_atlas_readiness_gate_results.md) |
| **M-6A Single-Grid Active Mask Readiness Gate** | **T2** | **deferred** | missing halo contract + CPU/GPU parity; [`phase_m_m6a_single_grid_active_mask_readiness_results.md`](../tests/phase_m_m6a_single_grid_active_mask_readiness_results.md) |
| **Product Scenario Selection Gate** | **T2** | **selected → M-5E** | full-grid scarcity/opportunity/logistics composite; no new substrate; [`phase_m_product_scenario_selection_gate_results.md`](../tests/phase_m_product_scenario_selection_gate_results.md) |
| **EML-GADGET Runtime Execution Gate** | **T2** | **landed (fixture)** | per-gadget EvalEML runtime via existing AccumulatorOp; [`phase_m_eml_gadget_runtime_execution_gate_test_results.md`](../tests/phase_m_eml_gadget_runtime_execution_gate_test_results.md) |
| **M-JIT track (closed at PROD-0)** | **T2** | **CLOSED — accepted (Opus/Mapping-SEAD authority 2026-05-30, PASS WITH CONDITIONS; [`sead_self_ai_track.md`](sead_self_ai_track.md) §8)** | Default-off `ProductionKernelRegistryShell` + explicit registered exact `ProductionCandidatePreview` cohort execution; intermediate ladder reports deleted — retained evidence: [`phase_m_jit_prod0_registry_shell_test_results.md`](../tests/phase_m_jit_prod0_registry_shell_test_results.md), [`phase_m_jit_exec1_cohort_execution_fixture_test_results.md`](../tests/phase_m_jit_exec1_cohort_execution_fixture_test_results.md), [`phase_m_jit_sqrt_candidate_battery_r1_test_results.md`](../tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md), [`phase_m_jit_grad0_spatial_observer_r1_test_results.md`](../tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md), [`phase_m_jit_grad1_observer_formula_fusion_test_results.md`](../tests/phase_m_jit_grad1_observer_formula_fusion_test_results.md) |

**M-JIT status:** Track **closed at M-JIT-PROD-0** (`d62b09d`) pending/after Opus acceptance. Explicit registered exact `ProductionCandidatePreview` cohort execution is available only via test-invoked `ProductionKernelRegistryShell` calls (`production_wiring=false`, default-off). **Follow-on tracks remain gated:** shader/software sqrt exact path ([`sqrt_candidates.md`](sqrt_candidates.md)); production scheduler; runtime kernel cache; default SimSession wiring; production economy→mapping bridge; atlas/active mask/source identity; dual-output `GradientXY`; native sqrt exact authority; approximate `mag2` feeding exact score inputs; semantic WGSL; ClauseThing implementation (proposal-only).

**E-phase / E11 / Resource Flow:** Reports documenting stalled or review-blocked E-phase work are intentionally retained on `master`. Restart evidence: [`e11_implementation_handoff.md`](e11_implementation_handoff.md), [`e11_readiness_review.md`](e11_readiness_review.md), [`e11_hierarchical_allocation_design.md`](e11_hierarchical_allocation_design.md). JIT doc closeout did not delete E-phase stalled evidence.

> Per-slice landing history (EML-GADGET-2A…2E, boundary/economy, etc.) now lives in the status table above and in `docs/worklog.md`. The accepted designs and binding rules are in the design notes and `docs/invariants.md`. Standing posture ("no semantic WGSL / no default wiring / `simthing-sim` map-free / defaults unchanged") is binding from `invariants.md` and asserted once per PR test report — not restated per slice here.

## Phase M boundary-resolution (tick / boundary / day) + example economy (ACCEPTED — Opus/product 2026-05-29, PASS WITH CONDITIONS)

**Accepted** ([`../reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md`](../reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md)): boundary doctrine accepted; Daily Economy Fixture V1 accepted as an example/product fixture only; the `ResourceEconomySpec` (discrete banking) vs Resource Flow E-11 (continuous, default-off) distinction accepted; the future-agent guardrails made **binding** in [`../invariants.md`](../invariants.md) ("Boundary resolution (tick / boundary / day)"). **Naming preference (product, 2026-05-29):** `tick` / `boundary` / `day` / `day_index` / `ticks_per_day` are the **preferred, endorsed names for their legibility** — the earlier R1/R2 pull toward abstract/illegible alternatives is reversed; keep the legible names. The guardrail is on *semantics* only: avoid Clausewitz/calendar semantics (calendar arithmetic / `Calendar`/`Pause` types / leap-date math / `DailyResolutionBoundary`), **not** the legible day-flavored naming that already pervades `simthing-sim`. **Phase M Resource Economy Authoring Ergonomics V1 landed** — spec/admission preview for discrete economy fixtures. **Phase M Economy + SEAD Product Fixture V1 landed** — test-level product fixture proving discrete economy boundary treasury stress can drive opt-in first-slice SEAD commitment through existing GPU EML/Threshold+EmitEvent (fixture orchestration only; no production economy→mapping bridge). **Phase M product-fixture chain ACCEPTED (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — [`../reviews/phase_m_product_fixture_chain_acceptance_opus_review.md`](../reviews/phase_m_product_fixture_chain_acceptance_opus_review.md). The economy→SEAD link stays in `tests/support` fixture orchestration: the CPU reads resolved treasury and *selects* authored EML weight profiles; it never computes urgency or emits the commitment (both stay GPU-resident). Binding row added to [`../invariants.md`](../invariants.md) Mapping section (economy→mapping influence is fixture-only; no production bridge without a separate gated decision). **Phase M EML-GADGET-1 ACCEPTED (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — [`../reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md`](../reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md). Tier-1 stateless gadgets (`FieldSampler`, `WeightedAccumulator`, algebraic `SoftStep`) compile in `simthing-spec` over existing `EvalEML` opcodes with CPU-oracle parity; R1 composition (per-gadget executable; multi-gadget `PerGadgetOnly`; preview ≠ runtime) and R2 node-cap (per executable tree) accepted. Binding rows added to [`../invariants.md`](../invariants.md) ("EML Gadget Library"). *(The #262 parking packet was reverted off master; the acceptance memo is the authoritative review artifact.)* **Phase M EML-GADGET-2 temporal-memory design ACCEPTED as a gate (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — [`../reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md`](../reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md). Explicit-column temporal memory, Layer-3 default, authored snapshot/copy bands (`Identity`+`ResetTarget`, existing substrate, no new opcode), bounded-feedback admission contract (default `0 ≤ decay < 1`; clamp required when feeding a hard threshold; analytically-bounded escape must be admission-checkable). Candidates: VelocityMonitor / Decay/EMA / BoundedFeedback accepted; Hysteresis conditional; Acceleration deferred. Binding rows in [`../invariants.md`](../invariants.md) ("EML Gadget Library"). **Current EML-GADGET-2 status:** 2A snapshot/copy, 2A R1 sequence hygiene, 2B VelocityMonitor + Decay/EMA, 2C BoundedFeedback, 2D Hysteresis (2D R1 exact CMP/SELECT compiler parity), and **2E explicit velocity-column Acceleration** have landed in `simthing-spec` spec/admission/compiler/oracle surfaces. Consolidated review: [`docs/reviews/phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md`](../reviews/phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md). Resource Economy Authoring Ergonomics R2 landed as spec/admission/preview-only `schedule_lines` helper. Position-history acceleration and dense per-cell temporal memory remain separately gated. Runtime gadget execution, chained scheduling, atlas/M-4A, and any production economy→mapping bridge remain unauthorized.

Phase M abstract boundary-resolution + example economy review packet landed.
The repo now distinguishes abstract substrate tick/boundary cadence from game-level daily interpretation. `ticks_per_day` and `day_index` remain the legible API names; despite the names, day/calendar semantics are not part of simthing-sim.
Daily Economy Fixture V1 remains a valid product/example fixture showing one game-level interpretation: one boundary as one day, with discrete ResourceEconomySpec banking.
No runtime behavior changed.
No DailyResolutionBoundary primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Review packet:** [`../reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md`](../reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md)

## Phase M Boundary Resolution Doctrine audit (landed — docs+test audit)

Phase M Boundary Resolution Doctrine audit landed.
The substrate exposes abstract deterministic tick/boundary cadence through `ticks_per_day`, `boundary_reached`, `day_index`, boundary handlers, persistent GPU values, discrete resource-economy transfers, and summary-tier readback.
Despite the names, `day_index` and `ticks_per_day` do not make day/calendar semantics part of simthing-sim. A host may interpret `day_index` as a day, turn, frame, season, orbital step, market close, learning epoch, or other unit.
No DailyResolutionBoundary runtime primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
Daily meaning remains only one possible host/spec interpretation of `day_index`.
Pause/speed remain host/UI orchestration concerns: the deterministic sim advances only when the host requests ticks.
Example discrete boundary banking may use the discrete resource economy substrate, not the continuous Resource Flow substrate by default.
The CPU boundary consumes resolved summaries/events/metadata at the boundary; it must not scan dense RegionCell grids by default, recompute gameplay state, or emit AI commitments via CPU planner logic.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

Queue-write child resource scale caveat addressed for first-slice by generic bulk fill.
Parent scalar writes remain O(1).
Multi-field, multi-map, atlas, perception, source identity, and broader production scaling remain separately gated.

## Phase M Daily Economy Fixture V1 (landed — opt-in product/example fixture)

Phase M Daily Economy Fixture V1 landed as a product/example fixture.
It proves that a game can interpret one abstract boundary as one day and run daily banking through existing discrete ResourceEconomySpec authoring: ticks_per_day=1, boundary_reached/day_index, ResourceEconomySpec production, discrete transfers into storage, upkeep transfers out, and threshold/event checks over resolved storage.
This does not make daily cadence canonical for SimThing.
Other simulations may interpret the same boundary machinery as turns, frames, months, seasons, orbital steps, or other semantic units.
No DailyResolutionBoundary runtime primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
The CPU boundary consumes resolved storage/events/metadata; it does not recompute economy state or emit planner decisions.
Resource Flow E-11 remains continuous/high-frequency oriented and default-off, not the default discrete boundary-banking substrate.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Audit:** [`../tests/phase_m_boundary_cadence_doctrine_audit.md`](../tests/phase_m_boundary_cadence_doctrine_audit.md)

## Phase M Map Residency V1 (landed — opt-in)

Phase M Map Residency V1 landed.
It adds first-slice residency status/reporting over the accepted GPU-resident path: HotExecutedThisTick, ResidentCached, ColdSkipped, and DisabledUnavailable.
Residency status is metadata only. CPU does not recompute threat/urgency, emit commitment events, or mutate true field values for cached/skipped maps.
ResidentCached preserves visibility of prior GPU parent summaries through metadata while cached commitment scans remain deferred in V1.
No SummaryValidity behavior changed.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception/fog, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Runtime:** `FirstSliceResidencyReport` on `FirstSliceMappingReport.residency`. No new RON field in V1.

**Test:** [`../tests/phase_m_first_slice_map_residency_test_results.md`](../tests/phase_m_first_slice_map_residency_test_results.md)

## Phase M Queue-Write Scale Hardening V1 (landed — opt-in)

Phase M Queue-Write Scale Hardening V1 landed.
The first-slice GPU bridge no longer uses per-child resource queue writes for the child resource column. It uses a generic bounded bulk/preinitialized fill path while preserving the GPU-resident stencil → accumulator → reduction → EML → threshold event flow.
Parent scalar weight writes remain constant-size and acceptable for the single-grid first-slice path.
No SummaryValidity behavior changed.
No CPU-side gameplay cache was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency expansion, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Implementation:** `AccumulatorOpSession::fill_slot_range_col` (generic substrate helper; bounds-checked; no CPU readback). First-slice bridge reports `gpu_bridge_bulk_col_fills=1`, `gpu_bridge_bulk_fill_values=cell_count`, `gpu_bridge_parent_scalar_writes=2` on executed ticks.

**Remaining caveat:** V1 uses a generic GPU fill dispatch for strided column fills when count > 1 rather than a single contiguous buffer write. Parent weight/personality columns remain constant-size queue writes (O(1), not O(cell_count)).

**Test:** [`../tests/phase_m_queue_write_scale_hardening_test_results.md`](../tests/phase_m_queue_write_scale_hardening_test_results.md)

## Phase M SummaryValidity V1 (landed — opt-in)

Phase M SummaryValidity V1 landed.
It adds a bounded first-slice summary validity policy/status so a clean or skipped RegionField can report whether its strategic parent summary is fresh, cached, zero-initial, or unavailable without rerunning dense field propagation or rederiving gameplay state on CPU.
The hot path remains GPU-resident; cached summaries retain GPU-resident parent summary values and report metadata only.
No CPU-side AI planner was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency system, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Policy/status (V1):** `RegionFieldSummaryPolicySpec::CachedUntilDirtyWithZeroInitial` (default when omitted) → `FreshThisTick` on executed ticks; `Cached { age_ticks }` on clean skipped ticks after prior execution; `ZeroInitial` before any execution; `InvalidOrUnavailable` under Disabled profile. Summary metadata only — CPU does not recompute threat/urgency on skip.

**Cached commitment scan:** deferred — threshold/event scan runs only when the dense path executes; cached ticks report validity metadata without CPU-side commitment emission.

**V1-R1 hygiene + parking verification (landed):** Runtime summary status moved out of `simthing-spec` into `simthing-driver` as `FirstSliceSummaryStatus`. Designer-facing summary policy (`RegionFieldSummaryPolicySpec`) and compiled admission data (`CompiledRegionFieldSummaryPolicy`) remain in spec. SummaryValidity behavior unchanged. Full targeted first-slice verification green. All V7.7 / Mapping ADR / SEAD guardrails intact.

**Known scale caveat:** child-resource per-slot queue writes resolved for first-slice via bulk fill; parent scalar writes remain O(1). Broader multi-field/atlas scaling still gated separately.

**Test:** [`../tests/phase_m_first_slice_summary_validity_test_results.md`](../tests/phase_m_first_slice_summary_validity_test_results.md)

## Phase M first-slice vertical proof (ACCEPTED — Opus/product 2026-05-28)

The full first-slice vertical SEAD slice is **accepted as complete for the single-grid, opt-in
path** (PASS WITH CONDITIONS): RON authoring (`FirstSliceScenarioSpec` / `RegionFieldSpec` /
`FirstSliceCommitmentSpec`) → explicit `MappingExecutionProfile` opt-in → GPU-resident field
propagation → parent `SlotRange` Sum → `field_urgency` EvalEML → Threshold + EmitEvent commitment.
Acceptance memo: [`../reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md`](../reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md).

**Conditions:** resolve the per-slot queue-write scale caveat before any multi-field/atlas scaling;
all prohibitions hold (no atlas, no default wiring, no perception, no `source_mask`, no full map
residency until separately gated). **SummaryValidity V1 landed (2026-05-28).** Named next
implementation step: **queue-write scale hardening or broader map residency** — **not** the M-4
atlas packer.

## Phase M product fixture (landed — opt-in)

Phase M product-facing first-slice scenario fixture landed. It drives the accepted
GPU-resident first-slice runtime from a small product-style RegionFieldSpec/RON fixture:
one grid, source_capped_normalized, H<=8, caller-managed seed-only clear, dirty
scheduling, SlotRange Sum reduction, and parent field_urgency EvalEML.

The fixture proves default-off behavior, explicit SparseRegionFieldV1 opt-in,
GPU-resident hot path with reduction_stencil_readbacks=0, finite propagated field
values, and personality/weight-sensitive urgency.

No atlas batching landed. No M-4A atlas masking landed. No active mask, perception,
map residency, behavioral source policy, or source_mask landed. No semantic WGSL landed.
simthing-sim remains map-free. Defaults unchanged.

Known caveat: First-slice bridge uses queue writes for child resource values and parent
weights. This is acceptable for the 10x10 first-slice fixture. Future multi-field/atlas
scale must replace per-slot resource writes with a generic preinitialized resource column,
fill helper, or GPU fill kernel after a separate measured design step.

See [`../tests/phase_m_first_slice_product_fixture_test_results.md`](../tests/phase_m_first_slice_product_fixture_test_results.md).

## Phase M product commitment fixture (landed — opt-in)

Phase M product commitment fixture landed. It extends the product-facing first-slice
fixture by using the existing threshold/event substrate over parent field_urgency,
proving the SEAD commitment path: GPU-resident field propagation -> parent reduction ->
EvalEML urgency -> threshold event.

Low-weight profile stays below threshold; high-weight profile crosses and emits the
expected event. No CPU-side AI planner was introduced.

No atlas batching landed. No M-4A atlas masking landed. No active mask, perception,
map residency, behavioral source policy, or source_mask landed. No semantic WGSL landed.
simthing-sim remains map-free. Defaults unchanged.

Known caveat: First-slice bridge uses queue writes for child resource values and parent
weights. This is acceptable for the 10x10 first-slice and commitment fixtures. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

See [`../tests/phase_m_first_slice_product_commitment_fixture_test_results.md`](../tests/phase_m_first_slice_product_commitment_fixture_test_results.md).

## Phase M CommitmentSpec fixture (landed — opt-in)

Phase M CommitmentSpec fixture landed. It moves the first-slice commitment threshold/event
binding into a designer/spec-facing RON-admitted configuration while preserving the existing
GPU-resident SEAD path: field propagation -> parent reduction -> field_urgency EvalEML ->
Threshold + EmitEvent.

Low-weight profile remains below the authored threshold; high-weight profile crosses and
emits the authored event. No CPU-side AI planner was introduced.

No atlas batching landed. No M-4A atlas masking landed. No active mask, perception,
map residency, behavioral source policy, or source_mask landed. No semantic WGSL landed.
simthing-sim remains map-free. Defaults unchanged.

Known caveat: First-slice bridge uses queue writes for child resource values and parent
weights. This is acceptable for the 10x10 first-slice and commitment fixtures. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

See [`../tests/phase_m_first_slice_commitment_spec_test_results.md`](../tests/phase_m_first_slice_commitment_spec_test_results.md).

## Phase M FirstSliceScenarioSpec fixture (landed — opt-in)

Phase M FirstSliceScenarioSpec fixture landed.
It wraps the accepted first-slice RegionFieldSpec + CommitmentSpec in a scenario-level RON
authoring shape that includes explicit MappingExecutionProfile.
Disabled scenarios admit as structure but do not execute. SparseRegionFieldV1 scenarios
execute the GPU-resident first-slice path and emit the authored commitment event only when
field_urgency crosses the authored threshold.
No CPU-side AI planner was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

Known caveat: First-slice bridge uses queue writes for child resource values and parent
weights. This is acceptable for the 10x10 first-slice scenario fixture. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

See [`../tests/phase_m_first_slice_scenario_spec_test_results.md`](../tests/phase_m_first_slice_scenario_spec_test_results.md).

## Phase M FirstSliceScenarioSpec-R1 hygiene (landed — opt-in)

Phase M FirstSliceScenarioSpec-R1 hygiene landed.
The scenario-level RON wrapper remains opt-in and GPU-resident. The public/test-only boundary
was clarified (`FirstSliceScenarioFixtureSession` moved to integration-test support code;
production retains `FirstSliceMappingSession::open_from_scenario_preview` only), scenario
budget estimate handling was hardened (estimator errors propagate instead of `.ok()`), and
the prior build/test crash history was documented with a final clean verification run.
No default SimSession wiring was introduced.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

Known caveat: First-slice bridge uses queue writes for child resource values and parent
weights. This is acceptable for the 10x10 first-slice scenario fixture. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

See [`../tests/phase_m_first_slice_scenario_spec_r1_hygiene_test_results.md`](../tests/phase_m_first_slice_scenario_spec_r1_hygiene_test_results.md).

## Phase M first-slice vertical proof (parked — Opus/product review)

Phase M first-slice vertical proof parked for Opus/product review.
The landed chain now covers scenario-level RON authoring with explicit MappingExecutionProfile,
RegionFieldSpec, CommitmentSpec, GPU-resident field propagation, parent reduction, field_urgency
EvalEML, and Threshold + EmitEvent commitment.
No additional runtime behavior landed in this parking pass.
No default SimSession wiring was introduced.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

Known caveat: First-slice bridge uses queue writes for child resource values and parent
weights. This is acceptable for the 10x10 first-slice scenario fixture. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

Review packet: [`../reviews/phase_m_first_slice_vertical_proof_review_packet.md`](../reviews/phase_m_first_slice_vertical_proof_review_packet.md) (accepted — Opus/product 2026-05-28)

## Phase M-first-slice (landed — opt-in)

Phase M-first-slice runtime landed behind explicit `MappingExecutionProfile::SparseRegionFieldV1` opt-in in `simthing-driver` (`FirstSliceMappingSession`). It exercises one bounded RegionField grid with `source_capped_normalized`, H≤8, caller-managed one-shot seed then zero, dirty skip, SlotRange Sum reduction, and parent `field_urgency` EvalEML.

**M-first-slice-R3 landed:** GPU-resident first-slice readiness/observability parking pass. The first-slice hot path remains GPU-resident through stencil, SlotRange Sum reduction, and field_urgency EvalEML. R3 adds readiness/cost-shape reporting and locks the no-hidden-readback invariant for Opus/product review. No atlas batching landed. No M-4A atlas masking landed. No active mask, perception, map residency, behavioral source policy, or source_mask landed. No semantic WGSL landed. simthing-sim remains map-free. Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and parent weights. This is acceptable for the 10×10 first slice. Future multi-field/atlas scale should replace per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU fill kernel after a separate measured design step.

**M-first-slice-R2 landed:** GPU-resident Layer 1→2→3 bridge. See [`../tests/phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md`](../tests/phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md).

See [`../tests/phase_m_first_slice_runtime_r3_readiness_test_results.md`](../tests/phase_m_first_slice_runtime_r3_readiness_test_results.md).

## Parked status (Phase M-4)

Phase M-4 isolation policy is **ratified** (Opus 2026-05-28); **atlas implementation remains gated.** Atlas batching remains provisional and unimplemented. The design note defines the **contract** — it is **not** implementation authorization:

- algebraic tile-local mask G=0 (**ratified preferred**) **or** gutter >= effective horizon (fallback) for homogeneous square batches (M-4A evidence)
- physical gutter is the fallback when algebraic masking is not configured/admitted or the layout is not homogeneous-square
- mandatory VRAM accounting
- per-tile seed clearing (column-wide `source_col` zeroing banned)
- full-tile protocol-oracle parity required
- t44/corridor agreement alone is **insufficient** for production acceptance

**M-4A (2026-05-19 sandbox; ratified 2026-05-28):** Algebraic tile-local masking sandbox completed and was reverted to parked state. Candidate code/results are preserved under `docs/workshop/` and `docs/tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`. For homogeneous square atlas batches, G=0 algebraic tile-local masking is the **ratified preferred isolation candidate** over physical G>=H gutters (Opus 2026-05-28, under human delegation — [`../reviews/m4_m4a_first_slice_oversight_opus_review.md`](../reviews/m4_m4a_first_slice_oversight_opus_review.md)). Physical gutter remains fallback; mixed-size local-bounds metadata remains deferred. **Ratification is of the isolation policy only — atlas remains unimplemented and `request_atlas_batching` stays rejected at admission until a §11-gate-passing M-4 PR.** No atlas implementation landed. No mapping runtime beyond the opt-in first slice landed.

For broader implications of M-4A algebraic masking, see the M-4 design note section **“Architectural Implications of Algebraic Tile-Local Masking.”**

## Current decision gate (resolved 2026-05-28)

Option B was taken and is complete: the first-slice runtime landed and was hardened through
R1/R2/R3 and **accepted by Opus as a stable base**
([`../reviews/m4_m4a_first_slice_oversight_opus_review.md`](../reviews/m4_m4a_first_slice_oversight_opus_review.md)).
The product-facing first-slice scenario fixture and commitment fixture (single grid, no atlas)
are now landed.
The atlas packer remains deferred.

| Option | Path | Status |
|--------|------|--------|
| **A** | Implement the generic M-4 atlas packer | **Deferred** — admissible only after a named multi-theater scenario, an approved VRAM budget, and a §11-gate-passing PR. Not next. |
| **B** | First-slice runtime wiring (single grid, no atlas) | **Done** — landed, hardened (R1/R2/R3), accepted. |
| **Next** | Product-facing first-slice scenario fixture on the landed runtime | **Done** — landed as an opt-in product fixture; no atlas/active-mask/perception/source_mask/new-WGSL/default-on. |
| **Next SEAD proof** | Threshold event over first-slice urgency | **Done** — landed as an opt-in commitment fixture; no CPU-side planner. |
| **Next authoring proof** | Designer-facing commitment threshold binding | **Done** — landed as RON-admitted `FirstSliceCommitmentSpec`; no default wiring. |
| **Next scenario wrapper** | Scenario-level RON with explicit execution profile | **Done** — landed as `FirstSliceScenarioSpec`; disabled admits without execution; SparseRegionFieldV1 executes GPU-resident path. |
| **Vertical proof parking** | Opus/product review packet | **Done** — parked; see [`../reviews/phase_m_first_slice_vertical_proof_review_packet.md`](../reviews/phase_m_first_slice_vertical_proof_review_packet.md). |

## Landed Phase M natives

- **M-1 landed:** generic `StructuredFieldStencilOp::execute_configured` execution API and column-aware reduction convenience over existing `SlotRange` Sum
- **M-1.1 landed:** no-readback dispatch/report path for future schedulers; readback explicit for tests/diagnostics and readback-derived stats
- **M-2 landed:** generic cadence scheduler and dirty macro-region skip helper
- **M-2.1 landed:** FieldScheduler API hardening — region identity keyed by `(FieldId, FieldRegionId)`; visitor-based scheduled execution
- **M-first-slice-R3 landed (opt-in):** GPU-resident readiness/observability parking — [`FirstSliceMappingSession`](../../crates/simthing-driver/src/first_slice_mapping_runtime.rs)
- **M-first-slice-R2 landed (opt-in):** GPU-resident Layer 1→2→3 bridge — [`FirstSliceMappingSession`](../../crates/simthing-driver/src/first_slice_mapping_runtime.rs)
- **M-3 landed:** RegionFieldSpec RON + mapping admission framework — designer/spec structure only; compiles/previews to generic substrate configs; MappingExecutionProfile default Disabled
- **M-4 design note landed (parked):** [`mapping_atlas_batching_isolation_design_note.md`](mapping_atlas_batching_isolation_design_note.md)

**Deferred (M-3):** Perception field enum/class names remain admission-category placeholders only.

Historical sandbox source, candidate notes, revert reports, and full logs live under
[`archive/mapping/`](archive/mapping/) and [`../tests/archive/`](../tests/archive/).
They remain valid evidence but are not active guidance.

The opt-in first-slice runtime is landed and accepted as a stable base (R1/R2/R3), and the
Option 3 product-facing first-slice scenario fixture plus threshold commitment fixture and
RON-admitted CommitmentSpec binding are now landed (single grid, no atlas).
It is **not** wired into the default session pass graph and `MappingExecutionProfile`
default remains `Disabled`. Do not begin the M-4 atlas packer (Option 4): it waits for a
named multi-theater scenario, an approved VRAM budget, and a §11-gate-passing M-4
implementation PR.

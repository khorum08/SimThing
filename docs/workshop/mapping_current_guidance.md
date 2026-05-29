# Mapping Current Guidance

Authoritative decision:

- [`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md)

Constitutional surfacing:

- [`../design_v7_7.md`](../design_v7_7.md)
- [`../design_v7_6.md`](../design_v7_6.md)
- [`../invariants.md`](../invariants.md) — Mapping (Sparse RegionCell) rows

Active read order:

1. `docs/invariants.md`
2. `docs/adr/mapping_sparse_regioncell.md`
3. `docs/design_v7_6.md`
4. `docs/design_v7_7.md`
5. [`mapping_atlas_batching_isolation_design_note.md`](mapping_atlas_batching_isolation_design_note.md) (M-4 atlas contract — **provisional, unimplemented, parked**)
6. [`mapping_atlas_algebraic_mask_candidate_notes.md`](mapping_atlas_algebraic_mask_candidate_notes.md) (M-4A sandbox evidence — **candidate only, reverted**)
7. Cited `docs/tests/` evidence before changing any classification
8. [`../reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md`](../reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md) (boundary resolution + example economy — **accepted; binding guardrails in invariants**)
9. [`../reviews/phase_m_product_fixture_chain_acceptance_opus_review.md`](../reviews/phase_m_product_fixture_chain_acceptance_opus_review.md) (product-fixture chain — **ACCEPTED**; packet: [`phase_m_product_fixture_chain_review_packet.md`](../reviews/phase_m_product_fixture_chain_review_packet.md))
10. [`eml_gadget_library_design_note.md`](eml_gadget_library_design_note.md) (EML Gadget Library — **EML-GADGET-1 + R1 + R2 landed; parked for review**)
11. [`../reviews/phase_m_eml_gadget_tier1_review_packet.md`](../reviews/phase_m_eml_gadget_tier1_review_packet.md) (EML-GADGET-1 parking packet — **parked for Opus/product review**)

## Phase M boundary-resolution (tick / boundary / day) + example economy (ACCEPTED — Opus/product 2026-05-29, PASS WITH CONDITIONS)

**Accepted** ([`../reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md`](../reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md)): boundary doctrine accepted; Daily Economy Fixture V1 accepted as an example/product fixture only; the `ResourceEconomySpec` (discrete banking) vs Resource Flow E-11 (continuous, default-off) distinction accepted; the future-agent guardrails made **binding** in [`../invariants.md`](../invariants.md) ("Boundary resolution (tick / boundary / day)"). **Naming preference (product, 2026-05-29):** `tick` / `boundary` / `day` / `day_index` / `ticks_per_day` are the **preferred, endorsed names for their legibility** — the earlier R1/R2 pull toward abstract/illegible alternatives is reversed; keep the legible names. The guardrail is on *semantics* only: avoid Clausewitz/calendar semantics (calendar arithmetic / `Calendar`/`Pause` types / leap-date math / `DailyResolutionBoundary`), **not** the legible day-flavored naming that already pervades `simthing-sim`. **Phase M Resource Economy Authoring Ergonomics V1 landed** — spec/admission preview for discrete economy fixtures. **Phase M Economy + SEAD Product Fixture V1 landed** — test-level product fixture proving discrete economy boundary treasury stress can drive opt-in first-slice SEAD commitment through existing GPU EML/Threshold+EmitEvent (fixture orchestration only; no production economy→mapping bridge). **Phase M product-fixture chain ACCEPTED (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — [`../reviews/phase_m_product_fixture_chain_acceptance_opus_review.md`](../reviews/phase_m_product_fixture_chain_acceptance_opus_review.md). The economy→SEAD link stays in `tests/support` fixture orchestration: the CPU reads resolved treasury and *selects* authored EML weight profiles; it never computes urgency or emits the commitment (both stay GPU-resident). Binding row added to [`../invariants.md`](../invariants.md) Mapping section (economy→mapping influence is fixture-only; no production bridge without a separate gated decision). **Next implementation step (sequenced):** **Opus/product acceptance of EML-GADGET-1** first ([`../reviews/phase_m_eml_gadget_tier1_review_packet.md`](../reviews/phase_m_eml_gadget_tier1_review_packet.md)). Then EML-GADGET-2 design review, Authoring Ergonomics R2, or designer preview UX — do not implement EML-GADGET-2 or resume R2 until acceptance. **Phase M EML-GADGET-1 + R1 + R2 landed (2026-05-29)** — per-gadget executable templates; multi-gadget `PerGadgetOnly`; node cap per tree.

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

Review packet: [`../reviews/phase_m_first_slice_vertical_proof_review_packet.md`](../reviews/phase_m_first_slice_vertical_proof_review_packet.md)

Parking verification: [`../tests/phase_m_first_slice_vertical_proof_parking_test_results.md`](../tests/phase_m_first_slice_vertical_proof_parking_test_results.md)

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

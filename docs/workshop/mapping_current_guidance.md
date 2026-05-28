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
The **named next mapping step is a product-facing first-slice scenario fixture (single grid,
no atlas)** — not the atlas packer.

| Option | Path | Status |
|--------|------|--------|
| **A** | Implement the generic M-4 atlas packer | **Deferred** — admissible only after a named multi-theater scenario, an approved VRAM budget, and a §11-gate-passing PR. Not next. |
| **B** | First-slice runtime wiring (single grid, no atlas) | **Done** — landed, hardened (R1/R2/R3), accepted. |
| **Next** | Product-facing first-slice scenario fixture on the landed runtime | **Named next step** (Composer-class; no atlas/active-mask/perception/source_mask/new-WGSL/default-on). |

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

Do not implement production mapping runtime until first-slice session wiring is separately
gated after an explicit Option A or Option B decision.

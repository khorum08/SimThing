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

Single-grid edge-boundary parity confirms invalid neighbors are nullified by generic boundary semantics (`BoundaryMode::Zero`), not semantic map code. RegionField budget preview estimates designer-facing VRAM footprint and rejects over-budget specs at compile preview.

**Not** wired into default session pass graph. **No** atlas batching. **No** M-4A algebraic atlas masking. **No** active mask, perception, map residency, or behavioral source policy. M-4 remains parked at decision gate.

See [`../tests/phase_m_first_slice_runtime_test_results.md`](../tests/phase_m_first_slice_runtime_test_results.md).

## Parked status (Phase M-4)

Phase M-4 design note is **parked** pending human + Opus sign-off. Atlas batching remains provisional and unimplemented. The design note defines the **future contract only** — it is **not** implementation authorization:

- gutter >= effective horizon **or** algebraic tile-local mask for homogeneous square batches (M-4A evidence — pending sign-off)
- physical gutter remains fallback when algebraic masking is not configured
- mandatory VRAM accounting
- per-tile seed clearing (column-wide `source_col` zeroing banned)
- full-tile protocol-oracle parity required
- t44/corridor agreement alone is **insufficient** for production acceptance

**M-4A (2026-05-19):** Algebraic tile-local masking sandbox completed and was reverted to parked state. Candidate code/results are preserved under `docs/workshop/` and `docs/tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`. For homogeneous square atlas batches, G=0 algebraic tile-local masking is now the **preferred M-4 implementation candidate** over physical G>=H gutters, pending human + Opus sign-off. Physical gutter remains fallback. No atlas implementation landed. No mapping runtime landed.

For broader implications of M-4A algebraic masking, see the M-4 design note section **“Architectural Implications of Algebraic Tile-Local Masking.”**

## Current decision gate

Choose **one** explicitly — do not treat the design note as auto-authorization:

| Option | Path |
|--------|------|
| **A** | After human + Opus sign-off, implement the generic M-4 atlas packer |
| **B** | Defer atlas and proceed to first-slice runtime wiring, because the named first slice uses one grid and no atlas |

**Option B** (first-slice runtime wiring) is a **separate explicit gate** — not authorized by the M-4 design note alone.

## Landed Phase M natives

- **M-1 landed:** generic `StructuredFieldStencilOp::execute_configured` execution API and column-aware reduction convenience over existing `SlotRange` Sum
- **M-1.1 landed:** no-readback dispatch/report path for future schedulers; readback explicit for tests/diagnostics and readback-derived stats
- **M-2 landed:** generic cadence scheduler and dirty macro-region skip helper
- **M-2.1 landed:** FieldScheduler API hardening — region identity keyed by `(FieldId, FieldRegionId)`; visitor-based scheduled execution
- **M-first-slice landed (opt-in):** [`FirstSliceMappingSession`](../../crates/simthing-driver/src/first_slice_mapping_runtime.rs) — explicit `MappingExecutionProfile::SparseRegionFieldV1` only; not default session wiring
- **M-3 landed:** RegionFieldSpec RON + mapping admission framework — designer/spec structure only; compiles/previews to generic substrate configs; MappingExecutionProfile default Disabled
- **M-4 design note landed (parked):** [`mapping_atlas_batching_isolation_design_note.md`](mapping_atlas_batching_isolation_design_note.md)

**Deferred (M-3):** Perception field enum/class names remain admission-category placeholders only.

Historical sandbox source, candidate notes, revert reports, and full logs live under
[`archive/mapping/`](archive/mapping/) and [`../tests/archive/`](../tests/archive/).
They remain valid evidence but are not active guidance.

Do not implement production mapping runtime until first-slice session wiring is separately
gated after an explicit Option A or Option B decision.

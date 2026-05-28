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
5. [`mapping_atlas_batching_isolation_design_note.md`](mapping_atlas_batching_isolation_design_note.md) (M-4 atlas contract — provisional, unimplemented)
6. Cited `docs/tests/` evidence before changing any classification

Current next work:

- Phase M generic natives in [`../accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md)
- **M-1 landed:** generic `StructuredFieldStencilOp::execute_configured` execution API and column-aware reduction convenience over existing `SlotRange` Sum
- **M-1.1 landed:** no-readback dispatch/report path for future schedulers; readback explicit for tests/diagnostics and readback-derived stats
- **M-2 landed:** generic cadence scheduler and dirty macro-region skip helper
- **M-2.1 landed:** FieldScheduler API hardening — region identity keyed by `(FieldId, FieldRegionId)`; visitor-based scheduled execution
- **M-3 landed:** RegionFieldSpec RON + mapping admission framework — designer/spec structure only; compiles/previews to generic substrate configs; MappingExecutionProfile default Disabled
- **M-4 design note landed:** [`mapping_atlas_batching_isolation_design_note.md`](mapping_atlas_batching_isolation_design_note.md) — atlas batching isolation + VRAM accounting contract; atlas remains provisional and unimplemented
- Next step: **human + Opus sign-off** on M-4 design note, then either implement generic atlas packer or choose first-slice runtime wiring that avoids atlas entirely

**Deferred (M-3):** Perception field enum/class names (`true_field`, `observed_field`, `confidence_field`, `deception_field`) remain admission-category placeholders only; no perception runtime or authoritative-column write guards until a later phase.

**Atlas (M-4):** Short-term isolation `gutter >= effective_horizon`; local-bounds metadata deferred; full-tile protocol-oracle parity required for production acceptance (t44 alone insufficient); VRAM multiplier reporting mandatory.

Historical sandbox source, candidate notes, revert reports, and full logs live under
[`archive/mapping/`](archive/mapping/) and [`../tests/archive/`](../tests/archive/).
They remain valid evidence but are not active guidance.

Do not implement production mapping runtime until first-slice session wiring is separately
gated after Phase M natives land.

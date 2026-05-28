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
5. Cited `docs/tests/` evidence before changing any classification

Current next work:

- Phase M generic natives in [`../accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md)
- **M-1 landed:** generic `StructuredFieldStencilOp::execute_configured` execution API and column-aware reduction convenience over existing `SlotRange` Sum
- **M-1.1 landed:** no-readback dispatch/report path for future schedulers; readback explicit for tests/diagnostics and readback-derived stats
- **M-2 landed:** generic cadence scheduler and dirty macro-region skip helper
- **M-2.1 landed:** FieldScheduler API hardening — region identity keyed by `(FieldId, FieldRegionId)`; visitor-based scheduled execution
- **M-3 landed:** RegionFieldSpec RON + mapping admission framework — designer/spec structure only; compiles/previews to generic substrate configs; MappingExecutionProfile default Disabled
- Next implementation task: **Opus-gated M-4** atlas batching isolation + VRAM accounting design (provisional), unless first-slice runtime wiring is explicitly selected after M-3 review

Historical sandbox source, candidate notes, revert reports, and full logs live under
[`archive/mapping/`](archive/mapping/) and [`../tests/archive/`](../tests/archive/).
They remain valid evidence but are not active guidance.

Do not implement production mapping runtime until first-slice session wiring is separately
gated after Phase M natives land.

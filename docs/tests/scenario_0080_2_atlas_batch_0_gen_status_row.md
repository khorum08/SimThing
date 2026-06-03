# SCENARIO-0080-2 — ATLAS-BATCH-0-GEN Status Row

| Date | Rung | Status | Evidence | Next |
|---|---|---|---|---|
| 2026-06-03 | `ATLAS-BATCH-0-GEN` | IMPLEMENTED / TESTS ADDED — execution pending | `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs`; `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_gen.rs`; `docs/tests/scenario_0080_2_atlas_batch_0_gen_report.md` | `ATLAS-BATCH-0-LOC` — turn descriptor into Location gridcell SimThings once Opus authors the LOC contract. |

§0.5 posture: holds principles 1–6 for this rung. The change is pure fixture data, not a runtime subsystem; it does not implement conflict/resource-flow behavior, allocation, GPU threshold decisions, `simthing-sim` semantics, or default wiring. Later rungs must prove behavior through real SimThing reductions.

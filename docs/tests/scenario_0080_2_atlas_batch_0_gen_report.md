# SCENARIO-0080-2 — ATLAS-BATCH-0-GEN Test Report

**Date:** 2026-06-03  
**Rung:** `ATLAS-BATCH-0-GEN` — static map generator  
**Scope:** pure topology descriptor only. No GPU, no engine wiring, no SimThing instantiation, no economy, no arenas.

## Harness citations

- `docs/design_0_0_8_0.md` §0 — transient constitution and §0.5 harness discipline.
- `docs/invariants.md` — Scenario Proof, AccumulatorOp v2, Resource Flow Substrate.
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5 — rehearsal design, ATLAS-BATCH-0, OWNER routing, rung ladder.
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` — concrete 13-system dress-rehearsal map and §8.1 anticipated emergence.
- `crates/simthing-core/src/accumulator_op.rs` — GPU-resident Accumulator primitive vocabulary used by later arenas.
- `docs/workshop/sead_self_ai_track.md` — SEAD field-as-policy charter and no-CPU-planner posture.

## Implemented artifact

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs`
  - `DressRehearsalMap` descriptor.
  - Seeded constructor: `DressRehearsalMap::from_seed(seed)`.
  - Canonical constructor: `DressRehearsalMap::canonical()`.
  - Plain owner fields on systems, planets, surface occupants, starports, and fleets.
  - Deterministic coordinate transform chosen from the seed; no runtime behavior, no slot allocation, no `Location` materialization.

## Test artifact

- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_gen.rs`
  - Determinism: same seed produces identical descriptor.
  - Shape/counts: 20×20 galaxy; 13 systems; 10 Terran + 3 Pirate; 13 planets; 13 factories; 13 pop cohorts; 4 starports; 10 pirate ships + 3 patrol ships.
  - Bounds: every system grid is 10×10; every planet surface is 10×10; generated system, planet, factory, and pop cells are in bounds; galactic system cells are unique.
  - Placement: Terran systems have at least 2 empty galactic cells between every pair, and every Terran system participates in the 2–4 empty-cell local spacing band; every Pirate system is within 1 empty cell of a Terran system.
  - Starports/fleets: starports are at system center cell `(5,5)`; starting fleets are placed at owner starport systems.

## Command

```bash
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_gen
```

## Execution status

Not executed locally in this connector session: the GitHub connector allowed repository writes, but a sandbox `git clone` could not resolve `github.com`, so there was no local checkout to run Cargo against. The Rust test target above is the intended verification command for CI/local execution.

## Status row

| Rung | Status | Evidence | Notes |
|---|---|---|---|
| `ATLAS-BATCH-0-GEN` | IMPLEMENTED / TESTS ADDED — execution pending | `dress_rehearsal_atlas_batch_0_gen.rs`; integration test target above | Pure descriptor fixture only; no production wiring; LOC/PACK/STORE remain untouched. |

## §0.5 posture line

Holds §0.5 principles 1–6 for this rung: the change is a data-only descriptor with no subsystem runtime, no resource-flow implementation, no allocation-depth claim, no GPU/CPU planner decision logic, no `simthing-sim` semantics, and no default wiring; later rungs must prove behavior through real SimThing reductions while GEN stays opt-in fixture data only.

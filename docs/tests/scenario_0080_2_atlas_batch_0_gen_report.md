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

## Remedial (2026-06-03)

Corrected `TERRAN_BASE_CELLS` / `PIRATE_BASE_CELLS` so every Terran system has an in-band neighbor at 2–4 empty cells (Chebyshev 3..=5). The prior layout left y=8 and y=14 rows ~6 cells apart (5 empty cells between), failing `terran_spacing_and_pirate_adjacency_hold`.

## Test artifact

- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_gen.rs`
  - Gate id/status consts.
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

```text
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_gen
Result: 6 passed; 0 failed
```

(Includes remedial spacing-band fix and `docs_status_matches_gate`.)

Pre-existing workspace warnings only (`simthing-core` EML deprecations; unrelated `simthing-driver` soak import). No dead-code warnings from this rung after hygiene.

## Status row

| Rung | Status | Evidence | Notes |
|---|---|---|---|
| `ATLAS-BATCH-0-GEN` | IMPLEMENTED / PASS | `dress_rehearsal_atlas_batch_0_gen.rs`; test target above | Pure descriptor fixture only; no production wiring; LOC/PACK/STORE remain untouched. |

## §0.5 posture line

Holds §0.5 principles 1–6 for this rung: the change is a data-only descriptor with no subsystem runtime, no resource-flow implementation, no allocation-depth claim, no GPU/CPU planner decision logic, no `simthing-sim` semantics, and no default wiring; later rungs must prove behavior through real SimThing reductions while GEN stays opt-in fixture data only.

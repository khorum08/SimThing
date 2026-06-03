# SCENARIO-0080-2 — ATLAS-BATCH-0-LOC Test Report

**Date:** 2026-06-03  
**Rung:** `ATLAS-BATCH-0-LOC` — Location gridcell layout materialization  
**Scope:** fixture-only descriptor layer from green GEN; no `SimThing` instantiation, no GPU slots, no numeric columns, no production wiring.

## Harness citations

- `docs/design_0_0_8_0.md` §0 — transient constitution and §0.5 harness discipline.
- `docs/invariants.md` — Scenario Proof; single indexing home; semantic-free `simthing-sim`.
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5 — ATLAS-BATCH-0 ladder, GEN closure, LOC gate.
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` — 20×20 / 13-system topology.
- `crates/simthing-core/src/accumulator_op.rs` — Accumulator vocabulary for later rungs (not edited).
- `docs/workshop/sead_self_ai_track.md` — SEAD charter (not edited).
- `docs/handoffs/dress_rehearsal_codex_handoff_2_atlas_batch_0_loc.md` — Opus LOC contract.
- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs` — consumed GEN descriptor.
- `docs/tests/scenario_0080_2_atlas_batch_0_gen_report.md` — green GEN evidence.

## Implemented artifact

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs`
  - `LocationMaterialization::from_map` / `canonical()` from `DressRehearsalMap`.
  - **27 Locations:** 1 galactic (20×20) + 13 star systems (10×10) + 13 planet surfaces (10×10).
  - **56 occupants:** 13 planets, 4 starports, 13 factories, 13 pop cohorts, 3 patrol + 10 pirate fleets (galactic tier).
  - Single `cell_index(map_base, width, x, y)` indexing home; contiguous `map_base` ranges → `total_cell_slots = 3000`.
  - Typed `ChannelSet` / `ChannelDescriptor` only (no numeric columns).
  - Test-only `#[path]` inclusion; **not** exported from `lib.rs`.

## Test artifact

- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_loc.rs` — 9 tests (determinism, bounds, indexing, slot partition, co-location, owners, channels, planet↔surface links, gate status).

## Command

```bash
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_loc
```

## Execution status

**Result:** `9 passed; 0 failed`

**Raw evidence:** [`scenario_0080_2_atlas_batch_0_loc_cargo_test_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_loc_cargo_test_2026_06_03.txt)

**Warnings (pre-existing / test-binary GEN re-export):** `simthing-core` EML deprecations; `simthing-driver` soak import; unused GEN helpers when GEN is compiled only as LOC's private submodule in the LOC test binary — unrelated to LOC correctness.

## Status row

| Rung | Status | Evidence | Notes |
|---|---|---|---|
| `ATLAS-BATCH-0-LOC` | IMPLEMENTED / PASS | `dress_rehearsal_atlas_batch_0_loc.rs`; test target above; raw log | PACK/STORE unimplemented; M-4A / REENROLL parked. |

## §0.5 posture line

Holds §0.5 principles 1–6 for this rung: fixture-only structural descriptor materialization from accepted GEN; no subsystem runtime, no resource-flow implementation, no allocation-depth claim beyond deterministic cell-range layout, no GPU/CPU planner decision logic, no `simthing-sim` semantics, no default wiring.

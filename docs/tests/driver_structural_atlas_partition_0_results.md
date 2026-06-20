# DRIVER-STRUCTURAL-ATLAS-PARTITION-0 — driver structural atlas partition/admission

> **Lifecycle: PROBATION** — driver partition/admission landed; partition tests and scheduler integration pass; cross-partition N4 halo explicitly deferred. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** DRIVER-STRUCTURAL-ATLAS-PARTITION-0  
**Base:** `master` after PR #773 / SIM-MAPPING-ATLAS-SCHEDULER-0

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |
| `docs/tests/sim_mapping_atlas_scheduler_0_results.md` | PROBATION (PR #773) | Retained |
| `docs/tests/structural_n4_theater_compile_0_results.md` | PROBATION (PR #769) | Retained |
| `docs/tests/driver_structural_atlas_partition_0_results.md` | PROBATION | Created (this file) |
| `docs/design_0_0_8_3_studio_production.md` | Living synthesis | Updated |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority | Untouched |

## Why this is the next rung

PR #773 proved sim can schedule multiple compiled plans. PR #769 typed `AtlasDeferred` for oversize frames but did not partition them. This PR adds the driver-owned bridge: oversized structural authority → bounded partition theaters → generic mapping plans → sim scheduler.

## Orientation answers

| Question | Answer |
|---|---|
| What returns AtlasDeferred today? | `compile_structural_n4_theater` when frame exceeds `REGION_FIELD_STANDARD_MAX_GRID` or cell cap |
| New driver API? | `compile_structural_n4_atlas` → `Single` / `Partitioned` / `Deferred` |
| Scenario authority read in driver only? | **Yes** |
| Original frame preserved? | **Yes** — `CompiledStructuralN4Atlas.original_frame_width/height` |
| Partition without shrinking? | **Yes** — scenario `structural_grid.frame` unchanged |
| Local vs global coords? | `StructuralTheaterOrigin` + local theater coords; global recoverable as `origin + local` |
| Cross-partition N4 edges? | **Explicitly deferred** via `DeferredCrossPartitionN4Edge` |
| sim upward deps clean? | **Yes** |
| Sim scheduler gets compiled plans only? | **Yes** |
| Forbidden semantics? | **None** |
| Scenario mutated? | **No** |

## Driver partition/admission API

**Module:** `crates/simthing-driver/src/structural_n4_atlas_partition.rs`

- `StructuralAtlasPartitionProfile` — max theater width/height (default 10), halo flag (false)
- `CompiledStructuralN4Atlas` — original frame + partition theaters + deferred edges
- `PartitionedStructuralN4Theater` — local theater + origin + partition index
- `compile_structural_n4_atlas(scenario, profile, partition_profile)`

## Proofs

| Proof | Status |
|---|---|
| Terran Pirate 8×8 single theater | PASS |
| 11×11 synthetic partitioned (≥2 tiles) | PASS |
| Original frame metadata preserved | PASS |
| Per-theater max grid cap respected | PASS |
| Scenario authority not mutated | PASS |
| Cross-partition N4 edge deferred | PASS |
| Partition → mapping plan → atlas scheduler integration | PASS — REAL_ADAPTER_OBSERVED |
| None after ProofReadback | PASS |
| Forbidden-token guard | PASS |

## Cross-partition edge behavior

First-slice: cross-partition N4 adjacency recorded in `deferred_cross_partition_edges`. No halo exchange, no dense-global fallback, no route/border semantics. Halo overlap (`include_overlap_halo`) returns error — deferred to future PR.

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test structural_n4_atlas_partition` | PASS (7/7) |
| `cargo test -p simthing-driver --test structural_n4_atlas_scheduler_integration` | PASS (1/1) |
| `cargo test -p simthing-driver --test terran_pirate_mapping_atlas_scheduler` | PASS |
| `cargo test -p simthing-driver --test mapping_plan_compile` | PASS |
| `cargo test -p simthing-driver --test structural_n4_theater_compile` | PASS |
| `cargo test -p simthing-driver terran_pirate` | PASS |
| `cargo check -p simthing-sim` | PASS |
| `cargo test -p simthing-sim --test mapping_atlas_scheduler` | PASS |
| `cargo test -p simthing-sim --test mapping_plan_tick` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS |
| `git diff --check` | PASS |

## Deferred work

- Cross-partition N4 halo exchange runtime
- Big-endian / portable byte-proof backlog
- DA promotion (requires owner sign-off)

## DA status

**PROBATION** — no DA approval claimed.
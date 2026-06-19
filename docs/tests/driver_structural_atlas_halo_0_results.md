# DRIVER-STRUCTURAL-ATLAS-HALO-0 — one-cell structural halo admission

> **Lifecycle: PROBATION** — one-cell structural N4 halo admission landed; halo tests and scheduler integration pass; full validation pass. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** DRIVER-STRUCTURAL-ATLAS-HALO-0  
**Base:** `master` after PR #774 / DRIVER-STRUCTURAL-ATLAS-PARTITION-0

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |
| `docs/tests/driver_structural_atlas_partition_0_results.md` | PROBATION (PR #774) | Retained |
| `docs/tests/sim_mapping_atlas_scheduler_0_results.md` | PROBATION (PR #773) | Retained |
| `docs/tests/driver_structural_atlas_halo_0_results.md` | PROBATION | Created (this file) |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority | Untouched |

## Why this is the next rung

PR #774 recorded cross-partition N4 edges in `deferred_cross_partition_edges` instead of silently dropping them. Partitioned execution remained first-slice limited at theater boundaries. This PR admits one-cell structural N4 halo metadata so adjacent cells across partition boundaries appear in bounded theater inputs without route/border semantics or dense-global fallback.

## Orientation answers

| Question | Answer |
|---|---|
| What cross-partition edge behavior was deferred? | With `include_overlap_halo: false`, edges recorded in `deferred_cross_partition_edges` only (PR #774 behavior) |
| New halo admission API? | `StructuralAtlasPartitionProfile.include_overlap_halo: true` + `StructuralTheaterHaloCell` / `StructuralTheaterCellRole` on `PartitionedStructuralN4Theater` |
| Halo uses structural N4 adjacency only? | **Yes** — one-cell N4 neighbors from adjacent partitions |
| Original frame metadata preserved? | **Yes** — `CompiledStructuralN4Atlas.original_frame_width/height` unchanged |
| Halo cells mutate scenario authority? | **No** — metadata/input projection only |
| Owned vs halo distinguished? | **Yes** — `halo_cells` metadata + `cell_role()`; halo cells excluded from `system_placements` |
| Global coordinates recoverable? | **Yes** — `global_from_local()` + `StructuralTheaterHaloCell.global_coord` |
| Sim scheduler unchanged? | **Yes** — accepts already-compiled generic plans only |
| Sim imports scenario/spec/driver? | **No** |
| Route/path/border semantics? | **None** |
| New GPU primitive/shader? | **No** |

## Current deferred cross-partition behavior

- **Halo disabled:** `deferred_cross_partition_edges` populated; `halo_coverage` empty; partition theaters have empty `halo_cells`.
- **Halo enabled:** deferred edge records retained as provenance; `halo_coverage` records per-edge halo admission; halo cells admitted in adjacent theaters.

## Driver halo admission API

**Module:** `crates/simthing-driver/src/structural_n4_atlas_partition.rs`

- `StructuralTheaterCellRole` — `Owned` | `Halo`
- `StructuralTheaterHaloCell` — `global_coord`, `local_coord`, `source_partition_index`
- `StructuralTheaterCoordPadding` — west/north padding for non-negative GPU locals
- `CrossPartitionHaloCoverage` — provenance over retained deferred edges
- `PartitionedStructuralN4Theater.halo_cells` + `coord_padding`
- `CompiledStructuralN4Atlas.halo_coverage`
- `StructuralTheaterCompileError::HaloExceedsTheaterCap`
- When `include_overlap_halo: true`, owned tile span reserves one cell per axis for east/south halo within cap

## Proofs

| Proof | Status |
|---|---|
| Halo disabled preserves PR #774 deferral | PASS |
| Halo enabled adds one-cell structural N4 halo | PASS |
| Owned vs halo distinguished | PASS |
| Original frame metadata preserved | PASS |
| Scenario authority not mutated | PASS |
| Cap deferral when halo exceeds theater cap | PASS |
| Global/local coordinate recovery | PASS |
| Halo → mapping plan → atlas scheduler integration | PASS — REAL_ADAPTER_OBSERVED |
| None after ProofReadback | PASS |
| Forbidden-token guard | PASS |
| PR #774 partition tests regression | PASS |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test structural_n4_atlas_halo` | PASS (8/8) |
| `cargo test -p simthing-driver --test structural_n4_atlas_halo_scheduler_integration` | PASS (1/1) |
| `cargo test -p simthing-driver --test structural_n4_atlas_partition` | PASS |
| `cargo test -p simthing-driver --test structural_n4_atlas_scheduler_integration` | PASS |
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

- Big-endian / portable byte-proof backlog
- DA promotion (requires owner sign-off)

## DA status

**PROBATION** — no DA approval claimed.
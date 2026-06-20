# RECURSIVE-SPATIAL-GRID-DEFAULTS-0 — universal 1×1 interior-grid defaults

> **Lifecycle: PROBATION** — recursive spatial doctrine finalized. Pending owner DA approval.

**Date:** 2026-06-19
**PR:** #791 — RECURSIVE-SPATIAL-GRID-DEFAULTS-0
**Merge:** `1807faa9`
**Base:** `master` after PR #790 / PLANET-LOCAL-GRID-REMEDIATION-0 (`18123e55`)

## Defects remediated

1. Universal **1×1** interior-grid default for spatial gridcells (star-system **10×10** exception).
2. Inert galactic gridcells admit implicit **1×1 receiver** local cells at (0,0).
3. Production doc stale “planets deferred” language corrected.
4. Owner/RF channel scope documented as metadata, not spatial parentage.
5. PR #790 evidence metadata confirmed (#790 / `18123e55`).

## Recursive spatial model

- Every spatial gridcell `Location` has an interior child grid; default **1×1**.
- Inert gridcells: **1×1 receiver** for falloff/heatmap/RF-readiness (implicit or materialized).
- Star-system gridcells: **10×10** local grid for planet local gridcells.
- Planet gridcells: local col/row under star-system; interior default **1×1**.
- Non-grid children (cohort, fleet, infrastructure, leader) allowed under planet gridcells.

## Owner / RF channel doctrine

Owners are GameSession children and RF channel scopes, not spatial parents. Ownership changes update metadata/properties/columns, not spatial parentage. RF resolves locally first; surplus/deficit reduces upward.

## GPU boundary

| Check | Status |
|-------|--------|
| GalaxyMap structural readiness | PASS |
| Inert receiver-cell local-grid admission | PASS |
| Star-system local-grid admission | PASS |
| Planet local-grid admission | PASS |
| Star-system local-grid GPU operator | DEFERRED |
| No new GPU/WGSL | PASS |

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (22/22) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo test -p simthing-mapeditor --test studio_planet_child_location_display` | PASS (11/11) |
| `cargo test -p simthing-driver --test planet_child_location_structural_readiness` | PASS (6/6) |
| `git diff --check` | PASS |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12/12) |
| Post-merge validation on `master` (`a270fe366`) | PASS |

## Files changed

- `crates/simthing-spec/src/spec/spatial_local_grid.rs` (new)
- `crates/simthing-spec/src/spec/planet_child_location.rs`
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/planet_child_location_admission.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-mapeditor/src/studio_scenario_document.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_planet_child_location_display.rs`
- `crates/simthing-driver/tests/planet_child_location_structural_readiness.rs`
- `scenarios/corpus/inert_gridcell_receiver_1x1_admitted.simthing-scenario.json` (new)
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/planet_local_grid_remediation_0_results.md`
- `docs/tests/recursive_spatial_grid_defaults_0_results.md`

## DA status

**PROBATION** — not DA-promoted.
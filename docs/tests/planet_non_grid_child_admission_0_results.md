# PLANET-NON-GRID-CHILD-ADMISSION-0 — cohort/fleet/infrastructure under planet gridcells

> **Lifecycle: PROBATION** — non-grid child admission landed. Simulation behavior remains deferred. Pending owner DA approval.

**Date:** 2026-06-19
**PR:** #792 — PLANET-NON-GRID-CHILD-ADMISSION-0
**Merge:** `6790907b`
**Base:** `master` after PR #791 / RECURSIVE-SPATIAL-GRID-DEFAULTS-0 (`1807faa9`)

## What changed

- Planet gridcells may own admitted non-grid children: `Cohort`, `Fleet`, `Station`, `Custom(Infrastructure|Leader)`.
- Non-grid children do not require local col/row; local coordinate metadata on non-grid children rejects.
- Non-grid children are not GalaxyMap `structural_grid` placements.
- Non-grid children may carry owner/channel metadata without spatial reparenting.
- `evaluate_planet_child_locations` reports `planet_non_grid_child_count` and typed deferrals (`PlanetNonGridChildSimulationDeferred`, `PlanetNonGridChildUnsupportedKind`).
- Studio projects `planet_non_grid_children` on `StudioScenarioDocument`.
- Driver structural N4 remains gridcell-only; no GPU/sim runtime changes.

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (25/25) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo test -p simthing-mapeditor --test studio_planet_child_location_display` | PASS (12/12) |

## GPU boundary

| Check | Status |
|-------|--------|
| GalaxyMap structural readiness | PASS |
| Planet non-grid children excluded from structural_grid | PASS |
| No new GPU/WGSL | PASS |

## DA status

**PROBATION** — not DA-promoted.
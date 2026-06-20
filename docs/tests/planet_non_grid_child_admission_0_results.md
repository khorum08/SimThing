# PLANET-NON-GRID-CHILD-ADMISSION-0 — cohort/fleet/infrastructure under planet gridcells

> **Lifecycle: PROBATION** — non-grid child admission landed. Simulation behavior remains deferred. Pending owner DA approval.

**Date:** 2026-06-19
**PR:** #792 — PLANET-NON-GRID-CHILD-ADMISSION-0
**Merge:** `6790907b`
**Evidence metadata commit:** `9ac4d9e16`
**Base:** `master` after PR #791 / RECURSIVE-SPATIAL-GRID-DEFAULTS-0 (`1807faa9`)

## PR summary

Formalize non-grid child admission under planet local gridcells for `Cohort`, `Fleet`, `Station`, and `Custom(Infrastructure|Leader)` nodes. Reject local col/row on non-grid children, report `planet_non_grid_child_count` in ingestion, project Studio `planet_non_grid_children` views, and extend the admitted planet corpus fixture. Builds on RECURSIVE-SPATIAL-GRID-DEFAULTS-0 (#791). No simulation engine, GPU, WGSL, or sim runtime changes.

## Process foreground status

| Check | Status |
|-------|--------|
| Agent background shells | **NONE** — all prior harness terminals completed or timed out |
| `cargo` / `rustc` / `gh` | **NONE** running |
| Orphaned `git` helpers | **NONE** at report time |
| Active work | Post-merge validation on `master` (`9ac4d9e16`) |

## Defect remediated

PR #791 allowed non-grid children under planet gridcells in tests but did not formalize admission, counting, typed deferrals, ingestion propagation, or Studio projection.

## Admission model

- Planet gridcells may own admitted non-grid children: `Cohort`, `Fleet`, `Station`, `Custom(Infrastructure|Leader)`.
- Non-grid children do **not** require local col/row; local coordinate metadata on non-grid children **rejects** (`PlanetNonGridChildHasLocalCoordinate`).
- Non-grid children are **not** GalaxyMap `structural_grid` placements.
- Non-grid children may carry owner/channel metadata; ownership does not require spatial reparenting.
- Admitted non-grid children receive typed deferral `PlanetNonGridChildSimulationDeferred`; unsupported kinds defer as `PlanetNonGridChildUnsupportedKind`.
- `evaluate_planet_child_locations` reports `planet_non_grid_child_count`.
- Studio projects `planet_non_grid_children` on `StudioScenarioDocument`.
- Driver structural N4 remains gridcell-only.

## Owner / RF channel doctrine (preserved)

Owners are GameSession children and RF channel scopes, not spatial parents. Non-grid children may carry `owner_ref` metadata; resolution remains deferred.

## GPU boundary

| Check | Status |
|-------|--------|
| GalaxyMap structural readiness | PASS (by design — no structural_grid pollution) |
| Planet non-grid children excluded from structural_grid | PASS (by design) |
| Star-system local-grid GPU operator | DEFERRED |
| No new GPU/WGSL | PASS (by design) |

## Files changed

- `crates/simthing-spec/src/spec/planet_child_location.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/planet_child_location_admission.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-mapeditor/src/studio_scenario_document.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_planet_child_location_display.rs`
- `scenarios/corpus/planet_child_location_admitted.simthing-scenario.json`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/planet_non_grid_child_admission_0_results.md`

## Validation commands

> Report authored before validation; statuses recorded by post-merge run on `master` (`9ac4d9e16`).

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (25/25) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12/12) |
| `cargo test -p simthing-mapeditor --test studio_planet_child_location_display` | PASS (12/12) |
| `cargo test -p simthing-driver --test planet_child_location_structural_readiness` | PASS (6/6) |
| `git diff --check` | PASS |
| Post-merge validation on `master` (`9ac4d9e16`) | PASS |

## DA status

**PROBATION** — not DA-promoted.
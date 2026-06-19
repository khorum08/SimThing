# PLANET-LOCAL-GRID-REMEDIATION-0 — restore recursive planet gridcell semantics

> **Lifecycle: PROBATION** — ontology remediation landed. Pending owner DA approval.

**Date:** 2026-06-19
**PR:** #790 — PLANET-LOCAL-GRID-REMEDIATION-0
**Merge:** `18123e55`
**Base:** `master` after PR #789 / PLANET-CHILD-LOCATION-ADMISSION-0 (`617a8c5d`)

## Ontology drift summary (PR #789)

PR #789 admitted planets as `Location` children under star-system gridcells but drifted toward treating them as non-grid child objects without local `col`/`row` placement inside a star-system local grid frame.

## Corrected recursive SimThing gridcell model

- Every spatial container is a SimThing; spatial SimThings arrange child gridcell SimThings.
- GalaxyMap arranges **galactic** gridcell Location SimThings via `structural_grid`.
- Star-system galactic gridcells arrange **local** gridcell Location SimThings in a star-system local grid.
- Planet = star-system-local gridcell Location SimThing (`LOCAL_GRIDCELL_ROLE_PLANET` + `planet_id` + local coordinates).
- Planet gridcells are **not** GalaxyMap `structural_grid` placements.

## Star-system local 10×10 grid model

- Default frame: `STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS/ROWS` = 10×10.
- Optional override via `STAR_SYSTEM_LOCAL_GRID_FRAME_COLS/ROWS` on star-system gridcell.
- Local child gridcells carry `LOCAL_GRIDCELL_COL/ROW` and `LOCAL_GRIDCELL_ROLE` (`inert` / `planet`).

## Planet local gridcell representation

- `make_planet_gridcell(planet_id, col, row, display_name)`
- `PlanetLocalGridCommand` for add/move/remove/set-display-name on local gridcells
- Deprecated aliases retained: `PlanetChildLocationCommand`, `make_planet_child_location`, `GALAXY_CHILD_LOCATION_ROLE_PLANET`

## Validation/admission behavior

`evaluate_planet_child_locations` / fail-closed `validate_planet_child_locations`:

- Admits planet local gridcells under star-system galactic gridcells with valid local coordinates inside frame
- Rejects: inert-parent, missing coordinates, out-of-frame, duplicate local coordinate, missing/duplicate planet_id, GalaxyMap structural_grid listing
- Typed deferrals: `PlanetSimulationDeferred`, `UnsupportedChildLocationRole`, `DeepPlanetChildDeferred`, `PlanetOwnershipResolutionDeferred`

Report fields: `local_gridcell_count`, `local_inert_gridcell_count`, `planet_gridcell_count`.

## Ingestion integration behavior

- Valid planet local-grid cells admitted with `PlanetSimulationDeferred` (not blanket `PlanetsNotYetAdmitted`)
- Invalid placement rejects ingestion
- `ScenarioIngestionResult.planet_child_location` exposes local/planet gridcell counts

## Studio display/edit behavior

- `StudioPlanetChildView`: parent star-system id, local frame 10×10, local col/row, local role, planet_id, display name, owner ref, admission/deferrals
- `studio_apply_planet_child_location_command` uses `PlanetLocalGridCommand`; draft-then-swap; no GPU dispatch; no sim tick

## Driver structural readiness behavior

- GalaxyMap structural N4 admission remains gridcell-only (2 galactic placements in admitted fixture)
- Planet local gridcells excluded from `structural_grid.placements`
- Invalid under-inert rejected at ingestion

## Corpus fixture lifecycle cleanup

- Durable fixtures normalized to local-grid semantics (static JSON; no test writers)
- Removed `write_planet_child_location_corpus_fixtures` test
- Added `normal_tests_do_not_write_corpus_fixtures` guard

## GPU boundary status

- Star-system local grid GPU operator: **deferred**
- GalaxyMap structural readiness: **PASS** (unchanged)
- No new WGSL/shader/primitive

## Production synthesis update summary

- Added § PLANET-LOCAL-GRID-REMEDIATION-0 to `docs/0.8.3 Simthing Studio Production.md`
- Marked PR #789 ontology as remediated (retained PROBATION prerequisite)

## Evidence lifecycle cleanup summary

- `#789` row corrected to PR #789 / merge `617a8c5d`; noted ontology remediation requirement
- `planet_child_location_admission_0_results.md` updated with PR/merge + remediation note
- Added PLANET-LOCAL-GRID-REMEDIATION-0 PROBATION row
- No DA promotion

## Specified vs implemented ledger

| Requirement | Status |
|-------------|--------|
| Planet = star-system-local gridcell | PASS |
| Default 10×10 local frame | PASS |
| Local col/row required | PASS |
| Fail-closed validation | PASS |
| No corpus test writers | PASS |
| Studio local-grid display | PASS |
| Driver N4 gridcell-only | PASS |
| No GPU/WGSL/sim changes | PASS |

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS (pre-existing warnings only) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (16/16) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12/12) |
| `cargo test -p simthing-spec --test scenario_galaxymap_worldstate` | PASS (12/12) |
| `cargo test -p simthing-spec --test structural_placement_edit_commands` | PASS (10/10) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test planet_child_location_structural_readiness` | PASS (4/4) |
| `cargo test -p simthing-driver --test structural_edit_compile_readiness` | SKIP (not re-run; no driver source changes) |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test studio_planet_child_location_display` | PASS (9/9) |
| `cargo test -p simthing-mapeditor --test studio_structural_placement_edit_commands` | PASS (9/9) |
| `cargo test -p simthing-mapeditor --test studio_ingestion_admission_report` | PASS (12/12) |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS (10/10) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/planet_child_location.rs`
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/planet_child_location_admission.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-mapeditor/src/studio_scenario_document.rs`
- `crates/simthing-mapeditor/src/studio_planet_child_location.rs`
- `crates/simthing-mapeditor/tests/studio_planet_child_location_display.rs`
- `crates/simthing-mapeditor/tests/studio_ingestion_admission_report.rs` (deferral kind alignment)
- `crates/simthing-driver/tests/planet_child_location_structural_readiness.rs`
- `scenarios/corpus/planet_child_location_*.simthing-scenario.json` (4 fixtures)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/planet_child_location_admission_0_results.md`
- `docs/tests/planet_local_grid_remediation_0_results.md` (this file)

## Deleted/archived artifacts

- Removed `write_planet_child_location_corpus_fixtures` test (fixture writer)
- No live ledger or DA ruling files deleted

## Deferred next rung recommendation

- Star-system local grid GPU structural operator (if needed beyond scenario authority)
- Planet simulation/economy/population/cohort execution under planet local gridcells
- Planet ownership resolution and deeper Location nesting admission

## DA status

**PROBATION** — not DA-promoted; owner approval required for promotion.
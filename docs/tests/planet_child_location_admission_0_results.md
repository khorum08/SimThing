# PLANET-CHILD-LOCATION-ADMISSION-0 — planets as child Locations under star-system gridcells

> **Lifecycle: PROBATION** — planet child-location admission and Studio display landed. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #789 — PLANET-CHILD-LOCATION-ADMISSION-0
**Merge:** `617a8c5d`
**Base:** `master` after PR #788 / STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 (`df681533`)

> **Ontology note:** PR #789 is retained as **PROBATION** prerequisite evidence but its planet-as-free-child-object model was remediated by **PLANET-LOCAL-GRID-REMEDIATION-0**. See `docs/tests/planet_local_grid_remediation_0_results.md`. Not DA-promoted.

## Current defect / ontology gap summary

Structural placement edit commands could mutate gridcells, but planets/child Locations under star-system gridcells had no typed admission, validation, ingestion reporting, or Studio display. Ingestion emitted blanket `PlanetsNotYetAdmitted` deferrals for any nested Location child.

## Planet child-location representation model

- Planet = `SimThingKind::Location` + `GALAXY_CHILD_LOCATION_ROLE_PLANET` metadata
- Required: `PLANET_ID_PROPERTY_ID`
- Optional: `PLANET_DISPLAY_NAME_PROPERTY_ID`, `PLANET_OWNER_REF_PROPERTY_ID`, etc.
- Planet is a child of star-system gridcell Location; not a `structural_grid` placement

## Validation/admission model

`evaluate_planet_child_locations` / `validate_planet_child_locations` in `planet_child_location.rs`:

- Admits planets under star-system gridcells
- Rejects planets under inert gridcells, missing/duplicate planet ids, planets listed in structural_grid
- Typed deferrals: `PlanetSimulationDeferred`, `UnsupportedChildLocationRole`, `DeepChildLocationDeferred`, `PlanetOwnershipResolutionDeferred`

## Ingestion integration status

**PASS** — `ScenarioIngestionResult.planet_child_location` populated; blanket `PlanetsNotYetAdmitted` replaced for valid planet nodes; legacy Custom Planet children get `UnsupportedChildLocationRole`.

## Studio display/edit status

**PASS** — `StudioPlanetChildView` on `StudioScenarioDocument.planets`; `studio_apply_planet_child_location_command` for Add/Remove/SetDisplayName; rebuilds document/projection/admission.

## Driver structural readiness proof

**PASS** — `planet_child_scenario_reaches_structural_n4_admission`, `planet_child_not_counted_as_structural_gridcell`, `planet_child_locations_do_not_expand_structural_grid_placements`; invalid under-inert rejected at ingestion before compile path.

## GPU boundary status

**PASS** — unchanged from #786/#788; structural N4 remains gridcell-only; no new WGSL/shader; Studio planet code does not dispatch GPU or call sim tick.

## Fixture list

- `scenarios/corpus/planet_child_location_admitted.simthing-scenario.json`
- `scenarios/corpus/planet_child_location_under_inert_rejected.simthing-scenario.json`
- `scenarios/corpus/planet_child_location_duplicate_id_rejected.simthing-scenario.json`
- `scenarios/corpus/planet_child_location_unsupported_child_deferred.simthing-scenario.json`

## Production synthesis update summary

Added § PLANET-CHILD-LOCATION-ADMISSION-0; reprioritized Next Production Rungs.

## Evidence lifecycle cleanup summary

**PASS** — PR #788 evidence-index row corrected to `#788` / `df681533`; `structural_placement_edit_commands_0_results.md` merge metadata updated; live ledger preserved; no DA promotion.

## Specified-vs-implemented ledger

| Requirement | Status |
|---|---|
| Planet admission API in spec | PASS |
| Planets under star-system gridcells | PASS |
| Inert gridcell rejection | PASS |
| Missing/duplicate planet id rejection | PASS |
| Unsupported/deeper child typed deferrals | PASS |
| Planets not in structural_grid | PASS |
| Ingestion planet report integration | PASS |
| Studio planet display | PASS |
| Planet edit commands + Studio wrapper | PASS |
| Driver structural N4 gridcell-only | PASS |
| No GPU/sim tick in Studio | PASS |
| #788 evidence metadata cleanup | PASS |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (11/11) |
| `cargo test -p simthing-spec --test structural_placement_edit_commands` | PASS |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS |
| `cargo test -p simthing-spec --test scenario_galaxymap_worldstate` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test planet_child_location_structural_readiness` | PASS (4/4) |
| `cargo test -p simthing-driver --test structural_edit_compile_readiness` | PASS |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test studio_planet_child_location_display` | PASS (7/7) |
| `cargo test -p simthing-mapeditor --test studio_structural_placement_edit_commands` | PASS |
| `cargo test -p simthing-mapeditor --test studio_ingestion_admission_report` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/planet_child_location.rs` (new)
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/planet_child_location_admission.rs` (new)
- `crates/simthing-spec/tests/scenario_ingestion_admission.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-mapeditor/src/studio_planet_child_location.rs` (new)
- `crates/simthing-mapeditor/src/studio_scenario_document.rs`
- `crates/simthing-mapeditor/src/studio_structural_edit.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_planet_child_location_display.rs` (new)
- `crates/simthing-driver/tests/planet_child_location_structural_readiness.rs` (new)
- `scenarios/corpus/planet_child_location_*.simthing-scenario.json` (4 files)
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/structural_placement_edit_commands_0_results.md`
- `docs/tests/planet_child_location_admission_0_results.md` (new)

## Deleted/archived artifacts

None.

## Deferred next rung recommendation

1. Full owner-silo state mutation if still deferred.
2. Broader scenario corpus ingestion UX / batch reports.
3. UI affordances for planet/structural editing if command layers remain headless.
4. Cohort/population/resource overlays under admitted planets.

## DA status

**PROBATION** — pending owner DA approval. No DA promotion in this PR.
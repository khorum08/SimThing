# SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `scenario-planet-surface-gridcell-tier-0`
- PR: #851
- Merge SHA: `8c651e0eb5e082f5bc743590247219fee491978d`

## Mission

Remediate the FINAL-DA-REVIEW-0 HOLD blocker: restore the constitutionally required 1×1 surface gridcell tier below planet gridcells, re-home gameplay SimThings onto the surface gridcell, and move recursive Accumulator Flow settlement to the surface arena before upward bubbling.

## Opus/Fable HOLD baseline

`docs/tests/scenario_runtime_saveload_final_da_review_0_results.md` ruled HOLD because gameplay SimThings were homed directly on planet gridcells and the surface Location tier was deferred as `DeepPlanetChildDeferred`.

## Constitution / ADR alignment

Restores sparse spatial / bounded-theater doctrine: planet interior 1×1 surface gridcell Location with role `surface`; gameplay children under surface; RF settles at surface arena first. ScenarioSpec remains authority. Owner identity remains metadata/channel scope. CPU work remains oracle/reference/serialization/reporting only. GPU-compatible row/table runtime shape preserved. No new savefile format, persistent history, or GPU dispatch.

## Spatial hierarchy before remediation

Planet gridcell → gameplay children (cohort/fleet/infrastructure) directly; Location child under planet deferred as `DeepPlanetChildDeferred`.

## Spatial hierarchy after remediation

Planet gridcell → 1×1 surface gridcell (role `surface`, row=0, col=0) → gameplay children (cohorts, fleets, infrastructure, leaders).

## Planet surface gridcell admission

`LOCAL_GRIDCELL_ROLE_SURFACE` (`"surface"`) added. `make_planet_gridcell` auto-includes default surface child. Admission evaluates surface at (0,0); duplicate/missing surface are errors.

## Gameplay child re-homing

`evaluate_planet_non_grid_child` runs under surface gridcell only. `collect_planet_non_grid_children`, `planet_child_rf`, and `owner_silo_disburse_down` iterate `planet_gameplay_children` via surface.

## Direct planet gameplay child rejection

`PlanetDirectGameplayChildRequiresSurfaceGridcell` error for direct non-Location gameplay under planet gridcell.

## RF surface-arena settlement

Recursive RF walks surface as Location arena; gameplay participant rows use `parent_location_id_raw` = surface gridcell id. Report fields: `surface_arena_count`, `gameplay_rows_parented_to_surface`, `surface_to_planet_bubbling_present`.

## Surface → planet → star → galaxy bubbling

Surface arenas settle siblings locally; net surplus/deficit bubbles to planet, then star-system, then galaxy. Proven in RF runtime report and tests.

## STEAD / link / tree preservation

Corpus fixture and closing-track roundtrip/candidate tests preserve STEAD IDs, links, and spatial tree with surface tier added.

## Fixture migration

`scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json` and `scenarios/corpus/planet_child_location_admitted.simthing-scenario.json` regenerated with surface tier. New surface SimThing IDs are auto-assigned at fixture build time (deterministic builder order); gameplay child IDs and owner/resource metadata preserved.

## Candidate mutation/save/reopen preservation

Candidate from runtime, save/reopen, and Studio adoption tests assert surface tier / spatial tree preservation on owner_silo corpus.

## Studio UI preservation

Studio runtime save/load status and reopen-adopt tests load surface-tier corpus; UI/Bevy/runtime/GPU remain non-authoritative.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows were deleted. Scratch `agent-tools/regen_owner_silo_corpus.rs` removed. Ignored manual corpus regen tests remain in `disburse_down_fixture.rs` and `planet_child_location_admission.rs` only.

This PR is not a hygiene loop. It fixes the DA-blocking spatial-model defect identified by final review: the planet 1×1 surface gridcell tier was collapsed. The remediation restores the mandated surface tier, re-homes gameplay SimThings onto the surface gridcell, and moves recursive Accumulator Flow settlement to the surface arena before upward bubbling.

## Boundary / non-goals

No new savefile format, persistent history, GPU dispatch, pathfinding, combat, economy execution, fleet movement, or DA promotion. FINAL-DA-REVIEW-0 HOLD remains until re-run.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | SUBSTITUTE — Windows path-length error; scoped fmt PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-spec --test planet_child_location` | PASS (9) |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (25 + 1 ignored) |
| `cargo test -p simthing-spec --test loaded_scenario_recursive_rf_runtime` | PASS (15) |
| `cargo test -p simthing-spec --test scenario_stead_map_roundtrip` | PASS (11) |
| `cargo test -p simthing-spec --test loaded_scenario_runtime_report_chain` | PASS (18) |
| `cargo test -p simthing-spec --test scenario_candidate_from_runtime` | PASS (18) |
| `cargo test -p simthing-spec --test scenario_candidate_save_reopen` | PASS (24) |
| `cargo test -p simthing-driver --test loaded_scenario_recursive_rf_runtime` | PASS (10) |
| `cargo test -p simthing-driver --test loaded_scenario_runtime_report_chain` | PASS (9) |
| `cargo test -p simthing-driver --test scenario_candidate_from_runtime` | PASS (9) |
| `cargo test -p simthing-driver --test scenario_candidate_save_reopen` | PASS (8) |
| `cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui` | PASS (14) |
| `cargo test -p simthing-mapeditor --test studio_candidate_reopen_adopt` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/planet_child_location.rs`
- `crates/simthing-spec/src/spec/loaded_scenario_recursive_rf_runtime.rs`
- `crates/simthing-spec/src/spec/owner_silo_disburse_down.rs`
- `crates/simthing-spec/src/spec/planet_child_rf.rs`
- `crates/simthing-spec/src/spec/recursive_local_rf.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/planet_child_location.rs` (new)
- `crates/simthing-spec/tests/planet_child_location_admission.rs`
- `crates/simthing-spec/tests/disburse_down_fixture.rs`
- `crates/simthing-spec/tests/scenario_stead_map_roundtrip.rs`
- `crates/simthing-spec/tests/scenario_candidate_from_runtime.rs`
- `crates/simthing-spec/tests/scenario_candidate_save_reopen.rs`
- `crates/simthing-driver/tests/disburse_down_fixture.rs`
- `crates/simthing-driver/tests/loaded_scenario_recursive_rf_runtime.rs`
- `crates/simthing-mapeditor/tests/studio_scenario_runtime_saveload_ui.rs`
- `crates/simthing-mapeditor/tests/studio_candidate_reopen_adopt.rs`
- `scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json`
- `scenarios/corpus/planet_child_location_admitted.simthing-scenario.json`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/scenario_planet_surface_gridcell_tier_0_results.md`

## Known gaps

- FINAL-DA-REVIEW-0 HOLD not lifted until re-run.
- Replace-existing candidate save flow still deferred.
- Persistent history and GPU dispatch still deferred.

## Next recommended action

Re-run `SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0` after this PR merges and metadata is finalized.
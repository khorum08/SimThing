# STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 — canonical gridcell placement edit commands

> **Lifecycle: PROBATION** — spec-owned structural placement edit commands and Studio presentation wrapper landed. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #788 — STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0
**Merge:** `df681533`
**Base:** `master` after PR #787 / STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0 (`5f738f05`)

## Current defect / mutation-surface summary

Studio could load, display, edit safe metadata, save, and display ingestion/admission reports, but lacked explicit validated commands for structural gridcell placement and role edits over canonical Scenario authority.

## Structural edit command API model

`simthing-spec/src/spec/structural_edit.rs`:

- `GridcellRoleEdit`: `Inert`, `StarSystem`, `UnknownUnsupported` (testing/deferral only)
- `StructuralPlacementCommand`: `AddGridcell`, `MoveGridcell`, `RemoveGridcell`, `SetGridcellRole`
- `StructuralPlacementEditReport` with applied/rejected counts, warnings, errors
- `apply_structural_placement_command` — draft clone, apply, validate (`validate_scenario_root_authority`, `validate_stead_mapping_consistency`, `validate_scenario_links`), swap on success; no partial mutation on rejection

## Canonical authority mutation model

- Commands operate on canonical Scenario authority via existing `game_session_galaxy_map` helpers.
- Add creates Location gridcell under GalaxyMap + `structural_grid` placement + Cohort child; syncs `map_container_id`.
- Move updates placement coordinates and col/row property mirrors.
- Remove deletes tree child and placement; prunes incident links with warning.
- SetGridcellRole updates gridcell role metadata.
- Rejects: non-canonical root, missing GameSession/GalaxyMap, duplicate id/coordinate, missing gridcell, invalid coordinates, stale placements, gridcell not under GalaxyMap.

## STEAD consistency proof

**PASS** — `structural_edit_preserves_stead_mapping_consistency` and post-edit `validate_stead_mapping_consistency` in `apply_structural_placement_command`. `structural_grid.map_container_id` bound to GalaxyMap on each edit.

## Studio wrapper / projection rebuild proof

**PASS** — `studio_apply_structural_placement_command` calls spec API, rebuilds hydration/view_model/structural_projection/gpu_residency_readiness/scenario_document/admission_summary. Tests: `studio_add_gridcell_rebuilds_document_projection_and_admission`, `studio_set_gridcell_role_updates_display`.

## Save/reload roundtrip proof

**PASS** — `studio_move_gridcell_roundtrips_save_reload`, `studio_remove_gridcell_roundtrips_save_reload`, `studio_structural_edit_reload_preserves_admission_summary`. Uses `minimal_scenario_galaxymap.simthing-scenario.json`; not Terran Pirate.

## Driver structural readiness proof

**PASS** — `edited_scenario_reaches_structural_n4_admission`, `invalid_structural_edit_does_not_reach_driver_compile`, `studio_edited_scenario_driver_structural_n4_readiness_preserved`.

## GPU boundary status

**PASS** — unchanged from #786; structural readiness preserved. Studio structural edit does not dispatch GPU or call sim tick (`studio_structural_edit_does_not_dispatch_gpu`, `studio_structural_edit_does_not_call_sim_tick`). No new WGSL/shader files.

## Production synthesis update summary

Added § STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0; reprioritized Next Production Rungs (planet admission, owner-silo mutation, corpus UX, structural edit UI).

## Evidence lifecycle cleanup summary

**PASS** — PR #787 evidence-index row corrected to `#787` / `5f738f05`; `studio_ingestion_admission_report_display_0_results.md` merge metadata updated; live ledger preserved; no DA promotion; no prerequisite report deletion.

## Specified-vs-implemented ledger

| Requirement | Status |
|---|---|
| Spec owns structural placement edit API | PASS |
| Studio wrapper calls spec API | PASS |
| Add/move/remove/set-role update authority | PASS |
| GalaxyMap + structural_grid consistency | PASS |
| STEAD mapping consistency preserved | PASS |
| Invalid edits rejected without partial mutation | PASS |
| Studio rebuilds document/projection/admission | PASS |
| Save/reload roundtrip | PASS |
| Driver structural N4 readiness preserved | PASS |
| No Studio GPU dispatch | PASS |
| No sim tick ownership | PASS |
| No new engines/GPU/WGSL | PASS |
| #787 evidence metadata cleanup | PASS |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test structural_placement_edit_commands` | PASS (10/10) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS |
| `cargo test -p simthing-spec --test scenario_galaxymap_worldstate` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test structural_edit_compile_readiness` | PASS (2/2) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test studio_structural_placement_edit_commands` | PASS (9/9) |
| `cargo test -p simthing-mapeditor --test studio_ingestion_admission_report` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/structural_edit.rs` (new)
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/scenario.rs` (`game_session_child_mut` pub(crate))
- `crates/simthing-spec/tests/structural_placement_edit_commands.rs` (new)
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-mapeditor/src/studio_structural_edit.rs` (new)
- `crates/simthing-mapeditor/src/studio_scenario_document.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_structural_placement_edit_commands.rs` (new)
- `crates/simthing-driver/tests/structural_edit_compile_readiness.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/studio_ingestion_admission_report_display_0_results.md`
- `docs/tests/structural_placement_edit_commands_0_results.md` (new)

## Deleted/archived artifacts

None.

## Deferred next rung recommendation

1. Planet/child-location admission.
2. Full owner-silo state mutation if still deferred.
3. Broader scenario corpus ingestion UX / batch reports.
4. UI affordances for structural editing if command layer is headless.

## DA status

**PROBATION** — pending owner DA approval. No DA promotion in this PR.
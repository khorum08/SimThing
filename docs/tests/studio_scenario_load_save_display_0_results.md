# STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 — Studio canonical Scenario tree load/save/display

> **Lifecycle: PROBATION** — Studio document/view model, canonical fixture load/save/display, safe metadata roundtrip, and Terran Pirate legacy routing landed. No runtime tick ownership, no GPU dispatch. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #782 — STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0  
**Merge SHA:** `b4037fc9`  
**Base:** `master` after PR #781 / SESSION-GALAXYMAP-WORLDSTATE-0 (`c7ce8f55`)

## Current defect summary

PR #781 established canonical GalaxyMap under GameSession and corpus fixtures, but Studio still lacked a first-class document/projection for the full Scenario tree and safe canonical save/reload after metadata edits. Studio hydration used `resolve_map_container` (already GalaxyMap-aware) but had no explicit Scenario → GameSession → Owner(s) + GalaxyMap view model or legacy/canonical authority routing.

## Studio load/save/display model

`StudioScenarioDocument` (mapeditor) exposes:

```text
StudioScenarioDocument
  authority_kind: CanonicalScenario | LegacyWorldRoot
  scenario_id, schema_version, source_label
  game_session: StudioGameSessionView
  owners: Vec<StudioOwnerView>
  galaxy_map: StudioGalaxyMapView
  gridcells: Vec<StudioGridcellView>
```

`StudioSession` now carries `scenario_document` alongside existing hydration/view_model projections. Canonical fixtures route through `validate_scenario_root_authority`; Terran Pirate routes through `validate_legacy_world_root_compatibility` as `LegacyWorldRoot`.

## Canonical Scenario fixture coverage

| Fixture | Coverage |
|---|---|
| `scenarios/corpus/minimal_scenario_root.simthing-scenario.json` | Scenario metadata, GameSession, Owner, empty GalaxyMap |
| `scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json` | Full tree + 2 gridcells (inert + star_system) + structural placements |

## Canonical GalaxyMap fixture coverage

- Galaxy map rebuilt from `game_session_galaxy_map` / `resolve_map_container`, not World-root assumptions.
- Two gridcells visible in document and hydration (`occupied_cells == 2`, `view_model.stars.len() == 2`).
- Structural col/row and generated system id read from canonical mirrored properties.

## Safe edit/roundtrip coverage

| Edit | API | Roundtrip |
|---|---|---|
| Owner display name | `set_owner_display_name` | PASS |
| GalaxyMap display name | `set_galaxy_map_display_name` | PASS |
| Full tree preservation (no edit) | `save_studio_scenario_with_document_roundtrip` | PASS |

Structural placement edits remain deferred.

## Terran Pirate legacy compatibility status

**PASS** — `terran_pirate_skeleton` loads as `LegacyWorldRoot`: no GameSession/Owners in document, map from `resolve_map_container`, `terran_pirate_skeleton` integration tests unchanged.

## Spec helper APIs added

Read helpers in `simthing-spec/src/spec/scenario.rs`:

- `owner_display_name`, `owner_archetype`, `owner_color_index`, `owner_silo_marker`
- `galaxy_map_display_name`, `galaxy_map_role`
- `gridcell_role`, `gridcell_generated_system_id`, `gridcell_structural_col`, `gridcell_structural_row`

Edit helpers:

- `set_owner_display_name`, `set_galaxy_map_display_name`
- internal `game_session_child_mut` for safe GameSession child mutation

## Lower-layer driver proof expansion

**SKIP** — existing PR #781 proof `canonical_galaxymap_mapping_compile.rs` already admits canonical GalaxyMap fixture through `compile_structural_n4_theater`. No new GPU work in this PR.

## Production synthesis update summary

Added § STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 to `docs/0.8.3 Simthing Studio Production.md`. Reprioritized Next Production Rungs (ingestion, resource-flow silos, GPU execution after authority).

## Evidence lifecycle cleanup summary

- Added PROBATION row to `docs/tests/current_evidence_index.md`.
- SESSION-GALAXYMAP-WORLDSTATE-0 row already records PR #781 / `c7ce8f55`.
- No scratch logs or duplicate contradictory evidence introduced.

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| Load canonical Scenario-root fixtures | `load_canonical_studio_document_from_path` | PASS |
| Display Scenario → GameSession → Owner(s) + GalaxyMap | `StudioScenarioDocument` | PASS |
| Rebuild galaxy map from GalaxyMap child | `studio_galaxy_map_gridcells_from_spec` + hydration | PASS |
| Distinguish inert / star_system gridcells | `StudioGridcellRole` | PASS |
| Save/reload safe metadata edits | roundtrip helpers + tests | PASS |
| Terran Pirate legacy compatibility | `LegacyWorldRoot` routing | PASS |
| No runtime sim / GPU dispatch in document path | source guards + test | PASS |
| No Studio/GalaxyMap/Owner/Scenario engines | not introduced | PASS |
| Driver proof from Studio export | not expanded | SKIP (PR #781 covers) |
| GENERAL-SCENARIO-INGESTION-ADMISSION-0 | not implemented | SKIP (deferred) |
| SESSION-RESOURCE-FLOW-SILOS-0 | not implemented | SKIP (deferred) |
| Structural placement edits | not implemented | SKIP (deferred) |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test scenario_serializable_simthing_root` | PASS |
| `cargo test -p simthing-spec --test scenario_gamesession_child` | PASS |
| `cargo test -p simthing-spec --test scenario_owner_entities` | PASS |
| `cargo test -p simthing-spec --test scenario_galaxymap_worldstate` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS (10/10) |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS (10/10, 1 ignored) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test canonical_galaxymap_mapping_compile` | PASS (2/2) |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | PASS (11 files) |

## Files changed

| Path | Change |
|---|---|
| `crates/simthing-spec/src/spec/scenario.rs` | Read/edit helpers, `game_session_child_mut` |
| `crates/simthing-spec/src/spec/mod.rs` | Exports |
| `crates/simthing-spec/src/lib.rs` | Exports |
| `crates/simthing-mapeditor/src/studio_scenario_document.rs` | New document/view model |
| `crates/simthing-mapeditor/src/lib.rs` | Module + exports |
| `crates/simthing-mapeditor/src/session.rs` | `scenario_document` on `StudioSession` |
| `crates/simthing-mapeditor/src/scenario_io.rs` | `ScenarioDocument` IO error |
| `crates/simthing-mapeditor/tests/canonical_scenario_load_save_display.rs` | 10 integration tests |
| `docs/tests/studio_scenario_load_save_display_0_results.md` | This report |
| `docs/tests/current_evidence_index.md` | PROBATION row |
| `docs/0.8.3 Simthing Studio Production.md` | § STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 |

## Deleted/archived artifacts

None.

## Deferred next rung recommendation

1. **GENERAL-SCENARIO-INGESTION-ADMISSION-0** — broader scenario ingestion admission reports.
2. **SESSION-RESOURCE-FLOW-SILOS-0** — Owner stockpile/silo execution.
3. Studio structural placement edit surface and full editor mutation commands.

## DA status

**PROBATION** — pending owner/DA approval. No DA promotion claimed.
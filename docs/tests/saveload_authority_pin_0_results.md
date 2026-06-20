# SAVELOAD-AUTHORITY-PIN-0 - SimThing-Spec Studio authority pin

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added SAVELOAD-AUTHORITY-PIN-0 PROBATION row |
| `docs/tests/saveload_authority_pin_0_results.md` | PROBATION | This report |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Production synthesis, not DA approval |
| `docs/tests/studio_hydration_boundary_0_results.md` | PROBATION | Prior report remains superseded by scenario-authority wording |
| `docs/tests/studio_simthing_spec_boundary_1_results.md` | PROBATION | Prior scenario-authority report remains branch evidence |
| `docs/clausething/MapGeneratorCLI.md` | ACTIVE_DOC | Updated to name sole scenario authority and loaded-ID reservation |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | ACTIVE_TRACK_DOC | Updated with SAVELOAD-AUTHORITY-PIN-0 synthesis |

## simthing-core / simthing-spec compatibility review

`simthing-core` changes are additive: `advance_simthing_id_allocator_past`, `SimThing::max_id_in_subtree`, `reserve_simthing_ids_from_tree`, and exports were added. Existing `SimThingId::new`, `SimThingId::from_session_raw`, recursive `SimThing`, properties, overlays, and children behavior were not removed or signature-changed.

`simthing-spec` changes are additive over the placeholder-filled `SimThingScenarioSpec` surface: authority label, structural property IDs, loaded-scenario reservation helper, STEAD mapping validator, and validation error types were added. Existing scenario structs and exports remain.

## Clobber / dependency assessment

No existing code or dependencies were clobbered. The branch does not touch `crates/simthing-gpu/src`, `crates/simthing-driver/src`, `crates/simthing-sim/src`, or `crates/simthing-mapgenerator/src`.

## Canonical authority pin

The canonical Studio save/load authority is `SimThingScenarioSpec`, also documented as the SimThing-Spec-compliant scenario authority. `StudioSession::scenario_authority` is required for a valid session. `StudioHydrationBoundary`, `StudioHydratedGrid`, `StudioGalaxyViewModel`, render anchors, Bevy entities/transforms, render metadata, settings dialog state, and camera state are projections or editor metadata.

## STEAD mapping validation

`validate_stead_mapping_consistency` checks root world shape, duplicate SimThing ids, duplicate gridcell coordinates, duplicate system ids, missing placements, orphan gridcell Locations, missing child payloads, mirrored structural property mismatch, render-coordinate leakage, and occupied-cell count mismatch.

## ID allocator / reservation status

Loaded scenarios can reserve existing ids through `reserve_simthing_ids_from_scenario`, which advances the process-local SimThing id allocator past the maximum id in the loaded tree. Tests prove future spawns do not collide with loaded ids.

## RF readiness status

`StudioRfAccumulatorReadiness::from_scenario` derives from the canonical scenario tree and structural placements. It rejects missing gridcell placement state and does not accept render-anchor-only input.

## Heatmap readiness status

`StudioHeatmapReadiness::from_scenario` derives from the canonical structural frame and placements. It distinguishes bounded-theater eligibility from atlas-required dense execution deferral and does not treat atlas deferral as layout failure.

## Tests added

- `loaded_tree_reserves_existing_simthing_ids`
- `simthing_spec_scenario_serializes_and_deserializes`
- `simthing_spec_roundtrip_preserves_root_tree`
- `simthing_spec_roundtrip_preserves_structural_grid`
- `simthing_spec_roundtrip_preserves_links`
- `simthing_spec_roundtrip_preserves_gridcell_children`
- `loaded_scenario_reserves_existing_simthing_ids`
- `new_simthing_after_loaded_scenario_does_not_collide`
- `from_session_raw_or_equivalent_rejects_duplicate_ids`
- `stead_mapping_validator_accepts_valid_scenario`
- `stead_mapping_validator_rejects_missing_placement`
- `stead_mapping_validator_rejects_duplicate_coordinates`
- `stead_mapping_validator_rejects_or_ignores_render_only_coordinates_as_authority`
- `gridcell_structural_properties_match_structural_grid_if_mirrored`
- `rf_readiness_uses_simthing_spec_scenario`
- `rf_readiness_rejects_missing_gridcell_placement`
- `rf_readiness_rejects_render_anchor_only_input`
- `rf_readiness_participant_count_matches_location_gridcells`
- `heatmap_readiness_uses_simthing_spec_scenario`
- `heatmap_readiness_uses_structural_frame_not_render_data`
- `heatmap_readiness_reports_atlas_required_for_oversized_grid`
- `heatmap_readiness_does_not_mark_atlas_required_as_layout_failure`
- `studio_projection_rebuilds_from_scenario_authority`
- `view_model_rebuilds_from_scenario_authority`
- `model_change_applies_to_scenario_before_projection`

## Commands run

- `cargo fmt --all`
- `cargo fmt --all -- --check`
- `cargo check -p simthing-core`
- `cargo test -p simthing-core`
- `cargo check -p simthing-spec`
- `cargo test -p simthing-spec --test region_field_spec_admission`
- `cargo test -p simthing-spec`
- `cargo check -p simthing-mapeditor`
- `cargo test -p simthing-mapeditor`
- `cargo test -p simthing-mapgenerator`
- `cargo test -p simthing-clausething --test stead_spatial_contract_guards`
- `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy`
- `cargo test -p simthing-clausething --test mapgen_rf_stead_binding`
- `cargo test -p simthing-clausething --test mapgen_movement_front`

## Files changed

- `crates/simthing-core/src/ids.rs`
- `crates/simthing-core/src/lib.rs`
- `crates/simthing-core/src/simthing.rs`
- `crates/simthing-mapeditor/src/hydration.rs`
- `crates/simthing-mapeditor/src/session.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/tests/region_field_spec_admission.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/saveload_authority_pin_0_results.md`

## Production synthesis doc summary

`docs/design_0_0_8_3_studio_production.md` consolidates the constitutional spine, MapGeneratorCLI lineage, ClauseThing boundary, STEAD doctrine, generated galaxy authority, Studio projection layers, Bevy render state, RF readiness, heatmap readiness, save/load authority plan, evidence lifecycle, risks, deferred work, and next rungs.

## Deferred features

Save/load UI behavior, new-map flow, live SimThing simulation, RF execution, heatmap rendering, pathfinding, route/predecessor semantics, movement-order logic, semantic WGSL, simulation GPU kernels, Clausewitz UI importer, CSS/WebView, and new SimThing kinds remain deferred.

## DA status

PROBATION pending DA approval. This branch pins the architecture and adds tests/docs, but does not pre-file owner approval.

# STUDIO-SIMTHING-SPEC-BOUNDARY-1 - generated scenarios become SimThing-Spec authority

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added STUDIO-SIMTHING-SPEC-BOUNDARY-1 PROBATION row |
| `docs/tests/studio_simthing_spec_boundary_1_results.md` | PROBATION | This report |
| `docs/tests/studio_hydration_boundary_0_results.md` | PROBATION | Marked BOUNDARY-0 hydrated-grid authority wording as superseded |
| `docs/clausething/MapGeneratorCLI.md` | ACTIVE_DOC | Studio architecture names SimThing-Spec scenario authority before Studio projection |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | ACTIVE_TRACK_DOC | Studio row now distinguishes scenario authority from projection/presentation metadata |
| Screenshots | Not retained | No visual artifact required for this non-render authority boundary |

## Owner clarification

The DA request makes generated scenario authority a SimThing-Spec obligation before save/load work. Studio may keep editor projections, but Bevy entities, render anchors, DTOs, and the flat hydrated grid are not canonical authority.

## BOUNDARY-0 evaluation

STUDIO-HYDRATION-BOUNDARY-0 created a Studio projection/index that caught invalid generated output before render. It did not create a durable SimThing-Spec scenario authority. Its earlier "hydrated grid is future authority" wording is now explicitly superseded: `StudioHydrationBoundary` and `StudioHydratedGrid` are derived editor structures.

## Canonical SimThing-Spec authority representation

`simthing-spec::SimThingScenarioSpec` is now the post-generation authority shape used by Studio. It contains a recursive `SimThing` world root, a galaxy map container child, structural grid frame metadata, one structural placement per generated system, generated base link endpoint metadata, and generator provenance.

`StudioSession` carries this scenario alongside the generation output and derives the Studio hydration boundary from it.

## Recursive hierarchy proof

The generated hierarchy is:

```text
World
  -> galaxy map Location
       -> Location/gridcell SimThing per generated system
            -> child payload SimThing
```

Tests prove every generated system has exactly one structural placement, every placement has a unique `(col,row)`, every system id is unique, every placement points at a gridcell `Location`, and every gridcell has children.

## RF/Accumulator readiness

`rf_accumulator_readiness_from_simthing_spec` derives its readiness from the SimThing-Spec structural placements and `StructuralGridFrame` only. The report exposes grid width, height, occupied cells, and whether the scenario has structural placements; it does not depend on Bevy/render coordinates.

## Heatmap readiness

`heatmap_readiness_from_simthing_spec` derives from the SimThing-Spec structural frame. Standard bounded theaters are reported as bounded-theater ready; oversized dense execution reports `AtlasRequired`, preserving the STEAD distinction between valid large layout and execution-profile deferral.

## Tests added or updated

- `successful_generation_produces_simthing_spec_scenario`
- `simthing_spec_scenario_has_world_root`
- `world_root_has_galaxy_map_container_child`
- `galaxy_map_container_has_one_location_gridcell_per_generated_system`
- `each_generated_system_gridcell_has_structural_col_row`
- `each_generated_system_gridcell_has_children`
- `no_duplicate_gridcell_coordinates_in_simthing_spec`
- `no_duplicate_system_ids_in_simthing_spec`
- `studio_flat_grid_is_projection_not_authority`
- `view_model_derives_from_simthing_spec_scenario`
- `view_model_rebuilds_from_simthing_spec_scenario`
- `bevy_render_metadata_not_written_to_simthing_spec`
- `future_save_authority_manifest_mentions_studio_projection`
- `future_save_manifest_names_simthing_spec_as_authority`
- `rf_accumulator_readiness_derives_from_simthing_spec_structural_placements`
- `rf_accumulator_readiness_uses_no_render_metadata`
- `heatmap_readiness_derives_from_simthing_spec_structural_frame`
- `heatmap_readiness_reports_bounded_theater_for_small_grid`
- `heatmap_readiness_reports_atlas_required_for_oversized_dense_execution`

## Commands run

- `cargo fmt --all`
- `cargo fmt --all -- --check`
- `cargo check -p simthing-mapeditor`
- `cargo test -p simthing-mapeditor`
- `cargo test -p simthing-mapgenerator`
- `cargo test -p simthing-clausething --test stead_spatial_contract_guards`
- `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy`
- `cargo test -p simthing-clausething --test mapgen_rf_stead_binding`
- `cargo test -p simthing-clausething --test mapgen_movement_front`

All validation commands passed. Existing warning noise remains in `simthing-core`, `simthing-driver`,
`simthing-clausething`, and `simthing-mapgenerator`; this branch did not introduce or repair those warnings.

## Files changed

- `Cargo.lock`
- `crates/simthing-mapeditor/Cargo.toml`
- `crates/simthing-mapeditor/src/hydration.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/src/session.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/spec/scenario.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/studio_hydration_boundary_0_results.md`
- `docs/tests/studio_simthing_spec_boundary_1_results.md`

## Save/load authority update

Future save/load must serialize the SimThing-Spec-compliant scenario authority plus generation provenance, report summary, structural frame/placements, editor render settings, camera/view state, and UI settings. Studio DTOs, view models, render anchors, Bevy entities/transforms, star radii, hyperlane thickness/opacity, camera-depth buckets, and screen-space positions are projections only.

## Deferred features

Save/load UI behavior, new-map flow, live SimThing simulation, heatmap rendering, RF simulation, pathfinding, movement-order logic, route/predecessor semantics, GPU kernels, Clausewitz import, CSS/WebView, and new SimThing kinds remain deferred.

## DA status

PROBATION pending DA approval. The branch implements the requested authority boundary, but CURRENT_EVIDENCE promotion still requires owner sign-off after review.

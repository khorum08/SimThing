# STUDIO-HYDRATION-BOUNDARY-0 - generated maps hydrate into Studio SimThing grids

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added STUDIO-HYDRATION-BOUNDARY-0 PROBATION row |
| `docs/tests/studio_hydration_boundary_0_results.md` | PROBATION | This report |
| `docs/clausething/MapGeneratorCLI.md` | ACTIVE_DOC | Studio architecture now names the hydration boundary before view/render |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | ACTIVE_TRACK_DOC | Studio row now lists the hydration boundary before view/render; STUDIO-SIMTHING-SPEC-BOUNDARY-1 supersedes hydrated-grid-as-authority wording |
| Screenshots | Not retained | No visual artifact required for this non-render authority boundary |

## Boundary diagnosis

Existing ClauseThing/MapGen hydration lives in:

- `crates/simthing-clausething/src/hydrate_scenario.rs`
- `crates/simthing-clausething/src/mapgen_lattice.rs`

The Studio editor previously bypassed that hydration family: `run_generation` produced a
`GenerationRunOutput`, then `StudioSession` built `StudioGalaxyViewModel` directly from
`GalaxyGenerationResult` and `GenerationReport`. That made Bevy/view data the first structured consumer
inside the editor, which was the wrong table-setting boundary before save/load.

## New Studio hydration boundary

`crates/simthing-mapeditor/src/hydration.rs` adds `StudioHydrationBoundary` and
`hydrate_generation_into_studio_grid`. Successful Studio generation now means:

```text
run_generation
  -> GenerationRunOutput
  -> hydrate_generation_into_studio_grid
  -> StudioHydrationBoundary
  -> StudioSession hydrated projection/index
  -> StudioGalaxyViewModel::from_hydration
  -> Bevy presentation
```

`StudioSession::from_generation` now returns `Result<StudioSession, StudioHydrationError>` and refuses to
adopt invalid generated output. The UI reports the hydration error through the existing generation error
surface and leaves the active session unchanged.

## Gridcell representation

- The hydrated grid contains a `World` root and a `Location` map container.
- Every generated system maps to exactly one `Location` gridcell SimThing.
- Every gridcell preserves structural `structural_col` and `structural_row`.
- Duplicate generated system ids and duplicate gridcell coordinates fail hydration.
- Base hyperlane endpoint pairs are copied into the boundary only after endpoint validation.

## Children representation

Every star/gridcell owns a child star payload (`Cohort`) carrying the generated system id. This establishes
recursive SimThing containment for future save/load/live-sim work without introducing a new kind or changing
runtime simulation semantics.

## View and render projection

`StudioGalaxyViewModel::from_hydration` derives stars, render anchors, and hyperlane view data from the
hydrated grid. The legacy `from_generation` helper now hydrates first and then projects. Bevy transforms,
render height, star size/aura, camera-depth fades, ribbon width, visibility, and Settings values remain
presentation-only and are not written to hydration.

## Save/load table-setting

STUDIO-SIMTHING-SPEC-BOUNDARY-1 supersedes this report's original hydrated-grid-as-authority wording. The
hydrated Studio grid remains a projection/index over generated map structure; future save/load authority is
now the SimThing-Spec-compliant scenario, alongside the generation profile and report summary. This pass did
not implement save/load UI behavior. New-map flow, live SimThing adoption, route/predecessor semantics,
movement orders, pathfinding, GPU kernels, Clausewitz import, and CSS/WebView remain deferred.

## Tests added or updated

- `successful_generation_produces_hydration_boundary`
- `studio_session_requires_hydrated_grid`
- `hydrated_grid_has_world_root_and_map_container`
- `hydrated_grid_cells_are_location_simthings`
- `every_generated_system_has_one_gridcell_simthing`
- `every_gridcell_has_structural_col_row`
- `every_star_gridcell_has_children`
- `no_duplicate_gridcell_coordinates`
- `no_duplicate_system_ids`
- `view_model_derives_from_hydration_not_raw_render_metadata`
- `view_model_preserves_structural_coords_from_hydration`
- `bevy_render_metadata_not_written_to_hydration`
- `failed_hydration_does_not_adopt_session`
- `future_save_authority_manifest_mentions_studio_projection`

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
git diff --check
git diff --name-only master...HEAD
```

## Files changed

- `crates/simthing-mapeditor/Cargo.toml`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/hydration.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/src/session.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/studio_hydration_boundary_0_results.md`

## Deferred features

No runtime/GPU/spec production code changed. This pass does not alter MapGenerator topology, ClauseThing
production hydration, live simulation, save/load UI behavior, pathfinding, movement-order semantics,
semantic WGSL, GPU kernels, Clausewitz import, CSS/WebView, or SimThing kind doctrine.

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.

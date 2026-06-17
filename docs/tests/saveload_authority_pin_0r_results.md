# SAVELOAD-AUTHORITY-PIN-0R — Authority hardening

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added SAVELOAD-AUTHORITY-PIN-0R PROBATION row |
| `docs/tests/saveload_authority_pin_0r_results.md` | PROBATION | This report |
| `docs/0.8.3 Simthing Studio Production.md` | PROBATION | Standing Studio production synthesis updated |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Space-free alias synced |
| `docs/tests/saveload_authority_pin_0_results.md` | PROBATION | Prior pin remains branch context |

## Authority hardening summary

Hardened the Studio save/load substrate without adding UI. Whole `SimThingScenarioSpec` is the serialized authority. Invalid STEAD mapping no longer classifies heatmap readiness as bounded or atlas-required. Model edits apply to scenario authority first; projections rebuild from it.

## simthing-core compatibility / clobber review

Additive and compatibility-preserving:

- `SimThingIdReservationError` with `DuplicateId` and `IdSpaceExhausted`
- `advance_simthing_id_allocator_past` now returns `Result`
- `reserve_simthing_ids_from_tree` now returns `Result` and rejects duplicate ids before advancing the allocator

No changes to `simthing-gpu`, `simthing-driver`, `simthing-sim`, or `simthing-mapgenerator`.

## map_container_id validation

- `resolve_map_container` / `resolve_map_container_mut` bind `structural_grid.map_container_id` to a `SimThing` raw id in the recursive tree
- Container must be `Location` and a direct child of `World`
- Gridcells must be children of the declared container; orphans under other parents are rejected
- Generation now records the map container's actual raw id instead of the `studio_galaxy_map` logical alias

## Structural integer exactness decision

Chose the Cursor-scale remedy: f32 mirror range-check (`SCENARIO_STRUCTURAL_INTEGER_MAX = 16_777_216`) rather than a breaking `PropertyValue` enum expansion. Primary authority remains `structural_grid.placements`.

## Scenario serialization target

- `serialize_scenario_authority` / `deserialize_scenario_authority` added in `simthing-spec`
- Deserialize validates STEAD mapping and reserves ids
- Naked `root: SimThing` serialization is not exposed as sufficient authority (tested)

## ID allocator / load safety

- `reserve_simthing_ids_from_scenario` returns `Result<(), SimThingIdReservationError>`
- Duplicate ids in loaded tree rejected
- `u32::MAX` subtree max id reports `IdSpaceExhausted`

## RF readiness invalid-STEAD behavior

Invalid STEAD mapping yields `ready_for_spatial_rf_over_locations = false` with `deferred_reason`. Participant count derives from gridcells under the declared map container.

## Heatmap readiness invalid-STEAD behavior

`StudioHeatmapReadinessKind::InvalidSteadMapping` added. Removed fallback that classified invalid scenarios as bounded/atlas. `AtlasRequired` remains valid-structure execution deferral.

## Tests added / hardened

Consolidated across `simthing-spec` (`scenario.rs` tests) and `simthing-mapeditor` (`hydration.rs` tests):

- Heatmap invalid STEAD: not ready, no bounded/atlas classification, atlas-not-layout-failure, valid small/oversized grids
- Map container: missing/dangling/non-Location/wrong-parent/accepted declared container, no first-Location fallback
- Structural integers: exact roundtrip, above-max rejection, placement primary authority, mirror match, mismatch rejection
- Serde: root-alone insufficient, roundtrip root/grid/container/links/provenance
- IDs: reserve, no collision, duplicate reject, exhausted id space
- Model-first edit: `apply_gridcell_property_edit`, projection rebuild, view model non-authoritative, render metadata not written to authority
- RF: invalid STEAD deferred, declared map container, participant count, render-anchor reject

## Commands run

- `cargo fmt --all`
- `cargo fmt --all -- --check`
- `cargo check -p simthing-core`
- `cargo test -p simthing-core`
- `cargo check -p simthing-spec`
- `cargo test -p simthing-spec`
- `cargo check -p simthing-mapeditor`
- `cargo test -p simthing-mapeditor`
- `cargo test -p simthing-mapgenerator`
- `cargo test -p simthing-clausething --test stead_spatial_contract_guards`
- `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy`
- `cargo test -p simthing-clausething --test mapgen_rf_stead_binding`
- `cargo test -p simthing-clausething --test mapgen_movement_front`
- `git diff --check`

## Files changed

- `crates/simthing-core/src/ids.rs`
- `crates/simthing-core/src/lib.rs`
- `crates/simthing-core/src/simthing.rs`
- `crates/simthing-spec/Cargo.toml`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-mapeditor/src/hydration.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/saveload_authority_pin_0r_results.md`

## Production synthesis doc update

Both `docs/0.8.3 Simthing Studio Production.md` and the space-free alias received the `## SAVELOAD-AUTHORITY-PIN-0R — Authority hardening` section plus updated Known Risks, Deferred Work, Next Production Rungs, and Evidence lifecycle rows.

## Deferred work

Save/load file IO and UI, ClauseThing import, full editor mutation surface, exact typed properties, heatmap/RF visualization, atlas execution, live simulation.

## DA status

**PROBATION** — not DA-approved. Owner sign-off required before CURRENT_EVIDENCE promotion.
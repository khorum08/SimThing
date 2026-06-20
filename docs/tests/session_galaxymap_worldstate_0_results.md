# SESSION-GALAXYMAP-WORLDSTATE-0 — admit GalaxyMap under GameSession

> **Lifecycle: PROBATION** — GalaxyMap validation, Owner directness hardening, STEAD spatial authority reconnect, fixtures, and driver structural admission proof landed. Studio tree editing and resource-flow silos deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #781 — SESSION-GALAXYMAP-WORLDSTATE-0  
**Merge SHA:** `c7ce8f55`  
**Base:** `master` after PR #780 / SESSION-OWNER-ENTITIES-0

## Current defect summary

PR #780 required Owner children under GameSession but left spatial authority on transitional `World` subtrees and did not reject Owner/Faction nodes nested under future GalaxyMap children. Canonical save-game shape requires exactly one GalaxyMap / WorldStateMap spatial root under GameSession with `structural_grid.map_container_id` bound to it.

## Owner directness hardening summary

`find_owner_not_direct_gamesession_child` now traverses the full Scenario tree. Owner/Faction SimThings are valid only as direct GameSession children. Nested Owners under GalaxyMap, CapabilityTree, or Scenario root are rejected with `OwnerNotDirectGameSessionChild`.

## GalaxyMap / WorldStateMap representation decision

**Chosen:** `SimThingKind::Location` + role metadata (`galaxy_map`). No new `SimThingKind::GalaxyMap`, no GalaxyMapEngine, no WorldEngine.

## GalaxyMap id/property model

| Property ID | Semantics |
|---|---|
| `GALAXY_MAP_ID_PROPERTY_ID` (8_300_400) | Non-empty canonical map id |
| `GALAXY_MAP_ROLE_PROPERTY_ID` (8_300_401) | Role string `galaxy_map` |
| `GALAXY_MAP_DISPLAY_NAME_PROPERTY_ID` (8_300_402) | Display label |
| `GALAXY_GRIDCELL_ROLE_PROPERTY_ID` (8_300_403) | Gridcell role (`inert` / `star_system`) |

## Canonical spatial authority model

```text
Scenario
└── GameSession
    ├── Owner(s)
    └── GalaxyMap (Location + galaxy_map role)  ← spatial root
        └── Location gridcells
```

`spatial_authority_root` resolves canonical GalaxyMap first; legacy `World` under GameSession or direct Scenario `World` remains explicit compatibility for STEAD on Terran Pirate–class fixtures.

## structural_grid.map_container_id binding decision

For canonical fixtures, `map_container_id` resolves to the GalaxyMap SimThing id (spatial root self-binding). Legacy World→Location map-container pattern still resolves map container as a direct child of World.

`MapContainerNotWorldChild` renamed to `MapContainerNotSpatialRootChild`.

## Fixture update

`scenarios/corpus/minimal_scenario_root.simthing-scenario.json`:

```text
Scenario → GameSession → Owner + GalaxyMap
map_container_id → GalaxyMap raw id
```

## Canonical spatial fixture

`scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json`:

```text
Scenario → GameSession → Owner + GalaxyMap
  └── gridcell A (inert), gridcell B (star_system)
8×8 structural grid, 2 placements
```

## Legacy World-root compatibility decision

Terran Pirate / World-root fixtures load via `validate_legacy_world_root_compatibility` only. `validate_session_galaxy_map` returns `LegacyWorldRootHasNoGalaxyMapRequirement` on World roots.

## GPU-resident lower-layer proof status

**PASS** — `crates/simthing-driver/tests/canonical_galaxymap_mapping_compile.rs` proves canonical GalaxyMap fixture admits through `compile_structural_n4_theater` (8×8 single-cell admit + oversize atlas deferral). Mapping plan compile / sim resident tick not expanded in this PR (reuses existing lower-layer assets only).

## Tests added/changed

| Test | File |
|---|---|
| 12 GalaxyMap validation tests | `scenario_galaxymap_worldstate.rs` |
| Owner nested-under-GalaxyMap/CapabilityTree | `scenario_owner_entities.rs` |
| Minimal helpers include GalaxyMap | `scenario_gamesession_child.rs`, `scenario_serializable_simthing_root.rs` |
| STEAD unit tests use GalaxyMap spatial root | `scenario.rs` |
| e10 GalaxyMap/Owner directness guards | `e10_resource_flow_admission.rs` |
| Driver structural N4 admission | `canonical_galaxymap_mapping_compile.rs` |

## Production synthesis update

Generated Galaxy Authority: `Scenario → GameSession → Owner(s) → GalaxyMap / WorldStateMap`. GalaxyMap documented as root spatial Location; World explicitly not canonical spatial root. Terran Pirate remains lower-layer golden fixture.

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| Exactly one GalaxyMap under GameSession | `validate_session_galaxy_map` | PASS |
| Owner directness tree-wide | `find_owner_not_direct_gamesession_child` | PASS |
| Location + role GalaxyMap | `make_galaxy_map` / `is_galaxy_map_entity` | PASS |
| map_container_id → GalaxyMap | `resolve_map_container` self-binding | PASS |
| STEAD accepts GalaxyMap root | `validate_stead_mapping_consistency` | PASS |
| Minimal + spatial corpus fixtures | Updated/added | PASS |
| Legacy World-root separate | Explicit error path | PASS |
| Driver structural N4 proof | `canonical_galaxymap_mapping_compile.rs` | PASS |
| Studio full tree editing | Not implemented | SKIP (deferred) |
| Resource-flow silos | Not implemented | SKIP (deferred) |
| Planets | Not implemented | SKIP (deferred) |

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
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test canonical_galaxymap_mapping_compile` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | PASS (14 files) |

## Files changed

| Path | Change |
|---|---|
| `crates/simthing-spec/src/spec/scenario.rs` | GalaxyMap API, Owner hardening, STEAD spatial authority |
| `crates/simthing-spec/src/spec/mod.rs` | Exports |
| `crates/simthing-spec/src/lib.rs` | Exports |
| `crates/simthing-spec/tests/scenario_galaxymap_worldstate.rs` | New tests |
| `crates/simthing-spec/tests/scenario_owner_entities.rs` | Owner hardening tests + helpers |
| `crates/simthing-spec/tests/scenario_gamesession_child.rs` | Helper |
| `crates/simthing-spec/tests/scenario_serializable_simthing_root.rs` | Helper |
| `crates/simthing-spec/tests/e10_resource_flow_admission.rs` | Guards |
| `crates/simthing-driver/tests/canonical_galaxymap_mapping_compile.rs` | Driver proof |
| `scenarios/corpus/minimal_scenario_root.simthing-scenario.json` | GalaxyMap child |
| `scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json` | New spatial fixture |
| `docs/tests/session_galaxymap_worldstate_0_results.md` | This report |
| `docs/tests/current_evidence_index.md` | New row |
| `docs/design_0_0_8_3_studio_production.md` | Authority update |

## Deleted/archived artifacts

None.

## Deferred next rungs

- **STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0** — full Studio scenario tree load/save/edit/display
- **GENERAL-SCENARIO-INGESTION-ADMISSION-0** — broader ingestion if needed before Studio display

## Deferred later rung

- **SESSION-RESOURCE-FLOW-SILOS-0** — stockpile reduce-up/disburse-down execution

## DA status

**PROBATION** — pending owner/DA approval. No DA promotion claimed.
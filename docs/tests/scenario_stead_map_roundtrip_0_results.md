# SCENARIO-STEAD-MAP-ROUNDTRIP-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `scenario-stead-map-roundtrip-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Implement Closing Track Rung 1: prove STEAD map roundtrip over canonical ScenarioSpec I/O (#828) and retire the obsolete `docs/0.8.3 Simthing Studio Production.md` production alias.

## Canonical IO baseline

Reuses `prove_scenario_canonical_load_save_roundtrip` and `compile_scenario_canonical_io_plan_from_json_str` from SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 (#828). No canonical JSON load/save reimplementation.

## Retired obsolete production alias

Deleted `docs/0.8.3 Simthing Studio Production.md`. Retargeted production synthesis references to `docs/design_0_0_8_3_studio_production.md`. Per-rung historical references in `docs/tests/*_results.md` updated accordingly.

## STEAD ID preservation

`ScenarioSteadIdRow` extraction before/after canonical roundtrip is deterministically stable on `owner_silo_disburse_down_scoped` corpus fixture.

## Link integrity preservation

`ScenarioSteadLinkRow` extraction stable across roundtrip (empty link set preserved on fixture).

## Spatial tree preservation

`ScenarioSpatialTreeRow` extraction preserves parent/child depth, Location flags, and interior grid dimensions across roundtrip.

## Owner metadata vs spatial parentage

`owner_metadata_not_spatial_parentage` reports true: Owners remain direct GameSession children; owner_ref metadata does not imply spatial parentage.

## RF metadata preservation

`ScenarioRfMetadataRow` extraction preserves owner_ref, resource_key, and scope channel metadata across roundtrip.

## Recursive RF prerequisite proof

`local_rf_parent_node_resolution_prerequisites_present` reports true: Location tree, spatial gridcell interior grids, parent Location arenas, and RF channel keys are discoverable.

## Studio projection rebuild readiness

`studio_projection_rebuild_ready` reports true after STEAD validation, link validation, ID reservation, and canonical ingestion admission.

## Boundary / non-goals

Runtime mutation, savefile persistence, semantic execution, Studio UI wiring, and GPU dispatch remain deferred. No fixtures modified. No Terran Pirate edits.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test scenario_stead_map_roundtrip` | PASS (10) |
| `cargo test -p simthing-spec --test scenario_canonical_io` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test scenario_stead_map_roundtrip` | PASS (8) |
| `cargo test -p simthing-driver --test scenario_canonical_io` | PASS |
| `git diff --check` | PASS |
| Alias doc deleted | PASS |
| Dead alias link grep | PASS |

## Files changed

- `crates/simthing-spec/src/spec/scenario_stead_map_roundtrip.rs` (new)
- `crates/simthing-spec/tests/scenario_stead_map_roundtrip.rs` (new)
- `crates/simthing-driver/src/scenario_stead_map_roundtrip_compile.rs` (new)
- `crates/simthing-driver/tests/scenario_stead_map_roundtrip.rs` (new)
- `docs/design_0_0_8_3_studio_production.md` (rung 1 DONE; rung 2 NEXT)
- `docs/tests/current_evidence_index.md`
- `docs/tests/scenario_stead_map_roundtrip_0_results.md` (this report)
- `docs/0.8.3 Simthing Studio Production.md` (deleted)
- Reference retargets across `docs/tests/*` and `crates/*`

## Evidence lifecycle

**SCENARIO-STEAD-MAP-ROUNDTRIP-0** — PROBATION. Not DA-promoted.

## Known gaps

- Closing track Rungs 2–7 not yet implemented.
- Studio session envelope (Rung 2) is next.

## Next recommended action

Implement **LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0** (Closing Track Rung 2).

This rung is not another hygiene-only wrapper. It moves the Scenario Runtime + Save/Load Closing Track forward by proving that canonical ScenarioSpec load/save/reload preserves STEAD IDs, links, ownership metadata, RF metadata, and spatial tree shape required by Studio import/export and recursive RF runtime. It also retires the obsolete docs/0.8.3 Simthing Studio Production.md alias so future production-track updates target docs/design_0_0_8_3_studio_production.md.
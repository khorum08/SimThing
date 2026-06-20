# SESSION-OWNER-ENTITIES-0 — admit Owner entities under GameSession

> **Lifecycle: PROBATION** — Owner child validation, property model, fixture update, and tests landed. GalaxyMap and resource-flow silos deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #780 — SESSION-OWNER-ENTITIES-0  
**Merge SHA:** `cbe5e87d`  
**Base:** `master` after PR #779 / SCENARIO-GAMESESSION-CHILD-0

## Current defect summary

PR #779 required `Scenario -> GameSession` but allowed an empty GameSession with no Owner children. The intended save-game tree requires Owner entities as direct GameSession children before GalaxyMap enforcement.

## Owner child validation model

```text
Scenario
└── GameSession (exactly one)
    ├── Owner (≥1 required for canonical validation)
    ├── Owner (optional additional)
    └── Custom("GalaxyMap") / future children (allowed, not counted as Owners)
```

- `game_session_owners(spec)` — direct Owner/Faction children of GameSession
- `validate_session_owner_entities(spec)` — runs after GameSession validation in canonical mode
- Errors: `MissingOwnerEntities`, `OwnerMissingId`, `DuplicateOwnerId`, `OwnerNotDirectGameSessionChild`, `LegacyWorldRootHasNoOwnerRequirement`

## Owner id property model

| Property ID | Semantics |
|---|---|
| `OWNER_ID_PROPERTY_ID` (8_300_300) | Non-empty canonical owner id (string encoding) |
| `OWNER_DISPLAY_NAME_PROPERTY_ID` (8_300_301) | Display label |
| `OWNER_ARCHETYPE_PROPERTY_ID` (8_300_302) | Owner kind/archetype (not faction terminology) |
| `OWNER_COLOR_INDEX_PROPERTY_ID` (8_300_303) | Reserved for future Studio use |
| `OWNER_SILO_MARKER_PROPERTY_ID` (8_300_304) | Inert stockpile placeholder (u32 0) |

Identity lives on Owner SimThing properties — not sidecar authority.

## Capability/tree placeholder decision

**Chosen:** `Custom("CapabilityTree")` child under Owner in minimal fixture. No new privileged kind; no capability gameplay logic.

## Stockpile/silo placeholder decision

**Chosen:** single inert `owner_silo_marker` u32 property (value 0) on each Owner via `apply_owner_entity_metadata`. Full `owner_stockpile_capacity/current/reserved` and reduce-up/disburse-down deferred to **SESSION-RESOURCE-FLOW-SILOS-0**.

## Fixture update

`scenarios/corpus/minimal_scenario_root.simthing-scenario.json`:

```text
Scenario (metadata + lossless seed)
└── GameSession
    └── Owner (owner_id: "minimal_owner", CapabilityTree placeholder child)
```

## Legacy World-root compatibility

Terran Pirate / World-root fixtures load via `validate_legacy_world_root_compatibility` only. `validate_session_owner_entities` returns `LegacyWorldRootHasNoOwnerRequirement` on World roots.

## Tests added/changed

| Test | File |
|---|---|
| `scenario_requires_at_least_one_owner_child` | `scenario_owner_entities.rs` |
| `scenario_owner_child_must_be_direct_gamesession_child` | same |
| `scenario_owner_id_roundtrips` | same |
| `scenario_duplicate_owner_ids_are_rejected` | same |
| `scenario_missing_owner_id_is_rejected` | same |
| `scenario_non_owner_gamesession_child_does_not_count_as_owner` | same |
| `scenario_owner_validation_preserves_gamesession_requirement` | same |
| `legacy_world_root_does_not_satisfy_owner_validation` | same |
| `minimal_owner_fixture_deserializes` | same |
| `scenario_owner_preserves_lossless_metadata_roundtrip` | same |
| Minimal spec helpers add Owner | `scenario_gamesession_child.rs`, `scenario_serializable_simthing_root.rs` |
| Internal STEAD scenarios add Owner under GameSession | `scenario.rs` unit tests |
| e10 owner validation + doctrine guards | `e10_resource_flow_admission.rs` |

## Production synthesis update

- Generated Galaxy Authority: `Scenario -> GameSession -> Owner(s)`
- Owners documented as GameSession children, not overlays, not spatial parents
- GalaxyMap and resource-flow silos explicitly deferred

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| At least one Owner under GameSession | `validate_session_owner_entities` | PASS |
| Unique non-empty owner_id | Property validation + tests | PASS |
| Owner identity on properties | `OWNER_*_PROPERTY_ID` constants | PASS |
| Capability placeholder | `Custom("CapabilityTree")` optional child | PASS |
| Silo placeholder | `owner_silo_marker` | PASS |
| Legacy World-root separate | Explicit error path | PASS |
| Minimal fixture Scenario->GameSession->Owner | Corpus updated | PASS |
| Lossless seed preserved | Roundtrip test | PASS |
| GalaxyMap | Not implemented | SKIP (deferred) |
| Resource-flow silos | Not implemented | SKIP (deferred) |
| Owner/faction engine | Not introduced | PASS |
| GPU / Studio runtime / MapGenerator | Not touched | PASS |

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
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | PASS (11 files) |

## Files changed

| Path | Change |
|---|---|
| `crates/simthing-spec/src/spec/scenario.rs` | Owner properties, validation, helpers |
| `crates/simthing-spec/src/spec/mod.rs` | Exports |
| `crates/simthing-spec/src/lib.rs` | Exports |
| `crates/simthing-spec/tests/scenario_owner_entities.rs` | New tests |
| `crates/simthing-spec/tests/scenario_gamesession_child.rs` | Owner in minimal helper |
| `crates/simthing-spec/tests/scenario_serializable_simthing_root.rs` | Owner in minimal helper |
| `crates/simthing-spec/tests/e10_resource_flow_admission.rs` | Guards |
| `scenarios/corpus/minimal_scenario_root.simthing-scenario.json` | Owner child |
| `docs/tests/session_owner_entities_0_results.md` | This report |
| `docs/tests/current_evidence_index.md` | New row |
| `docs/design_0_0_8_3_studio_production.md` | Authority + section |

## Deleted/archived artifacts

None.

## Deferred next rungs

- **SESSION-GALAXYMAP-WORLDSTATE-0** — GalaxyMap/WorldStateMap under GameSession
- **SESSION-RESOURCE-FLOW-SILOS-0** — stockpile reduce-up/disburse-down execution

## DA status

**PROBATION** — pending owner/DA approval. No DA promotion claimed.
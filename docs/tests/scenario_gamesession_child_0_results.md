# SCENARIO-GAMESESSION-CHILD-0 — require GameSession under Scenario root

> **Lifecycle: PROBATION** — GameSession kind, canonical child validation, fixture update, and tests landed. Owners/GalaxyMap deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** SCENARIO-GAMESESSION-CHILD-0  
**Base:** `master` after PR #778 / SCENARIO-METADATA-LOSSLESS-0

## Current defect summary

PR #777 made `SimThingKind::Scenario` the canonical serializable file root and PR #778 made Scenario-root metadata lossless. The minimal canonical corpus fixture and validation still allowed a Scenario root with **no** `GameSession` child — a temporary state only. The intended save-game tree is `Scenario -> GameSession -> (Owners, GalaxyMap)`.

## Canonical term

**GameSession** — added as `SimThingKind::GameSession` and `SimThingKindTag::GameSession`. No separate `Session` alias; doctrine comments refer to "GameSession / Session" conceptually but serialized kind string is `"GameSession"`.

## GameSession kind/tag implementation

| Surface | Change |
|---|---|
| `SimThingKind::GameSession` | Added in `simthing-core` — authority marker only, not a runtime engine |
| `SimThingKindTag::GameSession` | Added in `property.rs` for fission template compatibility |
| `kind_matches` | `"GameSession"` matches `SimThingKind::GameSession` |
| `kind_tag_to_kind` | `fission.rs` arm added |

## Canonical validation rule

```text
Scenario (file root)
└── GameSession (exactly one direct child)
```

- `game_session_child(spec)` resolves the sole direct `GameSession` child.
- `validate_scenario_game_session_child(spec)` enforces exactly one direct child with `kind == GameSession`.
- Integrated into `validate_scenario_root_authority(..., Canonical)`.
- Errors: `MissingGameSessionChild`, `MultipleGameSessionChildren`, `GameSessionChildWrongKind`, `LegacyWorldRootHasNoGameSessionRequirement`.

## Legacy World-root compatibility

| Path | GameSession requirement |
|---|---|
| Canonical `Scenario` deserialize | Required |
| Legacy `World` deserialize (Terran Pirate) | Not required — `LegacyWorldRootHasNoGameSessionRequirement` if canonical validation attempted |

`spatial_authority_root` continues transitional STEAD lookup: `Scenario -> GameSession -> World`, then fallback `Scenario -> World` for legacy transitional layouts. World is not presented as the running session root.

## Fixture update

`scenarios/corpus/minimal_scenario_root.simthing-scenario.json`:

```text
Scenario (metadata + lossless seed chunks)
└── GameSession (id 2, empty)
```

No Owners, no GalaxyMap. `structural_grid` remains empty.

## Seed metadata preservation

`scenario_gamesession_preserves_lossless_metadata_roundtrip` proves `0x1234_5678_9ABC_DEF0` and sidecar sync survive serialize/deserialize with GameSession child present. PR #778 seed tests in `scenario_serializable_simthing_root.rs` unchanged and passing.

## Tests added/changed

| Test | File |
|---|---|
| `scenario_requires_exactly_one_gamesession_child` | `scenario_gamesession_child.rs` |
| `scenario_missing_gamesession_child_is_rejected` | same |
| `scenario_multiple_gamesession_children_are_rejected` | same |
| `scenario_world_child_does_not_count_as_gamesession` | same |
| `scenario_gamesession_child_roundtrips` | same |
| `scenario_gamesession_preserves_lossless_metadata_roundtrip` | same |
| `legacy_world_root_compatibility_does_not_satisfy_canonical_gamesession_validation` | same |
| `minimal_gamesession_fixture_deserializes` | same |
| `arbitrary_non_scenario_root_still_rejected` | same |
| `minimal_scenario_spec` adds GameSession child | `scenario_serializable_simthing_root.rs` |
| Internal STEAD scenarios wrap `GameSession -> World` | `scenario.rs` unit tests |
| e10 GameSession + production-doc guards | `e10_resource_flow_admission.rs` |

## Production synthesis cleanup

- **Generated Galaxy Authority** section updated: current authority is `Scenario -> GameSession`; `root: World` removed from active summary.
- World-root and Terran Pirate classified as legacy/lower-layer only.
- Next Production Rungs reprioritized: SESSION-OWNER-ENTITIES-0, SESSION-GALAXYMAP-WORLDSTATE-0, etc.

## Evidence lifecycle

| Artifact | Status |
|---|---|
| `current_evidence_index.md` | Updated — SCENARIO-GAMESESSION-CHILD-0 PROBATION row; #777/#778 marked prerequisites |
| `scenario_serializable_simthing_root_0_results.md` | Unchanged PROBATION prerequisite |
| `scenario_metadata_lossless_0_results.md` | Unchanged PROBATION prerequisite |
| This report | PROBATION |

No live ledger deletion. No DA promotion.

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| `SimThingKind::GameSession` | Added + tag + kind_matches | PASS |
| Exactly one GameSession child | `validate_scenario_game_session_child` | PASS |
| Reject missing/multiple/wrong-kind | Tests + errors | PASS |
| World child ≠ GameSession | `GameSessionChildWrongKind` test | PASS |
| Legacy World-root separate | Explicit compatibility path | PASS |
| Minimal fixture Scenario -> GameSession | Corpus JSON updated | PASS |
| Lossless seed preserved | Roundtrip test | PASS |
| Production doc cleanup | Authority section updated | PASS |
| Owners / GalaxyMap | Not implemented | SKIP (deferred) |
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
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |

## Files changed

| Path | Change |
|---|---|
| `crates/simthing-core/src/simthing.rs` | `SimThingKind::GameSession` + `kind_matches` |
| `crates/simthing-core/src/property.rs` | `SimThingKindTag::GameSession` |
| `crates/simthing-sim/src/fission.rs` | `kind_tag_to_kind` arm |
| `crates/simthing-spec/src/spec/scenario.rs` | Validation + spatial root update |
| `crates/simthing-spec/src/spec/mod.rs` | Exports |
| `crates/simthing-spec/src/lib.rs` | Exports |
| `crates/simthing-spec/tests/scenario_gamesession_child.rs` | New tests |
| `crates/simthing-spec/tests/scenario_serializable_simthing_root.rs` | GameSession in minimal spec |
| `crates/simthing-spec/tests/e10_resource_flow_admission.rs` | Guards |
| `scenarios/corpus/minimal_scenario_root.simthing-scenario.json` | GameSession child |
| `docs/tests/scenario_gamesession_child_0_results.md` | This report |
| `docs/tests/current_evidence_index.md` | New row |
| `docs/0.8.3 Simthing Studio Production.md` | Authority cleanup + section |

## Deleted/archived artifacts

None.

## Deferred next rung

**SESSION-OWNER-ENTITIES-0** — Owner entities as GameSession sibling children.

## DA status

**PROBATION** — pending owner/DA approval. No DA promotion claimed.
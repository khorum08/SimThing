# SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 — Scenario SimThing as canonical file root

> **Lifecycle: PROBATION** — Scenario kind, metadata-on-root, canonical validation, legacy compatibility, and roundtrip tests landed. GameSession child enforcement deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0  
**Base:** `master` after PR #776 / SCENARIO-SESSION-OWNER-ROOT-REVISED-0

## Current defect summary

`SimThingScenarioSpec` wrapped `scenario_id`, `provenance`, and a bare `root: SimThing` with `validate_stead_mapping_consistency` requiring `root.kind == World`. That contradicted the intended save-game ontology where the **Scenario SimThing** is the serializable file root and metadata lives on that root.

## Canonical Scenario-root model

```text
Scenario SimThing                    ← serializable file root (this PR)
├── metadata properties on root      ← scenario_id, schema_version, source, seed, shape
├── (optional) World spatial subtree ← transitional until GameSession PR
└── GameSession child                ← deferred to SCENARIO-GAMESESSION-CHILD-0

structural_grid + links              ← remain spec envelope fields (STEAD authority)
```

## Sidecar-to-property migration decision

| Field | Canonical authority | Transitional sidecar |
|---|---|---|
| `scenario_id` | `SCENARIO_ID_PROPERTY_ID` on Scenario root | `SimThingScenarioSpec.scenario_id` (serde mirror) |
| schema version | `SCENARIO_SCHEMA_VERSION_PROPERTY_ID` | — |
| provenance.source | `SCENARIO_SOURCE_LABEL_PROPERTY_ID` | `provenance.source` |
| provenance.generator_shape | `SCENARIO_GENERATOR_SHAPE_PROPERTY_ID` | `provenance.generator_shape` |
| provenance.generator_seed | `SCENARIO_GENERATOR_SEED_PROPERTY_ID` (u64 as two f32) | `provenance.generator_seed` |

String metadata uses length-prefixed UTF-8 bytes encoded in `PropertyValue.data` (documented in `scenario.rs` helpers). `serialize_scenario_authority` syncs sidecar mirrors from root metadata before write.

**Status:** PARTIAL sidecar deprecation — sidecars remain for legacy IO compatibility; canonical reads use `canonical_scenario_id()` and root properties.

## Legacy World-root compatibility decision

| Path | API | Outcome |
|---|---|---|
| Canonical | `validate_scenario_root_authority(..., Canonical)` | Requires `SimThingKind::Scenario` + metadata properties |
| Legacy | `validate_legacy_world_root_compatibility` | Returns `LegacyWorldRootAdmitted` (named marker) when `root.kind == World` and sidecar `scenario_id` present |
| Deserialize | `deserialize_scenario_authority` | Branches on root kind; rejects arbitrary non-Scenario/non-World roots |

Terran Pirate (`scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` and Studio fixture copy) remains **legacy/lower-layer golden fixture only** — not canonical Scenario shape.

## Fixture list

| Fixture | Role |
|---|---|
| `scenarios/corpus/minimal_scenario_root.simthing-scenario.json` | Canonical minimal Scenario-root corpus sample |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Legacy World-root golden fixture (unchanged) |
| `crates/simthing-mapeditor/tests/fixtures/terran_pirate_skeleton.simthing-scenario.json` | Legacy Studio golden fixture (unchanged) |

## Tests added/changed

| Test | Location |
|---|---|
| `scenario_root_roundtrips_metadata_properties` | `crates/simthing-spec/tests/scenario_serializable_simthing_root.rs` |
| `scenario_root_is_canonical_authority` | same |
| `scenario_root_fixture_deserializes_from_corpus` | same |
| `legacy_world_root_fixture_uses_explicit_compatibility_path` | same |
| `scenario_sidecar_metadata_is_transitional_not_authority` | same |
| `scenario_root_rejects_missing_metadata_if_canonical_mode_requires_it` | same |
| `scenario_root_does_not_accept_arbitrary_non_scenario_root` | same |
| `canonical_serialize_prefers_scenario_kind_in_json` | same |
| Internal scenario STEAD tests migrated to Scenario→World wrapper | `crates/simthing-spec/src/spec/scenario.rs` |
| e10 scenario-root guards extended | `crates/simthing-spec/tests/e10_resource_flow_admission.rs` |

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| `SimThingKind::Scenario` | Added in `simthing-core` + `SimThingKindTag` | PASS |
| Metadata on Scenario root properties | Property IDs + sync helpers | PASS |
| Canonical Scenario-root validation | `validate_scenario_root_authority` | PASS |
| Legacy World-root explicit compatibility | `validate_legacy_world_root_compatibility` | PASS |
| Minimal Scenario-root fixture + roundtrip | Corpus fixture + tests | PASS |
| Sidecar fields transitional (not canonical) | Documented + mismatch guards | PASS (PARTIAL deprecation) |
| GameSession child requirement | Not validated | DEFERRED |
| Owner entities / GalaxyMap tree | Not in scope | DEFERRED |
| Studio full tree edit/display | IO preserved; Terran Pirate loads | PASS (compat only) |
| No runtime Scenario engine | No engine added | PASS |
| No GPU changes | No GPU diff | PASS |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS (72/72) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec -j 1` | PASS (all integration + unit tests) |
| `cargo test -p simthing-spec --test scenario_serializable_simthing_root` | PASS (8/8) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_owner_doctrine_and_evidence_reclassification_guards` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS (10/10, 1 ignored) |
| `git diff --check` | PASS |

**Changed paths (vs `master`):**

- `crates/simthing-core/src/simthing.rs`
- `crates/simthing-core/src/property.rs`
- `crates/simthing-sim/src/fission.rs`
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/scenario_serializable_simthing_root.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `scenarios/corpus/minimal_scenario_root.simthing-scenario.json`
- `docs/tests/scenario_serializable_simthing_root_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deferred next rung

**SCENARIO-GAMESESSION-CHILD-0** — require exactly one GameSession child under Scenario; running game root is no longer World.

## DA status

User design-authority correction for Scenario-as-file-root applied. No broad DA closure of full Scenario → GameSession → Owners → GalaxyMap implementation.
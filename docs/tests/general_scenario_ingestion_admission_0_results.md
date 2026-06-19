# GENERAL-SCENARIO-INGESTION-ADMISSION-0 — arbitrary Scenario ingestion and typed admission

> **Lifecycle: PROBATION** — Generic ingestion API, typed classification/deferrals, corpus fixtures, and driver compile-readiness reuse landed. Studio ingestion UI deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #783 — GENERAL-SCENARIO-INGESTION-ADMISSION-0  
**Merge SHA:** `397a048d`  
**Base:** `master` after PR #782 / STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 (`b4037fc9`)

## Current defect summary

PR #782 made canonical Scenario trees loadable/displayable in Studio, but scenario admission remained fixture-driven with no generic ingestion API classifying arbitrary files as Admitted / PartiallyAdmitted / Rejected / Unsupported with typed deferrals.

## Ingestion API location and boundary decision

| Crate | Responsibility |
|---|---|
| `simthing-spec` | `scenario_ingestion.rs`: parse JSON, validate canonical tree, classify owners/map/structural surfaces, typed deferrals/errors |
| `simthing-driver` | `scenario_ingestion_compile.rs`: `evaluate_scenario_compile_readiness` via existing `compile_structural_n4_theater` |
| `simthing-mapeditor` | Not ingestion authority (Studio display deferred) |
| `simthing-gpu` | Not touched |

## Classification model

```rust
ScenarioIngestionClassification::{Admitted, PartiallyAdmitted, Rejected, Unsupported}
```

- **Rejected** — invalid authority (missing GameSession, Owner, GalaxyMap, duplicate owner ids, bad map_container_id, serde failure).
- **Admitted** — valid canonical tree, no feature deferrals.
- **PartiallyAdmitted** — valid tree with typed feature deferrals (capability tree, unknown gridcell role, legacy World-root).
- **Unsupported** — valid tree whose primary gap is unsupported-but-valid features (e.g. planet-only additions).

## Typed deferral model

`ScenarioDeferralKind` includes: `LegacyWorldRootCompatibility`, `PlanetsNotYetAdmitted`, `OwnerResourceFlowNotYetExecuted`, `CapabilityTreeNotYetExecuted`, `StudioStructuralPlacementEditNotYetSupported`, `MappingPlanCompileDeferred`, `GpuResidentExecutionDeferred`, `UnsupportedGridcellRole`, `UnsupportedChildLocationDepth`.

Each deferral carries path/SimThing id, reason, `scenario_remains_valid`, and `compile_can_continue`.

## Corpus fixture list

| Fixture | Purpose |
|---|---|
| `minimal_scenario_root.simthing-scenario.json` | Canonical valid (empty grid) |
| `minimal_scenario_galaxymap.simthing-scenario.json` | Canonical spatial |
| `invalid_missing_gamesession.simthing-scenario.json` | Reject |
| `invalid_missing_owner.simthing-scenario.json` | Reject |
| `invalid_duplicate_owner_ids.simthing-scenario.json` | Reject |
| `invalid_missing_galaxymap.simthing-scenario.json` | Reject |
| `invalid_bad_map_container.simthing-scenario.json` | Reject |
| `unsupported_planet_child_valid_schema.simthing-scenario.json` | Unsupported deferral |
| `unsupported_unknown_gridcell_role.simthing-scenario.json` | PartiallyAdmitted deferral |
| `legacy_world_root_terran_pirate_reference.txt` | Path reference to Terran Pirate fixture |

## Spec validation coverage

12 tests in `scenario_ingestion_admission.rs` — all PASS.

## Driver/GPU-resident compile-readiness coverage

4 tests in `scenario_ingestion_compile_readiness.rs` — structural N4 admit/deferral and mapping-plan deferred reporting. No new GPU primitives.

## Legacy Terran Pirate compatibility status

**PASS** — ingested as `PartiallyAdmitted` with `LegacyWorldRootCompatibility` deferral; not promoted to canonical ontology.

## Studio boundary status

**SKIP** — `StudioIngestionReportDisplayDeferred`; Studio does not own ingestion authority or GPU dispatch.

## PR #782 evidence lifecycle cleanup

**PASS** — `#782` / `b4037fc9` already recorded in evidence index; `studio_scenario_load_save_display_0_results.md` validation table has explicit PASS statuses.

## Production synthesis update summary

Added § GENERAL-SCENARIO-INGESTION-ADMISSION-0; reprioritized Next Production Rungs (resource-flow silos, Studio ingestion UI, GPU execution).

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| Generic ingestion API | `ingest_scenario` / `ingest_scenario_from_str` | PASS |
| Four-way classification | `ScenarioIngestionClassification` | PASS |
| Typed deferrals | `ScenarioDeferralKind` | PASS |
| Invalid canonical rejection | corpus + tests | PASS |
| Unsupported valid features | planet/unknown role fixtures | PASS |
| Driver compile readiness | `evaluate_scenario_compile_readiness` | PASS |
| Terran Pirate legacy only | reference + deferral | PASS |
| Studio ingestion UI | not implemented | SKIP (deferred) |
| Resource-flow silos | not implemented | SKIP (deferred) |
| New GPU primitives | none | PASS |

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
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12/12) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test canonical_galaxymap_mapping_compile` | PASS (2/2) |
| `cargo test -p simthing-driver --test scenario_ingestion_compile_readiness` | PASS (4/4) |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS (10/10) |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS (10/10, 1 ignored) |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | PASS (19 files) |

## Files changed

| Path | Change |
|---|---|
| `crates/simthing-spec/src/spec/scenario_ingestion.rs` | Ingestion API |
| `crates/simthing-spec/src/spec/mod.rs` | Exports |
| `crates/simthing-spec/src/lib.rs` | Exports |
| `crates/simthing-spec/tests/scenario_ingestion_admission.rs` | 12 tests |
| `crates/simthing-spec/tests/e10_resource_flow_admission.rs` | Ingestion guards |
| `crates/simthing-driver/src/scenario_ingestion_compile.rs` | Compile readiness |
| `crates/simthing-driver/src/lib.rs` | Module export |
| `crates/simthing-driver/tests/scenario_ingestion_compile_readiness.rs` | 4 tests |
| `scenarios/corpus/*.simthing-scenario.json` | Invalid/unsupported fixtures |
| `scenarios/corpus/legacy_world_root_terran_pirate_reference.txt` | Legacy reference |
| `docs/tests/general_scenario_ingestion_admission_0_results.md` | This report |
| `docs/tests/current_evidence_index.md` | PROBATION row |
| `docs/0.8.3 Simthing Studio Production.md` | § GENERAL-SCENARIO-INGESTION-ADMISSION-0 |

## Deleted/archived artifacts

None.

## Deferred next rung recommendation

1. **SESSION-RESOURCE-FLOW-SILOS-0**
2. Studio ingestion report display / admission UI
3. GPU-resident execution after admitted authority enforcement

## DA status

**PROBATION** — pending owner/DA approval. No DA promotion claimed.
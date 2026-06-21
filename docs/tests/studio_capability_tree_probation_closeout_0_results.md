# STUDIO-CAPABILITY-TREE-PROBATION-CLOSEOUT-0 Results

## Status

PASS — complete closeout (23/23 rows promoted)

## PR / branch / merge

- Branch: `studio-capability-tree-probation-closeout-0`
- PR: #855
- Merge SHA: `fcf39b93846aacde1cfb9e52e0caf0fa50067f2a`

## Mission

Close parent capability-tree PROBATION rows now covered by the DA-approved Scenario Runtime + Save/Load closing track. Lifecycle consolidation only — no code, schema, or runtime changes.

## Rows reviewed for closure

All 23 listed parent capability rows were reviewed against existing result reports and the DA-approved child evidence chain.

## Rows promoted

All 23 listed rows promoted to **CURRENT_EVIDENCE — DA-APPROVED 2026-06-20 (executive DA, CAPABILITY-TREE-CLOSEOUT-0)**.

## Rows left PROBATION

None. All listed rows had existing result reports and are substantiated by the DA-approved child ladder (#828–#854).

## Evidence chain used

Parent rows closed by reference to the DA-approved Scenario Runtime + Save/Load closing track and recursively reviewed RF/property-view ladder — not by rerunning implementation.

## DA-approved child ladder

SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 (#828), SCENARIO-STEAD-MAP-ROUNDTRIP-0 (#834), LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 (#836), LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 (#838), LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 (#840), SCENARIO-CANDIDATE-FROM-RUNTIME-0 (#842), SCENARIO-CANDIDATE-SAVE-REOPEN-0 (#844), SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 (#845), STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 (#846), STUDIO-CANDIDATE-REOPEN-ADOPT-0 (#847), SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 (#848), SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 (#851), SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0 (#852), STUDIO-PRODUCTION-POST-DA-CLOSURE-DOCS-0 (#854).

## ScenarioSpec authority baseline

ScenarioSpec remains the only serialized scenario authority. Studio UI state, Bevy ECS state, runtime reports, property-view rows, and GPU buffers remain non-authoritative.

## Corrected spatial hierarchy baseline

```text
Serializable Scenario SimThing container
  -> GameSession SimThing
     -> Owner gridcells / Owner SimThings as GameSession children
     -> GalaxyGridcell SimThing
        -> child grid containing:
           -> StarMap gridcell SimThing
           -> inert 1×1 gridcell SimThings in the same galaxy grid
              -> child grid containing:
                 -> Planet gridcell SimThing
                 -> inert 1×1 gridcell SimThings in the same starmap/system grid
                    -> child grid containing:
                       -> 1×1 surface gridcell SimThing
                          -> pop cohorts / fleets / buildings / infrastructure / leaders
                          -> other non-grid gameplay child SimThings
```

## Accumulator Flow / GPU-residency baseline

Accumulator Flow is the generic accumulation substrate. RF is a domain expression of Accumulator Flow, not a separate economy engine. Accumulator Flow works over SimThing row/table surfaces and remains GPU-resident in shape. CPU paths in these evidence rows are oracle/reference/proof/reporting/serialization/file-IO only, not production simulation authority.

The recursive RF / property-view ladder is closed as evidence for GPU-compatible Accumulator Flow row/table shape and authority boundaries, not as evidence of final GPU dispatch. GPU dispatch remains deferred.

## Studio capability baseline

Parent rows now record current evidence for: ScenarioSpec save/load authority, backend scenario file IO, Studio save/load controls, scenario-native loaded sessions, Studio projections from ScenarioSpec authority, general scenario ingestion/admission, owner-silo RF, and the recursive spatial-tree RF/property-view ladder through scenario-candidate boundary.

## Deferred boundaries

Still deferred: replace-existing candidate save / overwrite confirmation; persistent history / timeline as product feature; GPU dispatch / WGSL execution; pathfinding; combat; economy execution; fleet movement / supply; non-canonical savefile format.

## Evidence lifecycle and cleanup

No prior result reports deleted. No live evidence rows deleted. Unrelated PROBATION rows untouched. New evidence in this file.

## Boundary / non-goals

This PR is not a new proof ladder. It closes parent capability-tree PROBATION rows by tying them to the DA-approved Scenario Runtime + Save/Load closing track and the recursively reviewed RF/property-view ladder. No code, schema, runtime behavior, GPU dispatch, or Studio workflow is changed.

## Closure table

| Parent capability row | Previous lifecycle | New lifecycle | Evidence chain | Notes |
|---|---|---|---|---|
| SAVELOAD-AUTHORITY-PIN-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #828–#854 | `saveload_authority_pin_0_results.md` |
| SAVELOAD-AUTHORITY-PIN-0R | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #828–#854 | `saveload_authority_pin_0r_results.md` |
| SCENARIO-SAVELOAD-IO-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #828–#854 | `scenario_saveload_io_0_results.md` |
| SCENARIO-SAVELOAD-UI-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #828–#854 | `scenario_saveload_ui_0_results.md` |
| SCENARIO-NATIVE-SESSION-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #828–#854 | `scenario_native_session_0_results.md` |
| STUDIO-SIMTHING-SPEC-BOUNDARY-1 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #828–#854 | `studio_simthing_spec_boundary_1_results.md` |
| GENERAL-SCENARIO-INGESTION-ADMISSION-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #828–#854 | `general_scenario_ingestion_admission_0_results.md` |
| SESSION-RESOURCE-FLOW-SILOS-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #838–#854 | `session_resource_flow_silos_0_results.md` |
| RECURSIVE-LOCAL-RF-EVALUATOR-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `recursive_local_rf_evaluator_0_results.md` |
| RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `recursive_local_rf_gpu_residency_remediation_0r_results.md` |
| PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `planet_child_recursive_rf_reconciliation_0_results.md` |
| RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `runtime_tick_recursive_rf_source_0_results.md` |
| RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `runtime_tick_recursive_rf_selectable_source_0_results.md` |
| OWNER-SILO-RECURSIVE-RF-SOURCE-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `owner_silo_recursive_rf_source_0_results.md` |
| LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `local_allocation_recursive_rf_source_0_results.md` |
| LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `local_effect_application_recursive_rf_source_0_results.md` |
| SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `semantic_local_effects_recursive_rf_source_0_results.md` |
| SEMANTIC-EFFECT-EXECUTION-BOUNDARY-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `semantic_effect_execution_boundary_0_results.md` |
| SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `semantic_participant_delta_preview_0_results.md` |
| RUNTIME-PARTICIPANT-STATE-MUTATION-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `runtime_participant_state_mutation_0_results.md` |
| RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `runtime_participant_property_mutation_boundary_0_results.md` |
| SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 | PROBATION | CURRENT_EVIDENCE — DA-APPROVED (CAPABILITY-TREE-CLOSEOUT-0) | #826–#854 | `scenario_property_mutation_authority_boundary_0_results.md` |

## Validation

| Command | Result |
|---|---|
| `cargo fmt -p simthing-spec -p simthing-driver -p simthing-mapeditor -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `git diff --check` | PASS |
| Placeholder guard | PASS |
| Alias guard | PASS |
| Unrelated-row guard | PASS |
| Doc/evidence guards | PASS |

## Files changed

- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/studio_capability_tree_probation_closeout_0_results.md`

## Known gaps

- Next production track still pending project-owner selection.
- Approved deferrals (replace-existing save, history, GPU dispatch, etc.) remain open.
- Unrelated PROBATION rows (mapgen, Terran Pirate, GPU backend, etc.) intentionally untouched.

## Next recommended action

Project owner selects the next production track. Do not reopen Scenario Runtime + Save/Load or rerun parent capability proofs unless regression review is explicitly requested.
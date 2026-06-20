# LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `loaded-scenario-runtime-report-chain-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Implement Closing Track Rung 4: attach the full recursive runtime report chain to loaded ScenarioSpec sessions, composing #838 recursive RF runtime and prior landed runtime ladder helpers.

## Constitution / ADR alignment

Aligns with 0.0.8.3+ design, ScenarioSpec authority, STEAD contract, AccumulatorOp v2, Resource Flow substrate, and sparse spatial mapping doctrine. CPU work is oracle/reference/shadow/report formatting only.

## Recursive RF runtime baseline

Composes `evaluate_loaded_scenario_recursive_rf_runtime_from_json_str` (#838). Canonical IO (#828), STEAD roundtrip (#834), and session envelope (#836) are reused through that path.

## Owner-silo / disburse-down stage

`evaluate_owner_silo_disburse_down_with_rf_source` with `RecursiveLocalRfSelectable` reports disburse results on owner_silo corpus fixture.

## Local allocation stage

`evaluate_runtime_local_allocation_with_rf_source` with `RecursiveOwnerSiloSelectable` reports allocation rows.

## Local effects stage

`evaluate_local_effect_application_with_rf_source` with `RecursiveLocalAllocationSelectable` reports application records.

## Semantic projection stage

`evaluate_semantic_local_effects_with_rf_source` with `RecursiveLocalEffectSelectable` reports semantic outputs.

## Semantic execution records stage

`evaluate_semantic_effect_execution_boundary` with `RecursiveSemanticLocalEffectsSelectable` reports execution records.

## Semantic delta preview stage

`evaluate_semantic_participant_delta_preview` with `RecursiveSemanticExecutionSelectable` reports delta preview records.

## Runtime participant state rows stage

`evaluate_runtime_participant_state_mutation` with `RecursiveDeltaPreviewSelectable` reports ephemeral runtime state mutation rows.

## Runtime participant property-view rows stage

`evaluate_runtime_participant_property_mutation_boundary` with `RecursiveRuntimeStateSelectable` reports runtime property-view rows.

## GPU-compatible row/table surface

`gpu_compatible_row_table_surface` composes #838 recursive RF runtime row/table target.

## CPU oracle-only boundary

`cpu_oracle_only` is true; no production simulation authority on CPU path.

## Explicit report-mode-only boundary

`explicit_runtime_report_mode_only` is true; all chain stages are `report_only` with `mutation_deferred`.

## Scenario authority preservation

`prove_loaded_scenario_runtime_report_chain_preserves_authority` composes recursive RF and property-view boundary authority proofs.

## Candidate mutation deferral

Candidate ScenarioSpec mutation is not performed; `scenario_authority_mutation_deferred` remains true.

## Savefile / persistent history / UI / GPU dispatch deferrals

Savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows were deleted. No scratch result reports beyond this canonical rung report were retained.

## Boundary / non-goals

No ScenarioSpec mutation, candidate mutation, savefile/history writes, Studio UI wiring, or GPU dispatch. No fixture edits.

This rung is not another hygiene-only wrapper. It moves the Scenario Runtime + Save/Load Closing Track forward by attaching the full recursive runtime report chain to loaded ScenarioSpec sessions. It composes prior landed runtime reports into a single loaded-scenario report surface while preserving ScenarioSpec as authority and keeping candidate mutation, savefile persistence, Studio UI wiring, and GPU dispatch deferred.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver -- --check` | PASS |
| `cargo test -p simthing-spec --test loaded_scenario_runtime_report_chain` | PASS (18) |
| `cargo test -p simthing-driver --test loaded_scenario_runtime_report_chain` | PASS (9) |
| `cargo test -p simthing-spec --test loaded_scenario_recursive_rf_runtime` | PASS (15) |
| `cargo test -p simthing-spec --test runtime_participant_state_mutation` | PASS (14) |
| `cargo test -p simthing-spec --test runtime_participant_property_mutation_boundary` | PASS (14) |
| Regression (#828/#834/#836) | PASS |
| `git diff --check` | PASS |
| Alias deletion guard | PASS |

## Files changed

- `crates/simthing-spec/src/spec/loaded_scenario_runtime_report_chain.rs` (new)
- `crates/simthing-spec/tests/loaded_scenario_runtime_report_chain.rs` (new)
- `crates/simthing-driver/src/loaded_scenario_runtime_report_chain_compile.rs` (new)
- `crates/simthing-driver/tests/loaded_scenario_runtime_report_chain.rs` (new)
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-driver/src/lib.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/loaded_scenario_runtime_report_chain_0_results.md` (this report)

## Known gaps

- Closing track Rungs 5–7 not yet implemented.
- Candidate ScenarioSpec mutation (Rung 5) is next.

## Next recommended action

Implement **SCENARIO-CANDIDATE-FROM-RUNTIME-0** (Closing Track Rung 5).
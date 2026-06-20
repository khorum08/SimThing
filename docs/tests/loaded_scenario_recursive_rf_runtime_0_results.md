# LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `loaded-scenario-recursive-rf-runtime-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Implement Closing Track Rung 3: attach recursive Accumulator RF runtime reporting to loaded ScenarioSpec spatial trees, composing #836 session envelope and reusing #828/#834 through that path.

## Constitution / ADR alignment

Aligns with 0.0.8.3+ SimThing core design, ScenarioSpec authority model, STEAD mapping contract, AccumulatorOp v2 doctrine, Resource Flow substrate doctrine, and sparse spatial mapping doctrine. CPU work is oracle/reference/shadow/report formatting only.

## Session envelope baseline

Composes `evaluate_loaded_scenario_studio_session_envelope_from_json_str` (#836). Canonical IO (#828) and STEAD map roundtrip (#834) are not reimplemented.

## Spatial tree traversal

Walks loaded ScenarioSpec Location hierarchy via `evaluate_recursive_local_rf` from GalaxyMap root downward.

## RF participant rows

`LoadedScenarioRfParticipantRow` flat rows extracted from spatial-tree participants with owner/resource/scope channel metadata on `owner_silo_disburse_down_scoped` corpus fixture.

## Parent Location arena rows

`LoadedScenarioRfParentArenaRow` reports per parent Location arena local totals and upward net flow.

## Owner/resource/scope channel rows

`LoadedScenarioRfChannelRow` keyed by owner/resource/scope at each parent Location arena.

## Local parent-node resolution proof

`local_parent_node_resolution_first` proves local arena settlement (`locally_matched_total == min(total_surplus, total_demand)`) before upward reduction.

## Sibling settlement before upward bubbling proof

`sibling_settlement_before_upward_bubbling` proves sibling surplus/deficit settles within parent arena before net upward bubbling.

## GPU-compatible row/table surface

`gpu_compatible_row_table_surface` reports flat participant, parent-arena, and channel row tables suitable for GPU residency lowering.

## CPU oracle-only boundary

`cpu_oracle_only` is true; no production simulation authority on CPU path.

## Scenario authority preservation

`prove_loaded_scenario_recursive_rf_runtime_preserves_authority` composes recursive local RF authority proof; ScenarioSpec digest unchanged.

## Mutation / persistence / UI / GPU dispatch deferrals

Runtime mutation, semantic execution, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows were deleted. No scratch result reports were created beyond this canonical rung report. New result evidence lives in `docs/tests/loaded_scenario_recursive_rf_runtime_0_results.md`.

## Boundary / non-goals

No ScenarioSpec mutation, semantic execution, savefile/history writes, Studio UI wiring, or GPU dispatch. No new GPU primitives/WGSL. No fixture edits.

This rung is not another hygiene-only wrapper. It moves the Scenario Runtime + Save/Load Closing Track forward by attaching recursive Accumulator RF runtime reporting to loaded ScenarioSpec spatial trees. It produces GPU-compatible parent-arena, participant, and owner/resource/scope channel rows while proving local parent-node settlement before upward bubbling and preserving ScenarioSpec as authority.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver -- --check` | PASS |
| `cargo test -p simthing-spec --test loaded_scenario_recursive_rf_runtime` | PASS (15) |
| `cargo test -p simthing-driver --test loaded_scenario_recursive_rf_runtime` | PASS (9) |
| `cargo test -p simthing-spec --test loaded_scenario_studio_session_envelope` | PASS (11) |
| `cargo test -p simthing-spec --test scenario_stead_map_roundtrip` | PASS (10) |
| `cargo test -p simthing-spec --test scenario_canonical_io` | PASS (7) |
| `cargo test -p simthing-driver --test loaded_scenario_studio_session_envelope` | PASS (8) |
| `cargo test -p simthing-driver --test scenario_stead_map_roundtrip` | PASS (8) |
| `cargo test -p simthing-driver --test scenario_canonical_io` | PASS (4) |
| `git diff --check` | PASS |
| Alias deletion guard | PASS |

## Files changed

- `crates/simthing-spec/src/spec/loaded_scenario_recursive_rf_runtime.rs` (new)
- `crates/simthing-spec/tests/loaded_scenario_recursive_rf_runtime.rs` (new)
- `crates/simthing-driver/src/loaded_scenario_recursive_rf_runtime_compile.rs` (new)
- `crates/simthing-driver/tests/loaded_scenario_recursive_rf_runtime.rs` (new)
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-driver/src/lib.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/loaded_scenario_recursive_rf_runtime_0_results.md` (this report)

## Known gaps

- Closing track Rungs 4–7 not yet implemented.
- Loaded scenario runtime report chain (Rung 4) is next.

## Next recommended action

Implement **LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0** (Closing Track Rung 4).
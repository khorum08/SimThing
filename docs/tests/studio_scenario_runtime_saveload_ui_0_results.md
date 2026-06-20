# STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `studio-scenario-runtime-saveload-ui-0`
- PR: #846
- Merge SHA: `83bc5810276724fc044bd0361d6cf817f8f513c8`

## Mission

Expose loaded Scenario Runtime + Save/Load workflow in Studio UI: digest, validation, recursive RF readiness, report-chain readiness, candidate readiness, Save Candidate, Reopen Candidate.

## Constitution / ADR alignment

Aligns with 0.0.8.3+ ScenarioSpec authority model. Composes landed spec/driver helpers from rungs 0–6 and #845 hardening without making UI/Bevy/runtime/GPU surfaces authoritative.

## UI status surface

`StudioScenarioRuntimeSaveLoadStatus` reports loaded digest, validation, runtime readiness, candidate readiness, and save/reopen eligibility.

## Loaded scenario digest

Composes canonical IO load report from loaded session JSON.

## Validation and projection readiness

Composes STEAD roundtrip and loaded session envelope for stead validation and projection rebuild readiness.

## Recursive RF runtime readiness

Composes `compile_loaded_scenario_recursive_rf_runtime_plan_from_json_str`.

## Runtime report-chain readiness

Composes `compile_loaded_scenario_runtime_report_chain_plan_from_json_str`.

## Candidate readiness

Composes `compile_scenario_candidate_save_reopen_plan_from_json_str`.

## Save Candidate create-new workflow

`save_candidate_scenario_for_studio_create_new` uses hardened `write_candidate_scenario_canonical_json_atomic` (#845).

## Existing target preservation

Create-new policy rejects existing targets and preserves file contents.

## Reopen Candidate workflow

`reopen_candidate_scenario_for_studio` loads canonical JSON and validates STEAD/projection readiness.

## Failed operation session preservation

Failed save/reopen commands do not mutate loaded `StudioSession`.

## Non-authority UI / Bevy / runtime / GPU surfaces

UI state, Bevy ECS, runtime reports, and GPU buffers explicitly non-authoritative.

## Persistent history / GPU dispatch deferrals

Persistent history and GPU dispatch remain deferred.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows may be deleted. Scratch or duplicate result reports created during this PR must be deleted before merge unless referenced by current_evidence_index.md. New result evidence for this rung must live in docs/tests/studio_scenario_runtime_saveload_ui_0_results.md.

## Boundary / non-goals

No distinct savefile format, persistent history, GPU dispatch, combat, pathfinding, or fixture edits.

This rung is not another hygiene-only wrapper. It completes the Studio-visible Scenario Runtime + Save/Load workflow by exposing loaded scenario validation, recursive runtime readiness, candidate readiness, Save Candidate, and Reopen Candidate while preserving ScenarioSpec authority and keeping UI, Bevy state, runtime reports, and GPU buffers non-authoritative.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-mapeditor -p simthing-spec -p simthing-driver -- --check` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui` | PASS (13) |
| `cargo test -p simthing-spec --test scenario_candidate_save_reopen` | PASS |
| `cargo test -p simthing-spec --test scenario_candidate_from_runtime` | PASS |
| `cargo test -p simthing-spec --test scenario_canonical_io` | PASS (7) |
| `cargo test -p simthing-driver --test scenario_candidate_save_reopen` | PASS |
| `cargo test -p simthing-driver --test scenario_candidate_from_runtime` | PASS |
| `cargo test -p simthing-driver --test scenario_canonical_io` | PASS (4) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-mapeditor/src/scenario_runtime_saveload_ui.rs`
- `crates/simthing-mapeditor/tests/studio_scenario_runtime_saveload_ui.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/Cargo.toml`

## Known gaps

Explicit replace-existing candidate save deferred until Studio overwrite confirmation flow.

## Next recommended action

SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 (Rung 8).
# STUDIO-CANDIDATE-REOPEN-ADOPT-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `studio-candidate-reopen-adopt-0`
- PR: #847
- Merge SHA: `8cefd9c8da067a4a10fe9b4f87cc712c27aaf890`

## Mission

Pre-DA remedial fix: successful Reopen Candidate adopts reopened candidate ScenarioSpec into active Studio session.

## Constitution / ADR alignment

Aligns with ScenarioSpec authority model. Composes existing canonical IO, STEAD validation, session load, and runtime status helpers from Rung 7.

## Rung 7 baseline

`STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0` (#846) exposed status surface and Save/Reopen commands.

## Defect found

`execute_reopen_candidate_action` validated candidate files but restored prior session/status even on successful reopen.

## Successful reopen adoption behavior

`reopen_candidate_scenario_for_studio_session` loads candidate via canonical IO, validates readiness, builds `StudioSession`, and returns adoption payload for UI.

## Loaded digest/status refresh

Adopted session refreshes `StudioScenarioRuntimeSaveLoadStatus` from reopened candidate authority digest.

## Failed reopen preservation

Failed reopen leaves session and runtime status unchanged except for message.

## Non-authority UI / Bevy / runtime / GPU surfaces

UI state, Bevy ECS, runtime reports, and GPU buffers remain non-authoritative.

## Persistent history / GPU dispatch deferrals

Persistent history and GPU dispatch remain deferred.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows may be deleted. New result evidence lives in `docs/tests/studio_candidate_reopen_adopt_0_results.md`.

## Boundary / non-goals

No replace-existing save, persistent history, GPU dispatch, or distinct savefile format.

This PR is not a hygiene loop. It fixes the pre-DA Reopen Candidate workflow by ensuring that a successful candidate reopen adopts the reopened candidate ScenarioSpec into the active Studio session, while failed reopen preserves the current session.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-mapeditor -- --check` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test studio_candidate_reopen_adopt` | PASS (10) |
| `cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui` | PASS (13) |
| `cargo test -p simthing-spec --test scenario_candidate_save_reopen` | PASS |
| `cargo test -p simthing-spec --test scenario_candidate_from_runtime` | PASS |
| `cargo test -p simthing-spec --test scenario_canonical_io` | PASS |
| `cargo test -p simthing-driver --test scenario_candidate_save_reopen` | PASS |
| `cargo test -p simthing-driver --test scenario_candidate_from_runtime` | PASS |
| `cargo test -p simthing-driver --test scenario_canonical_io` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-mapeditor/src/scenario_runtime_saveload_ui.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_candidate_reopen_adopt.rs`

## Known gaps

Replace-existing candidate save flow remains deferred.

## Next recommended action

SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 (Rung 8).
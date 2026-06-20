# SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 Results

## Status

PENDING — validation in progress

## PR / branch / merge

- Branch: `scenario-candidate-save-reopen-harden-0`
- PR: pending
- Merge SHA: pending

## Mission

Pre-UI remedial hardening of the Rung 6 candidate ScenarioSpec canonical JSON writer: same-directory temp file, create-new policy, existing target preserved on error, no target removal before guaranteed write.

## Constitution / ADR alignment

Aligns with 0.0.8.3+ ScenarioSpec authority model. Hardens `write_candidate_scenario_canonical_json_atomic` without changing candidate save/reopen digest proofs from #844.

## Rung 6 baseline

Composes landed `SCENARIO-CANDIDATE-SAVE-REOPEN-0` (#844) candidate save/reopen path unchanged for in-memory canonical serialization and reopen validation.

## Write semantics before hardening

Prior helper wrote a temp file, removed existing `output_path` if present, then renamed temp — creating a failure window for existing targets.

## Write semantics after hardening

Create-new policy: fail when `output_path` exists; write to same-directory uniquely suffixed temp; flush/sync; rename only after successful write; cleanup temp on failure.

## Existing target preservation

Existing file content unchanged when create-new write is attempted against an existing path.

## Same-directory temp file

Temp path derived from output filename in the output parent directory only.

## Temp cleanup behavior

Temp file removal attempted on write or rename failure.

## Candidate reopen regression

Rung 6 digest stability and reopen validation tests continue to pass.

## Canonical ScenarioSpec JSON only

No alternate savefile format introduced.

## No distinct savefile / history / UI / GPU dispatch

Persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows may be deleted. Scratch or duplicate result reports created during this PR must be deleted before merge unless referenced by current_evidence_index.md. New result evidence for this rung must live in docs/tests/scenario_candidate_save_reopen_harden_0_results.md.

## Boundary / non-goals

No Studio UI, GPU dispatch, persistent history, fixture edits, or distinct savefile format.

This PR is not a hygiene loop. It is a pre-UI safety hardening pass for the candidate ScenarioSpec canonical JSON writer landed in Rung 6. It prevents the upcoming Studio UI rung from exposing a helper that can remove an existing target before a successful write strategy is guaranteed.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver -- --check` | pending |
| `cargo test -p simthing-spec --test scenario_candidate_save_reopen` | pending |
| `cargo test -p simthing-driver --test scenario_candidate_save_reopen` | pending |

## Files changed

- `crates/simthing-spec/src/spec/scenario_candidate_save_reopen.rs`
- `crates/simthing-spec/tests/scenario_candidate_save_reopen.rs`
- `docs/tests/scenario_candidate_save_reopen_harden_0_results.md`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`

## Known gaps

Explicit replace-existing helper deferred until Studio UI overwrite confirmation flow (Rung 7).

## Next recommended action

Implement STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 (Rung 7).
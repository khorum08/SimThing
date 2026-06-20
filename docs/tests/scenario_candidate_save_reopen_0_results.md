# SCENARIO-CANDIDATE-SAVE-REOPEN-0 Results

## Status

PENDING — validation in progress

## PR / branch / merge

- Branch: `scenario-candidate-save-reopen-0`
- PR: pending
- Merge SHA: pending

## Mission

Implement Closing Track Rung 6: save cloned candidate ScenarioSpec as canonical ScenarioSpec JSON, reopen it, validate STEAD/link/RF metadata/spatial tree/projection readiness, and prove candidate authority digest stability.

## Constitution / ADR alignment

Aligns with 0.0.8.3+ design and ScenarioSpec authority model. Composes #842 `evaluate_scenario_candidate_from_runtime_from_json_str` and #828 canonical IO save/load. Reuses `clone_scenario_candidate_with_runtime_property_view` for candidate materialization.

## Candidate-from-runtime baseline

Composes `evaluate_scenario_candidate_from_runtime_from_json_str` (#842) via `ScenarioCandidateSaveReopenSource::ScenarioCandidateFromRuntime`.

## Candidate canonical serialization

`save_scenario_spec_to_canonical_json` (#828) serializes cloned candidate ScenarioSpec to deterministic canonical JSON with stable authority digest.

## Atomic temp-to-rename write

`write_candidate_scenario_canonical_json_atomic` writes `.simthing-scenario.json.tmp` sibling then renames to target path. Tests use `std::env::temp_dir()` only.

## Candidate reopen

`load_scenario_spec_from_json_str` (#828) reloads saved candidate canonical JSON with ingestion admission report.

## Candidate digest stability

`candidate_authority_digest_before_save == reopened_authority_digest` after canonical save/reopen. `prove_scenario_candidate_save_reopen_digest_stability` passes.

## Original ScenarioSpec authority preservation

`prove_scenario_candidate_from_runtime_preserves_original_authority` composes #842 original-authority proof; loaded original JSON unchanged.

## STEAD / link / RF metadata / spatial tree validation

Reopened candidate validated through `evaluate_scenario_stead_map_roundtrip_from_json_str` (#834) and row extraction comparisons against pre-save candidate.

## Studio projection rebuild readiness

`evaluate_loaded_scenario_studio_session_envelope_from_json_str` (#836) reports `studio_projection_rebuild_ready` for reopened candidate JSON.

## Canonical ScenarioSpec JSON only

`canonical_scenario_json_only` true; candidate artifact uses #828 canonical serialization only.

## No distinct savefile format

`no_distinct_savefile_format_introduced` true; no persistent history or alternate savefile schema.

## Persistent history / UI / GPU dispatch deferrals

Persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows may be deleted. Scratch or duplicate result reports created during this PR must be deleted before merge unless referenced by current_evidence_index.md. Superseded result reports may be moved to an archive only if the PR explains the supersession and updates all references. New result evidence for this rung must live in docs/tests/scenario_candidate_save_reopen_0_results.md.

## Boundary / non-goals

No Studio UI wiring, GPU dispatch, persistent history, fixture edits, or distinct savefile format.

This rung is not another hygiene-only wrapper. It moves the Scenario Runtime + Save/Load Closing Track forward by saving the cloned candidate ScenarioSpec as canonical ScenarioSpec JSON, reopening it, and proving digest, STEAD, link, RF metadata, spatial tree, and projection readiness stability. Studio UI wiring remains deferred to the next rung.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver -- --check` | pending |
| `cargo test -p simthing-spec --test scenario_candidate_save_reopen` | pending |
| `cargo test -p simthing-driver --test scenario_candidate_save_reopen` | pending |

## Files changed

- `crates/simthing-spec/src/spec/scenario_candidate_save_reopen.rs`
- `crates/simthing-spec/tests/scenario_candidate_save_reopen.rs`
- `crates/simthing-driver/src/scenario_candidate_save_reopen_compile.rs`
- `crates/simthing-driver/tests/scenario_candidate_save_reopen.rs`
- `docs/tests/scenario_candidate_save_reopen_0_results.md`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`

## Known gaps

Studio UI save/reopen workflow deferred to Rung 7 (STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0).

## Next recommended action

Merge this rung, then implement STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 (Rung 7).
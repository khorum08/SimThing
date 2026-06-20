# SCENARIO-CANDIDATE-FROM-RUNTIME-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `scenario-candidate-from-runtime-0`
- PR: #842
- Merge SHA: `d66fbd856c32cfc88a85953e10455e20824d74b2`

## Mission

Implement Closing Track Rung 5: generate cloned candidate ScenarioSpec from loaded runtime property-view rows while preserving loaded original authority.

## Constitution / ADR alignment

Aligns with 0.0.8.3+ design and ScenarioSpec authority model. Reuses landed `scenario_property_mutation_authority_boundary` (`clone_scenario_candidate_with_runtime_property_view`) rather than duplicating candidate mutation logic.

## Runtime report chain baseline

Composes `evaluate_loaded_scenario_runtime_report_chain_from_json_str` (#840).

## Candidate clone construction

`clone_scenario_candidate_with_runtime_property_view` clones loaded ScenarioSpec and applies recursive runtime property-view rows to candidate only.

## Runtime property-view source rows

Sourced from #840 chain `runtime_property_view_rows_ready` stage via landed property mutation boundary.

## Candidate property mutation records

`ScenarioCandidatePropertyMutationRecord` maps landed boundary records with participant id, property id, owner/resource/scope metadata, and before/runtime/after values.

## Original ScenarioSpec authority preservation

`original_authority_digest_before == original_authority_digest_after` on loaded original; `prove_scenario_candidate_from_runtime_preserves_original_authority` passes.

## Candidate authority digest change

`candidate_authority_changed` true when mutation records exist; candidate digest differs from original while original digest remains stable.

## STEAD / link / spatial tree preservation

Candidate STEAD ID, link, and spatial tree rows match original loaded scenario after preview-property mutation.

## Owner metadata vs spatial parentage

`owner_metadata_not_spatial_parentage` composes STEAD roundtrip proof from loaded JSON.

## GPU-compatible source rows

`gpu_compatible_source_rows` composes runtime property-view rows from #840 GPU-compatible chain surface.

## CPU candidate-serialization-only boundary

`cpu_candidate_serialization_only` true; candidate materialization is authority-boundary serialization, not production simulation.

## Candidate save / savefile / history / UI / GPU dispatch deferrals

Candidate save, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows were deleted. No scratch result reports beyond this canonical rung report were retained.

## Boundary / non-goals

No original mutation, candidate save, savefile/history, Studio UI, or GPU dispatch. No fixture edits.

This rung is not another hygiene-only wrapper. It moves the Scenario Runtime + Save/Load Closing Track forward by materializing a cloned candidate ScenarioSpec from loaded runtime property-view rows while proving that the loaded original ScenarioSpec remains unchanged. Candidate save/reopen is intentionally deferred to the next rung.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver -- --check` | PASS |
| `cargo test -p simthing-spec --test scenario_candidate_from_runtime` | PASS (17) |
| `cargo test -p simthing-driver --test scenario_candidate_from_runtime` | PASS (9) |
| `cargo test -p simthing-spec --test loaded_scenario_runtime_report_chain` | PASS (18) |
| `cargo test -p simthing-spec --test scenario_property_mutation_authority_boundary` | PASS (12) |
| Regression (#828/#834/#836/#838) | PASS |
| `git diff --check` | PASS |
| Alias deletion guard | PASS |

## Files changed

- `crates/simthing-spec/src/spec/scenario_candidate_from_runtime.rs` (new)
- `crates/simthing-spec/tests/scenario_candidate_from_runtime.rs` (new)
- `crates/simthing-driver/src/scenario_candidate_from_runtime_compile.rs` (new)
- `crates/simthing-driver/tests/scenario_candidate_from_runtime.rs` (new)
- `crates/simthing-spec/src/spec/scenario_property_mutation_authority_boundary.rs` (reuse helper)
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-driver/src/lib.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/scenario_candidate_from_runtime_0_results.md` (this report)

## Known gaps

- Closing track Rungs 6–7 not yet implemented.
- Candidate save/reopen (Rung 6) is next.

## Next recommended action

Implement **SCENARIO-CANDIDATE-SAVE-REOPEN-0** (Closing Track Rung 6).
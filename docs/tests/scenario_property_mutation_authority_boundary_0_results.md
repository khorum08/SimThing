# SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `scenario-property-mutation-authority-boundary-0`
- PR: #826
- Merge SHA: `90de96070376cb86a12d78c80bb3c5857b2bae8f`

## Mission

Apply recursive-source runtime participant property-view rows to a cloned ScenarioSpec candidate behind explicit source mode without mutating the input ScenarioSpec, savefiles, fixtures, or persistent history.

## Pre-flight metadata check

- #824/#825 metadata verified on `master`: PR #824, merge `633fb0c06a6b4ed877d4dbb4b030c6ce9c17ade7`; no `TBD` placeholders for #824 in evidence index, result report, or production doc.
- #824 proved runtime property-view rows without ScenarioSpec SimThing property mutation. This rung applies those rows to a cloned candidate only; input ScenarioSpec unchanged.

## Anti-loop production-path statement

This rung intentionally crosses the Scenario property mutation boundary only on a cloned candidate ScenarioSpec. Recursive-source runtime participant property-view rows now apply to deterministic candidate ScenarioSpec property records behind explicit source mode. The input ScenarioSpec, savefiles, fixtures, and persistent history remain unchanged. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target.

## GPU-residency doctrine preservation

- Reuses runtime participant property mutation boundary (#824) and prior recursive RF ladder surfaces.
- Candidate property mutation represented as flat GPU-compatible row/table data on cloned ScenarioSpec values inside report path only.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.

## Cloned ScenarioSpec mutation model

- Added `ScenarioPropertyMutationSourceMode`, `ScenarioPropertyMutationRecord`, `ScenarioPropertyMutationAuthorityBoundaryReport`.
- Added `evaluate_scenario_property_mutation_authority_boundary`, `prove_scenario_property_mutation_boundary_preserves_original_authority`, and `replay_scenario_property_mutation_authority_boundary`.
- Driver: `compile_scenario_property_mutation_authority_boundary_plan` composes property mutation boundary plan and scenario authority boundary report.
- Scenario ingestion: `scenario_property_mutation_boundary_ready` / `scenario_property_mutation_boundary_deferred` readiness flags.

## Candidate property mutation records

- `mutation_records` capture before/runtime-view/candidate-after per applied property-view row.
- `owner_ref`, `resource_key`, `scope_id`, and string `property_id` preserved for traceability.
- Candidate `SimThing.properties` receive reserved preview `SimPropertyId` values as `PropertyValue { data: vec![f32] }`.

## Original authority preservation proof

- `prove_scenario_property_mutation_boundary_preserves_original_authority` — PASS.
- Input ScenarioSpec authority digest before == after.

## Candidate authority digest proof

- `candidate_after_authority_digest` differs from `original_before_authority_digest` when mutation records exist — PASS.

## Replay determinism proof

- `replay_scenario_property_mutation_authority_boundary` with bounded replay_count (1..=64) — PASS.

## Savefile / persistent history deferral

- `input_scenario_property_mutation_deferred`, `savefile_mutation_deferred`, and `persistent_history_deferred` true on report and records.

## Resource-key / generic channel note

- Typed recursive RF metadata preserved in mutation records; candidate property writes still use `generic` writeback alignment from property-view rows.
- Typed semantic mutation channels remain deferred.

## Prior ladder preservation proof

- Default runtime RF tick, tick shell, local effect application, and semantic local effects compile paths unchanged — PASS.
- Legacy default semantic path remains preserved.

## Boundary / non-goals

- No input ScenarioSpec mutation.
- No savefile or persistent timeline mutation.
- No fixture writes.
- No new GPU primitive, WGSL, or fused recursive RF kernel.
- No economy/combat/movement/route/pathfinding/Studio GPU dispatch.

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test scenario_property_mutation_authority_boundary` | PASS (12/12) |
| `cargo test -p simthing-spec --test runtime_participant_property_mutation_boundary` | PASS (14/14) |
| `cargo test -p simthing-spec --test runtime_participant_state_mutation` | PASS (14/14) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test scenario_property_mutation_authority_boundary` | PASS (10/10) |
| `cargo test -p simthing-driver --test runtime_participant_property_mutation_boundary` | PASS (10/10) |
| `cargo test -p simthing-driver --test runtime_participant_state_mutation` | PASS (10/10) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/scenario_property_mutation_authority_boundary.rs`
- `crates/simthing-spec/src/spec/scenario.rs` (preview SimPropertyId constants)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/scenario_property_mutation_authority_boundary.rs`
- `crates/simthing-driver/src/scenario_property_mutation_authority_boundary_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/scenario_property_mutation_authority_boundary.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/scenario_property_mutation_authority_boundary_0_results.md`

## Evidence lifecycle

PROBATION — pending merge and DA review.

## Known gaps

- Savefile/persistent history boundary for mutated candidate ScenarioSpec not evaluated.
- Typed semantic mutation channels remain deferred.
- Studio presentation remains deferred.

## Deferred next rung

1. Produce comprehensive new-chat digest after this rung lands.
2. Evaluate savefile/persistent history boundary for mutated candidate ScenarioSpec.
3. Typed semantic mutation channels remain deferred.
4. Studio presentation remains deferred.

## DA status

Not submitted — evidence PROBATION pending DA review.
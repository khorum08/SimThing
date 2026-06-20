# RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `runtime-participant-property-mutation-boundary-0`
- PR: #824
- Merge SHA: `633fb0c06a6b4ed877d4dbb4b030c6ce9c17ade7`

## Mission

Apply runtime state rows to runtime-only participant property view without Scenario/savefile persistence.

## Pre-flight metadata check

- #822/#823 metadata verified on `master`: PR #822, merge `4d0b1ed2722402ea138f980149b038cd4a9bb8b3`; no `TBD` placeholders for #822 in evidence index, result report, or production doc.
- #822 proved that semantic delta preview records can apply to deterministic runtime-only participant state rows without mutating Scenario authority, SimThing.properties, savefiles, or persistent history. This rung applies those runtime state rows to a runtime-only participant property view and proves the mutation boundary. ScenarioSpec SimThing.properties remain unchanged.

## Anti-loop production-path statement

This rung is not another hygiene-only comparison layer. It applies recursive-source runtime participant state rows to a runtime-only participant property view and produces before/mutation/after runtime property view reports. It still stops before ScenarioSpec SimThing property mutation, Scenario authority mutation, savefile mutation, and persistent history.

## GPU-residency doctrine preservation

- Reuses runtime participant state mutation (#822) and prior recursive RF ladder surfaces.
- Runtime property view mutation represented as flat GPU-compatible before/mutation/after rows.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.

## Runtime property mutation boundary model

- Added `RuntimeParticipantPropertyMutationSourceMode`, `RuntimeParticipantPropertyViewRow`, `RuntimeParticipantPropertyMutationBoundaryRecord`, `RuntimeParticipantPropertyMutationBoundaryReport`.
- Added `evaluate_runtime_participant_property_mutation_boundary`, `prove_runtime_participant_property_mutation_boundary_preserves_authority`, and `replay_runtime_participant_property_mutation_boundary`.
- Driver: `compile_runtime_participant_property_mutation_boundary_plan` composes state mutation plan and property boundary report.
- Scenario ingestion: `runtime_participant_property_mutation_boundary_ready` / `runtime_participant_property_mutation_boundary_deferred` readiness flags.

## Before / mutation / after property view rows

- `before_property_view_rows` initialized from runtime state before-rows at 0.0.
- `mutation_records` capture before/runtime_state/after per applied state mutation.
- `after_property_view_rows` reflect runtime state after-row values in property view.

## Target property mapping

- `runtime.preview.applied`, `runtime.preview.satisfied`, `runtime.preview.shortfall` used as runtime table property IDs only.
- Properties are not written to ScenarioSpec SimThing.properties.

## Scenario property mutation deferral proof

- `scenario_simthing_property_mutation_deferred`, `scenario_authority_mutation_deferred`, `savefile_mutation_deferred`, and `persistent_history_deferred` true on report and records.
- Default state-mutation and delta-preview compile paths unchanged.

## Replay determinism proof

- `replay_runtime_participant_property_mutation_boundary` with bounded replay_count (1..=64) — PASS.
- Identical mutation_records and after_property_view_rows across replays.

## Resource-key / generic channel note

- Typed recursive RF metadata preserved in source rows; property view records still use `generic` writeback alignment.
- Typed property mutation channels remain deferred.

## Prior ladder preservation proof

- `compile_runtime_rf_tick_plan`, `compile_runtime_tick_shell_plan`, `compile_local_effect_application_plan`, and `compile_semantic_local_effects_plan` outputs unchanged.
- Legacy default semantic path remains preserved.

## Authority preservation proof

- `prove_runtime_participant_property_mutation_boundary_preserves_authority` — PASS.
- Serialized Scenario authority before == after.

## Boundary / non-goals

- No ScenarioSpec SimThing property mutation.
- No ScenarioSpec mutation.
- No savefile or persistent timeline mutation.
- No new GPU primitive or WGSL.
- No fused recursive RF kernel.
- No Studio GPU dispatch.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test runtime_participant_property_mutation_boundary` | PASS (14 tests) |
| `cargo test -p simthing-spec --test runtime_participant_state_mutation` | PASS |
| `cargo test -p simthing-spec --test semantic_participant_delta_preview` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test runtime_participant_property_mutation_boundary` | PASS (10 tests) |
| `cargo test -p simthing-driver --test runtime_participant_state_mutation` | PASS |
| `cargo test -p simthing-driver --test semantic_participant_delta_preview` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/runtime_participant_property_mutation_boundary.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/runtime_participant_property_mutation_boundary.rs`
- `crates/simthing-driver/src/runtime_participant_property_mutation_boundary_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/runtime_participant_property_mutation_boundary.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/runtime_participant_property_mutation_boundary_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Runtime property view rows do not yet drive ScenarioSpec SimThing property mutation.
- ScenarioSpec SimThing property mutation remains deferred.
- Typed property mutation channels not yet authoritative.
- Studio presentation of property mutation boundary reports remains deferred.

## Deferred next rung

1. Evaluate controlled ScenarioSpec property mutation authority separately.
2. Only after ScenarioSpec property mutation boundary, evaluate savefile/persistent history boundary.
3. Typed semantic mutation channels remain deferred.
4. Studio presentation remains deferred.

## DA status

Not DA-promoted.

This rung intentionally moves beyond runtime state rows while still stopping before Scenario/property persistence. Recursive-source runtime participant state rows now apply to deterministic runtime-only participant property view rows behind explicit source mode. The produced before/mutation/after property view rows do not mutate ScenarioSpec SimThing.properties, Scenario authority, savefiles, or persistent history. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target.
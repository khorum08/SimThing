# RUNTIME-PARTICIPANT-STATE-MUTATION-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `runtime-participant-state-mutation-0`
- PR: #822
- Merge SHA: `4d0b1ed2722402ea138f980149b038cd4a9bb8b3`

## Mission

Apply semantic delta previews to runtime-only participant state rows without Scenario/property persistence.

## Pre-flight metadata check

- #820/#821 metadata verified on `master`: PR #820, merge `d55775e19bec5a9b783b3469b39d36708098ae77`; no `TBD` placeholders for #820 in evidence index, result report, or production doc.
- #820 proved that recursive-source semantic execution records can produce runtime-only participant property delta previews without mutation. This rung applies those preview deltas to an ephemeral runtime participant state table only. Participant SimThing properties, Scenario authority, savefiles, and persistent history remain unchanged.

## Anti-loop production-path statement

This rung is not another hygiene-only comparison layer. It applies recursive-source semantic delta preview records to runtime-only participant state rows and produces before/mutation/after runtime state reports. It still stops before participant SimThing property mutation, Scenario mutation, savefile mutation, and persistent history.

## GPU-residency doctrine preservation

- Reuses semantic participant delta preview (#820) and prior recursive RF ladder surfaces.
- Runtime mutation represented as flat GPU-compatible before/mutation/after rows.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.

## Runtime state mutation model

- Added `RuntimeParticipantStateMutationSourceMode`, `RuntimeParticipantStateMutationKind`, `RuntimeParticipantStateRow`, `RuntimeParticipantStateMutationRecord`, `RuntimeParticipantStateMutationReport`.
- Added `evaluate_runtime_participant_state_mutation`, `prove_runtime_participant_state_mutation_preserves_authority`, and `replay_runtime_participant_state_mutation`.
- Driver: `compile_runtime_participant_state_mutation_plan` composes delta preview plan and mutation report.
- Scenario ingestion: `runtime_participant_state_mutation_ready` / `runtime_participant_state_mutation_deferred` readiness flags.

## Before / mutation / after rows

- `before_rows` initialized from delta preview target participant/property pairs at 0.0.
- `mutation_records` capture before/delta/after per applied preview.
- `after_rows` reflect runtime table state after all deltas applied.

## Target property mapping

- `runtime.preview.applied`, `runtime.preview.satisfied`, `runtime.preview.shortfall` used as runtime table property IDs only.
- Properties are not written to SimThing.properties.

## Mutation deferral proof

- `participant_property_mutation_deferred`, `scenario_authority_mutation_deferred`, `savefile_mutation_deferred`, and `persistent_history_deferred` true on report and records.
- Default semantic, execution-boundary, and delta-preview compile paths unchanged.

## Replay determinism proof

- `replay_runtime_participant_state_mutation` with bounded replay_count (1..=64) — PASS.
- Identical mutation_records and after_rows across replays.

## Resource-key / generic channel note

- Typed recursive RF metadata preserved in source rows; delta preview/execution records still use `generic` writeback alignment.
- Typed property mutation channels remain deferred.

## Prior ladder preservation proof

- `compile_runtime_rf_tick_plan`, `compile_runtime_tick_shell_plan`, `compile_local_effect_application_plan`, and `compile_semantic_local_effects_plan` outputs unchanged.
- Legacy default semantic path remains preserved.

## Authority preservation proof

- `prove_runtime_participant_state_mutation_preserves_authority` — PASS.
- Serialized Scenario authority before == after.

## Boundary / non-goals

- No participant SimThing property mutation.
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
| `cargo test -p simthing-spec --test runtime_participant_state_mutation` | PASS (14 tests) |
| `cargo test -p simthing-spec --test semantic_participant_delta_preview` | PASS |
| `cargo test -p simthing-spec --test semantic_effect_execution_boundary` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test runtime_participant_state_mutation` | PASS (10 tests) |
| `cargo test -p simthing-driver --test semantic_participant_delta_preview` | PASS |
| `cargo test -p simthing-driver --test semantic_effect_execution_boundary` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/runtime_participant_state_mutation.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/runtime_participant_state_mutation.rs`
- `crates/simthing-driver/src/runtime_participant_state_mutation_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/runtime_participant_state_mutation.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/runtime_participant_state_mutation_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Runtime state rows do not yet drive participant SimThing property mutation.
- Participant SimThing property mutation remains deferred.
- Typed property mutation channels not yet authoritative.
- Studio presentation of runtime state mutation reports remains deferred.

## Deferred next rung

1. Prove controlled participant SimThing property mutation boundary from runtime state rows, without ScenarioSpec/savefile persistence.
2. Only after property mutation boundary, evaluate savefile/persistent history boundary.
3. Typed semantic mutation channels remain deferred.
4. Studio presentation remains deferred.

## DA status

Not DA-promoted.

This rung intentionally moves beyond delta-preview reports while still stopping before Scenario/property persistence. Recursive-source semantic delta preview records now apply to deterministic runtime-only participant state rows behind explicit source mode. The produced before/mutation/after rows do not mutate participant SimThing properties, Scenario authority, savefiles, or persistent history. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target.
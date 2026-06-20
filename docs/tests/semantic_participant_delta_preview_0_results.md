# SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `semantic-participant-delta-preview-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Derive runtime-only participant property delta previews from semantic execution records without mutation.

## Pre-flight metadata check

- #818/#819 metadata verified on `master`: PR #818, merge `4ddb7b4a918c9e4502de4d12b5fe50784cb19bc3`; no `TBD` placeholders for #818 in evidence index, result report, or production doc.
- #818 proved that recursive-source semantic local effect projections can produce deterministic runtime execution records without mutation. This rung derives participant property delta previews from those execution records, still without mutating participant SimThings, Scenario authority, savefiles, or persistent history.

## Anti-loop production-path statement

This rung is not another hygiene-only comparison layer. It converts recursive-source semantic execution records into deterministic runtime participant property delta preview records. It still stops before participant property mutation, Scenario mutation, savefile mutation, and persistent history.

## GPU-residency doctrine preservation

- Reuses semantic execution boundary (#818) and prior recursive RF ladder surfaces.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.

## Delta preview model

- Added `ParticipantDeltaPreviewSourceMode`, `ParticipantDeltaPreviewKind`, `ParticipantPropertyDeltaPreviewRecord`, `SemanticParticipantDeltaPreviewReport`.
- Added `evaluate_semantic_participant_delta_preview` and `prove_semantic_participant_delta_preview_preserves_authority`.
- Driver: `compile_semantic_participant_delta_preview_plan` composes execution boundary plan and delta preview report.
- Scenario ingestion: `semantic_participant_delta_preview_ready` / `semantic_participant_delta_preview_deferred` readiness flags.

## Runtime delta preview records

- Recursive-source semantic execution records convert into deterministic runtime-only participant property delta preview records behind explicit source mode.
- Records preserve source execution id, source_simthing_id_raw, owner_ref, resource_key, scope_id, delta kind, amount, and provisional target property mapping.
- Deterministic ordering with checked totals.

## Target property mapping

- `RuntimeAppliedAmountExecution` → `runtime.preview.applied`
- `ResourceSatisfiedExecution` → `runtime.preview.satisfied`
- `ResourceShortfallExecution` → `runtime.preview.shortfall`
- Mapping is provisional; properties are not written to SimThings.

## Mutation deferral proof

- `participant_property_mutation_deferred`, `scenario_authority_mutation_deferred`, `savefile_mutation_deferred`, and `persistent_history_deferred` true on report and records.
- Default semantic local effects and execution-boundary compile paths unchanged.

## Resource-key / generic channel note

- #812/#813/#814/#816/#818 preserve typed recursive RF metadata in aggregate source rows while using `generic` writeback-channel alignment; preserved in this rung for delta preview records.
- Typed property mutation channels remain deferred until a later multi-resource channel rung.

## Prior ladder preservation proof

- `compile_runtime_rf_tick_plan`, `compile_runtime_tick_shell_plan`, `compile_local_effect_application_plan`, and `compile_semantic_local_effects_plan` outputs unchanged.
- Legacy default semantic path remains preserved.

## Authority preservation proof

- `prove_semantic_participant_delta_preview_preserves_authority` — PASS.
- Serialized Scenario authority before == after.

## Boundary / non-goals

- No participant SimThing property mutation.
- No ScenarioSpec mutation.
- No savefile or persistent timeline mutation.
- No default semantic execution source replacement.
- No new GPU primitive or WGSL.
- No fused recursive RF kernel.
- No Studio GPU dispatch.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test semantic_participant_delta_preview` | PASS (13 tests) |
| `cargo test -p simthing-spec --test semantic_effect_execution_boundary` | PASS |
| `cargo test -p simthing-spec --test semantic_local_effects_recursive_source` | PASS |
| `cargo test -p simthing-spec --test semantic_local_effects` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test semantic_participant_delta_preview` | PASS (10 tests) |
| `cargo test -p simthing-driver --test semantic_effect_execution_boundary` | PASS |
| `cargo test -p simthing-driver --test semantic_local_effects_recursive_source` | PASS |
| `cargo test -p simthing-driver --test semantic_local_effects` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/semantic_participant_delta_preview.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/semantic_participant_delta_preview.rs`
- `crates/simthing-driver/src/semantic_participant_delta_preview_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/semantic_participant_delta_preview.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/semantic_participant_delta_preview_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Delta preview records do not yet drive participant property mutation.
- Participant property mutation remains deferred.
- Typed property mutation channels not yet authoritative for multi-resource economy.
- Studio presentation of delta preview reports remains deferred.

## Deferred next rung

1. Add controlled runtime-only participant state mutation from delta preview records.
2. Prove mutation is runtime-state-only and still does not mutate ScenarioSpec/savefile.
3. Only after runtime-state mutation proof, evaluate persistence/savefile boundary.
4. Typed semantic mutation channels remain deferred.
5. Studio presentation remains deferred.

## DA status

Not DA-promoted.

This rung intentionally moves beyond execution-boundary reports while still stopping before mutation. Recursive-source semantic execution records now produce deterministic runtime participant property delta preview records behind explicit source mode. These preview records do not mutate participant SimThing properties, Scenario authority, savefiles, or persistent history. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target.
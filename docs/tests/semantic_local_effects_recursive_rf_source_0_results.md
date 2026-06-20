# SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `semantic-local-effects-recursive-rf-source-0`
- PR: #816
- Merge SHA: `148b3b03dbac65967d91551dd69295302ce63093`

## Mission

Feed recursive-source local effect application into semantic local effect projection behind explicit source mode.

## Pre-flight metadata check

- #814 metadata verified on `master`: PR #814, merge `51fb53f6fe01bb3ecff187708b881f5aedc8423e`; no `TBD` placeholders for #814 in evidence index, result report, or production doc.
- #815 metadata follow-up merged (`9b2ffcc49e90c80af46344312a1d7e2b4b2cbdc9`).

## Anti-loop statement

#814 integrated recursive-source local allocation into local participant effect previews and local effect application reports behind explicit source mode. This rung intentionally advances the immediate production path by feeding that recursive-source local effect application report into semantic local effect projection behind explicit source mode. It still stops before semantic execution and participant property mutation.

This rung is not another hygiene-only comparison layer. It produces semantic local effect projection reports using recursive-source local effect application. It still stops before semantic execution and participant property mutation.

## GPU-residency doctrine preservation

- Reuses recursive RF aggregate source rows from #808, owner-silo recursive source surfaces from #812, local allocation recursive source from #813, and local effect recursive source from #814.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.

## Resource-key / generic-channel note

- #812/#813/#814 preserve typed recursive RF metadata in aggregate source rows while using `generic` writeback-channel alignment for owner-silo/local-allocation/local-effect compatibility; preserved in this rung for semantic projection.
- Typed semantic resource channels remain deferred until a later multi-resource channel rung.

## Semantic local effects recursive source integration

- Added `SemanticLocalEffectRfSourceMode`, `SemanticLocalEffectRfSourceSelection`, `SemanticLocalEffectRfSourceReport`.
- Added `semantic_local_effects_from_local_effect_application_report` adapter and `evaluate_semantic_local_effects_with_rf_source`.
- Driver: `compile_semantic_local_effects_recursive_source_plan` composes local effect recursive source plan and semantic report.
- Scenario ingestion: `semantic_local_effects_recursive_source_ready` / `semantic_local_effects_recursive_source_deferred` readiness flags.

## Legacy default preservation proof

- `LegacyPlanetChildOwnerSilo` mode returns unchanged legacy semantic local effects report.
- `compile_runtime_rf_tick_plan`, `compile_runtime_tick_shell_plan`, `compile_local_effect_application_plan`, and `compile_semantic_local_effects_plan` outputs unchanged.

## Downstream deferral proof

- `semantic_execution_deferred` and `participant_property_mutation_deferred` true.
- Default semantic local effect totals unchanged after compile.

## Authority preservation proof

- `prove_semantic_local_effects_recursive_source_preserves_authority` — PASS.

## Boundary / non-goals

- No default semantic source replacement.
- No semantic effect execution.
- No participant property mutation.
- No ScenarioSpec mutation.
- No savefile or persistent timeline mutation.
- No new GPU primitive or WGSL.
- No fused recursive RF kernel.
- No Studio GPU dispatch.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver` | PASS |
| `cargo test -p simthing-spec --test semantic_local_effects_recursive_source` | PASS (13 tests) |
| `cargo test -p simthing-driver --test semantic_local_effects_recursive_source` | PASS (12 tests) |
| `cargo test -p simthing-driver --test local_effect_recursive_source` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/semantic_local_effects_recursive_rf_source.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/semantic_local_effects_recursive_source.rs`
- `crates/simthing-driver/src/semantic_local_effects_recursive_source_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/semantic_local_effects_recursive_source.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/semantic_local_effects_recursive_rf_source_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive semantic projection does not drive semantic execution.
- Participant property mutation remains deferred.
- Typed semantic resource channels not yet authoritative for multi-resource economy.
- Studio presentation of recursive semantic local effect reports remains deferred.

## Deferred next rung

1. Evaluate semantic effect execution authority behind explicit source mode.
2. Participant property mutation authority remains deferred until semantic execution gates pass.
3. Typed semantic resource channels must be restored before multi-resource economy semantics are authoritative.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive semantic local effect reports remains deferred.

## DA status

Not DA-promoted.

This semantic projection integration rung does not grant CPU production simulation authority. Recursive RF drives semantic local effect projection proof reports only behind explicit source mode. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target and is not wired into semantic execution in this rung.
# LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `local-effect-application-recursive-rf-source-0`
- PR: #814
- Merge SHA: `51fb53f6fe01bb3ecff187708b881f5aedc8423e`

## Mission

Feed recursive-source local allocation into local participant effect previews and local effect application behind explicit source mode.

## Pre-flight metadata check

- #813 metadata verified on `master`: PR #813, merge `2a32494f747dd50becfea586eb7ba4d5f2335fbc`; no `TBD` placeholders for #813 in evidence index, result report, or production doc.

## Anti-loop statement

#813 integrated recursive-source owner-silo/disburse-down into runtime local allocation reports behind explicit source mode. This rung intentionally advances the immediate production path by feeding that recursive-source local allocation report into local participant effect previews and local effect application proof reports behind explicit source mode. It still stops before semantic local effects and semantic execution.

This rung is not another hygiene-only comparison layer. It produces local participant effect previews and local effect application reports using recursive-source local allocation. It still stops before semantic local effects and semantic execution.

## GPU-residency doctrine preservation

- Reuses recursive RF aggregate source rows from #808, owner-silo recursive source surfaces from #812, and local allocation recursive source surfaces from #813.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.

## Resource-key / generic-channel note

- #812/#813 preserve typed recursive RF metadata in aggregate source rows while using `generic` writeback-channel alignment for owner-silo/local-allocation compatibility; preserved in this rung for local effects.
- Typed resource metadata remains in recursive RF aggregate source rows (e.g. `food` on sibling fixture).
- Typed local-effect/semantic resource channels remain deferred until a later multi-resource channel rung.

## Local effect recursive source integration

- Added `LocalEffectRfSourceMode`, `LocalEffectRfSourceSelection`, `LocalEffectApplicationRfSourceReport`.
- Added `local_participant_effects_from_runtime_local_allocation_report` and `local_effect_application_from_participant_effects_report` adapters.
- Added `evaluate_local_effect_application_with_rf_source` and `prove_local_effect_recursive_source_preserves_authority`.
- Driver: `compile_local_effect_recursive_source_plan` composes local allocation recursive source plan and local effect report.
- Scenario ingestion: `local_effect_recursive_source_ready` / `local_effect_recursive_source_deferred` readiness flags.

## Legacy default preservation proof

- `LegacyPlanetChildOwnerSilo` mode returns unchanged legacy participant effects and application reports.
- `compile_runtime_rf_tick_plan`, `compile_runtime_tick_shell_plan`, `compile_local_effect_application_plan`, and `compile_semantic_local_effects_plan` outputs unchanged.

## Downstream deferral proof

- `semantic_effect_integration_deferred` true; recursive application does not feed semantic local effects.
- Semantic local effect totals unchanged after compile.

## Authority preservation proof

- `prove_local_effect_recursive_source_preserves_authority` — PASS.

## Boundary / non-goals

- No default local effect source replacement.
- No semantic local effects from recursive source.
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
| `cargo test -p simthing-spec --test local_effect_recursive_source` | PASS (13 tests) |
| `cargo test -p simthing-driver --test local_effect_recursive_source` | PASS (12 tests) |
| `cargo test -p simthing-spec --test local_effect_application` | PASS |
| `cargo test -p simthing-driver --test local_allocation_recursive_source` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/local_effect_recursive_rf_source.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/local_effect_recursive_source.rs`
- `crates/simthing-driver/src/local_effect_recursive_source_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/local_effect_recursive_source.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/local_effect_application_recursive_rf_source_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive local effect application does not feed semantic local effects.
- Semantic effect execution remains deferred.
- Typed local-effect/semantic resource channels not yet authoritative for multi-resource economy.
- Studio presentation of recursive local effect reports remains deferred.

## Deferred next rung

1. Integrate recursive local effect application into semantic local effects behind explicit source mode.
2. Semantic effect execution authority remains deferred until recursive local effects path is proven through semantic layer.
3. Typed local-effect/semantic resource channels must be restored before multi-resource economy semantics are authoritative.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive local effect reports remains deferred.

## DA status

Not DA-promoted.

This local effect integration rung does not grant CPU production simulation authority. Recursive RF drives local participant effect previews and local effect application proof reports only behind explicit source mode. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target and is not wired into semantic local effects or semantic execution in this rung.
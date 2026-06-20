# LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `local-allocation-recursive-rf-source-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Feed recursive-source owner-silo/disburse-down into runtime local allocation behind explicit source mode.

## Pre-flight metadata check

- #812 metadata verified on `master`: PR #812, merge `807965d8f94e5a54085c9373f5802d9154850448`; no `TBD` placeholders for #812 in evidence index, result report, or production doc.

## Anti-loop statement

#812 integrated recursive RF into owner-silo/disburse-down reports behind explicit source mode. This rung intentionally advances the immediate production path by feeding that recursive-source owner-silo disburse report into runtime local allocation behind explicit source mode. It still stops before local effects and semantic execution.

This rung is not another hygiene-only comparison layer. It produces a runtime local allocation report using recursive-source owner-silo disburse-down. It still stops before local effects and semantic execution.

## GPU-residency doctrine preservation

- Reuses recursive RF aggregate source rows from #808 and owner-silo recursive source surfaces from #812.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.

## Resource-key / generic-channel note

- #812 normalizes recursive owner-silo demand bucket `resource_key` to `generic` for writeback-channel alignment; preserved in this rung.
- Typed resource metadata remains in recursive RF aggregate source rows (e.g. `food` on sibling fixture).
- Typed owner-silo/local-allocation channels remain deferred until a later multi-resource channel rung.

## Local allocation recursive source integration

- Added `LocalAllocationRfSourceMode`, `LocalAllocationRfSourceSelection`, `RuntimeLocalAllocationRfSourceReport`.
- Added `runtime_local_allocation_from_owner_silo_disburse_report` adapter and `evaluate_runtime_local_allocation_with_rf_source`.
- Driver: `compile_local_allocation_recursive_source_plan` composes owner-silo recursive source plan and allocation report.

## Legacy default preservation proof

- `LegacyPlanetChildOwnerSilo` mode returns unchanged legacy allocation report.
- `compile_runtime_rf_tick_plan`, `compile_runtime_tick_shell_plan`, `compile_local_effect_application_plan`, and `compile_semantic_local_effects_plan` outputs unchanged.

## Downstream deferral proof

- `local_effect_integration_deferred` and `semantic_effect_integration_deferred` true.
- Local effect application and semantic local effect totals unchanged after compile.

## Authority preservation proof

- `prove_local_allocation_recursive_source_preserves_authority` — PASS.

## Boundary / non-goals

- No default local allocation source replacement.
- No local effect application from recursive allocation.
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
| `cargo fmt --all` | PASS |
| `cargo test -p simthing-spec --test local_allocation_recursive_source` | PASS (14 tests) |
| `cargo test -p simthing-driver --test local_allocation_recursive_source` | PASS (12 tests) |
| `cargo test -p simthing-spec --test runtime_local_allocation` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/local_allocation_recursive_rf_source.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/local_allocation_recursive_source.rs`
- `crates/simthing-driver/src/local_allocation_recursive_source_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/local_allocation_recursive_source.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/local_allocation_recursive_rf_source_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive local allocation does not feed local effect application.
- Semantic effects remain legacy/deferred.
- Typed owner-silo resource channels not yet authoritative for multi-resource economy.
- Studio presentation of recursive local allocation reports remains deferred.

## Deferred next rung

1. Integrate recursive local allocation into local effect application behind explicit source mode.
2. Semantic effect execution authority remains deferred until recursive allocation path is proven through local effects.
3. Typed owner-silo resource channels must be restored before multi-resource economy semantics are authoritative.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive local allocation reports remains deferred.

## DA status

Not DA-promoted.

This local allocation integration rung does not grant CPU production simulation authority. Recursive RF drives local allocation proof reports only behind explicit source mode. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target and is not wired into local effects or semantic effects in this rung.
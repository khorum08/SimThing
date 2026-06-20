# RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `runtime-tick-recursive-rf-selectable-source-0`
- PR: #811
- Merge SHA: `7deabc9daaa07db1abf728d21b0b6d42f9ef521e`

## Mission

Promote recursive RF from preview-only to explicitly selectable tick-shell RF report source behind mode flag, without default replacement or downstream integration.

## Pre-flight metadata check

- #810 metadata verified on `master`: PR #810, merge `5d283140a104dd8955da2e3b3a379ef418b28c11`; no `TBD` placeholders for #810 in evidence index, result report, or production doc.
- #810 added optional side-by-side RF source comparison. Recursive RF is currently preview-only and the legacy planet-child/owner-silo RF source remains default. This rung adds explicit selectable-source mode, but does not change default tick-shell behavior or feed recursive RF into owner-silo/disburse-down, local allocation, local effects, or semantic effects.

## GPU-residency doctrine preservation

- Recursive RF remains a GPU-compatible flat row/table target via `recursive_local_rf_aggregate_source_rows`.
- CPU responsibilities remain oracle/reference/shadow comparison, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.
- `gpu_residency_doctrine_preserved` asserted in selection compile plans.

## Selectable-source model

- Extended `RuntimeRfTickSourceMode` with `RecursiveSelectable`.
- Added `RuntimeRfTickSourceSelectionGate`, `RuntimeRfTickSelectedSourceReport`.
- Added `evaluate_runtime_rf_tick_source_selection` and `prove_runtime_rf_tick_source_selection_preserves_authority`.
- Driver wrapper: `compile_runtime_rf_tick_source_selection_plan`, `compile_runtime_tick_shell_with_selectable_rf_source_plan`.

## Equivalence / selection gates

- Participant projection gate: legacy planet-child participant rows must match recursive direct-participant projections.
- Legacy fixture gate: `owner_silo_disburse_down_scoped` — compatible projection, `RecursiveSelectable` allowed.
- Redistribution delta gate: `recursive_local_rf_sibling_redistribution` — deltas documented, not silent equivalence.
- Downstream deferral gate: recursive selection is RF-report-only; owner-silo, allocation, local effects, semantic effects remain deferred.

## Legacy default preservation proof

- `LegacyDefault` and `SideBySideComparison` select `LegacyPlanetChildOwnerSilo`.
- `compile_runtime_rf_tick_plan` output unchanged before/after selection compile.
- `compile_runtime_tick_shell_plan` output unchanged unless explicit wrapper helper is used.

## Recursive selectable report-only proof

- `RecursiveSelectable` selects `RecursiveLocalRf` only when gates pass.
- `recursive_source_selected_for_rf_report_only` true when selection allowed.
- `owner_silo_integration_deferred`, `local_allocation_integration_deferred`, `local_effect_integration_deferred`, `semantic_effect_integration_deferred` all true.

## Downstream deferral proof

- Selection compile plan preserves default tick shell and default runtime RF tick plan.
- Local effect application totals unchanged.
- Semantic local effect totals unchanged.

## Authority preservation proof

- `prove_runtime_rf_tick_source_selection_preserves_authority` — PASS for owner_silo and sibling fixtures.

## Boundary / non-goals

- No default runtime RF source replacement.
- No owner-silo/disburse-down integration from recursive source.
- No local allocation integration from recursive source.
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
| `cargo test -p simthing-spec --test runtime_rf_tick_source_selection` | PASS (13 tests) |
| `cargo test -p simthing-driver --test runtime_rf_tick_source_selection` | PASS (15 tests) |
| `cargo test -p simthing-spec --test runtime_rf_tick_source` | PASS (14 tests) |
| `cargo test -p simthing-driver --test runtime_rf_tick_source` | PASS (13 tests) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/runtime_rf_tick_source.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/runtime_rf_tick_source_selection.rs`
- `crates/simthing-driver/src/runtime_rf_tick_source_select_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/runtime_rf_tick_source_selection.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/runtime_tick_recursive_rf_selectable_source_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive RF selectable source is RF-report-only; not wired into owner-silo/disburse-down.
- Local allocation, local effects, and semantic effects remain legacy/deferred.
- Studio presentation of selectable RF source reports remains deferred.

## Deferred next rung

1. Integrate recursive RF selectable source into owner-silo/disburse-down behind explicit source mode.
2. Only after owner-silo integration, evaluate recursive RF local allocation/effect path.
3. Semantic effect execution authority remains deferred until recursive RF source is proven through owner-silo/allocation path.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of selectable RF source reports remains deferred.

## DA status

Not DA-promoted.

This selectable-source rung does not grant CPU production simulation authority. Recursive RF selection is RF-report-source selection only in explicit wrapper plans. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target and is not wired into owner-silo disburse-down, local allocation, local effects, or semantic effects in this rung.
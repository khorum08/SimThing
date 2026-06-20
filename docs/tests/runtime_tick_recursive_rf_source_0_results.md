# RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `runtime-tick-recursive-rf-source-0`
- PR: #810
- Merge SHA: `5d283140a104dd8955da2e3b3a379ef418b28c11`

## Mission

Add optional side-by-side RF source comparison for runtime tick shell without replacing legacy planet-child/owner-silo default source.

## Pre-flight metadata check

- #809 metadata verified on `master`: PR #809, merge `7ccd571f1dbab9f507037f55ad955420cddc1020`; no `TBD` placeholders for #809 in evidence index, result report, or production doc.

## GPU-residency doctrine preservation

#809 established reconciliation between the legacy planet-child RF ladder and recursive Location RF projections. This rung integrates recursive RF into the tick-shell proof surface only as an optional side-by-side source. The legacy planet-child/owner-silo RF tick source remains the default until an explicit future replacement rung.

## Legacy default source proof

- `default_source_kind` and `selected_source_kind` remain `LegacyPlanetChildOwnerSilo` in side-by-side mode.
- `compile_runtime_rf_tick_plan` output unchanged before/after comparison compile.

## Recursive preview source proof

- `recursive_source_available` true for owner_silo fixture.
- `recursive_source_preview_only` true.
- `RuntimeRfTickSourceMode::RecursivePreview` selects `RecursiveLocalRf` as selected kind without driving allocation/effects.

## Side-by-side comparison proof

- Composes legacy tick report, recursive evaluation, and reconciliation deltas.
- Owner_silo fixture: `reconciliation_compatible` true.
- Sibling fixture: scope/redistribution deltas recorded without treating as tick-shell failure.

## Tick-shell non-replacement proof

- `compile_runtime_tick_shell_plan` unchanged by default.
- `compile_runtime_tick_shell_with_rf_source_comparison_plan` adds wrapper only.
- Local effect and semantic totals unchanged.

## Authority preservation proof

- `prove_runtime_rf_tick_source_preserves_authority` — PASS.

## Boundary / non-goals

- No default runtime RF source replacement.
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
| `cargo test -p simthing-spec --test runtime_rf_tick_source` | PASS (14 tests) |
| `cargo test -p simthing-driver --test runtime_rf_tick_source` | PASS (13 tests) |
| `cargo test -p simthing-spec --test recursive_rf_reconciliation` | PASS |
| `cargo test -p simthing-driver --test recursive_rf_reconciliation` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/runtime_rf_tick_source.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/runtime_rf_tick_source.rs`
- `crates/simthing-driver/src/runtime_rf_tick_source_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/runtime_rf_tick_source.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/runtime_tick_recursive_rf_source_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive RF not selectable as tick-shell source (preview only).
- No owner-silo/disburse-down integration from recursive source.
- Semantic effect execution remains deferred.

## Deferred next rung

1. Promote recursive RF source from preview to selectable tick-shell source behind explicit mode flag.
2. Add legacy-vs-recursive tick-shell equivalence gates for fixtures that should match.
3. Integrate recursive RF source into owner-silo/disburse-down only after selectable-source proof.
4. Semantic effect execution authority remains deferred until recursive RF tick-shell source is proven.
5. Runtime tick persistent history/replay storage remains deferred.
6. Star-system local-grid GPU operators remain deferred.
7. Fleet movement/combat remains deferred.
8. Studio presentation of recursive RF source comparison reports remains deferred.

## DA status

Not DA-promoted.

This optional tick-source comparison does not grant CPU production simulation authority. CPU work remains oracle/reference/shadow comparison, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target and is not wired into owner-silo disburse-down, local allocation, local effects, or semantic effects in this rung.
# PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `planet-child-recursive-rf-reconciliation-0`
- PR: #809
- Merge SHA: `7ccd571f1dbab9f507037f55ad955420cddc1020`

## Mission

Add reconciliation/projection report between legacy planet-child RF ladder and recursive Location RF evaluator without replacing either path.

## Pre-flight metadata check

- #808 metadata verified on `master`: PR #808, merge `b68bd43456df16d6ba64993ad71f94c05e821103`; no `TBD` placeholders for #808 in evidence index, result report, or production doc.

## GPU-residency doctrine preservation

#808 restored recursive RF aggregate proof coverage and GPU-residency doctrine. This rung must preserve that doctrine: recursive RF remains a GPU-resident row/table execution target, and CPU remains oracle/reference/shadow/bookkeeping/compile-plan/interface only.

## Reconciliation model

- `PlanetChildRfProjectionRow` — legacy planet-child participant projection.
- `RecursiveRfProjectionRow` — recursive aggregate source and arena settlement projection.
- `RecursiveRfReconciliationBucket` — per-scope surplus/demand comparison with deltas.
- `RecursiveRfReconciliationMismatch` — deterministic mismatch evidence with typed kinds.

## Legacy projection proof

- `project_planet_child_rf_ladder_rows()` derives rows from `planet_child_rf_participant_inputs`.
- Owner_silo fixture projects ≥3 participant rows with generic resource key fallback.

## Recursive projection proof

- `project_recursive_local_rf_rows()` derives rows from aggregate source rows and arena settlements.
- Direct participant, child location output, and arena settlement source kinds are represented.

## Compatibility grain

1. **Participant-row compatibility:** legacy planet-child participant rows are present in recursive direct-participant projections at equivalent source/owner/surplus/demand grain.
2. **Settlement/projection compatibility:** owner_silo fixture buckets are fully compatible; sibling redistribution documents parent-level net bubbling deltas via `ScopeProjectionMismatch`.

## Mismatch report

- Deterministic ordering by owner, resource_key, mismatch_kind, source id, location id.
- Sibling fixture records `ScopeProjectionMismatch` at star-system arena when net surplus bubbles upward after local redistribution.

## Prior ladder preservation proof

- `previous_ladder_preserved` true for owner_silo and sibling fixtures.
- Runtime RF tick, local effect application, and semantic local effect totals unchanged before/after reconciliation compile.

## Tick-shell non-replacement proof

- `tick_shell_source_replacement_deferred` true.
- Runtime tick shell still derives from planet-child/owner-silo ladder.

## Authority preservation proof

- `prove_recursive_rf_reconciliation_preserves_authority` — PASS.
- Scenario authority digest before == after.

## Boundary / non-goals

- No tick-shell RF source replacement.
- No semantic effect execution.
- No participant property mutation.
- No ScenarioSpec mutation.
- No savefile or persistent timeline mutation.
- No new GPU primitive or WGSL.
- No fused recursive RF kernel.
- No Studio GPU dispatch.
- No MapGenerator/ClauseThing/Terran Pirate fixture edits.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test recursive_rf_reconciliation` | PASS (12 tests) |
| `cargo test -p simthing-spec --test recursive_local_rf` | PASS (23 tests) |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test recursive_rf_reconciliation` | PASS (10 tests) |
| `cargo test -p simthing-driver --test recursive_local_rf` | PASS (19 tests) |
| `cargo test -p simthing-driver --test local_effect_application` | PASS |
| `cargo test -p simthing-driver --test semantic_local_effects` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/recursive_rf_reconciliation.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/recursive_rf_reconciliation.rs`
- `crates/simthing-driver/src/recursive_rf_reconciliation_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/recursive_rf_reconciliation.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/planet_child_recursive_rf_reconciliation_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive RF not yet integrated into runtime tick shell as GPU-resident RF source.
- Side-by-side tick-shell legacy vs recursive RF source reports not yet implemented.
- Semantic effect execution remains deferred.

## Deferred next rung

1. Integrate recursive local RF evaluator into runtime tick shell as optional GPU-resident RF source.
2. Add side-by-side tick-shell reports comparing legacy RF source and recursive RF source.
3. Semantic effect execution authority remains deferred until recursive RF tick-shell source is proven.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive RF/reconciliation proof reports remains deferred.

## DA status

Not DA-promoted.

This reconciliation does not grant CPU production simulation authority. CPU work remains oracle/reference/shadow reconciliation, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target and is not wired into the runtime tick shell in this rung.
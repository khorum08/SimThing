# OWNER-SILO-RECURSIVE-RF-SOURCE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `owner-silo-recursive-rf-source-0`
- PR: #812
- Merge SHA: `807965d8f94e5a54085c9373f5802d9154850448`

## Mission

Integrate selectable recursive RF source into owner-silo/disburse-down behind explicit source mode.

## Pre-flight metadata check

- #811 metadata verified on `master`: PR #811, merge `7deabc9daaa07db1abf728d21b0b6d42f9ef521e`; no `TBD` placeholders for #811 in evidence index, result report, or production doc.

## Anti-loop statement

#811 completed the report-source selection guardrail. This rung intentionally moves out of hygiene/report-only looping and into the immediate production path: recursive RF may now drive owner-silo/disburse-down proof reports behind explicit source mode. Default legacy behavior remains unchanged and downstream allocation/effects/semantic execution remain deferred.

This rung is not another hygiene-only comparison layer. It produces an owner-silo/disburse-down report using the recursive RF selected source. It still stops before local allocation/effects/semantic execution.

## GPU-residency doctrine preservation

- Recursive RF aggregate source rows reused from #808 GPU-residency remediation surfaces.
- CPU responsibilities remain oracle/reference/shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- No new GPU primitive, WGSL, or fused recursive RF kernel introduced.
- `gpu_residency_doctrine_preserved` asserted in driver compile plans.

## Recursive demand bucket projection grain

- **Primary grain**: per-Location-arena `RecursiveLocalRfAggregateSourceRow` entries with `demand > 0`.
- **Parent redistribution grain**: arena settlement `net_deficit_to_parent > 0` and root `net_deficit > 0` rows.
- **Owner-silo channel alignment**: demand buckets normalize `resource_key` to `generic` to match planet-child reduce-up writeback channels.
- Scope id format: `location/{arena_location_id_raw}` with optional `/parent_deficit` or `/root_deficit` suffixes.

## Owner-silo recursive source integration

- Added `OwnerSiloRfSourceMode`, `OwnerSiloRfSourceSelection`, `OwnerSiloDisburseDownReport`, `OwnerSiloRfSourceDisburseReport`.
- Added `owner_silo_demand_buckets_from_recursive_local_rf` and `evaluate_owner_silo_disburse_down_with_rf_source`.
- Driver: `compile_owner_silo_recursive_source_plan` composes selection, recursive RF, reconciliation, and disburse reports.
- Recursive mode runs actual owner-silo disburse-down against recursive demand buckets.

## Legacy default preservation proof

- `LegacyPlanetChildOwnerSilo` mode returns unchanged legacy disburse report.
- `compile_runtime_rf_tick_plan` and `compile_runtime_tick_shell_plan` outputs unchanged.

## Downstream deferral proof

- `local_allocation_integration_deferred`, `local_effect_integration_deferred`, `semantic_effect_integration_deferred` all true.
- Local effect application and semantic local effect totals unchanged after compile.

## Authority preservation proof

- `prove_owner_silo_recursive_source_preserves_authority` — PASS.

## Boundary / non-goals

- No default owner-silo source replacement.
- No default runtime RF source replacement.
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
| `cargo test -p simthing-spec --test owner_silo_recursive_source` | PASS (14 tests) |
| `cargo test -p simthing-driver --test owner_silo_recursive_source` | PASS (14 tests) |
| `cargo test -p simthing-spec --test owner_silo_disburse_down` | PASS (11 tests) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/owner_silo_recursive_rf_source.rs`
- `crates/simthing-spec/src/spec/owner_silo_disburse_down.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/owner_silo_recursive_source.rs`
- `crates/simthing-driver/src/owner_silo_recursive_source_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/owner_silo_recursive_source.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/owner_silo_recursive_rf_source_0_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive owner-silo disburse-down does not feed local allocation.
- Local effects and semantic effects remain legacy/deferred.
- Studio presentation of recursive owner-silo source reports remains deferred.

## Deferred next rung

1. Integrate recursive owner-silo disburse-down into local allocation behind explicit source mode.
2. Only after allocation integration, evaluate recursive RF local effect path.
3. Semantic effect execution authority remains deferred until recursive RF source is proven through allocation path.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive owner-silo source reports remains deferred.

## DA status

Not DA-promoted.

This owner-silo integration rung does not grant CPU production simulation authority. Recursive RF drives owner-silo/disburse-down proof reports only behind explicit source mode. CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting. Recursive RF remains a GPU-resident row/table target and is not wired into local allocation, local effects, or semantic effects in this rung.
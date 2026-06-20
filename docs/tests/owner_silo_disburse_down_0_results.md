# OWNER-SILO-DISBURSE-DOWN-0 Results

## Status

PASS — focused validation complete; merge pending

## PR / branch / merge

- Branch: `owner-silo-disburse-down-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Add runtime/proof-only owner-silo disburse-down allocation oracle consuming OWNER-SILO-RUNTIME-WRITEBACK-0 results and allocating available owner/resource capacity to scoped local demand buckets without Scenario authority mutation.

## Constitutional alignment

- Scenario / ScenarioSpec remains serializable authority; runtime disburse-down is proof-only.
- Owner/channel scope is metadata/properties/columns, not spatial parentage.
- Allocation application to planet/cohort/fleet state remains deferred.

## Implemented changes

- **simthing-spec** `owner_silo_disburse_down.rs`: demand bucket types, `owner_silo_demand_buckets_from_planet_child_rf`, `apply_owner_silo_runtime_disburse_down_cpu`, `owner_silo_demand_aggregate_totals`.
- **simthing-spec** `scenario.rs`: `OWNER_FLOW_DEMAND_PROPERTY_ID`, `OWNER_FLOW_PRIORITY_PROPERTY_ID`, `apply_participant_owner_flow_demand_metadata`.
- **simthing-driver** `owner_silo_disburse_down_compile.rs`: `compile_owner_silo_disburse_down_plan`, GPU demand aggregate proof plans.
- **Ingestion** `owner_silo_disburse_down_ready` / `owner_silo_disburse_down_deferred` compile-readiness flags.
- Durable corpus fixture `scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json`.

## Demand bucket model

- `RuntimeOwnerSiloDemandBucket` derived from planet gridcells and admitted non-grid children with `owner_flow_demand` metadata.
- Missing demand metadata means no demand; missing priority defaults to `OWNER_FLOW_DEFAULT_PRIORITY` (100).
- Active demand without owner/channel reference rejects fail-closed.

## CPU allocation oracle proof

- Groups by `owner_ref + resource_key`; `available = writeback_result.next_current`.
- Sorts by priority (lower wins), then `scope_id`, then `source_simthing_id_raw`.
- Records allocated/unmet per demand; never exceeds available or requested.

Fixture expectations (`owner_silo_disburse_down_scoped`):
- `owner_a` available 62 → cohort 20 allocated, fleet 42 allocated / 8 unmet, remaining 0.
- `owner_b` available 45 → cohort 10 allocated, remaining 35.

## GPU proof path

- Reuses `compile_participant_channel_sum_plan` (existing AccumulatorOp) to prove aggregate requested demand per owner/resource.
- GPU proof covers demand aggregate totals only; CPU oracle applies priority allocation semantics.
- REAL_ADAPTER_OBSERVED on driver GPU test when adapter available; honest SKIP otherwise.

## Boundary / non-goals

- No ScenarioSpec mutation.
- No new GPU primitive/WGSL.
- No economy/planet/combat/orbit engine.
- No route/pathfinding state.
- No Studio GPU dispatch.
- No MapGenerator/ClauseThing changes.
- No Terran Pirate fixture edits.
- Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test owner_silo_runtime_writeback` | PASS (4) |
| `cargo test -p simthing-spec --test owner_silo_disburse_down` | PASS (11; 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test owner_silo_runtime_writeback` | PASS (13) |
| `cargo test -p simthing-driver --test owner_silo_disburse_down` | PASS (11); GPU REAL_ADAPTER_OBSERVED |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/owner_silo_disburse_down.rs` (new)
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/owner_silo_disburse_down.rs` (new)
- `crates/simthing-spec/tests/disburse_down_fixture.rs` (new)
- `crates/simthing-driver/src/owner_silo_disburse_down_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/owner_silo_disburse_down.rs` (new)
- `crates/simthing-driver/tests/disburse_down_fixture.rs` (new)
- `scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/owner_silo_disburse_down_0_results.md`

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/owner_silo_disburse_down_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |
| `scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json` | durable corpus fixture |

## Known gaps

- Allocation application to runtime local participant state deferred.
- Studio presentation deferred.
- Empty demand set returns zero-allocation compile plan (writeback still required).

## Deferred next rung

1. Allocation application to runtime local participant state.
2. Runtime tick integration after writeback/disburse-down/application boundary proof.

## DA status

Not DA-promoted.
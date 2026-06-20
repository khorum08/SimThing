# RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `runtime-local-allocation-application-0`
- PR: #799
- Merge SHA: `1c01df47`

## Mission

Apply OWNER-SILO-DISBURSE-DOWN-0 allocation results to runtime-local participant allocation state without mutating Scenario authority.

## Constitutional alignment

- Scenario / ScenarioSpec remains serializable authority.
- Runtime allocation state is proof-only; no participant SimThing property mutation.
- Economy execution remains deferred.

## Implemented changes

- **simthing-spec** `runtime_local_allocation.rs`: `RuntimeLocalAllocationState`, `RuntimeLocalAllocationApplicationReport`, `apply_runtime_local_allocations_from_disburse_down`, `runtime_local_allocation_aggregate_totals`.
- **simthing-spec** `owner_silo_disburse_down.rs`: propagate `planet_id` and `star_system_gridcell_id_raw` into disburse-down allocations.
- **simthing-driver** `runtime_local_allocation_compile.rs`: `compile_runtime_local_allocation_application_plan`, GPU allocated aggregate proof plans.
- **Ingestion** `runtime_local_allocation_ready` / `runtime_local_allocation_deferred` compile-readiness flags.
- Reuses `scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json`.

## Runtime allocation state model

- One `RuntimeLocalAllocationState` per nonzero demand allocation with `source_simthing_id_raw`.
- Duplicate source per owner/resource/scope rejects fail-closed.
- Empty disburse-down returns zero report with `economy_execution_deferred=true`.

## CPU application oracle proof

Fixture expectations:
- `owner_a` cohort: requested 20 → allocated 20, unmet 0
- `owner_a` fleet: requested 50 → allocated 42, unmet 8
- `owner_b` cohort: requested 10 → allocated 10, unmet 0
- Report totals: allocated 72, unmet 8, allocation_count 3

## GPU proof path

- Reuses `compile_participant_channel_sum_plan` for allocated totals per owner/resource.
- GPU proof covers allocation aggregate totals; CPU oracle applies participant allocation semantics.
- REAL_ADAPTER_OBSERVED on driver GPU test when adapter available; honest SKIP otherwise.

## Boundary / non-goals

- No ScenarioSpec mutation.
- No participant property mutation.
- No new GPU primitive/WGSL.
- No economy/combat/movement engines.
- Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test owner_silo_disburse_down` | PASS (11; 1 ignored) |
| `cargo test -p simthing-spec --test runtime_local_allocation` | PASS (9; 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test owner_silo_disburse_down` | PASS |
| `cargo test -p simthing-driver --test runtime_local_allocation` | PASS (10); GPU REAL_ADAPTER_OBSERVED |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/runtime_local_allocation.rs` (new)
- `crates/simthing-spec/src/spec/owner_silo_disburse_down.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/runtime_local_allocation.rs` (new)
- `crates/simthing-driver/src/runtime_local_allocation_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/runtime_local_allocation.rs` (new)
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/runtime_local_allocation_application_0_results.md`

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/runtime_local_allocation_application_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/design_0_0_8_3_studio_production.md` | living production synthesis |

## Known gaps

- Full economy execution deferred.
- Studio presentation deferred.
- No separate corpus fixture (reuses disburse-down scoped fixture).

## Deferred next rung

1. Runtime tick integration over RF writeback/disburse-down/allocation reports.

## DA status

Not DA-promoted.
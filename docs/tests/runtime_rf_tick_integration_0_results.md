# RUNTIME-RF-TICK-INTEGRATION-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `runtime-rf-tick-integration-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Compose RF participant admission, scoped reduce-up, owner-silo runtime writeback, owner-silo disburse-down, and runtime-local allocation application into one deterministic runtime tick report.

## Doctrine reaffirmed

RF is generic flow accumulation over SimThings, not an economy engine. GPU is proof/cache over AccumulatorOp, not Scenario authority. Scenario / ScenarioSpec remains serializable authority. Owners are GameSession child SimThings and RF channel scopes, not spatial parents. Local RF resolves locally, then reduces upward by owner/resource/scope. This tick integration composes reports only: participant admission → reduce-up → writeback → disburse-down → local allocation; it does not execute economy, consumption, supply, combat, movement, or savefile effects.

## Constitutional alignment

- Scenario / ScenarioSpec remains serializable authority.
- Runtime tick report composes proof state only.
- Economy execution and local effect application remain deferred.

## Implemented changes

- **simthing-spec** `runtime_rf_tick.rs`: `RuntimeRfTickReport`, `evaluate_runtime_rf_tick`, stage error/deferral kinds; chains #795–#799 helpers in order.
- **simthing-driver** `runtime_rf_tick_compile.rs`: `RuntimeRfTickPlan`, `compile_runtime_rf_tick_plan`, `RuntimeRfTickGpuProofSummary` composing existing stage compile helpers.
- **Ingestion** `runtime_rf_tick_ready` / `runtime_rf_tick_deferred` via `integrate_runtime_rf_tick`.
- Reuses `scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json`.
- Production doc § RUNTIME-RF-TICK-INTEGRATION-0 added; #795–#799 sections verified present (no synthesis repair required).

## Runtime RF tick report model

Fixture expectations (`owner_silo_disburse_down_scoped`):
- participant_count 4; reduce_up_bucket_count 2
- writeback: owner_a 50→62, owner_b 40→45
- disburse-down: owner_a cohort 20; owner_a fleet 42 allocated / 8 unmet; owner_b cohort 10
- local allocation: allocation_count 3; allocated_total 72; unmet_total 8
- All stage ready flags true; all deferred flags true for economy / scenario mutation / local effects

## CPU composition oracle proof

- `evaluate_runtime_rf_tick` rejects earlier-stage failures before marking later stages ready.
- Summary totals match constituent report totals.
- Scenario authority unchanged across evaluation and compile.

## GPU proof path

- GPU proof remains stage-local over existing AccumulatorOp surfaces from #795–#799.
- `RuntimeRfTickPlan` composes those proof plans and CPU oracles into one tick boundary report.
- No new GPU primitive or WGSL.
- Driver tests assert stage plan summary presence; real-adapter paths inherited from stage tests.

## Boundary / non-goals

- No ScenarioSpec mutation.
- No new GPU primitive/WGSL.
- No economy/combat/movement engines.
- No route/pathfinding/predecessor state.
- Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-spec --test runtime_rf_tick` | PASS (8; 1 ignored) |
| `cargo test -p simthing-spec --test runtime_local_allocation` | PASS (9; 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo test -p simthing-driver --test runtime_rf_tick` | PASS (10) |
| `cargo test -p simthing-driver --test runtime_local_allocation` | PASS (10); GPU REAL_ADAPTER_OBSERVED |
| `cargo test -p simthing-driver --test owner_silo_disburse_down` | PASS (11) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/runtime_rf_tick.rs` (new)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/runtime_rf_tick.rs` (new)
- `crates/simthing-driver/src/runtime_rf_tick_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/runtime_rf_tick.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/runtime_rf_tick_integration_0_results.md`

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/runtime_rf_tick_integration_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |

## Known gaps

- Runtime tick execution shell deferred.
- Studio presentation deferred.
- GPU proof is stage-local composition summary, not a single fused kernel tick.

## Deferred next rung

1. Runtime tick execution shell over composed RF tick reports.

## DA status

Not DA-promoted.
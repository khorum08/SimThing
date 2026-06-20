# LOCAL-PARTICIPANT-EFFECTS-0 Results

## Status

PASS — focused validation complete; PR #803 merged.

## PR / branch / merge

- Branch: `local-participant-effects-0`
- PR: #803
- Merge SHA: `0b5b1791b00525ab07e2158af5c888e193d18b25`

## Mission

Add runtime/proof-only local participant effect previews under the runtime tick shell from runtime-local allocation state. Each effect records source SimThing id, owner/resource/scope, requested, allocated, unmet, and satisfied/unsatisfied status while preserving Scenario authority unchanged.

## Pre-flight metadata check

PASS — #802 (`RUNTIME-TICK-EXECUTION-SHELL-0`) post-merge metadata verified on current `master`:

- No `TBD` placeholders in `docs/tests/runtime_tick_execution_shell_0_results.md`, `docs/tests/current_evidence_index.md`, or `docs/0.8.3 Simthing Studio Production.md` for #802.
- RUNTIME-TICK-EXECUTION-SHELL-0 section present in production doc with PR #802 and merge SHA.

## Doctrine preservation

All 16 reaffirmed RF/location/owner-channel doctrine points preserved. Effect previews are runtime/proof reports only; no economy engine, no Scenario mutation, no participant property mutation, no Studio GPU dispatch.

## Implemented changes

- `crates/simthing-spec/src/spec/local_participant_effects.rs` — `RuntimeLocalParticipantEffect`, `LocalParticipantEffectsReport`, `local_participant_effects_from_allocations`, `evaluate_local_participant_effects`.
- `crates/simthing-driver/src/local_participant_effects_compile.rs` — `LocalParticipantEffectsPlan`, `LocalParticipantEffectAggregateProofPlan`, `compile_local_participant_effects_plan`.
- `crates/simthing-spec/src/spec/scenario_ingestion.rs` — `local_participant_effects_ready` / `local_participant_effects_deferred`.
- Spec tests: 12 pass. Driver tests: 11 pass (GPU adapter observed when available).

## Runtime local effect model

`evaluate_local_participant_effects` validates tick shell readiness, derives allocation states from composed RF tick report, converts each allocation to an effect preview with `satisfied = unmet == 0` and `effect_application_deferred = true`. Deterministic ordering by owner_ref/resource_key/scope_id/source_simthing_id_raw.

## CPU effect preview proof

Fixture `owner_silo_disburse_down_scoped.simthing-scenario.json` at tick_id=1:

- owner_a cohort: requested 20, allocated 20, unmet 0, satisfied true
- owner_a fleet: requested 50, allocated 42, unmet 8, satisfied false
- owner_b cohort: requested 10, allocated 10, unmet 0, satisfied true
- effect_count 3, allocated_total 72, unmet_total 8, satisfied_count 2, unsatisfied_count 1

## GPU proof path

GPU proof covers aggregate effect preview totals only (allocated and unmet per owner/resource via existing AccumulatorOp). CPU oracle defines satisfied/unsatisfied and deferral semantics. No new GPU primitive or WGSL.

## Boundary / non-goals

No economy execution, consumption, supply effects, combat, movement, route/pathfinding, savefile mutation, participant property mutation, Studio GPU dispatch, new GPU primitives, new WGSL, or privileged engine tokens. No MapGenerator/ClauseThing/Terran Pirate fixture changes. Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test local_participant_effects` | PASS (12) |
| `cargo test -p simthing-spec --test runtime_tick_shell` | PASS (8; 1 ignored) |
| `cargo test -p simthing-spec --test runtime_rf_tick` | PASS (8; 1 ignored) |
| `cargo test -p simthing-spec --test runtime_local_allocation` | PASS (9; 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test local_participant_effects` | PASS (11) |
| `cargo test -p simthing-driver --test runtime_tick_shell` | PASS (10) |
| `cargo test -p simthing-driver --test runtime_local_allocation` | PASS (10) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/local_participant_effects.rs` (new)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/local_participant_effects.rs` (new)
- `crates/simthing-driver/src/local_participant_effects_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/local_participant_effects.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/local_participant_effects_0_results.md` (new)

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/local_participant_effects_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |

## Known gaps

- Studio presentation of local participant effect proof preview deferred.
- Real local effect application semantics (consumption/supply) not yet implemented.
- Runtime tick history/replay evidence not yet implemented.

## Deferred next rung

1. Runtime tick history/replay evidence over tick shell and local effects.
2. Local effect application semantics remain deferred until explicit authority boundary proof.
3. Star-system local-grid GPU operators remain deferred.
4. Fleet movement/combat remains deferred.
5. Studio presentation of RF/tick/effect proof reports remains deferred.

## DA status

Not DA-promoted.
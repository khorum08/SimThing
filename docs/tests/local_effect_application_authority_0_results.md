# LOCAL-EFFECT-APPLICATION-AUTHORITY-0 Results

## Status

PASS — focused validation complete; PR/merge metadata finalized after merge.

## PR / branch / merge

- Branch: `local-effect-application-authority-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Add runtime/proof-only local effect application authority boundary that converts local participant effect previews into deterministic runtime application records while preserving Scenario authority and deferring semantic effects, participant property mutation, and savefile mutation.

## Pre-flight metadata check

PASS — #804 (`RUNTIME-TICK-HISTORY-REPLAY-0`) post-merge metadata verified on current `master`:

- No `TBD` placeholders in `docs/tests/runtime_tick_history_replay_0_results.md`, `docs/tests/current_evidence_index.md`, or `docs/0.8.3 Simthing Studio Production.md` for #804.
- RUNTIME-TICK-HISTORY-REPLAY-0 section present in production doc with PR #804 and merge SHA.

## Doctrine preservation

All 16 reaffirmed RF/location/owner-channel doctrine points preserved. Application records are runtime/proof only; no economy engine, no Scenario mutation, no participant property mutation, no savefile persistence, no Studio GPU dispatch.

## Implemented changes

- `crates/simthing-spec/src/spec/local_effect_application.rs` — `RuntimeLocalEffectApplicationRecord`, `RuntimeLocalEffectApplicationReport`, `apply_runtime_local_effect_records`, `evaluate_runtime_local_effect_application`, `prove_local_effect_application_preserves_authority`.
- `crates/simthing-driver/src/local_effect_application_compile.rs` — `LocalEffectApplicationPlan`, `compile_local_effect_application_plan`.
- `crates/simthing-spec/src/spec/scenario_ingestion.rs` — `local_effect_application_ready` / `local_effect_application_deferred`.
- Spec tests: 12 pass. Driver tests: 12 pass (GPU adapter observed when available).

## Runtime local effect application model

`apply_runtime_local_effect_records` converts each `RuntimeLocalParticipantEffect` into an application record with `runtime_applied_amount = allocated`, all deferral flags true. `prove_local_effect_application_preserves_authority` verifies Scenario authority digest before == after.

## Authority boundary proof

Fixture `owner_silo_disburse_down_scoped.simthing-scenario.json` at tick_id=1:

- application_count 3, requested_total 80, allocated_total 72, unmet_total 8, runtime_applied_total 72
- satisfied_count 2, unsatisfied_count 1
- scenario_authority_digest before == after
- all deferred flags true

## GPU proof path

GPU proof covers aggregate application-record totals only (runtime_applied and unmet per owner/resource via existing AccumulatorOp). CPU oracle defines authority boundary and deferral semantics. No new GPU primitive or WGSL.

## Boundary / non-goals

No ScenarioSpec mutation, participant property mutation, savefile/persistent timeline storage, semantic economy execution, consumption, supply effects, combat, movement, Studio GPU dispatch, new GPU primitives, new WGSL, or privileged engine tokens. No MapGenerator/ClauseThing/Terran Pirate fixture changes. Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test local_effect_application` | PASS (12) |
| `cargo test -p simthing-spec --test runtime_tick_history` | PASS (11) |
| `cargo test -p simthing-spec --test local_participant_effects` | PASS (12) |
| `cargo test -p simthing-spec --test runtime_tick_shell` | PASS (8; 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test local_effect_application` | PASS (12) |
| `cargo test -p simthing-driver --test runtime_tick_history` | PASS (10) |
| `cargo test -p simthing-driver --test local_participant_effects` | PASS (11) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/local_effect_application.rs` (new)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/local_effect_application.rs` (new)
- `crates/simthing-driver/src/local_effect_application_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/local_effect_application.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/local_effect_application_authority_0_results.md` (new)

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/local_effect_application_authority_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |

## Known gaps

- Studio presentation of local effect application proof preview deferred.
- Semantic local effect execution (consumption/supply) not yet implemented.
- Persistent timeline/savefile storage not implemented.

## Deferred next rung

1. Runtime tick persistent history/replay storage remains deferred.
2. Semantic local effect execution remains deferred until typed effect semantics are introduced.
3. Star-system local-grid GPU operators remain deferred.
4. Fleet movement/combat remains deferred.
5. Studio presentation of RF/tick/effect/replay proof reports remains deferred.

## DA status

Not DA-promoted.
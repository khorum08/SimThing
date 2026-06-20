# RUNTIME-TICK-HISTORY-REPLAY-0 Results

## Status

PASS — focused validation complete; PR #804 merged.

## PR / branch / merge

- Branch: `runtime-tick-history-replay-0`
- PR: #804
- Merge SHA: `d787a1c5782b47332f9c020c23b77fbb6982d047`

## Mission

Add deterministic runtime tick history/replay evidence over tick shell and local participant effects. Record Scenario authority digest, tick id, stage summaries, RF/effect totals, deferred flags, and entry digest; prove replay determinism without Scenario mutation, savefile mutation, or persistent timeline storage.

## Pre-flight metadata check

PASS — #803 (`LOCAL-PARTICIPANT-EFFECTS-0`) post-merge metadata verified on current `master`:

- No `TBD` placeholders in `docs/tests/local_participant_effects_0_results.md`, `docs/tests/current_evidence_index.md`, or `docs/0.8.3 Simthing Studio Production.md` for #803.
- LOCAL-PARTICIPANT-EFFECTS-0 section present in production doc with PR #803 and merge SHA.

## Doctrine preservation

All 16 reaffirmed RF/location/owner-channel doctrine points preserved. History/replay is runtime/proof metadata only; no economy engine, no Scenario mutation, no participant property mutation, no savefile persistence, no Studio GPU dispatch.

## Implemented changes

- `crates/simthing-spec/src/spec/runtime_tick_history.rs` — `RuntimeTickHistoryEntry`, `RuntimeTickReplayReport`, `evaluate_runtime_tick_history_entry`, `replay_runtime_tick_history`, `scenario_authority_digest` (FNV-1a over canonical serialization).
- `crates/simthing-driver/src/runtime_tick_history_compile.rs` — `RuntimeTickHistoryPlan`, `compile_runtime_tick_history_plan`.
- `crates/simthing-spec/src/spec/scenario_ingestion.rs` — `runtime_tick_history_ready` / `runtime_tick_history_deferred`.
- Spec tests: 11 pass. Driver tests: 10 pass.

## Runtime tick history model

`evaluate_runtime_tick_history_entry` composes runtime tick shell and local participant effects into a deterministic entry with `scenario_authority_digest` (FNV-1a of serialized authority) and `entry_digest` (FNV-1a of typed entry fields). `replay_runtime_tick_history` evaluates the same tick `replay_count` times (bounded 1..=64) and verifies matching entry digests.

## Deterministic replay proof

Fixture `owner_silo_disburse_down_scoped.simthing-scenario.json` at tick_id=1, replay_count=3:

- all_replays_match = true
- local_effect_count = 3, allocated_total = 72, unmet_total = 8
- satisfied_count = 2, unsatisfied_count = 1
- all deferred flags true, persistent_history_deferred = true
- entry digest stable across repeated evaluations; changes when Scenario authority changes

## GPU proof path

GPU proof remains stage-local over existing AccumulatorOp surfaces. `RuntimeTickHistoryPlan` records deterministic CPU replay evidence and references existing stage-local proof availability; it does not introduce a fused replay kernel. No new GPU primitive or WGSL.

## Boundary / non-goals

No savefile mutation, persistent timeline storage, full runtime sim loop, economy execution, participant property mutation, Studio GPU dispatch, new GPU primitives, new WGSL, or privileged engine tokens. No MapGenerator/ClauseThing/Terran Pirate fixture changes. Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test runtime_tick_history` | PASS (11) |
| `cargo test -p simthing-spec --test local_participant_effects` | PASS (12) |
| `cargo test -p simthing-spec --test runtime_tick_shell` | PASS (8; 1 ignored) |
| `cargo test -p simthing-spec --test runtime_rf_tick` | PASS (8; 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test runtime_tick_history` | PASS (10) |
| `cargo test -p simthing-driver --test local_participant_effects` | PASS (11) |
| `cargo test -p simthing-driver --test runtime_tick_shell` | PASS (10) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/runtime_tick_history.rs` (new)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/runtime_tick_history.rs` (new)
- `crates/simthing-driver/src/runtime_tick_history_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/runtime_tick_history.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/runtime_tick_history_replay_0_results.md` (new)

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/runtime_tick_history_replay_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |

## Known gaps

- Studio presentation of tick history/replay proof preview deferred.
- Persistent timeline/savefile storage not implemented.
- Real local effect application semantics not yet implemented.

## Deferred next rung

1. Local effect application semantics remain deferred until explicit authority boundary proof.
2. Runtime tick persistent history/replay storage remains deferred.
3. Star-system local-grid GPU operators remain deferred.
4. Fleet movement/combat remains deferred.
5. Studio presentation of RF/tick/effect/replay proof reports remains deferred.

## DA status

Not DA-promoted.
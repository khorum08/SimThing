# RECURSIVE-LOCAL-RF-EVALUATOR-0 Results

## Status

PASS — focused validation complete; merge pending.

## PR / branch / merge

- Branch: `recursive-local-rf-evaluator-0`
- PR: TBD
- Merge SHA: TBD

## Mission

Implement proof-only recursive local RF evaluator over Location gridcell SimThings while preserving prior planet-child RF, owner-silo, tick, effect, and semantic proof channels.

## Pre-flight metadata check

PASS — #806 (`SEMANTIC-LOCAL-EFFECT-TYPES-0`) post-merge metadata verified on current `master`:

- No `TBD` placeholders in `docs/tests/semantic_local_effect_types_0_results.md`, `docs/tests/current_evidence_index.md`, or `docs/0.8.3 Simthing Studio Production.md` for #806.
- SEMANTIC-LOCAL-EFFECT-TYPES-0 section present in production doc with PR #806 and merge SHA.

## Current implementation gap closed

The codebase implemented recursive Location/gridcell ontology but did not implement a generic recursive RF evaluator over arbitrary Location gridcells. This rung adds `evaluate_recursive_local_rf` treating every spatial gridcell Location as a local RF arena, aggregating child Location outputs at parent Locations, and demonstrating sibling surplus-to-deficit redistribution before net bubbling upward.

## Prior channel-closure audit

- Local gridcell nodes are Location SimThings with interior local grids.
- Star-system gridcells default to 10×10 interior grids; other spatial gridcells default to 1×1.
- `planet_child_rf` and `owner_silo_disburse_down` remain compatibility/proof slices.
- Runtime tick/effect/semantic rungs consume that proof slice and were not broken.
- Generic resource channel support added via `OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID` with `"generic"` fallback.
- No previous rung prevents recursive Location-gridcell arena evaluation.

## Prior ladder compatibility proof

- `owner_silo_disburse_down_scoped` fixture: planet-child participants found in recursive evaluator (4/4).
- Semantic local effects totals unchanged after recursive evaluation on same fixture.
- Compatibility adapter `recursive_local_rf_participant_rows_from_planet_child_inputs` preserves owner/resource/surplus/deficit mapping.

## Doctrine preservation

All 17 reaffirmed RF/location/owner-channel doctrine points preserved. Recursive evaluation is proof/runtime state only; no economy engine, no Scenario mutation, no participant property mutation, no savefile persistence, no Studio GPU dispatch.

## Implemented changes

- `crates/simthing-spec/src/spec/recursive_local_rf.rs` — recursive evaluator, authority proof, compatibility bridge.
- `crates/simthing-spec/src/spec/scenario.rs` — `OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID`, `owner_flow_resource_key`.
- `crates/simthing-driver/src/recursive_local_rf_compile.rs` — `RecursiveLocalRfPlan`, GPU aggregate proof.
- `crates/simthing-spec/tests/sibling_redistribution_fixture.rs` — sibling redistribution test fixture builder.
- Spec tests: 17 pass. Driver tests: 10 pass (GPU adapter observed when available).

## Recursive Location evaluator model

Every spatial gridcell Location acts as a local RF evaluator nexus. Each arena gathers direct participant rows and child Location RF outputs, groups by owner/resource, resolves surplus against demand locally, and emits only net surplus/deficit upward.

## Sibling redistribution proof

Sibling fixture `recursive_local_rf_sibling_redistribution` (in-memory test builder):

- owner_a / food: planet_a net_surplus 30, planet_b net_deficit 20
- star_system locally_matched_total 20, net_surplus_to_parent 10, net_deficit_to_parent 0

## Resource-key metadata / fallback proof

- Explicit `OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID` used when present (sibling fixture uses `"food"`).
- Missing resource key falls back to `"generic"` (disburse-down fixture compatibility preserved).

## Authority preservation proof

`prove_recursive_local_rf_preserves_authority` verifies Scenario authority digest before == after. No SimThing properties mutated. No savefile written.

## GPU proof path

GPU proof covers aggregate arena surplus/demand totals per owner/resource via existing AccumulatorOp. CPU oracle defines recursive settlement and bubbling semantics. No new GPU primitive or WGSL.

## Boundary / non-goals

No ScenarioSpec mutation, participant property mutation, savefile/persistent timeline storage, semantic economy execution, consumption, supply effects, combat, movement, Studio GPU dispatch, new GPU primitives, new WGSL, or privileged engine tokens. No MapGenerator/ClauseThing/Terran Pirate fixture changes. Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test recursive_local_rf` | PASS (17) |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (25) |
| `cargo test -p simthing-spec --test runtime_local_allocation` | PASS (9; 1 ignored) |
| `cargo test -p simthing-spec --test local_effect_application` | PASS (12) |
| `cargo test -p simthing-spec --test semantic_local_effects` | PASS (12) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test recursive_local_rf` | PASS (10) |
| `cargo test -p simthing-driver --test local_effect_application` | PASS (12) |
| `cargo test -p simthing-driver --test semantic_local_effects` | PASS (12) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/recursive_local_rf.rs` (new)
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/recursive_local_rf.rs` (new)
- `crates/simthing-spec/tests/sibling_redistribution_fixture.rs` (new)
- `crates/simthing-driver/src/recursive_local_rf_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/recursive_local_rf.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/recursive_local_rf_evaluator_0_results.md` (new)

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/recursive_local_rf_evaluator_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |

## Known gaps

- Planet-child RF ladder not yet reconciled with recursive evaluator as single RF truth source.
- Recursive evaluator not integrated into runtime tick shell.
- Studio presentation of recursive RF proof preview deferred.

## Deferred next rung

1. Reconcile planet-child RF ladder with recursive local RF evaluator outputs.
2. Integrate recursive local RF evaluator into runtime tick shell as optional source of RF truth.
3. Semantic effect execution authority remains deferred until recursive RF evaluator is integrated into tick shell.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive RF proof reports remains deferred.

## DA status

Not DA-promoted.
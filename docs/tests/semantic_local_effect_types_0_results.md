# SEMANTIC-LOCAL-EFFECT-TYPES-0 Results

## Status

PASS — focused validation complete; PR #806 merged.

## PR / branch / merge

- Branch: `semantic-local-effect-types-0`
- PR: #806
- Merge SHA: `51c9080de837553649915643be29807808e27b4c`

## Mission

Define runtime/proof-only typed semantic local effect outputs derived from local effect application records. Introduce ResourceSatisfied, ResourceShortfall, and RuntimeAppliedAmount effect kinds with deterministic semantics while preserving Scenario authority and deferring semantic execution, participant property mutation, and savefile mutation.

## Pre-flight metadata check

PASS — #805 (`LOCAL-EFFECT-APPLICATION-AUTHORITY-0`) post-merge metadata verified on current `master`:

- No `TBD` placeholders in `docs/tests/local_effect_application_authority_0_results.md`, `docs/tests/current_evidence_index.md`, or `docs/design_0_0_8_3_studio_production.md` for #805.
- LOCAL-EFFECT-APPLICATION-AUTHORITY-0 section present in production doc with PR #805 and merge SHA.

## Doctrine preservation

All 16 reaffirmed RF/location/owner-channel doctrine points preserved. Typed semantic outputs are runtime/proof only; no economy engine, no Scenario mutation, no participant property mutation, no savefile persistence, no Studio GPU dispatch.

## Implemented changes

- `crates/simthing-spec/src/spec/semantic_local_effects.rs` — `SemanticLocalEffectKind`, `SemanticLocalEffectOutput`, `SemanticLocalEffectReport`, `semantic_local_effects_from_application`, `evaluate_semantic_local_effects`, `prove_semantic_local_effects_preserve_authority`, `semantic_local_effects_aggregate_totals`.
- `crates/simthing-driver/src/semantic_local_effects_compile.rs` — `SemanticLocalEffectsPlan`, `compile_semantic_local_effects_plan`, GPU aggregate proof helpers reusing AccumulatorOp.
- `crates/simthing-spec/src/spec/scenario_ingestion.rs` — `semantic_local_effects_ready` / `semantic_local_effects_deferred`.
- Spec tests: 12 pass. Driver tests: 12 pass (GPU adapter observed when available).

## Typed semantic effect model

`RuntimeAppliedAmount` records quantity applied at runtime. `ResourceSatisfied` records semantic satisfaction status. `ResourceShortfall` records unmet demand. Every output preserves source SimThing id, owner/resource/scope, and deferral flags.

Fixture `owner_silo_disburse_down_scoped.simthing-scenario.json` at tick_id=1:

- output_count 6, runtime_applied_total 72, shortfall_total 8
- satisfied_output_count 2, shortfall_output_count 1
- owner_a cohort: RuntimeAppliedAmount 20, ResourceSatisfied 20
- owner_a fleet: RuntimeAppliedAmount 42, ResourceShortfall 8
- owner_b cohort: RuntimeAppliedAmount 10, ResourceSatisfied 10

## Authority preservation proof

`prove_semantic_local_effects_preserve_authority` verifies Scenario authority digest before == after. No SimThing properties mutated. No savefile written.

## GPU proof path

GPU proof covers aggregate runtime_applied and shortfall totals per owner/resource via existing AccumulatorOp. CPU oracle defines typed semantic output and deferral semantics. Zero-shortfall channels use one zero input slot for AccumulatorOp validity. No new GPU primitive or WGSL.

## Boundary / non-goals

No ScenarioSpec mutation, participant property mutation, savefile/persistent timeline storage, semantic economy execution, consumption, supply effects, combat, movement, Studio GPU dispatch, new GPU primitives, new WGSL, or privileged engine tokens. No MapGenerator/ClauseThing/Terran Pirate fixture changes. Studio presentation deferred.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test semantic_local_effects` | PASS (12) |
| `cargo test -p simthing-spec --test local_effect_application` | PASS (12) |
| `cargo test -p simthing-spec --test runtime_tick_history` | PASS (11) |
| `cargo test -p simthing-spec --test local_participant_effects` | PASS (12) |
| `cargo test -p simthing-spec --test runtime_tick_shell` | PASS (8; 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12; 1 ignored) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test semantic_local_effects` | PASS (12) |
| `cargo test -p simthing-driver --test local_effect_application` | PASS (12) |
| `cargo test -p simthing-driver --test runtime_tick_history` | PASS (10) |
| `cargo test -p simthing-driver --test local_participant_effects` | PASS (11) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/semantic_local_effects.rs` (new)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/semantic_local_effects.rs` (new)
- `crates/simthing-driver/src/semantic_local_effects_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/semantic_local_effects.rs` (new)
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/semantic_local_effect_types_0_results.md` (new)

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/semantic_local_effect_types_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/design_0_0_8_3_studio_production.md` | living production synthesis |

## Known gaps

- Studio presentation of typed semantic local effects preview deferred.
- Semantic effect execution (consumption/supply mutation) not yet implemented.
- Persistent timeline/savefile storage not implemented.

## Deferred next rung

1. Semantic effect execution authority boundary remains deferred until explicit mutation rung.
2. Runtime tick persistent history/replay storage remains deferred.
3. Star-system local-grid GPU operators remain deferred.
4. Fleet movement/combat remains deferred.
5. Studio presentation of RF/tick/effect/replay proof reports remains deferred.

## DA status

Not DA-promoted.
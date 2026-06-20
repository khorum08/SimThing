# SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `scenario-canonical-load-save-roundtrip-0`
- PR: #828
- Merge SHA: `ee651acdf4b4a0e94b707300974d285ea2664754`

## Mission

Prove headless ScenarioSpec canonical JSON load/save roundtrip suitable for Studio import/export.

## Pre-flight metadata check

- #826/#827 metadata verified on `master` for SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0.

## Anti-loop production-path statement

This rung proves ScenarioSpec file I/O correctness for Studio import/export. It does not implement savefile persistence, runtime mutation, semantic execution, Studio UI, or Studio GPU dispatch.

## Canonical load/save model

- Added `ScenarioCanonicalLoadReport`, `ScenarioCanonicalSaveReport`, `ScenarioCanonicalRoundtripReport`.
- Added `load_scenario_spec_from_json_str`, `save_scenario_spec_to_canonical_json`, `prove_scenario_canonical_load_save_roundtrip`.
- Driver: `compile_scenario_canonical_io_plan_from_json_str` reports Studio import/export readiness with runtime/savefile deferrals.

## Authority digest stability proof

- Initial and roundtrip authority digests match after canonical serialize/reload — PASS.

## Ingestion readiness proof

- Studio canonical ingestion profile evaluates admission on load — PASS.

## Fixture safety proof

- Tests read corpus fixtures only; temp-dir writes used for file roundtrip — PASS.

## Boundary / non-goals

- No semantic execution, runtime mutation, or savefile/persistent history format.
- No Studio UI or GPU dispatch.
- No MapGenerator/ClauseThing or Terran Pirate fixture edits.

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test scenario_canonical_io` | PASS (7/7) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test scenario_canonical_io` | PASS (4/4) |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/scenario_canonical_io.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/scenario_canonical_io.rs`
- `crates/simthing-driver/src/scenario_canonical_io_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/scenario_canonical_io.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/scenario_canonical_load_save_roundtrip_0_results.md`

## Evidence lifecycle

PROBATION — pending merge and DA review.

## Known gaps

- Savefile/persistent history format not introduced.
- Studio UI presentation not introduced.
- Runtime tick execution not introduced.

## Deferred next rung

1. Wire Studio import/export UI to canonical I/O plan.
2. Evaluate savefile/persistent history boundary separately from ScenarioSpec canonical JSON.

## DA status

Not submitted — evidence PROBATION pending DA review.
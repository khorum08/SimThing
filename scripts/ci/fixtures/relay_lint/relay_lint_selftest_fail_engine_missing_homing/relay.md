## Status

PROBATION / proof-present.

## PR / branch / merge

- PR: #2010
- Branch: relay-engine-homing-missing
- Merge: orchestrator clearable

## What changed

- Adds an engine source file while omitting the dedicated placement classification.

## Load-bearing proofs

tested_code_sha: aaaaaaaa11111111222222223333333344444444
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| crates/simthing-kernel/src/new_generic_surface.rs | generic engine surface |

## Conformance

- The general scope ledger is present but cannot substitute for homing classification.

## Known gaps / next

- Dedicated homing classification intentionally omitted.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE |
| Triage entries | none |
| Risk class | semantic |
| Falsification check | relay_lint fixture |
| Recommended posture | light |

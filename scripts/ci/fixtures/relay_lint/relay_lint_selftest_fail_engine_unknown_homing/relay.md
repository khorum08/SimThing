## Status

PROBATION / proof-present.

## PR / branch / merge

- PR: #2011
- Branch: relay-engine-homing-unknown
- Merge: orchestrator clearable

## What changed

- Adds an engine source file with an unknown homing declaration.

## Load-bearing proofs

tested_code_sha: aaaaaaaa11111111222222223333333344444444
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| crates/simthing-kernel/src/new_generic_surface.rs | generic engine surface |

## Homing Boundary Classification

Homing-Boundary: generic-engine-helper

## Conformance

- Unknown closed-vocabulary value is intentionally planted.

## Known gaps / next

- None beyond the planted declaration defect.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE |
| Triage entries | none |
| Risk class | semantic |
| Falsification check | relay_lint fixture |
| Recommended posture | light |

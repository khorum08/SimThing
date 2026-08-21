## Status

PROBATION / proof-present.

## PR / branch / merge

- PR: #2013
- Branch: relay-workshop-elevation-missing-origin
- Merge: held for DA review

## What changed

- Claims a workshop elevation without identifying its workshop origin.

## Load-bearing proofs

tested_code_sha: aaaaaaaa11111111222222223333333344444444
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| crates/simthing-kernel/src/elevated_surface.rs | workshop elevation |

## Homing Boundary Classification

Homing-Boundary: workshop-elevation(WORKSHOP-ELEVATION-FIXTURE-0)

## Conformance

- Missing origin evidence is intentionally planted.

## Known gaps / next

- Homing-Origin is intentionally omitted.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE |
| Triage entries | none |
| Risk class | semantic |
| Falsification check | relay_lint fixture |
| Recommended posture | deep |

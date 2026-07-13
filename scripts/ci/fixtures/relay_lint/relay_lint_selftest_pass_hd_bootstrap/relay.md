## Status

PROBATION / proof-present.
Rung: HD-BOOTSTRAP-FIXTURE-0
HD-RECEIPT: 2d467056f951

## PR / branch / merge

- PR: #2101
- Branch: relay-hd-bootstrap
- Merge: not merged

## What changed

- Introduces one handoff object for the rung.

## Load-bearing proofs

tested_code_sha: 1111111122222222333333334444444455555555
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| handoffs/HD-BOOTSTRAP-FIXTURE-0.hd.md | relay classification |

## Conformance

- Bootstrap handoff receipt matches head object.

## Homing Boundary Classification

| Symbol | Classification |
|---|---|
| relay | relay classification |

## Known gaps / next

- None.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE |
| Triage entries | none |
| Risk class | semantic |
| Falsification check | relay_lint fixture |
| Recommended posture | light |

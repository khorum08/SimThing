## Status

PROBATION / proof-present / DA-review-pending.

## PR / branch / merge

- PR: #2000
- Branch: relay-missing-clearance
- Merge: held for DA review

## What changed

- Relay fixture shaped like a DA-review escalation without a clearance verdict.

## Load-bearing proofs

tested_code_sha: 1111111122222222333333334444444455555555
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| docs/tests/relay.md | relay classification |

## Conformance

- Fixture carries DA-review semantics.

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
| Recommended posture | deep |

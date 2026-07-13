## Status

PROBATION / proof-present.
Rung: HD-DRIFT-FIXTURE-0
HD-RECEIPT: deadbeefcafe

## PR / branch / merge

- PR: #2102
- Branch: relay-hd-drift
- Merge: not merged

## What changed

- Claims a receipt that no longer matches the base handoff.

## Load-bearing proofs

tested_code_sha: 2222222233333333444444445555555566666666
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| handoffs/HD-DRIFT-FIXTURE-0.hd.md | relay classification |

## Conformance

- Drift must fail before relay acceptance.

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

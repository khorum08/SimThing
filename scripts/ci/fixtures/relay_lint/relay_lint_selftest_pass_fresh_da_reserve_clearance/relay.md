## Status

PROBATION / proof-present / DA-review-pending.

## PR / branch / merge

- PR: #2002
- Branch: relay-fresh-clearance
- Merge: held for DA review

## What changed

- Relay fixture carries a fresh DA-RESERVE verdict bound to the tested head.

## Load-bearing proofs

tested_code_sha: cccccccc11111111222222223333333344444444
clearance_pr_head: cccccccc11111111222222223333333344444444
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| scripts/ci/relay_lint.sh | relay classification |

## Conformance

- Fixture carries DA-review semantics with fresh clearance binding.

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

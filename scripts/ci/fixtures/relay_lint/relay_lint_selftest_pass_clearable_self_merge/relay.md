## Status

PROBATION / proof-present / orchestrator-clearable.

## PR / branch / merge

- PR: #2011
- Branch: relay-clearable-self-merge
- Merge: orchestrator self-merge after green checks

## What changed

- Exercises the non-DA self-merge route with an explicit clearance verdict.

## Load-bearing proofs

tested_code_sha: bbbbbbbb11111111222222223333333344444444
clearance_pr_head: bbbbbbbb11111111222222223333333344444444
CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE
coverage_basis: PASS - fixture proof only

## Scope Ledger

| Path | Classification |
|---|---|
| docs/tests/relay.md | relay classification |

## Conformance

- The clearance class and merge route both name orchestrator self-merge.

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
| Risk class | ordinary |
| Falsification check | relay_lint fixture |
| Recommended posture | light |

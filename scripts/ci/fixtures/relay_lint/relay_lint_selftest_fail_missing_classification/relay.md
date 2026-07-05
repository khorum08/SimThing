## Status

**PROBATION** — missing classification tables.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | #1154 |

## What changed

- Fleet movement implementation.

## Load-bearing proofs (+ what each catches)

| Proof | Catches |
|---|---|
| movement test | reparent chain |

tested_code_sha: 5b03bfb1948d315b49a14a97cbe38f60ef08112d
coverage_basis: PASS — docs-only commits after tested SHA

## Scope Ledger

| Path | Notes |
|---|---|
| workshop only | no engine edits |

## Conformance (spine/D-directives held)

- Gradient movement only.

## Known gaps / next

- homing/scope tables omitted intentionally for fixture.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE |
| Triage entries | none |
| Risk class | semantic |
| Falsification check | run tests |
| Recommended posture | deep |
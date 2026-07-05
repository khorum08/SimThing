## Status

**PROBATION** — relay missing proof identity fields.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | #1154 |
| Branch | `tp-fleet-movement-0` |

## What changed

- Fleet movement post-hydration over PALMA reach.

## Load-bearing proofs (+ what each catches)

| Proof | Catches |
|---|---|
| gradient movement test | multi-tick reparent |

## Scope Ledger

| Path | Classification |
|---|---|
| workshop | workshop-homed |

## Conformance (spine/D-directives held)

- No CPU planner; gradient reparent only.

## Homing Boundary Classification

| Symbol | Classification |
|---|---|
| fleet hydrator | workshop-homed |

## Known gaps / next

- awaiting proof fields.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE |
| Triage entries | none |
| Risk class | semantic |
| Falsification check | run tp_fleet_movement_0 tests |
| Recommended posture | deep |
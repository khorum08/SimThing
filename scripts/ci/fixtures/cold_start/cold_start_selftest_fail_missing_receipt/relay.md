## Status

**PROBATION / DA-OWNER REVIEW** — Phase 7 blocked until DA clearance of Phase 6.2 fleet movement.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | [#1154](https://github.com/khorum08/SimThing/pull/1154) |
| Branch | `tp-fleet-movement-0` |
| Merge | held pending DA clearance |

## What changed

- Added `fleet_movement_post_hydration.rs` — multi-tick D-gradient fleet reparenting over PALMA reach.
- Enlarged theater to 7×7 / horizon 3; horizon truncation engages.
- Workshop-homed tests in `tp_fleet_movement_0.rs`; no engine-crate edits.

## Load-bearing proofs (+ what each catches)

| Proof | Catches |
|---|---|
| `fleet_traverses_three_cells_over_three_ticks_down_d_gradient` | Multi-step gradient movement |
| `arena_reenrollment_follows_each_reparent` | Arena re-enrollment after reparent |
| `fleet_movement_gpu_matches_cpu_oracle_on_larger_theater` | GPU==CPU on 7×7 theater |

```
DOCTRINE-TESTS-VERDICT: PASS
tested_code_sha: 5b03bfb1948d315b49a14a97cbe38f60ef08112d
coverage_basis: PASS — commits after tested_code_sha are docs/evidence-only and do not affect the tested binary
```

## Scope Ledger

| Path | Classification | Notes |
|---|---|---|
| `simthing-workshop/**` | workshop-homed scenario candidate | no engine edits |
| engine crates | untouched | no substrate widening |

## Conformance (spine/D-directives held)

- Movement is gradient-following reparenting; no route solver or CPU planner.
- No engine-crate gameplay semantics; Mechanism B consumer-side application only.

## Homing Boundary Classification

| Symbol | Classification | Action |
|---|---|---|
| `apply_fleet_movement_post_hydration` | workshop-homed scenario candidate | keep in workshop |
| engine crate source | untouched | no engine edits |

## Known gaps / next

- Phase 7 (`TP-COMMITMENTS-0`) blocked until DA graduation of this rung.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE — doctrine scan green at head |
| Triage entries | none |
| Risk class | gate-wiring |
| Falsification check | `cargo test -p simthing-workshop --test tp_fleet_movement_0` → 5/5 PASS at tested_code_sha |
| Recommended posture | deep — first live fleet movement over PALMA reach |
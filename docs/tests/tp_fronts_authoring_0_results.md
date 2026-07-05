# TP-FRONTS-AUTHORING-0 Results

## Status

**PROBATION / DA-OWNER REVIEW — not self-mergeable.** Phase 6.1 (`TP-PALMA-REACH-0`, `TP-FLEET-MOVEMENT-0`) blocked until DA clearance of 6.0.

## Identity

| Field | Value |
|---|---|
| PR | [#1151](https://github.com/khorum08/SimThing/pull/1151) |
| Branch | `tp-fronts-authoring-0` |
| Base | `origin/master` @ `9aa66c39152c36c688fd66cf4eef723b1780c089` |
| Tested code SHA | `862d06a41cdebd070b3ecccade2522620ef77aff` |
| Current PR head | `8abbf0f6491d2d4aee3fb5c796873bf79d52fcba` |
| Rung | Phase 6.0 `TP-FRONTS-AUTHORING-0` |
| Mechanism | **B — consumer-side application** from `simthing-workshop` |

## Mechanism B — consumer-side application

| Stage | Location | Notes |
|---|---|---|
| Base ClauseThing hydration | `hydrate_scenario` on generic TP clause + fleet payloads | No `simthing-clausething/src` front semantics |
| Workshop post-hydration | `simthing-workshop/src/fronts_post_hydration.rs` | Threat / suppression / disruption RF + Movement-Front surfaces |
| Test driver | `simthing-workshop/tests/tp_fronts_authoring_0.rs` | Applies hydrator after base pack |

**Base ClauseThing pipeline took no scenario-specific front edits.**

## Homing Boundary Classification

| Symbol / path | Would this exist without TP? | Classification | Action |
|---|---:|---|---|
| `apply_fronts_post_hydration` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `fronts_post_hydration.rs` theater/border selection | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `TP_THREAT_ARENA` / `TP_SUPPRESSION_ARENA` / `TP_DISRUPTION_ARENA` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `contested_border_settling_oracle` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `tp_fronts_authoring_0.rs` test driver | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `simthing-clausething/src/**` | — | untouched | no engine front semantics |
| `simthing-spec/src/**` | — | untouched | no substrate widening |
| `simthing-kernel/src/**` | — | untouched | no substrate widening |
| `simthing-sim/src/**` | — | untouched | no substrate widening |
| `simthing-driver/src/**` | — | dev-dep test imports only | no engine front semantics |
| `simthing-gpu/src/**` | — | dev-dep test imports only | no engine front semantics |

Engine source edits: **none.** Generic substrate widening: **none.** Gameplay semantics in engine crates: **no.**

## Substrate widening

| Item | Status |
|---|---|
| Engine crates touched | **none** |
| Generic future-utility justification | n/a — workshop-only delta |
| Gameplay semantics in engine crates | **no** |

## Front definitions

| Front | RF arena | Seed source | Theater column |
|---|---|---|---|
| Threat | `tp_front_threat` | Pirate fleet `weapon_damage_seed` (fallback 30) | pirate col 2 |
| Suppression | `tp_front_suppression` | Terran fleet `weapon_damage_seed` (fallback 40) | terran col 0 |
| Disruption | `tp_front_disruption` | Pirate posture feedstock (fallback 28) | pirate col 2 |

Bounded theater: one terran + one pirate contested-border system pair, 3×3 grid (STEAD §7 P1).

## On-device seed proof

| Proof | Test | Result |
|---|---|---|
| GPU indexed scatter == CPU projection oracle | `fronts_seed_from_arena_pressure_on_device` | PASS — bit-identical per arena binding (threat / suppression / disruption) |
| No readback-seeded fake field | — | yes — scatter reads installed RF intrinsic-flow columns |

## Settling contour proof

| Proof | Test | Result |
|---|---|---|
| Suppression vs disruption contour | `contested_boundary_settles_suppression_vs_disruption` | PASS — CPU horizon oracle shows non-zero contested-column mass; authored suppression > disruption; GPU field bit-identical to CPU oracle |
| CPU oracle authority | `contested_border_settling_oracle` | PASS |

## Candidate F exact-magnitude proof

| Proof | Test | Result |
|---|---|---|
| Candidate F bits authoritative | `candidate_f_exact_magnitude_gate_is_authoritative` | PASS — `sqrt_cr_f_bits` gate differs from native `sqrt` diagnostic |

## L3 urgency pressure

| Proof | Test | Result |
|---|---|---|
| L3 column exists and responds | `front_l3_urgency_pressure_updates_without_cpu_planner` | PASS — `field_urgency` parent formula; urgency + L2 pressure change with added disruption seed |
| No Phase 7 commitment semantics | — | yes — probe commitment threshold 10_000 / event `FRON`; tests use diagnostic readback only |

## No CPU planner proof

| Proof | Result |
|---|---|
| No attack/defend/raid/withdraw loop | yes — RF seed + Movement-Front tick + oracle comparisons only |
| No route/path/predecessor object | yes |

## Rustification / lifecycle

| Test | birth_track | class | verdict |
|---|---|---|---|
| `workshop_post_hydration_application_is_required` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `fronts_seed_from_arena_pressure_on_device` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `contested_boundary_settles_suppression_vs_disruption` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `front_l3_urgency_pressure_updates_without_cpu_planner` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `candidate_f_exact_magnitude_gate_is_authoritative` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |

## Citable GPU proof

```
DOCTRINE-TESTS-VERDICT: PASS
tested_code_sha: 862d06a41cdebd070b3ecccade2522620ef77aff
current_pr_head: 8abbf0f6491d2d4aee3fb5c796873bf79d52fcba
coverage_basis: PASS — commits after tested_code_sha are docs/evidence-only and do not affect the tested binary
profile: owner-local GPU / tp_fronts_authoring_0
owner_local: true
proof: fronts_seed_from_arena_pressure_on_device
result: PASS
```

## Load-bearing proofs (owner-local 2026-07-05)

| Command | Result |
|---|---|
| `cargo check -p simthing-workshop` | PASS |
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-workshop --test tp_fronts_authoring_0 -- --nocapture` | PASS (5/5) |
| `test_inventory_check` | INSPECT (2 pre-existing fixture extra rows; 0 missing) |
| `test_inventory_drift_check` | PASS |
| `test_lifecycle_boundary_check` | PASS |
| `test_lifecycle_expiry_check --schema` | PASS |
| `test_lifecycle_expiry_check --prove` | PASS |
| `gen_digest --check` | PASS |
| `doctrine_scan` | PASS (0 hard failures; 415 HEURISTIC INSPECT — pre-existing corpus) |
| `git diff --check origin/master...HEAD` | PASS |

## Known gaps / next

- PALMA reach (`TP-PALMA-REACH-0`): not started — blocked.
- Fleet movement (`TP-FLEET-MOVEMENT-0`): not started — blocked.
- Phase 6.1: blocked pending DA clearance of 6.0.

## Graduation routing

DA may graduate 6.0 when all handoff gates are green under owner review. Merge remains **held** until DA/Owner clearance.
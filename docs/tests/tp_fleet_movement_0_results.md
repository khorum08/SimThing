# TP-FLEET-MOVEMENT-0 Results

## Status

**PROBATION / DA-OWNER REVIEW — not self-mergeable.** Phase 7 (`TP-COMMITMENTS-0`) blocked until DA clearance of Phase 6.2.

## Identity

| Field | Value |
|---|---|
| PR | [#1154](https://github.com/khorum08/SimThing/pull/1154) |
| Branch | `tp-fleet-movement-0` |
| Base | `origin/master` @ `335f55c052920a721f18357dca2ce9a594db11db` |
| Tested code SHA | `5b03bfb1948d315b49a14a97cbe38f60ef08112d` |
| Current PR head | `c95ff92e0734c14372214baaa29031ac7d1baeef` |
| Rung | Phase 6.2 `TP-FLEET-MOVEMENT-0` |
| Mechanism | **B — consumer-side application** from `simthing-workshop` |

## Binding DA conditions

| Condition | Evidence | Result |
|---|---|---|
| **B1 — 7×7 / horizon 3 minimum** | `TP_MOVEMENT_GRID_SIZE=7`, `TP_MOVEMENT_HORIZON=3`; theater enlarged from prior 3×3 | PASS |
| **B1 — horizon truncation engages** | `bounded_theater_horizon_truncation_engages`; `horizon_truncation_engages_oracle` asserts zero source-col mass beyond horizon from seeds; seed at (3,3) beyond horizon from corner (6,6) | PASS — STEAD P1 engaged |
| **B2 — multi-step gradient movement** | `fleet_traverses_three_cells_over_three_ticks_down_d_gradient`; ≥3 adjacent reparents over ≥3 ticks following resident D | PASS |
| **B2 — arena re-enrollment** | `arena_reenrollment_follows_each_reparent`; enrollment authority tracks new cell before next tick | PASS |
| **B3 — larger-theater GPU==CPU** | `fleet_movement_gpu_matches_cpu_oracle_on_larger_theater` on 7×7 / horizon 3 | PASS |

## Mechanism

| Stage | Location | Notes |
|---|---|---|
| Base ClauseThing hydration | `hydrate_scenario` on generic TP clause + fleet payloads | No fleet-movement semantics in engine crates |
| Fronts post-hydration | `apply_fronts_post_hydration_with_theater` | 7×7 contested-border theater; threat / suppression / disruption RF |
| PALMA post-hydration | accepted Phase 6.1 chain inside `apply_fleet_movement_post_hydration` | W impedance + resident D over enlarged theater |
| Fleet movement | `fleet_movement_post_hydration.rs` | Local D-gradient probe → adjacent reparent → arena re-enrollment |
| Test driver | `simthing-workshop/tests/tp_fleet_movement_0.rs` | CPU oracle + owner-local GPU parity on larger theater |

**Movement consumes local gradient only:** current cell, N4 neighbors, resident D values, chosen adjacent lower-D cell. No route/path/predecessor object.

**Previous-value handling:** `TpFleetMovementState.prev_coord` explicit column; no EML previous-buffer read.

## Homing Boundary Classification

| Symbol / path | Would this exist without TP? | Classification | Action |
|---|---:|---|---|
| `apply_fleet_movement_post_hydration` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `fleet_movement_gradient_step` / `simulate_fleet_movement_cpu` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `horizon_truncation_engages_oracle` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `arena_enrollment_matches_fleet_cell` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `TpMovementObservation` (path-as-observation) | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `apply_fronts_post_hydration_with_theater` | yes | generic theater-size parameterization over existing fronts apply | keep in `simthing-workshop` |
| `collect_contested_border_systems` | yes | reusable contested-border extraction | keep in `simthing-workshop` |
| `tp_fleet_movement_0.rs` test driver | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| engine crate source (`simthing-clausething`, `simthing-spec`, `simthing-kernel`, `simthing-gpu`, etc.) | — | untouched | no engine edits |

Engine source edits: **none.** Generic substrate widening: **none** (workshop-only theater API parameterization). Gameplay semantics in engine crates: **no.**

## Substrate widening

| Item | Status |
|---|---|
| Engine crates touched | **none** |
| Generic future-utility justification | `apply_fronts_post_hydration_with_theater` is semantic-free theater sizing; no gameplay naming |
| Gameplay semantics in engine crates | **no** |

## Theater / horizon proof

| Field | Value |
|---|---|
| Theater dimensions | **7×7** (`TP_MOVEMENT_GRID_SIZE`) |
| Horizon | **3** (`TP_MOVEMENT_HORIZON` = `TP_FRONTS_DEFAULT_HORIZON`) |
| Reach dest seed | (0, 0) — terran suppression anchor |
| Truncation engagement seed | (3, 3) — terran suppression; Manhattan distance 6 from corner (6,6) > horizon 3 |
| Fleet corridor seeds | pirate threat/disruption along row 3: (6,3), (5,3), (4,3) |
| Zero mass beyond horizon | `horizon_truncation_engages_oracle` — source-col mass `0.0` (bit-exact) for all cells with `min_seed_dist > 3` |
| P1 truncation engaged | yes — prior 3×3 rungs never fired; 7×7 with seed at (3,3) and corner at (6,6) forces beyond-horizon cells |

## Movement path-as-observation, not route-object

`TpMovementObservation` records tick coordinates only. No route object, path object, predecessor map, or next_hop table exists. Observation is produced by repeated `fleet_movement_gradient_step` calls.

## Multi-tick D-gradient proof

| Field | Value |
|---|---|
| Fleet start | (6, 3) — `TP_MOVEMENT_FLEET_START` |
| Fleet end (after 3 ticks) | (3, 3) |
| Ticks | 3 gradient steps |
| Cells traversed | 3 — (6,3)→(5,3)→(4,3)→(3,3) |
| D-gradient followed | each step via `palma_reach_gradient_probe`; asserts `sampled_d < current D` and Manhattan distance 1 |
| Adjacent reparent only | yes — no teleporting |

## Arena re-enrollment proof

After each `fleet_movement_gradient_step`:

- `state.enrolled_system_id` updates to the new theater cell's `SimThingId`
- `enrollment.authoritative_coord` and `enrollment.authoritative_system_id` match fleet state
- Pre-tick and post-tick `arena_enrollment_matches_fleet_cell` assertions pass for all 3 ticks
- Old cell enrollment is superseded by new authoritative enrollment before the next tick

## GPU==CPU proof on larger theater

```
DOCTRINE-TESTS-VERDICT: PASS
tested_code_sha: 5b03bfb1948d315b49a14a97cbe38f60ef08112d
current_pr_head: c95ff92e0734c14372214baaa29031ac7d1baeef
coverage_basis: PASS — commits after tested_code_sha are docs/evidence-only and do not affect the tested binary
profile: owner-local GPU / tp_fleet_movement_0
owner_local: true
proof: fleet_movement_gpu_matches_cpu_oracle_on_larger_theater
result: PASS
```

GPU D field bit-exact vs CPU oracle on 7×7 theater; movement observation sequence matches CPU oracle using GPU-gathered D.

## Forbidden route/path/predecessor proof

| Proof | Test | Result |
|---|---|---|
| No forbidden identifiers in definitions | `forbidden_route_path_predecessor_tokens_absent` | PASS |
| No route surfaces on pack | — | yes — `palma_feedstock` only; no path/route objects |

## No Phase 7 leakage

| Proof | Result |
|---|---|
| No attack/raid/withdraw/reinforce decision loop | yes |
| No `ai_will_do` commitment semantics | yes |
| No CPU movement planner | yes — gradient probe + reparent only |
| No new opcode / WGSL / AccumulatorRole | yes |

## Rustification / lifecycle

| Test | birth_track | class | verdict |
|---|---|---|---|
| `bounded_theater_horizon_truncation_engages` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `fleet_traverses_three_cells_over_three_ticks_down_d_gradient` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `arena_reenrollment_follows_each_reparent` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `fleet_movement_gpu_matches_cpu_oracle_on_larger_theater` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `forbidden_route_path_predecessor_tokens_absent` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |

All new tests: `dsu_survivals = 0`.

## Load-bearing proofs (owner-local 2026-07-05)

| Command | Result |
|---|---|
| `cargo check -p simthing-workshop` | PASS |
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-workshop --test tp_fleet_movement_0 -- --nocapture` | PASS (5/5) |
| `test_inventory_check` | INSPECT (2 pre-existing fixture extra rows; 0 missing) |
| `test_inventory_drift_check` | PASS |
| `test_lifecycle_boundary_check` | PASS |
| `test_lifecycle_expiry_check --schema` | PASS |
| `test_lifecycle_expiry_check --prove` | PASS |
| `gen_digest --check` | PASS |
| `doctrine_scan` | PASS (0 hard failures; 415 HEURISTIC INSPECT — pre-existing corpus) |
| `git diff --check origin/master...HEAD` | PASS |

## Known gaps / next

- Phase 7 commitments (`TP-COMMITMENTS-0`): blocked pending DA clearance of `TP-FLEET-MOVEMENT-0`.
- Track closeout: Phase 6.2 graduation held for DA/Owner review.

## Graduation routing

DA may graduate Phase 6.2 when all handoff gates are green under owner review. Merge remains **held** until DA/Owner clearance. Phase 7 stays blocked until then.
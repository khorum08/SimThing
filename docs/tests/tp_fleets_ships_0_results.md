# TP-FLEETS-SHIPS-0 Results

## Status

**PROBATION / DA-OWNER REVIEW** — fleet/ship authoring rung. Not self-mergeable; DA/Owner clearance required.

## Identity

| Field | Value |
|---|---|
| PR | (open PR) |
| Branch | `tp-fleets-ships-0` |
| Base | `origin/master` @ `e46cd55f3fe5df74d14419409c6f03868cef3c07` or later |
| Head | Proof run: current branch tip at proof time |

## Scenario-envelope compliance

| Rule | Status |
|---|---|
| `birth_track = 0.0.8.5-terran-pirate` on new tests | yes |
| `TP-FLEETS-SHIPS-0` registered in `test_lifecycle_tracks.tsv` | no |
| TP-born artifacts marked canonical/inviolate/doctrine | no |
| Admission candidates surfaced | yes |

Admission candidates (post-completion review only, not canonized):

1. **Fleet-nested ship RF participant traversal** — `planet_child_rf` now admits `Cohort` children under surface `Fleet` nodes for reduce-up; candidate for general movables doctrine if scenario completes.
2. **Scenario-envelope fleet/ship property ids** (`8_301_500`–`8_301_505`) — candidate for admission only after scenario closeout; not core doctrine.

## Scope

Authoring/hydration only. No combat engine, movement simulation, route solver, diplomacy, AI commitments, GPU, or workspace cargo.

## Implemented authored forms

| Form | Lowering |
|---|---|
| `fleet_ship_payload { owner ownership_volume enemy_ownership_volume fleet_count ships_per_fleet border_fleet_count ship_class hull_seed weapon_damage_seed upkeep_per_ship resource }` | Deterministic fleet home selection from ownership volumes; fleets on surface gridcells; ships as `Cohort` fleet children |
| Fleet posture metadata | `border` / `interior` (Terran), `raid` / `garrison` (Pirate) |
| Ship HP/Damage/upkeep columns | Scenario-envelope property ids on ship `Cohort` nodes |
| Ship upkeep RF | Per-ship `owner_flow_deficit` + resource key; admitted via planet-child RF (including fleet-nested ships) |

## Fleet/ship count proof

| Faction | Fleets | Ships |
|---|---|---|
| Terran | 10 | 200 |
| Pirate | 10 | 400 |

## Distribution proof

| Faction | Rule | Border/raid | Interior/garrison |
|---|---|---|---|
| Terran | 60-40 | 6 fleets (`border`) | 4 fleets (`interior`) |
| Pirate | 80-20 | 8 fleets (`raid`) | 2 fleets (`garrison`) |

Border/raid homes are Chebyshev-adjacent to the enemy ownership volume. Interior/garrison homes are owned systems not adjacent to the enemy volume.

## Tree parentage proof

- Fleets parented under mandated 1×1 planet surface gridcells (DA fleet-homing ruling).
- Ships parented under exactly one fleet (`SimThingKind::Cohort`).
- No ship direct child of star system, owner, or GameSession.

## Owner-reference proof

- Every fleet and ship carries one `owner_flow_owner_ref`.
- Owner refs resolve to GameSession sibling owners (`terran`, `pirate`).
- Owners remain non-spatial (empty children).

## HP/Damage/upkeep column proof

Ship `Cohort` nodes carry scenario-envelope columns:

- `tp::hull` (HP seed)
- `tp::weapon_damage` (Damage seed)
- `tp::upkeep` (upkeep column seed)

Combat resolution is not implemented (Phase 4).

## RF upkeep proof

- Per-ship upkeep lowers through existing `owner_flow_deficit` participant metadata.
- `evaluate_planet_child_rf_admission` and `evaluate_planet_child_rf_reduce_up` admit fleet-nested ship participants.
- No fleet-specific upkeep subsystem.

## Reparent/enrollment compatibility proof

- Table proof reparents one Terran fleet between two owned star-system surfaces.
- Ship children remain under fleet after reparent.
- RF admission remains non-rejected after reparent.
- Full `FissionPolicy` + arena re-enrollment live path deferred to `TP-FLEET-MOVEMENT-0` (documented park).

## Unsupported-field hard-error proof

`unsupported_fleet_ship_payload_fields_hard_error_with_span` — unknown `fleet_ship_payload` fields hard-error with span.

## Test lifecycle / inventory updates

| Test | birth_track | retention basis | downstream consumer note | dsu_survivals |
|---|---|---|---|---|
| `tp_fleets_ships_0_table` | `0.0.8.5-terran-pirate` | `permanent-residue:oracle-parity` | Phase 4 combat rung | 0 |
| `unsupported_fleet_ship_payload_fields_hard_error_with_span` | `0.0.8.5-terran-pirate` | `permanent-residue:oracle-parity` | Phase 4 combat rung | 0 |

## Proof commands

| Command | Result |
|---|---|
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-clausething --test tp_fleets_ships_0 -- --nocapture` | PASS (2 passed) |
| `cargo test -p simthing-clausething --test tp_shipsize_decoder_0 -- --nocapture` | PASS |
| `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 -- --nocapture` | PASS |
| `bash scripts/ci/test_inventory_check.sh` | INSPECT (exit 0; 2 pre-existing extra fixture rows) |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_boundary_check.sh` | PASS (`TEST-LIFECYCLE-BOUNDARY-CHECK-VERDICT: PASS`) |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --prove` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | PASS failures=0 inspect=0 |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `git diff --check origin/master...HEAD` | PASS |

## Scope ledger

| Item | Touched? |
|---|---|
| simthing-clausething | yes — `hydrate_scenario.rs`, tests, fixture |
| simthing-spec | yes — minimal `planet_child_rf.rs` fleet-child traversal |
| simthing-gpu/driver/sim/mapeditor/tools | no |
| workflows / scans / allowlists | no |
| `test_lifecycle_tracks.tsv` | no |
| committed lab corpus | no |
| cargo workspace run | no |

## Graduation routing

- TP-FLEETS-SHIPS-0 complete
- PROBATION / DA-OWNER REVIEW
- DA/Owner clearance required
- not self-mergeable
- next rung after clearance: first Phase 4 combat rung (`TP-COMBAT-ARENA-0`), only if design row permits
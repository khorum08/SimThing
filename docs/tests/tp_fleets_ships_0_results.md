# TP-FLEETS-SHIPS-0 Results

## Status

**PROBATION / DA-OWNER REVIEW** — fleet/ship authoring rung (0R applied). Not self-mergeable; DA/Owner clearance required.

## Identity

| Field | Value |
|---|---|
| PR | [#1138](https://github.com/khorum08/SimThing/pull/1138) |
| Branch | `tp-fleets-ships-0` |
| Base | `origin/master` @ `e46cd55f3fe5df74d14419409c6f03868cef3c07` |
| Head | Proof run: current branch tip at proof time |

## 0R remedial fix (TP-FLEETS-SHIPS-0R)

| Fix | Status |
|---|---|
| `planet_child_rf.rs` Fleet/Cohort kind-specific traversal reverted | done |
| No net-new `SimThingKind::Fleet`/`Cohort` spec hits vs master | done |
| Generic RF recursion to fleet-nested ships | no — current depth is surface gameplay children only |
| §0.6 Deviation recorded for fleet-nested RF reduce-up depth | done |

**DA ruling (0R):** No kind-specific spec code landed. Fleets and ships are ordinary SimThings. RF upkeep is authored as ordinary owner/resource/upkeep metadata on ship nodes. RF reduce-up beyond current generic admission depth is **parked** as a §0.6 Deviation — not repaired with Fleet/Cohort-specific traversal.

## Scenario-envelope compliance

| Rule | Status |
|---|---|
| `birth_track = 0.0.8.5-terran-pirate` on new tests | yes |
| `TP-FLEETS-SHIPS-0` registered in `test_lifecycle_tracks.tsv` | no |
| TP-born artifacts marked canonical/inviolate/doctrine | no |

## Scope

Authoring/hydration only. No combat engine, movement simulation, route solver, diplomacy, AI commitments, GPU, or workspace cargo.

**No kind-specific spec code landed.**

## Implemented authored forms

| Form | Lowering |
|---|---|
| `fleet_ship_payload` | Deterministic fleet home selection; fleets on surface gridcells; ships as ordinary `Cohort` fleet children |
| Fleet posture metadata | `border` / `interior` (Terran), `raid` / `garrison` (Pirate) |
| Ship HP/Damage/upkeep columns | Scenario-envelope property ids on ship nodes |
| Ship upkeep RF metadata | Per-ship `owner_flow_deficit` + resource key (authored; not fleet subsystem) |

## Fleet/ship count proof

| Faction | Fleets | Ships |
|---|---|---|
| Terran | 10 | 200 |
| Pirate | 10 | 400 |

## Distribution proof

| Faction | Rule | Border/raid | Interior/garrison |
|---|---|---|---|
| Terran | 60-40 | 6 (`border`) | 4 (`interior`) |
| Pirate | 80-20 | 8 (`raid`) | 2 (`garrison`) |

## Tree parentage proof

- Fleets on mandated 1×1 surface gridcells.
- Ships under exactly one fleet.
- No ship direct child of star system, owner, or GameSession.

## Owner-reference proof

- Every fleet and ship carries `owner_flow_owner_ref`.
- Owner refs resolve to GameSession sibling owners.
- Owners remain non-spatial.

## HP/Damage/upkeep column proof

Ship nodes carry `tp::hull`, `tp::weapon_damage`, `tp::upkeep` scenario-envelope columns. Combat resolution deferred to Phase 4.

## RF upkeep proof

- Per-ship upkeep authored as ordinary `owner_flow_deficit` + `owner_flow_resource_key` metadata.
- **Not proven:** fleet-nested ship participation in `evaluate_planet_child_rf_reduce_up` at current generic admission depth.
- **§0.6 Deviation:** Fleet/ship RF metadata is authored and scenario-contained. Full fleet-nested RF reduce-up beyond current generic admission depth is parked. No Fleet/Cohort-specific spec traversal was added in 0R.

**Separate generic admission candidate (post-completion, not canonized):** generalize child-RF admission to arbitrary child depth over any SimThing, semantic-free, kind-agnostic. Terran/Pirate fleet→ship nesting is the triggering example only.

## Reparent/enrollment compatibility proof

- Fleet reparent between terran surfaces proven.
- Ship children remain under fleet; owner/deficit/resource metadata survives reparent.
- Full `FissionPolicy` + arena re-enrollment deferred to `TP-FLEET-MOVEMENT-0`.

## Unsupported-field hard-error proof

`unsupported_fleet_ship_payload_fields_hard_error_with_span` — span hard-error on unknown fields.

## Test lifecycle / inventory updates

| Test | birth_track | retention basis | downstream consumer note | dsu_survivals |
|---|---|---|---|---|
| `tp_fleets_ships_0_table` | `0.0.8.5-terran-pirate` | `permanent-residue:oracle-parity` | Phase 4 combat rung | 0 |
| `unsupported_fleet_ship_payload_fields_hard_error_with_span` | `0.0.8.5-terran-pirate` | `permanent-residue:oracle-parity` | Phase 4 combat rung | 0 |

## Proof commands

| Command | Result |
|---|---|
| `cargo check -p simthing-clausething` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-clausething --test tp_fleets_ships_0 -- --nocapture` | PASS (2 passed) |
| `cargo test -p simthing-clausething --test tp_shipsize_decoder_0 -- --nocapture` | PASS (1 passed, 1 ignored) |
| `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 -- --nocapture` | PASS (12 passed) |
| `bash scripts/ci/test_inventory_check.sh` | INSPECT (exit 0) |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_boundary_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --prove` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | PASS failures=0 inspect=0 |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `git diff --check origin/master...HEAD` | PASS |
| Fleet/Cohort kind-hit diff vs master | no diff (post-0R commit) |

## Scope ledger

| Item | Touched? |
|---|---|
| simthing-clausething | yes — retained `hydrate_scenario.rs`, tests (0R: test assertions only) |
| simthing-spec | yes — **0R reverted** `planet_child_rf.rs` to master (no net diff) |
| simthing-gpu/driver/sim/mapeditor/tools | no |
| workflows / scans / allowlists | no |
| `test_lifecycle_tracks.tsv` | no |
| Fleet/Cohort-specific spec traversal | no (reverted) |
| cargo workspace run | no |

## Graduation routing

- TP-FLEETS-SHIPS-0 / 0R complete
- PROBATION / DA-OWNER REVIEW
- DA/Owner clearance required
- not self-mergeable
- next rung after clearance: `TP-COMBAT-ARENA-0` only if DA clears and design row permits
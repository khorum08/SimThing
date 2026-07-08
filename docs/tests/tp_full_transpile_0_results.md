# TP-FULL-TRANSPILE-0 Results

## Status

**PROBATION / proof-present / clearance-pending.** Implementation complete; orchestrator/DA review required before merge.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-FULL-TRANSPILE-0` |
| Branch | `codex/tp-full-transpile-0` |
| Fixture | `crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause` |
| Test | `crates/simthing-clausething/tests/tp_full_transpile_0.rs` |
| birth_track | `0.0.8.5-terran-pirate` |

## Mechanism

Single native ClauseScript scenario file hydrates through existing `hydrate_scenario` into a `HydratedScenarioPack`, then projects to canonical `SimThingScenarioSpec` JSON via `save_scenario_spec_to_canonical_json` / `deserialize_scenario_authority`.

Composed clausething-native surfaces:

- embedded 1500-star disc (`static_galaxy_scenario`)
- Terran/Pirate owners as GameSession siblings
- ownership volumes 200 / 50 / 1250 neutrals
- planet/surface payloads (owned vs neutral)
- fleet/ship payloads (10×20 Terran, 10×40 Pirate)
- combat arena HP/Damage enrollments
- Movement-Front `field_operator` (SaturatingFlux, horizon-3, 16×16 theater feedstock)
- PALMA `palma_feedstock` W/D
- FIELD_POLICY `commitment` (ai_will_do urgency weights)

Workshop Mechanism-B live paths (diplomacy RF apply, dense MF execution, fleet D-gradient reparent) remain post-hydration consumers for `TP-LIVE-RUN-0`; authoring posture is declared in scenario metadata (`diplomacy_lane_profile`, `fleet_movement_profile`, `fronts_profile`).

## Scope Ledger

| In | Out |
|---|---|
| Fixture `.clause` + one integration test | `TP-LIVE-RUN-0` multi-tick / GPU |
| Inventory + boundary rows | DA closeout |
| Results doc | simthing-sim / kernel / WGSL / orientation |
| Canonical ScenarioSpec roundtrip of authority tree | Atlas scheduler |

## Full transpile proof

- `parse_raw_document` accepts `terran_pirate_galaxy.clause`
- `hydrate_scenario` succeeds with owners, volumes, payloads, combat, field_operator, palma, commitment
- `authority_root` present; GameSession → {Owner, Owner, GalaxyMap} validated

## Canonical roundtrip proof

- `save_scenario_spec_to_canonical_json` deterministic
- deserialize → re-save byte-identical JSON + stable authority digest
- roundtrip preserves scenario id, 1500 star systems, 2 owners, fleet tree

## Semantic-free boundary proof

- No edits to `simthing-sim`, `simthing-kernel`, WGSL, or GPU primitives in this PR
- ScenarioSpec may retain Terran/Pirate **authoring** ids/display names (legitimate)
- Root `SimThingKind` remains non-faction-named

## Scenario content proof

| Content | Evidence |
|---|---|
| Terran + Pirate owners | `pack.owners` + GameSession owner children |
| GalaxyMap / embedded base | 1500 systems; namespaced `tp_base::` targets; seed 770421 |
| Ownership 200/50 | volume assigned_systems lengths |
| Owned vs neutral planet payload | factory/cohort mins |
| Fleets/ships | 20 authored fleets + 2 combat fleets; ≥600 ships |
| Combat arena | 2 enrollments |
| MF / PALMA / commitment | region_fields + palma_feedstock + commitment Some |
| Diplomacy/movement posture | metadata profiles |

## Homing / substrate boundary

- All hydrate/transpile in `simthing-clausething` + existing `simthing-spec` canonical IO
- No engine crate edits
- STEAD lattice feedstock retained on embedded base / `grid_metadata`; authority-tree ScenarioSpec roundtrips the gameplay tree (placement re-bind onto authority nodes is install/live-run residue)

## Rustification / lifecycle

| Field | Value |
|---|---|
| birth_track | `0.0.8.5-terran-pirate` |
| class | `oracle-parity` |
| verdict | `KEEP` |
| dsu_survivals | `0` |
| per-rung lifecycle track | **not** created |

## Load-bearing proofs

```text
cargo test -p simthing-clausething --test tp_full_transpile_0 -- --nocapture
  → terran_pirate_galaxy_full_transpile_to_canonical_scenario_spec PASS
```

## Known gaps / next

TP-LIVE-RUN-0 remains next: non-vacuous multi-tick real-adapter or headless live run over the deterministically selected contested Terran/Pirate border theater.

TP-DA-CLOSEOUT-0 remains after live-run evidence.

## Graduation routing

| Field | Value |
|---|---|
| Risk class | data-deliverable / scenario-envelope integration |
| CI | Doctrine Scan required green |
| Recommended posture | PROBATION → orchestrator clearance or DA as router emits |
| Falsification | delete fixture or break hydrate → test FAIL |

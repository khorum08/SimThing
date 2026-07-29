# CANONICAL-ANCHOR-MATERIALIZATION-0 — candidate-evidence census (STOP)

- Track: 0.0.8.7 RF arena modernization (rung 5.3b)
- Status: **STOP before implementation** — census returned for DA ruling
- HD-RECEIPT: `c1599e8b6a89`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- Dispatch: orch `5123105021` · DA issuance `5122978247` · PR #1494 @ `7d3687cc`
- Census harness: `crates/simthing-driver/tests/canonical_anchor_materialization_0.rs`
- Method: hydrate unmodified `scenarios/terran_pirate_galaxy.clause`; collect lawful entity-key candidates per handoff vocabulary only (RF parent edges, owner policy-weight authority diagnostic, economy/threshold/need/overlay ScenarioListed, hosted-observation disruption locations). **No** kind / name / display / SessionRoot inference. **No** precedence invented among classes.

## Verdict

**STOP.** Exactly-one convergence fails for 9 of 25 Anchored properties (`exact=16 / zero=7 / conflict=2`). Implementation is not started.

## Provenance table

| # | canonical property | candidate host(s) | evidence class(es) | available span/provenance | convergence |
|---|---|---|---|---|---|
| 1 | `tp::hull` | — | — | — | **STOP(zero)** |
| 2 | `tp::weapon_damage` | — | — | — | **STOP(zero)** |
| 3 | `tp::upkeep` | — | — | — | **STOP(zero)** |
| 4 | `tp::combat_terran_ship_0_hull` | — | — | — | **STOP(zero)** |
| 5 | `tp::combat_terran_ship_0_weapon` | — | — | — | **STOP(zero)** |
| 6 | `tp::combat_pirate_ship_0_hull` | — | — | — | **STOP(zero)** |
| 7 | `tp::combat_pirate_ship_0_weapon` | — | — | — | **STOP(zero)** |
| 8 | `tp_economy::pirate_disruption_weight_current` | `pirate` | economy.emission.host_entity, economy.transfer.source_host_entity | emission.id=tp_economy_silo_current_pirate_disruption_weight; transfer.id=tp_economy_silo_transfer_pirate_disruption_weight | exactly-one |
| 9 | `tp_economy::pirate_disruption_weight_stockpile` | `pirate` | economy.recipe.input.host_entity, economy.transfer.target_host_entity | recipe.id=tp_economy_coupling_pirate_raid_suppresses_shipyard input[2]; transfer.id=tp_economy_silo_transfer_pirate_disruption_weight | exactly-one |
| 10 | `tp_economy::pirate_outpost_disruption_presence` | `pirate`, `pirate_outpost` | economy.emission / emit_on_threshold / recipe.input; hosted_observation.disruption_presence; overlay.ScenarioListed | economy hosts `pirate_outpost`; owner_policy overlay `tp_economy_owner_policy_pirate_disruption_policy` hosts `pirate`; presence overlay also `pirate_outpost` | **STOP(conflict)** |
| 11 | `tp_economy::terran_expansion_weight_current` | `terran` | economy.emission, economy.transfer.source | silo_current / silo_transfer terran_expansion_weight | exactly-one |
| 12 | `tp_economy::terran_expansion_weight_stockpile` | `terran` | economy.transfer.target, need_binding.locus | silo_transfer; need_binding.id=terran_manufacturing_need | exactly-one |
| 13 | `tp_economy::terran_minerals_current` | `terran` | economy.emission, economy.transfer.source | silo_current / silo_transfer terran_minerals | exactly-one |
| 14 | `tp_economy::terran_minerals_stockpile` | `terran` | economy.transfer.target | silo_transfer terran_minerals | exactly-one |
| 15 | `tp_economy::terran_shipyard_disrupted_hulls_quantity` | `terran_shipyard` | economy.recipe.target_host_entity | recipe.id=tp_economy_coupling_pirate_raid_suppresses_shipyard | exactly-one |
| 16 | `tp_economy::terran_shipyard_hulls_quantity` | `terran`, `terran_shipyard` | economy.recipe input/target; need_binding; overlay.ScenarioListed | economy/need → `terran_shipyard`; owner_policy overlays terran_expansion_policy + terran_manufacturing_policy → `terran` | **STOP(conflict)** |
| 17 | `tp_economy::terran_shipyard_minerals_quantity` | `terran_shipyard` | economy.emission, economy.recipe.input, overlay.ScenarioListed | quantity emission/recipe/overlay agree on `terran_shipyard` | exactly-one |
| 18–25 | `tp_economy::tp_base::studio_gridcell_system_{1055,1420,244,250,466,755,908,96}_disruption_presence` | matching `tp_base::studio_gridcell_system_*` | economy.emission + emit_on_threshold; hosted_observation; overlay.ScenarioListed | presence emission/threshold/overlay/observation agree per fleet-home entity | exactly-one (×8) |

### Diagnostic (not among the 25)

- Owner policy-weight authority hosts (`SimPropertyId(8_300_318)`): `[3003, 3004]` (Terran, Pirate).
- RF parent edges on authority/root: **0** on unmodified TP hydrate.

## STOP rows requiring DA ruling

1. **ZERO (7):** `tp::hull`, `tp::weapon_damage`, `tp::upkeep`, `tp::combat_terran_ship_0_hull`, `tp::combat_terran_ship_0_weapon`, `tp::combat_pirate_ship_0_hull`, `tp::combat_pirate_ship_0_weapon` — no lawful host_entity / ScenarioListed overlay / need / RF edge / hosted-observation registration names a host. Combat transfers currently carry `host_entity: None`.
2. **CONFLICT (2):**
   - `tp_economy::pirate_outpost_disruption_presence` → `{pirate, pirate_outpost}`
   - `tp_economy::terran_shipyard_hulls_quantity` → `{terran, terran_shipyard}`

No precedence was applied. Implementation remains blocked until DA rules ties and zero-candidate properties.

## Reproduce

```bash
cargo test -p simthing-driver --test canonical_anchor_materialization_0 -- --nocapture
```

Expected lock: `exact=16 / zero=7 / conflict=2`.

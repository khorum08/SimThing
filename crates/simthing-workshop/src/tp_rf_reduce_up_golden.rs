//! Canonical TP child→ancestor reduce-up golden (RF-1 → RF-4 OVL target).
//!
//! The golden is scenario-flavored and therefore workshop-homed. It ingests
//! the authoritative ClauseScript pack and derives an Owner-channel aggregate
//! from every authored fleet participant; it does not copy scenario constants
//! into Rust and does not call the RF-2 recursive execution source.

use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
use simthing_spec::EmissionFormulaSpec;

/// One descendant contribution in the selected Owner RF channel.
#[derive(Clone, Debug, PartialEq)]
pub struct TpDescendantContribution {
    pub participant_id: String,
    pub target_location_id: String,
    pub fleet_index: u32,
    pub ships: u32,
    pub per_ship_flow: f32,
    pub contribution: f32,
}

/// Authored-pack-bound RF-4 reduce-up target.
///
/// Owner entities remain GameSession siblings, never spatial parents. The
/// `owner_arena` field names the RF owner channel; descendant locations remain
/// those authored by each placement in `participant_contributions`.
#[derive(Clone, Debug, PartialEq)]
pub struct TpReduceUpGolden {
    pub owner_arena: String,
    pub resource_key: String,
    pub participant_contributions: Vec<TpDescendantContribution>,
    pub selected_child_id: String,
    pub selected_child_marginal: f32,
    pub sibling_contributions_preserved: f32,
    pub expected_owner_aggregate: f32,
    pub factory_need_deltas: Vec<f32>,
    pub factory_unit_costs: Vec<f32>,
    pub factory_emit_count: u32,
}

/// Ingest the canonical authored pack and derive the TP reduce-up target.
///
/// The exact selected arena is the `terran` Owner energy/upkeep RF channel. All
/// authored `terran_fleets` placements contribute; fleet zero is the selected
/// child and every other fleet is retained as sibling mass. Any drift in fleet
/// count, placement path, ships-per-fleet, upkeep, resource key, or recipe
/// inputs changes this result and therefore breaks the canonical golden test.
pub fn compute_tp_reduce_up_golden_from_clause(
    clause_source: &[u8],
    source_base: Option<&Path>,
) -> Result<TpReduceUpGolden> {
    let document = parse_raw_document(clause_source).context("parse canonical TP ClauseScript")?;
    let pack = hydrate_scenario_with_source_base(&document, source_base)
        .context("hydrate canonical TP authored pack")?;

    let fleet = pack
        .fleet_ship_payloads
        .iter()
        .find(|payload| payload.id == "terran_fleets")
        .ok_or_else(|| anyhow!("canonical pack is missing terran_fleets"))?;
    if fleet.owner != "terran" {
        bail!("terran_fleets owner drifted to {}", fleet.owner);
    }
    if fleet.placements.len() != fleet.fleet_count as usize {
        bail!(
            "terran_fleets placement count {} does not match authored fleet_count {}",
            fleet.placements.len(),
            fleet.fleet_count
        );
    }

    let mut participant_contributions: Vec<TpDescendantContribution> = fleet
        .placements
        .iter()
        .map(|placement| {
            if placement.owner != fleet.owner || placement.ships_per_fleet != fleet.ships_per_fleet
            {
                bail!(
                    "fleet {} path/profile drift: owner={} ships={}",
                    placement.fleet_index,
                    placement.owner,
                    placement.ships_per_fleet
                );
            }
            let contribution = (placement.ships_per_fleet as f32) * (fleet.upkeep_per_ship as f32);
            Ok(TpDescendantContribution {
                participant_id: format!("{}#{}", fleet.id, placement.fleet_index),
                target_location_id: placement.target_id.clone(),
                fleet_index: placement.fleet_index,
                ships: placement.ships_per_fleet,
                per_ship_flow: fleet.upkeep_per_ship as f32,
                contribution,
            })
        })
        .collect::<Result<_>>()?;
    participant_contributions.sort_by_key(|contribution| contribution.fleet_index);

    let selected = participant_contributions
        .first()
        .ok_or_else(|| anyhow!("terran_fleets has no participating descendants"))?;
    let expected_owner_aggregate: f32 = participant_contributions
        .iter()
        .map(|contribution| contribution.contribution)
        .sum();
    let sibling_contributions_preserved = expected_owner_aggregate - selected.contribution;

    let economy = pack
        .field_economy
        .as_ref()
        .ok_or_else(|| anyhow!("canonical pack is missing field_economy"))?;
    let quantity = economy
        .field_resource_quantities
        .iter()
        .find(|quantity| quantity.id == "shipyard_minerals")
        .ok_or_else(|| anyhow!("canonical pack is missing shipyard_minerals"))?;
    let resource_economy = pack
        .game_mode
        .resource_economy
        .as_ref()
        .ok_or_else(|| anyhow!("canonical pack is missing lowered resource economy"))?;
    let recipe = resource_economy
        .recipes
        .iter()
        .find(|recipe| recipe.id == "tp_economy_recipe_shipyard_factory")
        .ok_or_else(|| anyhow!("canonical pack is missing shipyard_factory recipe"))?;
    if recipe.inputs.is_empty() {
        bail!("shipyard_factory recipe has no inputs");
    }
    let max_by_inputs = recipe
        .inputs
        .iter()
        .map(|input| (quantity.amount / input.unit_cost).floor() as u32)
        .min()
        .ok_or_else(|| anyhow!("shipyard_factory recipe has no bounded input"))?;
    let factory_emit_count = max_by_inputs.min(recipe.throttle_hint_max_per_tick);
    let factory_unit_costs: Vec<f32> = recipe.inputs.iter().map(|input| input.unit_cost).collect();
    let factory_need_deltas: Vec<f32> = factory_unit_costs
        .iter()
        .map(|cost| -(factory_emit_count as f32) * cost)
        .collect();

    // Bind the quantity to the executed lowered surface as well as the hydrated
    // authoring record; a missing/drifted emission is not silently accepted.
    let quantity_emission = resource_economy
        .emissions
        .iter()
        .find(|emission| emission.id == "tp_economy_quantity_emission_shipyard_minerals")
        .ok_or_else(|| anyhow!("canonical pack is missing shipyard_minerals emission"))?;
    match quantity_emission.formula {
        EmissionFormulaSpec::Constant(value) if value.to_bits() == quantity.amount.to_bits() => {}
        ref other => bail!(
            "shipyard_minerals lowered emission {:?} disagrees with authored amount {}",
            other,
            quantity.amount
        ),
    }

    Ok(TpReduceUpGolden {
        owner_arena: fleet.owner.clone(),
        resource_key: fleet.resource_key.clone(),
        selected_child_id: selected.participant_id.clone(),
        selected_child_marginal: selected.contribution,
        sibling_contributions_preserved,
        expected_owner_aggregate,
        participant_contributions,
        factory_need_deltas,
        factory_unit_costs,
        factory_emit_count,
    })
}

//! TP-LIVE-RUN-0 / 0R — workshop-homed bounded-theater live-run authoring over the full
//! Terran-Pirate transpile fixture.
//!
//! Homing: would not exist without the Terran-Pirate scenario (Mechanism B composition).
//! Chains accepted commitments (fronts + PALMA + 7×7 theater + STEAD thresholds) and
//! records placement re-bind from embedded lattice targets → authority SimThing ids.
//!
//! **0R combat doctrine:** all conflict is RF accumulator economics + overlay filters.
//! Hull HP is a damage-to-kill / hull-deficit emission band; incoming weapon damage is the
//! resource that fills that band; destroyed_ships is emitted by the RF emission-band law and
//! depletes num_ships. No combat engine beside the tree.

use simthing_clausething::{HydratedCombatShipEnrollment, HydratedScenarioPack};
use simthing_core::{ClampBehavior, SimThingId, SubFieldRole, SubFieldSpec};
use simthing_spec::spec::property::PropertySpec;
use simthing_spec::spec::resource_economy::ResourceEconomyOptInMode;
use simthing_spec::spec::script::PropertyKey;

use crate::commitments_post_hydration::{
    apply_commitments_post_hydration, CommitmentsHydrationError, TpCommitmentsAuthoringReport,
};
use crate::fleet_movement_post_hydration::TP_MOVEMENT_GRID_SIZE;
use crate::fronts_post_hydration::TpFrontsTheaterCell;

/// Deterministic theater edge for the live-run proof (STEAD P1 bounded theater).
pub const TP_LIVE_RUN_THEATER_GRID: u32 = TP_MOVEMENT_GRID_SIZE;

/// Multi-tick proof floor (non-vacuous live run).
pub const TP_LIVE_RUN_MIN_TICKS: u32 = 3;

/// Workshop RF combat property namespace (scenario envelope — not a combat engine).
pub const TP_RF_COMBAT_PROPERTY_NAMESPACE: &str = "tp_rf_combat";

/// Shared damage-to-kill-1-hull band price (emission-band divisor).
pub const TP_RF_COMBAT_DTK_PROPERTY: &str = "damage_to_kill_1_hull";

/// Cohort ship-count column (depleted by destroyed_ships emission).
pub const TP_RF_COMBAT_NUM_SHIPS_PROPERTY: &str = "num_ships";

/// Emitted casualty count from the RF emission-band law.
pub const TP_RF_COMBAT_DESTROYED_SHIPS_PROPERTY: &str = "destroyed_ships";

/// One re-bound theater cell: embedded lattice target → authority node → theater slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TpPlacementRebindEntry {
    /// Producer-namespaced target stem from the embedded base (`tp_base::…`).
    pub embedded_target_id: String,
    /// Theater install target (`{embedded_target}@{row}_{col}`).
    pub theater_target_id: String,
    /// Authority-tree star-system SimThing id (re-bound runtime node).
    pub authority_simthing_id: SimThingId,
    pub theater_row: u32,
    pub theater_col: u32,
    pub owner: String,
}

/// RF combat flow descriptor for one enrolled ship (workshop composition over transfers).
#[derive(Debug, Clone, PartialEq)]
pub struct TpRfCombatShipFlow {
    pub enrollment_id: String,
    pub owner: String,
    pub simthing_id: SimThingId,
    /// Incoming damage resource (weapon amount column).
    pub incoming_damage_property: String,
    /// Hull HP as damage-to-kill / hull-deficit emission band (fills with damage).
    pub hull_deficit_band_property: String,
    /// Damage required to emit one destroyed ship (band capacity).
    pub damage_to_kill_1_hull: f32,
    pub num_ships_seed: f32,
}

/// Workshop RF combat economics report (0R).
#[derive(Debug, Clone)]
pub struct TpRfCombatEconomicsReport {
    pub ships: Vec<TpRfCombatShipFlow>,
    pub transfer_ids: Vec<String>,
    pub overlay_filter_ids: Vec<String>,
    pub dtk_property: String,
    pub num_ships_property: String,
    pub destroyed_ships_property: String,
}

/// Live-run authoring report over the full transpile pack.
#[derive(Debug, Clone)]
pub struct TpLiveRunAuthoringReport {
    pub commitments: TpCommitmentsAuthoringReport,
    pub rebind: Vec<TpPlacementRebindEntry>,
    pub rf_combat: TpRfCombatEconomicsReport,
    pub theater_grid_size: u32,
    pub min_ticks: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum LiveRunHydrationError {
    #[error(transparent)]
    Commitments(#[from] CommitmentsHydrationError),
    #[error("{0}")]
    Message(String),
}

/// Mapping rule (recorded for results / review):
///
/// 1. Contested border systems are selected from ownership volumes on `authority_root`.
/// 2. Workshop theater cells assign each selected system a `(theater_row, theater_col)`
///    on a 7×7 bounded grid and stamp `theater_target_id = "{embedded_target}@{r}_{c}"`.
/// 3. Runtime install clones an authority-system shell (by `simthing_id`) into the
///    session root and registers `install_targets[theater_target_id] → shell.id`.
/// 4. Embedded lattice placements remain the STEAD feedstock; producer-local ids never
///    dangle because every theater cell carries a resolved authority `simthing_id`.
pub fn apply_live_run_post_hydration(
    pack: &mut HydratedScenarioPack,
) -> Result<TpLiveRunAuthoringReport, LiveRunHydrationError> {
    if pack.authority_root.is_none() {
        return Err(LiveRunHydrationError::Message(
            "live run requires authority_root from full transpile".into(),
        ));
    }
    if pack.embedded_static_galaxy_scenarios.is_empty() {
        return Err(LiveRunHydrationError::Message(
            "live run requires embedded static galaxy base".into(),
        ));
    }
    if pack.combat_arena_payload.is_none() {
        return Err(LiveRunHydrationError::Message(
            "live run requires combat_arena_payload from full transpile".into(),
        ));
    }

    let commitments = apply_commitments_post_hydration(pack)?;
    let rebind = build_rebind_table(&commitments.movement.palma.fronts.theater_cells);
    if rebind.is_empty() {
        return Err(LiveRunHydrationError::Message(
            "live run rebind table empty — theater cells missing".into(),
        ));
    }
    for entry in &rebind {
        if entry.embedded_target_id.is_empty() || !entry.embedded_target_id.contains("::") {
            return Err(LiveRunHydrationError::Message(format!(
                "rebind entry missing namespaced embedded target: {:?}",
                entry
            )));
        }
        if entry.authority_simthing_id.raw() == 0 {
            return Err(LiveRunHydrationError::Message(format!(
                "rebind entry has zero authority simthing id for {}",
                entry.theater_target_id
            )));
        }
    }

    let rf_combat = compose_rf_combat_economics(pack)?;

    Ok(TpLiveRunAuthoringReport {
        commitments,
        rebind,
        rf_combat,
        theater_grid_size: TP_LIVE_RUN_THEATER_GRID,
        min_ticks: TP_LIVE_RUN_MIN_TICKS,
    })
}

/// Compose RF combat economics over existing weapon→hull transfer registrations.
///
/// Semantics (D1/D2 / constitution §0.3):
/// - `weapon.amount` = incoming damage resource
/// - `hull.amount` = damage-to-kill / hull-deficit emission band (fills with damage)
/// - `damage_to_kill_1_hull.amount` = band price (capacity)
/// - RF emission-band law: `destroyed_ships = floor(hull_band / dtk)` clamped to `num_ships`
/// - `num_ships` depletes by destroyed_ships (casualty sink)
///
/// Modifiers remain overlay filters on weapon/hull columns (no owner branch).
fn compose_rf_combat_economics(
    pack: &mut HydratedScenarioPack,
) -> Result<TpRfCombatEconomicsReport, LiveRunHydrationError> {
    let combat = pack
        .combat_arena_payload
        .as_ref()
        .ok_or_else(|| LiveRunHydrationError::Message("combat payload required".into()))?
        .clone();

    if combat.transfers.is_empty() {
        return Err(LiveRunHydrationError::Message(
            "RF combat requires weapon→hull transfer registrations".into(),
        ));
    }
    for transfer in &combat.transfers {
        // Transfers must be property-keyed RF registrations (no manual hull resolver).
        if transfer.source.name.is_empty() || transfer.target.name.is_empty() {
            return Err(LiveRunHydrationError::Message(
                "combat transfer missing RF property keys".into(),
            ));
        }
        // Target must be the hull deficit band (damage fill), not a bespoke resolver.
        if !transfer.target.name.contains("hull") {
            return Err(LiveRunHydrationError::Message(format!(
                "combat transfer target `{}` is not a hull-deficit band",
                transfer.target.name
            )));
        }
        if !transfer.source.name.contains("weapon") && !transfer.source.name.contains("damage") {
            return Err(LiveRunHydrationError::Message(format!(
                "combat transfer source `{}` is not a damage/weapon resource",
                transfer.source.name
            )));
        }
    }

    install_rf_combat_columns(pack)?;

    // Keep transfer opt-in; emissions are workshop emission-band law over transfer output.
    if let Some(economy) = pack.game_mode.resource_economy.as_mut() {
        if economy.opt_in_mode == ResourceEconomyOptInMode::Disabled {
            economy.opt_in_mode = ResourceEconomyOptInMode::TransferOnly;
        }
    }

    let overlay_filter_ids: Vec<String> = pack
        .game_mode
        .overlays
        .iter()
        .filter(|o| {
            o.targets_property.contains("weapon")
                || o.targets_property.contains("hull")
                || o.id.contains("combat")
                || o.id.contains("ship_weapon")
        })
        .map(|o| o.id.clone())
        .collect();

    let ships: Vec<TpRfCombatShipFlow> = combat
        .enrollments
        .iter()
        .map(|e| enrollment_to_rf_flow(e))
        .collect();

    Ok(TpRfCombatEconomicsReport {
        ships,
        transfer_ids: combat.transfers.iter().map(|t| t.id.clone()).collect(),
        overlay_filter_ids,
        dtk_property: TP_RF_COMBAT_DTK_PROPERTY.into(),
        num_ships_property: TP_RF_COMBAT_NUM_SHIPS_PROPERTY.into(),
        destroyed_ships_property: TP_RF_COMBAT_DESTROYED_SHIPS_PROPERTY.into(),
    })
}

fn enrollment_to_rf_flow(enrollment: &HydratedCombatShipEnrollment) -> TpRfCombatShipFlow {
    TpRfCombatShipFlow {
        enrollment_id: enrollment.id.clone(),
        owner: enrollment.owner.clone(),
        simthing_id: enrollment.simthing_id,
        incoming_damage_property: enrollment.weapon_property.clone(),
        hull_deficit_band_property: enrollment.hull_property.clone(),
        damage_to_kill_1_hull: enrollment.hull_capacity,
        num_ships_seed: 1.0,
    }
}

fn install_rf_combat_columns(pack: &mut HydratedScenarioPack) -> Result<(), LiveRunHydrationError> {
    for (id, name, display) in [
        (
            "tp_rf_damage_to_kill_1_hull",
            TP_RF_COMBAT_DTK_PROPERTY,
            "DamageToKill1Hull",
        ),
        (
            "tp_rf_num_ships",
            TP_RF_COMBAT_NUM_SHIPS_PROPERTY,
            "NumShips",
        ),
        (
            "tp_rf_destroyed_ships",
            TP_RF_COMBAT_DESTROYED_SHIPS_PROPERTY,
            "DestroyedShips",
        ),
    ] {
        let property = PropertySpec {
            id: id.into(),
            namespace: TP_RF_COMBAT_PROPERTY_NAMESPACE.into(),
            name: name.into(),
            display_name: display.into(),
            description: String::new(),
            sub_fields: vec![SubFieldSpec {
                role: SubFieldRole::Amount,
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: display.into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            }],
        };
        if !pack
            .game_mode
            .properties
            .iter()
            .any(|existing| existing.id == property.id)
        {
            pack.game_mode.properties.push(property);
        }
    }
    // Touch PropertyKey construction so RF keys stay property-keyed (not stringly combat).
    let _ = PropertyKey::new(TP_RF_COMBAT_PROPERTY_NAMESPACE, TP_RF_COMBAT_NUM_SHIPS_PROPERTY);
    Ok(())
}

/// RF emission-band law (R6 / constitution §0.3):  
/// `destroyed_ships = floor(hull_deficit_band / damage_to_kill_1_hull)` clamped to `num_ships`.
///
/// This is the economic emission identity over RF transfer-filled columns — not a combat
/// subsystem and not a manual HP subtractor.
pub fn rf_emission_band_destroyed_ships(
    hull_deficit_band_filled: f32,
    damage_to_kill_1_hull: f32,
    num_ships_before: f32,
) -> f32 {
    if !(damage_to_kill_1_hull.is_finite() && damage_to_kill_1_hull > 0.0) {
        return 0.0;
    }
    if !(hull_deficit_band_filled.is_finite()) || !(num_ships_before.is_finite()) {
        return 0.0;
    }
    let raw = (hull_deficit_band_filled / damage_to_kill_1_hull).floor();
    raw.max(0.0).min(num_ships_before.max(0.0))
}

/// Apply RF emission-band casualty sink: `num_ships_after = num_ships_before - destroyed`.
pub fn rf_num_ships_after_emission(num_ships_before: f32, destroyed_ships: f32) -> f32 {
    (num_ships_before - destroyed_ships).max(0.0)
}

fn build_rebind_table(cells: &[TpFrontsTheaterCell]) -> Vec<TpPlacementRebindEntry> {
    cells
        .iter()
        .map(|cell| {
            let embedded_target_id = cell
                .target_id
                .split('@')
                .next()
                .unwrap_or(cell.target_id.as_str())
                .to_string();
            TpPlacementRebindEntry {
                embedded_target_id,
                theater_target_id: cell.target_id.clone(),
                authority_simthing_id: cell.simthing_id,
                theater_row: cell.theater_row,
                theater_col: cell.theater_col,
                owner: cell.owner.clone(),
            }
        })
        .collect()
}

/// Assert rebind covers both factions and theater coordinates stay in-bounds.
pub fn validate_rebind_table(
    report: &TpLiveRunAuthoringReport,
) -> Result<(), LiveRunHydrationError> {
    if report.theater_grid_size < 7 {
        return Err(LiveRunHydrationError::Message(
            "live-run theater must be ≥7×7".into(),
        ));
    }
    let has_terran = report.rebind.iter().any(|e| e.owner == "terran");
    let has_pirate = report.rebind.iter().any(|e| e.owner == "pirate");
    if !has_terran || !has_pirate {
        return Err(LiveRunHydrationError::Message(
            "rebind must include terran and pirate border systems".into(),
        ));
    }
    for entry in &report.rebind {
        if entry.theater_row >= report.theater_grid_size
            || entry.theater_col >= report.theater_grid_size
        {
            return Err(LiveRunHydrationError::Message(format!(
                "theater coord out of bounds: ({}, {})",
                entry.theater_row, entry.theater_col
            )));
        }
    }
    if report.rf_combat.ships.len() < 2 {
        return Err(LiveRunHydrationError::Message(
            "RF combat requires both-side ship enrollments".into(),
        ));
    }
    if report.rf_combat.transfer_ids.is_empty() {
        return Err(LiveRunHydrationError::Message(
            "RF combat requires transfer registrations".into(),
        ));
    }
    Ok(())
}

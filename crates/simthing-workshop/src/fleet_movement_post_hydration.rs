//! TP-FLEET-MOVEMENT-0 — workshop-homed multi-tick D-gradient fleet reparenting.
//!
//! Chains accepted fronts + PALMA reach over a ≥7×7 / horizon-3 theater. Movement is
//! adjacent-cell gradient-following reparenting only — no route/path/predecessor objects.

use std::collections::BTreeMap;

use simthing_clausething::{
    build_palma_feedstock_from_region_field, HydratedOwnedSystem, HydratedScenarioPack,
};
use simthing_core::{SimThing, SimThingId, SimThingKind};
use simthing_spec::compile_w_impedance_compose_preview;

use crate::fronts_post_hydration::{
    apply_fronts_post_hydration_with_theater, collect_contested_border_systems,
    FrontsHydrationError, TpFrontsTheaterCell, TP_FRONTS_DEFAULT_HORIZON,
    TP_FRONTS_FIELD_OPERATOR_ID, TP_FRONTS_SOURCE_COL,
};
use crate::palma_reach_post_hydration::{
    build_tp_palma_w_compose, palma_reach_gradient_probe,
    PalmaReachGradientStep, PalmaReachHydrationError, TpPalmaReachAuthoringReport,
    TP_PALMA_D_OUTPUT_COL, TP_PALMA_W_OUTPUT_COL,
};

/// Minimum bounded theater edge for Phase 6.2 (STEAD P1 engagement).
pub const TP_MOVEMENT_GRID_SIZE: u32 = 7;

/// Movement theater uses the same L1 horizon as Phase 6.0 fronts.
pub const TP_MOVEMENT_HORIZON: u32 = TP_FRONTS_DEFAULT_HORIZON;

/// Minimum ticks and cells for multi-step movement proof (DA binding B2).
pub const TP_MOVEMENT_MIN_TICKS: u32 = 3;
pub const TP_MOVEMENT_MIN_CELLS: u32 = 3;

/// Workshop fleet movable SimThing session id (scenario-local proof only).
pub const TP_MOVEMENT_FLEET_SESSION_ID: u32 = 0x7F1E_0001;

/// Central horizon-truncation seed coordinate (theater row, col).
pub const TP_MOVEMENT_TRUNCATION_SEED: (u32, u32) = (3, 3);

/// Terran patrol / PALMA reach destination.
pub const TP_MOVEMENT_REACH_DEST: (u32, u32) = (0, 0);

/// Pirate fleet movement start (far from dest — follows D gradient inward).
pub const TP_MOVEMENT_FLEET_START: (u32, u32) = (6, 3);

#[derive(Debug, Clone)]
pub struct TpFleetMovementAuthoringReport {
    pub palma: TpPalmaReachAuthoringReport,
    pub fleet_start: (u32, u32),
    pub reach_dest: (u32, u32),
    pub truncation_seed: (u32, u32),
    pub horizon: u32,
    pub fleet_id: SimThingId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TpFleetTheaterCoord {
    pub row: u32,
    pub col: u32,
}

impl TpFleetTheaterCoord {
    pub fn slot(self, grid_size: u32) -> u32 {
        self.row * grid_size + self.col
    }
}

/// Observed movement coordinates per tick — path-as-observation, not a route object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TpMovementObservation {
    pub ticks: Vec<TpFleetTheaterCoord>,
}

#[derive(Debug, Clone)]
pub struct TpFleetMovementState {
    pub fleet_id: SimThingId,
    pub coord: TpFleetTheaterCoord,
    pub prev_coord: TpFleetTheaterCoord,
    pub enrolled_system_id: SimThingId,
}

#[derive(Debug, Clone)]
pub struct TpFleetArenaEnrollment {
    pub fleet_id: SimThingId,
    pub arena_name: String,
    pub authoritative_system_id: SimThingId,
    pub authoritative_coord: TpFleetTheaterCoord,
}

#[derive(Debug, thiserror::Error)]
pub enum FleetMovementHydrationError {
    #[error(transparent)]
    Fronts(#[from] FrontsHydrationError),
    #[error(transparent)]
    Palma(#[from] PalmaReachHydrationError),
    #[error("{0}")]
    Message(String),
}

/// Workshop-side fleet movement authoring over accepted fronts + PALMA reach.
pub fn apply_fleet_movement_post_hydration(
    pack: &mut HydratedScenarioPack,
) -> Result<TpFleetMovementAuthoringReport, FleetMovementHydrationError> {
    if pack.authority_root.is_none() {
        return Err(FleetMovementHydrationError::Message(
            "fleet movement requires authority_root".into(),
        ));
    }
    let terran = pack
        .ownership_volumes
        .iter()
        .find(|volume| volume.id == "terran_core")
        .ok_or(FrontsHydrationError::MissingOwnershipVolumes)?;
    let pirate = pack
        .ownership_volumes
        .iter()
        .find(|volume| volume.id == "pirate_border")
        .ok_or(FrontsHydrationError::MissingOwnershipVolumes)?;
    let root = pack
        .authority_root
        .as_ref()
        .expect("authority root checked above");

    let (terran_border, pirate_border) =
        collect_contested_border_systems(root, terran, pirate)?;
    let theater_cells =
        build_movement_theater_cells(root, &terran_border, &pirate_border, pack)?;
    let fronts =
        apply_fronts_post_hydration_with_theater(pack, theater_cells, TP_MOVEMENT_GRID_SIZE)?;

    let mut region_field = fronts.region_field.clone();
    region_field.horizon = TP_MOVEMENT_HORIZON;

    let palma = build_palma_feedstock_from_region_field(
        crate::palma_reach_post_hydration::TP_PALMA_FEEDSTOCK_ID,
        TP_FRONTS_FIELD_OPERATOR_ID,
        TP_PALMA_W_OUTPUT_COL,
        TP_PALMA_D_OUTPUT_COL,
        &region_field,
    )
    .map_err(|err| FleetMovementHydrationError::Message(err.to_string()))?;
    let w_compose = build_tp_palma_w_compose(&palma);
    compile_w_impedance_compose_preview(&w_compose).map_err(|err| {
        FleetMovementHydrationError::Message(format!("W impedance compose admission rejected: {err}"))
    })?;

    pack.palma_feedstock = Some(palma.clone());
    pack.w_impedance_compose = Some(w_compose);
    pack.game_mode.region_fields = vec![region_field];

    let reach_dest = TP_MOVEMENT_REACH_DEST;
    let palma_report = TpPalmaReachAuthoringReport {
        fronts,
        palma_feedstock: palma,
        w_compose: pack.w_impedance_compose.clone().expect("w compose installed"),
        reach_dest,
    };

    Ok(TpFleetMovementAuthoringReport {
        palma: palma_report,
        fleet_start: TP_MOVEMENT_FLEET_START,
        reach_dest,
        truncation_seed: TP_MOVEMENT_TRUNCATION_SEED,
        horizon: TP_MOVEMENT_HORIZON,
        fleet_id: SimThingId::from_session_raw(TP_MOVEMENT_FLEET_SESSION_ID),
    })
}

fn build_movement_theater_cells(
    root: &SimThing,
    terran_border: &[HydratedOwnedSystem],
    pirate_border: &[HydratedOwnedSystem],
    pack: &HydratedScenarioPack,
) -> Result<Vec<TpFrontsTheaterCell>, FleetMovementHydrationError> {
    let threat_fallback = fleet_weapon_rate(pack, "pirate").unwrap_or(30.0);
    let suppression_fallback = fleet_weapon_rate(pack, "terran").unwrap_or(40.0);
    let disruption_fallback = fleet_weapon_rate(pack, "pirate").unwrap_or(28.0);

    let mut cells = Vec::new();
    let mut used_targets = BTreeMap::new();

    let mut place = |row: u32,
                     col: u32,
                     owner: &str,
                     threat: f32,
                     suppression: f32,
                     disruption: f32,
                     pool: &[HydratedOwnedSystem],
                     pool_index: &mut usize| {
        if pool.is_empty() {
            return Err(FleetMovementHydrationError::Message(format!(
                "movement theater requires {owner} border systems"
            )));
        }
        let system = &pool[*pool_index % pool.len()];
        *pool_index += 1;
        let simthing_id = resolve_system_id(root, system.row, system.col).ok_or_else(|| {
            FleetMovementHydrationError::Message(format!(
                "movement theater system `{}` missing from authority tree",
                system.target_id
            ))
        })?;
        let theater_key = (row, col);
        if used_targets.values().any(|&(r, c)| (r, c) == theater_key) {
            return Ok(());
        }
        used_targets.insert(system.target_id.clone(), theater_key);
        cells.push(TpFrontsTheaterCell {
            target_id: format!("{}@{}_{}", system.target_id, row, col),
            theater_row: row,
            theater_col: col,
            owner: owner.into(),
            simthing_id,
            threat_rate: threat,
            suppression_rate: suppression,
            disruption_rate: disruption,
        });
        Ok::<(), FleetMovementHydrationError>(())
    };

    let mut terran_idx = 0usize;
    let mut pirate_idx = 0usize;

    place(
        TP_MOVEMENT_REACH_DEST.0,
        TP_MOVEMENT_REACH_DEST.1,
        "terran",
        0.0,
        suppression_fallback,
        0.0,
        terran_border,
        &mut terran_idx,
    )?;
    place(
        TP_MOVEMENT_TRUNCATION_SEED.0,
        TP_MOVEMENT_TRUNCATION_SEED.1,
        "terran",
        0.0,
        suppression_fallback * 0.75,
        0.0,
        terran_border,
        &mut terran_idx,
    )?;

    for step in 0..=TP_MOVEMENT_MIN_CELLS {
        let row = TP_MOVEMENT_FLEET_START.0.saturating_sub(step);
        let col = TP_MOVEMENT_FLEET_START.1;
        place(
            row,
            col,
            "pirate",
            threat_fallback,
            0.0,
            disruption_fallback,
            pirate_border,
            &mut pirate_idx,
        )?;
    }

    place(
        TP_MOVEMENT_GRID_SIZE - 1,
        TP_MOVEMENT_GRID_SIZE - 1,
        "pirate",
        threat_fallback * 0.5,
        0.0,
        disruption_fallback * 0.5,
        pirate_border,
        &mut pirate_idx,
    )?;

    for col in 1..3 {
        place(
            0,
            col,
            "terran",
            0.0,
            suppression_fallback * 0.35,
            0.0,
            terran_border,
            &mut terran_idx,
        )?;
    }

    if cells.is_empty() {
        return Err(FleetMovementHydrationError::Message(
            "movement theater requires at least one placed cell".into(),
        ));
    }
    Ok(cells)
}

fn fleet_weapon_rate(pack: &HydratedScenarioPack, owner: &str) -> Option<f32> {
    pack.fleet_ship_payloads
        .iter()
        .find(|payload| payload.owner == owner)
        .map(|payload| payload.weapon_damage_seed * payload.ships_per_fleet as f32)
}

fn resolve_system_id(root: &SimThing, row: u32, col: u32) -> Option<SimThingId> {
    let session = root
        .children
        .iter()
        .find(|child| child.kind == SimThingKind::GameSession)?;
    let galaxy_map = session
        .children
        .iter()
        .find(|child| simthing_spec::is_galaxy_map_entity(child))?;
    for star_system in &galaxy_map.children {
        let system_row = star_system
            .properties
            .get(&simthing_spec::SCENARIO_STRUCTURAL_ROW_PROPERTY_ID)
            .and_then(|value| value.raw_lanes().first().copied())
            .unwrap_or(0.0) as u32;
        let system_col = star_system
            .properties
            .get(&simthing_spec::SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
            .and_then(|value| value.raw_lanes().first().copied())
            .unwrap_or(0.0) as u32;
        if (system_row, system_col) == (row, col) {
            return Some(star_system.id);
        }
    }
    None
}

/// Manhattan distance on the theater grid.
pub fn theater_manhattan(a: TpFleetTheaterCoord, b: TpFleetTheaterCoord) -> u32 {
    a.row.abs_diff(b.row) + a.col.abs_diff(b.col)
}

/// Oracle: after horizon diffusion, source-col mass is zero beyond horizon hops from every seed.
pub fn horizon_truncation_engages_oracle(
    field_values: &[f32],
    seeds: &[(u32, u32, f32)],
    grid_size: u32,
    n_dims: u32,
    source_col: u32,
    horizon: u32,
) -> Result<(), String> {
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    let cells = grid_size * grid_size;
    for slot in 0..cells {
        let row = slot / grid_size;
        let col = slot % grid_size;
        let coord = TpFleetTheaterCoord { row, col };
        let min_seed_dist = seeds
            .iter()
            .map(|(seed_row, seed_col, _)| {
                theater_manhattan(
                    coord,
                    TpFleetTheaterCoord {
                        row: *seed_row,
                        col: *seed_col,
                    },
                )
            })
            .min()
            .unwrap_or(u32::MAX);
        let value = field_values
            .get(idx(slot, source_col))
            .copied()
            .unwrap_or(0.0);
        if min_seed_dist > horizon {
            if value.to_bits() != 0.0f32.to_bits() {
                return Err(format!(
                    "horizon truncation violated at ({row},{col}): mass={value} min_seed_dist={min_seed_dist} horizon={horizon}"
                ));
            }
        } else if seeds.iter().any(|(r, c, _)| *r == row && *c == col) {
            if value <= 0.0 {
                return Err(format!("seed cell ({row},{col}) must carry non-zero mass"));
            }
        }
    }
    let beyond = seeds
        .iter()
        .any(|(seed_row, seed_col, _)| {
            theater_manhattan(
                TpFleetTheaterCoord {
                    row: *seed_row,
                    col: *seed_col,
                },
                TpFleetTheaterCoord { row: 0, col: 0 },
            ) > horizon
                || theater_manhattan(
                    TpFleetTheaterCoord {
                        row: *seed_row,
                        col: *seed_col,
                    },
                    TpFleetTheaterCoord {
                        row: grid_size - 1,
                        col: grid_size - 1,
                    },
                ) > horizon
        });
    if !beyond {
        return Err(
            "movement theater must place at least one seed beyond horizon from a corner".into(),
        );
    }
    Ok(())
}

/// Initialize fleet movement state at the authored start coordinate.
pub fn init_fleet_movement_state(
    report: &TpFleetMovementAuthoringReport,
    cell_lookup: &BTreeMap<(u32, u32), SimThingId>,
) -> Result<TpFleetMovementState, String> {
    let start = TpFleetTheaterCoord {
        row: report.fleet_start.0,
        col: report.fleet_start.1,
    };
    let system_id = cell_lookup
        .get(&(start.row, start.col))
        .copied()
        .ok_or_else(|| format!("missing fleet start cell ({},{})", start.row, start.col))?;
    Ok(TpFleetMovementState {
        fleet_id: report.fleet_id,
        coord: start,
        prev_coord: start,
        enrolled_system_id: system_id,
    })
}

/// Initialize arena enrollment tracking for the fleet at its start cell.
pub fn init_fleet_arena_enrollment(
    report: &TpFleetMovementAuthoringReport,
    cell_lookup: &BTreeMap<(u32, u32), SimThingId>,
) -> Result<TpFleetArenaEnrollment, String> {
    let start = TpFleetTheaterCoord {
        row: report.fleet_start.0,
        col: report.fleet_start.1,
    };
    let system_id = cell_lookup
        .get(&(start.row, start.col))
        .copied()
        .ok_or_else(|| format!("missing fleet start enrollment cell"))?;
    Ok(TpFleetArenaEnrollment {
        fleet_id: report.fleet_id,
        arena_name: crate::fronts_post_hydration::TP_THREAT_ARENA.into(),
        authoritative_system_id: system_id,
        authoritative_coord: start,
    })
}

/// Reparent fleet state one adjacent D-gradient step and re-enroll arena authority.
pub fn fleet_movement_gradient_step(
    state: &mut TpFleetMovementState,
    enrollment: &mut TpFleetArenaEnrollment,
    d_flat: &[f32],
    grid_size: u32,
    cell_lookup: &BTreeMap<(u32, u32), SimThingId>,
) -> Result<PalmaReachGradientStep, String> {
    let from_slot = state.coord.slot(grid_size);
    let step = palma_reach_gradient_probe(d_flat, grid_size, grid_size, from_slot)
        .ok_or_else(|| "gradient probe found no finite adjacent D".to_string())?;
    if step.from_slot == step.to_slot {
        return Err("gradient step must move to a different cell".into());
    }
    let to_row = step.to_slot / grid_size;
    let to_col = step.to_slot % grid_size;
    let to_coord = TpFleetTheaterCoord {
        row: to_row,
        col: to_col,
    };
    if theater_manhattan(state.coord, to_coord) != 1 {
        return Err("movement must be adjacent-cell reparenting only".into());
    }
    if step.sampled_d >= d_flat[from_slot as usize] {
        return Err("gradient step must decrease resident D".into());
    }

    let new_system = cell_lookup
        .get(&(to_row, to_col))
        .copied()
        .ok_or_else(|| format!("missing theater cell ({to_row},{to_col})"))?;

    state.prev_coord = state.coord;
    state.coord = to_coord;
    state.enrolled_system_id = new_system;

    enrollment.authoritative_coord = to_coord;
    enrollment.authoritative_system_id = new_system;

    Ok(step)
}

/// Run multi-tick CPU movement oracle — returns observation only, not a route object.
pub fn simulate_fleet_movement_cpu(
    state: &mut TpFleetMovementState,
    enrollment: &mut TpFleetArenaEnrollment,
    d_flat: &[f32],
    grid_size: u32,
    cell_lookup: &BTreeMap<(u32, u32), SimThingId>,
    ticks: u32,
) -> Result<TpMovementObservation, String> {
    let mut obs = TpMovementObservation {
        ticks: vec![state.coord],
    };
    for _ in 0..ticks {
        fleet_movement_gradient_step(state, enrollment, d_flat, grid_size, cell_lookup)?;
        obs.ticks.push(state.coord);
    }
    Ok(obs)
}

/// Verify arena enrollment matches the fleet's current theater cell before the next tick.
pub fn arena_enrollment_matches_fleet_cell(
    enrollment: &TpFleetArenaEnrollment,
    state: &TpFleetMovementState,
    cell_lookup: &BTreeMap<(u32, u32), SimThingId>,
) -> Result<(), String> {
    if enrollment.authoritative_coord != state.coord {
        return Err("arena enrollment coord must match fleet coord".into());
    }
    if enrollment.authoritative_system_id != state.enrolled_system_id {
        return Err("arena enrollment system must match fleet enrolled system".into());
    }
    let expected = cell_lookup
        .get(&(state.coord.row, state.coord.col))
        .copied()
        .ok_or_else(|| "fleet cell missing from theater lookup".to_string())?;
    if expected != state.enrolled_system_id {
        return Err("fleet enrolled system must match authoritative theater cell".into());
    }
    Ok(())
}

/// Build theater (row,col) → system id lookup from authored cells.
pub fn movement_cell_lookup(
    theater_cells: &[TpFrontsTheaterCell],
) -> BTreeMap<(u32, u32), SimThingId> {
    theater_cells
        .iter()
        .map(|cell| ((cell.theater_row, cell.theater_col), cell.simthing_id))
        .collect()
}

pub fn movement_reach_dest(report: &TpFleetMovementAuthoringReport) -> (u32, u32) {
    report.reach_dest
}

pub fn movement_grid_size() -> u32 {
    TP_MOVEMENT_GRID_SIZE
}

pub fn movement_horizon() -> u32 {
    TP_MOVEMENT_HORIZON
}

pub fn movement_source_col() -> u32 {
    TP_FRONTS_SOURCE_COL
}
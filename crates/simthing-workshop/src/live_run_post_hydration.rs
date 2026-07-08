//! TP-LIVE-RUN-0 — workshop-homed bounded-theater live-run authoring over the full
//! Terran-Pirate transpile fixture.
//!
//! Homing: would not exist without the Terran-Pirate scenario (Mechanism B composition).
//! Chains accepted commitments (fronts + PALMA + 7×7 theater + STEAD thresholds) and
//! records placement re-bind from embedded lattice targets → authority SimThing ids.

use simthing_clausething::HydratedScenarioPack;
use simthing_core::SimThingId;

use crate::commitments_post_hydration::{
    apply_commitments_post_hydration, CommitmentsHydrationError, TpCommitmentsAuthoringReport,
};
use crate::fleet_movement_post_hydration::TP_MOVEMENT_GRID_SIZE;
use crate::fronts_post_hydration::TpFrontsTheaterCell;

/// Deterministic theater edge for the live-run proof (STEAD P1 bounded theater).
pub const TP_LIVE_RUN_THEATER_GRID: u32 = TP_MOVEMENT_GRID_SIZE;

/// Multi-tick proof floor (non-vacuous live run).
pub const TP_LIVE_RUN_MIN_TICKS: u32 = 3;

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

/// Live-run authoring report over the full transpile pack.
#[derive(Debug, Clone)]
pub struct TpLiveRunAuthoringReport {
    pub commitments: TpCommitmentsAuthoringReport,
    pub rebind: Vec<TpPlacementRebindEntry>,
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
    // Every rebind entry must resolve to a non-zero authority id (no dangling producer id).
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

    Ok(TpLiveRunAuthoringReport {
        commitments,
        rebind,
        theater_grid_size: TP_LIVE_RUN_THEATER_GRID,
        min_ticks: TP_LIVE_RUN_MIN_TICKS,
    })
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
    Ok(())
}

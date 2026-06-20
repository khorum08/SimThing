//! RECURSIVE-SPATIAL-GRID-DEFAULTS-0 — universal interior local-grid frame doctrine.
//!
//! Every spatial gridcell Location SimThing has an interior child grid.
//! Default interior grid is 1×1 unless explicitly expanded (star-system: 10×10).

use simthing_core::{SimThing, SimThingKind};

use super::scenario::{
    gridcell_role, scenario_metadata_string, scenario_metadata_u32,
    GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, LOCAL_GRIDCELL_COL_PROPERTY_ID, LOCAL_GRIDCELL_ROLE_INERT,
    LOCAL_GRIDCELL_ROLE_PROPERTY_ID, LOCAL_GRIDCELL_ROLE_RECEIVER, LOCAL_GRIDCELL_ROW_PROPERTY_ID,
    LOCAL_GRID_DEFAULT_COLS, LOCAL_GRID_DEFAULT_ROWS, LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
    LOCAL_GRID_FRAME_ROWS_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID,
};

pub(crate) fn local_gridcell_role(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, LOCAL_GRIDCELL_ROLE_PROPERTY_ID)
        .or_else(|| scenario_metadata_string(thing, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalGridFrame {
    pub cols: u32,
    pub rows: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalGridFrameErrorKind {
    FrameZeroDimension,
    FrameTooLarge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalGridFrameError {
    pub kind: LocalGridFrameErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalReceiverCellRole {
    Receiver,
    Inert,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalReceiverCell {
    pub parent_gridcell_simthing_id_raw: u32,
    pub col: u32,
    pub row: u32,
    pub role: LocalReceiverCellRole,
    pub is_implicit: bool,
    pub materialized_simthing_id_raw: Option<u32>,
}

pub fn is_local_coordinate_in_frame(frame: LocalGridFrame, col: u32, row: u32) -> bool {
    col < frame.cols && row < frame.rows
}

pub fn default_local_grid_frame_for_spatial_gridcell(gridcell: &SimThing) -> LocalGridFrame {
    match gridcell_role(gridcell).as_deref() {
        Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM) => LocalGridFrame {
            cols: STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
            rows: STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
        },
        _ => LocalGridFrame {
            cols: LOCAL_GRID_DEFAULT_COLS,
            rows: LOCAL_GRID_DEFAULT_ROWS,
        },
    }
}

pub fn explicit_local_grid_frame_for_spatial_gridcell(
    gridcell: &SimThing,
) -> Option<LocalGridFrame> {
    let star_cols = scenario_metadata_u32(gridcell, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID);
    let star_rows = scenario_metadata_u32(gridcell, STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID);
    if star_cols.is_some() || star_rows.is_some() {
        return Some(LocalGridFrame {
            cols: star_cols.unwrap_or(STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS),
            rows: star_rows.unwrap_or(STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS),
        });
    }

    let cols = scenario_metadata_u32(gridcell, LOCAL_GRID_FRAME_COLS_PROPERTY_ID)?;
    let rows = scenario_metadata_u32(gridcell, LOCAL_GRID_FRAME_ROWS_PROPERTY_ID)?;
    Some(LocalGridFrame { cols, rows })
}

pub fn local_grid_frame_for_spatial_gridcell(
    gridcell: &SimThing,
) -> Result<LocalGridFrame, LocalGridFrameError> {
    let frame = explicit_local_grid_frame_for_spatial_gridcell(gridcell)
        .unwrap_or_else(|| default_local_grid_frame_for_spatial_gridcell(gridcell));
    validate_local_grid_frame(gridcell, frame)
}

pub fn interior_local_grid_frame_for_gridcell(
    gridcell: &SimThing,
) -> Result<LocalGridFrame, LocalGridFrameError> {
    local_grid_frame_for_spatial_gridcell(gridcell)
}

fn validate_local_grid_frame(
    gridcell: &SimThing,
    frame: LocalGridFrame,
) -> Result<LocalGridFrame, LocalGridFrameError> {
    if frame.cols == 0 || frame.rows == 0 {
        return Err(LocalGridFrameError {
            kind: LocalGridFrameErrorKind::FrameZeroDimension,
            message: format!(
                "spatial gridcell {} local frame must be non-zero",
                gridcell.id.raw()
            ),
        });
    }
    if gridcell_role(gridcell).as_deref() == Some(GALAXY_GRIDCELL_ROLE_INERT) {
        if frame.cols > LOCAL_GRID_DEFAULT_COLS || frame.rows > LOCAL_GRID_DEFAULT_ROWS {
            return Err(LocalGridFrameError {
                kind: LocalGridFrameErrorKind::FrameTooLarge,
                message: format!(
                    "inert gridcell {} cannot expand local frame beyond 1x1 yet",
                    gridcell.id.raw()
                ),
            });
        }
    }
    Ok(frame)
}

pub fn is_receiver_local_gridcell(child: &SimThing) -> bool {
    if child.kind != SimThingKind::Location {
        return false;
    }
    matches!(
        local_gridcell_role(child).as_deref(),
        Some(LOCAL_GRIDCELL_ROLE_RECEIVER) | Some(LOCAL_GRIDCELL_ROLE_INERT)
    )
}

pub fn implicit_receiver_cell_for_gridcell(gridcell: &SimThing) -> LocalReceiverCell {
    LocalReceiverCell {
        parent_gridcell_simthing_id_raw: gridcell.id.raw(),
        col: 0,
        row: 0,
        role: LocalReceiverCellRole::Receiver,
        is_implicit: true,
        materialized_simthing_id_raw: None,
    }
}

pub fn materialized_receiver_cell(child: &SimThing, parent_raw: u32) -> LocalReceiverCell {
    let role = match local_gridcell_role(child).as_deref() {
        Some(LOCAL_GRIDCELL_ROLE_INERT) => LocalReceiverCellRole::Inert,
        _ => LocalReceiverCellRole::Receiver,
    };
    LocalReceiverCell {
        parent_gridcell_simthing_id_raw: parent_raw,
        col: scenario_metadata_u32(child, LOCAL_GRIDCELL_COL_PROPERTY_ID).unwrap_or(0),
        row: scenario_metadata_u32(child, LOCAL_GRIDCELL_ROW_PROPERTY_ID).unwrap_or(0),
        role,
        is_implicit: false,
        materialized_simthing_id_raw: Some(child.id.raw()),
    }
}

pub fn local_gridcell_col(child: &SimThing) -> Option<u32> {
    scenario_metadata_u32(child, LOCAL_GRIDCELL_COL_PROPERTY_ID)
}

pub fn local_gridcell_row(child: &SimThing) -> Option<u32> {
    scenario_metadata_u32(child, LOCAL_GRIDCELL_ROW_PROPERTY_ID)
}

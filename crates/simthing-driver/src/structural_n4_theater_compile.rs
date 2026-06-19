//! Compile `SimThingScenarioSpec` structural grid placements into an N4 theater surface.
//!
//! Grid N4 adjacency is derived from authoritative `(col,row)` placements only.
//! Hyperlane `scenario.links` are not consulted. Execution-profile admission may
//! defer oversize frames to atlas scheduling without shrinking structural layout.

use simthing_spec::{
    validate_stead_mapping_consistency, MappingExecutionProfile, SimThingScenarioSpec,
    SteadMappingError, REGION_FIELD_MAX_CELL_COUNT, REGION_FIELD_STANDARD_MAX_GRID,
};
use thiserror::Error;

/// Semantic-free structural grid coordinate (col, row).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructuralGridCoordinate {
    pub col: u32,
    pub row: u32,
}

/// One occupied structural placement admitted into the theater compile surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompiledStructuralPlacement {
    pub system_id: u32,
    pub col: u32,
    pub row: u32,
    pub location_id: String,
    pub simthing_id_raw: u32,
}

/// Driver-owned structural N4 theater derived from scenario authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompiledStructuralN4Theater {
    pub frame_width: u32,
    pub frame_height: u32,
    pub occupied_cells: Vec<StructuralGridCoordinate>,
    pub n4_edges: Vec<(StructuralGridCoordinate, StructuralGridCoordinate)>,
    pub system_placements: Vec<CompiledStructuralPlacement>,
    pub execution_profile: MappingExecutionProfile,
}

impl CompiledStructuralN4Theater {
    pub fn cell_slot(&self, coord: StructuralGridCoordinate) -> u32 {
        coord.row * self.frame_width + coord.col
    }

    pub fn placement_for_system(&self, system_id: u32) -> Option<&CompiledStructuralPlacement> {
        self.system_placements
            .iter()
            .find(|placement| placement.system_id == system_id)
    }

    pub fn coord_for_system(&self, system_id: u32) -> Option<StructuralGridCoordinate> {
        self.placement_for_system(system_id)
            .map(|placement| StructuralGridCoordinate {
                col: placement.col,
                row: placement.row,
            })
    }

    pub fn has_n4_edge(&self, a: StructuralGridCoordinate, b: StructuralGridCoordinate) -> bool {
        let edge = ordered_n4_edge(a, b);
        self.n4_edges.iter().any(|existing| *existing == edge)
    }

    pub fn occupied_set(&self) -> std::collections::BTreeSet<StructuralGridCoordinate> {
        self.occupied_cells.iter().copied().collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasDeferralReason {
    FrameExceedsStandardMaxGrid {
        width: u32,
        height: u32,
        max_grid: u32,
    },
    CellCountExceedsCap {
        cells: u32,
        cap: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StructuralTheaterAdmission {
    Admit(CompiledStructuralN4Theater),
    AtlasDeferred {
        frame_width: u32,
        frame_height: u32,
        occupied_cells: u64,
        reason: AtlasDeferralReason,
    },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StructuralTheaterCompileError {
    #[error(transparent)]
    SteadMapping(#[from] SteadMappingError),
    #[error(
        "occupied placement count {placements} does not match frame.occupied_cells {frame_occupied}"
    )]
    OccupiedCellCountMismatch {
        placements: usize,
        frame_occupied: u64,
    },
    #[error("placement system_id={system_id} at ({col},{row}) is outside frame {width}x{height}")]
    PlacementOutOfFrame {
        system_id: u32,
        col: u32,
        row: u32,
        width: u32,
        height: u32,
    },
    #[error("structural frame dimensions overflow")]
    FrameDimensionOverflow,
}

fn is_n4_neighbor(a: StructuralGridCoordinate, b: StructuralGridCoordinate) -> bool {
    (a.col.abs_diff(b.col) == 1 && a.row == b.row) || (a.row.abs_diff(b.row) == 1 && a.col == b.col)
}

fn ordered_n4_edge(
    a: StructuralGridCoordinate,
    b: StructuralGridCoordinate,
) -> (StructuralGridCoordinate, StructuralGridCoordinate) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

pub(crate) fn derive_n4_edges(
    occupied: &[StructuralGridCoordinate],
) -> Vec<(StructuralGridCoordinate, StructuralGridCoordinate)> {
    let mut edges = Vec::new();
    for i in 0..occupied.len() {
        for j in (i + 1)..occupied.len() {
            if is_n4_neighbor(occupied[i], occupied[j]) {
                edges.push(ordered_n4_edge(occupied[i], occupied[j]));
            }
        }
    }
    edges.sort_unstable();
    edges.dedup();
    edges
}

pub(crate) fn build_theater_geometry(
    scenario: &SimThingScenarioSpec,
    profile: MappingExecutionProfile,
) -> Result<CompiledStructuralN4Theater, StructuralTheaterCompileError> {
    validate_stead_mapping_consistency(scenario)?;

    let frame = &scenario.structural_grid.frame;
    let mut system_placements = Vec::with_capacity(scenario.structural_grid.placements.len());
    let mut occupied_cells = Vec::with_capacity(scenario.structural_grid.placements.len());

    for placement in &scenario.structural_grid.placements {
        if placement.col >= frame.width || placement.row >= frame.height {
            return Err(StructuralTheaterCompileError::PlacementOutOfFrame {
                system_id: placement.system_id,
                col: placement.col,
                row: placement.row,
                width: frame.width,
                height: frame.height,
            });
        }
        let coord = StructuralGridCoordinate {
            col: placement.col,
            row: placement.row,
        };
        occupied_cells.push(coord);
        system_placements.push(CompiledStructuralPlacement {
            system_id: placement.system_id,
            col: placement.col,
            row: placement.row,
            location_id: placement.location_id.clone(),
            simthing_id_raw: placement.simthing_id_raw,
        });
    }

    if system_placements.len() != frame.occupied_cells as usize {
        return Err(StructuralTheaterCompileError::OccupiedCellCountMismatch {
            placements: system_placements.len(),
            frame_occupied: frame.occupied_cells,
        });
    }

    occupied_cells.sort_unstable();
    system_placements.sort_by_key(|placement| placement.system_id);

    Ok(CompiledStructuralN4Theater {
        frame_width: frame.width,
        frame_height: frame.height,
        n4_edges: derive_n4_edges(&occupied_cells),
        occupied_cells,
        system_placements,
        execution_profile: profile,
    })
}

fn evaluate_execution_admission(
    theater: CompiledStructuralN4Theater,
) -> Result<StructuralTheaterAdmission, StructuralTheaterCompileError> {
    let cells = theater
        .frame_width
        .checked_mul(theater.frame_height)
        .ok_or(StructuralTheaterCompileError::FrameDimensionOverflow)?;

    if theater.frame_width > REGION_FIELD_STANDARD_MAX_GRID
        || theater.frame_height > REGION_FIELD_STANDARD_MAX_GRID
    {
        return Ok(StructuralTheaterAdmission::AtlasDeferred {
            frame_width: theater.frame_width,
            frame_height: theater.frame_height,
            occupied_cells: theater.occupied_cells.len() as u64,
            reason: AtlasDeferralReason::FrameExceedsStandardMaxGrid {
                width: theater.frame_width,
                height: theater.frame_height,
                max_grid: REGION_FIELD_STANDARD_MAX_GRID,
            },
        });
    }

    if cells > REGION_FIELD_MAX_CELL_COUNT {
        return Ok(StructuralTheaterAdmission::AtlasDeferred {
            frame_width: theater.frame_width,
            frame_height: theater.frame_height,
            occupied_cells: theater.occupied_cells.len() as u64,
            reason: AtlasDeferralReason::CellCountExceedsCap {
                cells,
                cap: REGION_FIELD_MAX_CELL_COUNT,
            },
        });
    }

    Ok(StructuralTheaterAdmission::Admit(theater))
}

/// Compile structural N4 theater geometry and evaluate bounded execution admission.
///
/// Reads `structural_grid.frame` and `structural_grid.placements` only. Does not
/// use `scenario.links`, render coordinates, emission order, or row-major fill.
pub fn compile_structural_n4_theater(
    scenario: &SimThingScenarioSpec,
    profile: MappingExecutionProfile,
) -> Result<StructuralTheaterAdmission, StructuralTheaterCompileError> {
    let theater = build_theater_geometry(scenario, profile)?;
    evaluate_execution_admission(theater)
}

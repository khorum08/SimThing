//! Shape strategy trait and in-memory placement output (PR3 — no emitter).

use thiserror::Error;

use crate::lattice::{CoreMask, LatticeCoord, SquareLattice};
use crate::occupancy::{OccupancyError, OccupancyGrid};
use crate::params::MapGeneratorParams;
use crate::rng::MapGenRng;
use crate::shape_registry::ShapeStrategyDescriptor;

/// One placed system seed — integer lattice cell only; no links, fields, or runtime payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedSystemSeed {
    pub id: u32,
    pub coord: LatticeCoord,
    pub bucket: Option<String>,
}

/// In-memory strategy output (producer-side only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapePlacement {
    pub systems: Vec<PlacedSystemSeed>,
}

#[derive(Debug, Error, PartialEq)]
pub enum ShapePlacementError {
    #[error("shape '{shape}' is not registered; registered shapes: {registered}")]
    UnknownShape { shape: String, registered: String },
    #[error(
        "shape '{shape}' is registered but has no executable strategy in PR3; executable shapes: {executable}"
    )]
    StrategyNotImplemented { shape: String, executable: String },
    #[error("explicit static cells required for shape '{shape}'")]
    ExplicitCellsRequired { shape: String },
    #[error("occupancy error: {0}")]
    Occupancy(#[from] OccupancyError),
    #[error("insufficient ellipse candidates for {requested} systems (found {available})")]
    InsufficientCandidates { requested: u32, available: usize },
}

/// Inputs shared by every registered shape strategy.
pub struct ShapeStrategyContext<'a> {
    pub params: &'a MapGeneratorParams,
    pub descriptor: &'a ShapeStrategyDescriptor,
    pub lattice: &'a SquareLattice,
    pub core_mask: &'a CoreMask,
    pub occupancy: &'a mut OccupancyGrid,
    pub rng: &'a mut MapGenRng,
    /// In-memory explicit cells for static / arbitrary_static strategies (PR3 test seam only).
    pub explicit_cells: Option<&'a [LatticeCoord]>,
}

/// Producer-side shape placement strategy (registry-resolved by name).
pub trait ShapeStrategy: Send + Sync {
    fn name(&self) -> &str;

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError>;
}

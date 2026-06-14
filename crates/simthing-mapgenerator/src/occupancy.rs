//! One-system-per-cell occupancy with deterministic collision relocation (PR2).

use thiserror::Error;

use crate::lattice::{CoreMask, LatticeCoord, SquareLattice};
use crate::rng::MapGenRng;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OccupancyError {
    #[error("coordinate out of lattice bounds")]
    OutOfBounds,
    #[error("coordinate is inside the core mask")]
    CoreMasked,
    #[error("cell already occupied")]
    AlreadyOccupied,
    #[error("no placeable cells remain on the lattice")]
    LatticeExhausted,
}

/// Occupancy set enforcing one system per gridcell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccupancyGrid {
    lattice: SquareLattice,
    core_mask: CoreMask,
    occupied: Vec<LatticeCoord>,
    occupied_set: std::collections::BTreeSet<(u32, u32)>,
}

impl OccupancyGrid {
    pub fn new(lattice: SquareLattice, core_mask: CoreMask) -> Self {
        Self {
            lattice,
            core_mask,
            occupied: Vec::new(),
            occupied_set: std::collections::BTreeSet::new(),
        }
    }

    pub fn lattice(&self) -> &SquareLattice {
        &self.lattice
    }

    pub fn core_mask(&self) -> &CoreMask {
        &self.core_mask
    }

    pub fn contains(&self, coord: LatticeCoord) -> bool {
        self.occupied_set.contains(&(coord.col, coord.row))
    }

    pub fn occupied_coords(&self) -> &[LatticeCoord] {
        &self.occupied
    }

    pub fn occupied_count(&self) -> usize {
        self.occupied.len()
    }

    fn validate_placeable(&self, coord: LatticeCoord) -> Result<(), OccupancyError> {
        if !self.lattice.contains(coord) {
            return Err(OccupancyError::OutOfBounds);
        }
        if !self.core_mask.is_placeable(coord) {
            return Err(OccupancyError::CoreMasked);
        }
        Ok(())
    }

    fn commit(&mut self, coord: LatticeCoord) {
        self.occupied_set.insert((coord.col, coord.row));
        self.occupied.push(coord);
    }

    /// Insert only when the cell is free; rejects duplicates.
    pub fn try_insert(&mut self, coord: LatticeCoord) -> Result<(), OccupancyError> {
        self.validate_placeable(coord)?;
        if self.contains(coord) {
            return Err(OccupancyError::AlreadyOccupied);
        }
        self.commit(coord);
        Ok(())
    }

    /// Insert at `coord` or relocate to another free placeable cell using deterministic probing.
    pub fn insert_or_relocate(
        &mut self,
        coord: LatticeCoord,
        rng: &mut MapGenRng,
    ) -> Result<LatticeCoord, OccupancyError> {
        if self.validate_placeable(coord).is_ok() && !self.contains(coord) {
            self.commit(coord);
            return Ok(coord);
        }
        self.insert_relocated(rng)
    }

    /// Place the next system at a deterministic free placeable cell (generic probe, not shape-aware).
    pub fn insert_next(&mut self, rng: &mut MapGenRng) -> Result<LatticeCoord, OccupancyError> {
        self.insert_relocated(rng)
    }

    fn insert_relocated(&mut self, rng: &mut MapGenRng) -> Result<LatticeCoord, OccupancyError> {
        let placeable: Vec<LatticeCoord> = self
            .lattice
            .iter_coords()
            .filter(|c| self.core_mask.is_placeable(*c) && !self.contains(*c))
            .collect();
        if placeable.is_empty() {
            return Err(OccupancyError::LatticeExhausted);
        }
        let start = rng.gen_index(placeable.len() as u32) as usize;
        for offset in 0..placeable.len() {
            let coord = placeable[(start + offset) % placeable.len()];
            if !self.contains(coord) {
                self.commit(coord);
                return Ok(coord);
            }
        }
        Err(OccupancyError::LatticeExhausted)
    }
}

//! One-system-per-cell occupancy with deterministic collision relocation (PR2).
//!
//! PR11: placeable cells are precomputed once; relocation probes the free-placeable list without
//! rebuilding the full lattice scan per insertion.

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
    /// Precomputed placeable cells in stable row-major order.
    placeable_cells: Vec<LatticeCoord>,
    /// Indices into `placeable_cells` that remain free.
    free_placeable_indices: Vec<usize>,
    /// Diagnostic counter; remains zero when relocation uses the precomputed free list only.
    placeable_full_scan_count: u32,
}

impl OccupancyGrid {
    pub fn new(lattice: SquareLattice, core_mask: CoreMask) -> Self {
        let placeable_cells: Vec<LatticeCoord> = lattice
            .iter_coords()
            .filter(|coord| core_mask.is_placeable(*coord))
            .collect();
        let free_placeable_indices: Vec<usize> = (0..placeable_cells.len()).collect();
        Self {
            lattice,
            core_mask,
            occupied: Vec::new(),
            occupied_set: std::collections::BTreeSet::new(),
            placeable_cells,
            free_placeable_indices,
            placeable_full_scan_count: 0,
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

    pub fn placeable_cell_count(&self) -> usize {
        self.placeable_cells.len()
    }

    pub fn free_placeable_count(&self) -> usize {
        self.free_placeable_indices.len()
    }

    pub fn placeable_full_scan_count(&self) -> u32 {
        self.placeable_full_scan_count
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

    fn remove_free_coord(&mut self, coord: LatticeCoord) {
        if let Some(index) = self
            .free_placeable_indices
            .iter()
            .position(|&placeable_index| self.placeable_cells[placeable_index] == coord)
        {
            self.free_placeable_indices.swap_remove(index);
        }
    }

    fn commit(&mut self, coord: LatticeCoord) {
        self.occupied_set.insert((coord.col, coord.row));
        self.occupied.push(coord);
        self.remove_free_coord(coord);
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
        if self.free_placeable_indices.is_empty() {
            return Err(OccupancyError::LatticeExhausted);
        }
        let start = rng.gen_index(self.free_placeable_indices.len() as u32) as usize;
        for offset in 0..self.free_placeable_indices.len() {
            let slot = (start + offset) % self.free_placeable_indices.len();
            let placeable_index = self.free_placeable_indices[slot];
            let coord = self.placeable_cells[placeable_index];
            if !self.contains(coord) {
                self.commit(coord);
                return Ok(coord);
            }
            self.free_placeable_indices.swap_remove(slot);
            if self.free_placeable_indices.is_empty() {
                break;
            }
        }
        Err(OccupancyError::LatticeExhausted)
    }
}

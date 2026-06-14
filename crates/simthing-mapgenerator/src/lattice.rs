//! Square integer lattice and producer-side core mask (PR2).

use thiserror::Error;

use crate::params::ScaleCoreParams;

/// Integer grid cell `(col, row)` on a square lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LatticeCoord {
    pub col: u32,
    pub row: u32,
}

/// Square lattice with `width == height`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SquareLattice {
    edge: u32,
}

/// Producer-side central void mask (integer squared-distance from lattice center).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreMask {
    center_col: u32,
    center_row: u32,
    /// Squared radius in **cell units** (integer; producer-side quantization only).
    radius_sq: u32,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LatticeError {
    #[error("square lattice edge must be > 0")]
    ZeroEdge,
}

impl SquareLattice {
    pub fn new(edge: u32) -> Result<Self, LatticeError> {
        if edge == 0 {
            return Err(LatticeError::ZeroEdge);
        }
        Ok(Self { edge })
    }

    pub fn edge(&self) -> u32 {
        self.edge
    }

    pub fn cell_count(&self) -> u32 {
        self.edge * self.edge
    }

    pub fn contains(&self, coord: LatticeCoord) -> bool {
        coord.col < self.edge && coord.row < self.edge
    }

    pub fn index(&self, coord: LatticeCoord) -> Option<u32> {
        if !self.contains(coord) {
            return None;
        }
        Some(coord.col + coord.row * self.edge)
    }

    pub fn from_index(&self, index: u32) -> Option<LatticeCoord> {
        if index >= self.cell_count() {
            return None;
        }
        Some(LatticeCoord {
            col: index % self.edge,
            row: index / self.edge,
        })
    }

    /// Stable row-major iteration over all cells.
    pub fn iter_coords(&self) -> impl Iterator<Item = LatticeCoord> + '_ {
        (0..self.edge).flat_map(move |row| (0..self.edge).map(move |col| LatticeCoord { col, row }))
    }

    /// Lattice center used for core masking (integer division).
    pub fn center(&self) -> LatticeCoord {
        LatticeCoord {
            col: self.edge / 2,
            row: self.edge / 2,
        }
    }

    /// Build a core mask from scale params.
    ///
    /// `core_radius` and `radius` are producer-side floats; the mask quantizes to integer cell
    /// units relative to half the lattice edge. No declarative output or sim authority.
    pub fn core_mask_from_scale(&self, core_radius: f64, radius: f64) -> CoreMask {
        let center = self.center();
        let radius_cells = quantize_core_radius_cells(self.edge, core_radius, radius);
        CoreMask {
            center_col: center.col,
            center_row: center.row,
            radius_sq: radius_cells.saturating_mul(radius_cells),
        }
    }

    /// Derive square edge from PR1 scale params (`lattice_size` or default 200×200).
    pub fn edge_from_scale(scale: &ScaleCoreParams) -> Result<u32, LatticeError> {
        if let Some(size) = scale.lattice_size {
            Self::new(size).map(|l| l.edge)
        } else {
            Self::new(200).map(|l| l.edge)
        }
    }
}

impl CoreMask {
    pub fn new(center_col: u32, center_row: u32, radius_cells: u32) -> Self {
        Self {
            center_col,
            center_row,
            radius_sq: radius_cells.saturating_mul(radius_cells),
        }
    }

    pub fn is_masked(&self, coord: LatticeCoord) -> bool {
        if self.radius_sq == 0 {
            return false;
        }
        let dc = coord.col.abs_diff(self.center_col);
        let dr = coord.row.abs_diff(self.center_row);
        let dist_sq = dc.saturating_mul(dc).saturating_add(dr.saturating_mul(dr));
        dist_sq <= self.radius_sq
    }

    pub fn is_placeable(&self, coord: LatticeCoord) -> bool {
        !self.is_masked(coord)
    }
}

fn quantize_core_radius_cells(edge: u32, core_radius: f64, radius: f64) -> u32 {
    if !core_radius.is_finite() || core_radius <= 0.0 {
        return 0;
    }
    if !radius.is_finite() || radius <= 0.0 {
        return 0;
    }
    let half = (edge / 2).max(1) as f64;
    let cells = (core_radius / radius * half).floor();
    if !cells.is_finite() || cells <= 0.0 {
        0
    } else {
        cells as u32
    }
}

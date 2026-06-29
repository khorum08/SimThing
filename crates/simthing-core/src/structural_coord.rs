//! Structural grid coordinates — integer lattice authority, never render floats.
//!
//! Structural map logic must not accept bare render `(f32, f32)` pairs. Construct
//! [`StructuralCoord`] from integer grid indices, or convert render space only
//! through [`RenderCoord::to_structural_cell`].
//!
//! Float construction is forbidden:
//!
//! ```compile_fail
//! use simthing_core::StructuralCoord;
//! let _ = StructuralCoord::new(1.0_f32, 2.0_f32);
//! ```
//!
//! Direct field construction is forbidden:
//!
//! ```compile_fail
//! use simthing_core::StructuralCoord;
//! let _ = StructuralCoord { col: 1, row: 2 };
//! ```

/// Authoritative integer gridcell coordinate on the structural lattice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructuralCoord {
    col: u32,
    row: u32,
}

impl StructuralCoord {
    /// Construct from integer structural grid indices.
    pub fn new(col: u32, row: u32) -> Self {
        Self { col, row }
    }

    pub fn col(self) -> u32 {
        self.col
    }

    pub fn row(self) -> u32 {
        self.row
    }

    pub fn as_tuple(self) -> (u32, u32) {
        (self.col, self.row)
    }

    pub fn into_tuple(self) -> (u32, u32) {
        (self.col, self.row)
    }

    /// Explicit render/UI → structural conversion (floor, non-negative).
    ///
    /// Prefer [`RenderCoord::to_structural_cell`] at UI boundaries; this exists
    /// for named admission when only raw render floats are available.
    pub fn from_render_floor(x: f32, y: f32) -> Self {
        Self {
            col: x.floor().max(0.0) as u32,
            row: y.floor().max(0.0) as u32,
        }
    }
}

/// Optional cosmetic render-space coordinate (never authoritative for STEAD).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderCoord {
    x: f32,
    y: f32,
}

impl RenderCoord {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
    }

    /// Explicit boundary conversion from render floats to structural grid cells.
    pub fn to_structural_cell(self) -> StructuralCoord {
        StructuralCoord::from_render_floor(self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::{RenderCoord, StructuralCoord};

    #[test]
    fn structural_coord_integer_roundtrip_preserved() {
        let coord = StructuralCoord::new(3, 7);
        assert_eq!(coord.col(), 3);
        assert_eq!(coord.row(), 7);
        assert_eq!(coord.as_tuple(), (3, 7));
        assert_eq!(coord.into_tuple(), (3, 7));
    }

    #[test]
    fn render_coord_to_structural_cell_floors_at_boundary() {
        let render = RenderCoord::new(3.9, 7.1);
        assert_eq!(render.to_structural_cell(), StructuralCoord::new(3, 7));
    }

    #[test]
    fn structural_coord_from_render_floor_matches_render_boundary() {
        assert_eq!(
            StructuralCoord::from_render_floor(4.2, -1.0),
            StructuralCoord::new(4, 0)
        );
    }
}

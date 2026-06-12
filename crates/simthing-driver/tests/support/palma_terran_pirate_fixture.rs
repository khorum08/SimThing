//! PALMA-PATH-3 — Terran convoy / pirate fleet field-sampling fixture helpers.
//!
//! Fixture names (Terran convoy, pirate fleet, blockade, fuel shortage) are readability only.
//! All field math is numeric `W` / `D` — the min-plus stencil never branches on semantics.

use simthing_feeder::BoundaryRequest;
use simthing_gpu::MinPlusStencilConfig;

use super::palma_min_plus_oracle::{cell_index, run_min_plus_relaxation};

pub const FIXTURE_WIDTH: u32 = 8;
pub const FIXTURE_HEIGHT: u32 = 8;
pub const FIXTURE_ITERATIONS: u32 = 64;

/// Destination station — min-plus seed `D = 0`.
pub const DESTINATION: (u32, u32) = (0, 0);

/// Terran convoy start cell (movable SimThing parentage would reparent toward sampled neighbor).
pub const CONVOY_START: (usize, usize) = (7, 7);

/// Gap cell in the partial-wall detour corridor (numeric `W` only).
pub const GAP_CELL: (usize, usize) = (4, 4);

/// Pirate fleet pressure anchor (numeric `W` bump on anchor + N4 cells).
pub const PIRATE_ANCHOR: (usize, usize) = (5, 5);

const PIRATE_W_BUMP: f32 = 40.0;
const BLOCKADE_W: f32 = 100.0;
const FUEL_SHORTAGE_W: f32 = 8.0;
const FUEL_SHORTAGE_THRESHOLD: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
}

impl GridCoord {
    pub fn idx(self, width: usize) -> usize {
        cell_index(self.x, self.y, width)
    }
}

/// One field-sampling step: convoy reads neighbor `D` values and picks the lowest finite cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldSampleStep {
    pub from: GridCoord,
    pub to: GridCoord,
    pub sampled_d: f32,
}

/// Numeric Location-owned impedance field before min-plus relaxation.
#[derive(Clone, Debug)]
pub struct LocationImpedanceField {
    pub w: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl LocationImpedanceField {
    pub fn len(&self) -> usize {
        self.w.len()
    }

    pub fn config(&self) -> MinPlusStencilConfig {
        super::palma_min_plus_oracle::test_config(
            self.width as u32,
            self.height as u32,
            DESTINATION,
        )
    }

    pub fn compute_d(&self) -> Result<Vec<f32>, simthing_gpu::MinPlusStencilError> {
        run_min_plus_relaxation(&self.w, &self.config(), FIXTURE_ITERATIONS)
    }
}

/// Build baseline `W = 1` with optional partial-wall gap, pirate pressure, blockade, fuel shortage.
pub fn build_location_w_field(
    gap_open: bool,
    pirate: Option<(usize, usize)>,
    fuel_shortage: bool,
) -> LocationImpedanceField {
    let width = FIXTURE_WIDTH as usize;
    let height = FIXTURE_HEIGHT as usize;
    let mut w = vec![1.0f32; width * height];

    for y in 1..=6 {
        if (4, y) != GAP_CELL {
            w[cell_index(4, y, width)] = BLOCKADE_W;
        }
    }
    if !gap_open {
        w[cell_index(GAP_CELL.0, GAP_CELL.1, width)] = BLOCKADE_W;
    }
    for x in 1..width {
        w[cell_index(x, 0, width)] = BLOCKADE_W;
    }
    for x in 0..width - 1 {
        w[cell_index(x, height - 1, width)] = BLOCKADE_W;
    }

    if let Some(anchor) = pirate {
        apply_pirate_pressure(&mut w, width, anchor, PIRATE_W_BUMP);
    }

    if fuel_shortage {
        for y in 0..height {
            for x in 0..width {
                if x + y >= FUEL_SHORTAGE_THRESHOLD {
                    w[cell_index(x, y, width)] += FUEL_SHORTAGE_W;
                }
            }
        }
    }

    LocationImpedanceField { w, width, height }
}

/// Numeric pirate-fleet pressure: raises `W` on anchor and its four neighbors.
pub fn apply_pirate_pressure(w: &mut [f32], width: usize, anchor: (usize, usize), bump: f32) {
    let (ax, ay) = anchor;
    let height = FIXTURE_HEIGHT as usize;
    let mut cells = vec![(ax, ay)];
    if ax > 0 {
        cells.push((ax - 1, ay));
    }
    if ax + 1 < FIXTURE_WIDTH as usize {
        cells.push((ax + 1, ay));
    }
    if ay > 0 {
        cells.push((ax, ay - 1));
    }
    if ay + 1 < height {
        cells.push((ax, ay + 1));
    }
    for (x, y) in cells {
        w[cell_index(x, y, width)] += bump;
    }
}

/// Clear blockade overlay on the gap cell (numeric `W` only).
pub fn clear_blockade_gap(w: &mut [f32], width: usize) {
    w[cell_index(GAP_CELL.0, GAP_CELL.1, width)] = 1.0;
}

/// Sample the lowest-`D` four-neighbor step from a movable convoy position. No route object.
pub fn sample_lowest_d_neighbor(
    d: &[f32],
    width: usize,
    height: usize,
    from: GridCoord,
) -> Option<FieldSampleStep> {
    let mut best: Option<FieldSampleStep> = None;
    let candidates = [
        (from.x.wrapping_sub(1), from.y),
        (from.x + 1, from.y),
        (from.x, from.y.wrapping_sub(1)),
        (from.x, from.y + 1),
    ];
    for (x, y) in candidates {
        if x >= width || y >= height {
            continue;
        }
        let v = d[cell_index(x, y, width)];
        if !v.is_finite() {
            continue;
        }
        let step = FieldSampleStep {
            from,
            to: GridCoord { x, y },
            sampled_d: v,
        };
        if best.as_ref().is_none_or(|b| v < b.sampled_d) {
            best = Some(step);
        }
    }
    best
}

/// Map a sampled grid step to the existing generic `BoundaryRequest::Reparent` shape.
///
/// `convoy_id` is the movable SimThing; `target_gridcell_id` is the Location-owned gridcell
/// SimThing parent the convoy would adopt — no movement engine, no new request variant.
pub fn reparent_toward_sampled_gridcell(
    convoy_id: simthing_core::SimThingId,
    target_gridcell_id: simthing_core::SimThingId,
) -> BoundaryRequest {
    BoundaryRequest::Reparent {
        child: convoy_id,
        new_parent: target_gridcell_id,
    }
}

/// Fixture gridcell SimThing ids (deterministic placeholders for BoundaryRequest shape proof).
pub fn gridcell_simthing_id(x: usize, y: usize) -> simthing_core::SimThingId {
    simthing_core::SimThingId::from_session_raw(1 + (y * FIXTURE_WIDTH as usize + x) as u32)
}

pub fn convoy_simthing_id() -> simthing_core::SimThingId {
    simthing_core::SimThingId::from_session_raw(9001)
}

//! CPU-only min-plus relaxation oracle for Location gridcell traversal-cost fields.
//!
//! Binding convention (PALMA-PATH-0 guide, cell-entry form):
//! ```text
//! D_next[cell] = W[cell] + min_{neighbor ∈ N4(cell)} D_current[neighbor]
//! ```
//! The destination cell is pinned to `D = 0` each iteration (single-source seed).
//!
//! The update sees only numeric `W` and `D` buffers — no fleet/pirate/blockade semantics.
//! Terran convoy / pirate fleet scenarios in tests use numeric `W` fields only.

pub const INF: f32 = f32::INFINITY;

#[inline]
pub fn cell_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// One min-plus relaxation step over a square `width × height` grid with four-neighbor adjacency.
pub fn relax_min_plus_cell_entry(
    d_current: &[f32],
    w: &[f32],
    width: usize,
    height: usize,
    dest_idx: usize,
) -> Vec<f32> {
    assert_eq!(d_current.len(), width * height);
    assert_eq!(w.len(), width * height);
    assert!(dest_idx < d_current.len());

    let mut d_next = vec![INF; d_current.len()];
    for y in 0..height {
        for x in 0..width {
            let i = cell_index(x, y, width);
            if i == dest_idx {
                d_next[i] = 0.0;
                continue;
            }

            let mut best_neighbor = INF;
            if x > 0 {
                best_neighbor = best_neighbor.min(d_current[cell_index(x - 1, y, width)]);
            }
            if x + 1 < width {
                best_neighbor = best_neighbor.min(d_current[cell_index(x + 1, y, width)]);
            }
            if y > 0 {
                best_neighbor = best_neighbor.min(d_current[cell_index(x, y - 1, width)]);
            }
            if y + 1 < height {
                best_neighbor = best_neighbor.min(d_current[cell_index(x, y + 1, width)]);
            }

            d_next[i] = if best_neighbor.is_finite() {
                w[i] + best_neighbor
            } else {
                INF
            };
        }
    }
    d_next
}

/// Run a fixed number of min-plus iterations; returns the final scalar field `D` only.
pub fn run_min_plus_relaxation(
    w: &[f32],
    width: usize,
    height: usize,
    dest_idx: usize,
    iterations: usize,
) -> Vec<f32> {
    let mut d = vec![INF; w.len()];
    d[dest_idx] = 0.0;
    for _ in 0..iterations {
        d = relax_min_plus_cell_entry(&d, w, width, height, dest_idx);
    }
    d
}

/// Build a uniform `W` field with an optional pirate/blockade high-impedance corridor band.
pub fn terran_pirate_grid_w_field(
    width: usize,
    height: usize,
    base_w: f32,
    blockade_cells: &[(usize, usize)],
    blockade_w: f32,
) -> Vec<f32> {
    let mut w = vec![base_w; width * height];
    for &(x, y) in blockade_cells {
        w[cell_index(x, y, width)] = blockade_w;
    }
    w
}

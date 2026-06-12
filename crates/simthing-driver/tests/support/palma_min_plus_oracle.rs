//! CPU-only min-plus relaxation oracle for Location gridcell traversal-cost fields.
//!
//! Thin wrappers over `simthing_gpu::min_plus_stencil` — canonical CPU logic lives in simthing-gpu
//! for PALMA-PATH-2 GPU parity.

pub use simthing_gpu::min_plus_stencil::{
    cell_index, cpu_min_plus_d_from_w as run_min_plus_relaxation, MIN_PLUS_INF as INF,
};

/// Build a uniform `W` field with optional high-impedance cells (numeric only).
pub fn terran_pirate_grid_w_field(
    width: usize,
    height: usize,
    base_w: f32,
    high_w_cells: &[(usize, usize)],
    high_w: f32,
) -> Vec<f32> {
    let mut w = vec![base_w; width * height];
    for &(x, y) in high_w_cells {
        w[cell_index(x, y, width)] = high_w;
    }
    w
}

/// Set explicit cells to a given impedance (including `INF` for blocked terrain).
pub fn set_w_cells(w: &mut [f32], width: usize, cells: &[(usize, usize)], value: f32) {
    for &(x, y) in cells {
        w[cell_index(x, y, width)] = value;
    }
}

/// Build a `MinPlusStencilConfig` for flat-grid driver tests.
pub fn test_config(
    width: u32,
    height: u32,
    dest: (u32, u32),
) -> simthing_gpu::MinPlusStencilConfig {
    simthing_gpu::MinPlusStencilConfig {
        width,
        height,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x: dest.0,
        dest_y: dest.1,
        inf_sentinel: INF,
    }
}

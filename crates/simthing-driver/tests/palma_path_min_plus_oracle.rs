//! PALMA-PATH-1R + PATH-2 — CPU oracle and GPU parity for min-plus Location fields.
//!
//! Proves numeric `D` relaxation over numeric `W`:
//! - uniform Manhattan-style baseline;
//! - full-row blockade raises cost (not “bends around”);
//! - partial wall with gap bends scalar field through lower-cost corridor;
//! - clearing `W` lowers `D`;
//! - `INF` blocked cells stay unreachable;
//! - scalar field only — no route object, no pathfinding engine.

mod support;

use simthing_gpu::{
    cpu_min_plus_d_from_w, cpu_min_plus_relaxation, extract_d_flat, max_d_field_error,
    pack_w_and_initial_d, GpuContext, MinPlusStencilOp,
};
use std::sync::Mutex;

use support::palma_min_plus_oracle::{
    cell_index, run_min_plus_relaxation, set_w_cells, terran_pirate_grid_w_field, test_config, INF,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PALMA min-plus GPU tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

const SMALL_WIDTH: usize = 5;
const SMALL_HEIGHT: usize = 5;
const SMALL_ITERATIONS: u32 = 32;

const DEST_SMALL: (u32, u32) = (0, 0);
const CONVOY_SMALL: (usize, usize) = (4, 4);

/// Full middle row — raises traversal cost but spans the grid (no detour around the row itself).
const FULL_ROW_BLOCKADE: &[(usize, usize)] = &[(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)];

const DETOUR_WIDTH: u32 = 8;
const DETOUR_HEIGHT: u32 = 8;
const DETOUR_ITERATIONS: u32 = 64;
const DEST_DETOUR: (u32, u32) = (0, 0);
const QUERY_DETOUR: (usize, usize) = (7, 7);
const GAP_CELL: (usize, usize) = (4, 4);

/// Partial vertical wall with a single gap, plus blocked top/bottom bypass rows so crossing
/// x = 4 must use the gap corridor (numeric W only).
fn detour_wall_w_field(gap_open: bool) -> Vec<f32> {
    let width = DETOUR_WIDTH as usize;
    let height = DETOUR_HEIGHT as usize;
    let mut w = vec![1.0f32; width * height];

    // Vertical wall at x = 4 with gap at (4, 4).
    for y in 1..=6 {
        if (4, y) != GAP_CELL {
            w[cell_index(4, y, width)] = 100.0;
        }
    }
    if !gap_open {
        w[cell_index(GAP_CELL.0, GAP_CELL.1, width)] = 100.0;
    }

    // Block eastward slide along y = 0 (except destination) and westward slide along y = 7
    // (except query) so the field cannot bypass the wall on the perimeter.
    for x in 1..width {
        w[cell_index(x, 0, width)] = 100.0;
    }
    for x in 0..width - 1 {
        w[cell_index(x, height - 1, width)] = 100.0;
    }

    w
}

fn small_config() -> simthing_gpu::MinPlusStencilConfig {
    test_config(SMALL_WIDTH as u32, SMALL_HEIGHT as u32, DEST_SMALL)
}

fn detour_config() -> simthing_gpu::MinPlusStencilConfig {
    test_config(DETOUR_WIDTH, DETOUR_HEIGHT, DEST_DETOUR)
}

fn assert_d_close(a: f32, b: f32, msg: &str) {
    if a.is_infinite() && b.is_infinite() {
        return;
    }
    assert!((a - b).abs() < 1e-4, "{msg}: a={a} b={b}");
}

fn gpu_d_matches_cpu(w: &[f32], config: &simthing_gpu::MinPlusStencilConfig, iterations: u32) {
    with_gpu(|ctx| {
        let cpu_d = cpu_min_plus_d_from_w(w, config, iterations).expect("cpu oracle");
        let values = pack_w_and_initial_d(w, config).expect("pack");
        let op = MinPlusStencilOp::new(ctx, config.clone()).expect("gpu op");
        op.upload_values(ctx, &values).expect("upload");
        let gpu_values = op.run_ping_pong(ctx, iterations).expect("gpu run");
        let gpu_d = extract_d_flat(&gpu_values, config).expect("extract d");
        let err = max_d_field_error(&cpu_d, &gpu_d);
        assert!(
            err < 1e-4,
            "GPU/CPU D parity max_err={err} iterations={iterations}"
        );
    });
}

#[test]
fn palma_min_plus_partial_wall_gap_bends_scalar_d_field() {
    let config = detour_config();
    let w_open = detour_wall_w_field(true);
    let w_closed = detour_wall_w_field(false);

    let d_open = run_min_plus_relaxation(&w_open, &config, DETOUR_ITERATIONS).expect("open gap");
    let d_closed =
        run_min_plus_relaxation(&w_closed, &config, DETOUR_ITERATIONS).expect("closed gap");

    let width = DETOUR_WIDTH as usize;
    let query = cell_index(QUERY_DETOUR.0, QUERY_DETOUR.1, width);
    let gap = cell_index(GAP_CELL.0, GAP_CELL.1, width);
    let wall = cell_index(4, 3, width);

    assert!(
        d_open[query] + 1e-3 < d_closed[query],
        "open gap should yield lower query D: open={} closed={}",
        d_open[query],
        d_closed[query]
    );
    assert!(
        d_open[gap] + 20.0 < d_open[wall],
        "scalar field should prefer gap corridor over high-W wall cell: gap={} wall={}",
        d_open[gap],
        d_open[wall]
    );

    gpu_d_matches_cpu(&w_open, &config, DETOUR_ITERATIONS);
    gpu_d_matches_cpu(&w_closed, &config, DETOUR_ITERATIONS);
}

#[test]
fn palma_min_plus_clearing_blockade_lowers_d_field() {
    let config = small_config();
    let w_blocked =
        terran_pirate_grid_w_field(SMALL_WIDTH, SMALL_HEIGHT, 1.0, FULL_ROW_BLOCKADE, 100.0);
    let w_cleared = terran_pirate_grid_w_field(SMALL_WIDTH, SMALL_HEIGHT, 1.0, &[], 1.0);

    let d_before = run_min_plus_relaxation(&w_blocked, &config, SMALL_ITERATIONS).expect("before");
    let d_after = run_min_plus_relaxation(&w_cleared, &config, SMALL_ITERATIONS).expect("after");

    let query = cell_index(CONVOY_SMALL.0, CONVOY_SMALL.1, SMALL_WIDTH);
    assert!(
        d_after[query] + 1e-4 < d_before[query],
        "clearing W should lower D"
    );
    assert_d_close(d_after[query], 8.0, "after clear");

    gpu_d_matches_cpu(&w_cleared, &config, SMALL_ITERATIONS);
}

#[test]
fn palma_min_plus_inf_blocked_query_stays_unreachable() {
    let config = small_config();
    let mut w = terran_pirate_grid_w_field(SMALL_WIDTH, SMALL_HEIGHT, 1.0, &[], 1.0);
    // Cut column x=2 and southeast corner on row y=4 — isolates (4,4) from dest (0,0).
    set_w_cells(
        &mut w,
        SMALL_WIDTH,
        &[(2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (3, 4), (4, 4)],
        INF,
    );

    let d = run_min_plus_relaxation(&w, &config, SMALL_ITERATIONS).expect("relax");
    let query = cell_index(4, 4, SMALL_WIDTH);
    assert!(d[query].is_infinite(), "isolated query should stay INF");
    assert!(d[cell_index(0, 0, SMALL_WIDTH)].abs() < 1e-6);

    gpu_d_matches_cpu(&w, &config, SMALL_ITERATIONS);
}

#[test]
fn palma_min_plus_emits_scalar_field_only_not_route_object() {
    let config = small_config();
    let w = terran_pirate_grid_w_field(SMALL_WIDTH, SMALL_HEIGHT, 1.0, FULL_ROW_BLOCKADE, 50.0);
    let d = run_min_plus_relaxation(&w, &config, 8).expect("relax");

    assert_eq!(d.len(), SMALL_WIDTH * SMALL_HEIGHT);
    assert!(d.iter().all(|v| v.is_finite() || v.is_infinite()));
    assert!(d[cell_index(0, 0, SMALL_WIDTH)].abs() < 1e-6);

    let values = pack_w_and_initial_d(&w, &config).expect("pack");
    let final_values = cpu_min_plus_relaxation(&values, &config, 8).expect("values relax");
    assert_eq!(final_values.len(), values.len());
}

#[test]
fn palma_min_plus_gpu_matches_cpu_on_uniform_grid() {
    let config = small_config();
    let w = terran_pirate_grid_w_field(SMALL_WIDTH, SMALL_HEIGHT, 1.0, &[], 1.0);
    gpu_d_matches_cpu(&w, &config, SMALL_ITERATIONS);
}

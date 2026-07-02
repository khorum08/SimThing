//! PALMA-PATH-4 — benchmark CPU per-mover Dijkstra vs CPU/GPU min-plus Location fields.
//!
//! Honest stowaway-heatmap hypothesis test. No pathfinding engine, movement policy, or route object.

mod support;

use std::sync::Mutex;

use simthing_gpu::GpuContext;

use support::palma_min_plus_oracle::test_config;
use support::palma_path_4_benchmark::{
    ci_smoke_matrix, format_row, representative_matrix, run_bench_row,
    verify_dijkstra_matches_min_plus, ImpedanceScenario, DEST,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PALMA PATH-4 GPU timings");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

#[test]
fn palma_path_4_cpu_per_mover_wins_single_near_dest_query_on_large_grid() {
    // Honest counterexample: one mover one cell from dest — Dijkstra early-exits while a full
    // 128×128×8 min-plus pass still touches the whole grid.
    let grid = 128_u32;
    let width = grid as usize;
    let config = support::palma_min_plus_oracle::test_config(grid, grid, DEST);
    let w =
        support::palma_path_4_benchmark::build_w_field(ImpedanceScenario::Uniform, width, width);
    let dest = (DEST.0 as usize, DEST.1 as usize);
    let near = (1_usize, 0);
    let movers = [near];
    let repeats = 5_u32;

    let per_query = support::palma_path_4_benchmark::time_cpu_per_mover_queries(
        &w, width, width, &movers, dest, repeats,
    );
    let field = support::palma_path_4_benchmark::time_cpu_field_update(&w, &config, 8, repeats);

    assert!(
        field > per_query * 2.0,
        "single near-dest query should beat full-grid field update: query={per_query:.1}us field={field:.1}us"
    );
}

/// Full benchmark matrix — run locally with `PALMA_PATH_4_BENCH=1`.
/// Stellaris-scale benchmark — run with `PALMA_PATH_4_BENCH=1`.

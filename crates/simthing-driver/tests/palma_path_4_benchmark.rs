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
fn palma_path_4_dijkstra_baseline_matches_min_plus_when_relaxed_enough() {
    let config = test_config(8, 8, DEST);
    let w = support::palma_path_4_benchmark::build_w_field(ImpedanceScenario::Mixed, 8, 8);
    verify_dijkstra_matches_min_plus(&w, &config, 64);
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

#[test]
fn palma_path_4_benchmark_smoke_matrix() {
    with_gpu(|ctx| {
        for (i, (grid, movers, churn, iterations, scenario)) in
            ci_smoke_matrix().into_iter().enumerate()
        {
            let include_gpu = i == 0 || (grid == 64 && movers == 100);
            let row = run_bench_row(
                grid,
                movers,
                churn,
                iterations,
                scenario,
                Some(ctx),
                include_gpu,
            );
            assert!(row.cpu_per_mover_us.is_finite() && row.cpu_per_mover_us > 0.0);
            assert!(row.cpu_field_us.is_finite());
            eprintln!("PALMA-PATH-4 {}", format_row(&row));
        }
    });
}

/// Full benchmark matrix — run locally with `PALMA_PATH_4_BENCH=1`.
#[test]
#[ignore = "full PALMA-PATH-4 matrix; set PALMA_PATH_4_BENCH=1"]
fn palma_path_4_benchmark_full_matrix() {
    if std::env::var("PALMA_PATH_4_BENCH").ok().as_deref() != Some("1") {
        eprintln!("skip palma_path_4_benchmark_full_matrix (set PALMA_PATH_4_BENCH=1)");
        return;
    }

    with_gpu(|ctx| {
        eprintln!("grid | movers | churn | iters | scenario | cpu_mover_us | cpu_field_us | cpu_sample_us | gpu_setup_us | gpu_cold_us | gpu_warm_us | break_even");
        eprintln!("---|---|---|---|---|---|---|---|---|---|---|---");
        for (grid, movers, churn, iterations, scenario) in representative_matrix() {
            let include_gpu = grid <= 128 && movers <= 1_000;
            let row = run_bench_row(
                grid,
                movers,
                churn,
                iterations,
                scenario,
                Some(ctx),
                include_gpu,
            );
            eprintln!("{}", format_row(&row));
        }
    });
}

#[test]
fn palma_path_4s_scenario_has_100_stars_and_150_fleets() {
    use support::palma_path_4_stellaris_scale::{
        build_scenario, fleets_with_unique_destinations, reduce_pressure_and_compose_w, BENCH_SEED,
        FACTION_COUNT, FLEETS_PER_FACTION, GRID, STAR_COUNT, TOTAL_FLEETS,
    };

    let scenario = build_scenario(BENCH_SEED);
    assert_eq!(scenario.stars.len(), STAR_COUNT);
    assert_eq!(scenario.fleets.len(), TOTAL_FLEETS);
    assert_eq!(
        scenario
            .stars
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len(),
        STAR_COUNT
    );

    let mut faction_counts = [0_usize; FACTION_COUNT];
    for fleet in &scenario.fleets {
        assert!(fleet.faction < FACTION_COUNT as u8);
        faction_counts[fleet.faction as usize] += 1;
        assert!(fleet.pos.0 < GRID as usize && fleet.pos.1 < GRID as usize);
        assert!(fleet.dest.0 < GRID as usize && fleet.dest.1 < GRID as usize);
    }
    assert_eq!(faction_counts[0], FLEETS_PER_FACTION);
    assert_eq!(faction_counts[1], FLEETS_PER_FACTION);
    assert!(scenario.distinct_destinations >= 10);

    let (w, pressure_us) = reduce_pressure_and_compose_w(&scenario, 5, BENCH_SEED);
    assert_eq!(w.len(), (GRID * GRID) as usize);
    assert!(pressure_us.is_finite() && pressure_us > 0.0);
    assert!(w.iter().all(|v| v.is_finite() && *v >= 1.0));

    let unique = fleets_with_unique_destinations(&scenario);
    assert_eq!(
        unique
            .iter()
            .map(|f| f.dest)
            .collect::<std::collections::HashSet<_>>()
            .len(),
        TOTAL_FLEETS
    );
}

/// Stellaris-scale benchmark — run with `PALMA_PATH_4_BENCH=1`.
#[test]
#[ignore = "180×180 150-fleet Stellaris-scale matrix; set PALMA_PATH_4_BENCH=1"]
fn palma_path_4s_stellaris_scale_benchmark() {
    if std::env::var("PALMA_PATH_4_BENCH").ok().as_deref() != Some("1") {
        eprintln!("skip palma_path_4s_stellaris_scale_benchmark (set PALMA_PATH_4_BENCH=1)");
        return;
    }

    use support::palma_path_4_stellaris_scale::{
        build_scenario, format_stellaris_row, run_stellaris_row, stellaris_churn_matrix,
        stellaris_iteration_matrix, BENCH_SEED, GRID,
    };

    let scenario = build_scenario(BENCH_SEED);
    eprintln!(
        "PALMA-PATH-4S grid={} stars={} fleets={} distinct_dests={}",
        GRID,
        scenario.stars.len(),
        scenario.fleets.len(),
        scenario.distinct_destinations
    );
    eprintln!("PALMA-PATH-4S strategy metrics (one line per churn/iter row):");

    with_gpu(|ctx| {
        for churn in stellaris_churn_matrix() {
            for iter in stellaris_iteration_matrix() {
                let include_gpu = churn == 0 && (iter == 4 || iter == 8);
                let row = run_stellaris_row(&scenario, churn, iter, Some(ctx), include_gpu);
                eprintln!("{}", format_stellaris_row(&row));
            }
        }
    });
}

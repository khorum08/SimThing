//! PALMA-PATH-4 — honest CPU per-mover vs CPU/GPU min-plus field benchmark helpers.
//!
//! Test-local only. No production pathfinding engine, route object, or movement policy.

use simthing_gpu::{
    cpu_min_plus_d_from_w, pack_w_and_initial_d, GpuContext, MinPlusStencilConfig, MinPlusStencilOp,
};

use super::palma_min_plus_oracle::{cell_index, test_config, INF};
use super::palma_terran_pirate_fixture::{sample_lowest_d_neighbor, GridCoord};

pub const DEST: (u32, u32) = (0, 0);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImpedanceScenario {
    Uniform,
    BlockadeGap,
    PirateIsland,
    FuelGradient,
    Mixed,
}

impl ImpedanceScenario {
    pub fn label(self) -> &'static str {
        match self {
            Self::Uniform => "uniform",
            Self::BlockadeGap => "blockade_gap",
            Self::PirateIsland => "pirate_island",
            Self::FuelGradient => "fuel_gradient",
            Self::Mixed => "mixed",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Uniform,
            Self::BlockadeGap,
            Self::PirateIsland,
            Self::FuelGradient,
            Self::Mixed,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct BenchRow {
    pub grid: u32,
    pub movers: usize,
    pub churn_pct: u32,
    pub iterations: u32,
    pub scenario: ImpedanceScenario,
    pub cpu_per_mover_us: f64,
    pub cpu_field_us: f64,
    pub cpu_sample_us: f64,
    pub gpu_setup_us: Option<f64>,
    pub gpu_field_cold_us: Option<f64>,
    pub gpu_field_warm_us: Option<f64>,
    pub break_even_movers: Option<f64>,
    pub field_wins: bool,
}

fn lcg_next(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    *state
}

fn jitter_w(w: &mut [f32], width: usize, churn_pct: u32, seed: u64) {
    let _ = width;
    if churn_pct == 0 {
        return;
    }
    let mut state = seed;
    let cells = w.len();
    let target = (cells as u64 * churn_pct as u64 / 100).max(1) as usize;
    for _ in 0..target {
        let idx = (lcg_next(&mut state) as usize) % cells;
        let delta = ((lcg_next(&mut state) % 1000) as f32 / 100.0) + 0.5;
        w[idx] += delta;
    }
}

fn bump_pirate(w: &mut [f32], width: usize, height: usize, ax: usize, ay: usize, bump: f32) {
    let mut cells = vec![(ax, ay)];
    if ax > 0 {
        cells.push((ax - 1, ay));
    }
    if ax + 1 < width {
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

/// Build numeric `W` for benchmark grids (fixture names are comments only).
pub fn build_w_field(scenario: ImpedanceScenario, width: usize, height: usize) -> Vec<f32> {
    let mut w = vec![1.0f32; width * height];
    let mid_x = width / 2;
    let gap_y = height / 2;
    let pirate_x = (width * 5).max(1) / 8;
    let pirate_y = (height * 5).max(1) / 8;
    let threshold = width + height - 6;

    match scenario {
        ImpedanceScenario::Uniform => {}
        ImpedanceScenario::BlockadeGap => {
            for y in 1..height.saturating_sub(1) {
                if (mid_x, y) != (mid_x, gap_y) {
                    w[cell_index(mid_x, y, width)] = 100.0;
                }
            }
            for x in 1..width {
                w[cell_index(x, 0, width)] = 100.0;
            }
            for x in 0..width.saturating_sub(1) {
                w[cell_index(x, height - 1, width)] = 100.0;
            }
        }
        ImpedanceScenario::PirateIsland => {
            bump_pirate(&mut w, width, height, pirate_x, pirate_y, 40.0);
        }
        ImpedanceScenario::FuelGradient => {
            for y in 0..height {
                for x in 0..width {
                    if x + y >= threshold {
                        w[cell_index(x, y, width)] += 8.0;
                    }
                }
            }
        }
        ImpedanceScenario::Mixed => {
            for y in 1..height.saturating_sub(1) {
                if (mid_x, y) != (mid_x, gap_y) {
                    w[cell_index(mid_x, y, width)] = 100.0;
                }
            }
            for x in 1..width {
                w[cell_index(x, 0, width)] = 100.0;
            }
            for x in 0..width.saturating_sub(1) {
                w[cell_index(x, height - 1, width)] = 100.0;
            }
            bump_pirate(&mut w, width, height, pirate_x, pirate_y, 40.0);
            for y in 0..height {
                for x in 0..width {
                    if x + y >= threshold {
                        w[cell_index(x, y, width)] += 8.0;
                    }
                }
            }
        }
    }

    w
}

pub fn config_for_grid(grid: u32) -> MinPlusStencilConfig {
    test_config(grid, grid, DEST)
}

pub fn mover_positions(grid: u32, movers: usize, seed: u64) -> Vec<(usize, usize)> {
    let width = grid as usize;
    let height = grid as usize;
    let mut state = seed;
    let mut out = Vec::with_capacity(movers);
    for _ in 0..movers {
        let x = (lcg_next(&mut state) as usize) % width;
        let y = (lcg_next(&mut state) as usize) % height;
        out.push((x, y));
    }
    out
}

/// Test-local 4-neighbor Dijkstra from `start` to `dest` under cell-entry costs matching min-plus.
pub fn dijkstra_cell_entry_query(
    w: &[f32],
    width: usize,
    height: usize,
    dest: (usize, usize),
    start: (usize, usize),
) -> f32 {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    #[derive(Copy, Clone, PartialEq)]
    struct State {
        cost: f32,
        x: usize,
        y: usize,
    }
    impl Eq for State {}
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .partial_cmp(&self.cost)
                .unwrap_or(Ordering::Equal)
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    if start == dest {
        return 0.0;
    }

    let mut dist = vec![INF; width * height];
    let start_idx = cell_index(start.0, start.1, width);
    dist[start_idx] = w[start_idx];
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: dist[start_idx],
        x: start.0,
        y: start.1,
    });

    while let Some(State { cost, x, y }) = heap.pop() {
        let i = cell_index(x, y, width);
        if cost > dist[i] {
            continue;
        }
        if (x, y) == dest {
            return cost;
        }
        for (nx, ny) in n4(x, y, width, height) {
            if (nx, ny) == dest {
                return cost;
            }
            let ni = cell_index(nx, ny, width);
            let wn = w[ni];
            if !wn.is_finite() {
                continue;
            }
            let nc = cost + wn;
            if nc < dist[ni] {
                dist[ni] = nc;
                heap.push(State {
                    cost: nc,
                    x: nx,
                    y: ny,
                });
            }
        }
    }
    dist[cell_index(dest.0, dest.1, width)]
}

fn n4(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(4);
    if x > 0 {
        out.push((x - 1, y));
    }
    if x + 1 < width {
        out.push((x + 1, y));
    }
    if y > 0 {
        out.push((x, y - 1));
    }
    if y + 1 < height {
        out.push((x, y + 1));
    }
    out
}

pub fn time_cpu_per_mover_queries(
    w: &[f32],
    width: usize,
    height: usize,
    movers: &[(usize, usize)],
    dest: (usize, usize),
    repeats: u32,
) -> f64 {
    let start = std::time::Instant::now();
    for _ in 0..repeats {
        for &(x, y) in movers {
            let _ = dijkstra_cell_entry_query(w, width, height, dest, (x, y));
        }
    }
    start.elapsed().as_secs_f64() * 1_000_000.0 / (repeats as f64 * movers.len() as f64)
}

pub fn time_cpu_field_update(
    w: &[f32],
    config: &MinPlusStencilConfig,
    iterations: u32,
    repeats: u32,
) -> f64 {
    let start = std::time::Instant::now();
    for _ in 0..repeats {
        let _ = cpu_min_plus_d_from_w(w, config, iterations).expect("cpu field");
    }
    start.elapsed().as_secs_f64() * 1_000_000.0 / repeats as f64
}

pub fn time_cpu_field_sampling(
    d: &[f32],
    width: usize,
    height: usize,
    movers: &[(usize, usize)],
    repeats: u32,
) -> f64 {
    let start = std::time::Instant::now();
    for _ in 0..repeats {
        for &(x, y) in movers {
            let _ = sample_lowest_d_neighbor(d, width, height, GridCoord { x, y });
        }
    }
    start.elapsed().as_secs_f64() * 1_000_000.0 / (repeats as f64 * movers.len() as f64)
}

pub fn time_gpu_field(
    ctx: &GpuContext,
    w: &[f32],
    config: &MinPlusStencilConfig,
    iterations: u32,
) -> (f64, f64, f64) {
    let values = pack_w_and_initial_d(w, config).expect("pack");

    let setup_start = std::time::Instant::now();
    let op = MinPlusStencilOp::new(ctx, config.clone()).expect("gpu op");
    let setup_us = setup_start.elapsed().as_secs_f64() * 1_000_000.0;

    let cold_start = std::time::Instant::now();
    op.upload_values(ctx, &values).expect("upload");
    op.run_ping_pong(ctx, iterations).expect("gpu run");
    let cold_us = cold_start.elapsed().as_secs_f64() * 1_000_000.0;

    let warm_start = std::time::Instant::now();
    op.upload_values(ctx, &values).expect("upload warm");
    op.run_ping_pong(ctx, iterations).expect("gpu warm");
    let warm_us = warm_start.elapsed().as_secs_f64() * 1_000_000.0;

    (setup_us, cold_us, warm_us)
}

pub fn estimate_break_even_movers(cpu_per_mover_us: f64, cpu_field_us: f64) -> Option<f64> {
    if cpu_per_mover_us <= 0.0 {
        return None;
    }
    Some(cpu_field_us / cpu_per_mover_us)
}

pub fn run_bench_row(
    grid: u32,
    movers: usize,
    churn_pct: u32,
    iterations: u32,
    scenario: ImpedanceScenario,
    gpu_ctx: Option<&GpuContext>,
    include_gpu: bool,
) -> BenchRow {
    let width = grid as usize;
    let height = grid as usize;
    let mut w = build_w_field(scenario, width, height);
    let seed = grid as u64
        ^ (movers as u64).wrapping_mul(0x9E37)
        ^ (churn_pct as u64).wrapping_mul(0x85EB)
        ^ (iterations as u64).wrapping_mul(0xC2B2)
        ^ (scenario as u8 as u64).wrapping_mul(0x27D4);
    jitter_w(&mut w, width, churn_pct, seed);

    let config = config_for_grid(grid);
    let positions = mover_positions(grid, movers, seed);
    let dest = (DEST.0 as usize, DEST.1 as usize);

    let repeats = bench_repeats(grid, movers);
    let cpu_per_mover_us = time_cpu_per_mover_queries(&w, width, height, &positions, dest, repeats);
    let cpu_field_us = time_cpu_field_update(&w, &config, iterations, repeats);
    let d = cpu_min_plus_d_from_w(&w, &config, iterations).expect("d field");
    let cpu_sample_us = time_cpu_field_sampling(&d, width, height, &positions, repeats);

    let (gpu_setup_us, gpu_field_cold_us, gpu_field_warm_us) = if include_gpu {
        if let Some(ctx) = gpu_ctx {
            let (setup, cold, warm) = time_gpu_field(ctx, &w, &config, iterations);
            (Some(setup), Some(cold), Some(warm))
        } else {
            (None, None, None)
        }
    } else {
        (None, None, None)
    };

    let movers_f = movers as f64;
    let break_even_movers = estimate_break_even_movers(cpu_per_mover_us, cpu_field_us);
    let field_wins = cpu_field_us + cpu_sample_us * movers_f < cpu_per_mover_us * movers_f;

    BenchRow {
        grid,
        movers,
        churn_pct,
        iterations,
        scenario,
        cpu_per_mover_us,
        cpu_field_us,
        cpu_sample_us,
        gpu_setup_us,
        gpu_field_cold_us,
        gpu_field_warm_us,
        break_even_movers,
        field_wins,
    }
}

fn bench_repeats(grid: u32, movers: usize) -> u32 {
    if grid >= 128 || movers >= 10_000 {
        1
    } else if grid >= 64 || movers >= 1_000 {
        2
    } else {
        3
    }
}

pub fn representative_matrix() -> Vec<(u32, usize, u32, u32, ImpedanceScenario)> {
    let mut rows = Vec::new();
    for &movers in &[10_usize, 100, 1_000, 10_000] {
        for &churn in &[0_u32, 1, 5, 20] {
            for &iter in &[1_u32, 2, 4, 8] {
                rows.push((32, movers, churn, iter, ImpedanceScenario::Uniform));
            }
        }
    }
    for &movers in &[10, 100, 1_000] {
        for &churn in &[0_u32, 5, 20] {
            for &iter in &[4_u32, 8] {
                rows.push((64, movers, churn, iter, ImpedanceScenario::Uniform));
                rows.push((64, movers, churn, iter, ImpedanceScenario::Mixed));
            }
        }
    }
    for &movers in &[10, 100, 1_000] {
        for &churn in &[0_u32, 5] {
            rows.push((128, movers, churn, 4, ImpedanceScenario::Uniform));
            rows.push((128, movers, churn, 8, ImpedanceScenario::Mixed));
        }
    }
    for &scenario in ImpedanceScenario::all() {
        rows.push((64, 100, 5, 4, scenario));
    }
    rows.push((256, 10, 0, 8, ImpedanceScenario::Uniform));
    rows.push((256, 100, 0, 8, ImpedanceScenario::Uniform));
    rows.push((256, 10, 5, 8, ImpedanceScenario::Mixed));
    rows
}

pub fn ci_smoke_matrix() -> Vec<(u32, usize, u32, u32, ImpedanceScenario)> {
    vec![
        (32, 10, 0, 4, ImpedanceScenario::Uniform),
        (32, 100, 5, 4, ImpedanceScenario::Uniform),
        (64, 100, 0, 8, ImpedanceScenario::Uniform),
        (64, 1_000, 5, 8, ImpedanceScenario::Mixed),
        (128, 100, 0, 4, ImpedanceScenario::Uniform),
    ]
}

pub fn format_row(row: &BenchRow) -> String {
    format!(
        "| {} | {} | {}% | {} | {} | {:.1} | {:.1} | {:.3} | {} | {} | {} | {} |",
        row.grid,
        row.movers,
        row.churn_pct,
        row.iterations,
        row.scenario.label(),
        row.cpu_per_mover_us,
        row.cpu_field_us,
        row.cpu_sample_us,
        fmt_opt(row.gpu_setup_us),
        fmt_opt(row.gpu_field_cold_us),
        fmt_opt(row.gpu_field_warm_us),
        row.break_even_movers
            .map(|v| {
                if v < 1.0 {
                    "<1".to_string()
                } else {
                    format!("{v:.0}")
                }
            })
            .unwrap_or_else(|| "-".into()),
    )
}

fn fmt_opt(v: Option<f64>) -> String {
    v.map(|x| format!("{x:.1}")).unwrap_or_else(|| "-".into())
}

pub fn verify_dijkstra_matches_min_plus(w: &[f32], config: &MinPlusStencilConfig, iterations: u32) {
    let width = config.width as usize;
    let height = config.height as usize;
    let d = cpu_min_plus_d_from_w(w, config, iterations).expect("min-plus");
    let dest = (config.dest_x as usize, config.dest_y as usize);
    for y in 0..height {
        for x in 0..width {
            if (x, y) == dest {
                continue;
            }
            let dijk = dijkstra_cell_entry_query(w, width, height, dest, (x, y));
            let relaxed = d[cell_index(x, y, width)];
            if relaxed.is_infinite() && dijk.is_infinite() {
                continue;
            }
            assert!(
                (dijk - relaxed).abs() < 0.05,
                "dijkstra vs min-plus at ({x},{y}): dijk={dijk} mp={relaxed} iters={iterations}"
            );
        }
    }
}

//! PALMA-PATH-4S — Stellaris-scale representative fleet movement field benchmark.
//!
//! 180×180 grid, 2 factions × 75 fleets, 100 spaced stars. Numeric W/D only — benchmark labels
//! for faction/fleet/star/SEAD; min-plus/GPU see scalar fields only.

use std::collections::{HashMap, HashSet};

use simthing_gpu::{
    cpu_min_plus_d_from_w, pack_w_and_initial_d, GpuContext, MinPlusStencilConfig, MinPlusStencilOp,
};

use super::palma_min_plus_oracle::{cell_index, test_config};
use super::palma_path_4_benchmark::dijkstra_cell_entry_query;
use super::palma_terran_pirate_fixture::{sample_lowest_d_neighbor, GridCoord};

pub const GRID: u32 = 180;
pub const STAR_COUNT: usize = 100;
pub const FACTION_COUNT: usize = 2;
pub const FLEETS_PER_FACTION: usize = 75;
pub const TOTAL_FLEETS: usize = FACTION_COUNT * FLEETS_PER_FACTION;
pub const BENCH_SEED: u64 = 0x5041_4C4D_4153_0004;

const HOSTILE_PRESSURE: f32 = 12.0;
const FRIENDLY_CONGESTION: f32 = 4.0;
const BLOCKADE_W: f32 = 80.0;
const FUEL_W: f32 = 6.0;
const STAR_TRAFFIC: f32 = 2.0;
const W_MAX: f32 = 120.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FleetSlot {
    pub faction: u8,
    pub pos: (usize, usize),
    pub dest: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct StellarisScaleScenario {
    pub stars: Vec<(usize, usize)>,
    pub fleets: Vec<FleetSlot>,
    pub faction_rally: [(usize, usize); FACTION_COUNT],
    pub distinct_destinations: usize,
}

#[derive(Clone, Debug)]
pub struct StellarisScaleRow {
    pub churn_pct: u32,
    pub iterations: u32,
    pub distinct_dests: usize,
    pub pressure_reduction_us: f64,
    pub cpu_per_fleet_total_us: f64,
    pub cpu_per_fleet_avg_us: f64,
    pub cpu_per_dest_fields_us: f64,
    pub cpu_faction_fields_us: f64,
    pub cpu_unique_dest_fields_us: f64,
    pub cpu_sample_total_us: f64,
    pub cpu_faction_sample_us: f64,
    pub gpu_setup_us: Option<f64>,
    pub gpu_cold_dispatch_us: Option<f64>,
    pub gpu_warm_dispatch_us: Option<f64>,
    pub gpu_readback_us: Option<f64>,
    pub total_tick_with_pressure_us: f64,
    pub path_eval_pressure_already_paid_us: f64,
}

fn lcg_next(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    *state
}

/// Stratified 10×10 regions on 180×180 — one star per region, deterministic jitter.
pub fn generate_stars(width: usize, height: usize, seed: u64) -> Vec<(usize, usize)> {
    let regions = 10_usize;
    let mut state = seed;
    let mut stars = Vec::with_capacity(STAR_COUNT);
    let mut used = HashSet::new();
    for ry in 0..regions {
        for rx in 0..regions {
            let cell_w = width / regions;
            let cell_h = height / regions;
            let base_x = rx * cell_w + cell_w / 2;
            let base_y = ry * cell_h + cell_h / 2;
            let jx = (lcg_next(&mut state) as usize) % cell_w.max(1);
            let jy = (lcg_next(&mut state) as usize) % cell_h.max(1);
            let x = (base_x + jx).min(width - 1);
            let y = (base_y + jy).min(height - 1);
            assert!(used.insert((x, y)), "duplicate star at ({x},{y})");
            stars.push((x, y));
        }
    }
    assert_eq!(stars.len(), STAR_COUNT);
    stars
}

pub fn build_scenario(seed: u64) -> StellarisScaleScenario {
    let width = GRID as usize;
    let height = GRID as usize;
    let stars = generate_stars(width, height, seed);
    let mut state = seed.wrapping_add(0xF1EE_0000);

    let stars_per_faction = STAR_COUNT / FACTION_COUNT;
    let faction_rally = [
        stars[stars_per_faction / 2],
        stars[stars_per_faction + stars_per_faction / 2],
    ];

    let mut fleets = Vec::with_capacity(TOTAL_FLEETS);
    for faction in 0..FACTION_COUNT {
        let owned_start = faction * stars_per_faction;
        let owned_end = owned_start + stars_per_faction;
        let hostile_start = if faction == 0 { stars_per_faction } else { 0 };
        let hostile_end = hostile_start + stars_per_faction;

        for i in 0..FLEETS_PER_FACTION {
            let home = stars[owned_start + (i % stars_per_faction)];
            let jx = (lcg_next(&mut state) as i32 % 7) - 3;
            let jy = (lcg_next(&mut state) as i32 % 7) - 3;
            let px = (home.0 as i32 + jx).clamp(0, width as i32 - 1) as usize;
            let py = (home.1 as i32 + jy).clamp(0, height as i32 - 1) as usize;

            let roll = lcg_next(&mut state) % 100;
            let dest = if roll < 50 {
                let idx = hostile_start + (lcg_next(&mut state) as usize % stars_per_faction);
                stars[idx]
            } else if roll < 80 {
                stars[owned_start + ((i + 3) % stars_per_faction)]
            } else {
                stars[lcg_next(&mut state) as usize % STAR_COUNT]
            };

            fleets.push(FleetSlot {
                faction: faction as u8,
                pos: (px, py),
                dest,
            });
        }
    }

    let distinct_destinations = fleets.iter().map(|f| f.dest).collect::<HashSet<_>>().len();
    StellarisScaleScenario {
        stars,
        fleets,
        faction_rally,
        distinct_destinations,
    }
}

fn bump_disk(pressure: &mut [f32], width: usize, cx: usize, cy: usize, bump: f32, radius: i32) {
    let height = pressure.len() / width;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let x = cx as i32 + dx;
            let y = cy as i32 + dy;
            if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
                continue;
            }
            let d = dx.abs() + dy.abs();
            if d > radius {
                continue;
            }
            let falloff = 1.0 - (d as f32 / (radius as f32 + 1.0));
            pressure[cell_index(x as usize, y as usize, width)] += bump * falloff;
        }
    }
}

/// Numeric pressure spread + W composition (SEAD/movement-front reduction stand-in).
pub fn reduce_pressure_and_compose_w(
    scenario: &StellarisScaleScenario,
    churn_pct: u32,
    seed: u64,
) -> (Vec<f32>, f64) {
    let width = GRID as usize;
    let height = GRID as usize;
    let cells = width * height;
    let start = std::time::Instant::now();

    let mut pressure = vec![0.0f32; cells];
    for fleet in &scenario.fleets {
        bump_disk(
            &mut pressure,
            width,
            fleet.pos.0,
            fleet.pos.1,
            HOSTILE_PRESSURE,
            3,
        );
    }
    for faction in 0..FACTION_COUNT {
        let mut centers = Vec::new();
        for f in &scenario.fleets {
            if f.faction == faction as u8 {
                centers.push(f.pos);
            }
        }
        for chunk in centers.chunks(5) {
            if let Some((cx, cy)) = chunk.first() {
                bump_disk(&mut pressure, width, *cx, *cy, FRIENDLY_CONGESTION, 2);
            }
        }
    }

    for _ in 0..2 {
        let prev = pressure.clone();
        for y in 0..height {
            for x in 0..width {
                let i = cell_index(x, y, width);
                let mut v = prev[i];
                if x > 0 {
                    v = v.max(prev[cell_index(x - 1, y, width)] * 0.85);
                }
                if x + 1 < width {
                    v = v.max(prev[cell_index(x + 1, y, width)] * 0.85);
                }
                if y > 0 {
                    v = v.max(prev[cell_index(x, y - 1, width)] * 0.85);
                }
                if y + 1 < height {
                    v = v.max(prev[cell_index(x, y + 1, width)] * 0.85);
                }
                pressure[i] = v * 0.92;
            }
        }
    }

    let mut w = vec![1.0f32; cells];
    for i in 0..cells {
        w[i] += pressure[i].min(50.0);
    }

    let mid = width / 2;
    for y in 0..height {
        for x in 0..width {
            if (x as i32 - mid as i32).abs() <= 1 && y > 20 && y < height - 20 && (y % 17 != 0) {
                w[cell_index(x, y, width)] = w[cell_index(x, y, width)].max(BLOCKADE_W);
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if x + y > width + height - 40 {
                w[cell_index(x, y, width)] += FUEL_W;
            }
        }
    }

    for &(sx, sy) in &scenario.stars {
        w[cell_index(sx, sy, width)] += STAR_TRAFFIC;
    }

    for v in &mut w {
        *v = v.clamp(1.0, W_MAX);
    }

    if churn_pct > 0 {
        let mut state = seed ^ churn_pct as u64;
        let target = (cells as u64 * churn_pct as u64 / 100).max(1) as usize;
        for _ in 0..target {
            let idx = (lcg_next(&mut state) as usize) % cells;
            w[idx] = (w[idx] + 0.5 + (lcg_next(&mut state) % 1000) as f32 / 200.0).min(W_MAX);
        }
    }

    let pressure_reduction_us = start.elapsed().as_secs_f64() * 1_000_000.0;
    (w, pressure_reduction_us)
}

fn config_for_dest(dest: (usize, usize)) -> MinPlusStencilConfig {
    test_config(GRID, GRID, (dest.0 as u32, dest.1 as u32))
}

pub fn time_all_fleet_dijkstra(w: &[f32], fleets: &[FleetSlot], repeats: u32) -> f64 {
    let width = GRID as usize;
    let start = std::time::Instant::now();
    for _ in 0..repeats {
        for fleet in fleets {
            let _ = dijkstra_cell_entry_query(w, width, width, fleet.dest, fleet.pos);
        }
    }
    start.elapsed().as_secs_f64() * 1_000_000.0 / repeats as f64
}

pub fn time_per_destination_fields(
    w: &[f32],
    fleets: &[FleetSlot],
    iterations: u32,
    repeats: u32,
) -> (f64, HashMap<(usize, usize), Vec<f32>>) {
    let dests: HashSet<_> = fleets.iter().map(|f| f.dest).collect();
    let start = std::time::Instant::now();
    let mut last_cache = HashMap::new();
    for _ in 0..repeats {
        let mut cache = HashMap::with_capacity(dests.len());
        for &dest in &dests {
            let config = config_for_dest(dest);
            let d = cpu_min_plus_d_from_w(w, &config, iterations).expect("cpu field");
            cache.insert(dest, d);
        }
        last_cache = cache;
    }
    let us = start.elapsed().as_secs_f64() * 1_000_000.0 / repeats as f64;
    (us, last_cache)
}

pub fn time_faction_objective_fields(
    w: &[f32],
    scenario: &StellarisScaleScenario,
    iterations: u32,
    repeats: u32,
) -> (f64, HashMap<u8, Vec<f32>>) {
    let start = std::time::Instant::now();
    let mut last = HashMap::new();
    for _ in 0..repeats {
        let mut cache = HashMap::new();
        for faction in 0..FACTION_COUNT {
            let rally = scenario.faction_rally[faction];
            let config = config_for_dest(rally);
            let d = cpu_min_plus_d_from_w(w, &config, iterations).expect("faction field");
            cache.insert(faction as u8, d);
        }
        last = cache;
    }
    let us = start.elapsed().as_secs_f64() * 1_000_000.0 / repeats as f64;
    (us, last)
}

pub fn fleets_with_unique_destinations(scenario: &StellarisScaleScenario) -> Vec<FleetSlot> {
    let width = GRID as usize;
    let mut out = Vec::with_capacity(TOTAL_FLEETS);
    for (i, fleet) in scenario.fleets.iter().enumerate() {
        let dest = if i < scenario.stars.len() {
            scenario.stars[i]
        } else {
            let x = (i * 17 + 3) % width;
            let y = (i * 23 + 7) % width;
            (x, y)
        };
        out.push(FleetSlot {
            faction: fleet.faction,
            pos: fleet.pos,
            dest,
        });
    }
    out
}

pub fn time_sample_fleets_on_fields(
    fleets: &[FleetSlot],
    fields: &HashMap<(usize, usize), Vec<f32>>,
    repeats: u32,
) -> f64 {
    let width = GRID as usize;
    let start = std::time::Instant::now();
    for _ in 0..repeats {
        for fleet in fleets {
            if let Some(d) = fields.get(&fleet.dest) {
                let _ = sample_lowest_d_neighbor(
                    d,
                    width,
                    width,
                    GridCoord {
                        x: fleet.pos.0,
                        y: fleet.pos.1,
                    },
                );
            }
        }
    }
    start.elapsed().as_secs_f64() * 1_000_000.0 / repeats as f64
}

pub fn time_sample_fleets_faction_fields(
    fleets: &[FleetSlot],
    fields: &HashMap<u8, Vec<f32>>,
    repeats: u32,
) -> f64 {
    let width = GRID as usize;
    let start = std::time::Instant::now();
    for _ in 0..repeats {
        for fleet in fleets {
            if let Some(d) = fields.get(&fleet.faction) {
                let _ = sample_lowest_d_neighbor(
                    d,
                    width,
                    width,
                    GridCoord {
                        x: fleet.pos.0,
                        y: fleet.pos.1,
                    },
                );
            }
        }
    }
    start.elapsed().as_secs_f64() * 1_000_000.0 / repeats as f64
}

pub fn time_gpu_primary_field_split(
    ctx: &GpuContext,
    w: &[f32],
    dest: (usize, usize),
    iterations: u32,
) -> (f64, f64, f64, f64) {
    let config = config_for_dest(dest);
    let values = pack_w_and_initial_d(w, &config).expect("pack");

    let setup_start = std::time::Instant::now();
    let op = MinPlusStencilOp::new(ctx, config).expect("gpu op");
    let setup_us = setup_start.elapsed().as_secs_f64() * 1_000_000.0;

    op.upload_values(ctx, &values).expect("upload");
    let cold_start = std::time::Instant::now();
    op.dispatch_ping_pong(ctx, iterations).expect("dispatch");
    let cold_dispatch_us = cold_start.elapsed().as_secs_f64() * 1_000_000.0;

    op.upload_values(ctx, &values).expect("upload warm");
    let warm_start = std::time::Instant::now();
    op.dispatch_ping_pong(ctx, iterations)
        .expect("warm dispatch");
    let warm_dispatch_us = warm_start.elapsed().as_secs_f64() * 1_000_000.0;

    let rb_start = std::time::Instant::now();
    let _ = op.readback_after_ping_pong(ctx, iterations);
    let readback_us = rb_start.elapsed().as_secs_f64() * 1_000_000.0;

    (setup_us, cold_dispatch_us, warm_dispatch_us, readback_us)
}

pub fn stellaris_churn_matrix() -> Vec<u32> {
    vec![0, 1, 5, 20]
}

pub fn stellaris_iteration_matrix() -> Vec<u32> {
    vec![1, 2, 4, 8]
}

pub fn run_stellaris_row(
    scenario: &StellarisScaleScenario,
    churn_pct: u32,
    iterations: u32,
    gpu_ctx: Option<&GpuContext>,
    include_gpu: bool,
) -> StellarisScaleRow {
    let seed = BENCH_SEED ^ (churn_pct as u64).wrapping_mul(0x85EB) ^ (iterations as u64);
    let (w, pressure_reduction_us) = reduce_pressure_and_compose_w(scenario, churn_pct, seed);

    let repeats = 1_u32;
    let cpu_per_fleet_total_us = time_all_fleet_dijkstra(&w, &scenario.fleets, repeats);
    let cpu_per_fleet_avg_us = cpu_per_fleet_total_us / TOTAL_FLEETS as f64;

    let (cpu_per_dest_fields_us, dest_fields) =
        time_per_destination_fields(&w, &scenario.fleets, iterations, repeats);
    let cpu_sample_total_us = time_sample_fleets_on_fields(&scenario.fleets, &dest_fields, repeats);

    let (cpu_faction_fields_us, faction_fields) =
        time_faction_objective_fields(&w, scenario, iterations, repeats);
    let cpu_faction_sample_us =
        time_sample_fleets_faction_fields(&scenario.fleets, &faction_fields, repeats);

    let unique_fleets = fleets_with_unique_destinations(scenario);
    let (cpu_unique_dest_fields_us, _) =
        time_per_destination_fields(&w, &unique_fleets, iterations, repeats);

    let (gpu_setup_us, gpu_cold_dispatch_us, gpu_warm_dispatch_us, gpu_readback_us) = if include_gpu
    {
        if let Some(ctx) = gpu_ctx {
            let primary_dest = scenario.faction_rally[0];
            let (s, c, w, r) = time_gpu_primary_field_split(ctx, &w, primary_dest, iterations);
            (Some(s), Some(c), Some(w), Some(r))
        } else {
            (None, None, None, None)
        }
    } else {
        (None, None, None, None)
    };

    let path_eval_pressure_already_paid_us =
        cpu_per_dest_fields_us + cpu_sample_total_us + gpu_warm_dispatch_us.unwrap_or(0.0);
    let total_tick_with_pressure_us =
        pressure_reduction_us + cpu_per_fleet_total_us.max(path_eval_pressure_already_paid_us);

    StellarisScaleRow {
        churn_pct,
        iterations,
        distinct_dests: scenario.distinct_destinations,
        pressure_reduction_us,
        cpu_per_fleet_total_us,
        cpu_per_fleet_avg_us,
        cpu_per_dest_fields_us,
        cpu_faction_fields_us,
        cpu_unique_dest_fields_us,
        cpu_sample_total_us,
        cpu_faction_sample_us,
        gpu_setup_us,
        gpu_cold_dispatch_us,
        gpu_warm_dispatch_us,
        gpu_readback_us,
        total_tick_with_pressure_us,
        path_eval_pressure_already_paid_us,
    }
}

pub fn format_stellaris_row(row: &StellarisScaleRow) -> String {
    format!(
        "| {}% | {} | {} | {:.0} | {:.0} | {:.1} | {:.0} | {:.0} | {:.0} | {:.0} | {} | {} | {} | {} | {:.0} | {:.0} |",
        row.churn_pct,
        row.iterations,
        row.distinct_dests,
        row.pressure_reduction_us,
        row.cpu_per_fleet_total_us,
        row.cpu_per_fleet_avg_us,
        row.cpu_per_dest_fields_us,
        row.cpu_faction_fields_us,
        row.cpu_unique_dest_fields_us,
        row.cpu_sample_total_us,
        fmt_opt(row.gpu_setup_us),
        fmt_opt(row.gpu_cold_dispatch_us),
        fmt_opt(row.gpu_warm_dispatch_us),
        fmt_opt(row.gpu_readback_us),
        row.total_tick_with_pressure_us,
        row.path_eval_pressure_already_paid_us,
    )
}

fn fmt_opt(v: Option<f64>) -> String {
    v.map(|x| format!("{x:.0}")).unwrap_or_else(|| "-".into())
}

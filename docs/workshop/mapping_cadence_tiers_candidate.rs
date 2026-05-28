//! Mapping optimization toolkit probe helpers (sandbox only).

use simthing_gpu::{
    cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
};
use std::time::Instant;

pub const N_DIMS: u32 = 4;
pub const SOURCE_COL: u32 = 0;
pub const TARGET_COL: u32 = 0;
pub const HORIZON: u32 = 8;
pub const TILE: u32 = 10;
pub const GUTTER: u32 = 1;
pub const SOURCE_CAP: f32 = 500.0;

pub const CLUSTER: [(u32, u32, f32); 4] = [(0, 0, 80.0), (0, 1, 60.0), (1, 0, 60.0), (1, 1, 40.0)];

pub fn idx(slot: u32, col: u32) -> usize {
    (slot * N_DIMS + col) as usize
}

pub fn slot_xy(x: u32, y: u32, w: u32) -> u32 {
    y * w + x
}

pub fn get(v: &[f32], slot: u32, col: u32) -> f32 {
    v[idx(slot, col)]
}

pub fn baseline_config(w: u32, h: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: N_DIMS,
        source_col: SOURCE_COL,
        target_col: TARGET_COL,
        horizon: HORIZON,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: Some(SOURCE_CAP),
        operator: StructuredFieldStencilOperator::SourceCappedNormalized,
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

pub fn l1_norm(v: &[f32], w: u32, h: u32) -> f32 {
    let mut s = 0.0f32;
    for y in 0..h {
        for x in 0..w {
            s += get(v, slot_xy(x, y, w), TARGET_COL).abs();
        }
    }
    s
}

pub fn seed_cluster(values: &mut [f32], w: u32, base_slot_offset: u32, scale: f32) {
    for &(lx, ly, v) in &CLUSTER {
        let slot = base_slot_offset + slot_xy(lx, ly, w);
        values[idx(slot, SOURCE_COL)] = v * scale;
    }
}

pub fn clear_cluster_sources(values: &mut [f32], w: u32, base_slot_offset: u32) {
    for &(lx, ly, _) in &CLUSTER {
        let slot = base_slot_offset + slot_xy(lx, ly, w);
        values[idx(slot, SOURCE_COL)] = 0.0;
    }
}

pub fn corridor_t44(values: &[f32], w: u32, base_slot: u32) -> f32 {
    get(values, base_slot + slot_xy(4, 4, w), TARGET_COL)
}

pub fn run_one_shot_h8(
    ctx: &GpuContext,
    config: StructuredFieldStencilConfig,
    values: &[f32],
    w: u32,
    base_slot: u32,
) -> (Vec<f32>, u32, f64) {
    let op = StructuredFieldStencilOp::new(ctx, config).expect("stencil op");
    let t0 = Instant::now();
    op.upload_values(ctx, values).unwrap();
    let mut dispatches = op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
    let mut cur = op.readback_after_ping_pong(ctx, 1);
    clear_cluster_sources(&mut cur, w, base_slot);
    op.upload_values(ctx, &cur).unwrap();
    let (out, d2) = op.run_configured_horizon(ctx).unwrap();
    dispatches += d2;
    let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
    (out, dispatches, wall_ms)
}

pub fn standalone_region_reference(
    ctx: &GpuContext,
    region_scale: f32,
) -> (Vec<f32>, f64, u32) {
    let config = baseline_config(TILE, TILE);
    let mut values = vec![0.0f32; config.values_len()];
    seed_cluster(&mut values, TILE, 0, region_scale);
    let (out, dispatches, wall_ms) = run_one_shot_h8(ctx, config, &values, TILE, 0);
    (out, wall_ms, dispatches)
}

pub fn atlas_pitch() -> u32 {
    TILE + 2 * GUTTER
}

pub fn atlas_dims(region_count: u32) -> (u32, u32, u32, u32) {
    let side = (region_count as f64).sqrt().ceil() as u32;
    let pitch = atlas_pitch();
    (side * pitch, side * pitch, side, pitch)
}

pub fn tile_origin(tile_col: u32, tile_row: u32, pitch: u32) -> (u32, u32) {
    (
        tile_col * pitch + GUTTER,
        tile_row * pitch + GUTTER,
    )
}

pub fn build_atlas_values(region_count: u32) -> (Vec<f32>, u32, u32, u32) {
    let (aw, ah, side, pitch) = atlas_dims(region_count);
    let mut values = vec![0.0f32; (aw * ah * N_DIMS) as usize];
    for rid in 0..region_count {
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, pitch);
        let base = slot_xy(ox, oy, aw);
        let scale = 1.0 + (rid as f32 * 0.05);
        seed_cluster(&mut values, aw, base, scale);
    }
    (values, aw, ah, pitch)
}

pub fn read_tile_corridor(values: &[f32], aw: u32, pitch: u32, tile_col: u32, tile_row: u32) -> f32 {
    let (ox, oy) = tile_origin(tile_col, tile_row, pitch);
    let base = slot_xy(ox, oy, aw);
    corridor_t44(values, aw, base)
}

pub fn tile_local_t44(values: &[f32], w: u32) -> f32 {
    corridor_t44(values, w, 0)
}

pub fn cpu_reference(scale: f32) -> Vec<f32> {
    let config = baseline_config(TILE, TILE);
    let mut values = vec![0.0f32; config.values_len()];
    seed_cluster(&mut values, TILE, 0, scale);
    let params = params_from_config(&config);
    let mut cur = values.clone();
    cur = cpu_horizon(&cur, &params, 1);
    clear_cluster_sources(&mut cur, TILE, 0);
    cpu_horizon(&cur, &params, HORIZON)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CadenceTier {
    EveryTick,
    Every4,
    Every10,
    Every60,
    EventTriggered,
}

pub fn should_update(tick: u32, tier: CadenceTier, event_dirty: bool) -> bool {
    match tier {
        CadenceTier::EveryTick => true,
        CadenceTier::Every4 => tick % 4 == 0,
        CadenceTier::Every10 => tick % 10 == 0,
        CadenceTier::Every60 => tick % 60 == 0,
        CadenceTier::EventTriggered => event_dirty,
    }
}

#[derive(Clone, Debug, Default)]
pub struct MacroRegionMeta {
    pub dirty_source_present: bool,
    pub dirty_neighbor_present: bool,
    pub residual_present: bool,
    pub topology_generation: u32,
    pub operator_generation: u32,
    pub cadence_due: bool,
}

pub fn region_skippable(
    m: &MacroRegionMeta,
    prev_topology: u32,
    prev_operator: u32,
) -> bool {
    !m.dirty_source_present
        && !m.dirty_neighbor_present
        && !m.residual_present
        && m.topology_generation == prev_topology
        && m.operator_generation == prev_operator
        && !m.cadence_due
}

pub fn should_schedule(m: &MacroRegionMeta, prev_topology: u32, prev_operator: u32) -> bool {
    !region_skippable(m, prev_topology, prev_operator)
}

pub fn active_source_mask(w: u32, h: u32) -> Vec<u32> {
    let mut mask = vec![0u32; (w * h) as usize];
    for &(lx, ly, _) in &CLUSTER {
        mask[slot_xy(lx, ly, w) as usize] = 1;
    }
    mask
}

pub fn dilate_mask(mask: &[u32], w: u32, h: u32, hops: u32) -> Vec<u32> {
    let mut cur = mask.to_vec();
    for _ in 0..hops {
        let mut next = cur.clone();
        for y in 0..h {
            for x in 0..w {
                let i = slot_xy(x, y, w) as usize;
                if cur[i] != 0 {
                    continue;
                }
                let mut active = false;
                for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 && (nx as u32) < w && (ny as u32) < h {
                        if cur[slot_xy(nx as u32, ny as u32, w) as usize] != 0 {
                            active = true;
                            break;
                        }
                    }
                }
                if active {
                    next[i] = 1;
                }
            }
        }
        cur = next;
    }
    cur
}

pub fn mask_ratio(mask: &[u32]) -> f32 {
    let active = mask.iter().filter(|&&v| v != 0).count() as f32;
    active / mask.len() as f32
}

pub fn run_with_mask(
    ctx: &GpuContext,
    config: StructuredFieldStencilConfig,
    values: &[f32],
    mask: &[u32],
    w: u32,
) -> (Vec<f32>, f64) {
    let mut op = StructuredFieldStencilOp::new(ctx, config).expect("stencil op");
    op.set_mask_mode(ctx, StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo)
        .unwrap();
    op.upload_mask(ctx, mask).unwrap();
    let t0 = Instant::now();
    op.upload_values(ctx, values).unwrap();
    let mut cur = op.readback_after_ping_pong(ctx, 0);
    op.upload_values(ctx, values).unwrap();
    let _ = op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
    cur = op.readback_after_ping_pong(ctx, 1);
    clear_cluster_sources(&mut cur, w, 0);
    op.upload_values(ctx, &cur).unwrap();
    let (out, _) = op.run_configured_horizon(ctx).unwrap();
    let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
    (out, wall_ms)
}

pub fn max_field_error(a: &[f32], b: &[f32], w: u32, h: u32) -> f32 {
    let mut max_err = 0.0f32;
    for y in 0..h {
        for x in 0..w {
            let slot = slot_xy(x, y, w);
            max_err = max_err.max((get(a, slot, TARGET_COL) - get(b, slot, TARGET_COL)).abs());
        }
    }
    max_err
}

pub fn pack_dirty_regions(
    dirty_indices: &[u32],
    side: u32,
    pitch: u32,
    full_values: &[f32],
    full_w: u32,
) -> (Vec<f32>, u32, u32, u32) {
    let n = dirty_indices.len() as u32;
    let pack_side = (n as f64).sqrt().ceil() as u32;
    let aw = pack_side * pitch;
    let ah = pack_side * pitch;
    let mut packed = vec![0.0f32; (aw * ah * N_DIMS) as usize];
    for (pi, &rid) in dirty_indices.iter().enumerate() {
        let tc = rid % side;
        let tr = rid / side;
        let (sx, sy) = tile_origin(tc, tr, pitch);
        let (dx, dy) = tile_origin((pi as u32) % pack_side, (pi as u32) / pack_side, pitch);
        for ly in 0..TILE {
            for lx in 0..TILE {
                let src = slot_xy(sx + lx, sy + ly, full_w);
                let dst = slot_xy(dx + lx, dy + ly, aw);
                for c in 0..N_DIMS {
                    packed[idx(dst, c)] = full_values[idx(src, c)];
                }
            }
        }
    }
    (packed, aw, ah, pack_side)
}

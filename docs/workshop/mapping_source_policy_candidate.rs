//! Mapping optimization remedial probe helpers (sandbox only).

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

pub fn seed_cluster(values: &mut [f32], w: u32, base_slot: u32, scale: f32) {
    for &(lx, ly, v) in &CLUSTER {
        let slot = base_slot + slot_xy(lx, ly, w);
        values[idx(slot, SOURCE_COL)] = v * scale;
    }
}

pub fn clear_seed_cells_only(values: &mut [f32], w: u32, base_slot: u32) {
    for &(lx, ly, _) in &CLUSTER {
        let slot = base_slot + slot_xy(lx, ly, w);
        values[idx(slot, SOURCE_COL)] = 0.0;
    }
}

pub fn clear_entire_source_column(values: &mut [f32], w: u32, h: u32) {
    for y in 0..h {
        for x in 0..w {
            values[idx(slot_xy(x, y, w), SOURCE_COL)] = 0.0;
        }
    }
}

pub fn corridor_t44(values: &[f32], w: u32, base_slot: u32) -> f32 {
    get(values, base_slot + slot_xy(4, 4, w), TARGET_COL)
}

pub fn clear_all_atlas_seeds(
    values: &mut [f32],
    aw: u32,
    region_count: u32,
    side: u32,
    gutter: u32,
) {
    for rid in 0..region_count {
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, gutter);
        clear_seed_cells_only(values, aw, slot_xy(ox, oy, aw));
    }
}

pub fn run_one_shot_h8_atlas(
    ctx: &GpuContext,
    config: StructuredFieldStencilConfig,
    values: &[f32],
    aw: u32,
    region_count: u32,
    side: u32,
    gutter: u32,
) -> (Vec<f32>, u32, f64) {
    let op = StructuredFieldStencilOp::new(ctx, config).expect("stencil op");
    let t0 = Instant::now();
    op.upload_values(ctx, values).unwrap();
    let mut dispatches = op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
    let mut cur = op.readback_after_ping_pong(ctx, 1);
    clear_all_atlas_seeds(&mut cur, aw, region_count, side, gutter);
    op.upload_values(ctx, &cur).unwrap();
    let (out, d2) = op.run_configured_horizon(ctx).unwrap();
    dispatches += d2;
    (out, dispatches, t0.elapsed().as_secs_f64() * 1000.0)
}

pub fn run_one_shot_h8(
    ctx: &GpuContext,
    config: StructuredFieldStencilConfig,
    values: &[f32],
    w: u32,
    base_slot: u32,
    clear_seeds: bool,
) -> (Vec<f32>, u32, f64) {
    let op = StructuredFieldStencilOp::new(ctx, config).expect("stencil op");
    let t0 = Instant::now();
    op.upload_values(ctx, values).unwrap();
    let mut dispatches = op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
    let mut cur = op.readback_after_ping_pong(ctx, 1);
    if clear_seeds {
        clear_seed_cells_only(&mut cur, w, base_slot);
    }
    op.upload_values(ctx, &cur).unwrap();
    let (out, d2) = op.run_configured_horizon(ctx).unwrap();
    dispatches += d2;
    (out, dispatches, t0.elapsed().as_secs_f64() * 1000.0)
}

pub fn standalone_region(ctx: &GpuContext, scale: f32, clear_seeds: bool) -> (Vec<f32>, f64) {
    let config = baseline_config(TILE, TILE);
    let mut values = vec![0.0f32; config.values_len()];
    seed_cluster(&mut values, TILE, 0, scale);
    let (out, _, wall) = run_one_shot_h8(ctx, config, &values, TILE, 0, clear_seeds);
    (out, wall)
}

pub fn atlas_pitch(gutter: u32) -> u32 {
    TILE + 2 * gutter
}

pub fn atlas_dims(region_count: u32, gutter: u32) -> (u32, u32, u32) {
    let side = (region_count as f64).sqrt().ceil() as u32;
    let pitch = atlas_pitch(gutter);
    (side * pitch, side * pitch, side)
}

pub fn tile_origin(tile_col: u32, tile_row: u32, gutter: u32) -> (u32, u32) {
    let pitch = atlas_pitch(gutter);
    (tile_col * pitch + gutter, tile_row * pitch + gutter)
}

pub fn build_atlas(region_count: u32, gutter: u32) -> (Vec<f32>, u32, u32, u32) {
    let (aw, ah, side) = atlas_dims(region_count, gutter);
    let pitch = atlas_pitch(gutter);
    let mut values = vec![0.0f32; (aw * ah * N_DIMS) as usize];
    for rid in 0..region_count {
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, gutter);
        seed_cluster(&mut values, aw, slot_xy(ox, oy, aw), 1.0 + rid as f32 * 0.05);
    }
    (values, aw, ah, pitch)
}

pub fn max_tile_error(
    atlas: &[f32],
    stand: &[f32],
    aw: u32,
    _pitch: u32,
    tc: u32,
    tr: u32,
    gutter: u32,
) -> f32 {
    let (ox, oy) = tile_origin(tc, tr, gutter);
    let mut max_err = 0.0f32;
    for ly in 0..TILE {
        for lx in 0..TILE {
            let a = get(atlas, slot_xy(ox + lx, oy + ly, aw), TARGET_COL);
            let s = get(stand, slot_xy(lx, ly, TILE), TARGET_COL);
            max_err = max_err.max((a - s).abs());
        }
    }
    max_err
}

pub fn gutter_overhead_percent(gutter: u32) -> f64 {
    let pitch = atlas_pitch(gutter) as f64;
    let useful = (TILE * TILE) as f64;
    let atlas_cells = pitch * pitch;
    100.0 * (atlas_cells / useful - 1.0)
}

pub fn vram_multiplier(gutter: u32) -> f64 {
    let pitch = atlas_pitch(gutter) as f64;
    (pitch * pitch) / (TILE * TILE) as f64
}

pub fn atlas_isolation_sweep(
    ctx: &GpuContext,
    region_count: u32,
    gutter: u32,
) -> (f32, f32, bool, u32) {
    let (values, aw, ah, pitch) = build_atlas(region_count, gutter);
    let side = (region_count as f64).sqrt().ceil() as u32;
    let config = baseline_config(aw, ah);
    let (atlas_out, _, _) = run_one_shot_h8_atlas(ctx, config, &values, aw, region_count, side, gutter);
    let mut max_t44_err = 0.0f32;
    let mut max_full_err = 0.0f32;
    let mut leak = false;
    for rid in 0..region_count {
        let scale = 1.0 + rid as f32 * 0.05;
        let (stand, _) = standalone_region(ctx, scale, true);
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, gutter);
        let atlas_t44 = corridor_t44(&atlas_out, aw, slot_xy(ox, oy, aw));
        let stand_t44 = corridor_t44(&stand, TILE, 0);
        let t44_err = (atlas_t44 - stand_t44).abs();
        max_t44_err = max_t44_err.max(t44_err);
        let err = max_tile_error(&atlas_out, &stand, aw, pitch, tc, tr, gutter);
        max_full_err = max_full_err.max(err);
        if t44_err > 0.05 {
            leak = true;
        }
    }
    (max_t44_err, max_full_err, leak, aw * ah)
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
                for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0
                        && ny >= 0
                        && (nx as u32) < w
                        && (ny as u32) < h
                        && cur[slot_xy(nx as u32, ny as u32, w) as usize] != 0
                    {
                        next[i] = 1;
                        break;
                    }
                }
            }
        }
        cur = next;
    }
    cur
}

pub fn run_with_field_mask(
    ctx: &GpuContext,
    config: StructuredFieldStencilConfig,
    values: &[f32],
    mask: &[u32],
    w: u32,
    base: u32,
) -> (Vec<f32>, f64) {
    let mut op = StructuredFieldStencilOp::new(ctx, config).expect("op");
    op.set_mask_mode(ctx, StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo)
        .unwrap();
    op.upload_mask(ctx, mask).unwrap();
    let t0 = Instant::now();
    op.upload_values(ctx, values).unwrap();
    let _ = op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
    let mut cur = op.readback_after_ping_pong(ctx, 1);
    clear_seed_cells_only(&mut cur, w, base);
    op.upload_values(ctx, &cur).unwrap();
    let (out, _) = op.run_configured_horizon(ctx).unwrap();
    (out, t0.elapsed().as_secs_f64() * 1000.0)
}

/// CPU model: separate seed buffer injects only on step 0; then seed identity cleared via mask.
pub fn cpu_seed_buffer_model(values: &[f32], w: u32, horizon: u32) -> Vec<f32> {
    let config = baseline_config(w, w);
    let params = params_from_config(&config);
    let mut field = values.to_vec();
    field = cpu_horizon(&field, &params, 1);
    clear_seed_cells_only(&mut field, w, 0);
    field = cpu_horizon(&field, &params, horizon);
    field
}

/// CPU model: unsafe column-wide zero after step 0 (masks propagated values in source_col).
pub fn cpu_column_zero_after_step0(values: &[f32], w: u32, horizon: u32) -> Vec<f32> {
    let config = baseline_config(w, w);
    let params = params_from_config(&config);
    let mut field = values.to_vec();
    field = cpu_horizon(&field, &params, 1);
    clear_entire_source_column(&mut field, w, w);
    field = cpu_horizon(&field, &params, horizon);
    field
}

/// CPU model: source_mask clears only seed identity cells after step 0.
pub fn cpu_source_mask_model(values: &[f32], w: u32, horizon: u32) -> Vec<f32> {
    cpu_seed_buffer_model(values, w, horizon)
}

pub fn cpu_caller_managed_protocol(values: &[f32], w: u32, horizon: u32) -> Vec<f32> {
    cpu_seed_buffer_model(values, w, horizon)
}

pub fn active_source_mask(w: u32, h: u32) -> Vec<u32> {
    let mut mask = vec![0u32; (w * h) as usize];
    for &(lx, ly, _) in &CLUSTER {
        mask[slot_xy(lx, ly, w) as usize] = 1;
    }
    mask
}

pub fn mask_ratio(mask: &[u32]) -> f32 {
    mask.iter().filter(|&&v| v != 0).count() as f32 / mask.len() as f32
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
    gutter: u32,
    full_values: &[f32],
    full_w: u32,
) -> (Vec<f32>, u32, u32, u32) {
    let pitch = atlas_pitch(gutter);
    let n = dirty_indices.len() as u32;
    let pack_side = (n as f64).sqrt().ceil() as u32;
    let aw = pack_side * pitch;
    let ah = pack_side * pitch;
    let mut packed = vec![0.0f32; (aw * ah * N_DIMS) as usize];
    for (pi, &rid) in dirty_indices.iter().enumerate() {
        let tc = rid % side;
        let tr = rid / side;
        let (sx, sy) = tile_origin(tc, tr, gutter);
        let (dx, dy) = tile_origin((pi as u32) % pack_side, (pi as u32) / pack_side, gutter);
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
    (packed, aw, ah, pitch)
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

pub fn region_skippable(m: &MacroRegionMeta, prev_topology: u32, prev_operator: u32) -> bool {
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

pub fn source_max(values: &[f32], w: u32, h: u32) -> f32 {
    let mut max_v = 0.0f32;
    for y in 0..h {
        for x in 0..w {
            max_v = max_v.max(get(values, slot_xy(x, y, w), SOURCE_COL).abs());
        }
    }
    max_v
}

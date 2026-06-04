//! Generic semantic-free atlas tile-local mask dispatch (C-0 / M-4 fixture support).
//!
//! Provides the protocol-faithful CPU oracle and a single-atlas-dispatch GPU path for
//! homogeneous square tile batches. Not wired into production pass graph.

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor,
    PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::context::GpuContext;
use crate::structured_field_stencil::{
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy,
};

const WGSL_ATLAS_MASK: &str = include_str!("shaders/structured_field_stencil_atlas_mask.wgsl");

/// Default C-0 fixture constants (semantic-free).
pub const C0_DEFAULT_N_DIMS: u32 = 4;
pub const C0_DEFAULT_SOURCE_COL: u32 = 0;
pub const C0_DEFAULT_TARGET_COL: u32 = 0;
pub const C0_DEFAULT_ALPHA: f32 = 1.0;
pub const C0_DEFAULT_GAMMA: f32 = 0.8;
pub const C0_DEFAULT_SOURCE_CAP: f32 = 500.0;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct AtlasMaskParamsGpu {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub source_col: u32,
    pub target_col: u32,
    pub tile_size: u32,
    pub alpha_self_decay: f32,
    pub gamma_neighbor: f32,
    pub source_cap: f32,
    pub variant: u32,
    pub use_tile_local_mask: u32,
    pub renorm_valid_neighbors: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasIsolationMode {
    /// Per-tile standalone reference (not atlas acceptance oracle).
    Standalone,
    /// Flush-packed atlas without tile-local mask (diagnostic only).
    FlushUnmasked,
    /// Algebraic tile-local mask G=0 (preferred C-0 path).
    FlushTileLocalMask,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasNormalizeVariant {
    FixedDenominator,
    ValidNeighborRenorm,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasIsolationPolicy {
    AlgebraicTileLocalMaskG0,
    PhysicalGutterGteH,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C0AtlasFixtureShape {
    pub tile_count: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub horizon: u32,
    pub n_dims: u32,
}

impl C0AtlasFixtureShape {
    pub fn c0_default() -> Self {
        Self {
            tile_count: 4,
            tile_width: 8,
            tile_height: 8,
            horizon: 8,
            n_dims: C0_DEFAULT_N_DIMS,
        }
    }

    pub fn tile_size(&self) -> u32 {
        assert_eq!(
            self.tile_width, self.tile_height,
            "C-0 v1 requires homogeneous square tiles"
        );
        self.tile_width
    }
}

pub fn atlas_cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

pub fn atlas_slot_xy(x: u32, y: u32, width: u32) -> u32 {
    y * width + x
}

pub fn atlas_side(tile_count: u32) -> u32 {
    (tile_count as f64).sqrt().ceil() as u32
}

pub fn atlas_dims(tile_count: u32, tile_size: u32) -> (u32, u32) {
    let side = atlas_side(tile_count);
    (side * tile_size, side * tile_size)
}

pub fn tile_origin(tile_col: u32, tile_row: u32, tile_size: u32) -> (u32, u32) {
    (tile_col * tile_size, tile_row * tile_size)
}

/// VRAM multiplier for physical gutter packing: (tile_size + 2*gutter)² / tile_size².
pub fn vram_multiplier(tile_size: u32, gutter: u32) -> f64 {
    let pitch = tile_size + 2 * gutter;
    let useful = (tile_size * tile_size) as f64;
    (pitch * pitch) as f64 / useful
}

pub fn atlas_config(
    width: u32,
    height: u32,
    horizon: u32,
    n_dims: u32,
    source_capped: bool,
) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width,
        height,
        n_dims,
        source_col: C0_DEFAULT_SOURCE_COL,
        target_col: C0_DEFAULT_TARGET_COL,
        horizon,
        alpha_self: C0_DEFAULT_ALPHA,
        gamma_neighbor: C0_DEFAULT_GAMMA,
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: if source_capped {
            Some(C0_DEFAULT_SOURCE_CAP)
        } else {
            None
        },
        operator: if source_capped {
            StructuredFieldStencilOperator::SourceCappedNormalized
        } else {
            StructuredFieldStencilOperator::Normalized
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: horizon > 8,
    }
}

pub fn seed_cluster(values: &mut [f32], width: u32, origin_slot: u32, scale: f32, n_dims: u32) {
    let ox = origin_slot % width;
    let oy = origin_slot / width;
    for &(dx, dy, v) in &[(0u32, 0, 80.0f32), (1, 0, 60.0), (0, 1, 60.0), (1, 1, 40.0)] {
        if ox + dx < width && oy + dy < width {
            values[atlas_cell_index(
                atlas_slot_xy(ox + dx, oy + dy, width),
                C0_DEFAULT_SOURCE_COL,
                n_dims,
            )] = v * scale;
        }
    }
}

pub fn build_flush_atlas(tile_count: u32, tile_size: u32, n_dims: u32) -> (Vec<f32>, u32, u32) {
    let (aw, ah) = atlas_dims(tile_count, tile_size);
    let side = atlas_side(tile_count);
    let mut values = vec![0.0f32; (aw * ah * n_dims) as usize];
    for rid in 0..tile_count {
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, tile_size);
        let scale = 1.0 + rid as f32 * 0.03;
        seed_cluster(
            values.as_mut_slice(),
            aw,
            atlas_slot_xy(ox, oy, aw),
            scale,
            n_dims,
        );
    }
    (values, aw, ah)
}

pub fn clear_seed_cells_only(values: &mut [f32], width: u32, origin_slot: u32, n_dims: u32) {
    let ox = origin_slot % width;
    let oy = origin_slot / width;
    for dy in 0..2 {
        for dx in 0..2 {
            if ox + dx < width && oy + dy < width {
                values[atlas_cell_index(
                    atlas_slot_xy(ox + dx, oy + dy, width),
                    C0_DEFAULT_SOURCE_COL,
                    n_dims,
                )] = 0.0;
            }
        }
    }
}

fn neighbor_valid_atlas(
    gx: u32,
    gy: u32,
    dx: i32,
    dy: i32,
    width: u32,
    height: u32,
    tile_size: u32,
    mode: AtlasIsolationMode,
) -> bool {
    let ngx = gx as i32 + dx;
    let ngy = gy as i32 + dy;
    if ngx < 0 || ngy < 0 || ngx >= width as i32 || ngy >= height as i32 {
        return false;
    }
    if mode != AtlasIsolationMode::FlushTileLocalMask {
        return true;
    }
    let tile_x = gx / tile_size;
    let tile_y = gy / tile_size;
    let local_x = gx - tile_x * tile_size;
    let local_y = gy - tile_y * tile_size;
    let nlx = local_x as i32 + dx;
    let nly = local_y as i32 + dy;
    nlx >= 0 && nlx < tile_size as i32 && nly >= 0 && nly < tile_size as i32
}

fn sample_atlas(
    buf: &[f32],
    gx: u32,
    gy: u32,
    dx: i32,
    dy: i32,
    width: u32,
    height: u32,
    tile_size: u32,
    mode: AtlasIsolationMode,
    n_dims: u32,
) -> f32 {
    if !neighbor_valid_atlas(gx, gy, dx, dy, width, height, tile_size, mode) {
        return 0.0;
    }
    let ngx = (gx as i32 + dx) as u32;
    let ngy = (gy as i32 + dy) as u32;
    buf[atlas_cell_index(
        atlas_slot_xy(ngx, ngy, width),
        C0_DEFAULT_SOURCE_COL,
        n_dims,
    )]
}

/// One atlas-protocol stencil step (CPU oracle).
pub fn cpu_atlas_stencil_step(
    values: &[f32],
    config: &StructuredFieldStencilConfig,
    tile_size: u32,
    mode: AtlasIsolationMode,
    norm: AtlasNormalizeVariant,
) -> Vec<f32> {
    let mut out = values.to_vec();
    let w = config.width;
    let h = config.height;
    let tc = config.target_col;
    let n_dims = config.n_dims;

    for y in 0..h {
        for x in 0..w {
            let center = sample_atlas(values, x, y, 0, 0, w, h, tile_size, mode, n_dims);
            let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            let mut neighbor_sum = 0.0f32;
            let mut neighbor_count = 4.0f32;

            if norm == AtlasNormalizeVariant::ValidNeighborRenorm {
                neighbor_count = 0.0;
                for &(dx, dy) in &dirs {
                    if neighbor_valid_atlas(x, y, dx, dy, w, h, tile_size, mode) {
                        neighbor_count += 1.0;
                        neighbor_sum +=
                            sample_atlas(values, x, y, dx, dy, w, h, tile_size, mode, n_dims);
                    }
                }
            } else {
                for &(dx, dy) in &dirs {
                    neighbor_sum +=
                        sample_atlas(values, x, y, dx, dy, w, h, tile_size, mode, n_dims);
                }
            }

            let mut next = config.alpha_self * center;
            if neighbor_count > 0.0 {
                next += config.gamma_neighbor * (neighbor_sum / neighbor_count);
            }
            if matches!(
                config.operator,
                StructuredFieldStencilOperator::SourceCappedNormalized
            ) {
                if let Some(cap) = config.source_cap {
                    next = next.clamp(0.0, cap);
                }
            }
            out[atlas_cell_index(atlas_slot_xy(x, y, w), tc, n_dims)] = next;
        }
    }
    out
}

pub fn cpu_atlas_horizon(
    values: &[f32],
    config: &StructuredFieldStencilConfig,
    tile_size: u32,
    mode: AtlasIsolationMode,
    norm: AtlasNormalizeVariant,
    hops: u32,
) -> Vec<f32> {
    let mut cur = values.to_vec();
    for _ in 0..hops {
        cur = cpu_atlas_stencil_step(&cur, config, tile_size, mode, norm);
    }
    cur
}

/// Exact per-tile-protocol CPU oracle for atlas batching (caller-managed seed clear).
pub fn cpu_caller_managed_atlas_protocol(
    values: &[f32],
    config: &StructuredFieldStencilConfig,
    tile_size: u32,
    tile_count: u32,
    mode: AtlasIsolationMode,
    norm: AtlasNormalizeVariant,
) -> Vec<f32> {
    let side = atlas_side(tile_count);
    let width = config.width;
    let n_dims = config.n_dims;
    let mut cur = values.to_vec();
    cur = cpu_atlas_horizon(&cur, config, tile_size, mode, norm, 1);
    for rid in 0..tile_count {
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, tile_size);
        clear_seed_cells_only(&mut cur, width, atlas_slot_xy(ox, oy, width), n_dims);
    }
    if config.horizon > 1 {
        cur = cpu_atlas_horizon(&cur, config, tile_size, mode, norm, config.horizon);
    }
    cur
}

pub fn max_full_tile_error(
    got: &[f32],
    expected: &[f32],
    width: u32,
    tile_size: u32,
    tile_count: u32,
    n_dims: u32,
) -> f32 {
    let side = atlas_side(tile_count);
    let target_col = C0_DEFAULT_TARGET_COL;
    let mut max_err = 0.0f32;
    for rid in 0..tile_count {
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, tile_size);
        for ly in 0..tile_size {
            for lx in 0..tile_size {
                let a = got
                    [atlas_cell_index(atlas_slot_xy(ox + lx, oy + ly, width), target_col, n_dims)];
                let b = expected
                    [atlas_cell_index(atlas_slot_xy(ox + lx, oy + ly, width), target_col, n_dims)];
                max_err = max_err.max((a - b).abs());
            }
        }
    }
    max_err
}

pub fn full_tile_l_inf(
    got: &[f32],
    expected: &[f32],
    width: u32,
    tile_size: u32,
    tile_count: u32,
    n_dims: u32,
) -> f32 {
    max_full_tile_error(got, expected, width, tile_size, tile_count, n_dims)
}

pub fn corridor_t44_max_error(
    got: &[f32],
    expected: &[f32],
    width: u32,
    tile_size: u32,
    tile_count: u32,
    n_dims: u32,
) -> f32 {
    let side = atlas_side(tile_count);
    let (px, py) = (
        4u32.min(tile_size.saturating_sub(1)),
        4u32.min(tile_size.saturating_sub(1)),
    );
    let target_col = C0_DEFAULT_TARGET_COL;
    let mut max_err = 0.0f32;
    for rid in 0..tile_count {
        let tc = rid % side;
        let tr = rid / side;
        let (ox, oy) = tile_origin(tc, tr, tile_size);
        let a = got[atlas_cell_index(atlas_slot_xy(ox + px, oy + py, width), target_col, n_dims)];
        let b =
            expected[atlas_cell_index(atlas_slot_xy(ox + px, oy + py, width), target_col, n_dims)];
        max_err = max_err.max((a - b).abs());
    }
    max_err
}

pub fn make_atlas_mask_params(
    width: u32,
    height: u32,
    tile_size: u32,
    n_dims: u32,
    source_capped: bool,
    use_tile_local_mask: bool,
    renorm: AtlasNormalizeVariant,
) -> AtlasMaskParamsGpu {
    AtlasMaskParamsGpu {
        width,
        height,
        n_dims,
        source_col: C0_DEFAULT_SOURCE_COL,
        target_col: C0_DEFAULT_TARGET_COL,
        tile_size,
        alpha_self_decay: C0_DEFAULT_ALPHA,
        gamma_neighbor: C0_DEFAULT_GAMMA,
        source_cap: if source_capped {
            C0_DEFAULT_SOURCE_CAP
        } else {
            0.0
        },
        variant: if source_capped { 5 } else { 1 },
        use_tile_local_mask: u32::from(use_tile_local_mask),
        renorm_valid_neighbors: u32::from(renorm == AtlasNormalizeVariant::ValidNeighborRenorm),
        _pad0: 0,
        _pad1: 0,
    }
}

pub struct AtlasMaskGpuOp {
    params_buffer: Buffer,
    input_buffer: Buffer,
    output_buffer: Buffer,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    params: AtlasMaskParamsGpu,
}

impl AtlasMaskGpuOp {
    pub fn new(ctx: &GpuContext, params: AtlasMaskParamsGpu, values_len: usize) -> Self {
        let device = &ctx.device;
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("atlas_mask_c0"),
            source: ShaderSource::Wgsl(WGSL_ATLAS_MASK.into()),
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("atlas_mask_bgl"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("atlas_mask_pl"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("atlas_mask_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "atlas_mask_stencil_step",
            compilation_options: Default::default(),
            cache: None,
        });
        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("atlas_mask_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let input_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("atlas_mask_in"),
            size: (values_len * 4) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("atlas_mask_out"),
            size: (values_len * 4) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        Self {
            params_buffer,
            input_buffer,
            output_buffer,
            pipeline,
            bind_group_layout,
            params,
        }
    }

    fn bind_group(
        &self,
        device: &wgpu::Device,
        input: &Buffer,
        output: &Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("atlas_mask_bg"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: input.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: output.as_entire_binding(),
                },
            ],
        })
    }

    pub fn upload(&self, ctx: &GpuContext, values: &[f32]) {
        ctx.queue
            .write_buffer(&self.input_buffer, 0, bytemuck::cast_slice(values));
    }

    pub fn dispatch_once(&self, ctx: &GpuContext, input: &Buffer, output: &Buffer) {
        let bind_group = self.bind_group(&ctx.device, input, output);
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("atlas_mask_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("atlas_mask_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((self.params.width + 7) / 8, (self.params.height + 7) / 8, 1);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn readback(&self, ctx: &GpuContext, buf: &Buffer, len: usize) -> Vec<f32> {
        let staging = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("atlas_mask_readback"),
            size: (len * 4) as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("atlas_mask_readback_enc"),
            });
        encoder.copy_buffer_to_buffer(buf, 0, &staging, 0, (len * 4) as u64);
        ctx.queue.submit(Some(encoder.finish()));
        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        ctx.device.poll(wgpu::Maintain::Wait);
        let data = slice.get_mapped_range();
        bytemuck::cast_slice(&data).to_vec()
    }

    fn run_horizon_pingpong(&self, ctx: &GpuContext, hops: u32) -> (Vec<f32>, u32) {
        let len = (self.params.width * self.params.height * self.params.n_dims) as usize;
        let mut dispatches = 0u32;
        let mut use_input = true;
        for _ in 0..hops {
            if use_input {
                self.dispatch_once(ctx, &self.input_buffer, &self.output_buffer);
            } else {
                self.dispatch_once(ctx, &self.output_buffer, &self.input_buffer);
            }
            dispatches += 1;
            use_input = !use_input;
        }
        let out_buf = if hops % 2 == 1 {
            &self.output_buffer
        } else {
            &self.input_buffer
        };
        (self.readback(ctx, out_buf, len), dispatches)
    }

    /// GPU atlas caller-managed protocol: one packed atlas dispatch path (not per-tile fake).
    pub fn gpu_caller_managed_atlas_protocol(
        &self,
        ctx: &GpuContext,
        values: &[f32],
        tile_count: u32,
        tile_size: u32,
        horizon: u32,
        n_dims: u32,
    ) -> (Vec<f32>, u32) {
        let side = atlas_side(tile_count);
        let width = self.params.width;
        self.upload(ctx, values);
        self.dispatch_once(ctx, &self.input_buffer, &self.output_buffer);
        let mut cur = self.readback(
            ctx,
            &self.output_buffer,
            (width * self.params.height * n_dims) as usize,
        );
        for rid in 0..tile_count {
            let tc = rid % side;
            let tr = rid / side;
            let (ox, oy) = tile_origin(tc, tr, tile_size);
            clear_seed_cells_only(&mut cur, width, atlas_slot_xy(ox, oy, width), n_dims);
        }
        self.upload(ctx, &cur);
        let (out, dispatches) = self.run_horizon_pingpong(ctx, horizon);
        (out, dispatches + 1)
    }
}

pub fn fnv64_hash_f32(values: &[f32], seed: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in seed {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for v in values {
        hash ^= u64::from(v.to_bits());
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn combined_fingerprint_hex(protocol_hash: u64, gpu_hash: u64) -> String {
    format!("{:016x}", protocol_hash ^ gpu_hash.rotate_left(17))
}

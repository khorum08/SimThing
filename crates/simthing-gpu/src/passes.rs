//! Compute pipelines and dispatch wrappers for Passes 0/1/2.
//!
//! Each pass owns its shader module, bind group layout, pipeline layout, and
//! pipeline. Bind groups are created per-dispatch from the supplied
//! `WorldGpuState` (cheap; lets us reuse one `Pipelines` instance across
//! multiple `WorldGpuState`s if needed).
//!
//! Uniform buffer (`Params { delta_time, n_dims }`) is shared across passes
//! and rewritten on each dispatch with the current dt.

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor,
    BufferUsages, CommandEncoderDescriptor, ComputePass, ComputePassDescriptor, ComputePipeline,
    ComputePipelineDescriptor, PipelineLayoutDescriptor, ShaderModuleDescriptor,
    ShaderSource, ShaderStages,
};
use wgpu::util::DeviceExt;

use crate::context::GpuContext;
use crate::world_state::{ReduceParams, WorldGpuState};

const WORKGROUP_SIZE: u32 = 64;
const MAX_DISPATCH_X_GROUPS: u32 = 65_535;

fn dispatch_linear(pass: &mut ComputePass<'_>, total_invocations: u32) {
    if total_invocations == 0 {
        return;
    }

    let groups = total_invocations.div_ceil(WORKGROUP_SIZE);
    let x = groups.min(MAX_DISPATCH_X_GROUPS);
    let y = groups.div_ceil(MAX_DISPATCH_X_GROUPS);
    pass.dispatch_workgroups(x, y, 1);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct PassParams {
    delta_time: f32,
    n_dims:     u32,
    _pad0:      u32,
    _pad1:      u32,
}

pub struct Pipelines {
    uniform_buffer: Buffer,

    snapshot_layout:   BindGroupLayout,
    snapshot_pipeline: ComputePipeline,

    velocity_layout:   BindGroupLayout,
    velocity_pipeline: ComputePipeline,

    intensity_layout:   BindGroupLayout,
    intensity_pipeline: ComputePipeline,

    overlay_layout:   BindGroupLayout,
    overlay_pipeline: ComputePipeline,

    intent_layout:   BindGroupLayout,
    intent_pipeline: ComputePipeline,

    threshold_layout:   BindGroupLayout,
    threshold_pipeline: ComputePipeline,

    reduction_layout:   BindGroupLayout,
    reduction_pipeline: ComputePipeline,
    /// Dedicated uniform buffer for reduction (separate from shared pass uniform
    /// so per-depth dispatches can be queued without race risk).
    reduction_uniform:  Buffer,
}

impl Pipelines {
    pub fn new(ctx: &GpuContext) -> Self {
        let device = &ctx.device;

        // Uniform buffer — small, frequently overwritten.
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("pass_params_uniform"),
            size:  std::mem::size_of::<PassParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ── Pass 0: snapshot ────────────────────────────────────────────────
        let snapshot_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("snapshot_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ true),  // values
                storage_entry(1, /*read_only*/ false), // previous_values
                storage_entry(2, /*read_only*/ true),  // output_vectors
                storage_entry(3, /*read_only*/ false), // previous_output_vectors
            ],
        });
        let snapshot_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("snapshot_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/snapshot.wgsl").into()),
        });
        let snapshot_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("snapshot_pl_layout"),
            bind_group_layouts: &[&snapshot_layout],
            push_constant_ranges: &[],
        });
        let snapshot_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("snapshot_pipeline"),
            layout: Some(&snapshot_pl_layout),
            module: &snapshot_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // ── Pass 1: velocity integration ────────────────────────────────────
        let velocity_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("velocity_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ false), // values (rw)
                storage_entry(1, /*read_only*/ true),  // pairs
                uniform_entry(2),                       // params
            ],
        });
        let velocity_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("velocity_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/velocity_integration.wgsl").into()),
        });
        let velocity_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("velocity_pl_layout"),
            bind_group_layouts: &[&velocity_layout],
            push_constant_ranges: &[],
        });
        let velocity_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("velocity_pipeline"),
            layout: Some(&velocity_pl_layout),
            module: &velocity_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // ── Pass 2: intensity update ────────────────────────────────────────
        let intensity_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("intensity_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ false), // values (rw)
                storage_entry(1, /*read_only*/ true),  // intensity_params
                uniform_entry(2),                       // pass_params
            ],
        });
        let intensity_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("intensity_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/intensity_update.wgsl").into()),
        });
        let intensity_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("intensity_pl_layout"),
            bind_group_layouts: &[&intensity_layout],
            push_constant_ranges: &[],
        });
        let intensity_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("intensity_pipeline"),
            layout: Some(&intensity_pl_layout),
            module: &intensity_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // ── Pass 3: overlay transform application ───────────────────────────
        // No uniform buffer: n_slots / n_dims derived from buffer lengths in shader.
        let overlay_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("overlay_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ false), // values (rw)
                storage_entry(1, /*read_only*/ true),  // overlay_deltas
                storage_entry(2, /*read_only*/ true),  // slot_delta_ranges
            ],
        });
        let overlay_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("overlay_shader"),
            source: ShaderSource::Wgsl(
                include_str!("shaders/transform_application.wgsl").into(),
            ),
        });
        let overlay_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("overlay_pl_layout"),
            bind_group_layouts: &[&overlay_layout],
            push_constant_ranges: &[],
        });
        let overlay_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("overlay_pipeline"),
            layout: Some(&overlay_pl_layout),
            module: &overlay_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // Per-tick feeder/player/AI intent deltas, applied before snapshot.
        let intent_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("intent_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ false), // values (rw)
                storage_entry(1, /*read_only*/ true),  // intent_deltas
                uniform_entry(2),                       // params
            ],
        });
        let intent_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("intent_delta_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/intent_delta.wgsl").into()),
        });
        let intent_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("intent_pl_layout"),
            bind_group_layouts: &[&intent_layout],
            push_constant_ranges: &[],
        });
        let intent_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("intent_pipeline"),
            layout: Some(&intent_pl_layout),
            module: &intent_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // ── Pass 7: threshold scan ───────────────────────────────────────────
        let threshold_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("threshold_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ true),  // values
                storage_entry(1, /*read_only*/ true),  // previous_values
                storage_entry(2, /*read_only*/ true),  // output_vectors
                storage_entry(3, /*read_only*/ true),  // previous_output_vectors
                storage_entry(4, /*read_only*/ true),  // registry
                storage_entry(5, /*read_only*/ false), // event_count (atomic u32)
                storage_entry(6, /*read_only*/ false), // event_candidates
                uniform_entry(7),                       // params
            ],
        });
        let threshold_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("threshold_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/threshold_scan.wgsl").into()),
        });
        let threshold_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("threshold_pl_layout"),
            bind_group_layouts: &[&threshold_layout],
            push_constant_ranges: &[],
        });
        let threshold_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("threshold_pipeline"),
            layout: Some(&threshold_pl_layout),
            module: &threshold_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // ── Passes 4–6: bottom-up reduction ─────────────────────────────────
        let reduction_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("reduction_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ true),  // values
                storage_entry(1, /*read_only*/ false), // output_vectors (rw)
                storage_entry(2, /*read_only*/ true),  // child_starts
                storage_entry(3, /*read_only*/ true),  // child_indices
                storage_entry(4, /*read_only*/ true),  // column_rules
                storage_entry(5, /*read_only*/ true),  // depth_slots
                uniform_entry(6),                       // ReduceParams
            ],
        });
        let reduction_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("reduction_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/reduction.wgsl").into()),
        });
        let reduction_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("reduction_pl_layout"),
            bind_group_layouts: &[&reduction_layout],
            push_constant_ranges: &[],
        });
        let reduction_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("reduction_pipeline"),
            layout: Some(&reduction_pl_layout),
            module: &reduction_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });
        let reduction_uniform = device.create_buffer(&BufferDescriptor {
            label: Some("reduction_uniform"),
            size:  std::mem::size_of::<ReduceParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            uniform_buffer,
            snapshot_layout, snapshot_pipeline,
            velocity_layout, velocity_pipeline,
            intensity_layout, intensity_pipeline,
            overlay_layout, overlay_pipeline,
            intent_layout, intent_pipeline,
            threshold_layout, threshold_pipeline,
            reduction_layout, reduction_pipeline, reduction_uniform,
        }
    }

    fn write_params(&self, ctx: &GpuContext, state: &WorldGpuState, dt: f32) {
        let p = PassParams {
            delta_time: dt,
            n_dims:     state.n_dims,
            _pad0: 0, _pad1: 0,
        };
        ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&p));
    }

    pub fn run_snapshot(&self, state: &WorldGpuState) {
        let ctx = &state.ctx;
        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("snapshot_bg"),
            layout: &self.snapshot_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.previous_values.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: state.output_vectors.as_entire_binding() },
                BindGroupEntry { binding: 3, resource: state.previous_output_vectors.as_entire_binding() },
            ],
        });

        let total = state.n_slots * state.n_dims;

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("snapshot_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("snapshot_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.snapshot_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            dispatch_linear(&mut pass, total);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn run_velocity_integration(&self, state: &WorldGpuState, dt: f32) {
        if state.n_governed_pairs == 0 { return; }
        let ctx = &state.ctx;
        self.write_params(ctx, state, dt);

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("velocity_bg"),
            layout: &self.velocity_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.governed_pairs.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
            ],
        });

        let total = state.n_slots * state.n_governed_pairs;

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("velocity_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("velocity_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.velocity_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            dispatch_linear(&mut pass, total);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn run_intensity_update(&self, state: &WorldGpuState, dt: f32) {
        if state.n_intensity_params == 0 { return; }
        let ctx = &state.ctx;
        self.write_params(ctx, state, dt);

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("intensity_bg"),
            layout: &self.intensity_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.intensity_params.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
            ],
        });

        let total = state.n_slots * state.n_intensity_params;

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("intensity_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("intensity_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.intensity_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            dispatch_linear(&mut pass, total);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    /// Pass 3: apply overlay deltas iteratively per slot.
    ///
    /// Reads from `state.overlay_deltas` (pre-uploaded by `upload_overlay_deltas`) and
    /// applies each op in place to `values`. No-ops if `state.n_overlay_deltas == 0`.
    pub fn run_apply_overlays(&self, state: &WorldGpuState) {
        if state.n_overlay_deltas == 0 { return; }
        let ctx = &state.ctx;

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("overlay_bg"),
            layout: &self.overlay_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.overlay_deltas.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: state.slot_delta_ranges.as_entire_binding() },
            ],
        });

        // One thread per slot.

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("overlay_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("overlay_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.overlay_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            dispatch_linear(&mut pass, state.n_slots);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    /// Apply folded per-tick intent deltas directly on the GPU.
    pub fn run_apply_intents(&self, state: &WorldGpuState) {
        if state.n_intent_deltas == 0 { return; }
        let ctx = &state.ctx;
        self.write_params(ctx, state, 0.0);

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("intent_bg"),
            layout: &self.intent_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.intent_deltas.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
            ],
        });

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("intent_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("intent_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.intent_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            dispatch_linear(&mut pass, state.n_intent_deltas);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    /// Passes 4–6: bottom-up reduction. Walks `state.depth_bucket_ranges` from
    /// deepest depth to root depth, dispatching one compute pass per depth.
    ///
    /// Pre-condition: `WorldGpuState::upload_reduction_topology` has been called
    /// and the topology matches the current tree shape. No-op if no buckets
    /// are present.
    /// Consolidated per-tick pipeline. Records intent deltas, snapshot,
    /// velocity, intensity, overlays, reduction, and threshold scan into one
    /// command encoder and submits once.
    pub fn run_tick_pipeline(&self, state: &WorldGpuState, dt: f32) {
        self.run_tick_pipeline_ex(state, dt, false);
    }

    /// Consolidated per-tick pipeline. When `skip_threshold_scan` is true the
    /// Pass 7 threshold dispatch is omitted (C-1 AccumulatorOp path).
    pub fn run_tick_pipeline_ex(
        &self,
        state: &WorldGpuState,
        dt: f32,
        skip_threshold_scan: bool,
    ) {
        let ctx = &state.ctx;

        state.reset_event_count();
        self.write_params(ctx, state, dt);

        let intent_bg = (state.n_intent_deltas > 0).then(|| {
            ctx.device.create_bind_group(&BindGroupDescriptor {
                label: Some("intent_bg"),
                layout: &self.intent_layout,
                entries: &[
                    BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                    BindGroupEntry { binding: 1, resource: state.intent_deltas.as_entire_binding() },
                    BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
                ],
            })
        });

        let snapshot_bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("snapshot_bg"),
            layout: &self.snapshot_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.previous_values.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: state.output_vectors.as_entire_binding() },
                BindGroupEntry { binding: 3, resource: state.previous_output_vectors.as_entire_binding() },
            ],
        });

        let velocity_bg = (state.n_governed_pairs > 0).then(|| {
            ctx.device.create_bind_group(&BindGroupDescriptor {
                label: Some("velocity_bg"),
                layout: &self.velocity_layout,
                entries: &[
                    BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                    BindGroupEntry { binding: 1, resource: state.governed_pairs.as_entire_binding() },
                    BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
                ],
            })
        });

        let intensity_bg = (state.n_intensity_params > 0).then(|| {
            ctx.device.create_bind_group(&BindGroupDescriptor {
                label: Some("intensity_bg"),
                layout: &self.intensity_layout,
                entries: &[
                    BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                    BindGroupEntry { binding: 1, resource: state.intensity_params.as_entire_binding() },
                    BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
                ],
            })
        });

        let overlay_bg = (state.n_overlay_deltas > 0).then(|| {
            ctx.device.create_bind_group(&BindGroupDescriptor {
                label: Some("overlay_bg"),
                layout: &self.overlay_layout,
                entries: &[
                    BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                    BindGroupEntry { binding: 1, resource: state.overlay_deltas.as_entire_binding() },
                    BindGroupEntry { binding: 2, resource: state.slot_delta_ranges.as_entire_binding() },
                ],
            })
        });

        let mut reduction_param_buffers = Vec::new();
        let mut reduction_depth_bgs = Vec::new();
        for &(depth_offset, bucket_size) in state.depth_bucket_ranges.iter().rev() {
            if bucket_size == 0 {
                continue;
            }
            let p = ReduceParams {
                n_dims:       state.n_dims,
                depth_offset,
                bucket_size,
                _pad:         0,
            };
            let buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("reduction_depth_uniform"),
                contents: bytemuck::bytes_of(&p),
                usage: BufferUsages::UNIFORM,
            });
            reduction_param_buffers.push(buf);
            let buf_ref = reduction_param_buffers.last().unwrap();
            let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
                label: Some("reduction_depth_bg"),
                layout: &self.reduction_layout,
                entries: &[
                    BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                    BindGroupEntry { binding: 1, resource: state.output_vectors.as_entire_binding() },
                    BindGroupEntry { binding: 2, resource: state.child_starts.as_entire_binding() },
                    BindGroupEntry { binding: 3, resource: state.child_indices.as_entire_binding() },
                    BindGroupEntry { binding: 4, resource: state.column_rules.as_entire_binding() },
                    BindGroupEntry { binding: 5, resource: state.depth_slots.as_entire_binding() },
                    BindGroupEntry { binding: 6, resource: buf_ref.as_entire_binding() },
                ],
            });
            reduction_depth_bgs.push((bucket_size, bg));
        }

        let threshold_bg = (state.n_thresholds > 0).then(|| {
            ctx.device.create_bind_group(&BindGroupDescriptor {
                label: Some("threshold_bg"),
                layout: &self.threshold_layout,
                entries: &[
                    BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                    BindGroupEntry { binding: 1, resource: state.previous_values.as_entire_binding() },
                    BindGroupEntry { binding: 2, resource: state.output_vectors.as_entire_binding() },
                    BindGroupEntry { binding: 3, resource: state.previous_output_vectors.as_entire_binding() },
                    BindGroupEntry { binding: 4, resource: state.threshold_registry.as_entire_binding() },
                    BindGroupEntry { binding: 5, resource: state.event_count.as_entire_binding() },
                    BindGroupEntry { binding: 6, resource: state.event_candidates.as_entire_binding() },
                    BindGroupEntry { binding: 7, resource: self.uniform_buffer.as_entire_binding() },
                ],
            })
        });

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("tick_pipeline_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("tick_pipeline_pass"),
                timestamp_writes: None,
            });

            if let Some(bg) = intent_bg.as_ref() {
                pass.set_pipeline(&self.intent_pipeline);
                pass.set_bind_group(0, bg, &[]);
                dispatch_linear(&mut pass, state.n_intent_deltas);
            }

            pass.set_pipeline(&self.snapshot_pipeline);
            pass.set_bind_group(0, &snapshot_bg, &[]);
            dispatch_linear(&mut pass, state.n_slots * state.n_dims);

            if let Some(bg) = velocity_bg.as_ref() {
                pass.set_pipeline(&self.velocity_pipeline);
                pass.set_bind_group(0, bg, &[]);
                dispatch_linear(&mut pass, state.n_slots * state.n_governed_pairs);
            }

            if let Some(bg) = intensity_bg.as_ref() {
                pass.set_pipeline(&self.intensity_pipeline);
                pass.set_bind_group(0, bg, &[]);
                dispatch_linear(&mut pass, state.n_slots * state.n_intensity_params);
            }

            if let Some(bg) = overlay_bg.as_ref() {
                pass.set_pipeline(&self.overlay_pipeline);
                pass.set_bind_group(0, bg, &[]);
                dispatch_linear(&mut pass, state.n_slots);
            }

            for (bucket_size, bg) in &reduction_depth_bgs {
                pass.set_pipeline(&self.reduction_pipeline);
                pass.set_bind_group(0, bg, &[]);
                dispatch_linear(&mut pass, *bucket_size);
            }

            if !skip_threshold_scan {
                if let Some(bg) = threshold_bg.as_ref() {
                    pass.set_pipeline(&self.threshold_pipeline);
                    pass.set_bind_group(0, bg, &[]);
                    dispatch_linear(&mut pass, state.n_thresholds);
                }
            }
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn run_reduction_passes(&self, state: &WorldGpuState) {
        if state.depth_bucket_ranges.is_empty() {
            return;
        }
        let ctx = &state.ctx;

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("reduction_bg"),
            layout: &self.reduction_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.output_vectors.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: state.child_starts.as_entire_binding() },
                BindGroupEntry { binding: 3, resource: state.child_indices.as_entire_binding() },
                BindGroupEntry { binding: 4, resource: state.column_rules.as_entire_binding() },
                BindGroupEntry { binding: 5, resource: state.depth_slots.as_entire_binding() },
                BindGroupEntry { binding: 6, resource: self.reduction_uniform.as_entire_binding() },
            ],
        });

        // Iterate buckets deepest-first so children's output_vectors rows are
        // written before parents read them.
        for &(depth_offset, bucket_size) in state.depth_bucket_ranges.iter().rev() {
            if bucket_size == 0 {
                continue;
            }
            let p = ReduceParams {
                n_dims:       state.n_dims,
                depth_offset,
                bucket_size,
                _pad:         0,
            };
            ctx.queue
                .write_buffer(&self.reduction_uniform, 0, bytemuck::bytes_of(&p));

            let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("reduction_encoder"),
            });
            {
                let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                    label: Some("reduction_pass"),
                    timestamp_writes: None,
                });
                pass.set_pipeline(&self.reduction_pipeline);
                pass.set_bind_group(0, &bg, &[]);
                dispatch_linear(&mut pass, bucket_size);
            }
            ctx.queue.submit(Some(encoder.finish()));
        }
    }

    /// Pass 7: scan registered thresholds for crossings between `previous_values`
    /// and `values`. Resets the event counter to zero, dispatches one thread per
    /// registration, and atomically appends `ThresholdEvent`s to
    /// `state.event_candidates`.
    ///
    /// Early-returns if no thresholds are registered. Caller is responsible for
    /// reading the result via `state.read_event_count()` + `state.read_event_candidates(n)`.
    pub fn run_threshold_scan(&self, state: &WorldGpuState) {
        // Reset the atomic counter before this tick's scan.
        state.reset_event_count();

        if state.n_thresholds == 0 { return; }
        let ctx = &state.ctx;

        // n_dims is the only field Pass 7 reads from the uniform; dt is ignored.
        self.write_params(ctx, state, 0.0);

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("threshold_bg"),
            layout: &self.threshold_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.previous_values.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: state.output_vectors.as_entire_binding() },
                BindGroupEntry { binding: 3, resource: state.previous_output_vectors.as_entire_binding() },
                BindGroupEntry { binding: 4, resource: state.threshold_registry.as_entire_binding() },
                BindGroupEntry { binding: 5, resource: state.event_count.as_entire_binding() },
                BindGroupEntry { binding: 6, resource: state.event_candidates.as_entire_binding() },
                BindGroupEntry { binding: 7, resource: self.uniform_buffer.as_entire_binding() },
            ],
        });

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("threshold_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("threshold_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.threshold_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            dispatch_linear(&mut pass, state.n_thresholds);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }
}

fn storage_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::evaluate::Evaluator;
    use simthing_core::{
        DimensionRegistry, IntensityBehavior, PropertyValue, SimProperty, SimThing,
        SimThingKind, SubFieldRole,
    };

    fn try_gpu() -> Option<GpuContext> {
        GpuContext::new_blocking().ok()
    }

    fn loyalty_property() -> SimProperty {
        let mut p = SimProperty::simple("core", "loyalty", 0);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
    }

    fn loyalty_property_wide(extra: usize) -> SimProperty {
        let mut p = SimProperty::simple("core", "loyalty", extra);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
    }

    fn assert_bits_eq(label: &str, cpu: &[f32], gpu: &[f32]) {
        assert_eq!(cpu.len(), gpu.len(), "{label}: length mismatch");
        for (i, (a, b)) in cpu.iter().zip(gpu.iter()).enumerate() {
            assert_eq!(
                a.to_bits(), b.to_bits(),
                "{label}: index {i} diverges — cpu={a} ({:08x}), gpu={b} ({:08x})",
                a.to_bits(), b.to_bits(),
            );
        }
    }

    #[test]
    fn snapshot_copies_values_to_previous() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        reg.register(loyalty_property());
        let state = WorldGpuState::new(ctx, &reg, 3);

        let input: Vec<f32> = (0..state.values_len()).map(|i| (i as f32) * 0.13 + 0.5).collect();
        state.write_values(&input);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);

        let prev = state.read_previous_values();
        assert_bits_eq("snapshot", &input, &prev);
    }

    /// Pass 1 alone, dt = 1.0. Two slots: one mid-range, one at floor with
    /// negative velocity (exercises velocity pinning).
    #[test]
    fn velocity_integration_matches_cpu_oracle_dt_one() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut slots = vec![vec![0.0f32; stride], vec![0.0f32; stride]];
        slots[0][a_off] = 0.4;  slots[0][v_off] =  0.07; slots[0][i_off] = 0.2;
        slots[1][a_off] = 0.0;  slots[1][v_off] = -0.05; slots[1][i_off] = 0.3;

        let dt = 1.0;

        // CPU oracle.
        let mut cpu: Vec<Vec<f32>> = slots.iter().cloned().collect();
        for d in &mut cpu {
            let mut pv = PropertyValue { data: d.clone() };
            pv.integrate(&layout, dt);
            *d = pv.data;
        }

        // GPU.
        let state = WorldGpuState::new(ctx, &reg, 2);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        for (s, d) in slots.iter().enumerate() {
            flat[s * n_dims .. s * n_dims + stride].copy_from_slice(d);
        }
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_velocity_integration(&state, dt);

        let gpu_flat = state.read_values();
        for s in 0..2 {
            let gpu_slice = &gpu_flat[s * n_dims .. s * n_dims + stride];
            assert_bits_eq(&format!("slot {s}"), &cpu[s], gpu_slice);
        }
    }

    /// Pass 1 alone, dt = 0.5. Non-power-of-2 multiplier exercises potential
    /// FMA fusion between (vel * dt) and (value + ...). If this fails by 1 ULP,
    /// the FMA assumption in agents.md Option B is wrong and we need to switch
    /// to explicit fma() on both sides.
    #[test]
    fn velocity_integration_matches_cpu_oracle_fractional_dt() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

        let mut d = vec![0.0f32; stride];
        d[a_off] = 0.4_f32 + 1e-7;   // not a clean fraction
        d[v_off] = 0.07_f32 - 3e-8;  // not a clean fraction

        let dt = 0.5;
        let mut pv = PropertyValue { data: d.clone() };
        pv.integrate(&layout, dt);

        let state = WorldGpuState::new(ctx, &reg, 1);
        let mut flat = vec![0.0f32; state.values_len()];
        flat[..stride].copy_from_slice(&d);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_velocity_integration(&state, dt);

        let gpu_flat = state.read_values();
        assert_bits_eq("fractional-dt", &pv.data, &gpu_flat[..stride]);
    }

    /// Pass 2 alone. Uses raw initial velocity (no Pass 1 first), so CPU
    /// `update_intensity` is called directly on initial data.
    #[test]
    fn intensity_update_matches_cpu_oracle() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let behavior = reg.property(id).intensity_behavior.clone().unwrap();
        let stride = layout.stride();

        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        // Slot 0: high velocity → build branch. Slot 1: low velocity → decay branch.
        let mut slots = vec![vec![0.0f32; stride], vec![0.0f32; stride]];
        slots[0][v_off] = 0.09;  slots[0][i_off] = 0.2;
        slots[1][v_off] = 0.001; slots[1][i_off] = 0.7;

        let dt = 0.5;

        let mut cpu: Vec<Vec<f32>> = slots.iter().cloned().collect();
        for d in &mut cpu {
            let mut pv = PropertyValue { data: d.clone() };
            pv.update_intensity(&behavior, &layout, dt);
            *d = pv.data;
        }

        let state = WorldGpuState::new(ctx, &reg, 2);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        for (s, d) in slots.iter().enumerate() {
            flat[s * n_dims .. s * n_dims + stride].copy_from_slice(d);
        }
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_intensity_update(&state, dt);

        let gpu_flat = state.read_values();
        for s in 0..2 {
            let gpu_slice = &gpu_flat[s * n_dims .. s * n_dims + stride];
            assert_bits_eq(&format!("slot {s}"), &cpu[s], gpu_slice);
        }
    }

    /// End-to-end parity: SlotAllocator + tree projection + Pass 0/1/2
    /// against simthing-core's `Evaluator` on a multi-node tree with multiple
    /// properties (one with intensity_behavior, one without). Verifies that
    /// the GPU pipeline driven from a real SimThing tree matches the CPU
    /// oracle bit-exactly across every (slot, property, column).
    #[test]
    fn tree_driven_pipeline_matches_evaluator() {
        use crate::projection::project_tree_to_values;
        use crate::slot::SlotAllocator;
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let loyalty_id = reg.register(loyalty_property());
        let food_id    = reg.register(SimProperty::simple("core", "food_security", 0));

        let l_layout = reg.property(loyalty_id).layout.clone();
        let f_layout = reg.property(food_id).layout.clone();

        let la = l_layout.offset_of(&SubFieldRole::Amount).unwrap();
        let lv = l_layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let li = l_layout.offset_of(&SubFieldRole::Intensity).unwrap();
        let fa = f_layout.offset_of(&SubFieldRole::Amount).unwrap();
        let fv = f_layout.offset_of(&SubFieldRole::Velocity).unwrap();

        // Build tree: World → 2 Locations → 2 Cohorts each. 7 nodes total.
        // Each cohort carries loyalty; only first cohort of each location
        // carries food_security too. Velocities span build/decay branches
        // and one cohort starts at the loyalty floor (pinning case).
        let cohort_specs: [(f32, f32, f32); 4] = [
            // (loyalty_amount, loyalty_velocity, loyalty_intensity)
            (0.40,  0.07,  0.20),  // mid-range, building intensity
            (0.85, -0.001, 0.60),  // near ceiling, decay branch
            (0.00, -0.05,  0.30),  // at floor, negative vel → pinning
            (0.50,  0.09,  0.10),  // mid-range, building
        ];

        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut cohort_ids = Vec::new();
        let mut loc_food_owners = Vec::new();
        for loc_i in 0..2 {
            let mut loc = SimThing::new(SimThingKind::Location, 0);
            for cj in 0..2 {
                let (la_v, lv_v, li_v) = cohort_specs[loc_i * 2 + cj];
                let mut cohort = SimThing::new(SimThingKind::Cohort, 0);

                let mut pv_l = PropertyValue::from_layout(&l_layout);
                pv_l.data[la] = la_v;
                pv_l.data[lv] = lv_v;
                pv_l.data[li] = li_v;
                cohort.add_property(loyalty_id, pv_l);

                if cj == 0 {
                    let mut pv_f = PropertyValue::from_layout(&f_layout);
                    pv_f.data[fa] = 0.7 + 0.05 * (loc_i as f32);
                    pv_f.data[fv] = 0.02;
                    cohort.add_property(food_id, pv_f);
                    loc_food_owners.push(cohort.id);
                }
                cohort_ids.push(cohort.id);
                loc.add_child(cohort);
            }
            world.add_child(loc);
        }

        let dt = 0.5;

        // CPU oracle.
        let cpu_snap = Evaluator::new(&reg, dt).evaluate(&world, 1);

        // Allocate slots for every node in the tree, then project.
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        pipelines.run_velocity_integration(&state, dt);
        pipelines.run_intensity_update(&state, dt);

        let gpu_flat = state.read_values();

        // Compare every CPU-snapshot entity's properties against the
        // corresponding slot row in the GPU buffer.
        for entity in &cpu_snap.entities {
            let slot = alloc.slot_of(entity.id)
                .unwrap_or_else(|| panic!("entity {:?} not allocated", entity.id));
            let slot_base = slot as usize * n_dims;

            for (prop_id, cpu_pv) in &entity.properties {
                let range = reg.column_range(*prop_id);
                let start = slot_base + range.start;
                let end   = start + cpu_pv.data.len();
                let gpu_data = &gpu_flat[start..end];
                let label = format!(
                    "entity {:?} slot {} prop {:?}",
                    entity.id, slot, prop_id,
                );
                assert_bits_eq(&label, &cpu_pv.data, gpu_data);
            }
        }

        // Pass 0 invariant: previous_values still matches the initial flat
        // buffer (snapshot ran before integration mutated values).
        let prev = state.read_previous_values();
        assert_bits_eq("Pass 0 (tree) snapshot", &flat, &prev);
    }

    /// Full Pass 0+1+2 pipeline matches simthing-core's `Evaluator` (the
    /// authoritative CPU oracle) on a single SimThing with one property
    /// and no overlays. Pass 0 result is verified via previous_values readback.
    #[test]
    fn full_pipeline_matches_evaluator() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[a_off] = 0.4;
        pv.data[v_off] = 0.07;
        pv.data[i_off] = 0.2;
        cohort.add_property(id, pv);
        let cohort_id = cohort.id;
        let initial_data = cohort.properties[&id].data.clone();

        let dt = 0.5;

        let evaluator = Evaluator::new(&reg, dt);
        let snap = evaluator.evaluate(&cohort, 1);
        let cpu_data = &snap.get(cohort_id).unwrap().properties[&id].data;

        let state = WorldGpuState::new(ctx, &reg, 1);
        let range = reg.column_range(id).clone();
        let mut flat = vec![0.0f32; state.values_len()];
        flat[range.start..range.start + stride].copy_from_slice(&initial_data);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        pipelines.run_velocity_integration(&state, dt);
        pipelines.run_intensity_update(&state, dt);

        // Pass 0 invariant: previous_values must equal the pre-pass values.
        let prev = state.read_previous_values();
        assert_bits_eq(
            "Pass 0 snapshot",
            &flat,
            &prev,
        );

        let gpu_flat = state.read_values();
        let gpu_data = &gpu_flat[range.start..range.start + stride];
        assert_bits_eq("Evaluator vs GPU pipeline", cpu_data, gpu_data);
    }

    #[test]
    fn run_tick_pipeline_matches_manual_pass_sequence() {
        use crate::world_state::{
            IntentDelta, DIR_DOWNWARD, THRESH_BUF_VALUES, ThresholdRegistration,
        };

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let amount_col = layout.offset_of(&SubFieldRole::Amount).unwrap() as u32;
        let velocity_col = layout.offset_of(&SubFieldRole::Velocity).unwrap() as usize;
        let intensity_col = layout.offset_of(&SubFieldRole::Intensity).unwrap() as usize;

        let mut initial = vec![0.0_f32; reg.total_columns];
        initial[amount_col as usize] = 0.35;
        initial[velocity_col] = -0.20;
        initial[intensity_col] = 0.10;

        let intent = [IntentDelta {
            slot: 0,
            col: amount_col,
            mul: 1.0,
            add: 0.05,
        }];
        let regs = [ThresholdRegistration {
            slot: 0,
            col: amount_col,
            threshold: 0.30,
            direction: DIR_DOWNWARD,
            event_kind: 77,
            buffer: THRESH_BUF_VALUES,
        }];

        let mut manual = WorldGpuState::new(ctx, &reg, 1);
        manual.write_values(&initial);
        manual.upload_intent_deltas(&intent);
        manual.upload_thresholds(&regs);
        let pipelines = Pipelines::new(&manual.ctx);

        pipelines.run_apply_intents(&manual);
        pipelines.run_snapshot(&manual);
        pipelines.run_velocity_integration(&manual, 1.0);
        pipelines.run_intensity_update(&manual, 1.0);
        pipelines.run_apply_overlays(&manual);
        pipelines.run_reduction_passes(&manual);
        pipelines.run_threshold_scan(&manual);

        let ctx2 = GpuContext::new_blocking().expect("second gpu context");
        let mut piped = WorldGpuState::new(ctx2, &reg, 1);
        piped.write_values(&initial);
        piped.upload_intent_deltas(&intent);
        piped.upload_thresholds(&regs);
        let pipelines2 = Pipelines::new(&piped.ctx);
        pipelines2.run_tick_pipeline(&piped, 1.0);

        assert_bits_eq("values", &manual.read_values(), &piped.read_values());
        assert_bits_eq(
            "previous_values",
            &manual.read_previous_values(),
            &piped.read_previous_values(),
        );
        assert_eq!(manual.read_event_count(), piped.read_event_count());
        assert_eq!(
            manual.read_event_candidates(1),
            piped.read_event_candidates(1),
        );
    }

    /// CPU oracle for Pass 7. Same crossing logic as the WGSL shader; used to
    /// produce reference events for the parity test below.
    fn cpu_threshold_scan(
        previous_values: &[f32],
        values:          &[f32],
        previous_output: &[f32],
        output:          &[f32],
        n_dims:          u32,
        regs:            &[crate::world_state::ThresholdRegistration],
    ) -> Vec<crate::world_state::ThresholdEvent> {
        use crate::world_state::{
            DIR_DOWNWARD, DIR_UPWARD, THRESH_BUF_OUTPUT, ThresholdEvent,
        };
        let mut events = Vec::new();
        for r in regs {
            let addr = (r.slot * n_dims + r.col) as usize;
            let (prev, curr) = if r.buffer == THRESH_BUF_OUTPUT {
                (previous_output[addr], output[addr])
            } else {
                (previous_values[addr], values[addr])
            };
            let up   = prev <= r.threshold && curr > r.threshold;
            let down = prev >= r.threshold && curr < r.threshold;
            let crossed = match r.direction {
                DIR_UPWARD   => up,
                DIR_DOWNWARD => down,
                _            => up || down,
            };
            if crossed {
                events.push(ThresholdEvent {
                    slot:       r.slot,
                    col:        r.col,
                    value:      curr,
                    event_kind: r.event_kind,
                });
            }
        }
        events
    }

    /// Pass 7 directly: set up `previous_values` and `values` so that each
    /// crossing direction fires exactly when expected, and a stationary-on-threshold
    /// case does NOT fire (strict crossing rule).
    #[test]
    fn threshold_scan_matches_cpu_oracle() {
        use crate::world_state::{
            DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_VALUES, ThresholdEvent,
            ThresholdRegistration,
        };

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut state = WorldGpuState::new(ctx, &reg, 4);
        let n_dims = state.n_dims as usize;

        // 4 slots, exercising all three direction modes:
        //   slot 0: DOWN crossing 0.40 → 0.10, threshold 0.30 → fires
        //   slot 1: UP   crossing 0.10 → 0.50, threshold 0.30 → fires
        //   slot 2: STATIONARY at 0.50, EITHER threshold 0.50 → does NOT fire
        //   slot 3: DOWN crossing 0.60 → 0.40, EITHER threshold 0.50 → fires
        let mut previous = vec![0.0_f32; state.values_len()];
        let mut current  = vec![0.0_f32; state.values_len()];
        previous[0 * n_dims] = 0.40;  current[0 * n_dims] = 0.10;
        previous[1 * n_dims] = 0.10;  current[1 * n_dims] = 0.50;
        previous[2 * n_dims] = 0.50;  current[2 * n_dims] = 0.50;
        previous[3 * n_dims] = 0.60;  current[3 * n_dims] = 0.40;

        state.write_previous_values(&previous);
        state.write_values(&current);

        let regs = vec![
            ThresholdRegistration { slot: 0, col: 0, threshold: 0.30, direction: DIR_DOWNWARD, event_kind: 100, buffer: THRESH_BUF_VALUES },
            ThresholdRegistration { slot: 1, col: 0, threshold: 0.30, direction: DIR_UPWARD,   event_kind: 101, buffer: THRESH_BUF_VALUES },
            ThresholdRegistration { slot: 2, col: 0, threshold: 0.50, direction: DIR_EITHER,   event_kind: 102, buffer: THRESH_BUF_VALUES },
            ThresholdRegistration { slot: 3, col: 0, threshold: 0.50, direction: DIR_EITHER,   event_kind: 103, buffer: THRESH_BUF_VALUES },
        ];
        state.upload_thresholds(&regs);

        let prev_out = vec![0.0_f32; state.values_len()];
        let out_flat = vec![0.0_f32; state.values_len()];
        let cpu = cpu_threshold_scan(
            &previous, &current, &prev_out, &out_flat, n_dims as u32, &regs,
        );
        assert_eq!(cpu.len(), 3, "oracle should produce exactly 3 events");

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_threshold_scan(&state);

        let count = state.read_event_count();
        let mut gpu: Vec<ThresholdEvent> = state.read_event_candidates(count);

        // GPU event order is nondeterministic (atomicAdd race). Sort both sides.
        let key = |e: &ThresholdEvent| (e.event_kind, e.slot, e.col);
        let mut cpu_sorted = cpu;
        cpu_sorted.sort_by_key(key);
        gpu.sort_by_key(key);

        assert_eq!(cpu_sorted.len(), gpu.len(),
            "event count mismatch: cpu={} gpu={}", cpu_sorted.len(), gpu.len());

        for (i, (c, g)) in cpu_sorted.iter().zip(gpu.iter()).enumerate() {
            assert_eq!(c.slot,       g.slot,       "event {i} slot");
            assert_eq!(c.col,        g.col,        "event {i} col");
            assert_eq!(c.event_kind, g.event_kind, "event {i} event_kind");
            assert_eq!(c.value.to_bits(), g.value.to_bits(),
                "event {i} value: cpu={} gpu={}", c.value, g.value);
        }
    }

    /// Pass 7 on `output_vectors`: upward crossing on a parent aggregate row.
    #[test]
    fn threshold_scan_on_output_vectors_matches_cpu_oracle() {
        use crate::world_state::{
            DIR_UPWARD, THRESH_BUF_OUTPUT, ThresholdRegistration,
        };

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut state = WorldGpuState::new(ctx, &reg, 2);
        let n_dims = state.n_dims as usize;

        let mut prev_out = vec![0.0_f32; state.values_len()];
        let mut curr_out = vec![0.0_f32; state.values_len()];
        // Parent slot 0: aggregate loyalty amount crosses 0.30 upward (0.20 -> 0.50).
        prev_out[0 * n_dims] = 0.20;
        curr_out[0 * n_dims] = 0.50;

        state.write_previous_output_vectors(&prev_out);
        state.write_output_vectors(&curr_out);

        let regs = vec![ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 0.30,
            direction: DIR_UPWARD,
            event_kind: 200,
            buffer: THRESH_BUF_OUTPUT,
        }];
        state.upload_thresholds(&regs);

        let prev_vals = vec![0.0_f32; state.values_len()];
        let curr_vals = vec![0.0_f32; state.values_len()];
        let cpu = cpu_threshold_scan(
            &prev_vals,
            &curr_vals,
            &prev_out,
            &curr_out,
            n_dims as u32,
            &regs,
        );
        assert_eq!(cpu.len(), 1);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_threshold_scan(&state);

        let count = state.read_event_count();
        let gpu = state.read_event_candidates(count);
        assert_eq!(gpu.len(), 1);
        assert_eq!(gpu[0].value.to_bits(), 0.50_f32.to_bits());
        assert_eq!(gpu[0].event_kind, 200);
    }

    /// Pass 7 with no registered thresholds: must be a no-op, no panic.
    #[test]
    fn threshold_scan_no_registrations_is_noop() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let state = WorldGpuState::new(ctx, &reg, 2);
        let pipelines = Pipelines::new(&state.ctx);
        state
            .ctx
            .queue
            .write_buffer(&state.event_count, 0, &42u32.to_le_bytes());
        pipelines.run_threshold_scan(&state); // n_thresholds == 0 → no-op
        assert_eq!(state.read_event_count(), 0);
    }

    /// End-to-end Pass 0+1+2+3+7: a velocity-integration tick crosses a threshold
    /// registered on the amount sub-field, and Pass 7 detects the crossing using
    /// the post-Pass-0 snapshot vs. post-integration values.
    #[test]
    fn threshold_scan_after_full_pipeline() {
        use crate::world_state::{DIR_DOWNWARD, THRESH_BUF_VALUES, ThresholdRegistration};

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

        let mut state = WorldGpuState::new(ctx, &reg, 1);
        let mut flat = vec![0.0_f32; state.values_len()];
        flat[a_off] = 0.35; // starts above threshold 0.30
        flat[v_off] = -0.10; // dt = 1.0 → ends at 0.25, crossing 0.30 downward
        state.write_values(&flat);
        let _ = stride;

        let regs = vec![
            ThresholdRegistration {
                slot: 0, col: a_off as u32,
                threshold:  0.30,
                direction:  DIR_DOWNWARD,
                event_kind: 7,
                buffer:     THRESH_BUF_VALUES,
            },
        ];
        state.upload_thresholds(&regs);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);                  // previous_* <- current
        pipelines.run_velocity_integration(&state, 1.0); // values amount: 0.35 - 0.10 = 0.25
        pipelines.run_intensity_update(&state, 1.0);
        pipelines.run_apply_overlays(&state);
        pipelines.run_reduction_passes(&state);
        pipelines.run_threshold_scan(&state);

        let count = state.read_event_count();
        assert_eq!(count, 1, "expected exactly one downward crossing");

        let events = state.read_event_candidates(count);
        assert_eq!(events[0].slot, 0);
        assert_eq!(events[0].col, a_off as u32);
        assert_eq!(events[0].event_kind, 7);
        // Post-integration value should be 0.25 bit-exact.
        assert_eq!(events[0].value.to_bits(), 0.25_f32.to_bits());
    }

    /// Week 2 success criterion: full Pass 0+1+2+3 pipeline at 1000 slots, 64 dims
    /// must complete within the 50 ms day-boundary budget. Marked `#[ignore]`
    /// because it's a wall-clock diagnostic, not a correctness test — run with
    /// `cargo test -- --ignored pipeline_timing`.
    ///
    /// One property with a 61-wide named vector: stride = 3 + 61 = 64 dims.
    /// Each of 1000 slots gets one local overlay (`Multiply` on amount), so the
    /// overlay batch has 1000 deltas — a realistic per-tick churn.
    #[test]
    #[ignore]
    fn pipeline_timing_1000_slots_64_dims() {
        use crate::overlay_prep::build_overlay_deltas;
        use crate::projection::project_tree_to_values;
        use crate::slot::SlotAllocator;
        use simthing_core::ids::OverlayId;
        use simthing_core::overlay::{Overlay, OverlayKind, OverlayLifecycle, OverlaySource,
                                     PropertyTransformDelta};
        use simthing_core::property::TransformOp;
        use std::time::Instant;

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        // standard(61): stride = 3 amount/velocity/intensity + 61 named = 64.
        let lid = reg.register(loyalty_property_wide(61));
        let layout = reg.property(lid).layout.clone();
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut world = SimThing::new(SimThingKind::World, 0);
        for k in 0..1000u32 {
            let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
            let mut pv = PropertyValue::from_layout(&layout);
            pv.data[a_off] = 0.5 + (k as f32) * 1e-4;
            pv.data[v_off] = 0.05;
            pv.data[i_off] = 0.3;
            cohort.add_property(lid, pv);
            cohort.add_overlay(Overlay {
                id:        OverlayId::new(),
                kind:      OverlayKind::Policy,
                source:    OverlaySource::Player,
                affects:   vec![],
                transform: PropertyTransformDelta {
                    property_id:      lid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(0.99))],
                },
                lifecycle: OverlayLifecycle::Permanent,
            });
            world.add_child(cohort);
        }

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        assert_eq!(state.n_dims,  64);
        assert!(state.n_slots >= 1000);

        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        let (od, ranges) = build_overlay_deltas(&world, &reg, &alloc);
        state.upload_overlay_deltas(&od, &ranges);

        let pipelines = Pipelines::new(&state.ctx);

        // Warm-up tick — first dispatch incurs pipeline cache + driver init.
        pipelines.run_snapshot(&state);
        pipelines.run_velocity_integration(&state, 0.5);
        pipelines.run_intensity_update(&state, 0.5);
        pipelines.run_apply_overlays(&state);
        let _ = state.read_values(); // force flush

        let t0 = Instant::now();
        pipelines.run_snapshot(&state);
        pipelines.run_velocity_integration(&state, 0.5);
        pipelines.run_intensity_update(&state, 0.5);
        pipelines.run_apply_overlays(&state);
        // Force the submitted work to complete before stopping the clock.
        let _ = state.read_values();
        let elapsed = t0.elapsed();

        eprintln!(
            "pipeline_timing_1000x64: {} slots × {} dims, {} overlay deltas → {:.2} ms",
            state.n_slots, state.n_dims, od.len(), elapsed.as_secs_f64() * 1000.0,
        );
        assert!(
            elapsed.as_millis() < 50,
            "Pass 0+1+2+3 took {} ms, exceeds 50 ms day-boundary budget",
            elapsed.as_millis(),
        );
    }

    /// Pass 0+1+2+3 against Evaluator on a tree with overlays at multiple levels.
    ///
    /// Tree: World (Multiply loyalty amount by 0.8)
    ///         └─ Location (Add -0.1 to loyalty velocity)
    ///               ├─ Cohort A (has loyalty; Set intensity to 0.5 locally)
    ///               └─ Cohort B (has loyalty; no local overlays)
    ///
    /// Covers all three op kinds, ancestor + local ordering, and the case where
    /// the ancestor overlay of the Location doesn't affect a node that lacks
    /// the property (World itself has no loyalty property).
    #[test]
    fn pass3_overlay_matches_evaluator() {
        use crate::overlay_prep::build_overlay_deltas;
        use crate::projection::project_tree_to_values;
        use crate::slot::SlotAllocator;
        use simthing_core::ids::OverlayId;
        use simthing_core::overlay::{Overlay, OverlayKind, OverlayLifecycle, OverlaySource,
                                     PropertyTransformDelta};
        use simthing_core::property::TransformOp;

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let lid = reg.register(loyalty_property());
        let layout = reg.property(lid).layout.clone();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let make_overlay = |deltas: Vec<(SubFieldRole, TransformOp)>| -> Overlay {
            Overlay {
                id:        OverlayId::new(),
                kind:      OverlayKind::Policy,
                source:    OverlaySource::Player,
                affects:   vec![],
                transform: PropertyTransformDelta {
                    property_id:      lid,
                    sub_field_deltas: deltas,
                },
                lifecycle: OverlayLifecycle::Permanent,
            }
        };

        // World: Multiply(0.8) on loyalty amount.
        let mut world = SimThing::new(SimThingKind::World, 0);
        world.add_overlay(make_overlay(vec![
            (SubFieldRole::Amount, TransformOp::Multiply(0.8)),
        ]));

        // Location: Add(-0.1) on loyalty velocity.
        let mut location = SimThing::new(SimThingKind::Location, 0);
        location.add_overlay(make_overlay(vec![
            (SubFieldRole::Velocity, TransformOp::Add(-0.1)),
        ]));

        // Cohort A: loyalty mid-range, building; local Set(0.5) on intensity.
        let mut cohort_a = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv_a = PropertyValue::from_layout(&layout);
        pv_a.data[a_off] = 0.60;
        pv_a.data[v_off] = 0.08;
        pv_a.data[i_off] = 0.20;
        cohort_a.add_property(lid, pv_a);
        cohort_a.add_overlay(make_overlay(vec![
            (SubFieldRole::Intensity, TransformOp::Set(0.5)),
        ]));
        let cohort_a_id = cohort_a.id;

        // Cohort B: loyalty near floor, negative velocity (exercises pinning + overlays).
        let mut cohort_b = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv_b = PropertyValue::from_layout(&layout);
        pv_b.data[a_off] = 0.05;
        pv_b.data[v_off] = -0.03;
        pv_b.data[i_off] = 0.40;
        cohort_b.add_property(lid, pv_b);
        let cohort_b_id = cohort_b.id;

        location.add_child(cohort_a);
        location.add_child(cohort_b);
        world.add_child(location);

        let dt = 0.5;

        // CPU oracle.
        let cpu_snap = Evaluator::new(&reg, dt).evaluate(&world, 1);

        // GPU: allocate slots, project initial values, upload overlay deltas, run passes.
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        // Build and upload the overlay delta batch for this tick.
        let (od, ranges) = build_overlay_deltas(&world, &reg, &alloc);
        state.upload_overlay_deltas(&od, &ranges);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        pipelines.run_velocity_integration(&state, dt);
        pipelines.run_intensity_update(&state, dt);
        pipelines.run_apply_overlays(&state);

        let gpu_flat = state.read_values();

        // Compare Cohort A.
        for &(entity_id, label) in &[(cohort_a_id, "cohort_a"), (cohort_b_id, "cohort_b")] {
            let entity = cpu_snap.get(entity_id).unwrap();
            let slot = alloc.slot_of(entity_id).unwrap();
            let slot_base = slot as usize * n_dims;
            let range = reg.column_range(lid);
            let start = slot_base + range.start;
            let end   = start + entity.properties[&lid].data.len();
            assert_bits_eq(label, &entity.properties[&lid].data, &gpu_flat[start..end]);
        }
    }

    /// Passes 4–6 parity: GPU output_vectors must match the CPU oracle
    /// bit-exactly on a 3-tier tree (World → Locations → Cohorts).
    ///
    /// Exercises Mean (amount, velocity, named cols), Max (intensity), and the
    /// canonical child iteration order on multiple parents at the same depth.
    #[test]
    fn reduction_matches_cpu_oracle() {
        use crate::projection::project_tree_to_values;
        use crate::reduction::{
            build_column_rule_descriptors, build_topology, cpu_reduce_oracle,
            encode_column_rules,
        };
        use crate::slot::SlotAllocator;

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let lid = reg.register(loyalty_property());
        let layout = reg.property(lid).layout.clone();
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        // Build: World → 2 Locations → 3, 2 cohorts respectively. 8 nodes.
        let cohort_data: [&[(f32, f32, f32)]; 2] = [
            &[(0.40, 0.07, 0.20), (0.85, -0.001, 0.60), (0.10, 0.05, 0.30)],
            &[(0.55, 0.03, 0.45), (0.20, -0.02, 0.10)],
        ];

        let mut world = SimThing::new(SimThingKind::World, 0);
        for cohorts in &cohort_data {
            let mut loc = SimThing::new(SimThingKind::Location, 0);
            for &(a, v, i) in *cohorts {
                let mut c = SimThing::new(SimThingKind::Cohort, 0);
                let mut pv = PropertyValue::from_layout(&layout);
                pv.data[a_off] = a;
                pv.data[v_off] = v;
                pv.data[i_off] = i;
                c.add_property(lid, pv);
                loc.add_child(c);
            }
            world.add_child(loc);
        }

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        let n_dims = state.n_dims as usize;

        // Project leaf values into the `values` buffer. Inner-node rows are zero;
        // the shader overwrites them via reduction.
        let mut flat = vec![0.0_f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        // Build + upload topology + rules.
        let topo = build_topology(&world, &alloc);
        let descriptors = build_column_rule_descriptors(&reg, n_dims);
        let rules_u32 = encode_column_rules(&descriptors);

        let mut depth_slots: Vec<u32> = Vec::new();
        let mut depth_ranges: Vec<(u32, u32)> = Vec::new();
        for bucket in &topo.depth_buckets {
            let offset = depth_slots.len() as u32;
            depth_slots.extend_from_slice(bucket);
            depth_ranges.push((offset, bucket.len() as u32));
        }
        state.upload_reduction_topology(
            &topo.child_starts,
            &topo.child_indices,
            &rules_u32,
            &depth_slots,
            depth_ranges,
        );

        // CPU oracle.
        let mut cpu_output = vec![0.0_f32; flat.len()];
        cpu_reduce_oracle(&topo, &descriptors, n_dims, &flat, &mut cpu_output);

        // GPU.
        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_reduction_passes(&state);

        let gpu_output = state.read_output_vectors();

        // Inner-node rows must match (cohorts are leaves — already covered by
        // the leaf branch which copies values → output_vectors, also bit-exact).
        assert_bits_eq("reduction full buffer", &cpu_output, &gpu_output);
    }

    /// WeightedMean parity: location loyalty = population-weighted cohort mean.
    #[test]
    fn weighted_mean_reduction_matches_cpu_oracle() {
        use crate::projection::project_tree_to_values;
        use crate::reduction::{
            build_column_rule_descriptors, build_topology, cpu_reduce_oracle,
            encode_column_rules,
        };
        use crate::slot::SlotAllocator;
        use simthing_core::ReductionRule;

        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let pop_id = reg.register(SimProperty::simple("demo", "population", 0));
        let pop_layout = reg.property(pop_id).layout.clone();
        let pop_a_off = pop_layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut loyalty = SimProperty::simple("core", "loyalty", 0);
        let loyalty_layout = loyalty.layout.clone();
        let loyalty_a_off = loyalty_layout.offset_of(&SubFieldRole::Amount).unwrap();
        loyalty.layout.sub_fields[0].reduction_override =
            Some(ReductionRule::WeightedMean { by: pop_id });
        let lid = reg.register(loyalty);

        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut loc = SimThing::new(SimThingKind::Location, 0);
        for (loyalty_amt, pop_amt) in [(0.40f32, 100.0), (0.80, 300.0)] {
            let mut c = SimThing::new(SimThingKind::Cohort, 0);
            let mut lpv = PropertyValue::from_layout(&loyalty_layout);
            lpv.data[loyalty_a_off] = loyalty_amt;
            c.add_property(lid, lpv);
            let mut ppv = PropertyValue::from_layout(&pop_layout);
            ppv.data[pop_a_off] = pop_amt;
            c.add_property(pop_id, ppv);
            loc.add_child(c);
        }
        world.add_child(loc);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        let n_dims = state.n_dims as usize;

        let mut flat = vec![0.0_f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        let topo = build_topology(&world, &alloc);
        let descriptors = build_column_rule_descriptors(&reg, n_dims);
        let rules_u32 = encode_column_rules(&descriptors);

        let mut depth_slots: Vec<u32> = Vec::new();
        let mut depth_ranges: Vec<(u32, u32)> = Vec::new();
        for bucket in &topo.depth_buckets {
            let offset = depth_slots.len() as u32;
            depth_slots.extend_from_slice(bucket);
            depth_ranges.push((offset, bucket.len() as u32));
        }
        state.upload_reduction_topology(
            &topo.child_starts,
            &topo.child_indices,
            &rules_u32,
            &depth_slots,
            depth_ranges,
        );

        let mut cpu_output = vec![0.0_f32; flat.len()];
        cpu_reduce_oracle(&topo, &descriptors, n_dims, &flat, &mut cpu_output);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_reduction_passes(&state);

        let gpu_output = state.read_output_vectors();
        assert_bits_eq("weighted mean reduction", &cpu_output, &gpu_output);
    }
}


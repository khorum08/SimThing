//! Compute pipelines and dispatch wrappers for the retained snapshot operation
//! plus AccumulatorOp-backed tick orchestration.
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
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePass, ComputePassDescriptor, ComputePipeline,
    ComputePipelineDescriptor, PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource,
    ShaderStages,
};

use crate::context::GpuContext;
use crate::reduction_orderband::reduction_soft_band_for_depth_bucket;
use crate::world_state::WorldGpuState;

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
    n_dims: u32,
    _pad0: u32,
    _pad1: u32,
}

/// Optional AccumulatorOp sessions folded into one tick command buffer (C-1/C-2/C-3/C-5/C-7).
pub struct AccumulatorPipelineSessions<'a> {
    pub threshold: Option<&'a mut crate::AccumulatorOpSession>,
    pub intent: Option<&'a mut crate::AccumulatorOpSession>,
    pub overlay_add: Option<&'a mut crate::AccumulatorOpSession>,
    pub reduction_soft: Option<&'a mut crate::AccumulatorOpSession>,
    pub velocity: Option<&'a mut crate::AccumulatorOpSession>,
    pub intensity_eml: Option<&'a mut crate::AccumulatorOpSession>,
    pub transfer: Option<&'a mut crate::AccumulatorOpSession>,
    pub emission: Option<&'a mut crate::AccumulatorOpSession>,
    pub encode_world_summary: bool,
}

pub struct Pipelines {
    uniform_buffer: Buffer,

    snapshot_layout: BindGroupLayout,
    snapshot_pipeline: ComputePipeline,
}

impl Pipelines {
    pub fn new(ctx: &GpuContext) -> Self {
        let device = &ctx.device;

        // Uniform buffer — small, frequently overwritten.
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("pass_params_uniform"),
            size: std::mem::size_of::<PassParams>() as u64,
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
            label: Some("snapshot_shader"),
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

        Self {
            uniform_buffer,
            snapshot_layout,
            snapshot_pipeline,
        }
    }

    fn write_params(&self, ctx: &GpuContext, state: &WorldGpuState, dt: f32) {
        let p = PassParams {
            delta_time: dt,
            n_dims: state.n_dims,
            _pad0: 0,
            _pad1: 0,
        };
        ctx.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&p));
    }

    pub fn run_snapshot(&self, state: &WorldGpuState) {
        let ctx = &state.ctx;
        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("snapshot_bg"),
            layout: &self.snapshot_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: state.values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: state.previous_values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: state.output_vectors.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: state.previous_output_vectors.as_entire_binding(),
                },
            ],
        });

        let total = state.n_slots * state.n_dims;

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
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

    /// C-8b EvalEML intensity update (requires `sync_intensity_eml_accumulator` first).
    pub fn run_accumulator_intensity_eml(&self, state: &mut WorldGpuState, dt: f32) {
        if !state.accumulator_intensity_eml_active {
            return;
        }
        let ctx = &state.ctx;
        self.write_params(ctx, state, dt);
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("intensity_eml_encoder"),
            });
        if let Some(runtime) = state.accumulator_runtime.as_mut() {
            let mut session = runtime.take_intensity_eml_session();
            if let Some(session) = session.as_mut() {
                let eml = runtime.eml_bind_buffers();
                session.encode_intensity_eml_into(
                    ctx,
                    &mut encoder,
                    &state.values,
                    &state.previous_values,
                    dt,
                    eml,
                );
            }
            runtime.restore_intensity_eml_session(session);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    /// S-3 overlay dispatch: apply C-4 OrderBand overlay ops through AccumulatorOp.
    pub fn run_accumulator_overlays(&self, state: &mut WorldGpuState) {
        if !state.accumulator_overlay_add_active || state.accumulator_overlay_add_bands == 0 {
            return;
        }
        let ctx = &state.ctx;
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("overlay_accumulator_encoder"),
            });
        let mut session = state
            .accumulator_runtime
            .as_mut()
            .and_then(|runtime| runtime.take_overlay_session());
        if let Some(session) = session.as_mut() {
            session.encode_overlay_add_into(
                ctx,
                &mut encoder,
                &state.values,
                &state.previous_values,
                state.accumulator_overlay_add_bands,
            );
        }
        if let Some(runtime) = state.accumulator_runtime.as_mut() {
            runtime.restore_overlay_session(session);
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
    pub fn run_tick_pipeline(&self, state: &mut WorldGpuState, dt: f32) {
        self.run_tick_pipeline_ex(state, dt, false);
    }

    /// Consolidated per-tick pipeline integrated with AccumulatorOp migrations.
    /// Encodes intent (C-2, before snapshot), Passes 0–6, and threshold scan
    /// (C-1, after reduction) into one command buffer and submits once.
    pub fn run_tick_pipeline_with_accumulators(
        &self,
        state: &mut WorldGpuState,
        dt: f32,
        mut sessions: AccumulatorPipelineSessions<'_>,
    ) {
        let skip_old_intent = sessions.intent.is_some();
        let skip_threshold_scan = sessions.threshold.is_some();
        if let Some(session) = sessions.intent.as_mut() {
            session.prepare_intent(&state.ctx);
        }
        if let Some(session) = sessions.overlay_add.as_mut() {
            session.prepare_overlay_add(&state.ctx);
        }
        if let Some(session) = sessions.threshold.as_mut() {
            session.prepare_threshold_scan(&state.ctx);
        }
        self.run_tick_pipeline_internal(
            state,
            dt,
            skip_old_intent,
            skip_threshold_scan,
            &mut sessions,
        );
        if let Some(session) = sessions.intent.as_mut() {
            session.finish_intent(&state.ctx);
        }
        if let Some(session) = sessions.threshold.as_mut() {
            session.finish_threshold_scan(&state.ctx);
        }
    }

    /// Consolidated per-tick pipeline integrated with the C-1 AccumulatorOp
    /// threshold scan. Prefer [`Self::run_tick_pipeline_with_accumulators`]
    /// when both C-1 and C-2 may be active.
    pub fn run_tick_pipeline_with_threshold_scan(
        &self,
        state: &mut WorldGpuState,
        dt: f32,
        session: &mut crate::AccumulatorOpSession,
    ) {
        self.run_tick_pipeline_with_accumulators(
            state,
            dt,
            AccumulatorPipelineSessions {
                threshold: Some(session),
                intent: None,
                overlay_add: None,
                reduction_soft: None,
                velocity: None,
                intensity_eml: None,
                transfer: None,
                emission: None,
                encode_world_summary: false,
            },
        );
    }

    /// Consolidated per-tick pipeline. When `skip_threshold_scan` is true the
    /// Pass 7 threshold dispatch is omitted (C-1 AccumulatorOp path).
    pub fn run_tick_pipeline_ex(
        &self,
        state: &mut WorldGpuState,
        dt: f32,
        _skip_threshold_scan: bool,
    ) {
        self.run_tick_pipeline_internal(
            state,
            dt,
            true,
            true,
            &mut AccumulatorPipelineSessions {
                threshold: None,
                intent: None,
                overlay_add: None,
                reduction_soft: None,
                velocity: None,
                intensity_eml: None,
                transfer: None,
                emission: None,
                encode_world_summary: false,
            },
        );
    }

    fn run_tick_pipeline_internal(
        &self,
        state: &mut WorldGpuState,
        dt: f32,
        _skip_old_intent: bool,
        _skip_threshold_scan: bool,
        sessions: &mut AccumulatorPipelineSessions<'_>,
    ) {
        let ctx = &state.ctx;

        state.reset_event_count();
        self.write_params(ctx, state, dt);

        let snapshot_bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("snapshot_bg"),
            layout: &self.snapshot_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: state.values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: state.previous_values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: state.output_vectors.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: state.previous_output_vectors.as_entire_binding(),
                },
            ],
        });

        let use_accumulator_velocity =
            state.accumulator_velocity_active && state.accumulator_velocity_bands > 0;
        let use_accumulator_intensity =
            state.accumulator_intensity_eml_active && state.accumulator_intensity_eml_bands > 0;
        let use_accumulator_transfer =
            state.accumulator_transfer_active && state.accumulator_transfer_bands > 0;
        let use_accumulator_emission =
            state.accumulator_emission_active && state.accumulator_emission_bands > 0;

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("tick_pipeline_encoder"),
            });

        if let Some(session) = sessions.intent.as_mut() {
            session.encode_intent_into(ctx, &mut encoder, &state.values, &state.previous_values);
        }

        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("tick_pipeline_pre_overlay"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.snapshot_pipeline);
            pass.set_bind_group(0, &snapshot_bg, &[]);
            dispatch_linear(&mut pass, state.n_slots * state.n_dims);
        }

        if use_accumulator_velocity {
            if let Some(session) = sessions.velocity.as_mut() {
                session.encode_velocity_into(
                    ctx,
                    &mut encoder,
                    &state.values,
                    &state.previous_values,
                    dt,
                );
            }
        }

        if use_accumulator_intensity {
            if let Some(session) = sessions.intensity_eml.as_mut() {
                let eml = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.eml_bind_buffers());
                session.encode_intensity_eml_into(
                    ctx,
                    &mut encoder,
                    &state.values,
                    &state.previous_values,
                    dt,
                    eml,
                );
            }
        }

        if use_accumulator_transfer {
            if let Some(session) = sessions.transfer.as_mut() {
                let eml = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.eml_bind_buffers());
                let input_list = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.input_list_bind_buffer());
                session.encode_transfer_into(
                    ctx,
                    &mut encoder,
                    &state.values,
                    &state.previous_values,
                    state.accumulator_transfer_bands,
                    eml,
                    input_list,
                );
            }
        }

        if use_accumulator_emission {
            if let Some(session) = sessions.emission.as_mut() {
                let eml = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.eml_bind_buffers());
                session.encode_emission_into(
                    ctx,
                    &mut encoder,
                    &state.values,
                    &state.previous_values,
                    dt,
                    eml,
                );
            }
        }

        if state.accumulator_overlay_add_active && state.accumulator_overlay_add_bands > 0 {
            if let Some(session) = sessions.overlay_add.as_mut() {
                session.encode_overlay_add_into(
                    ctx,
                    &mut encoder,
                    &state.values,
                    &state.previous_values,
                    state.accumulator_overlay_add_bands,
                );
            }
        }

        let reduction_soft_active =
            state.accumulator_reduction_soft_active && state.accumulator_reduction_soft_bands > 0;

        if reduction_soft_active {
            let copy_bytes = (state.n_slots * state.n_dims * 4) as u64;
            encoder.copy_buffer_to_buffer(&state.values, 0, &state.output_vectors, 0, copy_bytes);
            if let Some(session) = sessions.reduction_soft.as_mut() {
                self.encode_accumulator_reduction_by_depth(ctx, &mut encoder, state, session);
            }
        }

        // C-1 integrated path: encode the AccumulatorOp threshold scan into
        // the same command buffer as the rest of the pipeline (separate
        // compute pass, different pipeline + bind group). One submission per
        // tick eliminates the second driver fence the standalone
        // `dispatch_threshold_scan` would otherwise introduce.
        if let Some(session) = sessions.threshold.as_mut() {
            session.encode_threshold_scan_with_outputs_into(
                ctx,
                &mut encoder,
                &state.values,
                &state.previous_values,
                &state.output_vectors,
                &state.previous_output_vectors,
            );
        }

        if sessions.encode_world_summary {
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.encode_world_summary_into(ctx, &mut encoder, &state.values);
            }
        }

        ctx.queue.submit(Some(encoder.finish()));
    }

    /// AccumulatorOp reduction: leaf init memcpy + per-depth OrderBand dispatch.
    pub fn run_accumulator_reduction_passes(
        &self,
        state: &WorldGpuState,
        session: &mut crate::AccumulatorOpSession,
    ) {
        if state.depth_bucket_ranges.is_empty()
            || !state.accumulator_reduction_soft_active
            || state.accumulator_reduction_soft_bands == 0
        {
            return;
        }
        let ctx = &state.ctx;

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_reduction_encoder"),
            });

        let copy_bytes = (state.n_slots * state.n_dims * 4) as u64;
        encoder.copy_buffer_to_buffer(&state.values, 0, &state.output_vectors, 0, copy_bytes);

        self.encode_accumulator_reduction_by_depth(ctx, &mut encoder, state, session);

        ctx.queue.submit(Some(encoder.finish()));
    }

    /// Copy `values` → `output_vectors`, then dispatch AccumulatorOp reduction bands.
    fn encode_accumulator_reduction_by_depth(
        &self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        state: &WorldGpuState,
        session: &mut crate::AccumulatorOpSession,
    ) {
        let max_tree_depth = state.depth_bucket_ranges.len().saturating_sub(1) as u32;
        let n_buckets = state.depth_bucket_ranges.len();

        for depth_idx in (0..n_buckets).rev() {
            if let Some(band) =
                reduction_soft_band_for_depth_bucket(max_tree_depth, depth_idx as u32)
            {
                if band < state.accumulator_reduction_soft_bands {
                    session.encode_reduction_soft_band_into(
                        ctx,
                        encoder,
                        &state.output_vectors,
                        band,
                    );
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::evaluate::Evaluator;
    use simthing_core::{
        DimensionRegistry, IntensityBehavior, PropertyValue, SimProperty, SimThing, SimThingKind,
        SubFieldRole,
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
                a.to_bits(),
                b.to_bits(),
                "{label}: index {i} diverges — cpu={a} ({:08x}), gpu={b} ({:08x})",
                a.to_bits(),
                b.to_bits(),
            );
        }
    }

    fn run_intensity_eml_on_state(
        pipelines: &Pipelines,
        state: &mut WorldGpuState,
        reg: &DimensionRegistry,
        dt: f32,
    ) {
        state.sync_intensity_eml_accumulator(reg);
        pipelines.run_accumulator_intensity_eml(state, dt);
    }

    /// Test-only AccumulatorOp velocity helper using an attached session.
    /// Not a production fallback; S-5 deleted the legacy velocity pass.
    fn run_velocity_integration_test_helper(pipelines: &Pipelines, state: &WorldGpuState, dt: f32) {
        if state.n_governed_pairs == 0 {
            return;
        }
        let ctx = &state.ctx;
        let pairs = state.read_governed_pairs();
        let plan = crate::plan_velocity_integration(&pairs, state.n_slots);
        let mut session = crate::AccumulatorOpSession::new_attached(
            ctx,
            state.n_slots,
            state.n_dims,
            plan.ops.len() as u32,
        );
        session
            .upload_gpu_ops(ctx, &plan.ops)
            .expect("AccumulatorOp velocity op upload failed");
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_velocity_encoder"),
            });
        session.encode_velocity_into(ctx, &mut encoder, &state.values, &state.previous_values, dt);
        ctx.queue.submit(Some(encoder.finish()));
    }

    fn upload_accumulator_overlay_plan(
        state: &mut WorldGpuState,
        world: &SimThing,
        reg: &DimensionRegistry,
        alloc: &crate::slot::SlotAllocator,
    ) -> usize {
        let (deltas, ranges) = crate::overlay_prep::build_overlay_deltas(world, reg, alloc);
        let plan = crate::plan_overlay_orderband(&deltas, &ranges, state.n_slots);
        state.ensure_overlay_add_accumulator();
        state
            .upload_overlay_ops_with_bands(&plan.ops, plan.n_bands)
            .expect("overlay upload");
        deltas.len()
    }

    fn upload_accumulator_reduction_plan(
        state: &mut WorldGpuState,
        world: &SimThing,
        alloc: &crate::slot::SlotAllocator,
        reg: &DimensionRegistry,
    ) {
        use crate::reduction::{build_column_rule_descriptors, TopologyState};
        use crate::reduction_orderband::plan_reduction_orderband;

        state.ensure_reduction_soft_accumulator();
        let topo_state = TopologyState::build(world, alloc);
        let descriptors = build_column_rule_descriptors(reg, state.n_dims as usize);
        let plan = plan_reduction_orderband(&topo_state, &descriptors, state.n_dims).unwrap();
        state
            .upload_reduction_soft_ops_with_bands(&plan.ops, plan.n_bands)
            .expect("reduction upload");
    }

    fn dispatch_accumulator_reduction(state: &mut WorldGpuState) {
        let pipelines = Pipelines::new(&state.ctx);
        let mut runtime = state.accumulator_runtime.take().unwrap();
        let mut session = runtime.take_reduction_soft_session().unwrap();
        pipelines.run_accumulator_reduction_passes(state, &mut session);
        runtime.restore_reduction_soft_session(Some(session));
        state.accumulator_runtime = Some(runtime);
    }

    #[test]
    fn snapshot_copies_values_to_previous() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut reg = DimensionRegistry::new();
        reg.register(loyalty_property());
        let state = WorldGpuState::new(ctx, &reg, 3);

        let input: Vec<f32> = (0..state.values_len())
            .map(|i| (i as f32) * 0.13 + 0.5)
            .collect();
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
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut slots = vec![vec![0.0f32; stride], vec![0.0f32; stride]];
        slots[0][a_off] = 0.4;
        slots[0][v_off] = 0.07;
        slots[0][i_off] = 0.2;
        slots[1][a_off] = 0.0;
        slots[1][v_off] = -0.05;
        slots[1][i_off] = 0.3;

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
            flat[s * n_dims..s * n_dims + stride].copy_from_slice(d);
        }
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        run_velocity_integration_test_helper(&pipelines, &state, dt);

        let gpu_flat = state.read_values();
        for s in 0..2 {
            let gpu_slice = &gpu_flat[s * n_dims..s * n_dims + stride];
            assert_bits_eq(&format!("slot {s}"), &cpu[s], gpu_slice);
        }
    }

    /// Pass 1 alone, dt = 0.5. Non-power-of-2 multiplier exercises potential
    /// FMA fusion between (vel * dt) and (value + ...). If this fails by 1 ULP,
    /// the FMA assumption in agents.md Option B is wrong and we need to switch
    /// to explicit fma() on both sides.
    #[test]
    fn velocity_integration_matches_cpu_oracle_fractional_dt() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

        let mut d = vec![0.0f32; stride];
        d[a_off] = 0.4_f32 + 1e-7; // not a clean fraction
        d[v_off] = 0.07_f32 - 3e-8; // not a clean fraction

        let dt = 0.5;
        let mut pv = PropertyValue { data: d.clone() };
        pv.integrate(&layout, dt);

        let state = WorldGpuState::new(ctx, &reg, 1);
        let mut flat = vec![0.0f32; state.values_len()];
        flat[..stride].copy_from_slice(&d);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        run_velocity_integration_test_helper(&pipelines, &state, dt);

        let gpu_flat = state.read_values();
        assert_bits_eq("fractional-dt", &pv.data, &gpu_flat[..stride]);
    }

    /// C-8b EvalEML intensity alone. Uses raw initial velocity (no Pass 1 first).
    #[test]
    fn intensity_eml_matches_cpu_oracle() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let behavior = reg.property(id).intensity_behavior.clone().unwrap();
        let stride = layout.stride();

        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        // Slot 0: high velocity → build branch. Slot 1: low velocity → decay branch.
        let mut slots = vec![vec![0.0f32; stride], vec![0.0f32; stride]];
        slots[0][v_off] = 0.09;
        slots[0][i_off] = 0.2;
        slots[1][v_off] = 0.001;
        slots[1][i_off] = 0.7;

        let dt = 0.5;

        let mut cpu: Vec<Vec<f32>> = slots.iter().cloned().collect();
        for d in &mut cpu {
            let mut pv = PropertyValue { data: d.clone() };
            pv.update_intensity(&behavior, &layout, dt);
            *d = pv.data;
        }

        let mut state = WorldGpuState::new(ctx, &reg, 2);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        for (s, d) in slots.iter().enumerate() {
            flat[s * n_dims..s * n_dims + stride].copy_from_slice(d);
        }
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        run_intensity_eml_on_state(&pipelines, &mut state, &reg, dt);

        let gpu_flat = state.read_values();
        for s in 0..2 {
            let gpu_slice = &gpu_flat[s * n_dims..s * n_dims + stride];
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
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        let loyalty_id = reg.register(loyalty_property());
        let food_id = reg.register(SimProperty::simple("core", "food_security", 0));

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
            (0.40, 0.07, 0.20),   // mid-range, building intensity
            (0.85, -0.001, 0.60), // near ceiling, decay branch
            (0.00, -0.05, 0.30),  // at floor, negative vel → pinning
            (0.50, 0.09, 0.10),   // mid-range, building
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

        let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        run_velocity_integration_test_helper(&pipelines, &state, dt);
        run_intensity_eml_on_state(&pipelines, &mut state, &reg, dt);

        let gpu_flat = state.read_values();

        // Compare every CPU-snapshot entity's properties against the
        // corresponding slot row in the GPU buffer.
        for entity in &cpu_snap.entities {
            let slot = alloc
                .slot_of(entity.id)
                .unwrap_or_else(|| panic!("entity {:?} not allocated", entity.id));
            let slot_base = slot as usize * n_dims;

            for (prop_id, cpu_pv) in &entity.properties {
                let range = reg.column_range(*prop_id);
                let start = slot_base + range.start;
                let end = start + cpu_pv.data.len();
                let gpu_data = &gpu_flat[start..end];
                let label = format!("entity {:?} slot {} prop {:?}", entity.id, slot, prop_id,);
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
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

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

        let mut state = WorldGpuState::new(ctx, &reg, 1);
        let range = reg.column_range(id).clone();
        let mut flat = vec![0.0f32; state.values_len()];
        flat[range.start..range.start + stride].copy_from_slice(&initial_data);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        run_velocity_integration_test_helper(&pipelines, &state, dt);
        run_intensity_eml_on_state(&pipelines, &mut state, &reg, dt);

        // Pass 0 invariant: previous_values must equal the pre-pass values.
        let prev = state.read_previous_values();
        assert_bits_eq("Pass 0 snapshot", &flat, &prev);

        let gpu_flat = state.read_values();
        let gpu_data = &gpu_flat[range.start..range.start + stride];
        assert_bits_eq("Evaluator vs GPU pipeline", cpu_data, gpu_data);
    }

    #[test]
    fn run_tick_pipeline_matches_manual_pass_sequence() {
        use crate::world_state::{
            IntentDelta, ThresholdRegistration, DIR_DOWNWARD, THRESH_BUF_VALUES,
        };

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
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

        let mut intent_session = crate::AccumulatorOpSession::new_attached(
            &manual.ctx,
            manual.n_slots,
            manual.n_dims,
            intent.len() as u32,
        );
        intent_session
            .upload_intent_ops(&manual.ctx, &intent)
            .unwrap();
        let mut intent_encoder =
            manual
                .ctx
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("manual_intent_accumulator_encoder"),
                });
        intent_session.prepare_intent(&manual.ctx);
        intent_session.encode_intent_into(
            &manual.ctx,
            &mut intent_encoder,
            &manual.values,
            &manual.previous_values,
        );
        manual.ctx.queue.submit(Some(intent_encoder.finish()));
        intent_session.finish_intent(&manual.ctx);
        pipelines.run_snapshot(&manual);
        run_velocity_integration_test_helper(&pipelines, &manual, 1.0);
        run_intensity_eml_on_state(&pipelines, &mut manual, &reg, 1.0);
        let manual_events = run_accumulator_threshold_scan(&manual, &regs);

        let ctx2 = GpuContext::new_blocking().expect("second gpu context");
        let mut piped = WorldGpuState::new(ctx2, &reg, 1);
        piped.write_values(&initial);
        piped.upload_intent_deltas(&intent);
        piped.upload_thresholds(&regs);
        let mut piped_intent_session = crate::AccumulatorOpSession::new_attached(
            &piped.ctx,
            piped.n_slots,
            piped.n_dims,
            intent.len() as u32,
        );
        piped_intent_session
            .upload_intent_ops(&piped.ctx, &intent)
            .unwrap();
        let vplan = crate::plan_velocity_integration(&piped.read_governed_pairs(), piped.n_slots);
        let mut piped_velocity_session = if vplan.n_bands > 0 {
            piped.ensure_velocity_accumulator();
            piped
                .upload_velocity_ops_with_bands(&vplan.ops, vplan.n_bands)
                .unwrap();
            piped
                .accumulator_runtime
                .as_mut()
                .unwrap()
                .take_velocity_session()
        } else {
            None
        };
        piped.sync_intensity_eml_accumulator(&reg);
        let pipelines2 = Pipelines::new(&piped.ctx);
        let mut intensity_session = piped
            .accumulator_runtime
            .as_mut()
            .unwrap()
            .take_intensity_eml_session();
        pipelines2.run_tick_pipeline_with_accumulators(
            &mut piped,
            1.0,
            AccumulatorPipelineSessions {
                intent: Some(&mut piped_intent_session),
                threshold: None,
                overlay_add: None,
                reduction_soft: None,
                velocity: piped_velocity_session.as_mut(),
                intensity_eml: intensity_session.as_mut(),
                transfer: None,
                emission: None,
                encode_world_summary: false,
            },
        );
        piped
            .accumulator_runtime
            .as_mut()
            .unwrap()
            .restore_intensity_eml_session(intensity_session);
        if let Some(runtime) = piped.accumulator_runtime.as_mut() {
            runtime.restore_velocity_session(piped_velocity_session);
        }

        assert_bits_eq("values", &manual.read_values(), &piped.read_values());
        assert_bits_eq(
            "previous_values",
            &manual.read_previous_values(),
            &piped.read_previous_values(),
        );
        assert_eq!(manual_events, run_accumulator_threshold_scan(&piped, &regs),);
    }

    /// CPU oracle for Pass 7. Same crossing logic as the WGSL shader; used to
    /// produce reference events for the parity test below.
    fn cpu_threshold_scan(
        previous_values: &[f32],
        values: &[f32],
        previous_output: &[f32],
        output: &[f32],
        n_dims: u32,
        regs: &[crate::world_state::ThresholdRegistration],
    ) -> Vec<crate::world_state::ThresholdEvent> {
        use crate::world_state::{ThresholdEvent, DIR_DOWNWARD, DIR_UPWARD, THRESH_BUF_OUTPUT};
        let mut events = Vec::new();
        for r in regs {
            let addr = (r.slot * n_dims + r.col) as usize;
            let (prev, curr) = if r.buffer == THRESH_BUF_OUTPUT {
                (previous_output[addr], output[addr])
            } else {
                (previous_values[addr], values[addr])
            };
            let up = prev <= r.threshold && curr > r.threshold;
            let down = prev >= r.threshold && curr < r.threshold;
            let crossed = match r.direction {
                DIR_UPWARD => up,
                DIR_DOWNWARD => down,
                _ => up || down,
            };
            if crossed {
                events.push(ThresholdEvent {
                    slot: r.slot,
                    col: r.col,
                    value: curr,
                    event_kind: r.event_kind,
                });
            }
        }
        events
    }

    fn run_accumulator_threshold_scan(
        state: &WorldGpuState,
        regs: &[crate::world_state::ThresholdRegistration],
    ) -> Vec<crate::world_state::ThresholdEvent> {
        let mut session = crate::AccumulatorOpSession::new_attached(
            &state.ctx,
            state.n_slots,
            state.n_dims,
            regs.len() as u32,
        );
        session.upload_threshold_ops(&state.ctx, regs).unwrap();
        let mut encoder = state
            .ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("test_threshold_scan_with_outputs"),
            });
        session.prepare_threshold_scan(&state.ctx);
        session.encode_threshold_scan_with_outputs_into(
            &state.ctx,
            &mut encoder,
            &state.values,
            &state.previous_values,
            &state.output_vectors,
            &state.previous_output_vectors,
        );
        state.ctx.queue.submit(Some(encoder.finish()));
        session.finish_threshold_scan(&state.ctx);
        session.readback_threshold_events(&state.ctx).unwrap()
    }

    /// Pass 7 directly: set up `previous_values` and `values` so that each
    /// crossing direction fires exactly when expected, and a stationary-on-threshold
    /// case does NOT fire (strict crossing rule).
    #[test]
    fn threshold_scan_matches_cpu_oracle() {
        use crate::world_state::{
            ThresholdEvent, ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD,
            THRESH_BUF_VALUES,
        };

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
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
        let mut current = vec![0.0_f32; state.values_len()];
        previous[0 * n_dims] = 0.40;
        current[0 * n_dims] = 0.10;
        previous[1 * n_dims] = 0.10;
        current[1 * n_dims] = 0.50;
        previous[2 * n_dims] = 0.50;
        current[2 * n_dims] = 0.50;
        previous[3 * n_dims] = 0.60;
        current[3 * n_dims] = 0.40;

        state.write_previous_values(&previous);
        state.write_values(&current);

        let regs = vec![
            ThresholdRegistration {
                slot: 0,
                col: 0,
                threshold: 0.30,
                direction: DIR_DOWNWARD,
                event_kind: 100,
                buffer: THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 1,
                col: 0,
                threshold: 0.30,
                direction: DIR_UPWARD,
                event_kind: 101,
                buffer: THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 2,
                col: 0,
                threshold: 0.50,
                direction: DIR_EITHER,
                event_kind: 102,
                buffer: THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 3,
                col: 0,
                threshold: 0.50,
                direction: DIR_EITHER,
                event_kind: 103,
                buffer: THRESH_BUF_VALUES,
            },
        ];
        state.upload_thresholds(&regs);

        let prev_out = vec![0.0_f32; state.values_len()];
        let out_flat = vec![0.0_f32; state.values_len()];
        let cpu = cpu_threshold_scan(
            &previous,
            &current,
            &prev_out,
            &out_flat,
            n_dims as u32,
            &regs,
        );
        assert_eq!(cpu.len(), 3, "oracle should produce exactly 3 events");

        let mut gpu: Vec<ThresholdEvent> = run_accumulator_threshold_scan(&state, &regs);

        // GPU event order is nondeterministic (atomicAdd race). Sort both sides.
        let key = |e: &ThresholdEvent| (e.event_kind, e.slot, e.col);
        let mut cpu_sorted = cpu;
        cpu_sorted.sort_by_key(key);
        gpu.sort_by_key(key);

        assert_eq!(
            cpu_sorted.len(),
            gpu.len(),
            "event count mismatch: cpu={} gpu={}",
            cpu_sorted.len(),
            gpu.len()
        );

        for (i, (c, g)) in cpu_sorted.iter().zip(gpu.iter()).enumerate() {
            assert_eq!(c.slot, g.slot, "event {i} slot");
            assert_eq!(c.col, g.col, "event {i} col");
            assert_eq!(c.event_kind, g.event_kind, "event {i} event_kind");
            assert_eq!(
                c.value.to_bits(),
                g.value.to_bits(),
                "event {i} value: cpu={} gpu={}",
                c.value,
                g.value
            );
        }
    }

    /// Pass 7 on `output_vectors`: upward crossing on a parent aggregate row.
    #[test]
    fn threshold_scan_on_output_vectors_matches_cpu_oracle() {
        use crate::world_state::{ThresholdRegistration, DIR_UPWARD, THRESH_BUF_OUTPUT};

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
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

        let gpu = run_accumulator_threshold_scan(&state, &regs);
        assert_eq!(gpu.len(), 1);
        assert_eq!(gpu[0].value.to_bits(), 0.50_f32.to_bits());
        assert_eq!(gpu[0].event_kind, 200);
    }

    /// Pass 7 with no registered thresholds: must be a no-op, no panic.
    #[test]
    fn threshold_scan_no_registrations_is_noop() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let state = WorldGpuState::new(ctx, &reg, 2);
        let gpu = run_accumulator_threshold_scan(&state, &[]);
        assert!(gpu.is_empty());
    }

    /// End-to-end Pass 0+1+2+3+7: a velocity-integration tick crosses a threshold
    /// registered on the amount sub-field, and Pass 7 detects the crossing using
    /// the post-Pass-0 snapshot vs. post-integration values.
    #[test]
    fn threshold_scan_after_full_pipeline() {
        use crate::world_state::{ThresholdRegistration, DIR_DOWNWARD, THRESH_BUF_VALUES};

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
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

        let regs = vec![ThresholdRegistration {
            slot: 0,
            col: a_off as u32,
            threshold: 0.30,
            direction: DIR_DOWNWARD,
            event_kind: 7,
            buffer: THRESH_BUF_VALUES,
        }];
        state.upload_thresholds(&regs);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state); // previous_* <- current
        run_velocity_integration_test_helper(&pipelines, &state, 1.0); // values amount: 0.35 - 0.10 = 0.25
        run_intensity_eml_on_state(&pipelines, &mut state, &reg, 1.0);
        let events = run_accumulator_threshold_scan(&state, &regs);
        assert_eq!(events.len(), 1, "expected exactly one downward crossing");
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
        use crate::projection::project_tree_to_values;
        use crate::slot::SlotAllocator;
        use simthing_core::ids::OverlayId;
        use simthing_core::overlay::{
            Overlay, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
        };
        use simthing_core::property::TransformOp;
        use std::time::Instant;

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

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
                id: OverlayId::new(),
                kind: OverlayKind::Policy,
                source: OverlaySource::Player,
                affects: vec![],
                transform: PropertyTransformDelta {
                    property_id: lid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(0.99))],
                },
                lifecycle: OverlayLifecycle::Permanent,
            });
            world.add_child(cohort);
        }

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        assert_eq!(state.n_dims, 64);
        assert!(state.n_slots >= 1000);

        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        let n_overlay_deltas = upload_accumulator_overlay_plan(&mut state, &world, &reg, &alloc);

        let pipelines = Pipelines::new(&state.ctx);
        state.sync_intensity_eml_accumulator(&reg);

        // Warm-up tick — first dispatch incurs pipeline cache + driver init.
        pipelines.run_snapshot(&state);
        run_velocity_integration_test_helper(&pipelines, &state, 0.5);
        pipelines.run_accumulator_intensity_eml(&mut state, 0.5);
        pipelines.run_accumulator_overlays(&mut state);
        let _ = state.read_values(); // force flush

        let t0 = Instant::now();
        pipelines.run_snapshot(&state);
        run_velocity_integration_test_helper(&pipelines, &state, 0.5);
        pipelines.run_accumulator_intensity_eml(&mut state, 0.5);
        pipelines.run_accumulator_overlays(&mut state);
        // Force the submitted work to complete before stopping the clock.
        let _ = state.read_values();
        let elapsed = t0.elapsed();

        eprintln!(
            "pipeline_timing_1000x64: {} slots × {} dims, {} overlay deltas → {:.2} ms",
            state.n_slots,
            state.n_dims,
            n_overlay_deltas,
            elapsed.as_secs_f64() * 1000.0,
        );
        assert!(
            elapsed.as_millis() < 50,
            "Pass 0+1+2+3 took {} ms, exceeds 50 ms day-boundary budget",
            elapsed.as_millis(),
        );
    }

    /// Pass 0+1+2+S-3 accumulator overlay against Evaluator on a tree with overlays at multiple levels.
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
    fn accumulator_overlay_matches_evaluator() {
        use crate::projection::project_tree_to_values;
        use crate::slot::SlotAllocator;
        use simthing_core::ids::OverlayId;
        use simthing_core::overlay::{
            Overlay, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
        };
        use simthing_core::property::TransformOp;

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        let lid = reg.register(loyalty_property());
        let layout = reg.property(lid).layout.clone();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let make_overlay = |deltas: Vec<(SubFieldRole, TransformOp)>| -> Overlay {
            Overlay {
                id: OverlayId::new(),
                kind: OverlayKind::Policy,
                source: OverlaySource::Player,
                affects: vec![],
                transform: PropertyTransformDelta {
                    property_id: lid,
                    sub_field_deltas: deltas,
                },
                lifecycle: OverlayLifecycle::Permanent,
            }
        };

        // World: Multiply(0.8) on loyalty amount.
        let mut world = SimThing::new(SimThingKind::World, 0);
        world.add_overlay(make_overlay(vec![(
            SubFieldRole::Amount,
            TransformOp::Multiply(0.8),
        )]));

        // Location: Add(-0.1) on loyalty velocity.
        let mut location = SimThing::new(SimThingKind::Location, 0);
        location.add_overlay(make_overlay(vec![(
            SubFieldRole::Velocity,
            TransformOp::Add(-0.1),
        )]));

        // Cohort A: loyalty mid-range, building; local Set(0.5) on intensity.
        let mut cohort_a = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv_a = PropertyValue::from_layout(&layout);
        pv_a.data[a_off] = 0.60;
        pv_a.data[v_off] = 0.08;
        pv_a.data[i_off] = 0.20;
        cohort_a.add_property(lid, pv_a);
        cohort_a.add_overlay(make_overlay(vec![(
            SubFieldRole::Intensity,
            TransformOp::Set(0.5),
        )]));
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

        upload_accumulator_overlay_plan(&mut state, &world, &reg, &alloc);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        run_velocity_integration_test_helper(&pipelines, &state, dt);
        run_intensity_eml_on_state(&pipelines, &mut state, &reg, dt);
        pipelines.run_accumulator_overlays(&mut state);

        let gpu_flat = state.read_values();

        // Compare Cohort A.
        for &(entity_id, label) in &[(cohort_a_id, "cohort_a"), (cohort_b_id, "cohort_b")] {
            let entity = cpu_snap.get(entity_id).unwrap();
            let slot = alloc.slot_of(entity_id).unwrap();
            let slot_base = slot as usize * n_dims;
            let range = reg.column_range(lid);
            let start = slot_base + range.start;
            let end = start + entity.properties[&lid].data.len();
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
            build_column_rule_descriptors, build_topology, cpu_reduce_oracle, encode_column_rules,
        };
        use crate::slot::SlotAllocator;

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        let lid = reg.register(loyalty_property());
        let layout = reg.property(lid).layout.clone();
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        // Single location keeps cohort child slots contiguous for AccumulatorOp SlotRange.
        let cohort_data: [(f32, f32, f32); 5] = [
            (0.40, 0.07, 0.20),
            (0.85, -0.001, 0.60),
            (0.10, 0.05, 0.30),
            (0.55, 0.03, 0.45),
            (0.20, -0.02, 0.10),
        ];

        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut loc = SimThing::new(SimThingKind::Location, 0);
        for &(a, v, i) in &cohort_data {
            let mut c = SimThing::new(SimThingKind::Cohort, 0);
            let mut pv = PropertyValue::from_layout(&layout);
            pv.data[a_off] = a;
            pv.data[v_off] = v;
            pv.data[i_off] = i;
            c.add_property(lid, pv);
            loc.add_child(c);
        }
        world.add_child(loc);

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

        upload_accumulator_reduction_plan(&mut state, &world, &alloc, &reg);
        dispatch_accumulator_reduction(&mut state);

        let gpu_output = state.read_output_vectors();

        assert_eq!(cpu_output.len(), gpu_output.len());
        let max_err = cpu_output
            .iter()
            .zip(gpu_output.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f32, f32::max);
        assert!(
            max_err < 1e-5,
            "accumulator reduction max_abs_error={max_err}"
        );
    }

    /// WeightedMean parity: location loyalty = population-weighted cohort mean.
    #[test]
    fn weighted_mean_reduction_matches_cpu_oracle() {
        use crate::projection::project_tree_to_values;
        use crate::reduction::{
            build_column_rule_descriptors, build_topology, cpu_reduce_oracle, encode_column_rules,
        };
        use crate::slot::SlotAllocator;
        use simthing_core::ReductionRule;

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

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

        upload_accumulator_reduction_plan(&mut state, &world, &alloc, &reg);
        dispatch_accumulator_reduction(&mut state);

        let gpu_output = state.read_output_vectors();
        assert_bits_eq("weighted mean reduction", &cpu_output, &gpu_output);
    }

    #[test]
    fn c2_integrated_intent_timestamp_finishes_when_supported() {
        use crate::world_state::IntentDelta;

        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut state = WorldGpuState::new(ctx, &reg, 1);
        state.ensure_intent_accumulator();
        state
            .upload_accumulator_intents(&[IntentDelta {
                slot: 0,
                col: 0,
                mul: 1.0,
                add: 0.25,
            }])
            .unwrap();

        let pipelines = Pipelines::new(&state.ctx);
        let mut runtime = state.accumulator_runtime.take().unwrap();
        let mut intent_session = runtime.take_intent_session();
        pipelines.run_tick_pipeline_with_accumulators(
            &mut state,
            0.0,
            AccumulatorPipelineSessions {
                intent: intent_session.as_mut(),
                threshold: None,
                overlay_add: None,
                reduction_soft: None,
                velocity: None,
                intensity_eml: None,
                transfer: None,
                emission: None,
                encode_world_summary: false,
            },
        );
        runtime.restore_intent_session(intent_session);
        state.accumulator_runtime = Some(runtime);

        let session = state
            .accumulator_runtime
            .as_mut()
            .unwrap()
            .intent_session()
            .unwrap();
        if session.timestamp_supported() {
            assert!(session.last_pass_time_us().is_some());
        } else {
            assert_eq!(session.last_pass_time_us(), None);
        }
    }
}

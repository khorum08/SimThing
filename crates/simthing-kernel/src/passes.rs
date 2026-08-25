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
                    resource: state.resolved.values().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: state.resolved.previous_values().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: state.resolved.output_vectors().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: state.resolved.previous_output_vectors().as_entire_binding(),
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
        let values = &state.resolved.values();
        let previous = &state.resolved.previous_values();
        if let Some(runtime) = state.accumulator_runtime.as_mut() {
            let mut session = runtime.take_intensity_eml_session();
            if let Some(session) = session.as_mut() {
                let eml = runtime.eml_program_table();
                session.encode_intensity_eml_into(ctx, &mut encoder, values, previous, dt, eml);
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
                &state.resolved.values(),
                &state.resolved.previous_values(),
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
                    resource: state.resolved.values().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: state.resolved.previous_values().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: state.resolved.output_vectors().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: state.resolved.previous_output_vectors().as_entire_binding(),
                },
            ],
        });

        let use_accumulator_velocity =
            state.accumulator_velocity_active && state.accumulator_velocity_bands > 0;
        let use_accumulator_intensity =
            state.accumulator_intensity_eml_active && state.accumulator_intensity_eml_bands > 0;
        let transfer_active =
            state.accumulator_transfer_active && state.accumulator_transfer_bands > 0;
        let emission_active =
            state.accumulator_emission_active && state.accumulator_emission_bands > 0;

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("tick_pipeline_encoder"),
            });

        if let Some(session) = sessions.intent.as_mut() {
            session.encode_intent_into(
                ctx,
                &mut encoder,
                &state.resolved.values(),
                &state.resolved.previous_values(),
            );
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
                    &state.resolved.values(),
                    &state.resolved.previous_values(),
                    dt,
                );
            }
        }

        if use_accumulator_intensity {
            if let Some(session) = sessions.intensity_eml.as_mut() {
                let eml = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.eml_program_table());
                session.encode_intensity_eml_into(
                    ctx,
                    &mut encoder,
                    &state.resolved.values(),
                    &state.resolved.previous_values(),
                    dt,
                    eml,
                );
            }
        }

        if transfer_active {
            if let Some(session) = sessions.transfer.as_mut() {
                let eml = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.eml_program_table());
                let input_list = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.input_list_bind_buffer());
                session.encode_transfer_into(
                    ctx,
                    &mut encoder,
                    &state.resolved.values(),
                    &state.resolved.previous_values(),
                    state.accumulator_transfer_bands,
                    eml,
                    input_list,
                );
            }
        }

        if emission_active {
            if let Some(session) = sessions.emission.as_mut() {
                let eml = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|r| r.eml_program_table());
                session.encode_emission_into(
                    ctx,
                    &mut encoder,
                    &state.resolved.values(),
                    &state.resolved.previous_values(),
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
                    &state.resolved.values(),
                    &state.resolved.previous_values(),
                    state.accumulator_overlay_add_bands,
                );
            }
        }

        let reduction_soft_active =
            state.accumulator_reduction_soft_active && state.accumulator_reduction_soft_bands > 0;

        if reduction_soft_active {
            let copy_bytes = (state.n_slots * state.n_dims * 4) as u64;
            encoder.copy_buffer_to_buffer(
                &state.resolved.values(),
                0,
                &state.resolved.output_vectors(),
                0,
                copy_bytes,
            );
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
            session.encode_threshold_scan_with_anchor_maintain_into(
                ctx,
                &mut encoder,
                &state.resolved.values(),
                &state.resolved.previous_values(),
                &state.resolved.output_vectors(),
                &state.resolved.previous_output_vectors(),
                Some((
                    &state.anchor_table,
                    state.n_anchor_rows,
                    state.anchor_table_generation,
                )),
            );
        }

        if sessions.encode_world_summary {
            let values = &state.resolved.values();
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.encode_world_summary_into(ctx, &mut encoder, values);
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
        encoder.copy_buffer_to_buffer(
            &state.resolved.values(),
            0,
            &state.resolved.output_vectors(),
            0,
            copy_bytes,
        );

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
                        &state.resolved.output_vectors(),
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
    use crate::{PackedAccumulatorUpload, PackedIntentUpload, PackedThresholdUpload};
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
    fn run_velocity_integration_test_helper(
        _pipelines: &Pipelines,
        state: &WorldGpuState,
        dt: f32,
    ) {
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
            .upload_packed_ops(
                ctx,
                &PackedAccumulatorUpload::from_gpu_ops(plan.ops.to_vec()).unwrap(),
            )
            .expect("AccumulatorOp velocity op upload failed");
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_velocity_encoder"),
            });
        session.encode_velocity_into(
            ctx,
            &mut encoder,
            &state.resolved.values(),
            &state.resolved.previous_values(),
            dt,
        );
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
    /// End-to-end parity: SlotAllocator + tree projection + Pass 0/1/2
    /// against simthing-core's `Evaluator` on a multi-node tree with multiple
    /// properties (one with intensity_behavior, one without). Verifies that
    /// the GPU pipeline driven from a real SimThing tree matches the CPU
    /// oracle bit-exactly across every (slot, property, column).
    /// Full Pass 0+1+2 pipeline matches simthing-core's `Evaluator` (the
    /// authoritative CPU oracle) on a single SimThing with one property
    /// and no overlays. Pass 0 result is verified via previous_values readback.
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
        crate::world_state::cpu_oracle_threshold_events(
            previous_values,
            values,
            previous_output,
            output,
            n_dims,
            regs,
            0,
        )
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
        session
            .upload_packed_threshold_ops(
                &state.ctx,
                &PackedThresholdUpload::from_registrations(regs).unwrap(),
            )
            .unwrap();
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
            &state.resolved.values(),
            &state.resolved.previous_values(),
            &state.resolved.output_vectors(),
            &state.resolved.previous_output_vectors(),
        );
        state.ctx.queue.submit(Some(encoder.finish()));
        session.finish_threshold_scan(&state.ctx);
        session.readback_threshold_events(&state.ctx).unwrap()
    }
}

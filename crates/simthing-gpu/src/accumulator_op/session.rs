//! Persistent GPU buffer ownership for AccumulatorOp v2 Pass B.

use std::sync::atomic::{AtomicBool, Ordering};

use bytemuck;
use simthing_core::AccumulatorOp;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor,
    Maintain, MapMode, PipelineLayoutDescriptor, QuerySet, QuerySetDescriptor, QueryType,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::context::GpuContext;
use crate::world_state::{IntentDelta, ThresholdEvent, ThresholdRegistration};

use super::encode::{threshold_registrations_to_ops, EncodeError};
use super::types::AccumulatorOpGpu;
use super::types::{
    AccumulatorSummaryParams, AccumulatorTickParams, EmissionRecord, EmissionRecordGpu,
    SlotSummary, SlotSummaryGpu, ThresholdEmission, ThresholdEmissionGpu,
    DEFAULT_EMISSION_CAPACITY, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};

pub const WORKGROUP_SIZE: u32 = 64;

static DEBUG_READBACK_ALLOWED: AtomicBool = AtomicBool::new(false);

/// Allow `readback_full()` without emitting a warning (tests only).
pub fn set_debug_readback_allowed(allowed: bool) {
    DEBUG_READBACK_ALLOWED.store(allowed, Ordering::Relaxed);
}

#[derive(Debug, thiserror::Error)]
pub enum AccumulatorOpSessionError {
    #[error("failed to encode AccumulatorOp: {0}")]
    Encode(#[from] EncodeError),
    #[error("no ops uploaded")]
    NoOps,
    #[error("GPU buffer read failed")]
    Readback,
    #[error("emission buffer overflow: count={count}, capacity={capacity}")]
    EmissionOverflow { count: u32, capacity: u32 },
    #[error("threshold emission buffer overflow: count={count}, capacity={capacity}")]
    ThresholdEmissionOverflow { count: u32, capacity: u32 },
}

/// GPU-resident AccumulatorOp session (B-2 bootstrap + C-1 threshold scan).
pub struct AccumulatorOpSession {
    n_slots: u32,
    n_dims: u32,
    n_ops: u32,
    emission_capacity: u32,
    threshold_emission_capacity: u32,

    op_buffer: Buffer,
    values_buffer: Buffer,
    previous_values_buffer: Buffer,
    summary_buffer: Buffer,
    emission_buffer: Buffer,
    emission_count: Buffer,
    threshold_emission_buffer: Buffer,
    threshold_emission_count: Buffer,

    tick_uniform: Buffer,
    summary_uniform: Buffer,

    execute_layout: BindGroupLayout,
    execute_pipeline: ComputePipeline,
    summary_layout: BindGroupLayout,
    summary_pipeline: ComputePipeline,

    timestamp_supported: bool,
    timestamp_query_set: Option<QuerySet>,
    timestamp_resolve_buffer: Option<Buffer>,
    timestamp_readback_buffer: Option<Buffer>,
    last_pass_time_us: Option<u64>,

    /// Per-registration sidecar populated by `upload_threshold_ops`.
    threshold_event_kinds: Vec<u32>,
}

impl AccumulatorOpSession {
    pub fn new(ctx: &GpuContext, n_slots: u32, n_dims: u32) -> Self {
        Self::with_emission_capacity(ctx, n_slots, n_dims, DEFAULT_EMISSION_CAPACITY)
    }

    /// Attach to an existing world GPU context (C-1 integrated threshold scan).
    pub fn new_attached(
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        threshold_emission_capacity: u32,
    ) -> Self {
        Self::build(
            ctx,
            n_slots,
            n_dims,
            DEFAULT_EMISSION_CAPACITY,
            threshold_emission_capacity,
        )
    }

    pub fn with_emission_capacity(
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) -> Self {
        Self::build(
            ctx,
            n_slots,
            n_dims,
            emission_capacity,
            DEFAULT_THRESHOLD_EMISSION_CAPACITY,
        )
    }

    fn build(
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
        threshold_emission_capacity: u32,
    ) -> Self {
        assert!(n_slots > 0 && n_dims > 0, "n_slots and n_dims must be > 0");

        let device = &ctx.device;
        let values_len = (n_slots * n_dims) as u64 * 4;
        let summary_len = (n_slots as u64) * std::mem::size_of::<SlotSummaryGpu>() as u64;
        let emission_len =
            (emission_capacity as u64) * std::mem::size_of::<EmissionRecordGpu>() as u64;
        let threshold_emission_len = (threshold_emission_capacity as u64)
            * std::mem::size_of::<ThresholdEmissionGpu>() as u64;

        let op_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_op_buffer"),
            size: 4096,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let values_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_values"),
            size: values_len,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let previous_values_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_previous_values"),
            size: values_len,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let summary_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_summary"),
            size: summary_len,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let emission_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_emissions"),
            size: emission_len.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let emission_count = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_emission_count"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let threshold_emission_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_threshold_emissions"),
            size: threshold_emission_len.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let threshold_emission_count = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_threshold_emission_count"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let tick_uniform = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_tick_params"),
            size: std::mem::size_of::<AccumulatorTickParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let summary_uniform = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_summary_params"),
            size: std::mem::size_of::<AccumulatorSummaryParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("accumulator_op"),
            source: ShaderSource::Wgsl(include_str!("../shaders/accumulator_op.wgsl").into()),
        });

        let execute_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("accumulator_execute_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, false),
                storage_entry(2, false),
                storage_entry(3, false),
                uniform_entry(4),
                storage_entry(5, true),
                storage_entry(6, false),
                storage_entry(7, false),
            ],
        });

        let execute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("accumulator_execute_pipeline"),
            layout: Some(
                &device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("accumulator_execute_pl"),
                    bind_group_layouts: &[&execute_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &shader,
            entry_point: "execute_ops",
            compilation_options: Default::default(),
            cache: None,
        });

        let summary_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("accumulator_summary_layout"),
            entries: &[
                storage_entry(0, false),
                storage_entry(1, false),
                uniform_entry(2),
            ],
        });

        let summary_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("accumulator_summary_pipeline"),
            layout: Some(
                &device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("accumulator_summary_pl"),
                    bind_group_layouts: &[&summary_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &shader,
            entry_point: "write_summaries",
            compilation_options: Default::default(),
            cache: None,
        });

        let (timestamp_query_set, timestamp_resolve_buffer, timestamp_readback_buffer) =
            if ctx.timestamp_supported() {
                let query_set = device.create_query_set(&QuerySetDescriptor {
                    label: Some("accumulator_timestamp_query_set"),
                    ty: QueryType::Timestamp,
                    count: 2,
                });
                let resolve = device.create_buffer(&BufferDescriptor {
                    label: Some("accumulator_timestamp_resolve"),
                    size: 16,
                    usage: BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });
                let readback = device.create_buffer(&BufferDescriptor {
                    label: Some("accumulator_timestamp_readback"),
                    size: 16,
                    usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                (Some(query_set), Some(resolve), Some(readback))
            } else {
                (None, None, None)
            };

        let session = Self {
            n_slots,
            n_dims,
            n_ops: 0,
            emission_capacity,
            threshold_emission_capacity,
            op_buffer,
            values_buffer,
            previous_values_buffer,
            summary_buffer,
            emission_buffer,
            emission_count,
            threshold_emission_buffer,
            threshold_emission_count,
            tick_uniform,
            summary_uniform,
            execute_layout,
            execute_pipeline,
            summary_layout,
            summary_pipeline,
            timestamp_supported: ctx.timestamp_supported(),
            timestamp_query_set,
            timestamp_resolve_buffer,
            timestamp_readback_buffer,
            last_pass_time_us: None,
            threshold_event_kinds: Vec::new(),
        };

        session.reset_emission_count(ctx);
        session.reset_threshold_emission_count(ctx);
        session
    }

    pub fn n_slots(&self) -> u32 {
        self.n_slots
    }

    pub fn n_dims(&self) -> u32 {
        self.n_dims
    }

    pub fn values_len(&self) -> usize {
        (self.n_slots * self.n_dims) as usize
    }

    pub fn emission_capacity(&self) -> u32 {
        self.emission_capacity
    }

    pub fn threshold_emission_capacity(&self) -> u32 {
        self.threshold_emission_capacity
    }

    /// Whether this session was created with GPU timestamp query support.
    pub fn timestamp_supported(&self) -> bool {
        self.timestamp_supported
    }

    /// Duration of the last `execute_ops` pass in microseconds, if timestamp queries are supported.
    pub fn last_pass_time_us(&self) -> Option<u64> {
        self.last_pass_time_us
    }

    /// Upload initial or post-tick values matrix (row-major slot × dims).
    pub fn upload_values(&self, ctx: &GpuContext, values: &[f32]) {
        assert_eq!(values.len(), self.values_len());
        ctx.queue
            .write_buffer(&self.values_buffer, 0, bytemuck::cast_slice(values));
    }

    /// Upload previous-tick values for threshold crossing tests.
    pub fn upload_previous_values(&self, ctx: &GpuContext, values: &[f32]) {
        assert_eq!(values.len(), self.values_len());
        ctx.queue.write_buffer(
            &self.previous_values_buffer,
            0,
            bytemuck::cast_slice(values),
        );
    }

    /// Upload AccumulatorOp registrations after bootstrap subset + contention validation.
    pub fn upload_ops(
        &mut self,
        ctx: &GpuContext,
        ops: &[AccumulatorOp],
    ) -> Result<(), AccumulatorOpSessionError> {
        self.threshold_event_kinds.clear();
        let gpu_ops = AccumulatorOpGpu::encode_bootstrap_set(ops)?;

        let byte_len = gpu_ops.len() * std::mem::size_of::<AccumulatorOpGpu>();
        if self.op_buffer.size() < byte_len as u64 {
            self.op_buffer = ctx.device.create_buffer(&BufferDescriptor {
                label: Some("accumulator_op_buffer"),
                size: byte_len.max(4096) as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        ctx.queue
            .write_buffer(&self.op_buffer, 0, bytemuck::cast_slice(&gpu_ops));
        self.n_ops = gpu_ops.len() as u32;
        Ok(())
    }

    /// Upload threshold-gated EmitEvent ops from Pass 7 registrations (C-1).
    pub fn upload_threshold_ops(
        &mut self,
        ctx: &GpuContext,
        regs: &[ThresholdRegistration],
    ) -> Result<(), AccumulatorOpSessionError> {
        let (ops, event_kinds) = threshold_registrations_to_ops(regs)?;
        let gpu_ops = AccumulatorOpGpu::encode_threshold_set(&ops)?;

        let byte_len = gpu_ops.len() * std::mem::size_of::<AccumulatorOpGpu>();
        if self.op_buffer.size() < byte_len as u64 {
            self.op_buffer = ctx.device.create_buffer(&BufferDescriptor {
                label: Some("accumulator_op_buffer"),
                size: byte_len.max(4096) as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        ctx.queue
            .write_buffer(&self.op_buffer, 0, bytemuck::cast_slice(&gpu_ops));
        self.n_ops = gpu_ops.len() as u32;
        self.threshold_event_kinds = event_kinds;
        Ok(())
    }

    /// Upload folded intent deltas as affine AccumulatorOp registrations (C-2).
    pub fn upload_intent_ops(
        &mut self,
        ctx: &GpuContext,
        deltas: &[IntentDelta],
    ) -> Result<(), AccumulatorOpSessionError> {
        self.threshold_event_kinds.clear();
        if deltas.is_empty() {
            self.n_ops = 0;
            return Ok(());
        }
        let gpu_ops = AccumulatorOpGpu::encode_intent_deltas(deltas)?;

        let byte_len = gpu_ops.len() * std::mem::size_of::<AccumulatorOpGpu>();
        if self.op_buffer.size() < byte_len as u64 {
            self.op_buffer = ctx.device.create_buffer(&BufferDescriptor {
                label: Some("accumulator_op_buffer"),
                size: byte_len.max(4096) as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        ctx.queue
            .write_buffer(&self.op_buffer, 0, bytemuck::cast_slice(&gpu_ops));
        self.n_ops = gpu_ops.len() as u32;
        Ok(())
    }

    /// Dispatch Pass B for one OrderBand, then refresh per-slot summaries.
    pub fn tick(&mut self, ctx: &GpuContext, band: u32) -> Result<(), AccumulatorOpSessionError> {
        if self.n_ops == 0 {
            return Err(AccumulatorOpSessionError::NoOps);
        }

        self.reset_emission_count(ctx);
        self.reset_threshold_emission_count(ctx);
        self.last_pass_time_us = None;
        self.write_tick_uniform(ctx, band);

        let execute_bind_group = self.create_execute_bind_group(
            ctx,
            &self.values_buffer,
            &self.previous_values_buffer,
        );

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_tick_encoder"),
            });

        let timestamp_writes = self.timestamp_query_set.as_ref().map(|query_set| {
            wgpu::ComputePassTimestampWrites {
                query_set,
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: Some(1),
            }
        });

        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_execute_pass"),
                timestamp_writes,
            });
            pass.set_pipeline(&self.execute_pipeline);
            pass.set_bind_group(0, &execute_bind_group, &[]);
            let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
            pass.dispatch_workgroups(groups, 1, 1);
        }

        if let (Some(query_set), Some(resolve), Some(readback)) = (
            self.timestamp_query_set.as_ref(),
            self.timestamp_resolve_buffer.as_ref(),
            self.timestamp_readback_buffer.as_ref(),
        ) {
            encoder.resolve_query_set(query_set, 0..2, resolve, 0);
            encoder.copy_buffer_to_buffer(resolve, 0, readback, 0, 16);
        }

        self.dispatch_write_summaries(ctx, &mut encoder);
        ctx.queue.submit(Some(encoder.finish()));

        self.read_execute_pass_timestamp(ctx);
        Ok(())
    }

    /// Dispatch threshold scan against world GPU value buffers (C-1 integrated path).
    ///
    /// Convenience wrapper that owns its command buffer + submit. Use
    /// [`Self::encode_threshold_scan_into`] when batching with other GPU work
    /// in the same submission (see `Pipelines::run_tick_pipeline_with_threshold_scan`)
    /// — fewer submissions, fewer driver fences, less per-tick overhead.
    pub fn dispatch_threshold_scan(
        &mut self,
        ctx: &GpuContext,
        values: &Buffer,
        previous_values: &Buffer,
    ) -> Result<(), AccumulatorOpSessionError> {
        if self.n_ops == 0 {
            return Ok(());
        }

        self.reset_threshold_emission_count(ctx);
        self.last_pass_time_us = None;
        self.write_tick_uniform(ctx, 0);

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_threshold_scan_encoder"),
            });
        self.encode_threshold_scan_into(ctx, &mut encoder, values, previous_values);
        ctx.queue.submit(Some(encoder.finish()));
        self.read_execute_pass_timestamp(ctx);
        Ok(())
    }

    /// Encode the threshold-scan compute pass into the caller's existing
    /// command encoder. The caller is responsible for `queue.submit` and for
    /// calling [`Self::reset_threshold_emission_count`] before the encoder
    /// runs (typically just before encoding world pipeline passes that share
    /// the submission). `read_execute_pass_timestamp` must be called after
    /// the submission to populate `last_pass_time_us`.
    ///
    /// Returns immediately when no threshold ops are registered.
    pub fn encode_threshold_scan_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
    ) {
        if self.n_ops == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let execute_bind_group =
            self.create_execute_bind_group(ctx, values, previous_values);

        let timestamp_writes = self.timestamp_query_set.as_ref().map(|query_set| {
            wgpu::ComputePassTimestampWrites {
                query_set,
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: Some(1),
            }
        });

        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_threshold_scan_pass"),
                timestamp_writes,
            });
            pass.set_pipeline(&self.execute_pipeline);
            pass.set_bind_group(0, &execute_bind_group, &[]);
            let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
            pass.dispatch_workgroups(groups, 1, 1);
        }

        if let (Some(query_set), Some(resolve), Some(readback)) = (
            self.timestamp_query_set.as_ref(),
            self.timestamp_resolve_buffer.as_ref(),
            self.timestamp_readback_buffer.as_ref(),
        ) {
            encoder.resolve_query_set(query_set, 0..2, resolve, 0);
            encoder.copy_buffer_to_buffer(resolve, 0, readback, 0, 16);
        }
    }

    /// Prepare the session for a batched intent pass in the caller's encoder.
    pub fn prepare_intent(&self, ctx: &GpuContext) {
        if self.n_ops == 0 {
            return;
        }
        self.write_tick_uniform(ctx, 0);
    }

    /// Encode the C-2 affine intent pass into the caller's command encoder.
    /// Runs before snapshot in the consolidated tick pipeline. Returns
    /// immediately when no intent ops are registered.
    pub fn encode_intent_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
    ) {
        if self.n_ops == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let execute_bind_group =
            self.create_execute_bind_group(ctx, values, previous_values);

        let timestamp_writes = self.timestamp_query_set.as_ref().map(|query_set| {
            wgpu::ComputePassTimestampWrites {
                query_set,
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: Some(1),
            }
        });

        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_intent_pass"),
                timestamp_writes,
            });
            pass.set_pipeline(&self.execute_pipeline);
            pass.set_bind_group(0, &execute_bind_group, &[]);
            let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
            pass.dispatch_workgroups(groups, 1, 1);
        }

        if let (Some(query_set), Some(resolve), Some(readback)) = (
            self.timestamp_query_set.as_ref(),
            self.timestamp_resolve_buffer.as_ref(),
            self.timestamp_readback_buffer.as_ref(),
        ) {
            encoder.resolve_query_set(query_set, 0..2, resolve, 0);
            encoder.copy_buffer_to_buffer(resolve, 0, readback, 0, 16);
        }
    }

    /// Prepare the session for a batched threshold scan in the caller's
    /// encoder: reset the per-tick atomic counter and write the tick uniform.
    /// Pair with [`Self::encode_threshold_scan_into`] and call
    /// [`Self::read_execute_pass_timestamp`] after `queue.submit`.
    pub fn prepare_threshold_scan(&mut self, ctx: &GpuContext) {
        if self.n_ops == 0 {
            return;
        }
        self.reset_threshold_emission_count(ctx);
        self.write_tick_uniform(ctx, 0);
    }

    /// Public timestamp readback hook for callers that drove
    /// `encode_threshold_scan_into` directly.
    pub fn finish_threshold_scan(&mut self, ctx: &GpuContext) {
        self.read_execute_pass_timestamp(ctx);
    }

    /// Finish the intent timestamp query if timestamps are supported.
    /// Call immediately after the submission that drove `encode_intent_into`.
    pub fn finish_intent(&mut self, ctx: &GpuContext) {
        self.read_execute_pass_timestamp(ctx);
    }

    fn write_tick_uniform(&self, ctx: &GpuContext, band: u32) {
        let tick_params = AccumulatorTickParams {
            n_ops: self.n_ops,
            current_band: band,
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            emission_capacity: self.emission_capacity,
            threshold_emission_capacity: self.threshold_emission_capacity,
            _pad0: 0,
            _pad1: 0,
        };
        ctx.queue.write_buffer(
            &self.tick_uniform,
            0,
            bytemuck::bytes_of(&tick_params),
        );
    }

    fn create_execute_bind_group(
        &self,
        ctx: &GpuContext,
        values: &Buffer,
        previous_values: &Buffer,
    ) -> wgpu::BindGroup {
        ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("accumulator_execute_bg"),
            layout: &self.execute_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.op_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.emission_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.emission_count.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: self.tick_uniform.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: previous_values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: self.threshold_emission_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: self.threshold_emission_count.as_entire_binding(),
                },
            ],
        })
    }

    /// Provisional B-1/B-2 summary readback tier (checksum-only; final shape is B-4).
    pub fn readback_summary(&self, ctx: &GpuContext) -> Result<Vec<SlotSummary>, AccumulatorOpSessionError> {
        let bytes = self.read_buffer_bytes(ctx, &self.summary_buffer);
        let gpu: &[SlotSummaryGpu] = bytemuck::cast_slice(&bytes);
        Ok(gpu
            .iter()
            .map(|s| SlotSummary {
                slot: s.slot,
                checksum: s.checksum,
            })
            .collect())
    }

    /// Read compact threshold crossing records written this tick (C-1).
    pub fn readback_threshold_emissions(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<ThresholdEmission>, AccumulatorOpSessionError> {
        let count = self.read_threshold_emission_count(ctx)?;
        if count == 0 {
            return Ok(Vec::new());
        }
        if count > self.threshold_emission_capacity {
            return Err(AccumulatorOpSessionError::ThresholdEmissionOverflow {
                count,
                capacity: self.threshold_emission_capacity,
            });
        }
        let used =
            (count as u64) * std::mem::size_of::<ThresholdEmissionGpu>() as u64;
        let bytes = self.read_buffer_bytes_range(ctx, &self.threshold_emission_buffer, 0, used);
        let gpu: &[ThresholdEmissionGpu] = bytemuck::cast_slice(&bytes);
        Ok(gpu
            .iter()
            .map(|r| ThresholdEmission {
                reg_idx: r.reg_idx,
                slot: r.slot,
                col: r.col,
                value: r.value,
            })
            .collect())
    }

    /// Reconstruct Pass 7 `ThresholdEvent`s from compact threshold emissions.
    pub fn readback_threshold_events(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<ThresholdEvent>, AccumulatorOpSessionError> {
        let emissions = self.readback_threshold_emissions(ctx)?;
        Ok(emissions
            .into_iter()
            .map(|e| ThresholdEvent {
                slot: e.slot,
                col: e.col,
                value: e.value,
                event_kind: self.threshold_event_kinds[e.reg_idx as usize],
            })
            .collect())
    }

    /// Read compact emission records written by EmitEvent ops this tick.
    pub fn readback_emissions(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<EmissionRecord>, AccumulatorOpSessionError> {
        let count = self.read_emission_count(ctx)?;
        if count == 0 {
            return Ok(Vec::new());
        }
        if count > self.emission_capacity {
            return Err(AccumulatorOpSessionError::EmissionOverflow {
                count,
                capacity: self.emission_capacity,
            });
        }
        let used = (count as u64) * std::mem::size_of::<EmissionRecordGpu>() as u64;
        let bytes = self.read_buffer_bytes_range(ctx, &self.emission_buffer, 0, used);
        let gpu: &[EmissionRecordGpu] = bytemuck::cast_slice(&bytes);
        Ok(gpu
            .iter()
            .map(|r| EmissionRecord {
                reg_idx: r.reg_idx,
                emit_count: r.emit_count,
            })
            .collect())
    }

    /// Full values buffer readback — debug only unless explicitly allowed.
    ///
    /// The values buffer is stored as `atomic<i32>` on GPU (same bits as f32).
    /// Readback reinterprets i32 bits as f32 via cast_slice — this is exact,
    /// not an approximation.
    pub fn readback_full(&self, ctx: &GpuContext) -> Result<Vec<f32>, AccumulatorOpSessionError> {
        if !DEBUG_READBACK_ALLOWED.load(Ordering::Relaxed) {
            eprintln!(
                "warning: AccumulatorOpSession::readback_full() called outside test mode"
            );
        }
        Ok(self.read_buffer_f32(ctx, &self.values_buffer))
    }

    fn reset_emission_count(&self, ctx: &GpuContext) {
        ctx.queue
            .write_buffer(&self.emission_count, 0, &0u32.to_le_bytes());
    }

    fn reset_threshold_emission_count(&self, ctx: &GpuContext) {
        ctx.queue.write_buffer(
            &self.threshold_emission_count,
            0,
            &0u32.to_le_bytes(),
        );
    }

    fn read_emission_count(&self, ctx: &GpuContext) -> Result<u32, AccumulatorOpSessionError> {
        let bytes = self.read_buffer_bytes(ctx, &self.emission_count);
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_threshold_emission_count(
        &self,
        ctx: &GpuContext,
    ) -> Result<u32, AccumulatorOpSessionError> {
        let bytes = self.read_buffer_bytes(ctx, &self.threshold_emission_count);
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_execute_pass_timestamp(&mut self, ctx: &GpuContext) {
        // B-3 reads timestamps synchronously for testability.
        // Later production profiling can batch or sample timestamp readbacks.
        let Some(readback) = self.timestamp_readback_buffer.as_ref() else {
            self.last_pass_time_us = None;
            return;
        };

        ctx.device.poll(Maintain::Wait);
        let slice = readback.slice(..);
        slice.map_async(MapMode::Read, |_| {});
        ctx.device.poll(Maintain::Wait);
        let mapped = slice.get_mapped_range();
        if mapped.len() < 16 {
            self.last_pass_time_us = None;
            return;
        }

        let stamps: [u64; 2] = bytemuck::cast_slice(&mapped[..16]).try_into().unwrap();
        drop(mapped);
        readback.unmap();

        if stamps[1] >= stamps[0] {
            let delta = stamps[1] - stamps[0];
            let ns = delta as f64 * ctx.timestamp_period_ns() as f64;
            self.last_pass_time_us = Some((ns / 1000.0).round() as u64);
        } else {
            self.last_pass_time_us = None;
        }
    }

    fn dispatch_write_summaries(
        &self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let summary_params = AccumulatorSummaryParams {
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            _pad0: 0,
            _pad1: 0,
        };
        ctx.queue.write_buffer(
            &self.summary_uniform,
            0,
            bytemuck::bytes_of(&summary_params),
        );

        let summary_bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("accumulator_summary_bg"),
            layout: &self.summary_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.values_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.summary_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.summary_uniform.as_entire_binding(),
                },
            ],
        });

        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("accumulator_summary_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.summary_pipeline);
        pass.set_bind_group(0, &summary_bind_group, &[]);
        let groups = self.n_slots.div_ceil(WORKGROUP_SIZE);
        pass.dispatch_workgroups(groups, 1, 1);
    }

    fn read_buffer_f32(&self, ctx: &GpuContext, buf: &Buffer) -> Vec<f32> {
        let bytes = self.read_buffer_bytes(ctx, buf);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn read_buffer_bytes(&self, ctx: &GpuContext, buf: &Buffer) -> Vec<u8> {
        self.read_buffer_bytes_range(ctx, buf, 0, buf.size())
    }

    fn read_buffer_bytes_range(
        &self,
        ctx: &GpuContext,
        buf: &Buffer,
        offset: u64,
        size: u64,
    ) -> Vec<u8> {
        let staging = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_staging_read"),
            size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_read_encoder"),
            });
        encoder.copy_buffer_to_buffer(buf, offset, &staging, 0, size);
        ctx.queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        slice.map_async(MapMode::Read, |_| {});
        ctx.device.poll(Maintain::Wait);
        let mapped = slice.get_mapped_range();
        mapped.to_vec()
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
    use simthing_core::{
        AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec,
    };

    use crate::accumulator_op::encode::EncodeError;
    use crate::accumulator_op::{
        execute_ops_cpu, execute_threshold_ops_cpu, set_debug_readback_allowed,
        summaries_from_values, threshold_registrations_to_ops,
    };
    use crate::context::GpuContext;

    use super::*;

    fn gpu_session(n_slots: u32, n_dims: u32) -> (GpuContext, AccumulatorOpSession) {
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let session = AccumulatorOpSession::new(&ctx, n_slots, n_dims);
        (ctx, session)
    }

    fn bootstrap_ops() -> Vec<AccumulatorOp> {
        vec![
            AccumulatorOp {
                source: SourceSpec::SlotValue { slot: 0, col: 0 },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Constant(3.0),
                consume: ConsumeMode::SubtractFromSource,
                targets: vec![(1, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(5.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(2, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotRange { start: 3, count: 2 },
                combine: CombineFn::Sum,
                gate: GateSpec::OrderBand(1),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(5, 0)],
            },
        ]
    }

    fn bootstrap_initial_values(n_slots: u32, n_dims: u32) -> Vec<f32> {
        let mut values = vec![0.0f32; (n_slots * n_dims) as usize];
        values[0] = 100.0; // slot 0 col 0 pool
        values[3 * n_dims as usize] = 4.0;
        values[4 * n_dims as usize] = 6.0;
        values
    }

    #[test]
    fn session_readback_summary_matches_cpu_oracle() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let n_slots = 8u32;
        let n_dims = 2u32;
        let ops = bootstrap_ops();
        let initial = bootstrap_initial_values(n_slots, n_dims);

        let mut expected = initial.clone();
        execute_ops_cpu(&mut expected, &ops, 0, n_dims).unwrap();
        execute_ops_cpu(&mut expected, &ops, 1, n_dims).unwrap();
        let expected_summaries = summaries_from_values(&expected, n_slots, n_dims);

        let mut session = AccumulatorOpSession::new(&ctx, n_slots, n_dims);
        session.upload_values(&ctx, &initial);
        session.upload_ops(&ctx, &ops).unwrap();
        session.tick(&ctx, 0).unwrap();
        session.tick(&ctx, 1).unwrap();

        let gpu_summaries = session.readback_summary(&ctx).unwrap();
        assert_eq!(gpu_summaries, expected_summaries);

        let gpu_values = session.readback_full(&ctx).unwrap();
        assert_eq!(gpu_values, expected);
    }

    #[test]
    fn accumulator_scale_constant_zero_writes_zero() {
        set_debug_readback_allowed(true);
        let n_dims = 1u32;
        let mut values = vec![10.0, 7.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(0.0),
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };

        let (ctx, mut session) = gpu_session(2, n_dims);
        session.upload_values(&ctx, &values);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
        session.tick(&ctx, 0).unwrap();
        values = session.readback_full(&ctx).unwrap();
        assert_eq!(values[1], 7.0);
    }

    #[test]
    fn accumulator_transfer_clamps_to_available_source() {
        set_debug_readback_allowed(true);
        let n_dims = 1u32;
        let (ctx, mut session) = gpu_session(2, n_dims);

        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(10.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        session.upload_values(&ctx, &values);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
        session.tick(&ctx, 0).unwrap();
        values = session.readback_full(&ctx).unwrap();
        assert_eq!(values, vec![0.0, 5.0]);

        let mut values = vec![10.0, 0.0];
        let op_small = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(3.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        session.upload_values(&ctx, &values);
        session.upload_ops(&ctx, std::slice::from_ref(&op_small)).unwrap();
        session.tick(&ctx, 0).unwrap();
        values = session.readback_full(&ctx).unwrap();
        assert_eq!(values, vec![7.0, 3.0]);
    }

    #[test]
    fn accumulator_transfer_rejects_negative_requested_transfer() {
        set_debug_readback_allowed(true);
        let n_dims = 1u32;
        let (ctx, mut session) = gpu_session(2, n_dims);
        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(-3.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        session.upload_values(&ctx, &values);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
        session.tick(&ctx, 0).unwrap();
        values = session.readback_full(&ctx).unwrap();
        assert_eq!(values, vec![5.0, 0.0]);
    }

    #[test]
    fn upload_ops_allows_duplicate_target_same_band() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(2.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(4, 1);
        session.upload_ops(&ctx, &ops).unwrap();
    }

    #[test]
    fn upload_ops_rejects_duplicate_consumed_source_same_band() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::SlotValue { slot: 0, col: 0 },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Constant(1.0),
                consume: ConsumeMode::SubtractFromSource,
                targets: vec![(1, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotValue { slot: 0, col: 0 },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Constant(2.0),
                consume: ConsumeMode::SubtractFromSource,
                targets: vec![(2, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(4, 1);
        let err = session.upload_ops(&ctx, &ops).unwrap_err();
        assert!(matches!(
            err,
            AccumulatorOpSessionError::Encode(EncodeError::BootstrapContention {
                band: 0,
                slot: 0,
                col: 0
            })
        ));
    }

    #[test]
    fn upload_ops_allows_same_target_in_different_order_bands() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(2.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(1),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(4, 1);
        session.upload_ops(&ctx, &ops).unwrap();
    }

    #[test]
    fn upload_ops_allows_always_and_orderband_same_target() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(2.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(1),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(4, 1);
        session.upload_ops(&ctx, &ops).unwrap();
    }

    #[test]
    fn upload_ops_allows_always_consume_and_orderband_write_same_cell() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::SlotValue { slot: 0, col: 0 },
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Constant(1.0),
                consume: ConsumeMode::SubtractFromSource,
                targets: vec![(1, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(2.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(1),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(0, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(4, 2);
        session.upload_ops(&ctx, &ops).unwrap();
    }

    #[test]
    fn upload_ops_allows_orderband_consume_and_always_write_same_cell() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(2.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(0, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotValue { slot: 0, col: 0 },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(1),
                scale: ScaleSpec::Constant(1.0),
                consume: ConsumeMode::SubtractFromSource,
                targets: vec![(1, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(4, 2);
        session.upload_ops(&ctx, &ops).unwrap();
    }

    #[test]
    fn upload_ops_allows_two_always_writers_same_cell() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(2.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(4, 1);
        session.upload_ops(&ctx, &ops).unwrap();
    }

    #[test]
    fn atomic_same_cell_add_conserves_total() {
        set_debug_readback_allowed(true);
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(0, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(0, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(0, 0)],
            },
        ];
        let (ctx, mut session) = gpu_session(1, 1);
        session.upload_values(&ctx, &[0.0]);
        session.upload_ops(&ctx, &ops).unwrap();
        session.tick(&ctx, 0).unwrap();
        let values = session.readback_full(&ctx).unwrap();
        assert_eq!(values[0], 3.0);
    }

    #[test]
    fn summary_checksum_stable_across_two_ticks() {
        set_debug_readback_allowed(true);
        let noop = AccumulatorOp {
            source: SourceSpec::Constant(0.0),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(0, 0)],
        };
        let (ctx, mut session) = gpu_session(2, 1);
        session.upload_values(&ctx, &[10.0, 20.0]);
        session.upload_ops(&ctx, std::slice::from_ref(&noop)).unwrap();
        session.tick(&ctx, 0).unwrap();
        let first = session.readback_summary(&ctx).unwrap();
        session.tick(&ctx, 0).unwrap();
        let second = session.readback_summary(&ctx).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn summary_checksum_changes_when_values_change() {
        set_debug_readback_allowed(true);
        let noop = AccumulatorOp {
            source: SourceSpec::Constant(0.0),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(0, 0)],
        };
        let (ctx, mut session) = gpu_session(1, 1);
        session.upload_values(&ctx, &[10.0]);
        session.upload_ops(&ctx, std::slice::from_ref(&noop)).unwrap();
        session.tick(&ctx, 0).unwrap();
        let pre = session.readback_summary(&ctx).unwrap()[0].checksum;

        let add_op = AccumulatorOp {
            source: SourceSpec::Constant(5.0),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(0, 0)],
        };
        session.upload_ops(&ctx, std::slice::from_ref(&add_op)).unwrap();
        session.tick(&ctx, 0).unwrap();
        let post = session.readback_summary(&ctx).unwrap()[0].checksum;
        assert_ne!(pre, post);
    }

    #[test]
    fn threshold_none_writes_target_on_crossing_not_event() {
        use simthing_core::ThresholdDirection;

        set_debug_readback_allowed(true);
        let (ctx, mut session) = gpu_session(2, 1);
        session.upload_values(&ctx, &[0.5, 0.0]);
        session.upload_previous_values(&ctx, &[0.2, 0.0]);

        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: 0.3,
                direction: ThresholdDirection::Upward,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
        session.tick(&ctx, 0).unwrap();

        let values = session.readback_full(&ctx).unwrap();
        assert!(
            (values[1] - 0.5).abs() < 1e-5,
            "target not written: {}",
            values[1]
        );
        let emissions = session.readback_threshold_emissions(&ctx).unwrap();
        assert!(emissions.is_empty(), "spurious emission: {emissions:?}");
    }

    #[test]
    fn b2_emit_event_writes_compact_record() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let mut session = AccumulatorOpSession::new(&ctx, 2, 1);
        let op = AccumulatorOp {
            source: SourceSpec::Constant(3.7),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![(0, 0)],
        };
        session.upload_values(&ctx, &[0.0, 0.0]);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
        session.tick(&ctx, 0).unwrap();

        let emissions = session.readback_emissions(&ctx).unwrap();
        assert_eq!(
            emissions,
            vec![EmissionRecord {
                reg_idx:    0,
                emit_count: 3,
            }]
        );
        let values = session.readback_full(&ctx).unwrap();
        assert_eq!(values[0], 3.7);
    }

    #[test]
    fn b2_emit_event_zero_count_writes_no_record() {
        set_debug_readback_allowed(true);
        let (ctx, mut session) = gpu_session(2, 1);
        let op = AccumulatorOp {
            source: SourceSpec::Constant(0.3),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![(0, 0)],
        };
        session.upload_values(&ctx, &[0.0, 0.0]);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
        session.tick(&ctx, 0).unwrap();
        assert!(session.readback_emissions(&ctx).unwrap().is_empty());
    }

    #[test]
    fn b2_emission_overflow_is_reported() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let mut session = AccumulatorOpSession::with_emission_capacity(&ctx, 4, 1, 1);
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(5.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::EmitEvent,
                targets: vec![(0, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(3.0),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::EmitEvent,
                targets: vec![(1, 0)],
            },
        ];
        session.upload_values(&ctx, &[0.0; 4]);
        session.upload_ops(&ctx, &ops).unwrap();
        session.tick(&ctx, 0).unwrap();
        assert!(matches!(
            session.readback_emissions(&ctx),
            Err(AccumulatorOpSessionError::EmissionOverflow {
                count: 2,
                capacity: 1,
            })
        ));
    }

    #[test]
    fn b2_encodes_weighted_mean_stub() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotRange { start: 0, count: 2 },
            combine: CombineFn::WeightedMean { weight_col: 1 },
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        let (ctx, mut session) = gpu_session(4, 2);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
    }

    #[test]
    fn b2_encodes_eval_eml_stub() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::EvalEML { tree_id: 1 },
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        let (ctx, mut session) = gpu_session(4, 1);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
    }

    #[test]
    fn b2_encodes_threshold_gate_with_none_consume() {
        use simthing_core::ThresholdDirection;
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: 1.0,
                direction: ThresholdDirection::Upward,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        let (ctx, mut session) = gpu_session(4, 1);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
    }

    #[test]
    fn b2_encodes_conjunctive_crossing_stub() {
        use simthing_core::InputSpec;
        let op = AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![InputSpec {
                    slot: 0,
                    col: 0,
                    unit_cost: 1.0,
                }],
            },
            combine: CombineFn::MinAcrossInputs,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::SubtractFromAllInputs,
            targets: vec![(1, 0)],
        };
        let (ctx, mut session) = gpu_session(4, 1);
        session.upload_ops(&ctx, std::slice::from_ref(&op)).unwrap();
    }

    fn trivial_1000_ops() -> Vec<AccumulatorOp> {
        (0..1000)
            .map(|i| AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(i, 0)],
            })
            .collect()
    }

    #[test]
    fn b3_timestamp_query_path_does_not_panic() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let mut session = AccumulatorOpSession::new(&ctx, 1000, 1);
        let ops = trivial_1000_ops();
        session.upload_values(&ctx, &vec![0.0; 1000]);
        session.upload_ops(&ctx, &ops).unwrap();
        session.tick(&ctx, 0).unwrap();

        if session.timestamp_supported() {
            assert!(session.last_pass_time_us().is_some());
        } else {
            assert_eq!(session.last_pass_time_us(), None);
        }
    }

    #[test]
    fn b3_timestamp_query_reports_plausible_time_when_supported() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let mut session = AccumulatorOpSession::new(&ctx, 1000, 1);
        let ops = trivial_1000_ops();
        session.upload_values(&ctx, &vec![0.0; 1000]);
        session.upload_ops(&ctx, &ops).unwrap();
        session.tick(&ctx, 0).unwrap();

        if let Some(us) = session.last_pass_time_us() {
            assert!(us > 0);
            assert!(us < 10_000);
        }
    }

    #[test]
    fn b3_timestamp_does_not_alter_semantics() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let n_slots = 8u32;
        let n_dims = 2u32;
        let ops = bootstrap_ops();
        let initial = bootstrap_initial_values(n_slots, n_dims);

        let mut expected = initial.clone();
        execute_ops_cpu(&mut expected, &ops, 0, n_dims).unwrap();
        execute_ops_cpu(&mut expected, &ops, 1, n_dims).unwrap();
        let expected_summaries = summaries_from_values(&expected, n_slots, n_dims);

        let mut session = AccumulatorOpSession::new(&ctx, n_slots, n_dims);
        session.upload_values(&ctx, &initial);
        session.upload_ops(&ctx, &ops).unwrap();
        session.tick(&ctx, 0).unwrap();
        session.tick(&ctx, 1).unwrap();

        assert_eq!(session.readback_summary(&ctx).unwrap(), expected_summaries);
        assert_eq!(session.readback_full(&ctx).unwrap(), expected);

        if session.timestamp_supported() {
            assert!(session.last_pass_time_us().is_some());
        } else {
            assert_eq!(session.last_pass_time_us(), None);
        }
    }

    fn threshold_test_regs() -> Vec<ThresholdRegistration> {
        use crate::world_state::{DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_VALUES};
        vec![
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
        ]
    }

    fn setup_threshold_values(n_dims: u32) -> (Vec<f32>, Vec<f32>) {
        let n = 3 * n_dims as usize;
        let mut previous = vec![0.0_f32; n];
        let mut current = vec![0.0_f32; n];
        previous[0] = 0.40;
        current[0] = 0.10;
        previous[1 * n_dims as usize] = 0.10;
        current[1 * n_dims as usize] = 0.50;
        previous[2 * n_dims as usize] = 0.50;
        current[2 * n_dims as usize] = 0.50;
        (previous, current)
    }

    #[test]
    fn c1_threshold_gpu_matches_cpu_oracle() {
        let Some(_) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu");
        let n_dims = 1u32;
        let (previous, current) = setup_threshold_values(n_dims);
        let regs = threshold_test_regs();
        let (ops, kinds) = threshold_registrations_to_ops(&regs).unwrap();

        let mut expected_values = current.clone();
        let mut expected =
            execute_threshold_ops_cpu(&previous, &mut expected_values, &ops, n_dims).unwrap();
        expected.sort_by_key(|e| (e.slot, e.col, e.reg_idx));

        let mut session = AccumulatorOpSession::new_attached(&ctx, 3, n_dims, 16);
        session.upload_values(&ctx, &current);
        session.upload_previous_values(&ctx, &previous);
        session.upload_threshold_ops(&ctx, &regs).unwrap();
        session.tick(&ctx, 0).unwrap();

        let mut gpu = session.readback_threshold_emissions(&ctx).unwrap();
        gpu.sort_by_key(|e| (e.slot, e.col, e.reg_idx));
        assert_eq!(gpu.len(), 2);
        assert_eq!(gpu[0].slot, 0);
        assert_eq!(gpu[1].slot, 1);

        let events = session.readback_threshold_events(&ctx).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_kind, kinds[0]);
        assert_eq!(events[1].event_kind, kinds[1]);
    }

    #[test]
    fn c1_threshold_event_kinds_round_trip() {
        let Some(_) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let ctx = GpuContext::new_blocking().expect("gpu");
        let n_dims = 1u32;
        let (previous, current) = setup_threshold_values(n_dims);
        let regs = threshold_test_regs();
        let (_, kinds) = threshold_registrations_to_ops(&regs).unwrap();

        let mut session = AccumulatorOpSession::new_attached(&ctx, 3, n_dims, 16);
        session.upload_values(&ctx, &current);
        session.upload_previous_values(&ctx, &previous);
        session.upload_threshold_ops(&ctx, &regs).unwrap();
        session.tick(&ctx, 0).unwrap();

        let events = session.readback_threshold_events(&ctx).unwrap();
        for ev in events {
            let idx = regs
                .iter()
                .position(|r| r.slot == ev.slot && r.col == ev.col)
                .expect("slot/col");
            assert_eq!(ev.event_kind, kinds[idx]);
        }
    }

    #[test]
    fn c1_threshold_emission_overflow_reported() {
        let Some(_) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let ctx = GpuContext::new_blocking().expect("gpu");
        let n_dims = 1u32;
        let mut previous = vec![0.0, 0.0];
        let mut current = vec![1.0, 1.0];
        previous[0] = 0.0;
        current[0] = 1.0;
        previous[1] = 0.0;
        current[1] = 1.0;
        let regs = vec![
            ThresholdRegistration {
                slot: 0,
                col: 0,
                threshold: 0.5,
                direction: crate::world_state::DIR_UPWARD,
                event_kind: 1,
                buffer: crate::world_state::THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 1,
                col: 0,
                threshold: 0.5,
                direction: crate::world_state::DIR_UPWARD,
                event_kind: 2,
                buffer: crate::world_state::THRESH_BUF_VALUES,
            },
        ];

        let mut session = AccumulatorOpSession::new_attached(&ctx, 2, n_dims, 1);
        session.upload_values(&ctx, &current);
        session.upload_previous_values(&ctx, &previous);
        session.upload_threshold_ops(&ctx, &regs).unwrap();
        session.tick(&ctx, 0).unwrap();

        assert!(matches!(
            session.readback_threshold_emissions(&ctx),
            Err(AccumulatorOpSessionError::ThresholdEmissionOverflow {
                count: 2,
                capacity: 1,
            })
        ));
    }

    #[test]
    fn c2_accumulator_intent_matches_cpu_affine() {
        use super::super::cpu_oracle::execute_intent_deltas_cpu;

        let Some(_) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu");
        let n_dims = 3u32;

        let run_case = |initial: f32, mul: f32, add: f32| {
            let values = vec![initial, 0.0, 0.0];
            let deltas = [IntentDelta {
                slot: 0,
                col:  0,
                mul,
                add,
            }];
            let mut expected = values.clone();
            execute_intent_deltas_cpu(&mut expected, &deltas, n_dims);

            let mut session = AccumulatorOpSession::new_attached(&ctx, 1, n_dims, 16);
            session.upload_values(&ctx, &values);
            session.upload_intent_ops(&ctx, &deltas).unwrap();
            session.tick(&ctx, 0).unwrap();
            assert_eq!(session.readback_full(&ctx).unwrap(), expected);
        };

        run_case(10.0, 2.0, 3.0);
        run_case(99.0, 0.0, 5.0);
        run_case(4.0, -1.0, 2.0);
    }

    #[test]
    fn c2_empty_intent_set_noops() {
        let Some(_) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut session = AccumulatorOpSession::new_attached(&ctx, 1, 1, 16);
        session.upload_intent_ops(&ctx, &[]).unwrap();
        session.prepare_intent(&ctx);
    }
}

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

use super::encode::EncodeError;
use super::types::AccumulatorOpGpu;
use super::types::{
    AccumulatorSummaryParams, AccumulatorTickParams, EmissionRecord,
    EmissionRecordGpu, SlotSummary, SlotSummaryGpu, DEFAULT_EMISSION_CAPACITY,
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
}

/// GPU-resident AccumulatorOp session (B-2 production-shaped kernel subset).
///
/// Persistent buffers and compact emission readback are production-shaped, but
/// the kernel supports only non-contended Identity/Sum, clamped SlotValue
/// transfer, and Identity EmitEvent. Not integrated with `BoundaryProtocol`.
pub struct AccumulatorOpSession {
    ctx: GpuContext,
    n_slots: u32,
    n_dims: u32,
    n_ops: u32,
    emission_capacity: u32,

    op_buffer: Buffer,
    values_buffer: Buffer,
    summary_buffer: Buffer,
    emission_buffer: Buffer,
    emission_count: Buffer,

    tick_uniform: Buffer,
    summary_uniform: Buffer,

    execute_layout: BindGroupLayout,
    execute_pipeline: ComputePipeline,
    summary_layout: BindGroupLayout,
    summary_pipeline: ComputePipeline,

    timestamp_query_set: Option<QuerySet>,
    timestamp_resolve_buffer: Option<Buffer>,
    timestamp_readback_buffer: Option<Buffer>,
    last_pass_time_us: Option<u64>,
}

impl AccumulatorOpSession {
    pub fn new(ctx: GpuContext, n_slots: u32, n_dims: u32) -> Self {
        Self::with_emission_capacity(ctx, n_slots, n_dims, DEFAULT_EMISSION_CAPACITY)
    }

    pub fn with_emission_capacity(
        ctx: GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) -> Self {
        assert!(n_slots > 0 && n_dims > 0, "n_slots and n_dims must be > 0");

        let device = &ctx.device;
        let values_len = (n_slots * n_dims) as u64 * 4;
        let summary_len = (n_slots as u64) * std::mem::size_of::<SlotSummaryGpu>() as u64;
        let emission_len =
            (emission_capacity as u64) * std::mem::size_of::<EmissionRecordGpu>() as u64;

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
                storage_entry(0, true),
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
            ctx,
            n_slots,
            n_dims,
            n_ops: 0,
            emission_capacity,
            op_buffer,
            values_buffer,
            summary_buffer,
            emission_buffer,
            emission_count,
            tick_uniform,
            summary_uniform,
            execute_layout,
            execute_pipeline,
            summary_layout,
            summary_pipeline,
            timestamp_query_set,
            timestamp_resolve_buffer,
            timestamp_readback_buffer,
            last_pass_time_us: None,
        };

        session.reset_emission_count();
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

    /// Whether this session was created with GPU timestamp query support.
    pub fn timestamp_supported(&self) -> bool {
        self.timestamp_query_set.is_some()
    }

    /// Duration of the last `execute_ops` pass in microseconds, if timestamp queries are supported.
    pub fn last_pass_time_us(&self) -> Option<u64> {
        self.last_pass_time_us
    }

    /// Upload initial or post-tick values matrix (row-major slot × dims).
    pub fn upload_values(&self, values: &[f32]) {
        assert_eq!(values.len(), self.values_len());
        self.ctx
            .queue
            .write_buffer(&self.values_buffer, 0, bytemuck::cast_slice(values));
    }

    /// Upload AccumulatorOp registrations after bootstrap subset + contention validation.
    pub fn upload_ops(&mut self, ops: &[AccumulatorOp]) -> Result<(), AccumulatorOpSessionError> {
        let gpu_ops = AccumulatorOpGpu::encode_bootstrap_set(ops)?;

        let byte_len = gpu_ops.len() * std::mem::size_of::<AccumulatorOpGpu>();
        if self.op_buffer.size() < byte_len as u64 {
            self.op_buffer = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("accumulator_op_buffer"),
                size: byte_len.max(4096) as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        self.ctx
            .queue
            .write_buffer(&self.op_buffer, 0, bytemuck::cast_slice(&gpu_ops));
        self.n_ops = gpu_ops.len() as u32;
        Ok(())
    }

    /// Dispatch Pass B for one OrderBand, then refresh per-slot summaries.
    pub fn tick(&mut self, band: u32) -> Result<(), AccumulatorOpSessionError> {
        if self.n_ops == 0 {
            return Err(AccumulatorOpSessionError::NoOps);
        }

        self.reset_emission_count();
        self.last_pass_time_us = None;

        let tick_params = AccumulatorTickParams {
            n_ops: self.n_ops,
            current_band: band,
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            emission_capacity: self.emission_capacity,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        };
        self.ctx.queue.write_buffer(
            &self.tick_uniform,
            0,
            bytemuck::bytes_of(&tick_params),
        );

        let execute_bind_group = self.ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("accumulator_execute_bg"),
            layout: &self.execute_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.op_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.values_buffer.as_entire_binding(),
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
            ],
        });

        let mut encoder = self
            .ctx
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

        self.dispatch_write_summaries(&mut encoder);
        self.ctx.queue.submit(Some(encoder.finish()));

        self.read_execute_pass_timestamp();
        Ok(())
    }

    /// Provisional B-1/B-2 summary readback tier (checksum-only; final shape is B-4).
    pub fn readback_summary(&self) -> Result<Vec<SlotSummary>, AccumulatorOpSessionError> {
        let bytes = self.read_buffer_bytes(&self.summary_buffer);
        let gpu: &[SlotSummaryGpu] = bytemuck::cast_slice(&bytes);
        Ok(gpu
            .iter()
            .map(|s| SlotSummary {
                slot: s.slot,
                checksum: s.checksum,
            })
            .collect())
    }

    /// Read compact emission records written by EmitEvent ops this tick.
    pub fn readback_emissions(&self) -> Result<Vec<EmissionRecord>, AccumulatorOpSessionError> {
        let count = self.read_emission_count()?;
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
        let bytes = self.read_buffer_bytes_range(&self.emission_buffer, 0, used);
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
    pub fn readback_full(&self) -> Result<Vec<f32>, AccumulatorOpSessionError> {
        if !DEBUG_READBACK_ALLOWED.load(Ordering::Relaxed) {
            eprintln!(
                "warning: AccumulatorOpSession::readback_full() called outside test mode"
            );
        }
        Ok(self.read_buffer_f32(&self.values_buffer))
    }

    fn reset_emission_count(&self) {
        self.ctx
            .queue
            .write_buffer(&self.emission_count, 0, &0u32.to_le_bytes());
    }

    fn read_emission_count(&self) -> Result<u32, AccumulatorOpSessionError> {
        let bytes = self.read_buffer_bytes(&self.emission_count);
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_execute_pass_timestamp(&mut self) {
        // B-3 reads timestamps synchronously for testability.
        // Later production profiling can batch or sample timestamp readbacks.
        let Some(readback) = self.timestamp_readback_buffer.as_ref() else {
            self.last_pass_time_us = None;
            return;
        };

        self.ctx.device.poll(Maintain::Wait);
        let slice = readback.slice(..);
        slice.map_async(MapMode::Read, |_| {});
        self.ctx.device.poll(Maintain::Wait);
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
            let ns = delta as f64 * self.ctx.timestamp_period_ns() as f64;
            self.last_pass_time_us = Some((ns / 1000.0).round() as u64);
        } else {
            self.last_pass_time_us = None;
        }
    }

    fn dispatch_write_summaries(
        &self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let summary_params = AccumulatorSummaryParams {
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            _pad0: 0,
            _pad1: 0,
        };
        self.ctx.queue.write_buffer(
            &self.summary_uniform,
            0,
            bytemuck::bytes_of(&summary_params),
        );

        let summary_bind_group = self.ctx.device.create_bind_group(&BindGroupDescriptor {
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

    fn read_buffer_f32(&self, buf: &Buffer) -> Vec<f32> {
        let bytes = self.read_buffer_bytes(buf);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn read_buffer_bytes(&self, buf: &Buffer) -> Vec<u8> {
        self.read_buffer_bytes_range(buf, 0, buf.size())
    }

    fn read_buffer_bytes_range(
        &self,
        buf: &Buffer,
        offset: u64,
        size: u64,
    ) -> Vec<u8> {
        let staging = self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_staging_read"),
            size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_read_encoder"),
            });
        encoder.copy_buffer_to_buffer(buf, offset, &staging, 0, size);
        self.ctx.queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        slice.map_async(MapMode::Read, |_| {});
        self.ctx.device.poll(Maintain::Wait);
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
        execute_ops_cpu, set_debug_readback_allowed, summaries_from_values,
    };
    use crate::context::GpuContext;

    use super::*;

    fn gpu_session(n_slots: u32, n_dims: u32) -> AccumulatorOpSession {
        let ctx = GpuContext::new_blocking().expect("gpu context");
        AccumulatorOpSession::new(ctx, n_slots, n_dims)
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

        let mut session = AccumulatorOpSession::new(ctx, n_slots, n_dims);
        session.upload_values(&initial);
        session.upload_ops(&ops).unwrap();
        session.tick(0).unwrap();
        session.tick(1).unwrap();

        let gpu_summaries = session.readback_summary().unwrap();
        assert_eq!(gpu_summaries, expected_summaries);

        let gpu_values = session.readback_full().unwrap();
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

        let mut session = gpu_session(2, n_dims);
        session.upload_values(&values);
        session.upload_ops(std::slice::from_ref(&op)).unwrap();
        session.tick(0).unwrap();
        values = session.readback_full().unwrap();
        assert_eq!(values[1], 7.0);
    }

    #[test]
    fn accumulator_transfer_clamps_to_available_source() {
        set_debug_readback_allowed(true);
        let n_dims = 1u32;
        let mut session = gpu_session(2, n_dims);

        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(10.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        session.upload_values(&values);
        session.upload_ops(std::slice::from_ref(&op)).unwrap();
        session.tick(0).unwrap();
        values = session.readback_full().unwrap();
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
        session.upload_values(&values);
        session.upload_ops(std::slice::from_ref(&op_small)).unwrap();
        session.tick(0).unwrap();
        values = session.readback_full().unwrap();
        assert_eq!(values, vec![7.0, 3.0]);
    }

    #[test]
    fn accumulator_transfer_rejects_negative_requested_transfer() {
        set_debug_readback_allowed(true);
        let n_dims = 1u32;
        let mut session = gpu_session(2, n_dims);
        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(-3.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        session.upload_values(&values);
        session.upload_ops(std::slice::from_ref(&op)).unwrap();
        session.tick(0).unwrap();
        values = session.readback_full().unwrap();
        assert_eq!(values, vec![5.0, 0.0]);
    }

    #[test]
    fn upload_ops_rejects_duplicate_target_same_band() {
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
        let mut session = gpu_session(4, 1);
        let err = session.upload_ops(&ops).unwrap_err();
        assert!(matches!(
            err,
            AccumulatorOpSessionError::Encode(EncodeError::BootstrapContention {
                band: 0,
                slot: 1,
                col: 0
            })
        ));
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
        let mut session = gpu_session(4, 1);
        let err = session.upload_ops(&ops).unwrap_err();
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
        let mut session = gpu_session(4, 1);
        session.upload_ops(&ops).unwrap();
    }

    #[test]
    fn upload_ops_rejects_always_and_orderband_same_target() {
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
        let mut session = gpu_session(4, 1);
        assert!(matches!(
            session.upload_ops(&ops),
            Err(AccumulatorOpSessionError::Encode(EncodeError::BootstrapContention {
                slot: 1,
                col: 0,
                ..
            }))
        ));
    }

    #[test]
    fn upload_ops_rejects_always_consume_and_orderband_write_same_cell() {
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
        let mut session = gpu_session(4, 2);
        assert!(matches!(
            session.upload_ops(&ops),
            Err(AccumulatorOpSessionError::Encode(EncodeError::BootstrapContention {
                slot: 0,
                col: 0,
                ..
            }))
        ));
    }

    #[test]
    fn upload_ops_rejects_orderband_consume_and_always_write_same_cell() {
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
        let mut session = gpu_session(4, 2);
        assert!(matches!(
            session.upload_ops(&ops),
            Err(AccumulatorOpSessionError::Encode(EncodeError::BootstrapContention {
                slot: 0,
                col: 0,
                ..
            }))
        ));
    }

    #[test]
    fn upload_ops_rejects_two_always_writers_same_cell() {
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
        let mut session = gpu_session(4, 1);
        assert!(matches!(
            session.upload_ops(&ops),
            Err(AccumulatorOpSessionError::Encode(EncodeError::BootstrapContention {
                band: u32::MAX,
                slot: 1,
                col: 0,
            }))
        ));
    }

    #[test]
    fn b2_emit_event_writes_compact_record() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let mut session = AccumulatorOpSession::new(ctx, 2, 1);
        let op = AccumulatorOp {
            source: SourceSpec::Constant(3.7),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![(0, 0)],
        };
        session.upload_values(&[0.0, 0.0]);
        session.upload_ops(std::slice::from_ref(&op)).unwrap();
        session.tick(0).unwrap();

        let emissions = session.readback_emissions().unwrap();
        assert_eq!(
            emissions,
            vec![EmissionRecord {
                reg_idx:    0,
                emit_count: 3,
            }]
        );
        let values = session.readback_full().unwrap();
        assert_eq!(values[0], 3.7);
    }

    #[test]
    fn b2_emit_event_zero_count_writes_no_record() {
        set_debug_readback_allowed(true);
        let mut session = gpu_session(2, 1);
        let op = AccumulatorOp {
            source: SourceSpec::Constant(0.3),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![(0, 0)],
        };
        session.upload_values(&[0.0, 0.0]);
        session.upload_ops(std::slice::from_ref(&op)).unwrap();
        session.tick(0).unwrap();
        assert!(session.readback_emissions().unwrap().is_empty());
    }

    #[test]
    fn b2_emission_overflow_is_reported() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let mut session = AccumulatorOpSession::with_emission_capacity(ctx, 4, 1, 1);
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
        session.upload_values(&[0.0; 4]);
        session.upload_ops(&ops).unwrap();
        session.tick(0).unwrap();
        assert!(matches!(
            session.readback_emissions(),
            Err(AccumulatorOpSessionError::EmissionOverflow {
                count: 2,
                capacity: 1,
            })
        ));
    }

    #[test]
    fn b2_rejects_weighted_mean() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotRange { start: 0, count: 2 },
            combine: CombineFn::WeightedMean { weight_col: 1 },
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        let mut session = gpu_session(4, 2);
        assert!(matches!(
            session.upload_ops(std::slice::from_ref(&op)),
            Err(AccumulatorOpSessionError::Encode(EncodeError::Unsupported(_)))
        ));
    }

    #[test]
    fn b2_rejects_eval_eml() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::EvalEML { tree_id: 1 },
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        let mut session = gpu_session(4, 1);
        assert!(matches!(
            session.upload_ops(std::slice::from_ref(&op)),
            Err(AccumulatorOpSessionError::Encode(EncodeError::Unsupported(_)))
        ));
    }

    #[test]
    fn b2_rejects_threshold_gate_until_c1() {
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
        let mut session = gpu_session(4, 1);
        assert!(matches!(
            session.upload_ops(std::slice::from_ref(&op)),
            Err(AccumulatorOpSessionError::Encode(EncodeError::Unsupported(_)))
        ));
    }

    #[test]
    fn b2_rejects_conjunctive_crossing() {
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
        let mut session = gpu_session(4, 1);
        assert!(matches!(
            session.upload_ops(std::slice::from_ref(&op)),
            Err(AccumulatorOpSessionError::Encode(EncodeError::Unsupported(_)))
        ));
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
        let mut session = AccumulatorOpSession::new(ctx, 1000, 1);
        let ops = trivial_1000_ops();
        session.upload_values(&vec![0.0; 1000]);
        session.upload_ops(&ops).unwrap();
        session.tick(0).unwrap();

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
        let mut session = AccumulatorOpSession::new(ctx, 1000, 1);
        let ops = trivial_1000_ops();
        session.upload_values(&vec![0.0; 1000]);
        session.upload_ops(&ops).unwrap();
        session.tick(0).unwrap();

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

        let mut session = AccumulatorOpSession::new(ctx, n_slots, n_dims);
        session.upload_values(&initial);
        session.upload_ops(&ops).unwrap();
        session.tick(0).unwrap();
        session.tick(1).unwrap();

        assert_eq!(session.readback_summary().unwrap(), expected_summaries);
        assert_eq!(session.readback_full().unwrap(), expected);

        if session.timestamp_supported() {
            assert!(session.last_pass_time_us().is_some());
        } else {
            assert_eq!(session.last_pass_time_us(), None);
        }
    }
}

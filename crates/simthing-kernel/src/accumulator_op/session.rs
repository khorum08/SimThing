//! Persistent GPU buffer ownership for AccumulatorOp v2 Pass B.

use std::sync::atomic::{AtomicBool, Ordering};

use bytemuck;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor,
    Maintain, MapMode, PipelineLayoutDescriptor, QuerySet, QuerySetDescriptor, QueryType,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::context::GpuContext;
use crate::gpu_readback::{EmissionRecordReadback, KernelReadbackError, ThresholdEmissionReadback};
use crate::sealed::ThresholdEvent;

use super::encode::EncodeError;
use super::packed_session_upload::{
    PackedAccumulatorUpload, PackedIntentUpload, PackedThresholdUpload,
};
use super::types::{AccumulatorInputGpu, AccumulatorOpGpu};
use super::types::{
    AccumulatorSummaryParams, AccumulatorTickParams, EmissionRecord, EmlNodeGpu, EmlTreeRangeGpu,
    SlotSummary, SlotSummaryGpu, ThresholdEmission, DEFAULT_EMISSION_CAPACITY,
    DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};

pub const WORKGROUP_SIZE: u32 = 64;
const EXECUTE_MODE_COMPACT_VELOCITY: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ValuesFillParams {
    start_slot: u32,
    count: u32,
    col: u32,
    n_dims: u32,
    value: f32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

static DEBUG_READBACK_ALLOWED: AtomicBool = AtomicBool::new(false);

/// Allow `readback_full()` without emitting a warning (tests and explicit proof readback only).
pub fn set_debug_readback_allowed(allowed: bool) {
    DEBUG_READBACK_ALLOWED.store(allowed, Ordering::Relaxed);
}

/// Current debug readback gate state.
pub fn debug_readback_allowed() -> bool {
    DEBUG_READBACK_ALLOWED.load(Ordering::Relaxed)
}

/// RAII guard that restores the prior debug readback gate on drop (including panic unwind).
pub struct DebugReadbackGuard {
    previous: bool,
}

impl DebugReadbackGuard {
    /// Set the debug readback gate to `enabled` until this guard is dropped.
    pub fn new(enabled: bool) -> Self {
        let previous = debug_readback_allowed();
        set_debug_readback_allowed(enabled);
        Self { previous }
    }
}

impl Drop for DebugReadbackGuard {
    fn drop(&mut self) {
        set_debug_readback_allowed(self.previous);
    }
}

/// Scope debug readback gating to the lifetime of the returned guard.
pub fn scoped_debug_readback_allowed(enabled: bool) -> DebugReadbackGuard {
    DebugReadbackGuard::new(enabled)
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
    #[error("copy exceeds values buffer: offset={offset}, bytes={bytes}, capacity={capacity}")]
    CopyOutOfBounds {
        offset: u64,
        bytes: u64,
        capacity: u64,
    },
    #[error("invalid slot {slot} for n_slots={n_slots}")]
    InvalidSlot { slot: u32, n_slots: u32 },
    #[error("invalid column {col} for n_dims={n_dims}")]
    InvalidColumn { col: u32, n_dims: u32 },
    #[error("non-finite fill value")]
    NonFiniteValue,
    #[error("invalid slot range: start={start_slot}, count={count}, n_slots={n_slots}")]
    InvalidSlotRange {
        start_slot: u32,
        count: u32,
        n_slots: u32,
    },
    #[error("op upload byte size overflow: ops={ops}, op_size={op_size}")]
    OpUploadByteSizeOverflow { ops: usize, op_size: usize },
    #[error("input-list upload byte size overflow: inputs={inputs}, input_size={input_size}")]
    InputListUploadByteSizeOverflow { inputs: usize, input_size: usize },
}

impl From<KernelReadbackError> for AccumulatorOpSessionError {
    fn from(err: KernelReadbackError) -> Self {
        match err {
            KernelReadbackError::EmissionOverflow { count, capacity } => {
                Self::EmissionOverflow { count, capacity }
            }
            KernelReadbackError::ThresholdEmissionOverflow { count, capacity } => {
                Self::ThresholdEmissionOverflow { count, capacity }
            }
        }
    }
}

/// GPU-resident AccumulatorOp session (B-2 bootstrap + C-1 threshold scan).
pub struct AccumulatorOpSession {
    n_slots: u32,
    n_dims: u32,
    n_ops: u32,

    op_buffer: Buffer,
    values_buffer: Buffer,
    previous_values_buffer: Buffer,
    summary_buffer: Buffer,
    emission_readback: EmissionRecordReadback,
    threshold_emission_readback: ThresholdEmissionReadback,

    tick_uniform: Buffer,
    summary_uniform: Buffer,

    eml_node_buffer: Buffer,
    eml_range_buffer: Buffer,
    input_list_buffer: Buffer,

    execute_layout: BindGroupLayout,
    execute_pipeline: ComputePipeline,
    /// AO-WGSL-0: fused multi-band OrderBand fast path (`execute_orderband_bands`).
    orderband_fast_pipeline: ComputePipeline,
    /// AO-WGSL-0: dynamic-offset bind layout for the fast path (binding 4 dynamic).
    orderband_fast_layout: BindGroupLayout,
    /// AO-WGSL-0: growable band-params uniform; one buffer/bind-group per encode,
    /// dynamic-offset indexed per band. Avoids O(n_bands) allocation churn.
    orderband_fast_uniform: Buffer,
    /// Band capacity of `orderband_fast_uniform` at the device uniform stride.
    orderband_fast_uniform_bands: u32,
    summary_layout: BindGroupLayout,
    summary_pipeline: ComputePipeline,
    fill_uniform: Buffer,
    fill_layout: BindGroupLayout,
    fill_pipeline: ComputePipeline,

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

    /// C-5 reduction session sized for an external values buffer.
    ///
    /// This constructor only allocates session resources; binding 1 is routed to
    /// `output_vectors` per dispatch via [`Self::encode_reduction_soft_into`] /
    /// [`Self::encode_reduction_soft_band_into`]. The `_external_values` argument
    /// documents the intended world buffer but is not stored.
    pub fn new_attached_for_external_values(
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        _external_values: &Buffer,
    ) -> Self {
        Self::build(
            ctx,
            n_slots,
            n_dims,
            DEFAULT_EMISSION_CAPACITY,
            DEFAULT_THRESHOLD_EMISSION_CAPACITY,
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

        let op_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_op_buffer"),
            size: 4096,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
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

        let emission_readback = EmissionRecordReadback::new(device, emission_capacity);
        let threshold_emission_readback =
            ThresholdEmissionReadback::new(device, threshold_emission_capacity);

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

        let (eml_node_buffer, eml_range_buffer) = mk_dummy_eml_buffers(device);
        let input_list_buffer = mk_dummy_input_list_buffer(device);

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
                storage_entry(8, true),
                storage_entry(9, true),
                storage_entry(10, true),
                storage_entry(11, true),
                storage_entry(12, true),
            ],
        });

        let execute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("accumulator_execute_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("accumulator_execute_pl"),
                bind_group_layouts: &[&execute_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "execute_ops",
            compilation_options: Default::default(),
            cache: None,
        });

        let orderband_fast_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("accumulator_ao_wgsl0_fast_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, false),
                storage_entry(2, false),
                storage_entry(3, false),
                uniform_entry_dynamic(4),
                storage_entry(5, true),
                storage_entry(6, false),
                storage_entry(7, false),
                storage_entry(8, true),
                storage_entry(9, true),
                storage_entry(10, true),
                storage_entry(11, true),
                storage_entry(12, true),
            ],
        });

        let orderband_fast_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("accumulator_ao_wgsl0_fast_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("accumulator_ao_wgsl0_fast_pl"),
                bind_group_layouts: &[&orderband_fast_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: super::wgsl_path::AO_WGSL0_ENTRY_POINT,
            compilation_options: Default::default(),
            cache: None,
        });

        let orderband_fast_uniform = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_ao_wgsl0_fast_uniform"),
            size: ao_wgsl0_uniform_stride(device),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
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
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("accumulator_summary_pl"),
                bind_group_layouts: &[&summary_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "write_summaries",
            compilation_options: Default::default(),
            cache: None,
        });

        let fill_uniform = device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_values_fill_uniform"),
            size: std::mem::size_of::<ValuesFillParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let fill_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("accumulator_values_fill"),
            source: ShaderSource::Wgsl(include_str!("../shaders/values_fill.wgsl").into()),
        });

        let fill_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("accumulator_values_fill_layout"),
            entries: &[storage_entry(0, false), uniform_entry(1)],
        });

        let fill_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("accumulator_values_fill_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("accumulator_values_fill_pl"),
                bind_group_layouts: &[&fill_layout],
                push_constant_ranges: &[],
            })),
            module: &fill_shader,
            entry_point: "fill_slot_range_col",
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
            op_buffer,
            values_buffer,
            previous_values_buffer,
            summary_buffer,
            emission_readback,
            threshold_emission_readback,
            tick_uniform,
            summary_uniform,
            eml_node_buffer,
            eml_range_buffer,
            input_list_buffer,
            execute_layout,
            execute_pipeline,
            orderband_fast_pipeline,
            orderband_fast_layout,
            orderband_fast_uniform,
            orderband_fast_uniform_bands: 1,
            summary_layout,
            summary_pipeline,
            fill_uniform,
            fill_layout,
            fill_pipeline,
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

    /// Sum of persistent GPU buffer allocations for this session (excludes ephemeral
    /// per-readback staging buffers). Used by runtime profiling captures.
    pub fn values_buffer_size_bytes(&self) -> u64 {
        self.values_buffer.size()
    }

    pub fn persistent_buffer_bytes(&self) -> u64 {
        self.op_buffer.size()
            + self.values_buffer.size()
            + self.previous_values_buffer.size()
            + self.summary_buffer.size()
            + self.emission_readback.total_buffer_bytes()
            + self.threshold_emission_readback.total_buffer_bytes()
            + self.tick_uniform.size()
            + self.summary_uniform.size()
            + self.eml_node_buffer.size()
            + self.eml_range_buffer.size()
            + self.input_list_buffer.size()
            + self.fill_uniform.size()
            + self.orderband_fast_uniform.size()
            + self
                .timestamp_resolve_buffer
                .as_ref()
                .map(|b| b.size())
                .unwrap_or(0)
            + self
                .timestamp_readback_buffer
                .as_ref()
                .map(|b| b.size())
                .unwrap_or(0)
    }

    pub fn emission_capacity(&self) -> u32 {
        self.emission_readback.capacity()
    }

    pub fn threshold_emission_capacity(&self) -> u32 {
        self.threshold_emission_readback.capacity()
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

    pub fn values_byte_len(&self) -> u64 {
        (self.values_len() * std::mem::size_of::<f32>()) as u64
    }

    fn slot_col_byte_offset(&self, slot: u32, col: u32) -> u64 {
        ((slot * self.n_dims + col) * std::mem::size_of::<f32>() as u32) as u64
    }

    /// Zero the entire values buffer via queue writes.
    pub fn zero_values_buffer(&self, ctx: &GpuContext) {
        let zeros = vec![0.0f32; self.values_len()];
        ctx.queue
            .write_buffer(&self.values_buffer, 0, bytemuck::cast_slice(&zeros));
    }

    /// Snapshot the current values buffer into the previous-values buffer
    /// on-device — the threshold kernel's edge baseline for the next scan.
    /// Pure data movement; no readback.
    pub fn copy_values_to_previous(&self, ctx: &GpuContext) {
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_values_to_previous_copy"),
            });
        encoder.copy_buffer_to_buffer(
            &self.values_buffer,
            0,
            &self.previous_values_buffer,
            0,
            self.values_byte_len(),
        );
        ctx.queue.submit(Some(encoder.finish()));
    }

    /// Copy a prefix from an external GPU buffer into the values buffer.
    pub fn copy_values_prefix_from_buffer(
        &self,
        ctx: &GpuContext,
        src: &Buffer,
        src_offset_bytes: u64,
        dst_offset_bytes: u64,
        bytes: u64,
    ) -> Result<(), AccumulatorOpSessionError> {
        let capacity = self.values_byte_len();
        if bytes == 0 {
            return Ok(());
        }
        if dst_offset_bytes.saturating_add(bytes) > capacity {
            return Err(AccumulatorOpSessionError::CopyOutOfBounds {
                offset: dst_offset_bytes,
                bytes,
                capacity,
            });
        }
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_values_prefix_copy"),
            });
        encoder.copy_buffer_to_buffer(
            src,
            src_offset_bytes,
            &self.values_buffer,
            dst_offset_bytes,
            bytes,
        );
        ctx.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Fill `count` contiguous slots starting at `start_slot` in column `col` with `value`.
    ///
    /// Generic substrate helper: bounds-checked, finite-value validated, no CPU readback.
    /// Uses a single scalar queue write when `count == 1`; otherwise one bulk GPU fill dispatch.
    pub fn fill_slot_range_col(
        &self,
        ctx: &GpuContext,
        start_slot: u32,
        count: u32,
        col: u32,
        value: f32,
    ) -> Result<(), AccumulatorOpSessionError> {
        if count == 0 {
            return Ok(());
        }
        if !value.is_finite() {
            return Err(AccumulatorOpSessionError::NonFiniteValue);
        }
        if col >= self.n_dims {
            return Err(AccumulatorOpSessionError::InvalidColumn {
                col,
                n_dims: self.n_dims,
            });
        }
        if start_slot >= self.n_slots {
            return Err(AccumulatorOpSessionError::InvalidSlot {
                slot: start_slot,
                n_slots: self.n_slots,
            });
        }
        let end = start_slot.saturating_add(count);
        if end > self.n_slots || end < start_slot {
            return Err(AccumulatorOpSessionError::InvalidSlotRange {
                start_slot,
                count,
                n_slots: self.n_slots,
            });
        }

        if count == 1 {
            let offset = self.slot_col_byte_offset(start_slot, col);
            ctx.queue
                .write_buffer(&self.values_buffer, offset, bytemuck::bytes_of(&value));
            return Ok(());
        }

        let params = ValuesFillParams {
            start_slot,
            count,
            col,
            n_dims: self.n_dims,
            value,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        };
        ctx.queue
            .write_buffer(&self.fill_uniform, 0, bytemuck::bytes_of(&params));

        let bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("accumulator_values_fill_bind_group"),
            layout: &self.fill_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.values_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.fill_uniform.as_entire_binding(),
                },
            ],
        });

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_values_fill_encoder"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_values_fill_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.fill_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((count + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE, 1, 1);
        }
        ctx.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Write specific `(slot, col)` values into the values buffer via queue writes.
    pub fn write_slot_col_values(
        &self,
        ctx: &GpuContext,
        writes: &[(u32, u32, f32)],
    ) -> Result<(), AccumulatorOpSessionError> {
        for &(slot, col, value) in writes {
            if slot >= self.n_slots {
                return Err(AccumulatorOpSessionError::InvalidSlot {
                    slot,
                    n_slots: self.n_slots,
                });
            }
            if col >= self.n_dims {
                return Err(AccumulatorOpSessionError::InvalidColumn {
                    col,
                    n_dims: self.n_dims,
                });
            }
            let offset = self.slot_col_byte_offset(slot, col);
            ctx.queue
                .write_buffer(&self.values_buffer, offset, bytemuck::bytes_of(&value));
        }
        Ok(())
    }

    /// Authoritative resolved-values buffer (kernel-internal bind/readback only).
    pub(crate) fn values_buffer(&self) -> &Buffer {
        &self.values_buffer
    }

    /// Write max candidate-F magnitude bits into this session's values buffer.
    pub fn apply_candidate_f_exact_magnitude(
        &self,
        ctx: &GpuContext,
        request: crate::CandidateFMagnitudeRequest<'_>,
    ) -> Result<crate::CandidateFMagnitudeReport, crate::CandidateFMagnitudeError> {
        crate::candidate_f_magnitude::write_max_candidate_f_magnitude_bits(
            ctx,
            request.gradients,
            self.values_buffer(),
            request.target_slot,
            request.target_col,
            self.n_dims,
        )?;
        Ok(crate::CandidateFMagnitudeReport)
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

    fn resolve_eml_buffers<'a>(
        &'a self,
        eml: Option<(&'a Buffer, &'a Buffer)>,
    ) -> (&'a Buffer, &'a Buffer) {
        eml.unwrap_or((&self.eml_node_buffer, &self.eml_range_buffer))
    }

    /// Upload a packed bootstrap / pre-encoded accumulator op packet.
    pub fn upload_packed_ops(
        &mut self,
        ctx: &GpuContext,
        upload: &PackedAccumulatorUpload,
    ) -> Result<(), AccumulatorOpSessionError> {
        self.threshold_event_kinds.clear();
        self.write_input_list_buffer(ctx, upload.input_list())?;
        self.write_op_bytes(ctx, upload.ops())?;
        self.n_ops = upload.ops().len() as u32;
        Ok(())
    }

    /// Write packed ops without clearing threshold event-kind sidecar metadata.
    pub fn write_packed_op_buffer(
        &mut self,
        ctx: &GpuContext,
        upload: &PackedAccumulatorUpload,
    ) -> Result<(), AccumulatorOpSessionError> {
        if upload.ops().is_empty() {
            self.n_ops = 0;
            return Ok(());
        }
        self.write_op_bytes(ctx, upload.ops())?;
        self.n_ops = upload.ops().len() as u32;
        Ok(())
    }

    fn write_input_list_buffer(
        &mut self,
        ctx: &GpuContext,
        flat_inputs: &[AccumulatorInputGpu],
    ) -> Result<(), AccumulatorOpSessionError> {
        if flat_inputs.is_empty() {
            return Ok(());
        }
        let input_size = std::mem::size_of::<AccumulatorInputGpu>();
        let byte_len = flat_inputs.len().checked_mul(input_size).ok_or(
            AccumulatorOpSessionError::InputListUploadByteSizeOverflow {
                inputs: flat_inputs.len(),
                input_size,
            },
        )?;
        if self.input_list_buffer.size() < byte_len as u64 {
            self.input_list_buffer = ctx.device.create_buffer(&BufferDescriptor {
                label: Some("accumulator_input_list_buffer"),
                size: byte_len.max(std::mem::size_of::<AccumulatorInputGpu>()) as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        ctx.queue.write_buffer(
            &self.input_list_buffer,
            0,
            bytemuck::cast_slice(flat_inputs),
        );
        Ok(())
    }

    fn write_op_bytes(
        &mut self,
        ctx: &GpuContext,
        gpu_ops: &[AccumulatorOpGpu],
    ) -> Result<(), AccumulatorOpSessionError> {
        if gpu_ops.is_empty() {
            self.n_ops = 0;
            return Ok(());
        }
        let op_size = std::mem::size_of::<AccumulatorOpGpu>();
        let byte_len = gpu_ops.len().checked_mul(op_size).ok_or(
            AccumulatorOpSessionError::OpUploadByteSizeOverflow {
                ops: gpu_ops.len(),
                op_size,
            },
        )?;
        if self.op_buffer.size() < byte_len as u64 {
            self.op_buffer = ctx.device.create_buffer(&BufferDescriptor {
                label: Some("accumulator_op_buffer"),
                size: byte_len.max(4096) as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        ctx.queue
            .write_buffer(&self.op_buffer, 0, bytemuck::cast_slice(gpu_ops));
        Ok(())
    }

    /// Upload threshold-gated EmitEvent ops from a packed threshold packet (C-1).
    pub fn upload_packed_threshold_ops(
        &mut self,
        ctx: &GpuContext,
        upload: &PackedThresholdUpload,
    ) -> Result<(), AccumulatorOpSessionError> {
        self.write_op_bytes(ctx, upload.ops())?;
        self.n_ops = upload.ops().len() as u32;
        self.threshold_event_kinds = upload.threshold_event_kinds().to_vec();
        Ok(())
    }

    pub fn append_packed_threshold_ops(
        &mut self,
        ctx: &GpuContext,
        upload: &PackedThresholdUpload,
    ) -> Result<(), AccumulatorOpSessionError> {
        if upload.ops().is_empty() {
            return Ok(());
        }
        let gpu_ops = upload.ops();
        let op_size = std::mem::size_of::<AccumulatorOpGpu>();
        let old_count = self.n_ops as u64;
        let new_count = old_count + gpu_ops.len() as u64;
        let needed_bytes = new_count * op_size as u64;

        if self.op_buffer.size() < needed_bytes {
            let new_buffer = ctx.device.create_buffer(&BufferDescriptor {
                label: Some("accumulator_op_buffer"),
                size: needed_bytes.max(4096),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            if old_count > 0 {
                let mut encoder = ctx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("append_threshold_ops:preserve"),
                    });
                encoder.copy_buffer_to_buffer(
                    &self.op_buffer,
                    0,
                    &new_buffer,
                    0,
                    old_count * op_size as u64,
                );
                ctx.queue.submit(Some(encoder.finish()));
            }
            self.op_buffer = new_buffer;
        }

        ctx.queue.write_buffer(
            &self.op_buffer,
            old_count * op_size as u64,
            bytemuck::cast_slice(gpu_ops),
        );
        self.n_ops = new_count as u32;
        self.threshold_event_kinds
            .extend_from_slice(upload.threshold_event_kinds());
        Ok(())
    }

    pub fn ensure_threshold_emission_capacity(&mut self, ctx: &GpuContext, capacity: u32) {
        self.threshold_emission_readback
            .ensure_capacity(&ctx.device, capacity);
    }

    /// Upload folded intent deltas as a packed intent packet (C-2).
    pub fn upload_packed_intent_ops(
        &mut self,
        ctx: &GpuContext,
        upload: &PackedIntentUpload,
    ) -> Result<(), AccumulatorOpSessionError> {
        self.threshold_event_kinds.clear();
        if upload.ops().is_empty() {
            self.n_ops = 0;
            return Ok(());
        }
        self.write_op_bytes(ctx, upload.ops())?;
        self.n_ops = upload.ops().len() as u32;
        Ok(())
    }

    /// Number of GPU ops currently loaded in the op buffer.
    pub fn n_ops(&self) -> u32 {
        self.n_ops
    }

    /// Clear the op buffer without touching threshold event-kind sidecar metadata.
    pub fn clear_op_buffer(&mut self) {
        self.n_ops = 0;
    }

    /// Restore threshold event kinds after a non-threshold op upload.
    pub fn restore_threshold_event_kinds(&mut self, kinds: &[u32]) {
        self.threshold_event_kinds = kinds.to_vec();
    }

    /// Dispatch Pass B for one OrderBand, then refresh per-slot summaries.
    pub fn tick(&mut self, ctx: &GpuContext, band: u32) -> Result<(), AccumulatorOpSessionError> {
        self.tick_with_eml(ctx, band, None)
    }

    /// Dispatch with optional external EML program-table bindings.
    pub fn tick_with_eml(
        &mut self,
        ctx: &GpuContext,
        band: u32,
        eml: Option<&super::eml_program_table::EmlGpuProgramTable>,
    ) -> Result<(), AccumulatorOpSessionError> {
        let eml_bufs = eml.map(super::eml_program_table::EmlGpuProgramTable::bind_buffers);
        self.tick_with_eml_buffers(ctx, band, eml_bufs)
    }

    fn tick_with_eml_buffers(
        &mut self,
        ctx: &GpuContext,
        band: u32,
        eml: Option<(&Buffer, &Buffer)>,
    ) -> Result<(), AccumulatorOpSessionError> {
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
            eml,
        );

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_tick_encoder"),
            });

        let timestamp_writes =
            self.timestamp_query_set
                .as_ref()
                .map(|query_set| wgpu::ComputePassTimestampWrites {
                    query_set,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
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
        self.encode_threshold_scan_with_outputs_into(
            ctx,
            encoder,
            values,
            previous_values,
            previous_values,
            previous_values,
        );
    }

    pub fn encode_threshold_scan_with_outputs_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        output_values: &Buffer,
        previous_output_values: &Buffer,
    ) {
        if self.n_ops == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let execute_bind_group = self.create_execute_bind_group_with_threshold_buffers(
            ctx,
            values,
            previous_values,
            &self.tick_uniform,
            None,
            None,
            previous_output_values,
            output_values,
        );

        let timestamp_writes =
            self.timestamp_query_set
                .as_ref()
                .map(|query_set| wgpu::ComputePassTimestampWrites {
                    query_set,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
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

        let execute_bind_group = self.create_execute_bind_group(ctx, values, previous_values, None);

        let timestamp_writes =
            self.timestamp_query_set
                .as_ref()
                .map(|query_set| wgpu::ComputePassTimestampWrites {
                    query_set,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
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

    /// Prepare the session for a batched overlay OrderBand pass in the caller's encoder.
    pub fn prepare_overlay_add(&self, _ctx: &GpuContext) {
        // Band uniforms are written per band inside `encode_overlay_add_into`.
    }

    /// Encode overlay OrderBand ops into the command encoder at the overlay position.
    /// Dispatches `n_bands` OrderBand passes in ascending order within the encoder.
    /// Does NOT submit — caller owns the encoder and submits with other passes.
    pub fn encode_overlay_add_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        n_bands: u32,
    ) {
        if self.n_ops == 0 || n_bands == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
        let mut band_uniforms = Vec::with_capacity(n_bands as usize);

        for band in 0..n_bands {
            let tick_params = AccumulatorTickParams {
                n_ops: self.n_ops,
                current_band: band,
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                emission_capacity: self.emission_readback.capacity(),
                threshold_emission_capacity: self.threshold_emission_readback.capacity(),
                dt_bits: 0,
                _pad1: 0,
            };
            let tick_uniform = ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("accumulator_overlay_orderband_band_uniform"),
                    contents: bytemuck::bytes_of(&tick_params),
                    usage: BufferUsages::UNIFORM,
                });
            let execute_bind_group = self.create_execute_bind_group_with_uniform(
                ctx,
                values,
                previous_values,
                &tick_uniform,
                None,
                None,
            );
            band_uniforms.push(tick_uniform);

            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_overlay_orderband_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.execute_pipeline);
            pass.set_bind_group(0, &execute_bind_group, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }

        drop(band_uniforms);
    }

    /// Encode OrderBand ops with optional EML program-table bindings (E-11).
    pub fn encode_orderband_with_eml_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        n_bands: u32,
        dt: f32,
        eml: Option<&super::eml_program_table::EmlGpuProgramTable>,
    ) {
        let eml_bufs = eml.map(super::eml_program_table::EmlGpuProgramTable::bind_buffers);
        self.encode_orderband_with_eml_buffers_into(
            ctx,
            encoder,
            values,
            previous_values,
            n_bands,
            dt,
            eml_bufs,
        );
    }

    fn encode_orderband_with_eml_buffers_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        n_bands: u32,
        dt: f32,
        eml: Option<(&Buffer, &Buffer)>,
    ) {
        if self.n_ops == 0 || n_bands == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
        let mut band_uniforms = Vec::with_capacity(n_bands as usize);

        for band in 0..n_bands {
            let tick_params = AccumulatorTickParams {
                n_ops: self.n_ops,
                current_band: band,
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                emission_capacity: self.emission_readback.capacity(),
                threshold_emission_capacity: self.threshold_emission_readback.capacity(),
                dt_bits: dt.to_bits(),
                _pad1: 0,
            };
            let tick_uniform = ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("accumulator_orderband_eml_band_uniform"),
                    contents: bytemuck::bytes_of(&tick_params),
                    usage: BufferUsages::UNIFORM,
                });
            let execute_bind_group = self.create_execute_bind_group_with_uniform(
                ctx,
                values,
                previous_values,
                &tick_uniform,
                eml,
                None,
            );
            band_uniforms.push(tick_uniform);

            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_orderband_eml_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.execute_pipeline);
            pass.set_bind_group(0, &execute_bind_group, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }

        drop(band_uniforms);
    }

    /// AO-WGSL-0: encode OrderBand ops through the generic semantic-free fast path.
    ///
    /// Uses the dedicated `execute_orderband_bands` pipeline entry while preserving
    /// global band ordering: one band per dispatch, sequential bands in one compute
    /// pass. Performance: a single growable dynamic-offset uniform holds every band's
    /// params and a single bind group is reused across bands via per-band dynamic
    /// offsets — this avoids the O(n_bands) per-tick GPU buffer/bind-group allocation
    /// churn that dominates at endgame scale (deep hierarchies → many bands per tick).
    /// `tick_params._pad1` stores total band count for harness reporting.
    pub fn encode_orderband_fast_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        n_bands: u32,
        dt: f32,
        eml: Option<&super::eml_program_table::EmlGpuProgramTable>,
    ) {
        let eml_bufs = eml.map(super::eml_program_table::EmlGpuProgramTable::bind_buffers);
        self.encode_orderband_fast_buffers_into(
            ctx,
            encoder,
            values,
            previous_values,
            n_bands,
            dt,
            eml_bufs,
        );
    }

    fn encode_orderband_fast_buffers_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        n_bands: u32,
        dt: f32,
        eml: Option<(&Buffer, &Buffer)>,
    ) {
        if self.n_ops == 0 || n_bands == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let stride = ao_wgsl0_uniform_stride(&ctx.device);
        self.ensure_orderband_fast_uniform(ctx, n_bands, stride);

        let params_size = std::mem::size_of::<AccumulatorTickParams>();
        let mut params_bytes = vec![0u8; (stride * n_bands as u64) as usize];
        for band in 0..n_bands {
            let tick_params = AccumulatorTickParams {
                n_ops: self.n_ops,
                current_band: band,
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                emission_capacity: self.emission_readback.capacity(),
                threshold_emission_capacity: self.threshold_emission_readback.capacity(),
                dt_bits: dt.to_bits(),
                _pad1: n_bands,
            };
            let off = (band as u64 * stride) as usize;
            params_bytes[off..off + params_size].copy_from_slice(bytemuck::bytes_of(&tick_params));
        }
        ctx.queue
            .write_buffer(&self.orderband_fast_uniform, 0, &params_bytes);

        let bind_group = self.create_orderband_fast_bind_group(ctx, values, previous_values, eml);

        let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("accumulator_ao_wgsl0_fast_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.orderband_fast_pipeline);
        for band in 0..n_bands {
            let dyn_offset = (band as u64 * stride) as u32;
            pass.set_bind_group(0, &bind_group, &[dyn_offset]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
    }

    /// Grow the AO-WGSL-0 fast-path band-params uniform to hold `n_bands` entries.
    fn ensure_orderband_fast_uniform(&mut self, ctx: &GpuContext, n_bands: u32, stride: u64) {
        if self.orderband_fast_uniform_bands >= n_bands {
            return;
        }
        self.orderband_fast_uniform = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("accumulator_ao_wgsl0_fast_uniform"),
            size: stride * n_bands as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.orderband_fast_uniform_bands = n_bands;
    }

    /// Build the single AO-WGSL-0 fast-path bind group. Binding 4 is a dynamic-offset
    /// view of `orderband_fast_uniform` (one `AccumulatorTickParams` window). Bindings
    /// match the resource-flow execute layout exactly (previous/output values aliased
    /// to `previous_values`), so semantics are identical to the legacy path.
    fn create_orderband_fast_bind_group(
        &self,
        ctx: &GpuContext,
        values: &Buffer,
        previous_values: &Buffer,
        eml: Option<(&Buffer, &Buffer)>,
    ) -> wgpu::BindGroup {
        let (eml_nodes, eml_ranges) = self.resolve_eml_buffers(eml);
        let uniform_binding = wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer: &self.orderband_fast_uniform,
            offset: 0,
            size: Some(
                std::num::NonZeroU64::new(std::mem::size_of::<AccumulatorTickParams>() as u64)
                    .expect("AccumulatorTickParams is non-empty"),
            ),
        });
        ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("accumulator_ao_wgsl0_fast_bg"),
            layout: &self.orderband_fast_layout,
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
                    resource: self.emission_readback.records_binding().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.emission_readback.count_binding().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: uniform_binding,
                },
                BindGroupEntry {
                    binding: 5,
                    resource: previous_values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: self
                        .threshold_emission_readback
                        .records_binding()
                        .as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: self
                        .threshold_emission_readback
                        .count_binding()
                        .as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 8,
                    resource: eml_nodes.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 9,
                    resource: eml_ranges.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 10,
                    resource: self.input_list_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 11,
                    resource: previous_values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 12,
                    resource: previous_values.as_entire_binding(),
                },
            ],
        })
    }

    /// Encode C-5 soft-reduction OrderBand ops against `output_vectors`.
    /// Binding 1 reads/writes `output_vectors` (not `values`).
    pub fn encode_reduction_soft_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        output_vectors: &Buffer,
        n_bands: u32,
    ) {
        if self.n_ops == 0 || n_bands == 0 {
            return;
        }
        for band in 0..n_bands {
            self.encode_reduction_soft_band_into(ctx, encoder, output_vectors, band);
        }
    }

    /// Encode one C-5 soft-reduction OrderBand against `output_vectors`.
    pub fn encode_reduction_soft_band_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        output_vectors: &Buffer,
        band: u32,
    ) {
        if self.n_ops == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
        let tick_params = AccumulatorTickParams {
            n_ops: self.n_ops,
            current_band: band,
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            emission_capacity: self.emission_readback.capacity(),
            threshold_emission_capacity: self.threshold_emission_readback.capacity(),
            dt_bits: 0,
            _pad1: 0,
        };
        let tick_uniform = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("accumulator_reduction_soft_band_uniform"),
                contents: bytemuck::bytes_of(&tick_params),
                usage: BufferUsages::UNIFORM,
            });
        let execute_bind_group = self.create_execute_bind_group_with_uniform(
            ctx,
            output_vectors,
            &self.previous_values_buffer,
            &tick_uniform,
            None,
            None,
        );

        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("accumulator_reduction_soft_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.execute_pipeline);
        pass.set_bind_group(0, &execute_bind_group, &[]);
        pass.dispatch_workgroups(groups, 1, 1);
    }

    /// Encode C-7 velocity integration ops into the caller's command encoder.
    /// Runs after snapshot at the legacy Pass 1 position. `dt` is written to
    /// tick params for this dispatch only — uploaded ops stay dt-independent.
    pub fn encode_velocity_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        dt: f32,
    ) {
        if self.n_ops == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let total_invocations = (self.n_ops as u64) * (self.n_slots as u64);
        assert!(
            total_invocations <= u32::MAX as u64,
            "compact velocity dispatch exceeds u32 addressable range"
        );
        let max_groups_x = ctx.device.limits().max_compute_workgroups_per_dimension as u64;
        let max_invocations_per_chunk = max_groups_x * WORKGROUP_SIZE as u64;
        let mut base = 0u64;
        while base < total_invocations {
            let chunk_invocations = (total_invocations - base).min(max_invocations_per_chunk);
            let groups = chunk_invocations.div_ceil(WORKGROUP_SIZE as u64) as u32;
            let tick_params = AccumulatorTickParams {
                n_ops: self.n_ops,
                current_band: base as u32,
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                emission_capacity: self.emission_readback.capacity(),
                threshold_emission_capacity: self.threshold_emission_readback.capacity(),
                dt_bits: dt.to_bits(),
                _pad1: EXECUTE_MODE_COMPACT_VELOCITY,
            };
            let tick_uniform = ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("accumulator_velocity_tick_uniform"),
                    contents: bytemuck::bytes_of(&tick_params),
                    usage: BufferUsages::UNIFORM,
                });
            let execute_bind_group = self.create_execute_bind_group_with_uniform(
                ctx,
                values,
                previous_values,
                &tick_uniform,
                None,
                None,
            );

            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_velocity_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.execute_pipeline);
            pass.set_bind_group(0, &execute_bind_group, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
            base += chunk_invocations;
        }
    }

    /// Encode C-8b intensity EvalEML ops into the caller's command encoder.
    /// Runs after velocity and before overlay. `dt` is written to tick params only.
    pub fn encode_intensity_eml_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        dt: f32,
        eml: Option<(&Buffer, &Buffer)>,
    ) {
        if self.n_ops == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let tick_params = AccumulatorTickParams {
            n_ops: self.n_ops,
            current_band: 0,
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            emission_capacity: self.emission_readback.capacity(),
            threshold_emission_capacity: self.threshold_emission_readback.capacity(),
            dt_bits: dt.to_bits(),
            _pad1: 0,
        };
        let tick_uniform = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("accumulator_intensity_eml_tick_uniform"),
                contents: bytemuck::bytes_of(&tick_params),
                usage: BufferUsages::UNIFORM,
            });
        let execute_bind_group = self.create_execute_bind_group_with_uniform(
            ctx,
            values,
            previous_values,
            &tick_uniform,
            eml,
            None,
        );

        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("accumulator_intensity_eml_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.execute_pipeline);
        pass.set_bind_group(0, &execute_bind_group, &[]);
        let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
        pass.dispatch_workgroups(groups, 1, 1);
    }

    /// Encode C-8c transfer ops into the caller's command encoder (after intensity, before overlay).
    /// Dispatches `n_bands` OrderBand passes in ascending order within the encoder.
    pub fn encode_transfer_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        n_bands: u32,
        eml: Option<(&Buffer, &Buffer)>,
        input_list: Option<&Buffer>,
    ) {
        if self.n_ops == 0 || n_bands == 0 {
            return;
        }
        self.last_pass_time_us = None;

        let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
        let mut band_uniforms = Vec::with_capacity(n_bands as usize);

        for band in 0..n_bands {
            let tick_params = AccumulatorTickParams {
                n_ops: self.n_ops,
                current_band: band,
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                emission_capacity: self.emission_readback.capacity(),
                threshold_emission_capacity: self.threshold_emission_readback.capacity(),
                dt_bits: 0,
                _pad1: 0,
            };
            let tick_uniform = ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("accumulator_transfer_tick_uniform"),
                    contents: bytemuck::bytes_of(&tick_params),
                    usage: BufferUsages::UNIFORM,
                });
            let execute_bind_group = self.create_execute_bind_group_with_uniform(
                ctx,
                values,
                previous_values,
                &tick_uniform,
                eml,
                input_list,
            );
            band_uniforms.push(tick_uniform);

            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("accumulator_transfer_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.execute_pipeline);
            pass.set_bind_group(0, &execute_bind_group, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }

        drop(band_uniforms);
    }

    /// Encode C-8d emission ops into the caller's command encoder (after transfer, before overlay).
    pub fn encode_emission_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
        previous_values: &Buffer,
        dt: f32,
        eml: Option<(&Buffer, &Buffer)>,
    ) {
        if self.n_ops == 0 {
            return;
        }
        self.last_pass_time_us = None;
        self.reset_emission_count(ctx);

        let tick_params = AccumulatorTickParams {
            n_ops: self.n_ops,
            current_band: 0,
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            emission_capacity: self.emission_readback.capacity(),
            threshold_emission_capacity: self.threshold_emission_readback.capacity(),
            dt_bits: dt.to_bits(),
            _pad1: 0,
        };
        let tick_uniform = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("accumulator_emission_tick_uniform"),
                contents: bytemuck::bytes_of(&tick_params),
                usage: BufferUsages::UNIFORM,
            });
        let execute_bind_group = self.create_execute_bind_group_with_uniform(
            ctx,
            values,
            previous_values,
            &tick_uniform,
            eml,
            None,
        );

        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("accumulator_emission_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.execute_pipeline);
        pass.set_bind_group(0, &execute_bind_group, &[]);
        let groups = self.n_ops.div_ceil(WORKGROUP_SIZE);
        pass.dispatch_workgroups(groups, 1, 1);
    }

    fn write_tick_uniform(&self, ctx: &GpuContext, band: u32) {
        let tick_params = AccumulatorTickParams {
            n_ops: self.n_ops,
            current_band: band,
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            emission_capacity: self.emission_readback.capacity(),
            threshold_emission_capacity: self.threshold_emission_readback.capacity(),
            dt_bits: 0,
            _pad1: 0,
        };
        ctx.queue
            .write_buffer(&self.tick_uniform, 0, bytemuck::bytes_of(&tick_params));
    }

    fn create_execute_bind_group(
        &self,
        ctx: &GpuContext,
        values: &Buffer,
        previous_values: &Buffer,
        eml: Option<(&Buffer, &Buffer)>,
    ) -> wgpu::BindGroup {
        self.create_execute_bind_group_with_uniform(
            ctx,
            values,
            previous_values,
            &self.tick_uniform,
            eml,
            None,
        )
    }

    fn create_execute_bind_group_with_uniform(
        &self,
        ctx: &GpuContext,
        values: &Buffer,
        previous_values: &Buffer,
        tick_uniform: &Buffer,
        eml: Option<(&Buffer, &Buffer)>,
        input_list: Option<&Buffer>,
    ) -> wgpu::BindGroup {
        let (eml_nodes, eml_ranges) = self.resolve_eml_buffers(eml);
        let input_list_buf = input_list.unwrap_or(&self.input_list_buffer);
        self.create_execute_bind_group_with_threshold_buffers(
            ctx,
            values,
            previous_values,
            tick_uniform,
            Some((eml_nodes, eml_ranges)),
            Some(input_list_buf),
            previous_values,
            previous_values,
        )
    }

    fn create_execute_bind_group_with_threshold_buffers(
        &self,
        ctx: &GpuContext,
        values: &Buffer,
        previous_values: &Buffer,
        tick_uniform: &Buffer,
        resolved_eml: Option<(&Buffer, &Buffer)>,
        input_list: Option<&Buffer>,
        previous_output_values: &Buffer,
        output_values: &Buffer,
    ) -> wgpu::BindGroup {
        let (eml_nodes, eml_ranges) =
            resolved_eml.unwrap_or_else(|| self.resolve_eml_buffers(None));
        let input_list_buf = input_list.unwrap_or(&self.input_list_buffer);
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
                    resource: self.emission_readback.records_binding().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.emission_readback.count_binding().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: tick_uniform.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: previous_values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: self
                        .threshold_emission_readback
                        .records_binding()
                        .as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: self
                        .threshold_emission_readback
                        .count_binding()
                        .as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 8,
                    resource: eml_nodes.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 9,
                    resource: eml_ranges.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 10,
                    resource: input_list_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 11,
                    resource: previous_output_values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 12,
                    resource: output_values.as_entire_binding(),
                },
            ],
        })
    }

    /// Production B-4 summary readback tier (32 B/slot on GPU).
    pub fn readback_summary(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<SlotSummary>, AccumulatorOpSessionError> {
        let bytes = self.read_buffer_bytes(ctx, &self.summary_buffer);
        let gpu: &[SlotSummaryGpu] = bytemuck::cast_slice(&bytes);
        Ok(gpu
            .iter()
            .map(|s| SlotSummary {
                slot: s.slot,
                flags: s.flags,
                checksum_all: s.checksum_all,
                group_checksums: s.group_checksums,
            })
            .collect())
    }

    /// Read compact threshold crossing records written this tick (C-1).
    pub fn readback_threshold_emissions(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<ThresholdEmission>, AccumulatorOpSessionError> {
        self.threshold_emission_readback
            .read_threshold_emissions(&ctx.device, &ctx.queue)
            .map_err(AccumulatorOpSessionError::from)
    }

    /// Reconstruct Pass 7 `ThresholdEvent`s from compact threshold emissions.
    pub fn readback_threshold_events(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<ThresholdEvent>, AccumulatorOpSessionError> {
        self.threshold_emission_readback
            .read_threshold_events(&ctx.device, &ctx.queue, &self.threshold_event_kinds)
            .map_err(AccumulatorOpSessionError::from)
    }

    /// Total emission record attempts this tick (may exceed capacity on overflow).
    pub fn read_emission_count(&self, ctx: &GpuContext) -> Result<u32, AccumulatorOpSessionError> {
        Ok(self.emission_readback.read_count(&ctx.device, &ctx.queue))
    }

    /// Read up to `emission_capacity` records plus the total attempt count.
    pub fn readback_emissions_capped(
        &self,
        ctx: &GpuContext,
    ) -> Result<(u32, Vec<EmissionRecord>), AccumulatorOpSessionError> {
        self.emission_readback
            .read_records_capped(&ctx.device, &ctx.queue)
            .map_err(AccumulatorOpSessionError::from)
    }

    /// Read compact emission records written by EmitEvent ops this tick.
    pub fn readback_emissions(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<EmissionRecord>, AccumulatorOpSessionError> {
        self.emission_readback
            .read_records(&ctx.device, &ctx.queue)
            .map_err(AccumulatorOpSessionError::from)
    }

    /// Full values buffer readback — debug only unless explicitly allowed.
    ///
    /// The values buffer is stored as `atomic<i32>` on GPU (same bits as f32).
    /// Readback reinterprets i32 bits as f32 via cast_slice — this is exact,
    /// not an approximation.
    pub fn readback_full(&self, ctx: &GpuContext) -> Result<Vec<f32>, AccumulatorOpSessionError> {
        if !DEBUG_READBACK_ALLOWED.load(Ordering::Relaxed) {
            eprintln!("warning: AccumulatorOpSession::readback_full() called outside test mode");
        }
        Ok(self.read_buffer_f32(ctx, &self.values_buffer))
    }

    fn reset_emission_count(&self, ctx: &GpuContext) {
        self.emission_readback.reset_count(&ctx.queue);
    }

    fn reset_threshold_emission_count(&self, ctx: &GpuContext) {
        self.threshold_emission_readback.reset_count(&ctx.queue);
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

    fn dispatch_write_summaries(&self, ctx: &GpuContext, encoder: &mut wgpu::CommandEncoder) {
        self.dispatch_write_summaries_for_values(ctx, encoder, &self.values_buffer);
    }

    fn dispatch_write_summaries_for_values(
        &self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
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
                    resource: values.as_entire_binding(),
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

    /// Encode B-4 summaries against an external values buffer (integrated world path).
    pub fn encode_world_summaries_into(
        &self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &Buffer,
    ) {
        self.dispatch_write_summaries_for_values(ctx, encoder, values);
    }

    /// Standalone submit: write summaries for an external values buffer.
    pub(crate) fn dispatch_world_summaries(&self, ctx: &GpuContext, values: &Buffer) {
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("accumulator_world_summary_encoder"),
            });
        self.encode_world_summaries_into(ctx, &mut encoder, values);
        ctx.queue.submit(Some(encoder.finish()));
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

/// AO-WGSL-0: uniform binding with per-band dynamic offsets (fast-path layout).
fn uniform_entry_dynamic(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: true,
            min_binding_size: None,
        },
        count: None,
    }
}

/// AO-WGSL-0: device-aligned stride for one `AccumulatorTickParams` window in the
/// dynamic-offset band-params uniform. Dynamic offsets must be multiples of the
/// device `min_uniform_buffer_offset_alignment`.
fn ao_wgsl0_uniform_stride(device: &wgpu::Device) -> u64 {
    let align = device.limits().min_uniform_buffer_offset_alignment.max(1) as u64;
    let size = std::mem::size_of::<AccumulatorTickParams>() as u64;
    size.div_ceil(align) * align
}

fn mk_dummy_input_list_buffer(device: &wgpu::Device) -> Buffer {
    use super::types::AccumulatorInputGpu;
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("input_list_dummy"),
        contents: bytemuck::bytes_of(&AccumulatorInputGpu {
            slot: 0,
            col: 0,
            unit_cost_bits: 1.0f32.to_bits(),
            flags: 0,
        }),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    })
}

fn mk_dummy_eml_buffers(device: &wgpu::Device) -> (Buffer, Buffer) {
    let node = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("eml_dummy_node"),
        contents: bytemuck::bytes_of(&EmlNodeGpu {
            opcode: 0,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }),
        usage: BufferUsages::STORAGE,
    });
    let range = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("eml_dummy_range"),
        contents: bytemuck::bytes_of(&EmlTreeRangeGpu {
            node_offset: 0,
            node_count: 0,
            execution_class: 0,
            flags: 0,
        }),
        usage: BufferUsages::STORAGE,
    });
    (node, range)
}

#[cfg(test)]
mod tests {
    use simthing_core::{
        AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex,
        SourceSpec,
    };

    use crate::accumulator_op::encode::EncodeError;
    use crate::accumulator_op::{
        combine_kind, consume_kind, execute_ops_cpu, execute_threshold_ops_cpu, gate_kind,
        scale_kind, set_debug_readback_allowed, source_kind, summaries_from_values,
        threshold_registrations_to_ops, AccumulatorOpGpu, PackedAccumulatorUpload,
        PackedIntentUpload, PackedThresholdUpload,
    };
    use crate::context::GpuContext;
    use crate::{IntentDelta, ThresholdRegistration};

    use super::*;

    fn gpu_session(n_slots: u32, n_dims: u32) -> (GpuContext, AccumulatorOpSession) {
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let session = AccumulatorOpSession::new(&ctx, n_slots, n_dims);
        (ctx, session)
    }

    fn bootstrap_ops() -> Vec<AccumulatorOp> {
        vec![
            AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: SlotIndex::new(0),
                    col: ColumnIndex::new(0),
                },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Constant(3.0),
                consume: ConsumeMode::SubtractFromSource,
                targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
            },
            AccumulatorOp {
                source: SourceSpec::Constant(5.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(SlotIndex::new(2), ColumnIndex::new(0))],
            },
            AccumulatorOp {
                source: SourceSpec::SlotRange {
                    start: SlotIndex::new(3),
                    count: 2,
                    col: ColumnIndex::new(0),
                },
                combine: CombineFn::Sum,
                gate: GateSpec::OrderBand(1),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(SlotIndex::new(5), ColumnIndex::new(0))],
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
        session
            .upload_packed_ops(&ctx, &PackedAccumulatorUpload::from_ops(&ops).unwrap())
            .unwrap();
        session.tick(&ctx, 0).unwrap();
        session.tick(&ctx, 1).unwrap();

        let gpu_summaries = session.readback_summary(&ctx).unwrap();
        assert_eq!(gpu_summaries, expected_summaries);

        let gpu_values = session.readback_full(&ctx).unwrap();
        assert_eq!(gpu_values, expected);
    }

    fn trivial_1000_ops() -> Vec<AccumulatorOp> {
        (0..1000)
            .map(|i| AccumulatorOp {
                source: SourceSpec::Constant(1.0),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(SlotIndex::new(i), ColumnIndex::new(0))],
            })
            .collect()
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
        expected.sort_by_key(|e| (e.slot(), e.col(), e.reg_idx()));

        let mut session = AccumulatorOpSession::new_attached(&ctx, 3, n_dims, 16);
        session.upload_values(&ctx, &current);
        session.upload_previous_values(&ctx, &previous);
        session
            .upload_packed_threshold_ops(
                &ctx,
                &PackedThresholdUpload::from_registrations(&regs).unwrap(),
            )
            .unwrap();
        session.tick(&ctx, 0).unwrap();

        let mut gpu = session.readback_threshold_emissions(&ctx).unwrap();
        gpu.sort_by_key(|e| (e.slot(), e.col(), e.reg_idx()));
        assert_eq!(gpu.len(), 2);
        assert_eq!(gpu[0].slot(), 0);
        assert_eq!(gpu[1].slot(), 1);

        let events = session.readback_threshold_events(&ctx).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_kind(), kinds[0]);
        assert_eq!(events[1].event_kind(), kinds[1]);
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
                col: 0,
                mul,
                add,
            }];
            let mut expected = values.clone();
            execute_intent_deltas_cpu(&mut expected, &deltas, n_dims);

            let mut session = AccumulatorOpSession::new_attached(&ctx, 1, n_dims, 16);
            session.upload_values(&ctx, &values);
            session
                .upload_packed_intent_ops(&ctx, &PackedIntentUpload::from_deltas(&deltas).unwrap())
                .unwrap();
            session.tick(&ctx, 0).unwrap();
            assert_eq!(session.readback_full(&ctx).unwrap(), expected);
        };

        run_case(10.0, 2.0, 3.0);
        run_case(99.0, 0.0, 5.0);
        run_case(4.0, -1.0, 2.0);
    }

    #[test]
    fn governed_integration_executes_only_on_its_authored_orderband() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let initial = [10.0_f32, 2.0_f32];
        let values = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("governed_orderband_values"),
                contents: bytemuck::cast_slice(&initial),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            });
        let previous = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("governed_orderband_previous"),
                contents: bytemuck::cast_slice(&initial),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            });
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(1),
            },
            combine: CombineFn::IntegrateWithClamp {
                dt: 0.0,
                vel_max: 100.0,
                amount_min: f32::NEG_INFINITY,
                amount_max: f32::INFINITY,
            },
            gate: GateSpec::OrderBand(1),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![
                (SlotIndex::new(0), ColumnIndex::new(0)),
                (SlotIndex::new(0), ColumnIndex::new(1)),
            ],
        };
        let mut session = AccumulatorOpSession::new(&ctx, 1, 2);
        session
            .upload_packed_ops(&ctx, &PackedAccumulatorUpload::from_ops(&[op]).unwrap())
            .unwrap();

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("governed_nonmatching_orderband_encoder"),
            });
        session.encode_orderband_with_eml_into(
            &ctx,
            &mut encoder,
            &values,
            &previous,
            1,
            1.0,
            None,
        );
        ctx.queue.submit(Some(encoder.finish()));
        assert_eq!(
            session.read_buffer_f32(&ctx, &values),
            initial,
            "non-matching runtime band must mutate neither governed target"
        );

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("governed_matching_orderband_encoder"),
            });
        session.encode_orderband_with_eml_into(
            &ctx,
            &mut encoder,
            &values,
            &previous,
            2,
            1.0,
            None,
        );
        ctx.queue.submit(Some(encoder.finish()));
        assert_eq!(
            session.read_buffer_f32(&ctx, &values),
            vec![12.0, 2.0],
            "matching runtime band must integrate exactly once and retain eligible velocity"
        );
    }
}

//! Sparse, domain-free GPU execution for admitted ActionBand templates.
//!
//! This module owns numerical ActionBand state.  The CPU supplies immutable
//! descriptor tables at session build and the already-sealed Phase-5
//! [`BandCrossingDelta`] rows at a tick boundary; it never compares thresholds,
//! re-evaluates an ActionBand result, or chooses an emission destination.

use std::sync::mpsc;

use bytemuck::{Pod, Zeroable};
use simthing_core::EmlNodeGpu;
use thiserror::Error;
use wgpu::util::DeviceExt;

use crate::{debug_readback_allowed, BandCrossingDelta, EmlTreeRangeGpu, GpuContext};

pub const ACTIONBAND_NO_PROGRAM: u32 = u32::MAX;

pub mod target_kind {
    pub const POINT: u32 = 0;
    pub const SCALAR_AT_LEAST: u32 = 1;
    pub const SCALAR_AT_MOST: u32 = 2;
    pub const INTERVAL: u32 = 3;
    pub const AXIS_ALIGNED_BOX: u32 = 4;
    pub const LOCUS_RADIUS: u32 = 5;
    pub const PALMA_REACHABLE_SET: u32 = 6;
    pub const EML_PROJECTED_SET: u32 = 7;
}

/// Closed set of generic surfaces which an admitted binding may target.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionBandEmissionDestination {
    PropertyNext = 0,
    RfClaim = 1,
    CostBand = 2,
    OverlayEvent = 3,
    StructuralRequest = 4,
    Telemetry = 5,
}

impl ActionBandEmissionDestination {
    fn from_raw(raw: u32) -> Option<Self> {
        Some(match raw {
            0 => Self::PropertyNext,
            1 => Self::RfClaim,
            2 => Self::CostBand,
            3 => Self::OverlayEvent,
            4 => Self::StructuralRequest,
            5 => Self::Telemetry,
            _ => return None,
        })
    }
}

/// One row from the existing pre-admitted generic emission-binding table.
/// Destination kind is closed; ActionBand EML cannot manufacture or alter it.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ActionBandEmissionBindingGpu {
    destination_kind: u32,
    destination_index: u32,
    auxiliary0: u32,
    auxiliary1: u32,
}

impl ActionBandEmissionBindingGpu {
    pub fn new(
        destination: ActionBandEmissionDestination,
        destination_index: u32,
        auxiliary0: u32,
        auxiliary1: u32,
    ) -> Self {
        Self {
            destination_kind: destination as u32,
            destination_index,
            auxiliary0,
            auxiliary1,
        }
    }

    pub fn destination(self) -> ActionBandEmissionDestination {
        ActionBandEmissionDestination::from_raw(self.destination_kind)
            .expect("constructed from closed destination enum")
    }

    pub fn destination_index(self) -> u32 {
        self.destination_index
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandTemplateGpu {
    pub target_kind: u32,
    pub channel_start: u32,
    pub channel_count: u32,
    pub target_data_start: u32,
    pub projection_width: u32,
    pub band_start: u32,
    pub band_count: u32,
    pub membership_range: u32,
    pub projection_range: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandBandGpu {
    pub threshold_registration: u32,
    pub program_range: u32,
    pub binding_start: u32,
    pub binding_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandActiveInstanceGpu {
    pub slot: u32,
    pub template_index: u32,
    pub projection_start: u32,
    pub generation: u32,
    pub params: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandStateGpu {
    pub satisfied: u32,
    pub generation: u32,
    pub projection_start: u32,
    pub projection_len: u32,
    pub distance: f32,
    pub last_payload: f32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
struct ActionBandCrossingInputGpu {
    instance_row: u32,
    band_index: u32,
    output_start: u32,
    output_count: u32,
    post_value: f32,
    threshold: f32,
    reserved: [u32; 2],
}

/// GPU-authored fixed-surface emission.  All destination fields are copied
/// from the immutable admitted binding table, never from EML output.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandEmissionGpu {
    pub binding_index: u32,
    destination_kind: u32,
    pub destination_index: u32,
    pub generation: u32,
    pub value: f32,
    pub auxiliary0: u32,
    pub auxiliary1: u32,
    pub reserved: u32,
}

impl ActionBandEmissionGpu {
    pub fn destination(self) -> ActionBandEmissionDestination {
        ActionBandEmissionDestination::from_raw(self.destination_kind)
            .expect("GPU can only copy a validated admitted binding kind")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ActionBandDispatchParams {
    n_dims: u32,
    instance_count: u32,
    crossing_count: u32,
    emission_count: u32,
}

/// Label-free deterministic bucket formed from program and binding shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBandExecutionBucket {
    pub program_range: u32,
    pub destination_shape: Vec<ActionBandEmissionDestination>,
    pub band_indices: Vec<u32>,
}

/// Immutable, domain-free GPU tables compiled from the frozen admission product.
#[derive(Clone, Debug)]
pub struct ActionBandExecutionPlan {
    templates: Vec<ActionBandTemplateGpu>,
    target_channels: Vec<u32>,
    target_data: Vec<f32>,
    bands: Vec<ActionBandBandGpu>,
    band_binding_indices: Vec<u32>,
    emission_bindings: Vec<ActionBandEmissionBindingGpu>,
    eml_nodes: Vec<EmlNodeGpu>,
    eml_ranges: Vec<EmlTreeRangeGpu>,
    active_instances: Vec<ActionBandActiveInstanceGpu>,
    buckets: Vec<ActionBandExecutionBucket>,
    projection_floats: u32,
    fingerprint: u64,
}

impl ActionBandExecutionPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn from_admitted_numeric_tables(
        templates: Vec<ActionBandTemplateGpu>,
        target_channels: Vec<u32>,
        target_data: Vec<f32>,
        bands: Vec<ActionBandBandGpu>,
        band_binding_indices: Vec<u32>,
        emission_bindings: Vec<ActionBandEmissionBindingGpu>,
        eml_nodes: Vec<EmlNodeGpu>,
        eml_ranges: Vec<EmlTreeRangeGpu>,
        mut active_instances: Vec<ActionBandActiveInstanceGpu>,
        buckets: Vec<ActionBandExecutionBucket>,
        reserved_instance_rows: u32,
    ) -> Result<Self, ActionBandExecutionError> {
        if active_instances.len() > reserved_instance_rows as usize {
            return Err(ActionBandExecutionError::SparseRowBudgetExceeded {
                active: active_instances.len(),
                reserved: reserved_instance_rows,
            });
        }
        active_instances.sort_by_key(|row| (row.template_index, row.slot));
        if active_instances.windows(2).any(|rows| {
            (rows[0].template_index, rows[0].slot) == (rows[1].template_index, rows[1].slot)
        }) {
            return Err(ActionBandExecutionError::DuplicateActiveInstance);
        }

        let mut projection_floats = 0u32;
        for instance in &mut active_instances {
            let template = templates.get(instance.template_index as usize).ok_or(
                ActionBandExecutionError::UnknownTemplate(instance.template_index),
            )?;
            instance.projection_start = projection_floats;
            projection_floats = projection_floats
                .checked_add(template.projection_width)
                .ok_or(ActionBandExecutionError::TableOverflow)?;
        }

        validate_tables(
            &templates,
            &target_channels,
            &target_data,
            &bands,
            &band_binding_indices,
            &emission_bindings,
            &eml_ranges,
        )?;
        let fingerprint = plan_fingerprint(
            &templates,
            &target_channels,
            &target_data,
            &bands,
            &band_binding_indices,
            &emission_bindings,
            &eml_nodes,
            &eml_ranges,
            &active_instances,
        );
        Ok(Self {
            templates,
            target_channels,
            target_data,
            bands,
            band_binding_indices,
            emission_bindings,
            eml_nodes,
            eml_ranges,
            active_instances,
            buckets,
            projection_floats,
            fingerprint,
        })
    }

    pub fn active_instance_rows(&self) -> usize {
        self.active_instances.len()
    }

    pub fn hot_state_bytes(&self) -> usize {
        self.active_instances.len() * std::mem::size_of::<ActionBandStateGpu>()
            + self.projection_floats as usize * std::mem::size_of::<f32>()
    }

    pub fn buckets(&self) -> &[ActionBandExecutionBucket] {
        &self.buckets
    }

    pub fn numeric_fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// The only ActionBand crossing bridge. Input evidence is the existing
    /// sealed Phase-5 product; this method performs joins only, never compares.
    pub fn crossings_from_sealed(
        &self,
        deltas: &[BandCrossingDelta],
    ) -> Result<ActionBandCrossingBatch, ActionBandExecutionError> {
        let mut rows = Vec::new();
        let mut output_count = 0u32;
        for delta in deltas {
            for (band_index, band) in self.bands.iter().enumerate() {
                if band.threshold_registration != delta.reg_idx() {
                    continue;
                }
                for (instance_row, instance) in self.active_instances.iter().enumerate() {
                    let template = &self.templates[instance.template_index as usize];
                    let in_template = band_index as u32 >= template.band_start
                        && (band_index as u32) < template.band_start + template.band_count;
                    if in_template && instance.slot == delta.slot().raw() {
                        rows.push(ActionBandCrossingInputGpu {
                            instance_row: instance_row as u32,
                            band_index: band_index as u32,
                            output_start: output_count,
                            output_count: band.binding_count,
                            post_value: delta.post_value(),
                            threshold: delta.threshold(),
                            reserved: [0; 2],
                        });
                        output_count = output_count
                            .checked_add(band.binding_count)
                            .ok_or(ActionBandExecutionError::TableOverflow)?;
                    }
                }
            }
        }
        Ok(ActionBandCrossingBatch {
            rows,
            output_count,
            plan_fingerprint: self.fingerprint,
        })
    }
}

/// Opaque batch that cannot be forged from a second comparator or raw integers.
#[derive(Clone, Debug)]
pub struct ActionBandCrossingBatch {
    rows: Vec<ActionBandCrossingInputGpu>,
    output_count: u32,
    plan_fingerprint: u64,
}

impl ActionBandCrossingBatch {
    pub fn crossing_count(&self) -> usize {
        self.rows.len()
    }

    pub fn emission_count(&self) -> usize {
        self.output_count as usize
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionBandExecutionReadback {
    pub states: Vec<ActionBandStateGpu>,
    pub projection: Vec<f32>,
    pub emissions: Vec<ActionBandEmissionGpu>,
    /// Sum of ActionBand compute-pass GPU timestamps. `None` when the adapter
    /// does not expose timestamp queries; excludes copies and CPU readback.
    pub gpu_time_ns: Option<f64>,
}

#[derive(Debug, Error)]
pub enum ActionBandExecutionError {
    #[error("active ActionBand rows {active} exceed frozen reservation {reserved}")]
    SparseRowBudgetExceeded { active: usize, reserved: u32 },
    #[error("duplicate active ActionBand template/slot row")]
    DuplicateActiveInstance,
    #[error("ActionBand instance references unknown template {0}")]
    UnknownTemplate(u32),
    #[error("ActionBand descriptor table span is out of bounds")]
    InvalidTableSpan,
    #[error("ActionBand table size overflow")]
    TableOverflow,
    #[error("crossing batch belongs to a different immutable ActionBand plan")]
    ForeignCrossingBatch,
    #[error("ActionBand numerical readback is disabled outside an explicit proof scope")]
    ProofReadbackDisabled,
    #[error("GPU readback map failed")]
    MapFailed,
    #[error("ActionBand shader source markers are missing")]
    ShaderSourceMarkersMissing,
}

/// Zero-row execution has no GPU buffers and therefore zero hot bytes.
pub enum ActionBandGpuExecution {
    Inactive,
    Active(ActionBandGpuSession),
}

impl ActionBandGpuExecution {
    pub fn new(
        ctx: &GpuContext,
        plan: ActionBandExecutionPlan,
    ) -> Result<Self, ActionBandExecutionError> {
        if plan.active_instances.is_empty() {
            return Ok(Self::Inactive);
        }
        Ok(Self::Active(ActionBandGpuSession::new(ctx, plan)?))
    }

    pub fn active_instance_rows(&self) -> usize {
        match self {
            Self::Inactive => 0,
            Self::Active(session) => session.plan.active_instance_rows(),
        }
    }

    pub fn hot_state_bytes(&self) -> usize {
        match self {
            Self::Inactive => 0,
            Self::Active(session) => session.plan.hot_state_bytes(),
        }
    }
}

pub struct ActionBandGpuSession {
    plan: ActionBandExecutionPlan,
    layout: wgpu::BindGroupLayout,
    evaluate_pipeline: wgpu::ComputePipeline,
    emit_pipeline: wgpu::ComputePipeline,
    templates: wgpu::Buffer,
    target_channels: wgpu::Buffer,
    target_data: wgpu::Buffer,
    instances: wgpu::Buffer,
    state_current: wgpu::Buffer,
    state_next: wgpu::Buffer,
    projection_next: wgpu::Buffer,
    bands: wgpu::Buffer,
    band_binding_indices: wgpu::Buffer,
    emission_bindings: wgpu::Buffer,
    eml_nodes: wgpu::Buffer,
    eml_ranges: wgpu::Buffer,
    timestamp_query_set: Option<wgpu::QuerySet>,
    timestamp_resolve: Option<wgpu::Buffer>,
    timestamp_readback: Option<wgpu::Buffer>,
}

impl ActionBandGpuSession {
    fn new(
        ctx: &GpuContext,
        plan: ActionBandExecutionPlan,
    ) -> Result<Self, ActionBandExecutionError> {
        let shader_source = action_band_shader_source()?;
        let device = &ctx.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actionband_gpu_execution_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });
        let entries: Vec<_> = (0..16)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: if binding == 8 {
                        wgpu::BufferBindingType::Uniform
                    } else {
                        wgpu::BufferBindingType::Storage {
                            read_only: !matches!(binding, 5 | 6 | 7 | 13),
                        }
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("actionband_gpu_execution_bgl"),
            entries: &entries,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("actionband_gpu_execution_pl"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = |entry_point| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            })
        };
        let evaluate_pipeline = pipeline("actionband_evaluate");
        let emit_pipeline = pipeline("actionband_emit");

        let state = vec![ActionBandStateGpu::zeroed(); plan.active_instances.len()];
        let projection = vec![0.0f32; plan.projection_floats.max(1) as usize];
        let (timestamp_query_set, timestamp_resolve, timestamp_readback) =
            if ctx.timestamp_supported() {
                (
                    Some(device.create_query_set(&wgpu::QuerySetDescriptor {
                        label: Some("actionband_timestamp_query_set"),
                        ty: wgpu::QueryType::Timestamp,
                        count: 4,
                    })),
                    Some(device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("actionband_timestamp_resolve"),
                        size: 32,
                        usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                        mapped_at_creation: false,
                    })),
                    Some(staging(device, "actionband_timestamp_readback", 32)),
                )
            } else {
                (None, None, None)
            };
        Ok(Self {
            templates: storage(device, "actionband_templates", &plan.templates),
            target_channels: storage(device, "actionband_target_channels", &plan.target_channels),
            target_data: storage(device, "actionband_target_data", &plan.target_data),
            instances: storage(
                device,
                "actionband_active_instances",
                &plan.active_instances,
            ),
            state_current: storage_rw(device, "actionband_state_current", &state),
            state_next: storage_rw(device, "actionband_state_next", &state),
            projection_next: storage_rw(device, "actionband_projection_next", &projection),
            bands: storage(device, "actionband_bands", &plan.bands),
            band_binding_indices: storage(
                device,
                "actionband_band_binding_indices",
                &plan.band_binding_indices,
            ),
            emission_bindings: storage(
                device,
                "actionband_emission_bindings",
                &plan.emission_bindings,
            ),
            eml_nodes: storage(device, "actionband_eml_nodes", &plan.eml_nodes),
            eml_ranges: storage(device, "actionband_eml_ranges", &plan.eml_ranges),
            timestamp_query_set,
            timestamp_resolve,
            timestamp_readback,
            plan,
            layout,
            evaluate_pipeline,
            emit_pipeline,
        })
    }

    /// Dispatches against the authoritative world-state GPU buffer. StateCurrent
    /// is bound read-only and StateNext read-write, then the two private buffer
    /// owners are swapped only after submission.
    pub fn dispatch_and_readback(
        &mut self,
        ctx: &GpuContext,
        world_values: &wgpu::Buffer,
        n_dims: u32,
        crossings: &ActionBandCrossingBatch,
    ) -> Result<ActionBandExecutionReadback, ActionBandExecutionError> {
        if !debug_readback_allowed() {
            return Err(ActionBandExecutionError::ProofReadbackDisabled);
        }
        if crossings.plan_fingerprint != self.plan.fingerprint {
            return Err(ActionBandExecutionError::ForeignCrossingBatch);
        }
        let device = &ctx.device;
        let crossing_buffer = storage(device, "actionband_sealed_crossings", &crossings.rows);
        let emission_zeros =
            vec![ActionBandEmissionGpu::zeroed(); crossings.output_count.max(1) as usize];
        let emission_buffer = storage_rw(device, "actionband_emissions", &emission_zeros);
        let params = ActionBandDispatchParams {
            n_dims,
            instance_count: self.plan.active_instances.len() as u32,
            crossing_count: crossings.rows.len() as u32,
            emission_count: crossings.output_count,
        };
        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_dispatch_params"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let resources = [
            &self.templates,
            &self.target_channels,
            &self.target_data,
            &self.instances,
            &self.state_current,
            &self.state_next,
            &self.projection_next,
            world_values,
            &params_buffer,
            &self.bands,
            &self.band_binding_indices,
            &self.emission_bindings,
            &crossing_buffer,
            &emission_buffer,
            &self.eml_nodes,
            &self.eml_ranges,
        ];
        let bind_entries: Vec<_> = resources
            .iter()
            .enumerate()
            .map(|(binding, buffer)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: buffer.as_entire_binding(),
            })
            .collect();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("actionband_gpu_execution_bg"),
            layout: &self.layout,
            entries: &bind_entries,
        });

        let state_bytes =
            (self.plan.active_instances.len() * std::mem::size_of::<ActionBandStateGpu>()) as u64;
        let projection_bytes =
            (self.plan.projection_floats.max(1) as usize * std::mem::size_of::<f32>()) as u64;
        let emission_bytes = (crossings.output_count.max(1) as usize
            * std::mem::size_of::<ActionBandEmissionGpu>()) as u64;
        let state_stage = staging(device, "actionband_state_readback", state_bytes);
        let projection_stage = staging(device, "actionband_projection_readback", projection_bytes);
        let emission_stage = staging(device, "actionband_emission_readback", emission_bytes);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("actionband_gpu_execution_encoder"),
        });
        {
            let timestamp_writes = self.timestamp_query_set.as_ref().map(|query_set| {
                wgpu::ComputePassTimestampWrites {
                    query_set,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
                }
            });
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("actionband_target_evaluation"),
                timestamp_writes,
            });
            pass.set_pipeline(&self.evaluate_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((params.instance_count + 63) / 64, 1, 1);
        }
        if params.crossing_count > 0 {
            let timestamp_writes = self.timestamp_query_set.as_ref().map(|query_set| {
                wgpu::ComputePassTimestampWrites {
                    query_set,
                    beginning_of_pass_write_index: Some(2),
                    end_of_pass_write_index: Some(3),
                }
            });
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("actionband_sealed_crossing_emission"),
                timestamp_writes,
            });
            pass.set_pipeline(&self.emit_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((params.crossing_count + 63) / 64, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&self.state_next, 0, &state_stage, 0, state_bytes);
        encoder.copy_buffer_to_buffer(
            &self.projection_next,
            0,
            &projection_stage,
            0,
            projection_bytes,
        );
        encoder.copy_buffer_to_buffer(&emission_buffer, 0, &emission_stage, 0, emission_bytes);
        let timestamp_count = if params.crossing_count > 0 { 4 } else { 2 };
        if let (Some(query_set), Some(resolve), Some(readback)) = (
            self.timestamp_query_set.as_ref(),
            self.timestamp_resolve.as_ref(),
            self.timestamp_readback.as_ref(),
        ) {
            encoder.resolve_query_set(query_set, 0..timestamp_count, resolve, 0);
            encoder.copy_buffer_to_buffer(resolve, 0, readback, 0, u64::from(timestamp_count) * 8);
        }
        ctx.queue.submit(Some(encoder.finish()));

        let states =
            readback::<ActionBandStateGpu>(device, &state_stage, self.plan.active_instances.len())?;
        let projection = readback::<f32>(
            device,
            &projection_stage,
            self.plan.projection_floats as usize,
        )?;
        let emissions = readback::<ActionBandEmissionGpu>(
            device,
            &emission_stage,
            crossings.output_count as usize,
        )?;
        let gpu_time_ns = if let Some(timestamp_buffer) = self.timestamp_readback.as_ref() {
            let stamps = readback::<u64>(device, timestamp_buffer, timestamp_count as usize)?;
            let ticks = (stamps[1] - stamps[0])
                + if timestamp_count == 4 {
                    stamps[3] - stamps[2]
                } else {
                    0
                };
            Some(ticks as f64 * ctx.timestamp_period_ns() as f64)
        } else {
            None
        };
        std::mem::swap(&mut self.state_current, &mut self.state_next);
        Ok(ActionBandExecutionReadback {
            states,
            projection,
            emissions,
            gpu_time_ns,
        })
    }
}

fn validate_tables(
    templates: &[ActionBandTemplateGpu],
    target_channels: &[u32],
    target_data: &[f32],
    bands: &[ActionBandBandGpu],
    band_binding_indices: &[u32],
    emission_bindings: &[ActionBandEmissionBindingGpu],
    eml_ranges: &[EmlTreeRangeGpu],
) -> Result<(), ActionBandExecutionError> {
    for template in templates {
        let channels_end = template.channel_start as usize + template.channel_count as usize;
        if channels_end > target_channels.len() {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
        let data_len = match template.target_kind {
            target_kind::POINT => template.projection_width,
            target_kind::SCALAR_AT_LEAST
            | target_kind::SCALAR_AT_MOST
            | target_kind::LOCUS_RADIUS
            | target_kind::PALMA_REACHABLE_SET => 1,
            target_kind::INTERVAL => 2,
            target_kind::AXIS_ALIGNED_BOX => template.projection_width * 2,
            target_kind::EML_PROJECTED_SET => 0,
            _ => return Err(ActionBandExecutionError::InvalidTableSpan),
        };
        if template.target_data_start as usize + data_len as usize > target_data.len()
            || template.band_start as usize + template.band_count as usize > bands.len()
        {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
        for range in [template.membership_range, template.projection_range] {
            if range != ACTIONBAND_NO_PROGRAM && range as usize >= eml_ranges.len() {
                return Err(ActionBandExecutionError::InvalidTableSpan);
            }
        }
    }
    for band in bands {
        if band.binding_start as usize + band.binding_count as usize > band_binding_indices.len()
            || (band.program_range != ACTIONBAND_NO_PROGRAM
                && band.program_range as usize >= eml_ranges.len())
        {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
    }
    if band_binding_indices
        .iter()
        .any(|&i| i as usize >= emission_bindings.len())
    {
        return Err(ActionBandExecutionError::InvalidTableSpan);
    }
    if emission_bindings
        .iter()
        .any(|b| ActionBandEmissionDestination::from_raw(b.destination_kind).is_none())
    {
        return Err(ActionBandExecutionError::InvalidTableSpan);
    }
    Ok(())
}

fn plan_fingerprint(
    templates: &[ActionBandTemplateGpu],
    target_channels: &[u32],
    target_data: &[f32],
    bands: &[ActionBandBandGpu],
    binding_indices: &[u32],
    emission_bindings: &[ActionBandEmissionBindingGpu],
    eml_nodes: &[EmlNodeGpu],
    eml_ranges: &[EmlTreeRangeGpu],
    instances: &[ActionBandActiveInstanceGpu],
) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for bytes in [
        bytemuck::cast_slice::<_, u8>(templates),
        bytemuck::cast_slice::<_, u8>(target_channels),
        bytemuck::cast_slice::<_, u8>(target_data),
        bytemuck::cast_slice::<_, u8>(bands),
        bytemuck::cast_slice::<_, u8>(binding_indices),
        bytemuck::cast_slice::<_, u8>(emission_bindings),
        bytemuck::cast_slice::<_, u8>(eml_nodes),
        bytemuck::cast_slice::<_, u8>(eml_ranges),
        bytemuck::cast_slice::<_, u8>(instances),
    ] {
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}

fn action_band_shader_source() -> Result<String, ActionBandExecutionError> {
    let canonical = include_str!("../shaders/accumulator_op.wgsl");
    let start = canonical
        .find("struct EmlNodeGpu {")
        .ok_or(ActionBandExecutionError::ShaderSourceMarkersMissing)?;
    let end = canonical
        .find("fn atomic_add_f32_at")
        .ok_or(ActionBandExecutionError::ShaderSourceMarkersMissing)?;
    let shared_eml = &canonical[start..end];
    Ok(format!(
        "{shared_eml}\n{}",
        include_str!("../shaders/action_band_execution.wgsl")
    ))
}

fn storage<T: Pod>(device: &wgpu::Device, label: &str, rows: &[T]) -> wgpu::Buffer {
    let zero = [0u32; 8];
    let contents = if rows.is_empty() {
        bytemuck::cast_slice(&zero)
    } else {
        bytemuck::cast_slice(rows)
    };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents,
        usage: wgpu::BufferUsages::STORAGE,
    })
}

fn storage_rw<T: Pod>(device: &wgpu::Device, label: &str, rows: &[T]) -> wgpu::Buffer {
    let zero = [0u32; 8];
    let contents = if rows.is_empty() {
        bytemuck::cast_slice(&zero)
    } else {
        bytemuck::cast_slice(rows)
    };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    })
}

fn staging(device: &wgpu::Device, label: &str, size: u64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn readback<T: Pod>(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    len: usize,
) -> Result<Vec<T>, ActionBandExecutionError> {
    if len == 0 {
        return Ok(Vec::new());
    }
    let byte_len = (len * std::mem::size_of::<T>()) as u64;
    let slice = buffer.slice(..byte_len);
    let (tx, rx) = mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    device.poll(wgpu::Maintain::Wait);
    rx.recv()
        .ok()
        .and_then(Result::ok)
        .ok_or(ActionBandExecutionError::MapFailed)?;
    let mapped = slice.get_mapped_range();
    let result = bytemuck::cast_slice(&mapped).to_vec();
    drop(mapped);
    buffer.unmap();
    Ok(result)
}

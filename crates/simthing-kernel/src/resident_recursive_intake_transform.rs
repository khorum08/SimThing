//! Resident Current-to-Next demand mint.
//!
//! This stage reads immutable canonical `T_s.U`, applies the optional admitted
//! `f(U)`, adds authored N+1 demand, and emits an ordinary demand row. It never
//! writes a `T_s`-typed payload and never executes N+1 economics.

use bytemuck::{Pod, Zeroable};
use simthing_core::EmlNodeGpu;
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferUsages, CommandEncoder, ComputePassDescriptor,
    ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, ShaderModuleDescriptor,
    ShaderSource, ShaderStages,
};

use crate::{GpuContext, ResidentApportionmentPlan};

const PRODUCT_WORDS: u64 = 8;
const DEMAND_WORDS: u64 = 4;
const WORKGROUP_SIZE: u32 = 64;
const DEMAND_STATUS_OK: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct TransformParamsGpu {
    row_count: u32,
    input_start_row: u32,
    demand_generation: u32,
    _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct TransformMetaGpu {
    node_offset: u32,
    node_count: u32,
    cap: u32,
    is_bound: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
struct AuthoredDemandGpu {
    source_simthing_id_raw: u32,
    quantity: u32,
}

/// Ordinary demand emitted by the one Current-to-Next mint. This is
/// deliberately not layout-compatible with `ResidentConstrainedProduct`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Pod, Zeroable)]
pub struct ResidentTemporalDemand {
    source_simthing_id_raw: u32,
    quantity: u32,
    generation: u32,
    status: u32,
}

impl ResidentTemporalDemand {
    pub fn source_simthing_id(self) -> simthing_core::SimThingId {
        simthing_core::SimThingId::from_session_raw(self.source_simthing_id_raw)
    }

    pub const fn quantity(self) -> u32 {
        self.quantity
    }

    pub const fn generation(self) -> simthing_core::GenerationStamp {
        simthing_core::GenerationStamp::new(self.generation)
    }

    pub const fn is_successful(self) -> bool {
        self.status == DEMAND_STATUS_OK
    }
}

/// Stateless encoder for the optional authored transform within the one
/// resident Current-to-Next mint. Callers retain no writable policy state.
pub struct ResidentTemporalDemandMintSession {
    layout: wgpu::BindGroupLayout,
    pipeline: ComputePipeline,
}

impl ResidentTemporalDemandMintSession {
    pub fn new(ctx: &GpuContext) -> Self {
        let layout = ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("resident_temporal_demand_mint_layout"),
                entries: &[
                    uniform_layout_entry(0),
                    storage_layout_entry(1, true),
                    storage_layout_entry(2, false),
                    storage_layout_entry(3, true),
                    storage_layout_entry(4, true),
                    storage_layout_entry(5, true),
                ],
            });
        let shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("resident_temporal_demand_mint"),
            source: ShaderSource::Wgsl(
                include_str!("shaders/resident_recursive_intake_transform.wgsl").into(),
            ),
        });
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("resident_temporal_demand_mint_pipeline_layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("resident_temporal_demand_mint_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: "mint_temporal_demand",
                compilation_options: Default::default(),
                cache: None,
            });
        Self { layout, pipeline }
    }

    pub fn encode(
        &self,
        ctx: &GpuContext,
        encoder: &mut CommandEncoder,
        input: &Buffer,
        input_start_row: u32,
        output: &Buffer,
        plan: &ResidentApportionmentPlan,
        authored_demands: &[u32],
        demand_generation: simthing_core::GenerationStamp,
    ) -> Result<(), ResidentTemporalDemandMintError> {
        let row_count = u32::try_from(plan.claims().len())
            .map_err(|_| ResidentTemporalDemandMintError::RowCountNarrowing)?;
        if authored_demands.len() != row_count as usize {
            return Err(
                ResidentTemporalDemandMintError::AuthoredDemandCountMismatch {
                    expected: row_count,
                    observed: authored_demands.len(),
                },
            );
        }
        let expected_generation = plan
            .generation()
            .get()
            .checked_add(1)
            .ok_or(ResidentTemporalDemandMintError::GenerationOverflow)?;
        if demand_generation.get() != expected_generation {
            return Err(ResidentTemporalDemandMintError::DemandGenerationMismatch {
                expected: simthing_core::GenerationStamp::new(expected_generation),
                observed: demand_generation,
            });
        }
        let product_bytes = PRODUCT_WORDS * 4;
        let required_input = u64::from(input_start_row)
            .checked_add(u64::from(row_count))
            .and_then(|rows| rows.checked_mul(product_bytes))
            .ok_or(ResidentTemporalDemandMintError::ArithmeticOverflow)?;
        let required_output = u64::from(row_count)
            .checked_mul(DEMAND_WORDS * 4)
            .ok_or(ResidentTemporalDemandMintError::ArithmeticOverflow)?;
        if input.size() < required_input {
            return Err(ResidentTemporalDemandMintError::InputBufferTooSmall {
                required: required_input,
                observed: input.size(),
            });
        }
        if output.size() < required_output {
            return Err(ResidentTemporalDemandMintError::OutputBufferTooSmall {
                required: required_output,
                observed: output.size(),
            });
        }

        let mut metadata = vec![TransformMetaGpu::zeroed(); row_count as usize];
        let mut nodes = Vec::new();
        for (physical, claim) in plan.claims().iter().enumerate() {
            let Some(program) = plan.persistence_deformation(claim.semantic_row()) else {
                continue;
            };
            let node_offset = u32::try_from(nodes.len())
                .map_err(|_| ResidentTemporalDemandMintError::NodeCountNarrowing)?;
            let node_count = u32::try_from(program.value_program().nodes().len())
                .map_err(|_| ResidentTemporalDemandMintError::NodeCountNarrowing)?;
            nodes.extend_from_slice(program.value_program().nodes());
            metadata[physical] = TransformMetaGpu {
                node_offset,
                node_count,
                cap: program.cap(),
                is_bound: 1,
            };
        }
        // wgpu storage bindings cannot be empty. Unbound rows never inspect
        // this inert node.
        if nodes.is_empty() {
            nodes.push(EmlNodeGpu::zeroed());
        }

        let authored: Vec<_> = plan
            .claims()
            .iter()
            .zip(authored_demands)
            .map(|(claim, quantity)| AuthoredDemandGpu {
                source_simthing_id_raw: claim.source_simthing_id().raw(),
                quantity: *quantity,
            })
            .collect();

        let params = TransformParamsGpu {
            row_count,
            input_start_row,
            demand_generation: demand_generation.get(),
            _pad1: 0,
        };
        let params_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident_temporal_demand_mint_params"),
                contents: bytemuck::bytes_of(&params),
                usage: BufferUsages::UNIFORM,
            });
        let metadata_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident_temporal_demand_mint_meta"),
                contents: bytemuck::cast_slice(&metadata),
                usage: BufferUsages::STORAGE,
            });
        let node_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident_temporal_demand_mint_nodes"),
                contents: bytemuck::cast_slice::<EmlNodeGpu, u8>(&nodes),
                usage: BufferUsages::STORAGE,
            });
        let authored_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident_temporal_authored_demands"),
                contents: bytemuck::cast_slice(&authored),
                usage: BufferUsages::STORAGE,
            });
        let bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("resident_temporal_demand_mint_bind_group"),
            layout: &self.layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: input.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: output.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: metadata_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: node_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: authored_buffer.as_entire_binding(),
                },
            ],
        });
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("resident_temporal_demand_mint"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(row_count.div_ceil(WORKGROUP_SIZE), 1, 1);
        Ok(())
    }
}

fn uniform_layout_entry(binding: u32) -> BindGroupLayoutEntry {
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

fn storage_layout_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
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

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResidentTemporalDemandMintError {
    #[error("resident temporal demand row count cannot narrow to u32")]
    RowCountNarrowing,
    #[error("resident temporal mint has {observed} authored rows, expected {expected}")]
    AuthoredDemandCountMismatch { expected: u32, observed: usize },
    #[error("resident temporal demand transform node count cannot narrow to u32")]
    NodeCountNarrowing,
    #[error("resident temporal demand mint arithmetic overflow")]
    ArithmeticOverflow,
    #[error("resident temporal demand generation overflow")]
    GenerationOverflow,
    #[error("resident temporal demand generation is {observed:?}, expected {expected:?}")]
    DemandGenerationMismatch {
        expected: simthing_core::GenerationStamp,
        observed: simthing_core::GenerationStamp,
    },
    #[error("resident temporal demand source requires {required} bytes, found {observed}")]
    InputBufferTooSmall { required: u64, observed: u64 },
    #[error("resident temporal demand output requires {required} bytes, found {observed}")]
    OutputBufferTooSmall { required: u64, observed: u64 },
}

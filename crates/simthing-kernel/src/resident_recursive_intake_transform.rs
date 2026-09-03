//! Optional bounded EML transform at the resident Current-to-Next intake mint.
//!
//! The graduated exact clearing shader remains untouched. This generic stage
//! copies its canonical products into the already-existing recursive intake,
//! replacing only an admitted row's unresolved quantity with `f(U)`.

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
const WORKGROUP_SIZE: u32 = 64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct TransformParamsGpu {
    row_count: u32,
    input_start_row: u32,
    _pad0: u32,
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

/// Stateless encoder for the optional authored transform within the one
/// resident Current-to-Next mint. Callers retain no writable policy state.
pub struct ResidentRecursiveIntakeTransformSession {
    layout: wgpu::BindGroupLayout,
    pipeline: ComputePipeline,
}

impl ResidentRecursiveIntakeTransformSession {
    pub fn new(ctx: &GpuContext) -> Self {
        let layout = ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("resident_recursive_intake_transform_layout"),
                entries: &[
                    uniform_layout_entry(0),
                    storage_layout_entry(1, true),
                    storage_layout_entry(2, false),
                    storage_layout_entry(3, true),
                    storage_layout_entry(4, true),
                ],
            });
        let shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("resident_recursive_intake_transform"),
            source: ShaderSource::Wgsl(
                include_str!("shaders/resident_recursive_intake_transform.wgsl").into(),
            ),
        });
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("resident_recursive_intake_transform_pipeline_layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("resident_recursive_intake_transform_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: "mint_recursive_intake",
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
    ) -> Result<(), ResidentRecursiveIntakeTransformError> {
        let row_count = u32::try_from(plan.claims().len())
            .map_err(|_| ResidentRecursiveIntakeTransformError::RowCountNarrowing)?;
        let product_bytes = PRODUCT_WORDS * 4;
        let required_input = u64::from(input_start_row)
            .checked_add(u64::from(row_count))
            .and_then(|rows| rows.checked_mul(product_bytes))
            .ok_or(ResidentRecursiveIntakeTransformError::ArithmeticOverflow)?;
        let required_output = u64::from(row_count)
            .checked_mul(product_bytes)
            .ok_or(ResidentRecursiveIntakeTransformError::ArithmeticOverflow)?;
        if input.size() < required_input {
            return Err(ResidentRecursiveIntakeTransformError::InputBufferTooSmall {
                required: required_input,
                observed: input.size(),
            });
        }
        if output.size() < required_output {
            return Err(
                ResidentRecursiveIntakeTransformError::OutputBufferTooSmall {
                    required: required_output,
                    observed: output.size(),
                },
            );
        }

        let mut metadata = vec![TransformMetaGpu::zeroed(); row_count as usize];
        let mut nodes = Vec::new();
        for (physical, claim) in plan.claims().iter().enumerate() {
            let Some(program) = plan.persistence_deformation(claim.semantic_row()) else {
                continue;
            };
            let node_offset = u32::try_from(nodes.len())
                .map_err(|_| ResidentRecursiveIntakeTransformError::NodeCountNarrowing)?;
            let node_count = u32::try_from(program.value_program().nodes().len())
                .map_err(|_| ResidentRecursiveIntakeTransformError::NodeCountNarrowing)?;
            nodes.extend_from_slice(program.value_program().nodes());
            metadata[physical] = TransformMetaGpu {
                node_offset,
                node_count,
                cap: program.cap(),
                is_bound: 1,
            };
        }
        if nodes.is_empty() {
            return Err(ResidentRecursiveIntakeTransformError::NoBoundPrograms);
        }

        let params = TransformParamsGpu {
            row_count,
            input_start_row,
            _pad0: 0,
            _pad1: 0,
        };
        let params_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident_recursive_intake_transform_params"),
                contents: bytemuck::bytes_of(&params),
                usage: BufferUsages::UNIFORM,
            });
        let metadata_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident_recursive_intake_transform_meta"),
                contents: bytemuck::cast_slice(&metadata),
                usage: BufferUsages::STORAGE,
            });
        let node_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident_recursive_intake_transform_nodes"),
                contents: bytemuck::cast_slice::<EmlNodeGpu, u8>(&nodes),
                usage: BufferUsages::STORAGE,
            });
        let bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("resident_recursive_intake_transform_bind_group"),
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
            ],
        });
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("resident_recursive_intake_transform"),
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
pub enum ResidentRecursiveIntakeTransformError {
    #[error("resident recursive-intake transform row count cannot narrow to u32")]
    RowCountNarrowing,
    #[error("resident recursive-intake transform node count cannot narrow to u32")]
    NodeCountNarrowing,
    #[error("resident recursive-intake transform arithmetic overflow")]
    ArithmeticOverflow,
    #[error(
        "resident recursive-intake transform input requires {required} bytes, found {observed}"
    )]
    InputBufferTooSmall { required: u64, observed: u64 },
    #[error(
        "resident recursive-intake transform output requires {required} bytes, found {observed}"
    )]
    OutputBufferTooSmall { required: u64, observed: u64 },
    #[error("resident recursive-intake transform invoked without a bound program")]
    NoBoundPrograms,
}

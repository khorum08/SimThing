//! Generic indexed f32 gather-scatter (CT-3b+4a GPU pressure projection).
//!
//! Moves values between two storage buffers by explicit `(src, dst)` index
//! pairs in one dispatch — pure bounded data movement; all meaning is pinned
//! at the spec/driver layer that compiled the entries. Host-side validation
//! rejects out-of-bounds indices before anything reaches the GPU.

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::GpuContext;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ScatterEntry {
    pub src_index: u32,
    pub dst_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ScatterParams {
    n_entries: u32,
    _pad: [u32; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, thiserror::Error)]
pub enum IndexedScatterError {
    #[error("scatter entry {entry} src index {index} exceeds source length {len}")]
    SrcOutOfBounds { entry: usize, index: u32, len: u64 },
    #[error("scatter entry {entry} dst index {index} exceeds destination length {len}")]
    DstOutOfBounds { entry: usize, index: u32, len: u64 },
    #[error("duplicate scatter destination index {index}")]
    DuplicateDst { index: u32 },
    #[error("scatter entries must not be empty")]
    EmptyEntries,
}

/// Validate entries against buffer lengths (in f32 elements) and reject
/// duplicate destinations (write-order would be dispatch-dependent).
pub fn validate_scatter_entries(
    entries: &[ScatterEntry],
    src_len: u64,
    dst_len: u64,
) -> Result<(), IndexedScatterError> {
    let mut seen = std::collections::BTreeSet::new();
    for (i, entry) in entries.iter().enumerate() {
        if u64::from(entry.src_index) >= src_len {
            return Err(IndexedScatterError::SrcOutOfBounds {
                entry: i,
                index: entry.src_index,
                len: src_len,
            });
        }
        if u64::from(entry.dst_index) >= dst_len {
            return Err(IndexedScatterError::DstOutOfBounds {
                entry: i,
                index: entry.dst_index,
                len: dst_len,
            });
        }
        if !seen.insert(entry.dst_index) {
            return Err(IndexedScatterError::DuplicateDst {
                index: entry.dst_index,
            });
        }
    }
    Ok(())
}

/// CPU oracle: the same move applied to host slices.
pub fn cpu_scatter_indexed(src: &[f32], dst: &mut [f32], entries: &[ScatterEntry]) {
    for entry in entries {
        dst[entry.dst_index as usize] = src[entry.src_index as usize];
    }
}

/// One-dispatch indexed scatter between two device buffers (same device).
pub struct IndexedScatterOp {
    pipeline: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
}

impl IndexedScatterOp {
    pub fn new(ctx: &GpuContext) -> Self {
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("indexed_scatter"),
                source: wgpu::ShaderSource::Wgsl(include_str!("indexed_scatter.wgsl").into()),
            });
        let layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("indexed_scatter_layout"),
                entries: &[
                    storage_entry(0, true),
                    storage_entry(1, false),
                    storage_entry(2, true),
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("indexed_scatter_pipeline_layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("indexed_scatter_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: "scatter_indexed",
                compilation_options: Default::default(),
                cache: None,
            });
        Self { pipeline, layout }
    }

    /// Validate, upload entries, and run one scatter dispatch.
    pub fn dispatch(
        &self,
        ctx: &GpuContext,
        src: &wgpu::Buffer,
        dst: &wgpu::Buffer,
        entries: &[ScatterEntry],
    ) -> Result<(), IndexedScatterError> {
        validate_scatter_entries(entries, src.size() / 4, dst.size() / 4)?;
        if entries.is_empty() {
            return Ok(());
        }
        let bind_group = self.bind_group(ctx, src, dst, entries)?;
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("indexed_scatter_encoder"),
            });
        self.record_dispatch(&mut encoder, &bind_group, entries.len());
        ctx.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Record one scatter dispatch into an existing command encoder (no queue submit).
    pub fn record_dispatch(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bind_group: &wgpu::BindGroup,
        entry_count: usize,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("indexed_scatter_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.dispatch_workgroups(entry_count.div_ceil(64) as u32, 1, 1);
    }

    /// Build scatter bind group for batched encoder recording.
    pub fn bind_group(
        &self,
        ctx: &GpuContext,
        src: &wgpu::Buffer,
        dst: &wgpu::Buffer,
        entries: &[ScatterEntry],
    ) -> Result<wgpu::BindGroup, IndexedScatterError> {
        validate_scatter_entries(entries, src.size() / 4, dst.size() / 4)?;
        if entries.is_empty() {
            return Err(IndexedScatterError::EmptyEntries);
        }
        let entries_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("indexed_scatter_entries"),
                contents: bytemuck::cast_slice(entries),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let params = ScatterParams {
            n_entries: entries.len() as u32,
            _pad: [0; 3],
        };
        let params_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("indexed_scatter_params"),
                contents: bytemuck::bytes_of(&params),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        Ok(ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("indexed_scatter_bind"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: src.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: dst.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: entries_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        }))
    }
}

fn storage_entry(binding: u32, read_only: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

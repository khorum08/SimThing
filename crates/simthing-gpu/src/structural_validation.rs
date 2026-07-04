//! GPU structural validation over resident upload packet buffers.
//!
//! Projection/cache validation only — not model authority.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{BufferUsages, Device, Queue};

use crate::structural_upload::{
    readback_pod_blocking, PackedUpload, StructuralUploadError, StructuralUploadGpuBuffers,
    StructuralUploadGpuReport, StructuralUploadRows,
};

const WGSL_STRUCTURAL_VALIDATION: &str = include_str!("shaders/structural_validation.wgsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct StructuralValidationReportGpu {
    pub location_count: u32,
    pub link_count: u32,
    pub invalid_link_endpoint_count: u32,
    pub self_link_count: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StructuralValidationError {
    #[error(transparent)]
    Upload(#[from] StructuralUploadError),
}

pub const VALIDATION_REPORT_BYTES: u64 =
    std::mem::size_of::<StructuralValidationReportGpu>() as u64;

pub fn initial_validation_report(
    location_count: u32,
    link_count: u32,
) -> StructuralValidationReportGpu {
    StructuralValidationReportGpu {
        location_count,
        link_count,
        invalid_link_endpoint_count: 0,
        self_link_count: 0,
        reserved0: 0,
        reserved1: 0,
        reserved2: 0,
        reserved3: 0,
    }
}

pub fn validate_structural_upload_on_gpu(
    device: &Device,
    queue: &Queue,
    buffers: &StructuralUploadGpuBuffers,
    upload_report: &StructuralUploadGpuReport,
) -> Result<StructuralValidationReportGpu, StructuralValidationError> {
    let initial = initial_validation_report(upload_report.location_count, upload_report.link_count);
    if upload_report.link_count == 0 {
        return Ok(initial);
    }

    let report_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_validation_report"),
        contents: bytemuck::bytes_of(&initial),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    });
    let link_count_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_validation_link_count"),
        contents: bytemuck::bytes_of(&upload_report.link_count),
        usage: BufferUsages::UNIFORM,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("structural_validation_shader"),
        source: wgpu::ShaderSource::Wgsl(WGSL_STRUCTURAL_VALIDATION.into()),
    });
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("structural_validation_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
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
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("structural_validation_pl"),
        bind_group_layouts: &[&layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("structural_validation_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("structural_validation_bg"),
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.frame_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: buffers.link_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: report_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: link_count_buffer.as_entire_binding(),
            },
        ],
    });

    let workgroups = (upload_report.link_count + 63) / 64;
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("structural_validation_encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("structural_validation_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    readback_pod_blocking(device, queue, &report_buffer, "validation_report")
        .map_err(StructuralValidationError::Upload)
}

pub fn validate_structural_rows_on_gpu(
    device: &Device,
    queue: &Queue,
    rows: &StructuralUploadRows,
) -> Result<StructuralValidationReportGpu, StructuralValidationError> {
    let packed = PackedUpload::try_from(rows)?;
    let (buffers, report) =
        crate::structural_upload::upload_structural_rows_to_gpu(device, queue, &packed)?;
    validate_structural_upload_on_gpu(device, queue, &buffers, &report)
}

pub fn scan_for_forbidden_validation_tokens(source: &str, label: &str) {
    const FORBIDDEN: &[&str] = &[
        "route",
        "predecessor",
        "pathfinding",
        "movement_order",
        "fleet",
        "faction",
        "owner",
        "border",
        "frontline",
        "combat",
        "economy",
        "diplomacy",
    ];
    let lower = source.to_ascii_lowercase();
    for token in FORBIDDEN {
        assert!(
            !lower.contains(token),
            "{label} contains forbidden token: {token}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structural_upload::{
        upload_structural_rows_to_gpu, PackedUpload, StructuralFrameGpuRow, StructuralLinkGpuRow,
        StructuralLocationGpuRow, FRAME_ROW_BYTES, LINK_ROW_BYTES,
    };
    use crate::GpuContext;

    fn sample_rows() -> StructuralUploadRows {
        StructuralUploadRows {
            frame: StructuralFrameGpuRow {
                width: 8,
                height: 8,
                occupied_cells: 2,
                location_count: 2,
                link_count: 1,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
            locations: vec![
                StructuralLocationGpuRow {
                    dense_index: 0,
                    simthing_id_raw: 10,
                    system_id: 1,
                    row: 2,
                    col: 3,
                    reserved0: 0,
                    reserved1: 0,
                    reserved2: 0,
                },
                StructuralLocationGpuRow {
                    dense_index: 1,
                    simthing_id_raw: 11,
                    system_id: 2,
                    row: 2,
                    col: 4,
                    reserved0: 0,
                    reserved1: 0,
                    reserved2: 0,
                },
            ],
            links: vec![StructuralLinkGpuRow {
                from_dense_index: 0,
                to_dense_index: 1,
                reserved0: 0,
                reserved1: 0,
            }],
        }
    }

    fn single_location_zero_link_rows() -> StructuralUploadRows {
        StructuralUploadRows {
            frame: StructuralFrameGpuRow {
                width: 8,
                height: 8,
                occupied_cells: 1,
                location_count: 1,
                link_count: 0,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
            locations: vec![StructuralLocationGpuRow {
                dense_index: 0,
                simthing_id_raw: 10,
                system_id: 1,
                row: 2,
                col: 3,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            }],
            links: Vec::new(),
        }
    }

}

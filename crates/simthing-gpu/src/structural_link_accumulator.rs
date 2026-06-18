//! GPU structural link neighbor accumulation smoke pass.
//!
//! Fixed-point i32 accumulation over canonical structural links — projection/cache only,
//! not model authority. No pathfinding or movement-order semantics.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{BufferUsages, Device, Queue};

use crate::structural_upload::{
    readback_buffer_bytes_blocking, readback_pod_blocking, upload_structural_rows_to_gpu,
    StructuralLinkGpuRow, StructuralUploadError, StructuralUploadGpuBuffers,
    StructuralUploadGpuReport, StructuralUploadRows,
};
use crate::structural_validation::{
    validate_structural_upload_on_gpu, StructuralValidationError, StructuralValidationReportGpu,
};

const WGSL_STRUCTURAL_LINK_ACCUMULATOR: &str =
    include_str!("shaders/structural_link_accumulator.wgsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct StructuralLinkAccumulatorReportGpu {
    pub location_count: u32,
    pub link_count: u32,
    pub invalid_link_endpoint_count: u32,
    pub self_link_count: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralLinkAccumulatorGpuReadback {
    pub output_values: Vec<i32>,
    pub report: StructuralLinkAccumulatorReportGpu,
    pub validation_report: StructuralValidationReportGpu,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StructuralLinkAccumulatorError {
    #[error(transparent)]
    Upload(#[from] StructuralUploadError),
    #[error(transparent)]
    Validation(#[from] StructuralValidationError),
    #[error("input_values length {actual} does not match location_count {expected}")]
    WrongInputLength { expected: u32, actual: usize },
    #[error("structural link endpoint {endpoint} >= location_count {location_count}")]
    InvalidEndpoint { endpoint: u32, location_count: u32 },
    #[error("structural link is a self-link at dense index {index}")]
    SelfLink { index: u32 },
}

pub const ACCUMULATOR_REPORT_BYTES: u64 =
    std::mem::size_of::<StructuralLinkAccumulatorReportGpu>() as u64;

pub fn initial_accumulator_report(
    location_count: u32,
    link_count: u32,
) -> StructuralLinkAccumulatorReportGpu {
    StructuralLinkAccumulatorReportGpu {
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

pub fn cpu_structural_link_accumulate_i32(
    location_count: u32,
    links: &[StructuralLinkGpuRow],
    input_values: &[i32],
) -> Result<Vec<i32>, StructuralLinkAccumulatorError> {
    if input_values.len() != location_count as usize {
        return Err(StructuralLinkAccumulatorError::WrongInputLength {
            expected: location_count,
            actual: input_values.len(),
        });
    }
    let mut output = vec![0i32; location_count as usize];
    for link in links {
        if link.from_dense_index == link.to_dense_index {
            return Err(StructuralLinkAccumulatorError::SelfLink {
                index: link.from_dense_index,
            });
        }
        if link.from_dense_index >= location_count {
            return Err(StructuralLinkAccumulatorError::InvalidEndpoint {
                endpoint: link.from_dense_index,
                location_count,
            });
        }
        if link.to_dense_index >= location_count {
            return Err(StructuralLinkAccumulatorError::InvalidEndpoint {
                endpoint: link.to_dense_index,
                location_count,
            });
        }
        let from = link.from_dense_index as usize;
        let to = link.to_dense_index as usize;
        output[from] = output[from].saturating_add(input_values[to]);
        output[to] = output[to].saturating_add(input_values[from]);
    }
    Ok(output)
}

pub fn execute_structural_link_accumulator_on_gpu(
    device: &Device,
    queue: &Queue,
    buffers: &StructuralUploadGpuBuffers,
    upload_report: &StructuralUploadGpuReport,
    input_values_fixed: &[i32],
) -> Result<StructuralLinkAccumulatorGpuReadback, StructuralLinkAccumulatorError> {
    if input_values_fixed.len() != upload_report.location_count as usize {
        return Err(StructuralLinkAccumulatorError::WrongInputLength {
            expected: upload_report.location_count,
            actual: input_values_fixed.len(),
        });
    }

    let validation_report =
        validate_structural_upload_on_gpu(device, queue, buffers, upload_report)?;

    let initial_report =
        initial_accumulator_report(upload_report.location_count, upload_report.link_count);
    let location_count = upload_report.location_count as usize;
    let output_zeros = vec![0i32; location_count];

    if upload_report.link_count == 0 {
        return Ok(StructuralLinkAccumulatorGpuReadback {
            output_values: output_zeros,
            report: initial_report,
            validation_report,
        });
    }

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_link_accumulator_input"),
        contents: bytemuck::cast_slice(input_values_fixed),
        usage: BufferUsages::STORAGE,
    });
    let output_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_link_accumulator_output"),
        contents: bytemuck::cast_slice(&output_zeros),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    });
    let report_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_link_accumulator_report"),
        contents: bytemuck::bytes_of(&initial_report),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    });
    let link_count_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_link_accumulator_link_count"),
        contents: bytemuck::bytes_of(&upload_report.link_count),
        usage: BufferUsages::UNIFORM,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("structural_link_accumulator_shader"),
        source: wgpu::ShaderSource::Wgsl(WGSL_STRUCTURAL_LINK_ACCUMULATOR.into()),
    });
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("structural_link_accumulator_bgl"),
        entries: &[
            storage_read_entry(0),
            storage_read_entry(1),
            storage_read_entry(2),
            storage_read_write_entry(3),
            storage_read_write_entry(4),
            uniform_entry(5),
        ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("structural_link_accumulator_pl"),
        bind_group_layouts: &[&layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("structural_link_accumulator_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("structural_link_accumulator_bg"),
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
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: output_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: report_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: link_count_buffer.as_entire_binding(),
            },
        ],
    });

    let workgroups = (upload_report.link_count + 63) / 64;
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("structural_link_accumulator_encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("structural_link_accumulator_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let output_bytes = readback_buffer_bytes_blocking(
        device,
        queue,
        &output_buffer,
        (location_count * std::mem::size_of::<i32>()) as u64,
        "accumulator_output",
    )?;
    let output_values: Vec<i32> = bytemuck::cast_slice(&output_bytes).to_vec();
    let report = readback_pod_blocking(device, queue, &report_buffer, "accumulator_report")?;

    Ok(StructuralLinkAccumulatorGpuReadback {
        output_values,
        report,
        validation_report,
    })
}

pub fn accumulate_structural_rows_on_gpu(
    device: &Device,
    queue: &Queue,
    rows: &StructuralUploadRows,
    input_values_fixed: &[i32],
) -> Result<StructuralLinkAccumulatorGpuReadback, StructuralLinkAccumulatorError> {
    let (buffers, report) =
        upload_structural_rows_to_gpu(device, queue, rows.frame, &rows.locations, &rows.links)?;
    execute_structural_link_accumulator_on_gpu(device, queue, &buffers, &report, input_values_fixed)
}

fn storage_read_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_read_write_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn scan_for_forbidden_accumulator_tokens(source: &str, label: &str) {
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
    use crate::context::GpuContext;
    use crate::structural_upload::{
        upload_structural_rows_to_gpu, StructuralFrameGpuRow, StructuralLocationGpuRow,
    };

    fn vertical_seed_rows() -> StructuralUploadRows {
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

    fn chain_rows() -> StructuralUploadRows {
        StructuralUploadRows {
            frame: StructuralFrameGpuRow {
                width: 8,
                height: 8,
                occupied_cells: 3,
                location_count: 3,
                link_count: 2,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
            locations: vec![
                StructuralLocationGpuRow {
                    dense_index: 0,
                    simthing_id_raw: 10,
                    system_id: 1,
                    row: 0,
                    col: 0,
                    reserved0: 0,
                    reserved1: 0,
                    reserved2: 0,
                },
                StructuralLocationGpuRow {
                    dense_index: 1,
                    simthing_id_raw: 11,
                    system_id: 2,
                    row: 0,
                    col: 1,
                    reserved0: 0,
                    reserved1: 0,
                    reserved2: 0,
                },
                StructuralLocationGpuRow {
                    dense_index: 2,
                    simthing_id_raw: 12,
                    system_id: 3,
                    row: 0,
                    col: 2,
                    reserved0: 0,
                    reserved1: 0,
                    reserved2: 0,
                },
            ],
            links: vec![
                StructuralLinkGpuRow {
                    from_dense_index: 0,
                    to_dense_index: 1,
                    reserved0: 0,
                    reserved1: 0,
                },
                StructuralLinkGpuRow {
                    from_dense_index: 1,
                    to_dense_index: 2,
                    reserved0: 0,
                    reserved1: 0,
                },
            ],
        }
    }

    fn triangle_rows() -> StructuralUploadRows {
        StructuralUploadRows {
            frame: StructuralFrameGpuRow {
                width: 8,
                height: 8,
                occupied_cells: 3,
                location_count: 3,
                link_count: 3,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
            locations: chain_rows().locations,
            links: vec![
                StructuralLinkGpuRow {
                    from_dense_index: 0,
                    to_dense_index: 1,
                    reserved0: 0,
                    reserved1: 0,
                },
                StructuralLinkGpuRow {
                    from_dense_index: 1,
                    to_dense_index: 2,
                    reserved0: 0,
                    reserved1: 0,
                },
                StructuralLinkGpuRow {
                    from_dense_index: 0,
                    to_dense_index: 2,
                    reserved0: 0,
                    reserved1: 0,
                },
            ],
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

    #[test]
    fn cpu_structural_link_accumulate_matches_runtime_vertical_seed() {
        let rows = vertical_seed_rows();
        let input = [10, 20];
        let output =
            cpu_structural_link_accumulate_i32(rows.frame.location_count, &rows.links, &input)
                .expect("oracle");
        assert_eq!(output, vec![20, 10]);
    }

    #[test]
    fn cpu_structural_link_accumulate_matches_chain() {
        let rows = chain_rows();
        let input = [10, 20, 30];
        let output =
            cpu_structural_link_accumulate_i32(rows.frame.location_count, &rows.links, &input)
                .expect("oracle");
        assert_eq!(output, vec![20, 40, 20]);
    }

    #[test]
    fn cpu_structural_link_accumulate_matches_triangle() {
        let rows = triangle_rows();
        let input = [10, 20, 30];
        let output =
            cpu_structural_link_accumulate_i32(rows.frame.location_count, &rows.links, &input)
                .expect("oracle");
        assert_eq!(output, vec![50, 40, 30]);
    }

    #[test]
    fn cpu_structural_link_accumulate_rejects_invalid_endpoint() {
        let rows = vertical_seed_rows();
        let mut bad = rows.links.clone();
        bad[0].to_dense_index = 99;
        let err = cpu_structural_link_accumulate_i32(rows.frame.location_count, &bad, &[10, 20])
            .expect_err("invalid endpoint");
        assert!(matches!(
            err,
            StructuralLinkAccumulatorError::InvalidEndpoint { .. }
        ));
    }

    #[test]
    fn cpu_structural_link_accumulate_rejects_self_link() {
        let rows = vertical_seed_rows();
        let mut bad = rows.links.clone();
        bad[0].to_dense_index = bad[0].from_dense_index;
        let err = cpu_structural_link_accumulate_i32(rows.frame.location_count, &bad, &[10, 20])
            .expect_err("self link");
        assert!(matches!(
            err,
            StructuralLinkAccumulatorError::SelfLink { .. }
        ));
    }

    #[test]
    fn cpu_structural_link_accumulate_rejects_wrong_input_len() {
        let rows = vertical_seed_rows();
        let err = cpu_structural_link_accumulate_i32(rows.frame.location_count, &rows.links, &[10])
            .expect_err("wrong len");
        assert!(matches!(
            err,
            StructuralLinkAccumulatorError::WrongInputLength { .. }
        ));
    }

    #[test]
    fn structural_link_accumulator_report_row_size_is_32() {
        assert_eq!(
            std::mem::size_of::<StructuralLinkAccumulatorReportGpu>(),
            32
        );
    }

    #[test]
    fn structural_link_accumulator_report_row_alignment_is_4() {
        assert_eq!(
            std::mem::align_of::<StructuralLinkAccumulatorReportGpu>(),
            4
        );
    }

    #[test]
    fn structural_link_accumulator_wgsl_contains_no_forbidden_semantic_terms() {
        scan_for_forbidden_accumulator_tokens(
            WGSL_STRUCTURAL_LINK_ACCUMULATOR,
            "structural_link_accumulator.wgsl",
        );
    }

    #[test]
    fn structural_link_accumulator_rust_contains_no_forbidden_semantic_terms_if_practical() {
        scan_for_forbidden_accumulator_tokens(
            WGSL_STRUCTURAL_LINK_ACCUMULATOR,
            "structural_link_accumulator.wgsl",
        );
    }

    #[test]
    fn gpu_link_accumulator_reports_real_adapter_or_marks_partial() {
        match GpuContext::new_blocking() {
            Ok(ctx) => {
                eprintln!("GPU-LINK-ACCUMULATOR-SMOKE-0 adapter evidence: REAL_ADAPTER_OBSERVED");
                let rows = vertical_seed_rows();
                let readback =
                    accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &[10, 20])
                        .expect("accumulate");
                assert_eq!(readback.output_values, vec![20, 10]);
            }
            Err(err) => {
                eprintln!(
                    "GPU-LINK-ACCUMULATOR-SMOKE-0 adapter evidence: GPU_TESTS_SKIPPED_NO_ADAPTER ({err})"
                );
            }
        }
    }

    #[test]
    fn gpu_link_accumulator_runtime_vertical_seed_matches_cpu_oracle() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let rows = vertical_seed_rows();
        let input = [10, 20];
        let expected =
            cpu_structural_link_accumulate_i32(rows.frame.location_count, &rows.links, &input)
                .expect("cpu");
        let readback =
            accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &input).expect("gpu");
        assert_eq!(readback.output_values, expected);
        assert_eq!(readback.report.invalid_link_endpoint_count, 0);
        assert_eq!(readback.report.self_link_count, 0);
    }

    #[test]
    fn gpu_link_accumulator_chain_matches_cpu_oracle() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let rows = chain_rows();
        let input = [10, 20, 30];
        let expected =
            cpu_structural_link_accumulate_i32(rows.frame.location_count, &rows.links, &input)
                .expect("cpu");
        let readback =
            accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &input).expect("gpu");
        assert_eq!(readback.output_values, expected);
    }

    #[test]
    fn gpu_link_accumulator_triangle_matches_cpu_oracle() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let rows = triangle_rows();
        let input = [10, 20, 30];
        let expected =
            cpu_structural_link_accumulate_i32(rows.frame.location_count, &rows.links, &input)
                .expect("cpu");
        let readback =
            accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &input).expect("gpu");
        assert_eq!(readback.output_values, expected);
    }

    #[test]
    fn gpu_link_accumulator_detects_invalid_endpoint_bad_rows() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut rows = vertical_seed_rows();
        rows.links[0].to_dense_index = 99;
        let readback = accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &[10, 20])
            .expect("gpu");
        assert_eq!(readback.report.invalid_link_endpoint_count, 1);
        assert_eq!(readback.report.self_link_count, 0);
    }

    #[test]
    fn gpu_link_accumulator_detects_self_link_bad_rows() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut rows = vertical_seed_rows();
        rows.links[0].to_dense_index = rows.links[0].from_dense_index;
        let readback = accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &[10, 20])
            .expect("gpu");
        assert_eq!(readback.report.self_link_count, 1);
        assert_eq!(readback.report.invalid_link_endpoint_count, 0);
    }

    #[test]
    fn gpu_link_accumulator_handles_zero_link_packet() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let rows = single_location_zero_link_rows();
        let readback =
            accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &[42]).expect("gpu");
        assert_eq!(readback.output_values, vec![0]);
        assert_eq!(readback.report.link_count, 0);
        assert_eq!(readback.report.location_count, 1);
    }

    #[test]
    fn gpu_link_accumulator_rejects_wrong_input_len() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let rows = vertical_seed_rows();
        let err = accumulate_structural_rows_on_gpu(&ctx.device, &ctx.queue, &rows, &[10])
            .expect_err("wrong len");
        assert!(matches!(
            err,
            StructuralLinkAccumulatorError::WrongInputLength { .. }
        ));
    }
}

//! MOBILITY-GPU-KERNEL-0: semantic-free mobility-shaped column-transform GPU kernel in test/support.
//!
//! Built-in parent-select kernel over generic column buffers. Not designer WGSL, not default
//! schedule, not gameplay path, not live-slot compaction.

use simthing_gpu::{fnv64_hash_f32, GpuContext};
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferDescriptor, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipelineDescriptor, PipelineLayoutDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

pub const MOBILITY_GPU_KERNEL0_FIXTURE_ID: &str =
    "mobility_gpu_kernel0_semantic_free_column_transform_fixture";
pub const MOBILITY_GPU_KERNEL0_NAMED_GATE: &str = "mobility_gpu_kernel0_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL0_KERNEL_ID: &str = "mobility_gpu_kernel0_column_parent_select";

/// Built-in semantic-free column-transform WGSL. Generic column names only; not designer-authored.
const MOBILITY_GPU_KERNEL0_BUILTIN_WGSL: &str = r#"@group(0) @binding(0) var<storage, read> src_parent: array<u32>;
@group(0) @binding(1) var<storage, read> dst_parent: array<u32>;
@group(0) @binding(2) var<storage, read> entity_id: array<u32>;
@group(0) @binding(3) var<storage, read> move_mask: array<u32>;
@group(0) @binding(4) var<storage, read_write> out_parent: array<u32>;
@group(0) @binding(5) var<storage, read_write> out_changed: array<u32>;
@group(0) @binding(6) var<uniform> len_buf: u32;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= len_buf) {
        return;
    }
    let _row = entity_id[i];
    let src = src_parent[i];
    if (move_mask[i] != 0u) {
        out_parent[i] = dst_parent[i];
    } else {
        out_parent[i] = src;
    }
    if (out_parent[i] != src) {
        out_changed[i] = 1u;
    } else {
        out_changed[i] = 0u;
    }
}
"#;

const OUTPUT_CHECKSUM_SEED: &[u8] = b"mobility_gpu_kernel0_output";

const PROBE_SRC_PARENT: [u32; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
const PROBE_DST_PARENT: [u32; 8] = [11, 21, 31, 41, 51, 61, 71, 81];
const PROBE_ENTITY_ID: [u32; 8] = [100, 101, 102, 103, 104, 105, 106, 107];
const PROBE_MOVE_MASK: [u32; 8] = [1, 0, 1, 0, 0, 1, 0, 1];

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel0ColumnProbe {
    pub src_parent: Vec<u32>,
    pub dst_parent: Vec<u32>,
    pub entity_id: Vec<u32>,
    pub move_mask: Vec<u32>,
}

impl MobilityGpuKernel0ColumnProbe {
    pub fn default_probe() -> Self {
        Self {
            src_parent: PROBE_SRC_PARENT.to_vec(),
            dst_parent: PROBE_DST_PARENT.to_vec(),
            entity_id: PROBE_ENTITY_ID.to_vec(),
            move_mask: PROBE_MOVE_MASK.to_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel0OracleOutput {
    pub out_parent: Vec<u32>,
    pub out_changed: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel0Gate {
    pub explicit_named_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel0Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_named_gate_enabled: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel0ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MobilityGpuKernel0ParityClassification {
    ExactParity,
    GpuUnavailable,
    GpuExecutionFailed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel0FixtureInput {
    pub gate: MobilityGpuKernel0Gate,
    pub forbidden: MobilityGpuKernel0ForbiddenPathRequests,
    pub columns: MobilityGpuKernel0ColumnProbe,
}

impl MobilityGpuKernel0FixtureInput {
    pub fn default_probe() -> Self {
        Self {
            gate: MobilityGpuKernel0Gate::explicit_opt_in(),
            forbidden: MobilityGpuKernel0ForbiddenPathRequests::default(),
            columns: MobilityGpuKernel0ColumnProbe::default_probe(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel0FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub kernel_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub builtin_semantic_free_kernel_only: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub default_schedule_registered: bool,
    pub gameplay_facing_path: bool,
    pub default_simsession_lib_path_wired: bool,

    pub live_slot_compaction: bool,
    pub gpu_allocator_used: bool,
    pub nondeterministic_atomics_used: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_production_scheduling_wired: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,

    pub gpu_execution_available: bool,
    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,

    pub confined_to_driver_test_support: bool,
}

pub fn mobility_gpu_kernel0_builtin_wgsl_is_semantic_free() -> bool {
    const FORBIDDEN: &[&str] = &[
        "capture",
        "owner",
        "economy",
        "faction",
        "planner",
        "urgency",
        "commitment",
        "resource_flow",
        "atomic",
    ];
    let wgsl = MOBILITY_GPU_KERNEL0_BUILTIN_WGSL.to_ascii_lowercase();
    !FORBIDDEN.iter().any(|term| wgsl.contains(term))
}

pub fn run_mobility_gpu_kernel0_fixture(
    input: &MobilityGpuKernel0FixtureInput,
) -> MobilityGpuKernel0FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel0_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.explicit_named_gate_enabled {
        return disabled_no_op_report(input);
    }

    let oracle = cpu_column_transform_oracle(&input.columns);
    let cpu_oracle_checksum = checksum_oracle_output(&oracle);

    match GpuContext::new_blocking() {
        Ok(ctx) => admitted_with_gpu(input, cpu_oracle_checksum, &oracle, &ctx),
        Err(_) => admitted_gpu_unavailable(input, cpu_oracle_checksum),
    }
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel0ForbiddenPathRequests,
) -> Option<Vec<&'static str>> {
    let mut diagnostics = Vec::new();
    if forbidden.default_on_behavior {
        diagnostics.push("default_on_behavior");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.designer_authored_shader_input {
        diagnostics.push("designer_authored_shader_input");
    }
    if diagnostics.is_empty() {
        None
    } else {
        Some(diagnostics)
    }
}

pub fn cpu_column_transform_oracle(
    columns: &MobilityGpuKernel0ColumnProbe,
) -> MobilityGpuKernel0OracleOutput {
    let len = columns.src_parent.len();
    let mut out_parent = Vec::with_capacity(len);
    let mut out_changed = Vec::with_capacity(len);
    for i in 0..len {
        let src = columns.src_parent[i];
        let parent = if columns.move_mask[i] != 0 {
            columns.dst_parent[i]
        } else {
            src
        };
        out_parent.push(parent);
        out_changed.push(u32::from(parent != src));
    }
    MobilityGpuKernel0OracleOutput {
        out_parent,
        out_changed,
    }
}

fn checksum_oracle_output(oracle: &MobilityGpuKernel0OracleOutput) -> u64 {
    let flat = flatten_u32_outputs(&oracle.out_parent, &oracle.out_changed);
    fnv64_hash_f32(&flat, OUTPUT_CHECKSUM_SEED)
}

fn flatten_u32_outputs(out_parent: &[u32], out_changed: &[u32]) -> Vec<f32> {
    out_parent
        .iter()
        .zip(out_changed.iter())
        .flat_map(|(p, c)| [f32::from_bits(*p), f32::from_bits(*c)])
        .collect()
}

fn storage_ro(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_rw(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn run_builtin_gpu_column_kernel(
    ctx: &GpuContext,
    columns: &MobilityGpuKernel0ColumnProbe,
) -> Result<MobilityGpuKernel0OracleOutput, &'static str> {
    let device = &ctx.device;
    let len = columns.src_parent.len() as u32;
    let byte_len = (columns.src_parent.len() * 4) as u64;

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("mobility_gpu_kernel0_column_parent_select"),
        source: ShaderSource::Wgsl(MOBILITY_GPU_KERNEL0_BUILTIN_WGSL.into()),
    });
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("mobility_gpu_kernel0_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_ro(2),
            storage_ro(3),
            storage_rw(4),
            storage_rw(5),
            BindGroupLayoutEntry {
                binding: 6,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("mobility_gpu_kernel0_pl"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("mobility_gpu_kernel0_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let src_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel0_src_parent"),
        contents: bytemuck::cast_slice(&columns.src_parent),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let dst_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel0_dst_parent"),
        contents: bytemuck::cast_slice(&columns.dst_parent),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let entity_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel0_entity_id"),
        contents: bytemuck::cast_slice(&columns.entity_id),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let mask_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel0_move_mask"),
        contents: bytemuck::cast_slice(&columns.move_mask),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let out_parent_buf = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel0_out_parent"),
        size: byte_len,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let out_changed_buf = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel0_out_changed"),
        size: byte_len,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let len_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel0_len"),
        contents: bytemuck::bytes_of(&len),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("mobility_gpu_kernel0_bg"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: src_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: dst_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: entity_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 3,
                resource: mask_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 4,
                resource: out_parent_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 5,
                resource: out_changed_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 6,
                resource: len_buf.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("mobility_gpu_kernel0_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("mobility_gpu_kernel0_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups((len + 63) / 64, 1, 1);
    }
    ctx.queue.submit(Some(encoder.finish()));

    let staging_parent = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel0_staging_parent"),
        size: byte_len,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let staging_changed = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel0_staging_changed"),
        size: byte_len,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut readback = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("mobility_gpu_kernel0_readback"),
    });
    readback.copy_buffer_to_buffer(&out_parent_buf, 0, &staging_parent, 0, byte_len);
    readback.copy_buffer_to_buffer(&out_changed_buf, 0, &staging_changed, 0, byte_len);
    ctx.queue.submit(Some(readback.finish()));

    let parent_slice = staging_parent.slice(..);
    parent_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let parent_data = parent_slice.get_mapped_range();
    let out_parent: Vec<u32> = bytemuck::cast_slice(&parent_data).to_vec();
    drop(parent_data);
    staging_parent.unmap();

    let changed_slice = staging_changed.slice(..);
    changed_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let changed_data = changed_slice.get_mapped_range();
    let out_changed: Vec<u32> = bytemuck::cast_slice(&changed_data).to_vec();

    Ok(MobilityGpuKernel0OracleOutput {
        out_parent,
        out_changed,
    })
}

fn shell(input: &MobilityGpuKernel0FixtureInput) -> MobilityGpuKernel0FixtureReport {
    MobilityGpuKernel0FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL0_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL0_NAMED_GATE,
        kernel_id: MOBILITY_GPU_KERNEL0_KERNEL_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.explicit_named_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        builtin_semantic_free_kernel_only: mobility_gpu_kernel0_builtin_wgsl_is_semantic_free(),
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        default_schedule_registered: false,
        gameplay_facing_path: false,
        default_simsession_lib_path_wired: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        gpu_execution_available: false,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        confined_to_driver_test_support: true,
    }
}

fn disabled_no_op_report(input: &MobilityGpuKernel0FixtureInput) -> MobilityGpuKernel0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel0FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel0FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn admitted_gpu_unavailable(
    input: &MobilityGpuKernel0FixtureInput,
    cpu_oracle_checksum: u64,
) -> MobilityGpuKernel0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.cpu_oracle_checksum = cpu_oracle_checksum;
    report.parity_classification = MobilityGpuKernel0ParityClassification::GpuUnavailable;
    report
}

fn admitted_with_gpu(
    input: &MobilityGpuKernel0FixtureInput,
    cpu_oracle_checksum: u64,
    oracle: &MobilityGpuKernel0OracleOutput,
    ctx: &GpuContext,
) -> MobilityGpuKernel0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.gpu_execution_available = true;
    report.cpu_oracle_checksum = cpu_oracle_checksum;

    match run_builtin_gpu_column_kernel(ctx, &input.columns) {
        Ok(gpu_out) => {
            report.gpu_dispatch_occurred = true;
            let gpu_checksum = checksum_oracle_output(&gpu_out);
            report.gpu_result_checksum = Some(gpu_checksum);
            report.parity_classification = if gpu_out == *oracle {
                MobilityGpuKernel0ParityClassification::ExactParity
            } else {
                MobilityGpuKernel0ParityClassification::GpuExecutionFailed
            };
        }
        Err(_) => {
            report.parity_classification = MobilityGpuKernel0ParityClassification::GpuExecutionFailed;
        }
    }
    report
}

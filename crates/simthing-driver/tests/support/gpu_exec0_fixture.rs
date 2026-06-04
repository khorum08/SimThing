//! GPU-EXEC-0: semantic-free GPU execution readiness probe in `simthing-driver` test/support.
//!
//! Proves a generic opt-in/default-off execution path can run a built-in identity-buffer pass,
//! report CPU oracle and GPU checksums, and classify exact parity or honest unsupported.
//! Not mobility dispatch, not designer WGSL, not default schedule or gameplay path.

use simthing_gpu::{fnv64_hash_f32, GpuContext};
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferDescriptor, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipelineDescriptor, PipelineLayoutDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

pub const GPU_EXEC0_FIXTURE_ID: &str = "gpu_exec0_semantic_free_readiness_probe";
pub const GPU_EXEC0_NAMED_GATE: &str = "gpu_exec0_explicit_opt_in_gate";
pub const GPU_EXEC0_PASS_DESCRIPTOR_ID: &str = "gpu_exec0_identity_buffer_pass";
pub const MOBILITY_RUNTIME1B_DISPATCH_GATE: &str = "mobility_runtime1b_dispatch_closed";

/// Built-in semantic-free pass WGSL (identity buffer copy). Not mobility, not designer-authored.
const GPU_EXEC0_BUILTIN_WGSL: &str = r#"@group(0) @binding(0) var<storage, read> input_buf: array<f32>;
@group(0) @binding(1) var<storage, read_write> output_buf: array<f32>;
@group(0) @binding(2) var<uniform> len_buf: u32;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= len_buf) {
        return;
    }
    output_buf[i] = input_buf[i];
}
"#;

const PROBE_VALUES: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
const GPU_EXEC0_OUTPUT_CHECKSUM_SEED: &[u8] = b"gpu_exec0_pass_output";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GpuExec0Gate {
    pub explicit_named_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl GpuExec0Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_named_gate_enabled: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GpuExec0ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub mobility_shader_or_dispatch: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuExec0ParityClassification {
    ExactParity,
    GpuUnavailable,
    GpuExecutionFailed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GpuExec0FixtureInput {
    pub gate: GpuExec0Gate,
    pub forbidden: GpuExec0ForbiddenPathRequests,
    pub probe_values: Vec<f32>,
}

impl GpuExec0FixtureInput {
    pub fn default_probe() -> Self {
        Self {
            gate: GpuExec0Gate::explicit_opt_in(),
            forbidden: GpuExec0ForbiddenPathRequests::default(),
            probe_values: PROBE_VALUES.to_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GpuExec0FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub pass_descriptor_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub semantic_free_pass_descriptor_only: bool,
    pub mobility_shader_present: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub default_schedule_registered: bool,
    pub gameplay_facing_path: bool,
    pub default_simsession_lib_path_wired: bool,

    pub gpu_execution_available: bool,
    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: GpuExec0ParityClassification,

    pub runtime1b_dispatch_gate_closed: bool,
    pub confined_to_driver_test_support: bool,
}

pub fn run_gpu_exec0_fixture(input: &GpuExec0FixtureInput) -> GpuExec0FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["gpu_exec0_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.explicit_named_gate_enabled {
        return disabled_no_op_report(input);
    }

    let cpu_oracle = cpu_identity_oracle(&input.probe_values);
    let cpu_oracle_checksum = fnv64_hash_f32(&cpu_oracle, GPU_EXEC0_OUTPUT_CHECKSUM_SEED);

    match GpuContext::new_blocking() {
        Ok(ctx) => admitted_with_gpu(input, cpu_oracle_checksum, &cpu_oracle, &ctx),
        Err(_) => admitted_gpu_unavailable(input, cpu_oracle_checksum),
    }
}

fn validate_forbidden(forbidden: &GpuExec0ForbiddenPathRequests) -> Option<Vec<&'static str>> {
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
    if forbidden.mobility_shader_or_dispatch {
        diagnostics.push("mobility_shader_or_dispatch");
    }
    if diagnostics.is_empty() {
        None
    } else {
        Some(diagnostics)
    }
}

fn cpu_identity_oracle(values: &[f32]) -> Vec<f32> {
    values.to_vec()
}

fn run_builtin_gpu_identity_pass(
    ctx: &GpuContext,
    values: &[f32],
) -> Result<Vec<f32>, &'static str> {
    let device = &ctx.device;
    let len = values.len() as u32;
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("gpu_exec0_identity_pass"),
        source: ShaderSource::Wgsl(GPU_EXEC0_BUILTIN_WGSL.into()),
    });
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gpu_exec0_bgl"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
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
        label: Some("gpu_exec0_pl"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("gpu_exec0_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });
    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("gpu_exec0_in"),
        contents: bytemuck::cast_slice(values),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let output_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("gpu_exec0_out"),
        size: (values.len() * 4) as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let len_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("gpu_exec0_len"),
        contents: bytemuck::bytes_of(&len),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("gpu_exec0_bg"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: len_buffer.as_entire_binding(),
            },
        ],
    });
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("gpu_exec0_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("gpu_exec0_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups((len + 63) / 64, 1, 1);
    }
    ctx.queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&BufferDescriptor {
        label: Some("gpu_exec0_staging"),
        size: (values.len() * 4) as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut readback_encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("gpu_exec0_readback_enc"),
    });
    readback_encoder.copy_buffer_to_buffer(
        &output_buffer,
        0,
        &staging,
        0,
        (values.len() * 4) as u64,
    );
    ctx.queue.submit(Some(readback_encoder.finish()));
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let data = slice.get_mapped_range();
    Ok(bytemuck::cast_slice(&data).to_vec())
}

fn shell(input: &GpuExec0FixtureInput) -> GpuExec0FixtureReport {
    GpuExec0FixtureReport {
        fixture_id: GPU_EXEC0_FIXTURE_ID,
        named_gate: GPU_EXEC0_NAMED_GATE,
        pass_descriptor_id: GPU_EXEC0_PASS_DESCRIPTOR_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.explicit_named_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        semantic_free_pass_descriptor_only: true,
        mobility_shader_present: false,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        default_schedule_registered: false,
        gameplay_facing_path: false,
        default_simsession_lib_path_wired: false,
        gpu_execution_available: false,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: GpuExec0ParityClassification::GpuUnavailable,
        runtime1b_dispatch_gate_closed: true,
        confined_to_driver_test_support: true,
    }
}

fn disabled_no_op_report(input: &GpuExec0FixtureInput) -> GpuExec0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report.cpu_oracle_checksum = 0;
    report
}

fn rejected_report(
    input: &GpuExec0FixtureInput,
    diagnostics: Vec<&'static str>,
) -> GpuExec0FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn admitted_gpu_unavailable(
    input: &GpuExec0FixtureInput,
    cpu_oracle_checksum: u64,
) -> GpuExec0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.gpu_execution_available = false;
    report.cpu_oracle_checksum = cpu_oracle_checksum;
    report.gpu_result_checksum = None;
    report.parity_classification = GpuExec0ParityClassification::GpuUnavailable;
    report
}

fn admitted_with_gpu(
    input: &GpuExec0FixtureInput,
    cpu_oracle_checksum: u64,
    cpu_oracle: &[f32],
    ctx: &GpuContext,
) -> GpuExec0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.gpu_execution_available = true;
    report.cpu_oracle_checksum = cpu_oracle_checksum;

    match run_builtin_gpu_identity_pass(ctx, &input.probe_values) {
        Ok(gpu_out) => {
            report.gpu_dispatch_occurred = true;
            let gpu_checksum = fnv64_hash_f32(&gpu_out, GPU_EXEC0_OUTPUT_CHECKSUM_SEED);
            report.gpu_result_checksum = Some(gpu_checksum);
            report.parity_classification = if gpu_out == cpu_oracle {
                GpuExec0ParityClassification::ExactParity
            } else {
                GpuExec0ParityClassification::GpuExecutionFailed
            };
        }
        Err(_) => {
            report.parity_classification = GpuExec0ParityClassification::GpuExecutionFailed;
        }
    }
    report
}

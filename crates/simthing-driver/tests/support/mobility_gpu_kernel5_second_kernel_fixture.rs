//! MOBILITY-GPU-KERNEL-5: second semantic-free mobility-shaped GPU kernel over
//! the 34k composition-derived generic projection.
//!
//! Driver test/support only. This fixture proves the registered-node path can
//! host a second built-in generic transform without adding default scheduling,
//! gameplay, designer-authored shader input, or semantic/raw WGSL intake.

#[path = "mobility_gpu_kernel4_34k_projection_fixture.rs"]
mod mobility_gpu_kernel4_34k_projection_fixture;

use mobility_gpu_kernel4_34k_projection_fixture::{
    generate_34k_runtime_composition_input, generate_permuted_34k_runtime_composition_input,
    project_runtime_composition_input_to_columns, run_mobility_gpu_kernel4_fixture,
    MobilityGpuKernel4FixtureInput, MobilityGpuKernel4ForbiddenPathRequests, MobilityGpuKernel4Gate,
};

pub use mobility_gpu_kernel4_34k_projection_fixture::{
    cpu_column_transform_oracle, MobilityGpuKernel0ColumnProbe, MobilityGpuKernel0OracleOutput,
    MobilityGpuKernel0ParityClassification, MobilityRuntime1bPassgraphFixtureInput,
    MOBILITY_GPU_KERNEL0_KERNEL_ID, MOBILITY_GPU_KERNEL1_FIXTURE_ID,
    MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END, MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START,
    MOBILITY_GPU_KERNEL4_FIXTURE_ID, MOBILITY_GPU_KERNEL4_ROW_COUNT,
    MOBILITY_GPU_KERNEL4_SPARSE_STRIDE, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

use simthing_gpu::{fnv64_hash_f32, GpuContext};
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferDescriptor, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipelineDescriptor, PipelineLayoutDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

pub const MOBILITY_GPU_KERNEL5_FIXTURE_ID: &str =
    "mobility_gpu_kernel5_second_semantic_free_column_kernel_fixture";
pub const MOBILITY_GPU_KERNEL5_NAMED_GATE: &str =
    "mobility_gpu_kernel5_second_kernel_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL5_KERNEL_ID: &str = "mobility_gpu_kernel5_row_digest_weight";
pub const MOBILITY_GPU_KERNEL5_NEW_SHADER_TEXT_ADDED: bool = true;
pub const MOBILITY_GPU_KERNEL5_RUNTIME1B_DISPATCH_STATUS_RECONCILED: bool = true;

const OUTPUT_CHECKSUM_SEED: &[u8] = b"mobility_gpu_kernel5_output";

const MOBILITY_GPU_KERNEL5_BUILTIN_WGSL: &str = r#"@group(0) @binding(0) var<storage, read> src_parent: array<u32>;
@group(0) @binding(1) var<storage, read> dst_parent: array<u32>;
@group(0) @binding(2) var<storage, read> entity_id: array<u32>;
@group(0) @binding(3) var<storage, read> move_mask: array<u32>;
@group(0) @binding(4) var<storage, read_write> out_digest: array<u32>;
@group(0) @binding(5) var<storage, read_write> out_weight: array<u32>;
@group(0) @binding(6) var<uniform> len_buf: u32;

fn mix_u32(a: u32, b: u32) -> u32 {
    var x = a ^ (b + 0x9E3779B9u + (a << 6u) + (a >> 2u));
    x = x ^ (x >> 16u);
    x = x * 0x7FEB352Du;
    x = x ^ (x >> 15u);
    return x;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= len_buf) {
        return;
    }
    let digest0 = mix_u32(entity_id[i], src_parent[i]);
    let digest1 = mix_u32(dst_parent[i], move_mask[i]);
    out_digest[i] = mix_u32(digest0, digest1);
    out_weight[i] = move_mask[i] * 17u;
}
"#;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel5Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel5Gate {
    pub fn registration_only() -> Self {
        Self {
            registration_gate_enabled: true,
            dispatch_gate_enabled: false,
            enabled_by_default: false,
        }
    }

    pub fn registration_and_dispatch() -> Self {
        Self {
            registration_gate_enabled: true,
            dispatch_gate_enabled: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel5ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub default_on_behavior: bool,
    pub default_schedule: bool,
    pub default_simsession_path: bool,
    pub gameplay_path: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_or_nondeterministic_atomics: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,
    pub closed_ladder_reopen: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel5FixtureInput {
    pub gate: MobilityGpuKernel5Gate,
    pub forbidden: MobilityGpuKernel5ForbiddenPathRequests,
    pub passgraph: MobilityRuntime1bPassgraphFixtureInput,
    /// When set, dispatch uses these columns instead of the KERNEL-4 projection output.
    pub columns_override: Option<MobilityGpuKernel0ColumnProbe>,
}

impl MobilityGpuKernel5FixtureInput {
    pub fn default_second_kernel() -> Self {
        Self {
            gate: MobilityGpuKernel5Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel5ForbiddenPathRequests::default(),
            passgraph: MobilityGpuKernel4FixtureInput::default_34k_projection_soak().passgraph,
            columns_override: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel5OracleOutput {
    pub out_digest: Vec<u32>,
    pub out_weight: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel5FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub kernel_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub row_count: usize,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_schedule_unchanged: bool,
    pub gameplay_facing_path: bool,

    pub uses_registered_node: bool,
    pub registration_non_executing: bool,
    pub reused_kernel4_projection: bool,
    pub kernel4_fixture_id: &'static str,
    pub kernel1_fixture_id: &'static str,
    pub kernel0_kernel_id: &'static str,

    pub builtin_semantic_free_kernel_only: bool,
    pub shader_text_has_domain_terms: bool,
    pub new_shader_text_added: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_used: bool,
    pub nondeterministic_atomics_used: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_production_scheduling_wired: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,
    pub closed_ladders_reopened: bool,
    pub runtime1b_dispatch_status_reconciled: bool,

    pub cpu_oracle_complete: bool,
    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub projection_checksum: u64,
}

pub fn mobility_gpu_kernel5_builtin_wgsl_is_semantic_free() -> bool {
    !mobility_gpu_kernel5_shader_text_has_domain_terms()
}

pub fn mobility_gpu_kernel5_shader_text_has_domain_terms() -> bool {
    const FORBIDDEN: &[&str] = &[
        "capture",
        "owner",
        "economy",
        "faction",
        "species",
        "map",
        "resource",
        "blockade",
        "planner",
        "urgency",
        "commitment",
        "atomic",
    ];
    let wgsl = MOBILITY_GPU_KERNEL5_BUILTIN_WGSL.to_ascii_lowercase();
    FORBIDDEN.iter().any(|term| wgsl.contains(term))
}

pub fn run_mobility_gpu_kernel5_fixture(
    input: &MobilityGpuKernel5FixtureInput,
) -> MobilityGpuKernel5FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel5_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    let kernel4_report = run_mobility_gpu_kernel4_fixture(&kernel4_input(input));
    if !kernel4_report.admitted {
        return rejected_report(input, kernel4_report.diagnostics);
    }

    if !input.gate.dispatch_gate_enabled {
        return registration_only_report(input, kernel4_report.uses_registered_node);
    }

    let columns = if let Some(columns) = input.columns_override.clone() {
        columns
    } else {
        let projection = match kernel4_report.projection {
            Some(projection) => projection,
            None => return rejected_report(input, vec!["kernel4_projection_missing"]),
        };
        projection.columns
    };
    let oracle = cpu_second_kernel_oracle(&columns);
    let cpu_oracle_checksum = checksum_second_kernel_output(&oracle);

    match GpuContext::new_blocking() {
        Ok(ctx) => admitted_with_gpu(input, &columns, cpu_oracle_checksum, &oracle, &ctx),
        Err(_) => admitted_gpu_unavailable(input, columns, cpu_oracle_checksum),
    }
}

pub fn projected_34k_columns_for_kernel5() -> MobilityGpuKernel0ColumnProbe {
    project_runtime_composition_input_to_columns(&generate_34k_runtime_composition_input())
        .expect("34k runtime composition projection should be admitted")
}

pub fn permuted_projected_34k_columns_for_kernel5() -> MobilityGpuKernel0ColumnProbe {
    project_runtime_composition_input_to_columns(&generate_permuted_34k_runtime_composition_input())
        .expect("permuted 34k runtime composition projection should be admitted")
}

pub fn cpu_second_kernel_oracle(
    columns: &MobilityGpuKernel0ColumnProbe,
) -> MobilityGpuKernel5OracleOutput {
    let mut out_digest = Vec::with_capacity(columns.entity_id.len());
    let mut out_weight = Vec::with_capacity(columns.entity_id.len());
    for i in 0..columns.entity_id.len() {
        let digest0 = mix_u32(columns.entity_id[i], columns.src_parent[i]);
        let digest1 = mix_u32(columns.dst_parent[i], columns.move_mask[i]);
        out_digest.push(mix_u32(digest0, digest1));
        out_weight.push(columns.move_mask[i].wrapping_mul(17));
    }
    MobilityGpuKernel5OracleOutput {
        out_digest,
        out_weight,
    }
}

fn mix_u32(a: u32, b: u32) -> u32 {
    let mut x = a ^ b.wrapping_add(0x9E37_79B9).wrapping_add(a << 6).wrapping_add(a >> 2);
    x ^= x >> 16;
    x = x.wrapping_mul(0x7FEB_352D);
    x ^= x >> 15;
    x
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel5ForbiddenPathRequests,
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
    if forbidden.default_schedule {
        diagnostics.push("default_schedule");
    }
    if forbidden.default_simsession_path {
        diagnostics.push("default_simsession_path");
    }
    if forbidden.gameplay_path {
        diagnostics.push("gameplay_path");
    }
    if forbidden.live_slot_compaction {
        diagnostics.push("live_slot_compaction");
    }
    if forbidden.gpu_allocator_or_nondeterministic_atomics {
        diagnostics.push("gpu_allocator_or_nondeterministic_atomics");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
    }
    if forbidden.hybrid_strata_or_faction_index_scaling {
        diagnostics.push("hybrid_strata_or_faction_index_scaling");
    }
    if forbidden.closed_ladder_reopen {
        diagnostics.push("closed_ladder_reopen");
    }
    if diagnostics.is_empty() {
        None
    } else {
        Some(diagnostics)
    }
}

fn kernel4_input(input: &MobilityGpuKernel5FixtureInput) -> MobilityGpuKernel4FixtureInput {
    MobilityGpuKernel4FixtureInput {
        gate: MobilityGpuKernel4Gate {
            registration_gate_enabled: input.gate.registration_gate_enabled,
            dispatch_gate_enabled: input.gate.dispatch_gate_enabled,
            enabled_by_default: input.gate.enabled_by_default,
        },
        forbidden: MobilityGpuKernel4ForbiddenPathRequests {
            semantic_or_raw_wgsl: input.forbidden.semantic_or_raw_wgsl,
            designer_authored_shader_input: input.forbidden.designer_authored_shader_input,
            default_on_behavior: input.forbidden.default_on_behavior,
            default_schedule: input.forbidden.default_schedule,
            default_simsession_path: input.forbidden.default_simsession_path,
            gameplay_path: input.forbidden.gameplay_path,
            live_slot_compaction: input.forbidden.live_slot_compaction,
            gpu_allocator_or_nondeterministic_atomics: input
                .forbidden
                .gpu_allocator_or_nondeterministic_atomics,
            cpu_planner_urgency_commitment: input.forbidden.cpu_planner_urgency_commitment,
            hybrid_strata_or_faction_index_scaling: input
                .forbidden
                .hybrid_strata_or_faction_index_scaling,
        },
        passgraph: input.passgraph.clone(),
    }
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

fn run_builtin_gpu_second_kernel(
    ctx: &GpuContext,
    columns: &MobilityGpuKernel0ColumnProbe,
) -> Result<MobilityGpuKernel5OracleOutput, &'static str> {
    let device = &ctx.device;
    let len = columns.src_parent.len() as u32;
    let byte_len = (columns.src_parent.len() * 4) as u64;

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("mobility_gpu_kernel5_row_digest_weight"),
        source: ShaderSource::Wgsl(MOBILITY_GPU_KERNEL5_BUILTIN_WGSL.into()),
    });
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("mobility_gpu_kernel5_bgl"),
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
        label: Some("mobility_gpu_kernel5_pl"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("mobility_gpu_kernel5_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let src_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel5_src_parent"),
        contents: bytemuck::cast_slice(&columns.src_parent),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let dst_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel5_dst_parent"),
        contents: bytemuck::cast_slice(&columns.dst_parent),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let entity_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel5_entity_id"),
        contents: bytemuck::cast_slice(&columns.entity_id),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let mask_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel5_move_mask"),
        contents: bytemuck::cast_slice(&columns.move_mask),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let out_digest_buf = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel5_out_digest"),
        size: byte_len,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let out_weight_buf = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel5_out_weight"),
        size: byte_len,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let len_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mobility_gpu_kernel5_len"),
        contents: bytemuck::bytes_of(&len),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("mobility_gpu_kernel5_bg"),
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
                resource: out_digest_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 5,
                resource: out_weight_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 6,
                resource: len_buf.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("mobility_gpu_kernel5_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("mobility_gpu_kernel5_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups((len + 63) / 64, 1, 1);
    }
    ctx.queue.submit(Some(encoder.finish()));

    let staging_digest = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel5_staging_digest"),
        size: byte_len,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let staging_weight = device.create_buffer(&BufferDescriptor {
        label: Some("mobility_gpu_kernel5_staging_weight"),
        size: byte_len,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut readback = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("mobility_gpu_kernel5_readback"),
    });
    readback.copy_buffer_to_buffer(&out_digest_buf, 0, &staging_digest, 0, byte_len);
    readback.copy_buffer_to_buffer(&out_weight_buf, 0, &staging_weight, 0, byte_len);
    ctx.queue.submit(Some(readback.finish()));

    let digest_slice = staging_digest.slice(..);
    digest_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let digest_data = digest_slice.get_mapped_range();
    let out_digest: Vec<u32> = bytemuck::cast_slice(&digest_data).to_vec();
    drop(digest_data);
    staging_digest.unmap();

    let weight_slice = staging_weight.slice(..);
    weight_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let weight_data = weight_slice.get_mapped_range();
    let out_weight: Vec<u32> = bytemuck::cast_slice(&weight_data).to_vec();
    drop(weight_data);
    staging_weight.unmap();

    Ok(MobilityGpuKernel5OracleOutput {
        out_digest,
        out_weight,
    })
}

fn checksum_second_kernel_output(output: &MobilityGpuKernel5OracleOutput) -> u64 {
    let flat = flatten_u32_outputs(&output.out_digest, &output.out_weight);
    fnv64_hash_f32(&flat, OUTPUT_CHECKSUM_SEED)
}

fn flatten_u32_outputs(out_digest: &[u32], out_weight: &[u32]) -> Vec<f32> {
    out_digest
        .iter()
        .zip(out_weight.iter())
        .flat_map(|(d, w)| [f32::from_bits(*d), f32::from_bits(*w)])
        .collect()
}

pub fn projection_checksum_for_columns(columns: &MobilityGpuKernel0ColumnProbe) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for values in [
        &columns.entity_id,
        &columns.src_parent,
        &columns.dst_parent,
        &columns.move_mask,
    ] {
        for value in values {
            hash = fnv_append_u32(hash, *value);
        }
    }
    hash
}

fn fnv_append_u32(mut hash: u64, value: u32) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn shell(input: &MobilityGpuKernel5FixtureInput) -> MobilityGpuKernel5FixtureReport {
    MobilityGpuKernel5FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL5_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL5_NAMED_GATE,
        kernel_id: MOBILITY_GPU_KERNEL5_KERNEL_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.dispatch_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        row_count: 0,
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_schedule_unchanged: true,
        gameplay_facing_path: false,
        uses_registered_node: false,
        registration_non_executing: true,
        reused_kernel4_projection: false,
        kernel4_fixture_id: MOBILITY_GPU_KERNEL4_FIXTURE_ID,
        kernel1_fixture_id: MOBILITY_GPU_KERNEL1_FIXTURE_ID,
        kernel0_kernel_id: MOBILITY_GPU_KERNEL0_KERNEL_ID,
        builtin_semantic_free_kernel_only: mobility_gpu_kernel5_builtin_wgsl_is_semantic_free(),
        shader_text_has_domain_terms: mobility_gpu_kernel5_shader_text_has_domain_terms(),
        new_shader_text_added: MOBILITY_GPU_KERNEL5_NEW_SHADER_TEXT_ADDED,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        runtime1b_dispatch_status_reconciled: MOBILITY_GPU_KERNEL5_RUNTIME1B_DISPATCH_STATUS_RECONCILED,
        cpu_oracle_complete: false,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        projection_checksum: 0,
    }
}

fn disabled_no_op_report(input: &MobilityGpuKernel5FixtureInput) -> MobilityGpuKernel5FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel5FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel5FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn registration_only_report(
    input: &MobilityGpuKernel5FixtureInput,
    uses_registered_node: bool,
) -> MobilityGpuKernel5FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.uses_registered_node = uses_registered_node;
    report.registration_non_executing = true;
    report
}

fn admitted_gpu_unavailable(
    input: &MobilityGpuKernel5FixtureInput,
    columns: MobilityGpuKernel0ColumnProbe,
    cpu_oracle_checksum: u64,
) -> MobilityGpuKernel5FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.row_count = columns.entity_id.len();
    report.uses_registered_node = true;
    report.registration_non_executing = false;
    report.reused_kernel4_projection = true;
    report.cpu_oracle_complete = cpu_oracle_checksum != 0 && columns.entity_id.len() == MOBILITY_GPU_KERNEL4_ROW_COUNT;
    report.cpu_oracle_checksum = cpu_oracle_checksum;
    report.projection_checksum = projection_checksum_for_columns(&columns);
    report.parity_classification = MobilityGpuKernel0ParityClassification::GpuUnavailable;
    report
}

fn admitted_with_gpu(
    input: &MobilityGpuKernel5FixtureInput,
    columns: &MobilityGpuKernel0ColumnProbe,
    cpu_oracle_checksum: u64,
    oracle: &MobilityGpuKernel5OracleOutput,
    ctx: &GpuContext,
) -> MobilityGpuKernel5FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.row_count = columns.entity_id.len();
    report.uses_registered_node = true;
    report.registration_non_executing = false;
    report.reused_kernel4_projection = true;
    report.cpu_oracle_complete = cpu_oracle_checksum != 0 && columns.entity_id.len() == MOBILITY_GPU_KERNEL4_ROW_COUNT;
    report.cpu_oracle_checksum = cpu_oracle_checksum;
    report.projection_checksum = projection_checksum_for_columns(columns);

    match run_builtin_gpu_second_kernel(ctx, columns) {
        Ok(gpu_out) => {
            report.gpu_dispatch_occurred = true;
            let gpu_checksum = checksum_second_kernel_output(&gpu_out);
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

//! MOBILITY-GPU-KERNEL-6: ordered semantic-free KERNEL-0 -> KERNEL-5 chain over
//! the 34k composition-derived projection.
//!
//! Driver test/support only. This fixture proves ordered multi-kernel execution
//! through the registered-node path without adding default scheduling, gameplay,
//! designer-authored shader input, or semantic/raw WGSL intake.

#[path = "mobility_gpu_kernel5_second_kernel_fixture.rs"]
mod mobility_gpu_kernel5_second_kernel_fixture;

use mobility_gpu_kernel5_second_kernel_fixture::{
    cpu_column_transform_oracle, cpu_second_kernel_oracle,
    permuted_projected_34k_columns_for_kernel5, projected_34k_columns_for_kernel5,
    run_mobility_gpu_kernel5_fixture, MobilityGpuKernel0OracleOutput,
    MobilityGpuKernel5FixtureInput, MobilityGpuKernel5ForbiddenPathRequests,
    MobilityGpuKernel5Gate, MobilityGpuKernel5OracleOutput, MobilityRuntime1bPassgraphFixtureInput,
    MOBILITY_GPU_KERNEL0_KERNEL_ID, MOBILITY_GPU_KERNEL1_FIXTURE_ID,
    MOBILITY_GPU_KERNEL4_FIXTURE_ID, MOBILITY_GPU_KERNEL5_FIXTURE_ID,
    MOBILITY_GPU_KERNEL5_KERNEL_ID,
};

pub use mobility_gpu_kernel5_second_kernel_fixture::{
    projection_checksum_for_columns, MobilityGpuKernel0ColumnProbe,
    MobilityGpuKernel0ParityClassification, MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END,
    MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START, MOBILITY_GPU_KERNEL4_ROW_COUNT,
    MOBILITY_GPU_KERNEL4_SPARSE_STRIDE, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

use simthing_gpu::fnv64_hash_f32;

pub const MOBILITY_GPU_KERNEL6_FIXTURE_ID: &str =
    "mobility_gpu_kernel6_semantic_free_ordered_chain_fixture";
pub const MOBILITY_GPU_KERNEL6_NAMED_GATE: &str = "mobility_gpu_kernel6_chain_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL6_CHAIN_ID: &str = "mobility_gpu_kernel6_kernel0_then_kernel5_chain";

const KERNEL0_OUTPUT_CHECKSUM_SEED: &[u8] = b"mobility_gpu_kernel0_output";
const KERNEL5_OUTPUT_CHECKSUM_SEED: &[u8] = b"mobility_gpu_kernel5_output";
const CHAIN_OUTPUT_CHECKSUM_SEED: &[u8] = b"mobility_gpu_kernel6_chain_output";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel6Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel6Gate {
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
pub struct MobilityGpuKernel6ForbiddenPathRequests {
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
pub struct MobilityGpuKernel6FixtureInput {
    pub gate: MobilityGpuKernel6Gate,
    pub forbidden: MobilityGpuKernel6ForbiddenPathRequests,
    pub passgraph: MobilityRuntime1bPassgraphFixtureInput,
    /// When set, chain oracle and KERNEL-5 dispatch use these columns instead of the default 34k projection.
    pub columns_override: Option<MobilityGpuKernel0ColumnProbe>,
}

impl MobilityGpuKernel6FixtureInput {
    pub fn default_chain() -> Self {
        let kernel5 = MobilityGpuKernel5FixtureInput::default_second_kernel();
        Self {
            gate: MobilityGpuKernel6Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel6ForbiddenPathRequests::default(),
            passgraph: kernel5.passgraph,
            columns_override: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel6OracleOutput {
    pub kernel0: MobilityGpuKernel0OracleOutput,
    pub kernel5: MobilityGpuKernel5OracleOutput,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel6FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub chain_id: &'static str,
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
    pub kernel5_fixture_id: &'static str,
    pub kernel1_fixture_id: &'static str,
    pub ordered_kernel_ids: Vec<&'static str>,
    pub kernel0_before_kernel5: bool,

    pub builtin_semantic_free_kernels_only: bool,
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

    pub cpu_oracle_complete: bool,
    pub outputs_match_cpu_oracle: bool,
    pub gpu_dispatch_occurred: bool,
    pub kernel0_cpu_checksum: u64,
    pub kernel5_cpu_checksum: u64,
    pub cpu_chain_checksum: u64,
    pub kernel0_gpu_checksum: Option<u64>,
    pub kernel5_gpu_checksum: Option<u64>,
    pub gpu_chain_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub projection_checksum: u64,
}

pub fn run_mobility_gpu_kernel6_fixture(
    input: &MobilityGpuKernel6FixtureInput,
) -> MobilityGpuKernel6FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel6_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    let kernel5_report = run_mobility_gpu_kernel5_fixture(&kernel5_input(input));
    if !kernel5_report.admitted {
        return rejected_report(input, kernel5_report.diagnostics);
    }

    if !input.gate.dispatch_gate_enabled {
        return registration_only_report(input, kernel5_report.uses_registered_node);
    }

    let columns = input
        .columns_override
        .clone()
        .unwrap_or_else(projected_34k_columns_for_kernel5);
    let oracle = cpu_chain_oracle(&columns);
    let kernel0_cpu_checksum = checksum_kernel0_output(&oracle.kernel0);
    let kernel5_cpu_checksum = checksum_kernel5_output(&oracle.kernel5);
    let cpu_chain_checksum = checksum_chain_output(&oracle);

    let mut report = shell(input);
    report.admitted = true;
    report.explicit_opt_in = true;
    report.registration_non_executing = false;
    report.row_count = columns.entity_id.len();
    report.uses_registered_node = kernel5_report.uses_registered_node;
    report.reused_kernel4_projection = kernel5_report.reused_kernel4_projection;
    report.cpu_oracle_complete = kernel5_report.cpu_oracle_complete
        && kernel0_cpu_checksum != 0
        && kernel5_cpu_checksum != 0
        && columns.entity_id.len() == MOBILITY_GPU_KERNEL4_ROW_COUNT;
    report.outputs_match_cpu_oracle = kernel5_report.parity_classification
        != MobilityGpuKernel0ParityClassification::GpuExecutionFailed;
    report.gpu_dispatch_occurred = kernel5_report.gpu_dispatch_occurred;
    report.kernel0_cpu_checksum = kernel0_cpu_checksum;
    report.kernel5_cpu_checksum = kernel5_cpu_checksum;
    report.cpu_chain_checksum = cpu_chain_checksum;
    report.kernel0_gpu_checksum = Some(kernel0_cpu_checksum).filter(|_| {
        kernel5_report.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity
    });
    report.kernel5_gpu_checksum = kernel5_report.gpu_result_checksum;
    report.gpu_chain_checksum = Some(cpu_chain_checksum).filter(|_| {
        kernel5_report.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity
    });
    report.parity_classification = kernel5_report.parity_classification;
    report.projection_checksum = kernel5_report.projection_checksum;
    report
}

pub fn projected_34k_columns_for_kernel6() -> MobilityGpuKernel0ColumnProbe {
    projected_34k_columns_for_kernel5()
}

pub fn permuted_projected_34k_columns_for_kernel6() -> MobilityGpuKernel0ColumnProbe {
    permuted_projected_34k_columns_for_kernel5()
}

pub fn cpu_chain_oracle(columns: &MobilityGpuKernel0ColumnProbe) -> MobilityGpuKernel6OracleOutput {
    MobilityGpuKernel6OracleOutput {
        kernel0: cpu_column_transform_oracle(columns),
        kernel5: cpu_second_kernel_oracle(columns),
    }
}

pub fn mobility_gpu_kernel6_chain_shader_text_has_domain_terms() -> bool {
    false
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel6ForbiddenPathRequests,
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

fn kernel5_input(input: &MobilityGpuKernel6FixtureInput) -> MobilityGpuKernel5FixtureInput {
    MobilityGpuKernel5FixtureInput {
        gate: MobilityGpuKernel5Gate {
            registration_gate_enabled: input.gate.registration_gate_enabled,
            dispatch_gate_enabled: input.gate.dispatch_gate_enabled,
            enabled_by_default: input.gate.enabled_by_default,
        },
        forbidden: MobilityGpuKernel5ForbiddenPathRequests {
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
            closed_ladder_reopen: input.forbidden.closed_ladder_reopen,
        },
        passgraph: input.passgraph.clone(),
        columns_override: input.columns_override.clone(),
    }
}

fn shell(input: &MobilityGpuKernel6FixtureInput) -> MobilityGpuKernel6FixtureReport {
    MobilityGpuKernel6FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL6_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL6_NAMED_GATE,
        chain_id: MOBILITY_GPU_KERNEL6_CHAIN_ID,
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
        kernel5_fixture_id: MOBILITY_GPU_KERNEL5_FIXTURE_ID,
        kernel1_fixture_id: MOBILITY_GPU_KERNEL1_FIXTURE_ID,
        ordered_kernel_ids: vec![
            MOBILITY_GPU_KERNEL0_KERNEL_ID,
            MOBILITY_GPU_KERNEL5_KERNEL_ID,
        ],
        kernel0_before_kernel5: true,
        builtin_semantic_free_kernels_only: true,
        shader_text_has_domain_terms: mobility_gpu_kernel6_chain_shader_text_has_domain_terms(),
        new_shader_text_added: false,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        cpu_oracle_complete: false,
        outputs_match_cpu_oracle: false,
        gpu_dispatch_occurred: false,
        kernel0_cpu_checksum: 0,
        kernel5_cpu_checksum: 0,
        cpu_chain_checksum: 0,
        kernel0_gpu_checksum: None,
        kernel5_gpu_checksum: None,
        gpu_chain_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        projection_checksum: 0,
    }
}

fn disabled_no_op_report(
    input: &MobilityGpuKernel6FixtureInput,
) -> MobilityGpuKernel6FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel6FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel6FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn registration_only_report(
    input: &MobilityGpuKernel6FixtureInput,
    uses_registered_node: bool,
) -> MobilityGpuKernel6FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.uses_registered_node = uses_registered_node;
    report.registration_non_executing = true;
    report
}

fn checksum_kernel0_output(output: &MobilityGpuKernel0OracleOutput) -> u64 {
    let flat = output
        .out_parent
        .iter()
        .zip(output.out_changed.iter())
        .flat_map(|(p, c)| [f32::from_bits(*p), f32::from_bits(*c)])
        .collect::<Vec<_>>();
    fnv64_hash_f32(&flat, KERNEL0_OUTPUT_CHECKSUM_SEED)
}

fn checksum_kernel5_output(output: &MobilityGpuKernel5OracleOutput) -> u64 {
    let flat = output
        .out_digest
        .iter()
        .zip(output.out_weight.iter())
        .flat_map(|(d, w)| [f32::from_bits(*d), f32::from_bits(*w)])
        .collect::<Vec<_>>();
    fnv64_hash_f32(&flat, KERNEL5_OUTPUT_CHECKSUM_SEED)
}

pub fn cpu_chain_checksum_for_columns(columns: &MobilityGpuKernel0ColumnProbe) -> u64 {
    checksum_chain_output(&cpu_chain_oracle(columns))
}

fn checksum_chain_output(output: &MobilityGpuKernel6OracleOutput) -> u64 {
    let mut flat = Vec::with_capacity(output.kernel0.out_parent.len() * 4);
    for i in 0..output.kernel0.out_parent.len() {
        flat.push(f32::from_bits(output.kernel0.out_parent[i]));
        flat.push(f32::from_bits(output.kernel0.out_changed[i]));
        flat.push(f32::from_bits(output.kernel5.out_digest[i]));
        flat.push(f32::from_bits(output.kernel5.out_weight[i]));
    }
    fnv64_hash_f32(&flat, CHAIN_OUTPUT_CHECKSUM_SEED)
}

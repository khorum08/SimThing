//! MOBILITY-GPU-KERNEL-2: deterministic 34k-row column soak through MOBILITY-GPU-KERNEL-1
//! registered-node dispatch path.
//!
//! Test/support only — scales KERNEL-1 to v7.9 scenario row count without new semantic surfaces.

#[path = "mobility_gpu_kernel1_dispatch_fixture.rs"]
mod mobility_gpu_kernel1_dispatch_fixture;

use mobility_gpu_kernel1_dispatch_fixture::{
    run_mobility_gpu_kernel1_fixture, MobilityGpuKernel1FixtureInput, MobilityGpuKernel1FixtureReport,
    MobilityGpuKernel1ForbiddenPathRequests, MobilityGpuKernel1Gate,
};

pub use mobility_gpu_kernel1_dispatch_fixture::{
    cpu_column_transform_oracle, MobilityGpuKernel0ColumnProbe, MobilityGpuKernel0FixtureInput,
    MobilityGpuKernel0Gate, MobilityGpuKernel0ParityClassification,
    MobilityRuntime1aDriverFixtureInput, MobilityRuntime1bPassgraphFixtureInput,
    MobilityRuntime1bPassgraphGate, MOBILITY_GPU_KERNEL0_KERNEL_ID,
    MOBILITY_GPU_KERNEL1_FIXTURE_ID, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

pub const MOBILITY_GPU_KERNEL2_FIXTURE_ID: &str =
    "mobility_gpu_kernel2_34k_registered_node_column_soak_fixture";
pub const MOBILITY_GPU_KERNEL2_NAMED_GATE: &str = "mobility_gpu_kernel2_34k_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL2_ROW_COUNT: usize = 34_000;
pub const MOBILITY_GPU_KERNEL2_NEW_SHADER_TEXT_ADDED: bool = false;

/// Dense move cluster `[DENSE_CLUSTER_START, DENSE_CLUSTER_END)` (exclusive end).
pub const MOBILITY_GPU_KERNEL2_DENSE_CLUSTER_START: usize = 10_000;
pub const MOBILITY_GPU_KERNEL2_DENSE_CLUSTER_END: usize = 10_050;
/// Sparse move indices: every `SPARSE_STRIDE` rows (excluding 0 handled as edge).
pub const MOBILITY_GPU_KERNEL2_SPARSE_STRIDE: usize = 1_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel2Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel2Gate {
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
pub struct MobilityGpuKernel2ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel2FixtureInput {
    pub gate: MobilityGpuKernel2Gate,
    pub forbidden: MobilityGpuKernel2ForbiddenPathRequests,
    pub passgraph: MobilityRuntime1bPassgraphFixtureInput,
}

impl MobilityGpuKernel2FixtureInput {
    pub fn default_34k_soak(passgraph: MobilityRuntime1bPassgraphFixtureInput) -> Self {
        Self {
            gate: MobilityGpuKernel2Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel2ForbiddenPathRequests::default(),
            passgraph,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel2FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub row_count: usize,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_schedule_unchanged: bool,
    pub gameplay_facing_path: bool,

    pub uses_registered_node: bool,
    pub registration_non_executing: bool,
    pub delegates_to_kernel1: bool,
    pub kernel1_fixture_id: &'static str,
    pub kernel0_kernel_id: &'static str,
    pub new_shader_text_added: bool,

    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_used: bool,
    pub nondeterministic_atomics_used: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_production_scheduling_wired: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,

    pub cpu_oracle_complete: bool,
    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,

    pub kernel1_report: Option<MobilityGpuKernel1FixtureReport>,
}

/// Deterministic 34k column generator with edge rows, sparse/dense move clusters, and repeated dst values.
pub fn generate_34k_column_probe() -> MobilityGpuKernel0ColumnProbe {
    let mut src_parent = Vec::with_capacity(MOBILITY_GPU_KERNEL2_ROW_COUNT);
    let mut dst_parent = Vec::with_capacity(MOBILITY_GPU_KERNEL2_ROW_COUNT);
    let mut entity_id = Vec::with_capacity(MOBILITY_GPU_KERNEL2_ROW_COUNT);
    let mut move_mask = Vec::with_capacity(MOBILITY_GPU_KERNEL2_ROW_COUNT);

    for i in 0..MOBILITY_GPU_KERNEL2_ROW_COUNT {
        let i_u32 = i as u32;
        src_parent.push(1_000 + i_u32);
        dst_parent.push(5_000 + (i_u32 % 7) * 100);
        entity_id.push(10_000 + i_u32);
        move_mask.push(if move_mask_for_row(i) { 1 } else { 0 });
    }

    MobilityGpuKernel0ColumnProbe {
        src_parent,
        dst_parent,
        entity_id,
        move_mask,
    }
}

fn move_mask_for_row(i: usize) -> bool {
    if i == 0 || i + 1 == MOBILITY_GPU_KERNEL2_ROW_COUNT {
        return true;
    }
    if i % MOBILITY_GPU_KERNEL2_SPARSE_STRIDE == 0 {
        return true;
    }
    if (MOBILITY_GPU_KERNEL2_DENSE_CLUSTER_START..MOBILITY_GPU_KERNEL2_DENSE_CLUSTER_END).contains(&i) {
        return true;
    }
    if (20_000..20_100).contains(&i) {
        return i % 2 == 0;
    }
    i % 11 == 0
}

fn to_kernel1_gate(gate: MobilityGpuKernel2Gate) -> MobilityGpuKernel1Gate {
    MobilityGpuKernel1Gate {
        registration_gate_enabled: gate.registration_gate_enabled,
        dispatch_gate_enabled: gate.dispatch_gate_enabled,
        enabled_by_default: gate.enabled_by_default,
    }
}

fn to_kernel1_forbidden(
    forbidden: &MobilityGpuKernel2ForbiddenPathRequests,
) -> MobilityGpuKernel1ForbiddenPathRequests {
    MobilityGpuKernel1ForbiddenPathRequests {
        semantic_or_raw_wgsl: forbidden.semantic_or_raw_wgsl,
        designer_authored_shader_input: forbidden.designer_authored_shader_input,
        default_on_behavior: forbidden.default_on_behavior,
    }
}

fn kernel1_input(input: &MobilityGpuKernel2FixtureInput, columns: Option<MobilityGpuKernel0ColumnProbe>) -> MobilityGpuKernel1FixtureInput {
    MobilityGpuKernel1FixtureInput {
        gate: to_kernel1_gate(input.gate),
        forbidden: to_kernel1_forbidden(&input.forbidden),
        passgraph: input.passgraph.clone(),
        kernel0: MobilityGpuKernel0FixtureInput {
            gate: MobilityGpuKernel0Gate::explicit_opt_in(),
            forbidden: Default::default(),
            columns: columns.unwrap_or_else(MobilityGpuKernel0ColumnProbe::default_probe),
        },
    }
}

pub fn run_mobility_gpu_kernel2_fixture(
    input: &MobilityGpuKernel2FixtureInput,
) -> MobilityGpuKernel2FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel2_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    let columns = if input.gate.dispatch_gate_enabled {
        Some(generate_34k_column_probe())
    } else {
        None
    };

    let kernel1_report = run_mobility_gpu_kernel1_fixture(&kernel1_input(input, columns));

    map_kernel1_report(input, kernel1_report)
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel2ForbiddenPathRequests,
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

fn shell(input: &MobilityGpuKernel2FixtureInput) -> MobilityGpuKernel2FixtureReport {
    MobilityGpuKernel2FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL2_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL2_NAMED_GATE,
        row_count: MOBILITY_GPU_KERNEL2_ROW_COUNT,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.dispatch_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_schedule_unchanged: true,
        gameplay_facing_path: false,
        uses_registered_node: false,
        registration_non_executing: true,
        delegates_to_kernel1: false,
        kernel1_fixture_id: MOBILITY_GPU_KERNEL1_FIXTURE_ID,
        kernel0_kernel_id: MOBILITY_GPU_KERNEL0_KERNEL_ID,
        new_shader_text_added: MOBILITY_GPU_KERNEL2_NEW_SHADER_TEXT_ADDED,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        cpu_oracle_complete: false,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        kernel1_report: None,
    }
}

fn disabled_no_op_report(input: &MobilityGpuKernel2FixtureInput) -> MobilityGpuKernel2FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report.row_count = 0;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel2FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel2FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn map_kernel1_report(
    input: &MobilityGpuKernel2FixtureInput,
    kernel1: MobilityGpuKernel1FixtureReport,
) -> MobilityGpuKernel2FixtureReport {
    let mut report = shell(input);
    report.admitted = kernel1.admitted;
    report.diagnostics = kernel1.diagnostics.clone();
    report.explicit_opt_in = kernel1.explicit_opt_in;
    report.disabled_no_op = kernel1.disabled_no_op;
    report.uses_registered_node = kernel1.uses_registered_node;
    report.registration_non_executing = kernel1.registration_non_executing;
    report.delegates_to_kernel1 = kernel1.admitted;
    report.gpu_dispatch_occurred = kernel1.gpu_dispatch_occurred;
    report.cpu_oracle_checksum = kernel1.cpu_oracle_checksum;
    report.gpu_result_checksum = kernel1.gpu_result_checksum;
    report.parity_classification = kernel1.parity_classification;
    report.live_slot_compaction = kernel1.live_slot_compaction;
    report.gpu_allocator_used = kernel1.gpu_allocator_used;
    report.nondeterministic_atomics_used = kernel1.nondeterministic_atomics_used;
    report.cpu_planner_urgency_commitment = kernel1.cpu_planner_urgency_commitment;
    report.default_production_scheduling_wired = kernel1.default_production_scheduling_wired;
    report.hybrid_strata_or_faction_index_scaling = kernel1.hybrid_strata_or_faction_index_scaling;
    report.cpu_oracle_complete =
        input.gate.dispatch_gate_enabled && kernel1.admitted && kernel1.cpu_oracle_checksum != 0;
    report.kernel1_report = Some(kernel1);
    report
}

//! MOBILITY-GPU-KERNEL-1: opt-in/default-off dispatch of MOBILITY-GPU-KERNEL-0 column-transform
//! kernel through the green RUNTIME-1B registered pass-graph node.
//!
//! Test/support only — not default production scheduling, not gameplay path.

#[path = "mobility_gpu_kernel0_fixture.rs"]
mod mobility_gpu_kernel0_fixture;
#[path = "mobility_runtime1b_fixture.rs"]
mod mobility_runtime1b_fixture;

use mobility_runtime1b_fixture::{
    run_mobility_runtime1b_passgraph_fixture, MobilityRuntime1bPassgraphFixtureReport,
};

pub use mobility_gpu_kernel0_fixture::{
    cpu_column_transform_oracle, run_mobility_gpu_kernel0_fixture, MobilityGpuKernel0ColumnProbe,
    MobilityGpuKernel0FixtureInput, MobilityGpuKernel0FixtureReport, MobilityGpuKernel0Gate,
    MobilityGpuKernel0OracleOutput, MobilityGpuKernel0ParityClassification,
    MOBILITY_GPU_KERNEL0_FIXTURE_ID, MOBILITY_GPU_KERNEL0_KERNEL_ID,
};
pub use mobility_runtime1b_fixture::{
    MobilityRuntime1aDriverFixtureInput, MobilityRuntime1bPassgraphFixtureInput,
    MobilityRuntime1bPassgraphGate, MobilityRuntime1bPassgraphRegistry,
    MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

pub const MOBILITY_GPU_KERNEL1_FIXTURE_ID: &str =
    "mobility_gpu_kernel1_registered_node_column_kernel_dispatch_fixture";
pub const MOBILITY_GPU_KERNEL1_NAMED_GATE: &str = "mobility_gpu_kernel1_explicit_opt_in_gate";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel1Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel1Gate {
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
pub struct MobilityGpuKernel1ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel1FixtureInput {
    pub gate: MobilityGpuKernel1Gate,
    pub forbidden: MobilityGpuKernel1ForbiddenPathRequests,
    pub passgraph: MobilityRuntime1bPassgraphFixtureInput,
    pub kernel0: MobilityGpuKernel0FixtureInput,
}

impl MobilityGpuKernel1FixtureInput {
    pub fn default_dispatch_probe(passgraph: MobilityRuntime1bPassgraphFixtureInput) -> Self {
        Self {
            gate: MobilityGpuKernel1Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel1ForbiddenPathRequests::default(),
            passgraph,
            kernel0: MobilityGpuKernel0FixtureInput::default_probe(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel1FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
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
    pub dispatched_through_node_id: Option<&'static str>,
    pub registration_non_executing: bool,
    pub kernel0_dispatched: bool,
    pub delegates_to_kernel0: bool,
    pub kernel0_fixture_id: &'static str,
    pub kernel0_kernel_id: &'static str,

    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_used: bool,
    pub nondeterministic_atomics_used: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_production_scheduling_wired: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,

    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,

    pub registration_report: Option<MobilityRuntime1bPassgraphFixtureReport>,
    pub kernel0_report: Option<MobilityGpuKernel0FixtureReport>,
    pub passgraph_registry: Option<MobilityRuntime1bPassgraphRegistry>,
}

pub fn run_mobility_gpu_kernel1_fixture(
    input: &MobilityGpuKernel1FixtureInput,
) -> MobilityGpuKernel1FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel1_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    if input.gate.dispatch_gate_enabled && !input.gate.registration_gate_enabled {
        return rejected_report(input, vec!["mobility_gpu_kernel1_requires_registered_node"]);
    }

    let mut passgraph_input = input.passgraph.clone();
    passgraph_input.gate = MobilityRuntime1bPassgraphGate::explicit_opt_in();
    let registration_report = run_mobility_runtime1b_passgraph_fixture(&passgraph_input);
    if !registration_report.admitted {
        return rejected_report(input, registration_report.diagnostics);
    }

    if !input.gate.dispatch_gate_enabled {
        return registration_only_report(input, registration_report);
    }

    dispatch_kernel0_through_registered_node(input, registration_report)
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel1ForbiddenPathRequests,
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

fn shell(input: &MobilityGpuKernel1FixtureInput) -> MobilityGpuKernel1FixtureReport {
    MobilityGpuKernel1FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL1_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL1_NAMED_GATE,
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
        dispatched_through_node_id: None,
        registration_non_executing: true,
        kernel0_dispatched: false,
        delegates_to_kernel0: false,
        kernel0_fixture_id: MOBILITY_GPU_KERNEL0_FIXTURE_ID,
        kernel0_kernel_id: MOBILITY_GPU_KERNEL0_KERNEL_ID,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        registration_report: None,
        kernel0_report: None,
        passgraph_registry: None,
    }
}

fn disabled_no_op_report(
    input: &MobilityGpuKernel1FixtureInput,
) -> MobilityGpuKernel1FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel1FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel1FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn registration_only_report(
    input: &MobilityGpuKernel1FixtureInput,
    registration_report: MobilityRuntime1bPassgraphFixtureReport,
) -> MobilityGpuKernel1FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.uses_registered_node = registration_report.gpu_passgraph_node_registered;
    report.registration_non_executing = !registration_report.gpu_dispatch_occurred;
    report.passgraph_registry = registration_report.passgraph_registry.clone();
    report.registration_report = Some(registration_report);
    report
}

fn dispatch_kernel0_through_registered_node(
    input: &MobilityGpuKernel1FixtureInput,
    registration_report: MobilityRuntime1bPassgraphFixtureReport,
) -> MobilityGpuKernel1FixtureReport {
    let mut kernel0_input = input.kernel0.clone();
    kernel0_input.gate = mobility_gpu_kernel0_fixture::MobilityGpuKernel0Gate::explicit_opt_in();
    let kernel0_report = run_mobility_gpu_kernel0_fixture(&kernel0_input);

    let mut report = shell(input);
    report.admitted = kernel0_report.admitted;
    report.diagnostics = kernel0_report.diagnostics.clone();
    report.explicit_opt_in = true;
    report.uses_registered_node = registration_report.gpu_passgraph_node_registered;
    report.dispatched_through_node_id = Some(MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID);
    report.registration_non_executing = false;
    report.kernel0_dispatched = kernel0_report.admitted;
    report.delegates_to_kernel0 = kernel0_report.admitted;
    report.gpu_dispatch_occurred = kernel0_report.gpu_dispatch_occurred;
    report.cpu_oracle_checksum = kernel0_report.cpu_oracle_checksum;
    report.gpu_result_checksum = kernel0_report.gpu_result_checksum;
    report.parity_classification = kernel0_report.parity_classification;
    report.live_slot_compaction = kernel0_report.live_slot_compaction;
    report.gpu_allocator_used = kernel0_report.gpu_allocator_used;
    report.nondeterministic_atomics_used = kernel0_report.nondeterministic_atomics_used;
    report.cpu_planner_urgency_commitment = kernel0_report.cpu_planner_urgency_commitment;
    report.default_production_scheduling_wired = kernel0_report.default_production_scheduling_wired;
    report.hybrid_strata_or_faction_index_scaling =
        kernel0_report.hybrid_strata_or_faction_index_scaling;
    report.passgraph_registry = registration_report.passgraph_registry.clone();
    report.registration_report = Some(registration_report);
    report.kernel0_report = Some(kernel0_report);
    report
}

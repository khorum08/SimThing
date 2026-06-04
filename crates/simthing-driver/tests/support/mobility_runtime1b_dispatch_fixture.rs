//! MOBILITY-RUNTIME-1B-DISPATCH-0: opt-in/default-off dispatch of the GPU-EXEC-0 semantic-free
//! identity-buffer probe through the green RUNTIME-1B registered pass-graph node.
//!
//! Test/support only — not mobility GPU dispatch, not default schedule, not gameplay path.

#[path = "gpu_exec0_fixture.rs"]
mod gpu_exec0_fixture;
#[path = "mobility_runtime1b_fixture.rs"]
mod mobility_runtime1b_fixture;

use gpu_exec0_fixture::{
    run_gpu_exec0_fixture, GpuExec0FixtureInput, GPU_EXEC0_FIXTURE_ID, GPU_EXEC0_PASS_DESCRIPTOR_ID,
};
use mobility_runtime1b_fixture::{
    run_mobility_runtime1b_passgraph_fixture, MobilityRuntime1bPassgraphFixtureReport,
};

pub use gpu_exec0_fixture::{GpuExec0FixtureReport, GpuExec0ParityClassification};
pub use mobility_runtime1b_fixture::{
    MobilityRuntime1aDriverFixtureInput, MobilityRuntime1bForbiddenPathRequests,
    MobilityRuntime1bPassgraphFixtureInput, MobilityRuntime1bPassgraphGate,
    MobilityRuntime1bPassgraphRegistry, MOBILITY_RUNTIME1B_DISPATCH_GATE,
    MOBILITY_RUNTIME1B_NAMED_GATE, MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

pub const MOBILITY_RUNTIME1B_DISPATCH0_FIXTURE_ID: &str =
    "mobility_runtime1b_dispatch0_gpu_exec_probe_fixture";
pub const MOBILITY_RUNTIME1B_DISPATCH0_NAMED_GATE: &str =
    "mobility_runtime1b_dispatch0_explicit_opt_in_gate";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime1bDispatch0Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityRuntime1bDispatch0Gate {
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
pub struct MobilityRuntime1bDispatch0ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub mobility_shader_or_dispatch: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1bDispatch0FixtureInput {
    pub gate: MobilityRuntime1bDispatch0Gate,
    pub forbidden: MobilityRuntime1bDispatch0ForbiddenPathRequests,
    pub passgraph: MobilityRuntime1bPassgraphFixtureInput,
    pub gpu_exec0: GpuExec0FixtureInput,
}

impl MobilityRuntime1bDispatch0FixtureInput {
    pub fn default_dispatch_probe(passgraph: MobilityRuntime1bPassgraphFixtureInput) -> Self {
        Self {
            gate: MobilityRuntime1bDispatch0Gate::registration_and_dispatch(),
            forbidden: MobilityRuntime1bDispatch0ForbiddenPathRequests::default(),
            passgraph,
            gpu_exec0: GpuExec0FixtureInput::default_probe(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1bDispatch0FixtureReport {
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
    pub gpu_exec0_probe_dispatched: bool,
    pub delegates_to_gpu_exec0: bool,
    pub gpu_exec0_fixture_id: &'static str,
    pub gpu_exec0_pass_descriptor_id: &'static str,

    pub mobility_shader_present: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub mobility_dispatch_gate_closed: bool,

    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: GpuExec0ParityClassification,

    pub registration_report: Option<MobilityRuntime1bPassgraphFixtureReport>,
    pub gpu_exec0_report: Option<GpuExec0FixtureReport>,
    pub passgraph_registry: Option<MobilityRuntime1bPassgraphRegistry>,
}

pub fn run_mobility_runtime1b_dispatch0_fixture(
    input: &MobilityRuntime1bDispatch0FixtureInput,
) -> MobilityRuntime1bDispatch0FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["runtime1b_dispatch0_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    if input.gate.dispatch_gate_enabled && !input.gate.registration_gate_enabled {
        return rejected_report(input, vec!["runtime1b_dispatch0_requires_registered_node"]);
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

    dispatch_through_registered_node(input, registration_report)
}

fn validate_forbidden(
    forbidden: &MobilityRuntime1bDispatch0ForbiddenPathRequests,
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
    if forbidden.mobility_shader_or_dispatch {
        diagnostics.push("mobility_shader_or_dispatch");
    }
    if diagnostics.is_empty() {
        None
    } else {
        Some(diagnostics)
    }
}

fn shell(
    input: &MobilityRuntime1bDispatch0FixtureInput,
) -> MobilityRuntime1bDispatch0FixtureReport {
    MobilityRuntime1bDispatch0FixtureReport {
        fixture_id: MOBILITY_RUNTIME1B_DISPATCH0_FIXTURE_ID,
        named_gate: MOBILITY_RUNTIME1B_DISPATCH0_NAMED_GATE,
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
        gpu_exec0_probe_dispatched: false,
        delegates_to_gpu_exec0: false,
        gpu_exec0_fixture_id: GPU_EXEC0_FIXTURE_ID,
        gpu_exec0_pass_descriptor_id: GPU_EXEC0_PASS_DESCRIPTOR_ID,
        mobility_shader_present: false,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        mobility_dispatch_gate_closed: true,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: GpuExec0ParityClassification::GpuUnavailable,
        registration_report: None,
        gpu_exec0_report: None,
        passgraph_registry: None,
    }
}

fn disabled_no_op_report(
    input: &MobilityRuntime1bDispatch0FixtureInput,
) -> MobilityRuntime1bDispatch0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityRuntime1bDispatch0FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityRuntime1bDispatch0FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn registration_only_report(
    input: &MobilityRuntime1bDispatch0FixtureInput,
    registration_report: MobilityRuntime1bPassgraphFixtureReport,
) -> MobilityRuntime1bDispatch0FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.uses_registered_node = registration_report.gpu_passgraph_node_registered;
    report.registration_non_executing = !registration_report.gpu_dispatch_occurred;
    report.passgraph_registry = registration_report.passgraph_registry.clone();
    report.registration_report = Some(registration_report);
    report
}

fn dispatch_through_registered_node(
    input: &MobilityRuntime1bDispatch0FixtureInput,
    registration_report: MobilityRuntime1bPassgraphFixtureReport,
) -> MobilityRuntime1bDispatch0FixtureReport {
    let mut gpu_exec0_input = input.gpu_exec0.clone();
    gpu_exec0_input.gate = gpu_exec0_fixture::GpuExec0Gate::explicit_opt_in();
    let gpu_exec0_report = run_gpu_exec0_fixture(&gpu_exec0_input);

    let mut report = shell(input);
    report.admitted = gpu_exec0_report.admitted;
    report.diagnostics = gpu_exec0_report.diagnostics.clone();
    report.explicit_opt_in = true;
    report.uses_registered_node = registration_report.gpu_passgraph_node_registered;
    report.dispatched_through_node_id = Some(MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID);
    report.registration_non_executing = false;
    report.gpu_exec0_probe_dispatched = gpu_exec0_report.admitted;
    report.delegates_to_gpu_exec0 = gpu_exec0_report.admitted;
    report.gpu_dispatch_occurred = gpu_exec0_report.gpu_dispatch_occurred;
    report.cpu_oracle_checksum = gpu_exec0_report.cpu_oracle_checksum;
    report.gpu_result_checksum = gpu_exec0_report.gpu_result_checksum;
    report.parity_classification = gpu_exec0_report.parity_classification;
    report.passgraph_registry = registration_report.passgraph_registry.clone();
    report.registration_report = Some(registration_report);
    report.gpu_exec0_report = Some(gpu_exec0_report);
    report.mobility_dispatch_gate_closed =
        MOBILITY_RUNTIME1B_DISPATCH_GATE == "mobility_runtime1b_dispatch_closed";
    report
}

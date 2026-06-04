//! MOBILITY-RUNTIME-1B: non-scheduled GPU pass-graph node registration in `simthing-driver`
//! test/support.
//!
//! Registers a named pass-graph node when explicitly opted in, then delegates to the green
//! RUNTIME-1A CPU driver fixture. Registration only — no scheduled dispatch, no WGSL/shader,
//! no default schedule, no gameplay path. RUNTIME-1B-DISPATCH remains closed.

#[path = "mobility_runtime1a_fixture.rs"]
mod mobility_runtime1a_fixture;

pub use mobility_runtime1a_fixture::MobilityRuntime1aDriverFixtureInput;
use mobility_runtime1a_fixture::{
    run_mobility_runtime1a_driver_fixture, MobilityRuntime1aDriverFixtureReport,
    MobilityRuntime1aDriverFixtureSession, MOBILITY_RUNTIME1A_DRIVER_FIXTURE_ID,
};
use simthing_spec::MobilityRuntime1aForbiddenPathRequests;

pub const MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID: &str =
    "mobility_runtime1b_non_scheduled_passgraph_registration_fixture";
pub const MOBILITY_RUNTIME1B_NAMED_GATE: &str =
    "mobility_runtime1b_explicit_opt_in_passgraph_registration_gate";
pub const MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID: &str =
    "mobility_runtime1b_non_scheduled_composition_node";
/// Closed follow-on gate for real scheduled GPU dispatch (no mobility shader exists yet).
pub const MOBILITY_RUNTIME1B_DISPATCH_GATE: &str = "mobility_runtime1b_dispatch_closed";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime1bPassgraphGate {
    pub explicit_named_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityRuntime1bPassgraphGate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_named_gate_enabled: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime1bPassgraphRegistry {
    pub nodes: Vec<MobilityRuntime1bPassgraphNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityRuntime1bPassgraphNode {
    pub node_id: &'static str,
    pub scheduled: bool,
    pub wgsl_shader_present: bool,
    pub gpu_dispatch_enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1bPassgraphFixtureInput {
    pub gate: MobilityRuntime1bPassgraphGate,
    pub driver: MobilityRuntime1aDriverFixtureInput,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1bPassgraphFixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_simsession_behavior_unchanged: bool,
    pub default_schedule_unchanged: bool,

    pub gpu_passgraph_node_registered: bool,
    pub passgraph_node_on_default_schedule: bool,
    pub non_scheduled_registration_only: bool,
    pub gpu_dispatch_occurred: bool,
    pub wgsl_shader_introduced: bool,
    pub gameplay_facing_path: bool,

    pub runtime1b_dispatch_gate_closed: bool,
    pub delegated_to_runtime1a_driver_fixture: bool,
    pub runtime1a_driver_fixture_id: &'static str,
    pub driver_report: Option<MobilityRuntime1aDriverFixtureReport>,
    pub passgraph_registry: Option<MobilityRuntime1bPassgraphRegistry>,

    pub composition_invocations: u32,
}

pub fn run_mobility_runtime1b_passgraph_fixture(
    input: &MobilityRuntime1bPassgraphFixtureInput,
) -> MobilityRuntime1bPassgraphFixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["runtime1b_default_on_rejected"], false);
    }

    if !input.gate.explicit_named_gate_enabled {
        return disabled_no_op_report(input);
    }

    let mut registry = MobilityRuntime1bPassgraphRegistry::default();
    registry.nodes.push(MobilityRuntime1bPassgraphNode {
        node_id: MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
        scheduled: false,
        wgsl_shader_present: false,
        gpu_dispatch_enabled: false,
    });

    let mut driver_input = input.driver.clone();
    driver_input.session = MobilityRuntime1aDriverFixtureSession::with_explicit_opt_in();

    let driver_report = run_mobility_runtime1a_driver_fixture(&driver_input);
    if !driver_report.admitted {
        return rejected_report(input, driver_report.diagnostics, true);
    }

    admitted_report(input, registry, driver_report)
}

fn shell(
    input: &MobilityRuntime1bPassgraphFixtureInput,
) -> MobilityRuntime1bPassgraphFixtureReport {
    MobilityRuntime1bPassgraphFixtureReport {
        fixture_id: MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID,
        named_gate: MOBILITY_RUNTIME1B_NAMED_GATE,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.explicit_named_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_simsession_behavior_unchanged: !input.gate.explicit_named_gate_enabled,
        default_schedule_unchanged: true,
        gpu_passgraph_node_registered: false,
        passgraph_node_on_default_schedule: false,
        non_scheduled_registration_only: true,
        gpu_dispatch_occurred: false,
        wgsl_shader_introduced: false,
        gameplay_facing_path: false,
        runtime1b_dispatch_gate_closed: true,
        delegated_to_runtime1a_driver_fixture: false,
        runtime1a_driver_fixture_id: MOBILITY_RUNTIME1A_DRIVER_FIXTURE_ID,
        driver_report: None,
        passgraph_registry: None,
        composition_invocations: 0,
    }
}

fn disabled_no_op_report(
    input: &MobilityRuntime1bPassgraphFixtureInput,
) -> MobilityRuntime1bPassgraphFixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report.default_simsession_behavior_unchanged = true;
    report
}

fn rejected_report(
    input: &MobilityRuntime1bPassgraphFixtureInput,
    diagnostics: Vec<&'static str>,
    delegated_to_runtime1a: bool,
) -> MobilityRuntime1bPassgraphFixtureReport {
    let mut report = shell(input);
    report.admitted = false;
    report.diagnostics = diagnostics;
    report.delegated_to_runtime1a_driver_fixture = delegated_to_runtime1a;
    report
}

fn admitted_report(
    input: &MobilityRuntime1bPassgraphFixtureInput,
    registry: MobilityRuntime1bPassgraphRegistry,
    driver_report: MobilityRuntime1aDriverFixtureReport,
) -> MobilityRuntime1bPassgraphFixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.explicit_opt_in = true;
    report.gpu_passgraph_node_registered = true;
    report.non_scheduled_registration_only = registry.nodes.iter().all(|n| !n.scheduled);
    report.delegated_to_runtime1a_driver_fixture = true;
    report.composition_invocations = driver_report.composition_invocations;
    report.driver_report = Some(driver_report);
    report.passgraph_registry = Some(registry);
    report
}

pub type MobilityRuntime1bForbiddenPathRequests = MobilityRuntime1aForbiddenPathRequests;

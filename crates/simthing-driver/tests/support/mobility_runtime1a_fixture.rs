//! MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE: CPU-only default-off `simthing-driver` test/support fixture.
//!
//! Delegates to the green `simthing-spec` RUNTIME-1A production-fixture model. Confined to
//! `tests/support` — not wired into the default lib/session `SimSession` path. No GPU pass-graph,
//! default schedule, or gameplay integration.

use simthing_spec::{
    run_mobility_runtime1a_production_fixture, MobilityRuntime0CompositionInput,
    MobilityRuntime1aForbiddenPathRequests, MobilityRuntime1aProductionFixtureInput,
    MobilityRuntime1aProductionFixtureReport, MobilityRuntime1aSimSessionSurface,
    MOBILITY_RUNTIME1A_ID, MOBILITY_RUNTIME1A_NAMED_GATE,
};

pub const MOBILITY_RUNTIME1A_DRIVER_FIXTURE_ID: &str =
    "mobility_runtime1a_driver_test_support_cpu_fixture";
pub const MOBILITY_RUNTIME1A_DRIVER_NAMED_GATE: &str =
    "mobility_runtime1a_driver_explicit_opt_in_gate";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime1aDriverFixtureGate {
    pub explicit_named_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityRuntime1aDriverFixtureGate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_named_gate_enabled: true,
            enabled_by_default: false,
        }
    }
}

/// Test/support fixture session state. Default-off; does not mutate production `SimSession`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime1aDriverFixtureSession {
    pub gate: MobilityRuntime1aDriverFixtureGate,
    pub fixture_invocations: u32,
}

impl MobilityRuntime1aDriverFixtureSession {
    pub fn default_disabled() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: MobilityRuntime1aDriverFixtureGate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1aDriverFixtureInput {
    pub session: MobilityRuntime1aDriverFixtureSession,
    pub composition: MobilityRuntime0CompositionInput,
    pub forbidden: MobilityRuntime1aForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1aDriverFixtureReport {
    pub driver_fixture_id: &'static str,
    pub driver_named_gate: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub cpu_only: bool,
    pub disabled_no_op: bool,
    pub fixture_invoked: bool,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_simsession_behavior_unchanged: bool,
    pub passgraph_schedule_registered: bool,
    pub gpu_passgraph_registered: bool,
    pub gpu_runtime_hook_present: bool,
    pub gameplay_facing_path: bool,
    pub runtime1b_gate_closed: bool,

    pub delegated_to_spec: bool,
    pub spec_fixture_id: &'static str,
    pub spec_named_gate: &'static str,
    pub spec_report: Option<MobilityRuntime1aProductionFixtureReport>,

    pub composition_invocations: u32,
}

pub fn run_mobility_runtime1a_driver_fixture(
    input: &MobilityRuntime1aDriverFixtureInput,
) -> MobilityRuntime1aDriverFixtureReport {
    if input.session.gate.enabled_by_default {
        return rejected_report(
            input,
            vec!["runtime1a_driver_default_on_rejected"],
            false,
            false,
        );
    }

    let surface = if input.session.gate.explicit_named_gate_enabled {
        MobilityRuntime1aSimSessionSurface::with_explicit_opt_in()
    } else {
        MobilityRuntime1aSimSessionSurface::default_simsession()
    };

    let spec_input = MobilityRuntime1aProductionFixtureInput {
        surface,
        composition: input.composition.clone(),
        forbidden: input.forbidden.clone(),
    };

    let spec_report = run_mobility_runtime1a_production_fixture(&spec_input);
    if !spec_report.admitted {
        return rejected_report(
            input,
            spec_report.diagnostics,
            spec_report.explicit_opt_in,
            true,
        );
    }

    admitted_report(input, spec_report)
}

fn shell(input: &MobilityRuntime1aDriverFixtureInput) -> MobilityRuntime1aDriverFixtureReport {
    MobilityRuntime1aDriverFixtureReport {
        driver_fixture_id: MOBILITY_RUNTIME1A_DRIVER_FIXTURE_ID,
        driver_named_gate: MOBILITY_RUNTIME1A_DRIVER_NAMED_GATE,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.session.gate.explicit_named_gate_enabled,
        default_off: !input.session.gate.enabled_by_default,
        cpu_only: true,
        disabled_no_op: false,
        fixture_invoked: false,
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_simsession_behavior_unchanged: !input.session.gate.explicit_named_gate_enabled,
        passgraph_schedule_registered: false,
        gpu_passgraph_registered: false,
        gpu_runtime_hook_present: false,
        gameplay_facing_path: false,
        runtime1b_gate_closed: true,
        delegated_to_spec: false,
        spec_fixture_id: MOBILITY_RUNTIME1A_ID,
        spec_named_gate: MOBILITY_RUNTIME1A_NAMED_GATE,
        spec_report: None,
        composition_invocations: input.session.fixture_invocations,
    }
}

fn rejected_report(
    input: &MobilityRuntime1aDriverFixtureInput,
    diagnostics: Vec<&'static str>,
    explicit_opt_in: bool,
    delegated_to_spec: bool,
) -> MobilityRuntime1aDriverFixtureReport {
    let mut report = shell(input);
    report.admitted = false;
    report.diagnostics = diagnostics;
    report.explicit_opt_in = explicit_opt_in;
    report.delegated_to_spec = delegated_to_spec;
    report.spec_report = None;
    report
}

fn admitted_report(
    input: &MobilityRuntime1aDriverFixtureInput,
    spec_report: MobilityRuntime1aProductionFixtureReport,
) -> MobilityRuntime1aDriverFixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = spec_report.disabled_no_op;
    report.fixture_invoked = spec_report.fixture_invoked;
    report.default_simsession_behavior_unchanged =
        spec_report.default_simsession_behavior_unchanged;
    report.delegated_to_spec = true;
    report.composition_invocations = spec_report.composition_invocations;
    report.spec_report = Some(spec_report);
    report
}

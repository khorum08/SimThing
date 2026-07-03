//! GPU-EXEC-0 — semantic-free GPU execution readiness fixture tests.

#[path = "support/gpu_exec0_fixture.rs"]
mod gpu_exec0_fixture;

use gpu_exec0_fixture::{
    run_gpu_exec0_fixture, GpuExec0FixtureInput, GpuExec0ForbiddenPathRequests, GpuExec0Gate,
    GpuExec0ParityClassification, GPU_EXEC0_FIXTURE_ID, GPU_EXEC0_NAMED_GATE,
    GPU_EXEC0_PASS_DESCRIPTOR_ID, MOBILITY_RUNTIME1B_DISPATCH_GATE,
};

fn fixture_input() -> GpuExec0FixtureInput {
    GpuExec0FixtureInput::default_probe()
}

fn rejected_with(
    forbidden: GpuExec0ForbiddenPathRequests,
) -> gpu_exec0_fixture::GpuExec0FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_gpu_exec0_fixture(&input)
}

#[test]
fn gpu_exec0_explicit_opt_in_only() {
    let disabled = run_gpu_exec0_fixture(&GpuExec0FixtureInput {
        gate: GpuExec0Gate::default(),
        forbidden: GpuExec0ForbiddenPathRequests::default(),
        probe_values: fixture_input().probe_values,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(!disabled.gpu_dispatch_occurred);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    let rejected = run_gpu_exec0_fixture(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"gpu_exec0_default_on_rejected"));

    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn gpu_exec0_default_disabled_noop() {
    let report = run_gpu_exec0_fixture(&GpuExec0FixtureInput {
        gate: GpuExec0Gate::default(),
        forbidden: GpuExec0ForbiddenPathRequests::default(),
        probe_values: fixture_input().probe_values,
    });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert_eq!(report.cpu_oracle_checksum, 0);
    assert!(!report.gpu_dispatch_occurred);
}

#[test]
fn gpu_exec0_semantic_free_pass_descriptor_only() {
    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.semantic_free_pass_descriptor_only);
    assert_eq!(report.pass_descriptor_id, GPU_EXEC0_PASS_DESCRIPTOR_ID);
    assert_eq!(report.fixture_id, GPU_EXEC0_FIXTURE_ID);
    assert_eq!(report.named_gate, GPU_EXEC0_NAMED_GATE);
}
#[test]
fn gpu_exec0_no_mobility_shader_or_dispatch() {
    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.mobility_shader_present);
    assert!(report.runtime1b_dispatch_gate_closed);
    assert_eq!(
        MOBILITY_RUNTIME1B_DISPATCH_GATE,
        "mobility_runtime1b_dispatch_closed"
    );
}

#[test]
fn gpu_exec0_no_default_schedule() {
    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.default_schedule_registered);
}

#[test]
fn gpu_exec0_no_gameplay_path() {
    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.gameplay_facing_path);
    assert!(!report.default_simsession_lib_path_wired);
    assert!(report.confined_to_driver_test_support);
}

#[test]
fn gpu_exec0_reports_cpu_oracle_checksum() {
    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert_ne!(report.cpu_oracle_checksum, 0);
}
#[test]
fn gpu_exec0_classifies_exact_parity_or_honest_approximation() {
    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(matches!(
        report.parity_classification,
        GpuExec0ParityClassification::ExactParity | GpuExec0ParityClassification::GpuUnavailable
    ));
    if report.parity_classification == GpuExec0ParityClassification::ExactParity {
        assert_eq!(
            report.cpu_oracle_checksum,
            report.gpu_result_checksum.unwrap()
        );
    }
}

#[test]
fn gpu_exec0_preserves_runtime1b_dispatch_closed() {
    let report = run_gpu_exec0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.runtime1b_dispatch_gate_closed);
    assert!(!report.mobility_shader_present);
}

#[test]
fn gpu_exec0_no_default_runtime_cost_when_disabled() {
    let report = run_gpu_exec0_fixture(&GpuExec0FixtureInput {
        gate: GpuExec0Gate::default(),
        forbidden: GpuExec0ForbiddenPathRequests::default(),
        probe_values: fixture_input().probe_values,
    });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert!(!report.gpu_dispatch_occurred);
    assert_eq!(report.cpu_oracle_checksum, 0);
}

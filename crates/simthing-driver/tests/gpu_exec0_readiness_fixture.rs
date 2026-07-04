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

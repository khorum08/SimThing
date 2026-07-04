//! MOBILITY-GPU-KERNEL-7: deterministic multi-dispatch replay soak tests.

#[path = "support/mobility_gpu_kernel7_replay_fixture.rs"]
mod mobility_gpu_kernel7_replay_fixture;

use mobility_gpu_kernel7_replay_fixture::{
    cpu_chain_oracle, mobility_gpu_kernel7_replay_shader_text_has_domain_terms,
    permuted_projected_34k_columns_for_kernel6, projected_34k_columns_for_kernel6,
    run_mobility_gpu_kernel7_fixture, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel7FixtureInput, MobilityGpuKernel7ForbiddenPathRequests,
    MobilityGpuKernel7Gate, MOBILITY_GPU_KERNEL7_FIXTURE_ID, MOBILITY_GPU_KERNEL7_MIN_ITERATIONS,
    MOBILITY_GPU_KERNEL7_NAMED_GATE, MOBILITY_GPU_KERNEL7_NEW_SHADER_TEXT_ADDED,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel7FixtureInput {
    MobilityGpuKernel7FixtureInput::default_replay_soak()
}

fn rejected_with(
    forbidden: MobilityGpuKernel7ForbiddenPathRequests,
) -> mobility_gpu_kernel7_replay_fixture::MobilityGpuKernel7FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel7_fixture(&input)
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel7_replay_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(matches!(
        report.parity_classification,
        MobilityGpuKernel0ParityClassification::ExactParity
            | MobilityGpuKernel0ParityClassification::GpuUnavailable
    ));
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel7_replay_no_gpu_allocator_or_nondeterministic_atomics() {
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

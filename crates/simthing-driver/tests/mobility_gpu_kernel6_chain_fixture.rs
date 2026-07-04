//! MOBILITY-GPU-KERNEL-6: ordered semantic-free mobility GPU kernel chain tests.

#[path = "support/mobility_gpu_kernel6_chain_fixture.rs"]
mod mobility_gpu_kernel6_chain_fixture;

use mobility_gpu_kernel6_chain_fixture::{
    cpu_chain_oracle, mobility_gpu_kernel6_chain_shader_text_has_domain_terms,
    permuted_projected_34k_columns_for_kernel6, projected_34k_columns_for_kernel6,
    run_mobility_gpu_kernel6_fixture, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel6FixtureInput, MobilityGpuKernel6ForbiddenPathRequests,
    MobilityGpuKernel6Gate, MOBILITY_GPU_KERNEL6_CHAIN_ID, MOBILITY_GPU_KERNEL6_FIXTURE_ID,
    MOBILITY_GPU_KERNEL6_NAMED_GATE, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel6FixtureInput {
    MobilityGpuKernel6FixtureInput::default_chain()
}

fn rejected_with(
    forbidden: MobilityGpuKernel6ForbiddenPathRequests,
) -> mobility_gpu_kernel6_chain_fixture::MobilityGpuKernel6FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel6_fixture(&input)
}

#[test]
fn mobility_gpu_kernel6_chain_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(matches!(
        report.parity_classification,
        MobilityGpuKernel0ParityClassification::ExactParity
            | MobilityGpuKernel0ParityClassification::GpuUnavailable
    ));
}

#[test]
fn mobility_gpu_kernel6_chain_outputs_match_cpu_oracle() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(report.outputs_match_cpu_oracle);
    if report.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity {
        assert_eq!(
            report.cpu_chain_checksum,
            report.gpu_chain_checksum.unwrap()
        );
    }
}

#[test]
fn mobility_gpu_kernel6_chain_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

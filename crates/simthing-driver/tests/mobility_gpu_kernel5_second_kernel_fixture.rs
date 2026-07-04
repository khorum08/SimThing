//! MOBILITY-GPU-KERNEL-5: second semantic-free mobility-shaped GPU kernel tests.

#[path = "support/mobility_gpu_kernel5_second_kernel_fixture.rs"]
mod mobility_gpu_kernel5_second_kernel_fixture;

use mobility_gpu_kernel5_second_kernel_fixture::{
    cpu_second_kernel_oracle, mobility_gpu_kernel5_builtin_wgsl_is_semantic_free,
    mobility_gpu_kernel5_shader_text_has_domain_terms, permuted_projected_34k_columns_for_kernel5,
    projected_34k_columns_for_kernel5, run_mobility_gpu_kernel5_fixture,
    MobilityGpuKernel0ParityClassification, MobilityGpuKernel5FixtureInput,
    MobilityGpuKernel5ForbiddenPathRequests, MobilityGpuKernel5Gate,
    MOBILITY_GPU_KERNEL1_FIXTURE_ID, MOBILITY_GPU_KERNEL4_FIXTURE_ID,
    MOBILITY_GPU_KERNEL4_ROW_COUNT, MOBILITY_GPU_KERNEL5_FIXTURE_ID,
    MOBILITY_GPU_KERNEL5_KERNEL_ID, MOBILITY_GPU_KERNEL5_NAMED_GATE,
    MOBILITY_GPU_KERNEL5_NEW_SHADER_TEXT_ADDED,
    MOBILITY_GPU_KERNEL5_RUNTIME1B_DISPATCH_STATUS_RECONCILED,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel5FixtureInput {
    MobilityGpuKernel5FixtureInput::default_second_kernel()
}

fn rejected_with(
    forbidden: MobilityGpuKernel5ForbiddenPathRequests,
) -> mobility_gpu_kernel5_second_kernel_fixture::MobilityGpuKernel5FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel5_fixture(&input)
}

#[test]
fn mobility_gpu_kernel5_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(matches!(
        report.parity_classification,
        MobilityGpuKernel0ParityClassification::ExactParity
            | MobilityGpuKernel0ParityClassification::GpuUnavailable
    ));
    if report.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity {
        assert_eq!(
            report.cpu_oracle_checksum,
            report.gpu_result_checksum.unwrap()
        );
    }
}

#[test]
fn mobility_gpu_kernel5_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

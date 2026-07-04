//! MOBILITY-GPU-KERNEL-0 — semantic-free mobility column-transform kernel tests.

#[path = "support/mobility_gpu_kernel0_fixture.rs"]
mod mobility_gpu_kernel0_fixture;

use mobility_gpu_kernel0_fixture::{
    cpu_column_transform_oracle, mobility_gpu_kernel0_builtin_wgsl_is_semantic_free,
    run_mobility_gpu_kernel0_fixture, MobilityGpuKernel0ColumnProbe,
    MobilityGpuKernel0FixtureInput, MobilityGpuKernel0ForbiddenPathRequests,
    MobilityGpuKernel0Gate, MobilityGpuKernel0ParityClassification,
    MOBILITY_GPU_KERNEL0_FIXTURE_ID, MOBILITY_GPU_KERNEL0_KERNEL_ID,
    MOBILITY_GPU_KERNEL0_NAMED_GATE,
};

fn fixture_input() -> MobilityGpuKernel0FixtureInput {
    MobilityGpuKernel0FixtureInput::default_probe()
}

fn rejected_with(
    forbidden: MobilityGpuKernel0ForbiddenPathRequests,
) -> mobility_gpu_kernel0_fixture::MobilityGpuKernel0FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel0_fixture(&input)
}

#[test]
fn mobility_gpu_kernel0_column_transform_cpu_oracle() {
    let columns = MobilityGpuKernel0ColumnProbe::default_probe();
    let oracle = cpu_column_transform_oracle(&columns);
    assert_eq!(oracle.out_parent, vec![11, 20, 31, 40, 50, 61, 70, 81]);
    assert_eq!(oracle.out_changed, vec![1, 0, 1, 0, 0, 1, 0, 1]);

    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert_ne!(report.cpu_oracle_checksum, 0);
}

#[test]
fn mobility_gpu_kernel0_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
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
fn mobility_gpu_kernel0_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
}

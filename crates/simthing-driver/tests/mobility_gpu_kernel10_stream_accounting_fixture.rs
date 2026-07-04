//! MOBILITY-GPU-KERNEL-10: deterministic stream accounting summary tests.

#[path = "support/mobility_gpu_kernel10_stream_accounting_fixture.rs"]
mod mobility_gpu_kernel10_stream_accounting_fixture;

use mobility_gpu_kernel10_stream_accounting_fixture::{
    compute_stream_accounting, mobility_gpu_kernel10_shader_text_has_domain_terms,
    projected_34k_columns_for_kernel6, projection_checksum_for_columns,
    run_mobility_gpu_kernel10_fixture, run_mobility_gpu_kernel9_fixture,
    stream_cpu_checksum_from_frames, stream_gpu_checksum_from_frames,
    MobilityGpuKernel0ParityClassification, MobilityGpuKernel10FixtureInput,
    MobilityGpuKernel10ForbiddenPathRequests, MobilityGpuKernel10Gate,
    MobilityGpuKernel9FixtureInput, MOBILITY_GPU_KERNEL10_EXPECTED_FRAME_COUNT,
    MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT,
    MOBILITY_GPU_KERNEL10_EXPECTED_REPLAY_DISPATCH_ATTEMPTS,
    MOBILITY_GPU_KERNEL10_EXPECTED_ROW_COUNT_PER_VARIANT,
    MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED,
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANTS_PER_FRAME,
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANT_DISPATCH_ATTEMPTS, MOBILITY_GPU_KERNEL10_FIXTURE_ID,
    MOBILITY_GPU_KERNEL10_NAMED_GATE, MOBILITY_GPU_KERNEL10_NEW_SHADER_TEXT_ADDED,
    MOBILITY_GPU_KERNEL10_USES_WALL_CLOCK, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel10FixtureInput {
    MobilityGpuKernel10FixtureInput::default_stream_accounting()
}

fn kernel9_input() -> MobilityGpuKernel9FixtureInput {
    MobilityGpuKernel9FixtureInput::default_frame_stream()
}

fn rejected_with(
    forbidden: MobilityGpuKernel10ForbiddenPathRequests,
) -> mobility_gpu_kernel10_stream_accounting_fixture::MobilityGpuKernel10FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel10_fixture(&input)
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel10_accounting_total_dispatch_attempts_are_deterministic() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert_eq!(
        report.accounting.total_variant_dispatch_attempts,
        MOBILITY_GPU_KERNEL10_EXPECTED_VARIANT_DISPATCH_ATTEMPTS
    );
    assert_eq!(
        report.accounting.total_replay_dispatch_attempts,
        MOBILITY_GPU_KERNEL10_EXPECTED_REPLAY_DISPATCH_ATTEMPTS
    );
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel10_accounting_total_rows_processed_are_deterministic() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert_eq!(
        report.accounting.total_rows_processed,
        MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED
    );
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel10_accounting_cpu_oracle_rows_match_stream_rows() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert_eq!(
        report.accounting.total_cpu_oracle_rows,
        report.accounting.total_rows_processed
    );
    assert_eq!(
        report.accounting.total_cpu_oracle_rows,
        MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED
    );
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel10_accounting_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(matches!(
        report.accounting.parity_classification,
        MobilityGpuKernel0ParityClassification::ExactParity
            | MobilityGpuKernel0ParityClassification::GpuUnavailable
    ));
    assert_eq!(
        report.accounting.parity_classification,
        report.kernel9_report.parity_classification
    );
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel10_accounting_no_gpu_allocator_or_nondeterministic_atomics() {
    let mut forbidden = MobilityGpuKernel10ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]

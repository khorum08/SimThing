//! MOBILITY-GPU-KERNEL-11: deterministic budget-envelope assertion tests.

#[path = "support/mobility_gpu_kernel11_budget_envelope_fixture.rs"]
mod mobility_gpu_kernel11_budget_envelope_fixture;

use mobility_gpu_kernel11_budget_envelope_fixture::{
    active_stream_budget_envelope, evaluate_stream_budget_envelope,
    fake_over_budget_dispatches_accounting, fake_over_budget_rows_accounting,
    mobility_gpu_kernel11_shader_text_has_domain_terms, run_mobility_gpu_kernel11_fixture,
    stream_cpu_checksum_from_frames, zero_cost_budget_envelope,
    MobilityGpuKernel0ParityClassification, MobilityGpuKernel11FixtureInput,
    MobilityGpuKernel11ForbiddenPathRequests, MobilityGpuKernel11Gate,
    MOBILITY_GPU_KERNEL11_ENVELOPE_FRAME_COUNT, MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
    MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAY_DISPATCH_ATTEMPTS,
    MOBILITY_GPU_KERNEL11_ENVELOPE_ROW_COUNT_PER_VARIANT,
    MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_ROWS_PROCESSED,
    MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANTS_PER_FRAME,
    MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANT_DISPATCH_ATTEMPTS, MOBILITY_GPU_KERNEL11_FIXTURE_ID,
    MOBILITY_GPU_KERNEL11_NAMED_GATE, MOBILITY_GPU_KERNEL11_NEW_SHADER_TEXT_ADDED,
    MOBILITY_GPU_KERNEL11_USES_WALL_CLOCK, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel11FixtureInput {
    MobilityGpuKernel11FixtureInput::default_budget_envelope()
}

fn rejected_with(
    forbidden: MobilityGpuKernel11ForbiddenPathRequests,
) -> mobility_gpu_kernel11_budget_envelope_fixture::MobilityGpuKernel11FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel11_fixture(&input)
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel11_budget_cpu_oracle_rows_match_expected_envelope() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert_eq!(
        report.kernel10_report.accounting.total_cpu_oracle_rows,
        MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_ROWS_PROCESSED
    );
    assert_eq!(
        report.kernel10_report.accounting.total_cpu_oracle_rows,
        report.kernel10_report.accounting.total_rows_processed
    );
    assert!(report.budget_evaluation.within_envelope);
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel11_budget_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(matches!(
        report.kernel10_report.accounting.parity_classification,
        MobilityGpuKernel0ParityClassification::ExactParity
            | MobilityGpuKernel0ParityClassification::GpuUnavailable
    ));
    assert!(report.budget_evaluation.within_envelope);
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel11_budget_failure_diagnostics_are_deterministic() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    let fake_rows = fake_over_budget_rows_accounting(&report.kernel10_report.accounting);
    let fake_dispatches =
        fake_over_budget_dispatches_accounting(&report.kernel10_report.accounting);
    let envelope = active_stream_budget_envelope();
    let rows_eval_a = evaluate_stream_budget_envelope(&fake_rows, &envelope);
    let rows_eval_b = evaluate_stream_budget_envelope(&fake_rows, &envelope);
    let disp_eval_a = evaluate_stream_budget_envelope(&fake_dispatches, &envelope);
    let disp_eval_b = evaluate_stream_budget_envelope(&fake_dispatches, &envelope);
    assert_eq!(rows_eval_a, rows_eval_b);
    assert_eq!(disp_eval_a, disp_eval_b);
    let mut expected_rows = vec![
        "kernel11_budget_rows_over_envelope",
        "kernel11_budget_cpu_oracle_rows_over_envelope",
    ];
    if report.kernel10_report.accounting.parity_classification
        == MobilityGpuKernel0ParityClassification::ExactParity
    {
        expected_rows.push("kernel11_budget_gpu_rows_over_envelope");
    }
    assert_eq!(rows_eval_a.diagnostics, expected_rows);
    assert_eq!(
        disp_eval_a.diagnostics,
        vec![
            "kernel11_budget_variant_dispatch_over_envelope",
            "kernel11_budget_replay_dispatch_over_envelope",
        ]
    );
}

#[test]
fn mobility_gpu_kernel11_budget_no_gpu_allocator_or_nondeterministic_atomics() {
    let mut forbidden = MobilityGpuKernel11ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

//! MOBILITY-GPU-KERNEL-11: deterministic budget-envelope assertion tests.

#[path = "support/mobility_gpu_kernel11_budget_envelope_fixture.rs"]
mod mobility_gpu_kernel11_budget_envelope_fixture;

use mobility_gpu_kernel11_budget_envelope_fixture::{
    active_stream_budget_envelope, evaluate_stream_budget_envelope,
    fake_over_budget_dispatches_accounting, fake_over_budget_rows_accounting,
    mobility_gpu_kernel11_shader_text_has_domain_terms, run_mobility_gpu_kernel11_fixture,
    stream_cpu_checksum_from_frames, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel11FixtureInput, MobilityGpuKernel11ForbiddenPathRequests,
    MobilityGpuKernel11Gate, MOBILITY_GPU_KERNEL11_ENVELOPE_FRAME_COUNT,
    MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
    MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAY_DISPATCH_ATTEMPTS,
    MOBILITY_GPU_KERNEL11_ENVELOPE_ROW_COUNT_PER_VARIANT,
    MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_ROWS_PROCESSED,
    MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANTS_PER_FRAME,
    MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANT_DISPATCH_ATTEMPTS,
    MOBILITY_GPU_KERNEL11_FIXTURE_ID, MOBILITY_GPU_KERNEL11_NAMED_GATE,
    MOBILITY_GPU_KERNEL11_NEW_SHADER_TEXT_ADDED, MOBILITY_GPU_KERNEL11_USES_WALL_CLOCK,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID, zero_cost_budget_envelope,
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

#[test]
fn mobility_gpu_kernel11_budget_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel11_fixture(&MobilityGpuKernel11FixtureInput {
        gate: MobilityGpuKernel11Gate::default(),
        forbidden: MobilityGpuKernel11ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel11_fixture(&default_on).admitted);

    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel11_budget_default_disabled_noop() {
    let report = run_mobility_gpu_kernel11_fixture(&MobilityGpuKernel11FixtureInput {
        gate: MobilityGpuKernel11Gate::default(),
        forbidden: MobilityGpuKernel11ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert!(report.budget_evaluation.within_envelope);
    assert_eq!(report.kernel10_report.accounting.frame_count, 0);
    assert_eq!(report.kernel10_report.accounting.total_rows_processed, 0);
}

#[test]
fn mobility_gpu_kernel11_budget_registration_only_zero_dispatches() {
    let report = run_mobility_gpu_kernel11_fixture(&MobilityGpuKernel11FixtureInput {
        gate: MobilityGpuKernel11Gate::registration_only(),
        forbidden: MobilityGpuKernel11ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
    });
    assert!(report.registration_only_zero_dispatches);
    assert!(report.budget_evaluation.within_envelope);
    assert_eq!(report.kernel10_report.accounting.total_replay_dispatch_attempts, 0);
    assert_eq!(report.kernel10_report.accounting.total_rows_processed, 0);
}

#[test]
fn mobility_gpu_kernel11_budget_uses_registered_node() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    assert_eq!(
        MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
        "mobility_runtime1b_non_scheduled_composition_node"
    );
}

#[test]
fn mobility_gpu_kernel11_budget_reuses_kernel10_accounting() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.reuses_kernel10_accounting);
    assert_eq!(report.kernel10_fixture_id, "mobility_gpu_kernel10_stream_accounting_fixture");
    assert_eq!(
        report.kernel10_accounting_id,
        "mobility_gpu_kernel10_deterministic_stream_accounting_summary"
    );
    assert!(report.budget_evaluation.within_envelope);
}

#[test]
fn mobility_gpu_kernel11_budget_reuses_kernel9_frame_stream() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.reuses_kernel9_frame_stream);
    assert_eq!(
        report.kernel10_report.kernel9_fixture_id,
        "mobility_gpu_kernel9_multi_frame_variant_stream_fixture"
    );
    assert_eq!(
        report.kernel10_report.kernel9_report.frame_count,
        MOBILITY_GPU_KERNEL11_ENVELOPE_FRAME_COUNT
    );
}

#[test]
fn mobility_gpu_kernel11_budget_counts_match_expected_envelope() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    let accounting = &report.kernel10_report.accounting;
    assert_eq!(accounting.frame_count, MOBILITY_GPU_KERNEL11_ENVELOPE_FRAME_COUNT);
    assert_eq!(accounting.variants_per_frame, MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANTS_PER_FRAME);
    assert_eq!(accounting.replays_per_variant, MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT);
    assert!(report.budget_evaluation.within_envelope);
}

#[test]
fn mobility_gpu_kernel11_budget_rows_match_expected_envelope() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert_eq!(
        report.kernel10_report.accounting.row_count_per_variant,
        MOBILITY_GPU_KERNEL11_ENVELOPE_ROW_COUNT_PER_VARIANT
    );
    assert_eq!(
        report.kernel10_report.accounting.total_rows_processed,
        MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_ROWS_PROCESSED
    );
    assert!(report.budget_evaluation.within_envelope);
}

#[test]
fn mobility_gpu_kernel11_budget_dispatch_attempts_match_expected_envelope() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert_eq!(
        report.kernel10_report.accounting.total_variant_dispatch_attempts,
        MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANT_DISPATCH_ATTEMPTS
    );
    assert_eq!(
        report.kernel10_report.accounting.total_replay_dispatch_attempts,
        MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAY_DISPATCH_ATTEMPTS
    );
    assert!(report.budget_evaluation.within_envelope);
}

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

#[test]
fn mobility_gpu_kernel11_budget_gpu_rows_match_or_unavailable() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    match report.kernel10_report.accounting.parity_classification {
        MobilityGpuKernel0ParityClassification::ExactParity => {
            assert_eq!(
                report.kernel10_report.accounting.total_gpu_rows,
                Some(MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_ROWS_PROCESSED)
            );
        }
        MobilityGpuKernel0ParityClassification::GpuUnavailable => {
            assert!(report.kernel10_report.accounting.total_gpu_rows.is_none());
        }
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
            panic!("unexpected GpuExecutionFailed in budget envelope");
        }
    }
    assert!(report.budget_evaluation.within_envelope);
}

#[test]
fn mobility_gpu_kernel11_budget_preserves_kernel10_checksums() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.preserves_kernel10_checksums);
    assert!(report.kernel10_report.stream_checksum_matches_kernel9);
    assert_eq!(
        report.kernel10_report.accounting.aggregate_cpu_stream_checksum,
        stream_cpu_checksum_from_frames(&report.kernel10_report.kernel9_report.frames)
    );
}

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

#[test]
fn mobility_gpu_kernel11_budget_repeated_runs_identical() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.repeated_runs_identical);
    let second = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert_eq!(report.budget_evaluation, second.budget_evaluation);
}

#[test]
fn mobility_gpu_kernel11_budget_order_sensitive() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.order_sensitive);
    let reversed_frames: Vec<_> = report
        .kernel10_report
        .kernel9_report
        .frames
        .iter()
        .rev()
        .cloned()
        .collect();
    assert_ne!(
        stream_cpu_checksum_from_frames(&report.kernel10_report.kernel9_report.frames),
        stream_cpu_checksum_from_frames(&reversed_frames)
    );
}

#[test]
fn mobility_gpu_kernel11_budget_evaluation_does_not_mutate_accounting() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.evaluation_does_not_mutate_accounting);
    let envelope = active_stream_budget_envelope();
    let before = report.kernel10_report.accounting.clone();
    let _ = evaluate_stream_budget_envelope(&before, &envelope);
    assert_eq!(before, report.kernel10_report.accounting);
}

#[test]
fn mobility_gpu_kernel11_budget_rejects_over_budget_fake_rows() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    let fake = fake_over_budget_rows_accounting(&report.kernel10_report.accounting);
    let evaluation = evaluate_stream_budget_envelope(&fake, &active_stream_budget_envelope());
    assert!(!evaluation.within_envelope);
    assert!(evaluation.diagnostics.contains(&"kernel11_budget_rows_over_envelope"));
}

#[test]
fn mobility_gpu_kernel11_budget_rejects_over_budget_fake_dispatches() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    let fake = fake_over_budget_dispatches_accounting(&report.kernel10_report.accounting);
    let evaluation = evaluate_stream_budget_envelope(&fake, &active_stream_budget_envelope());
    assert!(!evaluation.within_envelope);
    assert!(evaluation
        .diagnostics
        .contains(&"kernel11_budget_replay_dispatch_over_envelope"));
}

#[test]
fn mobility_gpu_kernel11_budget_failure_diagnostics_are_deterministic() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    let fake_rows = fake_over_budget_rows_accounting(&report.kernel10_report.accounting);
    let fake_dispatches = fake_over_budget_dispatches_accounting(&report.kernel10_report.accounting);
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
fn mobility_gpu_kernel11_budget_no_wall_clock_or_timing_thresholds() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(!MOBILITY_GPU_KERNEL11_USES_WALL_CLOCK);
    assert!(!report.uses_wall_clock_or_timing_thresholds);
}

#[test]
fn mobility_gpu_kernel11_budget_no_designer_authored_shader_input() {
    assert!(!run_mobility_gpu_kernel11_fixture(&fixture_input()).designer_shader_input_present);
    let mut forbidden = MobilityGpuKernel11ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel11_budget_no_semantic_or_raw_wgsl() {
    assert!(!run_mobility_gpu_kernel11_fixture(&fixture_input()).semantic_or_raw_wgsl_present);
    let mut forbidden = MobilityGpuKernel11ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel11_budget_shader_text_has_no_domain_terms() {
    assert!(!mobility_gpu_kernel11_shader_text_has_domain_terms());
}

#[test]
fn mobility_gpu_kernel11_budget_no_new_shader_text_unless_documented() {
    assert!(!MOBILITY_GPU_KERNEL11_NEW_SHADER_TEXT_ADDED);
}

#[test]
fn mobility_gpu_kernel11_budget_no_default_schedule() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(report.default_schedule_unchanged);
    assert!(!report.default_production_scheduling_wired);
}

#[test]
fn mobility_gpu_kernel11_budget_no_default_simsession_path() {
    assert!(!run_mobility_gpu_kernel11_fixture(&fixture_input()).default_simsession_lib_path_wired);
}

#[test]
fn mobility_gpu_kernel11_budget_no_gameplay_path() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
}

#[test]
fn mobility_gpu_kernel11_budget_no_live_slot_compaction() {
    assert!(!run_mobility_gpu_kernel11_fixture(&fixture_input()).live_slot_compaction);
}

#[test]
fn mobility_gpu_kernel11_budget_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
}

#[test]
fn mobility_gpu_kernel11_budget_no_cpu_planner_urgency_commitment() {
    assert!(!run_mobility_gpu_kernel11_fixture(&fixture_input()).cpu_planner_urgency_commitment);
}

#[test]
fn mobility_gpu_kernel11_budget_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel11_fixture(&fixture_input());
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.closed_ladders_reopened);
}

#[test]
fn mobility_gpu_kernel11_budget_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel11_fixture(&MobilityGpuKernel11FixtureInput {
        gate: MobilityGpuKernel11Gate::default(),
        forbidden: MobilityGpuKernel11ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL11_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL11_NAMED_GATE);
    assert!(report.budget_evaluation.within_envelope);
    assert_eq!(report.envelope, zero_cost_budget_envelope());
}

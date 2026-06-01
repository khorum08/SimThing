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
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANT_DISPATCH_ATTEMPTS,
    MOBILITY_GPU_KERNEL10_FIXTURE_ID, MOBILITY_GPU_KERNEL10_NAMED_GATE,
    MOBILITY_GPU_KERNEL10_NEW_SHADER_TEXT_ADDED, MOBILITY_GPU_KERNEL10_USES_WALL_CLOCK,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
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

#[test]
fn mobility_gpu_kernel10_accounting_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel10_fixture(&MobilityGpuKernel10FixtureInput {
        gate: MobilityGpuKernel10Gate::default(),
        forbidden: MobilityGpuKernel10ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel10_fixture(&default_on).admitted);

    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel10_accounting_default_disabled_noop() {
    let report = run_mobility_gpu_kernel10_fixture(&MobilityGpuKernel10FixtureInput {
        gate: MobilityGpuKernel10Gate::default(),
        forbidden: MobilityGpuKernel10ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.accounting.frame_count, 0);
    assert_eq!(report.accounting.total_replay_dispatch_attempts, 0);
    assert_eq!(report.accounting.total_rows_processed, 0);
}

#[test]
fn mobility_gpu_kernel10_accounting_registration_only_zero_dispatches() {
    let report = run_mobility_gpu_kernel10_fixture(&MobilityGpuKernel10FixtureInput {
        gate: MobilityGpuKernel10Gate::registration_only(),
        forbidden: MobilityGpuKernel10ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT,
    });
    assert!(report.registration_only_zero_dispatches);
    assert_eq!(report.accounting.total_replay_dispatch_attempts, 0);
    assert_eq!(report.accounting.total_variant_dispatch_attempts, 0);
    assert_eq!(report.accounting.frame_count, 0);
}

#[test]
fn mobility_gpu_kernel10_accounting_uses_registered_node() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    assert_eq!(
        MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
        "mobility_runtime1b_non_scheduled_composition_node"
    );
}

#[test]
fn mobility_gpu_kernel10_accounting_reuses_kernel9_frame_stream() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.reuses_kernel9_frame_stream);
    assert_eq!(report.kernel9_fixture_id, "mobility_gpu_kernel9_multi_frame_variant_stream_fixture");
    assert_eq!(report.kernel9_stream_id, "mobility_gpu_kernel9_projection_variant_frame_stream_soak");
    assert_eq!(report.kernel9_report.frame_count, MOBILITY_GPU_KERNEL10_EXPECTED_FRAME_COUNT);
}

#[test]
fn mobility_gpu_kernel10_accounting_reuses_kernel8_variants() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.reuses_kernel8_variants);
    assert_eq!(
        report.kernel8_fixture_id,
        "mobility_gpu_kernel8_varied_input_projection_batch_fixture"
    );
}

#[test]
fn mobility_gpu_kernel10_accounting_reuses_kernel6_chain() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.reuses_kernel6_chain);
    assert_eq!(
        report.kernel6_chain_id,
        "mobility_gpu_kernel6_kernel0_then_kernel5_chain"
    );
}

#[test]
fn mobility_gpu_kernel10_accounting_counts_frames_variants_replays_rows() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    let accounting = &report.accounting;
    assert_eq!(accounting.frame_count, MOBILITY_GPU_KERNEL10_EXPECTED_FRAME_COUNT);
    assert_eq!(accounting.variants_per_frame, MOBILITY_GPU_KERNEL10_EXPECTED_VARIANTS_PER_FRAME);
    assert_eq!(accounting.replays_per_variant, MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT);
    assert_eq!(accounting.row_count_per_variant, MOBILITY_GPU_KERNEL10_EXPECTED_ROW_COUNT_PER_VARIANT);
}

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

#[test]
fn mobility_gpu_kernel10_accounting_total_rows_processed_are_deterministic() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert_eq!(
        report.accounting.total_rows_processed,
        MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED
    );
}

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

#[test]
fn mobility_gpu_kernel10_accounting_gpu_rows_match_or_unavailable() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    match report.accounting.parity_classification {
        MobilityGpuKernel0ParityClassification::ExactParity => {
            assert_eq!(
                report.accounting.total_gpu_rows,
                Some(report.accounting.total_rows_processed)
            );
        }
        MobilityGpuKernel0ParityClassification::GpuUnavailable => {
            assert!(report.accounting.total_gpu_rows.is_none());
        }
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
            panic!("unexpected GpuExecutionFailed in stream accounting");
        }
    }
}

#[test]
fn mobility_gpu_kernel10_accounting_stream_checksum_matches_kernel9() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.stream_checksum_matches_kernel9);
    assert_eq!(
        report.accounting.aggregate_cpu_stream_checksum,
        stream_cpu_checksum_from_frames(&report.kernel9_report.frames)
    );
    assert_eq!(
        report.accounting.aggregate_gpu_stream_checksum,
        stream_gpu_checksum_from_frames(&report.kernel9_report.frames)
    );
}

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

#[test]
fn mobility_gpu_kernel10_accounting_repeated_runs_identical() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.repeated_runs_identical);
    let second = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert_eq!(report.accounting, second.accounting);
}

#[test]
fn mobility_gpu_kernel10_accounting_order_sensitive() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.order_sensitive);
    let reversed_frames: Vec<_> = report
        .kernel9_report
        .frames
        .iter()
        .rev()
        .cloned()
        .collect();
    assert_ne!(
        stream_cpu_checksum_from_frames(&report.kernel9_report.frames),
        stream_cpu_checksum_from_frames(&reversed_frames)
    );
}

#[test]
fn mobility_gpu_kernel10_accounting_does_not_mutate_source_projection() {
    let before = projected_34k_columns_for_kernel6();
    let before_checksum = projection_checksum_for_columns(&before);
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    let after = projected_34k_columns_for_kernel6();
    assert_eq!(before, after);
    assert_eq!(before_checksum, projection_checksum_for_columns(&after));
    assert!(report.source_projection_unchanged);
}

#[test]
fn mobility_gpu_kernel10_accounting_does_not_mutate_kernel9_reports() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.kernel9_reports_unchanged_by_accounting);
    let standalone = run_mobility_gpu_kernel9_fixture(&kernel9_input());
    assert_eq!(report.kernel9_report, standalone);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_wall_clock_or_timing_thresholds() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(!MOBILITY_GPU_KERNEL10_USES_WALL_CLOCK);
    assert!(!report.uses_wall_clock_or_timing_thresholds);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_designer_authored_shader_input() {
    assert!(!run_mobility_gpu_kernel10_fixture(&fixture_input()).designer_shader_input_present);
    let mut forbidden = MobilityGpuKernel10ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel10_accounting_no_semantic_or_raw_wgsl() {
    assert!(!run_mobility_gpu_kernel10_fixture(&fixture_input()).semantic_or_raw_wgsl_present);
    let mut forbidden = MobilityGpuKernel10ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel10_accounting_shader_text_has_no_domain_terms() {
    assert!(!mobility_gpu_kernel10_shader_text_has_domain_terms());
}

#[test]
fn mobility_gpu_kernel10_accounting_no_new_shader_text_unless_documented() {
    assert!(!MOBILITY_GPU_KERNEL10_NEW_SHADER_TEXT_ADDED);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_default_schedule() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(report.default_schedule_unchanged);
    assert!(!report.default_production_scheduling_wired);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_default_simsession_path() {
    assert!(!run_mobility_gpu_kernel10_fixture(&fixture_input()).default_simsession_lib_path_wired);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_gameplay_path() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_live_slot_compaction() {
    assert!(!run_mobility_gpu_kernel10_fixture(&fixture_input()).live_slot_compaction);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_cpu_planner_urgency_commitment() {
    assert!(!run_mobility_gpu_kernel10_fixture(&fixture_input()).cpu_planner_urgency_commitment);
}

#[test]
fn mobility_gpu_kernel10_accounting_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.closed_ladders_reopened);
}

#[test]
fn mobility_gpu_kernel10_accounting_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel10_fixture(&MobilityGpuKernel10FixtureInput {
        gate: MobilityGpuKernel10Gate::default(),
        forbidden: MobilityGpuKernel10ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL10_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL10_NAMED_GATE);
    assert_eq!(report.accounting.total_rows_processed, 0);
}

#[test]
fn mobility_gpu_kernel10_accounting_compute_matches_kernel9_report() {
    let kernel9 = run_mobility_gpu_kernel9_fixture(&kernel9_input());
    let report = run_mobility_gpu_kernel10_fixture(&fixture_input());
    assert_eq!(compute_stream_accounting(&kernel9), report.accounting);
}

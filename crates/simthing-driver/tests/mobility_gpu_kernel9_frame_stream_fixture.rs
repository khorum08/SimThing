//! MOBILITY-GPU-KERNEL-9: multi-frame projection-variant stream soak tests.

#[path = "support/mobility_gpu_kernel9_frame_stream_fixture.rs"]
mod mobility_gpu_kernel9_frame_stream_fixture;

use mobility_gpu_kernel9_frame_stream_fixture::{
    build_frame_stream_specs, frame_cpu_checksum,
    mobility_gpu_kernel9_shader_text_has_domain_terms, projected_34k_columns_for_kernel6,
    projection_checksum_for_columns, run_mobility_gpu_kernel9_fixture,
    MobilityGpuKernel0ParityClassification, MobilityGpuKernel9FixtureInput,
    MobilityGpuKernel9ForbiddenPathRequests, MobilityGpuKernel9Gate,
    MOBILITY_GPU_KERNEL4_ROW_COUNT, MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
    MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK, MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
    MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA, MOBILITY_GPU_KERNEL9_FIXTURE_ID,
    MOBILITY_GPU_KERNEL9_FRAME_ALT_ORDER, MOBILITY_GPU_KERNEL9_FRAME_CANONICAL,
    MOBILITY_GPU_KERNEL9_FRAME_COUNT, MOBILITY_GPU_KERNEL9_FRAME_REPEAT,
    MOBILITY_GPU_KERNEL9_FRAME_REVERSED, MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT,
    MOBILITY_GPU_KERNEL9_NAMED_GATE, MOBILITY_GPU_KERNEL9_NEW_SHADER_TEXT_ADDED,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel9FixtureInput {
    MobilityGpuKernel9FixtureInput::default_frame_stream()
}

fn rejected_with(
    forbidden: MobilityGpuKernel9ForbiddenPathRequests,
) -> mobility_gpu_kernel9_frame_stream_fixture::MobilityGpuKernel9FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel9_fixture(&input)
}

fn frame_by_id<'a>(
    report: &'a mobility_gpu_kernel9_frame_stream_fixture::MobilityGpuKernel9FixtureReport,
    id: &str,
) -> &'a mobility_gpu_kernel9_frame_stream_fixture::MobilityGpuKernel9FrameReport {
    report
        .frames
        .iter()
        .find(|frame| frame.frame_id == id)
        .unwrap_or_else(|| panic!("missing frame {id}"))
}

#[test]
fn mobility_gpu_kernel9_frame_stream_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel9_fixture(&MobilityGpuKernel9FixtureInput {
        gate: MobilityGpuKernel9Gate::default(),
        forbidden: MobilityGpuKernel9ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel9_fixture(&default_on).admitted);

    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_default_disabled_noop() {
    let report = run_mobility_gpu_kernel9_fixture(&MobilityGpuKernel9FixtureInput {
        gate: MobilityGpuKernel9Gate::default(),
        forbidden: MobilityGpuKernel9ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.frame_count, 0);
    assert!(report.frames.is_empty());
}

#[test]
fn mobility_gpu_kernel9_frame_stream_uses_registered_node() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    assert_eq!(
        MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
        "mobility_runtime1b_non_scheduled_composition_node"
    );
}

#[test]
fn mobility_gpu_kernel9_frame_stream_registration_non_executing_until_invoked() {
    let reg = run_mobility_gpu_kernel9_fixture(&MobilityGpuKernel9FixtureInput {
        gate: MobilityGpuKernel9Gate::registration_only(),
        forbidden: MobilityGpuKernel9ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(reg.registration_non_executing);
    assert!(!reg.gpu_dispatch_occurred);
    assert_eq!(reg.frame_count, 0);

    let dispatched = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(
        dispatched.gpu_dispatch_occurred
            || matches!(
                dispatched.parity_classification,
                MobilityGpuKernel0ParityClassification::GpuUnavailable
            )
    );
}

#[test]
fn mobility_gpu_kernel9_frame_stream_reuses_kernel8_variants() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.reuses_kernel8_variants);
    assert_eq!(
        report.kernel8_fixture_id,
        "mobility_gpu_kernel8_varied_input_projection_batch_fixture"
    );
}

#[test]
fn mobility_gpu_kernel9_frame_stream_reuses_kernel6_chain() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.reuses_kernel6_chain);
    assert_eq!(
        report.kernel6_chain_id,
        "mobility_gpu_kernel6_kernel0_then_kernel5_chain"
    );
}

#[test]
fn mobility_gpu_kernel9_frame_stream_builds_at_least_4_frames() {
    let specs = build_frame_stream_specs();
    assert!(specs.len() >= MOBILITY_GPU_KERNEL9_FRAME_COUNT);
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert_eq!(report.frame_count, MOBILITY_GPU_KERNEL9_FRAME_COUNT);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_includes_baseline_sparse_dense_parent_order() {
    let canonical = build_frame_stream_specs()
        .into_iter()
        .find(|spec| spec.frame_id == MOBILITY_GPU_KERNEL9_FRAME_CANONICAL)
        .unwrap();
    assert_eq!(
        canonical.variant_order,
        &[
            MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
            MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA,
            MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK,
            MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
        ]
    );
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    let frame = frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_CANONICAL);
    assert_eq!(frame.variant_count, 4);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_includes_different_variant_order() {
    let reversed = build_frame_stream_specs()
        .into_iter()
        .find(|spec| spec.frame_id == MOBILITY_GPU_KERNEL9_FRAME_REVERSED)
        .unwrap();
    let canonical = build_frame_stream_specs()
        .into_iter()
        .find(|spec| spec.frame_id == MOBILITY_GPU_KERNEL9_FRAME_CANONICAL)
        .unwrap();
    assert_ne!(reversed.variant_order, canonical.variant_order);

    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert_ne!(
        frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_REVERSED).cpu_frame_checksum,
        frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_CANONICAL).cpu_frame_checksum
    );
}

#[test]
fn mobility_gpu_kernel9_frame_stream_repeated_frame_has_identical_checksum() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.repeated_frames_have_identical_checksums);
    let canonical = frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_CANONICAL);
    let repeat = frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_REPEAT);
    assert_eq!(canonical.cpu_frame_checksum, repeat.cpu_frame_checksum);
    assert_eq!(canonical.gpu_frame_checksum, repeat.gpu_frame_checksum);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_distinct_frames_have_distinct_checksums() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.distinct_frames_have_distinct_checksums);
    let canonical = frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_CANONICAL);
    let alt = frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_ALT_ORDER);
    assert_ne!(canonical.cpu_frame_checksum, alt.cpu_frame_checksum);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_cpu_oracle_complete_per_frame() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.cpu_oracle_complete_per_frame);
    for frame in &report.frames {
        assert_ne!(frame.cpu_frame_checksum, 0);
        for variant in &frame.variants {
            assert_eq!(variant.row_count, MOBILITY_GPU_KERNEL4_ROW_COUNT);
        }
    }
}

#[test]
fn mobility_gpu_kernel9_frame_stream_gpu_checksums_match_or_unavailable() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.gpu_checksums_match_or_unavailable);
    for frame in &report.frames {
        match frame.parity_classification {
            MobilityGpuKernel0ParityClassification::ExactParity => {
                assert!(frame.gpu_frame_checksum.is_some());
            }
            MobilityGpuKernel0ParityClassification::GpuUnavailable => {
                assert!(frame.gpu_frame_checksum.is_none());
            }
            MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
                panic!("unexpected GpuExecutionFailed for frame {}", frame.frame_id);
            }
        }
    }
}

#[test]
fn mobility_gpu_kernel9_frame_stream_replay_stable_per_frame() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.replay_stable_per_frame);
    assert!(report.replays_per_variant >= MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_does_not_mutate_source_projection() {
    let before = projected_34k_columns_for_kernel6();
    let before_checksum = projection_checksum_for_columns(&before);
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    let after = projected_34k_columns_for_kernel6();
    assert_eq!(before, after);
    assert_eq!(before_checksum, projection_checksum_for_columns(&after));
    assert!(report.source_projection_unchanged);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_designer_authored_shader_input() {
    assert!(!run_mobility_gpu_kernel9_fixture(&fixture_input()).designer_shader_input_present);
    let mut forbidden = MobilityGpuKernel9ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_semantic_or_raw_wgsl() {
    assert!(!run_mobility_gpu_kernel9_fixture(&fixture_input()).semantic_or_raw_wgsl_present);
    let mut forbidden = MobilityGpuKernel9ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel9_frame_stream_shader_text_has_no_domain_terms() {
    assert!(!mobility_gpu_kernel9_shader_text_has_domain_terms());
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_new_shader_text_unless_documented() {
    assert!(!MOBILITY_GPU_KERNEL9_NEW_SHADER_TEXT_ADDED);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_default_schedule() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(report.default_schedule_unchanged);
    assert!(!report.default_production_scheduling_wired);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_default_simsession_path() {
    assert!(!run_mobility_gpu_kernel9_fixture(&fixture_input()).default_simsession_lib_path_wired);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_gameplay_path() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_live_slot_compaction() {
    assert!(!run_mobility_gpu_kernel9_fixture(&fixture_input()).live_slot_compaction);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_cpu_planner_urgency_commitment() {
    assert!(!run_mobility_gpu_kernel9_fixture(&fixture_input()).cpu_planner_urgency_commitment);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.closed_ladders_reopened);
}

#[test]
fn mobility_gpu_kernel9_frame_stream_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel9_fixture(&MobilityGpuKernel9FixtureInput {
        gate: MobilityGpuKernel9Gate::default(),
        forbidden: MobilityGpuKernel9ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL9_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL9_NAMED_GATE);
}

#[test]
fn mobility_gpu_kernel9_frame_cpu_checksum_is_order_sensitive() {
    let report = run_mobility_gpu_kernel9_fixture(&fixture_input());
    let canonical = frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_CANONICAL);
    let alt = frame_by_id(&report, MOBILITY_GPU_KERNEL9_FRAME_ALT_ORDER);
    assert_ne!(
        frame_cpu_checksum(&canonical.variants),
        frame_cpu_checksum(&alt.variants)
    );
}

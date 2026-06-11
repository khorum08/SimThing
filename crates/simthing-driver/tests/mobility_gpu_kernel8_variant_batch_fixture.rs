//! MOBILITY-GPU-KERNEL-8: varied-input projection-batch replay soak tests.

#[path = "support/mobility_gpu_kernel8_variant_batch_fixture.rs"]
mod mobility_gpu_kernel8_variant_batch_fixture;

use mobility_gpu_kernel8_variant_batch_fixture::{
    build_projection_variants, dense_bulk_variant,
    mobility_gpu_kernel8_shader_text_has_domain_terms, parent_key_offset_variant,
    projected_34k_columns_for_kernel6, projection_checksum_for_columns,
    run_mobility_gpu_kernel8_fixture, sparse_delta_variant, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel8FixtureInput, MobilityGpuKernel8ForbiddenPathRequests,
    MobilityGpuKernel8Gate, MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END,
    MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START, MOBILITY_GPU_KERNEL4_ROW_COUNT,
    MOBILITY_GPU_KERNEL4_SPARSE_STRIDE, MOBILITY_GPU_KERNEL8_FIXTURE_ID,
    MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT, MOBILITY_GPU_KERNEL8_NAMED_GATE,
    MOBILITY_GPU_KERNEL8_NEW_SHADER_TEXT_ADDED, MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
    MOBILITY_GPU_KERNEL8_VARIANT_COUNT, MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK,
    MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET, MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel8FixtureInput {
    MobilityGpuKernel8FixtureInput::default_variant_batch()
}

fn rejected_with(
    forbidden: MobilityGpuKernel8ForbiddenPathRequests,
) -> mobility_gpu_kernel8_variant_batch_fixture::MobilityGpuKernel8FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel8_fixture(&input)
}

fn variant_by_id<'a>(
    report: &'a mobility_gpu_kernel8_variant_batch_fixture::MobilityGpuKernel8FixtureReport,
    id: &str,
) -> &'a mobility_gpu_kernel8_variant_batch_fixture::MobilityGpuKernel8VariantReport {
    report
        .variants
        .iter()
        .find(|variant| variant.variant_id == id)
        .unwrap_or_else(|| panic!("missing variant {id}"))
}

#[test]
fn mobility_gpu_kernel8_variant_batch_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel8_fixture(&MobilityGpuKernel8FixtureInput {
        gate: MobilityGpuKernel8Gate::default(),
        forbidden: MobilityGpuKernel8ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel8_fixture(&default_on).admitted);
}

#[test]
fn mobility_gpu_kernel8_variant_batch_default_disabled_noop() {
    let report = run_mobility_gpu_kernel8_fixture(&MobilityGpuKernel8FixtureInput {
        gate: MobilityGpuKernel8Gate::default(),
        forbidden: MobilityGpuKernel8ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.variant_count, 0);
    assert!(report.variants.is_empty());
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_uses_registered_node() {
    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    assert_eq!(
        MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
        "mobility_runtime1b_non_scheduled_composition_node"
    );
}

#[test]
fn mobility_gpu_kernel8_variant_batch_registration_non_executing_until_invoked() {
    let reg = run_mobility_gpu_kernel8_fixture(&MobilityGpuKernel8FixtureInput {
        gate: MobilityGpuKernel8Gate::registration_only(),
        forbidden: MobilityGpuKernel8ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(reg.registration_non_executing);
    assert!(!reg.gpu_dispatch_occurred);
    assert_eq!(reg.variant_count, 0);
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_reuses_kernel6_chain() {
    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    assert!(report.reuses_kernel6_chain);
    assert_eq!(
        report.kernel6_chain_id,
        "mobility_gpu_kernel6_kernel0_then_kernel5_chain"
    );
}

#[test]
fn mobility_gpu_kernel8_variant_batch_builds_at_least_4_variants() {
    let baseline = projected_34k_columns_for_kernel6();
    let variants = build_projection_variants(&baseline);
    assert!(variants.len() >= MOBILITY_GPU_KERNEL8_VARIANT_COUNT);
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_includes_baseline_34k_projection() {
    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    let baseline = variant_by_id(&report, MOBILITY_GPU_KERNEL8_VARIANT_BASELINE);
    assert_eq!(baseline.row_count, MOBILITY_GPU_KERNEL4_ROW_COUNT);
    assert_eq!(
        baseline.projection_checksum,
        projection_checksum_for_columns(&projected_34k_columns_for_kernel6())
    );
}

#[test]
fn mobility_gpu_kernel8_variant_batch_includes_sparse_delta_variant() {
    let baseline = projected_34k_columns_for_kernel6();
    let sparse = sparse_delta_variant(&baseline);
    assert_ne!(sparse.move_mask, baseline.move_mask);
    for i in (0..sparse.move_mask.len()).step_by(MOBILITY_GPU_KERNEL4_SPARSE_STRIDE) {
        assert_ne!(sparse.move_mask[i], baseline.move_mask[i]);
    }
}

#[test]
fn mobility_gpu_kernel8_variant_batch_includes_dense_bulk_variant() {
    let baseline = projected_34k_columns_for_kernel6();
    let dense = dense_bulk_variant(&baseline);
    for i in MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START..MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END {
        assert_eq!(dense.move_mask[i], 1);
    }
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_cpu_oracle_complete_per_variant() {
    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    assert!(report.cpu_oracle_complete_per_variant);
    for variant in &report.variants {
        assert_ne!(variant.cpu_chain_checksum, 0);
        assert_eq!(variant.row_count, MOBILITY_GPU_KERNEL4_ROW_COUNT);
    }
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_gpu_checksums_match_or_unavailable() {
    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    assert!(report.gpu_checksums_match_or_unavailable);
    for variant in &report.variants {
        match variant.parity_classification {
            MobilityGpuKernel0ParityClassification::ExactParity => {
                assert!(variant.gpu_chain_checksum.is_some());
            }
            MobilityGpuKernel0ParityClassification::GpuUnavailable => {
                assert!(variant.gpu_chain_checksum.is_none());
            }
            MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
                panic!(
                    "unexpected GpuExecutionFailed for variant {}",
                    variant.variant_id
                );
            }
        }
    }
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_replay_stable_per_variant() {
    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    assert!(report.replay_stable_per_variant);
    assert!(report.replays_per_variant >= MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT);
    for variant in &report.variants {
        assert!(variant.replays.len() >= MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT);
    }
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_distinct_variants_have_distinct_checksums() {
    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    assert!(report.distinct_variants_have_distinct_checksums);

    let parent = variant_by_id(&report, MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET);
    let baseline = variant_by_id(&report, MOBILITY_GPU_KERNEL8_VARIANT_BASELINE);
    assert_ne!(parent.projection_checksum, baseline.projection_checksum);
    assert_ne!(parent.cpu_chain_checksum, baseline.cpu_chain_checksum);
}

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_does_not_mutate_source_projection() {
    let before = projected_34k_columns_for_kernel6();
    let before_checksum = projection_checksum_for_columns(&before);
    let _report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    let after = projected_34k_columns_for_kernel6();
    assert_eq!(before, after);
    assert_eq!(before_checksum, projection_checksum_for_columns(&after));

    let report = run_mobility_gpu_kernel8_fixture(&fixture_input());
    assert!(report.source_projection_unchanged);
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_designer_authored_shader_input() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_semantic_or_raw_wgsl() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel8_variant_batch_shader_text_has_no_domain_terms() {
    assert!(!mobility_gpu_kernel8_shader_text_has_domain_terms());
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_new_shader_text_unless_documented() {
    assert!(!MOBILITY_GPU_KERNEL8_NEW_SHADER_TEXT_ADDED);
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_default_schedule() {
    let disabled = run_mobility_gpu_kernel8_fixture(&MobilityGpuKernel8FixtureInput {
        gate: MobilityGpuKernel8Gate::default(),
        forbidden: MobilityGpuKernel8ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(disabled.default_schedule_unchanged);
    assert!(!disabled.default_production_scheduling_wired);
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_default_simsession_path() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.default_simsession_path = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"default_simsession_path"));
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_gameplay_path() {
    let disabled = run_mobility_gpu_kernel8_fixture(&MobilityGpuKernel8FixtureInput {
        gate: MobilityGpuKernel8Gate::default(),
        forbidden: MobilityGpuKernel8ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(!disabled.gameplay_facing_path);
    assert!(disabled.confined_to_driver_test_support);
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_live_slot_compaction() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.live_slot_compaction = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"live_slot_compaction"));
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_gpu_allocator_or_nondeterministic_atomics() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_cpu_planner_urgency_commitment() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn mobility_gpu_kernel8_variant_batch_preserves_closed_ladder_posture() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.closed_ladder_reopen = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"closed_ladder_reopen"));
}

#[test]
fn mobility_gpu_kernel8_variant_batch_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel8_fixture(&MobilityGpuKernel8FixtureInput {
        gate: MobilityGpuKernel8Gate::default(),
        forbidden: MobilityGpuKernel8ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL8_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL8_NAMED_GATE);
}

#[test]
fn mobility_gpu_kernel8_parent_offset_variant_differs_from_baseline() {
    let baseline = projected_34k_columns_for_kernel6();
    let offset = parent_key_offset_variant(&baseline);
    assert_eq!(offset.entity_id, baseline.entity_id);
    assert_ne!(offset.src_parent, baseline.src_parent);
}

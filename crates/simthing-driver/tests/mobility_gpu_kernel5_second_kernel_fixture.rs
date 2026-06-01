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
fn mobility_gpu_kernel5_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel5_fixture(&MobilityGpuKernel5FixtureInput {
        gate: MobilityGpuKernel5Gate::default(),
        forbidden: MobilityGpuKernel5ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel5_fixture(&default_on).admitted);

    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel5_default_disabled_noop() {
    let report = run_mobility_gpu_kernel5_fixture(&MobilityGpuKernel5FixtureInput {
        gate: MobilityGpuKernel5Gate::default(),
        forbidden: MobilityGpuKernel5ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.row_count, 0);
    assert_eq!(report.cpu_oracle_checksum, 0);
    assert_eq!(report.projection_checksum, 0);
}

#[test]
fn mobility_gpu_kernel5_uses_registered_node() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    assert_eq!(report.kernel4_fixture_id, MOBILITY_GPU_KERNEL4_FIXTURE_ID);
    assert_eq!(report.kernel1_fixture_id, MOBILITY_GPU_KERNEL1_FIXTURE_ID);
    assert_eq!(MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID, "mobility_runtime1b_non_scheduled_composition_node");
}

#[test]
fn mobility_gpu_kernel5_registration_non_executing_until_invoked() {
    let reg = run_mobility_gpu_kernel5_fixture(&MobilityGpuKernel5FixtureInput {
        gate: MobilityGpuKernel5Gate::registration_only(),
        forbidden: MobilityGpuKernel5ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(reg.admitted);
    assert!(reg.registration_non_executing);
    assert!(reg.uses_registered_node);
    assert!(!reg.gpu_dispatch_occurred);
    assert_eq!(reg.row_count, 0);

    let dispatched = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!dispatched.registration_non_executing);
}

#[test]
fn mobility_gpu_kernel5_reuses_kernel4_34k_projection() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(report.reused_kernel4_projection);
    assert_eq!(report.row_count, MOBILITY_GPU_KERNEL4_ROW_COUNT);
    assert_ne!(report.projection_checksum, 0);
}

#[test]
fn mobility_gpu_kernel5_builtin_semantic_free_kernel_only() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(report.builtin_semantic_free_kernel_only);
    assert_eq!(report.kernel_id, MOBILITY_GPU_KERNEL5_KERNEL_ID);
    assert!(mobility_gpu_kernel5_builtin_wgsl_is_semantic_free());
}

#[test]
fn mobility_gpu_kernel5_cpu_oracle_complete() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(report.cpu_oracle_complete);
    assert_ne!(report.cpu_oracle_checksum, 0);
    let columns = projected_34k_columns_for_kernel5();
    let oracle = cpu_second_kernel_oracle(&columns);
    assert_eq!(oracle.out_digest.len(), MOBILITY_GPU_KERNEL4_ROW_COUNT);
    assert_eq!(oracle.out_weight.len(), MOBILITY_GPU_KERNEL4_ROW_COUNT);
    assert!(oracle.out_weight.iter().any(|&weight| weight == 17));
    assert!(oracle.out_weight.iter().any(|&weight| weight == 0));
}

#[test]
fn mobility_gpu_kernel5_reports_gpu_checksum_or_unavailable() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    match report.parity_classification {
        MobilityGpuKernel0ParityClassification::ExactParity => {
            assert!(report.gpu_dispatch_occurred);
            assert!(report.gpu_result_checksum.is_some());
        }
        MobilityGpuKernel0ParityClassification::GpuUnavailable => {
            assert!(!report.gpu_dispatch_occurred);
            assert!(report.gpu_result_checksum.is_none());
        }
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
            panic!("unexpected GpuExecutionFailed: {:?}", report);
        }
    }
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
        assert_eq!(report.cpu_oracle_checksum, report.gpu_result_checksum.unwrap());
    }
}

#[test]
fn mobility_gpu_kernel5_stable_under_input_permutation() {
    let baseline = projected_34k_columns_for_kernel5();
    let permuted = permuted_projected_34k_columns_for_kernel5();
    assert_eq!(baseline.entity_id, permuted.entity_id);
    assert_eq!(baseline.src_parent, permuted.src_parent);
    assert_eq!(baseline.dst_parent, permuted.dst_parent);
    assert_eq!(baseline.move_mask, permuted.move_mask);
    assert_eq!(
        cpu_second_kernel_oracle(&baseline),
        cpu_second_kernel_oracle(&permuted)
    );
}

#[test]
fn mobility_gpu_kernel5_no_designer_authored_shader_input() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.designer_shader_input_present);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel5_no_semantic_or_raw_wgsl() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.semantic_or_raw_wgsl_present);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel5_shader_text_has_no_domain_terms() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.shader_text_has_domain_terms);
    assert!(!mobility_gpu_kernel5_shader_text_has_domain_terms());
    assert!(MOBILITY_GPU_KERNEL5_NEW_SHADER_TEXT_ADDED);
}

#[test]
fn mobility_gpu_kernel5_no_default_schedule() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(report.default_schedule_unchanged);
    assert!(!report.default_production_scheduling_wired);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.default_schedule = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"default_schedule"));
}

#[test]
fn mobility_gpu_kernel5_no_default_simsession_path() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.default_simsession_lib_path_wired);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.default_simsession_path = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"default_simsession_path"));
}

#[test]
fn mobility_gpu_kernel5_no_gameplay_path() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.gameplay_path = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"gameplay_path"));
}

#[test]
fn mobility_gpu_kernel5_no_live_slot_compaction() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.live_slot_compaction);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.live_slot_compaction = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"live_slot_compaction"));
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

#[test]
fn mobility_gpu_kernel5_no_cpu_planner_urgency_commitment() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.cpu_planner_urgency_commitment);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn mobility_gpu_kernel5_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.closed_ladders_reopened);
    let mut forbidden = MobilityGpuKernel5ForbiddenPathRequests::default();
    forbidden.closed_ladder_reopen = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"closed_ladder_reopen"));
}

#[test]
fn mobility_gpu_kernel5_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel5_fixture(&MobilityGpuKernel5FixtureInput {
        gate: MobilityGpuKernel5Gate::default(),
        forbidden: MobilityGpuKernel5ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL5_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL5_NAMED_GATE);
    assert_eq!(report.row_count, 0);
}

#[test]
fn mobility_gpu_kernel5_reconciles_runtime1b_dispatch_status() {
    let report = run_mobility_gpu_kernel5_fixture(&fixture_input());
    assert!(report.runtime1b_dispatch_status_reconciled);
    assert!(MOBILITY_GPU_KERNEL5_RUNTIME1B_DISPATCH_STATUS_RECONCILED);
}

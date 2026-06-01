//! MOBILITY-GPU-KERNEL-6: ordered semantic-free mobility GPU kernel chain tests.

#[path = "support/mobility_gpu_kernel6_chain_fixture.rs"]
mod mobility_gpu_kernel6_chain_fixture;

use mobility_gpu_kernel6_chain_fixture::{
    cpu_chain_oracle, mobility_gpu_kernel6_chain_shader_text_has_domain_terms,
    permuted_projected_34k_columns_for_kernel6, projected_34k_columns_for_kernel6,
    run_mobility_gpu_kernel6_fixture, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel6FixtureInput, MobilityGpuKernel6ForbiddenPathRequests, MobilityGpuKernel6Gate,
    MOBILITY_GPU_KERNEL6_CHAIN_ID, MOBILITY_GPU_KERNEL6_FIXTURE_ID,
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
fn mobility_gpu_kernel6_chain_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel6_fixture(&MobilityGpuKernel6FixtureInput {
        gate: MobilityGpuKernel6Gate::default(),
        forbidden: MobilityGpuKernel6ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
        columns_override: None,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel6_fixture(&default_on).admitted);

    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel6_chain_default_disabled_noop() {
    let report = run_mobility_gpu_kernel6_fixture(&MobilityGpuKernel6FixtureInput {
        gate: MobilityGpuKernel6Gate::default(),
        forbidden: MobilityGpuKernel6ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
        columns_override: None,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.row_count, 0);
    assert_eq!(report.cpu_chain_checksum, 0);
    assert_eq!(report.projection_checksum, 0);
}

#[test]
fn mobility_gpu_kernel6_chain_uses_registered_node() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    assert_eq!(MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID, "mobility_runtime1b_non_scheduled_composition_node");
    assert_eq!(report.chain_id, MOBILITY_GPU_KERNEL6_CHAIN_ID);
}

#[test]
fn mobility_gpu_kernel6_chain_registration_non_executing_until_invoked() {
    let reg = run_mobility_gpu_kernel6_fixture(&MobilityGpuKernel6FixtureInput {
        gate: MobilityGpuKernel6Gate::registration_only(),
        forbidden: MobilityGpuKernel6ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
        columns_override: None,
    });
    assert!(reg.admitted);
    assert!(reg.registration_non_executing);
    assert!(!reg.gpu_dispatch_occurred);
    assert_eq!(reg.row_count, 0);

    let dispatched = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!dispatched.registration_non_executing);
}

#[test]
fn mobility_gpu_kernel6_chain_reuses_kernel4_34k_projection() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(report.reused_kernel4_projection);
    assert_eq!(report.row_count, 34_000);
    assert_ne!(report.projection_checksum, 0);
}

#[test]
fn mobility_gpu_kernel6_chain_runs_kernel0_then_kernel5_in_order() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(report.kernel0_before_kernel5);
    assert_eq!(
        report.ordered_kernel_ids,
        vec![
            "mobility_gpu_kernel0_column_parent_select",
            "mobility_gpu_kernel5_row_digest_weight"
        ]
    );
}

#[test]
fn mobility_gpu_kernel6_chain_cpu_oracle_complete() {
    let columns = projected_34k_columns_for_kernel6();
    let oracle = cpu_chain_oracle(&columns);
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(report.cpu_oracle_complete);
    assert_eq!(oracle.kernel0.out_parent.len(), report.row_count);
    assert_eq!(oracle.kernel0.out_changed.len(), report.row_count);
    assert_eq!(oracle.kernel5.out_digest.len(), report.row_count);
    assert_eq!(oracle.kernel5.out_weight.len(), report.row_count);
}

#[test]
fn mobility_gpu_kernel6_chain_reports_gpu_checksums_or_unavailable() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    match report.parity_classification {
        MobilityGpuKernel0ParityClassification::ExactParity => {
            assert!(report.gpu_dispatch_occurred);
            assert!(report.kernel0_gpu_checksum.is_some());
            assert!(report.kernel5_gpu_checksum.is_some());
            assert!(report.gpu_chain_checksum.is_some());
        }
        MobilityGpuKernel0ParityClassification::GpuUnavailable => {
            assert!(!report.gpu_dispatch_occurred);
            assert!(report.gpu_chain_checksum.is_none());
        }
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
            panic!("unexpected GpuExecutionFailed: {:?}", report);
        }
    }
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
        assert_eq!(report.cpu_chain_checksum, report.gpu_chain_checksum.unwrap());
    }
}

#[test]
fn mobility_gpu_kernel6_chain_stable_under_input_permutation() {
    let baseline = projected_34k_columns_for_kernel6();
    let permuted = permuted_projected_34k_columns_for_kernel6();
    assert_eq!(baseline, permuted);
    assert_eq!(cpu_chain_oracle(&baseline), cpu_chain_oracle(&permuted));
}

#[test]
fn mobility_gpu_kernel6_chain_no_designer_authored_shader_input() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.designer_shader_input_present);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel6_chain_no_semantic_or_raw_wgsl() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.semantic_or_raw_wgsl_present);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel6_chain_shader_text_has_no_domain_terms() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.shader_text_has_domain_terms);
    assert!(!report.new_shader_text_added);
    assert!(!mobility_gpu_kernel6_chain_shader_text_has_domain_terms());
}

#[test]
fn mobility_gpu_kernel6_chain_no_default_schedule() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(report.default_schedule_unchanged);
    assert!(!report.default_production_scheduling_wired);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.default_schedule = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"default_schedule"));
}

#[test]
fn mobility_gpu_kernel6_chain_no_default_simsession_path() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.default_simsession_lib_path_wired);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.default_simsession_path = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"default_simsession_path"));
}

#[test]
fn mobility_gpu_kernel6_chain_no_gameplay_path() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.gameplay_path = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"gameplay_path"));
}

#[test]
fn mobility_gpu_kernel6_chain_no_live_slot_compaction() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.live_slot_compaction);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.live_slot_compaction = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"live_slot_compaction"));
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

#[test]
fn mobility_gpu_kernel6_chain_no_cpu_planner_urgency_commitment() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.cpu_planner_urgency_commitment);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn mobility_gpu_kernel6_chain_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel6_fixture(&fixture_input());
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.closed_ladders_reopened);
    let mut forbidden = MobilityGpuKernel6ForbiddenPathRequests::default();
    forbidden.closed_ladder_reopen = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"closed_ladder_reopen"));
}

#[test]
fn mobility_gpu_kernel6_chain_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel6_fixture(&MobilityGpuKernel6FixtureInput {
        gate: MobilityGpuKernel6Gate::default(),
        forbidden: MobilityGpuKernel6ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
        columns_override: None,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL6_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL6_NAMED_GATE);
    assert_eq!(report.row_count, 0);
}

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
fn mobility_gpu_kernel0_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel0_fixture(&MobilityGpuKernel0FixtureInput {
        gate: MobilityGpuKernel0Gate::default(),
        forbidden: MobilityGpuKernel0ForbiddenPathRequests::default(),
        columns: fixture_input().columns,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(!disabled.gpu_dispatch_occurred);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    let rejected = run_mobility_gpu_kernel0_fixture(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"mobility_gpu_kernel0_default_on_rejected"));

    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel0_default_disabled_noop() {
    let report = run_mobility_gpu_kernel0_fixture(&MobilityGpuKernel0FixtureInput {
        gate: MobilityGpuKernel0Gate::default(),
        forbidden: MobilityGpuKernel0ForbiddenPathRequests::default(),
        columns: fixture_input().columns,
    });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert_eq!(report.cpu_oracle_checksum, 0);
    assert!(!report.gpu_dispatch_occurred);
}

#[test]
fn mobility_gpu_kernel0_builtin_semantic_free_kernel_only() {
    assert!(mobility_gpu_kernel0_builtin_wgsl_is_semantic_free());
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.builtin_semantic_free_kernel_only);
    assert_eq!(report.kernel_id, MOBILITY_GPU_KERNEL0_KERNEL_ID);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL0_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL0_NAMED_GATE);
}

#[test]
fn mobility_gpu_kernel0_rejects_semantic_or_raw_wgsl() {
    let mut forbidden = MobilityGpuKernel0ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel0_rejects_designer_authored_shader_input() {
    let mut forbidden = MobilityGpuKernel0ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel0_no_default_schedule() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.default_schedule_registered);
    assert!(!report.default_production_scheduling_wired);
}

#[test]
fn mobility_gpu_kernel0_no_gameplay_path() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
}

#[test]
fn mobility_gpu_kernel0_no_default_simsession_path() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.default_simsession_lib_path_wired);
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
fn mobility_gpu_kernel0_reports_gpu_checksum_or_unavailable() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    match report.parity_classification {
        MobilityGpuKernel0ParityClassification::ExactParity => {
            assert!(report.gpu_execution_available);
            assert!(report.gpu_dispatch_occurred);
            assert!(report.gpu_result_checksum.is_some());
        }
        MobilityGpuKernel0ParityClassification::GpuUnavailable => {
            assert!(!report.gpu_execution_available);
            assert!(report.gpu_result_checksum.is_none());
        }
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
            panic!("unexpected GpuExecutionFailed: {:?}", report);
        }
    }
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
fn mobility_gpu_kernel0_no_live_slot_compaction() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.live_slot_compaction);
}

#[test]
fn mobility_gpu_kernel0_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
}

#[test]
fn mobility_gpu_kernel0_no_cpu_planner_urgency_commitment() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.cpu_planner_urgency_commitment);
}

#[test]
fn mobility_gpu_kernel0_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.default_production_scheduling_wired);
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.default_simsession_lib_path_wired);
    assert!(!report.gameplay_facing_path);
}

#[test]
fn mobility_gpu_kernel0_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel0_fixture(&MobilityGpuKernel0FixtureInput {
        gate: MobilityGpuKernel0Gate::default(),
        forbidden: MobilityGpuKernel0ForbiddenPathRequests::default(),
        columns: fixture_input().columns,
    });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert!(!report.gpu_dispatch_occurred);
    assert_eq!(report.cpu_oracle_checksum, 0);
}

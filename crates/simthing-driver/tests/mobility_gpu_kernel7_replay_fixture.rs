//! MOBILITY-GPU-KERNEL-7: deterministic multi-dispatch replay soak tests.

#[path = "support/mobility_gpu_kernel7_replay_fixture.rs"]
mod mobility_gpu_kernel7_replay_fixture;

use mobility_gpu_kernel7_replay_fixture::{
    cpu_chain_oracle, mobility_gpu_kernel7_replay_shader_text_has_domain_terms,
    permuted_projected_34k_columns_for_kernel6, projected_34k_columns_for_kernel6,
    run_mobility_gpu_kernel7_fixture, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel7FixtureInput, MobilityGpuKernel7ForbiddenPathRequests, MobilityGpuKernel7Gate,
    MOBILITY_GPU_KERNEL7_FIXTURE_ID, MOBILITY_GPU_KERNEL7_MIN_ITERATIONS,
    MOBILITY_GPU_KERNEL7_NAMED_GATE, MOBILITY_GPU_KERNEL7_NEW_SHADER_TEXT_ADDED,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

fn fixture_input() -> MobilityGpuKernel7FixtureInput {
    MobilityGpuKernel7FixtureInput::default_replay_soak()
}

fn rejected_with(
    forbidden: MobilityGpuKernel7ForbiddenPathRequests,
) -> mobility_gpu_kernel7_replay_fixture::MobilityGpuKernel7FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel7_fixture(&input)
}

#[test]
fn mobility_gpu_kernel7_replay_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel7_fixture(&MobilityGpuKernel7FixtureInput {
        gate: MobilityGpuKernel7Gate::default(),
        forbidden: MobilityGpuKernel7ForbiddenPathRequests::default(),
        iterations: MOBILITY_GPU_KERNEL7_MIN_ITERATIONS,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel7_fixture(&default_on).admitted);

    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel7_replay_default_disabled_noop() {
    let report = run_mobility_gpu_kernel7_fixture(&MobilityGpuKernel7FixtureInput {
        gate: MobilityGpuKernel7Gate::default(),
        forbidden: MobilityGpuKernel7ForbiddenPathRequests::default(),
        iterations: MOBILITY_GPU_KERNEL7_MIN_ITERATIONS,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.iteration_count, 0);
    assert_eq!(report.cpu_chain_checksum, 0);
    assert!(report.iterations.is_empty());
}

#[test]
fn mobility_gpu_kernel7_replay_uses_registered_node() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    assert_eq!(MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID, "mobility_runtime1b_non_scheduled_composition_node");
}

#[test]
fn mobility_gpu_kernel7_replay_registration_non_executing_until_invoked() {
    let reg = run_mobility_gpu_kernel7_fixture(&MobilityGpuKernel7FixtureInput {
        gate: MobilityGpuKernel7Gate::registration_only(),
        forbidden: MobilityGpuKernel7ForbiddenPathRequests::default(),
        iterations: MOBILITY_GPU_KERNEL7_MIN_ITERATIONS,
    });
    assert!(reg.admitted);
    assert!(reg.registration_non_executing);
    assert!(!reg.gpu_dispatch_occurred);
    assert_eq!(reg.iteration_count, 0);

    let dispatched = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!dispatched.registration_non_executing);
}

#[test]
fn mobility_gpu_kernel7_replay_reuses_kernel6_chain() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.reuses_kernel6_chain);
    assert_eq!(report.kernel6_chain_id, "mobility_gpu_kernel6_kernel0_then_kernel5_chain");
}

#[test]
fn mobility_gpu_kernel7_replay_runs_at_least_8_iterations() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.iteration_count >= MOBILITY_GPU_KERNEL7_MIN_ITERATIONS);
    assert_eq!(report.iterations.len(), report.iteration_count);
}

#[test]
fn mobility_gpu_kernel7_replay_cpu_oracle_stable_across_iterations() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.cpu_oracle_stable_across_iterations);
    assert_ne!(report.cpu_chain_checksum, 0);
    assert!(report
        .iterations
        .iter()
        .all(|iteration| iteration.cpu_chain_checksum == report.cpu_chain_checksum));
}

#[test]
fn mobility_gpu_kernel7_replay_gpu_checksums_stable_or_unavailable() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.gpu_checksums_stable_or_unavailable);
    match report.parity_classification {
        MobilityGpuKernel0ParityClassification::ExactParity => {
            assert!(report.gpu_chain_checksum.is_some());
            assert!(report
                .iterations
                .iter()
                .all(|iteration| iteration.gpu_chain_checksum == report.gpu_chain_checksum));
        }
        MobilityGpuKernel0ParityClassification::GpuUnavailable => {
            assert!(report.gpu_chain_checksum.is_none());
            assert!(report
                .iterations
                .iter()
                .all(|iteration| iteration.gpu_chain_checksum.is_none()));
        }
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed => {
            panic!("unexpected GpuExecutionFailed: {:?}", report);
        }
    }
}

#[test]
fn mobility_gpu_kernel7_replay_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(matches!(
        report.parity_classification,
        MobilityGpuKernel0ParityClassification::ExactParity
            | MobilityGpuKernel0ParityClassification::GpuUnavailable
    ));
}

#[test]
fn mobility_gpu_kernel7_replay_does_not_mutate_source_projection() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.source_projection_unchanged);
}

#[test]
fn mobility_gpu_kernel7_replay_stable_under_input_permutation() {
    let baseline = projected_34k_columns_for_kernel6();
    let permuted = permuted_projected_34k_columns_for_kernel6();
    assert_eq!(cpu_chain_oracle(&baseline), cpu_chain_oracle(&permuted));
    assert!(run_mobility_gpu_kernel7_fixture(&fixture_input()).permutation_stable_oracle);
}

#[test]
fn mobility_gpu_kernel7_replay_no_designer_authored_shader_input() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.designer_shader_input_present);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel7_replay_no_semantic_or_raw_wgsl() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.semantic_or_raw_wgsl_present);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel7_replay_shader_text_has_no_domain_terms() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.shader_text_has_domain_terms);
    assert!(!mobility_gpu_kernel7_replay_shader_text_has_domain_terms());
}

#[test]
fn mobility_gpu_kernel7_replay_no_new_shader_text_unless_documented() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.new_shader_text_added);
    assert!(!MOBILITY_GPU_KERNEL7_NEW_SHADER_TEXT_ADDED);
}

#[test]
fn mobility_gpu_kernel7_replay_no_default_schedule() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(report.default_schedule_unchanged);
    assert!(!report.default_production_scheduling_wired);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.default_schedule = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"default_schedule"));
}

#[test]
fn mobility_gpu_kernel7_replay_no_default_simsession_path() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.default_simsession_lib_path_wired);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.default_simsession_path = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"default_simsession_path"));
}

#[test]
fn mobility_gpu_kernel7_replay_no_gameplay_path() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.gameplay_path = true;
    assert!(rejected_with(forbidden).diagnostics.contains(&"gameplay_path"));
}

#[test]
fn mobility_gpu_kernel7_replay_no_live_slot_compaction() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.live_slot_compaction);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.live_slot_compaction = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"live_slot_compaction"));
}

#[test]
fn mobility_gpu_kernel7_replay_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

#[test]
fn mobility_gpu_kernel7_replay_no_cpu_planner_urgency_commitment() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.cpu_planner_urgency_commitment);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn mobility_gpu_kernel7_replay_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel7_fixture(&fixture_input());
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.closed_ladders_reopened);
    let mut forbidden = MobilityGpuKernel7ForbiddenPathRequests::default();
    forbidden.closed_ladder_reopen = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"closed_ladder_reopen"));
}

#[test]
fn mobility_gpu_kernel7_replay_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel7_fixture(&MobilityGpuKernel7FixtureInput {
        gate: MobilityGpuKernel7Gate::default(),
        forbidden: MobilityGpuKernel7ForbiddenPathRequests::default(),
        iterations: MOBILITY_GPU_KERNEL7_MIN_ITERATIONS,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL7_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL7_NAMED_GATE);
    assert_eq!(report.iteration_count, 0);
}

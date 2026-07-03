//! MOBILITY-GPU-KERNEL-4: 34k composition-derived projection soak tests.

#[path = "support/mobility_gpu_kernel4_34k_projection_fixture.rs"]
mod mobility_gpu_kernel4_34k_projection_fixture;

use mobility_gpu_kernel4_34k_projection_fixture::{
    cpu_column_transform_oracle, encode_parent_key_for_projection, entity_for_row,
    generate_34k_runtime_composition_input, generate_permuted_34k_runtime_composition_input,
    move_mask_for_row, projected_34k_columns, run_mobility_gpu_kernel4_fixture,
    source_key_for_block, MobilityGpuKernel0ParityClassification, MobilityGpuKernel4FixtureInput,
    MobilityGpuKernel4ForbiddenPathRequests, MobilityGpuKernel4Gate,
    MOBILITY_GPU_KERNEL1_FIXTURE_ID, MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS,
    MOBILITY_GPU_KERNEL4_ALTERNATE_DEST_KEY, MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END,
    MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START, MOBILITY_GPU_KERNEL4_FIXTURE_ID,
    MOBILITY_GPU_KERNEL4_NAMED_GATE, MOBILITY_GPU_KERNEL4_NEW_SHADER_TEXT_ADDED,
    MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY, MOBILITY_GPU_KERNEL4_ROW_COUNT,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};
use simthing_spec::compose_mobility_runtime0;

fn fixture_input() -> MobilityGpuKernel4FixtureInput {
    MobilityGpuKernel4FixtureInput::default_34k_projection_soak()
}

fn rejected_with(
    forbidden: MobilityGpuKernel4ForbiddenPathRequests,
) -> mobility_gpu_kernel4_34k_projection_fixture::MobilityGpuKernel4FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel4_fixture(&input)
}

fn row_index(
    columns: &mobility_gpu_kernel4_34k_projection_fixture::MobilityGpuKernel0ColumnProbe,
    row: usize,
) -> usize {
    let entity = entity_for_row(row) as u32;
    columns
        .entity_id
        .iter()
        .position(|&candidate| candidate == entity)
        .expect("projected row entity should be present")
}

#[test]
fn mobility_gpu_kernel4_34k_projection_explicit_opt_in_only() {
    let disabled = run_mobility_gpu_kernel4_fixture(&MobilityGpuKernel4FixtureInput {
        gate: MobilityGpuKernel4Gate::default(),
        forbidden: MobilityGpuKernel4ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    assert!(!run_mobility_gpu_kernel4_fixture(&default_on).admitted);

    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn mobility_gpu_kernel4_34k_projection_default_disabled_noop() {
    let report = run_mobility_gpu_kernel4_fixture(&MobilityGpuKernel4FixtureInput {
        gate: MobilityGpuKernel4Gate::default(),
        forbidden: MobilityGpuKernel4ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.generated_composition_row_count, 0);
    assert_eq!(report.row_count, 0);
    assert_eq!(report.cpu_oracle_checksum, 0);
    assert!(report.projection.is_none());
}

#[test]
fn mobility_gpu_kernel4_34k_projection_uses_registered_node() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(report.uses_registered_node);
    let kernel3 = report.projection.expect("projection should run");
    assert_eq!(kernel3.row_count, MOBILITY_GPU_KERNEL4_ROW_COUNT);
    let nested = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert_eq!(
        nested.projection.as_ref().and_then(|_| nested
            .projection
            .as_ref()
            .map(|_| MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID)),
        Some(MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID)
    );
}

#[test]
fn mobility_gpu_kernel4_34k_projection_registration_non_executing_until_invoked() {
    let reg = run_mobility_gpu_kernel4_fixture(&MobilityGpuKernel4FixtureInput {
        gate: MobilityGpuKernel4Gate::registration_only(),
        forbidden: MobilityGpuKernel4ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(reg.admitted);
    assert!(reg.registration_non_executing);
    assert!(!reg.gpu_dispatch_occurred);
    assert!(reg.projection.is_none());

    let dispatched = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(
        dispatched.gpu_dispatch_occurred
            || matches!(
                dispatched.parity_classification,
                MobilityGpuKernel0ParityClassification::GpuUnavailable
            )
    );
}

#[test]
fn mobility_gpu_kernel4_34k_projects_runtime_composition_to_generic_columns() {
    let columns = projected_34k_columns();
    assert_eq!(columns.entity_id.len(), columns.src_parent.len());
    assert_eq!(columns.entity_id.len(), columns.dst_parent.len());
    assert_eq!(columns.entity_id.len(), columns.move_mask.len());
    assert_eq!(
        MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS,
        [
            "entity_id",
            "src_parent",
            "dst_parent",
            "move_mask",
            "out_parent",
            "out_changed"
        ]
    );

    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(report.composition_projected);
    assert!(report.generic_column_vocabulary_only);
    assert_eq!(
        report.projection.unwrap().columns.entity_id,
        columns.entity_id
    );
}

#[test]
fn mobility_gpu_kernel4_34k_projection_row_count_is_34000() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert_eq!(
        report.generated_composition_row_count,
        MOBILITY_GPU_KERNEL4_ROW_COUNT
    );
    assert_eq!(report.row_count, MOBILITY_GPU_KERNEL4_ROW_COUNT);
}

#[test]
fn mobility_gpu_kernel4_34k_projection_preserves_deterministic_row_order() {
    let composition = compose_mobility_runtime0(&generate_34k_runtime_composition_input());
    let reenroll = composition.reenroll_report.unwrap();
    let columns = projected_34k_columns();
    let expected_entity_order: Vec<u32> = reenroll
        .final_live_slices
        .iter()
        .map(|slice| slice.entity_id as u32)
        .collect();
    assert_eq!(columns.entity_id, expected_entity_order);
}

#[test]
fn mobility_gpu_kernel4_34k_projection_includes_moved_and_unmoved_entities() {
    let projection = run_mobility_gpu_kernel4_fixture(&fixture_input())
        .projection
        .unwrap();
    assert!(projection.moved_entity_count > 0);
    assert!(projection.unmoved_entity_count > 0);
    assert_eq!(
        projection.moved_entity_count + projection.unmoved_entity_count,
        MOBILITY_GPU_KERNEL4_ROW_COUNT as u32
    );
}

#[test]
fn mobility_gpu_kernel4_34k_projection_sparse_and_dense_masks_match_oracle() {
    let columns = projected_34k_columns();
    for row in [
        1_000,
        2_000,
        MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START,
        MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START + 25,
        MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END - 1,
        20_000,
        20_001,
    ] {
        let idx = row_index(&columns, row);
        assert_eq!(columns.move_mask[idx], u32::from(move_mask_for_row(row)));
    }
}

#[test]
fn mobility_gpu_kernel4_34k_projection_edge_rows_match_oracle() {
    let columns = projected_34k_columns();
    for row in [0, MOBILITY_GPU_KERNEL4_ROW_COUNT - 1] {
        let idx = row_index(&columns, row);
        assert_eq!(columns.move_mask[idx], 1);
        assert_eq!(
            columns.src_parent[idx],
            encode_parent_key_for_projection(&simthing_spec::MobilityAlloc0ParentKey {
                parent_id: 1,
                key_id: source_key_for_block(row / 100),
            })
        );
    }
}

#[test]
fn mobility_gpu_kernel4_34k_projection_repeated_destination_parents_match_oracle() {
    let columns = projected_34k_columns();
    let repeated = encode_parent_key_for_projection(&simthing_spec::MobilityAlloc0ParentKey {
        parent_id: 1,
        key_id: MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY,
    });
    let alternate = encode_parent_key_for_projection(&simthing_spec::MobilityAlloc0ParentKey {
        parent_id: 1,
        key_id: MOBILITY_GPU_KERNEL4_ALTERNATE_DEST_KEY,
    });
    let moved_destinations: Vec<u32> = columns
        .move_mask
        .iter()
        .enumerate()
        .filter_map(|(idx, mask)| (*mask != 0).then_some(columns.dst_parent[idx]))
        .collect();
    assert!(
        moved_destinations
            .iter()
            .filter(|&&dst| dst == repeated)
            .count()
            > 100
    );
    assert!(moved_destinations.contains(&alternate));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_ignores_owner_and_econ_semantics_in_shader() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.owner_econ_semantics_in_shader);
    assert!(report.generic_column_vocabulary_only);
    assert!(!report.semantic_or_raw_wgsl_present);
}

#[test]
fn mobility_gpu_kernel4_34k_projection_reports_gpu_checksum_or_unavailable() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
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
fn mobility_gpu_kernel4_34k_projection_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
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
fn mobility_gpu_kernel4_34k_projection_stable_under_input_permutation() {
    let baseline =
        mobility_gpu_kernel4_34k_projection_fixture::project_runtime_composition_input_to_columns(
            &generate_34k_runtime_composition_input(),
        )
        .unwrap();
    let permuted =
        mobility_gpu_kernel4_34k_projection_fixture::project_runtime_composition_input_to_columns(
            &generate_permuted_34k_runtime_composition_input(),
        )
        .unwrap();
    assert_eq!(baseline.entity_id, permuted.entity_id);
    assert_eq!(baseline.src_parent, permuted.src_parent);
    assert_eq!(baseline.dst_parent, permuted.dst_parent);
    assert_eq!(baseline.move_mask, permuted.move_mask);
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_designer_authored_shader_input() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.designer_shader_input_present);
    let mut forbidden = MobilityGpuKernel4ForbiddenPathRequests::default();
    forbidden.designer_authored_shader_input = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"designer_authored_shader_input"));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_semantic_or_raw_wgsl() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.semantic_or_raw_wgsl_present);
    let mut forbidden = MobilityGpuKernel4ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_new_shader_text_unless_documented() {
    assert!(!MOBILITY_GPU_KERNEL4_NEW_SHADER_TEXT_ADDED);
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.new_shader_text_added);
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_default_schedule() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(report.default_schedule_unchanged);
    assert!(!report.default_production_scheduling_wired);
    let mut forbidden = MobilityGpuKernel4ForbiddenPathRequests::default();
    forbidden.default_schedule = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"default_schedule"));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_default_simsession_path() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.default_simsession_lib_path_wired);
    let mut forbidden = MobilityGpuKernel4ForbiddenPathRequests::default();
    forbidden.default_simsession_path = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"default_simsession_path"));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_gameplay_path() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.gameplay_facing_path);
    assert!(report.confined_to_driver_test_support);
    let mut forbidden = MobilityGpuKernel4ForbiddenPathRequests::default();
    forbidden.gameplay_path = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gameplay_path"));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_live_slot_compaction() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.live_slot_compaction);
    let mut forbidden = MobilityGpuKernel4ForbiddenPathRequests::default();
    forbidden.live_slot_compaction = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"live_slot_compaction"));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
    let mut forbidden = MobilityGpuKernel4ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

#[test]
fn mobility_gpu_kernel4_34k_projection_preserves_closed_ladder_posture() {
    let report = run_mobility_gpu_kernel4_fixture(&fixture_input());
    assert!(!report.hybrid_strata_or_faction_index_scaling);
    assert!(!report.default_production_scheduling_wired);
    assert!(!report.closed_ladders_reopened);
}

#[test]
fn mobility_gpu_kernel4_34k_projection_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_gpu_kernel4_fixture(&MobilityGpuKernel4FixtureInput {
        gate: MobilityGpuKernel4Gate::default(),
        forbidden: MobilityGpuKernel4ForbiddenPathRequests::default(),
        passgraph: fixture_input().passgraph,
    });
    assert!(report.disabled_no_op);
    assert_eq!(report.fixture_id, MOBILITY_GPU_KERNEL4_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_GPU_KERNEL4_NAMED_GATE);
    assert_eq!(report.kernel1_fixture_id, MOBILITY_GPU_KERNEL1_FIXTURE_ID);
}

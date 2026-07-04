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

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

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel8_variant_batch_no_gpu_allocator_or_nondeterministic_atomics() {
    let mut forbidden = MobilityGpuKernel8ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

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

#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[ignore = "expensive mobility GPU replay/conformance gate; run explicitly for mobility GPU replay/accounting/budget changes"]
#[test]
fn mobility_gpu_kernel9_frame_stream_no_gpu_allocator_or_nondeterministic_atomics() {
    let mut forbidden = MobilityGpuKernel9ForbiddenPathRequests::default();
    forbidden.gpu_allocator_or_nondeterministic_atomics = true;
    assert!(rejected_with(forbidden)
        .diagnostics
        .contains(&"gpu_allocator_or_nondeterministic_atomics"));
}

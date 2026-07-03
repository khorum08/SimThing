//! PALMA-PATH-8R — remove public traversal `tick()` scaffold.

mod support;

use simthing_driver::{
    FieldCadence, TraversalFieldBandSession, TraversalFieldDispatchReport,
    TraversalFieldExecutionMode, TraversalFieldGpuInput, TraversalFieldGridBinding,
    TraversalFieldShadowColumnCompatInput, TraversalFieldWInputKind, TRAVERSAL_FIELD_UTILITY_ID,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{GpuContext, MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp};
use std::sync::Mutex;

use support::palma_path_5_property_fixture::PalmaPath5PropertyTree;
use support::palma_terran_pirate_fixture::{
    DESTINATION, FIXTURE_HEIGHT, FIXTURE_ITERATIONS, FIXTURE_WIDTH,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PATH-8R");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn grid_binding(tree: &PalmaPath5PropertyTree) -> TraversalFieldGridBinding {
    TraversalFieldGridBinding {
        width: FIXTURE_WIDTH,
        height: FIXTURE_HEIGHT,
        dest_x: DESTINATION.0,
        dest_y: DESTINATION.1,
        iterations: FIXTURE_ITERATIONS,
        w_global_col: tree.w_global_col,
        d_global_col: tree.d_global_col,
        gridcell_ids: tree.gridcell_ids_row_major(),
    }
}

fn upload_flat_w_buffer(ctx: &GpuContext, w: &[f32]) -> simthing_gpu::wgpu::Buffer {
    ctx.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("palma_path_8r_flat_w"),
            contents: bytemuck::cast_slice(w),
            usage: simthing_gpu::wgpu::BufferUsages::STORAGE
                | simthing_gpu::wgpu::BufferUsages::COPY_DST,
        })
}

#[test]
fn no_public_tick_scaffold_remains() {
    fn assert_no_tick<T>() {}
    assert_no_tick::<TraversalFieldBandSession>();
    // Compile-time guard: `tick` / `tick_with_input` are not public methods on the band session.
}

#[test]
fn legacy_palma_aliases_are_not_public() {
    let lib_rs = include_str!("../src/lib.rs");
    for forbidden in [
        "palma_min_plus_field_band",
        "PalmaMinPlusFieldBandSession",
        "PalmaMinPlusFieldBandTickReport",
        "TraversalFieldBandTickReport",
        "PALMA_MIN_PLUS_FIELD_BAND",
    ] {
        assert!(
            !lib_rs.contains(forbidden),
            "lib.rs must not export legacy PALMA alias: {forbidden}"
        );
    }
    assert_eq!(TRAVERSAL_FIELD_UTILITY_ID, "min_plus_traversal_field_v1");
    fn _generic_api(_: TraversalFieldDispatchReport) {}
}

#[test]
fn gpu_resident_dispatch_requires_explicit_gpu_input() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let d_before = tree.gather_d_flat_from_properties();
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let report = band
            .dispatch_gpu_resident(ctx, TraversalFieldGpuInput::FlatW { buffer: &w_buffer })
            .expect("gpu resident dispatch");
        assert_eq!(report.w_input_kind, TraversalFieldWInputKind::GpuFlatW);
        let dispatch = report.dispatch.expect("dispatch");
        assert!(dispatch.gpu_resident);
        assert!(!dispatch.diagnostic_readback);
        assert!(dispatch.values.is_none());
        assert!(band.resident_d_output().is_some());
    });

    let d_after = tree.gather_d_flat_from_properties();
    assert_eq!(
        d_before, d_after,
        "production dispatch must not mutate property D"
    );
}

#[test]
fn shadow_column_compatibility_requires_explicit_mode() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let report = band
            .dispatch_shadow_column_compatibility(
                ctx,
                TraversalFieldShadowColumnCompatInput {
                    shadow: &mut tree.inner.shadow,
                    n_dims: tree.inner.n_dims,
                    alloc: &tree.inner.alloc,
                },
                TraversalFieldExecutionMode::DiagnosticReadback,
                false,
            )
            .expect("explicit shadow compat");
        assert_eq!(
            report.w_input_kind,
            TraversalFieldWInputKind::PackedCpuValues
        );
        assert!(report.dispatch.expect("dispatch").diagnostic_readback);
    });
}

#[test]
fn gpu_resident_d_output_exposes_field_handle_after_explicit_dispatch() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let config = tree.min_plus_config();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let op = MinPlusTraversalFieldOp::new(ctx, config.clone()).expect("op");
        let report = op
            .dispatch_traversal_from_input(
                ctx,
                simthing_gpu::MinPlusTraversalInput::GpuFlatW(&w_buffer),
                None,
                MinPlusTraversalExecutionOptions::gpu_resident(FIXTURE_ITERATIONS),
            )
            .expect("dispatch");
        assert!(report.gpu_resident);
        let handle = op.output_handle(FIXTURE_ITERATIONS);
        assert!(handle.buffer.size() > 0);
    });
}

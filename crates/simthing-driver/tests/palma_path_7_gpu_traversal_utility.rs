//! PALMA-PATH-7 — production GPU min-plus traversal utility seating.

mod support;

use simthing_driver::{
    FieldCadence, TraversalFieldBandSession, TraversalFieldExecutionMode, TraversalFieldGpuInput,
    TraversalFieldGridBinding, TraversalFieldShadowColumnCompatInput,
    TRAVERSAL_FIELD_BAND_DEFAULT_ENABLED, TRAVERSAL_FIELD_UTILITY_ID,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    GpuContext, MinPlusTraversalExecutionMode, MinPlusTraversalExecutionOptions,
    MinPlusTraversalFieldOp,
};
use std::sync::Mutex;

use support::palma_path_5_property_fixture::{max_d_field_error_public, PalmaPath5PropertyTree};
use support::palma_terran_pirate_fixture::{
    DESTINATION, FIXTURE_HEIGHT, FIXTURE_ITERATIONS, FIXTURE_WIDTH,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PATH-7");
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
            label: Some("palma_path_7_flat_w"),
            contents: bytemuck::cast_slice(w),
            usage: simthing_gpu::wgpu::BufferUsages::STORAGE
                | simthing_gpu::wgpu::BufferUsages::COPY_DST,
        })
}

#[test]
fn traversal_utility_default_off_and_named_generically() {
    assert!(!TRAVERSAL_FIELD_BAND_DEFAULT_ENABLED);
    assert_eq!(TRAVERSAL_FIELD_UTILITY_ID, "min_plus_traversal_field_v1");
}

#[test]
fn gpu_resident_mode_dispatches_without_cpu_readback_or_shadow_mutation() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let d_props_before = tree.gather_d_flat_from_properties();
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let report = band
            .dispatch_gpu_resident(ctx, TraversalFieldGpuInput::FlatW { buffer: &w_buffer })
            .expect("dispatch");
        let dispatch = report.dispatch.expect("dispatch");
        assert!(dispatch.gpu_resident);
        assert!(!dispatch.diagnostic_readback);
        assert!(dispatch.values.is_none());
    });

    let d_props_after = tree.gather_d_flat_from_properties();
    assert_eq!(
        d_props_before, d_props_after,
        "GpuResident must not mutate property D columns"
    );
}

#[test]
fn gpu_resident_op_exposes_resident_buffer_handle() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let config = tree.min_plus_config();

    with_gpu(|ctx| {
        let op = MinPlusTraversalFieldOp::new(ctx, config.clone()).expect("op");
        let packed = simthing_gpu::pack_w_and_initial_d(&w, &config).expect("pack");
        let report = op
            .dispatch_traversal(
                ctx,
                &packed,
                None,
                MinPlusTraversalExecutionOptions::gpu_resident(FIXTURE_ITERATIONS),
            )
            .expect("dispatch");
        assert!(report.gpu_resident);
        assert!(!report.diagnostic_readback);
        let _buf = op.resident_values_buffer(FIXTURE_ITERATIONS);
    });
}

#[test]
fn diagnostic_readback_mode_preserves_path5_path6_writeback() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let cpu_d = tree.cpu_oracle_d_from_property_w().expect("oracle");
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        band.dispatch_shadow_column_compatibility(
            ctx,
            TraversalFieldShadowColumnCompatInput {
                shadow: &mut tree.inner.shadow,
                n_dims: tree.inner.n_dims,
                alloc: &tree.inner.alloc,
            },
            TraversalFieldExecutionMode::DiagnosticReadback,
            true,
        )
        .expect("diagnostic shadow compat");
    });

    tree.sync_d_from_shadow_to_properties()
        .expect("property writeback");
    let from_props = tree.gather_d_flat_from_properties();
    assert!(
        max_d_field_error_public(&cpu_d, &from_props) < 1e-4,
        "diagnostic writeback preserves PATH-5/6 behavior"
    );
}

#[test]
fn oracle_verification_mode_preserves_cpu_parity() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let report = band
            .dispatch_oracle_verification_shadow_compat(
                ctx,
                TraversalFieldShadowColumnCompatInput {
                    shadow: &mut tree.inner.shadow,
                    n_dims: tree.inner.n_dims,
                    alloc: &tree.inner.alloc,
                },
            )
            .expect("oracle shadow compat");
        let err = report
            .dispatch
            .and_then(|d| d.max_oracle_error)
            .expect("oracle err");
        assert!(err < 1e-4);
    });
}

#[test]
fn default_execution_mode_is_gpu_resident() {
    assert_eq!(
        TraversalFieldExecutionMode::default(),
        MinPlusTraversalExecutionMode::GpuResident
    );
}

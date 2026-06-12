//! PALMA-PATH-8 — connect min-plus traversal utility to GPU-native field graph.

mod support;

use simthing_driver::{
    FieldCadence, TraversalFieldBandSession, TraversalFieldExecutionMode,
    TraversalFieldGridBinding, TraversalFieldInput, TraversalFieldWInputKind,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    extract_d_flat, GpuContext, MinPlusTraversalExecutionOptions,
    MinPlusTraversalFieldOp, MinPlusTraversalInput, MinPlusTraversalWInputKind,
};
use std::sync::Mutex;

use support::palma_path_5_property_fixture::{max_d_field_error_public, PalmaPath5PropertyTree};
use support::palma_terran_pirate_fixture::{
    DESTINATION, FIXTURE_HEIGHT, FIXTURE_ITERATIONS, FIXTURE_WIDTH,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PATH-8");
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
            label: Some("palma_path_8_flat_w"),
            contents: bytemuck::cast_slice(w),
            usage: simthing_gpu::wgpu::BufferUsages::STORAGE
                | simthing_gpu::wgpu::BufferUsages::COPY_DST,
        })
}

fn poison_shadow_w_columns(tree: &mut PalmaPath5PropertyTree) {
    for v in tree.inner.shadow.iter_mut() {
        *v = f32::NAN;
    }
}

#[test]
fn gpu_w_input_dispatches_without_shadow_gather() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let mut poisoned = tree;
        poison_shadow_w_columns(&mut poisoned);

        let report = band
            .tick_with_input(
                ctx,
                TraversalFieldInput::GpuFlatW { buffer: &w_buffer },
                TraversalFieldExecutionMode::GpuResident,
                false,
            )
            .expect("tick");

        assert_eq!(report.w_input_kind, TraversalFieldWInputKind::GpuFlatW);
        let dispatch = report.dispatch.expect("dispatch");
        assert_eq!(dispatch.w_input_kind, MinPlusTraversalWInputKind::GpuFlatW);
        assert!(dispatch.gpu_resident);
        assert!(!dispatch.diagnostic_readback);
        assert!(dispatch.values.is_none());

        let output = band.resident_d_output().expect("resident D handle");
        assert_eq!(output.iterations, FIXTURE_ITERATIONS);
        assert!(output.buffer.size() > 0);
    });
}

#[test]
fn gpu_resident_d_output_exposes_field_handle() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let config = tree.min_plus_config();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let op = MinPlusTraversalFieldOp::new(ctx, config.clone()).expect("op");
        let report = op
            .dispatch_traversal_from_input(
                ctx,
                MinPlusTraversalInput::GpuFlatW(&w_buffer),
                None,
                MinPlusTraversalExecutionOptions::gpu_resident(FIXTURE_ITERATIONS),
            )
            .expect("dispatch");

        assert!(report.gpu_resident);
        let handle = op.output_handle(FIXTURE_ITERATIONS);
        assert_eq!(handle.side, report.resident_side);
        assert_eq!(
            handle.buffer.size() as usize,
            op.config().values_len() * 4
        );
    });
}

#[test]
fn shadow_column_input_remains_compatibility_mode() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let report = band
            .tick(
                ctx,
                &mut tree.inner.shadow,
                tree.inner.n_dims,
                &tree.inner.alloc,
                TraversalFieldExecutionMode::GpuResident,
                false,
            )
            .expect("tick");
        assert_eq!(
            report.w_input_kind,
            TraversalFieldWInputKind::PackedCpuValues
        );
        let dispatch = report.dispatch.expect("dispatch");
        assert_eq!(
            dispatch.w_input_kind,
            MinPlusTraversalWInputKind::PackedCpuValues
        );
    });
}

#[test]
fn diagnostic_readback_preserves_path7_visibility() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let cpu_d = tree.cpu_oracle_d_from_property_w().expect("oracle");
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let report = band
            .tick_with_input(
                ctx,
                TraversalFieldInput::GpuFlatW { buffer: &w_buffer },
                TraversalFieldExecutionMode::DiagnosticReadback,
                false,
            )
            .expect("tick");
        let dispatch = report.dispatch.expect("dispatch");
        assert!(dispatch.diagnostic_readback);
        let values = dispatch.values.expect("readback values");
        let config = band.binding().stencil_config();
        let gpu_d = extract_d_flat(&values, &config).expect("extract d");
        assert!(
            max_d_field_error_public(&cpu_d, &gpu_d) < 1e-4,
            "diagnostic readback preserves PATH-7 visibility"
        );
    });
}

#[test]
fn oracle_verification_preserves_cpu_parity() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let config = tree.min_plus_config();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let op = MinPlusTraversalFieldOp::new(ctx, config).expect("op");
        let report = op
            .dispatch_traversal_from_input(
                ctx,
                MinPlusTraversalInput::GpuFlatW(&w_buffer),
                Some(&w),
                MinPlusTraversalExecutionOptions::oracle_verification(FIXTURE_ITERATIONS),
            )
            .expect("dispatch");
        let err = report.max_oracle_error.expect("oracle err");
        assert!(err < 1e-4);
    });
}

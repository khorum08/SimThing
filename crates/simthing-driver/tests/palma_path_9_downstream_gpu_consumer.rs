//! PALMA-PATH-9 — downstream GPU consumer for resident traversal D (compact probe only).

mod support;

use simthing_driver::{
    FieldCadence, TraversalFieldBandSession, TraversalFieldExecutionMode, TraversalFieldGpuInput,
    TraversalFieldGridBinding, TraversalFieldShadowColumnCompatInput,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    cpu_min_plus_d_from_w, cpu_probe_d_at_candidates, GpuContext, MinPlusTraversalDProbeConfig,
    MinPlusTraversalDProbeOp, MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp,
    MinPlusTraversalInput, MinPlusTraversalWInputKind,
};
use std::sync::Mutex;

use support::palma_min_plus_oracle::cell_index;
use support::palma_path_5_property_fixture::PalmaPath5PropertyTree;
use support::palma_terran_pirate_fixture::{
    CONVOY_START, DESTINATION, FIXTURE_HEIGHT, FIXTURE_ITERATIONS, FIXTURE_WIDTH, PIRATE_ANCHOR,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PATH-9");
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

fn upload_flat_w_buffer(ctx: &GpuContext, w: &[f32]) -> wgpu::Buffer {
    ctx.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("palma_path_9_flat_w"),
            contents: bytemuck::cast_slice(w),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        })
}

fn convoy_neighbor_candidates(width: u32, height: u32) -> Vec<u32> {
    let (x, y) = CONVOY_START;
    let ix = x as i32;
    let iy = y as i32;
    [(ix - 1, iy), (ix + 1, iy), (ix, iy - 1), (ix, iy + 1)]
        .into_iter()
        .filter(|(nx, ny)| *nx >= 0 && *ny >= 0 && *nx < width as i32 && *ny < height as i32)
        .map(|(nx, ny)| cell_index(nx as usize, ny as usize, width as usize) as u32)
        .collect()
}

fn pirate_anchor_candidates(width: u32, height: u32) -> Vec<u32> {
    let (x, y) = PIRATE_ANCHOR;
    let ix = x as i32;
    let iy = y as i32;
    [
        (ix, iy),
        (ix - 1, iy),
        (ix + 1, iy),
        (ix, iy - 1),
        (ix, iy + 1),
    ]
    .into_iter()
    .filter(|(nx, ny)| *nx >= 0 && *ny >= 0 && *nx < width as i32 && *ny < height as i32)
    .map(|(nx, ny)| cell_index(nx as usize, ny as usize, width as usize) as u32)
    .collect()
}

fn assert_probe_matches_oracle(
    gpu: &simthing_gpu::MinPlusTraversalDProbeResult,
    oracle: &simthing_gpu::MinPlusTraversalDProbeResult,
) {
    assert_eq!(gpu.gathered.len(), oracle.gathered.len());
    for (g, o) in gpu.gathered.iter().zip(oracle.gathered.iter()) {
        assert!(
            (g - o).abs() < 1e-4,
            "gathered D mismatch: gpu={g} oracle={o}"
        );
    }
    assert!(
        (gpu.min_d - oracle.min_d).abs() < 1e-4,
        "min D mismatch: gpu={} oracle={}",
        gpu.min_d,
        oracle.min_d
    );
}

#[test]
fn resident_d_output_feeds_gpu_probe_without_full_d_readback() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let config = tree.min_plus_config();
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(&config);
    let candidates = convoy_neighbor_candidates(FIXTURE_WIDTH, FIXTURE_HEIGHT);
    let cpu_d = cpu_min_plus_d_from_w(&w, &config, FIXTURE_ITERATIONS).expect("cpu oracle");
    let oracle = cpu_probe_d_at_candidates(&cpu_d, &candidates, config.inf_sentinel);

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let mut band = TraversalFieldBandSession::new(grid_binding(&tree), FieldCadence::EveryTick)
            .expect("band");
        band.enable();

        let report = band
            .dispatch_gpu_resident(ctx, TraversalFieldGpuInput::FlatW { buffer: &w_buffer })
            .expect("dispatch");
        let dispatch = report.dispatch.expect("dispatch");
        assert!(dispatch.gpu_resident);
        assert!(!dispatch.diagnostic_readback);
        assert!(
            dispatch.values.is_none(),
            "production path must not read back full D"
        );

        let resident = band.resident_d_output().expect("resident D handle");
        let probe = MinPlusTraversalDProbeOp::new(ctx);
        let gpu_probe = probe
            .probe_resident_d(ctx, resident, &probe_config, &candidates, config.cells())
            .expect("gpu probe");

        assert_probe_matches_oracle(&gpu_probe, &oracle);
    });
}

#[test]
fn shadow_columns_not_required_for_downstream_gpu_probe() {
    let tree = PalmaPath5PropertyTree::build_default();
    let w = tree.gather_w_flat_from_properties();
    let config = tree.min_plus_config();
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(&config);
    let candidates = pirate_anchor_candidates(FIXTURE_WIDTH, FIXTURE_HEIGHT);
    let cpu_d = cpu_min_plus_d_from_w(&w, &config, FIXTURE_ITERATIONS).expect("cpu oracle");
    let oracle = cpu_probe_d_at_candidates(&cpu_d, &candidates, config.inf_sentinel);

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);
        let mut poisoned = tree;
        for v in poisoned.inner.shadow.iter_mut() {
            *v = f32::NAN;
        }

        let op = MinPlusTraversalFieldOp::new(ctx, config.clone()).expect("op");
        let report = op
            .dispatch_traversal_from_input(
                ctx,
                MinPlusTraversalInput::GpuFlatW(&w_buffer),
                None,
                MinPlusTraversalExecutionOptions::gpu_resident(FIXTURE_ITERATIONS),
            )
            .expect("dispatch");
        assert_eq!(report.w_input_kind, MinPlusTraversalWInputKind::GpuFlatW);
        assert!(report.gpu_resident);
        assert!(report.values.is_none());

        let resident = op.output_handle(FIXTURE_ITERATIONS);
        let gpu_probe = MinPlusTraversalDProbeOp::new(ctx)
            .probe_resident_d(ctx, resident, &probe_config, &candidates, config.cells())
            .expect("gpu probe");

        assert_probe_matches_oracle(&gpu_probe, &oracle);
    });
}

#[test]
fn diagnostic_modes_remain_explicit() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let w = tree.gather_w_flat_from_properties();
    let binding = grid_binding(&tree);
    let mut band = TraversalFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let w_buffer = upload_flat_w_buffer(ctx, &w);

        let diagnostic = band
            .dispatch_diagnostic_readback(ctx, TraversalFieldGpuInput::FlatW { buffer: &w_buffer })
            .expect("diagnostic readback");
        let diag_dispatch = diagnostic.dispatch.expect("dispatch");
        assert!(diag_dispatch.diagnostic_readback);
        assert!(diag_dispatch.values.is_some());

        band.disable();
        band.enable();
        let shadow = band
            .dispatch_shadow_column_compatibility(
                ctx,
                TraversalFieldShadowColumnCompatInput {
                    shadow: &mut tree.inner.shadow,
                    n_dims: tree.inner.n_dims,
                    alloc: &tree.inner.alloc,
                },
                TraversalFieldExecutionMode::DiagnosticReadback,
                true,
            )
            .expect("shadow compat");
        let shadow_dispatch = shadow.dispatch.expect("dispatch");
        assert_eq!(
            shadow_dispatch.w_input_kind,
            MinPlusTraversalWInputKind::PackedCpuValues
        );
        assert!(shadow_dispatch.diagnostic_readback);
    });
}

#[test]
fn no_route_or_predecessor_constructs() {
    let src = include_str!("../src/min_plus_traversal_field.rs");
    let probe_src = include_str!("../../simthing-gpu/src/min_plus_traversal_d_probe.rs");
    let forbidden = [
        "RouteObject",
        "PredecessorTable",
        "PathfindingEngine",
        "MovementPolicy",
        "GraphManager",
    ];
    for term in forbidden {
        assert!(
            !src.contains(term) && !probe_src.contains(term),
            "forbidden construct {term} must not appear in PATH-9 surface"
        );
    }
}

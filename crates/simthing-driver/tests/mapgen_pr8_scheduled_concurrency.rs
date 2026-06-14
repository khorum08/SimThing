//! MapGen PR8 — Gu-Yang ∥ PALMA scheduled-concurrency GPU measurement spike.
//!
//! Compares serial queue submits vs single-encoder scheduled W compose + PALMA min-plus over the
//! PR7 MapGen tiny slice. Compact D probe readback only — no full-field CPU decision readback.

use std::sync::Mutex;
use std::time::Instant;

use simthing_clausething::{
    build_w_impedance_compose_from_palma, generate_default_mapgen_palma_feedstock,
    parse_mapgen_neutral_document, MAPGEN_MF_CHOKE_OUTPUT_COL, MAPGEN_MF_SOURCE_COL,
    MAPGEN_PALMA_D_OUTPUT_COL,
};
use simthing_driver::{
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config,
    FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    dispatch_scheduled_w_palma_chain, dispatch_serial_w_palma_chain, GpuContext, MinPlusStencilOp,
    MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp, MinPlusTraversalExecutionOptions,
    MinPlusTraversalFieldOp, MinPlusTraversalInput, MinPlusTraversalWInputKind,
    WImpedanceComposeOp, MIN_PLUS_INF,
};
use simthing_spec::{
    compile_region_field_preview, compile_w_impedance_compose_preview, MappingExecutionProfile,
    RegionFieldOperatorSpec,
};

const RAW_FIXTURE: &str = include_str!(
    "../../simthing-clausething/tests/fixtures/mapgen/tiny_pentad_hub_slice_raw.clause"
);

const TRAVERSAL_ITERATIONS: u32 = 4;
const PROBE_TOLERANCE: f32 = 1e-3;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn mapgen_palma_pack() -> simthing_clausething::MapGenPalmaFeedstockAuthoring {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse fixture");
    generate_default_mapgen_palma_feedstock(&neutral).expect("PR7 palma feedstock")
}

fn eml_weights(pack: &simthing_clausething::HydratedScenarioPack) -> (f32, f32) {
    let formula = pack.game_mode.region_fields[0]
        .parent_formula
        .as_ref()
        .expect("PR6 parent_formula");
    (
        formula.weight_pressure.expect("weight_pressure"),
        formula.weight_resource.expect("weight_resource"),
    )
}

fn spare_choke_b_col(palma: &simthing_clausething::HydratedScenarioPalmaFeedstock) -> u32 {
    let choke_a = palma
        .choke_output_col
        .expect("palma feedstock requires choke_output_col");
    let claimed = [
        palma.source_col,
        choke_a,
        palma.w_output_col,
        palma.d_output_col,
    ];
    (0..palma.n_dims)
        .find(|col| !claimed.contains(col))
        .expect("tiny slice n_dims must leave spare compose choke_b column")
}

fn seed_interleaved_values(
    field: &simthing_spec::spec::region_field::RegionFieldSpec,
    palma: &simthing_clausething::HydratedScenarioPalmaFeedstock,
) -> Vec<f32> {
    let width = field.grid_size;
    let height = field.grid_size;
    let cells = (width * height) as usize;
    let n_dims = field.n_dims as usize;
    let mut values = vec![0.0f32; cells * n_dims];
    let idx = |slot: u32, col: u32| (slot as usize * n_dims) + col as usize;
    let choke_a = match &field.operator {
        RegionFieldOperatorSpec::SaturatingFlux {
            choke_output_col: Some(col),
            ..
        } => *col,
        _ => panic!("PR6 field must be SaturatingFlux"),
    };
    let choke_b = spare_choke_b_col(palma);
    for slot in 0..cells as u32 {
        values[idx(slot, field.source_col)] = 1.0;
        values[idx(slot, choke_a)] = 0.75;
        values[idx(slot, choke_b)] = 0.0;
    }
    values
}

fn upload_interleaved(ctx: &GpuContext, values: &[f32]) -> wgpu::Buffer {
    ctx.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mapgen_pr8_interleaved"),
            contents: bytemuck::cast_slice(values),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        })
}

fn probe_resident_d(
    ctx: &GpuContext,
    stencil: &MinPlusTraversalFieldOp,
    d_col: u32,
    dest: (u32, u32),
    probe_cell: u32,
    iterations: u32,
) -> f32 {
    let resident = stencil.output_handle(iterations);
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(stencil.config());
    let probe_result = MinPlusTraversalDProbeOp::new(ctx)
        .probe_resident_d(
            ctx,
            resident,
            &probe_config,
            &[probe_cell],
            stencil.config().cells(),
        )
        .expect("compact D probe");
    assert_eq!(probe_result.gathered.len(), 1);
    assert!(probe_result.gathered[0].is_finite());
    let _ = (d_col, dest);
    probe_result.gathered[0]
}

struct WPalmaHarnessContext {
    w_gpu: simthing_gpu::WImpedanceComposeConfig,
    stencil: MinPlusTraversalFieldOp,
    probe_cell: u32,
}

fn build_w_palma_harness(
    pack: &simthing_clausething::MapGenPalmaFeedstockAuthoring,
    ctx: &GpuContext,
) -> WPalmaHarnessContext {
    let field = &pack.pack.game_mode.region_fields[0];
    let palma = pack.pack.palma_feedstock.as_ref().expect("palma feedstock");
    let w_spec = build_w_impedance_compose_from_palma(palma);
    let w_compiled = compile_w_impedance_compose_preview(&w_spec).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let dest = (1, 1);
    let stencil = MinPlusTraversalFieldOp::new(
        ctx,
        composed_w_min_plus_stencil_config(&w_gpu, 0, palma.d_output_col, dest, MIN_PLUS_INF),
    )
    .expect("min-plus stencil");
    let probe_cell = dest.1 * field.grid_size + dest.0;
    WPalmaHarnessContext {
        w_gpu,
        stencil,
        probe_cell,
    }
}

fn run_mapping_tick(ctx: &GpuContext, pack: &simthing_clausething::MapGenPalmaFeedstockAuthoring) {
    let field = pack.pack.game_mode.region_fields[0].clone();
    let preview = compile_region_field_preview(&field).expect("region field admission");
    let commitment = preview.commitment.expect("commitment");
    let weights = eml_weights(&pack.pack);
    let mut mapping =
        FirstSliceMappingSession::open(ctx, MappingExecutionProfile::SparseRegionFieldV1, &field)
            .expect("open mapping");
    mapping
        .queue_seeds(&[FirstSliceSeed {
            row: 1,
            col: 1,
            value: 120.0,
        }])
        .expect("queue seed");
    let report = mapping
        .tick_with_commitment_spec_fixture(
            ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
        )
        .expect("mapping tick");
    assert!(report.mapping.enabled);
    assert!(report.mapping.scheduled);
    assert!(report.mapping.field_values.is_none());
    assert!(report.mapping.reduction_parent_value.is_none());
}

#[test]
fn mapgen_pr7_authoring_remains_default_off_before_gpu_exercise() {
    let authoring = mapgen_palma_pack();
    assert_eq!(
        authoring.pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert!(authoring.pack.palma_feedstock.is_some());
    assert!(authoring.pack.w_impedance_compose.is_some());
    assert_eq!(authoring.expansion_report.route_surface_count, 0);
    assert_eq!(authoring.expansion_report.predecessor_surface_count, 0);
}

#[test]
fn pr8_source_has_no_forbidden_vocabulary() {
    let batch_src = include_str!("../../simthing-gpu/src/scheduled_w_palma_batch.rs");
    for forbidden in [
        "pathfinding",
        "predecessor",
        "movement_order",
        "euclidean",
        "sqrt(",
    ] {
        assert!(
            !batch_src.contains(forbidden),
            "scheduled batch module must not reference `{forbidden}`"
        );
    }
    let bridge_src = include_str!("../src/w_impedance_compose_bridge.rs");
    for forbidden in ["pathfinding", "predecessor", "cpu_planner", "normalize("] {
        assert!(
            !bridge_src.contains(forbidden),
            "w compose bridge must not reference `{forbidden}`"
        );
    }
}

#[test]
fn gpu_measurement_skips_cleanly_when_no_adapter() {
    if try_gpu() {
        eprintln!("GPU adapter present — skipping no-adapter skip test");
        return;
    }
    eprintln!("skipping: no GPU adapter available (expected on some CI hosts)");
}

#[test]
fn serial_and_scheduled_w_palma_produce_comparable_compact_probes() {
    let Ok(ctx) = GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU adapter available");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let pack = mapgen_palma_pack();
    let field = &pack.pack.game_mode.region_fields[0];
    let palma = pack.pack.palma_feedstock.as_ref().expect("palma");
    run_mapping_tick(&ctx, &pack);

    let harness = build_w_palma_harness(&pack, &ctx);
    let w_op = WImpedanceComposeOp::new(&ctx);
    let values = seed_interleaved_values(field, palma);

    // Serial baseline
    let serial_buffer = upload_interleaved(&ctx, &values);
    let serial_start = Instant::now();
    let serial_submits = dispatch_serial_w_palma_chain(
        &ctx,
        &w_op,
        &harness.w_gpu,
        &serial_buffer,
        &harness.stencil,
        TRAVERSAL_ITERATIONS,
    )
    .expect("serial chain");
    let serial_elapsed = serial_start.elapsed().as_millis();
    let serial_probe = probe_resident_d(
        &ctx,
        &harness.stencil,
        palma.d_output_col,
        (1, 1),
        harness.probe_cell,
        TRAVERSAL_ITERATIONS,
    );

    // Scheduled-concurrency path (fresh buffer, identical seed)
    let scheduled_stencil = MinPlusTraversalFieldOp::new(&ctx, harness.stencil.config().clone())
        .expect("scheduled stencil session");
    let scheduled_buffer = upload_interleaved(&ctx, &values);
    let scheduled_start = Instant::now();
    dispatch_scheduled_w_palma_chain(
        &ctx,
        &w_op,
        &harness.w_gpu,
        &scheduled_buffer,
        &scheduled_stencil,
        TRAVERSAL_ITERATIONS,
    )
    .expect("scheduled chain");
    let scheduled_elapsed = scheduled_start.elapsed().as_millis();
    let scheduled_probe = probe_resident_d(
        &ctx,
        &scheduled_stencil,
        palma.d_output_col,
        (1, 1),
        harness.probe_cell,
        TRAVERSAL_ITERATIONS,
    );

    assert!(
        serial_submits > 1,
        "serial baseline must use multiple queue submits (got {serial_submits})"
    );
    assert_eq!(
        serial_submits,
        MinPlusStencilOp::serial_w_palma_queue_submit_count(TRAVERSAL_ITERATIONS)
    );
    assert!(
        (serial_probe - scheduled_probe).abs() <= PROBE_TOLERANCE,
        "serial D {serial_probe} vs scheduled D {scheduled_probe}"
    );
    assert!(serial_probe.is_finite());
    assert!(scheduled_probe.is_finite());

    eprintln!(
        "mapgen_pr8 evidence: serial_submits={serial_submits} scheduled_submits=1 \
         serial_probe={serial_probe} scheduled_probe={scheduled_probe} \
         serial_elapsed_ms={serial_elapsed} scheduled_elapsed_ms={scheduled_elapsed}"
    );
}

#[test]
fn scheduled_traversal_stays_gpu_resident_without_full_field_readback() {
    let Ok(ctx) = GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU adapter available");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let pack = mapgen_palma_pack();
    let field = &pack.pack.game_mode.region_fields[0];
    let palma = pack.pack.palma_feedstock.as_ref().expect("palma");
    let harness = build_w_palma_harness(&pack, &ctx);
    let values = seed_interleaved_values(field, palma);
    let buffer = upload_interleaved(&ctx, &values);

    let traversal =
        MinPlusTraversalFieldOp::new(&ctx, harness.stencil.config().clone()).expect("traversal op");
    WImpedanceComposeOp::new(&ctx)
        .compose_resident_field(&ctx, &buffer, &harness.w_gpu)
        .expect("w compose");

    let report = traversal
        .dispatch_traversal_from_input(
            &ctx,
            MinPlusTraversalInput::GpuInterleavedW(&buffer),
            None,
            MinPlusTraversalExecutionOptions::gpu_resident(TRAVERSAL_ITERATIONS),
        )
        .expect("gpu resident traversal");
    assert_eq!(
        report.w_input_kind,
        MinPlusTraversalWInputKind::GpuInterleavedW
    );
    assert!(report.gpu_resident);
    assert!(
        report.values.is_none(),
        "production traversal must not read back full D"
    );
    assert_eq!(report.iterations, TRAVERSAL_ITERATIONS);
    assert_eq!(palma.w_output_col, MAPGEN_PALMA_D_OUTPUT_COL - 1);
    assert_eq!(MAPGEN_MF_SOURCE_COL, 0);
    assert_eq!(MAPGEN_MF_CHOKE_OUTPUT_COL, 2);
}

#[test]
fn generated_mapgen_surfaces_have_no_route_or_predecessor_vocabulary() {
    let pack = mapgen_palma_pack();
    let json = serde_json::to_string(&pack.pack.game_mode).expect("serialize game mode");
    for forbidden in [
        "pathfinding",
        "predecessor",
        "movement_order",
        "route",
        "border",
    ] {
        assert!(
            !json.contains(forbidden),
            "generated game mode must not reference `{forbidden}`"
        );
    }
}

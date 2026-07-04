//! MapGen PR8 — Gu-Yang ∥ PALMA scheduled-concurrency GPU measurement spike.
//!
//! Compares serial queue submits vs single-encoder scheduled W compose + PALMA min-plus over the
//! PR7 MapGen tiny slice. Compact D probe readback only — no full-field CPU decision readback.
//!
//! 0R2 dependency-floor fixture: source text required by `mapgen_constitution_guards` survivor
//! `pr8_harness_documents_compact_probe_only_readback` via `include_str!`. Runnable PR8 tests
//! remain deleted; helpers kept only for survivor compile-time string checks.

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

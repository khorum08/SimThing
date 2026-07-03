//! BH-2C — prove BH-2B composed W feeds PALMA/min-plus traversal as GPU-resident impedance.
//!
//! Production chain (no movement policy, no pathfinding engine):
//! ```text
//! interleaved field buffer → WImpedanceComposeOp → MinPlusTraversalInput::GpuInterleavedW
//!   → MinPlusTraversalFieldOp (GpuResident) → MinPlusTraversalDProbeOp (compact readback)
//! ```
//!
//! Test-only helpers in this file are not production APIs.

mod support;

use simthing_driver::{
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    cpu_min_plus_relaxation, cpu_probe_d_at_candidates, cpu_w_impedance_compose_oracle,
    extract_d_flat, GpuContext, MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp,
    MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp, MinPlusTraversalInput,
    MinPlusTraversalWInputKind, WImpedanceComposeConfig, WImpedanceComposeOp, MIN_PLUS_INF,
};
use simthing_spec::{compile_w_impedance_compose_preview, WImpedanceComposeSpec};
use std::sync::Mutex;

use support::palma_min_plus_oracle::cell_index;
use support::palma_terran_pirate_fixture::{
    build_location_w_field, CONVOY_START, DESTINATION, FIXTURE_HEIGHT, FIXTURE_ITERATIONS,
    FIXTURE_WIDTH,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const BH2C_BASE_W_COL: u32 = 0;
const BH2C_CHOKE_A_COL: u32 = 1;
const BH2C_CHOKE_B_COL: u32 = 2;
const BH2C_OUTPUT_W_COL: u32 = 3;
const BH2C_D_COL: u32 = 4;
const BH2C_N_DIMS: u32 = 5;

const BH2C_HOT_PATH_FORBIDDEN: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "magnitude",
    "norm(",
];

const BH2C_FORBIDDEN_PRODUCTION_VOCAB: &[&str] = &[
    "Terran",
    "Pirate",
    "border",
    "frontline",
    "ambush",
    "fleet_ai",
    "pathfinding",
    "movement_engine",
    "route",
    "predecessor",
];

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-2C");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

/// Test-only fixture: interleaved buffer with base W + choke columns (not a production API).
fn build_interleaved_fixture(
    base_w_flat: &[f32],
    width: u32,
    height: u32,
    choke_a: f32,
    choke_b: f32,
) -> Vec<f32> {
    let cells = (width * height) as usize;
    assert_eq!(base_w_flat.len(), cells);
    let mut values = vec![0.0f32; cells * BH2C_N_DIMS as usize];
    for slot in 0..cells as u32 {
        values[idx(slot, BH2C_BASE_W_COL, BH2C_N_DIMS)] = base_w_flat[slot as usize];
        values[idx(slot, BH2C_CHOKE_A_COL, BH2C_N_DIMS)] = choke_a;
        values[idx(slot, BH2C_CHOKE_B_COL, BH2C_N_DIMS)] = choke_b;
    }
    values
}

fn compose_spec(weight_a: f32, weight_b: f32) -> WImpedanceComposeSpec {
    WImpedanceComposeSpec {
        width: FIXTURE_WIDTH,
        height: FIXTURE_HEIGHT,
        n_dims: BH2C_N_DIMS,
        base_w_col: BH2C_BASE_W_COL,
        choke_a_col: BH2C_CHOKE_A_COL,
        choke_b_col: BH2C_CHOKE_B_COL,
        profiles: vec![simthing_spec::WImpedanceComposeProfileSpec {
            weight_a,
            weight_b,
            output_w_col: BH2C_OUTPUT_W_COL,
        }],
    }
}

fn compose_config(weight_a: f32, weight_b: f32) -> WImpedanceComposeConfig {
    let spec = compose_spec(weight_a, weight_b);
    let compiled = compile_w_impedance_compose_preview(&spec).expect("admission");
    compiled_w_impedance_compose_to_gpu_config(&compiled)
}

fn stencil_config(compose: &WImpedanceComposeConfig) -> simthing_gpu::MinPlusStencilConfig {
    composed_w_min_plus_stencil_config(compose, 0, BH2C_D_COL, DESTINATION, MIN_PLUS_INF)
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

/// Test-only CPU oracle: compose W then min-plus relaxation on interleaved buffer.
fn cpu_oracle_probe(
    values: &[f32],
    compose: &WImpedanceComposeConfig,
    stencil: &simthing_gpu::MinPlusStencilConfig,
    candidates: &[u32],
) -> simthing_gpu::MinPlusTraversalDProbeResult {
    let composed = cpu_w_impedance_compose_oracle(values, compose);
    let final_values =
        cpu_min_plus_relaxation(&composed, stencil, FIXTURE_ITERATIONS).expect("cpu relaxation");
    let d = extract_d_flat(&final_values, stencil).expect("extract D");
    cpu_probe_d_at_candidates(&d, candidates, stencil.inf_sentinel)
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

/// Test-only GPU chain: compose W in-place, traverse from interleaved W, compact D probe.
fn run_compose_then_traversal_probe(
    ctx: &GpuContext,
    values: &[f32],
    compose: &WImpedanceComposeConfig,
    stencil: &simthing_gpu::MinPlusStencilConfig,
    candidates: &[u32],
) -> simthing_gpu::MinPlusTraversalDProbeResult {
    let buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bh2c_interleaved_field"),
            contents: bytemuck::cast_slice(values),
            usage: wgpu::BufferUsages::STORAGE,
        });

    WImpedanceComposeOp::new(ctx)
        .compose_resident_field(ctx, &buffer, compose)
        .expect("compose dispatch");

    let op = MinPlusTraversalFieldOp::new(ctx, stencil.clone()).expect("traversal op");
    let report = op
        .dispatch_traversal_from_input(
            ctx,
            MinPlusTraversalInput::GpuInterleavedW(&buffer),
            None,
            MinPlusTraversalExecutionOptions::gpu_resident(FIXTURE_ITERATIONS),
        )
        .expect("traversal dispatch");
    assert_eq!(
        report.w_input_kind,
        MinPlusTraversalWInputKind::GpuInterleavedW
    );
    assert!(report.gpu_resident);
    assert!(
        report.values.is_none(),
        "production traversal must not read back full D"
    );

    let resident = op.output_handle(FIXTURE_ITERATIONS);
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(stencil);
    MinPlusTraversalDProbeOp::new(ctx)
        .probe_resident_d(ctx, resident, &probe_config, candidates, stencil.cells())
        .expect("compact D probe")
}

#[test]
fn bh2c_composed_w_feeds_palma_gpu_traversal() {
    let field = build_location_w_field(true, None, false);
    let values = build_interleaved_fixture(&field.w, FIXTURE_WIDTH, FIXTURE_HEIGHT, 0.2, 0.1);
    let compose = compose_config(2.0, 1.0);
    let stencil = stencil_config(&compose);
    let candidates = convoy_neighbor_candidates(FIXTURE_WIDTH, FIXTURE_HEIGHT);
    let oracle = cpu_oracle_probe(&values, &compose, &stencil, &candidates);

    with_gpu(|ctx| {
        let gpu_probe =
            run_compose_then_traversal_probe(ctx, &values, &compose, &stencil, &candidates);
        assert_probe_matches_oracle(&gpu_probe, &oracle);
    });
}

#[test]
fn bh2c_choke_weight_changes_traversal_cost() {
    let field = build_location_w_field(true, None, false);
    let values = build_interleaved_fixture(&field.w, FIXTURE_WIDTH, FIXTURE_HEIGHT, 0.6, 0.4);
    let low_compose = compose_config(0.5, 0.5);
    let high_compose = compose_config(8.0, 6.0);
    let stencil_low = stencil_config(&low_compose);
    let stencil_high = stencil_config(&high_compose);
    let candidates = convoy_neighbor_candidates(FIXTURE_WIDTH, FIXTURE_HEIGHT);

    with_gpu(|ctx| {
        let low_probe =
            run_compose_then_traversal_probe(ctx, &values, &low_compose, &stencil_low, &candidates);
        let high_probe = run_compose_then_traversal_probe(
            ctx,
            &values,
            &high_compose,
            &stencil_high,
            &candidates,
        );
        assert!(
            high_probe.min_d > low_probe.min_d + 1e-3,
            "higher choke weights must raise min-plus D at candidates: low={} high={}",
            low_probe.min_d,
            high_probe.min_d
        );
    });
}

#[test]
fn bh2c_resident_d_no_full_field_readback() {
    let compose_src = include_str!("../../simthing-gpu/src/w_impedance_compose.rs");

    assert!(
        !compose_src.contains("MapMode::Read"),
        "W compose production path must not read back field buffer"
    );
    assert!(
        !compose_src.contains("copy_buffer_to_buffer"),
        "W compose production path must not stage full-field readback"
    );

    let field = build_location_w_field(false, None, false);
    let values = build_interleaved_fixture(&field.w, FIXTURE_WIDTH, FIXTURE_HEIGHT, 0.0, 0.0);
    let compose = compose_config(1.0, 1.0);
    let stencil = stencil_config(&compose);

    with_gpu(|ctx| {
        let buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bh2c_readback_guard_field"),
                contents: bytemuck::cast_slice(&values),
                usage: wgpu::BufferUsages::STORAGE,
            });
        WImpedanceComposeOp::new(ctx)
            .compose_resident_field(ctx, &buffer, &compose)
            .expect("compose");
        let op = MinPlusTraversalFieldOp::new(ctx, stencil.clone()).expect("op");
        let report = op
            .dispatch_traversal_from_input(
                ctx,
                MinPlusTraversalInput::GpuInterleavedW(&buffer),
                None,
                MinPlusTraversalExecutionOptions::gpu_resident(FIXTURE_ITERATIONS),
            )
            .expect("dispatch");
        assert!(report.gpu_resident);
        assert!(
            report.values.is_none(),
            "GpuResident must not read back full D"
        );
    });
}

#[test]
fn bh2c_no_route_or_predecessor_objects() {
    let bridge = include_str!("../src/w_impedance_compose_bridge.rs");
    let compose = include_str!("../../simthing-gpu/src/w_impedance_compose.rs");
    let stencil = include_str!("../../simthing-gpu/src/min_plus_stencil.rs");
    let forbidden = [
        "RouteObject",
        "PredecessorTable",
        "PathfindingEngine",
        "MovementPolicy",
        "GraphManager",
        "MovementEngine",
    ];
    for term in forbidden {
        for (name, src) in [
            ("bridge", bridge),
            ("compose", compose),
            ("stencil", stencil),
        ] {
            assert!(
                !src.contains(term),
                "forbidden construct {term} in BH-2C {name} surface"
            );
        }
    }
}

#[test]
fn bh2c_no_native_sqrt_in_hot_path() {
    let paths = [
        (
            "w_impedance_compose.rs",
            include_str!("../../simthing-gpu/src/w_impedance_compose.rs"),
        ),
        (
            "w_impedance_compose.wgsl",
            include_str!("../../simthing-gpu/src/shaders/w_impedance_compose.wgsl"),
        ),
        (
            "min_plus_stencil.rs",
            include_str!("../../simthing-gpu/src/min_plus_stencil.rs"),
        ),
        (
            "min_plus_stencil.wgsl",
            include_str!("../../simthing-gpu/src/shaders/min_plus_stencil.wgsl"),
        ),
        (
            "min_plus_traversal_d_probe.rs",
            include_str!("../../simthing-gpu/src/min_plus_traversal_d_probe.rs"),
        ),
        (
            "min_plus_traversal_d_probe.wgsl",
            include_str!("../../simthing-gpu/src/shaders/min_plus_traversal_d_probe.wgsl"),
        ),
        (
            "w_impedance_compose_bridge.rs",
            include_str!("../src/w_impedance_compose_bridge.rs"),
        ),
    ];
    for (name, src) in paths {
        for term in BH2C_HOT_PATH_FORBIDDEN {
            assert!(
                !src.contains(term),
                "forbidden token `{term}` in BH-2C hot path {name}"
            );
        }
    }
}

#[test]
fn bh2c_scaffolding_not_required_for_production_pass() {
    let bridge = include_str!("../src/w_impedance_compose_bridge.rs");
    assert!(
        bridge.contains("composed_w_min_plus_stencil_config"),
        "BH-2C live API must be named in production bridge"
    );
    assert!(
        !bridge.contains("cpu_oracle") && !bridge.contains("build_interleaved_fixture"),
        "production bridge must not depend on test scaffolding"
    );

    let compose_src = include_str!("../../simthing-gpu/src/w_impedance_compose.rs");
    let stencil_src = include_str!("../../simthing-gpu/src/min_plus_stencil.rs");
    assert!(
        !compose_src.contains("bh2c_palma") && !stencil_src.contains("bh2c_palma"),
        "GPU production ops must not reference BH-2C test module"
    );

    let test_src = include_str!("bh2c_palma_w_feedstock.rs");
    assert!(
        test_src.contains("fn build_interleaved_fixture")
            && test_src.contains("fn cpu_oracle_probe")
            && test_src.contains("fn run_compose_then_traversal_probe"),
        "test scaffolding must remain in test file only"
    );
}

#[test]
fn bh2c_forbidden_production_vocabulary() {
    let bridge = include_str!("../src/w_impedance_compose_bridge.rs");
    let compose_rust = include_str!("../../simthing-gpu/src/w_impedance_compose.rs");
    let compose_wgsl = include_str!("../../simthing-gpu/src/shaders/w_impedance_compose.wgsl");
    for term in BH2C_FORBIDDEN_PRODUCTION_VOCAB {
        assert!(!bridge.contains(term), "forbidden vocab `{term}` in bridge");
        assert!(
            !compose_rust.contains(term),
            "forbidden vocab `{term}` in compose Rust"
        );
        assert!(
            !compose_wgsl.contains(term),
            "forbidden vocab `{term}` in compose WGSL"
        );
    }
}

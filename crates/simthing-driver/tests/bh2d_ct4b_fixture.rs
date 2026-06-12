//! BH-2D — CT-4b 200×200 fixture proof over resident W/D/stress feedstock.
//!
//! Production chain (fixture proof only — no movement policy):
//! ```text
//! field pressure columns → BH-0/BH-1 choke readout → BH-2B W compose → BH-2S stress
//!   → PALMA GpuInterleavedW → resident D → compact probe
//! ```
//!
//! All fixture builders and CPU oracles are test-only scaffolding.

mod support;

use simthing_driver::{
    compiled_stress_compose_to_gpu_config, compiled_w_impedance_compose_to_gpu_config,
    composed_w_min_plus_stencil_config,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    cpu_min_plus_relaxation, cpu_probe_d_at_candidates, cpu_stress_compose_oracle,
    cpu_w_impedance_compose_oracle, extract_d_flat, GpuContext, MinPlusTraversalDProbeOp,
    MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp, MinPlusTraversalInput,
    MinPlusTraversalWInputKind, StressComposeConfig, StressComposeOp, WImpedanceComposeConfig,
    WImpedanceComposeOp, MIN_PLUS_INF, STRESS_OP_MISMATCH, STRESS_OP_OVERLAP,
};
use simthing_spec::{
    compile_stress_compose_preview, compile_w_impedance_compose_preview, StressComposeSpec,
    StressOperatorSpec, WImpedanceComposeSpec,
};
use std::sync::Mutex;

use support::ct4b_field_fixture::{
    Ct4bFixture, COL_BASE_W, COL_CHOKE_A, COL_CHOKE_B, COL_OUTPUT_W_PROFILE_0,
    COL_OUTPUT_W_PROFILE_1, COL_PRESSURE_A, COL_PRESSURE_B, COL_STRESS_MISMATCH,
    COL_STRESS_OVERLAP, CT4B_AUTOMATA_COUNT, CT4B_CELL_COUNT, CT4B_DEST, CT4B_FIELD_A_SOURCES,
    CT4B_FIELD_B_SOURCES, CT4B_HEIGHT, CT4B_MIN_PLUS_ITERATIONS, CT4B_N_DIMS, CT4B_PROBE_ANCHOR,
    CT4B_SOURCE_COUNT, CT4B_WIDTH,
};
use support::palma_min_plus_oracle::cell_index;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const BH2D_HOT_PATH_FORBIDDEN: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "magnitude",
    "norm(",
];

const BH2D_FORBIDDEN_PRODUCTION_VOCAB: &[&str] = &[
    "Terran",
    "Pirate",
    "culture",
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
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-2D");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32) -> usize {
    (slot * CT4B_N_DIMS + col) as usize
}

fn w_compose_spec() -> WImpedanceComposeSpec {
    WImpedanceComposeSpec {
        width: CT4B_WIDTH,
        height: CT4B_HEIGHT,
        n_dims: CT4B_N_DIMS,
        base_w_col: COL_BASE_W,
        choke_a_col: COL_CHOKE_A,
        choke_b_col: COL_CHOKE_B,
        profiles: vec![
            simthing_spec::WImpedanceComposeProfileSpec {
                weight_a: 1.0,
                weight_b: 0.5,
                output_w_col: COL_OUTPUT_W_PROFILE_0,
            },
            simthing_spec::WImpedanceComposeProfileSpec {
                weight_a: 6.0,
                weight_b: 4.0,
                output_w_col: COL_OUTPUT_W_PROFILE_1,
            },
        ],
    }
}

fn w_compose_config() -> WImpedanceComposeConfig {
    let compiled = compile_w_impedance_compose_preview(&w_compose_spec()).expect("w admission");
    compiled_w_impedance_compose_to_gpu_config(&compiled)
}

fn stress_compose_spec() -> StressComposeSpec {
    StressComposeSpec {
        width: CT4B_WIDTH,
        height: CT4B_HEIGHT,
        n_dims: CT4B_N_DIMS,
        choke_a_col: COL_CHOKE_A,
        choke_b_col: COL_CHOKE_B,
        profiles: vec![
            simthing_spec::StressComposeProfileSpec {
                operator: StressOperatorSpec::Overlap,
                output_col: COL_STRESS_OVERLAP,
            },
            simthing_spec::StressComposeProfileSpec {
                operator: StressOperatorSpec::Mismatch,
                output_col: COL_STRESS_MISMATCH,
            },
        ],
    }
}

fn stress_compose_config() -> StressComposeConfig {
    let compiled =
        compile_stress_compose_preview(&stress_compose_spec()).expect("stress admission");
    compiled_stress_compose_to_gpu_config(&compiled)
}

fn stencil_for_profile(
    compose: &WImpedanceComposeConfig,
    profile_index: usize,
) -> simthing_gpu::MinPlusStencilConfig {
    composed_w_min_plus_stencil_config(
        compose,
        profile_index,
        support::ct4b_field_fixture::COL_D,
        CT4B_DEST,
        MIN_PLUS_INF,
    )
}

/// Test-only CPU oracle after W compose + min-plus relaxation.
fn cpu_oracle_probe(
    values: &[f32],
    compose: &WImpedanceComposeConfig,
    profile_index: usize,
    candidates: &[u32],
) -> simthing_gpu::MinPlusTraversalDProbeResult {
    let composed = cpu_w_impedance_compose_oracle(values, compose);
    let stencil = stencil_for_profile(compose, profile_index);
    let final_values =
        cpu_min_plus_relaxation(&composed, &stencil, CT4B_MIN_PLUS_ITERATIONS).expect("cpu relax");
    let d = extract_d_flat(&final_values, &stencil).expect("extract d");
    cpu_probe_d_at_candidates(&d, candidates, stencil.inf_sentinel)
}

/// Test-only readback helper — not used on production-path assertions.
fn readback_buffer(ctx: &GpuContext, src: &wgpu::Buffer, len: usize) -> Vec<f32> {
    let bytes = (len * std::mem::size_of::<f32>()) as u64;
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("bh2d_test_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("bh2d_test_readback_enc"),
        });
    encoder.copy_buffer_to_buffer(src, 0, &staging, 0, bytes);
    ctx.queue.submit(Some(encoder.finish()));
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    ctx.device.poll(wgpu::Maintain::Wait);
    let data = slice.get_mapped_range();
    let out = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging.unmap();
    out
}

/// Production GPU chain: W compose → stress compose → PALMA GpuInterleavedW → compact D probe.
fn run_production_feedstock_probe(
    ctx: &GpuContext,
    values: &[f32],
    compose: &WImpedanceComposeConfig,
    stress: &StressComposeConfig,
    profile_index: usize,
    candidates: &[u32],
) -> simthing_gpu::MinPlusTraversalDProbeResult {
    let buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bh2d_resident_field"),
            contents: bytemuck::cast_slice(values),
            usage: wgpu::BufferUsages::STORAGE,
        });

    WImpedanceComposeOp::new(ctx)
        .compose_resident_field(ctx, &buffer, compose)
        .expect("w compose");

    StressComposeOp::new(ctx)
        .compose_resident_field(ctx, &buffer, stress)
        .expect("stress compose");

    let stencil = stencil_for_profile(compose, profile_index);
    let op = MinPlusTraversalFieldOp::new(ctx, stencil.clone()).expect("traversal op");
    let report = op
        .dispatch_traversal_from_input(
            ctx,
            MinPlusTraversalInput::GpuInterleavedW(&buffer),
            None,
            MinPlusTraversalExecutionOptions::gpu_resident(CT4B_MIN_PLUS_ITERATIONS),
        )
        .expect("palma dispatch");
    assert_eq!(
        report.w_input_kind,
        MinPlusTraversalWInputKind::GpuInterleavedW
    );
    assert!(report.gpu_resident);
    assert!(
        report.values.is_none(),
        "production path must not read back full D"
    );

    let resident = op.output_handle(CT4B_MIN_PLUS_ITERATIONS);
    let probe_config = simthing_gpu::MinPlusTraversalDProbeConfig::from_stencil_config(&stencil);
    MinPlusTraversalDProbeOp::new(ctx)
        .probe_resident_d(ctx, resident, &probe_config, candidates, stencil.cells())
        .expect("compact probe")
}

fn assert_probe_close(
    gpu: &simthing_gpu::MinPlusTraversalDProbeResult,
    oracle: &simthing_gpu::MinPlusTraversalDProbeResult,
) {
    assert_eq!(gpu.gathered.len(), oracle.gathered.len());
    for (g, o) in gpu.gathered.iter().zip(oracle.gathered.iter()) {
        assert!(
            (g - o).abs() < 1.0,
            "gathered D mismatch: gpu={g} oracle={o}"
        );
    }
    assert!(
        (gpu.min_d - oracle.min_d).abs() < 1.0,
        "min_d mismatch: gpu={} oracle={}",
        gpu.min_d,
        oracle.min_d
    );
}

#[test]
fn bh2d_ct4b_fixture_builds_200x200_generic_fields() {
    let fixture = Ct4bFixture::build_seeded();
    assert_eq!(CT4B_WIDTH, 200);
    assert_eq!(CT4B_HEIGHT, 200);
    assert_eq!(CT4B_CELL_COUNT, 40_000);
    assert_eq!(CT4B_SOURCE_COUNT, 100);
    assert_eq!(fixture.field_a_sources.len(), CT4B_FIELD_A_SOURCES);
    assert_eq!(fixture.field_b_sources.len(), CT4B_FIELD_B_SOURCES);
    assert_eq!(CT4B_AUTOMATA_COUNT, 150);
    assert_eq!(
        fixture.values_len(),
        (CT4B_CELL_COUNT * CT4B_N_DIMS) as usize
    );

    let mut seeded_pressure = 0usize;
    for &slot in &fixture.field_a_sources {
        assert!(fixture.values[idx(slot, COL_PRESSURE_A)] > 0.0);
        seeded_pressure += 1;
    }
    for &slot in &fixture.field_b_sources {
        assert!(fixture.values[idx(slot, COL_PRESSURE_B)] > 0.0);
        seeded_pressure += 1;
    }
    assert_eq!(seeded_pressure, CT4B_SOURCE_COUNT);

    with_gpu(|ctx| {
        let mut fluxed = fixture;
        fluxed.apply_gpu_flux_choke_both_fields(ctx);
        let anchor = CT4B_PROBE_ANCHOR;
        assert!(
            fluxed.choke_a_at(anchor.0, anchor.1) > 0.0
                || fluxed.choke_b_at(anchor.0, anchor.1) > 0.0
                || {
                    let mut local = 0.0f32;
                    for slot in 0..CT4B_CELL_COUNT {
                        local += fluxed.values[idx(slot, COL_CHOKE_A)];
                        local += fluxed.values[idx(slot, COL_CHOKE_B)];
                    }
                    local > 0.0
                },
            "flux readout must produce resident choke on the map"
        );
        let mut choke_sum = 0.0f32;
        for slot in 0..CT4B_CELL_COUNT {
            choke_sum += fluxed.values[idx(slot, COL_CHOKE_A)];
            choke_sum += fluxed.values[idx(slot, COL_CHOKE_B)];
        }
        assert!(
            choke_sum > 0.0,
            "choke columns must be populated after BH-0/BH-1"
        );
    });
}

#[test]
fn bh2d_two_profiles_produce_distinct_w_outputs() {
    let mut fixture = Ct4bFixture::build_seeded();
    with_gpu(|ctx| {
        fixture.apply_gpu_flux_choke_both_fields(ctx);
        let compose = w_compose_config();
        let buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bh2d_w_profiles"),
                contents: bytemuck::cast_slice(&fixture.values),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            });
        WImpedanceComposeOp::new(ctx)
            .compose_resident_field(ctx, &buffer, &compose)
            .expect("compose");
        let gpu = readback_buffer(ctx, &buffer, compose.values_len());
        let anchor_slot = cell_index(
            CT4B_PROBE_ANCHOR.0 as usize,
            CT4B_PROBE_ANCHOR.1 as usize,
            CT4B_WIDTH as usize,
        ) as u32;
        let w0 = gpu[idx(anchor_slot, COL_OUTPUT_W_PROFILE_0)];
        let w1 = gpu[idx(anchor_slot, COL_OUTPUT_W_PROFILE_1)];
        assert!(
            (w1 - w0).abs() > 1e-3,
            "profiles must differ at anchor: w0={w0} w1={w1}"
        );
    });
}

#[test]
fn bh2d_composed_w_feeds_resident_palma_d() {
    let mut fixture = Ct4bFixture::build_seeded();
    let compose = w_compose_config();
    let stress = stress_compose_config();
    let candidates = Ct4bFixture::probe_anchor_candidates();
    with_gpu(|ctx| {
        fixture.apply_gpu_flux_choke_both_fields(ctx);
        let oracle = cpu_oracle_probe(&fixture.values, &compose, 0, &candidates);
        let gpu_probe =
            run_production_feedstock_probe(ctx, &fixture.values, &compose, &stress, 0, &candidates);
        assert_probe_close(&gpu_probe, &oracle);
    });
}

#[test]
fn bh2d_profile_weight_changes_compact_d_probe() {
    let mut fixture = Ct4bFixture::build_seeded();
    let compose = w_compose_config();
    let stress = stress_compose_config();
    let candidates = Ct4bFixture::probe_anchor_candidates();
    with_gpu(|ctx| {
        fixture.apply_gpu_flux_choke_both_fields(ctx);
        let probe0 =
            run_production_feedstock_probe(ctx, &fixture.values, &compose, &stress, 0, &candidates);
        let probe1 =
            run_production_feedstock_probe(ctx, &fixture.values, &compose, &stress, 1, &candidates);
        assert!(
            (probe1.min_d - probe0.min_d).abs() > 1e-3,
            "profile weights must change compact D probe: p0={} p1={}",
            probe0.min_d,
            probe1.min_d
        );
    });
}

#[test]
fn bh2d_overlap_stress_available_as_field_policy_feedstock() {
    let mut fixture = Ct4bFixture::build_seeded();
    let stress = stress_compose_config();
    with_gpu(|ctx| {
        fixture.apply_gpu_flux_choke_both_fields(ctx);
        let buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bh2d_stress_feedstock"),
                contents: bytemuck::cast_slice(&fixture.values),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            });
        StressComposeOp::new(ctx)
            .compose_resident_field(ctx, &buffer, &stress)
            .expect("stress compose");
        let gpu = readback_buffer(ctx, &buffer, stress.values_len());
        let expected = cpu_stress_compose_oracle(&fixture.values, &stress);
        let anchor_slot = cell_index(
            CT4B_PROBE_ANCHOR.0 as usize,
            CT4B_PROBE_ANCHOR.1 as usize,
            CT4B_WIDTH as usize,
        ) as u32;
        let overlap = gpu[idx(anchor_slot, COL_STRESS_OVERLAP)];
        let mismatch = gpu[idx(anchor_slot, COL_STRESS_MISMATCH)];
        let exp_overlap = expected[idx(anchor_slot, COL_STRESS_OVERLAP)];
        let exp_mismatch = expected[idx(anchor_slot, COL_STRESS_MISMATCH)];
        assert!((overlap - exp_overlap).abs() < 1e-4);
        assert!((mismatch - exp_mismatch).abs() < 1e-4);
        assert_eq!(
            exp_overlap,
            fixture.values[idx(anchor_slot, COL_CHOKE_A)]
                * fixture.values[idx(anchor_slot, COL_CHOKE_B)]
        );
        assert!(
            overlap > 0.0 || mismatch > 0.0,
            "stress columns must be resident numeric feedstock"
        );
    });
}

#[test]
fn bh2d_no_full_field_cpu_readback_for_decision() {
    let compose_src = include_str!("../../simthing-gpu/src/w_impedance_compose.rs");
    let stress_src = include_str!("../../simthing-gpu/src/stress_compose.rs");
    assert!(!compose_src.contains("MapMode::Read"));
    assert!(!stress_src.contains("MapMode::Read"));
    assert!(!compose_src.contains("copy_buffer_to_buffer"));

    let mut fixture = Ct4bFixture::build_seeded();
    let compose = w_compose_config();
    let stress = stress_compose_config();
    let candidates = Ct4bFixture::probe_anchor_candidates();
    with_gpu(|ctx| {
        fixture.apply_gpu_flux_choke_both_fields(ctx);
        let _probe =
            run_production_feedstock_probe(ctx, &fixture.values, &compose, &stress, 0, &candidates);
    });
}

#[test]
fn bh2d_no_route_or_predecessor_objects() {
    let paths = [
        include_str!("../src/w_impedance_compose_bridge.rs"),
        include_str!("../../simthing-gpu/src/w_impedance_compose.rs"),
        include_str!("../../simthing-gpu/src/stress_compose.rs"),
        include_str!("../../simthing-gpu/src/min_plus_stencil.rs"),
        include_str!("support/ct4b_field_fixture.rs"),
    ];
    let forbidden = [
        "RouteObject",
        "PredecessorTable",
        "PathfindingEngine",
        "MovementPolicy",
        "MovementEngine",
        "GraphManager",
    ];
    for term in forbidden {
        for src in paths {
            assert!(!src.contains(term), "forbidden `{term}` in BH-2D surface");
        }
    }
}

#[test]
fn bh2d_scaffolding_promoted_or_quarantined() {
    let bridge = include_str!("../src/w_impedance_compose_bridge.rs");
    assert!(
        bridge.contains("composed_w_min_plus_stencil_config"),
        "BH-2C live API must remain the PALMA handoff surface"
    );
    assert!(
        !bridge.contains("Ct4bFixture") && !bridge.contains("cpu_oracle_probe"),
        "production bridge must not depend on BH-2D test scaffolding"
    );
    let fixture_src = include_str!("support/ct4b_field_fixture.rs");
    assert!(
        fixture_src.contains("Test-only") || fixture_src.contains("test-only"),
        "fixture module must be labeled test-only"
    );
    let test_src = include_str!("bh2d_ct4b_fixture.rs");
    assert!(
        test_src.contains("readback_buffer") && test_src.contains("cpu_oracle_probe"),
        "readback/oracle helpers must stay in test file"
    );
}

#[test]
fn bh2d_no_native_sqrt_in_hot_path() {
    let paths = [
        (
            "w_impedance_compose.rs",
            include_str!("../../simthing-gpu/src/w_impedance_compose.rs"),
        ),
        (
            "stress_compose.rs",
            include_str!("../../simthing-gpu/src/stress_compose.rs"),
        ),
        (
            "min_plus_stencil.rs",
            include_str!("../../simthing-gpu/src/min_plus_stencil.rs"),
        ),
        (
            "w_impedance_compose_bridge.rs",
            include_str!("../src/w_impedance_compose_bridge.rs"),
        ),
        (
            "ct4b_field_fixture.rs",
            include_str!("support/ct4b_field_fixture.rs"),
        ),
    ];
    for (name, src) in paths {
        for term in BH2D_HOT_PATH_FORBIDDEN {
            assert!(
                !src.contains(term),
                "forbidden `{term}` in BH-2D hot path {name}"
            );
        }
    }
}

#[test]
fn bh2d_forbidden_production_vocabulary() {
    let paths = [
        include_str!("support/ct4b_field_fixture.rs"),
        include_str!("../src/w_impedance_compose_bridge.rs"),
        include_str!("../src/stress_compose_bridge.rs"),
    ];
    for src in paths {
        for term in BH2D_FORBIDDEN_PRODUCTION_VOCAB {
            assert!(!src.contains(term), "forbidden production vocab `{term}`");
        }
    }
}

#[test]
fn bh2d_stress_operators_are_overlap_and_mismatch_only() {
    let stress = stress_compose_config();
    assert_eq!(stress.profiles.len(), 2);
    assert_eq!(stress.profiles[0].operator_kind, STRESS_OP_OVERLAP);
    assert_eq!(stress.profiles[1].operator_kind, STRESS_OP_MISMATCH);
}

//! BH-2S — GPU-resident stress field algebra tests.

use simthing_gpu::{
    cpu_stress_compose_oracle, GpuContext, StressComposeConfig, StressComposeOp,
    StressComposeProfile, STRESS_OP_MISMATCH, STRESS_OP_OVERLAP, STRESS_OP_VELOCITY,
    STRESS_OP_WEIGHTED,
};
use std::sync::Mutex;
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const BH2S_HOT_PATH_FORBIDDEN: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "magnitude",
    "norm(",
];

const BH2S_FORBIDDEN_PRODUCTION_VOCAB: &[&str] = &[
    "border",
    "frontline",
    "culture",
    "Terran",
    "Pirate",
    "ambush",
    "hegemony",
    "fleet_ai",
    "pathfinding",
    "movement_engine",
    "route",
    "predecessor",
];

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-2S tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn base_config(profiles: Vec<StressComposeProfile>) -> StressComposeConfig {
    StressComposeConfig {
        width: 8,
        height: 8,
        n_dims: 8,
        choke_a_col: 0,
        choke_b_col: 1,
        profiles,
    }
}

fn sample_values(config: &StressComposeConfig) -> Vec<f32> {
    let mut values = vec![0.0f32; config.values_len()];
    let nd = config.n_dims;
    for slot in 0..config.cells() {
        values[idx(slot, 0, nd)] = 0.1 + (slot % 7) as f32 * 0.05;
        values[idx(slot, 1, nd)] = 0.2 + (slot % 5) as f32 * 0.04;
        values[idx(slot, 2, nd)] = 0.05 + (slot % 3) as f32 * 0.02;
        values[idx(slot, 3, nd)] = 0.15;
    }
    values
}

fn readback_buffer(ctx: &GpuContext, src: &wgpu::Buffer, len: usize) -> Vec<f32> {
    let bytes = (len * std::mem::size_of::<f32>()) as u64;
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("bh2s_stress_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("bh2s_stress_readback_enc"),
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

fn run_gpu_compose(ctx: &GpuContext, config: &StressComposeConfig, values: &[f32]) -> Vec<f32> {
    let buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bh2s_stress_field"),
            contents: bytemuck::cast_slice(values),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let op = StressComposeOp::new(ctx);
    op.compose_resident_field(ctx, &buffer, config)
        .expect("compose dispatch");
    readback_buffer(ctx, &buffer, config.values_len())
}

#[test]
fn bh2s_overlap_stress_gpu_matches_cpu_oracle() {
    with_gpu(|ctx| {
        let config = base_config(vec![StressComposeProfile {
            operator_kind: STRESS_OP_OVERLAP,
            weight_a: 0.0,
            weight_b: 0.0,
            output_col: 4,
            choke_now_col: 0,
            choke_prev_col: 0,
        }]);
        let values = sample_values(&config);
        let expected = cpu_stress_compose_oracle(&values, &config);
        let gpu = run_gpu_compose(ctx, &config, &values);
        for slot in 0..config.cells() {
            let i = idx(slot, 4, config.n_dims);
            assert!((gpu[i] - expected[i]).abs() <= 1e-5);
        }
    });
}

#[test]
fn bh2s_mismatch_stress_gpu_matches_cpu_oracle() {
    with_gpu(|ctx| {
        let config = base_config(vec![StressComposeProfile {
            operator_kind: STRESS_OP_MISMATCH,
            weight_a: 0.0,
            weight_b: 0.0,
            output_col: 5,
            choke_now_col: 0,
            choke_prev_col: 0,
        }]);
        let values = sample_values(&config);
        let expected = cpu_stress_compose_oracle(&values, &config);
        let gpu = run_gpu_compose(ctx, &config, &values);
        for slot in 0..config.cells() {
            let i = idx(slot, 5, config.n_dims);
            assert!((gpu[i] - expected[i]).abs() <= 1e-5);
        }
    });
}

#[test]
fn bh2s_stress_columns_feed_threshold_without_cpu_planner() {
    let src = include_str!("../src/stress_compose.rs");
    assert!(!src.contains("MapMode::Read"));
    assert!(!src.contains("cpu_planner"));
    assert!(!src.contains("pathfinding"));

    let config = base_config(vec![StressComposeProfile {
        operator_kind: STRESS_OP_OVERLAP,
        weight_a: 0.0,
        weight_b: 0.0,
        output_col: 4,
        choke_now_col: 0,
        choke_prev_col: 0,
    }]);
    let mut values = sample_values(&config);
    values[idx(10, 0, config.n_dims)] = 0.9;
    values[idx(10, 1, config.n_dims)] = 0.8;
    let oracle = cpu_stress_compose_oracle(&values, &config);
    let stress = oracle[idx(10, 4, config.n_dims)];
    let threshold = 0.5;
    let crossed = stress > threshold;
    assert!(crossed, "oracle stress feeds threshold without CPU planner");
}

#[test]
fn bh2s_no_full_field_cpu_readback_for_decision() {
    let src = include_str!("../src/stress_compose.rs");
    assert!(!src.contains("MapMode::Read"));
    assert!(!src.contains("copy_buffer_to_buffer"));
}

#[test]
fn bh2s_multi_profile_weights_change_outputs_without_semantic_code() {
    with_gpu(|ctx| {
        let base = base_config(vec![
            StressComposeProfile {
                operator_kind: STRESS_OP_OVERLAP,
                weight_a: 0.0,
                weight_b: 0.0,
                output_col: 4,
                choke_now_col: 0,
                choke_prev_col: 0,
            },
            StressComposeProfile {
                operator_kind: STRESS_OP_WEIGHTED,
                weight_a: 2.0,
                weight_b: -0.5,
                output_col: 5,
                choke_now_col: 0,
                choke_prev_col: 0,
            },
        ]);
        let mut alt = base.clone();
        alt.profiles[1].weight_a = 5.0;

        let values = sample_values(&base);
        let gpu_base = run_gpu_compose(ctx, &base, &values);
        let gpu_alt = run_gpu_compose(ctx, &alt, &values);

        let mut weighted_differs = false;
        for slot in 0..base.cells() {
            let overlap = idx(slot, 4, base.n_dims);
            assert!((gpu_base[overlap] - gpu_alt[overlap]).abs() <= 1e-5);
            if (gpu_base[idx(slot, 5, base.n_dims)] - gpu_alt[idx(slot, 5, base.n_dims)]).abs()
                > 1e-5
            {
                weighted_differs = true;
            }
        }
        assert!(weighted_differs);
    });
}


//! BH-2B — GPU-resident W impedance composition tests.

use simthing_gpu::{
    cpu_w_impedance_compose_oracle, GpuContext, WImpedanceComposeConfig, WImpedanceComposeOp,
    WImpedanceComposeProfile,
};
use std::sync::Mutex;
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const BH2_HOT_PATH_FORBIDDEN: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "magnitude",
    "norm(",
];

const BH2_FORBIDDEN_PRODUCTION_VOCAB: &[&str] = &[
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
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-2 tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn two_profile_config(w: u32, h: u32) -> WImpedanceComposeConfig {
    WImpedanceComposeConfig {
        width: w,
        height: h,
        n_dims: 6,
        base_w_col: 0,
        choke_a_col: 1,
        choke_b_col: 2,
        profiles: vec![
            WImpedanceComposeProfile {
                weight_a: 1.0,
                weight_b: 0.5,
                output_w_col: 3,
            },
            WImpedanceComposeProfile {
                weight_a: 2.0,
                weight_b: -1.0,
                output_w_col: 4,
            },
        ],
    }
}

fn sample_values(config: &WImpedanceComposeConfig) -> Vec<f32> {
    let mut values = vec![0.0f32; config.values_len()];
    let nd = config.n_dims;
    for slot in 0..config.cells() {
        values[idx(slot, 0, nd)] = 1.0 + slot as f32 * 0.01;
        values[idx(slot, 1, nd)] = 0.1 * ((slot % 5) as f32);
        values[idx(slot, 2, nd)] = 0.2 * ((slot % 3) as f32);
    }
    values
}

fn readback_buffer(ctx: &GpuContext, src: &wgpu::Buffer, len: usize) -> Vec<f32> {
    let bytes = (len * std::mem::size_of::<f32>()) as u64;
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("bh2_w_compose_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("bh2_w_compose_readback_enc"),
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

fn run_gpu_compose(ctx: &GpuContext, config: &WImpedanceComposeConfig, values: &[f32]) -> Vec<f32> {
    let buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bh2_w_compose_field"),
            contents: bytemuck::cast_slice(values),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let op = WImpedanceComposeOp::new(ctx);
    op.compose_resident_field(ctx, &buffer, config)
        .expect("compose dispatch");
    readback_buffer(ctx, &buffer, config.values_len())
}

#[test]
fn bh2_w_composition_gpu_matches_cpu_oracle() {
    with_gpu(|ctx| {
        let config = two_profile_config(8, 8);
        let values = sample_values(&config);
        let expected = cpu_w_impedance_compose_oracle(&values, &config);
        let gpu = run_gpu_compose(ctx, &config, &values);
        assert_eq!(gpu.len(), expected.len());
        for (i, (g, e)) in gpu.iter().zip(expected.iter()).enumerate() {
            assert!(
                (g - e).abs() <= 1e-5,
                "slot mismatch at {i}: gpu={g} oracle={e}"
            );
        }
    });
}

#[test]
fn bh2_profile_weights_change_w_without_changing_choke_inputs() {
    with_gpu(|ctx| {
        let base = two_profile_config(4, 4);
        let mut alt = base.clone();
        alt.profiles[0].weight_a = 5.0;
        alt.profiles[1].weight_b = 3.0;

        let values = sample_values(&base);
        let gpu_base = run_gpu_compose(ctx, &base, &values);
        let gpu_alt = run_gpu_compose(ctx, &alt, &values);

        let nd = base.n_dims;
        let mut profile0_differs = false;
        let mut profile1_differs = false;
        for slot in 0..base.cells() {
            let p0 = idx(slot, 3, nd);
            let p1 = idx(slot, 4, nd);
            if (gpu_base[p0] - gpu_alt[p0]).abs() > 1e-5 {
                profile0_differs = true;
            }
            if (gpu_base[p1] - gpu_alt[p1]).abs() > 1e-5 {
                profile1_differs = true;
            }
            assert!(
                (gpu_base[idx(slot, 1, nd)] - gpu_alt[idx(slot, 1, nd)]).abs() <= 1e-5,
                "choke_a must be unchanged"
            );
            assert!(
                (gpu_base[idx(slot, 2, nd)] - gpu_alt[idx(slot, 2, nd)]).abs() <= 1e-5,
                "choke_b must be unchanged"
            );
        }
        assert!(profile0_differs, "profile 0 W must change with weight_a");
        assert!(profile1_differs, "profile 1 W must change with weight_b");
    });
}

#[test]
fn bh2_clear_choke_preserves_base_w() {
    with_gpu(|ctx| {
        let config = two_profile_config(4, 4);
        let mut values = sample_values(&config);
        for slot in 0..config.cells() {
            values[idx(slot, 1, config.n_dims)] = 0.0;
            values[idx(slot, 2, config.n_dims)] = 0.0;
        }
        let gpu = run_gpu_compose(ctx, &config, &values);
        for slot in 0..config.cells() {
            let base = values[idx(slot, 0, config.n_dims)];
            let w0 = gpu[idx(slot, 3, config.n_dims)];
            let w1 = gpu[idx(slot, 4, config.n_dims)];
            assert!((w0 - base).abs() <= 1e-5, "profile 0 should equal base_w");
            assert!((w1 - base).abs() <= 1e-5, "profile 1 should equal base_w");
        }
    });
}

#[test]
fn bh2_crowded_choke_raises_impedance() {
    with_gpu(|ctx| {
        let config = WImpedanceComposeConfig {
            width: 3,
            height: 3,
            n_dims: 4,
            base_w_col: 0,
            choke_a_col: 1,
            choke_b_col: 2,
            profiles: vec![WImpedanceComposeProfile {
                weight_a: 2.0,
                weight_b: 1.0,
                output_w_col: 3,
            }],
        };
        let mut clear = vec![0.0f32; config.values_len()];
        for slot in 0..config.cells() {
            clear[idx(slot, 0, config.n_dims)] = 1.0;
        }
        let mut crowded = clear.clone();
        for slot in 0..config.cells() {
            crowded[idx(slot, 1, config.n_dims)] = 0.8;
            crowded[idx(slot, 2, config.n_dims)] = 0.6;
        }
        let clear_gpu = run_gpu_compose(ctx, &config, &clear);
        let crowded_gpu = run_gpu_compose(ctx, &config, &crowded);
        let mut raised = false;
        for slot in 0..config.cells() {
            let c = clear_gpu[idx(slot, 3, config.n_dims)];
            let h = crowded_gpu[idx(slot, 3, config.n_dims)];
            assert!(h >= c - 1e-5);
            if h > c + 1e-5 {
                raised = true;
            }
        }
        assert!(raised, "crowded choke must raise at least one W cell");
    });
}

#[test]
fn bh2_multiple_profiles_write_distinct_w_outputs() {
    with_gpu(|ctx| {
        let config = two_profile_config(4, 4);
        let values = sample_values(&config);
        let gpu = run_gpu_compose(ctx, &config, &values);
        let nd = config.n_dims;
        let mut distinct = false;
        for slot in 0..config.cells() {
            let w0 = gpu[idx(slot, 3, nd)];
            let w1 = gpu[idx(slot, 4, nd)];
            if (w0 - w1).abs() > 1e-5 {
                distinct = true;
                break;
            }
        }
        assert!(distinct, "two profiles must produce different W fields");
    });
}

#[test]
fn bh2_no_pathfinding_or_movement_policy_constructs() {
    let rust_src = include_str!("../src/w_impedance_compose.rs");
    let wgsl_src = include_str!("../src/shaders/w_impedance_compose.wgsl");
    for term in BH2_FORBIDDEN_PRODUCTION_VOCAB {
        assert!(
            !rust_src.contains(term),
            "forbidden production vocab `{term}` in Rust hot path"
        );
        assert!(
            !wgsl_src.contains(term),
            "forbidden production vocab `{term}` in WGSL hot path"
        );
    }
}

#[test]
fn bh2_no_native_sqrt_in_hot_path() {
    let rust_src = include_str!("../src/w_impedance_compose.rs");
    let wgsl_src = include_str!("../src/shaders/w_impedance_compose.wgsl");
    for term in BH2_HOT_PATH_FORBIDDEN {
        assert!(
            !rust_src.contains(term),
            "forbidden token `{term}` in Rust hot path"
        );
        assert!(
            !wgsl_src.contains(term),
            "forbidden token `{term}` in WGSL hot path"
        );
    }
}
#[test]
fn bh2_impedance_feed_stays_gpu_resident() {
    with_gpu(|ctx| {
        let config = two_profile_config(4, 4);
        let values = sample_values(&config);
        let buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bh2_gpu_resident_field"),
                contents: bytemuck::cast_slice(&values),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let op = WImpedanceComposeOp::new(ctx);
        op.compose_resident_field(ctx, &buffer, &config)
            .expect("gpu resident compose");
    });
}

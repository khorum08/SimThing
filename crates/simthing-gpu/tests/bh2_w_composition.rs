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

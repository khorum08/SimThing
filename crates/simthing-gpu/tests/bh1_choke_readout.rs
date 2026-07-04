//! BH-1 — GPU-resident choke readout from SaturatingFlux (test-only CPU oracle).

use simthing_gpu::{
    cpu_compute_c_at, cpu_compute_choke_at, cpu_compute_choke_readout_at, cpu_horizon,
    cpu_stencil_step, params_from_config, GpuContext, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const BH_HOT_PATH_FORBIDDEN: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "magnitude",
    "norm(",
];

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-1 tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn choke_config(
    w: u32,
    h: u32,
    horizon: u32,
    u_sat: f32,
    chi: f32,
    choke_col: u32,
) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon,
        alpha_self: 1.0,
        gamma_neighbor: 0.0,
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col: Some(choke_col),
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

fn assert_bits_equal(gpu: &[f32], cpu: &[f32]) {
    assert_eq!(gpu.len(), cpu.len());
    let mut max_err = 0.0f32;
    for (i, (g, c)) in gpu.iter().zip(cpu.iter()).enumerate() {
        if g.to_bits() != c.to_bits() {
            max_err = max_err.max((g - c).abs());
            assert!(
                (g - c).abs() < 1e-4,
                "mismatch at index {i}: gpu={g} cpu={c} (bits gpu={} cpu={})",
                g.to_bits(),
                c.to_bits()
            );
        }
    }
    assert!(max_err < 1e-4, "max_err={max_err}");
}

fn run_gpu_horizon(
    ctx: &GpuContext,
    config: &StructuredFieldStencilConfig,
    values: &[f32],
    hops: u32,
) -> Vec<f32> {
    let op = StructuredFieldStencilOp::new(ctx, config.clone()).expect("op");
    op.upload_values(ctx, values).expect("upload");
    op.run_ping_pong(ctx, hops).expect("dispatch").0
}

fn sum_choke_column(values: &[f32], w: u32, h: u32, choke_col: u32, n_dims: u32) -> f32 {
    (0..w * h)
        .map(|slot| values[idx(slot, choke_col, n_dims)])
        .sum()
}

fn scan_for_forbidden_tokens(source: &str, label: &str) {
    let lower = source.to_ascii_lowercase();
    for token in BH_HOT_PATH_FORBIDDEN {
        assert!(
            !lower.contains(token),
            "{label} contains forbidden BH hot-path token `{token}`"
        );
    }
}

#[test]
fn bh1_choke_readout_gpu_matches_cpu_oracle() {
    with_gpu(|ctx| {
        for &(w, h) in &[(4u32, 4u32), (8u32, 8u32)] {
            for &horizon in &[1u32, 2u32] {
                let config = choke_config(w, h, horizon, 2.0, 0.25, 1);
                let params = params_from_config(&config);
                let mut values = vec![0.0f32; config.values_len()];
                values[idx(0, 0, 4)] = 5.0;
                values[idx(w * h - 1, 0, 4)] = 3.0;
                values[idx(w / 2, 0, 4)] = 1.5;
                let cpu = cpu_horizon(&values, &params, horizon);
                let gpu = run_gpu_horizon(ctx, &config, &values, horizon);
                assert_bits_equal(&gpu, &cpu);
            }
        }
    });
}

#[test]
fn bh1_crowded_fixture_choke_oracle_only() {
    // BH-1 readout oracle only — GPU consumption is BH-1R (`SaturatingFluxChokeThresholdOp`).
    let config = choke_config(3, 3, 1, 1.0, 0.25, 1);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(4, 0, 4)] = 2.0;
    values[idx(1, 0, 4)] = 2.0;
    values[idx(3, 0, 4)] = 2.0;
    values[idx(5, 0, 4)] = 2.0;
    values[idx(7, 0, 4)] = 2.0;

    let out = cpu_stencil_step(&values, &params);
    let total_choke = sum_choke_column(&out, 3, 3, 1, 4);
    let threshold = 0.5f32;
    assert!(
        total_choke > threshold,
        "crowded fixture sum(choke)={total_choke} should cross threshold {threshold}"
    );

    let c_center = cpu_compute_c_at(&values, &params, 1, 1);
    assert_eq!(
        cpu_compute_choke_at(c_center, 0.25).to_bits(),
        1.0f32.to_bits()
    );
}

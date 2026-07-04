//! BH-0 — GPU-resident SaturatingFlux operator tests.

use simthing_gpu::{
    cpu_compute_c_at, cpu_horizon, cpu_stencil_step, params_from_config, GpuContext,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, SATURATING_FLUX_CHI_CFL_MAX,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-0 tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn saturating_config(
    w: u32,
    h: u32,
    horizon: u32,
    u_sat: f32,
    chi: f32,
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
            choke_output_col: None,
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

fn total_mass(values: &[f32], w: u32, h: u32, col: u32, n_dims: u32) -> f32 {
    (0..w * h).map(|slot| values[idx(slot, col, n_dims)]).sum()
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

#[test]
fn bh0_saturating_flux_cpu_oracle_conserves_mass() {
    let config = saturating_config(4, 4, 16, 2.0, 0.25);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(0, 0, 4)] = 1.0;
    values[idx(1, 0, 4)] = 3.0;
    values[idx(5, 0, 4)] = 2.0;
    values[idx(10, 0, 4)] = 4.0;
    let initial = total_mass(&values, 4, 4, 0, 4);
    let mut cur = values;
    for _ in 0..16 {
        let next = cpu_stencil_step(&cur, &params);
        let sum = total_mass(&next, 4, 4, 0, 4);
        assert!(
            (sum - initial).abs() < 1e-4,
            "mass drift per step: initial={initial} current={sum}"
        );
        cur = next;
    }
}

#[test]
fn bh0_saturating_flux_gpu_matches_cpu_oracle() {
    with_gpu(|ctx| {
        for &(w, h) in &[(4u32, 4u32), (8u32, 8u32)] {
            for &horizon in &[1u32, 2u32, 4u32] {
                let config = saturating_config(w, h, horizon, 2.0, 0.25);
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

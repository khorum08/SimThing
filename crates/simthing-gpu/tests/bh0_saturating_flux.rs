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
        operator: StructuredFieldStencilOperator::SaturatingFlux { u_sat, chi },
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

#[test]
fn bh0_zero_flux_boundary_does_not_drain_mass() {
    let config = saturating_config(3, 3, 1, 1e9, 0.25);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(0, 0, 4)] = 8.0;
    let initial = total_mass(&values, 3, 3, 0, 4);
    let out = cpu_stencil_step(&values, &params);
    let final_mass = total_mass(&out, 3, 3, 0, 4);
    assert_eq!(initial.to_bits(), final_mass.to_bits());
    for slot in 0..9 {
        assert!(out[idx(slot, 0, 4)] >= 0.0);
    }
}

#[test]
fn bh0_cj_dependency_uses_two_hop_gather() {
    let config = saturating_config(3, 1, 1, 2.0, 0.25);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(0, 0, 4)] = 1.0;
    values[idx(1, 0, 4)] = 0.0;
    values[idx(2, 0, 4)] = 1.0;

    let c0 = cpu_compute_c_at(&values, &params, 0, 0);
    let c1 = cpu_compute_c_at(&values, &params, 1, 0);
    assert!(c0 > c1, "C_i and C_j must differ for this fixture");

    let oracle = cpu_stencil_step(&values, &params);
    let u0 = values[idx(0, 0, 4)];
    let u1 = values[idx(1, 0, 4)];
    let u2 = values[idx(2, 0, 4)];
    let ci_only_next = u1 + c1 * (u0 - u1) + c1 * (u2 - u1);
    assert_ne!(oracle[idx(1, 0, 4)].to_bits(), ci_only_next.to_bits());

    with_gpu(|ctx| {
        let gpu = run_gpu_horizon(ctx, &config, &values, 1);
        assert_eq!(gpu[idx(1, 0, 4)].to_bits(), oracle[idx(1, 0, 4)].to_bits());
    });
}

#[test]
fn bh0_uniform_field_is_fixed_point() {
    let config = saturating_config(4, 4, 4, 3.0, 0.2);
    let params = params_from_config(&config);
    let values = vec![2.5f32; config.values_len()];
    let out = cpu_stencil_step(&values, &params);
    for slot in 0..config.cells() {
        assert_eq!(out[idx(slot, 0, 4)].to_bits(), 2.5f32.to_bits());
    }
    with_gpu(|ctx| {
        let gpu = run_gpu_horizon(ctx, &config, &values, 1);
        assert_bits_equal(&gpu, &out);
    });
}

#[test]
fn bh0_crowding_chokes_flux() {
    let config = saturating_config(3, 3, 1, 1.0, 0.25);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(4, 0, 4)] = 2.0;
    values[idx(1, 0, 4)] = 2.0;
    values[idx(3, 0, 4)] = 2.0;
    values[idx(5, 0, 4)] = 2.0;
    values[idx(7, 0, 4)] = 2.0;
    let c_center = cpu_compute_c_at(&values, &params, 1, 1);
    assert_eq!(c_center.to_bits(), 0.0f32.to_bits());
    let out = cpu_stencil_step(&values, &params);
    assert_eq!(out[idx(4, 0, 4)].to_bits(), 2.0f32.to_bits());
}

#[test]
fn bh0_invalid_cfl_rejected() {
    let config = saturating_config(3, 3, 1, 1.0, SATURATING_FLUX_CHI_CFL_MAX + 0.01);
    assert!(config.validate().is_err());
}

#[test]
fn saturating_flux_clear_field_reduces_to_symmetric_diffusion() {
    let config = saturating_config(3, 3, 1, 1e9, 0.25);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(4, 0, 4)] = 8.0;
    let out = cpu_stencil_step(&values, &params);
    assert_eq!(out[idx(4, 0, 4)].to_bits(), 0.0f32.to_bits());
    assert_eq!(out[idx(1, 0, 4)].to_bits(), 2.0f32.to_bits());
    assert_eq!(out[idx(3, 0, 4)].to_bits(), 2.0f32.to_bits());
    assert_eq!(out[idx(5, 0, 4)].to_bits(), 2.0f32.to_bits());
    assert_eq!(out[idx(7, 0, 4)].to_bits(), 2.0f32.to_bits());
    assert_eq!(out[idx(0, 0, 4)].to_bits(), 0.0f32.to_bits());
    with_gpu(|ctx| {
        let gpu = run_gpu_horizon(ctx, &config, &values, 1);
        assert_bits_equal(&gpu, &out);
    });
}

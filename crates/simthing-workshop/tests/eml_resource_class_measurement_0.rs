//! Test/profiling-only matched measurement over the canonical field interpreter.

use std::time::Instant;

use simthing_core::EmlResourceClass;
use simthing_gpu::{
    compile_min_plus_field_sweep, compile_structured_field_sweeps, pack_w_and_initial_d,
    FieldSweepRegistration, FieldSweepSession, GpuContext, MinPlusStencilConfig, MinPlusStencilOp,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, MIN_PLUS_INF, SATURATING_FLUX_CHI_CFL_MAX,
};

const WARMUPS: usize = 4;
const SAMPLES: usize = 9;

fn bits_equal(left: &[f32], right: &[f32]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.to_bits() == right.to_bits())
}

fn synthetic_w(width: u32, height: u32) -> Vec<f32> {
    (0..width * height)
        .map(|index| 1.0 + (index % 7) as f32 * 0.125)
        .collect()
}

fn synthetic_flux_values(width: u32, height: u32, n_dims: u32) -> Vec<f32> {
    let mut values = vec![0.0; (width * height * n_dims) as usize];
    let center = ((height / 2) * width + width / 2) as usize * n_dims as usize;
    values[center] = 0.8;
    values
}

fn median(mut samples: Vec<f64>) -> f64 {
    samples.sort_by(f64::total_cmp);
    samples[samples.len() / 2]
}

fn time_field_session(
    ctx: &GpuContext,
    values: &[f32],
    registrations: &[FieldSweepRegistration],
    iterations: u32,
    resource_class: EmlResourceClass,
) -> (Vec<f64>, Vec<f32>) {
    let mut session = FieldSweepSession::new_with_profiling_resource_class(
        ctx,
        &registrations[0],
        resource_class,
    )
    .expect("profiling session");
    let run = |session: &mut FieldSweepSession| {
        session.upload_values(ctx, values).expect("upload");
        for registration in registrations {
            session
                .dispatch(ctx, registration, iterations)
                .expect("canonical dispatch");
        }
    };
    for _ in 0..WARMUPS {
        run(&mut session);
    }
    let mut samples = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let start = Instant::now();
        run(&mut session);
        samples.push(start.elapsed().as_secs_f64() * 1_000_000.0);
    }
    let output = session.readback(ctx).expect("readback");
    (samples, output)
}

#[test]
fn eml_resource_class_measurement_0_matched_palma_gu_yang() {
    let Ok(ctx) = GpuContext::new_blocking() else {
        if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() {
            panic!("required supported adapter unavailable");
        }
        return;
    };
    let adapter = ctx.adapter.get_info();
    if adapter.name != "NVIDIA GeForce RTX 4080 Laptop GPU"
        || adapter.backend != wgpu::Backend::Vulkan
    {
        if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() {
            panic!(
                "required NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan, got {} / {:?}",
                adapter.name, adapter.backend
            );
        }
        return;
    }

    let width = 16;
    let height = 16;
    let palma_iterations = 8;
    let w = synthetic_w(width, height);
    let palma_config = MinPlusStencilConfig {
        width,
        height,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x: 2,
        dest_y: 2,
        inf_sentinel: MIN_PLUS_INF,
    };
    let palma_values = pack_w_and_initial_d(&w, &palma_config).expect("PALMA values");
    let palma_registration = compile_min_plus_field_sweep(&palma_config).expect("PALMA EML");
    assert_eq!(
        palma_registration.resource_class(),
        EmlResourceClass::CompactStack4
    );
    let (palma_stack4, palma_stack4_output) = time_field_session(
        &ctx,
        &palma_values,
        std::slice::from_ref(&palma_registration),
        palma_iterations,
        EmlResourceClass::CompactStack4,
    );
    let (palma_stack32, palma_stack32_output) = time_field_session(
        &ctx,
        &palma_values,
        std::slice::from_ref(&palma_registration),
        palma_iterations,
        EmlResourceClass::LegacyFixed32,
    );
    assert!(bits_equal(&palma_stack4_output, &palma_stack32_output));

    let palma_bespoke = MinPlusStencilOp::new(&ctx, palma_config).expect("PALMA bespoke");
    for _ in 0..WARMUPS {
        palma_bespoke
            .upload_values(&ctx, &palma_values)
            .expect("upload");
        palma_bespoke
            .dispatch_ping_pong(&ctx, palma_iterations)
            .expect("dispatch");
    }
    let mut palma_bespoke_samples = Vec::new();
    for _ in 0..SAMPLES {
        let start = Instant::now();
        palma_bespoke
            .upload_values(&ctx, &palma_values)
            .expect("upload");
        palma_bespoke
            .dispatch_ping_pong(&ctx, palma_iterations)
            .expect("dispatch");
        palma_bespoke_samples.push(start.elapsed().as_secs_f64() * 1_000_000.0);
    }

    let (north, south, east, west) = StructuredFieldStencilConfig::zero_directional_weights();
    let gu_yang_config = StructuredFieldStencilConfig {
        width,
        height,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: north,
        weight_south: south,
        weight_east: east,
        weight_west: west,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 1.0,
            chi: SATURATING_FLUX_CHI_CFL_MAX,
            choke_output_col: None,
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Clamp,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    };
    let gu_yang_values = synthetic_flux_values(width, height, gu_yang_config.n_dims);
    let gu_yang_registrations =
        compile_structured_field_sweeps(&gu_yang_config).expect("Gu-Yang EML");
    assert!(gu_yang_registrations
        .iter()
        .all(|registration| registration.resource_class() == EmlResourceClass::CompactStack4));
    let (gu_yang_stack4, gu_yang_stack4_output) = time_field_session(
        &ctx,
        &gu_yang_values,
        &gu_yang_registrations,
        1,
        EmlResourceClass::CompactStack4,
    );
    let (gu_yang_stack32, gu_yang_stack32_output) = time_field_session(
        &ctx,
        &gu_yang_values,
        &gu_yang_registrations,
        1,
        EmlResourceClass::LegacyFixed32,
    );
    assert!(bits_equal(&gu_yang_stack4_output, &gu_yang_stack32_output));

    let gu_yang_bespoke =
        StructuredFieldStencilOp::new(&ctx, gu_yang_config).expect("Gu-Yang bespoke");
    for _ in 0..WARMUPS {
        gu_yang_bespoke
            .upload_values(&ctx, &gu_yang_values)
            .expect("upload");
        gu_yang_bespoke
            .dispatch_ping_pong(&ctx, 1)
            .expect("dispatch");
    }
    let mut gu_yang_bespoke_samples = Vec::new();
    for _ in 0..SAMPLES {
        let start = Instant::now();
        gu_yang_bespoke
            .upload_values(&ctx, &gu_yang_values)
            .expect("upload");
        gu_yang_bespoke
            .dispatch_ping_pong(&ctx, 1)
            .expect("dispatch");
        gu_yang_bespoke_samples.push(start.elapsed().as_secs_f64() * 1_000_000.0);
    }

    let palma_bespoke_median = median(palma_bespoke_samples.clone());
    let gu_yang_bespoke_median = median(gu_yang_bespoke_samples.clone());
    eprintln!(
        "EML_RC_MATCHED adapter={} backend={:?} case=PALMA class=stack4 median_us={:.3} worst_us={:.3} dispatches={} bespoke_median_us={:.3} median_ratio={:.4} worst_ratio={:.4}",
        adapter.name,
        adapter.backend,
        median(palma_stack4.clone()),
        palma_stack4.iter().copied().fold(0.0, f64::max),
        palma_registration.adjacency().degree_buckets().len() as u32 * palma_iterations,
        palma_bespoke_median,
        median(palma_stack4.clone()) / palma_bespoke_median,
        palma_stack4.iter().copied().fold(0.0, f64::max)
            / palma_bespoke_samples.iter().copied().fold(0.0, f64::max),
    );
    eprintln!(
        "EML_RC_MATCHED adapter={} backend={:?} case=PALMA class=stack32 median_us={:.3} worst_us={:.3} dispatches={} parity=bit-exact",
        adapter.name,
        adapter.backend,
        median(palma_stack32.clone()),
        palma_stack32.iter().copied().fold(0.0, f64::max),
        palma_registration.adjacency().degree_buckets().len() as u32 * palma_iterations,
    );
    eprintln!(
        "EML_RC_MATCHED adapter={} backend={:?} case=Gu-Yang class=stack4 median_us={:.3} worst_us={:.3} dispatches={} bespoke_median_us={:.3} median_ratio={:.4} worst_ratio={:.4}",
        adapter.name,
        adapter.backend,
        median(gu_yang_stack4.clone()),
        gu_yang_stack4.iter().copied().fold(0.0, f64::max),
        gu_yang_registrations
            .iter()
            .map(|registration| registration.adjacency().degree_buckets().len() as u32)
            .sum::<u32>(),
        gu_yang_bespoke_median,
        median(gu_yang_stack4.clone()) / gu_yang_bespoke_median,
        gu_yang_stack4.iter().copied().fold(0.0, f64::max)
            / gu_yang_bespoke_samples.iter().copied().fold(0.0, f64::max),
    );
    eprintln!(
        "EML_RC_MATCHED adapter={} backend={:?} case=Gu-Yang class=stack32 median_us={:.3} worst_us={:.3} dispatches={} parity=bit-exact",
        adapter.name,
        adapter.backend,
        median(gu_yang_stack32.clone()),
        gu_yang_stack32.iter().copied().fold(0.0, f64::max),
        gu_yang_registrations
            .iter()
            .map(|registration| registration.adjacency().degree_buckets().len() as u32)
            .sum::<u32>(),
    );
}

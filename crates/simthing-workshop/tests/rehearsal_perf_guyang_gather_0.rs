//! Current admitted Gu-Yang versus retained tiled implementation. Synthetic N4
//! sparse-pulse profile from the landed N4 parity referee; no timing-based verdict.

use simthing_gpu::{
    compile_structured_field_sweeps, cpu_horizon, execute_field_sweep_cpu_chain,
    params_from_config, FieldSweepSession, GpuContext, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
    SATURATING_FLUX_CHI_CFL_MAX,
};
use simthing_workshop::rehearsal_perf_guyang_gather_0::distribution;
use simthing_workshop::rehearsal_perf_tr_composite_0::GpuTimeline;
use std::time::Instant;

const WARMUP: usize = 3;
const REPETITIONS: usize = 7;

fn config(width: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width,
        height: width,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
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
    }
}

fn values(width: u32) -> Vec<f32> {
    let mut v = vec![0.0; (width * width * 4) as usize];
    for slot in 0..(width * width) as usize {
        for column in 1..4 {
            v[slot * 4 + column] = slot as f32 * 0.03125 + column as f32 * 7.0;
        }
    }
    v[(width / 2 * width + width / 2) as usize * 4] = 0.8;
    v
}

fn equal(a: &[f32], b: &[f32]) {
    assert_eq!(a.len(), b.len());
    for (i, (a, b)) in a.iter().zip(b).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "field-law parity cell {i}: {a} versus {b}"
        );
    }
}

#[test]
fn current_gather_and_retained_tiled_have_exact_field_parity_and_paired_samples() {
    let startup = Instant::now();
    let ctx = GpuContext::new_blocking().expect("REFERENCE-ADAPTER-UNAVAILABLE");
    let info = ctx.adapter.get_info();
    assert_eq!(
        info.name, "NVIDIA GeForce RTX 4080 Laptop GPU",
        "reference adapter changed"
    );
    println!(
        "M1_ADAPTER {:?} startup_ns={} timestamp_period_ns={} build_debug_assertions={}",
        info,
        startup.elapsed().as_nanos(),
        ctx.queue.get_timestamp_period(),
        cfg!(debug_assertions)
    );
    for width in [32, 128, 256] {
        let setup = Instant::now();
        let cfg = config(width);
        let input = values(width);
        let regs = compile_structured_field_sweeps(&cfg).expect("current field admission");
        assert_eq!(regs.len(), 2);
        let expected = cpu_horizon(&input, &params_from_config(&cfg), 1);
        equal(
            &expected,
            &execute_field_sweep_cpu_chain(&input, &regs).expect("current CPU oracle"),
        );
        let tiled_params_bytes = std::mem::size_of_val(&params_from_config(&cfg));
        let tiled = StructuredFieldStencilOp::new(&ctx, cfg).expect("retained tiled reference");
        let mut gather = FieldSweepSession::new(&ctx, &regs[0]).expect("current admitted gather");
        let setup_ns = setup.elapsed().as_nanos();
        let mut samples = [Vec::new(), Vec::new()];
        for repetition in 0..WARMUP + REPETITIONS {
            // Alternate arm order; the authored input is unchanged and uploaded afresh.
            let mut paired = [serde_json::Value::Null, serde_json::Value::Null];
            for arm in if repetition % 2 == 0 { [0, 1] } else { [1, 0] } {
                let timeline = GpuTimeline::new(&ctx.device, 2).expect("timestamp seam");
                let upload = Instant::now();
                if arm == 0 {
                    gather.upload_values(&ctx, &input).unwrap();
                } else {
                    tiled.upload_values(&ctx, &input).unwrap();
                }
                let upload_ns = upload.elapsed().as_nanos();
                timeline.mark(&ctx.device, &ctx.queue, 0);
                let launch = Instant::now();
                let before = gather.registration_dispatches();
                if arm == 0 {
                    gather.dispatch_chain(&ctx, &regs, 1).unwrap();
                } else {
                    assert_eq!(tiled.dispatch_ping_pong(&ctx, 1).unwrap(), 1);
                }
                let api_ns = launch.elapsed().as_nanos();
                if arm == 0 {
                    assert_eq!(gather.registration_dispatches() - before, 2);
                }
                timeline.mark(&ctx.device, &ctx.queue, 1);
                let egress = Instant::now();
                let times = timeline.read_ns(&ctx.device, &ctx.queue).unwrap();
                let timestamp_egress_ns = egress.elapsed().as_nanos();
                let readback = Instant::now();
                let result = if arm == 0 {
                    gather.readback(&ctx).unwrap()
                } else {
                    tiled.readback_after_ping_pong(&ctx, 1)
                };
                let values_readback_ns = readback.elapsed().as_nanos();
                equal(&expected, &result);
                paired[arm] = serde_json::json!({"gpu_queue_interval_ns": times[1], "upload_host_ns": upload_ns,
                    "dispatch_api_ns": api_ns, "timestamp_egress_ns": timestamp_egress_ns,
                    "values_readback_ns": values_readback_ns,
                    "api_minus_gpu_ns": api_ns as f64 - times[1]});
            }
            println!(
                "M1_RAW {}",
                serde_json::json!({"width": width, "phase": if repetition < WARMUP {"warmup"} else {"sample"}, "repetition": repetition, "arms_gather_tiled": paired})
            );
            if repetition >= WARMUP {
                for arm in 0..2 {
                    samples[arm].push(paired[arm]["gpu_queue_interval_ns"].as_f64().unwrap());
                }
            }
        }
        let paired_residual = samples[0]
            .iter()
            .zip(&samples[1])
            .map(|(g, t)| g - t)
            .collect();
        println!(
            "M1_SUMMARY {}",
            serde_json::json!({"width": width, "cells": width*width,
            "directed_eligibility_edges": 4*width*(width-1), "dimensions": 4, "source_fields": 1,
            "admitted_registrations": 2, "physical_dispatches_per_arm": [1,1], "hops": 1,
            "upload_bytes_per_arm": input.len()*4, "readback_bytes_per_arm": input.len()*4,
            "setup_ns": setup_ns, "warmup": WARMUP, "repetitions": REPETITIONS,
            "seed": 0, "source_policy": "deterministic inherited sparse pulse; no RNG",
            "simulation_buffer_inventory_bytes": {
                "tiled_persistent": 2*input.len()*4 + (width*width) as usize*4 + tiled_params_bytes,
                "gather_persistent_source_derived": 2*input.len()*4 + (width*width) as usize*(4+8+4) + (4*width*(width-1)) as usize*std::mem::size_of::<simthing_gpu::AccumulatorInputGpu>() + 3*regs[0].resource_class().max_tree_nodes() as usize*std::mem::size_of::<simthing_core::EmlNodeGpu>(),
                "per_arm_proof_readback": input.len()*4, "timestamp_resolve_and_readback": 32,
                "presentation": 0
            },
            "memory_limit": "buffer inventory, excludes opaque driver allocations and transient bind/uniform objects; hardware memory traffic counters unavailable",
            "gather": distribution(samples[0].clone()), "tiled": distribution(samples[1].clone()),
            "paired_gather_minus_tiled": distribution(paired_residual)})
        );
    }
}

#[test]
fn signed_samplewise_residuals_remain_negative_before_aggregation() {
    let d = distribution(vec![-9.0, 2.0, -3.0]);
    assert_eq!(d.raw_ns, [-9.0, 2.0, -3.0]);
    assert_eq!(d.median_ns, -3.0);
    assert_eq!(d.min_ns, -9.0);
}

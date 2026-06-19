//! SIM-MAPPING-ATLAS-SCHEDULER-0 — sim-owned multi-theater mapping scheduler proofs.

mod support;

use std::sync::Mutex;

use simthing_gpu::{
    cpu_w_impedance_compose_oracle, debug_readback_allowed, max_d_field_error, GpuContext,
    MinPlusStencilConfig, StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, WImpedanceComposeConfig, WImpedanceComposeProfile,
    MIN_PLUS_INF,
};
use simthing_sim::{
    cpu_min_plus_d_from_composed_interleaved, cpu_structured_field_horizon, gpu_context_blocking,
    CompiledMappingAtlas, CompiledMappingPlan, CompiledMappingStep, MappingAtlasTickInputs,
    MappingTheaterSlot, MappingTickInputs, SimGpuMappingAtlasScheduler,
    SimGpuMappingReadbackPolicy, SimTickError,
};

use support::readback_gate::with_isolated_readback_gate_test;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_ATLAS_TOKENS: &[&str] = &[
    "pathfinding",
    "predecessor",
    "came_from",
    "route_object",
    "movement_order",
    "border_service",
    "frontline_service",
    "cpu_planner",
    "semantic_wgsl",
];

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn saturating_flux_config(w: u32, h: u32, hops: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon: hops,
        alpha_self: 1.0,
        gamma_neighbor: 0.0,
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

fn w_compose_config(w: u32, h: u32) -> WImpedanceComposeConfig {
    WImpedanceComposeConfig {
        width: w,
        height: h,
        n_dims: 5,
        base_w_col: 0,
        choke_a_col: 1,
        choke_b_col: 2,
        profiles: vec![WImpedanceComposeProfile {
            weight_a: 1.0,
            weight_b: 0.25,
            output_w_col: 3,
        }],
    }
}

fn min_plus_config(w: u32, h: u32, dest: (u32, u32)) -> MinPlusStencilConfig {
    MinPlusStencilConfig {
        width: w,
        height: h,
        n_dims: 5,
        d_col: 4,
        w_col: 3,
        dest_x: dest.0,
        dest_y: dest.1,
        inf_sentinel: MIN_PLUS_INF,
    }
}

fn structured_field_only_plan() -> (CompiledMappingPlan, StructuredFieldStencilConfig, Vec<f32>) {
    let config = saturating_flux_config(3, 3, 1);
    let plan = CompiledMappingPlan {
        steps: vec![CompiledMappingStep::StructuredFieldStencil {
            config: config.clone(),
            hops: 1,
            interleaved_column_writes: Vec::new(),
        }],
        interleaved_width: 0,
        interleaved_height: 0,
        interleaved_n_dims: 0,
    };
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(0, 0, config.n_dims)] = 80.0;
    values[idx(1, 0, config.n_dims)] = 20.0;
    (plan, config, values)
}

fn w_compose_min_plus_plan() -> (CompiledMappingPlan, WImpedanceComposeConfig, Vec<f32>) {
    let compose = w_compose_config(5, 5);
    let stencil = min_plus_config(5, 5, (0, 0));
    let plan = CompiledMappingPlan {
        steps: vec![
            CompiledMappingStep::WImpedanceCompose {
                config: compose.clone(),
            },
            CompiledMappingStep::MinPlusStencil {
                config: stencil,
                iterations: 4,
            },
        ],
        interleaved_width: compose.width,
        interleaved_height: compose.height,
        interleaved_n_dims: compose.n_dims,
    };
    let len = (compose.width * compose.height * compose.n_dims) as usize;
    let mut interleaved = vec![0.0f32; len];
    for slot in 0..(compose.width * compose.height) {
        interleaved[idx(slot, compose.base_w_col, compose.n_dims)] = 1.0;
        interleaved[idx(slot, compose.choke_a_col, compose.n_dims)] = 0.5;
        interleaved[idx(slot, compose.choke_b_col, compose.n_dims)] = 0.0;
    }
    (plan, compose, interleaved)
}

fn two_theater_atlas() -> (
    CompiledMappingAtlas,
    StructuredFieldStencilConfig,
    Vec<f32>,
    WImpedanceComposeConfig,
    Vec<f32>,
) {
    let (plan0, config0, values0) = structured_field_only_plan();
    let (plan1, compose1, interleaved1) = w_compose_min_plus_plan();
    let atlas = CompiledMappingAtlas {
        plans: vec![plan0, plan1],
    };
    (atlas, config0, values0, compose1, interleaved1)
}

#[test]
fn mapping_atlas_scheduler_none_returns_no_proof_values_for_all_theaters() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (atlas, _, values0, _, interleaved1) = two_theater_atlas();
        let mut scheduler = SimGpuMappingAtlasScheduler::new(&ctx, atlas).expect("scheduler");
        let output = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        assert_eq!(output.theater_outputs.len(), 2);
        for theater in &output.theater_outputs {
            assert!(theater.proof_values.is_none());
        }
        assert!(!debug_readback_allowed());
        assert_eq!(scheduler.resident_tick_count(), 1);
        eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: none_no_readback_all_theaters");
    });
}

#[test]
fn mapping_atlas_scheduler_proof_returns_per_theater_outputs() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (atlas, config0, values0, compose1, interleaved1) = two_theater_atlas();
        let cpu0 = cpu_structured_field_horizon(&values0, &config0, 1);
        let mut cpu_interleaved = interleaved1.clone();
        cpu_interleaved = cpu_w_impedance_compose_oracle(&cpu_interleaved, &compose1);
        let stencil = match &atlas.plans[1].steps[1] {
            CompiledMappingStep::MinPlusStencil { config, .. } => config.clone(),
            _ => panic!("expected min plus"),
        };
        let cpu_d =
            cpu_min_plus_d_from_composed_interleaved(&cpu_interleaved, &stencil, 4).expect("cpu d");

        let mut scheduler = SimGpuMappingAtlasScheduler::new(&ctx, atlas).expect("scheduler");
        let output = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert_eq!(output.theater_outputs.len(), 2);
        let gpu0 = output.theater_outputs[0]
            .proof_values
            .as_ref()
            .expect("theater 0 proof");
        assert_eq!(gpu0.len(), cpu0.len());
        for (i, (g, c)) in gpu0.iter().zip(cpu0.iter()).enumerate() {
            assert!((g - c).abs() < 1e-4, "theater 0 mismatch at {i}");
        }
        let gpu_d = output.theater_outputs[1]
            .proof_values
            .as_ref()
            .expect("theater 1 proof");
        assert!(max_d_field_error(&cpu_d, gpu_d) < 1e-4);
        assert!(!debug_readback_allowed());
        eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: REAL_ADAPTER_OBSERVED (proof_per_theater)");
    });
}

#[test]
fn mapping_atlas_scheduler_proof_then_none_does_not_leak_readback() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (atlas, _, values0, _, interleaved1) = two_theater_atlas();
        let mut scheduler = SimGpuMappingAtlasScheduler::new(&ctx, atlas).expect("scheduler");
        let proof = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0.clone()],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert!(proof.theater_outputs[0].proof_values.is_some());
        assert!(proof.theater_outputs[1].proof_values.is_some());
        assert!(!debug_readback_allowed());

        let none = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        for theater in &none.theater_outputs {
            assert!(theater.proof_values.is_none());
        }
        assert!(!debug_readback_allowed());
        assert_eq!(scheduler.resident_tick_count(), 2);
        eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: REAL_ADAPTER_OBSERVED (proof_then_none)");
    });
}

#[test]
fn mapping_atlas_scheduler_none_then_proof_then_none_does_not_leak_readback() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (atlas, _, values0, _, interleaved1) = two_theater_atlas();
        let mut scheduler = SimGpuMappingAtlasScheduler::new(&ctx, atlas).expect("scheduler");

        let none1 = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0.clone()],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1.clone()),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick 1");
        for theater in &none1.theater_outputs {
            assert!(theater.proof_values.is_none());
        }
        assert!(!debug_readback_allowed());

        let proof = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0.clone()],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert!(proof.theater_outputs[0].proof_values.is_some());
        assert!(proof.theater_outputs[1].proof_values.is_some());
        assert!(!debug_readback_allowed());

        let none2 = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick 2");
        for theater in &none2.theater_outputs {
            assert!(theater.proof_values.is_none());
        }
        assert!(!debug_readback_allowed());
        assert_eq!(scheduler.resident_tick_count(), 3);
        eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: REAL_ADAPTER_OBSERVED (none_proof_none)");
    });
}

#[test]
fn mapping_atlas_scheduler_reuses_resident_states_across_two_ticks() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (atlas, _, values0, _, interleaved1) = two_theater_atlas();
        let mut scheduler = SimGpuMappingAtlasScheduler::new(&ctx, atlas).expect("scheduler");
        let ptr_first = std::ptr::from_ref(&scheduler) as usize;

        scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0.clone()],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1.clone()),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("tick 1");

        let ptr_second = std::ptr::from_ref(&scheduler) as usize;
        assert_eq!(ptr_first, ptr_second, "scheduler must not be reallocated");

        scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        MappingTickInputs {
                            structured_field_values: &[values0],
                            interleaved_values: None,
                        },
                        MappingTickInputs {
                            structured_field_values: &[],
                            interleaved_values: Some(&interleaved1),
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("tick 2");

        assert_eq!(scheduler.resident_tick_count(), 2);
        assert_eq!(
            scheduler
                .theater_resident_tick_count(MappingTheaterSlot(0))
                .expect("slot 0"),
            2
        );
        assert_eq!(
            scheduler
                .theater_resident_tick_count(MappingTheaterSlot(1))
                .expect("slot 1"),
            2
        );
    });
}

#[test]
fn mapping_atlas_scheduler_rejects_input_count_mismatch() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (atlas, _, values0, _, _) = two_theater_atlas();
        let mut scheduler = SimGpuMappingAtlasScheduler::new(&ctx, atlas).expect("scheduler");
        let err = scheduler
            .tick(
                &ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![MappingTickInputs {
                        structured_field_values: &[values0],
                        interleaved_values: None,
                    }],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect_err("mismatch");
        assert!(matches!(err, SimTickError::InvalidInputLength { .. }));
    });
}

#[test]
fn mapping_atlas_scheduler_forbidden_token_guard() {
    let source = include_str!("../src/mapping_atlas_scheduler.rs");
    for token in FORBIDDEN_ATLAS_TOKENS {
        assert!(
            !source.contains(token),
            "mapping_atlas_scheduler must not contain `{token}`"
        );
    }
    assert!(
        source.contains("state.tick(ctx, theater_input, readback)"),
        "scheduler must delegate readback policy to resident tick state"
    );
    assert!(
        !source.contains("readback_after_ping_pong"),
        "scheduler must not call proof readback helpers directly"
    );
}

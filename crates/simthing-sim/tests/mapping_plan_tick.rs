//! SIM-MAPPING-PLAN-TICK-SEAM-0 — sim-owned resident mapping tick proofs.

use std::sync::Mutex;

use simthing_gpu::{
    cpu_w_impedance_compose_oracle, max_d_field_error, GpuContext, MinPlusStencilConfig,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, WImpedanceComposeConfig, WImpedanceComposeProfile,
    MIN_PLUS_INF,
};
use simthing_sim::{
    cpu_min_plus_d_from_composed_interleaved, cpu_structured_field_horizon, gpu_context_blocking,
    CompiledMappingPlan, CompiledMappingStep, MappingTickInputs, SimGpuMappingReadbackPolicy,
    SimGpuMappingTickState, SimTickError,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_MAPPING_TOKENS: &[&str] = &[
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

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = gpu_context_blocking().expect("GPU required for mapping plan tick proof");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

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

fn seed_structured_values(config: &StructuredFieldStencilConfig) -> Vec<f32> {
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(0, 0, config.n_dims)] = 80.0;
    values[idx(1, 0, config.n_dims)] = 20.0;
    values
}

fn seed_interleaved_values(compose: &WImpedanceComposeConfig) -> Vec<f32> {
    let len = (compose.width * compose.height * compose.n_dims) as usize;
    let mut values = vec![0.0f32; len];
    for slot in 0..(compose.width * compose.height) {
        values[idx(slot, compose.base_w_col, compose.n_dims)] = 1.0;
        values[idx(slot, compose.choke_a_col, compose.n_dims)] = 0.5;
        values[idx(slot, compose.choke_b_col, compose.n_dims)] = 0.0;
    }
    values
}

#[test]
fn mapping_plan_tick_structured_field_none_returns_no_readback() {
    with_gpu(|ctx| {
        let config = saturating_flux_config(3, 3, 2);
        let plan = CompiledMappingPlan {
            steps: vec![CompiledMappingStep::StructuredFieldStencil {
                config: config.clone(),
                hops: 2,
                interleaved_column_writes: Vec::new(),
            }],
            interleaved_width: 0,
            interleaved_height: 0,
            interleaved_n_dims: 0,
        };
        let values = seed_structured_values(&config);
        let mut state = SimGpuMappingTickState::new(ctx, plan).expect("state");
        let output = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[values],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("tick");
        assert!(output.proof_values.is_none());
        assert_eq!(state.resident_tick_count(), 1);
        eprintln!("SIM-MAPPING-PLAN-TICK-SEAM-0: structured_field_none_no_readback");
    });
}

#[test]
fn mapping_plan_tick_structured_field_proof_matches_cpu_oracle() {
    with_gpu(|ctx| {
        let config = saturating_flux_config(3, 3, 2);
        let plan = CompiledMappingPlan {
            steps: vec![CompiledMappingStep::StructuredFieldStencil {
                config: config.clone(),
                hops: 2,
                interleaved_column_writes: Vec::new(),
            }],
            interleaved_width: 0,
            interleaved_height: 0,
            interleaved_n_dims: 0,
        };
        let values = seed_structured_values(&config);
        let cpu = cpu_structured_field_horizon(&values, &config, 2);
        let mut state = SimGpuMappingTickState::new(ctx, plan).expect("state");
        let output = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[values],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("tick");
        let gpu = output.proof_values.expect("proof readback");
        assert_eq!(gpu.len(), cpu.len());
        for (i, (g, c)) in gpu.iter().zip(cpu.iter()).enumerate() {
            assert!((g - c).abs() < 1e-4, "mismatch at {i}: gpu={g} cpu={c}");
        }
        eprintln!("SIM-MAPPING-PLAN-TICK-SEAM-0: REAL_ADAPTER_OBSERVED (structured_field)");
    });
}

#[test]
fn mapping_plan_tick_w_compose_min_plus_proof_matches_cpu_oracle() {
    with_gpu(|ctx| {
        let compose = w_compose_config(5, 5);
        let stencil = min_plus_config(5, 5, (0, 0));
        let plan = CompiledMappingPlan {
            steps: vec![
                CompiledMappingStep::WImpedanceCompose {
                    config: compose.clone(),
                },
                CompiledMappingStep::MinPlusStencil {
                    config: stencil.clone(),
                    iterations: 8,
                },
            ],
            interleaved_width: compose.width,
            interleaved_height: compose.height,
            interleaved_n_dims: compose.n_dims,
        };
        let interleaved = seed_interleaved_values(&compose);
        let mut cpu_interleaved = interleaved.clone();
        cpu_interleaved = cpu_w_impedance_compose_oracle(&cpu_interleaved, &compose);
        let cpu_d =
            cpu_min_plus_d_from_composed_interleaved(&cpu_interleaved, &stencil, 8).expect("cpu d");

        let mut state = SimGpuMappingTickState::new(ctx, plan).expect("state");
        let output = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[],
                    interleaved_values: Some(&interleaved),
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("tick");
        let gpu_d = output.proof_values.expect("proof d");
        assert!(max_d_field_error(&cpu_d, &gpu_d) < 1e-4);
        eprintln!("SIM-MAPPING-PLAN-TICK-SEAM-0: REAL_ADAPTER_OBSERVED (w_compose_min_plus)");
    });
}

#[test]
fn mapping_plan_tick_proof_then_none_does_not_leak_readback() {
    with_gpu(|ctx| {
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
        let values = seed_structured_values(&config);
        let mut state = SimGpuMappingTickState::new(ctx, plan).expect("state");
        let proof = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[values.clone()],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert!(proof.proof_values.is_some());

        let none = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[values],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        assert!(none.proof_values.is_none());
        assert_eq!(state.resident_tick_count(), 2);
    });
}

#[test]
fn mapping_plan_tick_reuses_resident_state_across_two_ticks() {
    with_gpu(|ctx| {
        let config = saturating_flux_config(3, 3, 1);
        let plan = CompiledMappingPlan {
            steps: vec![CompiledMappingStep::StructuredFieldStencil {
                config,
                hops: 1,
                interleaved_column_writes: Vec::new(),
            }],
            interleaved_width: 0,
            interleaved_height: 0,
            interleaved_n_dims: 0,
        };
        let values = seed_structured_values(&saturating_flux_config(3, 3, 1));
        let mut state = SimGpuMappingTickState::new(ctx, plan.clone()).expect("state");
        let ptr_first = std::ptr::from_ref(&state) as usize;
        state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[values.clone()],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("tick 1");
        let ptr_second = std::ptr::from_ref(&state) as usize;
        assert_eq!(ptr_first, ptr_second, "state must not be reallocated");
        state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[values],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("tick 2");
        assert_eq!(state.resident_tick_count(), 2);
        assert_eq!(state.plan().steps.len(), plan.steps.len());
    });
}

#[test]
fn mapping_plan_tick_rejects_empty_plan() {
    with_gpu(|ctx| {
        let plan = CompiledMappingPlan {
            steps: Vec::new(),
            interleaved_width: 0,
            interleaved_height: 0,
            interleaved_n_dims: 0,
        };
        let err = match SimGpuMappingTickState::new(ctx, plan) {
            Ok(_) => panic!("empty plan must fail"),
            Err(err) => err,
        };
        assert!(matches!(err, SimTickError::Readback(_)));
    });
}

#[test]
fn mapping_plan_tick_source_forbidden_token_guard() {
    let source = include_str!("../src/mapping_plan_tick.rs");
    for token in FORBIDDEN_MAPPING_TOKENS {
        assert!(
            !source.contains(token),
            "mapping_plan_tick must not contain `{token}`"
        );
    }
}

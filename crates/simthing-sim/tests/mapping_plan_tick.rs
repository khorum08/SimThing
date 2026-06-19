//! SIM-MAPPING-PLAN-TICK-SEAM-0 + SIM-MAPPING-READBACK-POLICY-HARDEN-0 —
//! sim-owned resident mapping tick proofs and None/ProofReadback discipline.

mod support;

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use simthing_gpu::{
    cpu_w_impedance_compose_oracle, debug_readback_allowed, max_d_field_error,
    scoped_debug_readback_allowed, GpuContext, MinPlusStencilConfig,
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

use support::readback_gate::with_isolated_readback_gate_test;

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
    let interleaved = seed_interleaved_values(&compose);
    (plan, compose, interleaved)
}

#[test]
fn mapping_tick_none_never_returns_proof_values_for_w_compose_min_plus() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (plan, _, interleaved) = w_compose_min_plus_plan();
        let mut state = SimGpuMappingTickState::new(&ctx, plan).expect("state");
        let output = state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[],
                    interleaved_values: Some(&interleaved),
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        assert!(output.proof_values.is_none());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 1);
        eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: w_compose_min_plus_none_no_readback");
    });
}

#[test]
fn mapping_tick_none_never_returns_proof_values_for_structured_field() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        let mut state = SimGpuMappingTickState::new(&ctx, plan).expect("state");
        let output = state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        assert!(output.proof_values.is_none());
        assert!(!debug_readback_allowed());
        eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: structured_field_none_no_readback");
    });
}

#[test]
fn mapping_tick_none_then_proof_then_none_does_not_leak_readback() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        let mut state = SimGpuMappingTickState::new(&ctx, plan).expect("state");

        let none1 = state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values.clone()],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick 1");
        assert!(none1.proof_values.is_none());
        assert!(!debug_readback_allowed());

        let proof = state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values.clone()],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert!(proof.proof_values.is_some());
        assert!(!debug_readback_allowed());

        let none2 = state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick 2");
        assert!(none2.proof_values.is_none());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 3);
        eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: REAL_ADAPTER_OBSERVED (none_proof_none)");
    });
}

#[test]
fn mapping_tick_proof_then_none_does_not_leak_readback() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        let mut state = SimGpuMappingTickState::new(&ctx, plan).expect("state");
        let proof = state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values.clone()],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert!(proof.proof_values.is_some());
        assert!(!debug_readback_allowed());

        let none = state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        assert!(none.proof_values.is_none());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 2);
        eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: REAL_ADAPTER_OBSERVED (proof_then_none)");
    });
}

#[test]
fn mapping_tick_resident_reuse_preserves_readback_policy() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        let mut state = SimGpuMappingTickState::new(&ctx, plan.clone()).expect("state");
        let ptr_first = std::ptr::from_ref(&state) as usize;

        assert!(state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values.clone()],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick")
            .proof_values
            .is_some());
        assert!(!debug_readback_allowed());

        assert!(state
            .tick(
                &ctx,
                MappingTickInputs {
                    structured_field_values: &[values.clone()],
                    interleaved_values: None,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick")
            .proof_values
            .is_none());
        assert!(!debug_readback_allowed());

        let ptr_second = std::ptr::from_ref(&state) as usize;
        assert_eq!(ptr_first, ptr_second, "state must not be reallocated");
        assert_eq!(state.resident_tick_count(), 2);
        assert_eq!(state.plan().steps.len(), plan.steps.len());
    });
}

#[test]
fn mapping_tick_error_does_not_leave_readback_enabled_if_guard_exists() {
    with_isolated_readback_gate_test(|| {
        let result: Result<(), SimTickError> = (|| {
            let _guard = scoped_debug_readback_allowed(true);
            assert!(debug_readback_allowed());
            Err(SimTickError::Readback(
                "simulated mapping readback failure".into(),
            ))
        })();
        assert!(result.is_err());
        assert!(!debug_readback_allowed());
    });
}

#[test]
fn mapping_tick_panic_restores_readback_guard_if_guard_exists() {
    with_isolated_readback_gate_test(|| {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _guard = scoped_debug_readback_allowed(true);
            assert!(debug_readback_allowed());
            panic!("simulated mapping readback panic");
        }));
        assert!(result.is_err());
        assert!(!debug_readback_allowed());
    });
}

fn mapping_tick_body_source() -> &'static str {
    let source = include_str!("../src/mapping_plan_tick.rs");
    let start = source.find("pub fn tick(").expect("tick function");
    let end = source[start..]
        .find("\nfn structured_field_values_buffer")
        .expect("tick body end");
    &source[start..start + end]
}

#[test]
fn mapping_plan_tick_does_not_silently_enable_debug_readback() {
    let tick_body = mapping_tick_body_source();
    assert!(
        !tick_body.contains("set_debug_readback_allowed"),
        "mapping tick must not silently enable debug readback"
    );
    let source = include_str!("../src/mapping_plan_tick.rs");
    assert!(source.contains("run_with_proof_readback_enabled"));
    assert!(source.contains("scoped_debug_readback_allowed"));
}

#[test]
fn mapping_plan_tick_none_branch_readback_policy_source_guard() {
    let source = include_str!("../src/mapping_plan_tick.rs");
    assert!(
        source.contains("SimGpuMappingReadbackPolicy::None => None"),
        "None policy must not populate proof_values"
    );
    assert!(
        source.contains("MinPlusTraversalExecutionMode::GpuResident"),
        "MinPlus None must use GpuResident"
    );
    assert!(
        source.contains("MinPlusTraversalExecutionMode::DiagnosticReadback"),
        "MinPlus ProofReadback must use DiagnosticReadback"
    );
    let tick_body = mapping_tick_body_source();
    assert!(
        !tick_body.contains("readback_after_ping_pong")
            || tick_body.contains("run_with_proof_readback_enabled"),
        "structured-field readback must be scoped to proof helper"
    );
    assert!(
        tick_body.contains("if readback == SimGpuMappingReadbackPolicy::ProofReadback"),
        "proof readback must be explicit"
    );
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

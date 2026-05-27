//! E-11 controlled flat-star burn-in scenario fixtures (default-off flag path).

mod support;

use simthing_driver::{
    build_execution_plan, plan_arena_allocation, run_flat_star_burn_in,
    ResourceFlowScenarioBurnInReport,
};
use simthing_sim::PipelineFlags;

use support::e11_burn_in_scenarios::{
    open_scenario_session, run_scenario_burn_in, small_flat_star_equal_weights,
    small_flat_star_repeated_boundary_sync, small_flat_star_skewed_weights,
    small_flat_star_zero_weights, assert_no_nan_in_leaf_allocated, scenario_cell_inputs,
};
use support::e11_flat_star::{leaf_slots, try_gpu};

const TICKS: u32 = 100;

fn run_named_scenario(
    fixture: &support::e11_burn_in_scenarios::BurnInScenarioFixture,
) -> ResourceFlowScenarioBurnInReport {
    let mut fx = open_scenario_session(fixture);
    run_scenario_burn_in(&mut fx, fixture, TICKS)
}

#[test]
fn e11_burn_in_equal_weights_100_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = small_flat_star_equal_weights();
    let report = run_named_scenario(&fixture);

    assert_eq!(report.scenario_name, fixture.name);
    assert_eq!(report.ticks_checked, TICKS);
    assert!(report.replay_bit_exact);
    assert_eq!(report.max_abs_error.to_bits(), 0.0_f32.to_bits());
    assert_eq!(report.arenas_planned, 1);
    assert!(report.total_ops > 0);
    assert!(report.n_bands >= 5);
}

#[test]
fn e11_burn_in_skewed_weights_100_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = small_flat_star_skewed_weights();
    let report = run_named_scenario(&fixture);

    assert_eq!(report.scenario_name, fixture.name);
    assert_eq!(report.ticks_checked, TICKS);
    assert!(report.replay_bit_exact);
    assert_eq!(report.max_abs_error.to_bits(), 0.0_f32.to_bits());
}

#[test]
fn e11_burn_in_zero_weights_100_ticks_no_nan() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = small_flat_star_zero_weights();
    let mut fx = open_scenario_session(&fixture);
    let report = run_scenario_burn_in(&mut fx, &fixture, TICKS);

    assert_eq!(report.ticks_checked, TICKS);
    assert!(report.replay_bit_exact);
    assert_eq!(report.max_abs_error.to_bits(), 0.0_f32.to_bits());

    let n_dims = fx.session.proto.registry.total_columns as u32;
    assert_no_nan_in_leaf_allocated(&fx.session.state, &fx.layout, fx.cols, n_dims);
}

#[test]
fn e11_burn_in_repeated_sync_upload_is_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = small_flat_star_repeated_boundary_sync();
    let mut fx = open_scenario_session(&fixture);

    let mut ops_samples = Vec::new();
    let mut band_samples = Vec::new();
    for _ in 0..4 {
        fx.session
            .sync_resource_flow_if_enabled()
            .expect("repeated boundary sync");
        ops_samples.push(
            fx.session
                .state
                .accumulator_runtime
                .as_ref()
                .unwrap()
                .resource_flow_ops
                .count,
        );
        band_samples.push(fx.session.state.accumulator_resource_flow_bands);
    }

    assert!(
        ops_samples.windows(2).all(|w| w[0] == w[1]),
        "op count must stay stable across repeated sync: {ops_samples:?}"
    );
    assert!(
        band_samples.windows(2).all(|w| w[0] == w[1]),
        "n_bands must stay stable across repeated sync: {band_samples:?}"
    );

    let leaves = leaf_slots(&fx.layout);
    let inputs = scenario_cell_inputs(&fixture, &fx.layout, fx.cols);
    let n_dims = fx.session.proto.registry.total_columns as u32;
    let n_bands = fx.session.state.accumulator_resource_flow_bands;

    let burn = run_flat_star_burn_in(
        &mut fx.session.state,
        &fx.layout,
        fx.cols,
        n_dims,
        &inputs,
        &leaves,
        n_bands,
        10,
        fx.session.scenario.dt,
    );

    assert_eq!(burn.max_abs_error.to_bits(), 0.0_f32.to_bits());
    assert_eq!(burn.ticks_checked, 10);
    assert_eq!(ops_samples[0], ops_samples[3]);
    assert_eq!(band_samples[0], band_samples[3]);
}

#[test]
fn e11_burn_in_flag_remains_default_false() {
    assert!(
        !PipelineFlags::default().use_accumulator_resource_flow,
        "controlled burn-in scenarios must not flip default-on"
    );
}

#[test]
fn e11_burn_in_no_nested_gpu_claims() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = small_flat_star_equal_weights();
    let fx = open_scenario_session(&fixture);

    assert_eq!(fx.layout.max_depth, 2, "E-11 remains flat-star D=2 only");
    for node in fx.layout.iter_all() {
        assert!(
            node.depth <= 1,
            "flat-star nodes must be root or leaf only, got depth {}",
            node.depth
        );
    }

    let arena = &fx.session.spec_state.arena_registry.arenas[0];
    assert!(
        arena.wildcard_max_expansion.is_none(),
        "burn-in scenarios must avoid wildcard admission"
    );
    assert!(
        !fx.session
            .spec_state
            .arena_participant_scaffold
            .index
            .by_host_and_arena
            .is_empty(),
        "burn-in scenarios must use explicit participants"
    );

    let execution = build_execution_plan(
        &fx.session.proto.registry,
        &fx.session.spec_state.arena_registry.arenas,
        &fx.session.proto.root,
        &fx.session.proto.allocator,
        &fx.session.spec_state.arena_participant_scaffold,
        fx.session.spec_state.arena_registry.generation,
    )
    .expect("execution plan");

    assert_eq!(execution.arenas.len(), 1);
    assert_eq!(execution.arenas[0].max_depth, 2);

    let plan = plan_arena_allocation(
        &execution.arenas[0],
        &simthing_gpu::build_governed_pairs(&fx.session.proto.registry),
        fx.session.state.n_slots,
    )
    .expect("allocation plan");
    assert!(
        !plan.cpu_ops.is_empty(),
        "flat-star GPU path must emit allocation ops"
    );
}

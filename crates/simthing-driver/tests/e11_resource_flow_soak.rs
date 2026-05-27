//! E-11 controlled opt-in CI soak for flat-star Resource Flow (default-off flag path).

mod support;

use simthing_sim::PipelineFlags;

use support::e11_burn_in_scenarios::{
    assert_flat_star_only_no_nested_claims, assert_no_nan_in_leaf_allocated, open_scenario_session,
};
use support::e11_flat_star::try_gpu;
use support::e11_resource_flow_soak::{
    run_flat_star_soak, soak_equal_weights_1000, soak_repeated_resync_100,
    soak_skewed_weights_1000, soak_zero_weights_1000,
};

fn run_soak_test(soak: &support::e11_resource_flow_soak::ResourceFlowSoakFixture) {
    let mut fx = open_scenario_session(&soak.scenario);
    let report = run_flat_star_soak(&mut fx, soak);

    assert_eq!(report.scenario_name, soak.name);
    assert_eq!(report.ticks_checked, soak.ticks);
    assert_eq!(report.sync_cycles_checked, soak.sync_cycles.max(1));
    assert!(report.total_ops > 0);
    assert!(report.n_bands >= 5);
}

#[test]
fn e11_soak_equal_weights_1000_ticks_bit_exact() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let soak = soak_equal_weights_1000();
    run_soak_test(&soak);
}

#[test]
fn e11_soak_skewed_weights_1000_ticks_bit_exact() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let soak = soak_skewed_weights_1000();
    run_soak_test(&soak);
}

#[test]
fn e11_soak_zero_weights_1000_ticks_no_nan() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let soak = soak_zero_weights_1000();
    let mut fx = open_scenario_session(&soak.scenario);
    let report = run_flat_star_soak(&mut fx, &soak);

    assert_eq!(report.ticks_checked, 1000);
    assert!(report.replay_bit_exact);
    assert_eq!(report.max_abs_error.to_bits(), 0.0_f32.to_bits());

    let n_dims = fx.session.proto.registry.total_columns as u32;
    assert_no_nan_in_leaf_allocated(&fx.session.state, &fx.layout, fx.cols, n_dims);
}

#[test]
fn e11_soak_repeated_resync_100_cycles_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let soak = soak_repeated_resync_100();
    let mut fx = open_scenario_session(&soak.scenario);
    let report = run_flat_star_soak(&mut fx, &soak);

    assert_eq!(report.sync_cycles_checked, 100);
    assert_eq!(report.ticks_checked, 10);
    assert!(report.replay_bit_exact);
    assert_eq!(report.max_abs_error.to_bits(), 0.0_f32.to_bits());
}

#[test]
fn e11_soak_flag_default_false() {
    assert!(
        !PipelineFlags::default().use_accumulator_resource_flow,
        "controlled CI soak must not flip default-on"
    );
}

#[test]
fn e11_soak_flat_star_only_no_nested_claims() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let soak = soak_equal_weights_1000();
    let fx = open_scenario_session(&soak.scenario);
    assert_flat_star_only_no_nested_claims(&fx);
}

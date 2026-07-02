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


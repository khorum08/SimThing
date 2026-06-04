//! Controlled opt-in CI soak markers and runners for flat-star Resource Flow.

use simthing_driver::{
    run_flat_star_burn_in, sync_resource_flow_accumulator, ResourceFlowSoakSummaryReport,
};

use super::e11_burn_in_scenarios::{
    scenario_cell_inputs, small_flat_star_equal_weights, small_flat_star_repeated_boundary_sync,
    small_flat_star_skewed_weights, small_flat_star_zero_weights, BurnInScenarioFixture,
};
use super::e11_flat_star::{leaf_slots, FlatStarSession};

/// Opt-in marker for scenarios allowed to run Resource Flow soak in CI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceFlowSoakMode {
    Disabled,
    FlatStarOptIn,
}

#[derive(Clone, Debug)]
pub struct ResourceFlowSoakFixture {
    pub name: &'static str,
    pub scenario: BurnInScenarioFixture,
    pub mode: ResourceFlowSoakMode,
    pub ticks: u32,
    pub sync_cycles: u32,
    pub max_abs_error_allowed: f32,
    pub require_bit_exact: bool,
}

pub fn soak_equal_weights_1000() -> ResourceFlowSoakFixture {
    ResourceFlowSoakFixture {
        name: "soak_equal_weights_1000",
        scenario: small_flat_star_equal_weights(),
        mode: ResourceFlowSoakMode::FlatStarOptIn,
        ticks: 1000,
        sync_cycles: 0,
        max_abs_error_allowed: 0.0,
        require_bit_exact: true,
    }
}

pub fn soak_skewed_weights_1000() -> ResourceFlowSoakFixture {
    ResourceFlowSoakFixture {
        name: "soak_skewed_weights_1000",
        scenario: small_flat_star_skewed_weights(),
        mode: ResourceFlowSoakMode::FlatStarOptIn,
        ticks: 1000,
        sync_cycles: 0,
        max_abs_error_allowed: 0.0,
        require_bit_exact: true,
    }
}

pub fn soak_zero_weights_1000() -> ResourceFlowSoakFixture {
    ResourceFlowSoakFixture {
        name: "soak_zero_weights_1000",
        scenario: small_flat_star_zero_weights(),
        mode: ResourceFlowSoakMode::FlatStarOptIn,
        ticks: 1000,
        sync_cycles: 0,
        max_abs_error_allowed: 0.0,
        require_bit_exact: true,
    }
}

pub fn soak_repeated_resync_100() -> ResourceFlowSoakFixture {
    ResourceFlowSoakFixture {
        name: "soak_repeated_resync_100",
        scenario: small_flat_star_repeated_boundary_sync(),
        mode: ResourceFlowSoakMode::FlatStarOptIn,
        ticks: 10,
        sync_cycles: 100,
        max_abs_error_allowed: 0.0,
        require_bit_exact: true,
    }
}

pub fn assert_soak_opt_in(soak: &ResourceFlowSoakFixture) {
    assert_eq!(
        soak.mode,
        ResourceFlowSoakMode::FlatStarOptIn,
        "soak {name} must opt in explicitly",
        name = soak.name
    );
}

pub fn run_flat_star_soak(
    fx: &mut FlatStarSession,
    soak: &ResourceFlowSoakFixture,
) -> ResourceFlowSoakSummaryReport {
    assert_soak_opt_in(soak);

    let leaves = leaf_slots(&fx.layout);
    let inputs = scenario_cell_inputs(&soak.scenario, &fx.layout, fx.cols);
    let n_dims = fx.session.proto.registry.total_columns as u32;
    let mut sync_cycles_checked = 0u32;
    let last_sync = sync_resource_flow_accumulator(
        &mut fx.session.state,
        &fx.session.proto.registry,
        &fx.session.spec_state.arena_registry,
        &fx.session.spec_state.arena_participant_scaffold,
        &fx.session.proto.root,
        &fx.session.proto.allocator,
        true,
    )
    .expect("initial soak sync");
    sync_cycles_checked += 1;
    let initial_ops = fx
        .session
        .state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .resource_flow_ops
        .count;
    let initial_bands = fx.session.state.accumulator_resource_flow_bands;

    let mut ops_samples = Vec::new();
    let mut band_samples = Vec::new();
    for _ in 0..soak.sync_cycles.saturating_sub(1) {
        fx.session
            .sync_resource_flow_if_enabled()
            .expect("soak resync");
        sync_cycles_checked += 1;
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

    if soak.sync_cycles > 1 {
        for &ops in &ops_samples {
            assert_eq!(
                ops,
                initial_ops,
                "soak {name} op count unstable",
                name = soak.name
            );
        }
        for &bands in &band_samples {
            assert_eq!(
                bands,
                initial_bands,
                "soak {name} n_bands unstable",
                name = soak.name
            );
        }
    }

    let n_bands = fx.session.state.accumulator_resource_flow_bands;
    let burn = run_flat_star_burn_in(
        &mut fx.session.state,
        &fx.layout,
        fx.cols,
        n_dims,
        &inputs,
        &leaves,
        n_bands,
        soak.ticks,
        fx.session.scenario.dt,
    );

    let report = ResourceFlowSoakSummaryReport::from_parts(
        soak.name,
        &last_sync,
        &burn,
        sync_cycles_checked,
        soak.require_bit_exact,
    );
    report.assert_within_contract(soak.require_bit_exact, soak.max_abs_error_allowed);
    report
}

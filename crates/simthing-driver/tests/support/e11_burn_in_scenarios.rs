//! Named flat-star Resource Flow burn-in scenario fixtures (controlled, default-off path).

use std::collections::HashMap;

use simthing_driver::{
    run_flat_star_burn_in, sync_resource_flow_accumulator, ArenaTreeLayout, NodeColumnRefs,
    ResourceFlowScenarioBurnInReport,
};

use super::e11_flat_star::{
    flat_star_cell_inputs, leaf_slots, open_flat_star_session, root_slot, FlatStarSession,
};

pub const SMALL_FLAT_STAR_EQUAL_WEIGHTS: &str = "small_flat_star_equal_weights";
pub const SMALL_FLAT_STAR_SKEWED_WEIGHTS: &str = "small_flat_star_skewed_weights";
pub const SMALL_FLAT_STAR_ZERO_WEIGHTS: &str = "small_flat_star_zero_weights";
pub const SMALL_FLAT_STAR_REPEATED_BOUNDARY_SYNC: &str = "small_flat_star_repeated_boundary_sync";

#[derive(Clone, Debug)]
pub struct BurnInScenarioFixture {
    pub name: &'static str,
    pub hosted_count: usize,
    pub root_intrinsic_flow: f32,
    pub leaf_weights: Vec<f32>,
    pub expect_bit_exact: bool,
}

pub fn small_flat_star_equal_weights() -> BurnInScenarioFixture {
    BurnInScenarioFixture {
        name: SMALL_FLAT_STAR_EQUAL_WEIGHTS,
        hosted_count: 3,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0, 1.0],
        expect_bit_exact: true,
    }
}

pub fn small_flat_star_skewed_weights() -> BurnInScenarioFixture {
    BurnInScenarioFixture {
        name: SMALL_FLAT_STAR_SKEWED_WEIGHTS,
        hosted_count: 3,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0, 3.0],
        expect_bit_exact: true,
    }
}

pub fn small_flat_star_zero_weights() -> BurnInScenarioFixture {
    BurnInScenarioFixture {
        name: SMALL_FLAT_STAR_ZERO_WEIGHTS,
        hosted_count: 3,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![0.0, 0.0],
        expect_bit_exact: true,
    }
}

pub fn small_flat_star_repeated_boundary_sync() -> BurnInScenarioFixture {
    BurnInScenarioFixture {
        name: SMALL_FLAT_STAR_REPEATED_BOUNDARY_SYNC,
        hosted_count: 3,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0, 1.0],
        expect_bit_exact: true,
    }
}

pub fn open_scenario_session(fixture: &BurnInScenarioFixture) -> FlatStarSession {
    let fx = open_flat_star_session(fixture.hosted_count, true);
    assert!(
        fx.session.proto.flags.use_accumulator_resource_flow,
        "scenario {name} must explicitly enable resource flow",
        name = fixture.name
    );
    assert_eq!(
        fx.layout.max_depth,
        2,
        "scenario {name} must stay flat-star D=2",
        name = fixture.name
    );
    fx
}

pub fn scenario_cell_inputs(
    fixture: &BurnInScenarioFixture,
    layout: &ArenaTreeLayout,
    cols: NodeColumnRefs,
) -> HashMap<(u32, u32), f32> {
    flat_star_cell_inputs(
        root_slot(layout),
        &leaf_slots(layout),
        cols,
        fixture.root_intrinsic_flow,
        &fixture.leaf_weights,
    )
}

pub fn run_scenario_burn_in(
    fx: &mut FlatStarSession,
    fixture: &BurnInScenarioFixture,
    ticks: u32,
) -> ResourceFlowScenarioBurnInReport {
    let leaves = leaf_slots(&fx.layout);
    let inputs = scenario_cell_inputs(fixture, &fx.layout, fx.cols);
    let n_dims = fx.session.proto.registry.total_columns as u32;
    let n_bands = fx.session.state.accumulator_resource_flow_bands;

    let sync = sync_resource_flow_accumulator(
        &mut fx.session.state,
        &fx.session.proto.registry,
        &fx.session.spec_state.arena_registry,
        &fx.session.spec_state.arena_participant_scaffold,
        &fx.session.proto.root,
        &fx.session.proto.allocator,
        &[],
        &[],
        true,
    )
    .expect("scenario sync");

    let burn = run_flat_star_burn_in(
        &mut fx.session.state,
        &fx.layout,
        fx.cols,
        n_dims,
        &inputs,
        &leaves,
        n_bands,
        ticks,
        fx.session.scenario.dt,
    );

    let mut report = ResourceFlowScenarioBurnInReport::from_parts(fixture.name, &sync, &burn);
    report.replay_bit_exact =
        fixture.expect_bit_exact && burn.max_abs_error.to_bits() == 0.0_f32.to_bits();
    report
}

pub fn assert_flat_star_only_no_nested_claims(fx: &FlatStarSession) {
    use simthing_driver::{build_execution_plan, plan_arena_allocation};

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
        "Resource Flow soak must avoid wildcard admission"
    );
    assert!(
        !fx.session
            .spec_state
            .arena_participant_scaffold
            .index
            .by_host_and_arena
            .is_empty(),
        "Resource Flow soak must use explicit participants"
    );

    let execution = build_execution_plan_from_authoring(
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

pub fn assert_no_nan_in_leaf_allocated(
    state: &simthing_gpu::WorldGpuState,
    layout: &ArenaTreeLayout,
    cols: NodeColumnRefs,
    n_dims: u32,
) {
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    let gpu_out = state.read_values();
    for leaf in leaf_slots(layout) {
        let v = gpu_out[idx(leaf, cols.allocated_flow_col)];
        assert!(
            v.is_finite(),
            "leaf {leaf} allocated_flow must be finite, got {v}"
        );
    }
}

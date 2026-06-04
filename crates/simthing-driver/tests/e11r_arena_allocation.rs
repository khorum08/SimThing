//! E-11R — remedial hardening for landed flat-star allocation execution.

mod support;

use simthing_driver::{
    run_arena_allocation_oracle, HierarchyError, ResourceFlowSyncError, SessionError,
};

use simthing_driver::{install_atomic, SimSession};
use support::e11_flat_star::{
    fill_explicit_participants, flat_star_game_mode, flat_star_scenario, open_flat_star_session,
    standard_flat_star_inputs, try_gpu,
};

#[test]
fn e11r_resource_flow_sync_error_is_reported_when_flag_enabled() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = flat_star_game_mode(4);
    fill_explicit_participants(&mut game_mode, &scenario);

    let mut session = SimSession::open(scenario).expect("open session");
    let spec_state = install_atomic(
        &game_mode,
        &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )
    .expect("install atomic");

    session.proto.flags.use_accumulator_resource_flow = true;
    session.spec_state = spec_state;

    let err = session
        .sync_resource_flow_if_enabled()
        .expect_err("depth budget must fail when flag enabled");
    match err {
        SessionError::ResourceFlow(ResourceFlowSyncError::Hierarchy(
            HierarchyError::OrderBandDepthExceeded {
                needed: 5, max: 4, ..
            },
        )) => {}
        other => panic!("unexpected session error: {other:?}"),
    }
}

#[test]
fn e11_resource_flow_flag_uploads_and_dispatches_flat_star_ops() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut fx = open_flat_star_session(3, true);
    assert!(
        fx.session.state.accumulator_resource_flow_active,
        "session sync must upload resource-flow ops when flag enabled"
    );
    assert!(
        fx.session.state.accumulator_resource_flow_bands >= 5,
        "D=2 flat-star needs at least 5 bands"
    );

    let root_slot = fx.layout.participant_roots[0].participant_slot;
    let leaf_slots: Vec<_> = fx.layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    let cols = fx.cols;
    let n_dims = fx.session.proto.registry.total_columns as u32;

    let inputs = standard_flat_star_inputs(root_slot, &leaf_slots, cols);
    let mut flat = fx.session.state.read_values();
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    for (&(slot, col), &v) in &inputs {
        flat[idx(slot, col)] = v;
    }
    fx.session.state.write_values(&flat);

    let mut oracle = inputs.clone();
    run_arena_allocation_oracle(&fx.layout, &mut oracle, 1.0);

    fx.session
        .state
        .run_resource_flow_bands(fx.session.state.accumulator_resource_flow_bands, 1.0);

    let gpu_out = fx.session.state.read_values();
    for &leaf in &leaf_slots {
        let cpu = oracle
            .get(&(leaf, cols.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu_out[idx(leaf, cols.allocated_flow_col)];
        assert_eq!(
            cpu.to_bits(),
            gpu.to_bits(),
            "leaf {leaf} session-path aF parity"
        );
    }
}

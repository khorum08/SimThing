//! E-11 controlled burn-in scaffold (default-off flag, flat-star only).

mod support;

use simthing_driver::{
    plan_arena_allocation, run_flat_star_burn_in, sync_resource_flow_accumulator,
    ResourceFlowBurnInReport,
};
use simthing_sim::PipelineFlags;

use support::e11_flat_star::{open_flat_star_session, standard_flat_star_inputs, try_gpu};

#[test]
fn e11_burn_in_flat_star_two_ticks_replay_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fx = open_flat_star_session(3, true);
    let root_slot = fx.layout.participant_roots[0].participant_slot;
    let leaf_slots: Vec<_> = fx.layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    let inputs = standard_flat_star_inputs(root_slot, &leaf_slots, fx.cols);
    let n_dims = fx.session.proto.registry.total_columns as u32;
    let n_bands = fx.session.state.accumulator_resource_flow_bands;

    let mut session_a = fx.session;
    let report_a = run_flat_star_burn_in(
        &mut session_a.state,
        &fx.layout,
        fx.cols,
        n_dims,
        &inputs,
        &leaf_slots,
        n_bands,
        2,
        1.0,
    );

    let fx_b = open_flat_star_session(3, true);
    let mut session_b = fx_b.session;
    let report_b = run_flat_star_burn_in(
        &mut session_b.state,
        &fx_b.layout,
        fx_b.cols,
        n_dims,
        &inputs,
        &leaf_slots,
        n_bands,
        2,
        1.0,
    );

    assert_eq!(report_a.max_abs_error.to_bits(), 0.0_f32.to_bits());
    assert_eq!(report_b.max_abs_error.to_bits(), 0.0_f32.to_bits());
    assert_eq!(report_a.ticks_checked, 2);
    assert_eq!(report_b.ticks_checked, 2);

    let out_a = session_a.state.read_values();
    let out_b = session_b.state.read_values();
    assert_eq!(out_a.len(), out_b.len());
    for (a, b) in out_a.iter().zip(out_b.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "replay bit-exact across runs");
    }
}

#[test]
fn e11_burn_in_flag_off_clears_resource_flow_ops() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    assert!(
        !PipelineFlags::default().use_accumulator_resource_flow,
        "burn-in scaffold must not flip default-on"
    );

    let mut fx = open_flat_star_session(3, true);
    assert!(fx.session.state.accumulator_resource_flow_active);

    fx.session.proto.flags.use_accumulator_resource_flow = false;
    fx.session
        .sync_resource_flow_if_enabled()
        .expect("flag-off sync clears ops");

    assert!(
        !fx.session.state.accumulator_resource_flow_active,
        "flag off must clear resource-flow dispatch"
    );
    assert_eq!(fx.session.state.accumulator_resource_flow_bands, 0);
}

#[test]
fn e11_burn_in_flag_on_uploads_expected_op_count() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut fx = open_flat_star_session(3, true);
    let expected = plan_arena_allocation(
        &fx.layout,
        &simthing_gpu::build_governed_pairs(&fx.session.proto.registry),
        fx.session.state.n_slots,
    )
    .expect("plan")
    .cpu_ops
    .len() as u32;

    let sync = sync_resource_flow_accumulator(
        &mut fx.session.state,
        &fx.session.proto.registry,
        &fx.session.spec_state.arena_registry,
        &fx.session.spec_state.arena_participant_scaffold,
        &fx.session.proto.root,
        &fx.session.proto.allocator,
        &[],
        true,
    )
    .expect("resync");

    assert_eq!(sync.total_ops, expected);
    assert_eq!(
        fx.session
            .state
            .accumulator_runtime
            .as_ref()
            .unwrap()
            .resource_flow_ops
            .count,
        expected
    );

    let report = ResourceFlowBurnInReport::from_sync(&sync);
    assert_eq!(report.total_ops, expected);
    assert!(report.total_ops > 0);
    assert_eq!(report.arenas_planned, 1);
    assert!(report.n_bands >= 5);
    assert!(fx.session.state.accumulator_resource_flow_active);
}

#[test]
fn e11_burn_in_no_drift_against_cpu_oracle_over_100_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut fx = open_flat_star_session(3, true);
    let root_slot = fx.layout.participant_roots[0].participant_slot;
    let leaf_slots: Vec<_> = fx.layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    let inputs = standard_flat_star_inputs(root_slot, &leaf_slots, fx.cols);
    let n_dims = fx.session.proto.registry.total_columns as u32;
    let n_bands = fx.session.state.accumulator_resource_flow_bands;

    let tick_report = run_flat_star_burn_in(
        &mut fx.session.state,
        &fx.layout,
        fx.cols,
        n_dims,
        &inputs,
        &leaf_slots,
        n_bands,
        100,
        1.0,
    );

    assert_eq!(tick_report.ticks_checked, 100);
    assert_eq!(tick_report.max_abs_error.to_bits(), 0.0_f32.to_bits());
}

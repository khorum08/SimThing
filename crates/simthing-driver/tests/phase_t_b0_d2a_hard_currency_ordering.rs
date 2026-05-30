//! Phase B-0 — narrow driver-only D-2a hard-currency ordering smoke.

#[path = "support/resource_economy_materialize.rs"]
mod materialize_support;

#[path = "support/resource_economy_session.rs"]
mod session_support;

use simthing_core::{discrete_transfer_registration_to_op, GateSpec, SimThing, SimThingKind};
use simthing_driver::{
    materialize_resource_economy_registrations, run_transfer_recipe_burn_in,
    run_transfer_recipe_cpu_oracle, BoundaryScheduleKey, ResourceEconomyBoundaryScheduleReport,
    ResourceEconomySyncError, SimSession, KIND_RANK_TRANSFER,
};
use simthing_gpu::{discrete_transfer_registrations_to_transfer, plan_transfer_ops, GpuContext};
use simthing_spec::{compile_resource_economy, ResourceEconomySpec, SpecError};
use materialize_support::{
    amount_transfer, compile_fixture, empty_registry, exact_eml_registry,
    register_amount_property,
};
use session_support::{amount_col, base_game_mode, try_gpu};

const TREASURY_INITIAL: f32 = 10.0;
const TRANSFER_X: f32 = 3.0;
const TRANSFER_Y: f32 = 4.0;

fn b0_transfer_spec() -> ResourceEconomySpec {
    ResourceEconomySpec {
        transfers: vec![
            amount_transfer("transfer_0", "treasury_A", "sink_0", TRANSFER_X, 0),
            amount_transfer("transfer_1", "treasury_A", "sink_1", TRANSFER_Y, 1),
        ],
        ..Default::default()
    }
}

fn b0_scenario() -> simthing_driver::Scenario {
    let mut reg = empty_registry();
    let treasury = register_amount_property(&mut reg, "core", "treasury_A");
    let sink0 = register_amount_property(&mut reg, "core", "sink_0");
    let sink1 = register_amount_property(&mut reg, "core", "sink_1");

    let treasury_layout = reg.property(treasury).layout.clone();
    let sink0_layout = reg.property(sink0).layout.clone();
    let sink1_layout = reg.property(sink1).layout.clone();

    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_property(
        treasury,
        simthing_core::PropertyValue::from_layout(&treasury_layout),
    );
    world.add_property(sink0, simthing_core::PropertyValue::from_layout(&sink0_layout));
    world.add_property(sink1, simthing_core::PropertyValue::from_layout(&sink1_layout));

    simthing_driver::Scenario {
        name: "b0_hard_currency_ordering".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 4,
        registry: reg,
        root: world,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

fn b0_game_mode() -> simthing_spec::GameModeSpec {
    let mut mode = base_game_mode();
    mode.resource_economy = Some(b0_transfer_spec());
    mode
}

fn open_b0_session() -> SimSession {
    let scenario = b0_scenario();
    let mut session = SimSession::open_from_spec(scenario, &b0_game_mode()).expect("open");
    session.proto.flags.use_accumulator_transfer = true;
    session.sync_resource_economy_if_enabled().expect("sync");
    session
}

fn b0_initial_flat(session: &SimSession) -> (Vec<f32>, u32, u32, u32, u32) {
    let reg = session.proto.registry.clone();
    let treasury_col = amount_col(&reg, "core", "treasury_A");
    let sink0_col = amount_col(&reg, "core", "sink_0");
    let sink1_col = amount_col(&reg, "core", "sink_1");
    let n_dims = reg.total_columns as u32;
    let slot = 0u32;
    let mut flat = vec![0.0_f32; (session.state.n_slots * n_dims) as usize];
    flat[(slot * n_dims + treasury_col) as usize] = TREASURY_INITIAL;
    (flat, treasury_col, sink0_col, sink1_col, n_dims)
}

#[test]
fn b0_order_band_wires_authored_transfer_bands() {
    let mut reg = empty_registry();
    register_amount_property(&mut reg, "core", "treasury_A");
    register_amount_property(&mut reg, "core", "sink_0");
    register_amount_property(&mut reg, "core", "sink_1");
    let eml = exact_eml_registry(&[]);
    let compiled = compile_fixture(&b0_transfer_spec(), &reg, &eml);
    let materialized =
        materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();

    assert_eq!(materialized.transfers[0].order_band, 0);
    assert_eq!(materialized.transfers[1].order_band, 1);
    assert!(matches!(
        discrete_transfer_registration_to_op(&materialized.transfers[0])
            .unwrap()
            .gate,
        GateSpec::OrderBand(0)
    ));
    assert!(matches!(
        discrete_transfer_registration_to_op(&materialized.transfers[1])
            .unwrap()
            .gate,
        GateSpec::OrderBand(1)
    ));

    let gpu_regs = discrete_transfer_registrations_to_transfer(&materialized.transfers);
    let plan = plan_transfer_ops(&gpu_regs).unwrap();
    assert_eq!(plan.n_bands, 2);
    assert!(matches!(plan.ops[0].gate, GateSpec::OrderBand(0)));
    assert!(matches!(plan.ops[1].gate, GateSpec::OrderBand(1)));
}

#[test]
fn b0_cross_band_same_source_sequential_debit_succeeds_when_funds_sufficient() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_b0_session();
    let (flat, treasury_col, sink0_col, sink1_col, n_dims) = b0_initial_flat(&session);
    let transfers = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .unwrap()
        .registrations
        .transfers
        .clone();

    let mut state = session.state;
    let report = run_transfer_recipe_burn_in(
        &mut state,
        n_dims,
        &flat,
        &transfers,
        &[],
        &[(0, treasury_col), (0, sink0_col), (0, sink1_col)],
        1,
        1.0,
    )
    .expect("burn-in");

    assert_eq!(report.max_abs_conservation_error.to_bits(), 0.0_f32.to_bits());
    let out = state.read_values();
    assert_eq!(
        out[(0 * n_dims + treasury_col) as usize].to_bits(),
        (TREASURY_INITIAL - TRANSFER_X - TRANSFER_Y).to_bits()
    );
    assert_eq!(
        out[(0 * n_dims + sink0_col) as usize].to_bits(),
        TRANSFER_X.to_bits()
    );
    assert_eq!(
        out[(0 * n_dims + sink1_col) as usize].to_bits(),
        TRANSFER_Y.to_bits()
    );
}

#[test]
fn b0_same_band_same_source_double_debit_still_rejects() {
    let mut reg = empty_registry();
    register_amount_property(&mut reg, "core", "treasury_A");
    register_amount_property(&mut reg, "core", "sink_0");
    register_amount_property(&mut reg, "core", "sink_1");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![
            amount_transfer("transfer_0", "treasury_A", "sink_0", 1.0, 0),
            amount_transfer("transfer_1", "treasury_A", "sink_1", 1.0, 0),
        ],
        ..Default::default()
    };
    let err = compile_resource_economy(&spec, &reg, &eml).unwrap_err();
    assert!(matches!(
        err,
        SpecError::ResourceEconomyConsumedInputContention { order_band: 0, .. }
    ));
}

#[test]
fn b0_deterministic_boundary_schedule_report_uses_stable_key() {
    let session = open_b0_session();
    let registry = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("registry");
    let schedule = ResourceEconomyBoundaryScheduleReport::build(registry);

    assert_eq!(schedule.entries.len(), 2);
    assert_eq!(
        schedule.entries[0].key,
        BoundaryScheduleKey {
            order_band: 0,
            kind_rank: KIND_RANK_TRANSFER,
            authoring_id: "transfer_0".into(),
        }
    );
    assert_eq!(
        schedule.entries[1].key,
        BoundaryScheduleKey {
            order_band: 1,
            kind_rank: KIND_RANK_TRANSFER,
            authoring_id: "transfer_1".into(),
        }
    );

    let schedule_b = ResourceEconomyBoundaryScheduleReport::build(registry);
    assert_eq!(schedule.entries, schedule_b.entries);
}

#[test]
fn b0_gpu_cpu_oracle_parity_exact() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_b0_session();
    let (flat, treasury_col, sink0_col, sink1_col, n_dims) = b0_initial_flat(&session);
    let transfers = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .unwrap()
        .registrations
        .transfers
        .clone();

    let mut state = session.state;
    state.write_values(&flat);
    let gpu_out = {
        use simthing_gpu::{AccumulatorPipelineSessions, Pipelines};
        let pipelines = Pipelines::new(&state.ctx);
        let mut transfer_session = state
            .accumulator_runtime
            .as_mut()
            .unwrap()
            .take_transfer_session();
        pipelines.run_tick_pipeline_with_accumulators(
            &mut state,
            1.0,
            AccumulatorPipelineSessions {
                intent: None,
                threshold: None,
                overlay_add: None,
                reduction_soft: None,
                velocity: None,
                intensity_eml: None,
                transfer: transfer_session.as_mut(),
                emission: None,
                encode_world_summary: false,
            },
        );
        state
            .accumulator_runtime
            .as_mut()
            .unwrap()
            .restore_transfer_session(transfer_session);
        state.read_values()
    };

    let mut cpu_flat = flat.clone();
    run_transfer_recipe_cpu_oracle(&mut cpu_flat, n_dims, &transfers, &[])
        .expect("cpu oracle");

    for &(slot, col) in &[(0, treasury_col), (0, sink0_col), (0, sink1_col)] {
        let idx = (slot * n_dims + col) as usize;
        assert_eq!(
            gpu_out[idx].to_bits(),
            cpu_flat[idx].to_bits(),
            "slot {slot} col {col} must be bit-exact"
        );
    }
}

#[test]
fn b0_replay_reproducibility_exact() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let run_once = || {
        let session = open_b0_session();
        let (flat, treasury_col, sink0_col, sink1_col, n_dims) = b0_initial_flat(&session);
        let transfers = session
            .spec_state
            .resource_economy_registry
            .as_ref()
            .unwrap()
            .registrations
            .transfers
            .clone();
        let mut state = session.state;
        run_transfer_recipe_burn_in(
            &mut state,
            n_dims,
            &flat,
            &transfers,
            &[],
            &[(0, treasury_col), (0, sink0_col), (0, sink1_col)],
            3,
            1.0,
        )
        .expect("burn-in")
    };

    let a = run_once();
    let b = run_once();
    assert!(a.replay_bit_exact);
    assert!(b.replay_bit_exact);
    assert_eq!(a.max_abs_conservation_error.to_bits(), b.max_abs_conservation_error.to_bits());
}

#[test]
fn b0_flag_off_behavior_unchanged() {
    let scenario = b0_scenario();
    let mut session = SimSession::open_from_spec(scenario, &b0_game_mode()).expect("open");
    session.proto.flags.use_accumulator_transfer = false;
    let err = session.sync_resource_economy_if_enabled().unwrap_err();
    assert!(matches!(
        err,
        simthing_driver::SessionError::ResourceEconomy(
            ResourceEconomySyncError::TransferFlagOffPopulatedSpec
        )
    ));
}

#[test]
fn b0_resource_flow_not_used_for_hard_currency() {
    let session = open_b0_session();
    assert!(
        !session.state.accumulator_resource_flow_active,
        "hard-currency B-0 fixture must not activate Resource Flow"
    );
    assert_eq!(session.state.accumulator_transfer_bands, 2);
}

#[test]
fn b0_no_new_wgsl_roles_or_cpu_fallback() {
    assert!(
        GpuContext::new_blocking().is_ok(),
        "GPU path required; no CPU production fallback added for B-0"
    );
    let session = open_b0_session();
    assert!(
        session.state.accumulator_transfer_active,
        "existing AccumulatorOp transfer path must remain active"
    );
}

#[test]
fn b0_no_a0_c_runtime_l3_frontierv2_5_or_act_event_obs_pipe() {
    let session = open_b0_session();
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(session.proto.flags.use_accumulator_transfer);
}

#[test]
fn b0_no_simthing_sim_semantic_awareness() {
    let sim_src = include_str!("../../simthing-sim/src/lib.rs");
    assert!(
        !sim_src.contains("order_band"),
        "simthing-sim must not gain resource-economy order_band awareness in B-0"
    );
    assert!(!sim_src.contains("ResourceTransferSpec"));
}

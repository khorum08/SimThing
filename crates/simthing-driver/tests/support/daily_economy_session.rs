//! Shared helpers for Phase M daily economy banking fixture tests.

#![allow(dead_code)]

use simthing_core::{DimensionRegistry, PropertyValue, SimProperty, SimThing, SimThingKind};
use simthing_driver::{Scenario, SimSession};
use simthing_gpu::{
    emit_on_threshold_registrations_to_gpu, GpuContext, ThresholdEvent,
    DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};
use simthing_spec::{
    deserialize_game_mode_ron, GameModeSpec, MappingExecutionProfile,
    ResourceFlowOptInMode,
};
pub const SURPLUS_RON: &str = include_str!("../fixtures/daily_economy_banking_scenario.ron");
pub const DEFICIT_RON: &str = include_str!("../fixtures/daily_economy_banking_deficit_scenario.ron");

pub const INITIAL_TREASURY: f32 = 100.0;
pub const SURPLUS_DAILY_NET: f32 = 7.0;
pub const DEFICIT_DAILY_NET: f32 = -6.0;
pub const LOW_STORAGE_EVENT_KIND: u32 = 0x4C4F5754;

pub fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

pub fn surplus_game_mode() -> GameModeSpec {
    deserialize_game_mode_ron(SURPLUS_RON).expect("surplus daily economy RON parses")
}

pub fn deficit_game_mode() -> GameModeSpec {
    deserialize_game_mode_ron(DEFICIT_RON).expect("deficit daily economy RON parses")
}

pub fn daily_economy_scenario(ticks_per_day: u32, max_days: u32) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let boot = registry.register(SimProperty::simple("session", "boot", 0));
    let boot_layout = registry.property(boot).layout.clone();
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_property(boot, PropertyValue::from_layout(&boot_layout));

    Scenario {
        name: "phase_m_daily_economy_banking".into(),
        ticks_per_day,
        max_days,
        dt: 1.0,
        n_slots: 8,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

pub fn open_daily_economy_session(
    game_mode: &GameModeSpec,
    ticks_per_day: u32,
    max_days: u32,
) -> SimSession {
    SimSession::open_from_spec(daily_economy_scenario(ticks_per_day, max_days), game_mode)
        .expect("open daily economy session")
}

pub fn open_daily_economy_session_with_thresholds(
    game_mode: &GameModeSpec,
    ticks_per_day: u32,
    max_days: u32,
) -> SimSession {
    let mut session = open_daily_economy_session(game_mode, ticks_per_day, max_days);
    sync_economy_threshold_emissions(&mut session);
    session
}

/// Upload materialized `emit_on_threshold` registrations through the existing GPU threshold bridge.
pub fn sync_economy_threshold_emissions(session: &mut SimSession) {
    let Some(registry) = session.spec_state.resource_economy_registry.as_ref() else {
        return;
    };
    if registry.registrations.emit_on_threshold.is_empty() {
        return;
    }
    let gpu_regs = emit_on_threshold_registrations_to_gpu(&registry.registrations.emit_on_threshold);
    session.state.ensure_threshold_accumulator(DEFAULT_THRESHOLD_EMISSION_CAPACITY);
    session
        .state
        .upload_accumulator_threshold_ops(&gpu_regs)
        .expect("upload emit_on_threshold registrations");
}

pub fn producer_amount(session: &SimSession) -> f32 {
    let reg = &session.proto.registry;
    let pid = reg.id_of("core", "producer").expect("producer registered");
    let col = reg
        .column_range(pid)
        .col_for_role(
            &simthing_core::SubFieldRole::Named("amount".into()),
            &reg.property(pid).layout,
        )
        .expect("producer amount column") as usize;
    session.state.read_values()[col]
}
pub fn treasury_col(session: &SimSession) -> u32 {
    let reg = &session.proto.registry;
    let pid = reg.id_of("core", "treasury").expect("treasury registered");
    reg.column_range(pid)
        .col_for_role(
            &simthing_core::SubFieldRole::Named("amount".into()),
            &reg.property(pid).layout,
        )
        .expect("treasury amount column") as u32
}

pub fn treasury_amount(session: &SimSession) -> f32 {
    let col = treasury_col(session) as usize;
    let values = session.state.read_values();
    values[col]
}

pub fn run_days_with_full_boundary(session: &mut SimSession, days: u32) {
    let cap = days.min(session.scenario.max_days);
    let mut boundaries_run = 0u64;

    while boundaries_run < cap as u64 {
        let tick = session.coord.tick(
            &session.rx,
            &mut session.patcher,
            &session.proto.registry,
            &session.proto.allocator,
            &session.pipelines,
            &mut session.state,
            session.scenario.dt,
        );

        if !tick.boundary_reached {
            continue;
        }

        let spec_state = &mut session.spec_state;
        let _outcome = session.proto.execute_with_boundary_hook(
            tick.events,
            &mut session.patcher,
            &mut session.coord,
            &mut session.state,
            tick.day_index,
            |ctx| spec_state.run_boundary_handlers(ctx),
        );
        session
            .sync_resource_economy_if_enabled()
            .expect("boundary economy sync");
        boundaries_run += 1;
    }
}

pub fn run_days_collecting_events(
    session: &mut SimSession,
    days: u32,
) -> (u64, Vec<ThresholdEvent>) {
    let cap = days.min(session.scenario.max_days);
    let mut boundaries_run = 0u64;
    let mut events = Vec::new();

    while boundaries_run < cap as u64 {
        let tick = session.coord.tick(
            &session.rx,
            &mut session.patcher,
            &session.proto.registry,
            &session.proto.allocator,
            &session.pipelines,
            &mut session.state,
            session.scenario.dt,
        );
        events.extend(tick.events.iter().copied());

        if tick.boundary_reached {
            if !session
                .spec_state
                .requires_boundary_tick(&tick.events, session.proto.threshold_registry())
                && session
                    .proto
                    .can_skip_empty_boundary(&tick.events, &session.patcher)
            {
                boundaries_run += 1;
                continue;
            }
            let spec_state = &mut session.spec_state;
            let _outcome = session.proto.execute_with_boundary_hook(
                tick.events,
                &mut session.patcher,
                &mut session.coord,
                &mut session.state,
                tick.day_index,
                |ctx| spec_state.run_boundary_handlers(ctx),
            );
            boundaries_run += 1;
            session.sync_resource_economy_if_enabled().expect("boundary sync");
        }
    }

    (boundaries_run, events)
}

pub fn assert_mapping_and_resource_flow_posture(session: &SimSession) {
    assert_eq!(
        session.resource_flow_execution_profile,
        simthing_spec::ResourceFlowExecutionProfile::DefaultDisabled
    );
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        game_mode_resource_flow_opt_in(&surplus_game_mode()),
        ResourceFlowOptInMode::Disabled
    );
}

fn game_mode_resource_flow_opt_in(mode: &GameModeSpec) -> ResourceFlowOptInMode {
    mode.resource_flow
        .as_ref()
        .map(|spec| spec.opt_in_mode)
        .unwrap_or(ResourceFlowOptInMode::Disabled)
}

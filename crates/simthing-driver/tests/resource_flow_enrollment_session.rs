//! E-2B — Resource Flow enrollment session-open integration tests.

mod support;

use simthing_core::{DimensionRegistry, SimThing, SimThingKind};
use simthing_driver::{build_execution_plan, install_atomic, resolve_node_columns, SimSession};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    ArenaSpec, EnrollmentSelectorSpec, FissionPolicySpec, GameModeSpec, InstallTargetSpec,
    PropertyKey, ResourceFlowSpec, SpecVersion,
};
use support::e11_flat_star::{register_food_flow, try_gpu};

fn cohort_scenario(hosted_count: usize, n_slots: u32) -> simthing_driver::Scenario {
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..hosted_count {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    let mut registry = DimensionRegistry::new();
    register_food_flow(&mut registry);
    simthing_driver::Scenario {
        name: "e2b_enrollment".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

fn flat_star_game_mode_with_enrollment(max_orderband_depth: u32) -> GameModeSpec {
    GameModeSpec {
        id: "e2b_flat_star".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                name: "food".into(),
                flow_property: PropertyKey::new("core", "food_flow"),
                balance_property: None,
                max_participants: 16,
                max_coupling_fanout: 4,
                max_orderband_depth,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                reserved_gap_per_intermediate: 0,
                expected_max_children_per_intermediate: 0,
                explicit_participants: vec![],
                enrollment: Some(EnrollmentSelectorSpec::InstallTarget(
                    InstallTargetSpec::AllOfKind {
                        kind: "Cohort".into(),
                    },
                )),
                wildcard_admission: None,
            }],
            couplings: vec![],
        ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
    }
}

#[test]
fn resource_flow_enrollment_session_open_flag_default_false() {
    let scenario = cohort_scenario(3, 32);
    let game_mode = flat_star_game_mode_with_enrollment(16);

    let mut session = SimSession::open(scenario).expect("open session");
    assert!(!session.proto.flags.use_accumulator_resource_flow);

    let spec_state = install_atomic(
        &game_mode,
        &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )
    .expect("install");

    assert_eq!(spec_state.arena_registry.participants.len(), 3);
    assert_eq!(spec_state.arena_participant_scaffold.reports.len(), 1);
    assert_eq!(
        spec_state.arena_participant_scaffold.reports[0].participant_count,
        3
    );
}

#[test]
fn resource_flow_enrollment_session_open_uploads_e11_flat_star_ops_when_flag_enabled() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let scenario = cohort_scenario(3, 32);
    let game_mode = flat_star_game_mode_with_enrollment(16);

    let mut session = SimSession::open(scenario).expect("open session");
    let spec_state = install_atomic(
        &game_mode,
        &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )
    .expect("install");

    session.proto.flags.use_accumulator_resource_flow = true;
    session
        .install_spec_state(spec_state)
        .expect("install spec state");

    assert!(
        session.state.accumulator_resource_flow_active,
        "E-11 flat-star ops must upload when flag enabled"
    );
    assert!(session.state.accumulator_resource_flow_bands >= 5);

    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow");
    let _cols =
        resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food").unwrap();
    let plan = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("execution plan");
    assert_eq!(plan.arenas.len(), 1);
    assert_eq!(plan.arenas[0].max_depth, 2);
}

#[test]
fn resource_flow_enrollment_session_open_without_manual_fill_explicit_participants() {
    let scenario = cohort_scenario(2, 32);
    let game_mode = flat_star_game_mode_with_enrollment(16);

    let mut session = SimSession::open(scenario).expect("open");
    let spec_state = install_atomic(
        &game_mode,
        &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )
    .expect("install without fill_explicit_participants");

    assert_eq!(spec_state.arena_registry.participants.len(), 2);
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}

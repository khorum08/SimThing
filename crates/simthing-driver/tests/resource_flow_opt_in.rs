//! RF-T1 — limited scenario-class Resource Flow execution opt-in flagging.

mod support;

use simthing_core::{DimensionRegistry, SimThing, SimThingKind};
use simthing_driver::{build_execution_plan, SimSession};
use simthing_sim::{BoundaryOutcome, FissionOutcome, PipelineFlags};
use simthing_spec::{
    ArenaSpec, EnrollmentSelectorSpec, FissionPolicySpec, GameModeSpec, InstallTargetSpec,
    PropertyKey, ResourceFlowOptInMode, ResourceFlowSpec, SpecVersion, WildcardAdmissionSpec,
};
use support::e11_burn_in_scenarios::assert_flat_star_only_no_nested_claims;
use support::e11_flat_star::{
    fill_explicit_participants, flat_star_game_mode, flat_star_scenario, try_gpu, FlatStarSession,
};

fn populated_flow_game_mode(opt_in: ResourceFlowOptInMode) -> GameModeSpec {
    let mut mode = flat_star_game_mode(16);
    mode.resource_flow.as_mut().unwrap().opt_in_mode = opt_in;
    mode
}

fn open_with_opt_in(hosted_count: usize, opt_in: ResourceFlowOptInMode) -> SimSession {
    let scenario = flat_star_scenario(hosted_count, 32);
    let mut game_mode = populated_flow_game_mode(opt_in);
    fill_explicit_participants(&mut game_mode, &scenario);
    SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec")
}

fn fission_inherit_game_mode(scenario: &simthing_driver::Scenario) -> GameModeSpec {
    let mut mode = GameModeSpec {
        id: "rf_t1_fission".into(),
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
            opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
            arenas: vec![ArenaSpec {
                name: "food".into(),
                flow_property: PropertyKey::new("core", "food_flow"),
                balance_property: None,
                max_participants: 16,
                max_coupling_fanout: 4,
                max_orderband_depth: 16,
                fission_policy: FissionPolicySpec::Inherit,
                reserved_orderband_depth: 0,
                reserved_gap_per_intermediate: 0,
                expected_max_children_per_intermediate: 0,
                explicit_participants: vec![],
                enrollment: None,
                wildcard_admission: None,
            }],
            couplings: vec![],
            base_obligations: vec![],
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    };
    fill_explicit_participants(&mut mode, scenario);
    mode
}

fn fission_scenario() -> (
    simthing_driver::Scenario,
    simthing_core::SimThingId,
    simthing_core::SimThingId,
) {
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    let parent_id = root.children[0].id;
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let child_id = child.id;
    root.children[0].add_child(child);

    let mut registry = DimensionRegistry::new();
    support::e11_flat_star::register_food_flow(&mut registry);
    let scenario = simthing_driver::Scenario {
        name: "rf_t1_fission".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };
    (scenario, parent_id, child_id)
}

#[test]
fn resource_flow_opt_in_disabled_keeps_flag_false() {
    if !try_gpu().is_some() {
        eprintln!("skipping GPU assertions: no GPU");
    }

    let session = open_with_opt_in(3, ResourceFlowOptInMode::Disabled);
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(!session.state.accumulator_resource_flow_active);
}

#[test]
fn resource_flow_opt_in_flat_star_enables_resource_flow_flag_only() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let session = open_with_opt_in(3, ResourceFlowOptInMode::FlatStarOptIn);
    assert!(session.proto.flags.use_accumulator_resource_flow);
    assert!(!session.proto.flags.use_accumulator_transfer);
    assert!(!session.proto.flags.use_accumulator_emission);
    assert!(session.state.accumulator_resource_flow_active);
    assert!(!session.state.accumulator_transfer_active);
    assert!(!session.state.accumulator_emission_active);
}

#[test]
fn resource_flow_opt_in_does_not_enable_resource_economy_transfer() {
    let session = open_with_opt_in(3, ResourceFlowOptInMode::FlatStarOptIn);
    assert!(!session.proto.flags.use_accumulator_transfer);
}

#[test]
fn resource_flow_opt_in_does_not_enable_resource_economy_emission() {
    let session = open_with_opt_in(3, ResourceFlowOptInMode::FlatStarOptIn);
    assert!(!session.proto.flags.use_accumulator_emission);
}

#[test]
fn resource_flow_opt_in_populated_spec_without_opt_in_stays_inactive() {
    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = populated_flow_game_mode(ResourceFlowOptInMode::Disabled);
    fill_explicit_participants(&mut game_mode, &scenario);
    assert_eq!(
        game_mode.resource_flow.as_ref().unwrap().arenas.len(),
        1,
        "spec must be populated"
    );

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open");
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(session.spec_state.arena_registry.participants.len(), 3);
}

#[test]
fn resource_flow_opt_in_flat_star_session_open_uploads_ops() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let session = open_with_opt_in(3, ResourceFlowOptInMode::FlatStarOptIn);
    assert!(session.state.accumulator_resource_flow_active);
    assert!(session.state.accumulator_resource_flow_bands >= 5);

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
fn resource_flow_opt_in_dynamic_enrollment_resyncs_after_fission() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (scenario, parent_id, child_id) = fission_scenario();
    let game_mode = fission_inherit_game_mode(&scenario);
    let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open");
    assert!(session.proto.flags.use_accumulator_resource_flow);

    let ops_before = session
        .state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .resource_flow_ops
        .count;
    let gen_before = session.spec_state.arena_registry.generation;

    let outcome = BoundaryOutcome {
        fission: FissionOutcome {
            fissions_executed: 1,
            fission_pairs: vec![(parent_id, child_id)],
            ..Default::default()
        },
        ..Default::default()
    };
    session
        .react_to_fission_resource_flow_enrollment(&outcome)
        .expect("dynamic enrollment");

    let report = session
        .last_resource_flow_dynamic_enrollment_report
        .as_ref()
        .expect("report");
    assert_eq!(report.admissions.len(), 1);
    assert!(report.rejections.is_empty());
    assert!(session.spec_state.arena_registry.generation > gen_before);

    let ops_after = session
        .state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .resource_flow_ops
        .count;
    assert!(ops_after >= ops_before);
}

#[test]
fn resource_flow_opt_in_no_nested_gpu_claims() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = populated_flow_game_mode(ResourceFlowOptInMode::FlatStarOptIn);
    fill_explicit_participants(&mut game_mode, &scenario);
    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open");

    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow");
    let cols = simthing_driver::resolve_node_columns(
        &session.proto.registry.property(flow_id).layout,
        "food",
    )
    .expect("cols");
    let layout = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("plan")
    .arenas
    .into_iter()
    .next()
    .expect("arena");

    let fx = FlatStarSession {
        session,
        layout,
        cols,
    };
    assert_flat_star_only_no_nested_claims(&fx);
}

#[test]
fn resource_flow_opt_in_wildcard_rejected_at_session_open() {
    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = populated_flow_game_mode(ResourceFlowOptInMode::FlatStarOptIn);
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.resource_flow.as_mut().unwrap().arenas[0].wildcard_admission =
        Some(WildcardAdmissionSpec {
            max_expansion: Some(4),
            expanded_count: 0,
        });

    let err = match SimSession::open_from_spec(scenario, &game_mode) {
        Err(e) => e,
        Ok(_) => panic!("wildcard FlatStarOptIn must be rejected at session open"),
    };
    assert!(
        err.to_string().contains("wildcard"),
        "expected wildcard rejection, got {err}"
    );
}

#[test]
fn resource_flow_opt_in_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("ResourceFlowOptInMode"));
}

#[test]
fn resource_flow_opt_in_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("resource_flow_opt_in"));
    let sync = include_str!("../../simthing-driver/src/arena_allocation_sync.rs");
    assert!(!sync.contains("wgsl"));
}

#[test]
fn resource_flow_global_pipeline_flags_default_remain_false() {
    let flags = PipelineFlags::default();
    assert!(!flags.use_accumulator_resource_flow);
    assert_eq!(
        ResourceFlowSpec::default().opt_in_mode,
        ResourceFlowOptInMode::Disabled
    );
}

#[test]
fn resource_flow_enrollment_without_opt_in_uses_install_target_only() {
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..3 {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    let mut registry = DimensionRegistry::new();
    support::e11_flat_star::register_food_flow(&mut registry);
    let scenario = simthing_driver::Scenario {
        name: "rf_t1_enroll".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };

    let game_mode = GameModeSpec {
        id: "rf_t1_enroll".into(),
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
            opt_in_mode: ResourceFlowOptInMode::Disabled,
            arenas: vec![ArenaSpec {
                name: "food".into(),
                flow_property: PropertyKey::new("core", "food_flow"),
                balance_property: None,
                max_participants: 16,
                max_coupling_fanout: 4,
                max_orderband_depth: 16,
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
            base_obligations: vec![],
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    };

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open");
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(session.spec_state.arena_registry.participants.len(), 3);
}

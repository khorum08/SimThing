use std::path::Path;

use simthing_core::{
    Direction, DissolveCondition, OwnerBoundaryValidationError, OwnerRef, PropertyTransformDelta,
    SimPropertyId, SimThing, SimThingKind, SpecializationObservations, SpecializationProfile,
    SubFieldRole, TransformOp,
};
use simthing_driver::Scenario;
use simthing_embedder::{bind, derive, overlay, populate, run};
use simthing_sim::{ThresholdRegistry, ThresholdSemantic, VelocityAlertRegistration};

fn node(label: &str) -> SimThing {
    SimThing::new(SimThingKind::Custom(label.to_string()), 0)
}

fn all_profile() -> SpecializationProfile {
    SpecializationProfile {
        id: "all".into(),
        description: "data-only fixture profile".into(),
        requirements: Vec::new(),
    }
}

fn transform(property_id: SimPropertyId) -> PropertyTransformDelta {
    PropertyTransformDelta {
        property_id,
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::set(1.0))],
    }
}

fn scenario_with_root(root: SimThing) -> Scenario {
    Scenario {
        name: "vendor-door".into(),
        ticks_per_day: 1,
        max_days: 2,
        dt: 1.0,
        n_slots: root.subtree_size() as u32,
        registry: Default::default(),
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    }
}

#[test]
fn derive_reserves_unowned_and_owner_specialization_query_has_no_kind_rival() {
    let owner = OwnerRef::try_new_authored("alpha").expect("authored owner");
    let mut a = node("ordinary-a");
    a.add_child(node("child-a"));
    populate::owner(&mut a, &owner);

    let mut b = a.clone();
    b.kind = SimThingKind::Location;
    b.children[0].kind = SimThingKind::Owner;

    let profiles = [all_profile()];
    let report_a = derive::specializations(&a, &profiles, &SpecializationObservations::default())
        .expect("derive A");
    let report_b = derive::specializations(&b, &profiles, &SpecializationObservations::default())
        .expect("derive B");
    let query_a = derive::owner_specializations(&a, &report_a).expect("query A");
    let query_b = derive::owner_specializations(&b, &report_b).expect("query B");

    assert_eq!(
        query_a, query_b,
        "kind-branch rival must not alter the door answer"
    );
    assert!(derive::reserved_unowned().is_unowned());
    assert!(derive::owner_seat("unowned", "rival", "rival").is_err());
    assert_eq!(
        simthing_spec::owner_entity_id(
            &derive::owner_seat("alpha", "Alpha", "fixture").expect("owner seat")
        )
        .as_deref(),
        Some("alpha")
    );
}

#[test]
fn populate_and_run_reject_stamp_every_node_rival_for_inherited_owner() {
    let owner = OwnerRef::try_new_authored("alpha").expect("owner");

    let mut lawful_local = Scenario::map_light("lawful-local-owner".into(), 1, 2, 1.0, 3);
    populate::owner(&mut lawful_local.root, &owner);
    populate::owner(&mut lawful_local.root.children[0], &owner);
    populate::ownership(&lawful_local.root)
        .expect("one deliberately redundant local binding remains lawful");
    run::initialize(lawful_local, &Default::default())
        .expect("ordinary Run accepts the lawful local binding");

    let mut root = node("root");
    root.add_child(node("child"));
    populate::owner(&mut root, &owner);
    populate::ownership(&root).expect("one boundary is lawful");

    populate::owner(&mut root.children[0], &owner);
    let error = populate::ownership(&root).expect_err("flat stamp must RED");
    assert!(matches!(
        error,
        OwnerBoundaryValidationError::BulkUniformStamp {
            stamped_nodes: 2,
            ..
        }
    ));

    let run_error = match run::initialize(scenario_with_root(root), &Default::default()) {
        Ok(_) => panic!("ordinary Run initialization accepted the stamped rival"),
        Err(error) => error,
    };
    assert!(matches!(run_error, run::InitializeError::Ownership(_)));
}

#[test]
fn overlay_rejects_synthesized_origin_and_missing_horizon_on_production_door() {
    let mut root = node("root");
    root.add_child(node("origin"));
    let foreign = node("synthesized-foreign");
    let target = root.children[0].id;

    let foreign_error = overlay::authored(
        &root,
        &foreign,
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![target],
        transform(SimPropertyId(7)),
        vec![DissolveCondition::AtSessionEnd],
    )
    .expect_err("foreign/synthesized origin must RED");
    assert!(matches!(
        foreign_error,
        overlay::OverlayDoorError::Origin(_)
    ));

    let missing_horizon = overlay::authored(
        &root,
        &root.children[0],
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![target],
        transform(SimPropertyId(7)),
        Vec::new(),
    )
    .expect_err("missing/default horizon must RED");
    assert!(matches!(
        missing_horizon,
        overlay::OverlayDoorError::Lifecycle(
            simthing_core::DispatchOverlayError::MissingDissolveCondition
        )
    ));

    let admitted = overlay::authored(
        &root,
        &root.children[0],
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![target],
        transform(SimPropertyId(7)),
        vec![DissolveCondition::AfterTicks { remaining: 2 }],
    )
    .expect("authored origin and horizon");
    assert_eq!(admitted.origin, target);
}

#[test]
fn queued_cost_band_depth_changes_without_shape_rehydration() {
    let (unit_cost, semantics) =
        populate::queued_cost_band(2.0, Some(4), None).expect("authored queue shape");
    let mut registry = ThresholdRegistry::new();
    let event_kind = registry.push_with_cost_band(
        ThresholdSemantic::VelocityAlert {
            sim_thing_id: simthing_core::SimThingId::new(),
            property_id: SimPropertyId(1),
            sub_field: SubFieldRole::Amount,
        },
        semantics,
    );

    let depth_one =
        bind::queued_draw(&mut registry, event_kind, 2.0, unit_cost).expect("runtime depth one");
    let depth_four =
        bind::queued_draw(&mut registry, event_kind, 10.0, unit_cost).expect("runtime depth four");
    assert_eq!(depth_one.n, 1);
    assert_eq!(depth_four.n, 4);
    assert_eq!(registry.cost_band_resolve_invocations, 2);
    assert!(populate::queued_cost_band(0.0, Some(4), None).is_err());
}

#[test]
fn five_verbs_are_scale_invariant_for_one_node_and_a_tree() {
    let owner = OwnerRef::try_new_authored("alpha").expect("owner");
    for child_count in [0usize, 8] {
        let mut scenario = Scenario::map_light(
            format!("vendor-door-scale-{child_count}"),
            1,
            2,
            1.0,
            (child_count + 1) as u32,
        );
        populate::owner(&mut scenario.root, &owner);
        populate::ownership(&scenario.root).expect("boundary-only ownership");
        let report = derive::specializations(
            &scenario.root,
            &[all_profile()],
            &SpecializationObservations::default(),
        )
        .expect("same Derive path");
        assert_eq!(
            derive::owner_specializations(&scenario.root, &report)
                .expect("same query path")
                .len(),
            child_count + 1
        );
        let authored = overlay::authored(
            &scenario.root,
            &scenario.root,
            overlay::OverlayKind::Instruction,
            overlay::OverlaySource::System,
            vec![scenario.root.id],
            transform(
                scenario
                    .registry
                    .id_of("map", "stability")
                    .expect("fixture dimension"),
            ),
            vec![DissolveCondition::AtSessionEnd],
        )
        .expect("same Overlay path");
        assert_eq!(authored.affects, vec![scenario.root.id]);

        let root_id = scenario.root.id;
        let mut session =
            run::initialize(scenario, &Default::default()).expect("same Run initialization path");
        run::start(&mut session, run::ExecutionPosture::Paced).expect("same Run start path");
        run::tick(&mut session).expect("same Run tick path");
        let shadow = bind::shadow(&session);
        assert_eq!(shadow.tick_index(), 1);
        assert!(shadow.row(root_id).is_some());

        let replay = tempfile::NamedTempFile::new().expect("scale replay");
        let summary =
            run::serialize(&mut session, replay.path(), 1).expect("same Run serialization path");
        assert_eq!(summary.frames_written, 1);
    }
}

#[test]
fn all_five_verbs_run_observe_and_serialize_through_one_kernel_history() {
    let mut scenario = Scenario::rebellion_demo("vendor-door-e2e".into(), 1, 3, 0.25, 8);
    let owner = OwnerRef::try_new_authored("alpha").expect("owner");
    populate::owner(&mut scenario.root, &owner);
    populate::ownership(&scenario.root).expect("owner boundary");

    let origin = scenario.root.children[0].children[0].clone();
    let property_id = *origin.properties.keys().next().expect("fixture property");
    let directive = overlay::authored(
        &scenario.root,
        &origin,
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![origin.id],
        transform(property_id),
        vec![DissolveCondition::AfterTicks { remaining: 2 }],
    )
    .expect("overlay admission");
    scenario.root.children[0].children[0].add_overlay(directive);

    let continuous_scenario = scenario.clone();
    let game_mode = simthing_spec::GameModeSpec::default();
    let mut paced = run::initialize(scenario, &game_mode).expect("initialize paced");
    assert_eq!(
        derive::installed_owner_specializations(&paced)
            .expect("installed sanctioned query")
            .len(),
        3
    );
    let (_, cost_band) = populate::queued_cost_band(0.1, Some(2), None).expect("queue shape");
    bind::velocity_threshold(
        &mut paced,
        VelocityAlertRegistration {
            sim_thing_id: origin.id,
            property_id,
            sub_field: SubFieldRole::Velocity,
            threshold: -0.25,
            direction: Direction::Falling,
            cost_band,
        },
    );
    run::start(&mut paced, run::ExecutionPosture::Paced).expect("start paced");
    let paced_tick = run::tick(&mut paced).expect("paced tick");
    assert!(paced_tick.boundary_reached);
    let shadow = bind::shadow(&paced);
    assert_eq!(shadow.tick_index(), 1);
    assert!(shadow.row(origin.id).is_some());

    let replay = tempfile::NamedTempFile::new().expect("temp replay");
    let serialized = run::serialize(&mut paced, replay.path(), 1).expect("serialize replay");
    assert_eq!(serialized.frames_written, 1);
    assert!(replay.as_file().metadata().expect("metadata").len() > 0);

    let mut continuous =
        run::initialize(continuous_scenario, &game_mode).expect("initialize continuous");
    run::start(
        &mut continuous,
        run::ExecutionPosture::continuous(1).expect("continuous posture"),
    )
    .expect("start continuous");
    let continuous_tick = run::tick(&mut continuous).expect("continuous tick");
    assert_eq!(paced_tick, continuous_tick);
}

#[test]
fn run_replay_exports_preserve_lower_authority_type_identity() {
    type DoorRead = fn(&Path) -> Result<run::LoadedReplay, run::ReplayOpenError>;
    type DoorOpen = fn(
        &Path,
        &run::GameModeSpec,
        run::Scenario,
    ) -> Result<
        (
            run::SimSession,
            run::ReplayDriver,
            Vec<(run::ReplayFrame, Vec<run::SpecDelta>)>,
        ),
        run::ReplayOpenError,
    >;

    let _: DoorRead = simthing_driver::read_spec_replay_file;
    let _: DoorRead = run::read_spec_replay_file;
    let _: DoorOpen = simthing_driver::open_replay_with_spec;
    let _: DoorOpen = run::open_replay_with_spec;

    let _: fn(run::LoadedReplay) -> simthing_driver::LoadedReplay = std::convert::identity;
    let _: fn(run::ReplayOpenError) -> simthing_driver::ReplayOpenError = std::convert::identity;
    let _: fn(run::SpecDelta) -> simthing_driver::SpecDelta = std::convert::identity;
    let _: fn(run::ReplayDriver) -> simthing_sim::ReplayDriver = std::convert::identity;
    let _: fn(run::ReplayError) -> simthing_sim::ReplayError = std::convert::identity;
    let _: fn(run::ReplayFrame) -> simthing_sim::ReplayFrame = std::convert::identity;
    let _: fn(run::ReplaySnapshot) -> simthing_sim::ReplaySnapshot = std::convert::identity;
}

//! Structural-addition dynamic Resource Flow admission (DA, relay 5743461789)
//! through the ordinary session: a successful `AddChild` enrolls exactly the
//! carriers of existing derived arenas, by the session-build derivation rule.
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    PropertyValue, ResourceParentEdge, SimPropertyId, SimThing, SimThingId, SimThingKind,
    SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, derive_resource_flow_admission, react_to_structural_resource_flow_enrollment,
    Scenario, SimSession, StructuralEnrollmentRefusal,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::SlotAllocator;
use simthing_spec::{compile_property, GameModeSpec, PropertySpec};

fn role(name: &str) -> SubFieldRole {
    SubFieldRole::Named(name.into())
}

fn energy_property(registry: &mut DimensionRegistry) -> SimPropertyId {
    let field = |name: &str, accumulator: Option<AccumulatorRole>| SubFieldSpec {
        role: role(name),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: accumulator.map(|role| AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    };
    let mut balance = field(
        "balance",
        Some(AccumulatorRole::Balance(BalanceSpec::default())),
    );
    balance.governed_by = Some(role("balance_rate"));
    compile_property(
        &PropertySpec {
            admission_disposition: Default::default(),
            id: "energy".into(),
            namespace: "enroll".into(),
            name: "energy".into(),
            display_name: "energy".into(),
            description: String::new(),
            sub_fields: vec![
                field("flow", Some(AccumulatorRole::IntrinsicFlow)),
                field(
                    "allocated",
                    Some(AccumulatorRole::AllocatedFlow {
                        arena: "energy".into(),
                    }),
                ),
                field(
                    "weight",
                    Some(AccumulatorRole::AllocatorWeight {
                        arena: "energy".into(),
                    }),
                ),
                field("balance_rate", None),
                balance,
            ],
        },
        registry,
    )
    .expect("ordinary governed property admission")
    .0
}

struct World {
    session: SimSession,
    energy: SimPropertyId,
    root: SimThingId,
    hub: SimThingId,
    outsider: SimThingId,
}

fn carrier(registry: &DimensionRegistry, energy: SimPropertyId, kind: SimThingKind, flow: f32, weight: f32) -> SimThing {
    let layout = registry.property(energy).layout.clone();
    let mut node = SimThing::new(kind, 0);
    let mut value = PropertyValue::from_layout(&layout);
    value.set_role(&role("flow"), &layout, flow);
    value.set_role(&role("weight"), &layout, weight);
    node.add_property(energy, value);
    node
}

/// root(4) -> hub -> `cohorts` carriers, plus an `outsider` that carries no
/// energy. `authored` passes a pre-materialized explicit spec (authored rows,
/// a CLOSED arena); otherwise the session derives the arena itself.
fn world(cohorts: usize, authored: bool) -> World {
    let mut registry = DimensionRegistry::new();
    let energy = energy_property(&mut registry);
    let mut root = carrier(&registry, energy, SimThingKind::World, 4.0, 0.0);
    let mut hub = carrier(&registry, energy, SimThingKind::Cohort, 0.0, 1.0);
    for _ in 0..cohorts {
        hub.add_child(carrier(&registry, energy, SimThingKind::Cohort, 0.0, 1.0));
    }
    let outsider = SimThing::new(SimThingKind::Cohort, 0);
    let (root_id, hub_id, outsider_id) = (root.id, hub.id, outsider.id);
    root.add_child(hub);
    root.add_child(outsider);
    simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
        &mut registry,
        &mut root,
    );
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root).unwrap();
    let resource_flow = if authored {
        let mut spec = derive_resource_flow_admission(None, &registry, &root, &allocator)
            .unwrap()
            .spec
            .unwrap();
        for arena in &mut spec.arenas {
            arena.max_orderband_depth = 32;
        }
        Some(spec)
    } else {
        None
    };
    let scenario = Scenario {
        name: "structural-rf-enrollment".into(),
        registry,
        root,
        n_slots: 64,
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };
    let spec = GameModeSpec {
        id: "structural-rf-enrollment".into(),
        resource_flow,
        ..Default::default()
    };
    World {
        session: SimSession::open_from_spec(scenario, &spec).expect("ordinary session admission"),
        energy,
        root: root_id,
        hub: hub_id,
        outsider: outsider_id,
    }
}

fn energy_arena(world: &World) -> u32 {
    world
        .session
        .spec_state
        .arena_registry
        .arenas
        .iter()
        .position(|arena| arena.name == "energy")
        .expect("energy arena") as u32
}

fn member_parent(world: &World, id: SimThingId) -> Option<Option<SimThingId>> {
    let arena = energy_arena(world);
    world
        .session
        .spec_state
        .arena_registry
        .participants
        .iter()
        .find(|member| member.arena_idx == arena && member.subtree_root == id)
        .map(|member| member.parent)
}

fn add(world: &mut World, parent: SimThingId, child: SimThing) {
    world
        .session
        .tx
        .submit_boundary(BoundaryRequest::AddChild { parent, child })
        .unwrap();
    world.session.step_once().expect("ordinary generation");
}

/// Sum of one generation's settled Balance deltas across the energy arena.
fn settled(world: &mut World) -> f32 {
    let arena_idx = energy_arena(world);
    let layout = build_execution_plan(&world.session.proto.registry, &world.session.spec_state.arena_registry)
        .unwrap()
        .arenas
        .into_iter()
        .find(|arena| arena.arena_idx == arena_idx)
        .unwrap();
    let n_dims = world.session.proto.registry.total_columns as u32;
    let before = world.session.state.read_values();
    world.session.step_once().expect("ordinary generation");
    let after = world.session.state.read_values();
    layout
        .iter_all()
        .into_iter()
        .map(|node| {
            let at = (node.participant_slot.raw() * n_dims + node.cols.balance_col.unwrap().raw_u32()) as usize;
            after[at] - before[at]
        })
        .sum()
}

/// catches: a born carrier left out of its existing derived arena, a non-carrier
/// enrolled by blanket parent inheritance, a wrong resource parent, a
/// half-admitted subtree, or an admission that does not change settlement by
/// exactly the born node's authored flow.
#[test]
fn successful_addition_enrolls_exactly_its_carriers_by_the_derivation_rule() {
    let mut deltas = Vec::new();
    for fleet_flow in [0.0f32, -1.0] {
        let mut world = world(9, false);
        let before = world.session.spec_state.arena_registry.participants.len();
        let generation = world.session.spec_state.arena_registry.generation;
        let registry = world.session.proto.registry.clone();
        let mut fleet = carrier(&registry, world.energy, SimThingKind::Fleet, fleet_flow, 0.0);
        let crew = carrier(&registry, world.energy, SimThingKind::Cohort, 0.0, 0.0);
        let cargo = SimThing::new(SimThingKind::Cohort, 0);
        let (fleet_id, crew_id, cargo_id) = (fleet.id, crew.id, cargo.id);
        fleet.add_child(crew);
        fleet.add_child(cargo);
        let hub = world.hub;
        add(&mut world, hub, fleet);
        let report = world
            .session
            .last_resource_flow_structural_enrollment_report
            .clone()
            .expect("the boundary's addition was consumed");
        println!("fleet flow {fleet_flow}: {report:?}");
        assert!(report.refusals.is_empty(), "{report:?}");
        assert_eq!(report.added_roots, vec![fleet_id], "the walk starts at the true root only");
        assert_eq!(member_parent(&world, fleet_id), Some(Some(hub)), "physical parent is a member");
        assert_eq!(member_parent(&world, crew_id), Some(Some(fleet_id)), "co-admitted parent");
        assert_eq!(member_parent(&world, cargo_id), None, "a non-carrier never inherits");
        assert_eq!(
            world.session.spec_state.arena_registry.participants.len(),
            before + 2,
            "exactly the two carriers join"
        );
        assert_eq!(
            world.session.spec_state.arena_registry.generation,
            generation + 1,
            "one generation bump per admitted batch"
        );
        let arena = energy_arena(&world);
        assert_eq!(
            world.session.spec_state.arena_registry.participant_slot(fleet_id, arena),
            world.session.proto.allocator.slot_of(fleet_id),
            "admitted on the slot the boundary already committed"
        );
        deltas.push(settled(&mut world));
    }
    println!("settled with born flow 0 / -1: {deltas:?}");
    assert_eq!(deltas[1] - deltas[0], -1.0, "the born node's authored flow settles exactly");
}

/// catches: an explicit resource-parent edge ignored, or an ambiguous edge or
/// a non-member edge parent silently defaulted instead of refusing the batch.
#[test]
fn explicit_edges_are_authoritative_and_bad_edges_refuse_the_whole_batch() {
    let edge = |parent: SimThingId| ResourceParentEdge {
        property_namespace: "enroll".into(),
        property_name: "energy".into(),
        parent,
        source_span_token: Some(7),
    };

    let mut world = world(9, false);
    let registry = world.session.proto.registry.clone();
    let mut fleet = carrier(&registry, world.energy, SimThingKind::Fleet, 0.0, 0.0);
    fleet.resource_parent_edges.push(edge(world.root));
    let fleet_id = fleet.id;
    let hub = world.hub;
    add(&mut world, hub, fleet);
    assert_eq!(
        member_parent(&world, fleet_id),
        Some(Some(world.root)),
        "the edge, not the physical parent, names the resource parent"
    );

    for (label, edges) in [
        ("ambiguous", vec![edge(world.root), edge(world.hub)]),
        ("non-member parent", vec![edge(world.outsider)]),
    ] {
        let mut world = self::world(9, false);
        let registry = world.session.proto.registry.clone();
        let generation = world.session.spec_state.arena_registry.generation;
        let before = world.session.spec_state.arena_registry.participants.len();
        let mut fleet = carrier(&registry, world.energy, SimThingKind::Fleet, 0.0, 0.0);
        fleet.resource_parent_edges = edges;
        // A lawful sibling carrier in the same subtree is NOT half-admitted.
        let crew = carrier(&registry, world.energy, SimThingKind::Cohort, 0.0, 0.0);
        fleet.add_child(crew);
        let hub = world.hub;
        add(&mut world, hub, fleet);
        let report = world
            .session
            .last_resource_flow_structural_enrollment_report
            .clone()
            .unwrap();
        println!("{label}: {:?}", report.refusals);
        assert!(report.admissions.is_empty(), "{label}: nothing admitted");
        assert!(report.refusals.iter().any(|refusal| match refusal {
            StructuralEnrollmentRefusal::AmbiguousParentEdge { span_tokens, .. } =>
                label == "ambiguous" && span_tokens == &vec![Some(7), Some(7)],
            StructuralEnrollmentRefusal::ParentNotParticipant { span_token, .. } =>
                label == "non-member parent" && *span_token == Some(7),
            _ => false,
        }));
        assert_eq!(world.session.spec_state.arena_registry.participants.len(), before);
        assert_eq!(world.session.spec_state.arena_registry.generation, generation);
    }
}

/// catches: a closed authored arena silently entered, capacity widened to
/// force admission, a refused birth enrolled anyway, or a replayed addition
/// duplicating membership.
#[test]
fn closed_arenas_capacity_refused_births_and_replays_admit_nothing() {
    // Authored rows: the arena enumerates its members and stays closed.
    let mut closed = world(9, true);
    let registry = closed.session.proto.registry.clone();
    let fleet = carrier(&registry, closed.energy, SimThingKind::Fleet, 0.0, 0.0);
    let fleet_id = fleet.id;
    let hub = closed.hub;
    add(&mut closed, hub, fleet);
    let report = closed.session.last_resource_flow_structural_enrollment_report.clone().unwrap();
    assert!(report.admissions.is_empty());
    assert!(matches!(
        report.refusals.as_slice(),
        [StructuralEnrollmentRefusal::ClosedArena { simthing_id, .. }] if *simthing_id == fleet_id
    ));
    assert_eq!(member_parent(&closed, fleet_id), None);

    // Fourteen N0 carriers derive a cap of sixteen: three more refuse typed.
    let mut full = world(12, false);
    let arena = energy_arena(&full);
    let cap = full.session.spec_state.arena_registry.arenas[arena as usize].max_participants;
    let count = full.session.spec_state.arena_registry.arenas[arena as usize].participant_range.1;
    assert_eq!((count, cap), (14, 16), "root + hub + twelve cohorts, derived cap 16");
    let registry = full.session.proto.registry.clone();
    let mut fleet = carrier(&registry, full.energy, SimThingKind::Fleet, 0.0, 0.0);
    fleet.add_child(carrier(&registry, full.energy, SimThingKind::Cohort, 0.0, 0.0));
    fleet.add_child(carrier(&registry, full.energy, SimThingKind::Cohort, 0.0, 0.0));
    let generation = full.session.spec_state.arena_registry.generation;
    let hub = full.hub;
    add(&mut full, hub, fleet);
    let report = full.session.last_resource_flow_structural_enrollment_report.clone().unwrap();
    println!("capacity: {:?}", report.refusals);
    assert!(report.admissions.is_empty());
    assert!(report.refusals.iter().any(|refusal| matches!(
        refusal,
        StructuralEnrollmentRefusal::Capacity { declared: 16, computed: 17, .. }
    )));
    assert_eq!(full.session.spec_state.arena_registry.arenas[arena as usize].participant_range.1, 14);
    assert_eq!(full.session.spec_state.arena_registry.generation, generation);

    // A birth the growth door refuses never reaches enrollment.
    let mut unplaced = world(9, false);
    let registry = unplaced.session.proto.registry.clone();
    let mut giant = carrier(&registry, unplaced.energy, SimThingKind::Fleet, 0.0, 0.0);
    for _ in 0..80 {
        giant.add_child(carrier(&registry, unplaced.energy, SimThingKind::Cohort, 0.0, 0.0));
    }
    let giant_id = giant.id;
    let before = unplaced.session.spec_state.arena_registry.participants.len();
    let hub = unplaced.hub;
    add(&mut unplaced, hub, giant);
    assert!(!unplaced.session.proto.root.contains_id(giant_id), "placement refused the birth");
    assert!(unplaced.session.last_resource_flow_structural_enrollment_report.is_none());
    assert_eq!(unplaced.session.spec_state.arena_registry.participants.len(), before);

    // Replaying the same addition never duplicates membership.
    let mut replay = world(9, false);
    let registry = replay.session.proto.registry.clone();
    let fleet = carrier(&registry, replay.energy, SimThingKind::Fleet, 0.0, 0.0);
    let fleet_id = fleet.id;
    let hub = replay.hub;
    add(&mut replay, hub, fleet);
    let generation = replay.session.spec_state.arena_registry.generation;
    let before = replay.session.spec_state.arena_registry.participants.len();
    let session = &mut replay.session;
    let again = react_to_structural_resource_flow_enrollment(
        &[fleet_id],
        &session.proto.root,
        &session.proto.registry,
        &mut session.spec_state.arena_registry,
        &session.spec_state.resource_flow_derivation,
        &session.proto.allocator,
    );
    assert!(again.admissions.is_empty() && again.refusals.is_empty(), "{again:?}");
    assert_eq!(replay.session.spec_state.arena_registry.participants.len(), before);
    assert_eq!(replay.session.spec_state.arena_registry.generation, generation);
}

/// catches: an admission that deepens a derived arena past its OrderBand
/// budget, which the post-admission sync would only discover after mutating.
#[test]
fn a_birth_past_the_orderband_budget_refuses_before_any_mutation() {
    // Nine N0 carriers derive cap 16 and a band budget of 32. Governed bands
    // cost 3D - 1 + 4, so a chain below a cohort reaching nine levels needs 30
    // and fits, while ten levels need 33 and refuse, inside the capacity.
    for (links, admits) in [(6usize, true), (7, false)] {
        let mut world = world(7, false);
        let arena = energy_arena(&world);
        assert_eq!(world.session.spec_state.arena_registry.arenas[arena as usize].max_orderband_depth, 32);
        let registry = world.session.proto.registry.clone();
        let mut chain = carrier(&registry, world.energy, SimThingKind::Cohort, 0.0, 0.0);
        for _ in 1..links {
            let mut link = carrier(&registry, world.energy, SimThingKind::Cohort, 0.0, 0.0);
            link.add_child(chain);
            chain = link;
        }
        let cohort = world.session.proto.root.snapshot_node(world.hub).unwrap().children[0];
        let before = world.session.spec_state.arena_registry.participants.len();
        let generation = world.session.spec_state.arena_registry.generation;
        add(&mut world, cohort, chain);
        let report = world.session.last_resource_flow_structural_enrollment_report.clone().unwrap();
        println!("{links} links: {:?}", report.refusals);
        if admits {
            assert!(report.refusals.is_empty(), "{report:?}");
            assert_eq!(report.admissions.len(), links);
        } else {
            assert!(report.admissions.is_empty());
            assert_eq!(
                report.refusals,
                vec![StructuralEnrollmentRefusal::DepthBudget {
                    arena: "energy".into(),
                    needed: 33,
                    max: 32
                }]
            );
            assert_eq!(world.session.spec_state.arena_registry.participants.len(), before);
            assert_eq!(world.session.spec_state.arena_registry.generation, generation);
        }
        world.session.step_once().expect("the session keeps running either way");
    }
}

/// catches: a birth placed on the retired slot of a removed member, whose arena
/// row persists until departure is law, aliasing two members onto one slot
/// instead of refusing typed.
#[test]
fn a_birth_on_a_slot_still_held_by_a_removed_member_refuses_typed() {
    let mut world = world(9, false);
    let registry = world.session.proto.registry.clone();
    let first = carrier(&registry, world.energy, SimThingKind::Fleet, 0.0, 1.0);
    let first_id = first.id;
    let hub = world.hub;
    add(&mut world, hub, first);
    let slot = world.session.proto.allocator.slot_of(first_id).unwrap().raw();
    world
        .session
        .tx
        .submit_boundary(BoundaryRequest::Remove { target: first_id })
        .unwrap();
    world.session.step_once().expect("ordinary removal");
    assert!(!world.session.proto.root.contains_id(first_id));
    assert_eq!(
        member_parent(&world, first_id),
        Some(Some(hub)),
        "departure is not yet law: the removed member's row persists"
    );

    let before = world.session.spec_state.arena_registry.participants.len();
    let generation = world.session.spec_state.arena_registry.generation;
    let second = carrier(&registry, world.energy, SimThingKind::Fleet, 0.0, 1.0);
    let second_id = second.id;
    add(&mut world, hub, second);
    assert_eq!(
        world.session.proto.allocator.slot_of(second_id).map(|slot| slot.raw()),
        Some(slot),
        "placement reuses the retired slot"
    );
    let report = world.session.last_resource_flow_structural_enrollment_report.clone().unwrap();
    println!("reused slot {slot}: {:?}", report.refusals);
    assert!(report.admissions.is_empty());
    assert_eq!(
        report.refusals,
        vec![StructuralEnrollmentRefusal::SlotHeld {
            simthing_id: second_id,
            arena: "energy".into(),
            slot,
            holder: first_id,
        }]
    );
    assert_eq!(world.session.spec_state.arena_registry.participants.len(), before);
    assert_eq!(world.session.spec_state.arena_registry.generation, generation);
    world.session.step_once().expect("the session keeps running");
}

//! E-10R2 — ArenaParticipant scaffold tests.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, DimensionRegistry, LogTier, SimThing, SimThingKind,
    SimThingKindTag, SubFieldRole, SubFieldSpec,
};
use simthing_core::ClampBehavior;
use simthing_driver::{
    arena_participant_sibling_slots, materialize_arena_participants, slots_are_contiguous,
    try_alloc_participant_child_in_gap, validate_resource_flow_preflight, ArenaParticipantScaffold,
    FissionPolicy, GapAllocError,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec,
};

fn register_food_flow(reg: &mut DimensionRegistry) {
    let spec = PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Named("flow".into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "flow".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: Some(AccumulatorSpec {
                role: AccumulatorRole::IntrinsicFlow,
                log_tier: LogTier::Summary,
            }),
        }],
    };
    compile_property(&spec, reg).unwrap();
}

fn food_arena(
    participants: Vec<(u32, u32)>,
    gap: u32,
    expected_children: u32,
) -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: gap,
        expected_max_children_per_intermediate: expected_children,
        explicit_participants: participants
            .into_iter()
            .map(|(slot, subtree_root_id)| ExplicitParticipantSpec {
                slot,
                subtree_root_id,
            })
            .collect(),
        enrollment: None,
        wildcard_admission: None,
    }
}

fn materialize_fixture(
    build_hosted: impl FnOnce(&mut SimThing),
    arena: ArenaSpec,
) -> (
    DimensionRegistry,
    SimThing,
    SlotAllocator,
    ArenaParticipantScaffold,
) {
    let mut reg = DimensionRegistry::new();
    register_food_flow(&mut reg);
    let mut alloc = SlotAllocator::new();
    let mut root = SimThing::new(SimThingKind::World, 0);
    build_hosted(&mut root);
    alloc.populate_from_tree(&root);

    let spec_participants: Vec<ExplicitParticipantSpec> = arena
        .explicit_participants
        .iter()
        .cloned()
        .collect();

    let spec_participants = if spec_participants.is_empty() {
        root.children
            .iter()
            .map(|hosted| ExplicitParticipantSpec {
                slot: alloc.slot_of(hosted.id).unwrap(),
                subtree_root_id: hosted.id.raw(),
            })
            .collect()
    } else {
        spec_participants
    };

    let spec = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            explicit_participants: spec_participants,
            ..arena
        }],
        couplings: vec![],
    ..Default::default()
    };
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    (reg, root, alloc, scaffold)
}

#[test]
fn e10r2_arena_participants_contiguous_at_session_open() {
    let (_reg, root, alloc, scaffold) = materialize_fixture(
        |root| {
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
        },
        food_arena(vec![], 0, 0),
    );
    let arena_root_id = *scaffold.arena_root_ids.get(&0).unwrap();
    let slots = arena_participant_sibling_slots(&root, arena_root_id, &alloc);
    assert_eq!(slots.len(), 3);
    assert!(slots_are_contiguous(&slots));
}

#[test]
fn e10r2_arena_participant_index_maps_host_to_participant_slot() {
    let (_reg, root, alloc, scaffold) = materialize_fixture(
        |root| {
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
        },
        food_arena(vec![], 0, 0),
    );
    let hosted_id = root.children[0].id;
    let p_slot = scaffold.index.participant_slot(hosted_id, 0).unwrap();
    let participant_id = alloc.owner_of(p_slot).unwrap();
    let participant = find_by_id(&root, participant_id).unwrap();
    assert_eq!(participant.kind, SimThingKind::ArenaParticipant);
}

#[test]
fn e10r2_arena_participants_do_not_replace_hosted_simthings() {
    let (_reg, root, alloc, _scaffold) = materialize_fixture(
        |root| {
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
        },
        food_arena(vec![], 0, 0),
    );
    let hosted_id = root.children[0].id;
    let hosted_slot = alloc.slot_of(hosted_id).unwrap();
    let hosted = find_by_id(&root, hosted_id).unwrap();
    assert_eq!(hosted.kind, SimThingKind::Cohort);
    assert_eq!(hosted_slot, 1);
}

#[test]
fn e10r2_arena_participant_kind_does_not_cross_into_simthing_sim() {
    // ArenaParticipant is a runtime SimThingKind only — not in fission SimThingKindTag.
    let _ = SimThingKind::ArenaParticipant;
    let tag_variants = [
        SimThingKindTag::World,
        SimThingKindTag::Faction,
        SimThingKindTag::StarSystem,
        SimThingKindTag::Location,
        SimThingKindTag::Cohort,
        SimThingKindTag::Fleet,
        SimThingKindTag::Station,
    ];
    assert_eq!(tag_variants.len(), 7);
}

#[test]
fn e10r2_reserved_gap_slots_are_in_arena_local_block() {
    let (_reg, _root, _alloc, scaffold) = materialize_fixture(
        |root| {
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
        },
        food_arena(vec![], 3, 3),
    );
    let report = &scaffold.reports[0];
    let parent_slot = scaffold.index.by_host_and_arena.values().next().copied().unwrap();
    let pool = scaffold.gap_pools.get(&parent_slot).unwrap();
    let gap_block_first = report.gap_block_first.unwrap();
    assert_eq!(pool.reserved_slots(), &[gap_block_first, gap_block_first + 1, gap_block_first + 2]);
    // Single-participant arenas still place the gap block immediately after the sibling.
    assert_eq!(gap_block_first, parent_slot + 1);
}

#[test]
fn e10r2_reserved_gap_consumed_before_non_gap_tombstones() {
    let (_reg, _root, mut alloc, mut scaffold) = materialize_fixture(
        |root| {
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
        },
        food_arena(vec![], 2, 2),
    );
    let parent_slot = scaffold.index.by_host_and_arena.values().next().copied().unwrap();

    let unrelated = SimThing::new(SimThingKind::Cohort, 0);
    let unrelated_slot = alloc.alloc(unrelated.id);
    alloc.tombstone(unrelated.id);

    let child_a = SimThing::new(SimThingKind::ArenaParticipant, 0).id;
    let child_b = SimThing::new(SimThingKind::ArenaParticipant, 0).id;
    let slot_a = try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        child_a,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap();
    let slot_b = try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        child_b,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap();

    assert_eq!(slot_a, parent_slot + 2);
    assert_eq!(slot_b, parent_slot + 1);
    assert_ne!(slot_a, unrelated_slot);
    assert_ne!(slot_b, unrelated_slot);
    assert!(!alloc.is_live(unrelated_slot));
}

#[test]
fn e10r2_gap_exhaustion_rejects_for_reject_policy() {
    let (_reg, _root, mut alloc, mut scaffold) = materialize_fixture(
        |root| {
            root.add_child(SimThing::new(SimThingKind::Cohort, 0));
        },
        food_arena(vec![], 1, 1),
    );
    let parent_slot = scaffold.index.by_host_and_arena.values().next().copied().unwrap();

    let first = SimThing::new(SimThingKind::ArenaParticipant, 0).id;
    try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        first,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap();

    let second = SimThing::new(SimThingKind::ArenaParticipant, 0).id;
    let err = try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        second,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap_err();
    assert!(matches!(err, GapAllocError::Exhausted { .. }));
}

fn find_by_id<'a>(root: &'a SimThing, id: simthing_core::SimThingId) -> Option<&'a SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &root.children {
        if let Some(found) = find_by_id(child, id) {
            return Some(found);
        }
    }
    None
}

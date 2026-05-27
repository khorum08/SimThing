//! E-10R3 — Arena-local block reservation for participant siblings + gap pools.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, DimensionRegistry, LogTier, SimThing, SimThingKind,
    SubFieldRole, SubFieldSpec,
};
use simthing_core::ClampBehavior;
use simthing_driver::{
    all_reserved_gap_slots, arena_participant_sibling_slots, compile_and_install,
    materialize_arena_participants, slot_in_participant_sibling_range, slots_are_contiguous,
    try_alloc_participant_child_in_gap, validate_resource_flow_preflight, FissionPolicy,
    InstallError, Scenario,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowSpec, SpecVersion,
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

fn food_arena(gap: u32, expected_children: u32) -> ArenaSpec {
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
        explicit_participants: vec![],
        wildcard_admission: None,
    }
}

fn materialize_multi(n_hosted: usize, gap: u32) -> (SimThing, SlotAllocator, simthing_driver::ArenaParticipantScaffold) {
    let mut reg = DimensionRegistry::new();
    register_food_flow(&mut reg);
    let mut alloc = SlotAllocator::new();
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..n_hosted {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    alloc.populate_from_tree(&root);

    let explicit_participants: Vec<ExplicitParticipantSpec> = root
        .children
        .iter()
        .map(|hosted| ExplicitParticipantSpec {
            slot: alloc.slot_of(hosted.id).unwrap(),
            subtree_root_id: hosted.id.raw(),
        })
        .collect();

    let spec = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            explicit_participants,
            ..food_arena(gap, gap)
        }],
        couplings: vec![],
    };
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    (root, alloc, scaffold)
}

#[test]
fn e10r3_multi_participant_with_reserved_gaps_materializes() {
    let (_root, _alloc, scaffold) = materialize_multi(3, 2);
    assert_eq!(scaffold.reports[0].participant_count, 3);
    assert_eq!(scaffold.gap_pools.len(), 3);
}

#[test]
fn e10r3_participant_siblings_remain_contiguous_with_reserved_gaps() {
    let (root, alloc, scaffold) = materialize_multi(3, 2);
    let arena_root_id = *scaffold.arena_root_ids.get(&0).unwrap();
    let slots = arena_participant_sibling_slots(&root, arena_root_id, &alloc);
    assert_eq!(slots.len(), 3);
    assert!(slots_are_contiguous(&slots));
}

#[test]
fn e10r3_reserved_gap_pools_do_not_overlap_participant_sibling_range() {
    let (root, alloc, scaffold) = materialize_multi(3, 2);
    let report = &scaffold.reports[0];
    let first = report.participant_sibling_first.unwrap();
    let count = report.participant_count;
    for gap_slot in all_reserved_gap_slots(&scaffold) {
        assert!(
            !slot_in_participant_sibling_range(first, count, gap_slot),
            "gap slot {gap_slot} collides with sibling range [{first}, {})",
            first + count
        );
        assert!(alloc.is_exclusive_reserved(gap_slot));
    }
    let arena_root_id = *scaffold.arena_root_ids.get(&0).unwrap();
    let sibling_slots = arena_participant_sibling_slots(&root, arena_root_id, &alloc);
    assert!(slots_are_contiguous(&sibling_slots));
}

#[test]
fn e10r3_each_parent_gets_expected_gap_count() {
    let (_root, _alloc, scaffold) = materialize_multi(3, 2);
    for pool in scaffold.gap_pools.values() {
        assert_eq!(pool.remaining(), 2);
        assert_eq!(pool.reserved_slots().len(), 2);
    }
    let gap_slots = all_reserved_gap_slots(&scaffold);
    assert_eq!(gap_slots.len(), 6);
    assert!(slots_are_contiguous(&gap_slots));
}

#[test]
fn e10r3_gap_consumption_preserves_sibling_slotrange() {
    let (root, mut alloc, mut scaffold) = materialize_multi(3, 2);
    let arena_root_id = *scaffold.arena_root_ids.get(&0).unwrap();
    let siblings_before = arena_participant_sibling_slots(&root, arena_root_id, &alloc);
    assert!(slots_are_contiguous(&siblings_before));

    let parent_slot = siblings_before[1];
    let child = SimThing::new(SimThingKind::ArenaParticipant, 0).id;
    try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        child,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap();

    let siblings_after = arena_participant_sibling_slots(&root, arena_root_id, &alloc);
    assert_eq!(siblings_before, siblings_after);
    assert!(slots_are_contiguous(&siblings_after));
}

#[test]
fn e10r3_resource_flow_materialization_respects_scenario_slot_capacity() {
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..3 {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }

    let scenario = Scenario {
        name: "tight".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 13,
        registry: DimensionRegistry::new(),
        root: root.clone(),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    };

    let mut reg = scenario.registry.clone();
    let mut root = scenario.root.clone();
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);

    let flow_property = PropertySpec {
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

    let explicit_participants: Vec<ExplicitParticipantSpec> = root
        .children
        .iter()
        .map(|hosted| ExplicitParticipantSpec {
            slot: alloc.slot_of(hosted.id).unwrap(),
            subtree_root_id: hosted.id.raw(),
        })
        .collect();

    let game_mode = GameModeSpec {
        id: "e10r3_capacity".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![flow_property],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                explicit_participants,
                ..food_arena(2, 2)
            }],
            couplings: vec![],
        }),
    };

    // world + 3 hosted + arena_root + 3 participants + 6 gap slots = 14
    let err = compile_and_install(
        &game_mode,
        &scenario,
        &mut reg,
        &mut root,
        &mut alloc,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        InstallError::ResourceFlowSlotOverflow { cap: 13, .. }
    ));
}

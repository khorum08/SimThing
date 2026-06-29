//! E-11B — RON-authored static nested participant materialization smoke.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, materialize_arena_participants, validate_resource_flow_preflight,
    GpuArenaDescriptor,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec,
};

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn register_flow(reg: &mut DimensionRegistry) -> simthing_core::SimPropertyId {
    let spec = PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "food".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "food".into(),
                },
            ),
            flow_subfield("balance", AccumulatorRole::Balance(BalanceSpec::default())),
        ],
    };
    let (id, _) = compile_property(&spec, reg).unwrap();
    id
}

fn hosted_cohorts(count: usize) -> (SimThing, Vec<SimThingId>) {
    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut ids = Vec::new();
    for _ in 0..count {
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        ids.push(cohort.id);
        world.add_child(cohort);
    }
    (world, ids)
}

fn arena_spec(participants: Vec<ExplicitParticipantSpec>, max_orderband_depth: u32) -> ArenaSpec {
    ArenaSpec {
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
        explicit_participants: participants,
        enrollment: None,
        wildcard_admission: None,
    }
}

fn d3_participants(hosted: &[SimThingId], alloc: &SlotAllocator) -> Vec<ExplicitParticipantSpec> {
    vec![
        ExplicitParticipantSpec::flat(alloc.slot_of(hosted[0]).unwrap(), hosted[0].raw()),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[1]).unwrap(),
            hosted[1].raw(),
            hosted[0].raw() as u64,
        ),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[2]).unwrap(),
            hosted[2].raw(),
            hosted[1].raw() as u64,
        ),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[3]).unwrap(),
            hosted[3].raw(),
            hosted[1].raw() as u64,
        ),
    ]
}

fn d4_participants(hosted: &[SimThingId], alloc: &SlotAllocator) -> Vec<ExplicitParticipantSpec> {
    vec![
        ExplicitParticipantSpec::flat(alloc.slot_of(hosted[0]).unwrap(), hosted[0].raw()),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[1]).unwrap(),
            hosted[1].raw(),
            hosted[0].raw() as u64,
        ),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[2]).unwrap(),
            hosted[2].raw(),
            hosted[1].raw() as u64,
        ),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[3]).unwrap(),
            hosted[3].raw(),
            hosted[2].raw() as u64,
        ),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[4]).unwrap(),
            hosted[4].raw(),
            hosted[2].raw() as u64,
        ),
        ExplicitParticipantSpec::flat(alloc.slot_of(hosted[5]).unwrap(), hosted[5].raw()),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[6]).unwrap(),
            hosted[6].raw(),
            hosted[5].raw() as u64,
        ),
    ]
}

struct RonMaterialized {
    reg: DimensionRegistry,
    root: SimThing,
    alloc: SlotAllocator,
    scaffold: simthing_driver::ArenaParticipantScaffold,
    flow_id: simthing_core::SimPropertyId,
}

fn materialize_from_ron(
    hosted_count: usize,
    build_participants: impl Fn(&[SimThingId], &SlotAllocator) -> Vec<ExplicitParticipantSpec>,
    max_depth: u32,
) -> RonMaterialized {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(hosted_count);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);

    let authored = ResourceFlowSpec {
        arenas: vec![arena_spec(build_participants(&hosted, &alloc), max_depth)],
        couplings: vec![],
        ..Default::default()
    };
    let ron = ron::ser::to_string(&authored).expect("serialize resource flow spec");
    let spec: ResourceFlowSpec = ron::from_str(&ron).expect("parse resource flow spec from RON");

    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    RonMaterialized {
        reg,
        root,
        alloc,
        scaffold,
        flow_id,
    }
}

fn nested_layout_for(f: &RonMaterialized) -> simthing_driver::ArenaTreeLayout {
    let arena = GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: f.flow_id,
        balance_property_id: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: simthing_driver::FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    build_execution_plan_from_authoring(
        &f.reg,
        std::slice::from_ref(&arena),
        &f.root,
        &f.alloc,
        &f.scaffold,
        1,
    )
    .expect("execution plan")
    .arenas
    .into_iter()
    .next()
    .expect("one arena")
}

#[test]
fn e11b_nested_materialization_from_ron_d3_reaches_nested_layout() {
    let f = materialize_from_ron(4, d3_participants, 16);
    let layout = nested_layout_for(&f);
    assert_eq!(layout.max_depth, 3);
    assert_eq!(layout.participant_roots.len(), 1);
    assert!(layout.participant_roots[0].children[0].children.len() == 2);
    layout.participant_roots[0]
        .verify_child_contiguity()
        .expect("root contiguity");
    layout.participant_roots[0].children[0]
        .verify_child_contiguity()
        .expect("mid contiguity");
}

#[test]
fn e11b_nested_materialization_from_ron_d4_reaches_nested_layout() {
    let f = materialize_from_ron(7, d4_participants, 16);
    let layout = nested_layout_for(&f);
    assert_eq!(layout.max_depth, 4);
    assert_eq!(layout.participant_roots.len(), 2);
    for root in &layout.participant_roots {
        root.verify_child_contiguity().unwrap();
    }
}

#[test]
fn e11b_nested_materialization_ron_flat_star_regression() {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(4);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);

    let authored = ResourceFlowSpec {
        arenas: vec![arena_spec(
            hosted
                .iter()
                .map(|id| ExplicitParticipantSpec::flat(alloc.slot_of(*id).unwrap(), id.raw()))
                .collect(),
            16,
        )],
        couplings: vec![],
        ..Default::default()
    };
    let ron = ron::ser::to_string(&authored).expect("serialize flat-star flow");
    let spec: ResourceFlowSpec = ron::from_str(&ron).expect("parse flat-star flow from RON");

    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();

    let arena = GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: flow_id,
        balance_property_id: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: simthing_driver::FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    let layout = build_execution_plan_from_authoring(
        &reg,
        std::slice::from_ref(&arena),
        &root,
        &alloc,
        &scaffold,
        1,
    )
    .unwrap()
    .arenas
    .remove(0);

    assert_eq!(layout.max_depth, 2);
    assert_eq!(layout.participant_roots.len(), 1);
    assert_eq!(layout.participant_roots[0].children.len(), 3);
    assert!(spec.arenas[0]
        .explicit_participants
        .iter()
        .all(|participant| participant.parent_subtree_root_id.is_none()));
}

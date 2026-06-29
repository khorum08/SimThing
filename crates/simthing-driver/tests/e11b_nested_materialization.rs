//! E-11B-1 explicit nested participant materialization tests.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, materialize_arena_participants, slots_are_contiguous,
    validate_resource_flow_preflight, GpuArenaDescriptor, HierarchyNode,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec, SpecError,
};
use std::path::Path;

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

struct MaterializedFixture {
    reg: DimensionRegistry,
    root: SimThing,
    alloc: SlotAllocator,
    scaffold: simthing_driver::ArenaParticipantScaffold,
    flow_id: simthing_core::SimPropertyId,
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

fn materialize_from_participants(
    hosted_count: usize,
    build_participants: impl Fn(&[SimThingId], &SlotAllocator) -> Vec<ExplicitParticipantSpec>,
    max_depth: u32,
) -> MaterializedFixture {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(hosted_count);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let participants = build_participants(&hosted, &alloc);
    let spec = ResourceFlowSpec {
        arenas: vec![arena_spec(participants, max_depth)],
        couplings: vec![],
        ..Default::default()
    };
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    MaterializedFixture {
        reg,
        root,
        alloc,
        scaffold,
        flow_id,
    }
}

fn layout_for(f: &MaterializedFixture) -> simthing_driver::ArenaTreeLayout {
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

fn assert_child_contiguity(layout: &simthing_driver::ArenaTreeLayout) {
    for root in &layout.participant_roots {
        root.verify_child_contiguity().unwrap();
    }
}

fn direct_child_slots(node: &HierarchyNode) -> Vec<u32> {
    node.children
        .iter()
        .map(|child| child.participant_slot)
        .collect()
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

#[test]
fn e11b_explicit_nested_materialization_flat_star_regression() {
    let mut reg = DimensionRegistry::new();
    register_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(4);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);

    let participants: Vec<_> = hosted
        .iter()
        .map(|id| ExplicitParticipantSpec::flat(alloc.slot_of(*id).unwrap(), id.raw()))
        .collect();
    let spec = ResourceFlowSpec {
        arenas: vec![arena_spec(participants, 16)],
        couplings: vec![],
        ..Default::default()
    };
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();

    let layout = build_execution_plan_from_authoring(
        &reg,
        &[GpuArenaDescriptor {
            name: "food".into(),
            flow_property_id: reg.id_of("core", "food_flow").unwrap(),
            balance_property_id: None,
            max_participants: 16,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: simthing_driver::FissionPolicy::Reject,
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        }],
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
}

#[test]
fn e11b_explicit_nested_materialization_d3_contiguous_per_parent() {
    let f = materialize_from_participants(4, d3_participants, 16);
    let layout = layout_for(&f);
    assert_eq!(layout.max_depth, 3);
    assert_child_contiguity(&layout);

    let root = &layout.participant_roots[0];
    assert_eq!(root.children.len(), 1);
    assert!(slots_are_contiguous(&direct_child_slots(root)));
    let mid = &root.children[0];
    assert_eq!(mid.children.len(), 2);
    assert!(slots_are_contiguous(&direct_child_slots(mid)));
}

#[test]
fn e11b_explicit_nested_materialization_d4_contiguous_per_parent() {
    let f = materialize_from_participants(7, d4_participants, 16);
    let layout = layout_for(&f);
    assert_eq!(layout.max_depth, 4);
    assert_eq!(layout.participant_roots.len(), 2);
    assert_child_contiguity(&layout);

    for root in &layout.participant_roots {
        if !root.children.is_empty() {
            assert!(slots_are_contiguous(&direct_child_slots(root)));
        }
    }
}

#[test]
fn e11b_explicit_nested_materialization_build_execution_plan_uses_nested_layout() {
    let f = materialize_from_participants(4, d3_participants, 16);
    let layout = layout_for(&f);
    assert!(layout.max_depth > 2);
    assert!(layout.participant_roots[0].children[0].children.len() > 0);
    assert_ne!(layout.band_layout.total_bands_used, 5);
}

#[test]
fn e11b_explicit_nested_materialization_missing_parent_rejected() {
    let mut reg = DimensionRegistry::new();
    register_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(2);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let participants = vec![
        ExplicitParticipantSpec::flat(alloc.slot_of(hosted[0]).unwrap(), hosted[0].raw()),
        ExplicitParticipantSpec::nested(alloc.slot_of(hosted[1]).unwrap(), hosted[1].raw(), 9_999),
    ];
    let spec = ResourceFlowSpec {
        arenas: vec![arena_spec(participants, 16)],
        couplings: vec![],
        ..Default::default()
    };
    let err = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap_err();
    let simthing_driver::InstallError::Spec(spec_err) = err else {
        panic!("expected spec error");
    };
    assert!(matches!(
        spec_err,
        SpecError::UnknownExplicitParticipantParent { .. }
    ));
}

#[test]
fn e11b_explicit_nested_materialization_cycle_rejected() {
    let mut reg = DimensionRegistry::new();
    register_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(2);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let participants = vec![
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[0]).unwrap(),
            hosted[0].raw(),
            hosted[1].raw() as u64,
        ),
        ExplicitParticipantSpec::nested(
            alloc.slot_of(hosted[1]).unwrap(),
            hosted[1].raw(),
            hosted[0].raw() as u64,
        ),
    ];
    let spec = ResourceFlowSpec {
        arenas: vec![arena_spec(participants, 16)],
        couplings: vec![],
        ..Default::default()
    };
    let err = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap_err();
    let simthing_driver::InstallError::Spec(spec_err) = err else {
        panic!("expected spec error");
    };
    assert!(matches!(
        spec_err,
        SpecError::ExplicitParticipantParentCycle { .. }
    ));
}

#[test]
fn e11b_explicit_nested_materialization_default_parent_none_roundtrip() {
    let json = r#"{"slot":3,"subtree_root_id":42}"#;
    let parsed: ExplicitParticipantSpec = serde_json::from_str(json).unwrap();
    assert_eq!(parsed.slot, 3);
    assert_eq!(parsed.subtree_root_id, 42);
    assert_eq!(parsed.parent_subtree_root_id, None);
    let encoded = serde_json::to_string(&parsed).unwrap();
    assert!(!encoded.contains("parent_subtree_root_id"));
}

#[test]
fn e11b_explicit_nested_materialization_no_simthing_sim_imports() {
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    let boundary = include_str!("../../simthing-sim/src/boundary.rs");
    for src in [sim_lib, boundary] {
        assert!(!src.contains("ArenaRegistry"));
        assert!(!src.contains("ArenaParticipant"));
        assert!(!src.contains("arena_hierarchy"));
        assert!(!src.contains("resource_flow_compile"));
    }
}

#[test]
fn e11b_explicit_nested_materialization_global_flag_default_false() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}

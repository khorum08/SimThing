//! E-2B-5 — Policy A dynamic fission enrollment tests.

mod support;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SimThing,
    SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    all_reserved_gap_slots, arena_participant_sibling_slots, build_execution_plan,
    materialize_arena_participants, plan_arena_allocation, react_to_fission_resource_flow_enrollment,
    run_flat_star_burn_in, slots_are_contiguous, validate_resource_flow_preflight,
    ArenaRegistryBuilder, FissionPolicy, GpuArenaDescriptor, SimSession,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::{FissionOutcome, PipelineFlags};
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec,
};
use std::collections::HashMap;

use support::e11_flat_star::{try_gpu, standard_flat_star_inputs};

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

fn register_food_flow_with_allocation(reg: &mut DimensionRegistry) {
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
        ],
    };
    compile_property(&spec, reg).unwrap();
}

fn register_research_flow(reg: &mut DimensionRegistry) {
    let spec = PropertySpec {
        id: "research_flow".into(),
        namespace: "core".into(),
        name: "research_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "research".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "research".into(),
                },
            ),
        ],
    };
    compile_property(&spec, reg).unwrap();
}

struct EnrollmentFixture {
    root: SimThing,
    alloc: SlotAllocator,
    reg: DimensionRegistry,
    spec_state: simthing_driver::SpecSessionState,
    parent_id: simthing_core::SimThingId,
    child_id: simthing_core::SimThingId,
}

fn open_enrollment_fixture(
    parent_count: usize,
    max_participants: u32,
    fission_policy: FissionPolicySpec,
    gap: u32,
) -> EnrollmentFixture {
    let mut reg = DimensionRegistry::new();
    register_food_flow_with_allocation(&mut reg);

    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..parent_count {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    let parent_id = root.children[0].id;
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let child_id = child.id;
    root.children[0].add_child(child);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);

    let explicit: Vec<ExplicitParticipantSpec> = root
        .children
        .iter()
        .take(parent_count)
        .map(|hosted| ExplicitParticipantSpec {
            slot: alloc.slot_of(hosted.id).unwrap(),
            subtree_root_id: hosted.id.raw(),
        })
        .collect();

    let spec = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            name: "food".into(),
            flow_property: PropertyKey::new("core", "food_flow"),
            balance_property: None,
            max_participants,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: gap,
            expected_max_children_per_intermediate: gap,
            explicit_participants: explicit,
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
    };

    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    let (arena_registry, _) =
        simthing_driver::compile_and_materialize_resource_flow(&spec, &reg).unwrap();

    let mut spec_state = simthing_driver::SpecSessionState::new();
    spec_state.arena_registry = arena_registry;
    spec_state.arena_participant_scaffold = scaffold;

    EnrollmentFixture {
        root,
        alloc,
        reg,
        spec_state,
        parent_id,
        child_id,
    }
}

fn fission_outcome(
    parent: simthing_core::SimThingId,
    child: simthing_core::SimThingId,
) -> FissionOutcome {
    FissionOutcome {
        fissions_executed: 1,
        fission_pairs: vec![(parent, child)],
        ..Default::default()
    }
}

fn apply_dynamic_enrollment(
    fx: &mut EnrollmentFixture,
) -> simthing_driver::DynamicFissionEnrollmentReport {
    react_to_fission_resource_flow_enrollment(
        &fission_outcome(fx.parent_id, fx.child_id),
        &mut fx.spec_state.arena_registry,
        &mut fx.spec_state.arena_participant_scaffold,
        &mut fx.root,
        &fx.reg,
        &mut fx.alloc,
    )
}

#[test]
fn e2b5_parent_enrolled_child_inherits_arena_membership() {
    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Inherit, 0);
    let gen0 = fx.spec_state.arena_registry.generation;
    let report = apply_dynamic_enrollment(&mut fx);

    assert_eq!(report.admissions.len(), 1);
    assert!(report.rejections.is_empty());
    assert_eq!(report.admissions[0].child_id, fx.child_id);
    assert_eq!(report.generation_after, gen0 + 1);
    assert!(
        fx.spec_state
            .arena_participant_scaffold
            .index
            .participant_slot(fx.child_id, 0)
            .is_some()
    );
}

#[test]
fn e2b5_parent_enrolled_in_two_arenas_child_inherits_both() {
    let mut reg = DimensionRegistry::new();
    register_food_flow_with_allocation(&mut reg);
    register_research_flow(&mut reg);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut parent_node = SimThing::new(SimThingKind::Cohort, 0);
    let parent_id = parent_node.id;
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let child_id = child.id;
    parent_node.add_child(child);
    root.add_child(parent_node);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    for _ in 0..7 {
        alloc.alloc(SimThing::new(SimThingKind::Cohort, 0).id);
    }

    let food_spec = ResourceFlowSpec {
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
            explicit_participants: vec![ExplicitParticipantSpec {
                slot: alloc.slot_of(parent_id).unwrap(),
                subtree_root_id: parent_id.raw(),
            }],
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
    };
    validate_resource_flow_preflight(&food_spec, &alloc).unwrap();
    let mut scaffold = materialize_arena_participants(&food_spec, &reg, &mut root, &mut alloc).unwrap();

    let food_participant_slot = *scaffold.index.by_host_and_arena.get(&(parent_id, 0)).unwrap();
    assert_eq!(food_participant_slot + 1, 12, "fixture layout: food sibling must end at 11");

    for _ in 0..8 {
        alloc.alloc(SimThing::new(SimThingKind::Cohort, 0).id);
    }

    let research_only = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            name: "research".into(),
            flow_property: PropertyKey::new("core", "research_flow"),
            balance_property: None,
            max_participants: 16,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicySpec::Inherit,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants: vec![ExplicitParticipantSpec {
                slot: alloc.slot_of(parent_id).unwrap(),
                subtree_root_id: parent_id.raw(),
            }],
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
    };
    validate_resource_flow_preflight(&research_only, &alloc).unwrap();
    let research_scaffold =
        materialize_arena_participants(&research_only, &reg, &mut root, &mut alloc).unwrap();
    scaffold.reports.extend(research_scaffold.reports);
    if let Some(&research_root) = research_scaffold.arena_root_ids.get(&0) {
        scaffold.arena_root_ids.insert(1, research_root);
    }
    for ((hosted, _), slot) in research_scaffold.index.by_host_and_arena {
        scaffold
            .index
            .by_host_and_arena
            .insert((hosted, 1), slot);
    }
    scaffold.gap_pools.extend(research_scaffold.gap_pools);

    let padding_ids: Vec<_> = (12u32..20)
        .filter_map(|slot| alloc.owner_of(slot))
        .collect();
    for id in padding_ids {
        alloc.tombstone(id);
    }

    let full_spec = ResourceFlowSpec {
        arenas: vec![food_spec.arenas[0].clone(), research_only.arenas[0].clone()],
        couplings: vec![],
    };
    let (arena_registry, _) =
        simthing_driver::compile_and_materialize_resource_flow(&full_spec, &reg).unwrap();

    let mut spec_state = simthing_driver::SpecSessionState::new();
    spec_state.arena_registry = arena_registry;
    spec_state.arena_participant_scaffold = scaffold;

    let report = react_to_fission_resource_flow_enrollment(
        &fission_outcome(parent_id, child_id),
        &mut spec_state.arena_registry,
        &mut spec_state.arena_participant_scaffold,
        &mut root,
        &reg,
        &mut alloc,
    );

    assert_eq!(report.admissions.len(), 2);
    assert!(report.rejections.is_empty());
    assert!(
        spec_state
            .arena_participant_scaffold
            .index
            .participant_slot(child_id, 0)
            .is_some()
    );
    assert!(
        spec_state
            .arena_participant_scaffold
            .index
            .participant_slot(child_id, 1)
            .is_some()
    );
}

#[test]
fn e2b5_reject_policy_does_not_enroll_child() {
    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Reject, 0);
    let report = apply_dynamic_enrollment(&mut fx);

    assert!(report.admissions.is_empty());
    assert_eq!(report.rejections.len(), 1);
    assert!(
        fx.spec_state
            .arena_participant_scaffold
            .index
            .participant_slot(fx.child_id, 0)
            .is_none()
    );
}

#[test]
fn e2b5_max_participants_exceeded_rejects_child() {
    let mut fx = open_enrollment_fixture(1, 1, FissionPolicySpec::Inherit, 0);
    let report = apply_dynamic_enrollment(&mut fx);

    assert!(report.admissions.is_empty());
    assert_eq!(report.rejections.len(), 1);
    assert!(report.rejections[0].reason.contains("max_participants"));
}

#[test]
fn e2b5_contiguity_extension_impossible_rejects_no_compaction() {
    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Inherit, 2);
    let arena_root_id = fx.spec_state.arena_participant_scaffold.arena_root_ids[&0];
    let siblings_before = arena_participant_sibling_slots(&fx.root, arena_root_id, &fx.alloc);
    let report = apply_dynamic_enrollment(&mut fx);

    assert!(report.admissions.is_empty());
    assert_eq!(report.rejections.len(), 1);
    let siblings_after = arena_participant_sibling_slots(&fx.root, arena_root_id, &fx.alloc);
    assert_eq!(siblings_before, siblings_after);
    assert_eq!(siblings_before.len(), 1);
}

#[test]
fn e2b5_appended_child_is_arena_root_sibling() {
    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Inherit, 0);
    apply_dynamic_enrollment(&mut fx);

    let arena_root_id = fx.spec_state.arena_participant_scaffold.arena_root_ids[&0];
    let arena_root = fx
        .root
        .children
        .iter()
        .find(|c| c.id == arena_root_id)
        .expect("arena root");
    let participant_children: Vec<_> = arena_root
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::ArenaParticipant)
        .collect();
    assert_eq!(participant_children.len(), 2);
}

#[test]
fn e2b5_appended_child_preserves_sibling_contiguity() {
    let mut fx = open_enrollment_fixture(2, 16, FissionPolicySpec::Inherit, 0);
    apply_dynamic_enrollment(&mut fx);

    let arena_root_id = fx.spec_state.arena_participant_scaffold.arena_root_ids[&0];
    let siblings = arena_participant_sibling_slots(&fx.root, arena_root_id, &fx.alloc);
    assert_eq!(siblings.len(), 3);
    assert!(slots_are_contiguous(&siblings));
}

#[test]
fn e2b5_gap_pool_untouched_for_flat_star_append() {
    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Inherit, 2);
    let pools_before: HashMap<_, _> = fx
        .spec_state
        .arena_participant_scaffold
        .gap_pools
        .iter()
        .map(|(k, v)| (*k, v.remaining()))
        .collect();
    let gaps_before = all_reserved_gap_slots(&fx.spec_state.arena_participant_scaffold);

    let report = apply_dynamic_enrollment(&mut fx);
    assert!(report.admissions.is_empty());

    let pools_after: HashMap<_, _> = fx
        .spec_state
        .arena_participant_scaffold
        .gap_pools
        .iter()
        .map(|(k, v)| (*k, v.remaining()))
        .collect();
    let gaps_after = all_reserved_gap_slots(&fx.spec_state.arena_participant_scaffold);
    assert_eq!(pools_before, pools_after);
    assert_eq!(gaps_before, gaps_after);
}

#[test]
fn e2b5_registry_generation_increments_on_dynamic_enrollment() {
    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Inherit, 0);
    let gen0 = fx.spec_state.arena_registry.generation;
    let report = apply_dynamic_enrollment(&mut fx);
    assert_eq!(report.generation_before, gen0);
    assert_eq!(report.generation_after, gen0 + 1);
    assert_eq!(fx.spec_state.arena_registry.generation, gen0 + 1);
}

#[test]
fn e2b5_flag_enabled_resync_uploads_ops_including_new_leaf() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut fx = open_enrollment_fixture(2, 16, FissionPolicySpec::Inherit, 0);
    apply_dynamic_enrollment(&mut fx);

    let scenario = support::e11_flat_star::flat_star_scenario(2, 64);
    let mut session = SimSession::open(scenario).expect("open");
    session.proto.root = fx.root;
    session.proto.allocator = fx.alloc;
    session.proto.registry = fx.reg;
    session.spec_state = fx.spec_state;
    session.proto.flags.use_accumulator_resource_flow = true;

    let plan_before = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("plan");
    assert_eq!(plan_before.arenas[0].participant_roots[0].children.len(), 2);

    session.sync_resource_flow_if_enabled().expect("sync");
    let expected_ops = plan_arena_allocation(
        &plan_before.arenas[0],
        &simthing_gpu::build_governed_pairs(&session.proto.registry),
        session.state.n_slots,
    )
    .expect("plan ops")
    .cpu_ops
    .len() as u32;
    assert!(expected_ops > 0);
    assert_eq!(
        session
            .state
            .accumulator_runtime
            .as_ref()
            .unwrap()
            .resource_flow_ops
            .count,
        expected_ops
    );
}

#[test]
fn e2b5_flag_default_false_no_gpu_sync_required() {
    assert!(
        !PipelineFlags::default().use_accumulator_resource_flow,
        "default flag must remain false"
    );

    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Inherit, 0);
    apply_dynamic_enrollment(&mut fx);

    let scenario = support::e11_flat_star::flat_star_scenario(1, 32);
    let mut session = SimSession::open(scenario).expect("open");
    session.proto.root = fx.root;
    session.proto.allocator = fx.alloc;
    session.proto.registry = fx.reg;
    session.spec_state = fx.spec_state;
    session.sync_resource_flow_if_enabled().expect("sync ok");
    assert!(!session.state.accumulator_resource_flow_active);
}

#[test]
fn e2b5_replay_same_seed_same_dynamic_enrollment() {
    let fx_template = open_enrollment_fixture(2, 16, FissionPolicySpec::Reevaluate, 0);

    let mut fx_a = EnrollmentFixture {
        root: fx_template.root.clone(),
        alloc: fx_template.alloc.clone(),
        reg: fx_template.reg.clone(),
        spec_state: {
            let mut s = simthing_driver::SpecSessionState::new();
            s.arena_registry = fx_template.spec_state.arena_registry.clone();
            s.arena_participant_scaffold = fx_template.spec_state.arena_participant_scaffold.clone();
            s
        },
        parent_id: fx_template.parent_id,
        child_id: fx_template.child_id,
    };
    let mut fx_b = EnrollmentFixture {
        root: fx_template.root.clone(),
        alloc: fx_template.alloc.clone(),
        reg: fx_template.reg.clone(),
        spec_state: {
            let mut s = simthing_driver::SpecSessionState::new();
            s.arena_registry = fx_template.spec_state.arena_registry.clone();
            s.arena_participant_scaffold = fx_template.spec_state.arena_participant_scaffold.clone();
            s
        },
        parent_id: fx_template.parent_id,
        child_id: fx_template.child_id,
    };

    let report_a = apply_dynamic_enrollment(&mut fx_a);
    let report_b = apply_dynamic_enrollment(&mut fx_b);

    assert_eq!(report_a, report_b);
    assert_eq!(
        fx_a.spec_state.arena_registry.participants,
        fx_b.spec_state.arena_registry.participants
    );
    assert_eq!(
        fx_a.spec_state.arena_participant_scaffold.index,
        fx_b.spec_state.arena_participant_scaffold.index
    );
}

#[test]
fn e2b5_100_tick_flat_star_burn_in_after_dynamic_enrollment() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut fx = open_enrollment_fixture(2, 16, FissionPolicySpec::Inherit, 0);
    apply_dynamic_enrollment(&mut fx);

    let scenario = support::e11_flat_star::flat_star_scenario(2, 64);
    let mut session = SimSession::open(scenario).expect("open");
    session.proto.root = fx.root;
    session.proto.allocator = fx.alloc;
    session.proto.registry = fx.reg;
    session.spec_state = fx.spec_state;
    session.proto.flags.use_accumulator_resource_flow = true;
    session.sync_resource_flow_if_enabled().expect("sync");

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

    let cols = simthing_driver::resolve_node_columns(
        &session
            .proto
            .registry
            .property(
                session
                    .proto
                    .registry
                    .id_of("core", "food_flow")
                    .unwrap(),
            )
            .layout,
        "food",
    )
    .unwrap();
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_slots: Vec<_> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    let inputs = standard_flat_star_inputs(root_slot, &leaf_slots, cols);
    let n_dims = session.proto.registry.total_columns as u32;
    let n_bands = session.state.accumulator_resource_flow_bands;

    let report = run_flat_star_burn_in(
        &mut session.state,
        &layout,
        cols,
        n_dims,
        &inputs,
        &leaf_slots,
        n_bands,
        100,
        1.0,
    );
    assert_eq!(report.ticks_checked, 100);
    assert_eq!(report.max_abs_error.to_bits(), 0.0_f32.to_bits());
}

#[test]
fn e2b5_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("resource_flow_fission"));
}

#[test]
fn e2b5_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("resource_flow_fission"));
    let sync = include_str!("../../simthing-driver/src/arena_allocation_sync.rs");
    assert!(!sync.contains("wgsl"));
}

#[test]
fn e2b5_reevaluate_policy_maps_to_inherit_only() {
    let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Reevaluate, 0);
    let report = apply_dynamic_enrollment(&mut fx);
    assert_eq!(report.admissions.len(), 1);
    assert!(report.rejections.is_empty());
}

#[test]
fn e2b5_runtime_admit_updates_participant_range() {
    let mut b = ArenaRegistryBuilder::new();
    let food = b.push_arena(GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: simthing_core::SimPropertyId(0),
        balance_property_id: None,
        max_participants: 4,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicy::Inherit,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    });
    let parent = simthing_core::SimThingId::new();
    b.admit_participant(food, 1, parent).unwrap();
    let (mut reg, _) = b.build().unwrap();
    reg.admit_participant_runtime(food, 2, simthing_core::SimThingId::new())
        .unwrap();
    assert_eq!(reg.arenas[0].participant_range, (0, 2));
    assert_eq!(reg.participants.len(), 2);
}

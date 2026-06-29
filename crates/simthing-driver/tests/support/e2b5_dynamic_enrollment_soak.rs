//! Named fixtures and session builders for E-2B-5R dynamic enrollment soak.

use std::collections::HashMap;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SimThing,
    SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    all_reserved_gap_slots, arena_participant_sibling_slots, build_execution_plan,
    initial_dynamic_enrollment_sync, materialize_arena_participants,
    react_to_fission_resource_flow_enrollment_on_authoring, resolve_node_columns,
    run_dynamic_enrollment_gpu_burn_in, run_dynamic_enrollment_resync_cycles, slots_are_contiguous,
    validate_resource_flow_preflight, DynamicEnrollmentBoundaryMetrics,
    DynamicEnrollmentSoakReport, DynamicFissionEnrollmentReport, SimSession,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::{BoundaryOutcome, FissionOutcome};
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec,
};

use super::e11_flat_star::{flat_star_cell_inputs, flat_star_scenario};

type CellKey = (u32, u32);

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

pub fn register_food_flow_with_allocation(reg: &mut DimensionRegistry) {
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

pub struct EnrollmentSoakSetup {
    pub root: SimThing,
    pub alloc: SlotAllocator,
    pub reg: DimensionRegistry,
    pub spec_state: simthing_driver::SpecSessionState,
    pub fission: FissionOutcome,
    pub child_ids: Vec<SimThingId>,
}

pub struct EnrolledSoakSession {
    pub session: SimSession,
    pub layout: simthing_driver::ArenaTreeLayout,
    pub cols: simthing_driver::NodeColumnRefs,
    pub leaf_slots: Vec<u32>,
    pub inputs: HashMap<CellKey, f32>,
    pub enrollment_report: DynamicFissionEnrollmentReport,
    pub boundary_metrics: DynamicEnrollmentBoundaryMetrics,
}

#[derive(Clone, Debug)]
pub struct DynamicEnrollmentSoakFixture {
    pub name: &'static str,
    pub ticks: u32,
    pub sync_cycles: u32,
    pub resource_flow_enabled: bool,
    pub require_bit_exact: bool,
    pub expected_admissions: u32,
    pub expected_rejections: u32,
    pub expect_generation_bump: bool,
    pub expect_gpu_active: bool,
}

fn fission_outcome(pairs: Vec<(SimThingId, SimThingId)>) -> FissionOutcome {
    FissionOutcome {
        fissions_executed: pairs.len() as u32,
        fission_pairs: pairs,
        ..Default::default()
    }
}

pub fn open_single_fission_setup(
    parent_count: usize,
    max_participants: u32,
    gap: u32,
) -> EnrollmentSoakSetup {
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
        .map(|hosted| {
            ExplicitParticipantSpec::flat(alloc.slot_of(hosted.id).unwrap(), hosted.id.raw())
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
            fission_policy: FissionPolicySpec::Inherit,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: gap,
            expected_max_children_per_intermediate: gap,
            explicit_participants: explicit,
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        ..Default::default()
    };

    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    let (arena_registry, _) =
        simthing_driver::compile_and_materialize_resource_flow(&spec, &reg).unwrap();

    let mut spec_state = simthing_driver::SpecSessionState::new();
    spec_state.arena_registry = arena_registry;
    spec_state.arena_participant_scaffold = scaffold;

    EnrollmentSoakSetup {
        root,
        alloc,
        reg,
        spec_state,
        fission: fission_outcome(vec![(parent_id, child_id)]),
        child_ids: vec![child_id],
    }
}

fn open_multi_fission_setup(parent_count: usize, max_participants: u32) -> EnrollmentSoakSetup {
    let mut reg = DimensionRegistry::new();
    register_food_flow_with_allocation(&mut reg);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut pairs = Vec::new();
    let mut child_ids = Vec::new();
    for _ in 0..parent_count {
        let mut parent = SimThing::new(SimThingKind::Cohort, 0);
        let parent_id = parent.id;
        let child = SimThing::new(SimThingKind::Cohort, 0);
        let child_id = child.id;
        child_ids.push(child_id);
        parent.add_child(child);
        pairs.push((parent_id, child_id));
        root.add_child(parent);
    }

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);

    let explicit: Vec<ExplicitParticipantSpec> = root
        .children
        .iter()
        .map(|hosted| {
            ExplicitParticipantSpec::flat(alloc.slot_of(hosted.id).unwrap(), hosted.id.raw())
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
            fission_policy: FissionPolicySpec::Inherit,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants: explicit,
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        ..Default::default()
    };

    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    let (arena_registry, _) =
        simthing_driver::compile_and_materialize_resource_flow(&spec, &reg).unwrap();

    let mut spec_state = simthing_driver::SpecSessionState::new();
    spec_state.arena_registry = arena_registry;
    spec_state.arena_participant_scaffold = scaffold;

    EnrollmentSoakSetup {
        root,
        alloc,
        reg,
        spec_state,
        fission: fission_outcome(pairs),
        child_ids,
    }
}

fn open_two_arena_setup() -> EnrollmentSoakSetup {
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
            explicit_participants: vec![ExplicitParticipantSpec::flat(
                alloc.slot_of(parent_id).unwrap(),
                parent_id.raw(),
            )],
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        ..Default::default()
    };
    validate_resource_flow_preflight(&food_spec, &alloc).unwrap();
    let mut scaffold =
        materialize_arena_participants(&food_spec, &reg, &mut root, &mut alloc).unwrap();

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
            explicit_participants: vec![ExplicitParticipantSpec::flat(
                alloc.slot_of(parent_id).unwrap(),
                parent_id.raw(),
            )],
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        ..Default::default()
    };
    validate_resource_flow_preflight(&research_only, &alloc).unwrap();
    let research_scaffold =
        materialize_arena_participants(&research_only, &reg, &mut root, &mut alloc).unwrap();
    scaffold.reports.extend(research_scaffold.reports);
    if let Some(&research_root) = research_scaffold.arena_root_ids.get(&0) {
        scaffold.arena_root_ids.insert(1, research_root);
    }
    for ((hosted, _), slot) in research_scaffold.index.by_host_and_arena {
        scaffold.index.by_host_and_arena.insert((hosted, 1), slot);
    }
    scaffold.gap_pools.extend(research_scaffold.gap_pools);

    for id in (12u32..20)
        .filter_map(|slot| alloc.owner_of(slot))
        .collect::<Vec<_>>()
    {
        alloc.tombstone(id);
    }

    let full_spec = ResourceFlowSpec {
        arenas: vec![food_spec.arenas[0].clone(), research_only.arenas[0].clone()],
        couplings: vec![],
        ..Default::default()
    };
    let (arena_registry, _) =
        simthing_driver::compile_and_materialize_resource_flow(&full_spec, &reg).unwrap();

    let mut spec_state = simthing_driver::SpecSessionState::new();
    spec_state.arena_registry = arena_registry;
    spec_state.arena_participant_scaffold = scaffold;

    EnrollmentSoakSetup {
        root,
        alloc,
        reg,
        spec_state,
        fission: fission_outcome(vec![(parent_id, child_id)]),
        child_ids: vec![child_id],
    }
}

pub fn open_enrolled_soak_session(
    setup: &mut EnrollmentSoakSetup,
    hosted_count: usize,
    n_slots: u32,
    resource_flow_enabled: bool,
) -> EnrolledSoakSession {
    let mut scenario = flat_star_scenario(hosted_count, n_slots);
    scenario.registry = setup.reg.clone();
    let mut session = SimSession::open(scenario).expect("open session");

    session.proto.root = setup.root.clone();
    session.proto.allocator = setup.alloc.clone();
    session.spec_state.arena_registry = setup.spec_state.arena_registry.clone();
    session.spec_state.arena_participant_scaffold =
        setup.spec_state.arena_participant_scaffold.clone();
    session.proto.flags.use_accumulator_resource_flow = resource_flow_enabled;

    let gen_before = setup.spec_state.arena_registry.generation;
    let outcome = BoundaryOutcome {
        fission: setup.fission.clone(),
        ..Default::default()
    };
    session
        .react_to_fission_resource_flow_enrollment(&outcome)
        .expect("boundary enrollment");

    let enrollment_report = session
        .last_resource_flow_dynamic_enrollment_report
        .clone()
        .unwrap_or(DynamicFissionEnrollmentReport {
            generation_before: gen_before,
            generation_after: session.spec_state.arena_registry.generation,
            ..Default::default()
        });

    setup.root = session.proto.root.clone();
    setup.alloc = session.proto.allocator.clone();
    setup.spec_state.arena_registry = session.spec_state.arena_registry.clone();
    setup.spec_state.arena_participant_scaffold =
        session.spec_state.arena_participant_scaffold.clone();

    let mut resource_flow_syncs = if enrollment_report.any_admissions() && resource_flow_enabled {
        1
    } else {
        0
    };
    if resource_flow_enabled && enrollment_report.any_admissions() {
        initial_dynamic_enrollment_sync(&mut session).expect("initial sync");
    }

    let layout = build_execution_plan_from_authoring(
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
    .expect("food arena");

    let flow_id = session.proto.registry.id_of("core", "food_flow").unwrap();
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food")
        .expect("cols");
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_slots: Vec<_> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    let leaf_count = leaf_slots.len();
    let weights: Vec<f32> = (0..leaf_count)
        .map(|i| if i % 2 == 0 { 1.0 } else { 3.0 })
        .collect();
    let inputs = flat_star_cell_inputs(root_slot, &leaf_slots, cols, 10.0, &weights);

    let boundary_metrics = DynamicEnrollmentBoundaryMetrics::from_enrollment_report(
        &enrollment_report,
        1,
        setup.fission.fissions_executed,
        resource_flow_syncs,
    );

    EnrolledSoakSession {
        session,
        layout,
        cols,
        leaf_slots,
        inputs,
        enrollment_report,
        boundary_metrics,
    }
}

pub fn run_enrollment_only_soak(
    setup: &mut EnrollmentSoakSetup,
    fixture: &DynamicEnrollmentSoakFixture,
) -> DynamicEnrollmentSoakReport {
    let arena_root_id = setup.spec_state.arena_participant_scaffold.arena_root_ids[&0];
    let siblings_before = arena_participant_sibling_slots(&setup.root, arena_root_id, &setup.alloc);
    let participants_before = setup.spec_state.arena_registry.participants.clone();
    let index_before = setup.spec_state.arena_participant_scaffold.index.clone();
    let gen_before = setup.spec_state.arena_registry.generation;
    let gap_before: std::collections::HashSet<_> =
        all_reserved_gap_slots(&setup.spec_state.arena_participant_scaffold)
            .into_iter()
            .collect();

    let enrollment_report = react_to_fission_resource_flow_enrollment_on_authoring(
        &setup.fission,
        &mut setup.spec_state.arena_registry,
        &mut setup.spec_state.arena_participant_scaffold,
        &mut setup.root,
        &setup.reg,
        &mut setup.alloc,
    );

    let siblings_after = arena_participant_sibling_slots(&setup.root, arena_root_id, &setup.alloc);
    assert_eq!(
        siblings_before,
        siblings_after,
        "fixture {name} must not mutate sibling block",
        name = fixture.name
    );
    assert_eq!(
        setup.spec_state.arena_registry.participants,
        participants_before,
        "fixture {name} must not mutate registry participants",
        name = fixture.name
    );
    assert_eq!(
        setup.spec_state.arena_participant_scaffold.index,
        index_before,
        "fixture {name} must not mutate scaffold index",
        name = fixture.name
    );
    assert_eq!(
        setup.spec_state.arena_registry.generation,
        gen_before,
        "fixture {name} must not bump generation",
        name = fixture.name
    );
    let gap_after: std::collections::HashSet<_> =
        all_reserved_gap_slots(&setup.spec_state.arena_participant_scaffold)
            .into_iter()
            .collect();
    assert_eq!(gap_before, gap_after);

    let metrics = DynamicEnrollmentBoundaryMetrics::from_enrollment_report(
        &enrollment_report,
        1,
        setup.fission.fissions_executed,
        0,
    );
    let report = DynamicEnrollmentSoakReport::enrollment_only(fixture.name, &metrics);
    assert_eq!(report.admissions_observed, fixture.expected_admissions);
    assert_eq!(report.rejections_observed, fixture.expected_rejections);
    report
}

pub fn run_dynamic_enrollment_soak(
    fx: &mut EnrolledSoakSession,
    fixture: &DynamicEnrollmentSoakFixture,
) -> DynamicEnrollmentSoakReport {
    assert_eq!(
        fx.enrollment_report.admissions.len() as u32,
        fixture.expected_admissions,
        "fixture {name} admission count",
        name = fixture.name
    );
    assert_eq!(
        fx.enrollment_report.rejections.len() as u32,
        fixture.expected_rejections,
        "fixture {name} rejection count",
        name = fixture.name
    );

    if fixture.expect_generation_bump {
        assert!(
            fx.boundary_metrics.generation_end > fx.boundary_metrics.generation_start,
            "fixture {name} expected generation bump",
            name = fixture.name
        );
    } else {
        assert_eq!(
            fx.boundary_metrics.generation_end,
            fx.boundary_metrics.generation_start,
            "fixture {name} expected no generation bump",
            name = fixture.name
        );
    }

    assert_eq!(
        fx.session.state.accumulator_resource_flow_active,
        fixture.expect_gpu_active,
        "fixture {name} gpu active flag",
        name = fixture.name
    );

    if fixture.sync_cycles > 0 && fixture.resource_flow_enabled {
        let (syncs, _, _) =
            run_dynamic_enrollment_resync_cycles(&mut fx.session, fixture.sync_cycles)
                .expect("resync cycles");
        fx.boundary_metrics.resource_flow_syncs_observed += syncs;
    }

    if fixture.ticks == 0 || !fixture.resource_flow_enabled {
        return DynamicEnrollmentSoakReport::enrollment_only(fixture.name, &fx.boundary_metrics);
    }

    let n_dims = fx.session.proto.registry.total_columns as u32;
    let n_bands = fx.session.state.accumulator_resource_flow_bands;
    let burn = run_dynamic_enrollment_gpu_burn_in(
        &mut fx.session.state,
        &fx.layout,
        fx.cols,
        n_dims,
        &fx.inputs,
        &fx.leaf_slots,
        n_bands,
        fixture.ticks,
        fx.session.scenario.dt,
    );

    let report = DynamicEnrollmentSoakReport::from_parts(
        fixture.name,
        &fx.boundary_metrics,
        &burn,
        fixture.require_bit_exact,
    );
    report.assert_within_contract(fixture.require_bit_exact);
    report
}

pub fn dynamic_enrollment_single_fission_inherit() -> DynamicEnrollmentSoakFixture {
    DynamicEnrollmentSoakFixture {
        name: "dynamic_enrollment_single_fission_inherit",
        ticks: 100,
        sync_cycles: 0,
        resource_flow_enabled: true,
        require_bit_exact: true,
        expected_admissions: 1,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: true,
    }
}

pub fn dynamic_enrollment_multiple_fissions_same_arena() -> DynamicEnrollmentSoakFixture {
    DynamicEnrollmentSoakFixture {
        name: "dynamic_enrollment_multiple_fissions_same_arena",
        ticks: 100,
        sync_cycles: 0,
        resource_flow_enabled: true,
        require_bit_exact: true,
        expected_admissions: 2,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: true,
    }
}

pub fn dynamic_enrollment_two_arenas_inherit() -> DynamicEnrollmentSoakFixture {
    DynamicEnrollmentSoakFixture {
        name: "dynamic_enrollment_two_arenas_inherit",
        ticks: 100,
        sync_cycles: 0,
        resource_flow_enabled: true,
        require_bit_exact: true,
        expected_admissions: 2,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: true,
    }
}

pub fn dynamic_enrollment_reject_when_cap_full() -> DynamicEnrollmentSoakFixture {
    DynamicEnrollmentSoakFixture {
        name: "dynamic_enrollment_reject_when_cap_full",
        ticks: 0,
        sync_cycles: 0,
        resource_flow_enabled: true,
        require_bit_exact: true,
        expected_admissions: 0,
        expected_rejections: 1,
        expect_generation_bump: false,
        expect_gpu_active: false,
    }
}

pub fn dynamic_enrollment_contiguity_blocked_no_compaction() -> DynamicEnrollmentSoakFixture {
    DynamicEnrollmentSoakFixture {
        name: "dynamic_enrollment_contiguity_blocked_no_compaction",
        ticks: 0,
        sync_cycles: 0,
        resource_flow_enabled: true,
        require_bit_exact: true,
        expected_admissions: 0,
        expected_rejections: 1,
        expect_generation_bump: false,
        expect_gpu_active: false,
    }
}

pub fn dynamic_enrollment_flag_off_no_gpu_sync() -> DynamicEnrollmentSoakFixture {
    DynamicEnrollmentSoakFixture {
        name: "dynamic_enrollment_flag_off_no_gpu_sync",
        ticks: 0,
        sync_cycles: 0,
        resource_flow_enabled: false,
        require_bit_exact: true,
        expected_admissions: 1,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: false,
    }
}

pub fn dynamic_enrollment_repeated_resync() -> DynamicEnrollmentSoakFixture {
    DynamicEnrollmentSoakFixture {
        name: "dynamic_enrollment_repeated_resync",
        ticks: 10,
        sync_cycles: 100,
        resource_flow_enabled: true,
        require_bit_exact: true,
        expected_admissions: 1,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: true,
    }
}

pub fn open_fixture_session(fixture: &DynamicEnrollmentSoakFixture) -> EnrolledSoakSession {
    match fixture.name {
        "dynamic_enrollment_reject_when_cap_full"
        | "dynamic_enrollment_contiguity_blocked_no_compaction" => {
            panic!(
                "fixture {name} uses run_enrollment_only_soak, not open_fixture_session",
                name = fixture.name
            );
        }
        "dynamic_enrollment_single_fission_inherit" => {
            let mut setup = open_single_fission_setup(2, 16, 0);
            open_enrolled_soak_session(&mut setup, 2, 64, fixture.resource_flow_enabled)
        }
        "dynamic_enrollment_multiple_fissions_same_arena" => {
            let mut setup = open_multi_fission_setup(2, 16);
            open_enrolled_soak_session(&mut setup, 2, 64, fixture.resource_flow_enabled)
        }
        "dynamic_enrollment_two_arenas_inherit" => {
            let mut setup = open_two_arena_setup();
            open_enrolled_soak_session(&mut setup, 1, 128, fixture.resource_flow_enabled)
        }
        "dynamic_enrollment_flag_off_no_gpu_sync" => {
            let mut setup = open_single_fission_setup(1, 16, 0);
            open_enrolled_soak_session(&mut setup, 1, 32, false)
        }
        "dynamic_enrollment_repeated_resync" => {
            let mut setup = open_single_fission_setup(2, 16, 0);
            open_enrolled_soak_session(&mut setup, 2, 64, true)
        }
        _ => panic!("unknown soak fixture {}", fixture.name),
    }
}

pub fn assert_reject_no_partial_mutation(fx: &EnrolledSoakSession, child_id: SimThingId) {
    assert!(fx
        .session
        .spec_state
        .arena_participant_scaffold
        .index
        .participant_slot(child_id, 0)
        .is_none());
}

pub fn assert_contiguity_unchanged_on_reject(
    setup: &EnrollmentSoakSetup,
    fx: &EnrolledSoakSession,
) {
    let arena_root_id = setup.spec_state.arena_participant_scaffold.arena_root_ids[&0];
    let siblings = arena_participant_sibling_slots(
        &fx.session.proto.root,
        arena_root_id,
        &fx.session.proto.allocator,
    );
    assert_eq!(siblings.len(), 1);
}

pub fn clone_enrolled_for_replay(fx: &EnrolledSoakSession) -> EnrolledSoakSession {
    let scenario = flat_star_scenario(
        fx.session.proto.root.children.len(),
        fx.session.scenario.n_slots,
    );
    let mut session = SimSession::open(scenario).expect("replay open");
    session.proto.root = fx.session.proto.root.clone();
    session.proto.allocator = fx.session.proto.allocator.clone();
    session.proto.registry = fx.session.proto.registry.clone();
    session.spec_state.arena_registry = fx.session.spec_state.arena_registry.clone();
    session.spec_state.arena_participant_scaffold =
        fx.session.spec_state.arena_participant_scaffold.clone();
    session.proto.flags = fx.session.proto.flags.clone();
    initial_dynamic_enrollment_sync(&mut session).expect("replay sync");

    EnrolledSoakSession {
        session,
        layout: fx.layout.clone(),
        cols: fx.cols,
        leaf_slots: fx.leaf_slots.clone(),
        inputs: fx.inputs.clone(),
        enrollment_report: fx.enrollment_report.clone(),
        boundary_metrics: fx.boundary_metrics.clone(),
    }
}

pub fn run_replay_burn_in(fx: &mut EnrolledSoakSession, ticks: u32) -> DynamicEnrollmentSoakReport {
    let fixture = dynamic_enrollment_single_fission_inherit();
    let mut replay_fixture = fixture.clone();
    replay_fixture.ticks = ticks;
    run_dynamic_enrollment_soak(fx, &replay_fixture)
}

pub fn assert_sibling_contiguity_after_admission(fx: &EnrolledSoakSession) {
    let arena_root_id = fx
        .session
        .spec_state
        .arena_participant_scaffold
        .arena_root_ids[&0];
    let siblings = arena_participant_sibling_slots(
        &fx.session.proto.root,
        arena_root_id,
        &fx.session.proto.allocator,
    );
    assert!(slots_are_contiguous(&siblings));
}

pub fn child_id_for_reject_fixture() -> SimThingId {
    let setup = open_single_fission_setup(1, 1, 0);
    setup.child_ids[0]
}

pub fn child_id_for_contiguity_fixture() -> SimThingId {
    let setup = open_single_fission_setup(1, 16, 2);
    setup.child_ids[0]
}

pub fn reserved_gap_slots_unchanged(setup: &EnrollmentSoakSetup, fx: &EnrolledSoakSession) {
    let before: std::collections::HashSet<_> =
        all_reserved_gap_slots(&setup.spec_state.arena_participant_scaffold)
            .into_iter()
            .collect();
    let after: std::collections::HashSet<_> =
        all_reserved_gap_slots(&fx.session.spec_state.arena_participant_scaffold)
            .into_iter()
            .collect();
    assert_eq!(before, after);
}

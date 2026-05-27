//! E-11R — remedial hardening for landed flat-star allocation execution.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SubFieldRole,
    SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, install_atomic, resolve_node_columns, run_arena_allocation_oracle,
    HierarchyError, ResourceFlowSyncError, SessionError, SimSession,
};
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowSpec, SpecVersion,
};
use simthing_gpu::GpuContext;
use std::collections::HashMap;

use simthing_driver::Scenario;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

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

fn register_food_flow(reg: &mut DimensionRegistry) {
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

fn flat_star_scenario(hosted_count: usize, n_slots: u32) -> Scenario {
    let mut root = simthing_core::SimThing::new(simthing_core::SimThingKind::World, 0);
    for _ in 0..hosted_count {
        root.add_child(simthing_core::SimThing::new(
            simthing_core::SimThingKind::Cohort,
            0,
        ));
    }
    let mut registry = DimensionRegistry::new();
    register_food_flow(&mut registry);
    Scenario {
        name: "e11r_flat_star".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    }
}

fn flat_star_game_mode(max_orderband_depth: u32, _hosted_count: usize) -> GameModeSpec {
    GameModeSpec {
        id: "e11r_flat_star".into(),
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
                explicit_participants: Vec::new(),
                wildcard_admission: None,
            }],
            couplings: vec![],
        }),
    }
}

fn fill_explicit_participants(game_mode: &mut GameModeSpec, scenario: &Scenario) {
    let mut alloc = simthing_gpu::SlotAllocator::new();
    alloc.populate_from_tree(&scenario.root);
    let participants: Vec<_> = scenario
        .root
        .children
        .iter()
        .map(|c| ExplicitParticipantSpec {
            slot: alloc.slot_of(c.id).unwrap(),
            subtree_root_id: c.id.raw(),
        })
        .collect();
    game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants = participants;
}

#[test]
fn e11r_resource_flow_sync_error_is_reported_when_flag_enabled() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = flat_star_game_mode(4, 3);
    fill_explicit_participants(&mut game_mode, &scenario);

    let mut session = SimSession::open(scenario).expect("open session");
    let spec_state = install_atomic(
        &game_mode,
        &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )
    .expect("install atomic");

    session.proto.flags.use_accumulator_resource_flow = true;
    session.spec_state = spec_state;

    let err = session
        .sync_resource_flow_if_enabled()
        .expect_err("depth budget must fail when flag enabled");
    match err {
        SessionError::ResourceFlow(ResourceFlowSyncError::Hierarchy(
            HierarchyError::OrderBandDepthExceeded { needed: 5, max: 4, .. },
        )) => {}
        other => panic!("unexpected session error: {other:?}"),
    }
}

#[test]
fn e11_resource_flow_flag_uploads_and_dispatches_flat_star_ops() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = flat_star_game_mode(16, 3);
    fill_explicit_participants(&mut game_mode, &scenario);

    let mut session = SimSession::open(scenario).expect("open session");
    let spec_state = install_atomic(
        &game_mode,
        &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )
    .expect("install atomic");

    session.proto.flags.use_accumulator_resource_flow = true;
    session.install_spec_state(spec_state).expect("install spec");

    assert!(
        session.state.accumulator_resource_flow_active,
        "session sync must upload resource-flow ops when flag enabled"
    );
    assert!(
        session.state.accumulator_resource_flow_bands >= 5,
        "D=2 flat-star needs at least 5 bands"
    );

    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow registered");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food")
        .expect("column refs");

    let layout = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("execution plan")
    .arenas
    .into_iter()
    .next()
    .expect("one arena");

    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_slots: Vec<_> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();

    let n_dims = session.proto.registry.total_columns as u32;
    let mut flat = session.state.read_values();
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    flat[idx(root_slot, cols.intrinsic_flow_col)] = 10.0;
    flat[idx(leaf_slots[0], cols.weight_col)] = 1.0;
    flat[idx(leaf_slots[1], cols.weight_col)] = 3.0;
    session.state.write_values(&flat);

    let mut oracle = HashMap::from([
        ((root_slot, cols.intrinsic_flow_col), 10.0),
        ((leaf_slots[0], cols.weight_col), 1.0),
        ((leaf_slots[1], cols.weight_col), 3.0),
    ]);
    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);

    session
        .state
        .run_resource_flow_bands(session.state.accumulator_resource_flow_bands, 1.0);

    let gpu_out = session.state.read_values();
    for &leaf in &leaf_slots {
        let cpu = oracle
            .get(&(leaf, cols.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu_out[idx(leaf, cols.allocated_flow_col)];
        assert_eq!(cpu.to_bits(), gpu.to_bits(), "leaf {leaf} session-path aF parity");
    }
}

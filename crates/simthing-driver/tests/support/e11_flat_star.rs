//! Flat-star D=2 Resource Flow fixture (session-path).

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SubFieldRole,
    SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, install_atomic, resolve_node_columns, ArenaTreeLayout, NodeColumnRefs,
    Scenario, SimSession,
};
use simthing_gpu::GpuContext;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowSpec, SpecVersion,
};

pub fn try_gpu() -> Option<GpuContext> {
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

pub fn register_food_flow(reg: &mut DimensionRegistry) {
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

pub fn flat_star_scenario(hosted_count: usize, n_slots: u32) -> Scenario {
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
        name: "e11_flat_star".into(),
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

pub fn flat_star_game_mode(max_orderband_depth: u32) -> GameModeSpec {
    GameModeSpec {
        id: "e11_flat_star".into(),
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
                enrollment: None,
                wildcard_admission: None,
            }],
            couplings: vec![],
            ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
    }
}

pub fn fill_explicit_participants(game_mode: &mut GameModeSpec, scenario: &Scenario) {
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

pub struct FlatStarSession {
    pub session: SimSession,
    pub layout: ArenaTreeLayout,
    pub cols: NodeColumnRefs,
}

pub fn open_flat_star_session(hosted_count: usize, flag_enabled: bool) -> FlatStarSession {
    let scenario = flat_star_scenario(hosted_count, 32);
    let mut game_mode = flat_star_game_mode(16);
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

    session.proto.flags.use_accumulator_resource_flow = flag_enabled;
    session
        .install_spec_state(spec_state)
        .expect("install spec state");

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

    FlatStarSession {
        session,
        layout,
        cols,
    }
}

pub fn root_slot(layout: &ArenaTreeLayout) -> u32 {
    layout.participant_roots[0].participant_slot
}

pub fn leaf_slots(layout: &ArenaTreeLayout) -> Vec<u32> {
    layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect()
}

pub fn flat_star_cell_inputs(
    root_slot: u32,
    leaf_slots: &[u32],
    cols: NodeColumnRefs,
    root_intrinsic_flow: f32,
    leaf_weights: &[f32],
) -> std::collections::HashMap<(u32, u32), f32> {
    let mut inputs =
        std::collections::HashMap::from([((root_slot, cols.intrinsic_flow_col), root_intrinsic_flow)]);
    for (slot, &weight) in leaf_slots.iter().zip(leaf_weights.iter()) {
        inputs.insert((*slot, cols.weight_col), weight);
    }
    inputs
}

pub fn standard_flat_star_inputs(
    root_slot: u32,
    leaf_slots: &[u32],
    cols: NodeColumnRefs,
) -> std::collections::HashMap<(u32, u32), f32> {
    flat_star_cell_inputs(root_slot, leaf_slots, cols, 10.0, &[1.0, 3.0])
}

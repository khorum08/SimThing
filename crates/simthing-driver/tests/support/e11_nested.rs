//! Static nested Resource Flow fixtures (A-0 / E-11B).

#![allow(dead_code)]

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, EmlExpressionRegistry,
    LogTier, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, install_atomic, materialize_arena_participants, max_disbursement_band,
    nested_hierarchy_materialization_report, plan_arena_allocation, register_child_share_formula,
    resolve_node_columns, run_arena_allocation_oracle, validate_resource_flow_preflight,
    ArenaParticipantScaffold, ArenaTreeLayout, GpuArenaDescriptor, HierarchyNode, NodeColumnRefs,
    SimSession,
};
use simthing_gpu::{GpuContext, SlotAllocator, WorldGpuState};
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowSpec, SpecVersion,
};
use std::collections::HashMap;
use std::path::Path;

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

pub fn register_food_flow(reg: &mut DimensionRegistry) -> simthing_core::SimPropertyId {
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

pub fn arena_spec(
    participants: Vec<ExplicitParticipantSpec>,
    max_orderband_depth: u32,
    gap_k: u32,
) -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants: 32,
        max_coupling_fanout: 4,
        max_orderband_depth,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: gap_k,
        expected_max_children_per_intermediate: 0,
        explicit_participants: participants,
        enrollment: None,
        wildcard_admission: None,
    }
}

pub fn hosted_cohorts(count: usize) -> (SimThing, Vec<SimThingId>) {
    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut ids = Vec::new();
    for _ in 0..count {
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        ids.push(cohort.id);
        world.add_child(cohort);
    }
    (world, ids)
}

/// Handoff D=3: FactionRoot → Planet_A/B → two factories each.
pub fn a0_d3_participants(hosted: &[SimThingId], alloc: &SlotAllocator) -> Vec<ExplicitParticipantSpec> {
    let slot = |id: SimThingId| alloc.slot_of(id).unwrap();
    let raw = |id: SimThingId| id.raw();
    let parent = |id: SimThingId| id.raw() as u64;
    vec![
        ExplicitParticipantSpec::flat(slot(hosted[0]), raw(hosted[0])),
        ExplicitParticipantSpec::nested(slot(hosted[1]), raw(hosted[1]), parent(hosted[0])),
        ExplicitParticipantSpec::nested(slot(hosted[2]), raw(hosted[2]), parent(hosted[0])),
        ExplicitParticipantSpec::nested(slot(hosted[3]), raw(hosted[3]), parent(hosted[1])),
        ExplicitParticipantSpec::nested(slot(hosted[4]), raw(hosted[4]), parent(hosted[1])),
        ExplicitParticipantSpec::nested(slot(hosted[5]), raw(hosted[5]), parent(hosted[2])),
        ExplicitParticipantSpec::nested(slot(hosted[6]), raw(hosted[6]), parent(hosted[2])),
    ]
}

/// Proven D=4 topology (E-11B): one depth-4 branch plus a second shallow root.
pub fn a0_d4_participants(hosted: &[SimThingId], alloc: &SlotAllocator) -> Vec<ExplicitParticipantSpec> {
    let slot = |id: SimThingId| alloc.slot_of(id).unwrap();
    let raw = |id: SimThingId| id.raw();
    let parent = |id: SimThingId| id.raw() as u64;
    vec![
        ExplicitParticipantSpec::flat(slot(hosted[0]), raw(hosted[0])),
        ExplicitParticipantSpec::nested(slot(hosted[1]), raw(hosted[1]), parent(hosted[0])),
        ExplicitParticipantSpec::nested(slot(hosted[2]), raw(hosted[2]), parent(hosted[1])),
        ExplicitParticipantSpec::nested(slot(hosted[3]), raw(hosted[3]), parent(hosted[2])),
        ExplicitParticipantSpec::nested(slot(hosted[4]), raw(hosted[4]), parent(hosted[2])),
        ExplicitParticipantSpec::flat(slot(hosted[5]), raw(hosted[5])),
        ExplicitParticipantSpec::nested(slot(hosted[6]), raw(hosted[6]), parent(hosted[5])),
    ]
}

pub struct MaterializedNestedFixture {
    pub reg: DimensionRegistry,
    pub root: SimThing,
    pub alloc: SlotAllocator,
    pub scaffold: ArenaParticipantScaffold,
    pub flow_id: simthing_core::SimPropertyId,
    pub cols: NodeColumnRefs,
}

pub fn materialize_nested(
    hosted_count: usize,
    build_participants: impl Fn(&[SimThingId], &SlotAllocator) -> Vec<ExplicitParticipantSpec>,
    max_depth: u32,
    gap_k: u32,
) -> MaterializedNestedFixture {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_food_flow(&mut reg);
    let cols = resolve_node_columns(&reg.property(flow_id).layout, "food").unwrap();
    let (mut root, hosted) = hosted_cohorts(hosted_count);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let participants = build_participants(&hosted, &alloc);
    let spec = ResourceFlowSpec {
        arenas: vec![arena_spec(participants, max_depth.max(16), gap_k)],
        couplings: vec![],
        ..Default::default()
    };
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    MaterializedNestedFixture {
        reg,
        root,
        alloc,
        scaffold,
        flow_id,
        cols,
    }
}

pub fn layout_for(f: &MaterializedNestedFixture) -> ArenaTreeLayout {
    let arena = GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: f.flow_id,
        balance_property_id: None,
        max_participants: 32,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: simthing_driver::FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    build_execution_plan(
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

pub fn leaves(layout: &ArenaTreeLayout) -> Vec<&HierarchyNode> {
    layout
        .iter_all()
        .into_iter()
        .filter(|node| node.children.is_empty())
        .collect()
}

fn idx(n_dims: u32, slot: u32, col: u32) -> usize {
    (slot * n_dims + col) as usize
}

pub struct ParityResult {
    pub max_abs_error: f32,
    pub l_inf: f32,
    pub leaf_count: usize,
}

pub fn run_gpu_allocation(
    f: &MaterializedNestedFixture,
    layout: &ArenaTreeLayout,
    values: &[f32],
) -> Vec<f32> {
    let ctx = GpuContext::new_blocking().expect("gpu");
    let n_slots = f.alloc.capacity() as u32;
    let mut state = WorldGpuState::new(ctx, &f.reg, n_slots);
    state.write_values(values);

    let plan = plan_arena_allocation(layout, &[], n_slots).unwrap();
    let mut eml = EmlExpressionRegistry::new();
    register_child_share_formula(&mut eml, f.cols).unwrap();
    state
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    state.run_resource_flow_bands(plan.n_bands, 1.0);
    state.read_values()
}

pub fn assert_nested_cpu_gpu_parity(
    f: &MaterializedNestedFixture,
    layout: &ArenaTreeLayout,
    root_intrinsic: f32,
) -> ParityResult {
    let c = f.cols;
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_nodes = leaves(layout);
    let n_dims = f.reg.total_columns as u32;
    let n_slots = f.alloc.capacity() as u32;
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    let mut oracle = HashMap::new();

    flat[idx(n_dims, root_slot, c.intrinsic_flow_col)] = root_intrinsic;
    oracle.insert((root_slot, c.intrinsic_flow_col), root_intrinsic);

    for node in layout.iter_all().into_iter().filter(|node| node.depth > 0) {
        flat[idx(n_dims, node.participant_slot, c.weight_col)] = 1.0;
        oracle.insert((node.participant_slot, c.weight_col), 1.0);
    }
    if let Some(first) = leaf_nodes.first() {
        flat[idx(n_dims, first.participant_slot, c.weight_col)] = 1.0;
        oracle.insert((first.participant_slot, c.weight_col), 1.0);
    }
    if leaf_nodes.len() > 1 {
        let second = leaf_nodes[1];
        flat[idx(n_dims, second.participant_slot, c.weight_col)] = 3.0;
        oracle.insert((second.participant_slot, c.weight_col), 3.0);
    }

    run_arena_allocation_oracle(layout, &mut oracle, 1.0);
    let gpu = run_gpu_allocation(f, layout, &flat);

    let mut max_abs_error = 0.0_f32;
    let mut l_inf = 0.0_f32;
    for leaf in &leaf_nodes {
        let cpu = oracle
            .get(&(leaf.participant_slot, c.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu_val = gpu[idx(n_dims, leaf.participant_slot, c.allocated_flow_col)];
        let err = (cpu - gpu_val).abs();
        max_abs_error = max_abs_error.max(err);
        l_inf = l_inf.max(err);
        assert_eq!(
            cpu.to_bits(),
            gpu_val.to_bits(),
            "leaf slot {} parity",
            leaf.participant_slot
        );
    }
    ParityResult {
        max_abs_error,
        l_inf,
        leaf_count: leaf_nodes.len(),
    }
}

pub struct NestedSession {
    pub session: SimSession,
    pub layout: ArenaTreeLayout,
    pub cols: NodeColumnRefs,
}

pub fn nested_game_mode(
    participants: Vec<ExplicitParticipantSpec>,
    max_orderband_depth: u32,
) -> GameModeSpec {
    GameModeSpec {
        id: "a0_nested".into(),
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
            arenas: vec![arena_spec(participants, max_orderband_depth, 0)],
            couplings: vec![],
            ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    }
}

pub fn open_nested_session(
    hosted_count: usize,
    build_participants: impl Fn(&[SimThingId], &SlotAllocator) -> Vec<ExplicitParticipantSpec>,
    max_orderband_depth: u32,
    flag_enabled: bool,
) -> NestedSession {
    let mut reg = DimensionRegistry::new();
    register_food_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(hosted_count);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let participants = build_participants(&hosted, &alloc);
    let game_mode = nested_game_mode(participants, max_orderband_depth);

    let scenario = simthing_driver::Scenario {
        name: "a0_nested".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 64,
        registry: reg,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };

    let mut session = SimSession::open(scenario).expect("open");
    let spec_state = install_atomic(
        &game_mode,
        &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )
    .expect("install");
    session.proto.flags.use_accumulator_resource_flow = flag_enabled;
    session.install_spec_state(spec_state).expect("install spec");
    if flag_enabled {
        session.sync_resource_flow_if_enabled().expect("sync rf");
    }

    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food")
        .expect("cols");
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

    NestedSession {
        session,
        layout,
        cols,
    }
}

pub fn assert_no_new_wgsl() {
    let gpu_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../simthing-gpu/src");
    let allowed = [
        "accumulator_op.wgsl",
        "accumulator_op_intent.wgsl",
        "accumulator_op_threshold.wgsl",
        "atlas_mask.wgsl",
        "snapshot.wgsl",
        "world_summary.wgsl",
    ];
    for entry in std::fs::read_dir(&gpu_src).expect("gpu src") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("wgsl") {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap();
        assert!(
            allowed.contains(&name),
            "unexpected WGSL file {name}"
        );
    }
}

pub fn integration_band_for_layout(layout: &ArenaTreeLayout) -> u32 {
    max_disbursement_band(layout) + 1
}

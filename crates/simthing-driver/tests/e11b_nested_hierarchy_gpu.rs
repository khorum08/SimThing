//! E-11B nested Resource Flow GPU kickoff tests.

#[path = "support/e11_flat_star.rs"]
mod e11_flat_star;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry,
    EmlExpressionRegistry, LogTier, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, child_share_cpu, materialize_arena_participants, plan_arena_allocation,
    register_child_share_formula, resolve_node_columns, run_arena_allocation_oracle,
    slot_in_participant_sibling_range, slots_are_contiguous, try_alloc_participant_child_in_gap,
    ArenaParticipantScaffold, FissionPolicy, GpuArenaDescriptor, HierarchyNode, NodeColumnRefs,
};
use simthing_gpu::{GpuContext, SlotAllocator, WorldGpuState};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec,
};
use std::collections::HashMap;
use std::path::Path;

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

fn arena_desc(
    flow_id: simthing_core::SimPropertyId,
    max_orderband_depth: u32,
) -> GpuArenaDescriptor {
    GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: flow_id,
        balance_property_id: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth,
        fission_policy: FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    }
}

struct ParticipantIds {
    node_id: SimThingId,
    hosted_id: SimThingId,
}

fn participant() -> (SimThing, ParticipantIds) {
    let node = SimThing::new(SimThingKind::ArenaParticipant, 0);
    let hosted = SimThing::new(SimThingKind::Cohort, 0).id;
    let ids = ParticipantIds {
        node_id: node.id,
        hosted_id: hosted,
    };
    (node, ids)
}

struct NestedFixture {
    reg: DimensionRegistry,
    root: SimThing,
    alloc: SlotAllocator,
    scaffold: ArenaParticipantScaffold,
    arena: GpuArenaDescriptor,
    cols: NodeColumnRefs,
    participant_nodes: Vec<ParticipantIds>,
}

fn nested_d3_fixture() -> NestedFixture {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_flow(&mut reg);
    let cols = resolve_node_columns(&reg.property(flow_id).layout, "food").unwrap();
    let arena = arena_desc(flow_id, 16);

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut arena_root = SimThing::new(SimThingKind::Custom("arena_root:food".into()), 0);
    let arena_root_id = arena_root.id;
    let (mut root_participant, root_ids) = participant();
    let (mut mid, mid_ids) = participant();
    let (leaf_a, leaf_a_ids) = participant();
    let (leaf_b, leaf_b_ids) = participant();

    let mut alloc = SlotAllocator::new();
    alloc.alloc(world.id);
    alloc.alloc(arena_root_id);
    for id in [
        root_ids.node_id,
        mid_ids.node_id,
        leaf_a_ids.node_id,
        leaf_b_ids.node_id,
    ] {
        alloc.alloc(id);
    }

    mid.add_child(leaf_a);
    mid.add_child(leaf_b);
    root_participant.add_child(mid);
    arena_root.add_child(root_participant);
    world.add_child(arena_root);

    let participant_nodes = vec![root_ids, mid_ids, leaf_a_ids, leaf_b_ids];
    let scaffold = scaffold_for(arena_root_id, &alloc, &participant_nodes);
    NestedFixture {
        reg,
        root: world,
        alloc,
        scaffold,
        arena,
        cols,
        participant_nodes,
    }
}

fn nested_d4_fixture() -> NestedFixture {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_flow(&mut reg);
    let cols = resolve_node_columns(&reg.property(flow_id).layout, "food").unwrap();
    let arena = arena_desc(flow_id, 16);

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut arena_root = SimThing::new(SimThingKind::Custom("arena_root:food".into()), 0);
    let arena_root_id = arena_root.id;
    let (mut root_participant, root_ids) = participant();
    let (mut mid, mid_ids) = participant();
    let (mut child, child_ids) = participant();
    let (leaf_a, leaf_a_ids) = participant();
    let (leaf_b, leaf_b_ids) = participant();

    let mut alloc = SlotAllocator::new();
    alloc.alloc(world.id);
    alloc.alloc(arena_root_id);
    for id in [
        root_ids.node_id,
        mid_ids.node_id,
        child_ids.node_id,
        leaf_a_ids.node_id,
        leaf_b_ids.node_id,
    ] {
        alloc.alloc(id);
    }

    child.add_child(leaf_a);
    child.add_child(leaf_b);
    mid.add_child(child);
    root_participant.add_child(mid);
    arena_root.add_child(root_participant);
    world.add_child(arena_root);

    let participant_nodes = vec![root_ids, mid_ids, child_ids, leaf_a_ids, leaf_b_ids];
    let scaffold = scaffold_for(arena_root_id, &alloc, &participant_nodes);
    NestedFixture {
        reg,
        root: world,
        alloc,
        scaffold,
        arena,
        cols,
        participant_nodes,
    }
}

fn scaffold_for(
    arena_root_id: SimThingId,
    alloc: &SlotAllocator,
    participant_nodes: &[ParticipantIds],
) -> ArenaParticipantScaffold {
    let mut scaffold = ArenaParticipantScaffold::default();
    scaffold.arena_root_ids.insert(0, arena_root_id);
    for ids in participant_nodes {
        scaffold.index.by_host_and_arena.insert(
            (ids.hosted_id, 0),
            alloc.slot_of(ids.node_id).expect("participant slot"),
        );
    }
    scaffold
}

fn layout_from_fixture(f: &NestedFixture) -> simthing_driver::ArenaTreeLayout {
    build_execution_plan(
        &f.reg,
        std::slice::from_ref(&f.arena),
        &f.root,
        &f.alloc,
        &f.scaffold,
        7,
    )
    .expect("nested execution plan")
    .arenas
    .into_iter()
    .next()
    .expect("one arena")
}

fn idx(n_dims: u32, slot: u32, col: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn leaves(layout: &simthing_driver::ArenaTreeLayout) -> Vec<&HierarchyNode> {
    layout
        .iter_all()
        .into_iter()
        .filter(|node| node.children.is_empty())
        .collect()
}

fn run_gpu_allocation(
    f: &NestedFixture,
    layout: &simthing_driver::ArenaTreeLayout,
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

fn assert_nested_cpu_gpu_parity(f: &NestedFixture, expected_depth: u32) {
    let layout = layout_from_fixture(f);
    assert_eq!(layout.max_depth, expected_depth);
    for root in &layout.participant_roots {
        root.verify_child_contiguity().unwrap();
    }

    let c = f.cols;
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_nodes = leaves(&layout);
    assert_eq!(leaf_nodes.len(), 2);

    let n_dims = f.reg.total_columns as u32;
    let n_slots = f.alloc.capacity() as u32;
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    let mut oracle = HashMap::new();

    flat[idx(n_dims, root_slot, c.intrinsic_flow_col)] = 16.0;
    oracle.insert((root_slot, c.intrinsic_flow_col), 16.0);

    for node in layout.iter_all().into_iter().filter(|node| node.depth > 0) {
        flat[idx(n_dims, node.participant_slot, c.weight_col)] = 1.0;
        oracle.insert((node.participant_slot, c.weight_col), 1.0);
    }
    flat[idx(n_dims, leaf_nodes[0].participant_slot, c.weight_col)] = 1.0;
    flat[idx(n_dims, leaf_nodes[1].participant_slot, c.weight_col)] = 3.0;
    oracle.insert((leaf_nodes[0].participant_slot, c.weight_col), 1.0);
    oracle.insert((leaf_nodes[1].participant_slot, c.weight_col), 3.0);

    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);
    let gpu = run_gpu_allocation(f, &layout, &flat);

    for leaf in leaf_nodes {
        let cpu = oracle
            .get(&(leaf.participant_slot, c.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu[idx(n_dims, leaf.participant_slot, c.allocated_flow_col)];
        assert_eq!(
            cpu.to_bits(),
            gpu.to_bits(),
            "leaf {} parity",
            leaf.participant_slot
        );
        assert!(!gpu.is_nan());
    }
}

#[test]
fn e11b_d3_static_nested_cpu_gpu_parity() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    assert_nested_cpu_gpu_parity(&nested_d3_fixture(), 3);
}

#[test]
fn e11b_d4_static_nested_cpu_gpu_parity() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    assert_nested_cpu_gpu_parity(&nested_d4_fixture(), 4);
}

#[test]
fn e11b_nested_execution_plan_has_depth_ordered_bands() {
    let f = nested_d4_fixture();
    let layout = layout_from_fixture(&f);
    let plan = plan_arena_allocation(&layout, &[], f.alloc.capacity() as u32).unwrap();
    assert_eq!(layout.max_depth, 4);
    assert_eq!(layout.band_layout.total_bands_used, 11);
    assert_eq!(plan.n_bands, 11);
    assert_eq!(plan.integration_band, 10);

    let mut seen = std::collections::BTreeSet::new();
    for op in &plan.cpu_ops {
        if let simthing_core::GateSpec::OrderBand(band) = op.gate {
            seen.insert(band);
        }
    }
    for band in 0..10 {
        assert!(seen.contains(&band), "missing OrderBand {band}");
    }
    assert!(
        !seen.contains(&10),
        "empty governed-pair fixture should not emit an integration op"
    );
}

#[test]
fn e11b_nested_preserves_participant_identity() {
    let f = nested_d4_fixture();
    let layout = layout_from_fixture(&f);
    let slots_by_host = &build_execution_plan(
        &f.reg,
        std::slice::from_ref(&f.arena),
        &f.root,
        &f.alloc,
        &f.scaffold,
        7,
    )
    .unwrap()
    .arena_participant_index;

    for ids in &f.participant_nodes {
        let expected = f.alloc.slot_of(ids.node_id).unwrap();
        assert_eq!(
            slots_by_host.get(&(ids.hosted_id, 0)),
            Some(&expected),
            "hosted participant identity should remain stable"
        );
        assert!(layout
            .iter_all()
            .iter()
            .any(|node| node.hosted_simthing_id == ids.hosted_id));
    }
}

#[test]
fn e11b_nested_rejects_gap_only_flat_star_leaf_claim() {
    let mut reg = DimensionRegistry::new();
    let _flow_id = register_flow(&mut reg);
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..2 {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let participants: Vec<_> = root
        .children
        .iter()
        .map(|child| {
            ExplicitParticipantSpec::flat(alloc.slot_of(child.id).unwrap(), child.id.raw())
        })
        .collect();
    let spec = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            name: "food".into(),
            flow_property: PropertyKey::new("core", "food_flow"),
            balance_property: None,
            max_participants: 4,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicySpec::Reject,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 1,
            expected_max_children_per_intermediate: 1,
            explicit_participants: participants,
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        ..Default::default()
    };
    let mut scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    let report = scaffold.reports[0].clone();
    let first = report.participant_sibling_first.unwrap();
    let count = report.participant_count;
    let parent_slot = first;
    let child_id = SimThing::new(SimThingKind::Cohort, 0).id;
    let gap_slot = try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        child_id,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap();

    assert!(
        !slot_in_participant_sibling_range(first, count, gap_slot),
        "gap-only child must not become an arena-root flat-star leaf"
    );
    let sibling_slots: Vec<u32> = (first..first + count).collect();
    assert!(slots_are_contiguous(&sibling_slots));
}

#[test]
fn e11b_nested_no_boundary_slot_compaction() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let f = nested_d4_fixture();
    let before: HashMap<_, _> = f
        .participant_nodes
        .iter()
        .map(|ids| (ids.node_id, f.alloc.slot_of(ids.node_id).unwrap()))
        .collect();
    assert_nested_cpu_gpu_parity(&f, 4);
    for (id, slot) in before {
        assert_eq!(f.alloc.slot_of(id), Some(slot));
    }
}

#[test]
fn e11b_nested_replay_same_seed_same_frames() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let f = nested_d3_fixture();
    let layout = layout_from_fixture(&f);
    let c = f.cols;
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_nodes = leaves(&layout);
    let n_dims = f.reg.total_columns as u32;
    let n_slots = f.alloc.capacity() as u32;
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    flat[idx(n_dims, root_slot, c.intrinsic_flow_col)] = 16.0;
    for node in layout.iter_all().into_iter().filter(|node| node.depth > 0) {
        flat[idx(n_dims, node.participant_slot, c.weight_col)] = 1.0;
    }
    flat[idx(n_dims, leaf_nodes[1].participant_slot, c.weight_col)] = 3.0;

    let a = run_gpu_allocation(&f, &layout, &flat);
    let b = run_gpu_allocation(&f, &layout, &flat);
    assert_eq!(a, b);
}

#[test]
fn e11b_nested_flat_star_regressions_unchanged() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = e11_flat_star::open_flat_star_session(4, false);
    assert_eq!(fx.layout.max_depth, 2);
    assert!(fx
        .layout
        .iter_all()
        .into_iter()
        .all(|node| node.depth == 0 || node.depth == 1));
    assert!(!fx.session.proto.flags.use_accumulator_resource_flow);
}

#[test]
fn e11b_nested_no_simthing_sim_arena_imports() {
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
fn e11b_nested_flag_default_false() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}

#[test]
fn e11b_child_share_formula_zero_weight_is_still_zero() {
    assert_eq!(
        child_share_cpu(5.0, 0.0, 0.0, 0.0).to_bits(),
        0.0_f32.to_bits()
    );
}

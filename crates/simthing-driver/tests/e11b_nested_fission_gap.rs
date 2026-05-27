//! E-11B-4 — nested fission / reserved-gap preservation hardening.

#[path = "support/e11_flat_star.rs"]
mod e11_flat_star;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry,
    EmlExpressionRegistry, LogTier, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    all_reserved_gap_slots, arena_participant_sibling_slots, build_execution_plan,
    gap_pool_snapshot, materialize_arena_participants, nested_fission_gap_report,
    plan_arena_allocation, refresh_fission_participant_child, register_child_share_formula,
    reserve_gap_pools_for_parent_slots, resolve_node_columns, run_arena_allocation_oracle,
    slot_in_participant_sibling_range, slots_are_contiguous, try_alloc_participant_child_in_gap,
    validate_resource_flow_preflight, ArenaParticipantAllocationReport, FissionPolicy,
    GapAllocError, GpuArenaDescriptor, HierarchyError, HierarchyNode, NodeColumnRefs,
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
    scaffold: simthing_driver::ArenaParticipantScaffold,
    arena: GpuArenaDescriptor,
    cols: NodeColumnRefs,
    arena_root_id: SimThingId,
    mid_ids: ParticipantIds,
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

    let mut scaffold = simthing_driver::ArenaParticipantScaffold::default();
    scaffold.arena_root_ids.insert(0, arena_root_id);
    for ids in [&root_ids, &mid_ids] {
        scaffold.index.by_host_and_arena.insert(
            (ids.hosted_id, 0),
            alloc.slot_of(ids.node_id).expect("slot"),
        );
    }

    NestedFixture {
        reg,
        root: world,
        alloc,
        scaffold,
        arena,
        cols,
        arena_root_id,
        mid_ids,
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

    let mut scaffold = simthing_driver::ArenaParticipantScaffold::default();
    scaffold.arena_root_ids.insert(0, arena_root_id);
    for ids in [&root_ids, &mid_ids] {
        scaffold.index.by_host_and_arena.insert(
            (ids.hosted_id, 0),
            alloc.slot_of(ids.node_id).expect("slot"),
        );
    }

    NestedFixture {
        reg,
        root: world,
        alloc,
        scaffold,
        arena,
        cols,
        arena_root_id,
        mid_ids,
    }
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

fn install_nested_gaps(f: &mut NestedFixture, gap_k: u32) {
    let layout = layout_from_fixture(f);
    let interiors = layout.interior_participant_slots();
    let gap_block_first = reserve_gap_pools_for_parent_slots(
        &mut f.scaffold,
        &mut f.alloc,
        &interiors,
        gap_k,
    );
    let siblings = arena_participant_sibling_slots(&f.root, f.arena_root_id, &f.alloc);
    f.scaffold.reports.push(ArenaParticipantAllocationReport {
        arena: "food".into(),
        root_slot: f.alloc.slot_of(f.arena_root_id).unwrap(),
        participant_count: siblings.len() as u32,
        reserved_gap_per_intermediate: gap_k,
        max_children_per_intermediate: 16,
        participant_sibling_first: siblings.first().copied(),
        gap_block_first,
    });
}

fn arena_sibling_range(f: &NestedFixture) -> (Option<u32>, u32) {
    let siblings = arena_participant_sibling_slots(&f.root, f.arena_root_id, &f.alloc);
    (siblings.first().copied(), siblings.len() as u32)
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
    }
}

#[test]
fn e11b_nested_reserved_gap_child_stays_outside_active_child_slotrange() {
    let mut f = nested_d3_fixture();
    install_nested_gaps(&mut f, 2);
    let layout = layout_from_fixture(&f);
    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();
    let mid_node = layout.find_node_by_slot(mid_slot).expect("mid node");
    let (arena_first, arena_count) = arena_sibling_range(&f);

    let report = nested_fission_gap_report(
        mid_slot,
        &mid_node.active_child_slots(),
        &f.scaffold,
        arena_first,
        arena_count,
    );
    assert!(report.gap_outside_active_child_span);
    assert!(report.gap_outside_arena_sibling_range);
    assert!(report.active_children_contiguous);

    let child_id = SimThing::new(SimThingKind::Cohort, 0).id;
    let gap_slot = try_alloc_participant_child_in_gap(
        &mut f.scaffold,
        mid_slot,
        child_id,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .expect("gap claim");

    let active = mid_node.active_child_slots();
    assert!(
        !active.contains(&gap_slot),
        "claimed gap child must stay outside active child SlotRange"
    );
    assert!(!slot_in_participant_sibling_range(
        arena_first.unwrap(),
        arena_count,
        gap_slot
    ));
}

#[test]
fn e11b_nested_gap_child_does_not_become_allocation_leaf() {
    let mut f = nested_d3_fixture();
    install_nested_gaps(&mut f, 1);
    let layout_before = layout_from_fixture(&f);
    let leaves_before: Vec<u32> = leaves(&layout_before)
        .iter()
        .map(|n| n.participant_slot)
        .collect();

    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();
    let gap_slot = try_alloc_participant_child_in_gap(
        &mut f.scaffold,
        mid_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .unwrap();

    let layout_after = layout_from_fixture(&f);
    let leaves_after: Vec<u32> = leaves(&layout_after)
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    assert_eq!(leaves_before, leaves_after);
    assert!(!layout_after.participant_slots().contains(&gap_slot));
    let (first, count) = arena_sibling_range(&f);
    assert!(!slot_in_participant_sibling_range(first.unwrap(), count, gap_slot));
}

#[test]
fn e11b_nested_parent_child_contiguity_preserved_after_gap_claim() {
    let mut f = nested_d3_fixture();
    install_nested_gaps(&mut f, 2);
    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();
    let siblings_before = arena_participant_sibling_slots(&f.root, f.arena_root_id, &f.alloc);

    try_alloc_participant_child_in_gap(
        &mut f.scaffold,
        mid_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .unwrap();

    let layout = layout_from_fixture(&f);
    for root in &layout.participant_roots {
        root.verify_child_contiguity().unwrap();
    }
    let siblings_after = arena_participant_sibling_slots(&f.root, f.arena_root_id, &f.alloc);
    assert_eq!(siblings_before, siblings_after);
    assert!(slots_are_contiguous(&siblings_after));
}

#[test]
fn e11b_nested_rejects_noncontiguous_active_children_without_compaction() {
    let mut f = nested_d3_fixture();
    install_nested_gaps(&mut f, 1);
    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();
    let flow_id = f.arena.flow_property_id;

    refresh_fission_participant_child(
        &mut f.scaffold,
        &mut f.root,
        mid_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        flow_id,
        &f.reg,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .expect("gap fission refresh should claim");

    let plan_err = build_execution_plan(
        &f.reg,
        std::slice::from_ref(&f.arena),
        &f.root,
        &f.alloc,
        &f.scaffold,
        8,
    )
    .unwrap_err();
    assert!(matches!(
        plan_err,
        HierarchyError::NonContiguousChildren { .. }
    ));
}

#[test]
fn e11b_nested_gap_claim_preserves_d3_cpu_gpu_parity_for_active_tree() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut f = nested_d3_fixture();
    install_nested_gaps(&mut f, 1);
    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();
    try_alloc_participant_child_in_gap(
        &mut f.scaffold,
        mid_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .unwrap();
    assert_nested_cpu_gpu_parity(&f, 3);
}

#[test]
fn e11b_nested_gap_claim_preserves_d4_cpu_gpu_parity_for_active_tree() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut f = nested_d4_fixture();
    install_nested_gaps(&mut f, 1);
    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();
    try_alloc_participant_child_in_gap(
        &mut f.scaffold,
        mid_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .unwrap();
    assert_nested_cpu_gpu_parity(&f, 4);
}

#[test]
fn e11b_nested_gap_exhaustion_rejects_without_partial_mutation() {
    let mut f = nested_d3_fixture();
    install_nested_gaps(&mut f, 1);
    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();
    let before = gap_pool_snapshot(&f.scaffold);
    let siblings_before = arena_participant_sibling_slots(&f.root, f.arena_root_id, &f.alloc);
    let layout_before = layout_from_fixture(&f);

    try_alloc_participant_child_in_gap(
        &mut f.scaffold,
        mid_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .unwrap();

    let err = try_alloc_participant_child_in_gap(
        &mut f.scaffold,
        mid_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .unwrap_err();
    assert!(matches!(err, GapAllocError::Exhausted { .. }));

    let after = gap_pool_snapshot(&f.scaffold);
    assert_eq!(before.get(&mid_slot).map(|v| v.len()), Some(1));
    assert_eq!(after.get(&mid_slot).map(|v| v.len()), Some(0));
    assert_eq!(
        arena_participant_sibling_slots(&f.root, f.arena_root_id, &f.alloc),
        siblings_before
    );
    assert_eq!(
        layout_from_fixture(&f).participant_slots(),
        layout_before.participant_slots()
    );
}

#[test]
fn e11b_nested_replay_same_seed_same_gap_state() {
    let mut f = nested_d3_fixture();
    install_nested_gaps(&mut f, 2);
    let mid_slot = f.alloc.slot_of(f.mid_ids.node_id).unwrap();

    fn claim_sequence(
        f: &mut NestedFixture,
        mid_slot: u32,
    ) -> (HashMap<u32, Vec<u32>>, Vec<u32>) {
        try_alloc_participant_child_in_gap(
            &mut f.scaffold,
            mid_slot,
            SimThing::new(SimThingKind::Cohort, 0).id,
            &mut f.alloc,
            FissionPolicy::Reject,
        )
        .unwrap();
        (
            gap_pool_snapshot(&f.scaffold),
            arena_participant_sibling_slots(&f.root, f.arena_root_id, &f.alloc),
        )
    }

    let (snap_a, siblings_a) = claim_sequence(&mut f, mid_slot);
    let mut f2 = nested_d3_fixture();
    install_nested_gaps(&mut f2, 2);
    let mid_slot2 = f2.alloc.slot_of(f2.mid_ids.node_id).unwrap();
    let (snap_b, siblings_b) = claim_sequence(&mut f2, mid_slot2);
    assert_eq!(snap_a, snap_b);
    assert_eq!(siblings_a, siblings_b);
}

#[test]
fn e11b_nested_flat_star_gap_regression_unchanged() {
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
        .map(|child| ExplicitParticipantSpec::flat(alloc.slot_of(child.id).unwrap(), child.id.raw()))
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
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let mut scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    let report = scaffold.reports[0].clone();
    let first = report.participant_sibling_first.unwrap();
    let count = report.participant_count;
    for gap_slot in all_reserved_gap_slots(&scaffold) {
        assert!(!slot_in_participant_sibling_range(first, count, gap_slot));
    }
    let arena_root_id = *scaffold.arena_root_ids.get(&0).unwrap();
    let parent_slot = arena_participant_sibling_slots(&root, arena_root_id, &alloc)[0];
    try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        SimThing::new(SimThingKind::Cohort, 0).id,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap();
    let siblings_after = arena_participant_sibling_slots(&root, arena_root_id, &alloc);
    assert!(slots_are_contiguous(&siblings_after));
    assert_eq!(siblings_after.len(), 2);
}

#[test]
fn e11b_nested_no_simthing_sim_arena_imports() {
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    let boundary = include_str!("../../simthing-sim/src/boundary.rs");
    for src in [sim_lib, boundary] {
        assert!(!src.contains("ArenaRegistry"));
        assert!(!src.contains("ArenaParticipant"));
        assert!(!src.contains("arena_hierarchy"));
        assert!(!src.contains("nested_fission_gap"));
    }
}

#[test]
fn e11b_nested_no_new_wgsl() {
    let wgsl_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../simthing-gpu/src/shaders");
    let entries: Vec<_> = std::fs::read_dir(&wgsl_root)
        .expect("shaders dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let allowed = ["accumulator_op.wgsl", "snapshot.wgsl", "world_summary.wgsl"];
    for name in &entries {
        assert!(
            allowed.contains(&name.as_str()),
            "unexpected WGSL file {name}"
        );
    }
}

#[test]
fn e11b_nested_flag_default_false() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    assert_eq!(
        ResourceFlowSpec::default().opt_in_mode,
        simthing_spec::ResourceFlowOptInMode::Disabled
    );
}

#[test]
fn e11b_nested_flat_star_regression_session_unchanged() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = e11_flat_star::open_flat_star_session(4, false);
    assert_eq!(fx.layout.max_depth, 2);
    assert!(!fx.session.proto.flags.use_accumulator_resource_flow);
}

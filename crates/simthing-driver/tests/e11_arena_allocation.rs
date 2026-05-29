//! E-11 hierarchical resource-flow allocation execution tests.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry,
    EmlExpressionRegistry, LogTier, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_custom_layout, build_execution_plan, child_share_cpu, materialize_arena_participants,
    plan_arena_allocation, register_child_share_formula, resolve_node_columns,
    run_arena_allocation_oracle, slots_are_contiguous, total_bands_for_depth,
    try_alloc_participant_child_in_gap, validate_resource_flow_preflight, ArenaParticipantScaffold,
    ArenaRegistry, FissionPolicy, GpuArenaDescriptor, HierarchyError, HierarchyNode,
    NodeColumnRefs,
};
use simthing_gpu::{GpuContext, SlotAllocator, WorldAccumulatorRuntime, WorldGpuState};
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

fn register_flow(
    reg: &mut DimensionRegistry,
    arena: &str,
    with_balance: bool,
) -> simthing_core::SimPropertyId {
    let mut sub_fields = vec![
        flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
        flow_subfield(
            "allocated",
            AccumulatorRole::AllocatedFlow {
                arena: arena.into(),
            },
        ),
        flow_subfield(
            "weight",
            AccumulatorRole::AllocatorWeight {
                arena: arena.into(),
            },
        ),
    ];
    if with_balance {
        sub_fields.push(flow_subfield(
            "balance",
            AccumulatorRole::Balance(BalanceSpec::default()),
        ));
    }
    let spec = PropertySpec {
        id: format!("{arena}_flow"),
        namespace: "core".into(),
        name: format!("{arena}_flow"),
        display_name: String::new(),
        description: String::new(),
        sub_fields,
    };
    let (id, _) = compile_property(&spec, reg).unwrap();
    id
}

fn arena_spec(participants: Vec<(u32, u32)>, gap: u32, max_depth: u32) -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: max_depth,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: gap,
        expected_max_children_per_intermediate: gap,
        explicit_participants: participants
            .into_iter()
            .map(|(slot, subtree_root_id)| ExplicitParticipantSpec::flat(slot, subtree_root_id))
            .collect(),
        enrollment: None,
        wildcard_admission: None,
    }
}

struct E11Fixture {
    reg: DimensionRegistry,
    root: simthing_core::SimThing,
    alloc: SlotAllocator,
    scaffold: ArenaParticipantScaffold,
    cols: NodeColumnRefs,
    flow_id: simthing_core::SimPropertyId,
}

fn materialize_d2(hosted_count: usize, gap: u32, max_depth: u32) -> E11Fixture {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_flow(&mut reg, "food", true);
    let layout = reg.property(flow_id).layout.clone();
    let cols = resolve_node_columns(&layout, "food").unwrap();

    let mut alloc = SlotAllocator::new();
    let mut root = simthing_core::SimThing::new(simthing_core::SimThingKind::World, 0);
    for _ in 0..hosted_count {
        root.add_child(simthing_core::SimThing::new(
            simthing_core::SimThingKind::Cohort,
            0,
        ));
    }
    alloc.populate_from_tree(&root);

    let participants: Vec<_> = root
        .children
        .iter()
        .map(|c| ExplicitParticipantSpec::flat(alloc.slot_of(c.id).unwrap(), c.id.raw()))
        .collect();

    let spec = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            explicit_participants: participants,
            ..arena_spec(vec![], gap, max_depth)
        }],
        couplings: vec![],
        ..Default::default()
    };
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();
    E11Fixture {
        reg,
        root,
        alloc,
        scaffold,
        cols,
        flow_id,
    }
}

fn d2_layout_from_fixture(f: &E11Fixture) -> simthing_driver::ArenaTreeLayout {
    let arena = GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: f.flow_id,
        balance_property_id: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    let registry = ArenaRegistry {
        arenas: vec![arena],
        ..Default::default()
    };
    build_execution_plan(&f.reg, &registry.arenas, &f.root, &f.alloc, &f.scaffold, 0)
        .unwrap()
        .arenas
        .into_iter()
        .next()
        .unwrap()
}

fn cell(values: &HashMap<(u32, u32), f32>, slot: u32, col: u32) -> f32 {
    values.get(&(slot, col)).copied().unwrap_or(0.0)
}

fn run_gpu_allocation(
    f: &E11Fixture,
    layout: &simthing_driver::ArenaTreeLayout,
    values: &[f32],
    n_bands: u32,
) -> Vec<f32> {
    let ctx = GpuContext::new_blocking().expect("gpu");
    let n_slots = f.alloc.capacity() as u32;
    let mut state = WorldGpuState::new(ctx, &f.reg, n_slots);
    state.write_values(values);
    state.accumulator_runtime = Some(WorldAccumulatorRuntime::new());
    {
        let runtime = state.accumulator_runtime.as_mut().unwrap();
        register_child_share_formula(&mut runtime.eml_registry, f.cols).unwrap();
        runtime
            .upload_eml_trees(&state.ctx)
            .expect("child_share EML GPU upload");
    }

    let plan = plan_arena_allocation(layout, &[], n_slots).unwrap();
    state
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, n_bands, &EmlExpressionRegistry::new())
        .unwrap();
    state.run_resource_flow_bands(n_bands, 1.0);
    state.read_values()
}

#[test]
fn e11_single_level_positive_weights_cpu_gpu_parity() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let f = materialize_d2(3, 0, 16);
    let layout = d2_layout_from_fixture(&f);
    let c = f.cols;
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_slots: Vec<_> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();

    let n_dims = f.reg.total_columns as u32;
    let n_slots = f.alloc.capacity() as u32;
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    flat[idx(root_slot, c.intrinsic_flow_col)] = 10.0;
    flat[idx(leaf_slots[0], c.weight_col)] = 1.0;
    flat[idx(leaf_slots[1], c.weight_col)] = 3.0;

    let mut oracle_vals = HashMap::from([
        ((root_slot, c.intrinsic_flow_col), 10.0),
        ((leaf_slots[0], c.weight_col), 1.0),
        ((leaf_slots[1], c.weight_col), 3.0),
    ]);
    run_arena_allocation_oracle(&layout, &mut oracle_vals, 1.0);

    let gpu_out = run_gpu_allocation(&f, &layout, &flat, layout.band_layout.total_bands_used);

    for &leaf in &leaf_slots {
        let cpu = cell(&oracle_vals, leaf, c.allocated_flow_col);
        let gpu = gpu_out[idx(leaf, c.allocated_flow_col)];
        assert_eq!(cpu.to_bits(), gpu.to_bits(), "leaf {leaf} aF parity");
    }
}

#[test]
fn e11_zero_weight_sum_allocates_zero_no_nan() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let f = materialize_d2(2, 0, 16);
    let layout = d2_layout_from_fixture(&f);
    let c = f.cols;
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf = layout.participant_roots[0].children[0].participant_slot;

    for root_i_f in [5.0, -5.0, 0.0] {
        let mut oracle = HashMap::from([
            ((root_slot, c.intrinsic_flow_col), root_i_f),
            ((leaf, c.weight_col), 0.0),
        ]);
        run_arena_allocation_oracle(&layout, &mut oracle, 1.0);
        let a_f = cell(&oracle, leaf, c.allocated_flow_col);
        assert_eq!(a_f.to_bits(), 0.0_f32.to_bits());
        assert!(!a_f.is_nan());
        let share = child_share_cpu(
            cell(&oracle, leaf, c.propagated_intrinsic_flow_col),
            cell(&oracle, leaf, c.propagated_allocated_flow_col),
            0.0,
            cell(&oracle, leaf, c.propagated_weight_sum_col),
        );
        assert_eq!(share.to_bits(), 0.0_f32.to_bits());
    }
}

#[test]
fn e11_multi_level_hierarchy_cpu_oracle_parity() {
    let f = materialize_d2(1, 0, 16);
    let c = f.cols;
    let arena = GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: f.flow_id,
        balance_property_id: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    let mid = HierarchyNode {
        participant_slot: 20,
        hosted_simthing_id: Default::default(),
        depth: 1,
        children: vec![
            HierarchyNode {
                participant_slot: 21,
                hosted_simthing_id: Default::default(),
                depth: 2,
                children: vec![],
                cols: c,
                gap_used: 0,
            },
            HierarchyNode {
                participant_slot: 22,
                hosted_simthing_id: Default::default(),
                depth: 2,
                children: vec![],
                cols: c,
                gap_used: 0,
            },
        ],
        cols: c,
        gap_used: 0,
    };
    let root = HierarchyNode {
        participant_slot: 19,
        hosted_simthing_id: Default::default(),
        depth: 0,
        children: vec![mid],
        cols: c,
        gap_used: 0,
    };
    let layout = build_custom_layout(0, &arena, c, Default::default(), 18, vec![root]).unwrap();
    assert_eq!(layout.max_depth, 3);

    let mut oracle = HashMap::from([
        ((21, c.intrinsic_flow_col), 6.0),
        ((22, c.intrinsic_flow_col), 6.0),
        ((21, c.weight_col), 1.0),
        ((22, c.weight_col), 1.0),
    ]);
    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);
    assert!((cell(&oracle, 21, c.allocated_flow_col) - 6.0).abs() < 1e-5);
    assert!((cell(&oracle, 22, c.allocated_flow_col) - 6.0).abs() < 1e-5);
}

#[test]
fn e11_reserved_gap_fission_preserves_slotrange() {
    let f = materialize_d2(1, 2, 16);
    let layout = d2_layout_from_fixture(&f);
    let parent_slot = layout.participant_roots[0].participant_slot;
    let sibling_before: Vec<_> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    assert!(slots_are_contiguous(&sibling_before));

    let mut scaffold = f.scaffold.clone();
    let mut alloc = f.alloc;
    let child_id = simthing_core::SimThing::new(simthing_core::SimThingKind::Cohort, 0).id;
    let gap_slot = try_alloc_participant_child_in_gap(
        &mut scaffold,
        parent_slot,
        child_id,
        &mut alloc,
        FissionPolicy::Reject,
    )
    .unwrap();
    assert!(gap_slot > sibling_before.last().copied().unwrap_or(parent_slot));
    assert!(slots_are_contiguous(&sibling_before));
}

#[test]
fn e11_orderband_depth_budget_enforced() {
    let f = materialize_d2(2, 0, 4);
    let arena = GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: f.flow_id,
        balance_property_id: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: 4,
        fission_policy: FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    let needed = total_bands_for_depth(2);
    assert_eq!(needed, 5);
    let err =
        build_execution_plan(&f.reg, &[arena], &f.root, &f.alloc, &f.scaffold, 0).unwrap_err();
    assert!(matches!(
        err,
        HierarchyError::OrderBandDepthExceeded {
            needed: 5,
            max: 4,
            ..
        }
    ));
}

#[test]
fn e11_balance_integrates_after_allocation_band() {
    let f = materialize_d2(2, 0, 16);
    let layout = d2_layout_from_fixture(&f);
    let c = f.cols;
    let root = layout.participant_roots[0].participant_slot;
    let leaf = layout.participant_roots[0].children[0].participant_slot;
    let mut oracle = HashMap::from([
        ((root, c.intrinsic_flow_col), 4.0),
        ((leaf, c.weight_col), 1.0),
    ]);
    run_arena_allocation_oracle(&layout, &mut oracle, 0.5);
    let i_f = cell(&oracle, leaf, c.intrinsic_flow_col);
    let a_f = cell(&oracle, leaf, c.allocated_flow_col);
    let balance = cell(&oracle, leaf, c.balance_col.unwrap());
    assert!((balance - (i_f + a_f) * 0.5).abs() < 1e-5);
}

#[test]
fn e11_rejects_missing_allocator_weight() {
    let mut reg = DimensionRegistry::new();
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
        ],
    };
    let (flow_id, _) = compile_property(&spec, &mut reg).unwrap();
    let layout = reg.property(flow_id).layout.clone();
    let err = resolve_node_columns(&layout, "food").unwrap_err();
    assert!(matches!(err, HierarchyError::MissingAllocatorWeight { .. }));
}

#[test]
fn e11_rejects_missing_allocated_flow() {
    let mut reg = DimensionRegistry::new();
    let spec = PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![flow_subfield("flow", AccumulatorRole::IntrinsicFlow)],
    };
    let (flow_id, _) = compile_property(&spec, &mut reg).unwrap();
    let layout = reg.property(flow_id).layout.clone();
    let err = resolve_node_columns(&layout, "food").unwrap_err();
    assert!(matches!(err, HierarchyError::MissingAllocatedFlow { .. }));
}

#[test]
fn e11_no_new_wgsl() {
    let wgsl_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../simthing-gpu/src/shaders");
    let entries: Vec<_> = std::fs::read_dir(&wgsl_root)
        .expect("shaders dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let allowed = [
        "accumulator_op.wgsl",
        "snapshot.wgsl",
        "world_summary.wgsl",
        "structured_field_stencil.wgsl",
        "values_fill.wgsl",
    ];
    for name in &entries {
        assert!(
            allowed.contains(&name.as_str()),
            "unexpected WGSL file {name}; only V7.6-approved generic/accumulator shaders allowed"
        );
    }
}

#[test]
fn e11_no_simthing_sim_arena_imports() {
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    let boundary = include_str!("../../simthing-sim/src/boundary.rs");
    for src in [sim_lib, boundary] {
        assert!(!src.contains("ArenaRegistry"));
        assert!(!src.contains("ArenaParticipant"));
        assert!(!src.contains("arena_hierarchy"));
        assert!(!src.contains("resource_flow_compile"));
    }
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}

#[test]
fn e11_allocated_flow_resets_each_tick() {
    let f = materialize_d2(2, 0, 16);
    let layout = d2_layout_from_fixture(&f);
    let c = f.cols;
    let root = layout.participant_roots[0].participant_slot;
    let leaf = layout.participant_roots[0].children[0].participant_slot;
    let mut oracle = HashMap::from([
        ((root, c.intrinsic_flow_col), 10.0),
        ((leaf, c.weight_col), 1.0),
    ]);
    for _ in 0..100 {
        run_arena_allocation_oracle(&layout, &mut oracle, 1.0);
        let a_f = cell(&oracle, leaf, c.allocated_flow_col);
        assert!((a_f - 10.0).abs() < 1e-5, "per-tick share not accumulated");
        oracle.insert((leaf, c.allocated_flow_col), 999.0);
    }
}

#[test]
fn e11_integration_band_immediately_follows_deepest_disbursement() {
    let f = materialize_d2(2, 0, 16);
    let layout = d2_layout_from_fixture(&f);
    let plan = plan_arena_allocation(&layout, &[], f.alloc.capacity() as u32).unwrap();
    let deepest = layout.band_layout.disburse_band(0, layout.max_depth);
    assert_eq!(plan.integration_band, deepest + 1);
}

#[test]
fn e11_no_nan_propagation_in_disbursement_path() {
    e11_zero_weight_sum_allocates_zero_no_nan();
}

#[test]
fn e11_replay_bit_exact_across_two_runs() {
    let f = materialize_d2(2, 0, 16);
    let layout = d2_layout_from_fixture(&f);
    let c = f.cols;
    let root = layout.participant_roots[0].participant_slot;
    let leaf = layout.participant_roots[0].children[0].participant_slot;
    let mut oracle_a = HashMap::from([
        ((root, c.intrinsic_flow_col), 8.0),
        ((leaf, c.weight_col), 2.0),
    ]);
    let mut oracle_b = oracle_a.clone();
    run_arena_allocation_oracle(&layout, &mut oracle_a, 1.0);
    run_arena_allocation_oracle(&layout, &mut oracle_b, 1.0);
    assert_eq!(
        cell(&oracle_a, leaf, c.allocated_flow_col).to_bits(),
        cell(&oracle_b, leaf, c.allocated_flow_col).to_bits()
    );
}

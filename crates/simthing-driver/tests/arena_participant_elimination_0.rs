//! ARENA-PARTICIPANT-DEPRECATION-0 — own-row sparse and dynamic live-GPU referees.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, ColumnIndex, DimensionRegistry, LogTier,
    SimThing, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, check_conservation, clone_for_replay, fixture_dynamic_single_fission,
    flat_star_observations, open_fixture_session, resolve_node_columns_for_property,
    run_arena_allocation_oracle, run_dynamic_enrollment_resync_cycles, run_resource_flow_burn_in,
    Scenario, SimSession,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowSpec, SpecVersion,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

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

fn sparse_owned_row_fixture() -> (Scenario, GameModeSpec) {
    let mut registry = DimensionRegistry::new();
    compile_property(
        &PropertySpec {
            id: "food_flow".into(),
            namespace: "remand".into(),
            name: "food_flow".into(),
            display_name: String::new(),
            description: String::new(),
            admission_disposition: Default::default(),
            sub_fields: vec![
                flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
                flow_subfield(
                    "allocated",
                    AccumulatorRole::AllocatedFlow {
                        arena: "sparse_food".into(),
                    },
                ),
                flow_subfield(
                    "weight",
                    AccumulatorRole::AllocatorWeight {
                        arena: "sparse_food".into(),
                    },
                ),
            ],
        },
        &mut registry,
    )
    .expect("register sparse food flow");

    let mut root = SimThing::new(SimThingKind::World, 0);
    let first = SimThing::new(SimThingKind::Cohort, 0);
    let first_id = first.id;
    root.add_child(first);
    root.add_child(SimThing::new(SimThingKind::Custom("gap-a".into()), 0));
    let second = SimThing::new(SimThingKind::Cohort, 0);
    let second_id = second.id;
    root.add_child(second);
    root.add_child(SimThing::new(SimThingKind::Custom("gap-b".into()), 0));
    let third = SimThing::new(SimThingKind::Cohort, 0);
    let third_id = third.id;
    root.add_child(third);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let participants = [first_id, second_id, third_id]
        .into_iter()
        .map(|id| {
            ExplicitParticipantSpec::flat(
                allocator.slot_of(id).expect("participant owns a row").raw(),
                id.raw(),
            )
        })
        .collect();

    let scenario = Scenario {
        name: "arena_elimination_sparse_owned_rows".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 16,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let game_mode = GameModeSpec {
        id: "arena_elimination_sparse_owned_rows".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: Vec::new(),
        overlays: Vec::new(),
        order_weight_classes: vec![],
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                name: "sparse_food".into(),
                flow_property: PropertyKey::new("remand", "food_flow"),
                balance_property: None,
                max_participants: 8,
                max_coupling_fanout: 4,
                max_orderband_depth: 8,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                explicit_participants: participants,
                enrollment: None,
                wildcard_admission: None,
            }],
            couplings: Vec::new(),
            ..Default::default()
        }),
        resource_economy: None,
        region_fields: Vec::new(),
        mapping_execution_profile: Default::default(),
    };
    (scenario, game_mode)
}

/// catches: sparse participant-owned rows race while reducing into one parent target cell.
#[test]
fn sparse_owned_rows_execute_single_writer_rf1_and_replay_exact_on_gpu() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
    let (scenario, game_mode) = sparse_owned_row_fixture();
    let mut session = SimSession::open_from_spec(scenario, &game_mode)
        .expect("sparse-row referee requires a supported live GPU");
    let adapter = session.state.ctx.adapter.get_info();

    let flow_id = session
        .proto
        .registry
        .id_of("remand", "food_flow")
        .expect("flow property");
    let cols = resolve_node_columns_for_property(&session.proto.registry, flow_id, "sparse_food")
        .expect("flow columns");
    let layout = build_execution_plan(&session.proto.registry, &session.spec_state.arena_registry)
        .expect("sparse execution plan")
        .arenas
        .into_iter()
        .next()
        .expect("one sparse arena");
    let root = layout.participant_roots[0].participant_slot;
    let leaves: Vec<_> = layout.participant_roots[0]
        .children
        .iter()
        .map(|child| child.participant_slot)
        .collect();
    assert_eq!(
        layout
            .participant_slots()
            .iter()
            .map(|slot| slot.raw())
            .collect::<Vec<_>>(),
        vec![1, 3, 5],
        "participants must retain deliberately non-contiguous owned rows"
    );

    let n_dims = session.state.n_dims;
    let cell_index = |slot: u32, col: ColumnIndex| (slot * n_dims + col.raw_u32()) as usize;
    let inputs = HashMap::from([
        ((root, cols.intrinsic_flow_col), 12.0_f32),
        ((leaves[0], cols.intrinsic_flow_col), 3.0_f32),
        ((leaves[1], cols.intrinsic_flow_col), 5.0_f32),
        ((leaves[0], cols.weight_col), 1.0_f32),
        ((leaves[1], cols.weight_col), 2.0_f32),
    ]);
    let mut initial = vec![0.0_f32; (session.state.n_slots * n_dims) as usize];
    for (&(slot, col), &value) in &inputs {
        initial[cell_index(slot.raw(), col)] = value;
    }

    let execute = |session: &mut SimSession| {
        session.state.install_resolved_values_at_boundary(&initial);
        session.state.run_resource_flow_bands(
            session.state.accumulator_resource_flow_bands,
            session.scenario.dt,
        );
        session.state.read_values()
    };
    let first = execute(&mut session);
    let replay = execute(&mut session);
    assert_eq!(
        first
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        replay
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        "sparse input-list execution must replay bit-exactly"
    );

    let mut oracle = inputs;
    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);
    let leaf_allocations: Vec<f32> = leaves
        .iter()
        .map(|slot| first[cell_index(slot.raw(), cols.allocated_flow_col)])
        .collect();
    assert_eq!(
        first[cell_index(root.raw(), cols.intrinsic_flow_sum_col)].to_bits(),
        8.0_f32.to_bits(),
        "sparse child intrinsic aggregate"
    );
    assert_eq!(
        first[cell_index(root.raw(), cols.weight_sum_col)].to_bits(),
        3.0_f32.to_bits(),
        "sparse child weight aggregate"
    );
    assert_eq!(
        leaf_allocations
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        vec![4.0_f32.to_bits(), 8.0_f32.to_bits()],
        "exact sparse-row allocations"
    );
    for &leaf in &leaves {
        assert_eq!(
            first[cell_index(leaf.raw(), cols.allocated_flow_col)].to_bits(),
            oracle
                .get(&(leaf, cols.allocated_flow_col))
                .copied()
                .unwrap_or(0.0)
                .to_bits(),
            "GPU sparse allocation must match the CPU oracle"
        );
    }

    let leaf_ids: Vec<_> = layout.participant_roots[0]
        .children
        .iter()
        .map(|child| child.hosted_simthing_id.raw() as u64)
        .collect();
    let (allocator, arena) = flat_star_observations(
        layout.participant_roots[0].hosted_simthing_id.raw() as u64,
        &leaf_ids,
        12.0,
        &leaf_allocations,
        Some(0.0),
        &[Some(0.0), Some(0.0)],
        0.0,
        0.0,
    );
    let conservation = check_conservation(&[], &[allocator], &[arena]);
    assert!(
        conservation.all_pass(),
        "sparse own-row RF-1: {conservation:?}"
    );
    println!(
        "ARENA-ELIM-SPARSE-GPU adapter={} backend={:?} device_type={:?} slots=[1,3,5] aggregate=8 weight_sum=3 allocations=[4,8] rf1=PASS replay=PASS",
        adapter.name, adapter.backend, adapter.device_type
    );
}

/// catches: fission enrollment remints a wrapper row, loses its parent edge, or resync mutates state.
#[test]
fn dynamic_fission_admits_the_existing_owned_row_once_and_replays_exactly() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
    let mut fixture = fixture_dynamic_single_fission();
    fixture.ticks = 4;
    fixture.sync_cycles = 0;
    let mut live = open_fixture_session(&fixture)
        .expect("dynamic-fission referee requires a supported live GPU");
    let adapter = live.session.state.ctx.adapter.get_info();
    let report = live
        .session
        .last_resource_flow_dynamic_enrollment_report
        .clone()
        .expect("dynamic enrollment report");
    assert_eq!(report.admissions.len(), 1);
    assert!(report.rejections.is_empty());
    assert_eq!(
        report.generation_after,
        report.generation_before + 1,
        "one admitted batch must bump registry generation exactly once"
    );

    let admission = &report.admissions[0];
    let owned_slot = live
        .session
        .proto
        .allocator
        .slot_of(admission.child_id)
        .expect("fission child already owns a row");
    assert_eq!(admission.participant_slot, owned_slot.raw());
    let member = live
        .session
        .spec_state
        .arena_registry
        .participants
        .iter()
        .find(|member| {
            member.arena_idx == admission.arena_idx && member.subtree_root == admission.child_id
        })
        .expect("admitted child member");
    assert_eq!(member.slot, owned_slot);
    assert_eq!(member.parent, Some(admission.parent_id));

    let stable_generation = live.session.spec_state.arena_registry.generation;
    let initial_ops = live
        .session
        .state
        .accumulator_runtime
        .as_ref()
        .expect("RF runtime")
        .resource_flow_ops
        .count;
    let initial_bands = live.session.state.accumulator_resource_flow_bands;
    let (syncs, final_ops, final_bands) =
        run_dynamic_enrollment_resync_cycles(&mut live.session, 3).expect("three stable resyncs");
    assert_eq!(syncs, 3);
    assert_eq!(final_ops, initial_ops);
    assert_eq!(final_bands, initial_bands);
    assert_eq!(
        live.session.spec_state.arena_registry.generation, stable_generation,
        "resync must not rebump registry generation"
    );

    let mut replay = clone_for_replay(&live, &fixture);
    fixture.expected_admissions = 0;
    fixture.expect_generation_bump = false;
    let live_burn =
        run_resource_flow_burn_in(&mut live, &fixture).expect("live own-row fission burn");
    let replay_burn =
        run_resource_flow_burn_in(&mut replay, &fixture).expect("replay own-row fission burn");
    assert_eq!(live_burn, replay_burn);
    assert_eq!(live_burn.max_abs_error.to_bits(), 0.0_f32.to_bits());
    assert!(live_burn.replay_bit_exact);
    assert_eq!(
        live.session.spec_state.arena_registry.generation, stable_generation,
        "GPU burn must not mutate membership generation"
    );
    println!(
        "ARENA-ELIM-FISSION-GPU adapter={} backend={:?} device_type={:?} child={} owned_slot={} parent={} generation={}->{} resyncs=3 rf1=PASS replay=PASS",
        adapter.name,
        adapter.backend,
        adapter.device_type,
        admission.child_id.raw(),
        owned_slot.raw(),
        admission.parent_id.raw(),
        report.generation_before,
        report.generation_after
    );
}
